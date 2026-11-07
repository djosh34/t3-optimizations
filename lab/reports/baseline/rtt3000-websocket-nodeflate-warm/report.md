# rtt3000, websocket-nodeflate, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 58 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.2s |
| Prompt reached the agent | 4.4s |
| Time to first text | 16.9s |
| Whole reply shown | 57.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.5s / 6.4s / 15.5s |
| Longest silence | 18.7 s |
| Approval shown after it was asked | 6.1s |
| Bytes on the wire down / up | 126777 / 100400 |
| Bytes the app received / sent | 105749 / 7652 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.5 | 0 | 222 |
| 20 | 5.5 | 397 | 621 |
| 30 | 15.5 | 397 | 1023 |
| 40 | 0.4 | 1077 | 1097 |
| 50 | 10.4 | 1077 | 1473 |
| 57 | 9.7 | 1378 | 1596 |
