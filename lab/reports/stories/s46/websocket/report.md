# story, websocket, warm start, seed 46

Profile story (graded): seed 46's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 85 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.6s |
| Prompt reached the agent | 1.0s |
| Time to first text | 11.9s |
| Whole reply shown | 83.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.9s / 1.9s / 17.9s |
| Longest silence | 42.8s |
| Approval shown after it was asked | 18.1s |
| Bytes on the wire down / up | 29729 / 76678 |
| Bytes the app received / sent | 128514 / 8237 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.9 | 0 | 357 |
| 20 | 8.9 | 397 | 758 |
| 30 | 18.9 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 2.7 | 1077 | 1181 |
| 80 | 4.9 | 1378 | 1551 |
| 83 | 7.9 | 1378 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
