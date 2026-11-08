# C, websocket-nodeflate, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.8s |
| Time to first text | 11.7s |
| Whole reply shown | 45.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1s / 5.1s / 14.1s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 125705 / 137802 |
| Bytes the app received / sent | 90612 / 6231 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 365 |
| 20 | 9.1 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.2 | 1378 | 1424 |
| 45 | 6.2 | 1378 | 1595 |
