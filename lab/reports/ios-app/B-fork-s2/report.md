# B, websocket-nodeflate, warm start, seed 2

Profile B (graded): congested LTE, queue and stall stress. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.8s |
| Prompt reached the agent | 0.6s |
| Time to first text | 11.3s |
| Whole reply shown | 46.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.3s / 14.3s |
| Longest silence | 17.2s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 122218 / 126532 |
| Bytes the app received / sent | 88111 / 6274 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 377 |
| 20 | 9.3 | 397 | 779 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.9 | 1378 | 1418 |
| 46 | 6.9 | 1378 | 1596 |
