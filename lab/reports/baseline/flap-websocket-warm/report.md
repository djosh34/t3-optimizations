# flap, websocket, warm start, seed 1

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit cc6b9676f812.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 18.2s |
| Whole reply shown | 47.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.5s / 9.5s / 17.5s |
| Longest silence | 18.2s |
| Approval shown after it was asked | 3.5s |
| Bytes on the wire down / up | 42740 / 127153 |
| Bytes the app received / sent | 104023 / 6940 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.5 | 0 | 385 |
| 20 | 9.5 | 397 | 787 |
| 30 | 19.5 | 397 | 1077 |
| 40 | 8.3 | 1077 | 1402 |
| 46 | 6.5 | 1378 | 1596 |
