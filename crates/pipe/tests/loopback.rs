//! A str0m client in the browser's role connects to the pipe over loopback
//! UDP, the way the lab's WebKit does: it offers an unordered data channel
//! without retransmits, and both ends exchange ticks.

use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::{Duration, Instant};

use pipe::tick::Tick;
use pipe::{Config, Pipe, PipeEvent};
use str0m::change::SdpAnswer;
use str0m::channel::{ChannelConfig, ChannelId, Reliability};
use str0m::net::{Protocol, Receive};
use str0m::{Candidate, Event, Input, Output, Rtc};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The browser's side: a full-ICE str0m peer on its own socket.
struct Client {
    rtc: Rtc,
    socket: UdpSocket,
    addr: SocketAddr,
    events: Vec<Event>,
}

impl Client {
    fn new() -> Result<Client, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        let addr = socket.local_addr()?;
        let mut rtc = Rtc::builder()
            .set_crypto_provider(Arc::new(str0m::crypto::from_feature_flags()))
            .build(Instant::now());
        rtc.add_local_candidate(Candidate::host(addr, "udp")?)
            .ok_or("client candidate refused")?;
        Ok(Client {
            rtc,
            socket,
            addr,
            events: Vec::new(),
        })
    }

    /// Sends what is due and reads the socket for up to `wait`.
    fn step(&mut self, wait: Duration) -> TestResult {
        loop {
            match self.rtc.poll_output()? {
                Output::Transmit(t) => {
                    self.socket.send_to(&t.contents, t.destination)?;
                }
                Output::Event(e) => self.events.push(e),
                Output::Timeout(_) => break,
            }
        }
        self.socket.set_read_timeout(Some(wait))?;
        let mut buf = [0u8; 2000];
        match self.socket.recv_from(&mut buf) {
            Ok((n, source)) => {
                let input = Input::Receive(
                    Instant::now(),
                    Receive {
                        proto: Protocol::Udp,
                        source,
                        destination: self.addr,
                        contents: buf[..n].try_into()?,
                    },
                );
                self.rtc.handle_input(input)?;
            }
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                self.rtc.handle_input(Input::Timeout(Instant::now()))?;
            }
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }

    fn open_channel(&self) -> Option<ChannelId> {
        self.events.iter().find_map(|e| match e {
            Event::ChannelOpen(id, _) => Some(*id),
            _ => None,
        })
    }

    fn ticks(&self) -> Vec<Tick> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::ChannelData(d) => Tick::decode(&d.data),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn ticks_cross_both_ways() -> TestResult {
    let mut pipe = Pipe::bind(Config {
        bind: "127.0.0.1:0".parse()?,
        advertise: None,
    })?;
    let mut client = Client::new()?;

    let mut change = client.rtc.sdp_api();
    change.add_channel_with_config(ChannelConfig {
        label: "pipe".into(),
        ordered: false,
        reliability: Reliability::MaxRetransmits { retransmits: 0 },
        ..Default::default()
    });
    let (offer, pending) = change.apply().ok_or("no offer")?;
    let (peer, answer) = pipe.accept_offer(&offer.to_sdp_string())?;
    assert!(
        answer.contains("a=ice-lite"),
        "answer is not ICE-lite:\n{answer}"
    );
    assert!(
        answer.contains("a=setup:passive"),
        "server is not the DTLS server:\n{answer}"
    );
    assert!(answer.contains(&format!("{} typ host", pipe.host().port())));
    client
        .rtc
        .sdp_api()
        .accept_answer(pending, SdpAnswer::from_sdp_string(&answer)?)?;

    let deadline = Instant::now() + Duration::from_secs(10);
    let mut server_events = Vec::new();
    let mut server_ticks = Vec::new();
    let mut server_open = false;
    let mut seq = 0;
    while Instant::now() < deadline {
        pipe.poll(
            Instant::now() + Duration::from_millis(5),
            &mut server_events,
        )?;
        for ev in server_events.drain(..) {
            match ev {
                PipeEvent::Open(id) => {
                    assert_eq!(id, peer);
                    server_open = true;
                }
                PipeEvent::Data(_, data) => server_ticks.extend(Tick::decode(&data)),
                _ => {}
            }
        }
        client.step(Duration::from_millis(5))?;

        if let (true, Some(id), 0..3) = (server_open, client.open_channel(), seq) {
            let tick = Tick { seq, sent_ms: 1 };
            assert!(pipe.send(peer, &tick.encode())?);
            let mut channel = client.rtc.channel(id).ok_or("client channel gone")?;
            assert!(channel.write(true, &tick.encode())?);
            seq += 1;
        }
        if client.ticks().len() == 3 && server_ticks.len() == 3 {
            assert_eq!(pipe.open_peers(), vec![peer]);
            return Ok(());
        }
    }
    Err(format!(
        "after 10 s: server open {server_open}, client got {:?}, server got {server_ticks:?}",
        client.ticks()
    )
    .into())
}

#[test]
fn bad_offer_is_an_error() -> TestResult {
    let mut pipe = Pipe::bind(Config {
        bind: "127.0.0.1:0".parse()?,
        advertise: None,
    })?;
    assert!(pipe.accept_offer("not sdp").is_err());
    Ok(())
}
