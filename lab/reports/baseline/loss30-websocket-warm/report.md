# loss30, websocket, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit 55faa86ace18.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 53 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.8s |
| Time to first text | 11.3s |
| Whole reply shown | 51.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.1s / 5.1s / 15.1s |
| Longest silence | 18.3 s |
| Approval shown after it was asked | 4.6s |
| Bytes on the wire down / up | 20882 / 56565 |
| Bytes the app received / sent | 93939 / 7589 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 365 |
| 20 | 9.1 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 4.7 | 1077 | 1263 |
| 50 | 6.9 | 1378 | 1596 |
| 51 | 7.9 | 1378 | 1596 |
