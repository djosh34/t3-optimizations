# Story seed 29 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.4 mean, 7.4 max, 12210 MB memory available at least.

Story: bursts@0, 2G@7, gap@14, bursts@27, rtt spike@35, bursts@38, bursts@44, 2G@49, bursts@52, 2G@57, gap@62, bursts@65, bursts@72, slow@74, gap@81, 2G@147, back@152.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 7.8s | 2.2s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | never | 27.8s |
| Whole reply shown | never | 55.5s |
| Newest-text lag from the server's release, max | 282.4s | 10.8s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 291.8s | 20.0s |
| Longest freeze | 298.7s | 10.9s |
| Hangs (freezes of 30 s or more) | 299 s at 0 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
