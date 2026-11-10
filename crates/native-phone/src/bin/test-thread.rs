//! Makes one of the tests' own threads through a sidecar, for the WebKit
//! checks of issue #84: a new thread titled `--title` (`t3opt test …`) in
//! the project `--project`, with GPT-6 Luna on pi at low thinking, and one
//! reply-only prompt. Waits until the reply is complete.
//!
//! ```sh
//! test-thread --origin http://127.0.0.1:12491 --pairing phone.json --project t3-optimizations \
//!   --title "t3opt test 84 safari" [--model openai-codex/gpt-6-luna]
//! ```
//!
//! stdout: one JSON line with the seconds it took.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use native_phone::{Error, NativePhone, Pairing, Via, new_id, prompt_command, succeeded};
use serde_json::{Value, json};

const PROMPT: &str = "Reply with the single word ready. Use no tools.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("test-thread: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let mut a = HashMap::new();
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let name = flag.strip_prefix("--").ok_or("flags only")?.to_owned();
        a.insert(name, it.next().ok_or("a flag needs a value")?);
    }
    let get = |k: &str| a.get(k).cloned().ok_or_else(|| format!("missing --{k}"));
    let v: Value = serde_json::from_str(&std::fs::read_to_string(get("pairing")?)?)?;
    let hex = v["token"].as_str().ok_or("pairing file: no token")?;
    let mut token = [0u8; 16];
    for (i, b) in token.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(2 * i..2 * i + 2).ok_or("bad token")?, 16)?;
    }
    let pairing = Pairing {
        device: v["device"]
            .as_str()
            .ok_or("pairing file: no device")?
            .to_owned(),
        token,
    };
    let model = a
        .get("model")
        .cloned()
        .unwrap_or_else(|| "openai-codex/gpt-6-luna".into());
    let started = Instant::now();
    let mut phone = NativePhone::connect(
        &get("origin")?,
        pairing,
        Via::Rtc {
            local: IpAddr::V4(Ipv4Addr::LOCALHOST),
            server: None,
        },
    )?;
    let project_title = get("project")?;
    let project = wait(&mut phone, Duration::from_secs(60), |p| {
        p.objects()
            .filter(|(m, o)| m["c"] == "project" && o.complete())
            .filter_map(|(_, o)| serde_json::from_slice::<Value>(&o.data).ok())
            .find(|p| p["title"] == project_title.as_str())
            .and_then(|p| p["id"].as_str().map(str::to_owned))
    })?;
    let thread = new_id("thread")?;
    let create = json!({
        "type": "thread.create", "createdBy": "user", "creationSource": "mobile",
        "commandId": new_id("command")?, "threadId": thread, "projectId": project,
        "title": get("title")?,
        "modelSelection": {"instanceId": "pi", "model": model, "options": [{"id": "thinking", "value": "low"}]},
        "runtimeMode": "approval-required", "interactionMode": "default",
        "branch": null, "worktreePath": null,
    });
    let seq = phone.command(&create);
    let reply = wait(&mut phone, Duration::from_secs(60), |p| {
        p.reply(seq).cloned()
    })?;
    if !succeeded(&reply) {
        return Err(format!("thread.create failed: {reply}").into());
    }
    phone.call("sidecar.focus", json!({"threadId": thread}));
    let seq = phone.command(&prompt_command(&thread, PROMPT)?);
    let reply = wait(&mut phone, Duration::from_secs(60), |p| {
        p.reply(seq).cloned()
    })?;
    if !succeeded(&reply) {
        return Err(format!("the prompt failed: {reply}").into());
    }
    let text = wait(&mut phone, Duration::from_secs(300), |p| {
        let done = p
            .bodies(&thread, "runs")
            .iter()
            .any(|r| r["status"] == "completed");
        let text: String = p
            .texts(&thread, "assistant")
            .into_iter()
            .map(|(_, runs)| runs.concat())
            .collect();
        (done && !text.is_empty()).then_some(text)
    })?;
    phone.call("sidecar.focus", json!({"threadId": null}));
    let end = Instant::now() + Duration::from_secs(2);
    while Instant::now() < end {
        phone.step(Duration::from_millis(5))?;
    }
    println!(
        "{}",
        json!({"seconds": started.elapsed().as_secs(), "reply_chars": text.chars().count()})
    );
    Ok(())
}

/// Steps the phone until `f` gives a value.
fn wait<T>(
    phone: &mut NativePhone,
    timeout: Duration,
    mut f: impl FnMut(&NativePhone) -> Option<T>,
) -> Result<T, Error> {
    let end = Instant::now() + timeout;
    loop {
        if let Some(v) = f(phone) {
            return Ok(v);
        }
        if Instant::now() > end {
            return Err("timed out".into());
        }
        phone.step(Duration::from_millis(5))?;
    }
}
