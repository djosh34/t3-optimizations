# Story seed 20 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.6 mean, 5.8 max, 12292 MB memory available at least.

Story: bursts@0, 2G@7, 2G@10, 2G@16, rtt spike@22, 2G@27, rtt spike@33, rtt spike@37, 2G@42, 2G@46, rtt spike@51, bursts@58, slow@65, slow@72, 2G@76, rtt spike@81, 2G@88, 2G@91, 2G@94, 2G@99, 2G@107, 2G@111, slow@115, 2G@121, bursts@125, bursts@129, 2G@136, 2G@141, bursts@148, back@152.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 10.4s | 5.9s |
| Prompt within 10 s (case 70) | not met | met |
| First reply text | 16.0s | 17.0s |
| Whole reply shown | 60.0s | 94.4s |
| Newest-text lag from the server's release, max | 4.2s | 26.8s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 19.5s | 32.5s |
| Longest freeze | 4.2s | 23.0s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
