# flap, websocket-nodeflate, warm start, seed 2

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 53 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 15.9s |
| Whole reply shown | 52.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 23.5s / 9.1s / 20.5s |
| Longest silence | 18.6s |
| Approval shown after it was asked | 7.5s |
| Bytes on the wire down / up | 150527 / 177164 |
| Bytes the app received / sent | 103952 / 6926 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 9.5 | 397 | 783 |
| 30 | 19.5 | 397 | 1077 |
| 40 | 4.8 | 1077 | 1270 |
| 50 | 7.1 | 1378 | 1596 |
| 52 | 9.1 | 1378 | 1596 |
