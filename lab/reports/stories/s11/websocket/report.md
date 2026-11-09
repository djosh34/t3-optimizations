# story, websocket, warm start, seed 11

Profile story (graded): seed 11's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 55 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.6s |
| Prompt reached the agent | 1.8s |
| Time to first text | 13.0s |
| Whole reply shown | 52.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.5s / 7.1s / 16.5s |
| Longest silence | 22.9s |
| Approval shown after it was asked | 3.0s |
| Bytes on the wire down / up | 31532 / 106601 |
| Bytes the app received / sent | 118997 / 7803 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.1 | 0 | 325 |
| 20 | 8.1 | 397 | 727 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 6.5 | 1077 | 1336 |
| 50 | 16.5 | 1077 | 1596 |
| 52 | 18.5 | 1077 | 1596 |
