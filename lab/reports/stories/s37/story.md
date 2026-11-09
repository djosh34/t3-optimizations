# Story seed 37 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 3.0 mean, 3.5 max, 12321 MB memory available at least.

Story: 2G@0, 2G@6, bursts@11, rtt spike@18, slow@25, 2G@29, bursts@36, slow@43, 2G@50, slow@58, 2G@63, bursts@66, gap@71, slow@75, slow@79, 2G@81, bursts@84, 2G@89, rtt spike@97, bursts@104, 2G@111, bursts@118, bursts@123, bursts@130, bursts@134, 2G@138, 2G@141, slow@147, back@154.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.3s | 2.4s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 11.6s | 13.4s |
| Whole reply shown | 50.7s | 55.8s |
| Newest-text lag from the server's release, max | 4.4s | 7.7s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 21.1s | 19.0s |
| Longest freeze | 4.4s | 6.5s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
