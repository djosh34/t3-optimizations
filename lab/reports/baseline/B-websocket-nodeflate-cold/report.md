# B, websocket-nodeflate, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 112 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 62.5s |
| Prompt confirmation | 2.7s |
| Prompt reached the agent | 2.4s |
| Time to first text | 13.3s |
| Whole reply shown | 47.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.5s / 5.4s / 14.5s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 3270340 / 381463 |
| Bytes the app received / sent | 206239 / 13357 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.5 | 0 | 303 |
| 20 | 7.5 | 397 | 701 |
| 30 | 17.5 | 397 | 1077 |
| 40 | 7.4 | 1077 | 1371 |
| 47 | 6.7 | 1378 | 1596 |
