# outage10, websocket-nodeflate, warm start, seed 2

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 43 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 24.5s |
| Whole reply shown | 43.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.5s / 6.5s / 20.5s |
| Longest silence | 24.5s |
| Approval shown after it was asked | 1.0s |
| Bytes on the wire down / up | 121302 / 107216 |
| Bytes the app received / sent | 83836 / 5038 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 19.5 | 0 | 783 |
| 30 | 1.3 | 1077 | 1132 |
| 40 | 3.6 | 1378 | 1505 |
| 43 | 6.6 | 1378 | 1596 |
