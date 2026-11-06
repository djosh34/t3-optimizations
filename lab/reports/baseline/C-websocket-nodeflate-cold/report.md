# C, websocket-nodeflate, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit b632e37735f3.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 180 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 132.5s |
| Prompt confirmation | 5.9s |
| Time to first text | 11.7s |
| Whole reply shown | 46.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1 s / 5.4 s / 14.1 s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 2.2s |
| Bytes on the wire down / up | 3248241 / 378166 |
| Bytes the app received / sent | 343865 / 28046 |
| Reply text received / of it duplicates | 1634 / 557 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.1 | 0 | 365 |
| 20 | 9.1 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.4 | 1077 | 1402 |
| 46 | 6.6 | 1378 | 1596 |
