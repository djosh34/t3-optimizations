# outage10, websocket-nodeflate, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 22.7s |
| Whole reply shown | 43.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.5s / 6.5s / 18.5s |
| Longest silence | 22.7s |
| Approval shown after it was asked | 1.5s |
| Bytes on the wire down / up | 126446 / 142709 |
| Bytes the app received / sent | 86270 / 6207 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 19.5 | 0 | 787 |
| 30 | 0.7 | 1077 | 1113 |
| 40 | 3.0 | 1378 | 1487 |
| 43 | 6.0 | 1378 | 1590 |
