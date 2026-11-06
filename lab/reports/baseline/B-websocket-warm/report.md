# B, websocket, warm start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit 1bf7497ef685-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.8s |
| Time to first text | 11.9s |
| Whole reply shown | 45.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2 s / 5.2 s / 14.2 s |
| Longest silence | 16.2 s |
| Approval shown after it was asked | 1.6s |
| Bytes on the wire down / up | 36968 / 106570 |
| Bytes the app received / sent | 247089 / 19555 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 369 |
| 20 | 9.2 | 397 | 770 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.3 | 1378 | 1425 |
| 45 | 6.3 | 1378 | 1596 |
