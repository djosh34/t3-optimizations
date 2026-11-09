# Story seed 12 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.0 mean, 7.0 max, 12265 MB memory available at least.

Story: 2G@0, 2G@3, slow@10, 2G@16, bursts@22, bursts@30, bursts@34, 2G@39, 2G@46, 2G@49, bursts@53, 2G@56, slow@63, gap@69, 2G@111, rtt spike@119, rtt spike@123, bursts@126, 2G@128, 2G@132, 2G@135, 2G@140, slow@145, 2G@148, back@151.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 3.6s | 4.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.3s | 16.8s |
| Whole reply shown | 181.9s | 52.0s |
| Newest-text lag from the server's release, max | 117.8s | 2.0s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 123.1s | 18.1s |
| Longest freeze | 117.7s | 2.0s |
| Hangs (freezes of 30 s or more) | 118 s at 64 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
