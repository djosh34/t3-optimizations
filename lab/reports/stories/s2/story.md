# Story seed 2 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.6 mean, 7.1 max, 12274 MB memory available at least.

Story: 2G@0, bursts@6, rtt spike@10, bursts@18, bursts@22, 2G@27, 2G@34, 2G@38, 2G@42, gap@44, bursts@65, 2G@72, 2G@75, bursts@79, bursts@84, 2G@92, 2G@94, 2G@99, bursts@104, bursts@109, bursts@116, 2G@120, rtt spike@126, 2G@131, 2G@135, bursts@140, gap@147, back@179.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.4s | 2.8s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 16.8s | 22.1s |
| Whole reply shown | 115.3s | 89.4s |
| Newest-text lag from the server's release, max | 71.5s | 7.5s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 79.0s | 20.2s |
| Longest freeze | 71.5s | 6.2s |
| Hangs (freezes of 30 s or more) | 71 s at 44 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
