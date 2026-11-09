# Story seed 4 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 6.9 mean, 7.4 max, 12066 MB memory available at least.

Story: bursts@0, 2G@8, 2G@14, slow@21, 2G@28, 2G@30, 2G@37, 2G@43, 2G@47, gap@54, 2G@58, bursts@61, rtt spike@68, 2G@73, 2G@80, bursts@83, 2G@86, bursts@90, bursts@93, bursts@95, 2G@103, gap@107, bursts@118, 2G@122, gap@127, back@197.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.9s | 2.3s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.6s | 16.2s |
| Whole reply shown | 60.1s | 51.3s |
| Newest-text lag from the server's release, max | 6.1s | 1.5s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 17.4s | 17.8s |
| Longest freeze | 6.1s | 1.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
