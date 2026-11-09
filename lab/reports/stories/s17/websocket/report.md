# story, websocket, warm start, seed 17

Profile story (graded): seed 17's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 222 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | 1.4s |
| Time to first text | 47.8s |
| Whole reply shown | 220.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 170.6s / 60.6s / 159.6s |
| Longest silence | 172.9s |
| Approval shown after it was asked | 19.8s |
| Bytes on the wire down / up | 79193 / 342170 |
| Bytes the app received / sent | 174720 / 14813 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 2 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 341 |
| 20 | 18.5 | 0 | 742 |
| 30 | 28.5 | 0 | 1077 |
| 40 | 38.5 | 0 | 1077 |
| 50 | 0.6 | 1077 | 1101 |
| 60 | 10.6 | 1077 | 1477 |
| 70 | 20.6 | 1077 | 1596 |
| 80 | 30.6 | 1077 | 1596 |
| 90 | 40.6 | 1077 | 1596 |
| 100 | 50.6 | 1077 | 1596 |
| 110 | 60.6 | 1077 | 1596 |
| 120 | 70.6 | 1077 | 1596 |
| 130 | 80.6 | 1077 | 1596 |
| 140 | 90.6 | 1077 | 1596 |
| 150 | 100.6 | 1077 | 1596 |
| 160 | 110.6 | 1077 | 1596 |
| 170 | 120.6 | 1077 | 1596 |
| 180 | 130.6 | 1077 | 1596 |
| 190 | 140.6 | 1077 | 1596 |
| 200 | 150.6 | 1077 | 1596 |
| 210 | 160.6 | 1077 | 1596 |
| 220 | 170.6 | 1077 | 1596 |
