# Vendored crates

## sctp-proto 0.10.8

The SCTP layer of str0m 0.25.0, which `crates/pipe` uses for the data channel to the phone. `crates/Cargo.toml` puts this copy in place of the crates.io one with `[patch.crates-io]`.

The first commit that added it is the crate exactly as published on crates.io (without the registry's `.cargo-ok` and the upstream `.github` folder). The patch is the next commit, so `git log -p -- crates/vendor/sctp-proto` shows every change. Each change is marked `t3-optimizations patch` in the code:

- `src/config.rs`: the retransmit timeout is capped at 2 s (`RTO_MAX`, was 60 s) and starts at 2 s (`RTO_INITIAL`, was 3 s). A lossy link is probed again at least every 2 s instead of after a backoff of up to a minute. str0m builds its SCTP config inside the crate without `with_rto_max_ms`, so the default is the only place to set it.
- `src/association/timer.rs`: the timer table's default uses `RTO_MAX` instead of its own `60000`.
- `src/association/mod.rs`: a retransmit timeout and a fast retransmit no longer cut the congestion window and `ssthresh`. Loss is not a congestion signal on the links this runs on; the sync protocol paces what it sends (issue #25). This changes only what this server sends. The phone-to-server direction is the browser's own SCTP.
- `src/endpoint/endpoint_test.rs`: `test_assoc_congestion_control_slow_reader` is ignored. It waits for a retransmit timeout to cut the congestion window to 1 MTU, which the patch removes, so it never ends.

Only `crates/pipe` (and through str0m the native test phone) uses this crate. Its tests run in `make check` and in CI's `rust` job (`cargo test --manifest-path vendor/sctp-proto/Cargo.toml`); the effect of the patch on a lossy link is measured in the lab (`lab/reports/pipe`, issue #49).

To move to a newer sctp-proto: copy the new release here unchanged in one commit, then apply the changes above again in the next.

## is 0.12.0

str0m's ICE agent, which `crates/Cargo.toml` also puts in place with `[patch.crates-io]`. The first commit is the crate as published; the next one is the patch, marked `t3-optimizations patch` in the code (issue #44):

- `src/agent.rs`: an ICE-lite agent never prunes the pair it sends on. Upstream prunes a pair after 15 s without a binding request from the remote. At 70% loss a phone's keepalive checks miss that window often, and the pair made again from its next check is not nominated (keepalives carry no USE-CANDIDATE), so the server had nothing to send on until the phone restarted ICE. In the lab this stalled case 70 for minutes.
- `src/agent.rs`: of nominated pairs with the same priority, an ICE-lite agent sends on the one the remote checked last, and compares pairs rather than priorities to decide whether to move. A phone that comes back from a new address and nominates it still moves the path, although the old pair is never pruned.
- `src/lib.rs`: `ice_lite_disconnect_reconnect` is ignored. It waits for the ICE-lite side to disconnect when the remote goes silent, which the patch removes.
- `src/agent.rs`: the test `ice_lite_keeps_its_pair_and_follows_a_new_address` shows both changes.

Its tests run in `make check` and CI's `rust` job (`cargo test --manifest-path vendor/is/Cargo.toml`).
