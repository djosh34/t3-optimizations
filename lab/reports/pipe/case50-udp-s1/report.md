# Pipe: case50, udp, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 591 | 1194 / 588 |
| Delivered | 49.2% | 49.2% |
| Longest gap | 0.92s (from 12.1s) | 0.98s (from 55.6s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.30s / 0.38s / 0.43s | 0.31s / 0.39s / 0.47s |

Bytes on the wire down / up: 39667 / 39536.

## Connection notes

Time after the switch.

- -3.0s phone: open
- -2.9s server: open 10.231.0.2:51611
