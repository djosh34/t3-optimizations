# Scroll regression for #116

These tests use stock t3 v0.1.7 and the published v0.2.0 sidecar and web bundle. They do not change production code. The findings are on issue #116.

## Replay the recorded failure

From the repository root:

```sh
node scripts/research116/assert-scroll.mjs scripts/research116/evidence/safari-native.json.gz
node scripts/research116/assert-scroll.mjs scripts/research116/evidence/safari-sidecar.json.gz
node scripts/research116/assert-scroll.mjs scripts/research116/evidence/home-native.json.gz
node scripts/research116/assert-scroll.mjs scripts/research116/evidence/home-sidecar.json.gz
bun scripts/research116/regression.ts
```

The native checks pass. Both sidecar checks and the source regression fail on v0.2.0. The source regression imports and bundles the actual `snapshots.ts`, not a reimplementation. It needs Bun, but no dependency installation.

The scroll check measures the same visible message after the driver stops sending input. It skips the first 300 ms, then requires at least one second of observations. The message must remain visible and its screen position must stay within four CSS pixels. Changing `scrollTop` alone is not a failure because prepending history should change it.

The diagnostic recordings use the same test. `home-no-standins-sidecar.json.gz` still fails. `home-atomic-history-sidecar.json.gz` passes. The latter freezes the first subscription snapshot until all 600 historical turn items and their corresponding messages are present with their fixture text prefixes. It batches publication, but does not check complete byte ranges. These are browser-boundary experiments, not proposed production fixes.

## Make a new recording

Use a fresh directory under `/tmp` and no other t3 instance. Ports are fixed to HTTP 13804 for stock t3, HTTP 12504 and UDP 12514 for the sidecar. Check that they are free first. Do not point this driver at live data.

1. Run `~/.local/share/t3opt/t3-0.1.7/t3 serve --base-dir "$base/stock" --host 127.0.0.1 --port 13804 --no-browser`. Add a project with `t3 project add --base-dir "$base/stock" --title research116 "$base"`.
2. Stop only that throwaway t3 process. Run `python3 scripts/research116/seed.py "$base/stock"`. The script uses Python to generate SQL and Bun's SQLite to execute it because this schema needs recent FTS5 support.
3. Copy `fake-pi.py` to `$base/fake-pi` and make it executable. In the throwaway `userdata/settings.json`, set `providerInstances.pi` to `{"driver":"pi","enabled":true,"config":{"binaryPath":"<absolute path to fake-pi>"}}` and set `idleCompaction` to `"off"`. Restart only the throwaway t3. The fixture speaks Pi's JSON protocol and streams text for 30 seconds. It calls no provider, tool or subagent.
4. Start the release sidecar by hand, in `systemd-run --user --scope --unit research116-sidecar -p MemoryMax=768M -p MemorySwapMax=0`, with `--t3-url http://127.0.0.1:13804 --t3-base-dir "$base/stock" --t3-command "$HOME/.local/share/t3opt/t3-0.1.7/t3" --data-dir "$base/sidecar" --http 127.0.0.1:12504 --udp 127.0.0.1:12514 --web-dir "$HOME/.local/share/t3opt/web" --preload 0`. The sidecar data directory must be fresh.
5. Mint two separate one-use tokens with `t3 pair --base-dir "$base/stock" --label research116-native` and `--label research116-sidecar`. Redirect their output to `$base/code-native` and `$base/code-sidecar`. Never commit them. Mint new tokens for each run.
6. Run the driver:

```sh
docker run --rm --network host --init --shm-size 512m \
  --user "$(id -u):$(id -g)" -e HOME=/tmp \
  -v "$PWD/scripts/research116:/check:ro" -v "$base:/data" \
  -v "$HOME/.local/share/t3opt/t3-0.1.7:/stock-t3:ro" \
  t3lab-phone:1.64.1 node /check/scroll.mjs safari
```

Repeat with `home`, for 402x874 at 3x. Both contexts otherwise use Playwright's iPhone 17 Pro device, including its user agent. The image is built from `lab/driver/Dockerfile`, with Playwright 1.64.1. Optional third arguments `no-standins` and `atomic-history` select the diagnostic interventions.

Mobile WebKit rejects Playwright's wheel API and has no swipe API. The driver dispatches a wheel navigation event so the application's follow state releases, then applies bounded `scrollTop` movements to the real viewport. Fast flings are repeated 1,800-pixel movements, not physical iOS inertia. The native GUI and sidecar receive the same movement schedule. Native earlier history is loaded through its normal button before the cached cases.

The driver records viewport geometry at animation-frame cadence and every shim thread subscription answer before delivery. It covers cold history, no-input holds, cached scrolling, fast movements and scrolling during a real stock-server reply from the fake provider. It runs the failing scroll assertion after saving the recording. `uptime` is logged at each phase.

Stop only your hand-run sidecar scope and throwaway t3 when finished. Do not change services or routes.

## Evidence format and safety

The `.json.gz` files contain all measured frames and shim thread answers for the four completed case sequences. They exclude configuration, auth, pairing and background RPC answers. `answers` is empty for native, whose socket is not the JavaScript shim.

The two videos put native on the left and the sidecar on the right. They start 18 seconds into the original recordings, after pairing, and retain all scroll cases. They are re-encoded at 20 fps. The Safari recording's post-case selector log timed out after the screenshots and JSON were saved; its browser closed in `finally`, and the video includes all cases. This was not a passing end-to-end run. The committed driver makes that optional log tolerate a missing selector.

Original browser videos include the one-use token entry. Do not publish them. Only trimmed throwaway-thread videos are included here. No recording uses the main t3code.
