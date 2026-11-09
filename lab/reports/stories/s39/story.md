# Story seed 39 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.3 mean, 5.3 max, 12262 MB memory available at least.

Story: 2G@0, gap@3, 2G@4, bursts@12, rtt spike@17, rtt spike@20, bursts@23, bursts@26, slow@32, 2G@39, bursts@42, rtt spike@48, slow@55, 2G@61, 2G@68, 2G@72, bursts@79, slow@85, 2G@91, bursts@96, 2G@100, rtt spike@106, slow@111, 2G@118, 2G@124, 2G@127, 2G@130, 2G@135, 2G@138, 2G@145, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 5.4s | 2.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.2s | 14.5s |
| Whole reply shown | 63.7s | 63.8s |
| Newest-text lag from the server's release, max | 7.4s | 5.4s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 17.8s | 18.2s |
| Longest freeze | 7.5s | 5.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
