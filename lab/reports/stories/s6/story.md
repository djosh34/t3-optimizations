# Story seed 6 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.5 mean, 6.2 max, 12173 MB memory available at least.

Story: rtt spike@0, slow@3, 2G@11, 2G@14, 2G@19, bursts@27, gap@31, 2G@43, gap@50, 2G@68, 2G@76, slow@82, bursts@90, 2G@95, 2G@98, rtt spike@102, slow@107, 2G@112, 2G@115, 2G@121, gap@126, back@171.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 5.7s | 7.2s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.2s | 16.9s |
| Whole reply shown | never | 71.5s |
| Newest-text lag from the server's release, max | 1.4s | 15.3s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 17.2s | 27.6s |
| Longest freeze | 267.9s | 15.3s |
| Hangs (freezes of 30 s or more) | 268 s at 30 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
