# story, websocket, warm start, seed 22

Profile story (graded): seed 22's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 50 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.0s |
| Prompt reached the agent | 2.3s |
| Time to first text | 14.9s |
| Whole reply shown | 49.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.6s / 5.4s / 14.6s |
| Longest silence | 15.6s |
| Approval shown after it was asked | 3.3s |
| Bytes on the wire down / up | 26042 / 89205 |
| Bytes the app received / sent | 113858 / 6425 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.6 | 0 | 311 |
| 20 | 7.6 | 397 | 710 |
| 30 | 17.6 | 397 | 1077 |
| 40 | 6.2 | 1077 | 1325 |
| 48 | 6.4 | 1378 | 1596 |
