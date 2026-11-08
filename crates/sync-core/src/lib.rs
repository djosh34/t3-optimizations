//! The sync protocol between the sidecar and the phone (issue #43).
//!
//! The server side ([`Server`], one per paired phone) holds objects: message
//! texts, thread status, the thread list, images, each with an id, a class,
//! a revision and its bytes. It sends the phone what it has not stored yet,
//! highest class first, and never sends again what the phone acked. The
//! phone side ([`Phone`]) stores what arrives, acks it only once stored, and
//! keeps a log of the owner's commands and calls until their replies are
//! stored.
//!
//! The crate does no I/O and reads no clock: the caller passes the time in
//! milliseconds and moves packets between [`Server::poll`] /
//! [`Phone::poll`] and the other side's `receive`. It builds natively for
//! the sidecar and for `wasm32-unknown-unknown` for the web app. See
//! the crate's `README.md` for the protocol.

mod endpoint;
mod inbox;
mod outbox;
mod phone;
mod ranges;
mod rate;
mod server;
pub mod sim;
mod store;
pub mod wire;

pub use endpoint::{Config, Stats};
pub use inbox::{Delivered, InObject};
pub use outbox::{Busy, Class};
pub use phone::{EntryKind, EntryState, Phone, Received, Reply};
pub use ranges::RangeSet;
pub use rate::RateConfig;
pub use server::{Error, Incoming, MAX_META, Server};
pub use wire::{DecodeError, Key};

/// Milliseconds on a clock the caller chooses; only differences matter.
pub type Millis = u64;

/// The most application bytes in one packet. With DTLS, SCTP and IP headers
/// a packet stays inside the 1280-byte MTU of the Tailscale tunnel.
pub const MAX_PACKET: usize = 1100;

/// Bytes a packet costs on the wire beyond its application bytes: IPv4 20,
/// UDP 8, a DTLS 1.2 AES-GCM record 37 (header 13, nonce 8, tag 16), SCTP's
/// common header 12 and DATA chunk header 16. Rate control counts them, so
/// small packets do not overfill the link.
pub const WIRE_OVERHEAD: usize = 93;
