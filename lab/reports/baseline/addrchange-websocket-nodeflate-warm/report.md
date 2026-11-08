# addrchange, websocket-nodeflate, warm start, seed 1

Profile addrchange (fault): the phone's address changes once, on a weak link. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 210 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.5s |
| Prompt reached the agent | 0.5s |
| Time to first text | 11.2s |
| Whole reply shown | 209.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 182.4s / 77.4s / 171.4s |
| Longest silence | 182.3s |
| Approval shown after it was asked | 166.9s |
| Bytes on the wire down / up | 275989 / 368456 |
| Bytes the app received / sent | 147244 / 11404 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.4 | 0 | 381 |
| 20 | 9.4 | 397 | 783 |
| 30 | 19.4 | 397 | 1077 |
| 40 | 29.4 | 397 | 1077 |
| 50 | 39.4 | 397 | 1077 |
| 60 | 49.4 | 397 | 1077 |
| 70 | 59.4 | 397 | 1077 |
| 80 | 69.4 | 397 | 1077 |
| 90 | 79.4 | 397 | 1077 |
| 100 | 89.4 | 397 | 1077 |
| 110 | 99.4 | 397 | 1077 |
| 120 | 109.4 | 397 | 1077 |
| 130 | 119.4 | 397 | 1077 |
| 140 | 129.4 | 397 | 1077 |
| 150 | 139.4 | 397 | 1077 |
| 160 | 149.4 | 397 | 1077 |
| 170 | 159.4 | 397 | 1077 |
| 180 | 169.4 | 397 | 1077 |
| 190 | 179.4 | 397 | 1077 |
| 200 | 5.3 | 1077 | 1288 |
| 209 | 6.5 | 1378 | 1596 |
