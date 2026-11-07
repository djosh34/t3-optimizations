# C, websocket-nodeflate, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.7s |
| Prompt reached the agent | 1.5s |
| Time to first text | 12.3s |
| Whole reply shown | 46.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.4s / 5.4s / 14.4s |
| Longest silence | 17.0 s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 124912 / 161784 |
| Bytes the app received / sent | 89134 / 6947 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 341 |
| 20 | 8.4 | 397 | 742 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.8 | 1378 | 1414 |
| 46 | 6.8 | 1378 | 1596 |
