# derp, websocket, warm start, seed 1

Profile derp (fault): all traffic in one TCP connection over a 10% loss link, like a DERP relay. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.8s |
| Prompt reached the agent | 0.4s |
| Time to first text | 10.9s |
| Whole reply shown | 44.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5s / 5.2s / 13.5s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 1.8s |
| Bytes on the wire down / up | 54587 / 139637 |
| Bytes the app received / sent | 96842 / 6261 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 9.5 | 397 | 783 |
| 30 | 0.2 | 1077 | 1090 |
| 40 | 2.5 | 1378 | 1469 |
| 44 | 6.5 | 1378 | 1596 |
