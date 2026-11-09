//! sync-core's [`Phone`] for the web app (issue #45), as a WebAssembly
//! module with a plain C ABI, so the web app needs no generated glue:
//! `t3code/apps/web/src/t3opt/phoneWasm.ts` is the other half.
//!
//! The module holds one phone. Every export starts with `phone_`, so none
//! clashes with a C library function in the native tests. Byte arguments go
//! in through memory from [`phone_alloc`]; byte results come back in one
//! output buffer that the caller reads with [`phone_out_ptr`] and
//! [`phone_out_len`] right after the call. Numbers that can exceed 2^53
//! (ids, sequence numbers) are `u64`, a `BigInt` in
//! JavaScript; times are `f64` milliseconds. Every call that can fail
//! returns a status: 0 for success, -1 with the error text in the output
//! buffer.
//!
//! The encodings, all integers little-endian:
//! - key: a kind byte (0 object, 1 reply, 2 entry) and a `u64` id;
//! - received: `u64` batch, then `u32` counts each followed by their items:
//!   changed (key, `u32` length, saved bytes), removed (key), replies (`u64`
//!   seq, `u32` length, payload), and last `u64` new bytes;
//! - object view: `u32` meta length, meta, `u64` revision, `u64` length,
//!   `u32` range count, `u64` start and end per range, then the whole
//!   content (zeros where the phone has no bytes yet);
//! - stats: 13 `f64`s in the order of [`stats_values`], -1 for none.

use std::cell::RefCell;
use std::collections::HashMap;

use sync_core::{Config, Delivered, EntryState, Key, Millis, Phone, RangeSet, Received};

thread_local! {
    static PHONE: RefCell<Option<Phone>> = const { RefCell::new(None) };
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static HANDED: RefCell<Handed> = RefCell::new(Handed::default());
}

/// What sync-core handed to the app as new since the page started, by
/// object and revision, to check that no byte is handed over twice (issue
/// #46). It watches only what `receive` returns, so it does not trust the
/// inbox it checks.
#[derive(Default)]
pub struct Handed {
    bytes: HashMap<Key, (u64, RangeSet)>,
    pub duplicates: u64,
}

impl Handed {
    pub fn note(&mut self, delivered: &[Delivered]) {
        for d in delivered {
            let (rev, set) = self.bytes.entry(d.key).or_insert((d.rev, RangeSet::new()));
            // A new revision is new content.
            if *rev != d.rev {
                *rev = d.rev;
                *set = RangeSet::new();
            }
            for r in &d.new {
                let missing: u64 = set.gaps(r.clone()).iter().map(|g| g.end - g.start).sum();
                self.duplicates += (r.end - r.start) - missing;
                set.insert(r.clone());
            }
        }
    }
}

fn ms(now: f64) -> Millis {
    if now.is_finite() && now > 0.0 {
        now as Millis
    } else {
        0
    }
}

fn set_out(bytes: Vec<u8>) {
    OUT.with(|o| {
        if let Ok(mut o) = o.try_borrow_mut() {
            *o = bytes;
        }
    });
}

/// Runs `f` on the phone and puts its bytes in the output buffer. Returns
/// the status.
fn with_phone(f: impl FnOnce(&mut Phone) -> Result<Vec<u8>, String>) -> i32 {
    let result = PHONE.with(|p| match p.try_borrow_mut() {
        Ok(mut p) => match p.as_mut() {
            Some(phone) => f(phone),
            None => Err("no phone: call phone_create or phone_load first".into()),
        },
        Err(_) => Err("the phone is busy".into()),
    });
    match result {
        Ok(bytes) => {
            set_out(bytes);
            0
        }
        Err(e) => {
            set_out(e.into_bytes());
            -1
        }
    }
}

fn install(phone: Phone) {
    PHONE.with(|p| {
        if let Ok(mut p) = p.try_borrow_mut() {
            *p = Some(phone);
        }
    });
    HANDED.with(|h| {
        if let Ok(mut h) = h.try_borrow_mut() {
            *h = Handed::default();
        }
    });
}

fn duplicates() -> u64 {
    HANDED.with(|h| h.try_borrow().map_or(0, |h| h.duplicates))
}

/// The bytes the caller wrote at `ptr`.
///
/// # Safety
/// `ptr` must come from [`phone_alloc`] with at least `len` bytes.
unsafe fn input<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() || len == 0 {
        return &[];
    }
    // SAFETY: the caller allocated `len` bytes at `ptr` with `alloc`.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

// The encodings.

fn put_u32(out: &mut Vec<u8>, v: usize) {
    out.extend_from_slice(&(v as u32).to_le_bytes());
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_key(out: &mut Vec<u8>, key: Key) {
    let (kind, id) = match key {
        Key::Object(id) => (0, id),
        Key::Reply(id) => (1, id),
        Key::Entry(id) => (2, id),
    };
    out.push(kind);
    put_u64(out, id);
}

pub fn encode_received(phone: &Phone, r: &Received) -> Vec<u8> {
    let mut out = Vec::new();
    put_u64(&mut out, r.batch);
    put_u32(&mut out, r.changed.len());
    for &key in &r.changed {
        put_key(&mut out, key);
        let saved = phone.save_object(key).unwrap_or_default();
        put_u32(&mut out, saved.len());
        out.extend_from_slice(&saved);
    }
    put_u32(&mut out, r.removed.len());
    for &key in &r.removed {
        put_key(&mut out, key);
    }
    put_u32(&mut out, r.replies.len());
    for reply in &r.replies {
        put_u64(&mut out, reply.seq);
        put_u32(&mut out, reply.payload.len());
        out.extend_from_slice(&reply.payload);
    }
    let new: u64 = r
        .delivered
        .iter()
        .flat_map(|d| d.new.iter())
        .map(|range| range.end - range.start)
        .sum();
    put_u64(&mut out, new);
    out
}

pub fn encode_object(phone: &Phone, id: u64) -> Option<Vec<u8>> {
    let o = phone.object(id)?;
    let mut out = Vec::with_capacity(o.data.len() + o.meta.len() + 64);
    put_u32(&mut out, o.meta.len());
    out.extend_from_slice(&o.meta);
    put_u64(&mut out, o.rev);
    put_u64(&mut out, o.len);
    put_u32(&mut out, o.have.ranges().len());
    for range in o.have.ranges() {
        put_u64(&mut out, range.start);
        put_u64(&mut out, range.end);
    }
    out.extend_from_slice(&o.data);
    // The data may be shorter than the length while bytes are missing.
    out.resize(out.len() + (o.len as usize).saturating_sub(o.data.len()), 0);
    Some(out)
}

/// The stats as numbers: send rate and delivered rate (wire bytes per
/// second), loss (0 to 1), smoothed and lowest RTT (ms), packets and wire
/// bytes sent and received, when the server was last heard (ms), unsynced
/// bytes, unacked packets (also those presumed lost), and bytes handed to
/// the app twice.
pub fn stats_values(phone: &Phone, duplicates: u64) -> [f64; 13] {
    let s = phone.stats();
    [
        s.send_rate,
        s.delivered_rate,
        s.loss,
        s.srtt_ms.unwrap_or(-1.0),
        s.min_rtt_ms.unwrap_or(-1.0),
        s.sent_packets as f64,
        s.sent_wire_bytes as f64,
        s.received_packets as f64,
        s.received_wire_bytes as f64,
        s.last_heard.map_or(-1.0, |t| t as f64),
        s.unsynced_bytes as f64,
        s.inflight_packets as f64,
        duplicates as f64,
    ]
}

/// Splits a load buffer: the state, then the saved objects, each with a
/// `u32` length before it.
pub fn split_load(buf: &[u8]) -> Result<(&[u8], Vec<&[u8]>), String> {
    let mut parts = Vec::new();
    let mut rest = buf;
    while !rest.is_empty() {
        let (len, tail) = rest
            .split_first_chunk::<4>()
            .ok_or("load buffer cut off in a length")?;
        let len = u32::from_le_bytes(*len) as usize;
        let part = tail.get(..len).ok_or("load buffer cut off in a part")?;
        parts.push(part);
        rest = &tail[len..];
    }
    if parts.is_empty() {
        return Err("load buffer without the state".into());
    }
    let state = parts.remove(0);
    Ok((state, parts))
}

// The ABI.

/// Memory for an argument of `len` bytes. Free it with [`phone_dealloc`].
#[unsafe(no_mangle)]
pub extern "C" fn phone_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

/// # Safety
/// `ptr` and `len` must be what [`phone_alloc`] gave and was asked for.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_dealloc(ptr: *mut u8, len: usize) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: `ptr` came from `alloc(len)`, a Vec of capacity `len.max(1)`.
    drop(unsafe { Vec::from_raw_parts(ptr, 0, len.max(1)) });
}

#[unsafe(no_mangle)]
pub extern "C" fn phone_out_ptr() -> *const u8 {
    OUT.with(|o| o.try_borrow().map_or(std::ptr::null(), |o| o.as_ptr()))
}

#[unsafe(no_mangle)]
pub extern "C" fn phone_out_len() -> usize {
    OUT.with(|o| o.try_borrow().map_or(0, |o| o.len()))
}

/// A phone with an empty cache. `token` points at 16 bytes; `epoch` must be
/// random.
///
/// # Safety
/// `token` must point at 16 bytes from [`phone_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_create(token: *const u8, epoch: u64, now: f64) -> i32 {
    // SAFETY: the caller wrote 16 bytes at `token`.
    let Ok(token) = <[u8; 16]>::try_from(unsafe { input(token, 16) }) else {
        set_out(b"the token must be 16 bytes".to_vec());
        return -1;
    };
    install(Phone::new(token, epoch, Config::phone(), ms(now)));
    0
}

/// The phone as it was stored: see [`split_load`].
///
/// # Safety
/// `ptr` must point at `len` bytes from [`phone_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_load(ptr: *const u8, len: usize, now: f64) -> i32 {
    // SAFETY: the caller wrote `len` bytes at `ptr`.
    let buf = unsafe { input(ptr, len) };
    let loaded = split_load(buf).and_then(|(state, objects)| {
        Phone::load(state, objects, Config::phone(), ms(now))
            .map_err(|e| format!("stored phone unreadable: {e:?}"))
    });
    match loaded {
        Ok(phone) => {
            install(phone);
            0
        }
        Err(e) => {
            set_out(e.into_bytes());
            -1
        }
    }
}

/// A new connection to the sidecar (not an ICE restart).
#[unsafe(no_mangle)]
pub extern "C" fn phone_connected() -> i32 {
    with_phone(|p| {
        p.connected();
        Ok(Vec::new())
    })
}

/// A packet from the sidecar. The output is the received encoding: store
/// it, then call [`phone_committed`] with its batch.
///
/// # Safety
/// `ptr` must point at `len` bytes from [`phone_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_receive(ptr: *const u8, len: usize, now: f64) -> i32 {
    // SAFETY: the caller wrote `len` bytes at `ptr`.
    let packet = unsafe { input(ptr, len) };
    with_phone(|p| {
        let r = p
            .receive(ms(now), packet)
            .map_err(|e| format!("undecodable packet: {e:?}"))?;
        HANDED.with(|h| {
            if let Ok(mut h) = h.try_borrow_mut() {
                h.note(&r.delivered);
            }
        });
        Ok(encode_received(p, &r))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn phone_committed(batch: u64) -> i32 {
    with_phone(|p| {
        p.committed(batch);
        Ok(Vec::new())
    })
}

/// 1 with the next packet in the output, 0 when none should go out now.
#[unsafe(no_mangle)]
pub extern "C" fn phone_poll(now: f64) -> i32 {
    let mut sent = false;
    let status = with_phone(|p| {
        let packet = p.poll(ms(now));
        sent = packet.is_some();
        Ok(packet.unwrap_or_default())
    });
    if status < 0 { status } else { i32::from(sent) }
}

/// When to call [`phone_poll`] next, in ms; -1 without a phone.
#[unsafe(no_mangle)]
pub extern "C" fn phone_next_wakeup(now: f64) -> f64 {
    let mut at = -1.0;
    with_phone(|p| {
        at = p.next_wakeup(ms(now)) as f64;
        Ok(Vec::new())
    });
    at
}

/// Adds an `orchestration.dispatchCommand` to the log. Returns its number,
/// 0 on error.
///
/// # Safety
/// `ptr` must point at `len` bytes from [`phone_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_command(ptr: *const u8, len: usize) -> u64 {
    // SAFETY: the caller wrote `len` bytes at `ptr`.
    let payload = unsafe { input(ptr, len) }.to_vec();
    let mut seq = 0;
    with_phone(|p| {
        seq = p.command(payload);
        Ok(Vec::new())
    });
    seq
}

/// Adds a call to the log; `background` 1 for the preventive sync's class.
/// Returns its number, 0 on error.
///
/// # Safety
/// `ptr` must point at `len` bytes from [`phone_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn phone_call(ptr: *const u8, len: usize, background: i32) -> u64 {
    // SAFETY: the caller wrote `len` bytes at `ptr`.
    let payload = unsafe { input(ptr, len) }.to_vec();
    let mut seq = 0;
    with_phone(|p| {
        seq = p.call(payload, background != 0);
        Ok(Vec::new())
    });
    seq
}

/// -1 when the entry has left the log, 0 queued, 1 sent, 2 the server has it.
#[unsafe(no_mangle)]
pub extern "C" fn phone_entry_state(seq: u64) -> i32 {
    let mut state = -1;
    with_phone(|p| {
        state = match p.entry_state(seq) {
            None => -1,
            Some(EntryState::Queued) => 0,
            Some(EntryState::Sent) => 1,
            Some(EntryState::ServerHas) => 2,
        };
        Ok(Vec::new())
    });
    state
}

/// The token, epoch and log, to store.
#[unsafe(no_mangle)]
pub extern "C" fn phone_save_state() -> i32 {
    with_phone(|p| Ok(p.save_state()))
}

/// 1 with the object view in the output, 0 when the phone has no such
/// object.
#[unsafe(no_mangle)]
pub extern "C" fn phone_object(id: u64) -> i32 {
    let mut found = false;
    let status = with_phone(|p| {
        let view = encode_object(p, id);
        found = view.is_some();
        Ok(view.unwrap_or_default())
    });
    if status < 0 { status } else { i32::from(found) }
}

/// The ids of every object, `u64` each.
#[unsafe(no_mangle)]
pub extern "C" fn phone_object_ids() -> i32 {
    with_phone(|p| {
        let mut out = Vec::new();
        for (id, _) in p.objects() {
            put_u64(&mut out, id);
        }
        Ok(out)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn phone_stats() -> i32 {
    with_phone(|p| {
        Ok(stats_values(p, duplicates())
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sync_core::{Class, Server};

    fn u32_at(b: &[u8], at: &mut usize) -> usize {
        let v = u32::from_le_bytes(b[*at..*at + 4].try_into().unwrap()) as usize;
        *at += 4;
        v
    }

    fn u64_at(b: &[u8], at: &mut usize) -> u64 {
        let v = u64::from_le_bytes(b[*at..*at + 8].try_into().unwrap());
        *at += 8;
        v
    }

    /// A server and a phone: an object and a reply arrive, the encodings
    /// carry them, and a phone loaded from the saved bytes has the object.
    #[test]
    fn encodings_round_trip() {
        let token = [3; 16];
        let mut server = Server::new(token, Config::server(), 0);
        let mut phone = Phone::new(token, 9, Config::phone(), 0);
        server
            .set(
                42,
                Class::Status,
                1,
                b"{\"c\":\"x\"}".to_vec(),
                b"hello".to_vec(),
            )
            .unwrap();
        let seq = phone.call(b"{\"tag\":\"t\"}".to_vec(), false);
        let mut saved = Vec::new();
        let mut replies = 0;
        for now in (0..4000).step_by(50) {
            while let Some(p) = phone.poll(now) {
                for inc in server.receive(now, &p).unwrap() {
                    if let sync_core::Incoming::Call { seq, .. } = inc {
                        server.reply(seq, b"{\"exit\":1}".to_vec()).unwrap();
                    }
                }
            }
            while let Some(p) = server.poll(now) {
                let r = phone.receive(now, &p).unwrap();
                let bytes = encode_received(&phone, &r);
                let mut at = 8;
                for _ in 0..u32_at(&bytes, &mut at) {
                    at += 9;
                    let n = u32_at(&bytes, &mut at);
                    saved.push(bytes[at..at + n].to_vec());
                    at += n;
                }
                for _ in 0..u32_at(&bytes, &mut at) {
                    at += 9;
                }
                for _ in 0..u32_at(&bytes, &mut at) {
                    assert_eq!(u64_at(&bytes, &mut at), seq);
                    let n = u32_at(&bytes, &mut at);
                    assert_eq!(&bytes[at..at + n], b"{\"exit\":1}");
                    at += n;
                    replies += 1;
                }
                phone.committed(r.batch);
            }
        }
        assert_eq!(replies, 1);
        let view = encode_object(&phone, 42).unwrap();
        let mut at = 0;
        let meta = u32_at(&view, &mut at);
        assert_eq!(&view[at..at + meta], b"{\"c\":\"x\"}");
        at += meta + 8;
        assert_eq!(u64_at(&view, &mut at), 5);
        assert_eq!(u32_at(&view, &mut at), 1);
        assert_eq!((u64_at(&view, &mut at), u64_at(&view, &mut at)), (0, 5));
        assert_eq!(&view[at..], b"hello");

        let mut buf = Vec::new();
        for part in std::iter::once(phone.save_state()).chain(saved) {
            buf.extend_from_slice(&(part.len() as u32).to_le_bytes());
            buf.extend_from_slice(&part);
        }
        let (state, objects) = split_load(&buf).unwrap();
        let loaded = Phone::load(state, objects, Config::phone(), 0).unwrap();
        assert!(loaded.object(42).unwrap().complete());
    }

    #[test]
    fn split_load_rejects_cut_buffers() {
        assert!(split_load(&[]).is_err());
        assert!(split_load(&[5, 0, 0, 0, 1]).is_err());
        assert!(split_load(&[1, 0]).is_err());
        assert_eq!(split_load(&[1, 0, 0, 0, 7]).unwrap().0, &[7]);
    }

    #[test]
    fn bytes_handed_over_twice_are_counted() {
        let d = |rev, new: std::ops::Range<u64>| Delivered {
            key: Key::Object(1),
            rev,
            new: vec![new],
        };
        let mut h = Handed::default();
        h.note(&[d(1, 0..10)]);
        h.note(&[d(1, 5..20)]);
        assert_eq!(h.duplicates, 5);
        // A new revision is new content.
        h.note(&[d(2, 0..10)]);
        assert_eq!(h.duplicates, 5);
    }
}
