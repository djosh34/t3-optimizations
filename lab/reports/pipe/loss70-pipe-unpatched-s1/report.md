# Pipe: loss70, pipe-unpatched, seed 1

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1199 / 305 | 1153 / 307 |
| Delivered | 25.4% | 26.6% |
| Longest gap | 16.25s (from 103.7s) | 3.03s (from 15.3s) |
| Gaps over 2 s | 5 | 3 |
| Tick age: median / p95 / max | 0.32s / 4.04s / 5.46s | 0.31s / 0.41s / 1.18s |

Bytes on the wire down / up: 70189 / 64588.

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
- +6.8s phone: state ice disconnected
- +6.8s phone: state connection disconnected
- +7.6s phone: state ice connected
- +7.6s phone: state connection connected
- +15.2s phone: state ice disconnected
- +15.2s phone: state connection disconnected
- +18.0s phone: state ice connected
- +18.0s phone: state connection connected
- +31.6s phone: state ice disconnected
- +31.6s phone: state connection disconnected
- +32.5s phone: state ice connected
- +32.5s phone: state connection connected
- +46.8s phone: state ice disconnected
- +46.8s phone: state connection disconnected
- +47.7s phone: state ice connected
- +47.7s phone: state connection connected
- +62.0s phone: state ice disconnected
- +62.0s phone: state connection disconnected
- +62.0s phone: state ice connected
- +62.0s phone: state connection connected
- +74.3s phone: state ice disconnected
- +74.3s phone: state connection disconnected
- +75.6s phone: state ice connected
- +75.6s phone: state connection connected
- +84.1s phone: state ice disconnected
- +84.1s phone: state connection disconnected
- +85.2s phone: state ice connected
- +85.2s phone: state connection connected
- +93.7s phone: state ice disconnected
- +93.7s phone: state connection disconnected
- +94.2s phone: state ice connected
- +94.2s phone: state connection connected
- +102.2s phone: state ice disconnected
- +102.2s phone: state connection disconnected
- +103.5s phone: state ice connected
- +103.5s phone: state connection connected
- +119.7s phone: state ice disconnected
- +119.7s phone: state connection disconnected
