//! Prints a seed's network story as JSON, for the lab (`lab/README.md`).
//!
//! ```sh
//! cargo run -p sync-core --example story -- 17
//! ```

use std::process::ExitCode;

use sync_core::sim::Story;

fn main() -> ExitCode {
    let Some(seed) = std::env::args().nth(1).and_then(|s| s.parse().ok()) else {
        eprintln!("usage: story <seed>");
        return ExitCode::FAILURE;
    };
    println!("{}", Story::generate(seed).to_json());
    ExitCode::SUCCESS
}
