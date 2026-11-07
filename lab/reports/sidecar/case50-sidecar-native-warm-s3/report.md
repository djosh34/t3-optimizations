# case50, sidecar-native, warm start, seed 3

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 50 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.3s |
| Prompt reached the agent | 1.0s |
| Time to first text | 3.2s |
| Whole reply shown | 50.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.1s / 0.8s / 1.9s |
| Longest silence | 6.2s |
| Approval shown after it was asked | 5.4s |
| Bytes on the wire down / up | 209242 / 99823 |
| Bytes the app received / sent | 179628 / 37787 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.1 | 315 | 357 |
| 20 | 1.4 | 701 | 758 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.3 | 1220 | 1274 |
| 50 | 1.2 | 1587 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 51%, 0 packets it could not read
