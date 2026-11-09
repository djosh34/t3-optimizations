# story, websocket, warm start, seed 37

Profile story (graded): seed 37's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 52 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.3s |
| Prompt reached the agent | 0.8s |
| Time to first text | 11.6s |
| Whole reply shown | 50.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.1s / 6.1s / 18.1s |
| Longest silence | 20.6s |
| Approval shown after it was asked | 4.7s |
| Bytes on the wire down / up | 32917 / 93854 |
| Bytes the app received / sent | 123207 / 7047 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 365 |
| 20 | 9.1 | 397 | 766 |
| 30 | 19.1 | 397 | 1077 |
| 40 | 7.0 | 1077 | 1351 |
| 50 | 9.2 | 1378 | 1596 |
