# outage60, websocket-nodeflate, warm start, seed 1

Profile outage60 (fault): one 60 s outage on a weak link. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 99 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 81.8s |
| Whole reply shown | 98.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 80.5s / 31.5s / 75.5s |
| Longest silence | 81.8s |
| Approval shown after it was asked | 54.4s |
| Bytes on the wire down / up | 208712 / 314638 |
| Bytes the app received / sent | 135889 / 12356 |
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
| 90 | 6.5 | 1077 | 1336 |
| 97 | 5.8 | 1378 | 1579 |
