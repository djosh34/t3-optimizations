# Story seed 23 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.7 mean, 6.0 max, 12230 MB memory available at least.

Story: slow@0, 2G@7, bursts@11, gap@18, 2G@20, 2G@27, 2G@33, gap@36, slow@44, 2G@47, bursts@50, rtt spike@56, slow@59, bursts@63, rtt spike@66, gap@73, gap@81, bursts@83, slow@89, 2G@96, bursts@102, bursts@107, 2G@111, rtt spike@116, bursts@122, 2G@127, slow@134, rtt spike@137, bursts@142, 2G@149, back@152.

| | websocket | webapp |
|---|---|---|
| Status | not done by deadline | done |
| Prompt confirmed | 3.9s | 12.5s |
| Prompt within 10 s (case 70) | met | not met |
| First reply text | 25.5s | 23.1s |
| Whole reply shown | never | 85.3s |
| Newest-text lag from the server's release, max | 12.7s | 12.4s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 22.2s | 25.8s |
| Longest freeze | 267.5s | 9.9s |
| Hangs (freezes of 30 s or more) | 268 s at 31 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
