# case70, sidecar-native, warm start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 68 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.2s |
| Prompt reached the agent | 1.5s |
| Time to first text | 5.4s |
| Whole reply shown | 68.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 12.3s / 2.2s / 9.3s |
| Longest silence | 21.6s |
| Approval shown after it was asked | 15.2s |
| Bytes on the wire down / up | 55163 / 28859 |
| Bytes the app received / sent | 46386 / 54106 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.3 | 165 | 341 |
| 20 | 0.7 | 710 | 742 |
| 30 | 2.7 | 1032 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 1.2 | 1077 | 1128 |
| 60 | 8.3 | 1189 | 1500 |
| 68 | 6.5 | 1551 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 71%, 0 packets it could not read
