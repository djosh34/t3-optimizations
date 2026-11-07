# addrchange, websocket, warm start, seed 1

Profile addrchange (fault): the phone's address changes once, on a weak link. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 53 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 10.9s |
| Whole reply shown | 52.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 26.4s / 6.4s / 23.4s |
| Longest silence | 26.3 s |
| Approval shown after it was asked | 9.8s |
| Bytes on the wire down / up | 77575 / 233337 |
| Bytes the app received / sent | 155043 / 12092 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 9.4 | 397 | 783 |
| 30 | 19.4 | 397 | 1077 |
| 40 | 2.1 | 1077 | 1166 |
| 50 | 4.4 | 1378 | 1537 |
| 52 | 6.4 | 1378 | 1596 |
