# rtt3000, websocket, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 59 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.6s |
| Prompt reached the agent | 3.0s |
| Time to first text | 16.0s |
| Whole reply shown | 58.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.8s / 5.4s / 15.8s |
| Longest silence | 22.2s |
| Approval shown after it was asked | 2.1s |
| Bytes on the wire down / up | 25101 / 93435 |
| Bytes the app received / sent | 86765 / 6639 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.8 | 0 | 279 |
| 20 | 6.8 | 397 | 677 |
| 30 | 16.8 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 7.2 | 1077 | 1363 |
| 58 | 7.4 | 1378 | 1596 |
