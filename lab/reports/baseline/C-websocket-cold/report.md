# C, websocket, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit 4054d6c96ed7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 182 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 136.1s |
| Prompt confirmation | 1.1s |
| Time to first text | 14.1s |
| Whole reply shown | 44.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1 s / 6.0 s / 14.1 s |
| Longest silence | 14.4 s |
| Approval shown after it was asked | 1.8s |
| Bytes on the wire down / up | 3078514 / 405169 |
| Bytes the app received / sent | 335646 / 26521 |
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
| 40 | 2.0 | 1378 | 1450 |
| 44 | 6.0 | 1378 | 1587 |
