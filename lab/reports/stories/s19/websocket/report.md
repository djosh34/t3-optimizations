# story, websocket, warm start, seed 19

Profile story (graded): seed 19's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 219 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.3s |
| Prompt reached the agent | 0.9s |
| Time to first text | 11.9s |
| Whole reply shown | 217.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 189.0s / 80.0s / 178.0s |
| Longest silence | 188.4s |
| Approval shown after it was asked | 172.8s |
| Bytes on the wire down / up | 79386 / 284860 |
| Bytes the app received / sent | 145774 / 12091 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 2 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 361 |
| 20 | 9.0 | 397 | 762 |
| 30 | 19.0 | 397 | 1077 |
| 40 | 29.0 | 397 | 1077 |
| 50 | 39.0 | 397 | 1077 |
| 60 | 49.0 | 397 | 1077 |
| 70 | 59.0 | 397 | 1077 |
| 80 | 69.0 | 397 | 1077 |
| 90 | 79.0 | 397 | 1077 |
| 100 | 89.0 | 397 | 1077 |
| 110 | 99.0 | 397 | 1077 |
| 120 | 109.0 | 397 | 1077 |
| 130 | 119.0 | 397 | 1077 |
| 140 | 129.0 | 397 | 1077 |
| 150 | 139.0 | 397 | 1077 |
| 160 | 149.0 | 397 | 1077 |
| 170 | 159.0 | 397 | 1077 |
| 180 | 169.0 | 397 | 1077 |
| 190 | 179.0 | 397 | 1077 |
| 200 | 189.0 | 397 | 1077 |
| 210 | 7.6 | 1077 | 1378 |
| 217 | 6.9 | 1378 | 1596 |
