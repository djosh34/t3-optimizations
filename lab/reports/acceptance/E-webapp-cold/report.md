# E, webapp, cold start, seed 1

Profile E (graded): owner extreme: 1 kbit/s, 75% bursty loss. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.2s |
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 898.0s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 4716 / 7445 |
| Bytes the app received / sent | 0 / 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.0 | 0 | 0 |
| 40 | 0.0 | 0 | 0 |
| 50 | 0.0 | 0 | 0 |
| 60 | 0.0 | 0 | 0 |
| 70 | 0.0 | 0 | 0 |
| 80 | 0.0 | 0 | 0 |
| 90 | 0.0 | 0 | 0 |
| 100 | 0.0 | 0 | 0 |
| 110 | 0.0 | 0 | 0 |
| 120 | 0.0 | 0 | 0 |
| 130 | 0.0 | 0 | 0 |
| 140 | 0.0 | 0 | 0 |
| 150 | 0.0 | 0 | 0 |
| 160 | 0.0 | 0 | 0 |
| 170 | 0.0 | 0 | 0 |
| 180 | 0.0 | 0 | 0 |
| 190 | 0.0 | 0 | 0 |
| 200 | 0.0 | 0 | 0 |
| 210 | 0.0 | 0 | 0 |
| 220 | 0.0 | 0 | 0 |
| 230 | 0.0 | 0 | 0 |
| 240 | 0.0 | 0 | 0 |
| 250 | 0.0 | 0 | 0 |
| 260 | 0.0 | 0 | 0 |
| 270 | 0.0 | 0 | 0 |
| 280 | 0.0 | 0 | 0 |
| 290 | 0.0 | 0 | 0 |
| 300 | 0.0 | 0 | 0 |
| 310 | 0.0 | 0 | 0 |
| 320 | 0.0 | 0 | 0 |
| 330 | 0.0 | 0 | 0 |
| 340 | 0.0 | 0 | 0 |
| 350 | 0.0 | 0 | 0 |
| 360 | 0.0 | 0 | 0 |
| 370 | 0.0 | 0 | 0 |
| 380 | 0.0 | 0 | 0 |
| 390 | 0.0 | 0 | 0 |
| 400 | 0.0 | 0 | 0 |
| 410 | 0.0 | 0 | 0 |
| 420 | 0.0 | 0 | 0 |
| 430 | 0.0 | 0 | 0 |
| 440 | 0.0 | 0 | 0 |
| 450 | 0.0 | 0 | 0 |
| 460 | 0.0 | 0 | 0 |
| 470 | 0.0 | 0 | 0 |
| 480 | 0.0 | 0 | 0 |
| 490 | 0.0 | 0 | 0 |
| 500 | 0.0 | 0 | 0 |
| 510 | 0.0 | 0 | 0 |
| 520 | 0.0 | 0 | 0 |
| 530 | 0.0 | 0 | 0 |
| 540 | 0.0 | 0 | 0 |
| 550 | 0.0 | 0 | 0 |
| 560 | 0.0 | 0 | 0 |
| 570 | 0.0 | 0 | 0 |
| 580 | 0.0 | 0 | 0 |
| 590 | 0.0 | 0 | 0 |
| 600 | 0.0 | 0 | 0 |
| 610 | 0.0 | 0 | 0 |
| 620 | 0.0 | 0 | 0 |
| 630 | 0.0 | 0 | 0 |
| 640 | 0.0 | 0 | 0 |
| 650 | 0.0 | 0 | 0 |
| 660 | 0.0 | 0 | 0 |
| 670 | 0.0 | 0 | 0 |
| 680 | 0.0 | 0 | 0 |
| 690 | 0.0 | 0 | 0 |
| 700 | 0.0 | 0 | 0 |
| 710 | 0.0 | 0 | 0 |
| 720 | 0.0 | 0 | 0 |
| 730 | 0.0 | 0 | 0 |
| 740 | 0.0 | 0 | 0 |
| 750 | 0.0 | 0 | 0 |
| 760 | 0.0 | 0 | 0 |
| 770 | 0.0 | 0 | 0 |
| 780 | 0.0 | 0 | 0 |
| 790 | 0.0 | 0 | 0 |
| 800 | 0.0 | 0 | 0 |
| 810 | 0.0 | 0 | 0 |
| 820 | 0.0 | 0 | 0 |
| 830 | 0.0 | 0 | 0 |
| 840 | 0.0 | 0 | 0 |
| 850 | 0.0 | 0 | 0 |
| 860 | 0.0 | 0 | 0 |
| 870 | 0.0 | 0 | 0 |
| 880 | 0.0 | 0 | 0 |
| 890 | 0.0 | 0 | 0 |
| 898 | 0.0 | 0 | 0 |

## Driver notes

- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2844,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2844,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2844,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3336,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3336,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3829,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3829,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3829,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4322,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4322,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4815,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4815,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4981,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5474,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5474,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5967,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5967,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5967,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6460,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6460,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6953,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6953,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6953,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7446,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7446,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7939,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7939,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7939,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8432,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8432,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8925,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8925,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8925,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9418,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9418,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9911,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9911,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9911,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10404,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10404,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10897,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10897,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10897,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11390,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11390,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11883,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11883,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11883,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12376,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12376,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13362,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13362,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13855,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13855,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14348,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14348,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14348,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14841,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14841,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15334,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15334,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15334,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15827,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15827,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16320,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16320,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16320,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16813,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16813,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17306,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17306,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17306,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17799,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17799,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18292,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18292,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18292,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18785,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18785,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19278,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19278,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19278,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19771,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19771,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20264,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20264,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"1014/1014 971/971","turnItems":"860/860 1058/1058 1108/1108 870/870 1058/1058 1108/1108 1299/1299","assistant":"21/21 32/32","messages":"23/901 456/456 550/550","providerSessions":"2688/2688","frame":"1032/1032","attempts":"662/662 662/662","checkpoints":"779/779 613/613 779/779","checkpointScopes":"726/726","providerThreads":"810/810","providerTurns":"778/778","user":"9/9 10/10","thread":"951/951","nodes":"809/809 1025/1025 809/809"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20264,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
18:00:41.023 fetch GET /api/auth/session
18:00:41.026 fetched GET /api/auth/session 200
18:00:41.136 fetch GET /.well-known/t3/environment
18:00:41.137 fetched GET /.well-known/t3/environment 200
18:00:41.157 fetch GET /.well-known/t3/environment
18:00:41.158 fetched GET /.well-known/t3/environment 200
18:00:41.165 rpc 0 subscribeServerConfig
18:00:41.165 chunk 0 38690 bytes
18:00:41.189 in Ack 0
18:00:41.193 rpc 1 subscribeServerLifecycle
18:00:41.193 rpc 2 subscribeServerLifecycle
18:00:41.198 fetch GET /api/auth/session
18:00:41.199 fetched GET /api/auth/session 200
18:00:41.199 rpc 3 subscribeDeviceState
18:00:41.202 rpc 4 preview.reportProfiles
18:00:41.309 rpc 5 subscribeProjectClones
18:00:41.354 rpc 6 orchestration.subscribeThread
18:00:41.355 chunk 6 32178 bytes
18:00:41.358 in Ack 6
18:00:41.367 fetch GET /api/orchestration/shell
18:00:41.420 rpc 7 server.refreshProviders
18:00:41.521 rpc 8 orchestration.dispatchCommand thread.visit
18:00:41.521 exit 8 {"_tag":"Success","value":{"sequence":0}}
18:00:41.606 fetched GET /api/orchestration/shell 200
18:00:41.606 rpc 9 subscribeTerminalMetadata
18:00:41.607 rpc 10 subscribePreviewEvents
18:00:41.608 rpc 11 subscribeVcsStatus
18:00:41.611 rpc 12 orchestration.subscribeShell
18:00:41.611 chunk 12 2847 bytes
18:00:41.613 rpc 13 assets.createUrl
18:00:41.613 rpc 14 preview.list
18:00:41.614 rpc 15 projects.readFile
18:00:41.614 rpc 16 assets.createUrl
18:00:41.663 in Ack 12
18:00:41.865 rpc 17 server.reportClientActivity
18:00:42.134 fetch POST /api/observability/v1/traces
18:00:42.134 fetched POST /api/observability/v1/traces 204
18:00:42.479 rpc 18 orchestration.dispatchCommand message.dispatch
18:00:43.479 fetch POST /api/observability/v1/traces
18:00:43.479 fetched POST /api/observability/v1/traces 204
18:00:46.162 in Ping 
18:00:51.162 in Ping 
18:00:56.162 in Ping 
18:00:56.868 in Interrupt 17
18:00:57.869 fetch POST /api/observability/v1/traces
18:00:57.869 fetched POST /api/observability/v1/traces 204
18:01:01.162 in Ping 
18:01:06.163 in Ping 
18:01:06.389 rpc 19 server.reportClientActivity
18:01:07.389 fetch POST /api/observability/v1/traces
18:01:07.389 fetched POST /api/observability/v1/traces 204
18:01:11.163 in Ping 
18:01:16.163 in Ping 
18:01:21.163 in Ping 
18:01:21.390 in Interrupt 19
18:01:22.390 fetch POST /api/observability/v1/traces
18:01:22.391 fetched POST /api/observability/v1/traces 204
18:01:26.164 in Ping 
18:01:31.164 in Ping 
18:01:31.388 rpc 20 server.reportClientActivity
18:01:32.388 fetch POST /api/observability/v1/traces
18:01:32.388 fetched POST /api/observability/v1/traces 204
18:01:36.167 in Ping 
18:01:41.171 in Ping 
18:01:46.175 in Ping 
18:01:46.389 in Interrupt 20
18:01:47.390 fetch POST /api/observability/v1/traces
18:01:47.390 fetched POST /api/observability/v1/traces 204
18:01:51.179 in Ping 
18:01:56.183 in Ping 
18:01:56.389 rpc 21 server.reportClientActivity
18:01:57.389 fetch POST /api/observability/v1/traces
18:01:57.389 fetched POST /api/observability/v1/traces 204
18:02:01.187 in Ping 
18:02:06.191 in Ping 
18:02:11.196 in Ping 
18:02:11.390 in Interrupt 21
18:02:12.398 fetch POST /api/observability/v1/traces
18:02:12.398 fetched POST /api/observability/v1/traces 204
18:02:16.199 in Ping 
18:02:21.203 in Ping 
18:02:21.389 rpc 22 server.reportClientActivity
18:02:22.390 fetch POST /api/observability/v1/traces
18:02:22.390 fetched POST /api/observability/v1/traces 204
18:02:26.207 in Ping 
18:02:31.211 in Ping 
18:02:36.215 in Ping 
18:02:36.390 in Interrupt 22
18:02:37.392 fetch POST /api/observability/v1/traces
18:02:37.392 fetched POST /api/observability/v1/traces 204
18:02:41.219 in Ping 
18:02:46.223 in Ping 
18:02:46.393 rpc 23 server.reportClientActivity
18:02:47.393 fetch POST /api/observability/v1/traces
18:02:47.393 fetched POST /api/observability/v1/traces 204
18:02:51.227 in Ping 
18:02:56.231 in Ping 
18:03:01.235 in Ping 
18:03:01.393 in Interrupt 23
18:03:02.395 fetch POST /api/observability/v1/traces
18:03:02.395 fetched POST /api/observability/v1/traces 204
18:03:06.239 in Ping 
18:03:11.243 in Ping 
18:03:11.390 rpc 24 server.reportClientActivity
18:03:12.389 fetch POST /api/observability/v1/traces
18:03:12.389 fetched POST /api/observability/v1/traces 204
18:03:16.247 in Ping 
18:03:21.251 in Ping 
18:03:26.255 in Ping 
18:03:26.390 in Interrupt 24
18:03:27.391 fetch POST /api/observability/v1/traces
18:03:27.391 fetched POST /api/observability/v1/traces 204
18:03:31.259 in Ping 
18:03:36.263 in Ping 
18:03:36.389 rpc 25 server.reportClientActivity
18:03:37.390 fetch POST /api/observability/v1/traces
18:03:37.390 fetched POST /api/observability/v1/traces 204
18:03:41.267 in Ping 
18:03:46.273 in Ping 
18:03:51.276 in Ping 
18:03:51.390 in Interrupt 25
18:03:52.391 fetch POST /api/observability/v1/traces
18:03:52.391 fetched POST /api/observability/v1/traces 204
18:03:56.279 in Ping 
18:04:01.283 in Ping 
18:04:01.390 rpc 26 server.reportClientActivity
18:04:02.390 fetch POST /api/observability/v1/traces
18:04:02.390 fetched POST /api/observability/v1/traces 204
18:04:06.287 in Ping 
18:04:11.291 in Ping 
18:04:16.295 in Ping 
18:04:16.390 in Interrupt 26
18:04:17.395 fetch POST /api/observability/v1/traces
18:04:17.395 fetched POST /api/observability/v1/traces 204
18:04:21.299 in Ping 
18:04:26.303 in Ping 
18:04:26.391 rpc 27 server.reportClientActivity
18:04:27.392 fetch POST /api/observability/v1/traces
18:04:27.392 fetched POST /api/observability/v1/traces 204
18:04:31.307 in Ping 
18:04:36.311 in Ping 
18:04:41.315 in Ping 
18:04:41.395 in Interrupt 27
18:04:42.400 fetch POST /api/observability/v1/traces
18:04:42.400 fetched POST /api/observability/v1/traces 204
18:04:46.319 in Ping 
18:04:51.325 in Ping 
18:04:51.393 rpc 28 server.reportClientActivity
18:04:52.396 fetch POST /api/observability/v1/traces
18:04:52.396 fetched POST /api/observability/v1/traces 204
18:04:56.327 in Ping 
18:05:01.331 in Ping 
18:05:06.335 in Ping 
18:05:06.395 in Interrupt 28
18:05:07.400 fetch POST /api/observability/v1/traces
18:05:07.400 fetched POST /api/observability/v1/traces 204
18:05:11.339 in Ping 
18:05:16.343 in Ping 
18:05:16.396 rpc 29 server.reportClientActivity
18:05:17.399 fetch POST /api/observability/v1/traces
18:05:17.399 fetched POST /api/observability/v1/traces 204
18:05:21.347 in Ping 
18:05:26.351 in Ping 
18:05:31.355 in Ping 
18:05:31.399 in Interrupt 29
18:05:32.404 fetch POST /api/observability/v1/traces
18:05:32.404 fetched POST /api/observability/v1/traces 204
18:05:36.359 in Ping 
18:05:41.363 in Ping 
18:05:41.401 rpc 30 server.reportClientActivity
18:05:42.403 fetch POST /api/observability/v1/traces
18:05:42.403 fetched POST /api/observability/v1/traces 204
18:05:46.367 in Ping 
18:05:51.371 in Ping 
18:05:56.375 in Ping 
18:05:56.403 in Interrupt 30
18:05:57.407 fetch POST /api/observability/v1/traces
18:05:57.407 fetched POST /api/observability/v1/traces 204
18:06:01.379 in Ping 
18:06:06.383 in Ping 
18:06:06.404 rpc 31 server.reportClientActivity
18:06:07.407 fetch POST /api/observability/v1/traces
18:06:07.407 fetched POST /api/observability/v1/traces 204
18:06:11.387 in Ping 
18:06:16.391 in Ping 
18:06:21.395 in Ping 
18:06:21.407 in Interrupt 31
18:06:22.414 fetch POST /api/observability/v1/traces
18:06:22.414 fetched POST /api/observability/v1/traces 204
18:06:26.399 in Ping 
18:06:31.403 in Ping 
18:06:31.408 rpc 32 server.reportClientActivity
18:06:32.411 fetch POST /api/observability/v1/traces
18:06:32.412 fetched POST /api/observability/v1/traces 204
18:06:36.407 in Ping 
18:06:41.411 in Ping 
18:06:46.411 in Interrupt 32
18:06:46.415 in Ping 
18:06:47.415 fetch POST /api/observability/v1/traces
18:06:47.415 fetched POST /api/observability/v1/traces 204
18:06:51.419 in Ping 
18:06:56.413 rpc 33 server.reportClientActivity
18:06:56.423 in Ping 
18:06:57.418 fetch POST /api/observability/v1/traces
18:06:57.418 fetched POST /api/observability/v1/traces 204
18:07:01.427 in Ping 
18:07:06.431 in Ping 
18:07:11.415 in Interrupt 33
18:07:11.435 in Ping 
18:07:12.419 fetch POST /api/observability/v1/traces
18:07:12.420 fetched POST /api/observability/v1/traces 204
18:07:16.439 in Ping 
18:07:21.416 rpc 34 server.reportClientActivity
18:07:21.443 in Ping 
18:07:22.420 fetch POST /api/observability/v1/traces
18:07:22.420 fetched POST /api/observability/v1/traces 204
18:07:26.447 in Ping 
18:07:31.451 in Ping 
18:07:36.419 in Interrupt 34
18:07:36.455 in Ping 
18:07:37.423 fetch POST /api/observability/v1/traces
18:07:37.423 fetched POST /api/observability/v1/traces 204
18:07:41.459 in Ping 
18:07:46.420 rpc 35 server.reportClientActivity
18:07:46.463 in Ping 
18:07:47.423 fetch POST /api/observability/v1/traces
18:07:47.423 fetched POST /api/observability/v1/traces 204
18:07:51.467 in Ping 
18:07:56.471 in Ping 
18:08:01.423 in Interrupt 35
18:08:01.475 in Ping 
18:08:02.428 fetch POST /api/observability/v1/traces
18:08:02.428 fetched POST /api/observability/v1/traces 204
18:08:06.479 in Ping 
18:08:11.424 rpc 36 server.reportClientActivity
18:08:11.483 in Ping 
18:08:12.427 fetch POST /api/observability/v1/traces
18:08:12.427 fetched POST /api/observability/v1/traces 204
18:08:16.487 in Ping 
18:08:21.491 in Ping 
18:08:26.427 in Interrupt 36
18:08:26.495 in Ping 
18:08:27.431 fetch POST /api/observability/v1/traces
18:08:27.431 fetched POST /api/observability/v1/traces 204
18:08:31.499 in Ping 
18:08:36.428 rpc 37 server.reportClientActivity
18:08:36.503 in Ping 
18:08:37.431 fetch POST /api/observability/v1/traces
18:08:37.431 fetched POST /api/observability/v1/traces 204
18:08:41.507 in Ping 
18:08:46.511 in Ping 
18:08:51.431 in Interrupt 37
18:08:51.515 in Ping 
18:08:52.435 fetch POST /api/observability/v1/traces
18:08:52.435 fetched POST /api/observability/v1/traces 204
18:08:56.519 in Ping 
18:09:01.432 rpc 38 server.reportClientActivity
18:09:01.523 in Ping 
18:09:02.436 fetch POST /api/observability/v1/traces
18:09:02.436 fetched POST /api/observability/v1/traces 204
18:09:06.527 in Ping 
18:09:11.531 in Ping 
18:09:16.435 in Interrupt 38
18:09:16.535 in Ping 
18:09:17.439 fetch POST /api/observability/v1/traces
18:09:17.439 fetched POST /api/observability/v1/traces 204
18:09:21.539 in Ping 
18:09:26.436 rpc 39 server.reportClientActivity
18:09:26.543 in Ping 
18:09:27.439 fetch POST /api/observability/v1/traces
18:09:27.439 fetched POST /api/observability/v1/traces 204
18:09:31.547 in Ping 
18:09:36.551 in Ping 
18:09:41.439 in Interrupt 39
18:09:41.555 in Ping 
18:09:42.443 fetch POST /api/observability/v1/traces
18:09:42.443 fetched POST /api/observability/v1/traces 204
18:09:46.559 in Ping 
18:09:51.440 rpc 40 server.reportClientActivity
18:09:51.563 in Ping 
18:09:52.443 fetch POST /api/observability/v1/traces
18:09:52.443 fetched POST /api/observability/v1/traces 204
18:09:56.567 in Ping 
18:10:01.571 in Ping 
18:10:06.443 in Interrupt 40
18:10:06.575 in Ping 
18:10:07.448 fetch POST /api/observability/v1/traces
18:10:07.448 fetched POST /api/observability/v1/traces 204
18:10:11.579 in Ping 
18:10:16.444 rpc 41 server.reportClientActivity
18:10:16.583 in Ping 
18:10:17.447 fetch POST /api/observability/v1/traces
18:10:17.447 fetched POST /api/observability/v1/traces 204
18:10:21.587 in Ping 
18:10:26.591 in Ping 
18:10:31.447 in Interrupt 41
18:10:31.595 in Ping 
18:10:32.452 fetch POST /api/observability/v1/traces
18:10:32.452 fetched POST /api/observability/v1/traces 204
18:10:36.599 in Ping 
18:10:41.448 rpc 42 server.reportClientActivity
18:10:41.603 in Ping 
18:10:42.452 fetch POST /api/observability/v1/traces
18:10:42.452 fetched POST /api/observability/v1/traces 204
18:10:46.607 in Ping 
18:10:51.611 in Ping 
18:10:56.451 in Interrupt 42
18:10:56.615 in Ping 
18:10:57.457 fetch POST /api/observability/v1/traces
18:10:57.457 fetched POST /api/observability/v1/traces 204
18:11:01.619 in Ping 
18:11:06.452 rpc 43 server.reportClientActivity
18:11:06.623 in Ping 
18:11:07.455 fetch POST /api/observability/v1/traces
18:11:07.455 fetched POST /api/observability/v1/traces 204
18:11:11.627 in Ping 
18:11:16.631 in Ping 
18:11:21.455 in Interrupt 43
18:11:21.635 in Ping 
18:11:22.459 fetch POST /api/observability/v1/traces
18:11:22.459 fetched POST /api/observability/v1/traces 204
18:11:26.639 in Ping 
18:11:31.457 rpc 44 server.reportClientActivity
18:11:31.643 in Ping 
18:11:32.459 fetch POST /api/observability/v1/traces
18:11:32.460 fetched POST /api/observability/v1/traces 204
18:11:36.647 in Ping 
18:11:41.651 in Ping 
18:11:46.459 in Interrupt 44
18:11:46.655 in Ping 
18:11:47.463 fetch POST /api/observability/v1/traces
18:11:47.463 fetched POST /api/observability/v1/traces 204
18:11:51.659 in Ping 
18:11:56.460 rpc 45 server.reportClientActivity
18:11:56.663 in Ping 
18:11:57.463 fetch POST /api/observability/v1/traces
18:11:57.463 fetched POST /api/observability/v1/traces 204
18:12:01.667 in Ping 
18:12:06.671 in Ping 
18:12:11.463 in Interrupt 45
18:12:11.675 in Ping 
18:12:12.468 fetch POST /api/observability/v1/traces
18:12:12.468 fetched POST /api/observability/v1/traces 204
18:12:16.679 in Ping 
18:12:21.464 rpc 46 server.reportClientActivity
18:12:21.683 in Ping 
18:12:22.467 fetch POST /api/observability/v1/traces
18:12:22.467 fetched POST /api/observability/v1/traces 204
18:12:26.687 in Ping 
18:12:31.691 in Ping 
18:12:36.467 in Interrupt 46
18:12:36.695 in Ping 
18:12:37.471 fetch POST /api/observability/v1/traces
18:12:37.471 fetched POST /api/observability/v1/traces 204
18:12:41.699 in Ping 
18:12:46.468 rpc 47 server.reportClientActivity
18:12:46.703 in Ping 
18:12:47.471 fetch POST /api/observability/v1/traces
18:12:47.471 fetched POST /api/observability/v1/traces 204
18:12:51.707 in Ping 
18:12:56.711 in Ping 
18:13:01.471 in Interrupt 47
18:13:01.716 in Ping 
18:13:02.475 fetch POST /api/observability/v1/traces
18:13:02.475 fetched POST /api/observability/v1/traces 204
18:13:06.719 in Ping 
18:13:11.472 rpc 48 server.reportClientActivity
18:13:11.723 in Ping 
18:13:12.476 fetch POST /api/observability/v1/traces
18:13:12.476 fetched POST /api/observability/v1/traces 204
18:13:16.727 in Ping 
18:13:21.731 in Ping 
18:13:26.475 in Interrupt 48
18:13:26.735 in Ping 
18:13:27.480 fetch POST /api/observability/v1/traces
18:13:27.480 fetched POST /api/observability/v1/traces 204
18:13:31.739 in Ping 
18:13:36.476 rpc 49 server.reportClientActivity
18:13:36.743 in Ping 
18:13:37.480 fetch POST /api/observability/v1/traces
18:13:37.480 fetched POST /api/observability/v1/traces 204
18:13:41.747 in Ping 
18:13:46.751 in Ping 
18:13:51.479 in Interrupt 49
18:13:51.755 in Ping 
18:13:52.483 fetch POST /api/observability/v1/traces
18:13:52.483 fetched POST /api/observability/v1/traces 204
18:13:56.759 in Ping 
18:14:01.480 rpc 50 server.reportClientActivity
18:14:01.763 in Ping 
18:14:02.483 fetch POST /api/observability/v1/traces
18:14:02.483 fetched POST /api/observability/v1/traces 204
18:14:06.767 in Ping 
18:14:11.771 in Ping 
18:14:16.484 in Interrupt 50
18:14:16.775 in Ping 
18:14:17.488 fetch POST /api/observability/v1/traces
18:14:17.488 fetched POST /api/observability/v1/traces 204
18:14:21.779 in Ping 
18:14:26.484 rpc 51 server.reportClientActivity
18:14:26.783 in Ping 
18:14:27.487 fetch POST /api/observability/v1/traces
18:14:27.488 fetched POST /api/observability/v1/traces 204
18:14:31.787 in Ping 
18:14:36.791 in Ping 
18:14:41.488 in Interrupt 51
18:14:41.795 in Ping 
18:14:42.495 fetch POST /api/observability/v1/traces
18:14:42.495 fetched POST /api/observability/v1/traces 204
18:14:46.799 in Ping 
18:14:51.488 rpc 52 server.reportClientActivity
18:14:51.803 in Ping 
18:14:52.491 fetch POST /api/observability/v1/traces
18:14:52.491 fetched POST /api/observability/v1/traces 204
18:14:56.807 in Ping 
18:15:01.811 in Ping 
18:15:06.491 in Interrupt 52
18:15:06.815 in Ping 
18:15:07.496 fetch POST /api/observability/v1/traces
18:15:07.496 fetched POST /api/observability/v1/traces 204
18:15:11.819 in Ping 
18:15:16.492 rpc 53 server.reportClientActivity
18:15:16.823 in Ping 
18:15:17.495 fetch POST /api/observability/v1/traces
18:15:17.495 fetched POST /api/observability/v1/traces 204
18:15:21.827 in Ping 
18:15:26.831 in Ping 
18:15:31.495 in Interrupt 53
18:15:31.835 in Ping 
18:15:32.499 fetch POST /api/observability/v1/traces
18:15:32.499 fetched POST /api/observability/v1/traces 204
18:15:36.839 in Ping 
