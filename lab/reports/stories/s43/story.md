# Story seed 43 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.9 mean, 5.8 max, 12247 MB memory available at least.

Story: 2G@0, slow@4, bursts@10, rtt spike@17, 2G@19, 2G@22, 2G@28, bursts@33, 2G@41, 2G@45, bursts@52, 2G@56, gap@63, bursts@92, 2G@96, 2G@100, 2G@103, gap@111, 2G@125, 2G@132, rtt spike@136, 2G@138, 2G@145, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 3.4s | 8.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.6s | 21.7s |
| Whole reply shown | 52.4s | 60.5s |
| Newest-text lag from the server's release, max | 3.3s | 5.9s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 17.7s | 19.7s |
| Longest freeze | 3.3s | 5.6s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
