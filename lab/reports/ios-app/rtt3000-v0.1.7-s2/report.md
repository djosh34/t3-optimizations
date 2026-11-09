# rtt3000, websocket-nodeflate, warm start, seed 2

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 16112c251278-dirty.

t3 server: v0.1.7.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 59 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.3s |
| Prompt reached the agent | 2.1s |
| Time to first text | 15.6s |
| Whole reply shown | 58.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.8s / 6.2s / 15.8s |
| Longest silence | 17.3s |
| Approval shown after it was asked | 5.9s |
| Bytes on the wire down / up | 120223 / 59535 |
| Bytes the app received / sent | 104193 / 7222 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.8 | 0 | 315 |
| 20 | 7.8 | 397 | 714 |
| 30 | 17.8 | 397 | 1077 |
| 40 | 1.2 | 1077 | 1128 |
| 50 | 3.5 | 1378 | 1500 |
| 58 | 11.5 | 1378 | 1596 |
