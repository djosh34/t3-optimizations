# Story seed 7 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.5 mean, 6.4 max, 12259 MB memory available at least.

Story: bursts@0, 2G@5, 2G@7, 2G@15, bursts@20, bursts@24, gap@27, 2G@32, rtt spike@36, 2G@44, bursts@50, 2G@52, rtt spike@59, bursts@61, bursts@65, bursts@70, 2G@75, 2G@81, rtt spike@84, 2G@91, bursts@96, rtt spike@103, 2G@109, 2G@112, 2G@120, slow@127, 2G@130, 2G@138, 2G@144, 2G@149, back@154.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 10.5s | 5.4s |
| Prompt within 10 s (case 70) | not met | met |
| First reply text | 15.9s | 17.9s |
| Whole reply shown | 68.9s | 54.1s |
| Newest-text lag from the server's release, max | 16.3s | 5.9s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 32.8s | 18.9s |
| Longest freeze | 16.2s | 3.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
