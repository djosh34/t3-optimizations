# Pipe: case50, udp, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 588 | 1194 / 582 |
| Delivered | 49.0% | 48.7% |
| Longest gap | 0.96s (from 55.4s) | 1.31s (from 67.8s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.31s / 0.39s / 0.47s | 0.31s / 0.39s / 0.45s |

Bytes on the wire down / up: 39536 / 39134.

## Connection notes

Time after the switch.

- -3.0s phone: open
- -2.9s server: open 10.231.0.2:45866
