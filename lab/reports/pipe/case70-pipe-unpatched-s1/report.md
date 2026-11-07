# Pipe: case70, pipe-unpatched, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 136 | 1153 / 318 |
| Delivered | 11.3% | 27.6% |
| Longest gap | 35.63s (from 84.4s) | 3.01s (from 98.8s) |
| Gaps over 2 s | 5 | 2 |
| Tick age: median / p95 / max | 22.94s / 40.63s / 41.34s | 0.43s / 0.55s / 1.71s |

Bytes on the wire down / up: 44804 / 58243.

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
- +7.6s phone: state ice connected
- +7.6s phone: state connection connected
- +20.4s phone: state ice disconnected
- +20.4s phone: state connection disconnected
- +21.4s phone: state ice connected
- +21.4s phone: state connection connected
- +30.5s phone: state ice disconnected
- +30.5s phone: state connection disconnected
- +31.7s phone: state ice connected
- +31.7s phone: state connection connected
- +49.0s phone: state ice disconnected
- +49.0s phone: state connection disconnected
- +49.5s phone: state ice connected
- +49.5s phone: state connection connected
- +59.1s phone: state ice disconnected
- +59.1s phone: state connection disconnected
- +60.2s phone: state ice connected
- +60.2s phone: state connection connected
- +76.5s phone: state ice disconnected
- +76.5s phone: state connection disconnected
- +76.5s phone: state ice connected
- +76.5s phone: state connection connected
- +98.5s phone: state ice disconnected
- +98.5s phone: state connection disconnected
- +101.2s phone: state ice connected
- +101.2s phone: state connection connected
- +110.2s phone: state ice disconnected
- +110.2s phone: state connection disconnected
- +110.2s phone: state ice connected
- +110.2s phone: state connection connected
