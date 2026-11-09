# story, websocket, warm start, seed 40

Profile story (graded): seed 40's 2G story. Commit 9c461bdca554.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ.

**Status: done** after 229 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.3s |
| Prompt reached the agent | 0.9s |
| Time to first text | 14.1s |
| Whole reply shown | 227.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 20.0s / 0.0s / 11.0s |
| Longest silence | 189.4s |
| Approval shown after it was asked | 13.2s |
| Bytes on the wire down / up | 87446 / 493648 |
| Bytes the app received / sent | 156917 / 12894 |
| Reply text received / of it duplicates | 2294 / 698 |
| WebSockets opened after the switch | 2 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 361 |
| 20 | 9.0 | 397 | 762 |
| 30 | 19.0 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 0.0 | 1077 | 1077 |
| 90 | 0.0 | 1077 | 1077 |
| 100 | 0.0 | 1077 | 1077 |
| 110 | 0.0 | 1077 | 1077 |
| 120 | 0.0 | 1077 | 1077 |
| 130 | 0.0 | 1077 | 1077 |
| 140 | 0.0 | 1077 | 1077 |
| 150 | 0.0 | 1077 | 1077 |
| 160 | 0.0 | 1077 | 1077 |
| 170 | 0.0 | 1077 | 1077 |
| 180 | 0.0 | 1077 | 1077 |
| 190 | 0.0 | 1077 | 1077 |
| 200 | 0.0 | 1077 | 1077 |
| 210 | 0.0 | 1077 | 1077 |
| 220 | 7.3 | 1077 | 1363 |
| 227 | 6.5 | 1378 | 1596 |

## Driver notes

- sample failed: locator.click: Timeout 2000ms exceeded.
- Scroll to end shows
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- Scroll to end shows
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- Scroll to end shows
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
- sample failed: locator.click: Timeout 2000ms exceeded.
