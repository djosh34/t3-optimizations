# outage10, websocket, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit 95709024129b.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.4s |
| Time to first text | 30.9s |
| Whole reply shown | 45.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 29.5s / 7.3s / 26.5s |
| Longest silence | 30.9 s |
| Approval shown after it was asked | 3.7s |
| Bytes on the wire down / up | 63681 / 187635 |
| Bytes the app received / sent | 129961 / 10271 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 381 |
| 20 | 19.5 | 0 | 783 |
| 30 | 29.5 | 0 | 1077 |
| 40 | 0.6 | 1378 | 1402 |
| 45 | 5.6 | 1378 | 1571 |
