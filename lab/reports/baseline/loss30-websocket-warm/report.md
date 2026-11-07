# loss30, websocket, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit 0c40f26f448c.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 59 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.3s |
| Time to first text | 12.1s |
| Whole reply shown | 58.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.4 s / 3.6 s / 13.3 s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 10.2s |
| Bytes on the wire down / up | 18044 / 40228 |
| Bytes the app received / sent | 247446 / 19179 |
| Reply text received / of it duplicates | 1602 / 525 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 7.4 | 1077 | 1371 |
| 58 | 7.6 | 1378 | 1596 |
