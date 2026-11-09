# Story seed 1 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.6 mean, 6.4 max, 12243 MB memory available at least.

Story: 2G@0, 2G@7, slow@12, 2G@16, bursts@23, 2G@26, 2G@28, rtt spike@36, 2G@41, 2G@46, 2G@53, 2G@60, 2G@68, slow@72, 2G@77, 2G@84, 2G@90, 2G@92, 2G@99, 2G@104, 2G@106, slow@114, bursts@116, gap@123, back@170.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.1s | 2.4s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.5s | 22.5s |
| Whole reply shown | 46.8s | 58.8s |
| Newest-text lag from the server's release, max | 2.6s | 3.3s |
| Lag within 5 s (case 50) | met | met |
| Newest-text lag from the agent, max | 17.3s | 19.5s |
| Longest freeze | 2.6s | 3.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
