# story, websocket, warm start, seed 9

Profile story (graded): seed 9's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 229 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.6s |
| Prompt reached the agent | 1.7s |
| Time to first text | 15.2s |
| Whole reply shown | 226.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 176.4s / 63.4s / 164.4s |
| Longest silence | 174.7s |
| Approval shown after it was asked | 3.9s |
| Bytes on the wire down / up | 57122 / 284708 |
| Bytes the app received / sent | 171037 / 12454 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.2 | 0 | 333 |
| 20 | 8.2 | 397 | 734 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 8.2 | 1077 | 1398 |
| 60 | 10.4 | 1378 | 1596 |
| 70 | 20.4 | 1378 | 1596 |
| 80 | 30.4 | 1378 | 1596 |
| 90 | 40.4 | 1378 | 1596 |
| 100 | 50.4 | 1378 | 1596 |
| 110 | 60.4 | 1378 | 1596 |
| 120 | 70.4 | 1378 | 1596 |
| 130 | 80.4 | 1378 | 1596 |
| 140 | 90.4 | 1378 | 1596 |
| 150 | 100.4 | 1378 | 1596 |
| 160 | 110.4 | 1378 | 1596 |
| 170 | 120.4 | 1378 | 1596 |
| 180 | 130.4 | 1378 | 1596 |
| 190 | 140.4 | 1378 | 1596 |
| 200 | 150.4 | 1378 | 1596 |
| 210 | 160.4 | 1378 | 1596 |
| 220 | 170.4 | 1378 | 1596 |
| 226 | 176.4 | 1378 | 1596 |
