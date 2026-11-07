//! The sidecar: connects a forked t3 server to phones over the sync
//! protocol (`sync-core`) on the WebRTC data channel (`pipe`), with a
//! WebSocket fallback. See `crates/sidecar/README.md`.

mod app;
mod config;
mod device;
mod http;
mod store;
mod t3;
mod world;

use std::process::ExitCode;

/// What the main loop hears from the other threads.
#[derive(Debug)]
pub enum Msg {
    T3(t3::In),
    Fetched(app::Fetched),
    Api(http::Api),
    Ws(http::WsEvent),
}

fn main() -> ExitCode {
    let cfg = match config::Config::from_args(std::env::args().skip(1).collect()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("sidecar: {e}");
            return ExitCode::from(2);
        }
    };
    match app::run(cfg) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("sidecar: {e}");
            ExitCode::FAILURE
        }
    }
}
