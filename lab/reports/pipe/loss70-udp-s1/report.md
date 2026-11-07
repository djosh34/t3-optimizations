# Pipe: loss70, udp, seed 1

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 361 | 1195 / 340 |
| Delivered | 30.1% | 28.5% |
| Longest gap | 1.86s (from 14.6s) | 1.85s (from 4.6s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.30s / 0.38s / 0.43s | 0.31s / 0.39s / 0.47s |

Bytes on the wire down / up: 24257 / 22920.

## Connection notes

Time after the switch.

- -3.0s phone: open
- -2.9s server: open 10.231.0.2:52401
