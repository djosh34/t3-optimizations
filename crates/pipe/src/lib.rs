//! Server side of the WebRTC data channel to the phone, on str0m.
//!
//! One UDP socket on a configured port serves every connection. The server
//! is ICE-lite with one host candidate, takes the DTLS server role (the
//! browser's offer says `actpass`, the answer `passive`) and skips the DTLS
//! cookie exchange: ICE has already proven the browser's address.
//!
//! A connection starts from the browser's SDP offer ([`Pipe::accept_offer`])
//! or, with no signaling at all, from credentials both sides already know
//! ([`Pipe::connect`], see [`creds`]). A STUN request that no connection
//! takes is reported as [`PipeEvent::Unknown`]; the caller answers it with
//! `connect` or [`Pipe::restart_ice`] and the pipe then hands the request
//! to that connection.
//!
//! SCTP is str0m's, with the patched sctp-proto from `crates/vendor`: the
//! retransmit timeout is capped at 2 s and loss never shrinks the congestion
//! window. Nothing here closes a connection on a timeout: one ends when the
//! browser closes its channel, on an error, or by [`Pipe::close`].

use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};
use str0m::change::SdpOffer;
use str0m::channel::ChannelId;
use str0m::config::{DtlsCert, Fingerprint};
use str0m::ice::StunMessage;
use str0m::net::{Protocol, Receive};
use str0m::{Candidate, Event, IceCreds, Input, Output, Rtc, RtcConfig, RtcError};

pub mod client;
pub mod creds;
pub mod tick;

/// Where the server listens and what it tells the browser.
#[derive(Debug, Clone)]
pub struct Config {
    /// The UDP address the socket binds. Port 0 takes any free port.
    pub bind: SocketAddr,
    /// The address of the host candidate in the answer, the one the browser
    /// sends to, when the socket binds a wildcard address such as 0.0.0.0.
    /// None uses the bound address. The port is always the socket's.
    pub advertise: Option<IpAddr>,
    /// One DTLS certificate for every connection, so a browser that kept an
    /// answer can check the server without signaling. None gives every
    /// connection a new one.
    pub cert: Option<DtlsCert>,
}

/// A connection without SDP: both sides' ICE credentials and the browser's
/// certificate are known in advance.
#[derive(Debug, Clone)]
pub struct Direct {
    pub local: IceCreds,
    pub remote: IceCreds,
    /// The browser's DTLS certificate. None accepts any certificate.
    pub remote_fingerprint: Option<Fingerprint>,
}

/// A connection, numbered in the order it was made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    /// str0m failed on this connection, for example on a bad packet from
    /// the browser. `Closed` follows; the other connections go on.
    Failed(PeerId, String),
    /// The connection is gone: the browser closed the data channel, the
    /// connection failed, or [`Pipe::close`] ended it.
    Closed(PeerId),
    /// A STUN binding request no connection takes, with its username
    /// fragments. Answer with [`Pipe::connect`] or [`Pipe::restart_ice`]
    /// within a few seconds and the request goes to that connection.
    Unknown {
        local_ufrag: String,
        remote_ufrag: String,
    },
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
    /// The crypto provider could not make a certificate.
    Certificate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Rtc(e) => write!(f, "{e}"),
            Error::Sdp(e) => write!(f, "invalid SDP offer: {e}"),
            Error::NoPeer(id) => write!(f, "no open data channel for peer {}", id.0),
            Error::Candidate(addr) => write!(f, "host candidate {addr} refused"),
            Error::Certificate => write!(f, "could not generate a DTLS certificate"),
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

/// A new self-signed DTLS certificate.
pub fn generate_cert() -> Result<DtlsCert, Error> {
    str0m::crypto::from_feature_flags()
        .dtls_provider
        .generate_certificate()
        .ok_or(Error::Certificate)
}

/// The SHA-256 fingerprint of a certificate, as SDP's `a=fingerprint` has it.
pub fn fingerprint(cert: &DtlsCert) -> Fingerprint {
    Fingerprint {
        hash_func: "sha-256".into(),
        bytes: Sha256::digest(&cert.certificate).to_vec(),
    }
}

/// A STUN request no connection took, kept a moment for the connection
/// the caller makes for it.
struct Unknown {
    at: Instant,
    source: SocketAddr,
    data: Vec<u8>,
}

/// How long an unknown request waits for its connection.
const UNKNOWN_KEEP: Duration = Duration::from_secs(5);
/// The most unknown requests kept; the oldest goes first.
const UNKNOWN_MAX: usize = 16;

struct Peer {
    id: PeerId,
    rtc: Rtc,
    channel: Option<ChannelId>,
    /// The server's current ICE username fragment on this connection.
    ufrag: String,
}

impl Peer {
    /// Ends this connection when str0m failed on it, so one browser's error
    /// does not stop the others. `drain` then reports it closed.
    fn check(&mut self, result: Result<(), RtcError>, events: &mut Vec<PipeEvent>) {
        if let Err(e) = result {
            events.push(PipeEvent::Failed(self.id, e.to_string()));
            self.rtc.disconnect();
        }
    }
}

/// The server: one UDP socket and its connections.
pub struct Pipe {
    socket: UdpSocket,
    host: SocketAddr,
    cert: Option<DtlsCert>,
    peers: Vec<Peer>,
    unknown: Vec<Unknown>,
    /// Events that came up while `send` flushed, for the next `poll`.
    held: Vec<PipeEvent>,
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
            cert: config.cert,
            peers: Vec::new(),
            unknown: Vec::new(),
            held: Vec::new(),
            next_id: 1,
            buf: vec![0; 2000],
        })
    }

    fn rtc_config(&self, local: Option<IceCreds>) -> RtcConfig {
        let mut config = Rtc::builder()
            .set_crypto_provider(Arc::new(str0m::crypto::from_feature_flags()))
            .set_ice_lite(true);
        if let Some(cert) = &self.cert {
            config = config.set_dtls_cert(cert.clone());
        }
        if let Some(local) = local {
            config = config.set_local_ice_credentials(local);
        }
        config
    }

    fn add(&mut self, mut rtc: Rtc) -> Result<PeerId, Error> {
        let candidate = Candidate::host(self.host, "udp").map_err(RtcError::from)?;
        if rtc.add_local_candidate(candidate).is_none() {
            return Err(Error::Candidate(self.host));
        }
        let id = PeerId(self.next_id);
        self.next_id += 1;
        let ufrag = rtc.direct_api().local_ice_credentials().ufrag;
        self.peers.push(Peer {
            id,
            rtc,
            channel: None,
            ufrag,
        });
        Ok(id)
    }

    /// Accepts a browser's SDP offer and returns the new connection and the
    /// SDP answer for the browser. `local` sets the server's ICE
    /// credentials; None picks random ones.
    pub fn accept_offer(
        &mut self,
        offer: &str,
        local: Option<IceCreds>,
    ) -> Result<(PeerId, String), Error> {
        let offer = SdpOffer::from_sdp_string(offer).map_err(|e| Error::Sdp(e.to_string()))?;
        let rtc = self.rtc_config(local).build(Instant::now());
        let id = self.add(rtc)?;
        let peer = self.peer(id)?;
        let answer = peer.rtc.sdp_api().accept_offer(offer)?;
        Ok((id, answer.to_sdp_string()))
    }

    /// Makes a connection without SDP. The browser connects to the server's
    /// host candidate with the credentials and certificate the caller gives.
    pub fn connect(&mut self, direct: Direct) -> Result<PeerId, Error> {
        let config = self
            .rtc_config(Some(direct.local))
            .set_fingerprint_verification(direct.remote_fingerprint.is_some());
        let mut rtc = config.build(Instant::now());
        let mut api = rtc.direct_api();
        api.set_ice_controlling(false);
        api.set_remote_ice_credentials(direct.remote);
        if let Some(fingerprint) = direct.remote_fingerprint {
            api.set_remote_fingerprint(fingerprint);
        }
        api.start_dtls(false)?;
        api.start_sctp(false);
        self.add(rtc)
    }

    /// Restarts ICE on a connection with new credentials, keeping its DTLS
    /// and SCTP state.
    pub fn restart_ice(
        &mut self,
        id: PeerId,
        local: IceCreds,
        remote: IceCreds,
    ) -> Result<(), Error> {
        let peer = self.peer(id)?;
        peer.ufrag = local.ufrag.clone();
        let mut api = peer.rtc.direct_api();
        api.set_local_ice_credentials(local);
        api.set_remote_ice_credentials(remote);
        Ok(())
    }

    /// Ends a connection at once, without telling the browser; for one a
    /// newer connection of the same browser replaced. `Closed` follows.
    pub fn close(&mut self, id: PeerId) {
        if let Ok(peer) = self.peer(id) {
            peer.rtc.disconnect();
        }
    }

    /// The server's ICE credentials on a connection.
    pub fn local_credentials(&mut self, id: PeerId) -> Result<IceCreds, Error> {
        Ok(self.peer(id)?.rtc.direct_api().local_ice_credentials())
    }

    fn peer(&mut self, id: PeerId) -> Result<&mut Peer, Error> {
        self.peers
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(Error::NoPeer(id))
    }

    /// Sends one message to the browser, in a UDP datagram of its own.
    /// Returns false when SCTP's send buffer is full and dropped it.
    ///
    /// SCTP bundles messages written together into one datagram of up to the
    /// MTU. The sync protocol sizes its packets to the link, small on a slow
    /// one, so a bundle loses several packets at once and fills a queue that
    /// counts packets with full-size ones: at 10 kbit/s in the lab a queue
    /// of 20 held 18 s. Sending what SCTP has after each message keeps one
    /// message per datagram.
    pub fn send(&mut self, id: PeerId, data: &[u8]) -> Result<bool, Error> {
        let peer = self.peer(id)?;
        let channel = peer.channel.ok_or(Error::NoPeer(id))?;
        let mut channel = peer.rtc.channel(channel).ok_or(Error::NoPeer(id))?;
        let written = channel.write(true, data)?;
        // Events of this flush wait for the next `poll`.
        let mut held = std::mem::take(&mut self.held);
        let flushed = self.drain(&mut held);
        self.held = held;
        flushed?;
        Ok(written)
    }

    /// Bytes queued in SCTP for a connection, not sent yet.
    pub fn buffered(&mut self, id: PeerId) -> Option<usize> {
        let peer = self.peer(id).ok()?;
        let channel = peer.channel?;
        Some(peer.rtc.channel(channel)?.buffered_amount())
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
    /// are new events, which it appends to `events`.
    pub fn poll(&mut self, until: Instant, events: &mut Vec<PipeEvent>) -> Result<(), Error> {
        let before = events.len();
        events.append(&mut self.held);
        self.replay_unknown(events);
        loop {
            let next = self.drain(events)?;
            if events.len() > before {
                return Ok(());
            }
            let now = Instant::now();
            let wake = next.map_or(until, |t| t.min(until));
            if wake <= now {
                for peer in &mut self.peers {
                    let result = peer.rtc.handle_input(Input::Timeout(now));
                    peer.check(result, events);
                }
                if until <= now {
                    // Sends what the timeouts made due and reports a peer
                    // that failed on them as closed too.
                    self.drain(events)?;
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
            let data = self.buf[..n].to_vec();
            if !self.deliver(source, &data, events) {
                self.keep_unknown(source, data, events);
            }
        }
    }

    /// Hands a datagram to the connection that takes it. False when none
    /// does.
    fn deliver(&mut self, source: SocketAddr, data: &[u8], events: &mut Vec<PipeEvent>) -> bool {
        let Ok(contents) = data.try_into() else {
            // Not STUN, DTLS or RTP: noise on the port.
            return true;
        };
        // A binding request goes by its username, not by its address: str0m
        // takes anything from a connection's current address, and would
        // drop the request of an ICE restart from the same address.
        let ufrag = StunMessage::parse(data)
            .ok()
            .filter(|m| m.is_binding_request())
            .and_then(|m| m.split_username().map(|(local, _)| local.to_owned()));
        let input = Input::Receive(
            Instant::now(),
            Receive {
                proto: Protocol::Udp,
                source,
                destination: self.host,
                contents,
            },
        );
        let peer = match &ufrag {
            Some(ufrag) => self.peers.iter_mut().find(|p| &p.ufrag == ufrag),
            None => self.peers.iter_mut().find(|p| p.rtc.accepts(&input)),
        };
        match peer {
            Some(peer) => {
                let result = peer.rtc.handle_input(input);
                peer.check(result, events);
                true
            }
            None => false,
        }
    }

    /// Reports a STUN binding request no connection took, and keeps it for
    /// the connection the caller makes. Anything else is dropped.
    fn keep_unknown(&mut self, source: SocketAddr, data: Vec<u8>, events: &mut Vec<PipeEvent>) {
        let Ok(message) = StunMessage::parse(&data) else {
            return;
        };
        if !message.is_binding_request() {
            return;
        }
        let Some((local, remote)) = message.split_username() else {
            return;
        };
        events.push(PipeEvent::Unknown {
            local_ufrag: local.to_owned(),
            remote_ufrag: remote.to_owned(),
        });
        if self.unknown.len() == UNKNOWN_MAX {
            self.unknown.remove(0);
        }
        self.unknown.push(Unknown {
            at: Instant::now(),
            source,
            data,
        });
    }

    /// Hands kept requests to connections made for them since.
    fn replay_unknown(&mut self, events: &mut Vec<PipeEvent>) {
        let now = Instant::now();
        let kept = std::mem::take(&mut self.unknown);
        for u in kept {
            if now.duration_since(u.at) > UNKNOWN_KEEP {
                continue;
            }
            if !self.deliver(u.source, &u.data, events) {
                self.unknown.push(u);
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
                let output = match peer.rtc.poll_output() {
                    Ok(output) => output,
                    Err(e) => {
                        peer.check(Err(e), events);
                        break;
                    }
                };
                match output {
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
                        // A connection carries one channel, so its close ends
                        // the connection.
                        peer.channel = None;
                        peer.rtc.disconnect();
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
