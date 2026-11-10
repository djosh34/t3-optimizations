//! Synthetic probes for issue #110, not a reproduction of the physical phone incident.
//! No server, sockets, real history or credentials are used.
//! Byte counts are deterministic. Wall times are diagnostic, not release gates.
#[allow(dead_code)]
#[path = "../../crates/sidecar/src/world.rs"]
mod world;
use std::{sync::Arc, time::Instant};
use sync_core::wire::{Body, Chunk, Key, Packet};
use sync_core::{Class, Config, Phone, Server};
fn packet(seq: u64, size: usize, at: usize, n: usize) -> Vec<u8> {
    Packet {
        seq,
        hello: None,
        ack: None,
        report: None,
        log_low: None,
        chunks: vec![Chunk {
            key: Key::Object(7),
            rev: 1,
            total_len: size as u64,
            offset: at as u64,
            keep: None,
            meta: Some(b"{\"c\":\"config\"}".to_vec()),
            body: Body::Raw(vec![b'x'; n]),
        }],
    }
    .encode()
}
fn main() {
    let mut w = world::World::default();
    for i in 0..16 {
        let id = format!("synthetic-{i}");
        w.set_thread(&id, &serde_json::json!({"snapshotSequence":1,"hasMoreHistory":false,"projection":{"thread":{"id":id},"turnItems":[{"id":format!("m-{i}"),"messageId":format!("m-{i}"),"type":"assistant_message","ordinal":1,"text":"x".repeat(2_000_000)}]}}));
    }
    let evicted = w.evict_over(4_000_000, |_| true);
    println!(
        "pinned cache_limit=4000000 world_bytes={} evicted_threads={}",
        w.bytes(),
        evicted.len()
    );
    for size in [563_000usize, 2_000_000, 8_000_000] {
        let mut p = Phone::new([1; 16], 1, Config::phone(), 0);
        let mut saved = 0usize;
        let mut viewed = 0usize;
        let mut packets = 0;
        let start = Instant::now();
        for at in (0..size).step_by(1000) {
            let r = p
                .receive(packets, &packet(packets, size, at, 1000.min(size - at)))
                .unwrap();
            saved += std::hint::black_box(phone_wasm::encode_received(&p, &r)).len();
            viewed += std::hint::black_box(phone_wasm::encode_object(&p, 7).unwrap()).len();
            p.committed(r.batch);
            packets += 1;
        }
        println!(
            "copy size={size} packets={packets} saved_bytes={saved} view_bytes={viewed} expansion={:.1} elapsed_ms={:.1}",
            (saved + viewed) as f64 / size as f64,
            start.elapsed().as_secs_f64() * 1000.
        );
    }
    let mut s = Server::new([2; 16], Config::server(), 0);
    let mut p = Phone::new([2; 16], 2, Config::phone(), 0);
    s.set(
        1,
        Class::History,
        0,
        vec![],
        Arc::<[u8]>::from(vec![b'a'; 8000]),
    )
    .unwrap();
    let mut last_down = 0;
    let mut max_gap = 0;
    let mut down = 0;
    for now in 0..430_000u64 {
        if now == 10_000 {
            let mut x = 0x123456789abcdefu64;
            let bytes: Vec<u8> = (0..3_300_000)
                .map(|_| {
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    (x as u8) & 0x7f
                })
                .collect();
            s.set(2, Class::History, 0, vec![], Arc::<[u8]>::from(bytes))
                .unwrap();
        }
        while let Some(bytes) = p.poll(now) {
            if now < 10_000 {
                s.receive(now, &bytes).unwrap();
            }
        }
        while let Some(bytes) = s.poll(now) {
            let r = p.receive(now, &bytes).unwrap();
            p.committed(r.batch);
            if now > 370_000 {
                max_gap = max_gap.max(now - last_down);
                down += 1;
            }
            last_down = now;
        }
    }
    let stats = s.stats();
    println!(
        "halfopen simulated_ms=430000 upstream_cut_ms=10000 server_rate={} server_last_heard={:?} unsynced={} down_packets_last_60s={down} max_down_gap_last_60s_ms={max_gap}",
        stats.send_rate, stats.last_heard, stats.unsynced_bytes
    );
    let content: Arc<[u8]> = Arc::from(vec![b'x'; 24_000_000]);
    for phones in [1, 2, 8] {
        let start = Instant::now();
        let mut servers: Vec<_> = (0..phones)
            .map(|_| Server::new([3; 16], Config::server(), 0))
            .collect();
        for server in &mut servers {
            server
                .set(1, Class::History, 0, vec![], content.clone())
                .unwrap();
        }
        println!(
            "fanout phones={phones} shared_payload_bytes={} set_ms={:.2}",
            content.len(),
            start.elapsed().as_secs_f64() * 1000.
        );
    }
}
