# story, websocket, warm start, seed 39

Profile story (graded): seed 39's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 65 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.4s |
| Prompt reached the agent | 2.1s |
| Time to first text | 13.2s |
| Whole reply shown | 63.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.8s / 4.8s / 14.7s |
| Longest silence | 30.2s |
| Approval shown after it was asked | 15.0s |
| Bytes on the wire down / up | 23878 / 60732 |
| Bytes the app received / sent | 118026 / 7828 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.8 | 0 | 318 |
| 20 | 7.8 | 397 | 718 |
| 30 | 17.8 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.7 | 1077 | 1270 |
| 60 | 14.7 | 1077 | 1596 |
| 63 | 10.0 | 1378 | 1596 |
