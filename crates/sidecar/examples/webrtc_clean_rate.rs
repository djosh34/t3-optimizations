//! Research #69: sync-core over the real sidecar Pipe/str0m, no t3 server.
//! One JSON-encoded SDP offer on stdin; prints ANSWER <json SDP> on stdout.
use pipe::{Pipe, PipeEvent};
use std::{
    io::{self, BufRead},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use sync_core::{Class, Config, Server};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<_> = std::env::args().collect();
    let mb: usize = args.get(1).and_then(|x| x.parse().ok()).unwrap_or(20);
    let size: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(20_000);
    let mut pipe = Pipe::bind(pipe::Config {
        bind: "127.0.0.1:0".parse()?,
        advertise: None,
        cert: None,
    })?;
    let start = Instant::now();
    let mut server = Server::new(*b"sim-token-000001", Config::server(), 0);
    for i in 0..(mb * 1_000_000 / size) {
        let text = format!(
            "Object {i}: {}",
            "sample history data, repeated words and text. ".repeat(size / 44 + 1)
        );
        server.set(
            i as u64,
            Class::Background,
            i as u64,
            vec![b'm'; 200],
            text.as_bytes()[..size].to_vec(),
        )?;
    }
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for line in io::stdin().lock().lines().map_while(Result::ok) {
            let _ = tx.send(line);
        }
    });
    let mut peer = None;
    let mut attached = false;
    let mut opened = None;
    let mut next_log = 0;
    let mut dropped = 0;
    let mut events = Vec::new();
    loop {
        if let Ok(line) = rx.try_recv() {
            let offer: String = serde_json::from_str(&line)?;
            let (id, answer) = pipe.accept_offer(&offer, None)?;
            peer = Some(id);
            println!("ANSWER {}", serde_json::to_string(&answer)?);
        }
        let now = start.elapsed().as_millis() as u64;
        let wake = if attached {
            server.next_wakeup(now).min(now + 5)
        } else {
            now + 5
        };
        pipe.poll(start + Duration::from_millis(wake), &mut events)?;
        for event in events.drain(..) {
            match event {
                PipeEvent::Open(_) => {
                    opened = Some(start.elapsed());
                    println!("OPEN");
                }
                PipeEvent::Data(_, bytes) => {
                    let _ = server.receive(start.elapsed().as_millis() as u64, &bytes)?;
                    attached = true;
                }
                PipeEvent::Failed(_, e) => println!("FAILED {e}"),
                _ => {}
            }
        }
        let now = start.elapsed().as_millis() as u64;
        if attached {
            while let Some(bytes) = server.poll(now) {
                if !pipe.send(peer.unwrap(), &bytes)? {
                    dropped += 1;
                }
            }
        }
        if attached && now >= next_log {
            next_log = now + 2000;
            let s = server.stats();
            println!(
                "STATS t={} rate={:.0} delivered={:.0} srtt={:.0} unsynced={} inflight={} buffered={} dropped={dropped}",
                (start.elapsed() - opened.unwrap()).as_millis(),
                s.send_rate,
                s.delivered_rate,
                s.srtt_ms.unwrap_or(0.),
                s.unsynced_bytes,
                s.inflight_packets,
                pipe.buffered(peer.unwrap()).unwrap_or(0)
            );
        }
        if attached && server.is_synced() {
            println!("DONE {}", (start.elapsed() - opened.unwrap()).as_millis());
            break;
        }
        if opened.is_some_and(|t| start.elapsed() - t > Duration::from_secs(180)) {
            println!("TIMEOUT");
            break;
        }
    }
    Ok(())
}
