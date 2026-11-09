# story, websocket, warm start, seed 6

Profile story (graded): seed 6's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.7s |
| Prompt reached the agent | 2.7s |
| Time to first text | 14.2s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 1077 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 0.0s / 7.2s |
| Longest silence | 267.9s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 47253 / 277625 |
| Bytes the app received / sent | 53544 / 3481 |
| Reply text received / of it duplicates | 1474 / 397 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.2 | 0 | 294 |
| 20 | 7.2 | 397 | 693 |
| 30 | 17.2 | 397 | 1077 |
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
| 140 | 0.0 | 1077 | 1077 |
| 150 | 0.0 | 1077 | 1077 |
| 160 | 0.0 | 1077 | 1077 |
| 170 | 0.0 | 1077 | 1077 |
| 180 | 0.0 | 1077 | 1077 |
| 190 | 0.0 | 1077 | 1077 |
| 200 | 0.0 | 1077 | 1077 |
| 210 | 0.0 | 1077 | 1077 |
| 220 | 0.0 | 1077 | 1077 |
| 230 | 0.0 | 1077 | 1077 |
| 240 | 0.0 | 1077 | 1077 |
| 250 | 0.0 | 1077 | 1077 |
| 260 | 0.0 | 1077 | 1077 |
| 270 | 0.0 | 1077 | 1077 |
| 280 | 0.0 | 1077 | 1077 |
| 290 | 0.0 | 1077 | 1077 |
| 298 | 0.0 | 1077 | 1077 |
