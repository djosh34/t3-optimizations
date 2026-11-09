# story, websocket, warm start, seed 29

Profile story (graded): seed 29's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 7.8s |
| Prompt reached the agent | 6.1s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 291.8s / 142.8s / 276.8s |
| Longest silence | 298.7s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 14046 / 39639 |
| Bytes the app received / sent | 37135 / 2342 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 3.8 | 0 | 153 |
| 20 | 13.8 | 0 | 555 |
| 30 | 23.8 | 0 | 957 |
| 40 | 33.8 | 0 | 1077 |
| 50 | 43.8 | 0 | 1077 |
| 60 | 53.8 | 0 | 1077 |
| 70 | 63.8 | 0 | 1077 |
| 80 | 73.8 | 0 | 1077 |
| 90 | 83.8 | 0 | 1077 |
| 100 | 93.8 | 0 | 1077 |
| 110 | 103.8 | 0 | 1077 |
| 120 | 113.8 | 0 | 1077 |
| 130 | 123.8 | 0 | 1077 |
| 140 | 133.8 | 0 | 1077 |
| 150 | 143.8 | 0 | 1077 |
| 160 | 153.8 | 0 | 1077 |
| 170 | 163.8 | 0 | 1077 |
| 180 | 173.8 | 0 | 1077 |
| 190 | 183.8 | 0 | 1077 |
| 200 | 193.8 | 0 | 1077 |
| 210 | 203.8 | 0 | 1077 |
| 220 | 213.8 | 0 | 1077 |
| 230 | 223.8 | 0 | 1077 |
| 240 | 233.8 | 0 | 1077 |
| 250 | 243.8 | 0 | 1077 |
| 260 | 253.8 | 0 | 1077 |
| 270 | 263.8 | 0 | 1077 |
| 280 | 273.8 | 0 | 1077 |
| 290 | 283.8 | 0 | 1077 |
| 298 | 291.8 | 0 | 1077 |
