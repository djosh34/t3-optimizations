# loss30, websocket-nodeflate, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit 0c40f26f448c.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 69 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 8.7s |
| Time to first text | 27.7s |
| Whole reply shown | 67.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 32.2 s / 10.2 s / 28.2 s |
| Longest silence | 22.7 s |
| Approval shown after it was asked | 15.8s |
| Bytes on the wire down / up | 104679 / 57490 |
| Bytes the app received / sent | 247083 / 20315 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.2 | 0 | 93 |
| 20 | 12.2 | 0 | 493 |
| 30 | 12.2 | 397 | 897 |
| 40 | 22.2 | 397 | 1077 |
| 50 | 32.2 | 397 | 1077 |
| 60 | 7.9 | 1077 | 1383 |
| 67 | 7.1 | 1378 | 1596 |
