//! How fast a side sends (issue #25): pace at a send rate, raise it while
//! round-trip time stays flat, lower it only when round-trip time grows.
//! Loss never lowers it. What arrived at the other side is measured from its
//! reports and kept apart, for the status line and the loss estimate.

use std::collections::VecDeque;

use crate::Millis;
use crate::wire::Report;

/// Settings of the rate control.
#[derive(Debug, Clone, Copy)]
pub struct RateConfig {
    /// Send rate at the start, in wire bytes per second.
    pub initial: f64,
    /// The rate never goes below this, so alive signals and acks always go
    /// out.
    pub floor: f64,
    pub ceiling: f64,
}

impl Default for RateConfig {
    fn default() -> Self {
        RateConfig {
            initial: 2_000.0,
            floor: 150.0,
            ceiling: 4_000_000.0,
        }
    }
}

/// The base round-trip time is the lowest sample of this window. It is long,
/// so a queue that stays full does not become the base; a path whose RTT
/// really grew is found by draining instead (see `decide`).
const MIN_RTT_WINDOW: Millis = 60_000;
/// Drains down to this fraction of the rate they started from, with the RTT
/// still up, mean the path changed.
const PATH_CHANGE_RATE: f64 = 0.3;
/// RTT counts as grown when every sample since the last decision is this
/// much above the base: a quarter more, plus a margin for jitter of at
/// least 150 ms, or three times the measured jitter.
const GROWTH_FACTOR: f64 = 1.25;
const GROWTH_MARGIN_MS: f64 = 150.0;
const JITTER_MARGINS: f64 = 3.0;
/// A decision needs this many RTT samples: the lowest of three rarely sits
/// in the jitter's upper tail.
const DECISION_SAMPLES: u32 = 3;
/// On RTT growth the rate goes to this fraction of the send capacity or of
/// itself, whichever is lower, but never below half of what it was.
const DRAIN: f64 = 0.75;
/// Increase per decision while RTT stays flat, and the most the rate may be
/// above the measured send capacity: half again at the start, then probing a
/// quarter above.
const SLOW_START_INCREASE: f64 = 1.5;
const PROBE_INCREASE: f64 = 1.25;
/// A sender that had nothing waiting for this long starts again from the
/// initial rate: what it knew about the link may be stale.
const IDLE_RESTART: Millis = 2_000;
/// Reports at least this far apart, and this many packets sent, give one
/// sample of delivery rate and loss.
const REPORT_SPAN: Millis = 1_000;
const REPORT_PACKETS: u64 = 10;
/// Weight of a new sample in the averages of delivery rate and loss.
const REPORT_WEIGHT: f64 = 0.3;

#[derive(Debug, Clone)]
pub struct RateControl {
    cfg: RateConfig,
    rate: f64,
    tokens: f64,
    last_refill: Millis,
    slow_start: bool,
    /// Whether the sender had data waiting on the rate since the last
    /// decision. Only then is a higher rate worth probing.
    limited: bool,
    last_limited: Option<Millis>,
    next_decision: Millis,
    srtt: Option<f64>,
    rttvar: f64,
    rtt_window: VecDeque<(Millis, f64)>,
    /// Average difference between consecutive RTT samples. A queue grows
    /// slowly from sample to sample, so this measures the jitter alone.
    jitter: f64,
    last_rtt: Option<f64>,
    /// The lowest RTT sample, and the number of samples, since the last
    /// decision.
    fresh_min: Option<f64>,
    fresh_count: u32,
    /// When the rate last changed. Only packets sent after it show what the
    /// new rate does.
    changed_at: Millis,
    /// The rate when the current run of drains started, and the RTT at the
    /// last drain.
    drain_from: Option<f64>,
    drain_rtt: Option<f64>,
    /// The peer's report that starts the current delivery sample.
    report_base: Option<Report>,
    delivered: f64,
    loss: f64,
}

/// Bytes of a full packet on the wire; the token bucket holds at most this
/// much, so the sender never bursts more than one packet ahead.
const BUCKET: f64 = (crate::MAX_PACKET + crate::WIRE_OVERHEAD) as f64;

impl RateControl {
    pub fn new(cfg: RateConfig, now: Millis) -> Self {
        RateControl {
            cfg,
            rate: cfg.initial.clamp(cfg.floor, cfg.ceiling),
            tokens: BUCKET,
            last_refill: now,
            slow_start: true,
            limited: false,
            last_limited: None,
            next_decision: now,
            srtt: None,
            rttvar: 0.0,
            rtt_window: VecDeque::new(),
            jitter: 0.0,
            last_rtt: None,
            fresh_min: None,
            fresh_count: 0,
            changed_at: now,
            drain_from: None,
            drain_rtt: None,
            report_base: None,
            delivered: 0.0,
            loss: 0.0,
        }
    }

    /// The send rate, wire bytes per second.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// What the peer reported receiving, wire bytes per second.
    pub fn delivered(&self) -> f64 {
        self.delivered
    }

    /// Fraction of packets lost on the way to the peer.
    pub fn loss(&self) -> f64 {
        self.loss
    }

    /// What the sender can send without the queue growing: what arrived,
    /// plus what was lost on the way. Loss does not lower it; it is kept
    /// apart from what arrived. None before the first delivery sample.
    pub fn capacity(&self) -> Option<f64> {
        (self.delivered > 0.0).then(|| self.delivered / (1.0 - self.loss.min(0.9)))
    }

    pub fn srtt(&self) -> Option<f64> {
        self.srtt
    }

    pub fn min_rtt(&self) -> Option<f64> {
        self.rtt_window.iter().map(|&(_, r)| r).reduce(f64::min)
    }

    /// Retransmit timeout: a resend of data that was not acked waits this
    /// long. The peer's ack interval is added, because acks are not sent
    /// for every packet.
    pub fn rto(&self, peer_ack_interval: Millis) -> Millis {
        match self.srtt {
            None => 1_000,
            Some(s) => {
                let v = s + (4.0 * self.rttvar).max(50.0) + peer_ack_interval as f64;
                (v as Millis).clamp(200, 3_000)
            }
        }
    }

    fn refill(&mut self, now: Millis) {
        let dt = now.saturating_sub(self.last_refill) as f64 / 1000.0;
        self.last_refill = now.max(self.last_refill);
        self.tokens = (self.tokens + dt * self.rate).min(BUCKET);
    }

    /// Whether a packet may go out now.
    pub fn can_send(&mut self, now: Millis) -> bool {
        self.refill(now);
        self.tokens >= 0.0
    }

    /// Notes that data waited on the rate. After an idle spell the rate
    /// starts again from the initial one, in slow start.
    pub fn note_limited(&mut self, now: Millis) {
        if self.last_limited.is_some_and(|t| now >= t + IDLE_RESTART) {
            self.rate = self.rate.min(self.cfg.initial.max(self.cfg.floor));
            self.slow_start = true;
            self.changed_at = now;
            // The delivery rate measured before the idle spell may be stale
            // too. The loss estimate stays: it is about the path, and too
            // low an estimate costs the repeated classes their copies.
            self.delivered = 0.0;
            self.report_base = None;
        }
        self.last_limited = Some(now);
        self.limited = true;
    }

    pub fn on_sent(&mut self, wire_bytes: usize) {
        self.tokens -= wire_bytes as f64;
    }

    /// When the next packet may go out.
    pub fn next_send(&self, now: Millis) -> Millis {
        let dt = now.saturating_sub(self.last_refill) as f64 / 1000.0;
        let tokens = (self.tokens + dt * self.rate).min(BUCKET);
        if tokens >= 0.0 {
            now
        } else {
            now + (-tokens / self.rate * 1000.0).ceil() as Millis
        }
    }

    pub fn next_decision(&self) -> Millis {
        self.next_decision
    }

    /// Takes an RTT sample of a packet sent at `sent_at`.
    pub fn on_rtt(&mut self, now: Millis, sample: f64, sent_at: Millis) {
        let sample = sample.max(1.0);
        match self.srtt {
            None => {
                self.srtt = Some(sample);
                self.rttvar = sample / 2.0;
            }
            Some(s) => {
                self.rttvar = 0.75 * self.rttvar + 0.25 * (s - sample).abs();
                self.srtt = Some(0.875 * s + 0.125 * sample);
            }
        }
        if let Some(prev) = self.last_rtt {
            self.jitter = 0.9 * self.jitter + 0.1 * (sample - prev).abs();
        }
        self.last_rtt = Some(sample);
        if sent_at >= self.changed_at {
            self.fresh_min = Some(self.fresh_min.map_or(sample, |m| m.min(sample)));
            self.fresh_count += 1;
        }
        self.rtt_window.push_back((now, sample));
        while self
            .rtt_window
            .front()
            .is_some_and(|&(t, _)| t + MIN_RTT_WINDOW < now)
        {
            self.rtt_window.pop_front();
        }
    }

    /// Takes the peer's cumulative counters. Packet numbers count from 0 on
    /// every connection, so the change of `max_seq` between two reports is
    /// how many packets were sent, and the change of `packets` how many
    /// arrived.
    pub fn on_report(&mut self, r: Report) {
        let Some(base) = self.report_base else {
            self.report_base = Some(r);
            return;
        };
        if r.time_ms < base.time_ms + REPORT_SPAN || r.max_seq < base.max_seq + REPORT_PACKETS {
            return;
        }
        let secs = (r.time_ms - base.time_ms) as f64 / 1000.0;
        let packets = r.packets.saturating_sub(base.packets) as f64;
        let wire =
            r.bytes.saturating_sub(base.bytes) as f64 + packets * crate::WIRE_OVERHEAD as f64;
        let delivered = wire / secs;
        let sent = (r.max_seq - base.max_seq) as f64;
        let loss = (1.0 - packets / sent).clamp(0.0, 1.0);
        if self.delivered == 0.0 {
            self.delivered = delivered;
            self.loss = loss;
        } else {
            let w = REPORT_WEIGHT;
            self.delivered = (1.0 - w) * self.delivered + w * delivered;
            self.loss = (1.0 - w) * self.loss + w * loss;
        }
        self.report_base = Some(r);
    }

    /// Forgets the peer's counters, which start again on a new connection.
    pub fn reset_reports(&mut self) {
        self.report_base = None;
    }

    /// Raises or lowers the rate, about once per base round trip. RTT
    /// growth lowers it; with fresh samples and no growth, a sender that
    /// had data waiting raises it; with no samples (an outage, or every
    /// ack lost) it stays.
    pub fn decide(&mut self, now: Millis) {
        if now < self.next_decision {
            return;
        }
        let base = self.min_rtt();
        let interval = base.unwrap_or(500.0).max(300.0);
        if self.fresh_count < DECISION_SAMPLES {
            // Too little feedback (an outage, or every ack lost): wait.
            self.next_decision = now + interval as Millis;
            return;
        }
        self.fresh_count = 0;
        let fresh = self.fresh_min.take();
        let margin = GROWTH_MARGIN_MS.max(JITTER_MARGINS * self.jitter);
        let grown = match (fresh, base) {
            (Some(f), Some(b)) => f > b * GROWTH_FACTOR + margin,
            _ => false,
        };
        // The rate is down to a fraction of where the drains started (or at
        // the floor), and the last drain did not bring the RTT down by a
        // tenth.
        let drained = self
            .drain_from
            .is_some_and(|r| self.rate <= (PATH_CHANGE_RATE * r).max(self.cfg.floor));
        let stuck = grown && drained && fresh >= self.drain_rtt.map(|r| 0.9 * r);
        if stuck {
            // No queue of ours explains an RTT that stays up at a fraction
            // of the rate: the path itself got slower. Its RTT is the new
            // base, and the rate grows again from here.
            let rtt = fresh.unwrap_or(interval);
            self.rtt_window.clear();
            self.rtt_window.push_back((now, rtt));
            self.drain_from = None;
            self.drain_rtt = None;
            self.slow_start = true;
            self.changed_at = now;
            self.next_decision = now + interval as Millis;
        } else if grown {
            self.drain_from.get_or_insert(self.rate);
            self.drain_rtt = fresh;
            // Below what the link takes, so the queue drains.
            let target = match self.capacity() {
                Some(c) => DRAIN * c.min(self.rate),
                None => DRAIN * self.rate,
            };
            self.rate = target.max(self.rate / 2.0).max(self.cfg.floor);
            self.changed_at = now;
            self.slow_start = false;
            self.next_decision = now + interval as Millis;
        } else {
            self.drain_from = None;
            if fresh.is_some() && self.limited {
                let up = if self.slow_start {
                    SLOW_START_INCREASE
                } else {
                    PROBE_INCREASE
                };
                // Never more than `up` above what the link was seen to take;
                // a capacity below the rate (more loss) keeps the rate.
                let mut rate = self.rate * up;
                if let Some(c) = self.capacity() {
                    rate = rate.min((c * up).max(self.rate));
                }
                self.rate = rate.min(self.cfg.ceiling);
                self.changed_at = now;
            }
            self.next_decision = now + interval as Millis;
        }
        self.limited = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loss_alone_never_lowers_the_rate() {
        let mut rc = RateControl::new(RateConfig::default(), 0);
        let start = rc.rate();
        let mut t = 0;
        let mut sent = 0;
        let mut got = 0;
        for i in 0..200 {
            t += 500;
            rc.on_rtt(t, 600.0 + (i % 3) as f64 * 10.0, t);
            sent += 10;
            got += 3; // 70% loss
            rc.on_report(Report {
                time_ms: t,
                bytes: got * 500,
                packets: got,
                max_seq: sent,
            });
            rc.note_limited(t);
            rc.decide(t);
        }
        assert!(rc.rate() > start, "rate {} fell below {start}", rc.rate());
        assert!((rc.loss() - 0.7).abs() < 0.05, "loss {}", rc.loss());
    }

    /// A link that takes `cap` bytes/s and queues the rest: RTT is the base
    /// plus the queue's delay. Returns the rates of the last minute and the
    /// largest queue delay of the last minute.
    fn run_queue(cap: f64, base: f64, secs: u64) -> (Vec<f64>, f64) {
        let mut rc = RateControl::new(RateConfig::default(), 0);
        let mut queue = 0.0; // bytes
        let mut rates = Vec::new();
        let mut worst = 0.0f64;
        for t in (0..secs * 1000).step_by(50) {
            queue = (queue + (rc.rate() - cap) * 0.05).max(0.0);
            let delay = queue / cap * 1000.0;
            rc.on_rtt(t, base + delay, t);
            rc.note_limited(t);
            rc.decide(t);
            if t >= (secs - 60) * 1000 {
                rates.push(rc.rate());
                worst = worst.max(delay);
            }
        }
        (rates, worst)
    }

    #[test]
    fn rate_settles_near_what_the_link_takes_with_a_short_queue() {
        for (cap, base) in [(1_250.0, 600.0), (12_500.0, 600.0), (200_000.0, 40.0)] {
            let (rates, worst) = run_queue(cap, base, 300);
            let mean = rates.iter().sum::<f64>() / rates.len() as f64;
            assert!(
                mean > 0.6 * cap && mean < 1.3 * cap,
                "cap {cap}: mean rate {mean}"
            );
            assert!(worst < 3_000.0, "cap {cap}: queue delay up to {worst} ms");
        }
    }

    #[test]
    fn a_slower_path_becomes_the_new_base() {
        let mut rc = RateControl::new(RateConfig::default(), 0);
        let mut t = 0;
        for _ in 0..100 {
            t += 50;
            rc.on_rtt(t, 40.0, t);
            rc.note_limited(t);
            rc.decide(t);
        }
        // The path's RTT goes from 40 ms to 600 ms; no queue of ours.
        for _ in 0..600 {
            t += 50;
            rc.on_rtt(t, 600.0, t);
            rc.note_limited(t);
            rc.decide(t);
        }
        assert_eq!(rc.min_rtt(), Some(600.0));
        assert!(
            rc.rate() > RateConfig::default().initial,
            "rate {}",
            rc.rate()
        );
    }

    #[test]
    fn pacing_spends_tokens() {
        let mut rc = RateControl::new(
            RateConfig {
                initial: 1000.0,
                ..RateConfig::default()
            },
            0,
        );
        assert!(rc.can_send(0));
        rc.on_sent(BUCKET as usize + 500);
        assert!(!rc.can_send(0));
        assert_eq!(rc.next_send(0), 500);
        assert!(rc.can_send(500));
    }
}
