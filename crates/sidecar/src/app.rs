//! The main loop: one thread that owns the data channel socket, the world
//! and every phone's sync state. Other threads (t3 WebSocket, fetches,
//! HTTP, WebSocket fallback connections) talk to it through `Msg`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use pipe::creds::{self, Side, Ufrag};
use pipe::{Direct, Pipe, PipeEvent};
use serde_json::{Value, json};
use sync_core::{Incoming, Millis, Server, hash_id};

use crate::Msg;
use crate::config::Config;
use crate::device::{Device, Link};
use crate::http::{self, Api, ApiError, HttpConfig, WsEvent};
use crate::store::{DataDir, DeviceRecord, append_line, hex, new_device_id, parse_token, random};
use crate::t3::{self, Auth, HttpRequest, HttpResponse, In, Out, T3Config, percent_encode};
use crate::world::{After, World};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A result from the fetch threads.
#[derive(Debug)]
pub enum Fetched {
    /// A thread's bounded snapshot, or with `page` (its cursor), a page of
    /// its history.
    Thread {
        thread: String,
        page: Option<String>,
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
    /// `GET /api/orchestration/threads/:id/bounded`: the newest part.
    Bounded(String),
    /// `GET /api/orchestration/threads/:id/history`: the page at `cursor`.
    History {
        thread: String,
        cursor: String,
    },
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
    ConfigStream,
    Config,
    Call {
        device: String,
        seq: u64,
    },
    /// `kind` is the command's type, for the log.
    Command {
        device: String,
        seq: u64,
        kind: String,
    },
    AssetUrl {
        id: String,
        mime: String,
    },
}

/// One of the sidecar's subscriptions to t3.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Sub {
    Shell,
    ConfigStream,
    Thread(String),
}

impl Pending {
    fn sub(&self) -> Option<Sub> {
        match self {
            Pending::Shell => Some(Sub::Shell),
            Pending::ConfigStream => Some(Sub::ConfigStream),
            Pending::Thread(t) => Some(Sub::Thread(t.clone())),
            _ => None,
        }
    }
}

/// A subscription that ended starts again after this long, twice as long
/// each time it ends again without a value, up to `RETRY_MAX`.
const RETRY_MIN: Duration = Duration::from_secs(1);
const RETRY_MAX: Duration = Duration::from_secs(60);

/// Subscriptions that ended, failed or that t3 lacks: when each starts
/// again. They never start again at once, so a missing call is no tight loop.
#[derive(Default)]
struct Retries {
    due: HashMap<Sub, Instant>,
    /// The wait after a subscription's next end; a value resets it.
    wait: HashMap<Sub, Duration>,
}

impl Retries {
    /// Returns how long the subscription waits before it starts again.
    fn ended(&mut self, sub: Sub, now: Instant) -> Duration {
        let wait = self.wait.get(&sub).copied().unwrap_or(RETRY_MIN);
        self.wait.insert(sub.clone(), (wait * 2).min(RETRY_MAX));
        self.due.insert(sub, now + wait);
        wait
    }

    fn value(&mut self, sub: &Sub) {
        self.wait.remove(sub);
    }

    fn waiting(&self, sub: &Sub) -> bool {
        self.due.contains_key(sub)
    }

    fn take_due(&mut self, now: Instant) -> Vec<Sub> {
        let due: Vec<Sub> = self
            .due
            .iter()
            .filter(|(_, at)| **at <= now)
            .map(|(s, _)| s.clone())
            .collect();
        for s in &due {
            self.due.remove(s);
        }
        due
    }
}
/// Fetch priorities, lowest first: a thread a phone has open (and the
/// descriptor), images, then threads sent ahead of time.
const OPEN: u8 = 0;
const IMAGES: u8 = 1;
const PRELOAD: u8 = 2;
/// Fetches that run at once: an open thread never waits behind a preload.
const FETCHERS: usize = 3;
/// A phone with fewer unsynced bytes than this has an idle link, which
/// preloads use.
const IDLE_BYTES: u64 = 1 << 20;
/// How often a preload may start: finding one scans every phone's sync
/// state.
const PRELOAD_EVERY: Duration = Duration::from_millis(250);
/// A history page of an open thread whose fetch failed is fetched again
/// after this long.
const PAGE_RETRY: Duration = Duration::from_secs(5);
/// Sync state is saved this often while it changes.
const SAVE_EVERY: Duration = Duration::from_secs(2);
/// The loop wakes at least this often to take messages from other threads.
const TICK: Duration = Duration::from_millis(5);
const REPORTS_MAX: u64 = 4 << 20;
const STATS_EVERY: Duration = Duration::from_secs(10);
/// How many of the objects that took the most packet bytes the stats name.
const BUSIEST_LOGGED: usize = 5;

pub struct App {
    data: DataDir,
    pipe: Pipe,
    world: World,
    devices: HashMap<String, Device>,
    rx: Receiver<Msg>,
    t3: Sender<Out>,
    next_request: u64,
    pending: HashMap<u64, Pending>,
    subscribed: bool,
    /// Threads with a live subscription, by request.
    thread_subs: HashMap<String, u64>,
    /// Subscribed threads whose snapshot has not come yet, and since when.
    snapshot_pending: HashMap<String, Instant>,
    /// Threads with a history page being fetched.
    paging: HashSet<String>,
    /// Open threads' pages whose fetch failed: the cursor, and when to try
    /// again.
    page_retries: HashMap<String, (String, Instant)>,
    /// The thread being preloaded: one at a time.
    preloading: Option<String>,
    /// The thread list's `updatedAt` of each thread when the world last had
    /// it current; one that differs is fetched again before a preload.
    fresh: HashMap<String, String>,
    /// Threads whose preload failed, with their `updatedAt` then; tried
    /// again once it changes.
    failed: HashMap<String, String>,
    retries: Retries,
    /// The world's byte limit: past it, threads are evicted.
    cache_limit: usize,
    /// How many of the newest threads are preloaded.
    preload: usize,
    queue: Arc<Queue>,
    /// WebSocket fallback connections: where packets go, and the phone once
    /// it said hello.
    ws_out: HashMap<u64, Sender<Vec<u8>>>,
    ws_device: HashMap<u64, String>,
    start: Instant,
    last_save: Instant,
    last_stats: Instant,
    last_preload: Instant,
    /// Packets SCTP dropped for a full send buffer since the last stats line.
    dropped: u64,
    /// A `server.getConfig` request is out, and the config changed again
    /// since it went out: it is fetched once more when the reply comes.
    config_pending: bool,
    config_again: bool,
    /// Phones' objects the world no longer has were deleted once the
    /// thread list and the config were here.
    pruned: bool,
}

/// Fetch jobs for the fetch threads, most urgent first, in order within a
/// priority.
#[derive(Default)]
struct Queue {
    jobs: Mutex<Jobs>,
    ready: Condvar,
}

/// The jobs by priority and number, and the last number given.
#[derive(Default)]
struct Jobs {
    last: u64,
    waiting: BTreeMap<(u8, u64), Job>,
}

impl Queue {
    /// A poisoned lock only means a fetch thread died; the queue is fine.
    fn lock(&self) -> MutexGuard<'_, Jobs> {
        self.jobs.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn push(&self, priority: u8, job: Job) {
        let mut q = self.lock();
        q.last += 1;
        let n = q.last;
        q.waiting.insert((priority, n), job);
        self.ready.notify_one();
    }

    fn pop(&self) -> Job {
        let mut q = self.lock();
        loop {
            if let Some((_, job)) = q.waiting.pop_first() {
                return job;
            }
            q = self.ready.wait(q).unwrap_or_else(|e| e.into_inner());
        }
    }
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
    let queue = Arc::new(Queue::default());
    for _ in 0..FETCHERS {
        spawn_fetcher(auth.clone(), queue.clone(), tx.clone());
    }
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
    let mut app = App::new(data, pipe, rx, t3_tx, queue, &cfg);
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
    fn new(
        data: DataDir,
        pipe: Pipe,
        rx: Receiver<Msg>,
        t3: Sender<Out>,
        queue: Arc<Queue>,
        cfg: &Config,
    ) -> App {
        App {
            data,
            pipe,
            world: World::default(),
            devices: HashMap::new(),
            rx,
            t3,
            next_request: 1,
            pending: HashMap::new(),
            subscribed: false,
            thread_subs: HashMap::new(),
            snapshot_pending: HashMap::new(),
            paging: HashSet::new(),
            page_retries: HashMap::new(),
            preloading: None,
            fresh: HashMap::new(),
            failed: HashMap::new(),
            retries: Retries::default(),
            cache_limit: cfg.cache_mb << 20,
            preload: cfg.preload,
            queue,
            ws_out: HashMap::new(),
            ws_device: HashMap::new(),
            start: Instant::now(),
            last_save: Instant::now(),
            last_stats: Instant::now(),
            last_preload: Instant::now(),
            dropped: 0,
            config_pending: false,
            config_again: false,
            pruned: false,
        }
    }

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
        self.evict_due();
        self.resubscribe_due();
        self.preload_due();
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
                "device {} stats: send {:.0} B/s, delivered {:.0} B/s, loss {:.0}%, srtt {} ms, unsynced {} B, backfill unsent {} B, unacked {} packets, sent {} received {} packets, SCTP queue {queued} B, {dropped} dropped by a full SCTP buffer",
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
        for d in self.devices.values_mut().filter(|d| d.link.is_some()) {
            let busy = d.server.take_busiest(BUSIEST_LOGGED);
            if busy.is_empty() {
                continue;
            }
            let list: Vec<String> = busy
                .iter()
                .map(|b| {
                    format!(
                        "{} ({:?} order {}, sent {} B, acked {}/{} B, {} new revisions)",
                        String::from_utf8_lossy(&b.meta),
                        b.class,
                        b.order,
                        b.sent,
                        b.acked,
                        b.len,
                        b.revs
                    )
                })
                .collect();
            log(format!(
                "device {} sent most: {}",
                d.record.id,
                list.join("; ")
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
                self.update_subscriptions();
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
        let mut came_up = false;
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
            let was_connected = d.link.is_some();
            d.link = Some(link);
            d.server.connected();
            came_up = !was_connected;
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
        // Its open thread gets a live subscription again.
        if came_up {
            self.update_subscriptions();
        }
    }

    fn incoming(&mut self, device: &str, inc: Incoming) {
        match inc {
            Incoming::Reset => {
                log(format!(
                    "device {device}: lost its cache, syncing everything"
                ));
                // The server dropped what the phone had stored; the thread
                // list, the config and the open thread go again, and the
                // preload refills the rest.
                if let Some(d) = self.devices.get_mut(device) {
                    d.sync(&self.world, self.world.ids().collect::<Vec<_>>());
                }
            }
            Incoming::Command { seq, payload } => match serde_json::from_slice::<Value>(&payload) {
                Ok(p) => {
                    let kind = p["type"].as_str().unwrap_or_default().to_owned();
                    self.request(
                        "orchestration.dispatchCommand",
                        p,
                        Pending::Command {
                            device: device.to_owned(),
                            seq,
                            kind,
                        },
                    )
                }
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
                        Ok(req) => self.queue.push(
                            OPEN,
                            Job::Http {
                                device: device.to_owned(),
                                seq,
                                req,
                            },
                        ),
                        Err(e) => self.reply(device, seq, &json!({ "error": e })),
                    }
                    return;
                }
                if tag == "sidecar.focus" || tag == "sidecar.close" {
                    let thread = args["threadId"].as_str().map(str::to_owned);
                    if let Some(d) = self.devices.get_mut(device) {
                        // Closing a thread that is no longer the open one
                        // changes nothing.
                        let focus = match tag {
                            "sidecar.focus" => thread,
                            _ if d.focus == thread => None,
                            _ => d.focus.clone(),
                        };
                        if let Some(t) = &focus {
                            self.world.touch(t);
                        }
                        d.set_focus(&self.world, focus);
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
                self.update_subscriptions();
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
                    self.subscribe(Sub::Shell);
                    self.subscribe(Sub::ConfigStream);
                    self.queue.push(OPEN, Job::Descriptor);
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
        if let Some(sub) = pending.sub() {
            self.retries.value(&sub);
        }
        match pending {
            Pending::Shell => {
                let mut prune = false;
                for v in &values {
                    self.world.apply_shell(v);
                    match v["kind"].as_str() {
                        Some("snapshot") | Some("thread.removed") => prune = true,
                        // A subscribed thread is current through its events.
                        Some("thread.updated") => {
                            if let Some(t) = v["thread"]["id"].as_str()
                                && self.thread_subs.contains_key(t)
                                && let Some(l) = self.world.listed_thread(t)
                            {
                                self.fresh.insert(t.to_owned(), l.updated_at.clone());
                            }
                        }
                        _ => {}
                    }
                }
                if prune {
                    self.prune();
                }
            }
            Pending::Thread(t) => {
                let mut stale = false;
                for v in &values {
                    match v["kind"].as_str() {
                        Some("snapshot") => {
                            if let Some(at) = self.snapshot_pending.remove(&t) {
                                log(format!(
                                    "open thread: its snapshot came in {} ms, {} kB",
                                    at.elapsed().as_millis(),
                                    v.to_string().len() >> 10
                                ));
                            }
                            self.thread_snapshot(&t, v);
                        }
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
            Pending::Call { device, seq } => {
                self.reply(&device, seq, &json!({"exit": exit}));
            }
            Pending::Command { device, seq, kind } => {
                self.reply(&device, seq, &json!({"exit": exit}));
                // The reply goes out before any backfill not sent yet: the
                // lab's acceptance report reads this line (issue #46).
                if let Some(d) = self.devices.get(&device) {
                    log(format!(
                        "device {device}: command {seq} ({kind}) replied, backfill unsent {} B",
                        d.server.stats().unsent_backfill_bytes
                    ));
                }
            }
            Pending::AssetUrl { id, mime } => match exit["value"]["relativeUrl"].as_str() {
                Some(url) if success => self.queue.push(
                    IMAGES,
                    Job::Image {
                        id,
                        mime,
                        url: url.to_owned(),
                    },
                ),
                _ => {
                    log(format!("image {id}: no URL: {exit}"));
                    self.world.image_failed(&id);
                }
            },
            Pending::Thread(t) => {
                self.thread_subs.remove(&t);
                self.snapshot_pending.remove(&t);
                self.ended(Sub::Thread(t), &exit);
            }
            Pending::Shell => self.ended(Sub::Shell, &exit),
            Pending::ConfigStream => self.ended(Sub::ConfigStream, &exit),
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
        }
    }

    fn subscribe(&mut self, sub: Sub) {
        match sub {
            Sub::Shell => self.request("orchestration.subscribeShell", json!({}), Pending::Shell),
            Sub::ConfigStream => self.request(
                "subscribeServerConfig",
                json!({"workspaceSnapshotsOnDemand": true}),
                Pending::ConfigStream,
            ),
            // The snapshot is t3's bounded window, the newest part; the
            // history pages follow (`thread_snapshot`).
            Sub::Thread(t) => {
                let id = self.next_request;
                self.request(
                    "orchestration.subscribeThread",
                    json!({"threadId": t, "acceptBoundedSnapshot": true}),
                    Pending::Thread(t.clone()),
                );
                self.snapshot_pending.insert(t.clone(), Instant::now());
                self.thread_subs.insert(t, id);
            }
        }
    }

    fn ended(&mut self, sub: Sub, exit: &Value) {
        let wait = self.retries.ended(sub.clone(), Instant::now());
        log(format!(
            "{sub:?} subscription ended, again in {} s: {exit}",
            wait.as_secs()
        ));
    }

    fn resubscribe_due(&mut self) {
        for sub in self.retries.take_due(Instant::now()) {
            match sub {
                // Only while a phone still has the thread open.
                Sub::Thread(_) => self.update_subscriptions(),
                sub => self.subscribe(sub),
            }
        }
        let now = Instant::now();
        let due: Vec<(String, String)> = self
            .page_retries
            .iter()
            .filter(|(_, (_, at))| *at <= now)
            .map(|(t, (cursor, _))| (t.clone(), cursor.clone()))
            .collect();
        for (thread, cursor) in due {
            self.page_retries.remove(&thread);
            if self.thread_subs.contains_key(&thread) {
                self.fetch_page(&thread, &cursor);
            }
        }
    }

    /// Subscribes to the threads connected phones have open, and only
    /// those. A thread a phone left is current as far as the subscription
    /// brought it; it is not fetched again for that.
    fn update_subscriptions(&mut self) {
        let wanted: BTreeSet<String> = self
            .devices
            .values()
            .filter(|d| d.connected())
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
                self.snapshot_pending.remove(&t);
                self.page_retries.remove(&t);
                let _ = self.t3.send(Out::Interrupt(id));
            }
        }
        for t in wanted {
            let sub = Sub::Thread(t.clone());
            if !self.thread_subs.contains_key(&t) && !self.retries.waiting(&sub) {
                self.subscribe(sub);
            }
        }
    }

    /// A thread's newest part, from its subscription or a preload: then
    /// its history pages, newest to oldest, unless the world has them all.
    fn thread_snapshot(&mut self, thread: &str, snapshot: &Value) {
        self.world.set_thread(thread, snapshot);
        if let Some(l) = self.world.listed_thread(thread) {
            self.fresh.insert(thread.to_owned(), l.updated_at.clone());
        }
        let cursor = snapshot["historyCursor"].as_str();
        if snapshot["hasMoreHistory"].as_bool() == Some(true)
            && !self.world.history_complete(thread)
            && let Some(cursor) = cursor
        {
            self.fetch_page(thread, cursor);
        }
        self.preload_done(thread);
    }

    fn fetch_page(&mut self, thread: &str, cursor: &str) {
        if !self.paging.insert(thread.to_owned()) {
            return;
        }
        // A retry of an older load's page would add its rows twice.
        self.page_retries.remove(thread);
        let priority = if self.thread_subs.contains_key(thread) {
            OPEN
        } else {
            PRELOAD
        };
        self.queue.push(
            priority,
            Job::History {
                thread: thread.to_owned(),
                cursor: cursor.to_owned(),
            },
        );
    }

    /// Ends a preload once the thread's pages are all here.
    fn preload_done(&mut self, thread: &str) {
        if self.preloading.as_deref() == Some(thread) && !self.paging.contains(thread) {
            self.preloading = None;
        }
    }

    /// Preloads one thread when no open thread is loading: the newest
    /// listed threads that are not archived, newest first, for connected
    /// phones whose link is idle and that lack them. A thread changed since
    /// the world had it is fetched again first.
    fn preload_due(&mut self) {
        if self.preload == 0 || self.preloading.is_some() {
            return;
        }
        if self.last_preload.elapsed() < PRELOAD_EVERY {
            return;
        }
        self.last_preload = Instant::now();
        let opening = self
            .thread_subs
            .keys()
            .any(|t| self.snapshot_pending.contains_key(t) || self.paging.contains(t));
        if opening {
            return;
        }
        let idle: Vec<String> = self
            .devices
            .iter()
            .filter(|(_, d)| d.connected() && d.server.unsynced_bytes() < IDLE_BYTES)
            .map(|(id, _)| id.clone())
            .collect();
        if idle.is_empty() {
            return;
        }
        let mut newest: Vec<(&String, &String)> = self
            .world
            .listed()
            .filter(|(_, l)| l.location == "active")
            .map(|(t, l)| (t, &l.updated_at))
            .collect();
        newest.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let mut pick = None;
        for (t, updated) in newest.into_iter().take(self.preload) {
            if self.thread_subs.contains_key(t) || self.failed.get(t) == Some(updated) {
                continue;
            }
            let fresh = self.fresh.get(t) == Some(updated);
            let lacking: Vec<String> = idle
                .iter()
                .filter(|d| self.devices.get(*d).is_some_and(|d| !d.has_thread(t)))
                .cloned()
                .collect();
            let stale = !fresh
                && idle
                    .iter()
                    .any(|d| self.devices.get(d).is_some_and(|d| d.wants(t)));
            if lacking.is_empty() && !stale {
                continue;
            }
            pick = Some((t.clone(), fresh, lacking));
            break;
        }
        let Some((t, fresh, lacking)) = pick else {
            return;
        };
        if fresh && self.world.resident(&t) && self.world.history_complete(&t) {
            self.world.touch(&t);
            for id in lacking {
                if let Some(d) = self.devices.get_mut(&id) {
                    d.give(&self.world, &t);
                }
            }
            return;
        }
        self.preloading = Some(t.clone());
        self.queue.push(PRELOAD, Job::Bounded(t));
    }

    /// Past the byte limit, evicts the threads used least recently, never
    /// one that is open or loading. Phones keep what they have; what they
    /// have not stored of it waits until the thread comes back.
    fn evict_due(&mut self) {
        if self.world.bytes() <= self.cache_limit {
            return;
        }
        let keep: HashSet<&String> = self
            .thread_subs
            .keys()
            .chain(&self.paging)
            .chain(self.snapshot_pending.keys())
            .chain(&self.preloading)
            .collect();
        let gone = self
            .world
            .evict_over(self.cache_limit, |t| keep.contains(&t.to_owned()));
        if gone.is_empty() {
            return;
        }
        for d in self.devices.values_mut() {
            for (thread, ids) in &gone {
                d.release(thread, ids);
            }
            d.dirty = true;
        }
        log(format!(
            "evicted {} threads, the world holds {} MB",
            gone.len(),
            self.world.bytes() >> 20
        ));
    }

    fn fetched(&mut self, f: Fetched) {
        match f {
            Fetched::Thread {
                thread,
                page,
                result,
            } => self.fetched_thread(&thread, page, result),
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

    fn fetched_thread(
        &mut self,
        thread: &str,
        page: Option<String>,
        result: Result<Value, String>,
    ) {
        if page.is_some() {
            self.paging.remove(thread);
        }
        match result {
            Ok(v) if page.is_some() => {
                let more = self.world.add_history(thread, &v);
                if more && let Some(cursor) = v["nextCursor"].as_str() {
                    self.fetch_page(thread, cursor);
                }
            }
            // A live subscription is fresher than a fetch.
            Ok(_) if self.thread_subs.contains_key(thread) => {}
            Ok(v) => {
                self.thread_snapshot(thread, &v);
                // Phones it was preloaded for get all of it from now on,
                // also those whose copy an eviction left unfinished.
                for d in self.devices.values_mut() {
                    if d.connected() && !d.has_thread(thread) {
                        d.give(&self.world, thread);
                    }
                }
            }
            Err(e) => {
                log(format!("thread {thread}: fetch failed: {e}"));
                if self.preloading.as_deref() == Some(thread)
                    && let Some(l) = self.world.listed_thread(thread)
                {
                    self.failed.insert(thread.to_owned(), l.updated_at.clone());
                }
                self.paging.remove(thread);
                if self.preloading.as_deref() == Some(thread) {
                    self.preloading = None;
                }
                if let Some(cursor) = page
                    && self.thread_subs.contains_key(thread)
                {
                    self.page_retries
                        .insert(thread.to_owned(), (cursor, Instant::now() + PAGE_RETRY));
                }
            }
        }
        self.preload_done(thread);
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
        if !self.pruned && self.world.config_loaded() {
            self.prune();
        }
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
    /// Deletes on phones the objects of threads that left the thread list,
    /// and, once the config is here, other objects the world no longer has
    /// (a project removed while the sidecar was down). Evicted threads stay
    /// on phones: they are still listed. Images go with the thread their
    /// meta names (`o`).
    fn prune(&mut self) {
        if !self.world.shell_loaded() {
            return;
        }
        self.pruned |= self.world.config_loaded();
        let listed: HashSet<String> = self.world.listed().map(|(t, _)| hash_id(t)).collect();
        for d in self.devices.values_mut() {
            let gone: Vec<u64> = d
                .server
                .metas()
                .filter(|&(id, meta)| {
                    let Ok(m) = serde_json::from_slice::<Value>(meta) else {
                        return false;
                    };
                    // An image another thread still shows is in the world.
                    let thread = match m["c"].as_str() {
                        Some("thread") => m["i"].as_str(),
                        Some("image") if self.world.get(id).is_some() => return false,
                        Some("image") => m["o"].as_str(),
                        _ => m["t"].as_str(),
                    };
                    match thread {
                        Some(h) => !listed.contains(h),
                        None => self.pruned && m["c"] != "image" && self.world.get(id).is_none(),
                    }
                })
                .map(|(id, _)| id)
                .collect();
            let before = d.server.object_ids().len();
            for id in gone {
                d.server.delete(id);
            }
            let deleted = before - d.server.object_ids().len();
            if deleted > 0 {
                log(format!(
                    "device {}: {deleted} objects the server no longer has, deleted",
                    d.record.id
                ));
                d.dirty = true;
            }
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
    // A path on the t3 server, never another host.
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

fn spawn_fetcher(auth: Arc<Auth>, queue: Arc<Queue>, to_main: Sender<Msg>) {
    thread::spawn(move || {
        loop {
            let get_json = |path: &str| {
                auth.get(path)
                    .and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?))
                    .map_err(|e| e.to_string())
            };
            let msg = match queue.pop() {
                Job::Bounded(thread) => {
                    let path = format!(
                        "/api/orchestration/threads/{}/bounded",
                        percent_encode(&thread)
                    );
                    Fetched::Thread {
                        result: get_json(&path),
                        thread,
                        page: None,
                    }
                }
                Job::History { thread, cursor } => {
                    let path = format!(
                        "/api/orchestration/threads/{}/history?cursor={}",
                        percent_encode(&thread),
                        percent_encode(&cursor)
                    );
                    Fetched::Thread {
                        result: get_json(&path),
                        thread,
                        page: Some(cursor),
                    }
                }
                Job::Image { id, mime, url } => {
                    let result = auth.get(&url).map_err(|e| e.to_string());
                    Fetched::Image { id, mime, result }
                }
                Job::Descriptor => Fetched::Descriptor(
                    get_json("/.well-known/t3/environment")
                        .and_then(|d| Ok((d, get_json("/api/auth/session")?))),
                ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ended_subscription_waits_longer_each_time_until_a_value() {
        let mut r = Retries::default();
        let t0 = Instant::now();
        let waits: Vec<u64> = (0..8).map(|_| r.ended(Sub::Shell, t0).as_secs()).collect();
        assert_eq!(waits, [1, 2, 4, 8, 16, 32, 60, 60]);
        assert!(r.waiting(&Sub::Shell));
        assert!(r.take_due(t0 + Duration::from_secs(59)).is_empty());
        assert_eq!(r.take_due(t0 + Duration::from_secs(60)), [Sub::Shell]);
        assert!(!r.waiting(&Sub::Shell));
        r.value(&Sub::Shell);
        assert_eq!(r.ended(Sub::Shell, t0), RETRY_MIN);
        // Each subscription has its own wait.
        assert_eq!(r.ended(Sub::Thread("t1".into()), t0), RETRY_MIN);
    }

    // The main loop's decisions, with phones on WebSocket links that carry
    // nothing and a t3 server that is only a channel.

    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::world::frame_id;

    struct Test {
        app: App,
        out: Receiver<Out>,
        dir: std::path::PathBuf,
    }

    impl Drop for Test {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn start(preload: usize) -> Result<Test, Error> {
        static N: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!(
            "sidecar-app-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let data = DataDir::new(dir.clone())?;
        let pipe = Pipe::bind(pipe::Config {
            bind: "127.0.0.1:0".parse()?,
            advertise: None,
            cert: Some(data.cert()?),
        })?;
        let (_, rx) = mpsc::channel();
        let (t3, out) = mpsc::channel();
        let cfg = Config {
            t3_url: "http://127.0.0.1:1".into(),
            t3_base_dir: dir.clone(),
            t3_command: vec!["t3".into()],
            data_dir: dir.clone(),
            http: "127.0.0.1:0".parse()?,
            udp: "127.0.0.1:0".parse()?,
            udp_advertise: None,
            web_dir: None,
            cache_mb: 256,
            preload,
        };
        let mut app = App::new(data, pipe, rx, t3, Arc::new(Queue::default()), &cfg);
        app.t3_in(In::Up);
        let mut t = Test { app, out, dir };
        t.requests();
        Ok(t)
    }

    impl Test {
        fn preload(&mut self) {
            self.app.last_preload = Instant::now() - PRELOAD_EVERY;
            self.app.preload_due();
        }

        fn phone(&mut self, id: &str, focus: Option<&str>, connected: bool) {
            let n = self.app.devices.len() as u8 + 1;
            let record = DeviceRecord {
                id: id.to_owned(),
                token: hex(&[n; 16]),
                fingerprint: None,
                generation: 0,
                focus: focus.map(str::to_owned),
            };
            let mut d = Device::new(record, [n; 16], Device::fresh_server([n; 16], 0));
            if connected {
                d.link = Some(Link::Ws(n.into()));
            }
            self.app.devices.insert(id.to_owned(), d);
        }

        fn link(&mut self, id: &str, up: bool) {
            if let Some(d) = self.app.devices.get_mut(id) {
                d.link = up.then_some(Link::Ws(99));
            }
            self.app.update_subscriptions();
        }

        fn call(&mut self, device: &str, tag: &str, thread: Option<&str>) {
            let payload = json!({"tag": tag, "payload": {"threadId": thread}});
            self.app.incoming(
                device,
                Incoming::Call {
                    seq: 1,
                    payload: payload.to_string().into_bytes(),
                    background: false,
                },
            );
        }

        /// The thread list, as `(id, location, updatedAt)`.
        fn shell(&mut self, threads: &[(&str, &str, &str)]) {
            let row = |t: &(&str, &str, &str)| json!({"id": t.0, "updatedAt": t.2});
            let active: Vec<Value> = threads
                .iter()
                .filter(|t| t.1 == "active")
                .map(row)
                .collect();
            let archive: Vec<Value> = threads
                .iter()
                .filter(|t| t.1 != "active")
                .map(row)
                .collect();
            let id = self.pending_id(|p| matches!(p, Pending::Shell));
            self.app.chunk(
                id,
                vec![json!({"kind": "snapshot", "snapshot": {"schemaVersion": 2, "snapshotSequence": 1,
                    "projects": [], "threads": active, "archivedThreads": archive}})],
            );
            self.app.sync_world();
        }

        fn thread_updated(&mut self, id: &str, updated: &str) {
            let sub = self.pending_id(|p| matches!(p, Pending::Shell));
            self.app.chunk(
                sub,
                vec![json!({"kind": "thread.updated", "location": "active",
                    "thread": {"id": id, "updatedAt": updated}})],
            );
        }

        fn pending_id(&self, f: impl Fn(&Pending) -> bool) -> u64 {
            self.app
                .pending
                .iter()
                .find(|(_, p)| f(p))
                .map_or(0, |(id, _)| *id)
        }

        /// A thread's snapshot from its subscription.
        fn subscription_snapshot(&mut self, thread: &str, mut snapshot: Value) {
            snapshot["kind"] = json!("snapshot");
            let id = self.pending_id(|p| matches!(p, Pending::Thread(t) if t == thread));
            assert!(id > 0, "no subscription to {thread}");
            self.app.chunk(id, vec![snapshot]);
            self.app.sync_world();
        }

        fn fetched(&mut self, thread: &str, page: bool, v: Value) {
            self.app.fetched(Fetched::Thread {
                thread: thread.to_owned(),
                page: page.then(|| "c1".to_owned()),
                result: Ok(v),
            });
            self.app.sync_world();
        }

        /// Jobs waiting, most urgent first, without taking them.
        fn queued(&self) -> Vec<String> {
            self.app
                .queue
                .lock()
                .waiting
                .iter()
                .map(|((p, _), j)| match j {
                    Job::Bounded(t) => format!("{p} bounded {t}"),
                    Job::History { thread, cursor } => format!("{p} history {thread} {cursor}"),
                    Job::Image { id, .. } => format!("{p} image {id}"),
                    Job::Descriptor => format!("{p} descriptor"),
                    Job::Http { .. } => format!("{p} http"),
                })
                .collect()
        }

        fn take_queued(&mut self) -> Vec<String> {
            let q = self.queued();
            self.app.queue.lock().waiting.clear();
            q
        }

        /// Requests sent to t3 since the last call: subscriptions with their
        /// payload, and interrupts.
        fn requests(&mut self) -> Vec<String> {
            self.out
                .try_iter()
                .map(|o| match o {
                    Out::Request { tag, payload, .. } => format!("{tag} {payload}"),
                    Out::Interrupt(_) => "interrupt".to_owned(),
                })
                .collect()
        }
    }

    fn snapshot(thread: &str, text: &str, more: bool) -> Value {
        json!({"snapshotSequence": 1, "hasMoreHistory": more, "historyCursor": if more { json!("c1") } else { Value::Null },
            "projection": {"thread": {"id": thread}, "visibleTurnItems": [], "updatedAt": "1",
                "turnItems": [{"id": "i1", "type": "assistant_message", "messageId": "m1", "ordinal": 1, "text": text}]}})
    }

    #[test]
    fn subscriptions_follow_connected_phones_and_their_open_threads() -> Result<(), Error> {
        let mut t = start(0)?;
        t.phone("a", Some("t1"), true);
        t.phone("b", Some("t2"), false);
        t.app.update_subscriptions();
        assert_eq!(
            t.requests(),
            [r#"orchestration.subscribeThread {"acceptBoundedSnapshot":true,"threadId":"t1"}"#]
        );
        t.link("b", true);
        assert_eq!(
            t.requests(),
            [r#"orchestration.subscribeThread {"acceptBoundedSnapshot":true,"threadId":"t2"}"#]
        );
        // A closes its thread, B goes away: both subscriptions end, and
        // neither thread is fetched again for that.
        t.call("a", "sidecar.close", Some("t1"));
        t.link("b", false);
        assert_eq!(t.requests(), ["interrupt", "interrupt"]);
        assert!(t.app.thread_subs.is_empty());
        assert_eq!(t.queued(), ["0 descriptor"]);
        // Closing a thread that is not the open one changes nothing.
        t.call("a", "sidecar.focus", Some("t3"));
        t.call("a", "sidecar.close", Some("t1"));
        assert_eq!(t.app.devices["a"].focus.as_deref(), Some("t3"));
        // B is back: its thread is subscribed again.
        t.link("b", true);
        assert_eq!(t.app.thread_subs.len(), 2);
        Ok(())
    }

    #[test]
    fn the_newest_live_threads_are_preloaded_one_at_a_time() -> Result<(), Error> {
        let mut t = start(2)?;
        t.take_queued();
        t.phone("a", None, true);
        t.shell(&[
            ("t1", "active", "2026-11-10T03"),
            ("t2", "active", "2026-11-10T02"),
            ("t3", "archive", "2026-11-10T04"),
            ("t4", "active", "2026-11-10T01"),
        ]);
        // The thread list alone fetches no thread.
        assert!(t.queued().is_empty());
        t.preload();
        t.preload();
        // The newest live one, archived ones never; one at a time.
        assert_eq!(t.take_queued(), ["2 bounded t1"]);
        t.fetched("t1", false, snapshot("t1", "one", true));
        assert!(t.app.devices["a"].wants("t1"), "the phone got t1");
        // Its older history next, then the next thread.
        assert_eq!(t.take_queued(), ["2 history t1 c1"]);
        t.preload();
        assert!(t.queued().is_empty());
        t.fetched("t1", true, json!({"items": [], "hasMoreHistory": false}));
        t.preload();
        assert_eq!(t.take_queued(), ["2 bounded t2"]);
        t.fetched("t2", false, snapshot("t2", "two", false));
        t.preload();
        // Only the newest 2.
        assert!(t.queued().is_empty());
        Ok(())
    }

    #[test]
    fn an_opened_thread_overtakes_a_preload() -> Result<(), Error> {
        let mut t = start(2)?;
        t.take_queued();
        t.phone("a", None, true);
        t.shell(&[
            ("t1", "active", "2"),
            ("t2", "active", "1"),
            ("old", "archive", "0"),
        ]);
        t.preload();
        assert_eq!(t.queued(), ["2 bounded t1"]);
        // The phone opens an archived thread while t1 preloads.
        t.call("a", "sidecar.focus", Some("old"));
        t.requests();
        t.subscription_snapshot("old", snapshot("old", "old text", true));
        // Its history goes before the preload.
        assert_eq!(t.queued(), ["0 history old c1", "2 bounded t1"]);
        assert!(matches!(t.app.queue.pop(), Job::History { thread, .. } if thread == "old"));
        t.take_queued();
        // No new preload while the opened thread loads.
        t.fetched("t1", false, snapshot("t1", "one", false));
        t.preload();
        assert!(t.queued().is_empty());
        // A failed page of the open thread is fetched again a little later.
        t.app.fetched(Fetched::Thread {
            thread: "old".into(),
            page: Some("c1".into()),
            result: Err("t3 was busy".into()),
        });
        t.app.resubscribe_due();
        assert!(t.queued().is_empty());
        if let Some((_, at)) = t.app.page_retries.get_mut("old") {
            *at = Instant::now();
        }
        t.app.resubscribe_due();
        assert_eq!(t.take_queued(), ["0 history old c1"]);
        // It fails again, and the phone closes and reopens the thread before
        // the retry: the new load's pages replace it, and nothing is
        // fetched twice.
        t.app.fetched(Fetched::Thread {
            thread: "old".into(),
            page: Some("c1".into()),
            result: Err("t3 was busy".into()),
        });
        t.call("a", "sidecar.close", Some("old"));
        assert!(t.app.page_retries.is_empty());
        t.call("a", "sidecar.focus", Some("old"));
        t.subscription_snapshot("old", snapshot("old", "old text", true));
        assert_eq!(t.take_queued(), ["0 history old c1"]);
        t.fetched("old", true, json!({"items": [], "hasMoreHistory": false}));
        t.app.resubscribe_due();
        assert!(t.queued().is_empty());
        t.preload();
        assert_eq!(t.queued(), ["2 bounded t2"]);
        Ok(())
    }

    #[test]
    fn a_thread_is_fetched_again_only_when_it_changed() -> Result<(), Error> {
        let mut t = start(1)?;
        t.take_queued();
        t.phone("a", None, true);
        t.shell(&[("t1", "active", "1")]);
        t.preload();
        assert_eq!(t.take_queued(), ["2 bounded t1"]);
        t.fetched("t1", false, snapshot("t1", "one", false));
        // The same list again, as after a reconnect to t3: nothing.
        t.shell(&[("t1", "active", "1")]);
        t.preload();
        assert!(t.queued().is_empty());
        // Changed: fetched again, as the phone has it.
        t.thread_updated("t1", "2");
        t.preload();
        assert_eq!(t.take_queued(), ["2 bounded t1"]);
        Ok(())
    }

    #[test]
    fn eviction_keeps_threads_on_phones_and_cleanup_follows_the_thread_list() -> Result<(), Error> {
        let mut t = start(2)?;
        t.take_queued();
        t.phone("a", Some("t1"), true);
        t.app.update_subscriptions();
        t.shell(&[("t1", "active", "2"), ("t2", "active", "1")]);
        t.subscription_snapshot("t1", snapshot("t1", "one", false));
        t.preload();
        t.fetched("t2", false, snapshot("t2", "two", false));
        let has = |t: &Test, thread: &str| {
            t.app.devices["a"]
                .server
                .object_ids()
                .contains(&frame_id(thread))
        };
        assert!(has(&t, "t1") && has(&t, "t2"));
        // Over the limit: t2 goes, t1 is open and stays.
        let unsynced = |t: &Test| t.app.devices["a"].server.stats().unsynced_bytes;
        let before = unsynced(&t);
        t.app.cache_limit = 0;
        t.app.evict_due();
        assert!(t.app.world.resident("t1") && !t.app.world.resident("t2"));
        // The phone keeps t2, and what it has not stored of it waits.
        assert!(has(&t, "t2"));
        assert!(unsynced(&t) < before);
        t.shell(&[("t1", "active", "2"), ("t2", "active", "1")]);
        assert!(has(&t, "t2"), "an evicted thread is not deleted");
        // The phone never stored t2 whole: with room again, the preload
        // sends it once more.
        t.app.cache_limit = 256 << 20;
        t.take_queued();
        t.preload();
        assert_eq!(t.take_queued(), ["2 bounded t2"]);
        t.fetched("t2", false, snapshot("t2", "two", false));
        // t2 leaves the list: deleted on the phone, though the world does
        // not hold it.
        t.shell(&[("t1", "active", "2")]);
        assert!(!has(&t, "t2") && has(&t, "t1"));
        Ok(())
    }

    #[test]
    fn a_shared_image_stays_while_another_thread_shows_it() -> Result<(), Error> {
        let mut t = start(0)?;
        t.phone("a", Some("b"), true);
        t.app.update_subscriptions();
        t.shell(&[("a", "active", "2"), ("b", "active", "1")]);
        let with_image = |thread: &str| {
            json!({"kind": "snapshot", "snapshotSequence": 1, "hasMoreHistory": false,
                "projection": {"thread": {"id": thread}, "visibleTurnItems": [], "updatedAt": "1",
                "turnItems": [{"id": "u", "type": "user_message", "messageId": "m", "ordinal": 1, "text": "look",
                    "attachments": [{"type": "image", "id": "img", "mimeType": "image/png"}]}]}})
        };
        // Both threads show it; its meta names "a", the first in order.
        t.app.world.set_thread("a", &with_image("a"));
        t.subscription_snapshot("b", with_image("b"));
        t.app.world.set_image("img", "image/png", vec![1, 2, 3]);
        t.app.sync_world();
        let image = crate::world::object_id(&["image", "img"]);
        let has = |t: &Test| t.app.devices["a"].server.object_ids().contains(&image);
        assert!(has(&t));
        // "a" leaves the list; "b" still shows the image.
        t.shell(&[("b", "active", "1")]);
        assert!(has(&t));
        Ok(())
    }

    #[test]
    fn fetch_jobs_go_most_urgent_first_in_order() {
        let q = Queue::default();
        q.push(PRELOAD, Job::Bounded("p".into()));
        q.push(IMAGES, Job::Descriptor);
        q.push(OPEN, Job::Bounded("o1".into()));
        q.push(OPEN, Job::Bounded("o2".into()));
        let order: Vec<String> = (0..4)
            .map(|_| match q.pop() {
                Job::Bounded(t) => t,
                _ => "image".into(),
            })
            .collect();
        assert_eq!(order, ["o1", "o2", "image", "p"]);
    }
}
