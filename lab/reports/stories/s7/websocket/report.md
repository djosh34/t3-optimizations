# story, websocket, warm start, seed 7

Profile story (graded): seed 7's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 70 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 10.5s |
| Prompt reached the agent | 2.1s |
| Time to first text | 15.9s |
| Whole reply shown | 68.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 32.8s / 7.8s / 28.8s |
| Longest silence | 29.4s |
| Approval shown after it was asked | 16.6s |
| Bytes on the wire down / up | 23627 / 69556 |
| Bytes the app received / sent | 104912 / 7547 |
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
| 40 | 27.8 | 397 | 1077 |
| 50 | 0.5 | 1077 | 1101 |
| 60 | 2.7 | 1378 | 1477 |
| 68 | 10.7 | 1378 | 1596 |
