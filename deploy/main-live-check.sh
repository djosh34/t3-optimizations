#!/usr/bin/env bash
# The live check of t3opt-sidecar-main against the host's own t3code (issue
# #85). It never opens the owner's threads in the web app:
#
# 1. crates/native-phone's open-threads opens four existing threads through
#    the sidecar with `sidecar.focus` (never `thread.visit`, so no read mark
#    changes): the newest, the oldest archived, the delegated-task thread
#    with the longest id and the largest. It reports, per thread, the time
#    from the request to the phone storing the thread's newest message.
# 2. deploy/main-live-check.mjs in Playwright WebKit (the lab's phone image):
#    as the Safari tab it starts the one `t3opt live check` thread in the
#    existing t3-optimizations project, with GPT-6 Luna at low thinking, and
#    times the reply; as the home-screen app it opens that thread. Once any
#    thread has the check's prompt, even from a run that failed after the
#    send, no thread is made again.
#
# Meanwhile it samples the memory of both services every 5 s into
# <out dir>/memory.csv. Each client pairs with its own code from
# `t3 pair --base-dir ~/.t3`, a login session in the t3code. stdout names no
# thread, project or message. The out dir holds thread ids (threads.tsv) and
# screenshots of the t3code: look at them here, never upload them, and
# delete the out dir after.
#
# Usage: deploy/main-live-check.sh <origin> <out dir>
#   such as https://lima-opencode-vm.tail2fc05a.ts.net:14443 /tmp/main-live-check
set -euo pipefail

origin="${1:?usage: main-live-check.sh <origin> <out dir>}"
out="$(realpath -m "${2:?usage: main-live-check.sh <origin> <out dir>}")"
here="$(cd "$(dirname "$0")" && pwd)"
image=t3lab-phone:1.64.1
t3="$HOME/.local/share/t3opt/t3-0.1.7/t3"
base="$HOME/.t3"
data="$HOME/.t3opt-sidecar-main"
project="t3-optimizations"
title="t3opt live check"
prompt="Reply with the words: t3opt live check ok"

code() {
  "$t3" pair --base-dir "$base" --label "$1" | sed -n 's/^Token: //p'
}
# Read-only queries on the t3code's database.
sql() {
  sqlite3 "file:$base/userdata/statev2.sqlite?mode=ro" "$1"
}
cgroup() {
  echo "/sys/fs/cgroup$(systemctl --user show -p ControlGroup --value "$1")"
}
# Usage: rss_mb <unit>: the resident memory of all its processes, in MB.
rss_mb() {
  local pids
  pids="$(paste -sd, "$(cgroup "$1")/cgroup.procs")"
  if [ -n "$pids" ]; then ps -o rss= -p "$pids" | awk '{s += $1} END {print int(s / 1024)}'; else echo 0; fi
}
oom_kills() {
  awk '$1 == "oom_kill" {print $2}' "$(cgroup "$1")/memory.events"
}

docker image inspect "$image" >/dev/null 2>&1 || docker build -t "$image" "$here/../lab/driver"
cargo build --release --quiet --manifest-path "$here/../crates/Cargo.toml" -p native-phone --bin open-threads
mkdir -p "$out"

side=t3opt-sidecar-main.service
main=t3code.service
restarts="$(systemctl --user show -p NRestarts --value "$side")"
ooms="$(oom_kills "$side") $(oom_kills "$main")"

echo "time,sidecar_rss_mb,sidecar_cgroup_mb,t3code_rss_mb" >"$out/memory.csv"
(
  while true; do
    cgroup_mb="$(systemctl --user show -p MemoryCurrent --value "$side" | awk '{print int($1 / 1048576)}')"
    echo "$(date -u +%FT%TZ),$(rss_mb "$side"),$cgroup_mb,$(rss_mb "$main")" >>"$out/memory.csv"
    sleep 5
  done
) &
sampler=$!
trap 'kill "$sampler" 2>/dev/null || true' EXIT

status=0

# 1. The owner's threads, through native-phone only.
{
  printf 'newest\t%s\n' "$(sql "select thread_id from orchestration_v2_projection_threads
    where archived_at is null and deleted_at is null and title not like 't3opt %'
    order by updated_at desc limit 1")"
  printf 'old archived\t%s\n' "$(sql "select thread_id from orchestration_v2_projection_threads
    where archived_at is not null and deleted_at is null order by updated_at limit 1")"
  printf 'delegated-task\t%s\n' "$(sql "select thread_id from orchestration_v2_projection_threads
    where thread_id like 'thread:delegated-task:%' and deleted_at is null
    order by length(thread_id) desc limit 1")"
  printf 'largest\t%s\n' "$(sql "select i.thread_id from orchestration_v2_projection_turn_items i
    join orchestration_v2_projection_threads t using (thread_id) where t.deleted_at is null
    group by i.thread_id order by sum(length(i.payload_json)) desc limit 1")"
} >"$out/threads.tsv"
"$here/../crates/target/release/open-threads" --origin "$origin" --pairing "$out/native-phone.json" \
  --code "$(code "$title native-phone")" \
  --t3-url http://127.0.0.1:3773 --t3-token-file "$data/t3-token" \
  --threads "$out/threads.tsv" --settle-ms 3000 || status=1

# 2. The check's own thread, in WebKit.
for size in safari home-screen; do
  if [ "$size" = safari ] &&
    [ "$(sql "select count(*) from orchestration_v2_projection_messages where role = 'user'
      and payload_json like '%t3opt live check ok%' and json_extract(payload_json, '\$.text') = '$prompt'")" != 0 ]; then
    echo "safari: a thread has the prompt already; not made again"
    continue
  fi
  docker run --rm --network host --init --shm-size 512m \
    --user "$(id -u):$(id -g)" -e HOME=/tmp \
    -v "$here:/check:ro" -v "$out:/out" \
    "$image" node /check/main-live-check.mjs "$origin" "$(code "$title $size")" \
    "$size" "$project" "$title" || status=1
done

kill "$sampler"
awk -F, 'NR > 1 {
    for (i = 2; i <= 4; i++) if ($i > max[i]) max[i] = $i
    n++
  } END {
    printf "memory, %d samples: sidecar max %d MB RSS, %d MB cgroup; t3code max %d MB RSS\n", n, max[2], max[3], max[4]
  }' "$out/memory.csv"
read -r side_ooms main_ooms <<<"$ooms"
side_ooms=$(($(oom_kills "$side") - side_ooms))
main_ooms=$(($(oom_kills "$main") - main_ooms))
restarts=$(($(systemctl --user show -p NRestarts --value "$side") - restarts))
echo "sidecar: $side_ooms OOM kills, $restarts restarts; t3code: $main_ooms OOM kills"
[ $((side_ooms + main_ooms + restarts)) = 0 ] || status=1
exit "$status"
