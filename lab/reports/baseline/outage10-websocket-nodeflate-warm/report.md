# outage10, websocket-nodeflate, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 23.4s |
| Whole reply shown | 43.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.4s / 6.6s / 19.4s |
| Longest silence | 23.4s |
| Approval shown after it was asked | 1.4s |
| Bytes on the wire down / up | 122379 / 52616 |
| Bytes the app received / sent | 87507 / 6247 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 19.4 | 0 | 783 |
| 30 | 0.6 | 1077 | 1109 |
| 40 | 2.9 | 1378 | 1483 |
| 43 | 5.9 | 1378 | 1587 |
