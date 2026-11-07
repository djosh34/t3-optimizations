# case70, websocket, cold start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 0c40f26f448c.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: app never loaded by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | never |
| Prompt confirmation | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | 0.0 s / 0.0 s / 0.0 s |
| Longest silence | 0.0 s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 3854 / 2435 |
| Bytes the app received / sent | 152458 / 12404 |
| Reply text received / of it duplicates | 32 / 16 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|

## Driver notes

- app did not load: stopped
