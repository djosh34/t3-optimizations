//! What one side received: objects, and which packets are stored and can be
//! acked.

use std::collections::BTreeMap;
use std::ops::Range;

use crate::Millis;
use crate::ranges::RangeSet;
use crate::wire::{Ack, Body, Chunk, Key, Report};

/// How many ranges of stored packets an ack carries.
const ACK_RANGES: usize = 16;
/// Stored packet numbers this far below the highest are forgotten.
const STORED_WINDOW: u64 = 4096;

/// A received object. Bytes not received yet are zero; `have` says which
/// bytes are real.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InObject {
    /// 0 until the first chunk.
    pub rev: u64,
    pub meta: Vec<u8>,
    /// Length at `rev`.
    pub len: u64,
    pub have: RangeSet,
    pub data: Vec<u8>,
}

impl InObject {
    pub fn complete(&self) -> bool {
        self.rev > 0 && self.have.covers(0..self.len)
    }

    /// The bytes of a range, if all of them are here.
    pub fn bytes(&self, r: Range<u64>) -> Option<&[u8]> {
        if !self.have.covers(r.clone()) {
            return None;
        }
        self.data.get(r.start as usize..r.end as usize)
    }
}

/// What one chunk changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivered {
    pub key: Key,
    pub rev: u64,
    /// Bytes that were new.
    pub new: Vec<Range<u64>>,
}

#[derive(Debug, Clone)]
struct Pending {
    batch: u64,
    seq: u64,
    at: Millis,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Inbox {
    pub objects: BTreeMap<Key, InObject>,
    stored: RangeSet,
    largest: Option<(u64, Millis)>,
    pending: Vec<Pending>,
    next_batch: u64,
    ack_due: bool,
    counters: Report,
}

impl Inbox {
    /// Applies a chunk. Chunks of an older revision than the one held are
    /// ignored; a newer revision first cuts the held bytes to `keep`.
    /// Returns what changed, or None when nothing did.
    pub fn apply(&mut self, c: Chunk) -> Option<Delivered> {
        let Body::Raw(data) = c.body else {
            // The decoder always hands over raw bytes.
            return None;
        };
        let o = self.objects.entry(c.key).or_default();
        if c.rev < o.rev {
            return None;
        }
        let mut changed = false;
        if c.rev > o.rev {
            let keep = c.keep.unwrap_or(0).min(o.len);
            o.rev = c.rev;
            o.have.truncate(keep);
            o.data.truncate(keep as usize);
            o.len = c.total_len;
            changed = true;
        }
        if let Some(meta) = c.meta
            && meta != o.meta
        {
            o.meta = meta;
            changed = true;
        }
        if c.total_len > o.len {
            o.len = c.total_len;
            changed = true;
        }
        let len = usize::try_from(o.len).ok()?;
        if o.data.len() < len {
            o.data.resize(len, 0);
        }
        let range = c.offset..c.offset + data.len() as u64;
        let new = o.have.gaps(range.clone());
        for r in &new {
            let from = (r.start - c.offset) as usize;
            let to = (r.end - c.offset) as usize;
            let (Some(dst), Some(src)) = (
                o.data.get_mut(r.start as usize..r.end as usize),
                data.get(from..to),
            ) else {
                continue;
            };
            dst.copy_from_slice(src);
        }
        o.have.insert(range);
        (changed || !new.is_empty()).then_some(Delivered {
            key: c.key,
            rev: o.rev,
            new,
        })
    }

    /// Counts a received packet. A packet with chunks waits in a batch until
    /// its content is stored; returns that batch.
    pub fn received(&mut self, seq: u64, bytes: usize, now: Millis, has_chunks: bool) -> u64 {
        self.counters.bytes += bytes as u64;
        self.counters.packets += 1;
        self.counters.max_seq = self.counters.max_seq.max(seq);
        let batch = self.next_batch;
        if has_chunks {
            self.pending.push(Pending {
                batch,
                seq,
                at: now,
            });
            self.next_batch += 1;
        }
        batch
    }

    /// Marks every batch up to `batch` as stored, so their packets get acked.
    pub fn commit(&mut self, batch: u64) {
        let (done, rest): (Vec<_>, Vec<_>) = self.pending.drain(..).partition(|p| p.batch <= batch);
        self.pending = rest;
        for p in done {
            self.stored.insert(p.seq..p.seq + 1);
            if self.largest.is_none_or(|(s, _)| p.seq > s) {
                self.largest = Some((p.seq, p.at));
            }
            self.ack_due = true;
        }
        if let Some(max) = self.stored.max() {
            self.stored.remove_below(max.saturating_sub(STORED_WINDOW));
        }
    }

    /// Whether packets were stored since the last `take_newly_stored`.
    pub fn newly_stored(&self) -> bool {
        self.ack_due
    }

    /// Whether packets were stored since the last call.
    pub fn take_newly_stored(&mut self) -> bool {
        std::mem::take(&mut self.ack_due)
    }

    /// The ack for the next packet, if anything is stored.
    pub fn ack(&mut self, now: Millis) -> Option<Ack> {
        let (_, at) = self.largest?;
        let ranges = self
            .stored
            .ranges()
            .iter()
            .rev()
            .take(ACK_RANGES)
            .cloned()
            .collect();
        Some(Ack {
            delay_ms: now.saturating_sub(at),
            ranges,
        })
    }

    pub fn report(&self, now: Millis) -> Report {
        Report {
            time_ms: now,
            ..self.counters
        }
    }

    /// A new connection: packet numbers start again.
    pub fn new_connection(&mut self) {
        self.stored = RangeSet::new();
        self.largest = None;
        self.pending.clear();
        self.ack_due = false;
        self.counters = Report::default();
    }
}

#[cfg(test)]
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;

    fn chunk(rev: u64, total: u64, offset: u64, keep: Option<u64>, data: &[u8]) -> Chunk {
        Chunk {
            key: Key::Object(1),
            rev,
            total_len: total,
            offset,
            keep,
            meta: Some(b"m".to_vec()),
            body: Body::Raw(data.to_vec()),
        }
    }

    #[test]
    fn newer_revision_replaces_from_keep_and_older_is_ignored() {
        let mut ib = Inbox::default();
        ib.apply(chunk(1, 11, 0, None, b"hello world"));
        // Rev 2 is "hello there": the first 6 bytes stay.
        ib.apply(chunk(2, 11, 6, Some(6), b"there"));
        // A reordered packet of rev 1 comes in late.
        assert_eq!(ib.apply(chunk(1, 11, 6, None, b"world")), None);
        let o = &ib.objects[&Key::Object(1)];
        assert_eq!(o.rev, 2);
        assert_eq!(o.bytes(0..11), Some(&b"hello there"[..]));
        assert!(o.complete());
    }

    #[test]
    fn duplicates_deliver_nothing_new() {
        let mut ib = Inbox::default();
        let d = ib.apply(chunk(1, 10, 5, None, b"world"));
        assert_eq!(d.map(|d| d.new), Some(vec![5..10]));
        assert_eq!(ib.apply(chunk(1, 10, 5, None, b"world")), None);
        let d = ib.apply(chunk(1, 10, 3, None, b"lowor"));
        assert_eq!(d.map(|d| d.new), Some(vec![3..5]));
        assert!(!ib.objects[&Key::Object(1)].complete());
    }

    #[test]
    fn only_stored_packets_are_acked() {
        let mut ib = Inbox::default();
        let b0 = ib.received(0, 100, 10, true);
        let b1 = ib.received(1, 100, 20, true);
        assert_eq!(ib.ack(30), None);
        ib.commit(b0);
        assert_eq!(ib.ack(30).map(|a| a.ranges), Some(vec![0..1]));
        ib.commit(b1);
        let a = ib.ack(50).expect("ack");
        assert_eq!((a.ranges, a.delay_ms), (vec![0..2], 30));
    }
}
