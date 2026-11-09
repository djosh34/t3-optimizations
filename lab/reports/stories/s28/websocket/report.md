# story, websocket, warm start, seed 28

Profile story (graded): seed 28's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 104 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.2s |
| Prompt reached the agent | 1.6s |
| Time to first text | 14.9s |
| Whole reply shown | 102.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 55.0s / 12.3s / 49.0s |
| Longest silence | 54.2s |
| Approval shown after it was asked | 3.0s |
| Bytes on the wire down / up | 42115 / 123511 |
| Bytes the app received / sent | 170056 / 11778 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 18.3 | 397 | 1077 |
| 40 | 1.8 | 1077 | 1152 |
| 50 | 4.0 | 1378 | 1524 |
| 60 | 14.0 | 1378 | 1596 |
| 70 | 24.0 | 1378 | 1596 |
| 80 | 34.0 | 1378 | 1596 |
| 90 | 44.0 | 1378 | 1596 |
| 100 | 54.0 | 1378 | 1596 |
| 101 | 55.0 | 1378 | 1596 |
