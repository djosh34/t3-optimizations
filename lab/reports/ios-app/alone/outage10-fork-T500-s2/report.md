# outage10, websocket-nodeflate, warm start, seed 2

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 23.9s |
| Whole reply shown | 43.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.5s / 6.5s / 19.5s |
| Longest silence | 23.9s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 135515 / 147186 |
| Bytes the app received / sent | 92884 / 5673 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 19.5 | 0 | 783 |
| 30 | 1.2 | 1077 | 1128 |
| 40 | 3.5 | 1378 | 1500 |
| 43 | 6.5 | 1378 | 1596 |
