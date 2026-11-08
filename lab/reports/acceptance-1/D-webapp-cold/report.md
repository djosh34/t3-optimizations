# D, webapp, cold start, seed 1

Profile D (graded): worst observed EDGE, inspired envelope. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 413 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.7s |
| Prompt confirmation | 290.7s |
| Prompt reached the agent | 285.1s |
| Time to first text | 322.6s |
| Whole reply shown | 409.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 36.8s / 0.0s / 15.8s |
| Longest silence | 322.6s |
| Approval shown after it was asked | 48.5s |
| Bytes on the wire down / up | 756136 / 417273 |
| Bytes the app received / sent | 365224 / 125322 |

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
| 290 | 4.8 | 0 | 197 |
| 300 | 14.8 | 0 | 597 |
| 310 | 24.8 | 0 | 1000 |
| 320 | 34.8 | 0 | 1077 |
| 330 | 0.0 | 1077 | 1077 |
| 340 | 0.0 | 1077 | 1077 |
| 350 | 0.0 | 1077 | 1077 |
| 360 | 0.0 | 1077 | 1077 |
| 370 | 0.0 | 1077 | 1077 |
| 380 | 0.0 | 1077 | 1077 |
| 390 | 7.3 | 1077 | 1367 |
| 400 | 13.2 | 1237 | 1596 |
| 409 | 14.3 | 1524 | 1596 |

## Driver notes

- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2184,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2184,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2676,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3168,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3168,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3661,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3661,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3661,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4154,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4154,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4647,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4647,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4813,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5306,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5306,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5799,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5799,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5799,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6292,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6292,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6785,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":154790,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3564,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6785,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":67758,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5940,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6785,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":67758,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5940,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7278,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":67758,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5940,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7278,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":67758,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5940,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7771,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":67758,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5940,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7771,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc open","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":14,"sentWireBytes":16632,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7771,"inflightPackets":9,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108","thread":"951/951","messages":"550/550 456/456 190/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150918722,"silences":0,"longestSilenceMs":351,"longestRecoveredGapMs":351},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":10228.265625,"minRttMs":10089,"sentPackets":22,"sentWireBytes":25371,"receivedPackets":22,"receivedWireBytes":25928,"lastHeardMs":285218,"unsyncedBytes":7586,"inflightPackets":14,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 239/849","thread":"951/951","messages":"550/550 456/456 259/901","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150927325,"silences":0,"longestSilenceMs":1542,"longestRecoveredGapMs":1542},"stats":{"sendRate":3000,"deliveredRate":951.7051956969559,"loss":0.41666666666666663,"srttMs":10823.255701124668,"minRttMs":9515,"sentPackets":29,"sentWireBytes":31365,"receivedPackets":56,"receivedWireBytes":63281,"lastHeardMs":293821,"unsyncedBytes":6421,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849","thread":"951/951","messages":"550/550 456/456 385/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150939326,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":951.7051956969559,"loss":0.41666666666666663,"srttMs":10823.255701124668,"minRttMs":9515,"sentPackets":29,"sentWireBytes":31365,"receivedPackets":121,"receivedWireBytes":101589,"lastHeardMs":305822,"unsyncedBytes":6913,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849","thread":"951/951","messages":"550/550 456/456 646/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150948888,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":951.7051956969559,"loss":0.41666666666666663,"srttMs":10823.255701124668,"minRttMs":9515,"sentPackets":41,"sentWireBytes":34791,"receivedPackets":164,"receivedWireBytes":127590,"lastHeardMs":315384,"unsyncedBytes":6913,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 339/1554","thread":"951/951","messages":"550/550 456/456 646/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150959335,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":705.3874620307507,"loss":0.45166666666666666,"srttMs":10588.336396173574,"minRttMs":9515,"sentPackets":69,"sentWireBytes":42778,"receivedPackets":190,"receivedWireBytes":144326,"lastHeardMs":325831,"unsyncedBytes":6177,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1142/1554","thread":"951/951","messages":"550/550 456/456 646/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150968886,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":575.1064506942528,"loss":0.5161666666666667,"srttMs":10322.401140405344,"minRttMs":8984,"sentPackets":95,"sentWireBytes":50094,"receivedPackets":217,"receivedWireBytes":159890,"lastHeardMs":335382,"unsyncedBytes":5873,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 891/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150979014,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":866.641053073778,"loss":0.39138320512820507,"srttMs":10490.896548925799,"minRttMs":8984,"sentPackets":133,"sentWireBytes":60775,"receivedPackets":267,"receivedWireBytes":187149,"lastHeardMs":345510,"unsyncedBytes":3290,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 891/901","attempts":"662/662 662/662 15/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"308/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794150986919,"silences":1,"longestSilenceMs":6431,"longestRecoveredGapMs":6431},"stats":{"sendRate":3000,"deliveredRate":762.9487131928519,"loss":0.19177777051282047,"srttMs":10427.744870425253,"minRttMs":8984,"sentPackets":159,"sentWireBytes":68052,"receivedPackets":312,"receivedWireBytes":207325,"lastHeardMs":353415,"unsyncedBytes":2303,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014 342/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 34/64","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 901/901","attempts":"662/662 662/662 638/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794151000881,"silences":2,"longestSilenceMs":6920,"longestRecoveredGapMs":6920},"stats":{"sendRate":3000,"deliveredRate":738.3913719622691,"loss":0.1842444393589743,"srttMs":10612.526761622095,"minRttMs":8984,"sentPackets":178,"sentWireBytes":73487,"receivedPackets":376,"receivedWireBytes":232398,"lastHeardMs":367377,"unsyncedBytes":1314,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 901/901","attempts":"662/662 662/662 638/638","nodes":"809/809 1025/1025 809/809","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794151011771,"silences":2,"longestSilenceMs":6920,"longestRecoveredGapMs":6920},"stats":{"sendRate":3000,"deliveredRate":545.828664206341,"loss":0.380971107551282,"srttMs":11203.085916419333,"minRttMs":8984,"sentPackets":197,"sentWireBytes":80224,"receivedPackets":422,"receivedWireBytes":253706,"lastHeardMs":378267,"unsyncedBytes":1806,"inflightPackets":91,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 901/901 550/550","attempts":"662/662 662/662 638/638","nodes":"809/809 1025/1025 809/809 1293/1377 777/965 515/1025","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794151022249,"silences":2,"longestSilenceMs":6920,"longestRecoveredGapMs":6920},"stats":{"sendRate":3000,"deliveredRate":1128.9127286294238,"loss":0.3236323644392586,"srttMs":10981.24408668593,"minRttMs":8984,"sentPackets":222,"sentWireBytes":91682,"receivedPackets":507,"receivedWireBytes":290000,"lastHeardMs":388744,"unsyncedBytes":1385,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"runs":"971/971 1014/1014 839/839","assistant":"32/32 21/21 1340/1340 321/444","user":"10/10 9/9 64/64","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108","thread":"951/951","messages":"550/550 456/456 901/901 550/550","attempts":"662/662 662/662 638/638","nodes":"809/809 1025/1025 809/809 1293/1377 965/965 515/1025","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794151032560,"silences":2,"longestSilenceMs":6920,"longestRecoveredGapMs":6920},"stats":{"sendRate":3000,"deliveredRate":1385.047765594294,"loss":0.23357985857523667,"srttMs":11291.762680720984,"minRttMs":9021,"sentPackets":260,"sentWireBytes":106716,"receivedPackets":572,"receivedWireBytes":323731,"lastHeardMs":399056,"unsyncedBytes":512,"inflightPackets":85,"duplicateBytes":0}}
- Scroll to end shows
- webapp {"objects":{"runs":"971/971 1014/1014 391/971","assistant":"32/32 21/21 1340/1340 708/723","user":"10/10 9/9 64/64","checkpointScopes":"726/726","turnItems":"1299/1299 1058/1058 860/860 870/870 1058/1058 1108/1108 1108/1108 849/849 1554/1554 932/932 1108/1108 439/1108","thread":"951/951","messages":"550/550 456/456 901/901 550/550","attempts":"662/662 662/662 638/638","nodes":"809/809 1025/1025 809/809 1293/1377 965/965 592/1025","checkpoints":"613/613 613/613 779/779","providerThreads":"810/810","providerSessions":"2688/2688","providerTurns":"778/778","frame":"1032/1032","runtimeRequests":"401/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":5,"iceRestarts":0,"newConnections":2,"handshakeMs":47245,"lastDataAt":1794151043894,"silences":2,"longestSilenceMs":6920,"longestRecoveredGapMs":6920},"stats":{"sendRate":3000,"deliveredRate":908.4964957472105,"loss":0.3602391036125182,"srttMs":10880.396529824146,"minRttMs":9021,"sentPackets":293,"sentWireBytes":121033,"receivedPackets":652,"receivedWireBytes":356105,"lastHeardMs":410390,"unsyncedBytes":492,"inflightPackets":92,"duplicateBytes":0}}
- shim trace
15:10:34.190 fetch GET /api/auth/session
15:10:34.194 fetched GET /api/auth/session 200
15:10:34.336 fetch GET /.well-known/t3/environment
15:10:34.338 fetched GET /.well-known/t3/environment 200
15:10:34.359 fetch GET /.well-known/t3/environment
15:10:34.360 fetched GET /.well-known/t3/environment 200
15:10:34.373 rpc 0 subscribeServerConfig
15:10:34.374 chunk 0 38690 bytes
15:10:34.403 in Ack 0
15:10:34.406 rpc 1 subscribeServerLifecycle
15:10:34.407 rpc 2 subscribeServerLifecycle
15:10:34.411 fetch GET /api/auth/session
15:10:34.412 fetched GET /api/auth/session 200
15:10:34.413 rpc 3 subscribeDeviceState
15:10:34.416 rpc 4 preview.reportProfiles
15:10:34.547 rpc 5 subscribeProjectClones
15:10:34.628 rpc 6 server.reportClientActivity
15:10:34.634 rpc 7 orchestration.subscribeThread
15:10:34.635 chunk 7 32012 bytes
15:10:34.638 in Ack 7
15:10:34.648 fetch GET /api/orchestration/shell
15:10:34.720 rpc 8 server.refreshProviders
15:10:34.856 rpc 9 orchestration.dispatchCommand thread.visit
15:10:34.952 fetched GET /api/orchestration/shell 200
15:10:34.953 rpc 10 subscribeTerminalMetadata
15:10:34.954 rpc 11 subscribePreviewEvents
15:10:34.956 rpc 12 subscribeVcsStatus
15:10:34.959 rpc 13 orchestration.subscribeShell
15:10:34.959 chunk 13 2847 bytes
15:10:34.961 rpc 14 assets.createUrl
15:10:34.962 rpc 15 preview.list
15:10:34.962 rpc 16 projects.readFile
15:10:34.963 rpc 17 assets.createUrl
15:10:35.033 in Ack 13
15:10:35.331 fetch POST /api/observability/v1/traces
15:10:35.331 fetched POST /api/observability/v1/traces 204
15:10:36.641 fetch POST /api/observability/v1/traces
15:10:36.641 fetched POST /api/observability/v1/traces 204
15:10:39.368 in Ping 
15:10:44.369 in Ping 
15:10:49.369 in Ping 
15:10:49.630 in Interrupt 6
15:10:49.882 rpc 18 server.reportClientActivity
15:10:50.633 fetch POST /api/observability/v1/traces
15:10:50.633 fetched POST /api/observability/v1/traces 204
15:10:54.370 in Ping 
15:10:59.370 in Ping 
15:11:04.370 in Ping 
15:11:04.883 in Interrupt 18
15:11:05.135 rpc 19 server.reportClientActivity
15:11:05.884 fetch POST /api/observability/v1/traces
15:11:05.884 fetched POST /api/observability/v1/traces 204
15:11:09.370 in Ping 
15:11:14.372 in Ping 
15:11:19.372 in Ping 
15:11:20.137 in Interrupt 19
15:11:21.138 fetch POST /api/observability/v1/traces
15:11:21.138 fetched POST /api/observability/v1/traces 204
15:11:24.372 in Ping 
15:11:24.590 rpc 20 server.reportClientActivity
15:11:25.590 fetch POST /api/observability/v1/traces
15:11:25.590 fetched POST /api/observability/v1/traces 204
15:11:29.374 in Ping 
15:11:34.379 in Ping 
15:11:39.381 in Ping 
15:11:39.590 in Interrupt 20
15:11:40.591 fetch POST /api/observability/v1/traces
15:11:40.591 fetched POST /api/observability/v1/traces 204
15:11:44.385 in Ping 
15:11:49.389 in Ping 
15:11:49.589 rpc 21 server.reportClientActivity
15:11:50.589 fetch POST /api/observability/v1/traces
15:11:50.590 fetched POST /api/observability/v1/traces 204
15:11:54.393 in Ping 
15:11:59.397 in Ping 
15:12:04.401 in Ping 
15:12:04.590 in Interrupt 21
15:12:05.591 fetch POST /api/observability/v1/traces
15:12:05.592 fetched POST /api/observability/v1/traces 204
15:12:09.405 in Ping 
15:12:14.410 in Ping 
15:12:14.597 rpc 22 server.reportClientActivity
15:12:15.597 fetch POST /api/observability/v1/traces
15:12:15.597 fetched POST /api/observability/v1/traces 204
15:12:19.413 in Ping 
15:12:24.417 in Ping 
15:12:29.422 in Ping 
15:12:29.598 in Interrupt 22
15:12:30.599 fetch POST /api/observability/v1/traces
15:12:30.599 fetched POST /api/observability/v1/traces 204
15:12:34.426 in Ping 
15:12:39.430 in Ping 
15:12:39.590 rpc 23 server.reportClientActivity
15:12:40.591 fetch POST /api/observability/v1/traces
15:12:40.591 fetched POST /api/observability/v1/traces 204
15:12:44.436 in Ping 
15:12:49.437 in Ping 
15:12:54.441 in Ping 
15:12:54.591 in Interrupt 23
15:12:55.592 fetch POST /api/observability/v1/traces
15:12:55.592 fetched POST /api/observability/v1/traces 204
15:12:59.445 in Ping 
15:13:04.449 in Ping 
15:13:04.591 rpc 24 server.reportClientActivity
15:13:05.591 fetch POST /api/observability/v1/traces
15:13:05.592 fetched POST /api/observability/v1/traces 204
15:13:09.455 in Ping 
15:13:14.457 in Ping 
15:13:19.461 in Ping 
15:13:19.592 in Interrupt 24
15:13:20.594 fetch POST /api/observability/v1/traces
15:13:20.594 fetched POST /api/observability/v1/traces 204
15:13:24.466 in Ping 
15:13:29.469 in Ping 
15:13:29.594 rpc 25 server.reportClientActivity
15:13:30.595 fetch POST /api/observability/v1/traces
15:13:30.595 fetched POST /api/observability/v1/traces 204
15:13:34.474 in Ping 
15:13:39.478 in Ping 
15:13:44.481 in Ping 
15:13:44.595 in Interrupt 25
15:13:45.596 fetch POST /api/observability/v1/traces
15:13:45.596 fetched POST /api/observability/v1/traces 204
15:13:49.485 in Ping 
15:13:54.492 in Ping 
15:13:54.593 rpc 26 server.reportClientActivity
15:13:55.593 fetch POST /api/observability/v1/traces
15:13:55.593 fetched POST /api/observability/v1/traces 204
15:13:59.501 in Ping 
15:14:04.501 in Ping 
15:14:09.505 in Ping 
15:14:09.594 in Interrupt 26
15:14:10.598 fetch POST /api/observability/v1/traces
15:14:10.598 fetched POST /api/observability/v1/traces 204
15:14:14.510 in Ping 
15:14:19.514 in Ping 
15:14:19.592 rpc 27 server.reportClientActivity
15:14:20.594 fetch POST /api/observability/v1/traces
15:14:20.594 fetched POST /api/observability/v1/traces 204
15:14:24.517 in Ping 
15:14:29.521 in Ping 
15:14:34.525 in Ping 
15:14:34.594 in Interrupt 27
15:14:35.598 fetch POST /api/observability/v1/traces
15:14:35.598 fetched POST /api/observability/v1/traces 204
15:14:39.530 in Ping 
15:14:44.535 in Ping 
15:14:44.602 rpc 28 server.reportClientActivity
15:14:45.614 fetch POST /api/observability/v1/traces
15:14:45.614 fetched POST /api/observability/v1/traces 204
15:14:49.537 in Ping 
15:14:54.544 in Ping 
15:14:59.545 in Ping 
15:14:59.606 in Interrupt 28
15:15:00.610 fetch POST /api/observability/v1/traces
15:15:00.610 fetched POST /api/observability/v1/traces 204
15:15:04.549 in Ping 
15:15:09.553 in Ping 
15:15:09.595 rpc 29 server.reportClientActivity
15:15:10.598 fetch POST /api/observability/v1/traces
15:15:10.599 fetched POST /api/observability/v1/traces 204
15:15:14.558 in Ping 
15:15:14.902 exit 9 {"_tag":"Success","value":{"sequence":69}}
15:15:14.930 rpc 30 orchestration.dispatchCommand message.dispatch
15:15:15.078 chunk 1 3864 bytes
15:15:15.079 in Ack 1
15:15:15.912 fetch POST /api/observability/v1/traces
15:15:15.912 fetched POST /api/observability/v1/traces 204
15:15:19.561 in Ping 
15:15:24.565 in Ping 
15:15:24.598 in Interrupt 29
15:15:25.603 fetch POST /api/observability/v1/traces
15:15:25.603 fetched POST /api/observability/v1/traces 204
15:15:27.034 chunk 7 31988 bytes
15:15:27.034 chunk 13 2823 bytes
15:15:27.035 exit 30 {"_tag":"Success","value":{"sequence":76}}
15:15:27.035 exit 15 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c84e6f7a-db80-41ab-9d73-17855884fb9a","sessions":[]}}
15:15:27.035 exit 16 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:15:27.045 in Ack 7
15:15:27.081 in Ack 13
15:15:28.051 fetch POST /api/observability/v1/traces
15:15:28.051 fetched POST /api/observability/v1/traces 204
15:15:29.570 in Ping 
15:15:34.267 chunk 13 944 bytes
15:15:34.267 in Ack 13
15:15:34.573 in Ping 
15:15:34.667 rpc 31 server.reportClientActivity
15:15:35.269 fetch POST /api/observability/v1/traces
15:15:35.269 fetched POST /api/observability/v1/traces 204
15:15:38.134 fetch POST /api/observability/v1/traces
15:15:38.134 fetched POST /api/observability/v1/traces 204
15:15:38.886 chunk 2 3864 bytes
15:15:38.887 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:10:11.654Z","configurableProviders":false,"con
15:15:38.887 in Ack 2
15:15:39.577 in Ping 
15:15:39.888 fetch POST /api/observability/v1/traces
15:15:39.888 fetched POST /api/observability/v1/traces 204
15:15:40.443 chunk 12 1427 bytes
15:15:40.443 exit 14 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:15:40.443 exit 17 {"_tag":"Success","value":{"expiresAt":1794154521359,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
15:15:40.444 in Ack 12
15:15:41.448 fetch POST /api/observability/v1/traces
15:15:41.448 fetched POST /api/observability/v1/traces 204
15:15:44.582 in Ping 
15:15:49.585 in Ping 
15:15:49.670 in Interrupt 31
15:15:50.219 chunk 7 34466 bytes
15:15:50.222 in Ack 7
15:15:50.472 chunk 7 34506 bytes
15:15:50.473 in Ack 7
15:15:50.675 fetch POST /api/observability/v1/traces
15:15:50.675 fetched POST /api/observability/v1/traces 204
15:15:51.865 chunk 7 34536 bytes
15:15:51.867 in Ack 7
15:15:52.116 chunk 7 34566 bytes
15:15:52.117 in Ack 7
15:15:52.366 chunk 7 34676 bytes
15:15:52.368 in Ack 7
15:15:52.616 chunk 7 34706 bytes
15:15:52.617 in Ack 7
15:15:52.866 chunk 7 34736 bytes
15:15:52.867 in Ack 7
15:15:52.898 fetch POST /api/observability/v1/traces
15:15:52.898 fetched POST /api/observability/v1/traces 204
15:15:53.660 chunk 7 34738 bytes
15:15:53.661 in Ack 7
15:15:54.590 in Ping 
15:15:54.664 fetch POST /api/observability/v1/traces
15:15:54.664 fetched POST /api/observability/v1/traces 204
15:15:55.138 chunk 7 34764 bytes
15:15:55.139 in Ack 7
15:15:55.388 chunk 7 34806 bytes
15:15:55.389 in Ack 7
15:15:56.140 fetch POST /api/observability/v1/traces
15:15:56.140 fetched POST /api/observability/v1/traces 204
15:15:58.519 chunk 7 35132 bytes
15:15:58.523 in Ack 7
15:15:58.770 chunk 7 35852 bytes
15:15:58.771 in Ack 7
15:15:59.020 chunk 7 37122 bytes
15:15:59.021 in Ack 7
15:15:59.524 fetch POST /api/observability/v1/traces
15:15:59.524 fetched POST /api/observability/v1/traces 204
15:15:59.593 in Ping 
15:15:59.603 rpc 32 server.reportClientActivity
15:16:00.606 fetch POST /api/observability/v1/traces
15:16:00.606 fetched POST /api/observability/v1/traces 204
15:16:04.597 in Ping 
15:16:09.601 in Ping 
15:16:13.104 chunk 7 40636 bytes
15:16:13.106 in Ack 7
15:16:14.107 fetch POST /api/observability/v1/traces
15:16:14.108 fetched POST /api/observability/v1/traces 204
15:16:14.607 in Interrupt 32
15:16:14.607 in Ping 
15:16:15.610 fetch POST /api/observability/v1/traces
15:16:15.611 fetched POST /api/observability/v1/traces 204
15:16:17.514 chunk 7 42725 bytes
15:16:17.516 in Ack 7
15:16:18.519 fetch POST /api/observability/v1/traces
15:16:18.519 fetched POST /api/observability/v1/traces 204
15:16:18.570 chunk 7 42727 bytes
15:16:18.571 in Ack 7
15:16:19.576 fetch POST /api/observability/v1/traces
15:16:19.576 fetched POST /api/observability/v1/traces 204
15:16:19.609 in Ping 
15:16:24.217 fetch POST /api/observability/v1/traces
15:16:24.217 fetched POST /api/observability/v1/traces 204
15:16:24.608 rpc 33 server.reportClientActivity
15:16:24.613 in Ping 
15:16:25.610 fetch POST /api/observability/v1/traces
15:16:25.610 fetched POST /api/observability/v1/traces 204
15:16:29.617 in Ping 
15:16:34.621 in Ping 
15:16:35.238 chunk 7 43638 bytes
15:16:35.239 in Ack 7
15:16:35.989 chunk 13 3197 bytes
15:16:35.990 in Ack 13
15:16:35.993 rpc 34 orchestration.dispatchCommand thread.visit
15:16:36.243 fetch POST /api/observability/v1/traces
15:16:36.243 fetched POST /api/observability/v1/traces 204
15:16:36.765 chunk 7 44649 bytes
15:16:36.767 in Ack 7
15:16:37.768 fetch POST /api/observability/v1/traces
15:16:37.768 fetched POST /api/observability/v1/traces 204
15:16:38.423 chunk 7 45288 bytes
15:16:38.425 in Ack 7
15:16:39.426 fetch POST /api/observability/v1/traces
15:16:39.426 fetched POST /api/observability/v1/traces 204
15:16:39.614 in Interrupt 33
15:16:39.625 in Ping 
15:16:40.622 fetch POST /api/observability/v1/traces
15:16:40.622 fetched POST /api/observability/v1/traces 204
15:16:44.629 in Ping 
15:16:46.264 fetch POST /api/observability/v1/traces
15:16:46.264 fetched POST /api/observability/v1/traces 204
15:16:46.691 chunk 7 47292 bytes
15:16:46.694 in Ack 7
15:16:47.696 fetch POST /api/observability/v1/traces
15:16:47.696 fetched POST /api/observability/v1/traces 204
15:16:49.611 rpc 35 server.reportClientActivity
15:16:49.633 in Ping 
15:16:50.614 fetch POST /api/observability/v1/traces
15:16:50.614 fetched POST /api/observability/v1/traces 204
15:16:51.513 chunk 7 48132 bytes
15:16:51.515 in Ack 7
15:16:52.517 fetch POST /api/observability/v1/traces
15:16:52.517 fetched POST /api/observability/v1/traces 204
15:16:53.154 chunk 3 876 bytes
15:16:53.155 in Ack 3
15:16:53.767 exit 34 {"_tag":"Success","value":{"sequence":94}}
15:16:53.771 rpc 36 orchestration.dispatchCommand runtime-request.respond
15:16:54.638 in Ping 
15:16:54.769 fetch POST /api/observability/v1/traces
15:16:54.769 fetched POST /api/observability/v1/traces 204
15:16:56.267 fetch POST /api/observability/v1/traces
15:16:56.267 fetched POST /api/observability/v1/traces 204
15:16:56.841 chunk 7 50027 bytes
15:16:56.843 in Ack 7
15:16:57.843 fetch POST /api/observability/v1/traces
15:16:57.843 fetched POST /api/observability/v1/traces 204
15:16:59.641 in Ping 
15:17:03.034 exit 4 {"_tag":"Success","value":null}
15:17:03.049 rpc 37 preview.reportProfiles
15:17:03.649 chunk 7 50993 bytes
15:17:03.650 in Ack 7
15:17:03.875 chunk 5 4 bytes
15:17:03.875 in Ack 5
15:17:04.039 fetch POST /api/observability/v1/traces
15:17:04.040 fetched POST /api/observability/v1/traces 204
15:17:04.157 chunk 13 944 bytes
15:17:04.157 in Ack 13
15:17:04.326 exit 36 {"_tag":"Success","value":{"sequence":97}}
15:17:04.407 chunk 7 53482 bytes
15:17:04.411 in Ack 7
15:17:04.628 in Interrupt 35
15:17:04.645 in Ping 
15:17:04.849 chunk 7 53514 bytes
15:17:04.851 in Ack 7
15:17:05.102 chunk 7 53524 bytes
15:17:05.104 in Ack 7
15:17:05.172 fetch POST /api/observability/v1/traces
15:17:05.172 fetched POST /api/observability/v1/traces 204
15:17:05.181 rpc 38 orchestration.dispatchCommand runtime-request.respond
15:17:05.351 chunk 7 53554 bytes
15:17:05.354 in Ack 7
15:17:05.625 chunk 7 53574 bytes
15:17:05.627 in Ack 7
15:17:05.876 chunk 7 53584 bytes
15:17:05.877 in Ack 7
15:17:06.182 fetch POST /api/observability/v1/traces
15:17:06.182 fetched POST /api/observability/v1/traces 204
15:17:06.351 chunk 7 53594 bytes
15:17:06.352 in Ack 7
15:17:06.607 chunk 7 53624 bytes
15:17:06.612 in Ack 7
15:17:06.857 chunk 7 53674 bytes
15:17:06.859 in Ack 7
15:17:07.108 chunk 7 53684 bytes
15:17:07.109 in Ack 7
15:17:07.354 fetch POST /api/observability/v1/traces
15:17:07.354 fetched POST /api/observability/v1/traces 204
15:17:07.559 chunk 7 53706 bytes
15:17:07.560 in Ack 7
15:17:07.810 chunk 7 53716 bytes
15:17:07.811 in Ack 7
15:17:08.070 chunk 7 53736 bytes
15:17:08.073 in Ack 7
15:17:08.323 chunk 7 53768 bytes
15:17:08.325 in Ack 7
15:17:08.562 fetch POST /api/observability/v1/traces
15:17:08.562 fetched POST /api/observability/v1/traces 204
15:17:08.575 chunk 7 53810 bytes
15:17:08.577 in Ack 7
15:17:09.578 fetch POST /api/observability/v1/traces
15:17:09.578 fetched POST /api/observability/v1/traces 204
15:17:09.649 in Ping 
15:17:10.230 chunk 7 53898 bytes
15:17:10.232 in Ack 7
15:17:10.481 chunk 7 53908 bytes
15:17:10.483 in Ack 7
15:17:10.733 chunk 7 53948 bytes
15:17:10.734 in Ack 7
15:17:10.984 chunk 7 53958 bytes
15:17:10.987 in Ack 7
15:17:11.232 fetch POST /api/observability/v1/traces
15:17:11.233 fetched POST /api/observability/v1/traces 204
15:17:11.233 chunk 7 53978 bytes
15:17:11.234 in Ack 7
15:17:11.484 chunk 7 53988 bytes
15:17:11.485 in Ack 7
15:17:11.734 chunk 7 54008 bytes
15:17:11.736 in Ack 7
15:17:11.987 chunk 7 54046 bytes
15:17:11.988 in Ack 7
15:17:12.237 fetch POST /api/observability/v1/traces
15:17:12.237 fetched POST /api/observability/v1/traces 204
15:17:12.238 chunk 7 54066 bytes
15:17:12.239 in Ack 7
15:17:12.489 chunk 7 54086 bytes
15:17:12.490 in Ack 7
15:17:12.739 chunk 7 54096 bytes
15:17:12.741 in Ack 7
15:17:13.050 chunk 7 54126 bytes
15:17:13.053 in Ack 7
15:17:13.247 fetch POST /api/observability/v1/traces
15:17:13.247 fetched POST /api/observability/v1/traces 204
15:17:13.303 chunk 7 54160 bytes
15:17:13.305 in Ack 7
15:17:13.554 chunk 7 54196 bytes
15:17:13.557 in Ack 7
15:17:14.306 fetch POST /api/observability/v1/traces
15:17:14.306 fetched POST /api/observability/v1/traces 204
15:17:14.627 rpc 39 server.reportClientActivity
15:17:14.654 in Ping 
15:17:14.705 chunk 7 54272 bytes
15:17:14.709 in Ack 7
15:17:14.954 chunk 7 54310 bytes
15:17:14.959 in Ack 7
15:17:15.204 chunk 7 54330 bytes
15:17:15.205 in Ack 7
15:17:15.289 exit 6 {"_tag":"Success","value":null}
15:17:15.457 chunk 7 54350 bytes
15:17:15.459 in Ack 7
15:17:15.626 fetch POST /api/observability/v1/traces
15:17:15.626 fetched POST /api/observability/v1/traces 204
15:17:15.780 chunk 7 54362 bytes
15:17:15.781 in Ack 7
15:17:16.031 chunk 7 53367 bytes
15:17:16.032 in Ack 7
15:17:16.435 chunk 7 53377 bytes
15:17:16.436 in Ack 7
15:17:16.686 chunk 7 53407 bytes
15:17:16.688 in Ack 7
15:17:16.782 fetch POST /api/observability/v1/traces
15:17:16.783 fetched POST /api/observability/v1/traces 204
15:17:17.439 chunk 7 53417 bytes
15:17:17.441 in Ack 7
15:17:17.689 chunk 7 53471 bytes
15:17:17.691 in Ack 7
15:17:17.940 chunk 7 53567 bytes
15:17:17.941 in Ack 7
15:17:18.190 chunk 7 53609 bytes
15:17:18.191 in Ack 7
15:17:18.452 chunk 7 53837 bytes
15:17:18.453 fetch POST /api/observability/v1/traces
15:17:18.453 fetched POST /api/observability/v1/traces 204
15:17:18.454 in Ack 7
15:17:18.703 chunk 7 53893 bytes
15:17:18.705 in Ack 7
15:17:19.203 chunk 7 53053 bytes
15:17:19.204 in Ack 7
15:17:19.455 fetch POST /api/observability/v1/traces
15:17:19.455 fetched POST /api/observability/v1/traces 204
15:17:19.657 in Ping 
15:17:24.045 chunk 7 52414 bytes
15:17:24.046 in Ack 7
15:17:24.661 in Ping 
15:17:24.893 chunk 10 36 bytes
15:17:24.894 in Ack 10
15:17:25.047 fetch POST /api/observability/v1/traces
15:17:25.047 fetched POST /api/observability/v1/traces 204
15:17:25.341 chunk 7 52450 bytes
15:17:25.342 in Ack 7
15:17:26.372 fetch POST /api/observability/v1/traces
15:17:26.372 fetched POST /api/observability/v1/traces 204
