# reorder, websocket-nodeflate, warm start, seed 1

Profile reorder (fault): 25% of packets reordered, 10% loss, 100 kbit/s. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 56 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.0s |
| Prompt reached the agent | 2.9s |
| Time to first text | 14.6s |
| Whole reply shown | 55.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.0s / 6.0s / 14.0s |
| Longest silence | 16.4s |
| Approval shown after it was asked | 2.8s |
| Bytes on the wire down / up | 129532 / 147475 |
| Bytes the app received / sent | 93305 / 8032 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.0 | 0 | 283 |
| 20 | 7.0 | 397 | 681 |
| 30 | 17.0 | 397 | 1077 |
| 40 | 6.7 | 1077 | 1344 |
| 50 | 9.0 | 1378 | 1596 |
| 55 | 14.0 | 1378 | 1596 |
