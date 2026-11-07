//! The phone's side of the data channel on str0m, doing what the web app
//! does in WebKit: a full ICE agent with its own UDP socket that offers one
//! unordered channel without retransmits, with ICE credentials derived from
//! its token ([`crate::creds`]) and one DTLS certificate it keeps.
//!
//! The first connection signals its offer over HTTPS and keeps the answer.
//! Later connections and ICE restarts need no signaling: the answer for them
//! is the kept one with the server's credentials for the new generation or
//! restart, as the web app makes it.

use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::Instant;

use str0m::change::{SdpAnswer, SdpPendingOffer};
use str0m::channel::{ChannelConfig, ChannelId, Reliability};
use str0m::config::DtlsCert;
use str0m::net::{Protocol, Receive};
use str0m::{Candidate, Event, IceCreds, Input, Output, Rtc, RtcError};

use crate::Error;
use crate::creds::{self, Side};

/// What happened on the connection.
#[derive(Debug, PartialEq, Eq)]
pub enum ClientEvent {
    Open,
    Data(Vec<u8>),
    IceState(String),
    /// The channel closed or str0m failed; the connection is over.
    Closed(String),
}

/// Who the phone is: what derives its ICE credentials.
#[derive(Debug, Clone)]
pub struct Identity {
    pub token: [u8; 16],
    pub device: String,
}

/// One connection, one `RTCPeerConnection` in the web app.
pub struct Client {
    rtc: Rtc,
    socket: UdpSocket,
    addr: SocketAddr,
    identity: Identity,
    generation: u64,
    restart: u64,
    pending: Option<SdpPendingOffer>,
    channel: Option<ChannelId>,
    closed: bool,
    /// Events that came up while `send` flushed, for the next `poll`.
    held: Vec<ClientEvent>,
    datagrams: u64,
    buf: Vec<u8>,
}

impl Client {
    /// A connection of generation `generation` on a new socket bound to
    /// `bind`. Use the same certificate for every generation.
    pub fn new(
        bind: SocketAddr,
        cert: DtlsCert,
        identity: Identity,
        generation: u64,
    ) -> Result<Client, Error> {
        let socket = UdpSocket::bind(bind)?;
        let addr = socket.local_addr()?;
        let local = creds::derive(
            &identity.token,
            &identity.device,
            Side::Phone,
            generation,
            0,
        );
        let mut rtc = Rtc::builder()
            .set_crypto_provider(Arc::new(str0m::crypto::from_feature_flags()))
            .set_dtls_cert(cert)
            .set_local_ice_credentials(local)
            .build(Instant::now());
        let candidate = Candidate::host(addr, "udp").map_err(RtcError::from)?;
        if rtc.add_local_candidate(candidate).is_none() {
            return Err(Error::Candidate(addr));
        }
        Ok(Client {
            rtc,
            socket,
            addr,
            identity,
            generation,
            restart: 0,
            pending: None,
            channel: None,
            closed: false,
            held: Vec::new(),
            datagrams: 0,
            buf: vec![0; 2000],
        })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn restart(&self) -> u64 {
        self.restart
    }

    /// The SDP offer for signaling. Call once, then [`Client::accept_answer`].
    pub fn offer(&mut self) -> Result<String, Error> {
        let mut change = self.rtc.sdp_api();
        change.add_channel_with_config(ChannelConfig {
            label: "sync".into(),
            ordered: false,
            reliability: Reliability::MaxRetransmits { retransmits: 0 },
            ..Default::default()
        });
        let (offer, pending) = change
            .apply()
            .ok_or(Error::Sdp("no offer to make".into()))?;
        self.pending = Some(pending);
        Ok(offer.to_sdp_string())
    }

    /// Takes the server's answer to [`Client::offer`].
    pub fn accept_answer(&mut self, answer: &str) -> Result<(), Error> {
        let pending = self
            .pending
            .take()
            .ok_or(Error::Sdp("no offer is waiting for an answer".into()))?;
        let answer = SdpAnswer::from_sdp_string(answer).map_err(|e| Error::Sdp(e.to_string()))?;
        self.rtc.sdp_api().accept_answer(pending, answer)?;
        Ok(())
    }

    /// Connects without signaling, from an answer the phone kept from an
    /// earlier connection.
    pub fn connect_cached(&mut self, kept_answer: &str) -> Result<(), Error> {
        let offer = self.offer()?;
        // WebKit's data channel is always mid "0"; str0m picks a new one
        // for every offer, which the kept answer must name.
        let answer = match sdp_mid(&offer) {
            Some(mid) => replace_mid(kept_answer, mid),
            None => kept_answer.to_owned(),
        };
        let answer = self.answer_for(&answer, self.restart);
        self.accept_answer(&answer)
    }

    /// Restarts ICE on this connection with the next restart number's
    /// credentials, from the kept answer, without signaling.
    pub fn restart_ice(&mut self, kept_answer: &str) -> Result<(), Error> {
        self.restart += 1;
        let mut change = self.rtc.sdp_api();
        change.ice_restart(true);
        let (_, pending) = change
            .apply()
            .ok_or(Error::Sdp("ICE restart made no offer".into()))?;
        self.pending = Some(pending);
        let answer = self.answer_for(kept_answer, self.restart);
        // str0m restarts ICE with random credentials when the answer is
        // accepted; the derived ones replace them after that.
        self.accept_answer(&answer)?;
        let local = creds::derive(
            &self.identity.token,
            &self.identity.device,
            Side::Phone,
            self.generation,
            self.restart,
        );
        self.rtc.direct_api().set_local_ice_credentials(local);
        Ok(())
    }

    /// The kept answer with the server's credentials for this generation
    /// and `restart`.
    fn answer_for(&self, kept: &str, restart: u64) -> String {
        let server = creds::derive(
            &self.identity.token,
            &self.identity.device,
            Side::Server,
            self.generation,
            restart,
        );
        replace_ice_credentials(kept, &server)
    }

    /// UDP datagrams read so far.
    pub fn datagrams(&self) -> u64 {
        self.datagrams
    }

    pub fn is_open(&self) -> bool {
        self.channel.is_some()
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Sends one message in a datagram of its own (see `Pipe::send`); false
    /// when SCTP's buffer is full or the channel is not open.
    pub fn send(&mut self, data: &[u8]) -> Result<bool, Error> {
        let Some(id) = self.channel else {
            return Ok(false);
        };
        let Some(mut channel) = self.rtc.channel(id) else {
            return Ok(false);
        };
        let written = channel.write(true, data)?;
        let mut held = std::mem::take(&mut self.held);
        let flushed = self.drain(&mut held);
        self.held = held;
        flushed?;
        Ok(written)
    }

    /// Closes the channel, as a page that unloads does.
    pub fn close(&mut self) {
        if let Some(id) = self.channel.take() {
            self.rtc.direct_api().close_data_channel(id);
        }
    }

    /// Sends what is due and reads the socket until `until`, or until there
    /// are new events, which it appends to `events`.
    pub fn poll(&mut self, until: Instant, events: &mut Vec<ClientEvent>) -> Result<(), Error> {
        let before = events.len();
        events.append(&mut self.held);
        loop {
            let next = self.drain(events)?;
            if events.len() > before || self.closed {
                return Ok(());
            }
            let now = Instant::now();
            let wake = next.map_or(until, |t| t.min(until));
            if wake <= now {
                self.input(Input::Timeout(now), events);
                if until <= now {
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
            self.datagrams += 1;
            let data = self.buf[..n].to_vec();
            let Ok(contents) = data.as_slice().try_into() else {
                continue;
            };
            let input = Input::Receive(
                Instant::now(),
                Receive {
                    proto: Protocol::Udp,
                    source,
                    destination: self.addr,
                    contents,
                },
            );
            if self.rtc.accepts(&input) {
                self.input(input, events);
            }
        }
    }

    fn input(&mut self, input: Input, events: &mut Vec<ClientEvent>) {
        if let Err(e) = self.rtc.handle_input(input) {
            self.fail(e.to_string(), events);
        }
    }

    fn fail(&mut self, why: String, events: &mut Vec<ClientEvent>) {
        if !self.closed {
            self.closed = true;
            self.channel = None;
            self.rtc.disconnect();
            events.push(ClientEvent::Closed(why));
        }
    }

    fn drain(&mut self, events: &mut Vec<ClientEvent>) -> Result<Option<Instant>, Error> {
        while self.rtc.is_alive() {
            let output = match self.rtc.poll_output() {
                Ok(output) => output,
                Err(e) => {
                    self.fail(e.to_string(), events);
                    return Ok(None);
                }
            };
            match output {
                Output::Timeout(t) => return Ok(Some(t)),
                Output::Transmit(t) => {
                    if let Err(e) = self.socket.send_to(&t.contents, t.destination) {
                        // Loss on a link that is down; SCTP and ICE retry.
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
                Output::Event(Event::ChannelOpen(id, _)) => {
                    self.channel = Some(id);
                    events.push(ClientEvent::Open);
                }
                Output::Event(Event::ChannelData(d)) => events.push(ClientEvent::Data(d.data)),
                Output::Event(Event::ChannelClose(_)) => {
                    self.fail("channel closed".into(), events);
                }
                Output::Event(Event::IceConnectionStateChange(state)) => {
                    events.push(ClientEvent::IceState(format!("{state:?}")));
                }
                Output::Event(_) => {}
            }
        }
        if !self.closed {
            self.fail("connection ended".into(), events);
        }
        Ok(None)
    }
}

/// The media id of the first `m=` section.
fn sdp_mid(sdp: &str) -> Option<&str> {
    sdp.lines().find_map(|l| l.strip_prefix("a=mid:"))
}

/// An SDP with its first media id, and the BUNDLE group naming it, set to
/// `mid`.
fn replace_mid(sdp: &str, mid: &str) -> String {
    let Some(old) = sdp_mid(sdp) else {
        return sdp.to_owned();
    };
    sdp.replace(&format!("a=mid:{old}"), &format!("a=mid:{mid}"))
        .replace(
            &format!("a=group:BUNDLE {old}"),
            &format!("a=group:BUNDLE {mid}"),
        )
}

/// An SDP with every `a=ice-ufrag` and `a=ice-pwd` set to `creds`, and the
/// old fragment replaced wherever else it appears (candidate lines).
pub fn replace_ice_credentials(sdp: &str, creds: &IceCreds) -> String {
    let old = sdp
        .lines()
        .find_map(|l| l.strip_prefix("a=ice-ufrag:"))
        .map(str::to_owned);
    let mut out = String::with_capacity(sdp.len());
    for line in sdp.split_inclusive('\n') {
        let end = if line.ends_with("\r\n") {
            "\r\n"
        } else if line.ends_with('\n') {
            "\n"
        } else {
            ""
        };
        if line.starts_with("a=ice-ufrag:") {
            out.push_str(&format!("a=ice-ufrag:{}{end}", creds.ufrag));
        } else if line.starts_with("a=ice-pwd:") {
            out.push_str(&format!("a=ice-pwd:{}{end}", creds.pass));
        } else if let Some(old) = old.as_deref().filter(|o| !o.is_empty()) {
            out.push_str(&line.replace(old, &creds.ufrag));
        } else {
            out.push_str(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_credentials_and_candidate_fragments() {
        let sdp = "v=0\r\na=ice-ufrag:old1\r\na=ice-pwd:oldpass\r\na=candidate:1 1 udp 1 10.0.0.1 9 typ host ufrag old1\r\n";
        let creds = IceCreds {
            ufrag: "new+s1+0".into(),
            pass: "newpass".into(),
        };
        assert_eq!(
            replace_ice_credentials(sdp, &creds),
            "v=0\r\na=ice-ufrag:new+s1+0\r\na=ice-pwd:newpass\r\na=candidate:1 1 udp 1 10.0.0.1 9 typ host ufrag new+s1+0\r\n"
        );
    }
}
