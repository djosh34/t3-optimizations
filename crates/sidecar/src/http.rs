//! The sidecar's HTTP side, on localhost; `tailscale serve` puts it on the
//! tailnet with HTTPS (outcome 9). One request per connection.
//!
//! - `GET /...`: the web app's files from `--web-dir`; paths without a file
//!   get `index.html`.
//! - `POST /api/pair` `{"code": "<pairing code>"}`: exchanges a code from
//!   the t3 server's `t3 pair` with that server, then answers with a new
//!   device id and its 128-bit token (hex). Issue #30.
//! - `POST /api/rtc/offer`, bearer token: an SDP offer in, the answer out.
//!   Only the first connection of a phone needs it.
//! - `POST /api/link-report`, bearer token: the web app's link statistics,
//!   kept in the data dir (issue #29).
//! - `GET /sync`: a WebSocket that carries the same packets as the data
//!   channel, for when WebRTC fails.

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};
use tungstenite::protocol::{Role, WebSocketConfig};
use tungstenite::{Message, WebSocket};

use crate::Msg;
use crate::store::{hex, parse_token};
use crate::t3;

/// What the HTTP side asks of the main loop.
#[derive(Debug)]
pub enum Api {
    /// Makes a device for a phone whose pairing code t3 accepted. Replies
    /// with the device id and token.
    NewDevice {
        reply: Sender<Result<(String, [u8; 16]), String>>,
    },
    Offer {
        token: [u8; 16],
        sdp: String,
        reply: Sender<Result<String, ApiError>>,
    },
    Report {
        token: [u8; 16],
        body: Vec<u8>,
        reply: Sender<Result<(), ApiError>>,
    },
}

#[derive(Debug)]
pub enum ApiError {
    /// No device has this token.
    Unknown,
    Bad(String),
}

/// A WebSocket fallback connection.
#[derive(Debug)]
pub enum WsEvent {
    Open { id: u64, out: Sender<Vec<u8>> },
    Data { id: u64, data: Vec<u8> },
    Closed { id: u64 },
}

pub struct HttpConfig {
    pub listen: SocketAddr,
    pub web_dir: Option<PathBuf>,
    /// The t3 server, for pairing codes.
    pub t3_url: String,
}

const MAX_HEAD: usize = 16 << 10;
const MAX_BODY: usize = 1 << 20;

/// Binds and serves in background threads. Returns the bound address.
pub fn spawn(cfg: HttpConfig, to_main: Sender<Msg>) -> io::Result<SocketAddr> {
    let listener = TcpListener::bind(cfg.listen)?;
    let addr = listener.local_addr()?;
    let cfg = Arc::new(cfg);
    let next_ws = Arc::new(AtomicU64::new(1));
    thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let (cfg, to_main, next_ws) = (cfg.clone(), to_main.clone(), next_ws.clone());
            thread::spawn(move || {
                if let Err(e) = serve(stream, &cfg, &to_main, &next_ws) {
                    eprintln!("sidecar: http: {e}");
                }
            });
        }
    });
    Ok(addr)
}

struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(String::as_str)
    }

    fn token(&self) -> Option<[u8; 16]> {
        parse_token(self.header("authorization")?.strip_prefix("Bearer ")?)
    }
}

struct Response {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

impl Response {
    fn text(status: u16, body: impl Into<String>) -> Response {
        Response {
            status,
            content_type: "text/plain; charset=utf-8",
            body: body.into().into_bytes(),
        }
    }

    fn json(status: u16, v: &Value) -> Response {
        Response {
            status,
            content_type: "application/json",
            body: v.to_string().into_bytes(),
        }
    }
}

fn serve(
    stream: TcpStream,
    cfg: &HttpConfig,
    to_main: &Sender<Msg>,
    next_ws: &AtomicU64,
) -> Result<(), Box<dyn std::error::Error>> {
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let Some(req) = read_request(&mut reader)? else {
        return Ok(());
    };
    if req.method == "GET"
        && req.path == "/sync"
        && req
            .header("upgrade")
            .is_some_and(|u| u.eq_ignore_ascii_case("websocket"))
    {
        return websocket(stream, &req, to_main, next_ws);
    }
    let response = route(&req, cfg, to_main);
    write_response(stream, &response)?;
    Ok(())
}

fn read_request(reader: &mut BufReader<TcpStream>) -> io::Result<Option<Request>> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(None);
    }
    let mut parts = line.split_whitespace();
    let (Some(method), Some(target)) = (parts.next(), parts.next()) else {
        return Err(io::Error::other("bad request line"));
    };
    let path = target.split('?').next().unwrap_or("/").to_owned();
    let mut headers = HashMap::new();
    let mut size = line.len();
    loop {
        let mut h = String::new();
        let n = reader.read_line(&mut h)?;
        size += n;
        if n == 0 || size > MAX_HEAD {
            return Err(io::Error::other("headers too long or cut off"));
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((name, value)) = h.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
        }
    }
    let len: usize = headers
        .get("content-length")
        .and_then(|l| l.parse().ok())
        .unwrap_or(0);
    if len > MAX_BODY {
        return Err(io::Error::other("body too large"));
    }
    let mut body = vec![0; len];
    reader.read_exact(&mut body)?;
    Ok(Some(Request {
        method: method.to_owned(),
        path,
        headers,
        body,
    }))
}

fn write_response(mut stream: TcpStream, r: &Response) -> io::Result<()> {
    let reason = match r.status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {} {reason}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
        r.status,
        r.content_type,
        r.body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(&r.body)?;
    stream.flush()
}

fn route(req: &Request, cfg: &HttpConfig, to_main: &Sender<Msg>) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("POST", "/api/pair") => pair(req, cfg, to_main),
        ("POST", "/api/rtc/offer") => {
            let Some(token) = req.token() else {
                return Response::text(401, "no token");
            };
            let Ok(sdp) = String::from_utf8(req.body.clone()) else {
                return Response::text(400, "the offer is not UTF-8");
            };
            match ask(to_main, |reply| Api::Offer { token, sdp, reply }) {
                Ok(Ok(answer)) => Response {
                    status: 200,
                    content_type: "application/sdp",
                    body: answer.into_bytes(),
                },
                Ok(Err(e)) => api_error(e),
                Err(e) => Response::text(500, e),
            }
        }
        ("POST", "/api/link-report") => {
            let Some(token) = req.token() else {
                return Response::text(401, "no token");
            };
            let body = req.body.clone();
            match ask(to_main, |reply| Api::Report { token, body, reply }) {
                Ok(Ok(())) => Response::text(204, ""),
                Ok(Err(e)) => api_error(e),
                Err(e) => Response::text(500, e),
            }
        }
        ("GET" | "HEAD", _) => static_file(cfg.web_dir.as_deref(), &req.path),
        _ => Response::text(405, "method not allowed"),
    }
}

fn api_error(e: ApiError) -> Response {
    match e {
        ApiError::Unknown => Response::text(401, "unknown token"),
        ApiError::Bad(why) => Response::text(400, why),
    }
}

/// Sends a request to the main loop and waits for its reply.
fn ask<T>(to_main: &Sender<Msg>, make: impl FnOnce(Sender<T>) -> Api) -> Result<T, String> {
    let (reply, rx) = mpsc::channel();
    to_main
        .send(Msg::Api(make(reply)))
        .map_err(|_| "the sidecar is stopping".to_owned())?;
    rx.recv_timeout(Duration::from_secs(10))
        .map_err(|e| format!("no reply: {e}"))
}

fn pair(req: &Request, cfg: &HttpConfig, to_main: &Sender<Msg>) -> Response {
    let code = serde_json::from_slice::<Value>(&req.body)
        .ok()
        .and_then(|v| v["code"].as_str().map(str::to_owned));
    let Some(code) = code else {
        return Response::text(400, r#"expected {"code": "..."}"#);
    };
    // t3 checks the code and uses it up; the session it opens for the
    // phone is not used, the phone speaks only to the sidecar.
    if let Err(e) = t3::exchange(&cfg.t3_url, code.trim(), "t3opt phone") {
        return Response::text(403, format!("the pairing code was not accepted: {e}"));
    }
    match ask(to_main, |reply| Api::NewDevice { reply }) {
        Ok(Ok((device, token))) => {
            Response::json(200, &json!({"device": device, "token": hex(&token)}))
        }
        Ok(Err(e)) => Response::text(500, e),
        Err(e) => Response::text(500, e),
    }
}

fn static_file(web_dir: Option<&Path>, path: &str) -> Response {
    let Some(root) = web_dir else {
        return Response::text(404, "no web app here (--web-dir)");
    };
    let rel = Path::new(path.trim_start_matches('/'));
    if rel.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Response::text(404, "not found");
    }
    let mut file = root.join(rel);
    if path == "/" || (!file.is_file() && rel.extension().is_none()) {
        file = root.join("index.html");
    }
    match fs::read(&file) {
        Ok(body) => Response {
            status: 200,
            content_type: content_type(&file),
            body,
        },
        Err(_) => Response::text(404, "not found"),
    }
}

fn content_type(file: &Path) -> &'static str {
    match file.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json" | "map") => "application/json",
        Some("webmanifest") => "application/manifest+json",
        Some("wasm") => "application/wasm",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// The WebSocket fallback: binary messages are sync packets both ways.
fn websocket(
    stream: TcpStream,
    req: &Request,
    to_main: &Sender<Msg>,
    next_ws: &AtomicU64,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = req
        .header("sec-websocket-key")
        .ok_or("no Sec-WebSocket-Key")?;
    let accept = tungstenite::handshake::derive_accept_key(key.as_bytes());
    let mut s = stream.try_clone()?;
    write!(
        s,
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    )?;
    s.flush()?;
    stream.set_read_timeout(Some(Duration::from_millis(20)))?;
    stream.set_nodelay(true)?;
    let config = WebSocketConfig::default().max_message_size(Some(64 << 10));
    let mut ws = WebSocket::from_raw_socket(stream, Role::Server, Some(config));
    let id = next_ws.fetch_add(1, Ordering::Relaxed);
    let (out, out_rx) = mpsc::channel::<Vec<u8>>();
    to_main.send(Msg::Ws(WsEvent::Open { id, out }))?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        loop {
            loop {
                match out_rx.try_recv() {
                    Ok(data) => ws.send(Message::binary(data))?,
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return Ok(()),
                }
            }
            match ws.read() {
                Ok(Message::Binary(data)) => to_main.send(Msg::Ws(WsEvent::Data {
                    id,
                    data: data.to_vec(),
                }))?,
                Ok(Message::Close(_)) => return Ok(()),
                Ok(_) => {}
                Err(tungstenite::Error::Io(e))
                    if matches!(
                        e.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                    ) => {}
                Err(tungstenite::Error::ConnectionClosed) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        }
    })();
    // The main loop may be gone already.
    let _ = to_main.send(Msg::Ws(WsEvent::Closed { id }));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_paths_stay_in_the_web_dir() -> io::Result<()> {
        let dir = std::env::temp_dir().join(format!("sidecar-web-{}", std::process::id()));
        fs::create_dir_all(dir.join("assets"))?;
        fs::write(dir.join("index.html"), "<!doctype html>")?;
        fs::write(dir.join("assets/app.js"), "x")?;
        assert_eq!(static_file(Some(&dir), "/").status, 200);
        assert_eq!(
            static_file(Some(&dir), "/assets/app.js").content_type,
            "text/javascript; charset=utf-8"
        );
        // A route of the app gets the app.
        assert_eq!(
            static_file(Some(&dir), "/thread/abc").body,
            b"<!doctype html>"
        );
        assert_eq!(static_file(Some(&dir), "/missing.js").status, 404);
        assert_eq!(static_file(Some(&dir), "/../etc/passwd").status, 404);
        fs::remove_dir_all(&dir)
    }
}
