#!/usr/bin/env bash
# Runs every pipe run of issue #49 into reports/pipe (or the dir given as
# the second argument), N at a time (first argument, default 3). Build first,
# as yourself: lab/scripts/build-pipe-lab.sh and `go build -o bin/lab
# ./cmd/lab` in lab/.
set -euo pipefail

cd "$(dirname "$0")/.."
jobs="${1:-3}"
out="${2:-reports/pipe}"
mkdir -p "$out"

runs=()
for seed in 1 2; do
  for profile in case50 loss70 case70; do
    for client in pipe pipe-unpatched udp; do
      runs+=("$profile $client $seed")
    done
  done
done

# Each run's stderr stays in <run>.err only when the run failed.
printf '%s\n' "${runs[@]}" | xargs -P "$jobs" -L 1 sh -c '
  err="'"$out"'/$0-$1-s$2.err"
  if sudo ./bin/lab run --profile "$0" --client "$1" --seed "$2" --out "'"$out"'" --name "$0-$1-s$2" >/dev/null 2>"$err"; then
    rm "$err"
  else
    echo "run $0 $1 seed $2 failed, see $err"
  fi
'
./bin/lab pipe-summary "$out"
