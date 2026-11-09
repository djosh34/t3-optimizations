# story, websocket, warm start, seed 34

Profile story (graded): seed 34's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 58 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.2s |
| Prompt reached the agent | 2.5s |
| Time to first text | 13.6s |
| Whole reply shown | 56.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.4s / 4.4s / 14.4s |
| Longest silence | 19.7s |
| Approval shown after it was asked | 7.5s |
| Bytes on the wire down / up | 27027 / 61898 |
| Bytes the app received / sent | 111590 / 6841 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 303 |
| 20 | 7.4 | 397 | 701 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 8.7 | 1077 | 1418 |
| 56 | 6.9 | 1378 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
