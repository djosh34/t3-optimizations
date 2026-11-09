# story, websocket, warm start, seed 8

Profile story (graded): seed 8's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 55 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.0s |
| Prompt reached the agent | 1.3s |
| Time to first text | 12.5s |
| Whole reply shown | 53.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.6s / 6.3s / 20.6s |
| Longest silence | 23.1s |
| Approval shown after it was asked | 9.2s |
| Bytes on the wire down / up | 26731 / 79497 |
| Bytes the app received / sent | 107933 / 6650 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.6 | 0 | 344 |
| 20 | 8.6 | 397 | 746 |
| 30 | 18.6 | 397 | 1077 |
| 40 | 1.3 | 1077 | 1132 |
| 50 | 3.6 | 1378 | 1505 |
| 53 | 6.6 | 1378 | 1596 |
