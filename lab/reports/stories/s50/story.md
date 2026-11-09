# Story seed 50 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.6 mean, 5.3 max, 12024 MB memory available at least.

Story: gap@0, bursts@2, 2G@5, 2G@9, 2G@14, slow@18, 2G@25, bursts@31, rtt spike@35, 2G@39, bursts@46, bursts@49, 2G@56, 2G@59, 2G@65, 2G@68, gap@75, gap@78, 2G@109, rtt spike@113, rtt spike@117, 2G@124, rtt spike@128, 2G@133, bursts@139, 2G@144, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 4.9s | 5.0s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 15.2s | 16.0s |
| Whole reply shown | 61.9s | 54.0s |
| Newest-text lag from the server's release, max | 6.3s | 2.0s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 18.1s | 17.6s |
| Longest freeze | 6.3s | 1.9s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
