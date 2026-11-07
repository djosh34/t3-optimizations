# B, websocket, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 117 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 71.1s |
| Prompt confirmation | 1.5s |
| Prompt reached the agent | 1.1s |
| Time to first text | 11.9s |
| Whole reply shown | 45.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.8s / 4.8s / 13.8s |
| Longest silence | 16.6 s |
| Approval shown after it was asked | 1.7s |
| Bytes on the wire down / up | 3067524 / 375039 |
| Bytes the app received / sent | 184662 / 13675 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.8 | 0 | 357 |
| 20 | 8.8 | 397 | 758 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.7 | 1378 | 1443 |
| 45 | 6.7 | 1378 | 1596 |
