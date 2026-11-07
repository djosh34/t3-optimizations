//! A phone without a browser: sync-core's `Phone` over the real data
//! channel (`pipe::client`) or the sidecar's WebSocket fallback, doing what
//! the web app does in WebKit. For the sidecar's integration tests and the
//! lab's `--client sidecar-native`.
//!
//! It pairs over HTTP, signals its first connection, keeps the answer and
//! its certificate, and connects again without signaling. What it receives
//! goes to a `Storage` first and is acked only then, like the web app's
//! IndexedDB; a reloaded page starts from that storage.

use std::collections::BTreeMap;
use std::io;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use pipe::client::{Client, ClientEvent, Identity};
use serde_json::{Value, json};
use str0m::config::DtlsCert;
use sync_core::{Config, InObject, Key, Millis, Phone, Stats};
use tungstenite::{Message, WebSocket};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// One clock for every phone of the process.
fn now_ms() -> Millis {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_millis() as Millis
}

/// What pairing gave the phone.
#[derive(Debug, Clone)]
pub struct Pairing {
    pub device: String,
    pub token: [u8; 16],
}

/// Pairs with a code from the forked server's `t3 pair`.
pub fn pair(origin: &str, code: &str) -> Result<Pairing, Error> {
    let response = ureq::post(&format!("{origin}/api/pair"))
        .config()
        .http_status_as_error(false)
        .build()
        .header("content-type", "application/json")
        .send(json!({"code": code}).to_string())?;
    let status = response.status().as_u16();
    let text = response.into_body().read_to_string()?;
    if status != 200 {
        return Err(format!("pairing: HTTP {status}: {text}").into());
    }
    let v: Value = serde_json::from_str(&text)?;
    let device = v["device"].as_str().ok_or("pairing: no device")?.to_owned();
    let token = v["token"]
        .as_str()
        .and_then(parse_hex16)
        .ok_or("pairing: no token")?;
    Ok(Pairing { device, token })
}

fn parse_hex16(s: &str) -> Option<[u8; 16]> {
    if s.len() != 32 {
        return None;
    }
    let mut out = [0u8; 16];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What the phone stored: its state and each object, as sync-core saves
/// them.
#[derive(Debug, Clone, Default)]
pub struct Storage {
    state: Vec<u8>,
    objects: BTreeMap<Key, Vec<u8>>,
}

enum Transport {
    Rtc(Box<Client>),
    Ws(Box<WebSocket<TcpStream>>),
}

/// How the phone reaches the sidecar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Via {
    /// The data channel from this local address. `server` replaces the
    /// address of the sidecar's candidate, for a relay in tests.
    Rtc {
        local: IpAddr,
        server: Option<SocketAddr>,
    },
    Ws,
}

/// How often the phone reconnected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Reconnects {
    pub ice_restarts: u64,
    /// New connections after the first, without signaling.
    pub generations: u64,
}

/// With nothing from the sidecar for this long, the phone restarts ICE,
/// and again every time this passes. A restart drops the path until ICE
/// connects again, which takes seconds at heavy loss, so it must be rare:
/// at 70% loss and one alive packet a second, 8 s of silence comes about
/// once in 2 minutes.
const RESTART_AFTER: Duration = Duration::from_secs(8);
/// When ICE itself reports the path lost, the restart comes after this
/// much silence.
const RESTART_WHEN_LOST: Duration = Duration::from_secs(2);
/// When ICE connects again but no data comes for this long, the sidecar
/// lost the connection (it restarted): the phone makes a new one. A new
/// connection needs a DTLS handshake, which takes minutes at 70% loss, so
/// this waits long enough not to mistake a lossy stretch for it.
const DEAD_AFTER: Duration = Duration::from_secs(30);

pub struct NativePhone {
    origin: String,
    pairing: Pairing,
    via: Via,
    cert: DtlsCert,
    kept_answer: Option<String>,
    generation: u64,
    transport: Transport,
    phone: Phone,
    storage: Storage,
    replies: BTreeMap<u64, Value>,
    closed: Option<String>,
    last_data: Instant,
    last_restart: Option<Instant>,
    ice_up_since: Option<Instant>,
    /// ICE reports the path lost (Disconnected or Failed).
    ice_lost: bool,
    reconnects: Reconnects,
}

impl NativePhone {
    /// A first page: an empty cache and a signaled first connection (or
    /// the WebSocket).
    pub fn connect(origin: &str, pairing: Pairing, via: Via) -> Result<NativePhone, Error> {
        let cert = pipe::generate_cert()?;
        let epoch = u64::from_le_bytes(random()?);
        let phone = Phone::new(pairing.token, epoch, Config::phone(), now_ms());
        let storage = Storage {
            state: phone.save_state(),
            objects: BTreeMap::new(),
        };
        let (transport, kept_answer) = match via {
            Via::Rtc { local, server } => {
                let identity = Identity {
                    token: pairing.token,
                    device: pairing.device.clone(),
                };
                let mut client = Client::new(SocketAddr::new(local, 0), cert.clone(), identity, 1)?;
                let offer = client.offer()?;
                let mut answer = signal(origin, &pairing, &offer)?;
                if let Some(addr) = server {
                    answer = replace_host_candidate(&answer, addr);
                }
                client.accept_answer(&answer)?;
                (Transport::Rtc(Box::new(client)), Some(answer))
            }
            Via::Ws => (open_ws(origin)?, None),
        };
        Ok(NativePhone {
            origin: origin.to_owned(),
            pairing,
            via,
            cert,
            kept_answer,
            generation: 1,
            transport,
            phone,
            storage,
            replies: BTreeMap::new(),
            closed: None,
            last_data: Instant::now(),
            last_restart: None,
            ice_up_since: None,
            ice_lost: false,
            reconnects: Reconnects::default(),
        })
    }

    fn identity(&self) -> Identity {
        Identity {
            token: self.pairing.token,
            device: self.pairing.device.clone(),
        }
    }

    /// A new connection of `generation`, without signaling.
    fn open(&self, generation: u64) -> Result<Transport, Error> {
        match self.via {
            Via::Rtc { local, .. } => {
                let kept = self.kept_answer.as_deref().ok_or("no kept answer")?;
                let mut client = Client::new(
                    SocketAddr::new(local, 0),
                    self.cert.clone(),
                    self.identity(),
                    generation,
                )?;
                client.connect_cached(kept)?;
                Ok(Transport::Rtc(Box::new(client)))
            }
            Via::Ws => open_ws(&self.origin),
        }
    }

    /// A reloaded page: the next generation, from what this page stored,
    /// connecting without signaling. This page's connection stays as it is.
    pub fn reload(&self) -> Result<NativePhone, Error> {
        let objects = self.storage.objects.values().map(Vec::as_slice);
        let mut phone = Phone::load(&self.storage.state, objects, Config::phone(), now_ms())
            .map_err(|e| format!("stored phone unreadable: {e:?}"))?;
        phone.connected();
        let generation = self.generation + 1;
        Ok(NativePhone {
            origin: self.origin.clone(),
            pairing: self.pairing.clone(),
            via: self.via,
            cert: self.cert.clone(),
            kept_answer: self.kept_answer.clone(),
            generation,
            transport: self.open(generation)?,
            phone,
            storage: self.storage.clone(),
            replies: self.replies.clone(),
            closed: None,
            last_data: Instant::now(),
            last_restart: None,
            ice_up_since: None,
            ice_lost: false,
            reconnects: Reconnects::default(),
        })
    }

    /// A new connection on this page, as the web app makes one when the
    /// old one is dead.
    fn reconnect(&mut self) -> Result<(), Error> {
        self.generation += 1;
        self.transport = self.open(self.generation)?;
        self.phone.connected();
        self.closed = None;
        self.last_data = Instant::now();
        self.last_restart = None;
        self.ice_up_since = None;
        self.ice_lost = false;
        self.reconnects.generations += 1;
        Ok(())
    }

    /// Restarts ICE on the data channel without signaling.
    pub fn restart_ice(&mut self) -> Result<(), Error> {
        let kept = self.kept_answer.clone().ok_or("no kept answer")?;
        match &mut self.transport {
            Transport::Rtc(c) => c.restart_ice(&kept)?,
            Transport::Ws(_) => return Err("the WebSocket has no ICE".into()),
        }
        self.last_restart = Some(Instant::now());
        self.ice_up_since = None;
        self.reconnects.ice_restarts += 1;
        Ok(())
    }

    pub fn reconnects(&self) -> Reconnects {
        self.reconnects
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Why the connection ended, if it did.
    pub fn closed(&self) -> Option<&str> {
        self.closed.as_deref()
    }

    pub fn is_open(&self) -> bool {
        match &self.transport {
            Transport::Rtc(c) => c.is_open(),
            Transport::Ws(_) => self.closed.is_none(),
        }
    }

    /// Restarts ICE while the sidecar is silent, and makes a new
    /// connection when the old one is dead.
    fn keep_alive(&mut self) -> Result<(), Error> {
        if self.closed.is_some() {
            return self.reconnect();
        }
        let wait = if self.ice_lost {
            RESTART_WHEN_LOST
        } else {
            RESTART_AFTER
        };
        if !matches!(self.transport, Transport::Rtc(_)) || self.last_data.elapsed() < wait {
            return Ok(());
        }
        if self.ice_up_since.is_some_and(|t| t.elapsed() >= DEAD_AFTER) {
            return self.reconnect();
        }
        if self.ice_up_since.is_none()
            && self
                .last_restart
                .is_none_or(|t| t.elapsed() >= RESTART_AFTER)
        {
            self.restart_ice()?;
        }
        Ok(())
    }

    /// Receives, stores and sends for up to `wait`.
    pub fn step(&mut self, wait: Duration) -> Result<(), Error> {
        self.keep_alive()?;
        let now = now_ms();
        let due = self.phone.next_wakeup(now).saturating_sub(now);
        let until = Instant::now() + wait.min(Duration::from_millis(due));
        let mut received = Vec::new();
        match &mut self.transport {
            Transport::Rtc(c) => {
                let mut events = Vec::new();
                c.poll(until, &mut events)?;
                for ev in events {
                    match ev {
                        ClientEvent::Data(d) => received.push(d),
                        ClientEvent::Closed(why) => self.closed = Some(why),
                        ClientEvent::IceState(s) if s == "Connected" || s == "Completed" => {
                            self.ice_lost = false;
                            self.ice_up_since.get_or_insert_with(Instant::now);
                        }
                        ClientEvent::IceState(s) => {
                            self.ice_lost = s == "Disconnected" || s == "Failed";
                            self.ice_up_since = None;
                        }
                        ClientEvent::Open => {}
                    }
                }
            }
            Transport::Ws(ws) => {
                let left = until.saturating_duration_since(Instant::now());
                ws.get_ref()
                    .set_read_timeout(Some(left.max(Duration::from_millis(1))))?;
                match ws.read() {
                    Ok(Message::Binary(d)) => received.push(d.to_vec()),
                    Ok(Message::Close(_)) => self.closed = Some("socket closed".into()),
                    Ok(_) => {}
                    Err(tungstenite::Error::Io(e))
                        if matches!(
                            e.kind(),
                            io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                        ) => {}
                    Err(e) => self.closed = Some(e.to_string()),
                }
            }
        }
        if !received.is_empty() {
            self.last_data = Instant::now();
            self.ice_up_since = None;
        }
        for d in received {
            self.receive(&d);
        }
        if self.closed.is_some() {
            return Ok(());
        }
        while let Some(packet) = self.phone.poll(now_ms()) {
            match &mut self.transport {
                // A full buffer is loss.
                Transport::Rtc(c) => {
                    c.send(&packet)?;
                }
                Transport::Ws(ws) => {
                    if let Err(e) = ws.send(Message::binary(packet)) {
                        self.closed = Some(e.to_string());
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    /// Stores what a packet brought, then acks it.
    fn receive(&mut self, data: &[u8]) {
        let Ok(r) = self.phone.receive(now_ms(), data) else {
            return;
        };
        for key in &r.changed {
            if let Some(saved) = self.phone.save_object(*key) {
                self.storage.objects.insert(*key, saved);
            }
        }
        for key in &r.removed {
            self.storage.objects.remove(key);
        }
        for reply in &r.replies {
            let v = serde_json::from_slice(&reply.payload).unwrap_or(Value::Null);
            self.replies.insert(reply.seq, v);
        }
        if !r.replies.is_empty() {
            self.storage.state = self.phone.save_state();
        }
        self.phone.committed(r.batch);
    }

    /// Queues an `orchestration.dispatchCommand`. Returns its number.
    pub fn command(&mut self, command: &Value) -> u64 {
        let seq = self.phone.command(command.to_string().into_bytes());
        self.storage.state = self.phone.save_state();
        seq
    }

    /// Queues a call. Returns its number.
    pub fn call(&mut self, tag: &str, payload: Value) -> u64 {
        let call = json!({"tag": tag, "payload": payload});
        let seq = self.phone.call(call.to_string().into_bytes(), false);
        self.storage.state = self.phone.save_state();
        seq
    }

    pub fn reply(&self, seq: u64) -> Option<&Value> {
        self.replies.get(&seq)
    }

    pub fn stats(&self) -> Stats {
        self.phone.stats()
    }

    /// Every object with its meta.
    pub fn objects(&self) -> impl Iterator<Item = (Value, &InObject)> {
        self.phone.objects().map(|(_, o)| {
            let meta = serde_json::from_slice(&o.meta).unwrap_or(Value::Null);
            (meta, o)
        })
    }

    /// The bodies of a collection of a thread, complete ones only.
    pub fn bodies(&self, thread: &str, collection: &str) -> Vec<Value> {
        self.objects()
            .filter(|(m, o)| m["t"] == thread && m["c"] == collection && o.complete() && o.len > 0)
            .filter_map(|(_, o)| serde_json::from_slice(&o.data).ok())
            .collect()
    }

    /// Texts of a thread by kind (`assistant`, `user`, ...): message key
    /// and every run of bytes the phone has, decoded.
    pub fn texts(&self, thread: &str, kind: &str) -> Vec<(String, Vec<String>)> {
        self.objects()
            .filter(|(m, _)| m["t"] == thread && m["k"] == kind && m["m"].is_string())
            .map(|(m, o)| {
                let runs = o
                    .have
                    .ranges()
                    .iter()
                    .filter_map(|r| o.bytes(r.clone()))
                    .map(|b| String::from_utf8_lossy(b).into_owned())
                    .collect();
                (m["m"].as_str().unwrap_or_default().to_owned(), runs)
            })
            .collect()
    }

    /// The whole text of one message, if the phone has all of it.
    pub fn full_text(&self, thread: &str, message: &str) -> Option<String> {
        self.objects()
            .find(|(m, _)| m["t"] == thread && m["m"] == message)
            .and_then(|(_, o)| {
                o.complete()
                    .then(|| String::from_utf8_lossy(&o.data).into_owned())
            })
    }
}

fn signal(origin: &str, pairing: &Pairing, offer: &str) -> Result<String, Error> {
    let response = ureq::post(&format!("{origin}/api/rtc/offer"))
        .header("authorization", &format!("Bearer {}", hex(&pairing.token)))
        .config()
        .http_status_as_error(false)
        .build()
        .send(offer)?;
    let status = response.status().as_u16();
    let text = response.into_body().read_to_string()?;
    if status != 200 {
        return Err(format!("signaling: HTTP {status}: {text}").into());
    }
    Ok(text)
}

/// An SDP answer whose host candidates name `addr` instead.
fn replace_host_candidate(sdp: &str, addr: SocketAddr) -> String {
    sdp.split_inclusive('\n')
        .map(|line| {
            let Some(rest) = line.strip_prefix("a=candidate:") else {
                return line.to_owned();
            };
            // foundation component transport priority address port typ ...
            let mut parts: Vec<&str> = rest.split(' ').collect();
            if parts.len() < 6 {
                return line.to_owned();
            }
            let (ip, port) = (addr.ip().to_string(), addr.port().to_string());
            parts[4] = &ip;
            parts[5] = &port;
            format!("a=candidate:{}", parts.join(" "))
        })
        .collect()
}

fn open_ws(origin: &str) -> Result<Transport, Error> {
    let host = origin
        .strip_prefix("http://")
        .ok_or("the sidecar origin must be http://")?
        .trim_end_matches('/');
    let stream = TcpStream::connect(host)?;
    stream.set_nodelay(true)?;
    let (ws, _) = tungstenite::client(format!("ws://{host}/sync").as_str(), stream)
        .map_err(|e| e.to_string())?;
    Ok(Transport::Ws(Box::new(ws)))
}

fn random<const N: usize>() -> Result<[u8; N], Error> {
    let mut out = [0u8; N];
    getrandom::fill(&mut out).map_err(|e| e.to_string())?;
    Ok(out)
}

/// A random id for t3 commands and messages.
pub fn new_id(prefix: &str) -> Result<String, Error> {
    Ok(format!("{prefix}:{}", hex(&random::<16>()?)))
}

/// The command that sends a prompt to a thread, as the web client sends it.
pub fn prompt_command(thread: &str, text: &str) -> Result<Value, Error> {
    Ok(json!({
        "type": "message.dispatch",
        "commandId": new_id("command")?,
        "createdBy": "user",
        "creationSource": "mobile",
        "threadId": thread,
        "messageId": new_id("message")?,
        "text": text,
        "attachments": [],
        "deliveryIntent": "auto",
        "dispatchMode": {"type": "start_immediately"},
    }))
}

/// The command that answers an approval request.
pub fn approve_command(thread: &str, request: &str) -> Result<Value, Error> {
    Ok(json!({
        "type": "runtime-request.respond",
        "commandId": new_id("command")?,
        "threadId": thread,
        "requestId": request,
        "decision": "accept",
    }))
}

/// Whether a reply is t3's success exit.
pub fn succeeded(reply: &Value) -> bool {
    reply["exit"]["_tag"] == "Success"
}
