//! Sets of byte offsets (or packet numbers) as sorted, disjoint half-open
//! ranges.

use std::ops::Range;

/// A set of `u64`s, kept as sorted, disjoint, non-adjacent ranges.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RangeSet {
    ranges: Vec<Range<u64>>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// The ranges, lowest first.
    pub fn ranges(&self) -> &[Range<u64>] {
        &self.ranges
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// How many values the set holds.
    pub fn len(&self) -> u64 {
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }

    /// The highest value in the set.
    pub fn max(&self) -> Option<u64> {
        self.ranges.last().map(|r| r.end - 1)
    }

    pub fn contains(&self, v: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&v))
    }

    /// Whether the set holds every value of `r`.
    pub fn covers(&self, r: Range<u64>) -> bool {
        r.is_empty()
            || self
                .ranges
                .iter()
                .any(|x| x.start <= r.start && r.end <= x.end)
    }

    pub fn insert(&mut self, r: Range<u64>) {
        if r.is_empty() {
            return;
        }
        let mut start = r.start;
        let mut end = r.end;
        // Merge every range that overlaps or touches the new one.
        self.ranges.retain(|x| {
            if x.end < start || x.start > end {
                true
            } else {
                start = start.min(x.start);
                end = end.max(x.end);
                false
            }
        });
        let at = self.ranges.partition_point(|x| x.start < start);
        self.ranges.insert(at, start..end);
    }

    pub fn remove(&mut self, r: Range<u64>) {
        if r.is_empty() {
            return;
        }
        let mut out = Vec::with_capacity(self.ranges.len() + 1);
        for x in self.ranges.drain(..) {
            if x.end <= r.start || x.start >= r.end {
                out.push(x);
                continue;
            }
            if x.start < r.start {
                out.push(x.start..r.start);
            }
            if r.end < x.end {
                out.push(r.end..x.end);
            }
        }
        self.ranges = out;
    }

    /// Keeps only the values below `end`.
    pub fn truncate(&mut self, end: u64) {
        self.remove(end..u64::MAX);
    }

    /// Keeps only the values at or above `start`.
    pub fn remove_below(&mut self, start: u64) {
        self.remove(0..start);
    }

    /// The parts of `within` that are not in the set, lowest first.
    pub fn gaps(&self, within: Range<u64>) -> Vec<Range<u64>> {
        let mut out = Vec::new();
        let mut at = within.start;
        for x in &self.ranges {
            if x.end <= at {
                continue;
            }
            if x.start >= within.end {
                break;
            }
            if x.start > at {
                out.push(at..x.start);
            }
            at = at.max(x.end);
        }
        if at < within.end {
            out.push(at..within.end);
        }
        out
    }
}

#[cfg(test)]
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;

    #[test]
    fn insert_merges_overlapping_and_touching() {
        let mut s = RangeSet::new();
        s.insert(10..20);
        s.insert(30..40);
        s.insert(20..25);
        assert_eq!(s.ranges(), &[10..25, 30..40]);
        s.insert(5..35);
        assert_eq!(s.ranges(), &[5..40]);
        assert_eq!(s.len(), 35);
        assert_eq!(s.max(), Some(39));
    }

    #[test]
    fn remove_splits() {
        let mut s = RangeSet::new();
        s.insert(0..100);
        s.remove(10..20);
        s.remove(90..200);
        assert_eq!(s.ranges(), &[0..10, 20..90]);
        s.truncate(50);
        assert_eq!(s.ranges(), &[0..10, 20..50]);
        s.remove_below(25);
        assert_eq!(s.ranges(), &[25..50]);
    }

    #[test]
    fn gaps_and_covers() {
        let mut s = RangeSet::new();
        s.insert(10..20);
        s.insert(30..40);
        assert_eq!(s.gaps(0..50), vec![0..10, 20..30, 40..50]);
        assert_eq!(s.gaps(15..35), vec![20..30]);
        assert!(s.covers(12..18));
        assert!(!s.covers(15..35));
        assert!(s.covers(5..5));
        assert!(s.contains(10) && !s.contains(20));
    }
}
