# C, websocket-nodeflate, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 188 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 139.6s |
| Prompt confirmation | 3.3s |
| Prompt reached the agent | 0.9s |
| Time to first text | 13.1s |
| Whole reply shown | 47.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.0s / 5.1s / 14.0s |
| Longest silence | 15.8s |
| Approval shown after it was asked | 3.3s |
| Bytes on the wire down / up | 3274680 / 408495 |
| Bytes the app received / sent | 204992 / 13860 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 365 |
| 20 | 9.0 | 397 | 762 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.9 | 1077 | 1383 |
| 46 | 6.1 | 1378 | 1595 |
