# Story seed 27 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.3 mean, 6.1 max, 12121 MB memory available at least.

Story: slow@0, 2G@6, 2G@12, bursts@16, rtt spike@19, 2G@24, 2G@28, 2G@32, slow@37, 2G@44, gap@47, 2G@59, 2G@66, 2G@69, slow@74, 2G@80, bursts@86, bursts@92, 2G@97, rtt spike@103, 2G@109, 2G@113, bursts@119, 2G@126, rtt spike@129, 2G@133, 2G@138, rtt spike@145, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 13.2s | 41.8s |
| Prompt within 10 s (case 70) | not met | not met |
| First reply text | 27.6s | 51.0s |
| Whole reply shown | 145.3s | 96.9s |
| Newest-text lag from the server's release, max | 9.4s | 25.5s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 18.8s | 37.5s |
| Longest freeze | 9.5s | 25.5s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
