//! A STUN request no connection takes is checked against the derived
//! password before anything acts on it: its username fragment is public.

use std::net::UdpSocket;
use std::time::{Duration, Instant};

use pipe::creds::{self, Side};
use pipe::{Config, Pipe, PipeEvent};
use str0m::ice::{StunMessage, TransId};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Sends a binding request for `local:remote`, signed with `password`, and
/// returns the pipe's events.
fn send_request(
    pipe: &mut Pipe,
    username: &str,
    password: &str,
) -> Result<Vec<PipeEvent>, Box<dyn std::error::Error>> {
    let provider = str0m::crypto::from_feature_flags();
    let hmac =
        |key: &[u8], payloads: &[&[u8]]| provider.sha1_hmac_provider.sha1_hmac(key, payloads);
    let request = StunMessage::binding_request(username, TransId::new(), true, 1, 100, true);
    let mut buf = vec![0u8; 1200];
    let n = request.to_bytes(Some(password.as_bytes()), &mut buf, hmac)?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(&buf[..n], pipe.host())?;
    let mut events = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(2);
    while events.is_empty() && Instant::now() < deadline {
        pipe.poll(Instant::now() + Duration::from_millis(20), &mut events)?;
    }
    Ok(events)
}

#[test]
fn only_a_request_signed_with_the_derived_password_is_authentic() -> TestResult {
    let mut pipe = Pipe::bind(Config {
        bind: "127.0.0.1:0".parse()?,
        advertise: None,
        cert: None,
    })?;
    let token = [9u8; 16];
    let server = creds::derive(&token, "dev1test", Side::Server, 7, 0);
    let phone = creds::derive(&token, "dev1test", Side::Phone, 7, 0);
    let username = format!("{}:{}", server.ufrag, phone.ufrag);

    // Someone who knows the public device id but not the token.
    let events = send_request(&mut pipe, &username, "not the derived password")?;
    assert!(
        matches!(&events[..], [PipeEvent::Unknown { local_ufrag, .. }] if *local_ufrag == server.ufrag)
    );
    assert!(!pipe.unknown_authentic(&server.ufrag, &server.pass));

    // The paired phone.
    send_request(&mut pipe, &username, &server.pass)?;
    assert!(pipe.unknown_authentic(&server.ufrag, &server.pass));
    Ok(())
}
