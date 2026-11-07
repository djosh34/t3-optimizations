//! The server end of the chaos lab's pipe runs (`lab run --client pipe`).
//!
//! `pipe-lab webrtc --udp <addr> --http <addr> --log <file>` serves the data
//! channel on the UDP address and takes SDP offers over HTTP: `POST /offer`
//! with the offer as the body answers with the SDP answer. `GET /` is an
//! empty page for the browser to load, so the offer comes from the same
//! origin.
//!
//! `pipe-lab udp --udp <addr> --log <file>` does the same over plain UDP,
//! without WebRTC: it sends its ticks to the address it last heard from.
//!
//! Both send a 25-byte tick every 100 ms and log every tick they send and
//! receive to the log file, one JSON object per line.

use std::fs::File;
use std::io::{self, LineWriter, Write};
use std::net::{SocketAddr, UdpSocket};
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use pipe::tick::{self, Tick};
use pipe::{Config, PeerId, Pipe, PipeEvent};

const USAGE: &str = "usage:
  pipe-lab webrtc --udp <addr> --http <addr> --log <file>
  pipe-lab udp --udp <addr> --log <file>";

const TICK_EVERY: Duration = Duration::from_millis(100);

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pipe-lab: {e}");
            ExitCode::FAILURE
        }
    }
}

struct Args {
    mode: String,
    udp: SocketAddr,
    http: Option<SocketAddr>,
    log: String,
}

fn parse(args: Vec<String>) -> Result<Args, BoxError> {
    let mut it = args.into_iter();
    let mode = it.next().ok_or(USAGE)?;
    let (mut udp, mut http, mut log) = (None, None, None);
    while let Some(flag) = it.next() {
        let value = it
            .next()
            .ok_or_else(|| format!("{flag} needs a value\n{USAGE}"))?;
        match flag.as_str() {
            "--udp" => udp = Some(value.parse()?),
            "--http" => http = Some(value.parse()?),
            "--log" => log = Some(value),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}").into()),
        }
    }
    Ok(Args {
        mode,
        udp: udp.ok_or(USAGE)?,
        http,
        log: log.ok_or(USAGE)?,
    })
}

fn run(args: Vec<String>) -> Result<(), BoxError> {
    let args = parse(args)?;
    let log = Log(LineWriter::new(File::create(&args.log)?));
    match (args.mode.as_str(), args.http) {
        ("webrtc", Some(http)) => webrtc(args.udp, http, log),
        ("udp", None) => plain_udp(args.udp, log),
        _ => Err(USAGE.into()),
    }
}

/// The event log, one JSON object per line, each with `t` in Unix ms.
struct Log(LineWriter<File>);

impl Log {
    fn write(&mut self, ev: &str, fields: &str) -> io::Result<()> {
        writeln!(self.0, r#"{{"t":{},"ev":"{ev}"{fields}}}"#, unix_ms())
    }

    /// Writes an event with a text field and, when given, the peer.
    fn with_text(&mut self, ev: &str, peer: Option<PeerId>, text: &str) -> io::Result<()> {
        let peer = peer.map_or(String::new(), |p| format!(r#","peer":{}"#, p.0));
        self.write(ev, &format!(r#"{peer},"text":{}"#, json_string(text)))
    }

    fn sent(&mut self, seq: u32) -> io::Result<()> {
        self.write("sent", &format!(r#","seq":{seq}"#))
    }

    fn received(&mut self, data: &[u8]) -> io::Result<()> {
        match Tick::decode(data) {
            Some(t) => self.write("recv", &format!(r#","seq":{},"sent":{}"#, t.seq, t.sent_ms)),
            None => self.with_text("note", None, &format!("not a tick: {} bytes", data.len())),
        }
    }
}

/// s as a JSON string, quotes included.
fn json_string(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

/// An offer from the HTTP thread and where its answer goes.
type Offer = (String, mpsc::Sender<Result<String, String>>);

fn webrtc(udp: SocketAddr, http: SocketAddr, mut log: Log) -> Result<(), BoxError> {
    let mut pipe = Pipe::bind(Config {
        bind: udp,
        advertise: None,
        cert: None,
    })?;
    let server = tiny_http::Server::http(http)?;
    let (offers, offers_rx) = mpsc::channel::<Offer>();
    thread::spawn(move || serve_http(server, offers));

    let mut seqs: Vec<(PeerId, u32)> = Vec::new();
    let mut events = Vec::new();
    let mut next_tick = Instant::now() + TICK_EVERY;
    loop {
        while let Ok((offer, reply)) = offers_rx.try_recv() {
            let answer = pipe.accept_offer(&offer, None);
            match &answer {
                Ok((id, _)) => log.write("offer", &format!(r#","peer":{}"#, id.0))?,
                Err(e) => log.with_text("note", None, &format!("bad offer: {e}"))?,
            }
            if reply
                .send(answer.map(|(_, sdp)| sdp).map_err(|e| e.to_string()))
                .is_err()
            {
                log.with_text(
                    "note",
                    None,
                    "the offer's HTTP request gave up before the answer",
                )?;
            }
        }
        // Wake often enough to pick up new offers.
        let until = next_tick.min(Instant::now() + Duration::from_millis(20));
        pipe.poll(until, &mut events)?;
        for ev in events.drain(..) {
            match ev {
                PipeEvent::Open(id) => {
                    log.write("open", &format!(r#","peer":{}"#, id.0))?;
                    seqs.push((id, 0));
                }
                PipeEvent::Data(_, data) => log.received(&data)?,
                PipeEvent::IceState(id, state) => log.with_text("ice", Some(id), &state)?,
                PipeEvent::Failed(id, error) => log.with_text("failed", Some(id), &error)?,
                PipeEvent::Closed(id) => {
                    log.write("closed", &format!(r#","peer":{}"#, id.0))?;
                    seqs.retain(|(p, _)| *p != id);
                }
                // Lab runs always signal their offer.
                PipeEvent::Unknown { .. } => {}
            }
        }
        if Instant::now() >= next_tick {
            next_tick = (next_tick + TICK_EVERY).max(Instant::now());
            for (id, seq) in &mut seqs {
                let data = Tick {
                    seq: *seq,
                    sent_ms: unix_ms(),
                }
                .encode();
                if pipe.send(*id, &data)? {
                    log.sent(*seq)?;
                } else {
                    log.with_text(
                        "note",
                        Some(*id),
                        &format!("send buffer full, tick {seq} dropped"),
                    )?;
                }
                *seq += 1;
            }
        }
    }
}

fn serve_http(server: tiny_http::Server, offers: mpsc::Sender<Offer>) {
    for mut request in server.incoming_requests() {
        let response = match (request.method(), request.url()) {
            (tiny_http::Method::Get, "/") => {
                html("<!doctype html><meta charset=utf-8><title>pipe</title>")
            }
            (tiny_http::Method::Post, "/offer") => {
                let mut offer = String::new();
                match request.as_reader().read_to_string(&mut offer) {
                    Ok(_) => answer(&offers, offer),
                    Err(e) => text(400, format!("read offer: {e}")),
                }
            }
            _ => text(404, "not found".into()),
        };
        if let Err(e) = request.respond(response) {
            eprintln!("pipe-lab: http response: {e}");
        }
    }
}

fn answer(offers: &mpsc::Sender<Offer>, offer: String) -> tiny_http::Response<io::Cursor<Vec<u8>>> {
    let (reply, reply_rx) = mpsc::channel();
    if offers.send((offer, reply)).is_err() {
        return text(500, "the pipe stopped".into());
    }
    match reply_rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(sdp)) => text(200, sdp),
        Ok(Err(e)) => text(400, e),
        Err(e) => text(500, format!("no answer: {e}")),
    }
}

fn text(status: u16, body: String) -> tiny_http::Response<io::Cursor<Vec<u8>>> {
    tiny_http::Response::from_string(body).with_status_code(status)
}

fn html(body: &str) -> tiny_http::Response<io::Cursor<Vec<u8>>> {
    let mut response = text(200, body.into());
    if let Ok(header) = tiny_http::Header::from_bytes("Content-Type", "text/html; charset=utf-8") {
        response.add_header(header);
    }
    response
}

/// Plain UDP: the comparison for the data channel, with nothing but the link
/// in between.
fn plain_udp(udp: SocketAddr, mut log: Log) -> Result<(), BoxError> {
    let socket = UdpSocket::bind(udp)?;
    let mut buf = [0u8; 2000];
    let mut phone: Option<SocketAddr> = None;
    let mut seq = 0u32;
    let mut next_tick = Instant::now() + TICK_EVERY;
    loop {
        let now = Instant::now();
        if now >= next_tick {
            next_tick = (next_tick + TICK_EVERY).max(now);
            if let Some(addr) = phone {
                let data = Tick {
                    seq,
                    sent_ms: unix_ms(),
                }
                .encode();
                match socket.send_to(&data, addr) {
                    Ok(_) => log.sent(seq)?,
                    Err(e) => log.with_text("note", None, &format!("send tick {seq}: {e}"))?,
                }
                seq += 1;
            }
            continue;
        }
        socket.set_read_timeout(Some(next_tick - now))?;
        match socket.recv_from(&mut buf) {
            Ok((n, source)) => {
                if n == tick::SIZE && phone != Some(source) {
                    log.with_text("open", None, &source.to_string())?;
                    phone = Some(source);
                }
                log.received(&buf[..n])?;
            }
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) => {}
            Err(e) => return Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::json_string;

    #[test]
    fn json_string_escapes() {
        assert_eq!(json_string(r#"a "b" \ c"#), r#""a \"b\" \\ c""#);
        assert_eq!(json_string("line\nbreak"), r#""line\u000abreak""#);
    }
}
