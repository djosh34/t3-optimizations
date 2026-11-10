#!/usr/bin/env bash
# Opening a thread the phone does not have yet, in Playwright WebKit (issue
# #84), once as the Safari tab and once as the home-screen app: for each, a
# new `t3opt test …` thread with one reply-only prompt (native-phone's
# test-thread, through the sidecar), then deploy/open-test-thread.mjs taps
# it in a freshly paired web app. Run the sidecar with --preload 0, so the
# thread reaches the phone only when it is opened.
#
# Usage: deploy/open-test-thread.sh <origin> <t3 base dir> <bin dir> <out dir>
#   bin dir: where crates/native-phone's test-thread is.
set -euo pipefail

origin="${1:?usage: open-test-thread.sh <origin> <t3 base dir> <bin dir> <out dir>}"
base="${2:?t3 base dir}"
bin="${3:?bin dir}"
out="$(realpath -m "${4:?out dir}")"
here="$(cd "$(dirname "$0")" && pwd)"
image=t3lab-phone:1.64.1
t3="$HOME/.local/share/t3opt/t3-0.1.7/t3"
code() {
  "$t3" pair --base-dir "$base" --label "$1" | sed -n 's/^Token: //p'
}

docker image inspect "$image" >/dev/null 2>&1 || docker build -t "$image" "$here/../lab/driver"
mkdir -p "$out"

status=0
for size in ${SIZES:-safari home-screen}; do
  title="t3opt test #84 $size $(date -u +%m%d-%H%M)"
  pairing=()
  [ -f "$out/test-phone.json" ] || pairing=(--code "$(code "t3opt test #84 thread maker")")
  "$bin/test-thread" --origin "$origin" --pairing "$out/test-phone.json" "${pairing[@]}" \
    --project t3-optimizations --title "$title"
  docker run --rm --network host --init --shm-size 512m \
    --user "$(id -u):$(id -g)" -e HOME=/tmp -e TRACE="${TRACE:-}" \
    -v "$here:/check:ro" -v "$out:/out" \
    "$image" node /check/open-test-thread.mjs "$origin" "$(code "t3opt test #84 webkit $size")" \
    "$size" "$title" ready || status=1
done
exit "$status"
