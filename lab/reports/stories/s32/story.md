# Story seed 32 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 3.4 mean, 4.9 max, 12296 MB memory available at least.

Story: bursts@0, 2G@5, 2G@8, 2G@14, 2G@21, gap@28, bursts@55, gap@62, bursts@65, rtt spike@69, 2G@75, 2G@77, 2G@82, slow@84, 2G@90, rtt spike@93, 2G@101, bursts@107, bursts@112, gap@117, back@170.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 2.8s | 3.7s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 18.3s | 15.6s |
| Whole reply shown | never | 105.0s |
| Newest-text lag from the server's release, max | 270.2s | 30.2s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 287.0s | 45.9s |
| Longest freeze | 280.8s | 24.1s |
| Hangs (freezes of 30 s or more) | 281 s at 18 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
