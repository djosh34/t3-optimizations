//! A paired phone: its token, its sync state and its connections.

use std::collections::BTreeMap;

use pipe::PeerId;
use sync_core::{Class, Config, Millis, Server};

use crate::app::log;
use crate::store::DeviceRecord;
use crate::world::{Kind, Obj, World};

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

    /// Puts objects of the world into this phone's sync state, in the
    /// class they have for this phone.
    pub fn sync(&mut self, world: &World, ids: impl IntoIterator<Item = u64>) {
        for id in ids {
            let Some(obj) = world.get(id) else { continue };
            let class = class_of(world, id, obj, self.focus.as_deref());
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
        self.sync(world, ids);
    }
}

/// The class of an object for a phone that has `focus` open (issue #16):
/// the newest assistant text first, then approvals and status, then older
/// messages, then the thread list, other threads, images.
pub fn class_of(world: &World, id: u64, obj: &Obj, focus: Option<&str>) -> Class {
    match obj.kind {
        Kind::Shell => return Class::ThreadList,
        Kind::Image => return Class::Image,
        Kind::Body | Kind::Text | Kind::Frame => {}
    }
    let current = obj.thread.as_deref().is_some_and(|t| Some(t) == focus);
    if !current {
        return Class::Background;
    }
    if obj.thread.as_deref().and_then(|t| world.tail(t)) == Some(id) {
        Class::Tail
    } else if obj.status {
        Class::Status
    } else {
        Class::History
    }
}
