# C, websocket-nodeflate, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit ac9de0f9fa93.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 186 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 138.3s |
| Prompt confirmation | 8.6s |
| Prompt reached the agent | 0.8s |
| Time to first text | 14.2s |
| Whole reply shown | 46.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.0s / 6.0s / 14.0s |
| Longest silence | 14.4 s |
| Approval shown after it was asked | 2.3s |
| Bytes on the wire down / up | 3247450 / 408767 |
| Bytes the app received / sent | 186090 / 13244 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 365 |
| 20 | 9.0 | 397 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.3 | 1077 | 1402 |
| 46 | 6.5 | 1378 | 1596 |
