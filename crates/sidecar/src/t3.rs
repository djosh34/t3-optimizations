//! The sidecar as a client of the forked t3 server: it pairs itself with
//! `t3 pair`, keeps the bearer token in its data dir, and speaks Effect RPC
//! JSON over the server's local WebSocket. Issues #4 and #13.
//!
//! One thread owns the WebSocket. It sends the main loop's requests, acks
//! every stream chunk, pings every 5 s, and reconnects by itself, sending
//! again every request that had no exit yet: subscriptions start over with
//! a snapshot, and t3 applies a command once per `commandId`.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tungstenite::{Message, WebSocket};

use crate::Msg;
use crate::store::write_private;

/// Where the forked server is and how to run its CLI.
#[derive(Debug, Clone)]
pub struct T3Config {
    /// Its HTTP origin, such as `http://127.0.0.1:13773`.
    pub url: String,
    /// Its `--base-dir`; every `t3` command passes it.
    pub base_dir: PathBuf,
    /// How to run `t3`, such as `["node", ".../apps/server/src/bin.ts"]`.
    pub command: Vec<String>,
    /// Where the bearer token is kept.
    pub token_file: PathBuf,
}

/// From the main loop to the t3 thread.
#[derive(Debug)]
pub enum Out {
    Request {
        id: u64,
        tag: String,
        payload: Value,
    },
    /// Ends a stream request.
    Interrupt(u64),
}

/// From the t3 thread to the main loop.
#[derive(Debug)]
pub enum In {
    /// Connected; every open request was sent again.
    Up,
    Down(String),
    Chunk {
        id: u64,
        values: Vec<Value>,
    },
    Exit {
        id: u64,
        exit: Value,
    },
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// The orchestration protocol this fork speaks (contracts/environment.ts).
pub const PROTOCOL: &str = "2";

const PING_EVERY: Duration = Duration::from_secs(5);
/// The server pings nothing; a socket with no frame for this long is dead.
const SILENT_LIMIT: Duration = Duration::from_secs(15);

/// The bearer token, shared by the WebSocket thread and the HTTP fetches.
/// When the server rejects it, the next use pairs again.
pub struct Auth {
    cfg: T3Config,
    token: Mutex<Option<String>>,
}

impl Auth {
    pub fn new(cfg: T3Config) -> Arc<Auth> {
        let token = fs::read_to_string(&cfg.token_file)
            .ok()
            .map(|t| t.trim().to_owned())
            .filter(|t| !t.is_empty());
        Arc::new(Auth {
            cfg,
            token: Mutex::new(token),
        })
    }

    pub fn url(&self) -> &str {
        &self.cfg.url
    }

    /// The bearer token, pairing first when there is none.
    pub fn bearer(&self) -> Result<String, Error> {
        let mut token = self.token.lock().map_err(|_| "auth lock poisoned")?;
        if let Some(t) = token.as_ref() {
            return Ok(t.clone());
        }
        let code = mint_pairing_code(&self.cfg, "t3opt sidecar")?;
        let t = exchange(&self.cfg.url, &code, "t3opt sidecar")?;
        write_private(&self.cfg.token_file, t.as_bytes())?;
        *token = Some(t.clone());
        Ok(t)
    }

    /// Forgets a token the server rejected.
    pub fn rejected(&self, bad: &str) {
        if let Ok(mut token) = self.token.lock()
            && token.as_deref() == Some(bad)
        {
            *token = None;
        }
    }

    /// GETs a path of the server with the bearer token, pairing again once
    /// if the server rejects it.
    pub fn get(&self, path: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{}{path}", self.cfg.url);
        for _ in 0..2 {
            let token = self.bearer()?;
            let response = ureq::get(&url)
                .header("authorization", &format!("Bearer {token}"))
                .header("x-t3-orchestration-protocol", PROTOCOL)
                .config()
                .http_status_as_error(false)
                .build()
                .call()?;
            let status = response.status().as_u16();
            if status == 401 {
                self.rejected(&token);
                continue;
            }
            let mut body = response.into_body();
            let bytes = body.with_config().limit(64 << 20).read_to_vec()?;
            if status != 200 {
                return Err(format!("GET {path}: HTTP {status}").into());
            }
            return Ok(bytes);
        }
        Err(format!("GET {path}: the server rejected a fresh token").into())
    }
}

/// Mints a one-time pairing code with the server's own CLI, `t3 pair
/// --base-dir <its dir>`. Codes live 5 minutes and work once.
pub fn mint_pairing_code(cfg: &T3Config, label: &str) -> Result<String, Error> {
    let (program, args) = cfg.command.split_first().ok_or("empty t3 command")?;
    let output = Command::new(program)
        .args(args)
        .arg("pair")
        .arg("--base-dir")
        .arg(&cfg.base_dir)
        .args(["--label", label])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let tail: Vec<_> = stderr.lines().rev().take(5).collect();
        return Err(format!("t3 pair failed ({}): {}", output.status, tail.join(" | ")).into());
    }
    stdout
        .lines()
        .find_map(|l| l.strip_prefix("Token: "))
        .map(|t| t.trim().to_owned())
        .ok_or_else(|| "t3 pair printed no token".into())
}

/// Exchanges a pairing code for a bearer token (t3's OAuth token exchange).
pub fn exchange(url: &str, code: &str, label: &str) -> Result<String, Error> {
    let response = ureq::post(&format!("{url}/oauth/token"))
        .config()
        .http_status_as_error(false)
        .build()
        .send_form([
            (
                "grant_type",
                "urn:ietf:params:oauth:grant-type:token-exchange",
            ),
            ("subject_token", code),
            (
                "subject_token_type",
                "urn:t3:params:oauth:token-type:environment-bootstrap",
            ),
            (
                "requested_token_type",
                "urn:ietf:params:oauth:token-type:access_token",
            ),
            ("client_label", label),
        ])?;
    let status = response.status().as_u16();
    let body: Value = serde_json::from_reader(response.into_body().into_reader())?;
    if status != 200 {
        return Err(format!("token exchange: HTTP {status}: {body}").into());
    }
    body["access_token"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "token exchange returned no access_token".into())
}

/// Starts the WebSocket thread. It runs until `out` is dropped.
pub fn spawn(auth: Arc<Auth>, out: Receiver<Out>, to_main: Sender<Msg>) {
    thread::spawn(move || {
        let mut open: BTreeMap<u64, (String, Value)> = BTreeMap::new();
        let mut backoff = Duration::from_millis(500);
        loop {
            let reason = match connect(&auth) {
                Ok(ws) => {
                    backoff = Duration::from_millis(500);
                    if to_main.send(Msg::T3(In::Up)).is_err() {
                        return;
                    }
                    match run(ws, &out, &to_main, &mut open) {
                        Ok(()) => return,
                        Err(e) => e.to_string(),
                    }
                }
                Err(e) => e.to_string(),
            };
            if to_main.send(Msg::T3(In::Down(reason))).is_err() {
                return;
            }
            // Requests made while down wait for the next connection.
            if !drain_out(&out, &mut open, None) {
                return;
            }
            thread::sleep(backoff);
            backoff = (backoff * 2).min(Duration::from_secs(10));
        }
    });
}

type Ws = WebSocket<TcpStream>;

fn connect(auth: &Auth) -> Result<Ws, Error> {
    let token = auth.bearer()?;
    let response = ureq::post(&format!("{}/api/auth/websocket-ticket/rpc", auth.url()))
        .header("authorization", &format!("Bearer {token}"))
        .config()
        .http_status_as_error(false)
        .build()
        .send_empty()?;
    let status = response.status().as_u16();
    if status == 401 {
        auth.rejected(&token);
        return Err("the server rejected the bearer token; pairing again".into());
    }
    let body: Value = serde_json::from_reader(response.into_body().into_reader())?;
    let ticket = body["ticket"]
        .as_str()
        .ok_or_else(|| format!("websocket ticket: HTTP {status}: {body}"))?;
    let host = auth
        .url()
        .strip_prefix("http://")
        .ok_or("the t3 URL must be http:// on this host")?
        .trim_end_matches('/');
    let stream = TcpStream::connect(host)?;
    stream.set_nodelay(true)?;
    let url = format!(
        "ws://{host}/ws?orchestrationProtocol={PROTOCOL}&wsTicket={}",
        percent_encode(ticket)
    );
    let (ws, _) = tungstenite::client(url.as_str(), stream).map_err(|e| e.to_string())?;
    ws.get_ref()
        .set_read_timeout(Some(Duration::from_millis(20)))?;
    Ok(ws)
}

/// Runs one connection until it fails, or until `out` is dropped (Ok).
fn run(
    mut ws: Ws,
    out: &Receiver<Out>,
    to_main: &Sender<Msg>,
    open: &mut BTreeMap<u64, (String, Value)>,
) -> Result<(), Error> {
    for (id, (tag, payload)) in open.iter() {
        send(&mut ws, &request(*id, tag, payload))?;
    }
    let mut last_ping = Instant::now();
    let mut last_heard = Instant::now();
    loop {
        if !drain_out(out, open, Some(&mut ws)) {
            return Ok(());
        }
        if last_ping.elapsed() >= PING_EVERY {
            send(&mut ws, &json!({"_tag": "Ping"}))?;
            last_ping = Instant::now();
        }
        if last_heard.elapsed() >= SILENT_LIMIT {
            return Err("no frame from the server for 15 s".into());
        }
        let message = match ws.read() {
            Ok(m) => m,
            Err(tungstenite::Error::Io(e))
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                continue;
            }
            Err(e) => return Err(e.into()),
        };
        last_heard = Instant::now();
        let text = match message {
            Message::Text(t) => t.to_string(),
            Message::Binary(b) => String::from_utf8(b.to_vec())?,
            Message::Close(_) => return Err("the server closed the socket".into()),
            _ => continue,
        };
        let value: Value = serde_json::from_str(&text)?;
        let frames = match value {
            Value::Array(frames) => frames,
            frame => vec![frame],
        };
        for frame in frames {
            handle_frame(&mut ws, frame, to_main, open)?;
        }
    }
}

fn handle_frame(
    ws: &mut Ws,
    frame: Value,
    to_main: &Sender<Msg>,
    open: &mut BTreeMap<u64, (String, Value)>,
) -> Result<(), Error> {
    let tag = frame["_tag"].as_str().unwrap_or_default();
    let id = || -> Option<u64> {
        let v = &frame["requestId"];
        v.as_u64().or_else(|| v.as_str()?.parse().ok())
    };
    match tag {
        "Chunk" => {
            let Some(id) = id() else { return Ok(()) };
            // Streams wait for an ack before the next chunk.
            send(ws, &json!({"_tag": "Ack", "requestId": id.to_string()}))?;
            let values = match &frame["values"] {
                Value::Array(v) => v.clone(),
                _ => Vec::new(),
            };
            to_main.send(Msg::T3(In::Chunk { id, values }))?;
        }
        "Exit" => {
            let Some(id) = id() else { return Ok(()) };
            open.remove(&id);
            to_main.send(Msg::T3(In::Exit {
                id,
                exit: frame["exit"].clone(),
            }))?;
        }
        "Defect" => return Err(format!("server defect: {}", frame["defect"]).into()),
        // Pong and anything newer.
        _ => {}
    }
    Ok(())
}

/// Sends what the main loop queued, when connected. False when the main
/// loop is gone.
fn drain_out(
    out: &Receiver<Out>,
    open: &mut BTreeMap<u64, (String, Value)>,
    mut ws: Option<&mut Ws>,
) -> bool {
    loop {
        let msg = match out.try_recv() {
            Ok(m) => m,
            Err(TryRecvError::Empty) => return true,
            Err(TryRecvError::Disconnected) => return false,
        };
        let frame = match msg {
            Out::Request { id, tag, payload } => {
                let frame = request(id, &tag, &payload);
                open.insert(id, (tag, payload));
                frame
            }
            Out::Interrupt(id) => {
                if open.remove(&id).is_none() {
                    continue;
                }
                json!({"_tag": "Interrupt", "requestId": id.to_string(), "interruptors": []})
            }
        };
        if let Some(ws) = ws.as_deref_mut()
            && send(ws, &frame).is_err()
        {
            // The read side sees the broken socket next; the request stays
            // open and goes out again on the next connection.
            return true;
        }
    }
}

fn request(id: u64, tag: &str, payload: &Value) -> Value {
    json!({"_tag": "Request", "id": id.to_string(), "tag": tag, "payload": payload, "headers": []})
}

fn send(ws: &mut Ws, frame: &Value) -> Result<(), Error> {
    ws.send(Message::text(frame.to_string()))?;
    Ok(())
}

/// Percent-encodes everything but unreserved URL characters.
pub fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encodes_tickets() {
        assert_eq!(percent_encode("a.b-c_d~e"), "a.b-c_d~e");
        assert_eq!(percent_encode("a+b/c="), "a%2Bb%2Fc%3D");
    }
}
