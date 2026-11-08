# case50, websocket, cold start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit e14300ea99fc.

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
| Longest silence | n/a, no prompt sent |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 34720 / 2566 |
| Bytes the app received / sent | 0 / 0 |
| Reply text received / of it duplicates | 0 / 0 |
| WebSockets opened after the switch | 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|

## Driver notes

- app did not load: stopped
- screenshot end failed: page.screenshot: Timeout 15000ms exceeded.
Call log:
  - taking page screenshot
  - waiting for fonts to load...

