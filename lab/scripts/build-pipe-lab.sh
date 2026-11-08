#!/usr/bin/env bash
# Builds the server end of the lab's pipe runs, as yourself:
#   lab/bin/pipe-lab            from crates/, with the patched sctp-proto
#   lab/bin/pipe-lab-unpatched  the same code with sctp-proto as published
# The unpatched build copies the crates it needs to a temp dir and drops the
# sctp-proto patch there; crates/ itself is not changed.
# --patched-only builds only the first, as CI does.
set -euo pipefail

repo="$(cd "$(dirname "$0")/../.." && pwd)"
bin="$repo/lab/bin"
mkdir -p "$bin"

cargo build --manifest-path "$repo/crates/Cargo.toml" --release -p pipe --bin pipe-lab
install -m 755 "$repo/crates/target/release/pipe-lab" "$bin/pipe-lab"
if [ "${1:-}" = --patched-only ]; then
  echo "built $bin/pipe-lab"
  exit 0
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cp -r "$repo/crates/Cargo.toml" "$repo/crates/Cargo.lock" "$repo/crates/rust-toolchain.toml" \
  "$repo/crates/pipe" "$repo/crates/sync-core" "$work/"
mkdir -p "$work/vendor"
cp -r "$repo/crates/vendor/is" "$repo/crates/vendor/dimpl" "$work/vendor/"
sed -i -e 's/^members = .*/members = ["sync-core", "pipe"]/' \
  -e '/^# str0m.s SCTP layer/d' -e '/^sctp-proto = { path/d' "$work/Cargo.toml"
if grep -q '^sctp-proto' "$work/Cargo.toml"; then
  echo "build-pipe-lab.sh: could not drop the sctp-proto patch from Cargo.toml" >&2
  exit 1
fi
# The same version as the vendored copy, not the newest 0.10.x: a direct
# dependency with an exact version pins the one str0m uses.
echo 'sctp-proto = "=0.10.8"' >> "$work/pipe/Cargo.toml"
# A target dir of its own keeps the patched and unpatched builds apart.
CARGO_TARGET_DIR="$repo/crates/target/unpatched" \
  cargo build --manifest-path "$work/Cargo.toml" --release -p pipe --bin pipe-lab
install -m 755 "$repo/crates/target/unpatched/release/pipe-lab" "$bin/pipe-lab-unpatched"
echo "built $bin/pipe-lab and $bin/pipe-lab-unpatched"
