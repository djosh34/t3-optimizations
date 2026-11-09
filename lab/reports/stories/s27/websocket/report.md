# story, websocket, warm start, seed 27

Profile story (graded): seed 27's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 146 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 13.2s |
| Prompt reached the agent | 8.1s |
| Time to first text | 27.6s |
| Whole reply shown | 145.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.8s / 0.0s / 14.8s |
| Longest silence | 103.5s |
| Approval shown after it was asked | 92.9s |
| Bytes on the wire down / up | 54857 / 170202 |
| Bytes the app received / sent | 137458 / 12305 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.8 | 0 | 77 |
| 20 | 11.8 | 0 | 478 |
| 30 | 11.8 | 397 | 882 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 0.0 | 1077 | 1077 |
| 90 | 0.0 | 1077 | 1077 |
| 100 | 0.0 | 1077 | 1077 |
| 110 | 0.0 | 1077 | 1077 |
| 120 | 0.0 | 1077 | 1077 |
| 130 | 0.0 | 1077 | 1077 |
| 140 | 2.1 | 1378 | 1456 |
| 145 | 7.1 | 1378 | 1596 |
