//! Research #69: a large text backlog on a clean 10 Mbit/s link, 20 ms
//! each way. MB counts raw content; chunks are compressed on the wire.
//! Run: cargo run --release -p sync-core --example clean_link_rate -- 200 600 webkit
use sync_core::sim::{LinkSpec, Scenario, run};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mb: usize = args.get(1).and_then(|x| x.parse().ok()).unwrap_or(20);
    let secs: u64 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(600);
    let mut link = LinkSpec::clean();
    link.delay_ms = 20;
    let s = Scenario {
        warmup_ms: 0,
        down: link.clone(),
        up: link,
        backfill_bytes: mb * 1_000_000,
        backfill_lead_ms: 0,
        prompts_at: vec![0],
        reply_len: 1,
        duration_ms: secs * 1000,
        stop_when_done: false,
        webkit: args.iter().any(|a| a == "webkit"),
        ..Scenario::case70(1)
    };
    let o = run(&s).unwrap();
    println!(
        "backlog={mb}MB synced={:?} match={} dup={} resent={} prompt={:?}",
        o.synced_ms,
        o.phone_matches,
        o.duplicate_bytes,
        o.acked_resent_bytes,
        o.prompt_confirmed_ms
    );
    for (t, server, phone) in o.trace {
        if t % 10_000 < 1000 || server.unsynced_bytes == 0 {
            println!(
                "t={t} rate={:.0} delivered={:.0} loss={:.3} srtt={:.0} unsynced={} sent={} wire_bytes={} up_rate={:.0} up_sent={}",
                server.send_rate,
                server.delivered_rate,
                server.loss,
                server.srtt_ms.unwrap_or(0.),
                server.unsynced_bytes,
                server.sent_packets,
                server.sent_wire_bytes,
                phone.send_rate,
                phone.sent_packets
            );
        }
        if server.unsynced_bytes == 0 && t > 0 {
            break;
        }
    }
}
