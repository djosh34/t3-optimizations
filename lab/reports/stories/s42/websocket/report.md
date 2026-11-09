# story, websocket, warm start, seed 42

Profile story (graded): seed 42's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.1s |
| Prompt reached the agent | 1.4s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 296.5s / 147.5s / 281.5s |
| Longest silence | 298.7s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 17973 / 85725 |
| Bytes the app received / sent | 27741 / 2280 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 344 |
| 20 | 18.5 | 0 | 746 |
| 30 | 28.5 | 0 | 1077 |
| 40 | 38.5 | 0 | 1077 |
| 50 | 48.5 | 0 | 1077 |
| 60 | 58.5 | 0 | 1077 |
| 70 | 68.5 | 0 | 1077 |
| 80 | 78.5 | 0 | 1077 |
| 90 | 88.5 | 0 | 1077 |
| 100 | 98.5 | 0 | 1077 |
| 110 | 108.5 | 0 | 1077 |
| 120 | 118.5 | 0 | 1077 |
| 130 | 128.5 | 0 | 1077 |
| 140 | 138.5 | 0 | 1077 |
| 150 | 148.5 | 0 | 1077 |
| 160 | 158.5 | 0 | 1077 |
| 170 | 168.5 | 0 | 1077 |
| 180 | 178.5 | 0 | 1077 |
| 190 | 188.5 | 0 | 1077 |
| 200 | 198.5 | 0 | 1077 |
| 210 | 208.5 | 0 | 1077 |
| 220 | 218.5 | 0 | 1077 |
| 230 | 228.5 | 0 | 1077 |
| 240 | 238.5 | 0 | 1077 |
| 250 | 248.5 | 0 | 1077 |
| 260 | 258.5 | 0 | 1077 |
| 270 | 268.5 | 0 | 1077 |
| 280 | 278.5 | 0 | 1077 |
| 290 | 288.5 | 0 | 1077 |
| 298 | 296.5 | 0 | 1077 |
