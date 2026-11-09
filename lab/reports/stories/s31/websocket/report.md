# story, websocket, warm start, seed 31

Profile story (graded): seed 31's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 213 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.1s |
| Prompt reached the agent | 1.3s |
| Time to first text | 13.9s |
| Whole reply shown | 211.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 172.5s / 66.5s / 161.5s |
| Longest silence | 181.9s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 65100 / 268821 |
| Bytes the app received / sent | 153031 / 9296 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 344 |
| 20 | 8.6 | 397 | 746 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.5 | 1077 | 1140 |
| 50 | 11.5 | 1077 | 1512 |
| 60 | 21.5 | 1077 | 1596 |
| 70 | 31.5 | 1077 | 1596 |
| 80 | 41.5 | 1077 | 1596 |
| 90 | 51.5 | 1077 | 1596 |
| 100 | 61.5 | 1077 | 1596 |
| 110 | 71.5 | 1077 | 1596 |
| 120 | 81.5 | 1077 | 1596 |
| 130 | 91.5 | 1077 | 1596 |
| 140 | 101.5 | 1077 | 1596 |
| 150 | 111.5 | 1077 | 1596 |
| 160 | 121.5 | 1077 | 1596 |
| 170 | 131.5 | 1077 | 1596 |
| 180 | 141.5 | 1077 | 1596 |
| 190 | 151.5 | 1077 | 1596 |
| 200 | 161.5 | 1077 | 1596 |
| 210 | 171.5 | 1077 | 1596 |
| 211 | 172.5 | 1077 | 1596 |

## Driver notes

- sample failed: locator.isEnabled: Timeout 30000ms exceeded.
