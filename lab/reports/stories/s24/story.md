# Story seed 24 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 6.8 mean, 7.8 max, 12266 MB memory available at least.

Story: 2G@0, slow@7, 2G@15, bursts@20, rtt spike@22, 2G@26, 2G@34, 2G@39, 2G@41, gap@47, 2G@52, bursts@56, 2G@59, gap@65, 2G@66, 2G@71, rtt spike@75, bursts@78, 2G@86, slow@89, 2G@95, gap@103, 2G@104, 2G@111, rtt spike@118, 2G@122, 2G@128, 2G@136, bursts@139, slow@147, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.4s | 2.8s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 18.7s | 18.8s |
| Whole reply shown | 58.7s | 56.5s |
| Newest-text lag from the server's release, max | 7.9s | 10.0s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 17.2s | 17.6s |
| Longest freeze | 7.9s | 5.9s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
