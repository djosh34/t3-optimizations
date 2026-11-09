# story, websocket, warm start, seed 43

Profile story (graded): seed 43's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 54 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.4s |
| Prompt reached the agent | 2.2s |
| Time to first text | 13.6s |
| Whole reply shown | 52.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.7s / 5.7s / 14.7s |
| Longest silence | 17.8s |
| Approval shown after it was asked | 4.2s |
| Bytes on the wire down / up | 39220 / 79509 |
| Bytes the app received / sent | 102064 / 6424 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.7 | 0 | 311 |
| 20 | 7.7 | 397 | 710 |
| 30 | 17.7 | 397 | 1077 |
| 40 | 2.8 | 1077 | 1189 |
| 50 | 5.1 | 1378 | 1558 |
| 52 | 7.1 | 1378 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
