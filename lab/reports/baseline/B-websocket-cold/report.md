# B, websocket, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit 1bf7497ef685-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 122 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 74.3s |
| Prompt confirmation | 1.4s |
| Time to first text | 11.8s |
| Whole reply shown | 46.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.9 s / 5.4 s / 13.9 s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 1.7s |
| Bytes on the wire down / up | 3071805 / 373753 |
| Bytes the app received / sent | 339689 / 26421 |
| Reply text received / of it duplicates | 1634 / 557 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.9 | 0 | 357 |
| 20 | 8.9 | 397 | 758 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.6 | 1378 | 1435 |
| 46 | 7.6 | 1378 | 1596 |
