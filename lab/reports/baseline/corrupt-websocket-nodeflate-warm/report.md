# corrupt, websocket-nodeflate, warm start, seed 1

Profile corrupt (fault): 10% of packets corrupted, 100 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.7s |
| Prompt reached the agent | 0.7s |
| Time to first text | 13.1s |
| Whole reply shown | 47.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.2s / 5.9s / 15.2s |
| Longest silence | 16.3s |
| Approval shown after it was asked | 3.5s |
| Bytes on the wire down / up | 128036 / 150655 |
| Bytes the app received / sent | 88105 / 6198 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.9 | 1077 | 1388 |
| 47 | 7.2 | 1378 | 1596 |
