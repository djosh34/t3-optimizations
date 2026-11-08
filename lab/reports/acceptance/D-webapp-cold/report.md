# D, webapp, cold start, seed 1

Profile D (graded): worst observed EDGE, inspired envelope. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 253 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.3s |
| Prompt confirmation | 168.3s |
| Prompt reached the agent | 163.0s |
| Time to first text | 168.0s |
| Whole reply shown | 250.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 37.2s / 0.0s / 27.2s |
| Longest silence | 168.0s |
| Approval shown after it was asked | 16.8s |
| Bytes on the wire down / up | 536923 / 271872 |
| Bytes the app received / sent | 242853 / 85276 |

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
| 170 | 6.2 | 28 | 283 |
| 180 | 16.2 | 28 | 681 |
| 190 | 26.2 | 28 | 1077 |
| 200 | 0.0 | 1077 | 1077 |
| 210 | 0.0 | 1077 | 1077 |
| 220 | 7.2 | 1077 | 1367 |
| 230 | 17.2 | 1077 | 1596 |
| 240 | 27.2 | 1077 | 1596 |
| 250 | 37.2 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2675,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2675,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2675,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3167,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3167,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3660,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3660,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3660,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4153,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":90534,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3549,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4153,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":90534,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3549,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4646,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":90534,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3549,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4646,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":90534,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3549,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4812,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":90534,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3549,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5305,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc open","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":10,"sentWireBytes":11830,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5305,"inflightPackets":7,"duplicateBytes":0}}
- prompt sent
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159792379,"silences":0,"longestSilenceMs":494,"longestRecoveredGapMs":494},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":9973.921875,"minRttMs":9805,"sentPackets":19,"sentWireBytes":20743,"receivedPackets":18,"receivedWireBytes":21396,"lastHeardMs":153782,"unsyncedBytes":4634,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159802299,"silences":0,"longestSilenceMs":751,"longestRecoveredGapMs":751},"stats":{"sendRate":3000,"deliveredRate":716.1016949152541,"loss":0.4,"srttMs":11075.871196746826,"minRttMs":9805,"sentPackets":32,"sentWireBytes":28336,"receivedPackets":56,"receivedWireBytes":66533,"lastHeardMs":163702,"unsyncedBytes":4634,"inflightPackets":20,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 120/190","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159812260,"silences":0,"longestSilenceMs":1960,"longestRecoveredGapMs":1960},"stats":{"sendRate":3000,"deliveredRate":703.4972816636835,"loss":0.32285714285714284,"srttMs":10802.135760009289,"minRttMs":9805,"sentPackets":54,"sentWireBytes":36819,"receivedPackets":81,"receivedWireBytes":91220,"lastHeardMs":173663,"unsyncedBytes":3261,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 143/513","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159818569,"silences":1,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":703.4972816636835,"loss":0.32285714285714284,"srttMs":10802.135760009289,"minRttMs":9805,"sentPackets":54,"sentWireBytes":36819,"receivedPackets":82,"receivedWireBytes":91775,"lastHeardMs":179972,"unsyncedBytes":3261,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 578/1173","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159832377,"silences":2,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":512.6752289105343,"loss":0.46599999999999997,"srttMs":11943.868790008128,"minRttMs":9805,"sentPackets":78,"sentWireBytes":43704,"receivedPackets":102,"receivedWireBytes":102521,"lastHeardMs":193780,"unsyncedBytes":2949,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 1340/1340","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"217/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159842478,"silences":2,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":452.56949775100543,"loss":0.45953333333333324,"srttMs":10910.671133986698,"minRttMs":8688,"sentPackets":95,"sentWireBytes":48499,"receivedPackets":139,"receivedWireBytes":118364,"lastHeardMs":203881,"unsyncedBytes":2628,"inflightPackets":58,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870 1004/1554","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 1340/1340","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159852809,"silences":2,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":397.70260322796366,"loss":0.5334380392156861,"srttMs":11626.192131009353,"minRttMs":8688,"sentPackets":120,"sentWireBytes":56755,"receivedPackets":186,"receivedWireBytes":147522,"lastHeardMs":214212,"unsyncedBytes":1385,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870 1374/1554","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 1340/1340 235/288","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159863646,"silences":2,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":576.1927119151737,"loss":0.38138463921568616,"srttMs":10702.794710706632,"minRttMs":8688,"sentPackets":140,"sentWireBytes":64766,"receivedPackets":239,"receivedWireBytes":176783,"lastHeardMs":225049,"unsyncedBytes":707,"inflightPackets":68,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870 1374/1554 1083/1083 153/1108","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 1340/1340 605/723","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"945/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159872451,"silences":2,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":587.3352697771523,"loss":0.35827359527706726,"srttMs":11172.358989838765,"minRttMs":8688,"sentPackets":159,"sentWireBytes":75201,"receivedPackets":289,"receivedWireBytes":203654,"lastHeardMs":233854,"unsyncedBytes":893,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"1025/1025 809/809 809/809 1025/1025","turnItems":"860/860 1058/1058 1108/1108 1299/1299 1058/1058 1108/1108 870/870 1374/1554 1025/1108 528/1108 47/849","messages":"901/901 550/550 456/456 550/550","checkpoints":"779/779 613/613 613/613","attempts":"662/662 662/662","assistant":"21/21 32/32 1340/1340 665/723","user":"10/10 9/9","providerThreads":"810/810","providerTurns":"778/778 778/778","thread":"951/951","runs":"971/971 1014/1014","providerSessions":"2688/2688","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":49216,"lastDataAt":1794159885521,"silences":3,"longestSilenceMs":6309,"longestRecoveredGapMs":6309},"stats":{"sendRate":3000,"deliveredRate":587.3352697771523,"loss":0.35827359527706726,"srttMs":10899.689116108919,"minRttMs":8688,"sentPackets":177,"sentWireBytes":79086,"receivedPackets":334,"receivedWireBytes":227832,"lastHeardMs":246924,"unsyncedBytes":0,"inflightPackets":49,"duplicateBytes":0}}
- shim trace
17:40:39.094 fetch GET /api/auth/session
17:40:39.097 fetched GET /api/auth/session 200
17:40:39.216 fetch GET /.well-known/t3/environment
17:40:39.217 fetched GET /.well-known/t3/environment 200
17:40:39.239 fetch GET /.well-known/t3/environment
17:40:39.240 fetched GET /.well-known/t3/environment 200
17:40:39.248 rpc 0 subscribeServerConfig
17:40:39.248 chunk 0 38690 bytes
17:40:39.272 in Ack 0
17:40:39.275 rpc 1 subscribeServerLifecycle
17:40:39.276 rpc 2 subscribeServerLifecycle
17:40:39.280 fetch GET /api/auth/session
17:40:39.281 fetched GET /api/auth/session 200
17:40:39.282 rpc 3 subscribeDeviceState
17:40:39.285 rpc 4 preview.reportProfiles
17:40:39.387 rpc 5 subscribeProjectClones
17:40:39.429 rpc 6 orchestration.subscribeThread
17:40:39.430 chunk 6 35300 bytes
17:40:39.434 in Ack 6
17:40:39.443 fetch GET /api/orchestration/shell
17:40:39.500 rpc 7 server.refreshProviders
17:40:39.595 rpc 8 orchestration.dispatchCommand thread.visit
17:40:39.595 exit 8 {"_tag":"Success","value":{"sequence":0}}
17:40:39.681 fetched GET /api/orchestration/shell 200
17:40:39.681 rpc 9 subscribeTerminalMetadata
17:40:39.682 rpc 10 subscribePreviewEvents
17:40:39.683 rpc 11 subscribeVcsStatus
17:40:39.687 rpc 12 orchestration.subscribeShell
17:40:39.687 chunk 12 2847 bytes
17:40:39.689 rpc 13 assets.createUrl
17:40:39.689 rpc 14 preview.list
17:40:39.690 rpc 15 projects.readFile
17:40:39.690 rpc 16 assets.createUrl
17:40:39.767 in Ack 12
17:40:39.936 rpc 17 server.reportClientActivity
17:40:40.212 fetch POST /api/observability/v1/traces
17:40:40.212 fetched POST /api/observability/v1/traces 204
17:40:41.208 rpc 18 orchestration.dispatchCommand message.dispatch
17:40:42.207 fetch POST /api/observability/v1/traces
17:40:42.207 fetched POST /api/observability/v1/traces 204
17:40:44.245 in Ping 
17:40:49.245 in Ping 
17:40:54.245 in Ping 
17:40:54.937 in Interrupt 17
17:40:55.938 fetch POST /api/observability/v1/traces
17:40:55.938 fetched POST /api/observability/v1/traces 204
17:40:59.246 in Ping 
17:41:04.246 in Ping 
17:41:04.468 rpc 19 server.reportClientActivity
17:41:05.468 fetch POST /api/observability/v1/traces
17:41:05.468 fetched POST /api/observability/v1/traces 204
17:41:09.246 in Ping 
17:41:14.246 in Ping 
17:41:19.247 in Ping 
17:41:19.469 in Interrupt 19
17:41:20.469 fetch POST /api/observability/v1/traces
17:41:20.469 fetched POST /api/observability/v1/traces 204
17:41:24.247 in Ping 
17:41:29.247 in Ping 
17:41:29.469 rpc 20 server.reportClientActivity
17:41:30.469 fetch POST /api/observability/v1/traces
17:41:30.469 fetched POST /api/observability/v1/traces 204
17:41:34.248 in Ping 
17:41:39.252 in Ping 
17:41:44.256 in Ping 
17:41:44.470 in Interrupt 20
17:41:45.471 fetch POST /api/observability/v1/traces
17:41:45.471 fetched POST /api/observability/v1/traces 204
17:41:49.260 in Ping 
17:41:54.264 in Ping 
17:41:54.470 rpc 21 server.reportClientActivity
17:41:55.470 fetch POST /api/observability/v1/traces
17:41:55.470 fetched POST /api/observability/v1/traces 204
17:41:59.268 in Ping 
17:42:04.273 in Ping 
17:42:09.276 in Ping 
17:42:09.472 in Interrupt 21
17:42:10.473 fetch POST /api/observability/v1/traces
17:42:10.473 fetched POST /api/observability/v1/traces 204
17:42:14.280 in Ping 
17:42:19.284 in Ping 
17:42:19.471 rpc 22 server.reportClientActivity
17:42:20.471 fetch POST /api/observability/v1/traces
17:42:20.471 fetched POST /api/observability/v1/traces 204
17:42:24.288 in Ping 
17:42:29.292 in Ping 
17:42:34.296 in Ping 
17:42:34.471 in Interrupt 22
17:42:35.472 fetch POST /api/observability/v1/traces
17:42:35.472 fetched POST /api/observability/v1/traces 204
17:42:39.300 in Ping 
17:42:44.304 in Ping 
17:42:44.470 rpc 23 server.reportClientActivity
17:42:45.471 fetch POST /api/observability/v1/traces
17:42:45.471 fetched POST /api/observability/v1/traces 204
17:42:49.308 in Ping 
17:42:54.312 in Ping 
17:42:59.316 in Ping 
17:42:59.471 in Interrupt 23
17:43:00.472 fetch POST /api/observability/v1/traces
17:43:00.472 fetched POST /api/observability/v1/traces 204
17:43:04.320 in Ping 
17:43:09.174 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"5e3d3613-8419-46aa-834d-a63e16271218","sessions":[]}}
17:43:09.324 in Ping 
17:43:09.340 chunk 1 3864 bytes
17:43:09.341 in Ack 1
17:43:09.471 rpc 24 server.reportClientActivity
17:43:09.671 chunk 2 3864 bytes
17:43:09.671 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:40:19.382Z","configurableProviders":false,"con
17:43:09.671 in Ack 2
17:43:10.175 fetch POST /api/observability/v1/traces
17:43:10.175 fetched POST /api/observability/v1/traces 204
17:43:14.328 in Ping 
17:43:19.332 in Ping 
17:43:24.336 in Ping 
17:43:24.471 in Interrupt 24
17:43:25.472 fetch POST /api/observability/v1/traces
17:43:25.472 fetched POST /api/observability/v1/traces 204
17:43:29.097 chunk 6 37762 bytes
17:43:29.097 chunk 12 2823 bytes
17:43:29.097 exit 18 {"_tag":"Success","value":{"sequence":76}}
17:43:29.097 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:43:29.101 in Ack 6
17:43:29.176 in Ack 12
17:43:29.340 in Ping 
17:43:29.347 chunk 6 37792 bytes
17:43:29.349 in Ack 6
17:43:29.589 rpc 25 server.reportClientActivity
17:43:30.102 fetch POST /api/observability/v1/traces
17:43:30.102 fetched POST /api/observability/v1/traces 204
17:43:31.153 chunk 6 37824 bytes
17:43:31.154 in Ack 6
17:43:31.610 chunk 6 37854 bytes
17:43:31.611 in Ack 6
17:43:31.861 chunk 6 37914 bytes
17:43:31.862 in Ack 6
17:43:32.111 chunk 6 37944 bytes
17:43:32.112 in Ack 6
17:43:32.155 fetch POST /api/observability/v1/traces
17:43:32.155 fetched POST /api/observability/v1/traces 204
17:43:32.360 chunk 6 37964 bytes
17:43:32.362 in Ack 6
17:43:33.363 fetch POST /api/observability/v1/traces
17:43:33.363 fetched POST /api/observability/v1/traces 204
17:43:34.344 in Ping 
17:43:38.570 chunk 6 38016 bytes
17:43:38.571 in Ack 6
17:43:39.348 in Ping 
17:43:39.573 fetch POST /api/observability/v1/traces
17:43:39.573 fetched POST /api/observability/v1/traces 204
17:43:44.355 in Ping 
17:43:44.440 chunk 6 38078 bytes
17:43:44.442 in Ack 6
17:43:44.589 in Interrupt 25
17:43:44.691 chunk 6 38108 bytes
17:43:44.693 in Ack 6
17:43:44.842 rpc 26 server.reportClientActivity
17:43:45.035 chunk 6 38138 bytes
17:43:45.036 in Ack 6
17:43:45.286 chunk 6 38168 bytes
17:43:45.287 in Ack 6
17:43:45.444 fetch POST /api/observability/v1/traces
17:43:45.444 fetched POST /api/observability/v1/traces 204
17:43:46.219 chunk 6 38200 bytes
17:43:46.220 in Ack 6
17:43:46.469 chunk 6 38230 bytes
17:43:46.471 in Ack 6
17:43:47.221 fetch POST /api/observability/v1/traces
17:43:47.221 fetched POST /api/observability/v1/traces 204
17:43:47.696 chunk 6 38312 bytes
17:43:47.697 in Ack 6
17:43:48.698 fetch POST /api/observability/v1/traces
17:43:48.698 fetched POST /api/observability/v1/traces 204
17:43:49.004 chunk 6 38342 bytes
17:43:49.005 in Ack 6
17:43:49.327 chunk 6 38372 bytes
17:43:49.328 in Ack 6
17:43:49.356 in Ping 
17:43:49.720 chunk 6 38402 bytes
17:43:49.722 in Ack 6
17:43:50.005 fetch POST /api/observability/v1/traces
17:43:50.006 fetched POST /api/observability/v1/traces 204
17:43:50.011 chunk 6 38432 bytes
17:43:50.012 in Ack 6
17:43:50.261 chunk 6 38452 bytes
17:43:50.262 in Ack 6
17:43:50.598 chunk 6 38482 bytes
17:43:50.599 in Ack 6
17:43:50.849 chunk 6 38512 bytes
17:43:50.851 in Ack 6
17:43:51.013 fetch POST /api/observability/v1/traces
17:43:51.013 fetched POST /api/observability/v1/traces 204
17:43:51.591 chunk 6 38542 bytes
17:43:51.593 in Ack 6
17:43:51.841 chunk 6 38572 bytes
17:43:51.844 in Ack 6
17:43:52.091 chunk 6 38602 bytes
17:43:52.092 in Ack 6
17:43:52.341 chunk 6 38806 bytes
17:43:52.342 in Ack 6
17:43:52.592 chunk 6 38888 bytes
17:43:52.593 in Ack 6
17:43:52.604 fetch POST /api/observability/v1/traces
17:43:52.604 fetched POST /api/observability/v1/traces 204
17:43:53.073 chunk 6 39066 bytes
17:43:53.075 in Ack 6
17:43:53.754 chunk 6 39086 bytes
17:43:53.756 in Ack 6
17:43:53.957 chunk 11 534 bytes
17:43:53.957 exit 13 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:43:53.957 exit 16 {"_tag":"Success","value":{"expiresAt":1794163403657,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
17:43:53.957 in Ack 11
17:43:54.005 chunk 6 39420 bytes
17:43:54.006 in Ack 6
17:43:54.076 fetch POST /api/observability/v1/traces
17:43:54.076 fetched POST /api/observability/v1/traces 204
17:43:54.256 chunk 6 39598 bytes
17:43:54.257 in Ack 6
17:43:54.360 in Ping 
17:43:54.585 chunk 6 39618 bytes
17:43:54.586 in Ack 6
17:43:54.835 chunk 6 39762 bytes
17:43:54.836 in Ack 6
17:43:55.258 fetch POST /api/observability/v1/traces
17:43:55.258 fetched POST /api/observability/v1/traces 204
17:43:56.174 chunk 6 39922 bytes
17:43:56.175 in Ack 6
17:43:56.425 chunk 6 40410 bytes
17:43:56.426 in Ack 6
17:43:57.177 fetch POST /api/observability/v1/traces
17:43:57.177 fetched POST /api/observability/v1/traces 204
17:43:59.364 in Ping 
17:43:59.842 in Interrupt 26
17:44:00.094 rpc 27 server.reportClientActivity
17:44:00.202 fetch POST /api/observability/v1/traces
17:44:00.202 fetched POST /api/observability/v1/traces 204
17:44:04.368 in Ping 
17:44:07.639 chunk 6 41421 bytes
17:44:07.640 in Ack 6
17:44:07.790 rpc 28 orchestration.dispatchCommand runtime-request.respond
17:44:08.641 fetch POST /api/observability/v1/traces
17:44:08.641 fetched POST /api/observability/v1/traces 204
17:44:09.372 in Ping 
17:44:10.206 fetch POST /api/observability/v1/traces
17:44:10.206 fetched POST /api/observability/v1/traces 204
17:44:14.376 in Ping 
17:44:15.094 in Interrupt 27
17:44:16.095 fetch POST /api/observability/v1/traces
17:44:16.095 fetched POST /api/observability/v1/traces 204
17:44:18.817 chunk 6 43889 bytes
17:44:18.818 exit 28 {"_tag":"Success","value":{"sequence":96}}
17:44:18.819 in Ack 6
17:44:19.067 chunk 6 43919 bytes
17:44:19.068 in Ack 6
17:44:19.318 chunk 6 43939 bytes
17:44:19.319 in Ack 6
17:44:19.380 in Ping 
17:44:19.572 chunk 6 43969 bytes
17:44:19.573 in Ack 6
17:44:19.820 fetch POST /api/observability/v1/traces
17:44:19.820 fetched POST /api/observability/v1/traces 204
17:44:19.822 chunk 6 43989 bytes
17:44:19.823 in Ack 6
17:44:19.933 rpc 29 orchestration.dispatchCommand runtime-request.respond
17:44:20.313 chunk 6 44009 bytes
17:44:20.314 in Ack 6
17:44:20.563 chunk 6 44049 bytes
17:44:20.564 in Ack 6
17:44:20.813 chunk 6 44069 bytes
17:44:20.814 in Ack 6
17:44:20.825 fetch POST /api/observability/v1/traces
17:44:20.825 fetched POST /api/observability/v1/traces 204
17:44:21.064 chunk 6 44101 bytes
17:44:21.065 in Ack 6
17:44:21.377 chunk 6 44121 bytes
17:44:21.378 in Ack 6
17:44:21.650 chunk 6 44131 bytes
17:44:21.651 in Ack 6
17:44:21.901 chunk 6 44141 bytes
17:44:21.902 in Ack 6
17:44:22.066 fetch POST /api/observability/v1/traces
17:44:22.066 fetched POST /api/observability/v1/traces 204
17:44:22.151 chunk 6 44213 bytes
17:44:22.152 in Ack 6
17:44:22.470 chunk 6 44223 bytes
17:44:22.471 in Ack 6
17:44:22.721 chunk 6 44233 bytes
17:44:22.722 in Ack 6
17:44:22.971 chunk 6 44293 bytes
17:44:22.972 in Ack 6
17:44:23.153 fetch POST /api/observability/v1/traces
17:44:23.153 fetched POST /api/observability/v1/traces 204
17:44:23.221 chunk 6 44313 bytes
17:44:23.222 in Ack 6
17:44:23.472 chunk 6 44323 bytes
17:44:23.473 in Ack 6
17:44:23.722 chunk 6 44343 bytes
17:44:23.723 in Ack 6
17:44:24.119 chunk 6 44365 bytes
17:44:24.120 in Ack 6
17:44:24.223 fetch POST /api/observability/v1/traces
17:44:24.223 fetched POST /api/observability/v1/traces 204
17:44:24.384 in Ping 
17:44:24.472 rpc 30 server.reportClientActivity
17:44:24.561 chunk 6 44375 bytes
17:44:24.562 in Ack 6
17:44:24.812 chunk 6 44405 bytes
17:44:24.813 in Ack 6
17:44:25.063 chunk 6 44447 bytes
17:44:25.064 in Ack 6
17:44:25.473 fetch POST /api/observability/v1/traces
17:44:25.473 fetched POST /api/observability/v1/traces 204
17:44:25.837 chunk 6 44457 bytes
17:44:25.838 in Ack 6
17:44:26.088 chunk 6 44487 bytes
17:44:26.089 in Ack 6
17:44:26.339 chunk 6 44507 bytes
17:44:26.340 in Ack 6
17:44:26.589 chunk 6 44553 bytes
17:44:26.591 in Ack 6
17:44:26.839 fetch POST /api/observability/v1/traces
17:44:26.839 fetched POST /api/observability/v1/traces 204
17:44:26.840 chunk 6 44585 bytes
17:44:26.841 in Ack 6
17:44:27.090 chunk 6 44617 bytes
17:44:27.091 in Ack 6
17:44:27.340 chunk 6 44627 bytes
17:44:27.341 in Ack 6
17:44:27.590 chunk 6 44661 bytes
17:44:27.591 in Ack 6
17:44:27.842 fetch POST /api/observability/v1/traces
17:44:27.842 fetched POST /api/observability/v1/traces 204
17:44:28.345 chunk 6 44671 bytes
17:44:28.346 in Ack 6
17:44:28.599 chunk 6 44701 bytes
17:44:28.600 in Ack 6
17:44:28.850 chunk 6 44725 bytes
17:44:28.851 in Ack 6
17:44:29.232 chunk 6 44757 bytes
17:44:29.234 in Ack 6
17:44:29.348 fetch POST /api/observability/v1/traces
17:44:29.348 fetched POST /api/observability/v1/traces 204
17:44:29.389 in Ping 
17:44:29.483 chunk 6 44777 bytes
17:44:29.484 in Ack 6
17:44:29.733 chunk 6 44843 bytes
17:44:29.734 in Ack 6
17:44:30.190 chunk 6 44871 bytes
17:44:30.192 in Ack 6
17:44:30.440 chunk 6 44891 bytes
17:44:30.441 in Ack 6
17:44:30.497 fetch POST /api/observability/v1/traces
17:44:30.497 fetched POST /api/observability/v1/traces 204
17:44:30.877 chunk 6 44917 bytes
17:44:30.878 in Ack 6
17:44:31.127 chunk 6 44939 bytes
17:44:31.128 in Ack 6
17:44:31.377 chunk 6 44965 bytes
17:44:31.378 in Ack 6
17:44:31.628 chunk 6 44927 bytes
17:44:31.629 in Ack 6
17:44:31.879 chunk 6 44959 bytes
17:44:31.880 fetch POST /api/observability/v1/traces
17:44:31.880 fetched POST /api/observability/v1/traces 204
17:44:31.881 in Ack 6
17:44:32.130 chunk 6 45035 bytes
17:44:32.131 in Ack 6
17:44:32.380 chunk 6 45089 bytes
17:44:32.381 in Ack 6
17:44:32.882 fetch POST /api/observability/v1/traces
17:44:32.882 fetched POST /api/observability/v1/traces 204
17:44:34.392 in Ping 
17:44:38.988 chunk 6 45132 bytes
17:44:38.989 in Ack 6
17:44:39.398 in Ping 
17:44:39.477 in Interrupt 30
17:44:39.990 fetch POST /api/observability/v1/traces
17:44:39.991 fetched POST /api/observability/v1/traces 204
17:44:40.596 chunk 6 45140 bytes
17:44:40.597 in Ack 6
17:44:40.845 chunk 6 45246 bytes
17:44:40.847 in Ack 6
17:44:41.598 fetch POST /api/observability/v1/traces
17:44:41.598 fetched POST /api/observability/v1/traces 204
17:44:44.400 in Ping 
17:44:45.709 chunk 6 45264 bytes
17:44:45.711 in Ack 6
17:44:45.954 exit 29 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
17:44:46.712 fetch POST /api/observability/v1/traces
17:44:46.712 fetched POST /api/observability/v1/traces 204
17:44:49.404 in Ping 
17:44:49.474 rpc 31 server.reportClientActivity
17:44:50.220 fetch POST /api/observability/v1/traces
17:44:50.220 fetched POST /api/observability/v1/traces 204
17:44:50.921 chunk 6 45412 bytes
17:44:50.922 in Ack 6
