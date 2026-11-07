#!/usr/bin/env bash
# Builds the sidecar and the native phone for the lab's sidecar-native runs,
# as yourself: lab/bin/sidecar and lab/bin/native-phone.
set -euo pipefail

repo="$(cd "$(dirname "$0")/../.." && pwd)"
bin="$repo/lab/bin"
mkdir -p "$bin"
cargo build --manifest-path "$repo/crates/Cargo.toml" --release -p sidecar -p native-phone
install -m 755 "$repo/crates/target/release/sidecar" "$bin/sidecar"
install -m 755 "$repo/crates/target/release/native-phone" "$bin/native-phone"
echo "built $bin/sidecar and $bin/native-phone"
