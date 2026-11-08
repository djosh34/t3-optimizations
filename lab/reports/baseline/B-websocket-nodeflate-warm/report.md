# B, websocket-nodeflate, warm start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.8s |
| Prompt reached the agent | 0.7s |
| Time to first text | 11.6s |
| Whole reply shown | 45.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 5.2s / 14.2s |
| Longest silence | 17.0s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 159224 / 133002 |
| Bytes the app received / sent | 92561 / 6260 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.7 | 1378 | 1443 |
| 45 | 6.7 | 1378 | 1596 |
