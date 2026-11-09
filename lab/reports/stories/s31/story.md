# Story seed 31 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 4.9 mean, 7.2 max, 12278 MB memory available at least.

Story: 2G@0, bursts@4, rtt spike@11, 2G@15, slow@19, bursts@26, slow@31, bursts@39, 2G@42, bursts@47, 2G@50, 2G@55, 2G@60, 2G@62, bursts@67, bursts@70, 2G@72, slow@79, 2G@85, 2G@92, 2G@95, bursts@98, 2G@104, bursts@112, rtt spike@118, 2G@125, rtt spike@129, bursts@135, gap@140, gap@141, 2G@147, back@151.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.1s | 2.4s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 13.9s | 16.1s |
| Whole reply shown | 211.4s | 78.4s |
| Newest-text lag from the server's release, max | 165.2s | 23.9s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 172.5s | 39.9s |
| Longest freeze | 165.1s | 23.1s |
| Hangs (freezes of 30 s or more) | 165 s at 46 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
