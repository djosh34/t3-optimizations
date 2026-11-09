//! The packet format. Both directions use the same format.
//!
//! ```text
//! packet  = head:u8 seq:v [hello] [ack] [report] [log_low:v] frame*
//! head    = 0x10 | 0x01 hello | 0x02 ack | 0x04 report | 0x08 log_low
//! hello   = token:16 bytes epoch:v
//! ack     = largest:v delay_ms:v extra:v first_len:v (gap:v len:v)*extra
//! report  = time_ms:v bytes:v packets:v max_seq:v
//! frame   = 0x01 flags:u8 key:v rev:v total_len:v offset:v [keep:v]
//!           [meta_len:v meta] [raw_len:v] data_len:v data
//! flags   = 0x01 meta | 0x02 brotli | 0x04 keep
//! ```
//!
//! `v` is an unsigned LEB128 varint. Ack ranges go from the highest packet
//! number down; `gap` counts the missing numbers between two ranges.

use std::fmt;
use std::io::Read;
use std::ops::Range;

/// Format version, in the high nibble of the first byte.
const VERSION: u8 = 0x10;
const HAS_HELLO: u8 = 0x01;
const HAS_ACK: u8 = 0x02;
const HAS_REPORT: u8 = 0x04;
const HAS_LOG_LOW: u8 = 0x08;

const FRAME_CHUNK: u8 = 0x01;
const CHUNK_META: u8 = 0x01;
const CHUNK_BROTLI: u8 = 0x02;
const CHUNK_KEEP: u8 = 0x04;

/// Largest object the decoder accepts, 256 MiB.
pub const MAX_OBJECT_LEN: u64 = 256 << 20;
/// Largest uncompressed chunk the decoder accepts.
pub const MAX_RAW_CHUNK: u64 = 16 << 10;
const MAX_ACK_RANGES: u64 = 64;

/// The largest packet number. Ranges of packet numbers end one above it, so
/// it must stay well below `u64::MAX`; a connection never gets near it.
pub const MAX_SEQ: u64 = 1 << 62;
/// The largest id a [`Key`] can carry: two bits of the varint name the kind.
pub const MAX_ID: u64 = (1 << 62) - 1;

/// What a chunk belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Key {
    /// Something the server syncs to the phone, numbered by the sidecar.
    Object(u64),
    /// The server's reply to the phone's log entry with this number.
    Reply(u64),
    /// A command or call in the phone's log, by its number.
    Entry(u64),
}

impl Key {
    pub(crate) fn encode(self) -> u64 {
        match self {
            Key::Object(id) => id << 2,
            Key::Reply(id) => (id << 2) | 1,
            Key::Entry(id) => (id << 2) | 2,
        }
    }

    pub(crate) fn decode(v: u64) -> Result<Key, DecodeError> {
        let id = v >> 2;
        match v & 3 {
            0 => Ok(Key::Object(id)),
            1 => Ok(Key::Reply(id)),
            2 => Ok(Key::Entry(id)),
            _ => Err(DecodeError::Invalid("key kind")),
        }
    }
}

/// The first message on a connection from the phone: who it is and which
/// cache it has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    pub token: [u8; 16],
    /// A number the phone picks at random when it starts with an empty cache.
    /// A new epoch tells the server to sync from zero.
    pub epoch: u64,
}

/// Which packets of the other side this side has stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ack {
    /// Time from receiving the largest packet to sending this ack.
    pub delay_ms: u64,
    /// Highest first, disjoint and not touching.
    pub ranges: Vec<Range<u64>>,
}

/// Cumulative counters of what this side received, for the other side's
/// rate control. Cumulative, so a lost report loses nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Report {
    /// The reporter's own clock.
    pub time_ms: u64,
    pub bytes: u64,
    pub packets: u64,
    pub max_seq: u64,
}

/// Bytes of one object, at one revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub key: Key,
    pub rev: u64,
    /// The object's length at this revision.
    pub total_len: u64,
    pub offset: u64,
    /// For a receiver on an older revision: how many of its first bytes stay
    /// valid at this revision.
    pub keep: Option<u64>,
    pub meta: Option<Vec<u8>>,
    pub body: Body,
}

/// The data of a chunk as it goes on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Raw(Vec<u8>),
    /// Brotli with only its built-in dictionary, so it decodes alone.
    Brotli {
        raw_len: u64,
        bytes: Vec<u8>,
    },
}

impl Body {
    pub fn raw_len(&self) -> u64 {
        match self {
            Body::Raw(b) => b.len() as u64,
            Body::Brotli { raw_len, .. } => *raw_len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Packet {
    pub seq: u64,
    pub hello: Option<Hello>,
    pub ack: Option<Ack>,
    pub report: Option<Report>,
    /// From the phone: the lowest log entry it still keeps.
    pub log_low: Option<u64>,
    pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Truncated,
    Version(u8),
    Invalid(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Truncated => write!(f, "packet ends early"),
            DecodeError::Version(v) => write!(f, "unknown packet version {v:#x}"),
            DecodeError::Invalid(what) => write!(f, "invalid {what}"),
        }
    }
}

impl std::error::Error for DecodeError {}

pub fn put_varint(out: &mut Vec<u8>, mut v: u64) {
    while v >= 0x80 {
        out.push((v as u8) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

pub fn varint_len(v: u64) -> usize {
    let bits = 64 - (v | 1).leading_zeros() as usize;
    bits.div_ceil(7)
}

/// Reads a packet front to back.
pub struct Reader<'a> {
    buf: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader { buf }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn u8(&mut self) -> Result<u8, DecodeError> {
        let (&b, rest) = self.buf.split_first().ok_or(DecodeError::Truncated)?;
        self.buf = rest;
        Ok(b)
    }

    pub fn varint(&mut self) -> Result<u64, DecodeError> {
        let mut v: u64 = 0;
        for shift in (0..64).step_by(7) {
            let b = self.u8()?;
            let part = u64::from(b & 0x7f);
            if shift == 63 && part > 1 {
                return Err(DecodeError::Invalid("varint"));
            }
            v |= part << shift;
            if b & 0x80 == 0 {
                return Ok(v);
            }
        }
        Err(DecodeError::Invalid("varint"))
    }

    pub fn bytes(&mut self, n: u64) -> Result<&'a [u8], DecodeError> {
        let n = usize::try_from(n).map_err(|_| DecodeError::Truncated)?;
        if n > self.buf.len() {
            return Err(DecodeError::Truncated);
        }
        let (head, rest) = self.buf.split_at(n);
        self.buf = rest;
        Ok(head)
    }
}

impl Packet {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(64);
        let mut head = VERSION;
        if self.hello.is_some() {
            head |= HAS_HELLO;
        }
        if self.ack.is_some() {
            head |= HAS_ACK;
        }
        if self.report.is_some() {
            head |= HAS_REPORT;
        }
        if self.log_low.is_some() {
            head |= HAS_LOG_LOW;
        }
        out.push(head);
        put_varint(&mut out, self.seq);
        if let Some(h) = &self.hello {
            out.extend_from_slice(&h.token);
            put_varint(&mut out, h.epoch);
        }
        if let Some(a) = &self.ack {
            encode_ack(&mut out, a);
        }
        if let Some(r) = &self.report {
            put_varint(&mut out, r.time_ms);
            put_varint(&mut out, r.bytes);
            put_varint(&mut out, r.packets);
            put_varint(&mut out, r.max_seq);
        }
        if let Some(low) = self.log_low {
            put_varint(&mut out, low);
        }
        for c in &self.chunks {
            encode_chunk(&mut out, c);
        }
        out
    }

    pub fn decode(buf: &[u8]) -> Result<Packet, DecodeError> {
        let mut r = Reader::new(buf);
        let head = r.u8()?;
        if head & 0xf0 != VERSION {
            return Err(DecodeError::Version(head & 0xf0));
        }
        let mut p = Packet {
            seq: r.varint()?,
            ..Packet::default()
        };
        if p.seq > MAX_SEQ {
            return Err(DecodeError::Invalid("packet number"));
        }
        if head & HAS_HELLO != 0 {
            let token: [u8; 16] = r
                .bytes(16)?
                .try_into()
                .map_err(|_| DecodeError::Truncated)?;
            p.hello = Some(Hello {
                token,
                epoch: r.varint()?,
            });
        }
        if head & HAS_ACK != 0 {
            p.ack = Some(decode_ack(&mut r)?);
        }
        if head & HAS_REPORT != 0 {
            p.report = Some(Report {
                time_ms: r.varint()?,
                bytes: r.varint()?,
                packets: r.varint()?,
                max_seq: r.varint()?,
            });
        }
        if head & HAS_LOG_LOW != 0 {
            p.log_low = Some(r.varint()?);
        }
        while !r.is_empty() {
            match r.u8()? {
                FRAME_CHUNK => p.chunks.push(decode_chunk(&mut r)?),
                _ => return Err(DecodeError::Invalid("frame type")),
            }
        }
        Ok(p)
    }
}

fn encode_ack(out: &mut Vec<u8>, a: &Ack) {
    let Some(first) = a.ranges.first() else {
        // An ack with no ranges is never built; encode it as acking nothing
        // useful rather than failing.
        put_varint(out, 0);
        put_varint(out, a.delay_ms);
        put_varint(out, 0);
        put_varint(out, 0);
        return;
    };
    put_varint(out, first.end - 1);
    put_varint(out, a.delay_ms);
    put_varint(out, a.ranges.len() as u64 - 1);
    put_varint(out, first.end - first.start);
    let mut prev_start = first.start;
    for r in &a.ranges[1..] {
        put_varint(out, prev_start - r.end);
        put_varint(out, r.end - r.start);
        prev_start = r.start;
    }
}

fn decode_ack(r: &mut Reader) -> Result<Ack, DecodeError> {
    let bad = DecodeError::Invalid("ack");
    let largest = r.varint()?;
    let delay_ms = r.varint()?;
    let extra = r.varint()?;
    if extra >= MAX_ACK_RANGES {
        return Err(bad);
    }
    let first_len = r.varint()?;
    let end = largest.checked_add(1).ok_or(DecodeError::Invalid("ack"))?;
    let mut ranges = Vec::new();
    if first_len > 0 {
        let start = end
            .checked_sub(first_len)
            .ok_or(DecodeError::Invalid("ack"))?;
        ranges.push(start..end);
    }
    let mut prev_start = end - first_len.min(end);
    for _ in 0..extra {
        let gap = r.varint()?;
        let len = r.varint()?;
        let end = prev_start
            .checked_sub(gap)
            .ok_or(DecodeError::Invalid("ack"))?;
        let start = end.checked_sub(len).ok_or(DecodeError::Invalid("ack"))?;
        if gap == 0 || len == 0 {
            return Err(DecodeError::Invalid("ack"));
        }
        ranges.push(start..end);
        prev_start = start;
    }
    Ok(Ack { delay_ms, ranges })
}

/// Bytes a chunk takes in a packet.
pub fn chunk_len(c: &Chunk) -> usize {
    let mut n = 2
        + varint_len(c.key.encode())
        + varint_len(c.rev)
        + varint_len(c.total_len)
        + varint_len(c.offset);
    if let Some(k) = c.keep {
        n += varint_len(k);
    }
    if let Some(m) = &c.meta {
        n += varint_len(m.len() as u64) + m.len();
    }
    n + match &c.body {
        Body::Raw(b) => varint_len(b.len() as u64) + b.len(),
        Body::Brotli { raw_len, bytes } => {
            varint_len(*raw_len) + varint_len(bytes.len() as u64) + bytes.len()
        }
    }
}

fn encode_chunk(out: &mut Vec<u8>, c: &Chunk) {
    let mut flags = 0;
    if c.meta.is_some() {
        flags |= CHUNK_META;
    }
    if matches!(c.body, Body::Brotli { .. }) {
        flags |= CHUNK_BROTLI;
    }
    if c.keep.is_some() {
        flags |= CHUNK_KEEP;
    }
    out.push(FRAME_CHUNK);
    out.push(flags);
    put_varint(out, c.key.encode());
    put_varint(out, c.rev);
    put_varint(out, c.total_len);
    put_varint(out, c.offset);
    if let Some(k) = c.keep {
        put_varint(out, k);
    }
    if let Some(m) = &c.meta {
        put_varint(out, m.len() as u64);
        out.extend_from_slice(m);
    }
    match &c.body {
        Body::Raw(b) => {
            put_varint(out, b.len() as u64);
            out.extend_from_slice(b);
        }
        Body::Brotli { raw_len, bytes } => {
            put_varint(out, *raw_len);
            put_varint(out, bytes.len() as u64);
            out.extend_from_slice(bytes);
        }
    }
}

/// Decodes a chunk; a compressed body comes back decompressed as `Body::Raw`.
fn decode_chunk(r: &mut Reader) -> Result<Chunk, DecodeError> {
    let flags = r.u8()?;
    if flags & !(CHUNK_META | CHUNK_BROTLI | CHUNK_KEEP) != 0 {
        return Err(DecodeError::Invalid("chunk flags"));
    }
    let key = Key::decode(r.varint()?)?;
    let rev = r.varint()?;
    let total_len = r.varint()?;
    let offset = r.varint()?;
    let keep = if flags & CHUNK_KEEP != 0 {
        Some(r.varint()?)
    } else {
        None
    };
    let meta = if flags & CHUNK_META != 0 {
        let n = r.varint()?;
        Some(r.bytes(n)?.to_vec())
    } else {
        None
    };
    let data = if flags & CHUNK_BROTLI != 0 {
        let raw_len = r.varint()?;
        if raw_len > MAX_RAW_CHUNK {
            return Err(DecodeError::Invalid("chunk length"));
        }
        let n = r.varint()?;
        decompress(r.bytes(n)?, raw_len)?
    } else {
        let n = r.varint()?;
        r.bytes(n)?.to_vec()
    };
    let end = offset
        .checked_add(data.len() as u64)
        .ok_or(DecodeError::Invalid("chunk range"))?;
    if total_len > MAX_OBJECT_LEN || end > total_len {
        return Err(DecodeError::Invalid("chunk range"));
    }
    if keep.is_some_and(|k| k > total_len) {
        return Err(DecodeError::Invalid("chunk keep"));
    }
    Ok(Chunk {
        key,
        rev,
        total_len,
        offset,
        keep,
        meta,
        body: Body::Raw(data),
    })
}

pub use crate::compress::compress;

fn decompress(bytes: &[u8], raw_len: u64) -> Result<Vec<u8>, DecodeError> {
    let mut out = Vec::new();
    // One byte more than expected shows a body that is too long.
    brotli::Decompressor::new(bytes, 4096)
        .take(raw_len + 1)
        .read_to_end(&mut out)
        .map_err(|_| DecodeError::Invalid("brotli body"))?;
    if out.len() as u64 != raw_len {
        return Err(DecodeError::Invalid("brotli length"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varints_round_trip() {
        for v in [0, 1, 127, 128, 300, 1 << 35, u64::MAX] {
            let mut out = Vec::new();
            put_varint(&mut out, v);
            assert_eq!(out.len(), varint_len(v), "{v}");
            assert_eq!(Reader::new(&out).varint(), Ok(v));
        }
        assert!(Reader::new(&[0xff; 11]).varint().is_err());
    }

    #[test]
    fn packet_round_trips() {
        let text = b"The sync loop keeps the newest text first. ".repeat(10);
        let p = Packet {
            seq: 77,
            hello: Some(Hello {
                token: [7; 16],
                epoch: 9,
            }),
            ack: Some(Ack {
                delay_ms: 12,
                ranges: vec![70..76, 60..65, 1..2],
            }),
            report: Some(Report {
                time_ms: 1,
                bytes: 2,
                packets: 3,
                max_seq: 4,
            }),
            log_low: Some(5),
            chunks: vec![
                Chunk {
                    key: Key::Object(3),
                    rev: 2,
                    total_len: 1000,
                    offset: 10,
                    keep: Some(4),
                    meta: Some(b"thread:1".to_vec()),
                    body: Body::Raw(b"hello".to_vec()),
                },
                Chunk {
                    key: Key::Reply(8),
                    rev: 1,
                    total_len: text.len() as u64,
                    offset: 0,
                    keep: None,
                    meta: None,
                    body: Body::Brotli {
                        raw_len: text.len() as u64,
                        bytes: compress(&text).expect("text compresses"),
                    },
                },
            ],
        };
        let bytes = p.encode();
        let expected_len: usize = p.chunks.iter().map(chunk_len).sum();
        assert!(bytes.len() > expected_len);
        let back = Packet::decode(&bytes).expect("decodes");
        assert_eq!(back.seq, 77);
        assert_eq!(back.hello, p.hello);
        assert_eq!(back.ack, p.ack);
        assert_eq!(back.report, p.report);
        assert_eq!(back.log_low, Some(5));
        assert_eq!(back.chunks[0], p.chunks[0]);
        assert_eq!(back.chunks[1].body, Body::Raw(text));
    }

    #[test]
    fn decoding_rejects_bad_input_without_panicking() {
        let p = Packet {
            seq: 1,
            chunks: vec![Chunk {
                key: Key::Entry(1),
                rev: 1,
                total_len: 5,
                offset: 0,
                keep: None,
                meta: None,
                body: Body::Raw(b"hello".to_vec()),
            }],
            ..Packet::default()
        };
        let bytes = p.encode();
        // Head and packet number alone are a valid packet; any cut inside
        // the chunk is not.
        assert!(Packet::decode(&bytes[..2]).is_ok());
        for n in (0..bytes.len()).filter(|&n| n != 2) {
            assert!(Packet::decode(&bytes[..n]).is_err(), "prefix {n}");
        }
        let mut wrong = bytes.clone();
        wrong[0] = 0x20;
        assert_eq!(Packet::decode(&wrong), Err(DecodeError::Version(0x20)));
        // Data beyond the object's length.
        let mut long = p.clone();
        long.chunks[0].total_len = 3;
        assert!(Packet::decode(&long.encode()).is_err());
        // Every single-byte change decodes or fails, never panics.
        for i in 0..bytes.len() {
            for b in [0x00, 0x7f, 0x80, 0xff] {
                let mut m = bytes.clone();
                m[i] = b;
                let _ = Packet::decode(&m);
            }
        }
    }
}
