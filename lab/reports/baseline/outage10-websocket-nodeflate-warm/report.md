# outage10, websocket-nodeflate, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 24.7s |
| Whole reply shown | 43.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.5s / 6.5s / 20.5s |
| Longest silence | 24.7 s |
| Approval shown after it was asked | 1.2s |
| Bytes on the wire down / up | 125159 / 133329 |
| Bytes the app received / sent | 85741 / 5904 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 19.5 | 0 | 787 |
| 30 | 0.9 | 1077 | 1120 |
| 40 | 3.2 | 1378 | 1497 |
| 43 | 6.2 | 1378 | 1596 |
