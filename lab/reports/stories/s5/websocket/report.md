# story, websocket, warm start, seed 5

Profile story (graded): seed 5's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.0s |
| Prompt reached the agent | 1.4s |
| Time to first text | 12.2s |
| Whole reply shown | 47.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.5s / 5.5s / 14.5s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 1.9s |
| Bytes on the wire down / up | 42516 / 64192 |
| Bytes the app received / sent | 104063 / 6460 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 341 |
| 20 | 8.5 | 397 | 742 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.3 | 1077 | 1402 |
| 47 | 7.6 | 1378 | 1596 |
