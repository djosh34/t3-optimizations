# flap, websocket, warm start, seed 1

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 14.2s |
| Whole reply shown | 45.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.4s / 7.4s / 15.4s |
| Longest silence | 15.4s |
| Approval shown after it was asked | 3.6s |
| Bytes on the wire down / up | 39655 / 98474 |
| Bytes the app received / sent | 108624 / 6057 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 9.4 | 397 | 783 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.3 | 1077 | 1406 |
| 45 | 5.6 | 1378 | 1575 |
