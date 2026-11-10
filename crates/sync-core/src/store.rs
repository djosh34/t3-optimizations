//! Helpers for the saved state of the phone and the server. The format uses
//! the same varints as packets and starts with a version byte.

use std::ops::Range;

use crate::ranges::RangeSet;
use crate::wire::{DecodeError, Reader, put_varint};

/// The phone's saved state; the server's has its own (`server.rs`).
pub const VERSION: u8 = 1;

pub fn put_bytes(out: &mut Vec<u8>, b: &[u8]) {
    put_varint(out, b.len() as u64);
    out.extend_from_slice(b);
}

pub fn put_ranges(out: &mut Vec<u8>, s: &RangeSet) {
    put_varint(out, s.ranges().len() as u64);
    for r in s.ranges() {
        put_varint(out, r.start);
        put_varint(out, r.end - r.start);
    }
}

pub fn bytes(r: &mut Reader) -> Result<Vec<u8>, DecodeError> {
    let n = r.varint()?;
    Ok(r.bytes(n)?.to_vec())
}

pub fn ranges(r: &mut Reader) -> Result<RangeSet, DecodeError> {
    let n = r.varint()?;
    let mut s = RangeSet::new();
    for _ in 0..n {
        let start = r.varint()?;
        let len = r.varint()?;
        let end = start
            .checked_add(len)
            .ok_or(DecodeError::Invalid("saved range"))?;
        s.insert(start..end);
    }
    Ok(s)
}

pub fn version(r: &mut Reader, want: u8) -> Result<(), DecodeError> {
    match r.u8()? {
        v if v == want => Ok(()),
        v => Err(DecodeError::Version(v)),
    }
}

pub fn token(r: &mut Reader) -> Result<[u8; 16], DecodeError> {
    r.bytes(16)?.try_into().map_err(|_| DecodeError::Truncated)
}

/// Checks that a range lies inside a buffer of `len` bytes.
pub fn within(r: &Range<u64>, len: u64) -> Result<(), DecodeError> {
    if r.end > len {
        return Err(DecodeError::Invalid("saved range"));
    }
    Ok(())
}
