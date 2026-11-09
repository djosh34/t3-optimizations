# story, websocket, warm start, seed 44

Profile story (graded): seed 44's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.9s |
| Prompt reached the agent | 1.9s |
| Time to first text | 17.8s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 397 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 286.0s / 137.0s / 271.0s |
| Longest silence | 280.6s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 17473 / 24532 |
| Bytes the app received / sent | 34800 / 3259 |
| Reply text received / of it duplicates | 397 / 0 |
| WebSockets opened after the switch | 1 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.0 | 0 | 325 |
| 20 | 8.0 | 397 | 727 |
| 30 | 18.0 | 397 | 1077 |
| 40 | 28.0 | 397 | 1077 |
| 50 | 38.0 | 397 | 1077 |
| 60 | 48.0 | 397 | 1077 |
| 70 | 58.0 | 397 | 1077 |
| 80 | 68.0 | 397 | 1077 |
| 90 | 78.0 | 397 | 1077 |
| 100 | 88.0 | 397 | 1077 |
| 110 | 98.0 | 397 | 1077 |
| 120 | 108.0 | 397 | 1077 |
| 130 | 118.0 | 397 | 1077 |
| 140 | 128.0 | 397 | 1077 |
| 150 | 138.0 | 397 | 1077 |
| 160 | 148.0 | 397 | 1077 |
| 170 | 158.0 | 397 | 1077 |
| 180 | 168.0 | 397 | 1077 |
| 190 | 178.0 | 397 | 1077 |
| 200 | 188.0 | 397 | 1077 |
| 210 | 198.0 | 397 | 1077 |
| 220 | 208.0 | 397 | 1077 |
| 230 | 218.0 | 397 | 1077 |
| 240 | 228.0 | 397 | 1077 |
| 250 | 238.0 | 397 | 1077 |
| 260 | 248.0 | 397 | 1077 |
| 270 | 258.0 | 397 | 1077 |
| 280 | 268.0 | 397 | 1077 |
| 290 | 278.0 | 397 | 1077 |
| 298 | 286.0 | 397 | 1077 |
