# loss30, websocket-nodeflate, warm start, seed 2

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 57 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.2s |
| Prompt reached the agent | 1.2s |
| Time to first text | 26.0s |
| Whole reply shown | 56.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 24.7s / 7.8s / 22.7s |
| Longest silence | 26.0s |
| Approval shown after it was asked | 6.9s |
| Bytes on the wire down / up | 97793 / 54592 |
| Bytes the app received / sent | 81791 / 5845 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.7 | 0 | 352 |
| 20 | 18.7 | 0 | 754 |
| 30 | 18.7 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 8.8 | 1077 | 1419 |
| 56 | 7.0 | 1378 | 1596 |
