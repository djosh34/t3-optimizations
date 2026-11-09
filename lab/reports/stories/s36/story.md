# Story seed 36 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 6.7 mean, 8.0 max, 12331 MB memory available at least.

Story: 2G@0, slow@7, 2G@14, slow@17, 2G@21, 2G@28, gap@32, 2G@33, slow@36, 2G@39, 2G@42, 2G@50, 2G@52, slow@55, 2G@58, 2G@60, bursts@63, 2G@70, bursts@76, gap@79, 2G@86, 2G@88, 2G@94, slow@100, rtt spike@103, slow@109, slow@112, 2G@120, 2G@122, rtt spike@125, gap@127, back@187.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.4s | 3.9s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.1s | 30.7s |
| Whole reply shown | 60.9s | 85.5s |
| Newest-text lag from the server's release, max | 8.6s | 33.1s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 25.4s | 38.9s |
| Longest freeze | 8.6s | 33.1s |
| Hangs (freezes of 30 s or more) | none | 33 s at 52 s, to the end |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
