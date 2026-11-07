# Vendored crates

## sctp-proto 0.10.8

The SCTP layer of str0m 0.25.0, which `crates/pipe` uses for the data channel to the phone. `crates/Cargo.toml` puts this copy in place of the crates.io one with `[patch.crates-io]`.

The first commit that added it is the crate exactly as published on crates.io (without the registry's `.cargo-ok` and the upstream `.github` folder). The patch is the next commit, so `git log -p -- crates/vendor/sctp-proto` shows every change. Each change is marked `t3-optimizations patch` in the code:

- `src/config.rs`: the retransmit timeout is capped at 2 s (`RTO_MAX`, was 60 s) and starts at 2 s (`RTO_INITIAL`, was 3 s). A lossy link is probed again at least every 2 s instead of after a backoff of up to a minute. str0m builds its SCTP config inside the crate without `with_rto_max_ms`, so the default is the only place to set it.
- `src/association/timer.rs`: the timer table's default uses `RTO_MAX` instead of its own `60000`.
- `src/association/mod.rs`: a retransmit timeout and a fast retransmit no longer cut the congestion window and `ssthresh`. Loss is not a congestion signal on the links this runs on; the sync protocol paces what it sends (issue #25). This changes only what this server sends. The phone-to-server direction is the browser's own SCTP.

Only `crates/pipe` uses this crate. Its own tests are left as published and are not part of `make check` or CI; the effect of the patch is measured in the lab (`lab/reports/pipe`, issue #49).

To move to a newer sctp-proto: copy the new release here unchanged in one commit, then apply the changes above again in the next.
