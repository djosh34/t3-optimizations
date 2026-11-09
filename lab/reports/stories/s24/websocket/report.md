# story, websocket, warm start, seed 24

Profile story (graded): seed 24's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 61 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 1.7s |
| Time to first text | 18.7s |
| Whole reply shown | 58.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 8.0s / 15.2s |
| Longest silence | 25.8s |
| Approval shown after it was asked | 4.6s |
| Bytes on the wire down / up | 26226 / 60681 |
| Bytes the app received / sent | 122918 / 8301 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.2 | 0 | 329 |
| 20 | 8.2 | 397 | 730 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 10.0 | 1077 | 1461 |
| 58 | 10.3 | 1378 | 1596 |
