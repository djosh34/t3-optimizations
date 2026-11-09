# story, websocket, warm start, seed 45

Profile story (graded): seed 45's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 55 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.2s |
| Prompt reached the agent | 2.1s |
| Time to first text | 16.3s |
| Whole reply shown | 53.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.8s / 7.8s / 15.8s |
| Longest silence | 20.7s |
| Approval shown after it was asked | 1.9s |
| Bytes on the wire down / up | 22945 / 55122 |
| Bytes the app received / sent | 113685 / 7500 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.8 | 0 | 315 |
| 20 | 7.8 | 397 | 714 |
| 30 | 17.8 | 397 | 1077 |
| 40 | 5.4 | 1077 | 1292 |
| 50 | 15.4 | 1077 | 1596 |
| 53 | 10.7 | 1378 | 1596 |
