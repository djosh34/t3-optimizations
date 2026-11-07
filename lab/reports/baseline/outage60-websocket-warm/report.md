# outage60, websocket, warm start, seed 1

Profile outage60 (fault): one 60 s outage on a weak link. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 101 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 84.8s |
| Whole reply shown | 100.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 83.5s / 33.5s / 78.5s |
| Longest silence | 84.8 s |
| Approval shown after it was asked | 57.5s |
| Bytes on the wire down / up | 74898 / 259550 |
| Bytes the app received / sent | 129683 / 10897 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 19.5 | 0 | 783 |
| 30 | 29.5 | 0 | 1077 |
| 40 | 39.5 | 0 | 1077 |
| 50 | 49.5 | 0 | 1077 |
| 60 | 59.5 | 0 | 1077 |
| 70 | 69.5 | 0 | 1077 |
| 80 | 79.5 | 0 | 1077 |
| 90 | 4.0 | 1077 | 1237 |
| 100 | 6.3 | 1378 | 1596 |
