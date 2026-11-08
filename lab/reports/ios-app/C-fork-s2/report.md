# C, websocket-nodeflate, warm start, seed 2

Profile C (graded): historical EDGE-like. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.7s |
| Time to first text | 11.3s |
| Whole reply shown | 45.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.2s / 5.2s / 14.2s |
| Longest silence | 17.2s |
| Approval shown after it was asked | 1.8s |
| Bytes on the wire down / up | 122390 / 127481 |
| Bytes the app received / sent | 92369 / 6303 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.3 | 1378 | 1429 |
| 45 | 6.3 | 1378 | 1596 |
