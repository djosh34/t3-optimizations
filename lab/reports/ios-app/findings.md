# Findings: the unchanged iOS app, v0.1.7 server against the fork

Issue [#47](https://github.com/djosh34/t3-optimizations/issues/47). The table of all 48 runs is in [README.md](README.md); the runs repeated alone are in [alone/](alone/).

## What was compared

- **Client:** the untouched v0.1.7 web client without WebSocket compression (`websocket-nodeflate`). It talks the way the iOS app's SocketRocket does (issue #7). Warm starts.
- **Servers:** the untouched v0.1.7 server (`lab/scripts/build-baseline-server.sh`) against the forked server of `t3code/`, on the same profiles and seeds (1 and 2).
- **The fork's changes:**
  - **Windowed acks:** up to 8 chunks go out before the server waits for the client's acks. Copied from upstream main.
  - **One update for a backed-up client:** a subscription whose oldest unacked chunk is 1 s old waits for the client. The text updates that queue up meanwhile are coalesced into the newest one.

The fork ran from the working tree. During the comparison its server code changed only by a removed comment (`ac62acf6`) and formatting (`d9f253b0`). The lab binary was `16112c25` with the runner script edited (`-dirty`). The one-array change to `OutputProtocol.ts` (`32fa25f5`) came after every run here and does not change behaviour.

## Run conditions

- **The comparison:** 2 runs at a time on 8 cores.
  - 2026-11-08T23:23Z to 2026-11-09T01:59Z: the first 36 runs, A to rtt3000.
  - 02:20Z to 02:55Z: the last 12 runs. The first attempt at those died at 01:59Z when the agent session that had started the runner was torn down.
  - **Load average** (1 min, sampled every minute from 23:50Z; the first 27 minutes, profiles A to C, are not sampled): median 10.9, peak 20.8 over 165 samples. The last 12 runs alone had a median of 11.2 and a peak of 12.6. The host's idle load is about 3 (it runs k3s), so 2 lab runs brought it to about 11.
- **The runs repeated alone** (`alone/`): 1 at a time.
  - outage10: load 2.6 to 4.3 at each start.
  - D and case50: load 3.6 to 6.0 at each start, except 15.5 at the start of `D-0-s1` (04:00Z). The cause was not found; it was not a run of this lab.
- **Path:** kernel TCP over a veth pair. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

## Results

| Profile | What happened |
|---|---|
| A, B, C | Both servers finish on both seeds, with the same timings (within 1.1 s). |
| D (8 s RTT, 16.7% bursty loss) | Neither finishes. The fork shows the first 397 characters on both seeds (first text at 24.1 s and 18.4 s); v0.1.7 shows no reply text. Repeated alone on seed 1: the same. |
| E, case70, rate100 | Neither shows reply text by the deadline on either seed. On case70 the fork's runs carried 35 to 37% fewer bytes down the wire; the phone received 80 application bytes and no reply text in all four runs, so the difference is not in text updates the app saw. Prompt confirmation is within 2.3 s between the servers, in both directions. |
| case50 | Seed 1: the fork showed 397 characters and v0.1.7 none. Repeated alone, neither showed any, so that difference was run-to-run spread. Seed 2: neither shows text. |
| loss30 | Seed 1: the fork showed 1,077 characters, v0.1.7 none. Seed 2: both finish, the fork with a longest silence of 26.0 s against 32.4 s. TCP at 30% loss varies widely between runs (issue #7), so one seed proves little here. |
| rtt3000 (3 s RTT) | Both finish on both seeds. The fork's first text is 1.5 to 1.8 s earlier, with the same max lag. The longest silence is 3.9 s shorter on seed 1 and 3.9 s longer on seed 2. |
| outage10 | Both finish. The fork's first text comes about 2 s later: 25.1 s against 23.0 s, and 24.7 s against 22.5 s. Repeated alone: 25.1 s against 22.7 s, and 24.5 s against 22.9 s. Only the first paragraph is late; the rest of the reply arrives at the same time or slightly earlier. |
| flap | Both finish. Mixed: the first text is 1.0 s later and 0.7 s earlier, and the max lag 1.0 s shorter and 2.0 s longer. |

## Which change does what

These runs were repeated alone with the fork's backed-up threshold set to 0 ms and 500 ms, by editing `BACKED_UP_AFTER_MS` in `t3code/apps/server/src/terminal/OutputProtocol.ts` before each fork run (`/tmp` script, not kept; the runs are `alone/*-0-*`, `*-T0-*` and `*-T500-*`). With 0 ms only one chunk is in flight, as in v0.1.7, so the ack window is off but the coalescing stays.

| Run, alone | v0.1.7 | fork, 1000 ms | fork, 500 ms | fork, 0 ms |
|---|---|---|---|---|
| D seed 1: first text, chars shown | never, 0 | 24.3 s, 397 | | never, 0 |
| outage10 seed 1: first text | 22.7 s | 25.1 s | 24.7 s | 22.4 s |
| outage10 seed 2: first text | 22.9 s | 24.5 s | 23.9 s | 23.0 s |

- **The D gain comes from the window.** Several chunks per 8 s round trip get through instead of one.
- **The outage10 cost comes from the window too.** Chunks the server sends just before the link goes down are already in TCP when the outage starts. Without the window they would still be in the server. Why that costs 2 s more is not proven. A guess: those chunks wait on TCP's retransmission backoff, where v0.1.7 sends its next chunk fresh after the link is back.
- **The coalescing:** no run here isolates it. The fewer wire bytes on case70 come from the two changes together, and may also be TCP behaving differently under loss; no control run separates them. No run shows the coalescing changing what the phone displays.

## Verdict

**Both changes stay, with the threshold at 1000 ms.**

- **The gains:** the window gets reply text through on a slow, long-delay link (D), where v0.1.7 shows nothing in 15 minutes. That holds on both seeds and when repeated alone. The fork also shows the first text sooner on rtt3000. On case70 the fork's runs carried fewer wire bytes, with the cause not isolated.
- **Neutral:** A, B, C, E, case70 (apart from the bytes) and rate100.
- **The cost, an open trade-off:** about 2 s more to the first text after a 10 s outage. It is reproduced alone, and a 500 ms threshold does not remove it. Removing it means switching the window off (0 ms), which also removes the D gain.
- **What is unclear:** loss30, case50 and flap vary from run to run more than the servers differ.
