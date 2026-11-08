# outage60, websocket-nodeflate, warm start, seed 1

Profile outage60 (fault): one 60 s outage on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 150 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 132.4s |
| Whole reply shown | 149.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 131.4s / 57.4s / 123.4s |
| Longest silence | 132.4s |
| Approval shown after it was asked | 105.5s |
| Bytes on the wire down / up | 216251 / 336528 |
| Bytes the app received / sent | 136415 / 11998 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 19.4 | 0 | 783 |
| 30 | 29.4 | 0 | 1077 |
| 40 | 39.4 | 0 | 1077 |
| 50 | 49.4 | 0 | 1077 |
| 60 | 59.4 | 0 | 1077 |
| 70 | 69.4 | 0 | 1077 |
| 80 | 79.4 | 0 | 1077 |
| 90 | 89.4 | 0 | 1077 |
| 100 | 99.4 | 0 | 1077 |
| 110 | 109.4 | 0 | 1077 |
| 120 | 119.4 | 0 | 1077 |
| 130 | 129.4 | 0 | 1077 |
| 140 | 5.5 | 1077 | 1297 |
| 148 | 5.7 | 1378 | 1579 |
