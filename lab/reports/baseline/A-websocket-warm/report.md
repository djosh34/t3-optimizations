# A, websocket, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.4s |
| Time to first text | 10.6s |
| Whole reply shown | 42.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5s / 4.8s / 13.5s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 0.6s |
| Bytes on the wire down / up | 162829 / 118056 |
| Bytes the app received / sent | 101830 / 5067 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 9.5 | 397 | 787 |
| 30 | 1.6 | 1077 | 1144 |
| 40 | 3.8 | 1378 | 1516 |
| 42 | 5.8 | 1378 | 1583 |
