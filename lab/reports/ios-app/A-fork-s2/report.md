# A, websocket-nodeflate, warm start, seed 2

Profile A (graded): weak 4G, still useful. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 43 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.3s |
| Time to first text | 10.5s |
| Whole reply shown | 42.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6s / 5.0s / 13.6s |
| Longest silence | 17.0s |
| Approval shown after it was asked | 0.5s |
| Bytes on the wire down / up | 262050 / 125559 |
| Bytes the app received / sent | 101287 / 5378 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 385 |
| 20 | 9.6 | 397 | 787 |
| 30 | 2.0 | 1077 | 1158 |
| 40 | 4.3 | 1378 | 1528 |
| 42 | 6.3 | 1378 | 1596 |
