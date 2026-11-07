# Pipe: case50, pipe, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1199 / 595 | 1154 / 539 |
| Delivered | 49.6% | 46.7% |
| Longest gap | 1.08s (from 44.0s) | 1.43s (from 76.2s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.32s / 0.40s / 0.47s | 0.32s / 0.39s / 0.46s |

Bytes on the wire down / up: 226836 / 146420.

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
- +11.2s phone: state ice disconnected
- +11.2s phone: state connection disconnected
- +12.0s phone: state ice connected
- +12.0s phone: state connection connected
- +19.5s phone: state ice disconnected
- +19.5s phone: state connection disconnected
- +20.4s phone: state ice connected
- +20.4s phone: state connection connected
- +53.5s phone: state ice disconnected
- +53.5s phone: state connection disconnected
- +54.1s phone: state ice connected
- +54.1s phone: state connection connected
- +62.6s phone: state ice disconnected
- +62.6s phone: state connection disconnected
- +62.8s phone: state ice connected
- +62.8s phone: state connection connected
- +96.1s phone: state ice disconnected
- +96.1s phone: state connection disconnected
- +96.6s phone: state ice connected
- +96.6s phone: state connection connected
