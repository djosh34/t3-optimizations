# rtt3000, websocket-nodeflate, warm start, seed 2

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 57 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.3s |
| Prompt reached the agent | 2.1s |
| Time to first text | 14.1s |
| Whole reply shown | 56.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.8s / 5.8s / 15.8s |
| Longest silence | 21.2s |
| Approval shown after it was asked | 5.8s |
| Bytes on the wire down / up | 112074 / 71049 |
| Bytes the app received / sent | 88150 / 6642 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.8 | 0 | 315 |
| 20 | 7.8 | 397 | 714 |
| 30 | 17.8 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 9.3 | 1077 | 1435 |
| 56 | 7.6 | 1378 | 1596 |
