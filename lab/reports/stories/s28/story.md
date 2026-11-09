# Story seed 28 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.6 mean, 6.2 max, 12147 MB memory available at least.

Story: 2G@0, 2G@4, gap@9, rtt spike@10, slow@13, 2G@15, bursts@22, 2G@26, slow@34, 2G@39, 2G@46, gap@49, rtt spike@63, bursts@69, 2G@72, 2G@79, slow@86, 2G@89, bursts@92, bursts@94, rtt spike@100, 2G@105, gap@110, 2G@126, gap@130, back@184.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 2.2s | 2.9s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.9s | 20.2s |
| Whole reply shown | 102.0s | 68.0s |
| Newest-text lag from the server's release, max | 49.8s | 17.2s |
| Lag within 5 s (case 50) | not met | not met |
| Newest-text lag from the agent, max | 55.0s | 24.9s |
| Longest freeze | 49.8s | 17.1s |
| Hangs (freezes of 30 s or more) | 50 s at 52 s, to the end | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 1 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
