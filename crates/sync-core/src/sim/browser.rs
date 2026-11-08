//! The browser's SCTP sender on the phone-to-server direction, as the
//! simulator's model of WebKit. The web app hands each sync packet to an
//! unordered data channel with `maxRetransmits: 0`, and WebKit's SCTP
//! (libwebrtc's dcsctp) decides when it goes out: a congestion window that
//! grows on SACKs and drops to one packet on a retransmit timeout, and a
//! timer that backs off. The SACKs travel on the other direction, so a full
//! queue there holds back what the phone sends (issue #46).
//!
//! The server's SCTP (the patched sctp-proto) acks past chunks it lost, so
//! every SACK acks up to the newest TSN it has, with no gap reports; a SACK
//! rides in every packet it sends. Only what decides timing is modelled:
//! the window, the timer and its backoff, abandoning what is in flight on a
//! timeout, and bundling messages into packets. dcsctp's defaults are used
//! where libwebrtc keeps them: retransmit timeout 400 ms to 60 s with an RTT
//! variance of at least 220 ms, a backed-off timer capped at 3 s, and an
//! initial window of 10 packets.

use std::collections::{BTreeMap, VecDeque};

use crate::Millis;

/// IPv4 20, UDP 8, DTLS 37, SCTP common header 12.
pub const PACKET_HEADER: usize = 77;
pub const DATA_CHUNK_HEADER: usize = 16;
/// A SACK chunk without gap reports.
pub const SACK_CHUNK: usize = 16;
/// The largest SCTP packet on the wire.
const MTU: usize = 1200;
const RTO_MIN: f64 = 400.0;
const RTO_MAX: f64 = 60_000.0;
const MIN_RTT_VARIANCE: f64 = 220.0;
const MAX_BACKOFF: f64 = 3_000.0;
/// The phone's SCTP acks every second DATA packet, or 200 ms after one.
const DELAYED_ACK: Millis = 200;

#[derive(Debug)]
struct Outstanding {
    sent_at: Millis,
    bytes: usize,
}

#[derive(Debug)]
pub struct BrowserSctp {
    /// Messages handed to the data channel and not sent yet.
    queue: VecDeque<Vec<u8>>,
    queued_bytes: usize,
    outstanding: BTreeMap<u64, Outstanding>,
    flight: usize,
    next_tsn: u64,
    cwnd: usize,
    ssthresh: usize,
    srtt: Option<f64>,
    rttvar: f64,
    rto: f64,
    /// Timeouts since the last new ack.
    backoff: u32,
    t3: Option<Millis>,
    /// DATA packets from the server not SACKed yet, and when the first
    /// arrived.
    unacked_data: u32,
    unacked_since: Option<Millis>,
}

impl Default for BrowserSctp {
    fn default() -> Self {
        BrowserSctp {
            queue: VecDeque::new(),
            queued_bytes: 0,
            outstanding: BTreeMap::new(),
            flight: 0,
            next_tsn: 0,
            cwnd: 10 * MTU,
            ssthresh: usize::MAX,
            srtt: None,
            rttvar: 0.0,
            rto: 500.0,
            backoff: 0,
            t3: None,
            unacked_data: 0,
            unacked_since: None,
        }
    }
}

/// What the phone's SCTP sends in one packet.
#[derive(Debug, Clone, Default)]
pub struct SctpPacket {
    pub messages: Vec<(u64, Vec<u8>)>,
    pub sack: bool,
}

impl SctpPacket {
    pub fn wire_len(&self) -> usize {
        let data: usize = self
            .messages
            .iter()
            .map(|(_, m)| DATA_CHUNK_HEADER + m.len())
            .sum();
        PACKET_HEADER + data + if self.sack { SACK_CHUNK } else { 0 }
    }
}

impl BrowserSctp {
    /// The data channel's `bufferedAmount`: bytes handed over and not sent.
    pub fn buffered(&self) -> usize {
        self.queued_bytes
    }

    pub fn send(&mut self, message: Vec<u8>) {
        self.queued_bytes += message.len();
        self.queue.push_back(message);
    }

    /// A DATA packet from the server arrived.
    pub fn on_server_data(&mut self, now: Millis) {
        self.unacked_data += 1;
        self.unacked_since.get_or_insert(now);
    }

    /// The next packet to send now, if any: queued messages the window
    /// allows, with a SACK when one is due.
    pub fn poll(&mut self, now: Millis) -> Option<SctpPacket> {
        self.expire(now);
        let mut p = SctpPacket::default();
        let mut size = PACKET_HEADER;
        while let Some(m) = self.queue.front() {
            let len = DATA_CHUNK_HEADER + m.len();
            if self.flight >= self.cwnd || size + len > MTU {
                break;
            }
            let Some(m) = self.queue.pop_front() else {
                break;
            };
            self.queued_bytes -= m.len();
            size += len;
            self.flight += len;
            let tsn = self.next_tsn;
            self.next_tsn += 1;
            self.outstanding.insert(
                tsn,
                Outstanding {
                    sent_at: now,
                    bytes: len,
                },
            );
            p.messages.push((tsn, m));
        }
        let sack_due =
            self.unacked_data >= 2 || self.unacked_since.is_some_and(|t| now >= t + DELAYED_ACK);
        if sack_due || (self.unacked_data > 0 && !p.messages.is_empty()) {
            p.sack = true;
            self.unacked_data = 0;
            self.unacked_since = None;
        }
        if !p.messages.is_empty() && self.t3.is_none() {
            self.t3 = Some(now + self.timer() as Millis);
        }
        (p.sack || !p.messages.is_empty()).then_some(p)
    }

    /// When `poll` may have something new without a packet arriving.
    pub fn next_wakeup(&self) -> Option<Millis> {
        let ack = self.unacked_since.map(|t| t + DELAYED_ACK);
        [self.t3, ack].into_iter().flatten().min()
    }

    fn timer(&self) -> f64 {
        let mut d = self.rto;
        for _ in 0..self.backoff {
            d *= 2.0;
            if d > MAX_BACKOFF {
                return MAX_BACKOFF;
            }
        }
        d
    }

    /// A retransmit timeout: with `maxRetransmits: 0` everything in flight
    /// is abandoned, and the window drops to one packet.
    fn expire(&mut self, now: Millis) {
        if self.t3.is_none_or(|t| now < t) {
            return;
        }
        self.outstanding.clear();
        self.flight = 0;
        self.ssthresh = (self.cwnd / 2).max(4 * MTU);
        self.cwnd = MTU;
        self.backoff += 1;
        self.t3 = None;
    }

    /// A SACK from the server: it acks every TSN up to `cumulative`.
    pub fn on_sack(&mut self, now: Millis, cumulative: u64) {
        self.expire(now);
        let acked: Vec<u64> = self
            .outstanding
            .range(..=cumulative)
            .map(|(&t, _)| t)
            .collect();
        let Some(&top) = acked.last() else {
            return;
        };
        let sent_at = self.outstanding.get(&top).map_or(now, |o| o.sent_at);
        let mut acked_bytes = 0;
        for tsn in acked {
            if let Some(o) = self.outstanding.remove(&tsn) {
                self.flight -= o.bytes;
                acked_bytes += o.bytes;
            }
        }
        self.on_rtt(now.saturating_sub(sent_at) as f64);
        self.backoff = 0;
        if self.cwnd <= self.ssthresh {
            self.cwnd += acked_bytes.min(MTU);
        } else {
            self.cwnd += MTU * acked_bytes / self.cwnd.max(1);
        }
        self.t3 = (!self.outstanding.is_empty()).then(|| now + self.timer() as Millis);
    }

    fn on_rtt(&mut self, r: f64) {
        match self.srtt {
            None => {
                self.srtt = Some(r);
                self.rttvar = r / 2.0;
            }
            Some(s) => {
                self.rttvar = 0.75 * self.rttvar + 0.25 * (s - r).abs();
                self.srtt = Some(0.875 * s + 0.125 * r);
            }
        }
        let s = self.srtt.unwrap_or(r);
        self.rto = (s + 4.0 * self.rttvar.max(MIN_RTT_VARIANCE)).clamp(RTO_MIN, RTO_MAX);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_timeout_abandons_what_is_in_flight_and_backs_off_up_to_3_s() {
        let mut b = BrowserSctp::default();
        b.send(vec![0; 500]);
        let p = b.poll(0).expect("a packet");
        assert_eq!(p.messages.len(), 1);
        let mut t = 0;
        let mut gaps = Vec::new();
        for _ in 0..6 {
            b.send(vec![0; 500]);
            let next = b.next_wakeup().expect("a timer");
            gaps.push(next - t);
            t = next;
            assert!(b.poll(t).is_some());
        }
        assert_eq!(gaps, vec![500, 1000, 2000, 3000, 3000, 3000]);
        assert_eq!(b.cwnd, MTU);
    }

    #[test]
    fn a_sack_frees_the_window_and_resets_the_backoff() {
        let mut b = BrowserSctp::default();
        for _ in 0..30 {
            b.send(vec![0; 1000]);
        }
        let mut sent = Vec::new();
        while let Some(p) = b.poll(0) {
            sent.extend(p.messages.iter().map(|(t, _)| *t));
        }
        // A chunk goes out while less than the window (10 packets of 1200
        // bytes) is in flight: 12 chunks of 1016 bytes.
        assert_eq!(sent.len(), 12);
        b.on_sack(400, sent.last().copied().unwrap_or(0));
        assert_eq!(b.flight, 0);
        assert_eq!(b.backoff, 0);
        assert!(b.poll(400).is_some());
    }
}
