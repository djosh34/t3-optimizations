# loss30, websocket-nodeflate, warm start, seed 2

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 73 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.9s |
| Prompt reached the agent | 1.3s |
| Time to first text | 32.4s |
| Whole reply shown | 72.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 30.6s / 9.6s / 26.6s |
| Longest silence | 32.4s |
| Approval shown after it was asked | 8.3s |
| Bytes on the wire down / up | 102496 / 70864 |
| Bytes the app received / sent | 85075 / 7164 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.6 | 0 | 344 |
| 20 | 18.6 | 0 | 746 |
| 30 | 28.6 | 0 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.1 | 1077 | 1242 |
| 60 | 6.4 | 1378 | 1596 |
| 70 | 16.4 | 1378 | 1596 |
| 72 | 18.4 | 1378 | 1596 |
