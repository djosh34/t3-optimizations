# Story seed 47 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.5 mean, 6.0 max, 12137 MB memory available at least.

Story: 2G@0, 2G@3, bursts@8, 2G@11, bursts@15, 2G@22, bursts@29, bursts@33, 2G@39, 2G@46, 2G@53, 2G@59, 2G@65, bursts@67, bursts@71, 2G@75, bursts@82, 2G@84, 2G@88, 2G@92, 2G@99, bursts@103, bursts@109, bursts@113, 2G@119, slow@122, 2G@126, slow@133, 2G@136, 2G@141, bursts@144, back@151.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 1.6s | 2.6s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 12.0s | 22.1s |
| Whole reply shown | 48.0s | 58.4s |
| Newest-text lag from the server's release, max | 1.8s | 2.8s |
| Lag within 5 s (case 50) | met | met |
| Newest-text lag from the agent, max | 17.7s | 17.8s |
| Longest freeze | 1.7s | 2.9s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
