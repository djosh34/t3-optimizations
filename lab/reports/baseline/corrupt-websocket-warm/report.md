# corrupt, websocket, warm start, seed 1

Profile corrupt (fault): 10% of packets corrupted, 100 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 11.2s |
| Whole reply shown | 47.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 4.8s / 14.3s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 1.6s |
| Bytes on the wire down / up | 36182 / 134327 |
| Bytes the app received / sent | 89991 / 6240 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 373 |
| 20 | 9.3 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.5 | 1077 | 1375 |
| 47 | 6.8 | 1378 | 1596 |
