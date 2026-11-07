//! What one side sends: objects, what the peer has stored of them, and which
//! bytes go in the next packet.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use crate::Millis;
use crate::ranges::RangeSet;
use crate::wire::{Body, Chunk, Key, MAX_RAW_CHUNK, chunk_len, compress, varint_len};

/// Priority classes, highest first ([issue #16]). A packet takes from the
/// highest class with data; within a class, the highest `order` goes first.
///
/// [issue #16]: https://github.com/djosh34/t3-optimizations/issues/16
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// The newest text of the last message: its newest bytes first, then its
    /// start.
    Tail,
    /// Results of the phone's commands and replies to calls the owner made.
    /// Small ones ride in every packet until stored. On the phone, its
    /// commands and calls.
    Urgent,
    /// Approvals and thread status.
    Status,
    /// Older messages of the current thread; give newer ones a higher order.
    History,
    ThreadList,
    /// Other threads' content and replies to background calls.
    Background,
    Image,
}

impl Class {
    pub(crate) fn from_u8(v: u8) -> Option<Class> {
        Some(match v {
            0 => Class::Tail,
            1 => Class::Urgent,
            2 => Class::Status,
            3 => Class::History,
            4 => Class::ThreadList,
            5 => Class::Background,
            6 => Class::Image,
            _ => return None,
        })
    }

    /// The classes whose loss costs the owner time: they get repeated copies
    /// sized to the measured loss, instead of waiting for a retransmit.
    fn repeats(self) -> bool {
        self <= Class::Status
    }
}

/// `Urgent` objects up to this size ride in every packet until stored.
pub const RIDE_MAX: u64 = 256;
/// Below this, Brotli does not pay.
const COMPRESS_MIN: usize = 48;
/// Packets not acked after this long are forgotten; their bytes were long
/// since eligible again.
const INFLIGHT_MAX_AGE: Millis = 60_000;

#[derive(Debug, Clone)]
pub(crate) struct OutObject {
    pub class: Class,
    pub order: u64,
    pub meta: Vec<u8>,
    pub rev: u64,
    pub content: Vec<u8>,
    /// Bytes of `rev` the peer has stored.
    pub acked: RangeSet,
    /// The peer has stored a chunk of `rev`, so it has `rev` and `meta`.
    pub rev_acked: bool,
    /// While `!rev_acked`: how many leading bytes of any older revision the
    /// peer may hold are still the same at `rev`.
    pub keep: u64,
    /// Bytes of `rev` sent and not yet acked, sorted and disjoint.
    sent: Vec<Sent>,
    /// When a chunk of `rev` last went out.
    rev_sent_at: Option<Millis>,
}

#[derive(Debug, Clone)]
struct Sent {
    range: Range<u64>,
    at: Millis,
    /// How often these bytes went out.
    count: u32,
}

impl OutObject {
    /// An object as it was saved, with nothing in flight.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restored(
        class: Class,
        order: u64,
        meta: Vec<u8>,
        rev: u64,
        content: Vec<u8>,
        acked: RangeSet,
        rev_acked: bool,
        keep: u64,
    ) -> Self {
        OutObject {
            class,
            order,
            meta,
            rev,
            content,
            acked,
            rev_acked,
            keep,
            sent: Vec::new(),
            rev_sent_at: None,
        }
    }

    /// Whether a chunk of this revision went out on this connection.
    pub fn was_sent(&self) -> bool {
        self.rev_sent_at.is_some()
    }

    fn len(&self) -> u64 {
        self.content.len() as u64
    }

    pub fn done(&self) -> bool {
        self.rev_acked && self.acked.covers(0..self.len())
    }

    fn unacked(&self) -> Vec<Range<u64>> {
        self.acked.gaps(0..self.len())
    }

    /// Unacked bytes that may go out now: never sent, sent longer ago than
    /// `rto`, or, in the repeated classes, sent fewer than `copies` times.
    fn eligible(&self, now: Millis, rto: Millis, copies: u32) -> Vec<Range<u64>> {
        let repeat = self.class.repeats();
        let busy = self
            .sent
            .iter()
            .filter(|s| now < s.at + rto && !(repeat && s.count < copies))
            .map(|s| s.range.clone());
        gaps_of(self.acked.ranges(), busy, 0..self.len())
    }

    fn announce_due(&self, now: Millis, rto: Millis) -> bool {
        !self.rev_acked && self.rev_sent_at.is_none_or(|t| now >= t + rto)
    }

    /// Notes a send. A send after the retransmit timeout starts a new round
    /// of copies.
    fn record_sent(&mut self, range: Range<u64>, now: Millis, rto: Millis) {
        self.rev_sent_at = Some(now);
        if range.is_empty() {
            return;
        }
        let mut count = 0;
        let mut out = Vec::with_capacity(self.sent.len() + 2);
        for s in self.sent.drain(..) {
            if s.range.end <= range.start || s.range.start >= range.end {
                out.push(s);
                continue;
            }
            if now < s.at + rto {
                count = count.max(s.count);
            }
            if s.range.start < range.start {
                out.push(Sent {
                    range: s.range.start..range.start,
                    ..s
                });
            }
            if range.end < s.range.end {
                out.push(Sent {
                    range: range.end..s.range.end,
                    ..s
                });
            }
        }
        out.push(Sent {
            range,
            at: now,
            count: count + 1,
        });
        out.sort_by_key(|s| s.range.start);
        self.sent = out;
    }

    fn on_acked(&mut self, range: Range<u64>) {
        self.rev_acked = true;
        if range.is_empty() {
            return;
        }
        self.acked.insert(range.clone());
        let mut out = Vec::with_capacity(self.sent.len() + 1);
        for s in self.sent.drain(..) {
            if s.range.end <= range.start || s.range.start >= range.end {
                out.push(s);
                continue;
            }
            if s.range.start < range.start {
                out.push(Sent {
                    range: s.range.start..range.start,
                    ..s
                });
            }
            if range.end < s.range.end {
                out.push(Sent {
                    range: range.end..s.range.end,
                    ..s
                });
            }
        }
        self.sent = out;
    }

    /// Forgets what is in flight; the bytes become eligible at once.
    fn forget_sent(&mut self) {
        self.sent.clear();
        self.rev_sent_at = None;
    }

    /// Forgets everything the peer stored.
    fn forget_peer(&mut self) {
        self.forget_sent();
        self.acked = RangeSet::new();
        self.rev_acked = false;
        self.keep = 0;
    }
}

/// The parts of `within` covered by neither `a` (sorted, disjoint) nor `b`
/// (sorted by start).
fn gaps_of(
    a: &[Range<u64>],
    b: impl Iterator<Item = Range<u64>>,
    within: Range<u64>,
) -> Vec<Range<u64>> {
    let mut b = b.peekable();
    let mut a = a.iter().cloned().peekable();
    let mut out = Vec::new();
    let mut at = within.start;
    loop {
        let next = match (a.peek(), b.peek()) {
            (Some(x), Some(y)) if x.start <= y.start => a.next(),
            (Some(_), Some(_)) => b.next(),
            (Some(_), None) => a.next(),
            (None, Some(_)) => b.next(),
            (None, None) => None,
        };
        let Some(r) = next else { break };
        if r.start >= within.end {
            break;
        }
        if r.start > at {
            out.push(at..r.start);
        }
        at = at.max(r.end);
    }
    if at < within.end {
        out.push(at..within.end);
    }
    out
}

/// A sent packet, kept until acked.
#[derive(Debug, Clone)]
struct SentPacket {
    at: Millis,
    chunks: Vec<(Key, u64, Range<u64>)>,
}

/// What an ack changed.
#[derive(Debug, Default)]
pub(crate) struct Acked {
    /// The highest newly acked packet and when it was sent.
    pub largest: Option<(u64, Millis)>,
    /// Objects the peer now has completely.
    pub done: Vec<Key>,
}

/// How one packet is filled.
pub(crate) struct Fill {
    pub now: Millis,
    pub rto: Millis,
    /// Copies of each byte in the repeated classes.
    pub copies: u32,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Outbox {
    objects: BTreeMap<Key, OutObject>,
    /// Send order: class, then highest order first.
    index: BTreeSet<(Class, Reverse<u64>, Key)>,
    inflight: BTreeMap<u64, SentPacket>,
}

impl Outbox {
    pub fn get(&self, key: Key) -> Option<&OutObject> {
        self.objects.get(&key)
    }

    pub fn objects(&self) -> impl Iterator<Item = (&Key, &OutObject)> {
        self.objects.iter()
    }

    /// Sets an object's content. Content that only grows keeps its revision
    /// and the peer gets the new bytes; anything else (a change in the
    /// middle, a shorter text, other meta) is a new revision that keeps the
    /// unchanged leading bytes.
    pub fn set(&mut self, key: Key, class: Class, order: u64, meta: Vec<u8>, content: Vec<u8>) {
        let Some(o) = self.objects.get_mut(&key) else {
            self.index.insert((class, Reverse(order), key));
            self.objects.insert(
                key,
                OutObject {
                    class,
                    order,
                    meta,
                    rev: 1,
                    content,
                    acked: RangeSet::new(),
                    rev_acked: false,
                    keep: 0,
                    sent: Vec::new(),
                    rev_sent_at: None,
                },
            );
            return;
        };
        if (o.class, o.order) != (class, order) {
            self.index.remove(&(o.class, Reverse(o.order), key));
            self.index.insert((class, Reverse(order), key));
            o.class = class;
            o.order = order;
        }
        if o.meta == meta && content.starts_with(&o.content) {
            o.content = content;
            return;
        }
        let prefix = o
            .content
            .iter()
            .zip(&content)
            .take_while(|(a, b)| a == b)
            .count() as u64;
        o.rev += 1;
        o.keep = if o.rev_acked {
            prefix
        } else {
            o.keep.min(prefix)
        };
        o.rev_acked = false;
        o.acked.truncate(prefix);
        o.forget_sent();
        o.meta = meta;
        o.content = content;
    }

    /// Inserts an object as it was saved.
    pub fn restore(&mut self, key: Key, o: OutObject) {
        self.index.insert((o.class, Reverse(o.order), key));
        self.objects.insert(key, o);
    }

    pub fn remove(&mut self, key: Key) {
        if let Some(o) = self.objects.remove(&key) {
            self.index.remove(&(o.class, Reverse(o.order), key));
        }
    }

    pub fn is_synced(&self) -> bool {
        self.objects.values().all(OutObject::done)
    }

    pub fn unsynced_bytes(&self) -> u64 {
        self.objects
            .values()
            .map(|o| o.len() - o.acked.len().min(o.len()))
            .sum()
    }

    pub fn inflight_packets(&self) -> usize {
        self.inflight.len()
    }

    /// A new connection: packets in flight are gone.
    pub fn forget_sent(&mut self) {
        self.inflight.clear();
        for o in self.objects.values_mut() {
            o.forget_sent();
        }
    }

    /// Sends this object again in full.
    pub fn forget_peer_of(&mut self, key: Key) {
        if let Some(o) = self.objects.get_mut(&key) {
            o.forget_peer();
        }
    }

    /// The peer lost everything it stored.
    pub fn forget_peer(&mut self) {
        self.inflight.clear();
        for o in self.objects.values_mut() {
            o.forget_peer();
        }
    }

    pub fn prune(&mut self, now: Millis) {
        let old: Vec<u64> = self
            .inflight
            .iter()
            .take_while(|(_, p)| p.at + INFLIGHT_MAX_AGE < now)
            .map(|(&seq, _)| seq)
            .collect();
        for seq in old {
            self.inflight.remove(&seq);
        }
    }

    pub fn on_ack(&mut self, ranges: &[Range<u64>]) -> Acked {
        let mut acked = Acked::default();
        for r in ranges {
            let seqs: Vec<u64> = self.inflight.range(r.clone()).map(|(&s, _)| s).collect();
            for seq in seqs {
                let Some(p) = self.inflight.remove(&seq) else {
                    continue;
                };
                if acked.largest.is_none_or(|(s, _)| seq > s) {
                    acked.largest = Some((seq, p.at));
                }
                for (key, rev, range) in p.chunks {
                    let Some(o) = self.objects.get_mut(&key) else {
                        continue;
                    };
                    if o.rev != rev {
                        continue;
                    }
                    let was_done = o.done();
                    o.on_acked(range);
                    if !was_done && o.done() {
                        acked.done.push(key);
                    }
                }
            }
        }
        acked
    }

    /// Fills up to `space` bytes of packet `seq` with chunks, in three
    /// passes: small `Urgent` objects that ride in every packet, then bytes
    /// that may go out by priority, then, with space left, any unacked bytes
    /// again, so the sender never idles until the peer has everything.
    /// Copies of the third pass are extra: they do not count as sends, so
    /// they never hold back a retransmit of the second.
    pub fn fill(&mut self, seq: u64, space: usize, f: &Fill, out: &mut Vec<Chunk>) -> usize {
        let mut left = space;
        let mut taken: BTreeMap<Key, RangeSet> = BTreeMap::new();
        let mut announced: BTreeSet<Key> = BTreeSet::new();
        let mut record: Vec<(Key, u64, Range<u64>)> = Vec::new();
        let mut timed: Vec<(Key, Range<u64>)> = Vec::new();
        let keys: Vec<(Class, Key)> = self.index.iter().map(|&(c, _, k)| (c, k)).collect();

        for pass in 0..3 {
            for &(class, key) in &keys {
                if pass == 0 && class != Class::Urgent {
                    continue;
                }
                let Some(o) = self.objects.get(&key) else {
                    continue;
                };
                if o.done() || (pass == 0 && o.len() > RIDE_MAX) {
                    continue;
                }
                let mut ranges = if pass == 1 {
                    o.eligible(f.now, f.rto, f.copies)
                } else {
                    o.unacked()
                };
                let t = taken.entry(key).or_default();
                ranges = ranges.into_iter().flat_map(|r| t.gaps(r)).collect();
                // Every byte is stored, but not the revision itself (a
                // shorter text, new meta, an empty object): an empty chunk
                // tells the peer about it.
                if ranges.is_empty() && o.acked.covers(0..o.len()) {
                    let due = pass != 1 || o.announce_due(f.now, f.rto);
                    if due
                        && !announced.contains(&key)
                        && let Some(c) = make_chunk(key, o, o.len()..o.len(), false, left)
                    {
                        left -= chunk_len(&c);
                        announced.insert(key);
                        record.push((key, o.rev, o.len()..o.len()));
                        if pass < 2 {
                            timed.push((key, o.len()..o.len()));
                        }
                        out.push(c);
                    }
                    continue;
                }
                let from_end = class == Class::Tail;
                if from_end {
                    ranges.reverse();
                }
                for r in ranges {
                    let mut r = r;
                    while !r.is_empty() {
                        let Some(c) = make_chunk(key, o, r.clone(), from_end, left) else {
                            break;
                        };
                        let got = c.offset..c.offset + c.body.raw_len();
                        if got.is_empty() {
                            break;
                        }
                        left -= chunk_len(&c);
                        t.insert(got.clone());
                        record.push((key, o.rev, got.clone()));
                        if pass < 2 {
                            timed.push((key, got.clone()));
                        }
                        out.push(c);
                        if from_end {
                            r.end = got.start;
                        } else {
                            r.start = got.end;
                        }
                    }
                    if left < MIN_CHUNK_SPACE {
                        break;
                    }
                }
                if left < MIN_CHUNK_SPACE {
                    break;
                }
            }
            if left < MIN_CHUNK_SPACE {
                break;
            }
        }

        for (key, range) in timed {
            if let Some(o) = self.objects.get_mut(&key) {
                o.record_sent(range, f.now, f.rto);
            }
        }
        if !record.is_empty() {
            self.inflight.insert(
                seq,
                SentPacket {
                    at: f.now,
                    chunks: record,
                },
            );
        }
        space - left
    }
}

/// A chunk needs at least this much space to be worth starting.
const MIN_CHUNK_SPACE: usize = 24;

/// Builds the largest chunk of `range` that fits in `space` bytes, from its
/// end when `from_end`, else from its start. Text is cut on UTF-8 character
/// boundaries where it can be, so a chunk shows on its own.
fn make_chunk(
    key: Key,
    o: &OutObject,
    range: Range<u64>,
    from_end: bool,
    space: usize,
) -> Option<Chunk> {
    let head = Chunk {
        key,
        rev: o.rev,
        total_len: o.len(),
        offset: range.end,
        keep: (!o.rev_acked && o.rev > 1).then_some(o.keep),
        meta: (!o.rev_acked).then(|| o.meta.clone()),
        body: Body::Raw(Vec::new()),
    };
    // The data length varint and one byte of compressed length grow with
    // the data; allow for both.
    let head_len = chunk_len(&head) + varint_len(space as u64) + 2;
    if range.is_empty() {
        return (chunk_len(&head) <= space).then_some(head);
    }
    if space < head_len + MIN_CHUNK_SPACE / 2 {
        return None;
    }
    let avail = space - head_len;
    let n = (range.end - range.start).min(MAX_RAW_CHUNK) as usize;
    let cut = |len: usize| -> Range<u64> {
        let len = len.min(n) as u64;
        if from_end {
            let start = char_start(&o.content, range.end - len, range.end);
            start..range.end
        } else {
            let end = char_end(&o.content, range.start + len, range.start);
            range.start..end
        }
    };
    let slice = |r: &Range<u64>| o.content.get(r.start as usize..r.end as usize);
    let mut len = n.min(avail.saturating_mul(3));
    for _ in 0..4 {
        let r = cut(len);
        let raw = slice(&r)?;
        let packed = if raw.len() >= COMPRESS_MIN {
            compress(raw)
        } else {
            None
        };
        match packed {
            Some(bytes) if bytes.len() <= avail => {
                return Some(Chunk {
                    offset: r.start,
                    body: Body::Brotli {
                        raw_len: raw.len() as u64,
                        bytes,
                    },
                    ..head
                });
            }
            Some(bytes) => len = len * avail / bytes.len() * 9 / 10,
            None if raw.len() <= avail => {
                return Some(Chunk {
                    offset: r.start,
                    body: Body::Raw(raw.to_vec()),
                    ..head
                });
            }
            None => len = avail,
        }
        if len == 0 {
            return None;
        }
    }
    let r = cut(len.min(avail));
    let raw = slice(&r)?;
    (raw.len() <= avail).then(|| Chunk {
        offset: r.start,
        body: Body::Raw(raw.to_vec()),
        ..head
    })
}

fn is_continuation(content: &[u8], at: u64) -> bool {
    content.get(at as usize).is_some_and(|b| b & 0xc0 == 0x80)
}

/// Moves a chunk start forward to the start of a UTF-8 character, by at most
/// three bytes and never to `end`.
fn char_start(content: &[u8], start: u64, end: u64) -> u64 {
    let mut s = start;
    while s < end && s < start + 3 && is_continuation(content, s) {
        s += 1;
    }
    if s < end && !is_continuation(content, s) {
        s
    } else {
        start
    }
}

/// Moves a chunk end back to the start of a UTF-8 character, by at most
/// three bytes and never to `start`.
fn char_end(content: &[u8], end: u64, start: u64) -> u64 {
    let mut e = end;
    while e > start && e + 3 > end && is_continuation(content, e) {
        e -= 1;
    }
    if e > start && !is_continuation(content, e) {
        e
    } else {
        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaps_of_merges_two_lists() {
        let a = [0..10, 50..60];
        let b = [5..20, 30..40];
        assert_eq!(
            gaps_of(&a, b.into_iter(), 0..100),
            vec![20..30, 40..50, 60..100]
        );
        assert_eq!(gaps_of(&[], std::iter::empty(), 0..5), vec![0..5]);
    }

    #[test]
    fn chunks_are_cut_on_character_boundaries() {
        let text = "aé€😀b".as_bytes(); // 1, 2, 3, 4 and 1 bytes
        assert_eq!(char_start(text, 2, 11), 3);
        assert_eq!(char_start(text, 7, 11), 10);
        assert_eq!(char_end(text, 5, 0), 3);
        assert_eq!(char_end(text, 9, 0), 6);
        assert_eq!(char_start(text, 0, 11), 0);
    }

    #[test]
    fn growth_keeps_the_revision_and_a_change_starts_a_new_one() {
        let mut ob = Outbox::default();
        let k = Key::Object(1);
        ob.set(k, Class::Tail, 0, b"m".to_vec(), b"hello".to_vec());
        ob.set(k, Class::Tail, 0, b"m".to_vec(), b"hello world".to_vec());
        assert_eq!(ob.get(k).map(|o| o.rev), Some(1));
        ob.set(k, Class::Tail, 0, b"m".to_vec(), b"hello there".to_vec());
        let o = ob.get(k).expect("object");
        assert_eq!((o.rev, o.keep), (2, 0));
    }
}
