# C, websocket, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.8s |
| Time to first text | 11.6s |
| Whole reply shown | 45.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1s / 4.9s / 14.1s |
| Longest silence | 17.3s |
| Approval shown after it was asked | 2.6s |
| Bytes on the wire down / up | 30710 / 98628 |
| Bytes the app received / sent | 92180 / 5643 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 369 |
| 20 | 9.1 | 397 | 770 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.9 | 1378 | 1418 |
| 45 | 5.9 | 1378 | 1587 |
