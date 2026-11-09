# story, websocket, warm start, seed 25

Profile story (graded): seed 25's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 64 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.1s |
| Prompt reached the agent | 1.4s |
| Time to first text | 12.7s |
| Whole reply shown | 62.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.5s / 5.5s / 19.5s |
| Longest silence | 23.2s |
| Approval shown after it was asked | 16.9s |
| Bytes on the wire down / up | 28653 / 73331 |
| Bytes the app received / sent | 110508 / 7712 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 344 |
| 20 | 8.5 | 397 | 746 |
| 30 | 18.5 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 3.1 | 1077 | 1200 |
| 60 | 5.3 | 1378 | 1567 |
| 62 | 7.3 | 1378 | 1596 |
