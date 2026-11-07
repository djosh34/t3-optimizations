//! Server side of the WebRTC data channel to the phone, on str0m.
//!
//! One UDP socket on a configured port serves every connection. The server
//! is ICE-lite with one host candidate, takes the DTLS server role (the
//! browser's offer says `actpass`, the answer `passive`) and skips the DTLS
//! cookie exchange: ICE has already proven the browser's address. Every
//! connection gets its own random ICE credentials and DTLS certificate.
//!
//! SCTP is str0m's, with the patched sctp-proto from `crates/vendor`: the
//! retransmit timeout is capped at 2 s and loss never shrinks the congestion
//! window.

use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::Instant;

use str0m::change::SdpOffer;
use str0m::channel::ChannelId;
use str0m::net::{Protocol, Receive};
use str0m::{Candidate, Event, Input, Output, Rtc, RtcError};

pub mod tick;

/// Where the server listens and what it tells the browser.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// The UDP address the socket binds. Port 0 takes any free port.
    pub bind: SocketAddr,
    /// The address of the host candidate in the answer, the one the browser
    /// sends to, when the socket binds a wildcard address such as 0.0.0.0.
    /// None uses the bound address. The port is always the socket's.
    pub advertise: Option<IpAddr>,
}

/// A connection, numbered in the order the offers came in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId(pub u64);

/// What happened on a connection.
#[derive(Debug, PartialEq, Eq)]
pub enum PipeEvent {
    /// The browser's data channel is open; `send` works from now on.
    Open(PeerId),
    /// A message from the browser.
    Data(PeerId, Vec<u8>),
    /// A change of the connection's ICE state, for logs.
    IceState(PeerId, String),
    /// The connection is gone.
    Closed(PeerId),
}

/// An error of the pipe.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Rtc(RtcError),
    /// The offer is not valid SDP.
    Sdp(String),
    /// No open connection with this id.
    NoPeer(PeerId),
    /// str0m refused the host candidate.
    Candidate(SocketAddr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Rtc(e) => write!(f, "{e}"),
            Error::Sdp(e) => write!(f, "invalid SDP offer: {e}"),
            Error::NoPeer(id) => write!(f, "no open data channel for peer {}", id.0),
            Error::Candidate(addr) => write!(f, "host candidate {addr} refused"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<RtcError> for Error {
    fn from(e: RtcError) -> Self {
        Error::Rtc(e)
    }
}

struct Peer {
    id: PeerId,
    rtc: Rtc,
    channel: Option<ChannelId>,
}

/// The server: one UDP socket and its connections.
pub struct Pipe {
    socket: UdpSocket,
    host: SocketAddr,
    peers: Vec<Peer>,
    next_id: u64,
    buf: Vec<u8>,
}

impl Pipe {
    /// Binds the UDP socket.
    pub fn bind(config: Config) -> Result<Pipe, Error> {
        let socket = UdpSocket::bind(config.bind)?;
        let mut host = socket.local_addr()?;
        if let Some(ip) = config.advertise {
            host.set_ip(ip);
        }
        Ok(Pipe {
            socket,
            host,
            peers: Vec::new(),
            next_id: 1,
            buf: vec![0; 2000],
        })
    }

    /// Accepts a browser's SDP offer and returns the new connection and the
    /// SDP answer for the browser.
    pub fn accept_offer(&mut self, offer: &str) -> Result<(PeerId, String), Error> {
        let offer = SdpOffer::from_sdp_string(offer).map_err(|e| Error::Sdp(e.to_string()))?;
        let mut rtc = Rtc::builder()
            .set_crypto_provider(Arc::new(str0m::crypto::from_feature_flags()))
            .set_ice_lite(true)
            .build(Instant::now());
        let candidate = Candidate::host(self.host, "udp").map_err(RtcError::from)?;
        if rtc.add_local_candidate(candidate).is_none() {
            return Err(Error::Candidate(self.host));
        }
        let answer = rtc.sdp_api().accept_offer(offer)?;
        let id = PeerId(self.next_id);
        self.next_id += 1;
        self.peers.push(Peer {
            id,
            rtc,
            channel: None,
        });
        Ok((id, answer.to_sdp_string()))
    }

    /// Queues one message to the browser. It leaves on the next `poll`.
    /// Returns false when SCTP's send buffer is full and dropped it.
    pub fn send(&mut self, id: PeerId, data: &[u8]) -> Result<bool, Error> {
        let peer = self
            .peers
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(Error::NoPeer(id))?;
        let channel = peer.channel.ok_or(Error::NoPeer(id))?;
        let mut channel = peer.rtc.channel(channel).ok_or(Error::NoPeer(id))?;
        Ok(channel.write(true, data)?)
    }

    /// The host candidate's address: where browsers send to.
    pub fn host(&self) -> SocketAddr {
        self.host
    }

    /// Ids of the connections with an open data channel.
    pub fn open_peers(&self) -> Vec<PeerId> {
        self.peers
            .iter()
            .filter(|p| p.channel.is_some())
            .map(|p| p.id)
            .collect()
    }

    /// Sends what is due and reads the socket until `until`, or until there
    /// are events, which it appends to `events`.
    pub fn poll(&mut self, until: Instant, events: &mut Vec<PipeEvent>) -> Result<(), Error> {
        loop {
            let next = self.drain(events)?;
            if !events.is_empty() {
                return Ok(());
            }
            let now = Instant::now();
            let wake = next.map_or(until, |t| t.min(until));
            if wake <= now {
                for peer in &mut self.peers {
                    peer.rtc.handle_input(Input::Timeout(now))?;
                }
                if until <= now {
                    return Ok(());
                }
                continue;
            }
            self.socket.set_read_timeout(Some(wake - now))?;
            let (n, source) = match self.socket.recv_from(&mut self.buf) {
                Ok(v) => v,
                Err(e)
                    if matches!(
                        e.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                    ) =>
                {
                    continue;
                }
                Err(e) => return Err(e.into()),
            };
            let Ok(contents) = self.buf[..n].try_into() else {
                // Not STUN, DTLS or RTP: noise on the port.
                continue;
            };
            let input = Input::Receive(
                Instant::now(),
                Receive {
                    proto: Protocol::Udp,
                    source,
                    destination: self.host,
                    contents,
                },
            );
            if let Some(peer) = self.peers.iter_mut().find(|p| p.rtc.accepts(&input)) {
                peer.rtc.handle_input(input)?;
            }
        }
    }

    /// Sends every connection's queued packets, collects its events and
    /// drops closed connections. Returns the earliest time a connection
    /// wants to be woken.
    fn drain(&mut self, events: &mut Vec<PipeEvent>) -> Result<Option<Instant>, Error> {
        let mut next: Option<Instant> = None;
        for peer in &mut self.peers {
            while peer.rtc.is_alive() {
                match peer.rtc.poll_output()? {
                    Output::Timeout(t) => {
                        next = Some(next.map_or(t, |n| n.min(t)));
                        break;
                    }
                    Output::Transmit(t) => {
                        if let Err(e) = self.socket.send_to(&t.contents, t.destination) {
                            // A full socket buffer or an unreachable phone is
                            // loss, which SCTP handles; anything else is ours.
                            if !matches!(
                                e.kind(),
                                io::ErrorKind::WouldBlock
                                    | io::ErrorKind::ConnectionRefused
                                    | io::ErrorKind::NetworkUnreachable
                                    | io::ErrorKind::HostUnreachable
                            ) {
                                return Err(e.into());
                            }
                        }
                    }
                    Output::Event(Event::ChannelOpen(channel, _)) => {
                        peer.channel = Some(channel);
                        events.push(PipeEvent::Open(peer.id));
                    }
                    Output::Event(Event::ChannelData(d)) => {
                        events.push(PipeEvent::Data(peer.id, d.data));
                    }
                    Output::Event(Event::ChannelClose(_)) => {
                        peer.channel = None;
                    }
                    Output::Event(Event::IceConnectionStateChange(state)) => {
                        events.push(PipeEvent::IceState(peer.id, format!("{state:?}")));
                    }
                    Output::Event(_) => {}
                }
            }
        }
        self.peers.retain(|p| {
            let alive = p.rtc.is_alive();
            if !alive {
                events.push(PipeEvent::Closed(p.id));
            }
            alive
        });
        Ok(next)
    }
}
