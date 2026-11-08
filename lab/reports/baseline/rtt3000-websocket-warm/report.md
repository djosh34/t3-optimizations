# rtt3000, websocket, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit 0f01ba755fce-dirty.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 70 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.1s |
| Prompt reached the agent | 1.6s |
| Time to first text | 15.0s |
| Whole reply shown | 69.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 32.0s / 10.3s / 28.0s |
| Longest silence | 36.1s |
| Approval shown after it was asked | 4.8s |
| Bytes on the wire down / up | 67615 / 79855 |
| Bytes the app received / sent | 110938 / 8141 |
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
| 40 | 3.0 | 1077 | 1196 |
| 50 | 13.0 | 1077 | 1564 |
| 60 | 23.0 | 1077 | 1596 |
| 69 | 32.0 | 1077 | 1596 |

## Driver notes

- sample failed: locator.isEnabled: Timeout 30000ms exceeded.
