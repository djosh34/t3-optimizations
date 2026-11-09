# story, websocket, warm start, seed 13

Profile story (graded): seed 13's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 67 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.2s |
| Prompt reached the agent | 1.5s |
| Time to first text | 12.6s |
| Whole reply shown | 65.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 30.2s / 9.2s / 26.2s |
| Longest silence | 35.1s |
| Approval shown after it was asked | 2.8s |
| Bytes on the wire down / up | 72596 / 98883 |
| Bytes the app received / sent | 128771 / 8000 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 341 |
| 20 | 8.4 | 397 | 742 |
| 30 | 18.4 | 397 | 1077 |
| 40 | 5.2 | 1077 | 1285 |
| 50 | 15.2 | 1077 | 1596 |
| 60 | 25.2 | 1077 | 1596 |
| 65 | 30.2 | 1077 | 1596 |

## Driver notes

- sample failed: locator.isEnabled: Timeout 30000ms exceeded.
