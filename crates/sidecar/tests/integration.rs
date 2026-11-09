//! The sidecar against a throwaway stock t3 server, with native phones
//! over the real data channel.
//!
//! Each test starts the stock v0.1.7 server from `t3code/` with a fresh temp
//! `--base-dir` and the lab's scripted agent as its only provider, the
//! sidecar binary, and a UDP relay between phone and sidecar that can cut
//! the link. They need node with t3code's dependencies installed (`make
//! setup`) and Go (to build the lab's agent), so they are ignored by plain
//! `cargo test`; CI's lab-smoke job runs `sidecar_end_to_end`, and the long
//! outage tests run on the VM:
//!
//! ```sh
//! cargo test -p sidecar --test integration -- --ignored --exact sidecar_end_to_end
//! cargo test -p sidecar --test integration -- --ignored --nocapture outage_
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use native_phone::{NativePhone, Via, approve_command, new_id, pair, prompt_command, succeeded};
use serde_json::{Value, json};

type R<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// The lab agent's measured prompt (lab/internal/agent/reply.go).
const MEASURED_PROMPT: &str = "Please explain how the sync loop recovers after a long outage...";
const WARMUP_REPLY: &str = "The lab thread is set up";

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn log(msg: impl std::fmt::Display) {
    eprintln!("[test {:>6.1}s] {msg}", started().elapsed().as_secs_f64());
}

fn started() -> Instant {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    *START.get_or_init(Instant::now)
}

/// A temp dir, deleted when dropped.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> R<TempDir> {
        static N: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "t3opt-sidecar-{name}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir)?;
        Ok(TempDir(dir))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            eprintln!("could not remove {}: {e}", self.0.display());
        }
    }
}

/// A child process, killed when dropped.
struct Proc(Child);

impl Drop for Proc {
    fn drop(&mut self) {
        // Already gone is fine.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn free_tcp_port() -> R<u16> {
    Ok(TcpListener::bind("127.0.0.1:0")?.local_addr()?.port())
}

fn free_udp_port() -> R<u16> {
    Ok(UdpSocket::bind("127.0.0.1:0")?.local_addr()?.port())
}

/// The stock t3 server with the lab's agent.
struct T3 {
    _proc: Proc,
    base: PathBuf,
    work: PathBuf,
    home: PathBuf,
    url: String,
    _dir: TempDir,
}

impl T3 {
    fn start() -> R<T3> {
        let dir = TempDir::new("t3")?;
        let base = dir.0.join("base");
        let work = dir.0.join("workspace");
        let home = dir.0.join("home");
        for d in [&base.join("userdata"), &work, &home] {
            fs::create_dir_all(d)?;
        }
        fs::write(
            work.join("README.md"),
            "Workspace of the lab's fake agent.\n",
        )?;
        let lab = dir.0.join("lab");
        let status = Command::new("go")
            .args(["build", "-o"])
            .arg(&lab)
            .arg("./cmd/lab")
            .current_dir(repo().join("lab"))
            .status()?;
        if !status.success() {
            return Err("go build of the lab failed".into());
        }
        let settings = json!({
            "defaultRuntimeMode": "approval-required",
            "providerInstances": {
                "codex": {"driver": "codex", "enabled": false},
                "claudeAgent": {"driver": "claudeAgent", "enabled": false},
                "lab": {"driver": "acpRegistry", "displayName": "Lab agent", "config": {
                    "source": "local", "commandPath": lab,
                    "commandArgs": ["fakeagent", "--log", dir.0.join("agent.jsonl"), "--rate", "50", "--soak-parts", "0"]}}
            }
        });
        fs::write(base.join("userdata/settings.json"), settings.to_string())?;
        let port = free_tcp_port()?;
        let mut child = Command::new("node")
            .arg(repo().join("t3code/apps/server/src/bin.ts"))
            .args(["serve", "--base-dir"])
            .arg(&base)
            .args(["--port", &port.to_string(), "--host", "127.0.0.1"])
            .current_dir(&work)
            .env("HOME", &home)
            .env("T3CODE_NO_BROWSER", "true")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let stdout = child.stdout.take().ok_or("no stdout")?;
        let proc = Proc(child);
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if line.contains("T3 Code server is ready.") {
                    let _ = tx.send(());
                }
            }
        });
        rx.recv_timeout(Duration::from_secs(180))
            .map_err(|_| "t3 server not ready after 3 minutes")?;
        log(format!("t3 server on port {port}"));
        Ok(T3 {
            _proc: proc,
            base,
            work,
            home,
            url: format!("http://127.0.0.1:{port}"),
            _dir: dir,
        })
    }

    fn command(&self) -> String {
        format!(
            "node {}",
            repo().join("t3code/apps/server/src/bin.ts").display()
        )
    }

    fn cli(&self, args: &[&str]) -> R<String> {
        let out = Command::new("node")
            .arg(repo().join("t3code/apps/server/src/bin.ts"))
            .args(args)
            .current_dir(&self.work)
            .env("HOME", &self.home)
            .output()?;
        if !out.status.success() {
            return Err(format!("t3 {args:?}: {}", String::from_utf8_lossy(&out.stderr)).into());
        }
        Ok(String::from_utf8(out.stdout)?)
    }

    /// Adds the project and starts a thread with the warm-up turn.
    fn warm_thread(&self) -> R<String> {
        let base = self.base.to_str().ok_or("base dir")?;
        let work = self.work.to_str().ok_or("work dir")?;
        self.cli(&["project", "add", "--base-dir", base, work])?;
        let out = self.cli(&[
            "thread",
            "start",
            "--base-dir",
            base,
            "--provider",
            "lab",
            "--model",
            "scripted",
            "--title",
            "Lab thread",
            "--json",
            "--wait",
            work,
            "lab:warmup",
        ])?;
        let last = out.lines().last().ok_or("no thread start output")?;
        let v: Value = serde_json::from_str(last)?;
        Ok(v["threadId"].as_str().ok_or("no thread id")?.to_owned())
    }

    fn pairing_code(&self) -> R<String> {
        let out = self.cli(&["pair", "--base-dir", self.base.to_str().ok_or("base dir")?])?;
        Ok(out
            .lines()
            .find_map(|l| l.strip_prefix("Token: "))
            .ok_or("no pairing token")?
            .trim()
            .to_owned())
    }
}

struct Sidecar {
    _proc: Proc,
    origin: String,
    /// Its log lines so far.
    log: Arc<Mutex<Vec<String>>>,
}

impl Sidecar {
    fn start(t3: &T3, data: &Path, http: u16, udp: u16, web_dir: Option<&Path>) -> R<Sidecar> {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_sidecar"));
        cmd.args(["--t3-url", &t3.url, "--t3-base-dir"])
            .arg(&t3.base)
            .args(["--t3-command", &t3.command(), "--data-dir"])
            .arg(data)
            .args([
                "--http",
                &format!("127.0.0.1:{http}"),
                "--udp",
                &format!("127.0.0.1:{udp}"),
            ]);
        if let Some(dir) = web_dir {
            cmd.arg("--web-dir").arg(dir);
        }
        let mut child = cmd
            .env("HOME", &t3.home)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().ok_or("no stdout")?;
        let stderr = child.stderr.take().ok_or("no stderr")?;
        let log: Arc<Mutex<Vec<String>>> = Arc::default();
        let lines = log.clone();
        thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                eprintln!("{line}");
                if let Ok(mut l) = lines.lock() {
                    l.push(line);
                }
            }
        });
        let mut line = String::new();
        BufReader::new(stdout).read_line(&mut line)?;
        if !line.starts_with("sidecar ready") {
            return Err(format!("sidecar did not start: {line:?}").into());
        }
        Ok(Sidecar {
            _proc: Proc(child),
            origin: format!("http://127.0.0.1:{http}"),
            log,
        })
    }

    fn logged(&self, text: &str) -> bool {
        self.log
            .lock()
            .is_ok_and(|l| l.iter().any(|line| line.contains(text)))
    }
}

/// A UDP relay between phones and the sidecar, one back socket per phone
/// address, like a NAT. While `down`, it drops everything both ways.
struct Relay {
    addr: SocketAddr,
    down: Arc<AtomicBool>,
}

impl Relay {
    fn start(target: SocketAddr) -> R<Relay> {
        let front = Arc::new(UdpSocket::bind("127.0.0.1:0")?);
        let addr = front.local_addr()?;
        let down = Arc::new(AtomicBool::new(false));
        let backs: Arc<Mutex<HashMap<SocketAddr, Arc<UdpSocket>>>> = Arc::default();
        let (f, d) = (front.clone(), down.clone());
        thread::spawn(move || {
            let mut buf = [0u8; 2048];
            while let Ok((n, phone)) = f.recv_from(&mut buf) {
                if d.load(Ordering::Relaxed) {
                    continue;
                }
                let back = {
                    let Ok(mut backs) = backs.lock() else { return };
                    if let Some(b) = backs.get(&phone) {
                        b.clone()
                    } else {
                        let Ok(b) = UdpSocket::bind("127.0.0.1:0") else {
                            return;
                        };
                        let b = Arc::new(b);
                        backs.insert(phone, b.clone());
                        let (b2, f2, d2) = (b.clone(), f.clone(), d.clone());
                        thread::spawn(move || {
                            let mut buf = [0u8; 2048];
                            while let Ok(n) = b2.recv(&mut buf) {
                                if !d2.load(Ordering::Relaxed) {
                                    let _ = f2.send_to(&buf[..n], phone);
                                }
                            }
                        });
                        b
                    }
                };
                let _ = back.send_to(&buf[..n], target);
            }
        });
        Ok(Relay { addr, down })
    }

    fn set_down(&self, down: bool) {
        self.down.store(down, Ordering::Relaxed);
    }
}

/// Steps phones until `done` or the timeout.
fn run_until(
    what: &str,
    timeout: Duration,
    phones: &mut [&mut NativePhone],
    mut done: impl FnMut(&[&mut NativePhone]) -> bool,
) -> R {
    let deadline = Instant::now() + timeout;
    loop {
        for p in phones.iter_mut() {
            p.step(Duration::from_millis(5))?;
        }
        if done(phones) {
            log(format!("{what}: done"));
            return Ok(());
        }
        if Instant::now() > deadline {
            return Err(format!("{what}: not done within {timeout:?}").into());
        }
    }
}

/// Steps phones for a while.
fn run_for(time: Duration, phones: &mut [&mut NativePhone]) -> R {
    let end = Instant::now() + time;
    while Instant::now() < end {
        for p in phones.iter_mut() {
            p.step(Duration::from_millis(5))?;
        }
    }
    Ok(())
}

/// Runs a call or command until its reply.
fn wait_reply(what: &str, phone: &mut NativePhone, seq: u64, timeout: Duration) -> R<Value> {
    run_until(what, timeout, &mut [phone], |p| p[0].reply(seq).is_some())?;
    Ok(phone.reply(seq).cloned().unwrap_or(Value::Null))
}

fn assistant_texts(phone: &NativePhone, thread: &str) -> Vec<String> {
    phone
        .texts(thread, "assistant")
        .into_iter()
        .map(|(_, runs)| runs.concat())
        .collect()
}

/// Everything a test needs: t3, the sidecar, a relay, and a paired phone
/// with the lab thread open.
struct Setup {
    t3: T3,
    data: TempDir,
    http: u16,
    udp: u16,
    sidecar: Option<Sidecar>,
    relay: Relay,
    thread: String,
}

impl Setup {
    fn start() -> R<(Setup, NativePhone)> {
        let t3 = T3::start()?;
        let thread = t3.warm_thread()?;
        log(format!("thread {thread} warmed up"));
        let data = TempDir::new("sidecar-data")?;
        let (http, udp) = (free_tcp_port()?, free_udp_port()?);
        let sidecar = Sidecar::start(&t3, &data.0, http, udp, None)?;
        let relay = Relay::start(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), udp))?;
        let pairing = pair(&sidecar.origin, &t3.pairing_code()?)?;
        log(format!("paired as device {}", pairing.device));
        let mut phone = NativePhone::connect(
            &sidecar.origin,
            pairing,
            Via::Rtc {
                local: IpAddr::V4(Ipv4Addr::LOCALHOST),
                server: Some(relay.addr),
            },
        )?;
        let seq = phone.call("sidecar.focus", json!({"threadId": thread}));
        wait_reply("focus", &mut phone, seq, Duration::from_secs(30))?;
        let t = thread.clone();
        run_until(
            "warm-up reply",
            Duration::from_secs(60),
            &mut [&mut phone],
            |p| {
                assistant_texts(p[0], &t)
                    .iter()
                    .any(|s| s.contains(WARMUP_REPLY))
            },
        )?;
        Ok((
            Setup {
                t3,
                data,
                http,
                udp,
                sidecar: Some(sidecar),
                relay,
                thread,
            },
            phone,
        ))
    }

    fn origin(&self) -> String {
        format!("http://127.0.0.1:{}", self.http)
    }

    fn restart_sidecar(&mut self) -> R {
        self.sidecar = None;
        self.sidecar = Some(Sidecar::start(
            &self.t3,
            &self.data.0,
            self.http,
            self.udp,
            None,
        )?);
        Ok(())
    }
}

/// A call the sidecar answers at once, to see that a phone is connected.
fn ping(what: &str, phone: &mut NativePhone, thread: &str, timeout: Duration) -> R {
    let seq = phone.call("sidecar.focus", json!({"threadId": thread}));
    wait_reply(what, phone, seq, timeout).map(|_| ())
}

#[test]
#[ignore = "needs node with t3code's dependencies and Go; CI's lab-smoke job runs it"]
fn sidecar_end_to_end() -> R {
    started();
    let (mut setup, mut phone) = Setup::start()?;
    let thread = setup.thread.clone();

    // A prompt: confirmed, and the reply streams as t3 releases it: whole
    // paragraphs, as t3 holds an open one back.
    let warm: Vec<String> = assistant_texts(&phone, &thread);
    let seq = phone.command(&prompt_command(&thread, MEASURED_PROMPT)?);
    let reply = wait_reply("prompt confirmed", &mut phone, seq, Duration::from_secs(30))?;
    assert!(succeeded(&reply), "prompt failed: {reply}");
    let mut first = None;
    run_until(
        "first paragraph",
        Duration::from_secs(60),
        &mut [&mut phone],
        |p| {
            first = assistant_texts(p[0], &thread)
                .into_iter()
                .find(|t| !t.is_empty() && !warm.contains(t));
            first.is_some()
        },
    )?;
    let first = first.ok_or("no reply text")?;
    assert!(
        first.ends_with('\n'),
        "the first text is not a whole paragraph: {first:?}"
    );

    // The approval, answered from the phone.
    let mut request = None;
    run_until(
        "approval request",
        Duration::from_secs(120),
        &mut [&mut phone],
        |p| {
            request = p[0]
                .bodies(&thread, "runtimeRequests")
                .into_iter()
                .find(|r| r["status"] == "pending")
                .and_then(|r| r["id"].as_str().map(str::to_owned));
            request.is_some()
        },
    )?;
    let request = request.ok_or("no request")?;
    let seq = phone.command(&approve_command(&thread, &request)?);
    let reply = wait_reply(
        "approval answered",
        &mut phone,
        seq,
        Duration::from_secs(30),
    )?;
    assert!(succeeded(&reply), "approval failed: {reply}");
    run_until(
        "reply finished",
        Duration::from_secs(180),
        &mut [&mut phone],
        |p| {
            let runs = p[0].bodies(&thread, "runs");
            !runs.is_empty()
                && runs.iter().all(|r| r["status"] == "completed")
                && assistant_texts(p[0], &thread)
                    .iter()
                    .map(String::len)
                    .sum::<usize>()
                    > 1500
        },
    )?;

    // A new thread, created from the phone.
    let project = phone
        .objects()
        .find(|(m, _)| m["c"] == "project")
        .and_then(|(m, _)| m["i"].as_str().map(str::to_owned))
        .ok_or("no project in the thread list")?;
    let new_thread = new_id("thread")?;
    let create = json!({
        "type": "thread.create", "createdBy": "user", "creationSource": "mobile",
        "commandId": new_id("command")?, "threadId": new_thread, "projectId": project,
        "title": "Made on the phone", "modelSelection": {"instanceId": "lab", "model": "scripted"},
        "runtimeMode": "approval-required", "interactionMode": "default",
        "branch": null, "worktreePath": null,
    });
    let seq = phone.command(&create);
    let reply = wait_reply("thread created", &mut phone, seq, Duration::from_secs(30))?;
    assert!(succeeded(&reply), "thread.create failed: {reply}");
    run_until(
        "new thread listed",
        Duration::from_secs(30),
        &mut [&mut phone],
        |p| {
            p[0].objects()
                .any(|(m, o)| m["c"] == "thread" && m["i"] == new_thread.as_str() && o.complete())
        },
    )?;

    // A sidecar restart: the phone reconnects by itself and nothing is
    // synced again.
    run_for(Duration::from_secs(4), &mut [&mut phone])?;
    let before = phone.stats().received_wire_bytes;
    let generation = phone.generation();
    setup.restart_sidecar()?;
    log("sidecar restarted");
    run_until(
        "reconnected after restart",
        Duration::from_secs(60),
        &mut [&mut phone],
        |p| p[0].generation() > generation && p[0].is_open(),
    )?;
    ping(
        "call after restart",
        &mut phone,
        &thread,
        Duration::from_secs(30),
    )?;
    run_for(Duration::from_secs(5), &mut [&mut phone])?;
    let resent = phone.stats().received_wire_bytes - before;
    log(format!(
        "after the restart: {resent} bytes, {before} before it"
    ));
    assert!(
        resent < before / 4,
        "the restart synced {resent} bytes again; the phone had received {before} before it"
    );

    // A reloaded page while the old page's connection is still alive.
    let mut reloaded = phone.reload()?;
    let seq = reloaded.call("sidecar.focus", json!({"threadId": thread}));
    run_until(
        "reloaded page answered",
        Duration::from_secs(30),
        &mut [&mut phone, &mut reloaded],
        |p| p[1].reply(seq).is_some(),
    )?;
    assert!(reloaded.generation() > phone.generation());
    drop(phone);
    let mut phone = reloaded;

    // A delayed offer of an older page is refused.
    let p = phone.pairing().clone();
    let mut old_page = pipe::client::Client::new(
        "127.0.0.1:0".parse()?,
        pipe::generate_cert()?,
        pipe::client::Identity {
            token: p.token,
            device: p.device.clone(),
        },
        1,
    )?;
    let token: String = p.token.iter().map(|b| format!("{b:02x}")).collect();
    let stale = ureq::post(&format!("{}/api/rtc/offer", setup.origin()))
        .header("authorization", &format!("Bearer {token}"))
        .config()
        .http_status_as_error(false)
        .build()
        .send(old_page.offer()?)?;
    assert_eq!(stale.status().as_u16(), 400, "a stale offer was accepted");

    // A 10 s outage, recovered by an ICE restart on the same connection.
    let generation = phone.generation();
    setup.relay.set_down(true);
    log("link down");
    run_for(Duration::from_secs(10), &mut [&mut phone])?;
    setup.relay.set_down(false);
    log("link up");
    ping(
        "call after the outage",
        &mut phone,
        &thread,
        Duration::from_secs(60),
    )?;
    let r = phone.reconnects();
    log(format!("after the outage: {r:?}"));
    assert!(r.ice_restarts > 0, "no ICE restart");
    assert_eq!(
        phone.generation(),
        generation,
        "the outage needed a new connection"
    );
    Ok(())
}

/// The data channel features in Playwright WebKit: derived ICE
/// credentials, an ICE restart and a new connection without signaling, and
/// the WebSocket fallback. Screenshots land in crates/target/webkit-transport.
#[test]
#[ignore = "needs docker, node with t3code's dependencies and Go; CI's lab-smoke job runs it"]
fn webkit_transport() -> R {
    started();
    let t3 = T3::start()?;
    let data = TempDir::new("sidecar-data")?;
    let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/webkit");
    let sidecar = Sidecar::start(&t3, &data.0, free_tcp_port()?, free_udp_port()?, Some(&web))?;
    let pairing = pair(&sidecar.origin, &t3.pairing_code()?)?;
    let token: String = pairing.token.iter().map(|b| format!("{b:02x}")).collect();
    let image = "t3lab-phone:1.64.1";
    let have = Command::new("docker")
        .args(["image", "inspect", image])
        .output()?;
    if !have.status.success() {
        let built = Command::new("docker")
            .args(["build", "-t", image])
            .arg(repo().join("lab/driver"))
            .status()?;
        if !built.success() {
            return Err("docker build of the phone image failed".into());
        }
    }
    let shots = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/webkit-transport");
    fs::create_dir_all(&shots)?;
    let uid = String::from_utf8(Command::new("id").arg("-u").output()?.stdout)?;
    let gid = String::from_utf8(Command::new("id").arg("-g").output()?.stdout)?;
    let args = json!({"origin": sidecar.origin, "device": pairing.device, "token": token});
    let out = Command::new("docker")
        .args([
            "run",
            "--rm",
            "--name",
            &format!("t3lab-webkit-{}", std::process::id()),
        ])
        .args(["--network", "host", "--init", "--shm-size", "512m"])
        .args([
            "--user",
            &format!("{}:{}", uid.trim(), gid.trim()),
            "-e",
            "HOME=/tmp",
        ])
        .arg("-v")
        .arg(format!("{}:/x:ro", web.display()))
        .arg("-v")
        .arg(format!("{}:/out", shots.canonicalize()?.display()))
        .args([image, "node", "/x/transport.mjs", &args.to_string()])
        .stderr(Stdio::inherit())
        .output()?;
    let text = String::from_utf8_lossy(&out.stdout);
    log(format!("WebKit: {text}"));
    if !out.status.success() {
        return Err(format!("the WebKit driver failed: {}", out.status).into());
    }
    let results: Value = serde_json::from_str(text.trim())?;
    for step in ["signaled", "restart", "cold", "websocket"] {
        assert_eq!(results[step]["ok"], true, "{step}: {}", results[step]);
    }
    assert!(
        results["signaled"]["ufrag"]
            .as_str()
            .is_some_and(|u| u.ends_with("+p1+0"))
    );
    assert!(
        sidecar.logged("ICE restart 1 on generation 1"),
        "the sidecar saw no ICE restart"
    );
    assert!(
        sidecar.logged("generation 2 connects without signaling"),
        "the sidecar saw no cold connect"
    );
    assert!(
        sidecar.logged("now on Ws("),
        "the sidecar never used the WebSocket"
    );
    log(format!(
        "screenshots in {}",
        shots.canonicalize()?.display()
    ));
    Ok(())
}

fn outage(minutes: u64) -> R {
    started();
    let (setup, mut phone) = Setup::start()?;
    let generation = phone.generation();
    setup.relay.set_down(true);
    log(format!("link down for {minutes} minutes"));
    run_for(Duration::from_secs(minutes * 60), &mut [&mut phone])?;
    setup.relay.set_down(false);
    log("link up");
    let up = Instant::now();
    ping(
        "call after the outage",
        &mut phone,
        &setup.thread,
        Duration::from_secs(120),
    )?;
    let r = phone.reconnects();
    log(format!(
        "recovered {:.1} s after the link came back; {} ICE restarts, generation {} (was {generation})",
        up.elapsed().as_secs_f64(),
        r.ice_restarts,
        phone.generation()
    ));
    assert_eq!(
        phone.generation(),
        generation,
        "the outage needed a new connection"
    );
    // The thread still syncs: a prompt gets its reply.
    let seq = phone.command(&prompt_command(&setup.thread, "lab:warmup")?);
    let reply = wait_reply(
        "prompt after the outage",
        &mut phone,
        seq,
        Duration::from_secs(60),
    )?;
    assert!(succeeded(&reply), "prompt failed: {reply}");
    Ok(())
}

#[test]
#[ignore = "takes 5 minutes; run on the VM, output posted on issue #44"]
fn outage_5_minutes() -> R {
    outage(5)
}

#[test]
#[ignore = "takes 60 minutes; run on the VM, output posted on issue #44"]
fn outage_60_minutes() -> R {
    outage(60)
}
