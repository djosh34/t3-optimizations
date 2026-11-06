# A, websocket-nodeflate, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit 1bf7497ef685.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.2s |
| Time to first text | 10.7s |
| Whole reply shown | 42.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.4 s / 5.0 s / 13.4 s |
| Longest silence | 16.9 s |
| Approval shown after it was asked | 0.4s |
| Bytes on the wire down / up | 267336 / 146653 |
| Bytes the app received / sent | 261769 / 18726 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 377 |
| 20 | 9.4 | 397 | 779 |
| 30 | 1.7 | 1077 | 1148 |
| 40 | 4.0 | 1378 | 1519 |
| 42 | 6.0 | 1378 | 1587 |
