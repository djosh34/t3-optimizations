# C, websocket-nodeflate, warm start, seed 2

Profile C (graded): historical EDGE-like. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.6s |
| Time to first text | 11.4s |
| Whole reply shown | 45.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.3s / 5.3s / 15.3s |
| Longest silence | 17.8s |
| Approval shown after it was asked | 1.9s |
| Bytes on the wire down / up | 115194 / 125374 |
| Bytes the app received / sent | 86444 / 6194 |
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
| 40 | 1.5 | 1378 | 1431 |
| 45 | 6.5 | 1378 | 1596 |
