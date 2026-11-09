# Story seed 25 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.8 mean, 6.6 max, 12343 MB memory available at least.

Story: 2G@0, 2G@5, bursts@10, 2G@17, 2G@21, slow@29, bursts@35, rtt spike@38, 2G@43, 2G@46, rtt spike@49, 2G@56, bursts@61, slow@63, 2G@71, 2G@77, gap@82, back@165.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.1s | 4.5s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.7s | 25.8s |
| Whole reply shown | 62.1s | 76.9s |
| Newest-text lag from the server's release, max | 7.6s | 13.7s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 23.5s | 29.9s |
| Longest freeze | 7.6s | 13.7s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
