# C, websocket, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.9s |
| Prompt reached the agent | 2.5s |
| Time to first text | 13.1s |
| Whole reply shown | 46.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.4s / 5.4s / 14.4s |
| Longest silence | 16.9 s |
| Approval shown after it was asked | 0.7s |
| Bytes on the wire down / up | 36465 / 132335 |
| Bytes the app received / sent | 91369 / 7035 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 303 |
| 20 | 7.4 | 397 | 701 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 0.7 | 1378 | 1409 |
| 46 | 6.7 | 1378 | 1596 |
