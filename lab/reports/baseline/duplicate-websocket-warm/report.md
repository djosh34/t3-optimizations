# duplicate, websocket, warm start, seed 1

Profile duplicate (fault): 20% of packets duplicated, 5% loss, 100 kbit/s. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.7s |
| Time to first text | 12.8s |
| Whole reply shown | 44.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.2s / 5.2s / 13.2s |
| Longest silence | 15.2s |
| Approval shown after it was asked | 1.2s |
| Bytes on the wire down / up | 45180 / 135831 |
| Bytes the app received / sent | 91249 / 6418 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.2 | 0 | 373 |
| 20 | 9.2 | 397 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 2.1 | 1378 | 1456 |
| 44 | 6.1 | 1378 | 1595 |
