# t3-optimizations

A loss-resilient path from a phone to a t3code server, for very bad mobile links: a sync protocol over a WebRTC data channel, a Rust sidecar, and a home-screen web app, tested in a Go chaos lab.

The plan lives in the [map issue](https://github.com/djosh34/t3-optimizations/issues/1).

## Layout

### `t3code/`

A copy of [t3code](https://github.com/pingdotgg/t3code) at tag `v0.1.7`, kept as plain files: no submodule and no GitHub fork. `t3code/UPSTREAM` names the commit. The `.repos/` reference folder of the tag was left out. Our changes to t3code are made here and show in this repo's history. Nothing goes upstream.

Every `t3` command started from this copy (serve, pair, dev, tests) passes `--base-dir` with a directory of its own. Without it, t3 uses `~/.t3`, the data of the official t3code on this host.

### `crates/`

A Cargo workspace (Rust stable, see `crates/rust-toolchain.toml`). `sync-core` holds the sync protocol and builds both natively and for `wasm32-unknown-unknown`, so the sidecar and the web app share it. [crates/sync-core/README.md](crates/sync-core/README.md) describes the protocol and its simulator.

`pipe` is the server side of the WebRTC data channel to the phone, on str0m: ICE-lite on one UDP port, DTLS server role without the cookie exchange, ICE credentials derived from the phone's token so new connections and ICE restarts need no signaling. Its SCTP layer is a patched copy of sctp-proto in `vendor/` (2 s retransmit cap, no congestion-window cut on loss), and its ICE agent a patched copy of `is` (the server keeps its path when the phone's checks are lost), both described in [crates/vendor/README.md](crates/vendor/README.md). Its `pipe-lab` binary is the server end of the lab's pipe runs.

`sidecar` connects the forked t3 server to phones over the sync protocol on that data channel, with a WebSocket fallback; see [crates/sidecar/README.md](crates/sidecar/README.md). `native-phone` is a phone without a browser, the sync protocol over the real data channel, for the sidecar's tests and the lab's `--client sidecar-native`.

### `lab/`

A Go module (Go 1.27) for the chaos lab, which runs t3 clients over shaped and faulty links and measures what the phone sees. See [lab/README.md](lab/README.md); the WebSocket baseline is in [lab/reports/baseline](lab/reports/baseline/README.md).

## Checks

Install once:

```sh
npm i -g vite-plus@1.4.0   # the version in t3code/pnpm-workspace.yaml
make setup                 # vp install in t3code/, then unsets core.hooksPath
```

`vp install` registers t3code's git hooks for the whole repo (`core.hooksPath`), and every worktree shares that setting. Always unset it after `vp install`, as `make setup` does:

```sh
scripts/unset-hooks-path.sh
```

Before every push, run:

```sh
make check
```

It runs the Rust checks (fmt, clippy, tests, WASM build), the Go checks (gofmt, vet, tests), `vp check` in `t3code/`, and the tests of each t3code package changed against `origin/main` (`make check BASE=<ref>` compares against another ref). The full t3code typecheck and test suites run only in CI: the `apps/server` typecheck alone needs more than 4 GB, more than the dev VM has free.

CI (`.github/workflows/ci.yml`) runs on every PR and on `main`, with four jobs that `main` requires:

- `rust`: the Rust checks.
- `go`: the Go checks.
- `t3code`: `vp install`, `vp check`, `vp run -r typecheck`, and the web and server unit tests.
- `lab-smoke`: three short chaos lab runs of the baseline client, one of the pipe, one of the sidecar (`--client sidecar-native`), and the sidecar's integration and WebKit tests.
