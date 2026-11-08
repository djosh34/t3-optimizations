# loss30, websocket-nodeflate, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 96 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.6s |
| Prompt reached the agent | 1.1s |
| Time to first text | 14.1s |
| Whole reply shown | 95.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 37.4s / 8.8s / 32.4s |
| Longest silence | 60.9s |
| Approval shown after it was asked | 19.0s |
| Bytes on the wire down / up | 139481 / 142997 |
| Bytes the app received / sent | 108510 / 8694 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.8 | 0 | 357 |
| 20 | 8.8 | 397 | 758 |
| 30 | 18.8 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 2.4 | 1077 | 1173 |
| 70 | 12.4 | 1077 | 1542 |
| 80 | 22.4 | 1077 | 1596 |
| 90 | 32.4 | 1077 | 1596 |
| 95 | 37.4 | 1077 | 1596 |
