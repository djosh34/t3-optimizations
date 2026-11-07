# Pipe: case70, pipe, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 325 | 1153 / 309 |
| Delivered | 27.1% | 26.8% |
| Longest gap | 1.70s (from 86.3s) | 4.43s (from 44.4s) |
| Gaps over 2 s | 0 | 4 |
| Tick age: median / p95 / max | 0.44s / 0.60s / 0.79s | 0.44s / 0.64s / 1.41s |

Bytes on the wire down / up: 69965 / 70360.

## Connection notes

Time after the switch.

- -3.1s phone: state ice checking
- -3.1s phone: state connection connecting
- -3.1s phone: state ice connected
- -3.1s server: ice Checking
- -3.0s phone: state connection connected
- -3.0s phone: open
- -3.0s server: ice Completed
- -3.0s server: open
- +6.6s phone: state ice disconnected
- +6.7s phone: state connection disconnected
- +8.0s phone: state ice connected
- +8.0s phone: state connection connected
- +19.5s phone: state ice disconnected
- +19.5s phone: state connection disconnected
- +20.3s phone: state ice connected
- +20.3s phone: state connection connected
- +33.9s phone: state ice disconnected
- +33.9s phone: state connection disconnected
- +35.4s phone: state ice connected
- +35.4s phone: state connection connected
- +44.4s phone: state ice disconnected
- +44.4s phone: state connection disconnected
- +48.3s phone: state ice connected
- +48.3s phone: state connection connected
- +57.8s phone: state ice disconnected
- +57.8s phone: state connection disconnected
- +59.2s phone: state ice connected
- +59.2s phone: state connection connected
- +68.2s phone: state ice disconnected
- +68.2s phone: state connection disconnected
- +69.1s phone: state ice connected
- +69.1s phone: state connection connected
- +78.2s phone: state ice disconnected
- +78.2s phone: state connection disconnected
- +79.3s phone: state ice connected
- +79.3s phone: state connection connected
- +96.1s phone: state ice disconnected
- +96.1s phone: state connection disconnected
- +97.8s phone: state ice connected
- +97.8s phone: state connection connected
- +106.8s phone: state ice disconnected
- +106.8s phone: state connection disconnected
- +110.7s phone: state ice connected
- +110.7s phone: state connection connected
- +119.8s phone: state ice disconnected
- +119.8s phone: state connection disconnected
