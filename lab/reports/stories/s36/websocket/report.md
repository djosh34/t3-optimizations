# story, websocket, warm start, seed 36

Profile story (graded): seed 36's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 62 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 1.5s |
| Time to first text | 13.1s |
| Whole reply shown | 60.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 25.4s / 7.6s / 22.4s |
| Longest silence | 23.9s |
| Approval shown after it was asked | 9.3s |
| Bytes on the wire down / up | 37366 / 84725 |
| Bytes the app received / sent | 120971 / 7703 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 341 |
| 20 | 8.4 | 397 | 742 |
| 30 | 18.4 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 9.3 | 1077 | 1440 |
| 60 | 11.6 | 1378 | 1596 |
