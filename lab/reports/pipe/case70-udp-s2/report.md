# Pipe: case70, udp, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 344 | 1193 / 352 |
| Delivered | 28.7% | 29.5% |
| Longest gap | 1.83s (from 4.7s) | 1.63s (from 44.3s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.36s / 0.44s / 0.52s | 0.35s / 0.43s / 0.48s |

Bytes on the wire down / up: 23048 / 23584.

## Connection notes

Time after the switch.

- -3.0s phone: open
- -2.9s server: open 10.231.0.2:42346
