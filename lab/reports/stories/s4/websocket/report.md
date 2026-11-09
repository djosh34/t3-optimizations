# story, websocket, warm start, seed 4

Profile story (graded): seed 4's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 61 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.9s |
| Prompt reached the agent | 2.5s |
| Time to first text | 13.6s |
| Whole reply shown | 60.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.4s / 5.0s / 14.4s |
| Longest silence | 18.4s |
| Approval shown after it was asked | 8.4s |
| Bytes on the wire down / up | 27807 / 99199 |
| Bytes the app received / sent | 118229 / 7470 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 303 |
| 20 | 7.4 | 397 | 701 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 2.2 | 1378 | 1461 |
| 60 | 12.2 | 1378 | 1596 |
