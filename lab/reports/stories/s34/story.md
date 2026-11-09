# Story seed 34 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 3.5 mean, 3.8 max, 12278 MB memory available at least.

Story: rtt spike@0, 2G@2, bursts@7, 2G@12, 2G@19, bursts@26, rtt spike@31, rtt spike@34, 2G@38, 2G@44, 2G@49, bursts@52, 2G@57, rtt spike@61, rtt spike@67, 2G@70, bursts@73, 2G@76, rtt spike@80, slow@85, bursts@88, gap@93, gap@99, 2G@100, 2G@106, 2G@114, gap@118, 2G@121, bursts@125, 2G@133, 2G@137, gap@142, 2G@144, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 3.2s | 3.5s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.6s | 14.1s |
| Whole reply shown | 56.4s | 52.3s |
| Newest-text lag from the server's release, max | 1.1s | 1.4s |
| Lag within 5 s (case 50) | met | met |
| Newest-text lag from the agent, max | 17.4s | 17.3s |
| Longest freeze | 1.1s | 1.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
