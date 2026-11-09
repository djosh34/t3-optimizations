#!/usr/bin/env bash
# Builds the sidecar and the native phone for the lab's sidecar-native runs,
# and the story generator for story runs, as yourself: lab/bin/sidecar,
# lab/bin/native-phone and lab/bin/story.
set -euo pipefail

repo="$(cd "$(dirname "$0")/../.." && pwd)"
bin="$repo/lab/bin"
mkdir -p "$bin"
cargo build --manifest-path "$repo/crates/Cargo.toml" --release -p sidecar -p native-phone
cargo build --manifest-path "$repo/crates/Cargo.toml" --release -p sync-core --example story
install -m 755 "$repo/crates/target/release/sidecar" "$bin/sidecar"
install -m 755 "$repo/crates/target/release/native-phone" "$bin/native-phone"
install -m 755 "$repo/crates/target/release/examples/story" "$bin/story"
echo "built $bin/sidecar, $bin/native-phone and $bin/story"
