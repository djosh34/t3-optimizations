#!/usr/bin/env bash
# Runs the untouched v0.1.7 web client without WebSocket compression, as the
# unchanged iOS app talks (issue #7), against the untouched v0.1.7 server and
# against the forked server, on the same profiles and seeds (issue #47).
# Keeps each run's report and raw events in
# lab/reports/ios-app/<profile>-<server>-s<seed>/ and writes the comparison to
# lab/reports/ios-app/README.md.
#
# Usage, as yourself from lab/ after `go build -o bin/lab ./cmd/lab`,
# scripts/build-baseline-client.sh and scripts/build-baseline-server.sh:
#   scripts/run-ios-app.sh [parallel runs, default 3] [seeds, default "1 2"] [profile ...]
set -euo pipefail

lab="$(cd "$(dirname "$0")/.." && pwd)"
cd "$lab"
jobs="${1:-3}"
seeds="${2:-1 2}"
shift 2 || shift $#
raw="$lab/reports/runs/ios-app"
dest="$lab/reports/ios-app"
mkdir -p "$raw" "$dest"
rm -f "$raw/failed"

if [[ $# -gt 0 ]]; then
  profiles=("$@")
else
  # Every graded profile and the faults a phone link shows; not F (no
  # connection check fits its up windows), the soak or the derp tunnel.
  profiles=(A B C D E case70 case50 loss30 rtt3000 rate100 outage10 flap)
fi

runs=()
for p in "${profiles[@]}"; do
  for seed in $seeds; do
    for server in v0.1.7 fork; do
      runs+=("$p $server $seed")
    done
  done
done

# One run, tried twice: a lab error (not a bad result) gets a second attempt.
# A run that fails both times leaves no report, so no older one stands in.
one() {
  local profile=$1 server=$2 seed=$3
  local name="$profile-$server-s$seed"
  rm -rf "${dest:?}/$name"
  for attempt in 1 2; do
    rm -rf "${raw:?}/$name"
    if sudo ./bin/lab run --profile "$profile" --client websocket-nodeflate --server "$server" \
      --seed "$seed" --out "$raw" --name "$name" >"$raw/$name.log" 2>&1; then
      # xargs runs this without the script's set -e, so a failed copy is checked here.
      if mkdir -p "$dest/$name" &&
        cp "$raw/$name"/{report.json,report.md,phone.jsonl,agent.jsonl} "$dest/$name/"; then
        tail -n 1 "$raw/$name.log"
        return 0
      fi
      echo "copying the report of $name failed"
      rm -rf "${dest:?}/$name"
      echo "$name" >>"$raw/failed"
      return 1
    fi
    echo "attempt $attempt of $name failed: $(tail -n 1 "$raw/$name.log")"
  done
  echo "$name" >>"$raw/failed"
  return 1
}
export -f one
export raw dest

status=0
printf '%s\n' "${runs[@]}" | xargs -P "$jobs" -L 1 bash -c 'one "$@"' _ || status=$?

{
  echo "# The unchanged iOS app: v0.1.7 server against the forked server"
  echo
  echo "The untouched v0.1.7 web client without WebSocket compression (\`websocket-nodeflate\`, as the"
  echo "iOS app's SocketRocket talks; issue #7), against the untouched v0.1.7 server and against the"
  echo "forked server of \`t3code/\`, on the same profiles and seeds. Warm starts. See"
  echo "[lab/README.md](../../README.md) for the profiles and what each measure means. What the runs"
  echo "show, the run conditions and the runs repeated alone are in [findings.md](findings.md)."
  echo
  echo "Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's"
  echo "userspace stack (issue #7), so production numbers differ."
  echo
  ./bin/lab servers "$dest"
  if [[ -s "$raw/failed" ]]; then
    echo
    echo "These runs failed twice with a lab error and have no report:"
    echo
    sort "$raw/failed" | sed 's/^/- /'
  fi
} >"$dest/README.md"

if [[ $status -ne 0 ]]; then
  echo "some runs failed twice; see the logs in $raw" >&2
fi
exit "$status"
