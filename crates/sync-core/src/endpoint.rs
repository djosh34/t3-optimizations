//! What the phone and the server share: packet numbers, acks, reports, rate
//! and the decision when a packet goes out.

use crate::inbox::Inbox;
use crate::outbox::{Acked, Fill, Outbox};
use crate::rate::{RateConfig, RateControl};
use crate::wire::{DecodeError, Hello, Packet};
use crate::{MAX_PACKET, Millis, WIRE_OVERHEAD};

/// Settings of one side.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub rate: RateConfig,
    /// With nothing else to send, an alive packet goes out this often.
    pub alive_interval: Millis,
    /// Stored packets are acked at most this often by a packet of its own;
    /// every packet that goes out anyway carries the ack too.
    pub ack_interval: Millis,
    /// The other side's `ack_interval`, part of the retransmit timeout.
    pub peer_ack_interval: Millis,
    /// Receive counters for the other side's rate control go out this
    /// often.
    pub report_interval: Millis,
    /// A packet holds about this many milliseconds of the send rate, between
    /// 200 bytes and `MAX_PACKET`. Smaller packets let a new urgent packet
    /// overtake sooner on a slow link.
    pub packet_time: Millis,
}

impl Config {
    pub fn server() -> Self {
        Config {
            rate: RateConfig::default(),
            alive_interval: 1_000,
            ack_interval: 100,
            peer_ack_interval: 200,
            report_interval: 500,
            packet_time: 100,
        }
    }

    /// The phone sends few packets, mostly acks; each carries a report, so
    /// the server learns the loss soon.
    pub fn phone() -> Self {
        Config {
            alive_interval: 2_000,
            ack_interval: 200,
            peer_ack_interval: 100,
            report_interval: 0,
            ..Config::server()
        }
    }
}

/// While the phone waits for the server's first packet on a connection, it
/// sends its hello this often.
const HELLO_INTERVAL: Millis = 200;
const MIN_PACKET: usize = 200;

/// Counters and estimates of one side, for the status line and reports.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stats {
    /// The send rate, wire bytes per second.
    pub send_rate: f64,
    /// What the other side reports receiving, wire bytes per second.
    pub delivered_rate: f64,
    /// Fraction of packets lost on the way to the other side.
    pub loss: f64,
    pub srtt_ms: Option<f64>,
    pub min_rtt_ms: Option<f64>,
    /// Packets and bytes, with the transport overhead counted per packet.
    pub sent_packets: u64,
    pub sent_wire_bytes: u64,
    pub received_packets: u64,
    pub received_wire_bytes: u64,
    /// When the last packet from the other side came in.
    pub last_heard: Option<Millis>,
    /// Bytes the other side has not stored yet.
    pub unsynced_bytes: u64,
    pub inflight_packets: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Endpoint {
    pub out: Outbox,
    pub inbox: Inbox,
    rate: RateControl,
    cfg: Config,
    next_seq: u64,
    last_sent: Option<Millis>,
    last_ack: Option<Millis>,
    last_report: Option<Millis>,
    /// Packets of their own that still carry the latest ack. An ack goes
    /// out as often as the repeated classes' data does, so one usually
    /// gets through at the measured loss.
    ack_sends_left: u32,
    stats: Stats,
}

impl Endpoint {
    pub fn new(cfg: Config, now: Millis) -> Self {
        Endpoint {
            out: Outbox::default(),
            inbox: Inbox::default(),
            rate: RateControl::new(cfg.rate, now),
            cfg,
            next_seq: 0,
            last_sent: None,
            last_ack: None,
            last_report: None,
            ack_sends_left: 0,
            stats: Stats::default(),
        }
    }

    /// A new connection: packet numbers start at 0 on both sides, and what
    /// was in flight is gone.
    pub fn new_connection(&mut self) {
        self.next_seq = 0;
        self.out.forget_sent();
        self.inbox.new_connection();
        self.rate.reset_reports();
    }

    /// Takes the ack and report of a decoded packet and counts it. Returns
    /// the batch its content belongs to, and what the ack changed.
    pub fn take(&mut self, now: Millis, p: &Packet, len: usize) -> (u64, Acked) {
        self.stats.received_packets += 1;
        self.stats.received_wire_bytes += (len + WIRE_OVERHEAD) as u64;
        self.stats.last_heard = Some(now);
        let mut acked = Acked::default();
        if let Some(a) = &p.ack {
            acked = self.out.on_ack(&a.ranges);
            // Only the highest acked packet gives an RTT sample, and only
            // the first time it is acked: the ack delay is its delay.
            let top = a.ranges.first().map(|r| r.end - 1);
            if let Some((seq, at)) = acked.largest
                && Some(seq) == top
            {
                let total = now.saturating_sub(at);
                let sample = total.saturating_sub(a.delay_ms.min(total));
                self.rate.on_rtt(now, sample as f64, at);
            }
        }
        if let Some(r) = p.report {
            self.rate.on_report(r);
        }
        let batch = self.inbox.received(p.seq, len, now, !p.chunks.is_empty());
        (batch, acked)
    }

    /// Packet size follows what the link delivers, not the send rate: on a
    /// slow link that queues whole packets, small ones wait less.
    fn packet_target(&self) -> usize {
        let delivered = self.rate.delivered();
        let rate = if delivered > 0.0 {
            delivered.min(self.rate.rate())
        } else {
            self.rate.rate()
        };
        let bytes = rate * self.cfg.packet_time as f64 / 1000.0;
        (bytes as usize)
            .saturating_sub(WIRE_OVERHEAD)
            .clamp(MIN_PACKET, MAX_PACKET)
    }

    /// Copies of each byte in the repeated classes, so that at the measured
    /// loss at least 90% of them arrive without waiting for a retransmit.
    fn copies(&self) -> u32 {
        let loss = self.rate.loss();
        if loss < 0.05 {
            return 1;
        }
        let n = (0.1f64.ln() / loss.min(0.95).ln()).ceil();
        (n as u32).clamp(1, 6)
    }

    /// The next packet, if one should go out now.
    pub fn poll(
        &mut self,
        now: Millis,
        hello: Option<Hello>,
        log_low: Option<u64>,
    ) -> Option<Vec<u8>> {
        self.rate.decide(now);
        self.out.prune(now);
        let unsynced = !self.out.is_synced();
        if !self.rate.can_send(now) {
            if unsynced {
                self.rate.note_limited(now);
            }
            return None;
        }
        let since = |t: Option<Millis>, every: Millis| t.is_none_or(|t| now >= t + every);
        if self.inbox.take_newly_stored() {
            self.ack_sends_left = self.copies();
        }
        let alive_due = since(self.last_sent, self.cfg.alive_interval);
        let ack_due = self.ack_sends_left > 0 && since(self.last_ack, self.cfg.ack_interval);
        let hello_due = hello.is_some() && since(self.last_sent, HELLO_INTERVAL);
        if !(unsynced || alive_due || ack_due || hello_due) {
            return None;
        }
        let report_due = since(self.last_report, self.cfg.report_interval);
        let mut p = Packet {
            seq: self.next_seq,
            hello,
            ack: self.inbox.ack(now),
            report: report_due.then(|| self.inbox.report(now)),
            log_low,
            chunks: Vec::new(),
        };
        if unsynced {
            self.rate.note_limited(now);
            let head = p.encode().len();
            let fill = Fill {
                now,
                rto: self.rate.rto(self.cfg.peer_ack_interval),
                fast_rto: self.rate.fast_rto(self.cfg.peer_ack_interval),
                copies: self.copies(),
            };
            let space = self.packet_target().saturating_sub(head);
            let max_space = MAX_PACKET.saturating_sub(head);
            self.out.fill(p.seq, space, max_space, &fill, &mut p.chunks);
        }
        let bytes = p.encode();
        self.next_seq += 1;
        self.last_sent = Some(now);
        if p.ack.is_some() {
            self.last_ack = Some(now);
            self.ack_sends_left = self.ack_sends_left.saturating_sub(1);
        }
        if report_due {
            self.last_report = Some(now);
        }
        self.rate.on_sent(bytes.len() + WIRE_OVERHEAD);
        self.stats.sent_packets += 1;
        self.stats.sent_wire_bytes += (bytes.len() + WIRE_OVERHEAD) as u64;
        Some(bytes)
    }

    /// When `poll` should be called next.
    pub fn next_wakeup(&self, now: Millis, hello: bool) -> Millis {
        let after = |t: Option<Millis>, every: Millis| t.map_or(now, |t| t + every);
        let send = self.rate.next_send(now);
        let mut at = self.rate.next_decision();
        at = at.min(after(self.last_sent, self.cfg.alive_interval).max(send));
        if !self.out.is_synced() {
            at = at.min(send);
        }
        if self.ack_sends_left > 0 || self.inbox.newly_stored() {
            at = at.min(after(self.last_ack, self.cfg.ack_interval).max(send));
        }
        if hello {
            at = at.min(after(self.last_sent, HELLO_INTERVAL).max(send));
        }
        at.max(now)
    }

    pub fn stats(&self) -> Stats {
        Stats {
            send_rate: self.rate.rate(),
            delivered_rate: self.rate.delivered(),
            loss: self.rate.loss(),
            srtt_ms: self.rate.srtt(),
            min_rtt_ms: self.rate.min_rtt(),
            unsynced_bytes: self.out.unsynced_bytes(),
            inflight_packets: self.out.inflight_packets(),
            ..self.stats.clone()
        }
    }
}

pub(crate) fn decode(bytes: &[u8]) -> Result<Packet, DecodeError> {
    if bytes.len() > MAX_PACKET {
        return Err(DecodeError::Invalid("packet size"));
    }
    Packet::decode(bytes)
}
