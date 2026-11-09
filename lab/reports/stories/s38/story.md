# Story seed 38 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.4 mean, 7.4 max, 12217 MB memory available at least.

Story: 2G@0, 2G@4, rtt spike@8, bursts@15, 2G@20, gap@22, 2G@30, 2G@35, gap@40, 2G@53, 2G@58, slow@65, slow@69, 2G@72, 2G@74, 2G@76, 2G@83, 2G@88, 2G@92, slow@99, gap@104, 2G@121, 2G@128, bursts@134, 2G@139, 2G@145, rtt spike@148, back@153.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 2.5s | 7.5s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 16.4s | 30.1s |
| Whole reply shown | never | 129.0s |
| Newest-text lag from the server's release, max | 247.6s | 52.9s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 254.6s | 55.6s |
| Longest freeze | 263.5s | 48.1s |
| Hangs (freezes of 30 s or more) | 264 s at 35 s, to the end | 48 s at 77 s |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
