# C, websocket, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit 4054d6c96ed7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.3s |
| Time to first text | 11.5s |
| Whole reply shown | 45.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2 s / 5.2 s / 14.2 s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 1.7s |
| Bytes on the wire down / up | 35445 / 113689 |
| Bytes the app received / sent | 246706 / 19790 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.4 | 1378 | 1431 |
| 45 | 6.4 | 1378 | 1596 |
