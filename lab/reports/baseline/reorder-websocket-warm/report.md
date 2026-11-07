# reorder, websocket, warm start, seed 1

Profile reorder (fault): 25% of packets reordered, 10% loss, 100 kbit/s. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.6s |
| Prompt reached the agent | 1.5s |
| Time to first text | 11.8s |
| Whole reply shown | 44.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.4s / 4.7s / 13.4s |
| Longest silence | 17.1 s |
| Approval shown after it was asked | 1.3s |
| Bytes on the wire down / up | 33707 / 121370 |
| Bytes the app received / sent | 90977 / 6375 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.4 | 0 | 336 |
| 20 | 8.4 | 397 | 738 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.7 | 1378 | 1440 |
| 44 | 5.7 | 1378 | 1575 |
