# Pipe: case50, pipe-unpatched, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 272 | 1153 / 551 |
| Delivered | 22.7% | 47.8% |
| Longest gap | 33.39s (from 53.5s) | 1.33s (from 99.6s) |
| Gaps over 2 s | 4 | 0 |
| Tick age: median / p95 / max | 0.34s / 33.44s / 34.83s | 0.31s / 0.39s / 0.46s |

Bytes on the wire down / up: 151633 / 111971.

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
- +7.3s phone: state ice connected
- +7.3s phone: state connection connected
- +15.3s phone: state ice disconnected
- +15.3s phone: state connection disconnected
- +16.1s phone: state ice connected
- +16.1s phone: state connection connected
- +26.6s phone: state ice disconnected
- +26.6s phone: state connection disconnected
- +26.9s phone: state ice connected
- +26.9s phone: state connection connected
- +34.9s phone: state ice disconnected
- +34.9s phone: state connection disconnected
- +35.6s phone: state ice connected
- +35.6s phone: state connection connected
- +77.2s phone: state ice disconnected
- +77.2s phone: state connection disconnected
- +78.0s phone: state ice connected
- +78.0s phone: state connection connected
- +99.5s phone: state ice disconnected
- +99.5s phone: state connection disconnected
- +100.1s phone: state ice connected
- +100.1s phone: state connection connected
- +108.4s phone: state ice disconnected
- +108.4s phone: state connection disconnected
- +109.2s phone: state ice connected
- +109.2s phone: state connection connected
