# outage60, websocket, warm start, seed 1

Profile outage60 (fault): one 60 s outage on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 103 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 87.6s |
| Whole reply shown | 102.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 86.5s / 35.5s / 80.5s |
| Longest silence | 87.6s |
| Approval shown after it was asked | 60.7s |
| Bytes on the wire down / up | 74695 / 291969 |
| Bytes the app received / sent | 133510 / 11786 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 19.5 | 0 | 787 |
| 30 | 29.5 | 0 | 1077 |
| 40 | 39.5 | 0 | 1077 |
| 50 | 49.5 | 0 | 1077 |
| 60 | 59.5 | 0 | 1077 |
| 70 | 69.5 | 0 | 1077 |
| 80 | 79.5 | 0 | 1077 |
| 90 | 1.5 | 1077 | 1140 |
| 100 | 3.7 | 1378 | 1512 |
| 102 | 5.7 | 1378 | 1579 |
