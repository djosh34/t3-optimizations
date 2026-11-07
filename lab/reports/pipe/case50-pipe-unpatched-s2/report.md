# Pipe: case50, pipe-unpatched, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 611 | 1153 / 535 |
| Delivered | 50.9% | 46.4% |
| Longest gap | 0.95s (from 23.4s) | 1.98s (from 69.7s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.32s / 0.39s / 0.47s | 0.32s / 0.40s / 0.46s |

Bytes on the wire down / up: 147971 / 153348.

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
- +16.1s phone: state ice disconnected
- +16.1s phone: state connection disconnected
- +16.9s phone: state ice connected
- +16.9s phone: state connection connected
- +40.5s phone: state ice disconnected
- +40.5s phone: state connection disconnected
- +41.2s phone: state ice connected
- +41.2s phone: state connection connected
- +60.3s phone: state ice disconnected
- +60.3s phone: state connection disconnected
- +61.3s phone: state ice connected
- +61.3s phone: state connection connected
- +69.4s phone: state ice disconnected
- +69.4s phone: state connection disconnected
- +70.9s phone: state ice connected
- +70.9s phone: state connection connected
- +79.0s phone: state ice disconnected
- +79.0s phone: state connection disconnected
- +79.9s phone: state ice connected
- +79.9s phone: state connection connected
- +88.0s phone: state ice disconnected
- +88.0s phone: state connection disconnected
- +88.3s phone: state ice connected
- +88.3s phone: state connection connected
