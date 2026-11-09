# story, websocket, warm start, seed 12

Profile story (graded): seed 12's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 184 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.6s |
| Prompt reached the agent | 2.6s |
| Time to first text | 14.3s |
| Whole reply shown | 181.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 123.1s / 32.1s / 113.1s |
| Longest silence | 121.7s |
| Approval shown after it was asked | 13.2s |
| Bytes on the wire down / up | 63342 / 246178 |
| Bytes the app received / sent | 176329 / 12523 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.3 | 0 | 294 |
| 20 | 7.3 | 397 | 693 |
| 30 | 17.3 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 9.8 | 1077 | 1452 |
| 70 | 12.1 | 1378 | 1596 |
| 80 | 22.1 | 1378 | 1596 |
| 90 | 32.1 | 1378 | 1596 |
| 100 | 42.1 | 1378 | 1596 |
| 110 | 52.1 | 1378 | 1596 |
| 120 | 62.1 | 1378 | 1596 |
| 130 | 72.1 | 1378 | 1596 |
| 140 | 82.1 | 1378 | 1596 |
| 150 | 92.1 | 1378 | 1596 |
| 160 | 102.1 | 1378 | 1596 |
| 170 | 112.1 | 1378 | 1596 |
| 180 | 122.1 | 1378 | 1596 |
| 181 | 123.1 | 1378 | 1596 |
