# C, websocket-nodeflate, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.6s |
| Time to first text | 11.4s |
| Whole reply shown | 45.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.3s / 14.3s |
| Longest silence | 17.0s |
| Approval shown after it was asked | 2.1s |
| Bytes on the wire down / up | 122679 / 134370 |
| Bytes the app received / sent | 87066 / 6223 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 373 |
| 20 | 9.3 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.7 | 1378 | 1440 |
| 45 | 6.7 | 1378 | 1596 |
