# A, websocket, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit 1bf7497ef685.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 54 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 10.3s |
| Prompt confirmation | 0.2s |
| Time to first text | 10.5s |
| Whole reply shown | 42.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6 s / 5.1 s / 13.6 s |
| Longest silence | 17.0 s |
| Approval shown after it was asked | 0.3s |
| Bytes on the wire down / up | 3192226 / 338777 |
| Bytes the app received / sent | 358396 / 25634 |
| Reply text received / of it duplicates | 1634 / 557 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 385 |
| 20 | 9.6 | 397 | 787 |
| 30 | 2.1 | 1077 | 1163 |
| 40 | 4.4 | 1378 | 1532 |
| 42 | 6.4 | 1378 | 1596 |
