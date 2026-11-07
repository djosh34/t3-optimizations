# case50, sidecar-native, warm start, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.1s |
| Prompt reached the agent | 0.8s |
| Time to first text | 2.8s |
| Whole reply shown | 48.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.2s / 1.1s / 2.2s |
| Longest silence | 4.9s |
| Approval shown after it was asked | 4.7s |
| Bytes on the wire down / up | 176637 / 83894 |
| Bytes the app received / sent | 147327 / 36928 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.2 | 279 | 365 |
| 20 | 0.6 | 742 | 766 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.0 | 1285 | 1325 |
| 48 | 0.7 | 1579 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 46%, 0 packets it could not read
