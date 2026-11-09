# Story seed 9 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 6.6 mean, 8.5 max, 12176 MB memory available at least.

Story: 2G@0, rtt spike@6, 2G@11, 2G@15, bursts@18, 2G@21, bursts@27, slow@33, 2G@40, bursts@45, 2G@48, gap@55, 2G@141, bursts@145, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.6s | 8.0s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 15.2s | 23.1s |
| Whole reply shown | 226.9s | 156.2s |
| Newest-text lag from the server's release, max | 171.1s | 103.9s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 176.4s | 111.5s |
| Longest freeze | 171.0s | 103.9s |
| Hangs (freezes of 30 s or more) | 171 s at 56 s | 104 s at 52 s, to the end |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
