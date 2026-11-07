# Pipe: loss70, pipe, seed 2

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 341 | 1154 / 299 |
| Delivered | 28.4% | 25.9% |
| Longest gap | 2.00s (from 104.3s) | 2.78s (from 19.8s) |
| Gaps over 2 s | 0 | 3 |
| Tick age: median / p95 / max | 0.32s / 0.39s / 0.47s | 0.32s / 0.56s / 1.65s |

Bytes on the wire down / up: 73295 / 69422.

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
- +7.4s phone: state ice connected
- +7.4s phone: state connection connected
- +19.4s phone: state ice disconnected
- +19.4s phone: state connection disconnected
- +22.1s phone: state ice connected
- +22.1s phone: state connection connected
- +35.1s phone: state ice disconnected
- +35.1s phone: state connection disconnected
- +36.0s phone: state ice connected
- +36.0s phone: state connection connected
- +47.8s phone: state ice disconnected
- +47.8s phone: state connection disconnected
- +49.0s phone: state ice connected
- +49.0s phone: state connection connected
- +57.0s phone: state ice disconnected
- +57.0s phone: state connection disconnected
- +57.8s phone: state ice connected
- +57.8s phone: state connection connected
- +72.1s phone: state ice disconnected
- +72.1s phone: state connection disconnected
- +73.4s phone: state ice connected
- +73.4s phone: state connection connected
- +105.5s phone: state ice disconnected
- +105.5s phone: state connection disconnected
- +106.2s phone: state ice connected
- +106.2s phone: state connection connected
- +114.3s phone: state ice disconnected
- +114.3s phone: state connection disconnected
- +115.4s phone: state ice connected
- +115.4s phone: state connection connected
