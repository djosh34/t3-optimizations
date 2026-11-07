//! The phone side of a lab run with `--client sidecar-native`. The lab
//! runs it in the phone container with `docker exec`, like driver.mjs.
//!
//! Argument: the path of a JSON config (see NativeConfig in
//! lab/internal/phone/phone.go).
//! stdout: one JSON event per line, each with `t` (Unix ms) and `ev`, the
//! same events driver.mjs prints.
//! stdin: "go" once the lab applied the profile, "stop" at the deadline.

use std::io::{BufRead, Write};
use std::net::IpAddr;
use std::process::ExitCode;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use native_phone::{Error, NativePhone, Via, approve_command, pair, prompt_command, succeeded};
use serde_json::{Value, json};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            emit("note", json!({"text": format!("native phone failed: {e}")}));
            eprintln!("native-phone: {e}");
            ExitCode::FAILURE
        }
    }
}

fn emit(ev: &str, data: Value) {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64);
    let mut line = json!({"t": t, "ev": ev});
    if let (Some(line), Value::Object(data)) = (line.as_object_mut(), data) {
        line.extend(data);
    }
    let mut out = std::io::stdout().lock();
    // The lab reads stdout; if it is gone there is no one to tell.
    let _ = writeln!(out, "{line}");
    let _ = out.flush();
}

struct Cfg {
    origin: String,
    code: String,
    thread: String,
    prompt: String,
    reply: String,
    kind: String,
    local: IpAddr,
}

fn config() -> Result<Cfg, Error> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: native-phone <config.json>")?;
    let v: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let s = |k: &str| -> Result<String, Error> {
        Ok(v[k]
            .as_str()
            .ok_or_else(|| format!("config: no {k}"))?
            .to_owned())
    };
    Ok(Cfg {
        origin: s("origin")?,
        code: s("code")?,
        thread: s("threadId")?,
        prompt: s("prompt")?,
        reply: s("reply")?,
        kind: s("kind")?,
        local: s("local")?.parse()?,
    })
}

fn commands() -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            if tx.send(line.trim().to_owned()).is_err() {
                break;
            }
        }
        // stdin closed: the lab is gone.
        let _ = tx.send("stop".to_owned());
    });
    rx
}

fn run() -> Result<(), Error> {
    let cfg = config()?;
    let stdin = commands();
    // The sidecar may still be starting.
    let started = Instant::now();
    let pairing = loop {
        match pair(&cfg.origin, &cfg.code) {
            Ok(p) => break p,
            Err(e) if started.elapsed() < Duration::from_secs(30) => {
                emit("note", json!({"text": format!("pairing, will retry: {e}")}));
                thread::sleep(Duration::from_secs(1));
            }
            Err(e) => return Err(e),
        }
    };
    emit("paired", json!({}));
    let via = Via::Rtc {
        local: cfg.local,
        server: None,
    };
    let mut phone = NativePhone::connect(&cfg.origin, pairing, via)?;
    let focus = phone.call("sidecar.focus", json!({"threadId": cfg.thread}));
    let deadline = Instant::now() + Duration::from_secs(120);
    while phone.reply(focus).is_none() || !warm(&phone, &cfg.thread) {
        if Instant::now() > deadline {
            return Err("the thread did not sync within 2 minutes on the clean link".into());
        }
        phone.step(Duration::from_millis(20))?;
    }
    emit("thread-open", json!({}));
    emit("ready", json!({}));

    let mut go = String::new();
    while go.is_empty() {
        if cfg.kind != "cold" {
            phone.step(Duration::from_millis(20))?;
        } else {
            thread::sleep(Duration::from_millis(20));
        }
        match stdin.try_recv() {
            Ok(c) => go = c,
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => go = "stop".into(),
        }
    }
    if go != "go" {
        emit("note", json!({"text": format!("expected go, got {go}")}));
        return Ok(());
    }
    if cfg.kind == "cold" {
        // The page was closed; it opens again from what it stored.
        emit("app-loading", json!({}));
        phone = phone.reload()?;
        emit("app-loaded", json!({}));
    }
    let base_rx = phone.stats().received_wire_bytes;
    let base_tx = phone.stats().sent_wire_bytes;

    let prompt = phone.command(&prompt_command(&cfg.thread, &cfg.prompt)?);
    emit("prompt-sent", json!({}));
    let mut confirmed = false;
    let mut shown = 0;
    let mut seen = vec![false; cfg.reply.len()];
    let mut approved: Vec<String> = Vec::new();
    let mut last_sample = Instant::now();
    let mut last_link = Instant::now();
    loop {
        match stdin.try_recv() {
            Ok(c) if c == "stop" => break,
            Ok(_) | Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => break,
        }
        phone.step(Duration::from_millis(20))?;
        if !confirmed && phone.reply(prompt).is_some_and(succeeded) {
            confirmed = true;
            emit("prompt-confirmed", json!({}));
        }
        if last_sample.elapsed() < Duration::from_millis(200) {
            continue;
        }
        last_sample = Instant::now();
        if last_link.elapsed() >= Duration::from_secs(10) {
            last_link = Instant::now();
            link(&phone);
        }
        for request in phone.bodies(&cfg.thread, "runtimeRequests") {
            let Some(id) = request["id"].as_str() else {
                continue;
            };
            if request["status"] == "pending" && !approved.iter().any(|a| a == id) {
                approved.push(id.to_owned());
                emit("approval-shown", json!({}));
                phone.command(&approve_command(&cfg.thread, id)?);
                emit("approval-clicked", json!({}));
            }
        }
        for (_, runs) in phone.texts(&cfg.thread, "assistant") {
            for run in runs {
                mark(&cfg.reply, &normalize(&run), &mut seen);
            }
        }
        let n = seen.iter().take_while(|s| **s).count();
        if n != shown {
            shown = n;
            emit("text", json!({"n": n}));
        }
        if shown == cfg.reply.len() {
            emit("complete", json!({}));
            break;
        }
    }
    let stats = phone.stats();
    let r = phone.reconnects();
    emit(
        "note",
        json!({"text": format!(
            "native phone: {} ICE restarts, {} new connections, loss up {:.0}%, {} packets it could not read{}",
            r.ice_restarts, r.generations, stats.loss * 100.0, phone.undecodable,
            phone.last_error.as_deref().map(|e| format!(" (last: {e})")).unwrap_or_default()
        )}),
    );
    emit(
        "stats",
        json!({
            "rxBytes": stats.received_wire_bytes - base_rx,
            "txBytes": stats.sent_wire_bytes - base_tx,
        }),
    );
    Ok(())
}

/// What the phone's side of the sync sees, for the run's log.
fn link(phone: &NativePhone) {
    let s = phone.stats();
    let r = phone.reconnects();
    emit(
        "link",
        json!({"text": format!(
            "send {:.0} B/s, delivered {:.0} B/s, loss {:.0}%, srtt {} ms, received {} packets, last heard {} ms ago, {} ICE restarts",
            s.send_rate, s.delivered_rate, s.loss * 100.0,
            s.srtt_ms.map_or("-".into(), |v| format!("{v:.0}")),
            s.received_packets,
            phone.silent_for().as_millis(),
            r.ice_restarts,
        )}),
    );
}

/// Whether the phone shows the warm-up reply.
fn warm(phone: &NativePhone, thread: &str) -> bool {
    phone
        .texts(thread, "assistant")
        .iter()
        .any(|(_, runs)| runs.iter().any(|r| r.contains("The lab thread is set up")))
}

/// Keeps ASCII letters and digits, as the lab compares text.
fn normalize(text: &str) -> String {
    text.chars().filter(char::is_ascii_alphanumeric).collect()
}

/// Marks which characters of the reply a run of text shows, as driver.mjs
/// does: pieces of at least 8 characters that appear in the reply, in
/// order; text the reply does not have is stepped over.
fn mark(reply: &str, text: &str, seen: &mut [bool]) {
    let mut rest = text;
    let mut from = 0;
    while rest.len() >= 8 {
        let (mut lo, mut hi) = (0, rest.len());
        while lo < hi {
            let mid = (lo + hi).div_ceil(2);
            if reply[from..].contains(&rest[..mid]) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        if lo >= 8 {
            let Some(at) = reply[from..].find(&rest[..lo]).map(|i| i + from) else {
                break;
            };
            for s in &mut seen[at..at + lo] {
                *s = true;
            }
            from = at + lo;
            rest = &rest[lo..];
        } else {
            rest = &rest[1..];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_pieces_of_the_reply() {
        let reply = "Thesyncloopstartsfromwhatthephonelastacknowledged";
        let mut seen = vec![false; reply.len()];
        mark(reply, "XXThesyncloopstarts", &mut seen);
        assert_eq!(
            seen.iter().take_while(|s| **s).count(),
            "Thesyncloopstarts".len()
        );
        // A piece from the middle does not make the prefix longer.
        mark(reply, "phonelastacknowledged", &mut seen);
        assert_eq!(
            seen.iter().take_while(|s| **s).count(),
            "Thesyncloopstarts".len()
        );
        mark(reply, "fromwhatthe", &mut seen);
        assert!(seen.iter().all(|s| *s));
    }
}
