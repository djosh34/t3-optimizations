# A, websocket, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 52 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 8.3s |
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.4s |
| Time to first text | 10.7s |
| Whole reply shown | 42.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5s / 5.1s / 13.5s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 0.5s |
| Bytes on the wire down / up | 3187181 / 323291 |
| Bytes the app received / sent | 204401 / 11796 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 9.5 | 397 | 783 |
| 30 | 1.9 | 1077 | 1152 |
| 40 | 4.1 | 1378 | 1524 |
| 42 | 6.1 | 1378 | 1590 |
