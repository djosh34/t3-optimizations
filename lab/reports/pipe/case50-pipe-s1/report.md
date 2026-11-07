# Pipe: case50, pipe, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 596 | 1153 / 547 |
| Delivered | 49.7% | 47.4% |
| Longest gap | 1.14s (from 32.4s) | 1.64s (from 107.3s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.32s / 0.40s / 0.46s | 0.32s / 0.39s / 0.44s |

Bytes on the wire down / up: 200992 / 154137.

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
- +54.7s phone: state ice disconnected
- +54.7s phone: state connection disconnected
- +55.1s phone: state ice connected
- +55.1s phone: state connection connected
- +63.7s phone: state ice disconnected
- +63.7s phone: state connection disconnected
- +63.8s phone: state ice connected
- +63.8s phone: state connection connected
- +107.1s phone: state ice disconnected
- +107.1s phone: state connection disconnected
- +107.9s phone: state ice connected
- +107.9s phone: state connection connected
