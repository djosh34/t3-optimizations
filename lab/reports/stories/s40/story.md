# Story seed 40 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.6 mean, 5.7 max, 12197 MB memory available at least.

Story: 2G@0, bursts@5, rtt spike@11, 2G@17, gap@22, rtt spike@24, rtt spike@32, gap@40, 2G@61, slow@65, rtt spike@72, bursts@74, slow@82, 2G@90, 2G@94, 2G@97, 2G@105, gap@113, back@171.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.3s | 1.8s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.1s | 15.6s |
| Whole reply shown | 227.4s | 67.5s |
| Newest-text lag from the server's release, max | 3.8s | 21.9s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 20.0s | 29.2s |
| Longest freeze | 3.8s | 21.9s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 2 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
