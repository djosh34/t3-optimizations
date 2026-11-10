//! A paired phone: its token, its sync state and its connections.

use std::collections::BTreeMap;

use pipe::PeerId;
use sync_core::{Class, Config, Millis, Server};

use crate::app::log;
use crate::store::DeviceRecord;
use crate::world::{Kind, Obj, World, frame_id};

/// A connection to a phone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Link {
    /// The WebRTC data channel.
    Rtc(PeerId),
    /// The WebSocket fallback, by connection number.
    Ws(u64),
}

pub struct Device {
    pub record: DeviceRecord,
    pub token: [u8; 16],
    pub server: Server,
    /// The thread the phone has open; it gets the top classes.
    pub focus: Option<String>,
    /// The connection packets go out on: the newest one that sent a hello.
    pub link: Option<Link>,
    /// Data channel connections by generation, with their restart number.
    pub peers: BTreeMap<u64, (PeerId, u64)>,
    /// The sync state changed since it was saved.
    pub dirty: bool,
}

impl Device {
    pub fn new(record: DeviceRecord, token: [u8; 16], server: Server) -> Device {
        Device {
            focus: record.focus.clone(),
            record,
            token,
            server,
            link: None,
            peers: BTreeMap::new(),
            dirty: true,
        }
    }

    pub fn fresh_server(token: [u8; 16], now: Millis) -> Server {
        Server::new(token, Config::server(), now)
    }

    pub fn connected(&self) -> bool {
        self.link.is_some()
    }

    /// Whether the phone gets a thread's changes: it has the thread open,
    /// or has it from before (its frame).
    pub fn wants(&self, thread: &str) -> bool {
        self.focus.as_deref() == Some(thread) || self.server.has(frame_id(thread))
    }

    fn wants_object(&self, world: &World, id: u64, obj: &Obj) -> bool {
        match obj.kind {
            Kind::Config | Kind::Shell => true,
            Kind::Image => world.image_threads(id).any(|t| self.wants(t)),
            Kind::Body | Kind::Text | Kind::Frame => {
                obj.thread.as_deref().is_some_and(|t| self.wants(t))
            }
        }
    }

    /// Puts what changed in the world into this phone's sync state: the
    /// thread list and config, and the threads it wants.
    pub fn sync(&mut self, world: &World, ids: impl IntoIterator<Item = u64>) {
        let ids: Vec<u64> = ids
            .into_iter()
            .filter(|&id| {
                world
                    .get(id)
                    .is_some_and(|o| self.wants_object(world, id, o))
            })
            .collect();
        self.set(world, ids);
    }

    /// Gives the phone a thread it did not want before.
    pub fn give(&mut self, world: &World, thread: &str) {
        self.set(world, world.thread_ids(thread).collect::<Vec<_>>());
    }

    /// The world evicted these objects: what the phone has not stored of
    /// them waits until they come back.
    pub fn release(&mut self, ids: &[u64]) {
        for &id in ids {
            self.server.release(id);
        }
    }

    /// Puts objects of the world into this phone's sync state, in the
    /// class they have for this phone.
    fn set(&mut self, world: &World, ids: Vec<u64>) {
        for id in ids {
            let Some(obj) = world.get(id) else { continue };
            let class = class_of(world, id, obj, self.focus.as_deref());
            if class == Class::Tail && !self.server.has(id) {
                // A new message's newest text goes out before any backfill
                // not sent yet: the lab's acceptance report reads this line
                // (issue #46).
                log(format!(
                    "device {}: new newest text, backfill unsent {} B",
                    self.record.id,
                    self.server.stats().unsent_backfill_bytes
                ));
            }
            // The world checks ids and meta sizes; an error here is a bug
            // there, and the object is left out.
            match self
                .server
                .set(id, class, obj.order, obj.meta.clone(), obj.content.clone())
            {
                Ok(()) => self.dirty = true,
                Err(e) => log(format!(
                    "device {}: object {id} left out: {e}",
                    self.record.id
                )),
            }
        }
    }

    /// Opens a thread: its objects and the old thread's change class.
    pub fn set_focus(&mut self, world: &World, thread: Option<String>) {
        if self.focus == thread {
            return;
        }
        let old = std::mem::replace(&mut self.focus, thread);
        self.record.focus = self.focus.clone();
        let mut ids: Vec<u64> = Vec::new();
        for t in [old.as_deref(), self.focus.as_deref()]
            .into_iter()
            .flatten()
        {
            ids.extend(world.thread_ids(t));
        }
        self.set(world, ids);
    }
}

/// The class of an object for a phone that has `focus` open (issue #16):
/// the newest assistant text first, then approvals and status, then older
/// messages, then the thread list, other threads, images. The server's
/// config, the thread list's header and the projects go first among the
/// older messages; the open thread's row goes with its status.
pub fn class_of(world: &World, id: u64, obj: &Obj, focus: Option<&str>) -> Class {
    match obj.kind {
        // The open thread's row in the list: the client shows the thread
        // only once its row is here.
        Kind::Shell if obj.thread.is_some() && obj.thread.as_deref() == focus => {
            return Class::Status;
        }
        Kind::Shell => return Class::ThreadList,
        // The web app shows nothing before it has these. They go after the
        // open thread's newest text and status, before its older messages
        // (highest order first), without the repeats of the status class.
        Kind::Config => return Class::History,
        Kind::Image => return Class::Image,
        Kind::Body | Kind::Text | Kind::Frame => {}
    }
    let current = obj.thread.as_deref().is_some_and(|t| Some(t) == focus);
    if !current {
        return Class::Background;
    }
    let thread = obj.thread.as_deref();
    if thread.and_then(|t| world.tail(t)) == Some(id) {
        Class::Tail
    } else if obj.status || thread.is_some_and(|t| world.in_current_turn(t, id)) {
        // The current turn's messages and their texts go with status, also
        // once finished. Without the owner's prompt as t3 placed it, the
        // client shows the prompt it sent last and the reply above it, out
        // of view; without a finished item, it shows that message as still
        // streaming and holds back its last paragraph (issue #46).
        Class::Status
    } else {
        Class::History
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::world::object_id;

    #[test]
    fn what_the_web_app_needs_first_goes_before_the_open_threads_older_messages() {
        let mut w = World::default();
        w.set_shell(&json!({"kind": "snapshot", "snapshot": {"schemaVersion": 2,
            "snapshotSequence": 1, "projects": [{"id": "p1"}], "threads": [{"id": "t1"}, {"id": "t2"}],
            "archivedThreads": []}}));
        w.set_config(&json!({"environment": {}}));
        let class = |id: u64| {
            let o = w.get(id).expect("object");
            (class_of(&w, id, o, Some("t1")), o.order)
        };
        assert_eq!(class(object_id(&["shell"])), (Class::History, u64::MAX));
        assert_eq!(class(object_id(&["config"])), (Class::History, u64::MAX));
        assert_eq!(
            class(object_id(&["project", "p1"])),
            (Class::History, u64::MAX)
        );
        assert_eq!(
            class(object_id(&["thread", "active", "t1"])).0,
            Class::Status
        );
        assert_eq!(
            class(object_id(&["thread", "active", "t2"])).0,
            Class::ThreadList
        );
    }
}
