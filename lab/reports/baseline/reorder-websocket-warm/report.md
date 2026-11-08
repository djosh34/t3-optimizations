# reorder, websocket, warm start, seed 1

Profile reorder (fault): 25% of packets reordered, 10% loss, 100 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.3s |
| Time to first text | 10.9s |
| Whole reply shown | 43.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6s / 4.6s / 13.6s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 0.8s |
| Bytes on the wire down / up | 35699 / 126589 |
| Bytes the app received / sent | 95835 / 5571 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 389 |
| 20 | 9.6 | 397 | 792 |
| 30 | 1.3 | 1077 | 1132 |
| 40 | 3.5 | 1378 | 1505 |
| 43 | 6.5 | 1378 | 1596 |
