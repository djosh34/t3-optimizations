# rtt3000, websocket-nodeflate, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 16112c251278-dirty.

t3 server: fork.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 54 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.1s |
| Prompt reached the agent | 1.6s |
| Time to first text | 13.5s |
| Whole reply shown | 52.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.3s / 5.3s / 15.3s |
| Longest silence | 17.0s |
| Approval shown after it was asked | 2.3s |
| Bytes on the wire down / up | 110002 / 61747 |
| Bytes the app received / sent | 87937 / 6598 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 333 |
| 20 | 8.3 | 397 | 734 |
| 30 | 18.3 | 397 | 1077 |
| 40 | 3.4 | 1077 | 1208 |
| 50 | 5.6 | 1378 | 1571 |
| 52 | 7.6 | 1378 | 1596 |
