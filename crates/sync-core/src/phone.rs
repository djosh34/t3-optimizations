//! The phone's side: receives what the server syncs, stores it, and keeps a
//! log of the owner's commands and calls until their results are stored.

use std::collections::BTreeMap;

use crate::Millis;
use crate::endpoint::{Config, Endpoint, Stats, decode};
use crate::inbox::{Delivered, InObject};
use crate::outbox::Class;
use crate::store;
use crate::wire::{DecodeError, Hello, Key, Reader, put_varint};

/// What a log entry is. The server answers each with one reply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    /// An `orchestration.dispatchCommand`: applied once, in order.
    Command,
    /// Any other server call the owner made.
    Call,
    /// A call made in the background, for the preventive sync.
    BackgroundCall,
}

impl EntryKind {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            EntryKind::Command => 0,
            EntryKind::Call => 1,
            EntryKind::BackgroundCall => 2,
        }
    }

    pub(crate) fn from_u8(v: u8) -> Option<EntryKind> {
        match v {
            0 => Some(EntryKind::Command),
            1 => Some(EntryKind::Call),
            2 => Some(EntryKind::BackgroundCall),
            _ => None,
        }
    }
}

/// Where a log entry is, for the status line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryState {
    /// Not sent on this connection yet.
    Queued,
    Sent,
    /// The server has stored all of it.
    ServerHas,
}

/// The server's reply to a log entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub seq: u64,
    pub payload: Vec<u8>,
}

/// What one packet changed. Store all of it, then call
/// [`Phone::committed`] with `batch`: only then does the phone ack it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Received {
    pub batch: u64,
    /// Saved objects to write again: [`Phone::save_object`] of each.
    pub changed: Vec<Key>,
    /// Saved objects to delete.
    pub removed: Vec<Key>,
    /// Finished replies. Their entries have left the log; write
    /// [`Phone::save_state`] too.
    pub replies: Vec<Reply>,
    /// The new bytes, for tests and statistics.
    pub delivered: Vec<Delivered>,
}

#[derive(Debug, Clone)]
pub struct Phone {
    ep: Endpoint,
    token: [u8; 16],
    epoch: u64,
    next_entry: u64,
    log: BTreeMap<u64, EntryKind>,
    /// Hello goes in every packet until the server's first packet on this
    /// connection.
    hello: bool,
}

impl Phone {
    /// A phone with an empty cache. `epoch` must be random: a new epoch tells
    /// the server to sync everything again.
    pub fn new(token: [u8; 16], epoch: u64, cfg: Config, now: Millis) -> Self {
        Phone {
            ep: Endpoint::new(cfg, now),
            token,
            epoch,
            next_entry: 1,
            log: BTreeMap::new(),
            hello: true,
        }
    }

    /// Call on every new connection to the server (not on an ICE restart,
    /// which keeps the connection). Sends the hello again and every log
    /// entry in full; the server applies each only once.
    pub fn connected(&mut self) {
        self.ep.new_connection();
        self.hello = true;
        for &seq in self.log.keys() {
            self.ep.out.forget_peer_of(Key::Entry(seq));
        }
    }

    /// Adds a command to the log. Returns its number.
    pub fn command(&mut self, payload: Vec<u8>) -> u64 {
        self.add(EntryKind::Command, payload)
    }

    /// Adds a call to the log. Returns its number.
    pub fn call(&mut self, payload: Vec<u8>, background: bool) -> u64 {
        let kind = if background {
            EntryKind::BackgroundCall
        } else {
            EntryKind::Call
        };
        self.add(kind, payload)
    }

    fn add(&mut self, kind: EntryKind, payload: Vec<u8>) -> u64 {
        let seq = self.next_entry;
        self.next_entry += 1;
        let class = if kind == EntryKind::BackgroundCall {
            Class::Background
        } else {
            Class::Urgent
        };
        // A command names the command before it that is still in the log, so
        // the server applies commands in order. One that left the log has
        // its result, so the server has applied it.
        let mut meta = vec![kind.to_u8()];
        if kind == EntryKind::Command {
            let prev = self
                .log
                .range(..seq)
                .rev()
                .find(|(_, k)| **k == EntryKind::Command)
                .map_or(0, |(s, _)| *s);
            put_varint(&mut meta, prev);
        }
        self.log.insert(seq, kind);
        // Lower numbers first: the highest order goes first.
        self.ep
            .out
            .set(Key::Entry(seq), class, u64::MAX - seq, meta, payload);
        seq
    }

    pub fn entry_state(&self, seq: u64) -> Option<EntryState> {
        self.log.get(&seq)?;
        let o = self.ep.out.get(Key::Entry(seq))?;
        Some(if o.done() {
            EntryState::ServerHas
        } else if o.was_sent() {
            EntryState::Sent
        } else {
            EntryState::Queued
        })
    }

    /// The numbers of the entries still waiting for a reply.
    pub fn pending(&self) -> impl Iterator<Item = u64> + '_ {
        self.log.keys().copied()
    }

    fn log_low(&self) -> u64 {
        self.log.keys().next().copied().unwrap_or(self.next_entry)
    }

    pub fn receive(&mut self, now: Millis, bytes: &[u8]) -> Result<Received, DecodeError> {
        let p = decode(bytes)?;
        let (batch, _) = self.ep.take(now, &p, bytes.len());
        self.hello = false;
        let mut r = Received {
            batch,
            ..Received::default()
        };
        for c in p.chunks {
            let key = c.key;
            match key {
                Key::Object(_) => {}
                // A reply to an entry that is no longer in the log came
                // already.
                Key::Reply(seq) if self.log.contains_key(&seq) => {}
                Key::Reply(_) | Key::Entry(_) => continue,
            }
            let Some(d) = self.ep.inbox.apply(c) else {
                continue;
            };
            r.delivered.push(d);
            if let Key::Reply(seq) = key {
                let done = self
                    .ep
                    .inbox
                    .objects
                    .get(&key)
                    .is_some_and(InObject::complete);
                if done {
                    let payload = self
                        .ep
                        .inbox
                        .objects
                        .remove(&key)
                        .map(|o| o.data)
                        .unwrap_or_default();
                    self.log.remove(&seq);
                    self.ep.out.remove(Key::Entry(seq));
                    r.changed.retain(|k| *k != key);
                    r.removed.push(key);
                    r.replies.push(Reply { seq, payload });
                    continue;
                }
            }
            if !r.changed.contains(&key) {
                r.changed.push(key);
            }
        }
        Ok(r)
    }

    /// Everything of `batch` and earlier is stored; their packets get acked.
    pub fn committed(&mut self, batch: u64) {
        self.ep.inbox.commit(batch);
    }

    /// The next packet to send, if one should go out now.
    pub fn poll(&mut self, now: Millis) -> Option<Vec<u8>> {
        let hello = self.hello.then_some(Hello {
            token: self.token,
            epoch: self.epoch,
        });
        let low = self.log_low();
        self.ep.poll(now, hello, Some(low))
    }

    /// When to call [`Phone::poll`] next.
    pub fn next_wakeup(&self, now: Millis) -> Millis {
        self.ep.next_wakeup(now, self.hello)
    }

    /// A synced object, by the id the server gave it.
    pub fn object(&self, id: u64) -> Option<&InObject> {
        self.ep.inbox.objects.get(&Key::Object(id))
    }

    pub fn objects(&self) -> impl Iterator<Item = (u64, &InObject)> {
        self.ep.inbox.objects.iter().filter_map(|(k, o)| match k {
            Key::Object(id) => Some((*id, o)),
            _ => None,
        })
    }

    pub fn stats(&self) -> Stats {
        self.ep.stats()
    }

    /// The token, the epoch and the log.
    pub fn save_state(&self) -> Vec<u8> {
        let mut out = vec![store::VERSION];
        out.extend_from_slice(&self.token);
        put_varint(&mut out, self.epoch);
        put_varint(&mut out, self.next_entry);
        put_varint(&mut out, self.log.len() as u64);
        for (&seq, &kind) in &self.log {
            put_varint(&mut out, seq);
            out.push(kind.to_u8());
            let payload = self
                .ep
                .out
                .get(Key::Entry(seq))
                .map(|o| o.content.as_slice())
                .unwrap_or_default();
            store::put_bytes(&mut out, payload);
        }
        out
    }

    /// One received object as stored, None if there is none.
    pub fn save_object(&self, key: Key) -> Option<Vec<u8>> {
        let o = self.ep.inbox.objects.get(&key)?;
        let mut out = vec![store::VERSION];
        put_varint(&mut out, key.encode());
        put_varint(&mut out, o.rev);
        store::put_bytes(&mut out, &o.meta);
        put_varint(&mut out, o.len);
        store::put_ranges(&mut out, &o.have);
        for r in o.have.ranges() {
            out.extend_from_slice(o.data.get(r.start as usize..r.end as usize)?);
        }
        Some(out)
    }

    /// The phone as it was saved by [`Phone::save_state`] and
    /// [`Phone::save_object`]. Call [`Phone::connected`] once a connection
    /// is up.
    pub fn load<'a>(
        state: &[u8],
        objects: impl IntoIterator<Item = &'a [u8]>,
        cfg: Config,
        now: Millis,
    ) -> Result<Phone, DecodeError> {
        let mut r = Reader::new(state);
        store::version(&mut r)?;
        let token = store::token(&mut r)?;
        let epoch = r.varint()?;
        let mut phone = Phone::new(token, epoch, cfg, now);
        phone.next_entry = r.varint()?;
        for _ in 0..r.varint()? {
            let seq = r.varint()?;
            let kind = EntryKind::from_u8(r.u8()?).ok_or(DecodeError::Invalid("entry kind"))?;
            let payload = store::bytes(&mut r)?;
            if seq >= phone.next_entry {
                return Err(DecodeError::Invalid("entry number"));
            }
            let next = phone.next_entry;
            phone.next_entry = seq;
            phone.add(kind, payload);
            phone.next_entry = next;
        }
        for bytes in objects {
            let mut r = Reader::new(bytes);
            store::version(&mut r)?;
            let key = Key::decode(r.varint()?)?;
            let rev = r.varint()?;
            let meta = store::bytes(&mut r)?;
            let len = r.varint()?;
            let have = store::ranges(&mut r)?;
            let size = usize::try_from(len).map_err(|_| DecodeError::Invalid("saved length"))?;
            let mut data = vec![0; size];
            for range in have.ranges() {
                store::within(range, len)?;
                let src = r.bytes(range.end - range.start)?;
                if let Some(dst) = data.get_mut(range.start as usize..range.end as usize) {
                    dst.copy_from_slice(src);
                }
            }
            phone.ep.inbox.objects.insert(
                key,
                InObject {
                    rev,
                    meta,
                    len,
                    have,
                    data,
                },
            );
        }
        Ok(phone)
    }
}
