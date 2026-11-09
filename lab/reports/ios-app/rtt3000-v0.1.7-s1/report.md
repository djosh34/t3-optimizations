# rtt3000, websocket-nodeflate, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 56 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.1s |
| Prompt reached the agent | 1.6s |
| Time to first text | 15.3s |
| Whole reply shown | 55.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.3s / 7.8s / 15.3s |
| Longest silence | 20.9s |
| Approval shown after it was asked | 5.5s |
| Bytes on the wire down / up | 115873 / 54802 |
| Bytes the app received / sent | 100960 / 6836 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 8.3 | 397 | 738 |
| 30 | 18.3 | 397 | 1077 |
| 40 | 3.8 | 1077 | 1233 |
| 50 | 13.8 | 1077 | 1595 |
| 55 | 11.1 | 1378 | 1596 |
