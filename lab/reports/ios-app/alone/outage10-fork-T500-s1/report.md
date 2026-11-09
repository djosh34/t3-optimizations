# outage10, websocket-nodeflate, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 24.7s |
| Whole reply shown | 43.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.4s / 6.4s / 20.4s |
| Longest silence | 24.7s |
| Approval shown after it was asked | 1.0s |
| Bytes on the wire down / up | 132328 / 152133 |
| Bytes the app received / sent | 85112 / 6253 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 19.4 | 0 | 783 |
| 30 | 1.1 | 1077 | 1124 |
| 40 | 3.3 | 1378 | 1499 |
| 43 | 6.3 | 1378 | 1596 |
