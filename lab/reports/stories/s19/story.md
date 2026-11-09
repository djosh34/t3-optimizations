# Story seed 19 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.2 mean, 5.2 max, 12133 MB memory available at least.

Story: 2G@0, 2G@5, 2G@12, 2G@19, gap@24, slow@38, slow@42, bursts@48, 2G@55, rtt spike@59, rtt spike@67, rtt spike@71, 2G@76, slow@82, rtt spike@84, gap@89, 2G@98, rtt spike@102, gap@108, bursts@111, 2G@118, rtt spike@124, slow@129, bursts@132, slow@135, rtt spike@139, 2G@144, back@151.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.3s | 6.0s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 11.9s | 15.8s |
| Whole reply shown | 217.2s | 74.5s |
| Newest-text lag from the server's release, max | 172.4s | 23.5s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 189.0s | 37.4s |
| Longest freeze | 172.4s | 21.3s |
| Hangs (freezes of 30 s or more) | 172 s at 28 s | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 2 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
