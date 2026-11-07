# B, websocket, warm start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.0s |
| Prompt reached the agent | 1.6s |
| Time to first text | 12.3s |
| Whole reply shown | 45.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.3s / 14.3s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 1.5s |
| Bytes on the wire down / up | 33329 / 132931 |
| Bytes the app received / sent | 100153 / 7637 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.3 | 1378 | 1429 |
| 45 | 6.3 | 1378 | 1596 |
