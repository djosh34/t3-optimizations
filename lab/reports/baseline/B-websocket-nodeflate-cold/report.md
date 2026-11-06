# B, websocket-nodeflate, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit fabbe3fb6bdd.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 126 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 66.2s |
| Prompt confirmation | 2.2s |
| Time to first text | 11.9s |
| Whole reply shown | 58.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.0 s / 5.3 s / 14.3 s |
| Longest silence | 16.7 s |
| Approval shown after it was asked | 7.3s |
| Bytes on the wire down / up | 3291691 / 402358 |
| Bytes the app received / sent | 363478 / 28300 |
| Reply text received / of it duplicates | 1634 / 557 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 365 |
| 20 | 9.0 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 4.1 | 1077 | 1242 |
| 50 | 6.3 | 1378 | 1596 |
| 58 | 14.3 | 1378 | 1596 |
