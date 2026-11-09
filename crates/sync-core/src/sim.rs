//! A deterministic simulator: a [`Server`] and a [`Phone`] over a shaped,
//! lossy link, with a scripted agent and the phone's storage. The same
//! scenario and seed give the same run, packet for packet.
//!
//! The link follows the lab's netem setup (`lab/README.md`): per direction
//! a rate, independent random loss applied before the rate limit (as netem
//! does), a queue limit in packets, a one-way delay with normal jitter
//! (which can reorder), duplication, outages, and loss in bursts (netem's
//! `gemodel`). A scenario can change the link as it runs, as a story does
//! ([`story`]). Every packet costs
//! [`WIRE_OVERHEAD`] bytes more than its application bytes.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::ops::Range;

use crate::Incoming;
use crate::ranges::RangeSet;
use crate::wire::{Key, Packet};
use crate::{Class, Config, Millis, Phone, Server, Stats, WIRE_OVERHEAD};

mod browser;
pub mod story;

pub use story::Story;

use browser::{BrowserSctp, PACKET_HEADER, SACK_CHUNK, SctpPacket};

/// splitmix64: small, fast, and the same on every platform.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Standard normal, by Box-Muller.
    pub fn normal(&mut self) -> f64 {
        let u = 1.0 - self.f64();
        let v = self.f64();
        (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
    }
}

/// One direction of the link.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkSpec {
    pub rate_bps: u64,
    /// Independent random loss, 0 to 1.
    pub loss: f64,
    pub delay_ms: u64,
    /// Standard deviation of the normal jitter.
    pub jitter_ms: u64,
    /// Packets waiting for the rate limit; more are dropped.
    pub queue: usize,
    pub duplicate: f64,
    /// Times when nothing gets through, counted from the profile's start.
    pub outages: Vec<Range<Millis>>,
    /// Loss in bursts, as netem's `gemodel p r 100% 0%`: per packet, the
    /// chance to go from good to bad and from bad to good; packets are lost
    /// while bad.
    pub burst: Option<(f64, f64)>,
}

impl LinkSpec {
    /// The link before the profile applies: fast and clean, a veth pair as
    /// in the lab.
    pub fn clean() -> Self {
        LinkSpec {
            rate_bps: 10_000_000,
            loss: 0.0,
            delay_ms: 1,
            jitter_ms: 0,
            queue: 1000,
            duplicate: 0.0,
            outages: Vec::new(),
            burst: None,
        }
    }

    /// The lab's case 70 and case 50 links (`lab/README.md`).
    pub fn lab(rate_bps: u64, loss: f64, queue: usize) -> Self {
        LinkSpec {
            rate_bps,
            loss,
            delay_ms: 300,
            jitter_ms: 50,
            queue,
            duplicate: 0.0,
            outages: Vec::new(),
            burst: None,
        }
    }
}

/// One direction of the simulated link, carrying packets of type `T`.
#[derive(Debug)]
struct Link<T> {
    spec: LinkSpec,
    /// Microseconds, when the rate limiter is free again.
    busy_until: u64,
    /// When each queued packet leaves the rate limiter, microseconds.
    queue: VecDeque<u64>,
    /// Arrival times, with a number that keeps equal times in order.
    flight: BinaryHeap<Reverse<(Millis, u64)>>,
    packets: HashMap<u64, T>,
    order: u64,
    /// Outages are relative to this time.
    start: Millis,
    /// In the bad state of `spec.burst`.
    bad: bool,
}

impl<T: Clone> Link<T> {
    fn new(spec: LinkSpec) -> Self {
        Link {
            spec,
            busy_until: 0,
            queue: VecDeque::new(),
            flight: BinaryHeap::new(),
            packets: HashMap::new(),
            order: 0,
            start: 0,
            bad: false,
        }
    }

    /// Steps the burst model for one packet; true if it is lost.
    fn burst_lost(&mut self, rng: &mut Rng) -> bool {
        let Some((p, r)) = self.spec.burst else {
            return false;
        };
        self.bad = if self.bad {
            rng.f64() >= r
        } else {
            rng.f64() < p
        };
        self.bad
    }

    fn down(&self, at: Millis) -> bool {
        let t = at.saturating_sub(self.start);
        at >= self.start && self.spec.outages.iter().any(|o| o.contains(&t))
    }

    /// Sends a packet of `wire` bytes on the wire.
    fn send(&mut self, now: Millis, wire: usize, packet: T, rng: &mut Rng) {
        if self.down(now) || rng.f64() < self.spec.loss || self.burst_lost(rng) {
            return;
        }
        let copies = if rng.f64() < self.spec.duplicate {
            2
        } else {
            1
        };
        let now_us = now * 1000;
        for _ in 0..copies {
            while self.queue.front().is_some_and(|&t| t <= now_us) {
                self.queue.pop_front();
            }
            if self.queue.len() >= self.spec.queue {
                return;
            }
            let depart =
                self.busy_until.max(now_us) + wire as u64 * 8 * 1_000_000 / self.spec.rate_bps;
            self.busy_until = depart;
            self.queue.push_back(depart);
            let jitter = rng.normal() * self.spec.jitter_ms as f64;
            let arrive = (depart as f64 / 1000.0 + self.spec.delay_ms as f64 + jitter)
                .max(depart as f64 / 1000.0)
                .ceil() as Millis;
            self.order += 1;
            self.flight.push(Reverse((arrive, self.order)));
            self.packets.insert(self.order, packet.clone());
        }
    }

    fn next_arrival(&self) -> Option<Millis> {
        self.flight.peek().map(|Reverse((t, _))| *t)
    }

    fn take(&mut self, now: Millis) -> Vec<T> {
        let mut out = Vec::new();
        while self.next_arrival().is_some_and(|t| t <= now) {
            let Some(Reverse((t, n))) = self.flight.pop() else {
                break;
            };
            if let Some(p) = self.packets.remove(&n)
                && !self.down(t)
            {
                out.push(p);
            }
        }
        out
    }

    /// Drops what is on the way: the page that would get it is gone.
    fn clear(&mut self) {
        self.flight.clear();
        self.packets.clear();
    }

    /// Switches to a new profile. Packets already queued keep their times.
    fn apply(&mut self, spec: LinkSpec, now: Millis) {
        self.spec = spec;
        self.start = now;
    }
}

/// What goes from the server to the phone.
#[derive(Debug, Clone)]
enum Down {
    /// A sync packet, with the SACK for the phone's SCTP that rides in it.
    Sync(Vec<u8>, Option<u64>),
    /// The server's SCTP acks the phone's DATA up to this TSN; it acks past
    /// lost chunks.
    Sack(u64),
}

/// What goes from the phone to the server.
#[derive(Debug, Clone)]
enum Up {
    Sync(Vec<u8>),
    /// A packet of the browser's SCTP.
    Sctp(SctpPacket),
}

/// One simulated run.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub seed: u64,
    /// Time on the clean link before the profile applies, with everything
    /// that exists at the start syncing.
    pub warmup_ms: Millis,
    pub down: LinkSpec,
    pub up: LinkSpec,
    /// How long the run goes on after the profile applies.
    pub duration_ms: Millis,
    /// Prompts typed at these times after the profile applies. The first
    /// starts the agent's reply.
    pub prompts_at: Vec<Millis>,
    pub reply_chars_per_sec: u64,
    pub reply_len: usize,
    /// Content of other threads that changes when the profile applies, so
    /// backfill competes with the reply.
    pub backfill_bytes: usize,
    /// The backfill appears this long before the profile applies, so the
    /// server is sending at the clean link's rate when the link turns bad,
    /// as in the lab, where the web app is still syncing at the switch.
    pub backfill_lead_ms: Millis,
    /// Kill the page at these times after the profile applies: what was
    /// received but not stored is gone, and the phone reloads from storage
    /// on a new connection.
    pub kills_at: Vec<Millis>,
    /// Time from receiving a packet to its content being stored.
    pub commit_delay_ms: Millis,
    /// The phone's packets go through a model of WebKit's SCTP sender
    /// ([`browser`]) instead of straight onto the link, and its SACKs share
    /// the server's direction.
    pub webkit: bool,
    /// The link changes to these, down and up, at these times after the
    /// profile applies.
    pub changes: Vec<(Millis, LinkSpec, LinkSpec)>,
    /// End the run once every prompt is confirmed and the reply is whole.
    pub stop_when_done: bool,
}

impl Scenario {
    /// The lab's case 70: 10 kbit/s and 70% loss each way, 300 ± 50 ms,
    /// queue 20, warm start, a 64-byte prompt and a reply at 50
    /// characters/s. Passes when the prompt is confirmed within 10 s.
    pub fn case70(seed: u64) -> Self {
        Scenario {
            seed,
            warmup_ms: 20_000,
            down: LinkSpec::lab(10_000, 0.7, 20),
            up: LinkSpec::lab(10_000, 0.7, 20),
            duration_ms: 900_000,
            prompts_at: vec![1_000],
            reply_chars_per_sec: 50,
            reply_len: 30_000,
            backfill_bytes: 200_000,
            backfill_lead_ms: 3_000,
            kills_at: Vec::new(),
            commit_delay_ms: 10,
            webkit: false,
            changes: Vec::new(),
            stop_when_done: false,
        }
    }

    /// A seed's network story ([`Story`]), with the lab's short prompt and a
    /// 2,000-character reply, through WebKit's SCTP. It ends once the reply
    /// is whole and the prompt confirmed, or [`story::RUN_MS`] after the
    /// story starts.
    pub fn story(seed: u64) -> Self {
        let story = Story::generate(seed);
        let mut changes: Vec<(Millis, LinkSpec, LinkSpec)> = story
            .segments
            .into_iter()
            .map(|s| (s.at_ms, s.down, s.up))
            .collect();
        let (_, down, up) = changes.remove(0);
        Scenario {
            down,
            up,
            changes,
            duration_ms: story::RUN_MS,
            prompts_at: vec![0],
            reply_len: 2_000,
            webkit: true,
            stop_when_done: true,
            ..Scenario::case70(seed)
        }
    }

    /// The lab's case 50: 100 kbit/s and 50% loss each way, queue 50,
    /// otherwise as case 70. Passes when the newest text is never more
    /// than 5 s behind.
    pub fn case50(seed: u64) -> Self {
        Scenario {
            down: LinkSpec::lab(100_000, 0.5, 50),
            up: LinkSpec::lab(100_000, 0.5, 50),
            ..Scenario::case70(seed)
        }
    }

    /// The same run with WebKit's SCTP on the phone's side.
    pub fn in_webkit(self) -> Self {
        Scenario {
            webkit: true,
            ..self
        }
    }

    /// Cuts the run to `ms` after the profile, with a reply that fits.
    pub fn shortened(mut self, ms: Millis) -> Self {
        self.duration_ms = ms;
        let reply_ms = ms.saturating_sub(30_000);
        self.reply_len = self
            .reply_len
            .min((reply_ms * self.reply_chars_per_sec / 1000) as usize);
        self
    }
}

/// What a run measured. Times are milliseconds after the profile applied.
#[derive(Debug, Clone, Default)]
pub struct Outcome {
    /// For each prompt: from typing it to its result being stored on the
    /// phone. None if that never happened.
    pub prompt_confirmed_ms: Vec<Option<Millis>>,
    /// Newest-text lag, sampled every 100 ms while the agent writes: how
    /// old the oldest text the agent produced but the phone does not have
    /// is (the lab's measure).
    pub lag_max_ms: Millis,
    pub lag_p95_ms: Millis,
    pub lag_median_ms: Millis,
    /// When the phone first had the whole reply.
    pub reply_done_ms: Option<Millis>,
    /// When the phone first had everything.
    pub synced_ms: Option<Millis>,
    /// Bytes stored on the phone that it had stored before.
    pub duplicate_bytes: u64,
    /// Bytes the server sent after the phone's ack for them had reached it.
    pub acked_resent_bytes: u64,
    /// Commands the server handed to the sidecar more than once.
    pub commands_twice: u64,
    pub largest_packet: usize,
    /// Whether every object on the phone equals the server's at the end.
    pub phone_matches: bool,
    pub server: Stats,
    pub phone: Stats,
    /// Both sides' statistics, every second after the profile applies.
    pub trace: Vec<(Millis, Stats, Stats)>,
}

/// The web app's `SEND_BUFFER` (`t3code/apps/web/src/t3opt/link.ts`).
const WEBKIT_SEND_BUFFER: usize = 1_200;
const TOKEN: [u8; 16] = *b"sim-token-000001";
const THREAD_LIST: u64 = 1;
const STATUS: u64 = 2;
const FIRST_HISTORY: u64 = 100;
const FIRST_OTHER: u64 = 1_000;
const FIRST_IMAGE: u64 = 5_000;
const REPLY: u64 = 10_000;
const HISTORY_MESSAGES: u64 = 20;

/// The lab agent's first paragraph (`lab/internal/agent/reply.go`).
const PARAGRAPH: &str = "The sync loop starts from what the phone last acknowledged. \
When the link comes back after an outage, the server does not replay every event it produced in the meantime. \
It looks at the newest state of the current thread, compares it with the acknowledged state, and sends the difference, newest text first. \
That keeps the first packets after an outage useful: the owner sees where the agent is now, not where it was a minute ago. \
Older messages follow when the newest ones are safe on the phone.\n\n";

fn text(len: usize, salt: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(len + PARAGRAPH.len());
    let mut part = 0;
    while out.len() < len {
        part += 1;
        out.extend_from_slice(format!("Part {salt}.{part}. ").as_bytes());
        out.extend_from_slice(PARAGRAPH.as_bytes());
    }
    out.truncate(len);
    out
}

fn noise(len: usize, rng: &mut Rng) -> Vec<u8> {
    (0..len).map(|_| rng.next_u64() as u8).collect()
}

struct Commit {
    at: Millis,
    batch: u64,
    received: crate::Received,
}

/// The phone with its storage: rows written when a batch is stored.
struct Device {
    phone: Phone,
    state: Vec<u8>,
    rows: BTreeMap<Key, Vec<u8>>,
    commits: VecDeque<Commit>,
    /// Stored bytes by object and revision, to find duplicates.
    stored: HashMap<(Key, u64), RangeSet>,
}

/// The server's side of the world: the server, the agent and what it set.
struct Host {
    server: Server,
    objects: BTreeMap<u64, (Class, u64, Vec<u8>, Vec<u8>)>,
    /// When each byte of the reply was produced.
    produced: Vec<Millis>,
    reply_text: Vec<u8>,
    replying: bool,
    commands: BTreeMap<u64, u64>,
}

impl Host {
    fn set(
        &mut self,
        id: u64,
        class: Class,
        order: u64,
        meta: Vec<u8>,
        content: Vec<u8>,
    ) -> Result<(), String> {
        self.server
            .set(id, class, order, meta.clone(), content.clone())
            .map_err(|e| e.to_string())?;
        self.objects.insert(id, (class, order, meta, content));
        Ok(())
    }
}

/// Runs a scenario. Fails only on a bug: an object the server refuses,
/// a reply it refuses, or saved phone state that does not load.
pub fn run(s: &Scenario) -> Result<Outcome, String> {
    let mut rng = Rng::new(s.seed);
    let mut down: Link<Down> = Link::new(LinkSpec::clean());
    let mut up: Link<Up> = Link::new(LinkSpec::clean());
    let mut sctp = BrowserSctp::default();
    // The newest TSN of the phone's SCTP the server has, for its SACKs.
    let mut sctp_cumulative: Option<u64> = None;
    let mut host = Host {
        server: Server::new(TOKEN, Config::server(), 0),
        objects: BTreeMap::new(),
        produced: Vec::new(),
        reply_text: text(s.reply_len, 0),
        replying: false,
        commands: BTreeMap::new(),
    };
    let mut dev = Device {
        phone: Phone::new(TOKEN, rng.next_u64(), Config::phone(), 0),
        state: Vec::new(),
        rows: BTreeMap::new(),
        commits: VecDeque::new(),
        stored: HashMap::new(),
    };
    let mut out = Outcome::default();

    // The world at the start: a thread with history, a thread list, a
    // status, other threads and two images.
    host.set(
        THREAD_LIST,
        Class::ThreadList,
        0,
        Vec::new(),
        text(2_000, 1),
    )?;
    host.set(STATUS, Class::Status, 0, Vec::new(), b"idle".to_vec())?;
    for i in 0..HISTORY_MESSAGES {
        let meta = format!("thread:1 message:{i}").into_bytes();
        host.set(
            FIRST_HISTORY + i,
            Class::History,
            i,
            meta,
            text(1_500, 10 + i),
        )?;
    }
    for i in 0..10 {
        host.set(
            FIRST_OTHER + i,
            Class::Background,
            i,
            Vec::new(),
            text(10_000, 100 + i),
        )?;
    }
    for i in 0..2 {
        let img = noise(30_000, &mut rng);
        host.set(FIRST_IMAGE + i, Class::Image, i, Vec::new(), img)?;
    }

    let t0 = s.warmup_ms;
    let end = t0 + s.duration_ms;
    let mut prompts: Vec<(Millis, Option<u64>)> =
        s.prompts_at.iter().map(|&t| (t0 + t, None)).collect();
    out.prompt_confirmed_ms = vec![None; prompts.len()];
    let mut kills: VecDeque<Millis> = s.kills_at.iter().map(|&t| t0 + t).collect();
    let mut changes: VecDeque<&(Millis, LinkSpec, LinkSpec)> = s.changes.iter().collect();
    let mut profile_applied = false;
    let mut backfill_set = false;
    let mut next_tick = t0;
    let mut next_sample = t0;
    let mut next_trace = t0;
    let mut lags: Vec<Millis> = Vec::new();
    // Chunks of each sent downlink packet, by packet number, and what the
    // server has seen acked, to check that acked bytes never go out again.
    let mut sent_chunks: HashMap<u64, Vec<(Key, u64, Range<u64>)>> = HashMap::new();
    let mut known_acked: HashMap<(Key, u64), RangeSet> = HashMap::new();
    let mut attached = false;

    let mut now: Millis = 0;
    while now <= end {
        if !profile_applied && now >= t0 {
            profile_applied = true;
            down.apply(s.down.clone(), now);
            up.apply(s.up.clone(), now);
        }
        while let Some((_, d, u)) = changes.front().filter(|(at, _, _)| now >= t0 + at) {
            down.apply(d.clone(), now);
            up.apply(u.clone(), now);
            changes.pop_front();
        }
        if s.stop_when_done
            && out.reply_done_ms.is_some()
            && out.prompt_confirmed_ms.iter().all(Option::is_some)
        {
            break;
        }
        if !backfill_set && now + s.backfill_lead_ms >= t0 {
            backfill_set = true;
            // Other threads change: new backfill.
            let n = s.backfill_bytes.div_ceil(20_000);
            for i in 0..n as u64 {
                host.set(
                    FIRST_OTHER + 100 + i,
                    Class::Background,
                    100 + i,
                    Vec::new(),
                    text(20_000, 500 + i),
                )?;
            }
        }

        for (i, (at, seq)) in prompts.iter_mut().enumerate() {
            if seq.is_none() && now >= *at {
                let payload = format!("{{\"type\":\"thread.turn.start\",\"prompt\":{i:>32}}}");
                *seq = Some(dev.phone.command(payload.into_bytes()));
            }
        }

        if kills.front().is_some_and(|&t| now >= t) {
            kills.pop_front();
            let rows: Vec<&[u8]> = dev.rows.values().map(Vec::as_slice).collect();
            dev.phone = Phone::load(&dev.state, rows, Config::phone(), now)
                .map_err(|e| format!("reload after kill: {e}"))?;
            dev.commits.clear();
            dev.phone.connected();
            host.server.connected();
            down.clear();
            up.clear();
            // A new page has a new SCTP association.
            sctp = BrowserSctp::default();
            sctp_cumulative = None;
            sent_chunks.clear();
        }

        // The agent writes the reply, a tenth of a second at a time.
        if host.replying && now >= next_tick {
            next_tick = now + 100;
            let have = host.produced.len();
            let step = (s.reply_chars_per_sec as usize / 10).max(1);
            let upto = (have + step).min(host.reply_text.len());
            host.produced.resize(upto, now);
            let content = host.reply_text[..upto].to_vec();
            host.set(
                REPLY,
                Class::Tail,
                0,
                b"thread:1 message:reply".to_vec(),
                content,
            )?;
            if upto == host.reply_text.len() {
                host.replying = false;
                host.set(STATUS, Class::Status, 0, Vec::new(), b"idle".to_vec())?;
            }
        }

        // Storage finishes.
        while dev.commits.front().is_some_and(|c| c.at <= now) {
            let Some(c) = dev.commits.pop_front() else {
                break;
            };
            for k in &c.received.changed {
                if let Some(row) = dev.phone.save_object(*k) {
                    dev.rows.insert(*k, row);
                }
            }
            for k in &c.received.removed {
                dev.rows.remove(k);
            }
            dev.state = dev.phone.save_state();
            dev.phone.committed(c.batch);
            for d in &c.received.delivered {
                let set = dev.stored.entry((d.key, d.rev)).or_default();
                for r in &d.new {
                    let had: u64 = r.end
                        - r.start
                        - set
                            .gaps(r.clone())
                            .iter()
                            .map(|g| g.end - g.start)
                            .sum::<u64>();
                    out.duplicate_bytes += had;
                    set.insert(r.clone());
                }
            }
            for reply in &c.received.replies {
                for (i, (at, seq)) in prompts.iter().enumerate() {
                    if *seq == Some(reply.seq) && out.prompt_confirmed_ms[i].is_none() {
                        out.prompt_confirmed_ms[i] = Some(now - at);
                    }
                }
            }
        }

        for packet in down.take(now) {
            let bytes = match packet {
                Down::Sync(bytes, sack) => {
                    if let Some(cumulative) = sack {
                        sctp.on_sack(now, cumulative);
                    }
                    bytes
                }
                Down::Sack(cumulative) => {
                    sctp.on_sack(now, cumulative);
                    continue;
                }
            };
            if s.webkit {
                sctp.on_server_data(now);
            }
            if let Ok(r) = dev.phone.receive(now, &bytes) {
                dev.commits.push_back(Commit {
                    at: now + s.commit_delay_ms,
                    batch: r.batch,
                    received: r,
                });
            }
        }
        let mut from_phone = Vec::new();
        for packet in up.take(now) {
            match packet {
                Up::Sync(bytes) => from_phone.push(bytes),
                Up::Sctp(p) if p.messages.is_empty() => {}
                Up::Sctp(p) => {
                    // The server's SCTP acks every DATA packet at once.
                    for (tsn, m) in p.messages {
                        sctp_cumulative = sctp_cumulative.max(Some(tsn));
                        from_phone.push(m);
                    }
                    if let Some(cumulative) = sctp_cumulative {
                        down.send(
                            now,
                            PACKET_HEADER + SACK_CHUNK,
                            Down::Sack(cumulative),
                            &mut rng,
                        );
                    }
                }
            }
        }
        for bytes in from_phone {
            if let Ok(p) = Packet::decode(&bytes)
                && let Some(a) = &p.ack
            {
                for r in &a.ranges {
                    for seq in r.clone() {
                        for (k, rev, range) in sent_chunks.get(&seq).into_iter().flatten() {
                            known_acked
                                .entry((*k, *rev))
                                .or_default()
                                .insert(range.clone());
                        }
                    }
                }
            }
            let Ok(incoming) = host.server.receive(now, &bytes) else {
                continue;
            };
            for i in incoming {
                if i == Incoming::Reset {
                    // A new epoch: the server starts from zero, and so does
                    // the check of what it knew was acked.
                    attached = true;
                    known_acked.clear();
                }
                if let Incoming::Command { seq, .. } = i {
                    let n = host.commands.entry(seq).or_default();
                    *n += 1;
                    if *n > 1 {
                        out.commands_twice += 1;
                        continue;
                    }
                    host.server
                        .reply(seq, format!("{{\"sequence\":{seq}}}").into_bytes())
                        .map_err(|e| e.to_string())?;
                    if host.produced.is_empty() && !host.replying {
                        host.replying = true;
                        next_tick = now;
                        host.set(STATUS, Class::Status, 0, Vec::new(), b"running".to_vec())?;
                    }
                }
            }
        }

        // Like the sidecar, the server sends only once the phone's hello
        // said who it is.
        while let Some(bytes) = attached.then(|| host.server.poll(now)).flatten() {
            out.largest_packet = out.largest_packet.max(bytes.len());
            if let Ok(p) = Packet::decode(&bytes) {
                let mut chunks = Vec::new();
                for c in &p.chunks {
                    let r = c.offset..c.offset + c.body.raw_len();
                    if let Some(acked) = known_acked.get(&(c.key, c.rev)) {
                        let gaps: u64 = acked.gaps(r.clone()).iter().map(|g| g.end - g.start).sum();
                        out.acked_resent_bytes += (r.end - r.start) - gaps;
                    }
                    chunks.push((c.key, c.rev, r));
                }
                sent_chunks.insert(p.seq, chunks);
            }
            let sack = sctp_cumulative.filter(|_| s.webkit);
            let wire = bytes.len() + WIRE_OVERHEAD + if sack.is_some() { SACK_CHUNK } else { 0 };
            down.send(now, wire, Down::Sync(bytes, sack), &mut rng);
        }
        if s.webkit {
            // The web app hands packets over while less than 1200 bytes wait.
            while sctp.buffered() < WEBKIT_SEND_BUFFER
                && let Some(bytes) = dev.phone.poll(now)
            {
                out.largest_packet = out.largest_packet.max(bytes.len());
                sctp.send(bytes);
            }
            while let Some(p) = sctp.poll(now) {
                up.send(now, p.wire_len(), Up::Sctp(p), &mut rng);
            }
        } else {
            while let Some(bytes) = dev.phone.poll(now) {
                out.largest_packet = out.largest_packet.max(bytes.len());
                up.send(now, bytes.len() + WIRE_OVERHEAD, Up::Sync(bytes), &mut rng);
            }
        }

        if now >= next_sample && now >= t0 {
            next_sample = now + 100;
            let lag = match dev.phone.object(REPLY) {
                _ if host.produced.is_empty() => None,
                None => host.produced.first().map(|&t| now - t),
                Some(o) => {
                    let produced = host.produced.len() as u64;
                    let missing = if o.rev == 0 {
                        Some(0)
                    } else {
                        o.have.gaps(0..produced).first().map(|g| g.start)
                    };
                    let at = missing.and_then(|i| host.produced.get(i as usize));
                    Some(at.map_or(0, |&t| now - t))
                }
            };
            if let Some(l) = lag {
                lags.push(l);
            }
            if out.reply_done_ms.is_none()
                && !host.reply_text.is_empty()
                && host.produced.len() == host.reply_text.len()
                && lag == Some(0)
            {
                out.reply_done_ms = Some(now - t0);
            }
            if out.synced_ms.is_none()
                && host.server.is_synced()
                && !host.replying
                && !host.produced.is_empty()
            {
                out.synced_ms = Some(now - t0);
            }
        }
        if now >= next_trace && now >= t0 {
            next_trace = now + 1000;
            out.trace
                .push((now - t0, host.server.stats(), dev.phone.stats()));
        }

        let mut next = end + 1;
        if attached {
            next = next.min(host.server.next_wakeup(now));
        }
        // A full send buffer empties only on the SCTP sender's own events.
        if !s.webkit || sctp.buffered() < WEBKIT_SEND_BUFFER {
            next = next.min(dev.phone.next_wakeup(now));
        }
        if let Some(t) = sctp.next_wakeup() {
            next = next.min(t);
        }
        for t in [
            down.next_arrival(),
            up.next_arrival(),
            dev.commits.front().map(|c| c.at),
        ]
        .into_iter()
        .flatten()
        {
            next = next.min(t);
        }
        if !profile_applied {
            next = next.min(t0);
        }
        if !backfill_set {
            next = next.min(t0.saturating_sub(s.backfill_lead_ms));
        }
        if host.replying {
            next = next.min(next_tick);
        }
        for (at, seq) in &prompts {
            if seq.is_none() {
                next = next.min(*at);
            }
        }
        if let Some(&k) = kills.front() {
            next = next.min(k);
        }
        if let Some((at, _, _)) = changes.front() {
            next = next.min(t0 + at);
        }
        if now >= t0 {
            next = next.min(next_sample).min(next_trace);
        }
        now = next.max(now + 1);
    }

    lags.sort_unstable();
    let q = |f: f64| -> Millis {
        if lags.is_empty() {
            return 0;
        }
        let i = (lags.len().saturating_sub(1) as f64 * f) as usize;
        lags.get(i).copied().unwrap_or(0)
    };
    out.lag_max_ms = q(1.0);
    out.lag_p95_ms = q(0.95);
    out.lag_median_ms = q(0.5);
    out.phone_matches = host.objects.iter().all(|(&id, (_, _, meta, content))| {
        dev.phone.object(id).is_some_and(|o| {
            o.complete()
                && &o.meta == meta
                && o.data.get(..o.len as usize) == Some(content.as_slice())
        })
    });
    out.server = host.server.stats();
    out.phone = dev.phone.stats();
    Ok(out)
}
