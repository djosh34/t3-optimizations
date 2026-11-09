# Story seed 11 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.6 mean, 6.1 max, 12256 MB memory available at least.

Story: 2G@0, rtt spike@3, 2G@6, 2G@13, 2G@21, 2G@28, 2G@35, bursts@43, 2G@49, 2G@52, 2G@54, 2G@58, bursts@62, 2G@70, 2G@75, bursts@80, 2G@85, rtt spike@87, 2G@95, bursts@98, 2G@103, 2G@108, slow@116, rtt spike@118, bursts@125, 2G@127, 2G@131, bursts@136, 2G@142, gap@148, back@172.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 5.6s | 8.4s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.0s | 20.0s |
| Whole reply shown | 52.7s | 60.6s |
| Newest-text lag from the server's release, max | 11.5s | 2.9s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 18.5s | 19.3s |
| Longest freeze | 11.4s | 2.9s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
