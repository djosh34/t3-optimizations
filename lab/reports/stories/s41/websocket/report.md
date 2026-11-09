# story, websocket, warm start, seed 41

Profile story (graded): seed 41's survival story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | 12.3s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 285.6s / 136.6s / 270.6s |
| Longest silence | 298.9s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 36411 / 261973 |
| Bytes the app received / sent | 2284 / 1778 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 7.6 | 0 | 311 |
| 30 | 17.6 | 0 | 710 |
| 40 | 27.6 | 0 | 1077 |
| 50 | 37.6 | 0 | 1077 |
| 60 | 47.6 | 0 | 1077 |
| 70 | 57.6 | 0 | 1077 |
| 80 | 67.6 | 0 | 1077 |
| 90 | 77.6 | 0 | 1077 |
| 100 | 87.6 | 0 | 1077 |
| 110 | 97.6 | 0 | 1077 |
| 120 | 107.6 | 0 | 1077 |
| 130 | 117.6 | 0 | 1077 |
| 140 | 127.6 | 0 | 1077 |
| 150 | 137.6 | 0 | 1077 |
| 160 | 147.6 | 0 | 1077 |
| 170 | 157.6 | 0 | 1077 |
| 180 | 167.6 | 0 | 1077 |
| 190 | 177.6 | 0 | 1077 |
| 200 | 187.6 | 0 | 1077 |
| 210 | 197.6 | 0 | 1077 |
| 220 | 207.6 | 0 | 1077 |
| 230 | 217.6 | 0 | 1077 |
| 240 | 227.6 | 0 | 1077 |
| 250 | 237.6 | 0 | 1077 |
| 260 | 247.6 | 0 | 1077 |
| 270 | 257.6 | 0 | 1077 |
| 280 | 267.6 | 0 | 1077 |
| 290 | 277.6 | 0 | 1077 |
| 298 | 285.6 | 0 | 1077 |
