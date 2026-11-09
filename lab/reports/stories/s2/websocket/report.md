# story, websocket, warm start, seed 2

Profile story (graded): seed 2's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 117 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 1.6s |
| Time to first text | 16.8s |
| Whole reply shown | 115.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 79.0s / 21.0s / 73.0s |
| Longest silence | 85.6s |
| Approval shown after it was asked | 4.5s |
| Bytes on the wire down / up | 38259 / 108196 |
| Bytes the app received / sent | 156367 / 11221 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 4.0 | 1077 | 1237 |
| 50 | 14.0 | 1077 | 1596 |
| 60 | 24.0 | 1077 | 1596 |
| 70 | 34.0 | 1077 | 1596 |
| 80 | 44.0 | 1077 | 1596 |
| 90 | 54.0 | 1077 | 1596 |
| 100 | 64.0 | 1077 | 1596 |
| 110 | 74.0 | 1077 | 1596 |
| 115 | 79.0 | 1077 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
