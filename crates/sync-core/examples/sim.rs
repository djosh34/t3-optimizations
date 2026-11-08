//! Runs the simulator's case 70 and case 50 for a range of seeds and prints
//! one line per run.
//!
//! ```sh
//! cargo run --release -p sync-core --example sim -- case70 1 10      # seeds 1 to 10
//! cargo run --release -p sync-core --example sim -- case50 1 3 120   # 2 minutes each
//! cargo run --release -p sync-core --example sim -- case70-webkit 1 30 180
//! ```
//!
//! A `-webkit` case sends the phone's packets through the model of WebKit's
//! SCTP sender (`src/sim/browser.rs`).

use std::process::ExitCode;

use sync_core::sim::{Scenario, run};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let case = args.first().map_or("case70", String::as_str);
    let from: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let to: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(from);
    // Seconds after the profile applies; the default is the lab's 15 min.
    let secs: Option<u64> = args.get(3).and_then(|s| s.parse().ok());
    println!(
        "case seed | prompt confirmed | lag max/p95/median | reply done | synced | dup | acked resent | twice | largest | match | down sent/delivered B/s | loss | srtt"
    );
    for seed in from..=to {
        let (base, webkit) = match case.strip_suffix("-webkit") {
            Some(base) => (base, true),
            None => (case, false),
        };
        let s = match base {
            "case70" => Scenario::case70(seed),
            "case50" => Scenario::case50(seed),
            other => {
                eprintln!("unknown case {other}; use case70 or case50, with -webkit or without");
                return ExitCode::FAILURE;
            }
        };
        let s = if webkit { s.in_webkit() } else { s };
        let s = match secs {
            Some(n) => s.shortened(n * 1000),
            None => s,
        };
        let o = match run(&s) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{case} seed {seed}: {e}");
                return ExitCode::FAILURE;
            }
        };
        let secs = |v: Option<u64>| {
            v.map_or("never".to_string(), |ms| {
                format!("{:.1} s", ms as f64 / 1000.0)
            })
        };
        let prompts: Vec<String> = o.prompt_confirmed_ms.iter().map(|&p| secs(p)).collect();
        println!(
            "{case} {seed} | {} | {:.1}/{:.1}/{:.1} s | {} | {} | {} | {} | {} | {} | {} | {:.0}/{:.0} | {:.2} | {:.0} ms",
            prompts.join(", "),
            o.lag_max_ms as f64 / 1000.0,
            o.lag_p95_ms as f64 / 1000.0,
            o.lag_median_ms as f64 / 1000.0,
            secs(o.reply_done_ms),
            secs(o.synced_ms),
            o.duplicate_bytes,
            o.acked_resent_bytes,
            o.commands_twice,
            o.largest_packet,
            o.phone_matches,
            o.server.send_rate,
            o.server.delivered_rate,
            o.server.loss,
            o.server.srtt_ms.unwrap_or(0.0),
        );
    }
    ExitCode::SUCCESS
}
