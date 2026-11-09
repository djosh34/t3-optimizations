# story, websocket, warm start, seed 15

Profile story (graded): seed 15's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 57 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.6s |
| Prompt reached the agent | 1.5s |
| Time to first text | 12.4s |
| Whole reply shown | 55.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.4s / 5.3s / 14.4s |
| Longest silence | 18.8s |
| Approval shown after it was asked | 6.1s |
| Bytes on the wire down / up | 29461 / 87867 |
| Bytes the app received / sent | 131865 / 7804 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 341 |
| 20 | 8.4 | 397 | 742 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 2.3 | 1077 | 1169 |
| 50 | 4.5 | 1378 | 1538 |
| 54 | 8.5 | 1378 | 1596 |
