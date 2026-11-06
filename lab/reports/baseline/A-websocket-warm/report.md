# A, websocket, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit 1bf7497ef685.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 43 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.3s |
| Time to first text | 10.7s |
| Whole reply shown | 42.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5 s / 5.0 s / 13.5 s |
| Longest silence | 16.7 s |
| Approval shown after it was asked | 0.3s |
| Bytes on the wire down / up | 166786 / 140758 |
| Bytes the app received / sent | 243103 / 19918 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 9.5 | 397 | 787 |
| 30 | 2.0 | 1077 | 1163 |
| 40 | 4.3 | 1378 | 1532 |
| 42 | 6.3 | 1378 | 1596 |
