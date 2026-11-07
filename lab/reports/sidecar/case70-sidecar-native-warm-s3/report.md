# case70, sidecar-native, warm start, seed 3

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 77 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.2s |
| Prompt reached the agent | 1.5s |
| Time to first text | 11.3s |
| Whole reply shown | 76.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 13.8s / 4.3s / 10.8s |
| Longest silence | 25.3s |
| Approval shown after it was asked | 26.9s |
| Bytes on the wire down / up | 52974 / 28191 |
| Bytes the app received / sent | 44452 / 54399 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 8.3 | 0 | 336 |
| 20 | 6.5 | 478 | 738 |
| 30 | 8.8 | 787 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 1.9 | 1077 | 1155 |
| 70 | 7.5 | 1250 | 1524 |
| 76 | 7.0 | 1492 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 64%, 0 packets it could not read
