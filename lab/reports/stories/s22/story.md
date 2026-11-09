# Story seed 22 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.5 mean, 5.1 max, 12107 MB memory available at least.

Story: 2G@0, 2G@4, slow@8, 2G@11, 2G@14, 2G@22, 2G@25, 2G@28, 2G@35, slow@41, 2G@44, 2G@47, bursts@52, 2G@58, slow@63, gap@68, 2G@70, 2G@75, 2G@79, 2G@81, 2G@86, 2G@91, bursts@98, 2G@106, 2G@110, 2G@116, 2G@123, 2G@130, 2G@136, slow@141, bursts@148, back@153.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 3.0s | 2.6s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.9s | 14.1s |
| Whole reply shown | 49.0s | 50.0s |
| Newest-text lag from the server's release, max | 2.5s | 3.7s |
| Lag within 5 s (case 50) | met | met |
| Newest-text lag from the agent, max | 17.6s | 18.1s |
| Longest freeze | 2.5s | 3.7s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
