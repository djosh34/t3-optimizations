# Pipe: loss70, pipe-unpatched, seed 2

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 197 | 1153 / 281 |
| Delivered | 16.4% | 24.4% |
| Longest gap | 46.69s (from 73.3s) | 3.21s (from 52.6s) |
| Gaps over 2 s | 6 | 7 |
| Tick age: median / p95 / max | 0.41s / 25.01s / 26.02s | 0.31s / 0.44s / 1.27s |

Bytes on the wire down / up: 43238 / 63229.

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
- +7.5s phone: state ice connected
- +7.5s phone: state connection connected
- +15.0s phone: state ice disconnected
- +15.0s phone: state connection disconnected
- +16.2s phone: state ice connected
- +16.2s phone: state connection connected
- +24.0s phone: state ice disconnected
- +24.0s phone: state connection disconnected
- +25.5s phone: state ice connected
- +25.5s phone: state connection connected
- +33.0s phone: state ice disconnected
- +33.0s phone: state connection disconnected
- +35.4s phone: state ice connected
- +35.4s phone: state connection connected
- +42.9s phone: state ice disconnected
- +42.9s phone: state connection disconnected
- +44.3s phone: state ice connected
- +44.3s phone: state connection connected
- +52.3s phone: state ice disconnected
- +52.3s phone: state connection disconnected
- +55.0s phone: state ice connected
- +55.0s phone: state connection connected
- +77.8s phone: state ice disconnected
- +77.8s phone: state connection disconnected
- +78.1s phone: state ice connected
- +78.1s phone: state connection connected
- +92.1s phone: state ice disconnected
- +92.1s phone: state connection disconnected
- +94.1s phone: state ice connected
- +94.1s phone: state connection connected
- +106.7s phone: state ice disconnected
- +106.7s phone: state connection disconnected
- +108.4s phone: state ice connected
- +108.4s phone: state connection connected
- +116.9s phone: state ice disconnected
- +116.9s phone: state connection disconnected
- +117.6s phone: state ice connected
- +117.6s phone: state connection connected
