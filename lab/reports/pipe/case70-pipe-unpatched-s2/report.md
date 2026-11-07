# Pipe: case70, pipe-unpatched, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 113 | 1153 / 301 |
| Delivered | 9.4% | 26.1% |
| Longest gap | 49.17s (from 32.2s) | 4.18s (from 110.6s) |
| Gaps over 2 s | 3 | 3 |
| Tick age: median / p95 / max | 9.01s / 52.09s / 52.68s | 0.43s / 0.58s / 0.89s |

Bytes on the wire down / up: 42122 / 59316.

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
- +7.9s phone: state ice connected
- +7.9s phone: state connection connected
- +15.4s phone: state ice disconnected
- +15.4s phone: state connection disconnected
- +16.2s phone: state ice connected
- +16.2s phone: state connection connected
- +24.3s phone: state ice disconnected
- +24.3s phone: state connection disconnected
- +24.9s phone: state ice connected
- +24.9s phone: state connection connected
- +32.5s phone: state ice disconnected
- +32.5s phone: state connection disconnected
- +32.6s phone: state ice connected
- +32.6s phone: state connection connected
- +41.7s phone: state ice disconnected
- +41.7s phone: state connection disconnected
- +41.7s phone: state ice connected
- +41.7s phone: state connection connected
- +77.4s phone: state ice disconnected
- +77.4s phone: state connection disconnected
- +78.8s phone: state ice connected
- +78.8s phone: state connection connected
- +110.5s phone: state ice disconnected
- +110.5s phone: state connection disconnected
- +114.0s phone: state ice connected
- +114.0s phone: state connection connected
