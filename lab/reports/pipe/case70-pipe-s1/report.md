# Pipe: case70, pipe, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1199 / 364 | 1153 / 317 |
| Delivered | 30.4% | 27.5% |
| Longest gap | 1.89s (from 94.1s) | 7.60s (from 41.0s) |
| Gaps over 2 s | 0 | 7 |
| Tick age: median / p95 / max | 0.44s / 0.71s / 1.13s | 0.46s / 3.24s / 4.90s |

Bytes on the wire down / up: 81642 / 70913.

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
- +6.6s phone: state connection disconnected
- +8.3s phone: state ice connected
- +8.3s phone: state connection connected
- +21.8s phone: state ice disconnected
- +21.8s phone: state connection disconnected
- +22.9s phone: state ice connected
- +22.9s phone: state connection connected
- +31.5s phone: state ice disconnected
- +31.5s phone: state connection disconnected
- +32.4s phone: state ice connected
- +32.4s phone: state connection connected
- +41.5s phone: state ice disconnected
- +41.5s phone: state connection disconnected
- +43.3s phone: state ice connected
- +43.3s phone: state connection connected
- +67.1s phone: state ice disconnected
- +67.1s phone: state connection disconnected
- +68.2s phone: state ice connected
- +68.2s phone: state connection connected
- +95.0s phone: state ice disconnected
- +95.0s phone: state connection disconnected
- +97.1s phone: state ice connected
- +97.1s phone: state connection connected
- +106.1s phone: state ice disconnected
- +106.1s phone: state connection disconnected
- +108.5s phone: state ice connected
- +108.5s phone: state connection connected
- +118.0s phone: state ice disconnected
- +118.0s phone: state connection disconnected
- +119.4s phone: state ice connected
- +119.4s phone: state connection connected
