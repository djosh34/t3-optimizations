#!/usr/bin/env bash
# Builds the untouched t3code v0.1.7 web client for the lab's baseline runs and
# prints the directory of the build. Later outcomes change t3code/, so the
# build comes from t3code/ as this repo first vendored it, never from the
# working tree. Run as the user, not as root.
set -euo pipefail

# The commit that vendored v0.1.7. Its apps/web and packages/ are the tag's.
commit=22099f5fdb147f0efee0704fb21b4e19946d736f

repo="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cache="${XDG_CACHE_HOME:-$HOME/.cache}/t3lab/baseline-client-${commit:0:12}"
if [[ -f "$cache/dist/index.html" ]]; then
  echo "$cache/dist"
  exit 0
fi

rm -rf "$cache"
mkdir -p "$cache/src"
git -C "$repo" archive "$commit" t3code | tar -x -C "$cache/src"
cd "$cache/src/t3code"
vp install --frozen-lockfile >&2
PATH="$PWD/node_modules/.bin:$PATH" vp run --filter @t3tools/web build >&2
mv apps/web/dist "$cache/dist"
cd "$cache"
rm -rf "$cache/src"
echo "$cache/dist"
