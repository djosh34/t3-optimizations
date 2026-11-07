# duplicate, websocket-nodeflate, warm start, seed 1

Profile duplicate (fault): 20% of packets duplicated, 5% loss, 100 kbit/s. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.7s |
| Time to first text | 11.5s |
| Whole reply shown | 48.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.2s / 6.2s / 18.2s |
| Longest silence | 20.8s |
| Approval shown after it was asked | 4.7s |
| Bytes on the wire down / up | 167042 / 200015 |
| Bytes the app received / sent | 97625 / 7658 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 19.2 | 397 | 1077 |
| 40 | 7.0 | 1077 | 1355 |
| 48 | 7.2 | 1378 | 1596 |
