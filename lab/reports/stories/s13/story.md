# Story seed 13 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.7 mean, 5.3 max, 12259 MB memory available at least.

Story: bursts@0, 2G@6, 2G@12, 2G@18, 2G@22, bursts@26, rtt spike@29, 2G@33, 2G@36, 2G@42, 2G@44, 2G@52, gap@58, rtt spike@73, 2G@76, 2G@82, slow@86, 2G@92, 2G@96, 2G@101, 2G@103, slow@105, bursts@109, 2G@114, gap@118, slow@129, bursts@134, 2G@140, 2G@146, back@153.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 5.2s | 4.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.6s | 18.4s |
| Whole reply shown | 65.9s | 54.6s |
| Newest-text lag from the server's release, max | 23.3s | 2.4s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 30.2s | 17.9s |
| Longest freeze | 23.3s | 2.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
