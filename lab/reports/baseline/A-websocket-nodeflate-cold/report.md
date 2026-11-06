# A, websocket-nodeflate, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit 1bf7497ef685-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 50 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 6.9s |
| Prompt confirmation | 0.2s |
| Time to first text | 10.7s |
| Whole reply shown | 41.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6 s / 4.6 s / 13.6 s |
| Longest silence | 16.6 s |
| Approval shown after it was asked | 0.3s |
| Bytes on the wire down / up | 3379927 / 336470 |
| Bytes the app received / sent | 347742 / 23979 |
| Reply text received / of it duplicates | 1634 / 557 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 385 |
| 20 | 9.6 | 397 | 787 |
| 30 | 2.2 | 1077 | 1163 |
| 40 | 4.4 | 1378 | 1532 |
| 41 | 5.4 | 1378 | 1567 |
