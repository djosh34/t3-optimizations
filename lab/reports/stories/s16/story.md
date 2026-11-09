# Story seed 16 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 2.1 mean, 5.4 max, 12179 MB memory available at least.

Story: gap@0, 2G@12, rtt spike@19, slow@26, gap@32, 2G@51, gap@58, 2G@70, bursts@75, bursts@83, bursts@85, 2G@92, bursts@97, bursts@100, 2G@104, 2G@107, bursts@111, 2G@116, 2G@120, gap@127, back@176.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | never | 15.9s |
| Prompt within 10 s (case 70) | not met | not met |
| First reply text | never | 52.3s |
| Whole reply shown | never | 73.0s |
| Newest-text lag from the server's release, max | 272.5s | 29.8s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 281.7s | 39.3s |
| Longest freeze | 298.8s | 27.2s |
| Hangs (freezes of 30 s or more) | 299 s at 0 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
