# corrupt, websocket-nodeflate, warm start, seed 1

Profile corrupt (fault): 10% of packets corrupted, 100 kbit/s. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.8s |
| Prompt reached the agent | 0.7s |
| Time to first text | 11.6s |
| Whole reply shown | 47.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 5.2s / 14.2s |
| Longest silence | 17.1 s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 135249 / 167251 |
| Bytes the app received / sent | 93402 / 7049 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 369 |
| 20 | 9.2 | 397 | 770 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.0 | 1077 | 1388 |
| 47 | 7.3 | 1378 | 1596 |
