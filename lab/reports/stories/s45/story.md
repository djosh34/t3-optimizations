# Story seed 45 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 6.6 mean, 7.2 max, 12310 MB memory available at least.

Story: bursts@0, slow@6, rtt spike@10, rtt spike@13, 2G@20, rtt spike@24, 2G@29, bursts@34, bursts@38, gap@43, 2G@45, 2G@52, gap@56, 2G@67, rtt spike@74, 2G@81, slow@84, rtt spike@92, 2G@100, 2G@107, 2G@110, 2G@115, 2G@123, gap@128, back@179.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 3.2s | 4.6s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 16.3s | 17.1s |
| Whole reply shown | 53.7s | 52.0s |
| Newest-text lag from the server's release, max | 8.9s | 5.8s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 17.8s | 18.4s |
| Longest freeze | 8.9s | 5.8s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
