# Story seed 42 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.1 mean, 6.2 max, 12059 MB memory available at least.

Story: 2G@0, bursts@3, rtt spike@7, gap@13, gap@35, gap@50, 2G@53, 2G@60, bursts@66, gap@72, rtt spike@83, bursts@86, 2G@92, bursts@97, 2G@100, 2G@106, 2G@110, 2G@113, 2G@121, 2G@124, 2G@126, rtt spike@128, 2G@135, slow@141, gap@146, back@167.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 2.1s | 5.8s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | never | 56.3s |
| Whole reply shown | never | 87.1s |
| Newest-text lag from the server's release, max | 287.2s | 44.3s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 296.5s | 51.1s |
| Longest freeze | 298.7s | 39.9s |
| Hangs (freezes of 30 s or more) | 299 s at 0 s, to the end | 40 s at 16 s |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
