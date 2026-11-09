# story, websocket, warm start, seed 14

Profile story (graded): seed 14's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 1.5s |
| Time to first text | 13.0s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 1077 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 249.2s / 100.2s / 234.2s |
| Longest silence | 268.1s |
| Approval shown after it was asked | 7.2s |
| Bytes on the wire down / up | 32358 / 58565 |
| Bytes the app received / sent | 81586 / 6125 |
| Reply text received / of it duplicates | 1474 / 397 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 341 |
| 20 | 8.4 | 397 | 742 |
| 30 | 18.4 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 1.2 | 1077 | 1128 |
| 60 | 11.2 | 1077 | 1500 |
| 70 | 21.2 | 1077 | 1596 |
| 80 | 31.2 | 1077 | 1596 |
| 90 | 41.2 | 1077 | 1596 |
| 100 | 51.2 | 1077 | 1596 |
| 110 | 61.2 | 1077 | 1596 |
| 120 | 71.2 | 1077 | 1596 |
| 130 | 81.2 | 1077 | 1596 |
| 140 | 91.2 | 1077 | 1596 |
| 150 | 101.2 | 1077 | 1596 |
| 160 | 111.2 | 1077 | 1596 |
| 170 | 121.2 | 1077 | 1596 |
| 180 | 131.2 | 1077 | 1596 |
| 190 | 141.2 | 1077 | 1596 |
| 200 | 151.2 | 1077 | 1596 |
| 210 | 161.2 | 1077 | 1596 |
| 220 | 171.2 | 1077 | 1596 |
| 230 | 181.2 | 1077 | 1596 |
| 240 | 191.2 | 1077 | 1596 |
| 250 | 201.2 | 1077 | 1596 |
| 260 | 211.2 | 1077 | 1596 |
| 270 | 221.2 | 1077 | 1596 |
| 280 | 231.2 | 1077 | 1596 |
| 290 | 241.2 | 1077 | 1596 |
| 298 | 249.2 | 1077 | 1596 |
