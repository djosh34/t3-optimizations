//! Regression for #91: a due send timer must not prevent receiving ready UDP.
use pipe::{Config, Pipe, PipeEvent};
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use str0m::ice::{StunMessage, TransId};

#[test]
fn ready_udp_is_received_even_when_the_send_deadline_has_passed()
-> Result<(), Box<dyn std::error::Error>> {
    let mut pipe = Pipe::bind(Config {
        bind: "127.0.0.1:0".parse()?,
        advertise: None,
        cert: None,
    })?;
    let provider = str0m::crypto::from_feature_flags();
    let hmac =
        |key: &[u8], payloads: &[&[u8]]| provider.sha1_hmac_provider.sha1_hmac(key, payloads);
    let request = StunMessage::binding_request("local:remote", TransId::new(), true, 1, 100, true);
    let mut buf = vec![0u8; 1200];
    let n = request.to_bytes(Some(b"password"), &mut buf, hmac)?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(&buf[..n], pipe.host())?;
    let mut events = Vec::new();
    for _ in 0..100 {
        pipe.poll(Instant::now() - Duration::from_millis(1), &mut events)?;
        if events
            .iter()
            .any(|e| matches!(e, PipeEvent::Unknown { .. }))
        {
            return Ok(());
        }
    }
    // A future deadline is the control: the same queued datagram is valid.
    pipe.poll(Instant::now() + Duration::from_millis(10), &mut events)?;
    assert!(
        events
            .iter()
            .any(|e| matches!(e, PipeEvent::Unknown { .. })),
        "control packet was invalid"
    );
    Err("100 expired-deadline polls ignored a ready datagram".into())
}
