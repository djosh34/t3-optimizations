//! Runs the simulator on many seeds' network stories (`Scenario::story`) on
//! every core, and checks that each holds: nothing stored twice, no acked
//! byte sent again, no command applied twice, no packet too large, and the
//! prompt confirmed and the reply whole before the run ends.
//!
//! ```sh
//! cargo run --release -p sync-core --example stories -- run 1 1000 out.csv
//! cargo run --release -p sync-core --example stories -- summary a.csv b.csv
//! ```
//!
//! `run` writes one CSV line per seed and exits 1 if a seed broke; the
//! broken seeds are printed. `summary` reads such files and prints the
//! spread of prompt time, reply time and text lag as Markdown.

use std::cell::Cell;
use std::fs;
use std::panic;
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

use sync_core::MAX_PACKET;
use sync_core::sim::{Scenario, Story, run};

const HEADER: &str = "seed,survival,prompt_ms,reply_ms,lag_max_ms,lag_p95_ms,broken";

/// One seed's result as a CSV line; `broken` is empty when the seed held.
fn simulate(seed: u64) -> String {
    let survival = Story::generate(seed).survival;
    let o = match run(&Scenario::story(seed)) {
        Ok(o) => o,
        Err(e) => return format!("{seed},{survival},,,,,error: {}", e.replace(',', ";")),
    };
    let mut broken = Vec::new();
    if o.duplicate_bytes > 0 {
        broken.push(format!("{} bytes stored twice", o.duplicate_bytes));
    }
    if o.acked_resent_bytes > 0 {
        broken.push(format!("{} acked bytes sent again", o.acked_resent_bytes));
    }
    if o.commands_twice > 0 {
        broken.push(format!("{} commands applied twice", o.commands_twice));
    }
    if o.largest_packet > MAX_PACKET {
        broken.push(format!("a packet of {} bytes", o.largest_packet));
    }
    let prompt = o.prompt_confirmed_ms.first().copied().flatten();
    if prompt.is_none() {
        broken.push("prompt never confirmed".into());
    }
    if o.reply_done_ms.is_none() {
        broken.push("reply never whole".into());
    }
    let opt = |v: Option<u64>| v.map_or(String::new(), |v| v.to_string());
    format!(
        "{seed},{survival},{},{},{},{},{}",
        opt(prompt),
        opt(o.reply_done_ms),
        o.lag_max_ms,
        o.lag_p95_ms,
        broken.join("; ")
    )
}

thread_local! {
    /// The seed this thread simulates, for the panic message.
    static SEED: Cell<u64> = const { Cell::new(0) };
}

fn run_range(from: u64, to: u64, out: &str) -> ExitCode {
    // A panic is a broken seed too: it ends the run, and says which seed.
    let report = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        eprintln!("broken: seed {} panicked", SEED.get());
        report(info);
    }));
    let next = AtomicU64::new(from);
    let threads = thread::available_parallelism().map_or(1, |n| n.get());
    let results: Vec<thread::Result<Vec<(u64, String)>>> = thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut lines = Vec::new();
                    loop {
                        let seed = next.fetch_add(1, Ordering::Relaxed);
                        if seed > to {
                            return lines;
                        }
                        SEED.set(seed);
                        lines.push((seed, simulate(seed)));
                    }
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join()).collect()
    });
    let mut lines = Vec::new();
    for r in results {
        match r {
            Ok(l) => lines.extend(l),
            Err(_) => return ExitCode::FAILURE,
        }
    }
    lines.sort();
    let broken: Vec<&String> = lines
        .iter()
        .map(|(_, l)| l)
        .filter(|l| !l.ends_with(','))
        .collect();
    let mut csv = String::from(HEADER);
    for (_, l) in &lines {
        csv.push('\n');
        csv.push_str(l);
    }
    csv.push('\n');
    if let Err(e) = fs::write(out, csv) {
        eprintln!("{out}: {e}");
        return ExitCode::FAILURE;
    }
    println!(
        "seeds {from} to {to}: {} run, {} broken",
        lines.len(),
        broken.len()
    );
    for l in &broken {
        println!("broken: {l}");
    }
    if broken.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

struct Row {
    seed: u64,
    survival: bool,
    prompt: Option<f64>,
    reply: Option<f64>,
    lag_max: Option<f64>,
    broken: String,
}

fn parse(line: &str) -> Option<Row> {
    let f: Vec<&str> = line.splitn(7, ',').collect();
    let secs = |s: &str| s.parse::<f64>().ok().map(|ms| ms / 1000.0);
    Some(Row {
        seed: f.first()?.parse().ok()?,
        survival: *f.get(1)? == "true",
        prompt: secs(f.get(2)?),
        reply: secs(f.get(3)?),
        lag_max: secs(f.get(4)?),
        broken: f.get(6)?.to_string(),
    })
}

/// "median / p95 / p99 / max" of the values, in seconds.
fn spread(mut v: Vec<f64>) -> String {
    if v.is_empty() {
        return "none".into();
    }
    v.sort_by(f64::total_cmp);
    let q = |f: f64| v[((v.len() - 1) as f64 * f) as usize];
    format!(
        "{:.1} / {:.1} / {:.1} / {:.1}",
        q(0.5),
        q(0.95),
        q(0.99),
        q(1.0)
    )
}

fn summary(files: &[String]) -> ExitCode {
    let mut rows = Vec::new();
    for file in files {
        let text = match fs::read_to_string(file) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{file}: {e}");
                return ExitCode::FAILURE;
            }
        };
        for line in text.lines().skip(1) {
            match parse(line) {
                Some(r) => rows.push(r),
                None => {
                    eprintln!("{file}: not a result line: {line}");
                    return ExitCode::FAILURE;
                }
            }
        }
    }
    rows.sort_by_key(|r| r.seed);
    let broken: Vec<&Row> = rows.iter().filter(|r| !r.broken.is_empty()).collect();
    println!(
        "{} seeds, {} survival stories, **{} broken**.\n",
        rows.len(),
        rows.iter().filter(|r| r.survival).count(),
        broken.len()
    );
    println!("| Stories | Prompt confirmed (s) | Reply whole (s) | Max text lag (s) |");
    println!("|---|---|---|---|");
    println!(
        "| | median / p95 / p99 / max | median / p95 / p99 / max | median / p95 / p99 / max |"
    );
    for (name, survival) in [("2G", false), ("survival", true)] {
        let pick = |f: fn(&Row) -> Option<f64>| -> Vec<f64> {
            rows.iter()
                .filter(|r| r.survival == survival)
                .filter_map(f)
                .collect()
        };
        println!(
            "| {name} | {} | {} | {} |",
            spread(pick(|r| r.prompt)),
            spread(pick(|r| r.reply)),
            spread(pick(|r| r.lag_max)),
        );
    }
    if !broken.is_empty() {
        println!("\nBroken seeds:\n");
        for r in broken {
            println!("- {}: {}", r.seed, r.broken);
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let num = |i: usize| args.get(i).and_then(|s| s.parse::<u64>().ok());
    match (
        args.first().map(String::as_str),
        num(1),
        num(2),
        args.get(3),
    ) {
        (Some("run"), Some(from), Some(to), Some(out)) => run_range(from, to, out),
        (Some("summary"), ..) if args.len() > 1 => summary(&args[1..]),
        _ => {
            eprintln!("usage: stories run <from> <to> <out.csv> | stories summary <csv>...");
            ExitCode::FAILURE
        }
    }
}
