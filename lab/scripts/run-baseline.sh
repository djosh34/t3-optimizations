#!/usr/bin/env bash
# Runs both baseline clients on every profile (warm start), and on the graded
# profiles also from a cold start. Keeps each run's report and raw events in
# lab/reports/baseline/<profile>-<client>-<kind>/ and writes the summary table
# to lab/reports/baseline/README.md.
#
# Usage, as yourself from lab/ after `go build -o bin/lab ./cmd/lab` and
# scripts/build-baseline-client.sh:
#   scripts/run-baseline.sh [parallel runs, default 3] [profile ...]
set -euo pipefail

lab="$(cd "$(dirname "$0")/.." && pwd)"
cd "$lab"
jobs="${1:-3}"
shift || true
raw="$lab/reports/runs/baseline"
dest="$lab/reports/baseline"
mkdir -p "$raw" "$dest"

if [[ $# -gt 0 ]]; then
  profiles=("$@")
else
  mapfile -t profiles < <(./bin/lab profiles | awk '{print $1}')
fi
mapfile -t graded < <(./bin/lab profiles | awk '$2 == "graded" {print $1}')

runs=()
for p in "${profiles[@]}"; do
  for c in websocket websocket-nodeflate; do
    runs+=("$p $c warm")
    if printf '%s\n' "${graded[@]}" | grep -qx "$p"; then
      runs+=("$p $c cold")
    fi
  done
done

# One run, tried twice: a lab error (not a bad result) gets a second attempt.
# A run that fails both times leaves no report, so no older one stands in.
one() {
  local profile=$1 client=$2 kind=$3
  local name="$profile-$client-$kind"
  rm -rf "${dest:?}/$name"
  for attempt in 1 2; do
    rm -rf "${raw:?}/$name"
    if sudo ./bin/lab run --profile "$profile" --client "$client" --kind "$kind" --seed 1 \
      --out "$raw" --name "$name" >"$raw/$name.log" 2>&1; then
      mkdir -p "$dest/$name"
      cp "$raw/$name"/{report.json,report.md,phone.jsonl,agent.jsonl} "$dest/$name/"
      tail -n 1 "$raw/$name.log"
      return 0
    fi
    echo "attempt $attempt of $name failed: $(tail -n 1 "$raw/$name.log")"
  done
  return 1
}
export -f one
export raw dest

status=0
printf '%s\n' "${runs[@]}" | xargs -P "$jobs" -L 1 bash -c 'one "$@"' _ || status=$?

{
  echo "# Baseline reports"
  echo
  echo "The untouched t3code v0.1.7 web client over WebSocket, with compression (\`websocket\`) and"
  echo "without it like the unchanged iOS app (\`websocket-nodeflate\`). Seed 1. See [lab/README.md](../../README.md)"
  echo "for the profiles and what each measure means. All runs ran at commit $(git describe --always --dirty --abbrev=12)."
  echo
  echo "Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's"
  echo "userspace stack (issue #7), so production numbers differ."
  echo
  ./bin/lab summary "$dest"
} >"$dest/README.md"

if [[ $status -ne 0 ]]; then
  echo "some runs failed twice; see the logs in $raw" >&2
fi
exit "$status"
