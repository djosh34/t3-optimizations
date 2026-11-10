//! Opens threads one after another, as the owner taps them, and reports how
//! long each took: for the live measurement of issue #84 against a sidecar
//! on a big history. It asks with `sidecar.focus` only and never sends
//! `thread.visit`, so no thread's read mark changes.
//!
//! ```sh
//! open-threads --origin http://127.0.0.1:12491 --pairing phone1.json [--code <t3 pair code>] \
//!   --t3-url http://127.0.0.1:3773 --t3-token-file <sidecar data dir>/t3-token --threads list.tsv
//! ```
//!
//! `--threads` has one `label<TAB>thread id` per line; `--settle-ms` waits
//! after connecting, `--dwell-ms` after each open (default 2000). The pairing file
//! keeps the device and token; without it, `--code` pairs a new device.
//! Before each open the tool reads the thread's newest item from t3's
//! bounded snapshot; then it times, from the focus call, until the phone
//! stored the thread's frame and that item (its text, if it has one).
//! stdout: one JSON line per thread with its label and the times, never an
//! id or a title. It fails when a thread's frame or newest item did not
//! arrive within `--timeout-s` (default 60).

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::ExitCode;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn wall_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

use native_phone::{Error, NativePhone, Pairing, Via, pair};
use serde_json::{Value, json};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("open-threads: {e}");
            ExitCode::FAILURE
        }
    }
}

fn args() -> Result<HashMap<String, String>, Error> {
    let mut out = HashMap::new();
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let name = flag
            .strip_prefix("--")
            .ok_or_else(|| format!("unexpected {flag}"))?;
        let value = it.next().ok_or_else(|| format!("{flag} needs a value"))?;
        out.insert(name.to_owned(), value);
    }
    Ok(out)
}

/// The sidecar's object id of a list of names (`crates/sidecar/src/world.rs`).
fn object_id(parts: &[&str]) -> u64 {
    let mut buf = Vec::new();
    for p in parts {
        buf.extend_from_slice(p.as_bytes());
        buf.push(0);
    }
    sync_core::hash(&buf) & sync_core::wire::MAX_ID
}

fn percent_encode(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}

/// The newest item of a thread, from t3's bounded snapshot: the sidecar's
/// object that holds it last (its text for a message or reasoning item,
/// else its body) and that object's length at least.
fn newest(t3: &str, token: &str, thread: &str) -> Result<Option<(u64, u64)>, Error> {
    let url = format!(
        "{t3}/api/orchestration/threads/{}/bounded",
        percent_encode(thread)
    );
    let body = ureq::get(&url)
        .header("authorization", &format!("Bearer {token}"))
        // As the sidecar sends it (crates/sidecar/src/t3.rs).
        .header("x-t3-orchestration-protocol", "2")
        .call()?
        .into_body()
        .with_config()
        .limit(64 << 20)
        .read_to_string()?;
    let v: Value = serde_json::from_str(&body)?;
    let Some(item) = v["projection"]["turnItems"]
        .as_array()
        .into_iter()
        .flatten()
        .max_by_key(|i| i["ordinal"].as_u64())
    else {
        return Ok(None);
    };
    let id = item["id"].as_str().unwrap_or_default();
    Ok(Some(match item["text"].as_str() {
        Some(text) => {
            let key = item["messageId"].as_str().unwrap_or(id);
            (object_id(&["text", thread, key]), text.len() as u64)
        }
        None => (object_id(&["body", thread, "turnItems", id]), 1),
    }))
}

fn run() -> Result<(), Error> {
    let a = args()?;
    let get = |k: &str| a.get(k).cloned().ok_or_else(|| format!("missing --{k}"));
    let origin = get("origin")?;
    let pairing_file = get("pairing")?;
    let pairing = match std::fs::read_to_string(&pairing_file) {
        Ok(text) => {
            let v: Value = serde_json::from_str(&text)?;
            let token = v["token"].as_str().ok_or("pairing file: no token")?;
            let mut t = [0u8; 16];
            for (i, b) in t.iter_mut().enumerate() {
                *b = u8::from_str_radix(token.get(2 * i..2 * i + 2).ok_or("bad token")?, 16)?;
            }
            Pairing {
                device: v["device"]
                    .as_str()
                    .ok_or("pairing file: no device")?
                    .to_owned(),
                token: t,
            }
        }
        Err(_) => {
            let p = pair(&origin, &get("code")?)?;
            let token: String = p.token.iter().map(|b| format!("{b:02x}")).collect();
            std::fs::write(
                &pairing_file,
                json!({"device": p.device, "token": token}).to_string(),
            )?;
            p
        }
    };
    let t3 = get("t3-url")?;
    let token = std::fs::read_to_string(get("t3-token-file")?)?
        .trim()
        .to_owned();
    let list = std::fs::read_to_string(get("threads")?)?;
    let dwell = Duration::from_millis(a.get("dwell-ms").map_or(Ok(2_000), |v| v.parse())?);
    let timeout = Duration::from_secs(a.get("timeout-s").map_or(Ok(60), |v| v.parse())?);
    let started = Instant::now();
    let mut phone = NativePhone::connect(
        &origin,
        pairing,
        if a.get("via").is_some_and(|v| v == "ws") {
            Via::Ws
        } else {
            Via::Rtc {
                local: IpAddr::V4(Ipv4Addr::LOCALHOST),
                server: None,
            }
        },
    )?;
    // Connected once a call comes back.
    let seq = phone.call("sidecar.focus", json!({"threadId": null}));
    while phone.reply(seq).is_none() {
        if started.elapsed() > timeout {
            return Err("no reply from the sidecar".into());
        }
        phone.step(Duration::from_millis(5))?;
    }
    println!(
        "{}",
        json!({"label": "connect", "ms": started.elapsed().as_millis() as u64})
    );
    // What a phone that just connected gets first is not part of an open.
    let settle = Duration::from_millis(a.get("settle-ms").map_or(Ok(0), |v| v.parse())?);
    let end = Instant::now() + settle;
    while Instant::now() < end {
        phone.step(Duration::from_millis(5))?;
    }
    let mut missed = 0;
    for line in list.lines().filter(|l| !l.trim().is_empty()) {
        let (label, thread) = line
            .split_once('\t')
            .ok_or("a line is label<TAB>thread id")?;
        let load = std::process::Command::new("uptime").output()?;
        let http_at = wall_ms();
        let preflight = Instant::now();
        let expect = newest(&t3, &token, thread)?;
        let http_ms = preflight.elapsed().as_secs_f64() * 1000.0;
        let frame = object_id(&["frame", thread]);
        let text = expect;
        phone.set_trace_target(text.map(|(id, _)| id));
        let request_at = wall_ms();
        let stats_before = phone.stats();
        let t0 = Instant::now();
        let seq = phone.call("sidecar.focus", json!({"threadId": thread}));
        let (mut frame_ms, mut newest_ms) = (None, None);
        let (mut first_ms, mut reply_ms) = (None, None);
        let mut max_step_ms = 0.0f64;
        while t0.elapsed() < timeout
            && (frame_ms.is_none() || (text.is_some() && newest_ms.is_none()))
        {
            let step_at = Instant::now();
            phone.step(Duration::from_millis(1))?;
            max_step_ms = max_step_ms.max(step_at.elapsed().as_secs_f64() * 1000.0);
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            if reply_ms.is_none() && phone.reply(seq).is_some() {
                reply_ms = Some(ms);
            }
            if first_ms.is_none() && text.is_some_and(|(id, _)| phone.object(id).is_some()) {
                first_ms = Some(ms);
            }
            if frame_ms.is_none() && phone.object(frame).is_some_and(|o| o.complete()) {
                frame_ms = Some(ms);
            }
            if newest_ms.is_none()
                && let Some((id, len)) = text
                && phone
                    .object(id)
                    .is_some_and(|o| o.complete() && o.len >= len)
            {
                newest_ms = Some(ms);
            }
        }
        if frame_ms.is_none() || (text.is_some() && newest_ms.is_none()) {
            missed += 1;
        }
        let end = Instant::now() + dwell;
        while Instant::now() < end {
            phone.step(Duration::from_millis(5))?;
        }
        let stats = phone.stats();
        println!(
            "{}",
            json!({
                "label": label,
                "device": phone.pairing().device,
                "request_at": request_at,
                "call_seq": seq,
                "http_at": http_at,
                "http_ms": http_ms,
                "target": text.map(|(id,_)| id),
                "frame_id": frame,
                "first_ms": first_ms,
                "reply_ms": reply_ms,
                "max_step_ms": max_step_ms,
                "uptime": String::from_utf8_lossy(&load.stdout).trim(),
                "packets_before": stats_before.received_packets,
                "packets_after": stats.received_packets,
                "undecodable": phone.undecodable,
                "frame_ms": frame_ms.map(|m| (m * 10.0).round() / 10.0),
                "newest_ms": newest_ms.map(|m| (m * 10.0).round() / 10.0),
                "newest_bytes": text.map(|(_, len)| len),
                "received_wire_bytes": stats.received_wire_bytes,
            })
        );
    }
    if missed > 0 {
        return Err(format!("{missed} threads did not arrive in time").into());
    }
    Ok(())
}
