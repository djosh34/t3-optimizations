# C, websocket, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 175 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 128.7s |
| Prompt confirmation | 1.1s |
| Prompt reached the agent | 0.8s |
| Time to first text | 11.6s |
| Whole reply shown | 44.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1s / 5.1s / 14.1s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 1.7s |
| Bytes on the wire down / up | 3073441 / 412090 |
| Bytes the app received / sent | 185963 / 13708 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 365 |
| 20 | 9.1 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 2.0 | 1378 | 1450 |
| 44 | 6.0 | 1378 | 1587 |
