# outage10, websocket-nodeflate, warm start, seed 2

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 23.0s |
| Whole reply shown | 44.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.4s / 6.4s / 19.4s |
| Longest silence | 23.0s |
| Approval shown after it was asked | 1.9s |
| Bytes on the wire down / up | 127869 / 133773 |
| Bytes the app received / sent | 89978 / 6201 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 19.4 | 0 | 783 |
| 30 | 0.4 | 1077 | 1097 |
| 40 | 2.6 | 1378 | 1473 |
| 44 | 6.6 | 1378 | 1596 |
