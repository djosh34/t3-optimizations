# flap, websocket-nodeflate, warm start, seed 1

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 50 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 17.5s |
| Whole reply shown | 49.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.4s / 8.4s / 18.4s |
| Longest silence | 17.5s |
| Approval shown after it was asked | 6.6s |
| Bytes on the wire down / up | 174199 / 148963 |
| Bytes the app received / sent | 111415 / 6727 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 9.4 | 397 | 783 |
| 30 | 19.4 | 397 | 1077 |
| 40 | 5.5 | 1077 | 1297 |
| 49 | 6.7 | 1378 | 1596 |
