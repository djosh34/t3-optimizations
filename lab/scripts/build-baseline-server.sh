#!/usr/bin/env bash
# Prepares the untouched t3code v0.1.7 server for the lab's `--server v0.1.7`
# runs and prints the directory of its t3code/ tree. Like
# build-baseline-client.sh, it comes from t3code/ as this repo first vendored
# it, never from the working tree. The server runs from source and serves the
# web client built into apps/web/dist of the same tree. Run as the user, not
# as root.
set -euo pipefail

# The commit that vendored v0.1.7; lab/internal/run names the same one.
commit=22099f5fdb147f0efee0704fb21b4e19946d736f

repo="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cache="${XDG_CACHE_HOME:-$HOME/.cache}/t3lab/baseline-server-${commit:0:12}"
if [[ -f "$cache/ready" ]]; then
  echo "$cache/t3code"
  exit 0
fi

rm -rf "$cache"
mkdir -p "$cache"
git -C "$repo" archive "$commit" t3code | tar -x -C "$cache"
cd "$cache/t3code"
# Not a git checkout, so the install's prepare script cannot register hooks.
vp install --frozen-lockfile >&2
PATH="$PWD/node_modules/.bin:$PATH" vp run --filter @t3tools/web build >&2
touch "$cache/ready"
echo "$cache/t3code"
