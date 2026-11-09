# Story seed 8 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.7 mean, 6.4 max, 12259 MB memory available at least.

Story: bursts@0, 2G@8, 2G@13, bursts@20, bursts@27, rtt spike@31, bursts@33, 2G@39, slow@47, bursts@53, rtt spike@61, slow@66, 2G@72, gap@76, 2G@81, 2G@84, 2G@91, 2G@98, 2G@106, 2G@109, gap@114, 2G@144, 2G@150, back@154.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.0s | 3.4s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.5s | 14.2s |
| Whole reply shown | 53.9s | 54.1s |
| Newest-text lag from the server's release, max | 7.3s | 4.8s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 23.6s | 21.4s |
| Longest freeze | 7.3s | 4.2s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
