# A, websocket-nodeflate, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 57 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 13.9s |
| Prompt confirmation | 0.2s |
| Prompt reached the agent | 0.3s |
| Time to first text | 10.6s |
| Whole reply shown | 42.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.6s / 4.6s / 13.6s |
| Longest silence | 16.8 s |
| Approval shown after it was asked | 0.4s |
| Bytes on the wire down / up | 3356853 / 347695 |
| Bytes the app received / sent | 200776 / 12276 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.6 | 0 | 385 |
| 20 | 9.6 | 397 | 787 |
| 30 | 2.1 | 1077 | 1163 |
| 40 | 4.4 | 1378 | 1532 |
| 41 | 5.4 | 1378 | 1567 |
