# flap, websocket-nodeflate, warm start, seed 1

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 17.8s |
| Whole reply shown | 47.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 20.5s / 9.5s / 17.5s |
| Longest silence | 17.8s |
| Approval shown after it was asked | 4.1s |
| Bytes on the wire down / up | 140859 / 138929 |
| Bytes the app received / sent | 84731 / 6586 |
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
| 40 | 7.7 | 1077 | 1378 |
| 47 | 14.7 | 1077 | 1596 |
