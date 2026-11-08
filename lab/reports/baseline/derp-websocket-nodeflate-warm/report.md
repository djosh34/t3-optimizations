# derp, websocket-nodeflate, warm start, seed 1

Profile derp (fault): all traffic in one TCP connection over a 10% loss link, like a DERP relay. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.1s |
| Prompt reached the agent | 1.9s |
| Time to first text | 13.5s |
| Whole reply shown | 48.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.0s / 5.6s / 15.0s |
| Longest silence | 16.5s |
| Approval shown after it was asked | 2.9s |
| Bytes on the wire down / up | 162581 / 172103 |
| Bytes the app received / sent | 91765 / 6258 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.0 | 0 | 325 |
| 20 | 8.0 | 397 | 727 |
| 30 | 18.0 | 397 | 1077 |
| 40 | 7.6 | 1077 | 1378 |
| 48 | 7.8 | 1378 | 1596 |
