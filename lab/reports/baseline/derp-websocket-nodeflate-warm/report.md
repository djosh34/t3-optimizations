# derp, websocket-nodeflate, warm start, seed 1

Profile derp (fault): all traffic in one TCP connection over a 10% loss link, like a DERP relay. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.0s |
| Prompt reached the agent | 1.2s |
| Time to first text | 12.0s |
| Whole reply shown | 46.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.7s / 4.9s / 14.7s |
| Longest silence | 17.7s |
| Approval shown after it was asked | 3.5s |
| Bytes on the wire down / up | 161780 / 199897 |
| Bytes the app received / sent | 94142 / 6961 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.7 | 0 | 352 |
| 20 | 8.7 | 397 | 754 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 7.9 | 1077 | 1383 |
| 46 | 6.1 | 1378 | 1595 |
