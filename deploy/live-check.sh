#!/usr/bin/env bash
# The live check of an installed t3opt: deploy/live-check.mjs in Playwright
# WebKit (the lab's phone image, built from lab/driver if docker lacks it),
# once as the Safari tab and once as the home-screen app, each with its own
# fresh pairing code from t3opt-server.
#
# Usage: deploy/live-check.sh <origin> <out dir>
#   such as https://lima-opencode-vm.tail2fc05a.ts.net:12443 /tmp/live-check
set -euo pipefail

origin="${1:?usage: live-check.sh <origin> <out dir>}"
out="$(realpath -m "${2:?usage: live-check.sh <origin> <out dir>}")"
here="$(cd "$(dirname "$0")" && pwd)"
image=t3lab-phone:1.64.1
t3="$HOME/.local/share/t3opt/t3-0.1.7/t3"

docker image inspect "$image" >/dev/null 2>&1 || docker build -t "$image" "$here/../lab/driver"
mkdir -p "$out"

status=0
for size in safari home-screen; do
  code="$("$t3" pair --base-dir "$HOME/.t3opt" --label "live check $size" | sed -n 's/^Token: //p')"
  [ -n "$code" ] || { echo "live-check: t3 pair printed no token" >&2; exit 1; }
  # The host's network, so the browser reaches the tailnet name.
  docker run --rm --network host --init --shm-size 512m \
    --user "$(id -u):$(id -g)" -e HOME=/tmp \
    -v "$here:/check:ro" -v "$out:/out" \
    "$image" node /check/live-check.mjs "$origin" "$code" "$size" || status=1
done
exit "$status"
