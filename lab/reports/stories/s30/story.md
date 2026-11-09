# Story seed 30 (2G)

Commit 9c461bdca554. Host: 4 CPUs, load 5.1 mean, 5.5 max, 12218 MB memory available at least.

Story: rtt spike@0, bursts@6, slow@11, 2G@18, 2G@26, bursts@30, bursts@36, bursts@42, 2G@45, 2G@49, rtt spike@57, bursts@61, slow@65, bursts@67, rtt spike@74, 2G@79, gap@81, 2G@91, rtt spike@94, 2G@100, rtt spike@104, gap@108, 2G@122, 2G@129, 2G@132, 2G@136, 2G@143, 2G@146, slow@149, back@151.

| | websocket | webapp |
|---|---|---|
| Status | done | done |
| Prompt confirmed | 4.5s | 6.2s |
| Prompt within 10 s (case 70) | met | met |
| First reply text | 14.2s | 17.4s |
| Whole reply shown | 62.7s | 51.0s |
| Newest-text lag from the server's release, max | 12.1s | 3.9s |
| Lag within 5 s (case 50) | not met | met |
| Newest-text lag from the agent, max | 28.3s | 18.0s |
| Longest freeze | 12.1s | 3.4s |
| Hangs (freezes of 30 s or more) | none | none |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 0 | 0 |
| Transports | - | webrtc |
| Stayed on the data channel (UDP) | - | met |
