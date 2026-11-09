# outage10, websocket-nodeflate, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 7978c8b2e106-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 22.4s |
| Whole reply shown | 44.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.5s / 6.4s / 18.5s |
| Longest silence | 22.4s |
| Approval shown after it was asked | 1.9s |
| Bytes on the wire down / up | 127793 / 146838 |
| Bytes the app received / sent | 87077 / 6201 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 19.5 | 0 | 787 |
| 30 | 0.4 | 1077 | 1101 |
| 40 | 2.7 | 1378 | 1477 |
| 43 | 5.7 | 1378 | 1579 |
