# story, websocket, warm start, seed 30

Profile story (graded): seed 30's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 64 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.5s |
| Prompt reached the agent | 2.6s |
| Time to first text | 14.2s |
| Whole reply shown | 62.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 28.3s / 6.3s / 24.3s |
| Longest silence | 27.4s |
| Approval shown after it was asked | 16.7s |
| Bytes on the wire down / up | 26836 / 110407 |
| Bytes the app received / sent | 112817 / 8084 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.3 | 0 | 299 |
| 20 | 7.3 | 397 | 697 |
| 30 | 17.3 | 397 | 1077 |
| 40 | 27.3 | 397 | 1077 |
| 50 | 2.6 | 1077 | 1181 |
| 60 | 4.8 | 1378 | 1551 |
| 62 | 6.8 | 1378 | 1596 |
