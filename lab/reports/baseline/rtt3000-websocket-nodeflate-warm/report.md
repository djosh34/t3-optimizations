# rtt3000, websocket-nodeflate, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 60 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 1.6s |
| Time to first text | 14.1s |
| Whole reply shown | 59.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.3s / 7.3s / 15.3s |
| Longest silence | 18.1s |
| Approval shown after it was asked | 6.6s |
| Bytes on the wire down / up | 136381 / 79519 |
| Bytes the app received / sent | 106624 / 7343 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 18.3 | 397 | 1077 |
| 40 | 3.0 | 1077 | 1200 |
| 50 | 5.3 | 1378 | 1567 |
| 59 | 14.3 | 1378 | 1596 |
