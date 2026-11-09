# story, websocket, warm start, seed 20

Profile story (graded): seed 20's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 62 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 10.4s |
| Prompt reached the agent | 4.4s |
| Time to first text | 16.0s |
| Whole reply shown | 60.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.5s / 5.5s / 16.5s |
| Longest silence | 19.8s |
| Approval shown after it was asked | 9.1s |
| Bytes on the wire down / up | 23500 / 24056 |
| Bytes the app received / sent | 113415 / 7777 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.5 | 0 | 222 |
| 20 | 5.5 | 397 | 621 |
| 30 | 15.5 | 397 | 1023 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 8.0 | 1077 | 1388 |
| 60 | 10.3 | 1378 | 1596 |
