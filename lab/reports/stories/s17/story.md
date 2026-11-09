# Story seed 17 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.1 mean, 6.0 max, 12273 MB memory available at least.

Story: 2G@0, gap@2, 2G@8, slow@14, slow@18, bursts@24, slow@32, bursts@39, 2G@45, 2G@52, gap@55, 2G@83, 2G@87, 2G@95, 2G@98, 2G@101, 2G@104, slow@108, 2G@111, rtt spike@118, gap@124, 2G@135, 2G@139, gap@146, gap@149, back@163.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | never | 26.9s |
| Prompt within 10 s (case 70) | not met | not met |
| First reply text | 47.8s | 46.0s |
| Whole reply shown | 220.7s | 104.7s |
| Newest-text lag from the server's release, max | 163.5s | 33.8s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 170.6s | 49.8s |
| Longest freeze | 163.4s | 33.8s |
| Hangs (freezes of 30 s or more) | 36 s at 12 s; 163 s at 57 s, to the end | 34 s at 53 s |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 2 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
