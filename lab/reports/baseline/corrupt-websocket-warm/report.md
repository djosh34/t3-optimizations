# corrupt, websocket, warm start, seed 1

Profile corrupt (fault): 10% of packets corrupted, 100 kbit/s. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.7s |
| Time to first text | 13.1s |
| Whole reply shown | 43.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 5.5s / 14.2s |
| Longest silence | 15.1 s |
| Approval shown after it was asked | 0.6s |
| Bytes on the wire down / up | 38784 / 135201 |
| Bytes the app received / sent | 93807 / 6380 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 1.3 | 1077 | 1132 |
| 40 | 3.5 | 1378 | 1505 |
| 43 | 6.5 | 1378 | 1596 |
