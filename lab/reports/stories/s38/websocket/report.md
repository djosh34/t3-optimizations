# story, websocket, warm start, seed 38

Profile story (graded): seed 38's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.5s |
| Prompt reached the agent | 1.4s |
| Time to first text | 16.4s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 1077 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 254.6s / 105.6s / 239.6s |
| Longest silence | 263.5s |
| Approval shown after it was asked | 7.6s |
| Bytes on the wire down / up | 55565 / 443822 |
| Bytes the app received / sent | 68949 / 5452 |
| Reply text received / of it duplicates | 1474 / 397 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 344 |
| 20 | 8.5 | 397 | 746 |
| 30 | 18.5 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 6.6 | 1077 | 1340 |
| 60 | 16.6 | 1077 | 1596 |
| 70 | 26.6 | 1077 | 1596 |
| 80 | 36.6 | 1077 | 1596 |
| 90 | 46.6 | 1077 | 1596 |
| 100 | 56.6 | 1077 | 1596 |
| 110 | 66.6 | 1077 | 1596 |
| 120 | 76.6 | 1077 | 1596 |
| 130 | 86.6 | 1077 | 1596 |
| 140 | 96.6 | 1077 | 1596 |
| 150 | 106.6 | 1077 | 1596 |
| 160 | 116.6 | 1077 | 1596 |
| 170 | 126.6 | 1077 | 1596 |
| 180 | 136.6 | 1077 | 1596 |
| 190 | 146.6 | 1077 | 1596 |
| 200 | 156.6 | 1077 | 1596 |
| 210 | 166.6 | 1077 | 1596 |
| 220 | 176.6 | 1077 | 1596 |
| 230 | 186.6 | 1077 | 1596 |
| 240 | 196.6 | 1077 | 1596 |
| 250 | 206.6 | 1077 | 1596 |
| 260 | 216.6 | 1077 | 1596 |
| 270 | 226.6 | 1077 | 1596 |
| 280 | 236.6 | 1077 | 1596 |
| 290 | 246.6 | 1077 | 1596 |
| 298 | 254.6 | 1077 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- Scroll to end shows
