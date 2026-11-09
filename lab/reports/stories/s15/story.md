# Story seed 15 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.9 mean, 7.0 max, 12190 MB memory available at least.

Story: 2G@0, 2G@7, 2G@10, rtt spike@14, slow@20, bursts@26, slow@31, 2G@33, 2G@36, rtt spike@43, 2G@48, 2G@53, 2G@57, rtt spike@60, gap@67, bursts@85, bursts@91, 2G@98, 2G@105, 2G@110, 2G@113, 2G@115, bursts@118, bursts@125, rtt spike@132, 2G@134, bursts@137, 2G@139, 2G@141, rtt spike@146, 2G@150, back@157.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.6s | 2.1s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.4s | 20.1s |
| Whole reply shown | 55.0s | 64.6s |
| Newest-text lag from the server's release, max | 3.3s | 10.4s |
| Lag within 5 s (case 50) | met | not met |
| Newest-text lag from the agent, max | 17.4s | 27.2s |
| Longest freeze | 3.3s | 10.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
