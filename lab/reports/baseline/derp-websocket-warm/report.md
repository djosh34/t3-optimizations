# derp, websocket, warm start, seed 1

Profile derp (fault): all traffic in one TCP connection over a 10% loss link, like a DERP relay. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.9s |
| Time to first text | 11.3s |
| Whole reply shown | 43.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.0s / 5.0s / 14.0s |
| Longest silence | 17.0 s |
| Approval shown after it was asked | 0.5s |
| Bytes on the wire down / up | 62891 / 164790 |
| Bytes the app received / sent | 92334 / 5671 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 365 |
| 20 | 9.0 | 397 | 766 |
| 30 | 1.1 | 1077 | 1124 |
| 40 | 3.3 | 1378 | 1499 |
| 43 | 6.3 | 1378 | 1596 |
