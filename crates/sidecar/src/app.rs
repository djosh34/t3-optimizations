//! The main loop: one thread that owns the data channel socket, the world
//! and every phone's sync state. Other threads (t3 WebSocket, fetches,
//! HTTP, WebSocket fallback connections) talk to it through `Msg`.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use pipe::creds::{self, Side, Ufrag};
use pipe::{Direct, Pipe, PipeEvent};
use serde_json::{Value, json};
use sync_core::{Incoming, Millis, Server};

use crate::Msg;
use crate::config::Config;
use crate::device::{Device, Link};
use crate::http::{self, Api, ApiError, HttpConfig, WsEvent};
use crate::store::{DataDir, DeviceRecord, append_line, hex, new_device_id, parse_token, random};
use crate::t3::{self, Auth, HttpRequest, HttpResponse, In, Out, T3Config, percent_encode};
use crate::world::{After, World};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A result from the fetch thread.
#[derive(Debug)]
pub enum Fetched {
    Thread {
        thread: String,
        result: Result<Value, String>,
    },
    Image {
        id: String,
        mime: String,
        result: Result<Vec<u8>, String>,
    },
    /// The environment descriptor and the sidecar's session.
    Descriptor(Result<(Value, Value), String>),
    /// A phone's `sidecar.http` request.
    Http {
        device: String,
        seq: u64,
        result: Result<HttpResponse, String>,
    },
}

enum Job {
    Thread(String),
    Image {
        id: String,
        mime: String,
        url: String,
    },
    Descriptor,
    Http {
        device: String,
        seq: u64,
        req: HttpRequest,
    },
}

/// What a t3 request is for.
#[derive(Debug, Clone)]
enum Pending {
    Shell,
    Thread(String),
    Live,
    ConfigStream,
    Config,
    Call { device: String, seq: u64 },
    Command { device: String, seq: u64 },
    AssetUrl { id: String, mime: String },
}

/// A thread snapshot is fetched this long after its last change.
const REFETCH_DELAY: Duration = Duration::from_millis(500);
/// Sync state is saved this often while it changes.
const SAVE_EVERY: Duration = Duration::from_secs(2);
/// The loop wakes at least this often to take messages from other threads.
const TICK: Duration = Duration::from_millis(5);
const REPORTS_MAX: u64 = 4 << 20;
const STATS_EVERY: Duration = Duration::from_secs(10);

pub struct App {
    data: DataDir,
    pipe: Pipe,
    world: World,
    devices: HashMap<String, Device>,
    rx: Receiver<Msg>,
    t3: Sender<Out>,
    jobs: Sender<Job>,
    next_request: u64,
    pending: HashMap<u64, Pending>,
    subscribed: bool,
    /// Threads with a live subscription, by request.
    thread_subs: HashMap<String, u64>,
    /// Threads to fetch, and when.
    refetch: HashMap<String, Instant>,
    fetching: HashSet<String>,
    /// WebSocket fallback connections: where packets go, and the phone once
    /// it said hello.
    ws_out: HashMap<u64, Sender<Vec<u8>>>,
    ws_device: HashMap<u64, String>,
    start: Instant,
    last_save: Instant,
    last_stats: Instant,
    /// Packets SCTP dropped for a full send buffer since the last stats line.
    dropped: u64,
    /// A `server.getConfig` request is out, and the config changed again
    /// since it went out: it is fetched once more when the reply comes.
    config_pending: bool,
    config_again: bool,
    /// Objects phones kept from before a restart that the forked server no
    /// longer has were deleted.
    pruned: bool,
}

/// Runs the sidecar until a fatal error.
pub fn run(cfg: Config) -> Result<(), Error> {
    let data = DataDir::new(cfg.data_dir.clone())?;
    let cert = data.cert()?;
    let pipe = Pipe::bind(pipe::Config {
        bind: cfg.udp,
        advertise: cfg.udp_advertise,
        cert: Some(cert),
    })?;
    let (tx, rx) = mpsc::channel();
    let auth = Auth::new(T3Config {
        url: cfg.t3_url.clone(),
        base_dir: cfg.t3_base_dir.clone(),
        command: cfg.t3_command.clone(),
        token_file: data.token_file(),
    });
    let (t3_tx, t3_rx) = mpsc::channel();
    t3::spawn(auth.clone(), t3_rx, tx.clone());
    let (job_tx, job_rx) = mpsc::channel();
    spawn_fetcher(auth, job_rx, tx.clone());
    let http = http::spawn(
        HttpConfig {
            listen: cfg.http,
            web_dir: cfg.web_dir.clone(),
            t3_url: cfg.t3_url.clone(),
        },
        tx,
    )?;
    // Tests and the lab read the bound addresses from this line.
    println!("sidecar ready http={http} udp={}", pipe.host());
    let mut app = App {
        data,
        pipe,
        world: World::default(),
        devices: HashMap::new(),
        rx,
        t3: t3_tx,
        jobs: job_tx,
        next_request: 1,
        pending: HashMap::new(),
        subscribed: false,
        thread_subs: HashMap::new(),
        refetch: HashMap::new(),
        fetching: HashSet::new(),
        ws_out: HashMap::new(),
        ws_device: HashMap::new(),
        start: Instant::now(),
        last_save: Instant::now(),
        last_stats: Instant::now(),
        dropped: 0,
        config_pending: false,
        config_again: false,
        pruned: false,
    };
    app.load_devices()?;
    app.update_subscriptions();
    loop {
        app.step()?;
    }
}

/// Logs a line with the Unix time in milliseconds, so it lines up with the
/// lab's other logs.
pub(crate) fn log(msg: impl std::fmt::Display) {
    eprintln!("sidecar: {} {msg}", unix_ms());
}

impl App {
    fn now(&self) -> Millis {
        self.start.elapsed().as_millis() as Millis
    }

    fn load_devices(&mut self) -> Result<(), Error> {
        let now = self.now();
        for record in self.data.devices()? {
            let Some(token) = parse_token(&record.token) else {
                log(format!(
                    "device {}: bad token in its record, skipped",
                    record.id
                ));
                continue;
            };
            let server = match self.data.load_sync(&record.id)? {
                Some(state) => match Server::load(&state, sync_core::Config::server(), now) {
                    Ok(s) => s,
                    Err(e) => {
                        log(format!(
                            "device {}: sync state unreadable ({e:?}), full sync",
                            record.id
                        ));
                        Device::fresh_server(token, now)
                    }
                },
                None => Device::fresh_server(token, now),
            };
            log(format!("device {} loaded", record.id));
            self.devices
                .insert(record.id.clone(), Device::new(record, token, server));
        }
        Ok(())
    }

    fn step(&mut self) -> Result<(), Error> {
        let now = Instant::now();
        let now_ms = self.now();
        let mut wake = now + TICK;
        for d in self.devices.values().filter(|d| d.link.is_some()) {
            let at = d.server.next_wakeup(now_ms);
            wake = wake.min(self.start + Duration::from_millis(at));
        }
        let mut events = Vec::new();
        self.pipe.poll(wake, &mut events)?;
        for ev in events {
            self.pipe_event(ev)?;
        }
        while let Ok(msg) = self.rx.try_recv() {
            self.message(msg)?;
        }
        self.sync_world();
        self.start_fetches();
        self.send_packets()?;
        self.save_due()?;
        self.log_stats();
        Ok(())
    }

    /// Every 10 s, one line per connected phone: what the sync sees.
    fn log_stats(&mut self) {
        if self.last_stats.elapsed() < STATS_EVERY {
            return;
        }
        self.last_stats = Instant::now();
        let dropped = std::mem::take(&mut self.dropped);
        for d in self.devices.values().filter(|d| d.link.is_some()) {
            let s = d.server.stats();
            let queued = match d.link {
                Some(Link::Rtc(peer)) => self.pipe.buffered(peer).unwrap_or(0),
                _ => 0,
            };
            log(format!(
                "device {} stats: send {:.0} B/s, delivered {:.0} B/s, loss {:.0}%, srtt {} ms, unsynced {} B, backfill unsent {} B, in flight {} packets, sent {} received {} packets, SCTP queue {queued} B, {dropped} dropped by a full SCTP buffer",
                d.record.id,
                s.send_rate,
                s.delivered_rate,
                s.loss * 100.0,
                s.srtt_ms.map_or("-".into(), |r| format!("{r:.0}")),
                s.unsynced_bytes,
                s.unsent_backfill_bytes,
                s.inflight_packets,
                s.sent_packets,
                s.received_packets,
            ));
        }
    }

    // The phones' side.

    fn pipe_event(&mut self, ev: PipeEvent) -> Result<(), Error> {
        match ev {
            PipeEvent::Unknown { local_ufrag, .. } => self.unknown_stun(&local_ufrag)?,
            PipeEvent::Data(peer, data) => self.packet(Link::Rtc(peer), &data),
            PipeEvent::Open(peer) => log(format!("data channel {} open", peer.0)),
            PipeEvent::IceState(peer, state) => {
                log(format!("data channel {}: ICE {state}", peer.0))
            }
            PipeEvent::DtlsUp(peer) => log(format!("data channel {}: DTLS up", peer.0)),
            PipeEvent::Failed(peer, e) => log(format!("data channel {} failed: {e}", peer.0)),
            PipeEvent::Closed(peer) => {
                log(format!("data channel {} closed", peer.0));
                for d in self.devices.values_mut() {
                    d.peers.retain(|_, (p, _)| *p != peer);
                    if d.link == Some(Link::Rtc(peer)) {
                        d.link = None;
                    }
                }
            }
        }
        Ok(())
    }

    /// A STUN request no connection took: a phone's new connection or an
    /// ICE restart, both without signaling.
    fn unknown_stun(&mut self, local_ufrag: &str) -> Result<(), Error> {
        let Some(u) = Ufrag::parse(local_ufrag).filter(|u| u.side == Side::Server) else {
            return Ok(());
        };
        let Some(d) = self.devices.get_mut(&u.device) else {
            return Ok(());
        };
        let local = creds::derive(&d.token, &u.device, Side::Server, u.generation, u.restart);
        let remote = creds::derive(&d.token, &u.device, Side::Phone, u.generation, u.restart);
        // The fragment is public; only a request signed with the derived
        // password may restart ICE, make a connection or move the
        // generation.
        if !self.pipe.unknown_authentic(local_ufrag, &local.pass) {
            log(format!(
                "device {}: STUN request for {local_ufrag} failed its integrity check",
                u.device
            ));
            return Ok(());
        }
        if let Some(&(peer, restart)) = d.peers.get(&u.generation) {
            if u.restart > restart {
                self.pipe.restart_ice(peer, local, remote)?;
                d.peers.insert(u.generation, (peer, u.restart));
                log(format!(
                    "device {}: ICE restart {} on generation {}",
                    u.device, u.restart, u.generation
                ));
            }
            return Ok(());
        }
        // A newer generation is a new page; an older one is a page that a
        // newer one replaced. The current generation without a connection
        // is a phone whose connection this sidecar lost by restarting: it
        // gets a fresh one, so the phone's ICE connects, its old DTLS
        // session fails, and it makes a new connection at once.
        if u.generation < d.record.generation {
            return Ok(());
        }
        let Some(fingerprint) = d.record.fingerprint.as_deref().and_then(|f| f.parse().ok()) else {
            log(format!(
                "device {}: no certificate known yet, it must signal first",
                u.device
            ));
            return Ok(());
        };
        let peer = self.pipe.connect(Direct {
            local,
            remote,
            remote_fingerprint: Some(fingerprint),
        })?;
        d.peers.insert(u.generation, (peer, u.restart));
        d.record.generation = u.generation;
        self.data.save_device(&d.record)?;
        log(format!(
            "device {}: generation {} connects without signaling",
            u.device, u.generation
        ));
        Ok(())
    }

    fn offer(&mut self, token: [u8; 16], sdp: &str) -> Result<String, ApiError> {
        let d = self
            .devices
            .values_mut()
            .find(|d| d.token == token)
            .ok_or(ApiError::Unknown)?;
        let ufrag = sdp
            .lines()
            .find_map(|l| l.strip_prefix("a=ice-ufrag:"))
            .map(str::trim);
        let derived = ufrag
            .and_then(Ufrag::parse)
            .filter(|u| u.side == Side::Phone && u.device == d.record.id);
        let fingerprint = sdp
            .lines()
            .find_map(|l| l.strip_prefix("a=fingerprint:"))
            .map(|f| f.trim().to_owned());
        // A phone that could not set derived credentials still connects,
        // with random ones; it then needs signaling every time.
        let (generation, restart, local) = match derived {
            // A delayed offer of a page a newer one replaced.
            Some(u) if u.generation < d.record.generation => {
                return Err(ApiError::Bad(format!(
                    "generation {} is older than {}",
                    u.generation, d.record.generation
                )));
            }
            Some(u) => {
                let local = creds::derive(
                    &d.token,
                    &d.record.id,
                    Side::Server,
                    u.generation,
                    u.restart,
                );
                (u.generation, u.restart, Some(local))
            }
            None => (d.record.generation + 1, 0, None),
        };
        let (peer, answer) = self
            .pipe
            .accept_offer(sdp, local)
            .map_err(|e| ApiError::Bad(e.to_string()))?;
        // A phone whose answer was lost posts its offer again: the
        // connection the first one made is dead to it, and nothing else
        // would ever close it.
        if let Some((old, _)) = d.peers.insert(generation, (peer, restart))
            && old != peer
        {
            self.pipe.close(old);
        }
        d.record.generation = d.record.generation.max(generation);
        if fingerprint.is_some() {
            d.record.fingerprint = fingerprint;
        }
        self.data
            .save_device(&d.record)
            .map_err(|e| ApiError::Bad(e.to_string()))?;
        log(format!(
            "device {}: generation {generation} signaled",
            d.record.id
        ));
        Ok(answer)
    }

    /// A packet from a phone. A connection carries packets to the phone's
    /// sync state once it said hello; a new one then replaces the old.
    fn packet(&mut self, link: Link, data: &[u8]) {
        let device = match link {
            Link::Rtc(peer) => self
                .devices
                .iter()
                .find(|(_, d)| d.peers.values().any(|(p, _)| *p == peer))
                .map(|(id, _)| id.clone()),
            Link::Ws(id) => self.ws_device.get(&id).cloned().or_else(|| {
                let hello = Server::hello(data)?;
                let id_of = self
                    .devices
                    .iter()
                    .find(|(_, d)| d.token == hello.token)
                    .map(|(id, _)| id.clone())?;
                self.ws_device.insert(id, id_of.clone());
                Some(id_of)
            }),
        };
        let Some(device) = device else { return };
        let now = self.now();
        let Some(d) = self.devices.get_mut(&device) else {
            return;
        };
        if d.link != Some(link) {
            match Server::hello(data) {
                Some(h) if h.token == d.token => {}
                _ => return,
            }
            // A connection of an older page never takes over from a newer
            // one.
            let generation_of = |l: Option<Link>| match l {
                Some(Link::Rtc(peer)) => d
                    .peers
                    .iter()
                    .find(|(_, (p, _))| *p == peer)
                    .map(|(g, _)| *g),
                _ => None,
            };
            if let (Some(new), Some(current)) = (generation_of(Some(link)), generation_of(d.link))
                && new < current
            {
                return;
            }
            if let Some(old) = d.link {
                match old {
                    Link::Rtc(peer) => self.pipe.close(peer),
                    Link::Ws(id) => {
                        self.ws_out.remove(&id);
                    }
                }
            }
            // Older generations are pages this one replaced.
            if let Link::Rtc(peer) = link {
                let generation = d
                    .peers
                    .iter()
                    .find(|(_, (p, _))| *p == peer)
                    .map(|(g, _)| *g);
                if let Some(generation) = generation {
                    for (_, (p, _)) in d.peers.range(..generation) {
                        self.pipe.close(*p);
                    }
                }
            }
            log(format!("device {}: now on {link:?}", d.record.id));
            d.link = Some(link);
            d.server.connected();
        }
        let incoming = match d.server.receive(now, data) {
            Ok(i) => i,
            Err(e) => {
                log(format!("device {device}: bad packet: {e:?}"));
                return;
            }
        };
        d.dirty = true;
        for inc in incoming {
            self.incoming(&device, inc);
        }
    }

    fn incoming(&mut self, device: &str, inc: Incoming) {
        match inc {
            Incoming::Reset => log(format!(
                "device {device}: lost its cache, syncing everything"
            )),
            Incoming::Command { seq, payload } => match serde_json::from_slice::<Value>(&payload) {
                Ok(p) => self.request(
                    "orchestration.dispatchCommand",
                    p,
                    Pending::Command {
                        device: device.to_owned(),
                        seq,
                    },
                ),
                Err(e) => self.reply(device, seq, &json!({"error": format!("bad command: {e}")})),
            },
            Incoming::Call { seq, payload, .. } => {
                let call: Value = serde_json::from_slice(&payload).unwrap_or(Value::Null);
                let (Some(tag), args) = (call["tag"].as_str(), call["payload"].clone()) else {
                    self.reply(
                        device,
                        seq,
                        &json!({"error": "a call is {\"tag\", \"payload\"}"}),
                    );
                    return;
                };
                if tag == "sidecar.http" {
                    match http_request(&args) {
                        Ok(req) => {
                            let _ = self.jobs.send(Job::Http {
                                device: device.to_owned(),
                                seq,
                                req,
                            });
                        }
                        Err(e) => self.reply(device, seq, &json!({ "error": e })),
                    }
                    return;
                }
                if tag == "sidecar.focus" {
                    let thread = args["threadId"].as_str().map(str::to_owned);
                    if let Some(d) = self.devices.get_mut(device) {
                        d.set_focus(&self.world, thread);
                        if let Err(e) = self.data.save_device(&d.record) {
                            log(format!("device {device}: could not save: {e}"));
                        }
                    }
                    self.update_subscriptions();
                    self.reply(
                        device,
                        seq,
                        &json!({"exit": {"_tag": "Success", "value": null}}),
                    );
                    return;
                }
                self.request(
                    tag,
                    args,
                    Pending::Call {
                        device: device.to_owned(),
                        seq,
                    },
                );
            }
        }
    }

    fn reply(&mut self, device: &str, seq: u64, value: &Value) {
        if let Some(d) = self.devices.get_mut(device) {
            // The phone may have dropped the entry already.
            if d.server.reply(seq, value.to_string().into_bytes()).is_ok() {
                d.dirty = true;
            }
        }
    }

    fn send_packets(&mut self) -> Result<(), Error> {
        let now = self.now();
        for d in self.devices.values_mut() {
            let Some(link) = d.link else { continue };
            while let Some(packet) = d.server.poll(now) {
                match link {
                    Link::Rtc(peer) => match self.pipe.send(peer, &packet) {
                        Ok(true) => {}
                        // A full send buffer is loss, which sync-core handles.
                        Ok(false) => self.dropped += 1,
                        Err(pipe::Error::NoPeer(_)) => break,
                        Err(e) => return Err(e.into()),
                    },
                    Link::Ws(id) => {
                        let sent = self
                            .ws_out
                            .get(&id)
                            .is_some_and(|out| out.send(packet).is_ok());
                        if !sent {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn save_due(&mut self) -> Result<(), Error> {
        if self.last_save.elapsed() < SAVE_EVERY {
            return Ok(());
        }
        self.last_save = Instant::now();
        for d in self.devices.values_mut().filter(|d| d.dirty) {
            self.data.save_sync(&d.record.id, &d.server.save())?;
            d.dirty = false;
        }
        Ok(())
    }

    // Messages from other threads.

    fn message(&mut self, msg: Msg) -> Result<(), Error> {
        match msg {
            Msg::T3(i) => self.t3_in(i),
            Msg::Fetched(f) => self.fetched(f),
            Msg::Api(a) => self.api(a)?,
            Msg::Ws(WsEvent::Open { id, out }) => {
                self.ws_out.insert(id, out);
            }
            Msg::Ws(WsEvent::Data { id, data }) => self.packet(Link::Ws(id), &data),
            Msg::Ws(WsEvent::Closed { id }) => {
                self.ws_out.remove(&id);
                self.ws_device.remove(&id);
                for d in self.devices.values_mut() {
                    if d.link == Some(Link::Ws(id)) {
                        d.link = None;
                    }
                }
            }
        }
        Ok(())
    }

    fn api(&mut self, api: Api) -> Result<(), Error> {
        match api {
            Api::NewDevice { reply } => {
                let made = self.new_device().map_err(|e| e.to_string());
                // The HTTP thread may have given up waiting.
                let _ = reply.send(made);
            }
            Api::Offer { token, sdp, reply } => {
                let _ = reply.send(self.offer(token, &sdp));
            }
            Api::Report { token, body, reply } => {
                let result = match self.devices.values().find(|d| d.token == token) {
                    None => Err(ApiError::Unknown),
                    Some(d) => {
                        let report: Value = serde_json::from_slice(&body).unwrap_or_else(|_| {
                            Value::String(String::from_utf8_lossy(&body).into_owned())
                        });
                        let line = json!({"t": unix_ms(), "device": d.record.id, "report": report});
                        append_line(
                            &self.data.link_reports(),
                            line.to_string().as_bytes(),
                            REPORTS_MAX,
                        )
                        .map_err(|e| ApiError::Bad(e.to_string()))
                    }
                };
                let _ = reply.send(result);
            }
        }
        Ok(())
    }

    fn new_device(&mut self) -> Result<(String, [u8; 16]), Error> {
        let id = new_device_id()?;
        let token = random::<16>()?;
        let record = DeviceRecord {
            id: id.clone(),
            token: hex(&token),
            fingerprint: None,
            generation: 0,
            focus: None,
        };
        self.data.save_device(&record)?;
        let mut d = Device::new(record, token, Device::fresh_server(token, self.now()));
        d.sync(&self.world, self.world.ids().collect::<Vec<_>>());
        self.devices.insert(id.clone(), d);
        log(format!("device {id} paired"));
        Ok((id, token))
    }

    // The t3 server's side.

    fn request(&mut self, tag: &str, payload: Value, pending: Pending) {
        let id = self.next_request;
        self.next_request += 1;
        self.pending.insert(id, pending);
        // The t3 thread lives as long as this loop.
        let _ = self.t3.send(Out::Request {
            id,
            tag: tag.to_owned(),
            payload,
        });
    }

    fn t3_in(&mut self, i: In) {
        match i {
            In::Up => {
                log("connected to t3");
                if !self.subscribed {
                    self.subscribed = true;
                    self.request("orchestration.subscribeShell", json!({}), Pending::Shell);
                    self.request("orchestration.subscribeLiveText", json!({}), Pending::Live);
                    self.request(
                        "subscribeServerConfig",
                        json!({"workspaceSnapshotsOnDemand": true}),
                        Pending::ConfigStream,
                    );
                    let _ = self.jobs.send(Job::Descriptor);
                }
            }
            In::Down(why) => log(format!("t3 connection down: {why}")),
            In::Chunk { id, values } => self.chunk(id, values),
            In::Exit { id, exit } => self.exit(id, exit),
        }
    }

    fn chunk(&mut self, id: u64, values: Vec<Value>) {
        let Some(pending) = self.pending.get(&id).cloned() else {
            return;
        };
        match pending {
            Pending::Shell => {
                for v in &values {
                    if v["kind"].as_str() == Some("snapshot") {
                        self.world.set_shell(v);
                        let listed: Vec<String> = self.world.listed_threads().cloned().collect();
                        for t in listed {
                            self.mark_refetch(&t);
                        }
                    } else {
                        self.world.apply_shell(v);
                        if v["kind"].as_str() == Some("thread.updated")
                            && let Some(t) = v["thread"]["id"].as_str()
                        {
                            self.mark_refetch(t);
                        }
                    }
                }
            }
            Pending::Thread(t) => {
                let mut stale = false;
                for v in &values {
                    match v["kind"].as_str() {
                        Some("snapshot") => self.world.set_thread(&t, v),
                        Some("event") => stale |= self.world.apply_event(&t, v) == After::Refetch,
                        _ => {}
                    }
                }
                if stale {
                    // A new subscription starts with a fresh snapshot.
                    if let Some(old) = self.thread_subs.remove(&t) {
                        self.pending.remove(&old);
                        let _ = self.t3.send(Out::Interrupt(old));
                    }
                    self.update_subscriptions();
                }
            }
            Pending::Live => {
                for v in &values {
                    self.world.apply_live(v);
                }
            }
            Pending::Call { device, seq } => {
                // A stream call gets its first chunk as the reply.
                self.pending.remove(&id);
                let _ = self.t3.send(Out::Interrupt(id));
                self.reply(&device, seq, &json!({"chunk": values}));
            }
            // Any change of the config: fetch it whole again.
            Pending::ConfigStream => self.fetch_config(),
            Pending::Command { .. } | Pending::AssetUrl { .. } | Pending::Config => {}
        }
    }

    fn fetch_config(&mut self) {
        if self.config_pending {
            self.config_again = true;
        } else {
            self.config_pending = true;
            self.request("server.getConfig", json!({}), Pending::Config);
        }
    }

    fn exit(&mut self, id: u64, exit: Value) {
        let Some(pending) = self.pending.remove(&id) else {
            return;
        };
        let success = exit["_tag"].as_str() == Some("Success");
        match pending {
            Pending::Call { device, seq } | Pending::Command { device, seq } => {
                self.reply(&device, seq, &json!({"exit": exit}));
            }
            Pending::AssetUrl { id, mime } => match exit["value"]["relativeUrl"].as_str() {
                Some(url) if success => {
                    let _ = self.jobs.send(Job::Image {
                        id,
                        mime,
                        url: url.to_owned(),
                    });
                }
                _ => {
                    log(format!("image {id}: no URL: {exit}"));
                    self.world.image_failed(&id);
                }
            },
            Pending::Thread(t) => {
                log(format!("thread {t}: subscription ended: {exit}"));
                self.thread_subs.remove(&t);
                self.update_subscriptions();
            }
            Pending::Shell => {
                log(format!("shell subscription ended: {exit}"));
                self.request("orchestration.subscribeShell", json!({}), Pending::Shell);
            }
            Pending::Live => {
                log(format!("live text subscription ended: {exit}"));
                self.request("orchestration.subscribeLiveText", json!({}), Pending::Live);
            }
            Pending::Config => {
                self.config_pending = false;
                if success {
                    self.world.set_config(&exit["value"]);
                } else {
                    log(format!("server config: {exit}"));
                }
                if std::mem::take(&mut self.config_again) {
                    self.fetch_config();
                }
            }
            Pending::ConfigStream => {
                log(format!("config subscription ended: {exit}"));
                self.request(
                    "subscribeServerConfig",
                    json!({"workspaceSnapshotsOnDemand": true}),
                    Pending::ConfigStream,
                );
            }
        }
    }

    /// Subscribes to the threads phones have open, and only those; the
    /// others are fetched when the thread list says they changed.
    fn update_subscriptions(&mut self) {
        let wanted: BTreeSet<String> = self
            .devices
            .values()
            .filter_map(|d| d.focus.clone())
            .collect();
        let gone: Vec<String> = self
            .thread_subs
            .keys()
            .filter(|t| !wanted.contains(*t))
            .cloned()
            .collect();
        for t in gone {
            if let Some(id) = self.thread_subs.remove(&t) {
                self.pending.remove(&id);
                let _ = self.t3.send(Out::Interrupt(id));
                self.mark_refetch(&t);
            }
        }
        for t in wanted {
            if !self.thread_subs.contains_key(&t) {
                let id = self.next_request;
                self.request(
                    "orchestration.subscribeThread",
                    json!({"threadId": t}),
                    Pending::Thread(t.clone()),
                );
                self.thread_subs.insert(t, id);
            }
        }
    }

    fn mark_refetch(&mut self, thread: &str) {
        if self.thread_subs.contains_key(thread) {
            return;
        }
        self.refetch
            .entry(thread.to_owned())
            .or_insert_with(|| Instant::now() + REFETCH_DELAY);
    }

    fn start_fetches(&mut self) {
        let now = Instant::now();
        let due: Vec<String> = self
            .refetch
            .iter()
            .filter(|(t, at)| **at <= now && !self.fetching.contains(*t))
            .map(|(t, _)| t.clone())
            .collect();
        for t in due {
            self.refetch.remove(&t);
            self.fetching.insert(t.clone());
            let _ = self.jobs.send(Job::Thread(t));
        }
    }

    fn fetched(&mut self, f: Fetched) {
        match f {
            Fetched::Thread { thread, result } => {
                self.fetching.remove(&thread);
                match result {
                    // A live subscription is fresher than a fetch.
                    Ok(v) if !self.thread_subs.contains_key(&thread) => {
                        self.world.set_thread(&thread, &v)
                    }
                    Ok(_) => {}
                    Err(e) => log(format!("thread {thread}: fetch failed: {e}")),
                }
            }
            Fetched::Descriptor(result) => match result {
                Ok((descriptor, session)) => {
                    self.world.set_descriptor(&descriptor);
                    self.world.set_session(&session);
                }
                Err(e) => log(format!("environment descriptor: {e}")),
            },
            Fetched::Http {
                device,
                seq,
                result,
            } => {
                let reply = match result {
                    Ok(r) => json!({"http": {
                        "status": r.status,
                        "contentType": r.content_type,
                        "body": STANDARD.encode(&r.body),
                    }}),
                    Err(e) => json!({ "error": e }),
                };
                self.reply(&device, seq, &reply);
            }
            Fetched::Image { id, mime, result } => match result {
                Ok(bytes) => self.world.set_image(&id, &mime, bytes),
                Err(e) => {
                    log(format!("image {id}: fetch failed: {e}"));
                    self.world.image_failed(&id);
                }
            },
        }
    }

    /// Puts what changed in the world into every phone's sync state.
    fn sync_world(&mut self) {
        for (id, mime) in self.world.take_wanted_images() {
            self.request(
                "assets.createUrl",
                json!({"resource": {"_tag": "attachment", "attachmentId": id, "mimeType": mime, "disposition": "inline"}}),
                Pending::AssetUrl { id, mime },
            );
        }
        for meta in std::mem::take(&mut self.world.skipped) {
            log(format!("object left out, its meta is too long: {meta}"));
        }
        self.prune_once();
        let changed = self.world.take_changed();
        if changed.is_empty() {
            return;
        }
        for d in self.devices.values_mut() {
            d.sync(&self.world, changed.iter().copied());
        }
    }
}

impl App {
    /// Once the world has everything the forked server has, deletes what
    /// phones kept from before a sidecar restart that is no longer there,
    /// such as a thread deleted while the sidecar was down.
    fn prune_once(&mut self) {
        if self.pruned || !self.world.complete() {
            return;
        }
        self.pruned = true;
        let known = self.world.known_ids();
        for d in self.devices.values_mut() {
            let gone: Vec<u64> = d
                .server
                .object_ids()
                .into_iter()
                .filter(|id| !known.contains(id))
                .collect();
            if gone.is_empty() {
                continue;
            }
            log(format!(
                "device {}: {} objects the server no longer has, deleted",
                d.record.id,
                gone.len()
            ));
            for id in gone {
                d.server.delete(id);
            }
            d.dirty = true;
        }
    }
}

/// Reads a `sidecar.http` payload: `{"method", "path", "contentType",
/// "body"}`, the body in base64.
fn http_request(args: &Value) -> Result<HttpRequest, String> {
    let method = args["method"]
        .as_str()
        .unwrap_or("GET")
        .to_ascii_uppercase();
    if !matches!(method.as_str(), "GET" | "POST" | "PUT" | "DELETE" | "PATCH") {
        return Err(format!("method {method} is not relayed"));
    }
    let path = args["path"].as_str().ok_or("no path")?;
    // A path on the forked server, never another host.
    if !path.starts_with('/') || path.starts_with("//") || path.contains("://") {
        return Err(format!("{path} is not a path on the server"));
    }
    let body = match args["body"].as_str() {
        Some(b) => STANDARD
            .decode(b)
            .map_err(|e| format!("body is not base64: {e}"))?,
        None => Vec::new(),
    };
    Ok(HttpRequest {
        method,
        path: path.to_owned(),
        content_type: args["contentType"].as_str().map(str::to_owned),
        body,
    })
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

fn spawn_fetcher(auth: Arc<Auth>, jobs: Receiver<Job>, to_main: Sender<Msg>) {
    thread::spawn(move || {
        for job in jobs {
            let msg = match job {
                Job::Thread(thread) => {
                    let path = format!("/api/orchestration/threads/{}", percent_encode(&thread));
                    let result = auth
                        .get(&path)
                        .and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?))
                        .map_err(|e| e.to_string());
                    Fetched::Thread { thread, result }
                }
                Job::Image { id, mime, url } => {
                    let result = auth.get(&url).map_err(|e| e.to_string());
                    Fetched::Image { id, mime, result }
                }
                Job::Descriptor => {
                    let get = |path: &str| {
                        auth.get(path)
                            .and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?))
                    };
                    Fetched::Descriptor(
                        get("/.well-known/t3/environment")
                            .and_then(|d| Ok((d, get("/api/auth/session")?)))
                            .map_err(|e| e.to_string()),
                    )
                }
                // A phone's request may be slow (an upload on a bad link
                // took long to arrive, the reply takes long too); it gets
                // its own thread, so fetches do not wait for it.
                Job::Http { device, seq, req } => {
                    let (auth, to_main) = (auth.clone(), to_main.clone());
                    thread::spawn(move || {
                        let result = auth.request(&req).map_err(|e| e.to_string());
                        let _ = to_main.send(Msg::Fetched(Fetched::Http {
                            device,
                            seq,
                            result,
                        }));
                    });
                    continue;
                }
            };
            if to_main.send(Msg::Fetched(msg)).is_err() {
                return;
            }
        }
    });
}
