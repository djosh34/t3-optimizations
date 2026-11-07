# A, websocket, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.3s |
| Prompt reached the agent | 0.5s |
| Time to first text | 10.9s |
| Whole reply shown | 42.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.4s / 5.0s / 13.4s |
| Longest silence | 16.7s |
| Approval shown after it was asked | 0.4s |
| Bytes on the wire down / up | 170960 / 153021 |
| Bytes the app received / sent | 117503 / 7261 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 377 |
| 20 | 9.4 | 397 | 779 |
| 30 | 1.7 | 1077 | 1148 |
| 40 | 4.0 | 1378 | 1519 |
| 42 | 6.0 | 1378 | 1587 |
