//! The server's side, one per paired phone: syncs objects to the phone and
//! hands the phone's commands and calls to the sidecar, each once.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use crate::Millis;
use crate::endpoint::{Config, Endpoint, Stats, decode};
use crate::inbox::InObject;
use crate::outbox::{Busy, Class, OutObject, Saved};
use crate::phone::EntryKind;
use crate::store;
use crate::wire::{DecodeError, Hello, Key, MAX_ID, Packet, Reader, put_varint};

/// What the phone sent that the sidecar must act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Incoming {
    /// The phone came with a new epoch: its cache is empty and it gets
    /// everything again.
    Reset,
    /// Run this `orchestration.dispatchCommand`, then answer with
    /// [`Server::reply`]. Commands come in the order the phone made them.
    Command { seq: u64, payload: Vec<u8> },
    /// Run this server call, then answer with [`Server::reply`].
    Call {
        seq: u64,
        payload: Vec<u8>,
        background: bool,
    },
}

/// The version of [`Server::save`]'s format; 2 saves objects without
/// their bytes.
const VERSION: u8 = 2;

/// The most meta bytes an object can have: with the chunk header and the
/// largest packet header, a chunk with its meta still fits a packet.
pub const MAX_META: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Object ids go up to [`MAX_ID`].
    IdTooLarge(u64),
    /// Meta goes up to [`MAX_META`] bytes, so a chunk with it fits a packet.
    MetaTooLarge(usize),
    /// No command or call with this number is waiting for a reply.
    NoEntry(u64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IdTooLarge(id) => write!(f, "object id {id} is above {MAX_ID}"),
            Error::MetaTooLarge(n) => write!(f, "meta of {n} bytes is above {MAX_META}"),
            Error::NoEntry(seq) => write!(f, "no command or call {seq} waits for a reply"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Server {
    ep: Endpoint,
    token: [u8; 16],
    epoch: Option<u64>,
    /// Entries handed to the sidecar, kept until the phone drops them from
    /// its log, so a late copy is never handed over twice.
    done: BTreeMap<u64, EntryKind>,
    /// Entries handed to the sidecar that it has not answered yet. They are
    /// not saved: a restarted sidecar gets them again when the phone sends
    /// them again on its new connection.
    unanswered: BTreeSet<u64>,
    /// Complete commands waiting for the command before them, by that
    /// command's number.
    held: BTreeMap<u64, (u64, Vec<u8>)>,
    log_low: u64,
}

impl Server {
    pub fn new(token: [u8; 16], cfg: Config, now: Millis) -> Self {
        Server {
            ep: Endpoint::new(cfg, now),
            token,
            epoch: None,
            done: BTreeMap::new(),
            unanswered: BTreeSet::new(),
            held: BTreeMap::new(),
            log_low: 0,
        }
    }

    /// The hello in a packet, if it has one. The sidecar finds the phone's
    /// `Server` by its token.
    pub fn hello(bytes: &[u8]) -> Option<Hello> {
        decode(bytes).ok()?.hello
    }

    pub fn token(&self) -> [u8; 16] {
        self.token
    }

    /// Call when a new connection of this phone replaces the old one (not
    /// on an ICE restart). Packets of the old connection must not reach
    /// [`Server::receive`] after this.
    pub fn connected(&mut self) {
        self.ep.new_connection();
    }

    /// Sets an object to sync. See [`Class`] for the order things go out in.
    /// `meta` is what the phone needs to show the object on its own (thread
    /// and message ids); it goes with the object's first chunks.
    ///
    /// The server keeps the bytes, shared with the caller, only until the
    /// phone stored them; then it keeps their length and hash, and a set with
    /// the same bytes changes nothing.
    pub fn set(
        &mut self,
        id: u64,
        class: Class,
        order: u64,
        meta: Vec<u8>,
        content: impl Into<Arc<[u8]>>,
    ) -> Result<(), Error> {
        if id > MAX_ID {
            return Err(Error::IdTooLarge(id));
        }
        if meta.len() > MAX_META {
            return Err(Error::MetaTooLarge(meta.len()));
        }
        self.ep
            .out
            .set(Key::Object(id), class, order, meta, content);
        Ok(())
    }

    /// Deletes an object the phone may have: it drops its copy.
    pub fn delete(&mut self, id: u64) {
        let Some(o) = self.ep.out.get(Key::Object(id)) else {
            return;
        };
        if o.len() == 0 {
            return;
        }
        let (class, order, meta) = (o.class, o.order, o.meta.clone());
        self.ep
            .out
            .set(Key::Object(id), class, order, meta, Vec::new());
    }

    /// Lets go of an object's bytes before the phone stored them all: it
    /// waits, unsent, until [`Server::set`] gives them again. Its revision
    /// and what the phone stored of it stay.
    pub fn release(&mut self, id: u64) {
        self.ep.out.release(Key::Object(id));
    }

    /// The result of a command or call, for the phone.
    pub fn reply(&mut self, seq: u64, payload: Vec<u8>) -> Result<(), Error> {
        let kind = *self.done.get(&seq).ok_or(Error::NoEntry(seq))?;
        self.unanswered.remove(&seq);
        let class = if kind == EntryKind::BackgroundCall {
            Class::Background
        } else {
            Class::Urgent
        };
        self.ep
            .out
            .set(Key::Reply(seq), class, kind.order(seq), Vec::new(), payload);
        Ok(())
    }

    pub fn receive(&mut self, now: Millis, bytes: &[u8]) -> Result<Vec<Incoming>, DecodeError> {
        let p = decode(bytes)?;
        let mut incoming = Vec::new();
        // The hello goes first: a new epoch wipes what the phone had, and
        // the acks in the same packet already belong to the new epoch.
        if let Some(h) = &p.hello
            && self.epoch != Some(h.epoch)
        {
            self.reset();
            self.epoch = Some(h.epoch);
            incoming.push(Incoming::Reset);
        }
        let (batch, acked) = self.ep.take(now, &p, bytes.len());
        // The server keeps what it received in memory at once; a lost entry
        // comes again when the phone reconnects.
        self.ep.inbox.commit(batch);
        for key in acked.done {
            match key {
                Key::Reply(_) => self.ep.out.remove(key),
                Key::Object(_) => self.ep.out.release(key),
                Key::Entry(_) => {}
            }
        }
        if let Some(low) = p.log_low {
            self.forget_below(low);
        }
        self.take_entries(p, &mut incoming);
        Ok(incoming)
    }

    fn take_entries(&mut self, p: Packet, incoming: &mut Vec<Incoming>) {
        for c in p.chunks {
            let Key::Entry(seq) = c.key else {
                continue;
            };
            if seq < self.log_low || self.done.contains_key(&seq) {
                continue;
            }
            if self.ep.inbox.apply(c).is_none() {
                continue;
            }
            let key = Key::Entry(seq);
            if !self
                .ep
                .inbox
                .objects
                .get(&key)
                .is_some_and(InObject::complete)
            {
                continue;
            }
            let Some(o) = self.ep.inbox.objects.remove(&key) else {
                continue;
            };
            let mut meta = Reader::new(&o.meta);
            let Some(kind) = meta.u8().ok().and_then(EntryKind::from_u8) else {
                // Not an entry this version knows; drop it.
                continue;
            };
            if kind != EntryKind::Command {
                self.done.insert(seq, kind);
                self.unanswered.insert(seq);
                incoming.push(Incoming::Call {
                    seq,
                    payload: o.data,
                    background: kind == EntryKind::BackgroundCall,
                });
                continue;
            }
            let Ok(prev) = meta.varint() else {
                continue;
            };
            self.held.insert(prev, (seq, o.data));
            self.release_commands(incoming);
        }
    }

    /// Hands over every held command whose previous command is applied.
    fn release_commands(&mut self, incoming: &mut Vec<Incoming>) {
        loop {
            let ready = self
                .held
                .keys()
                .find(|&&prev| prev == 0 || prev < self.log_low || self.done.contains_key(&prev))
                .copied();
            let Some((seq, payload)) = ready.and_then(|prev| self.held.remove(&prev)) else {
                return;
            };
            if seq < self.log_low || self.done.contains_key(&seq) {
                continue;
            }
            self.done.insert(seq, EntryKind::Command);
            self.unanswered.insert(seq);
            incoming.push(Incoming::Command { seq, payload });
        }
    }

    /// The phone no longer keeps entries below `low`.
    fn forget_below(&mut self, low: u64) {
        if low <= self.log_low {
            return;
        }
        self.log_low = low;
        self.done = self.done.split_off(&low);
        self.held.retain(|_, (seq, _)| *seq >= low);
        self.unanswered = self.unanswered.split_off(&low);
        self.ep
            .inbox
            .objects
            .retain(|k, _| !matches!(k, Key::Entry(s) if *s < low));
        self.ep
            .out
            .remove_where(|k| matches!(k, Key::Reply(s) if s < low));
    }

    fn reset(&mut self) {
        self.ep.out.forget_peer();
        self.ep.out.remove_where(|k| matches!(k, Key::Reply(_)));
        self.ep.inbox.objects.clear();
        self.done.clear();
        self.unanswered.clear();
        self.held.clear();
        self.log_low = 0;
    }

    /// The next packet to send, if one should go out now.
    pub fn poll(&mut self, now: Millis) -> Option<Vec<u8>> {
        self.ep.poll(now, None, None)
    }

    /// When to call [`Server::poll`] next.
    pub fn next_wakeup(&self, now: Millis) -> Millis {
        self.ep.next_wakeup(now, false)
    }

    /// The ids of the objects set and not deleted, so a restarted sidecar
    /// can delete those the forked server no longer has.
    pub fn object_ids(&self) -> Vec<u64> {
        self.ep
            .out
            .objects()
            .filter_map(|(k, o)| match k {
                Key::Object(id) if o.len() > 0 => Some(*id),
                _ => None,
            })
            .collect()
    }

    /// Whether an object with this id was set.
    pub fn has(&self, id: u64) -> bool {
        self.ep.out.get(Key::Object(id)).is_some()
    }

    /// Whether the phone stored this object's current revision whole.
    pub fn stored(&self, id: u64) -> bool {
        self.ep.out.get(Key::Object(id)).is_some_and(|o| o.done())
    }

    /// The objects set, with their meta, also deleted ones.
    pub fn metas(&self) -> impl Iterator<Item = (u64, &[u8])> {
        self.ep.out.objects().filter_map(|(k, o)| match k {
            Key::Object(id) => Some((*id, o.meta.as_slice())),
            _ => None,
        })
    }

    /// Bytes that wait to go to the phone; cheaper than [`Server::stats`].
    pub fn unsynced_bytes(&self) -> u64 {
        self.ep.out.unsynced_bytes()
    }

    /// Whether the phone has stored everything.
    pub fn is_synced(&self) -> bool {
        self.ep.out.is_synced()
    }

    pub fn stats(&self) -> Stats {
        self.ep.stats()
    }

    /// The `n` objects that took the most packet bytes since the last call.
    pub fn take_busiest(&mut self, n: usize) -> Vec<Busy> {
        self.ep.out.take_busiest(n)
    }

    /// Everything a restarted sidecar needs to go on without a full resync.
    ///
    /// Commands and calls the sidecar was handed but had not answered are
    /// left out: after a restart they are handed over again when the phone
    /// sends them again, which it does on every new connection. A command can
    /// so reach t3 twice across a sidecar crash; t3 keeps a receipt per
    /// `commandId` and applies it once.
    ///
    /// Objects are saved without their bytes: their length and hash. A
    /// restarted sidecar sets them again from the t3 server; what the phone
    /// has not stored of one waits until then.
    pub fn save(&self) -> Vec<u8> {
        let mut out = vec![VERSION];
        out.extend_from_slice(&self.token);
        match self.epoch {
            Some(e) => {
                out.push(1);
                put_varint(&mut out, e);
            }
            None => out.push(0),
        }
        put_varint(&mut out, self.log_low);
        let answered: Vec<_> = self
            .done
            .iter()
            .filter(|(seq, _)| !self.unanswered.contains(seq))
            .collect();
        put_varint(&mut out, answered.len() as u64);
        for (&seq, &kind) in answered {
            put_varint(&mut out, seq);
            out.push(kind.to_u8());
        }
        let objects: Vec<_> = self.ep.out.objects().collect();
        put_varint(&mut out, objects.len() as u64);
        for (key, o) in objects {
            put_varint(&mut out, key.encode());
            out.push(o.class as u8);
            put_varint(&mut out, o.order);
            store::put_bytes(&mut out, &o.meta);
            put_varint(&mut out, o.rev);
            match key {
                // A reply has no other source than this.
                Key::Reply(_) => {
                    out.push(1);
                    store::put_bytes(&mut out, o.bytes());
                }
                _ => {
                    out.push(0);
                    put_varint(&mut out, o.len());
                    out.extend_from_slice(&o.hash.to_be_bytes());
                }
            }
            store::put_ranges(&mut out, &o.acked);
            out.push(u8::from(o.rev_acked));
            put_varint(&mut out, o.keep);
        }
        out
    }

    pub fn load(bytes: &[u8], cfg: Config, now: Millis) -> Result<Server, DecodeError> {
        let mut r = Reader::new(bytes);
        // Version 1 saved every object's bytes.
        let version = r.u8()?;
        if version != 1 && version != VERSION {
            return Err(DecodeError::Version(version));
        }
        let token = store::token(&mut r)?;
        let mut s = Server::new(token, cfg, now);
        s.epoch = match r.u8()? {
            0 => None,
            _ => Some(r.varint()?),
        };
        s.log_low = r.varint()?;
        for _ in 0..r.varint()? {
            let seq = r.varint()?;
            let kind = EntryKind::from_u8(r.u8()?).ok_or(DecodeError::Invalid("entry kind"))?;
            s.done.insert(seq, kind);
        }
        for _ in 0..r.varint()? {
            let key = Key::decode(r.varint()?)?;
            let class = Class::from_u8(r.u8()?).ok_or(DecodeError::Invalid("class"))?;
            let order = r.varint()?;
            let meta = store::bytes(&mut r)?;
            let rev = r.varint()?;
            let tag = if version == 1 { 1 } else { r.u8()? };
            let content = match tag {
                0 => {
                    let len = r.varint()?;
                    let hash = r.bytes(8)?.try_into().map_err(|_| DecodeError::Truncated)?;
                    Saved::Released {
                        len,
                        hash: u64::from_be_bytes(hash),
                    }
                }
                _ => Saved::Bytes(store::bytes(&mut r)?),
            };
            let acked = store::ranges(&mut r)?;
            let rev_acked = r.u8()? != 0;
            let keep = r.varint()?;
            s.ep.out.restore(
                key,
                OutObject::restored(class, order, meta, rev, content, acked, rev_acked, keep),
            );
            if let Key::Object(_) = key {
                s.ep.out.release(key);
            }
        }
        Ok(s)
    }
}
