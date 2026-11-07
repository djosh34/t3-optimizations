# B, websocket-nodeflate, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 441 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 72.6s |
| Prompt confirmation | 5.0s |
| Prompt reached the agent | 0.9s |
| Time to first text | 13.9s |
| Whole reply shown | 367.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 321.9s / 137.9s / 302.9s |
| Longest silence | 316.7s |
| Approval shown after it was asked | 8.7s |
| Bytes on the wire down / up | 3448408 / 724539 |
| Bytes the app received / sent | 250234 / 19088 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 4 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 361 |
| 20 | 9.0 | 397 | 762 |
| 30 | 19.0 | 397 | 1077 |
| 40 | 2.6 | 1077 | 1181 |
| 50 | 12.6 | 1077 | 1551 |
| 60 | 14.9 | 1378 | 1596 |
| 70 | 24.9 | 1378 | 1596 |
| 80 | 34.9 | 1378 | 1596 |
| 90 | 44.9 | 1378 | 1596 |
| 100 | 54.9 | 1378 | 1596 |
| 110 | 64.9 | 1378 | 1596 |
| 120 | 74.9 | 1378 | 1596 |
| 130 | 84.9 | 1378 | 1596 |
| 140 | 94.9 | 1378 | 1596 |
| 150 | 104.9 | 1378 | 1596 |
| 160 | 114.9 | 1378 | 1596 |
| 170 | 124.9 | 1378 | 1596 |
| 180 | 134.9 | 1378 | 1596 |
| 190 | 144.9 | 1378 | 1596 |
| 200 | 154.9 | 1378 | 1596 |
| 210 | 164.9 | 1378 | 1596 |
| 220 | 174.9 | 1378 | 1596 |
| 230 | 184.9 | 1378 | 1596 |
| 240 | 194.9 | 1378 | 1596 |
| 250 | 204.9 | 1378 | 1596 |
| 260 | 214.9 | 1378 | 1596 |
| 270 | 224.9 | 1378 | 1596 |
| 280 | 234.9 | 1378 | 1596 |
| 290 | 244.9 | 1378 | 1596 |
| 300 | 254.9 | 1378 | 1596 |
| 310 | 264.9 | 1378 | 1596 |
| 320 | 274.9 | 1378 | 1596 |
| 330 | 284.9 | 1378 | 1596 |
| 340 | 294.9 | 1378 | 1596 |
| 350 | 304.9 | 1378 | 1596 |
| 360 | 314.9 | 1378 | 1596 |
| 367 | 321.9 | 1378 | 1596 |
