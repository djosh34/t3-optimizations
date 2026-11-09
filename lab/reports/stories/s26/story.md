# Story seed 26 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.5 mean, 7.5 max, 12235 MB memory available at least.

Story: gap@0, 2G@3, 2G@7, 2G@12, gap@15, 2G@16, gap@19, slow@23, 2G@30, bursts@32, gap@35, gap@96, rtt spike@103, rtt spike@109, bursts@115, bursts@120, 2G@127, slow@134, rtt spike@139, 2G@146, back@151.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 5.4s | 5.0s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 15.2s | 17.0s |
| Whole reply shown | never | 135.0s |
| Newest-text lag from the server's release, max | 1.6s | 82.1s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 17.9s | 98.6s |
| Longest freeze | 265.4s | 82.1s |
| Hangs (freezes of 30 s or more) | 265 s at 33 s, to the end | 82 s at 32 s |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
