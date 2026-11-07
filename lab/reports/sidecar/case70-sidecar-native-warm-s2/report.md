# case70, sidecar-native, warm start, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 67 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.5s |
| Prompt reached the agent | 1.3s |
| Time to first text | 4.4s |
| Whole reply shown | 67.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 9.9s / 2.2s / 6.9s |
| Longest silence | 18.3s |
| Approval shown after it was asked | 17.8s |
| Bytes on the wire down / up | 56481 / 28840 |
| Bytes the app received / sent | 45584 / 52182 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.9 | 69 | 344 |
| 20 | 3.8 | 593 | 746 |
| 30 | 2.9 | 1032 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.2 | 1077 | 1090 |
| 60 | 2.1 | 1393 | 1469 |
| 67 | 3.3 | 1595 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 69%, 0 packets it could not read
