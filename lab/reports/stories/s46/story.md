# Story seed 46 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.5 mean, 5.5 max, 12238 MB memory available at least.

Story: 2G@0, 2G@5, 2G@13, 2G@20, slow@27, 2G@33, rtt spike@35, gap@40, 2G@44, gap@51, bursts@67, bursts@69, slow@74, slow@78, bursts@80, bursts@86, rtt spike@91, 2G@99, 2G@105, gap@109, slow@111, 2G@115, 2G@121, 2G@128, bursts@132, gap@136, back@212.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.6s | 5.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 11.9s | 14.7s |
| Whole reply shown | 83.3s | 72.1s |
| Newest-text lag from the server's release, max | 6.1s | 14.7s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 22.9s | 26.4s |
| Longest freeze | 6.1s | 14.8s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
