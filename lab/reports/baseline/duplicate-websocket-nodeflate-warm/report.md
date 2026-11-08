# duplicate, websocket-nodeflate, warm start, seed 1

Profile duplicate (fault): 20% of packets duplicated, 5% loss, 100 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.7s |
| Prompt reached the agent | 0.6s |
| Time to first text | 11.6s |
| Whole reply shown | 46.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.3s / 14.3s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 3.0s |
| Bytes on the wire down / up | 153224 / 160887 |
| Bytes the app received / sent | 92278 / 6288 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.3 | 0 | 377 |
| 20 | 9.3 | 397 | 779 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 8.5 | 1077 | 1409 |
| 46 | 6.7 | 1378 | 1596 |
