# outage10, websocket-nodeflate, warm start, seed 2

Profile outage10 (fault): one 10 s outage on a weak link. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.6s |
| Time to first text | 22.5s |
| Whole reply shown | 43.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.3s / 6.3s / 18.3s |
| Longest silence | 22.5s |
| Approval shown after it was asked | 1.3s |
| Bytes on the wire down / up | 117224 / 54586 |
| Bytes the app received / sent | 89356 / 6193 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 377 |
| 20 | 19.3 | 0 | 779 |
| 30 | 0.8 | 1077 | 1113 |
| 40 | 3.0 | 1378 | 1487 |
| 43 | 6.0 | 1378 | 1590 |
