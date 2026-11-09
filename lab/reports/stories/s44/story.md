# Story seed 44 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.0 mean, 6.9 max, 12213 MB memory available at least.

Story: 2G@0, bursts@7, slow@12, bursts@15, gap@18, bursts@40, bursts@42, gap@50, rtt spike@56, bursts@59, 2G@65, 2G@69, 2G@75, gap@78, rtt spike@93, 2G@97, 2G@104, gap@110, 2G@112, 2G@120, bursts@124, 2G@130, 2G@137, gap@144, back@159.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 2.9s | 3.5s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 17.8s | 21.6s |
| Whole reply shown | never | 95.6s |
| Newest-text lag from the server's release, max | 269.5s | 19.2s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 286.0s | 33.5s |
| Longest freeze | 280.6s | 19.1s |
| Hangs (freezes of 30 s or more) | 281 s at 18 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
