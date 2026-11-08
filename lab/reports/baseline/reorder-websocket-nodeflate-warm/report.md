# reorder, websocket-nodeflate, warm start, seed 1

Profile reorder (fault): 25% of packets reordered, 10% loss, 100 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.7s |
| Prompt reached the agent | 0.4s |
| Time to first text | 11.2s |
| Whole reply shown | 47.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.5s / 5.5s / 14.5s |
| Longest silence | 17.6s |
| Approval shown after it was asked | 4.7s |
| Bytes on the wire down / up | 126784 / 136009 |
| Bytes the app received / sent | 92424 / 6309 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 9.5 | 397 | 787 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.3 | 1077 | 1367 |
| 47 | 6.5 | 1378 | 1596 |
