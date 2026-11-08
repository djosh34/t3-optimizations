# loss30, websocket, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 75 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.4s |
| Prompt reached the agent | 1.0s |
| Time to first text | 14.1s |
| Whole reply shown | 74.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 25.8s / 5.7s / 21.8s |
| Longest silence | 26.6s |
| Approval shown after it was asked | 9.3s |
| Bytes on the wire down / up | 23749 / 88561 |
| Bytes the app received / sent | 111264 / 8096 |
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
| 60 | 4.7 | 1077 | 1266 |
| 70 | 7.0 | 1378 | 1596 |
| 74 | 11.0 | 1378 | 1596 |
