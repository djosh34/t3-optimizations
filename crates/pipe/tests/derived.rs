//! Connections without signaling over loopback, with `pipe::client` in the
//! web app's role: a cold connect from a kept answer while the old
//! connection is still alive, and an ICE restart on the same connection.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use pipe::client::{Client, ClientEvent, Identity};
use pipe::creds::{self, Side, Ufrag};
use pipe::{Config, Direct, PeerId, Pipe, PipeEvent, fingerprint, generate_cert};
use str0m::config::DtlsCert;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const DEVICE: &str = "dev1test";

fn identity() -> Identity {
    Identity {
        token: [42; 16],
        device: DEVICE.into(),
    }
}

/// The server of the test: the pipe and how it answers unknown requests,
/// as the sidecar does.
struct Server {
    pipe: Pipe,
    phone_cert: DtlsCert,
    /// Connections by generation.
    peers: BTreeMap<u64, PeerId>,
    opened: Vec<PeerId>,
    closed: Vec<PeerId>,
    data: Vec<(PeerId, Vec<u8>)>,
}

impl Server {
    fn new(phone_cert: DtlsCert) -> Result<Server, Box<dyn std::error::Error>> {
        let pipe = Pipe::bind(Config {
            bind: "127.0.0.1:0".parse()?,
            advertise: None,
            cert: Some(generate_cert()?),
        })?;
        Ok(Server {
            pipe,
            phone_cert,
            peers: BTreeMap::new(),
            opened: Vec::new(),
            closed: Vec::new(),
            data: Vec::new(),
        })
    }

    fn step(&mut self) -> TestResult {
        let mut events = Vec::new();
        self.pipe
            .poll(Instant::now() + Duration::from_millis(2), &mut events)?;
        for ev in events {
            match ev {
                PipeEvent::Open(id) => self.opened.push(id),
                PipeEvent::Closed(id) => self.closed.push(id),
                PipeEvent::Data(id, d) => self.data.push((id, d)),
                PipeEvent::Unknown { local_ufrag, .. } => self.resolve(&local_ufrag)?,
                PipeEvent::IceState(..) | PipeEvent::DtlsUp(..) | PipeEvent::Failed(..) => {}
            }
        }
        Ok(())
    }

    fn resolve(&mut self, local_ufrag: &str) -> TestResult {
        let u = Ufrag::parse(local_ufrag).ok_or("not a derived fragment")?;
        assert_eq!((u.device.as_str(), u.side), (DEVICE, Side::Server));
        let id = identity();
        let local = creds::derive(&id.token, DEVICE, Side::Server, u.generation, u.restart);
        let remote = creds::derive(&id.token, DEVICE, Side::Phone, u.generation, u.restart);
        assert!(self.pipe.unknown_authentic(local_ufrag, &local.pass));
        match self.peers.get(&u.generation) {
            Some(&peer) => self.pipe.restart_ice(peer, local, remote)?,
            None => {
                let peer = self.pipe.connect(Direct {
                    local,
                    remote,
                    remote_fingerprint: Some(fingerprint(&self.phone_cert)),
                })?;
                self.peers.insert(u.generation, peer);
            }
        }
        Ok(())
    }
}

fn step_client(client: &mut Client, out: &mut Vec<ClientEvent>) -> TestResult {
    client.poll(Instant::now() + Duration::from_millis(2), out)?;
    Ok(())
}

/// Runs both until `done` or 10 s pass.
fn run_until(
    stage: &str,
    server: &mut Server,
    clients: &mut [(&mut Client, &mut Vec<ClientEvent>)],
    mut done: impl FnMut(&Server, &[(&mut Client, &mut Vec<ClientEvent>)]) -> bool,
) -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        server.step()?;
        for (client, events) in clients.iter_mut() {
            step_client(client, events)?;
        }
        if done(server, clients) {
            return Ok(());
        }
    }
    Err(format!(
        "{stage}: not done within 10 s; server opened {:?}, closed {:?}, peers {:?}, client events {:?}",
        server.opened,
        server.closed,
        server.peers,
        clients.iter().map(|(_, e)| format!("{e:?}")).collect::<Vec<_>>()
    )
    .into())
}

fn opened(events: &[ClientEvent]) -> bool {
    events.contains(&ClientEvent::Open)
}

#[test]
fn cold_connect_and_ice_restart_without_signaling() -> TestResult {
    let phone_cert = generate_cert()?;
    let mut server = Server::new(phone_cert.clone())?;

    // Generation 1 signals its offer, as the first connection does.
    let mut first = Client::new("127.0.0.1:0".parse()?, phone_cert.clone(), identity(), 1)?;
    let offer = first.offer()?;
    let local = creds::derive(&[42; 16], DEVICE, Side::Server, 1, 0);
    let (peer1, answer) = server.pipe.accept_offer(&offer, Some(local))?;
    server.peers.insert(1, peer1);
    first.accept_answer(&answer)?;
    let mut first_events = Vec::new();
    run_until(
        "signaled",
        &mut server,
        &mut [(&mut first, &mut first_events)],
        |s, c| s.opened.contains(&peer1) && opened(c[0].1),
    )?;

    // Generation 2 connects from the kept answer while generation 1 is
    // still alive, as a reloaded page does.
    let mut second = Client::new("127.0.0.1:0".parse()?, phone_cert.clone(), identity(), 2)?;
    second.connect_cached(&answer)?;
    let mut second_events = Vec::new();
    run_until(
        "cold",
        &mut server,
        &mut [
            (&mut first, &mut first_events),
            (&mut second, &mut second_events),
        ],
        |s, c| s.opened.len() == 2 && opened(c[1].1),
    )?;
    let peer2 = *server
        .peers
        .get(&2)
        .ok_or("no generation 2 on the server")?;
    assert_ne!(peer1, peer2);
    assert!(!first.is_closed(), "generation 1 died when 2 connected");

    // The server retires generation 1.
    server.pipe.close(peer1);
    assert!(second.send(b"after retire")?);
    run_until(
        "retire",
        &mut server,
        &mut [(&mut second, &mut second_events)],
        |s, _| {
            s.closed.contains(&peer1)
                && s.data
                    .iter()
                    .any(|(p, d)| *p == peer2 && d == b"after retire")
        },
    )?;

    // An ICE restart keeps the connection: no new channel, same peer.
    second.restart_ice(&answer)?;
    assert_eq!(second.restart(), 1);
    // The restart completes: the server takes the new credentials and the
    // phone's ICE connects again on them.
    let restarted_at = second_events.len();
    let restarted = format!("{DEVICE}+s2+1");
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        server.step()?;
        step_client(&mut second, &mut second_events)?;
        let switched = server.pipe.local_credentials(peer2)?.ufrag == restarted;
        let connected = second_events[restarted_at..]
            .iter()
            .any(|e| matches!(e, ClientEvent::IceState(s) if s == "Connected" || s == "Completed"));
        if switched && connected {
            break;
        }
        if Instant::now() > deadline {
            return Err(format!(
                "restart not done: server switched {switched}, phone events {:?}",
                &second_events[restarted_at..]
            )
            .into());
        }
    }
    // Data both ways over the restarted connection.
    let up = |s: &Server| {
        s.data
            .iter()
            .any(|(p, d)| *p == peer2 && d == b"up after restart")
    };
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut got_down = false;
    while !(got_down && up(&server)) {
        if Instant::now() > deadline {
            return Err(format!("after the restart: down {got_down}, up {}", up(&server)).into());
        }
        second.send(b"up after restart")?;
        server.pipe.send(peer2, b"down after restart")?;
        server.step()?;
        let mut evs = Vec::new();
        step_client(&mut second, &mut evs)?;
        got_down |= evs.contains(&ClientEvent::Data(b"down after restart".to_vec()));
    }
    assert_eq!(
        server.opened,
        vec![peer1, peer2],
        "the restart opened a new channel"
    );

    // Messages written together still leave one per datagram: SCTP would
    // bundle them (see Pipe::send).
    run_for_quiet(&mut server, &mut second)?;
    let before = second.datagrams();
    for i in 0..5u8 {
        assert!(server.pipe.send(peer2, &[i; 40])?);
    }
    let mut got = 0;
    let deadline = Instant::now() + Duration::from_secs(5);
    while got < 5 && Instant::now() < deadline {
        server.step()?;
        let mut evs = Vec::new();
        step_client(&mut second, &mut evs)?;
        got += evs
            .iter()
            .filter(|e| matches!(e, ClientEvent::Data(d) if d.len() == 40))
            .count();
    }
    assert_eq!(got, 5);
    let datagrams = second.datagrams() - before;
    assert!(datagrams >= 5, "5 messages came in {datagrams} datagrams");
    Ok(())
}

/// Runs both a moment so nothing is in flight.
fn run_for_quiet(server: &mut Server, client: &mut Client) -> TestResult {
    let end = Instant::now() + Duration::from_millis(300);
    while Instant::now() < end {
        server.step()?;
        step_client(client, &mut Vec::new())?;
    }
    Ok(())
}
