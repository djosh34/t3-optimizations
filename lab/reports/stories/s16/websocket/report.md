# story, websocket, warm start, seed 16

Profile story (graded): seed 16's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | 16.1s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 281.7s / 132.7s / 266.7s |
| Longest silence | 298.8s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 7818 / 5018 |
| Bytes the app received / sent | 0 / 1669 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 3.7 | 0 | 153 |
| 30 | 13.7 | 0 | 555 |
| 40 | 23.7 | 0 | 957 |
| 50 | 33.7 | 0 | 1077 |
| 60 | 43.7 | 0 | 1077 |
| 70 | 53.7 | 0 | 1077 |
| 80 | 63.7 | 0 | 1077 |
| 90 | 73.7 | 0 | 1077 |
| 100 | 83.7 | 0 | 1077 |
| 110 | 93.7 | 0 | 1077 |
| 120 | 103.7 | 0 | 1077 |
| 130 | 113.7 | 0 | 1077 |
| 140 | 123.7 | 0 | 1077 |
| 150 | 133.7 | 0 | 1077 |
| 160 | 143.7 | 0 | 1077 |
| 170 | 153.7 | 0 | 1077 |
| 180 | 163.7 | 0 | 1077 |
| 190 | 173.7 | 0 | 1077 |
| 200 | 183.7 | 0 | 1077 |
| 210 | 193.7 | 0 | 1077 |
| 220 | 203.7 | 0 | 1077 |
| 230 | 213.7 | 0 | 1077 |
| 240 | 223.7 | 0 | 1077 |
| 250 | 233.7 | 0 | 1077 |
| 260 | 243.7 | 0 | 1077 |
| 270 | 253.7 | 0 | 1077 |
| 280 | 263.7 | 0 | 1077 |
| 290 | 273.7 | 0 | 1077 |
| 298 | 281.7 | 0 | 1077 |
