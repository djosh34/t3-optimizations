# Story seed 35 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 2.4 mean, 6.7 max, 12203 MB memory available at least.

Story: gap@0, bursts@43, 2G@48, 2G@54, 2G@58, gap@66, slow@143, gap@149, bursts@150, back@152.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | never | 51.1s |
| Prompt within 10 s (case 70) | not met | not met |
| First reply text | never | 61.7s |
| Whole reply shown | never | 185.9s |
| Newest-text lag from the server's release, max | no text produced | 90.4s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | no text produced | 106.9s |
| Longest freeze | 298.4s | 90.4s |
| Hangs (freezes of 30 s or more) | 298 s at 0 s, to the end | 90 s at 77 s |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
