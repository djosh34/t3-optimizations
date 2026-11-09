# Story seed 5 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.4 mean, 6.0 max, 12411 MB memory available at least.

Story: 2G@0, slow@8, 2G@13, slow@17, 2G@22, 2G@25, 2G@28, bursts@32, bursts@40, 2G@45, 2G@50, bursts@56, 2G@59, 2G@63, bursts@67, 2G@74, 2G@81, gap@86, slow@98, bursts@100, gap@105, back@172.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.0s | 2.1s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.2s | 19.9s |
| Whole reply shown | 47.1s | 53.5s |
| Newest-text lag from the server's release, max | 1.4s | 6.8s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 17.5s | 19.2s |
| Longest freeze | 1.4s | 3.7s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
