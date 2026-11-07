# E, websocket-nodeflate, cold start, seed 1

Profile E (graded): owner extreme: 1 kbit/s, 75% bursty loss. Commit 3e5d09c36a45.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: app never loaded by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | never |
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 0.0 s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 7167 / 27189 |
| Bytes the app received / sent | 0 / 0 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|

## Driver notes

- app did not load: stopped
