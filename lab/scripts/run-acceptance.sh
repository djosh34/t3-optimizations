#!/usr/bin/env bash
# Runs the home-screen web app (--client webapp) on the full lab for issue
# #46: every profile from a warm start, the graded profiles from a cold start
# too, all on seed 1 like the baseline, and the two graded cases on more
# seeds. Keeps each run's report and raw events in
# lab/reports/acceptance/<profile>-webapp-<kind>[-s<seed>]/ and writes the
# acceptance report, with the baseline of lab/reports/baseline beside it, to
# lab/reports/acceptance/README.md.
#
# Usage, as yourself from lab/ after `go build -o bin/lab ./cmd/lab`,
# scripts/build-sidecar.sh and scripts/build-webapp.sh:
#   scripts/run-acceptance.sh [parallel runs, default 2] [profile ...]
#
# Keep the load average below the core count: each run's WebKit takes about
# 1.7 cores (most of it software rendering), and a starved browser stretches
# the timers the report grades. The report gives the load it ran under.
set -euo pipefail

lab="$(cd "$(dirname "$0")/.." && pwd)"
cd "$lab"
jobs="${1:-2}"
shift || true
raw="$lab/reports/runs/acceptance"
dest="$lab/reports/acceptance"
mkdir -p "$raw" "$dest"
rm -f "$raw/failed"

if [[ $# -gt 0 ]]; then
  profiles=("$@")
else
  mapfile -t profiles < <(./bin/lab profiles | awk '{print $1}')
fi
mapfile -t graded < <(./bin/lab profiles | awk '$2 == "graded" {print $1}')

# The graded cases' targets hold or fail per seed: eight seeds each.
extra_seeds=(2 3 4 5 6 7 8)

runs=()
for p in "${profiles[@]}"; do
  runs+=("$p warm 1")
  if printf '%s\n' "${graded[@]}" | grep -qx "$p"; then
    runs+=("$p cold 1")
  fi
  if [[ $p == case70 || $p == case50 ]]; then
    for s in "${extra_seeds[@]}"; do
      runs+=("$p warm $s")
    done
  fi
done

# One run, tried twice: a lab error (not a bad result) gets a second attempt.
# A run that fails both times leaves no report, so no older one stands in.
one() {
  local profile=$1 kind=$2 seed=$3
  local name="$profile-webapp-$kind"
  if [[ $seed != 1 ]]; then
    name+="-s$seed"
  fi
  rm -rf "${dest:?}/$name"
  for attempt in 1 2; do
    rm -rf "${raw:?}/$name"
    if sudo ./bin/lab run --profile "$profile" --client webapp --kind "$kind" --seed "$seed" \
      --out "$raw" --name "$name" >"$raw/$name.log" 2>&1; then
      mkdir -p "$dest/$name"
      cp "$raw/$name"/{report.json,report.md,phone.jsonl,agent.jsonl,sidecar.log} "$dest/$name/"
      tail -n 1 "$raw/$name.log"
      return 0
    fi
    echo "attempt $attempt of $name failed: $(tail -n 1 "$raw/$name.log")"
  done
  echo "$name" >>"$raw/failed"
  return 1
}
export -f one
export raw dest

# The 1-minute load average once a minute while the runs go.
load="$raw/load.log"
: >"$load"
(while true; do
  echo "$(date -u +%H:%M) $(cut -d' ' -f1 /proc/loadavg)" >>"$load"
  sleep 60
done) &
sampler=$!
trap 'kill "$sampler" 2>/dev/null || true' EXIT

status=0
started=$(date -u +%Y-%m-%dT%H:%MZ)
printf '%s\n' "${runs[@]}" | xargs -P "$jobs" -L 1 bash -c 'one "$@"' _ || status=$?
ended=$(date -u +%Y-%m-%dT%H:%MZ)
kill "$sampler" 2>/dev/null || true

{
  echo "# Acceptance on the full chaos lab"
  echo
  echo "The home-screen web app (\`--client webapp\`, issue #46) in WebKit on every profile of the lab, against the"
  echo "WebSocket baseline in [../baseline](../baseline/README.md) on the same profiles, starts and seed. See"
  echo "[lab/README.md](../../README.md) for the profiles and what each measure means."
  echo
  sort -n -k2 "$load" | awk -v jobs="$jobs" -v cores="$(nproc)" -v from="$started" -v to="$ended" '
    { v[NR] = $2 }
    END {
      if (NR == 0) { printf "%s runs at once, %s to %s, on %s cores; no load samples.\n", jobs, from, to, cores; exit }
      printf "%s runs at once, %s to %s, on %s cores. Load average (1 min, sampled every minute): median %s, peak %s.\n",
        jobs, from, to, cores, v[int((NR + 1) / 2)], v[NR]
    }'
  echo
  ./bin/lab acceptance "$dest" "$lab/reports/baseline"
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
