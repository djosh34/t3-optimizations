# Pipe reports

The WebRTC data channel alone, before any protocol is built on it (issue #49). Each end sends a 25-byte tick every 100 ms for 2 minutes after the switch to the profile: WebKit in the phone container, unordered with `maxRetransmits: 0`, and `pipe-lab webrtc` from `crates/pipe` on the server. See [lab/README.md](../../README.md#pipe-runs) for how a run works and what each measure means.

- `pipe`: str0m with the patched sctp-proto (2 s retransmit cap, no congestion-window cut on loss; [crates/vendor/README.md](../../../crates/vendor/README.md)).
- `pipe-unpatched`: the same code with sctp-proto as published.
- `udp`: plain UDP on the same link, the best any transport can do there.

Profiles: case50 (50% random loss, 100 kbit/s), loss70 (70%, 100 kbit/s) and case70 (70%, 10 kbit/s), all 300 ± 50 ms each way. Seeds 1 and 2. Every run was made with the lab and `pipe-lab` built from commit 663e401402c1, with `scripts/run-pipe.sh 3`, three runs at a time on the dev VM.

## Result

Down is server to phone: the direction the patch changes and the one that carries the thread's text. Longest gap between two ticks arriving, both seeds:

| Profile | pipe | pipe-unpatched | udp |
|---|---|---|---|
| case50 | 1.14 s, 1.08 s | 33.39 s, 0.95 s | 0.92 s, 0.96 s |
| loss70 | 1.45 s, 2.00 s | 16.25 s, 46.69 s | 1.86 s, 1.85 s |
| case70 | 1.89 s, 1.70 s | 35.63 s, 49.17 s | 1.87 s, 1.83 s |

With the patch, the downlink is as good as plain UDP: the longest gap stays within 0.25 s of UDP's, the same share of ticks arrives, and ticks are no older than UDP's (p95 0.4 s at 100 kbit/s, 0.6 to 0.7 s at 10 kbit/s against 0.43 s). Without it, SCTP backs its retransmit timer off and cuts its window to one packet, and ticks wait: gaps of 16 to 49 s, ticks up to 53 s old, and only 9 to 25% delivered at 70% loss. One unpatched run (case50, seed 2) had no such stall in its 2 minutes.

Up is phone to server, WebKit's own SCTP, which the patch does not touch. At 50% it is close to UDP (1.4 to 1.6 s against 1.0 to 1.3 s). At 70% its longest gaps are 1.9 to 7.6 s against UDP's 1.6 to 1.9 s, with and without the patch, and up to 41 ticks in a row go missing where UDP loses at most 18. Random loss of 70% almost never takes 30 in a row (0.7^30 is about 2 in 100,000). So WebKit's sender stalls now and then. A guess, not proven: libwebrtc caps its retransmit-timer backoff near 3 s, which fits the 3 to 4 s gaps at 100 kbit/s. The protocol built on top (issue #43) has to live with this direction as it is.

## All runs

| Run | Client | Profile | Seed | Down: longest gap / delivered / age p95 | Up: longest gap / delivered / age p95 |
|---|---|---|---|---|---|
| [case50-pipe-s1](case50-pipe-s1/report.md) | pipe | case50 | 1 | 1.14s / 49.7% / 0.40s | 1.64s / 47.4% / 0.39s |
| [case50-pipe-s2](case50-pipe-s2/report.md) | pipe | case50 | 2 | 1.08s / 49.6% / 0.40s | 1.43s / 46.7% / 0.39s |
| [case50-pipe-unpatched-s1](case50-pipe-unpatched-s1/report.md) | pipe-unpatched | case50 | 1 | 33.39s / 22.7% / 33.44s | 1.33s / 47.8% / 0.39s |
| [case50-pipe-unpatched-s2](case50-pipe-unpatched-s2/report.md) | pipe-unpatched | case50 | 2 | 0.95s / 50.9% / 0.39s | 1.98s / 46.4% / 0.40s |
| [case50-udp-s1](case50-udp-s1/report.md) | udp | case50 | 1 | 0.92s / 49.2% / 0.38s | 0.98s / 49.2% / 0.39s |
| [case50-udp-s2](case50-udp-s2/report.md) | udp | case50 | 2 | 0.96s / 49.0% / 0.39s | 1.31s / 48.7% / 0.39s |
| [case70-pipe-s1](case70-pipe-s1/report.md) | pipe | case70 | 1 | 1.89s / 30.4% / 0.71s | 7.60s / 27.5% / 3.24s |
| [case70-pipe-s2](case70-pipe-s2/report.md) | pipe | case70 | 2 | 1.70s / 27.1% / 0.60s | 4.43s / 26.8% / 0.64s |
| [case70-pipe-unpatched-s1](case70-pipe-unpatched-s1/report.md) | pipe-unpatched | case70 | 1 | 35.63s / 11.3% / 40.63s | 3.01s / 27.6% / 0.55s |
| [case70-pipe-unpatched-s2](case70-pipe-unpatched-s2/report.md) | pipe-unpatched | case70 | 2 | 49.17s / 9.4% / 52.09s | 4.18s / 26.1% / 0.58s |
| [case70-udp-s1](case70-udp-s1/report.md) | udp | case70 | 1 | 1.87s / 30.1% / 0.43s | 1.85s / 28.5% / 0.43s |
| [case70-udp-s2](case70-udp-s2/report.md) | udp | case70 | 2 | 1.83s / 28.7% / 0.44s | 1.63s / 29.5% / 0.43s |
| [loss70-pipe-s1](loss70-pipe-s1/report.md) | pipe | loss70 | 1 | 1.45s / 30.7% / 0.39s | 1.87s / 25.2% / 0.39s |
| [loss70-pipe-s2](loss70-pipe-s2/report.md) | pipe | loss70 | 2 | 2.00s / 28.4% / 0.39s | 2.78s / 25.9% / 0.56s |
| [loss70-pipe-unpatched-s1](loss70-pipe-unpatched-s1/report.md) | pipe-unpatched | loss70 | 1 | 16.25s / 25.4% / 4.04s | 3.03s / 26.6% / 0.41s |
| [loss70-pipe-unpatched-s2](loss70-pipe-unpatched-s2/report.md) | pipe-unpatched | loss70 | 2 | 46.69s / 16.4% / 25.01s | 3.21s / 24.4% / 0.44s |
| [loss70-udp-s1](loss70-udp-s1/report.md) | udp | loss70 | 1 | 1.86s / 30.1% / 0.38s | 1.85s / 28.5% / 0.39s |
| [loss70-udp-s2](loss70-udp-s2/report.md) | udp | loss70 | 2 | 1.85s / 28.5% / 0.39s | 1.64s / 29.4% / 0.39s |
