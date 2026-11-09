# Story seed 14 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.1 mean, 6.4 max, 12035 MB memory available at least.

Story: 2G@0, 2G@2, 2G@5, 2G@11, 2G@19, 2G@24, rtt spike@27, bursts@31, 2G@34, bursts@37, slow@40, gap@44, rtt spike@48, bursts@53, 2G@57, gap@63, 2G@65, bursts@71, bursts@75, gap@79, gap@120, back@162.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 2.4s | 6.9s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.0s | 18.1s |
| Whole reply shown | never | 73.6s |
| Newest-text lag from the server's release, max | 242.3s | 28.2s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 249.2s | 35.3s |
| Longest freeze | 268.1s | 28.1s |
| Hangs (freezes of 30 s or more) | 268 s at 31 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
