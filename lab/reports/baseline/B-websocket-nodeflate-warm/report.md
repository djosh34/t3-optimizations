# B, websocket-nodeflate, warm start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit fabbe3fb6bdd.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.9s |
| Time to first text | 12.1s |
| Whole reply shown | 45.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5 s / 4.9 s / 13.5 s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 2.1s |
| Bytes on the wire down / up | 122622 / 138040 |
| Bytes the app received / sent | 248892 / 20575 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.5 | 0 | 344 |
| 20 | 8.5 | 397 | 746 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.1 | 1378 | 1424 |
| 45 | 6.1 | 1378 | 1595 |
