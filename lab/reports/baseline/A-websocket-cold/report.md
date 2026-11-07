# A, websocket, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 53 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 10.0s |
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.3s |
| Time to first text | 10.5s |
| Whole reply shown | 41.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6s / 4.6s / 13.6s |
| Longest silence | 17.0 s |
| Approval shown after it was asked | 0.2s |
| Bytes on the wire down / up | 3189240 / 331692 |
| Bytes the app received / sent | 200436 / 11823 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 385 |
| 20 | 9.6 | 397 | 787 |
| 30 | 2.3 | 1077 | 1169 |
| 40 | 4.6 | 1378 | 1538 |
| 41 | 5.6 | 1378 | 1571 |
