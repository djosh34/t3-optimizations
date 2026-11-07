# Pipe: loss70, pipe, seed 1

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit 663e401402c1.

A 25-byte tick every 100 ms each way for 120 s after the switch to the profile.

| Measure | Down (server to phone) | Up (phone to server) |
|---|---|---|
| Ticks sent / received | 1200 / 368 | 1153 / 290 |
| Delivered | 30.7% | 25.2% |
| Longest gap | 1.45s (from 10.3s) | 1.87s (from 32.1s) |
| Gaps over 2 s | 0 | 0 |
| Tick age: median / p95 / max | 0.31s / 0.39s / 0.43s | 0.31s / 0.39s / 0.51s |

Bytes on the wire down / up: 83328 / 72225.

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
- +20.5s phone: state ice disconnected
- +20.5s phone: state connection disconnected
- +21.8s phone: state ice connected
- +21.8s phone: state connection connected
- +31.9s phone: state ice disconnected
- +31.9s phone: state connection disconnected
- +33.1s phone: state ice connected
- +33.1s phone: state connection connected
- +40.6s phone: state ice disconnected
- +40.6s phone: state connection disconnected
- +41.7s phone: state ice connected
- +41.7s phone: state connection connected
- +49.3s phone: state ice disconnected
- +49.3s phone: state connection disconnected
- +50.2s phone: state ice connected
- +50.2s phone: state connection connected
- +58.2s phone: state ice disconnected
- +58.2s phone: state connection disconnected
- +59.4s phone: state ice connected
- +59.4s phone: state connection connected
- +67.5s phone: state ice disconnected
- +67.5s phone: state connection disconnected
- +67.8s phone: state ice connected
- +67.8s phone: state connection connected
- +75.9s phone: state ice disconnected
- +75.9s phone: state connection disconnected
- +76.5s phone: state ice connected
- +76.5s phone: state connection connected
- +92.8s phone: state ice disconnected
- +92.8s phone: state connection disconnected
- +93.3s phone: state ice connected
- +93.3s phone: state connection connected
- +106.1s phone: state ice disconnected
- +106.1s phone: state connection disconnected
- +106.7s phone: state ice connected
- +106.7s phone: state connection connected
- +114.8s phone: state ice disconnected
- +114.8s phone: state connection disconnected
- +116.0s phone: state ice connected
- +116.0s phone: state connection connected
