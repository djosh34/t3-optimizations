# story, websocket, warm start, seed 50

Profile story (graded): seed 50's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 63 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.9s |
| Prompt reached the agent | 3.8s |
| Time to first text | 15.2s |
| Whole reply shown | 61.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.1s / 5.1s / 14.1s |
| Longest silence | 27.5s |
| Approval shown after it was asked | 9.8s |
| Bytes on the wire down / up | 22328 / 63222 |
| Bytes the app received / sent | 124572 / 8272 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.1 | 0 | 247 |
| 20 | 6.1 | 397 | 643 |
| 30 | 16.1 | 397 | 1049 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.4 | 1077 | 1254 |
| 60 | 6.7 | 1378 | 1596 |
| 61 | 7.7 | 1378 | 1596 |
