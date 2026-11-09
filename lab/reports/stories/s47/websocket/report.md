# story, websocket, warm start, seed 47

Profile story (graded): seed 47's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.6s |
| Prompt reached the agent | 1.2s |
| Time to first text | 12.0s |
| Whole reply shown | 48.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.7s / 5.7s / 14.7s |
| Longest silence | 17.1s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 46420 / 89608 |
| Bytes the app received / sent | 119031 / 6510 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.7 | 0 | 352 |
| 20 | 8.7 | 397 | 754 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.8 | 1077 | 1379 |
| 48 | 8.0 | 1378 | 1596 |
