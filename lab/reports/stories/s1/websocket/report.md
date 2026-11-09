# story, websocket, warm start, seed 1

Profile story (graded): seed 1's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.1s |
| Prompt reached the agent | 1.6s |
| Time to first text | 13.5s |
| Whole reply shown | 46.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.9s / 14.3s |
| Longest silence | 16.2s |
| Approval shown after it was asked | 1.6s |
| Bytes on the wire down / up | 40823 / 96937 |
| Bytes the app received / sent | 121349 / 6534 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 333 |
| 20 | 8.3 | 397 | 734 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.7 | 1077 | 1414 |
| 46 | 6.9 | 1378 | 1596 |
