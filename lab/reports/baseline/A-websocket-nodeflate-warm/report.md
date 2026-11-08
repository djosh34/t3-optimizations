# A, websocket-nodeflate, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit e14300ea99fc.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 43 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.4s |
| Time to first text | 10.7s |
| Whole reply shown | 42.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.5s / 4.8s / 13.5s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 0.7s |
| Bytes on the wire down / up | 261891 / 127164 |
| Bytes the app received / sent | 94116 / 5107 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 9.5 | 397 | 787 |
| 30 | 1.6 | 1077 | 1144 |
| 40 | 3.8 | 1378 | 1516 |
| 42 | 5.8 | 1378 | 1583 |
