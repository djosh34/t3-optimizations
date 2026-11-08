#!/usr/bin/env bash
# Builds the home-screen web app (issue #45) that the sidecar serves with
# --web-dir: the t3code web client with the sync shim (T3OPT_BUILD=1), the
# phone's sync-core as WebAssembly (phone.wasm) and the service worker
# (sw.js). Run as the user, not as root.
#
# Usage: build-webapp.sh [out dir] [label]
# The out dir defaults to lab/bin/webapp; the label, shown in the app's
# details panel, to the commit. `lab webapp-e2e` updates the app from one
# build to another: build lab/bin/webapp-next with any other label.
set -euo pipefail

repo="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
out="$(realpath -m "${1:-$repo/lab/bin/webapp}")"
label="${2:-$(git -C "$repo" rev-parse --short=12 HEAD)}"

cargo build --release --manifest-path "$repo/crates/Cargo.toml" \
  -p phone-wasm --target wasm32-unknown-unknown >&2

cd "$repo/t3code/apps/web"
export PATH="$repo/t3code/node_modules/.bin:$PATH"
# No source maps: the phone downloads every file of the shell.
T3OPT_BUILD=1 T3OPT_BUILD_LABEL="$label" T3CODE_WEB_SOURCEMAP=0 vp build --outDir "$out" --emptyOutDir >&2
cp "$repo/crates/target/wasm32-unknown-unknown/release/phone_wasm.wasm" "$out/phone.wasm"
node scripts/t3opt-sw.mjs "$out" >&2
echo "$out"
