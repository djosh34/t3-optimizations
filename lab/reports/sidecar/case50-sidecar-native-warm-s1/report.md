# case50, sidecar-native, warm start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 225ea00fa9b4.

WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale.

**Status: done** after 50 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.2s |
| Prompt reached the agent | 0.8s |
| Time to first text | 3.5s |
| Whole reply shown | 50.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 5.3s / 1.1s / 2.7s |
| Longest silence | 7.3s |
| Approval shown after it was asked | 6.3s |
| Bytes on the wire down / up | 130486 / 63452 |
| Bytes the app received / sent | 105374 / 37157 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 307 | 369 |
| 20 | 1.5 | 705 | 770 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.9 | 1196 | 1237 |
| 50 | 1.7 | 1537 | 1596 |

## Driver notes

- native phone: 0 ICE restarts, 0 new connections, loss up 63%, 0 packets it could not read
