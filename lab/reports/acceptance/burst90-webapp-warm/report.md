# burst90, webapp, warm start, seed 1

Profile burst90 (fault): 90% bursty loss (bad runs of 20 packets), 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 292 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 39.5s |
| Prompt reached the agent | 18.0s |
| Time to first text | 215.6s |
| Whole reply shown | 291.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 196.9s / 52.9s / 183.9s |
| Longest silence | 215.6s |
| Approval shown after it was asked | 174.7s |
| Bytes on the wire down / up | 83994 / 53053 |
| Bytes the app received / sent | 65798 / 96917 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 1.9 | 0 | 82 |
| 30 | 11.9 | 0 | 482 |
| 40 | 21.9 | 0 | 886 |
| 50 | 31.9 | 0 | 1077 |
| 60 | 41.9 | 0 | 1077 |
| 70 | 51.9 | 0 | 1077 |
| 80 | 61.9 | 0 | 1077 |
| 90 | 71.9 | 0 | 1077 |
| 100 | 81.9 | 0 | 1077 |
| 110 | 91.9 | 0 | 1077 |
| 120 | 101.9 | 0 | 1077 |
| 130 | 111.9 | 0 | 1077 |
| 140 | 121.9 | 0 | 1077 |
| 150 | 131.9 | 0 | 1077 |
| 160 | 141.9 | 0 | 1077 |
| 170 | 151.9 | 0 | 1077 |
| 180 | 161.9 | 0 | 1077 |
| 190 | 171.9 | 0 | 1077 |
| 200 | 181.9 | 0 | 1077 |
| 210 | 191.9 | 0 | 1077 |
| 220 | 0.0 | 1077 | 1077 |
| 230 | 0.0 | 1077 | 1077 |
| 240 | 0.0 | 1077 | 1077 |
| 250 | 8.6 | 1090 | 1424 |
| 260 | 18.6 | 1090 | 1596 |
| 270 | 28.6 | 1090 | 1596 |
| 280 | 38.6 | 1090 | 1596 |
| 290 | 47.3 | 1142 | 1596 |
| 291 | 48.3 | 1142 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166290570,"silences":0,"longestSilenceMs":1000,"longestRecoveredGapMs":1000},"stats":{"sendRate":10125,"deliveredRate":4035.724983082785,"loss":0,"srttMs":5.142368255852244,"minRttMs":1,"sentPackets":99,"sentWireBytes":31740,"receivedPackets":118,"receivedWireBytes":68437,"lastHeardMs":8598,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166294831,"silences":1,"longestSilenceMs":2014,"longestRecoveredGapMs":2014},"stats":{"sendRate":5062.5,"deliveredRate":4035.724983082785,"loss":0,"srttMs":5.142368255852244,"minRttMs":null,"sentPackets":119,"sentWireBytes":40319,"receivedPackets":126,"receivedWireBytes":72027,"lastHeardMs":12859,"unsyncedBytes":509,"inflightPackets":23,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166294831,"silences":1,"longestSilenceMs":2014,"longestRecoveredGapMs":2014},"stats":{"sendRate":632.8125,"deliveredRate":4035.724983082785,"loss":0,"srttMs":5.142368255852244,"minRttMs":null,"sentPackets":130,"sentWireBytes":45380,"receivedPackets":126,"receivedWireBytes":72027,"lastHeardMs":12859,"unsyncedBytes":1007,"inflightPackets":34,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166294831,"silences":1,"longestSilenceMs":2014,"longestRecoveredGapMs":2014},"stats":{"sendRate":50,"deliveredRate":4035.724983082785,"loss":0,"srttMs":5.142368255852244,"minRttMs":null,"sentPackets":132,"sentWireBytes":46695,"receivedPackets":126,"receivedWireBytes":72027,"lastHeardMs":12859,"unsyncedBytes":1007,"inflightPackets":36,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 25/1063","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166330361,"silences":2,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":142,"sentWireBytes":49922,"receivedPackets":128,"receivedWireBytes":73054,"lastHeardMs":48389,"unsyncedBytes":498,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 112/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166337243,"silences":3,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":151,"sentWireBytes":52490,"receivedPackets":134,"receivedWireBytes":76133,"lastHeardMs":55271,"unsyncedBytes":996,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 112/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166337243,"silences":3,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":151,"sentWireBytes":52490,"receivedPackets":134,"receivedWireBytes":76133,"lastHeardMs":55271,"unsyncedBytes":996,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 112/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166356354,"silences":4,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":169,"sentWireBytes":57633,"receivedPackets":135,"receivedWireBytes":76646,"lastHeardMs":74382,"unsyncedBytes":1494,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 252/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166365735,"silences":5,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":187,"sentWireBytes":62796,"receivedPackets":138,"receivedWireBytes":78181,"lastHeardMs":83763,"unsyncedBytes":1494,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166381335,"silences":6,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":187,"sentWireBytes":62796,"receivedPackets":143,"receivedWireBytes":80739,"lastHeardMs":99363,"unsyncedBytes":1494,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166390230,"silences":8,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":199,"sentWireBytes":66179,"receivedPackets":147,"receivedWireBytes":82784,"lastHeardMs":108258,"unsyncedBytes":1992,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166396919,"silences":9,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":217,"sentWireBytes":71286,"receivedPackets":150,"receivedWireBytes":84316,"lastHeardMs":114947,"unsyncedBytes":1992,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166408140,"silences":11,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":217,"sentWireBytes":71286,"receivedPackets":154,"receivedWireBytes":86355,"lastHeardMs":126168,"unsyncedBytes":2656,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166421195,"silences":14,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":217,"sentWireBytes":71286,"receivedPackets":163,"receivedWireBytes":90963,"lastHeardMs":139223,"unsyncedBytes":2656,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 457/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166429387,"silences":15,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":217,"sentWireBytes":71286,"receivedPackets":168,"receivedWireBytes":93520,"lastHeardMs":147415,"unsyncedBytes":2656,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 612/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166441666,"silences":17,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":225,"sentWireBytes":74111,"receivedPackets":172,"receivedWireBytes":95557,"lastHeardMs":159694,"unsyncedBytes":3155,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 614/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166451090,"silences":18,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":245,"sentWireBytes":81145,"receivedPackets":177,"receivedWireBytes":98096,"lastHeardMs":169118,"unsyncedBytes":3653,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 614/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166458243,"silences":19,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":260,"sentWireBytes":86425,"receivedPackets":183,"receivedWireBytes":101137,"lastHeardMs":176271,"unsyncedBytes":3653,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 614/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166470194,"silences":20,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":2836.339333396045,"loss":0.2571428571428572,"srttMs":456.9995722238707,"minRttMs":3620,"sentPackets":275,"sentWireBytes":91716,"receivedPackets":187,"receivedWireBytes":103170,"lastHeardMs":188222,"unsyncedBytes":4151,"inflightPackets":58,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 848/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"575/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166481307,"silences":22,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":1988.3237950975736,"loss":0.47189189189189196,"srttMs":857.8746256958868,"minRttMs":3664,"sentPackets":285,"sentWireBytes":95254,"receivedPackets":192,"receivedWireBytes":106356,"lastHeardMs":199335,"unsyncedBytes":3764,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 848/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"575/1005"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166483432,"silences":23,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":1988.3237950975736,"loss":0.47189189189189196,"srttMs":857.8746256958868,"minRttMs":3664,"sentPackets":285,"sentWireBytes":95254,"receivedPackets":193,"receivedWireBytes":106998,"lastHeardMs":201461,"unsyncedBytes":4262,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 848/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"575/1005"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166483432,"silences":23,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":3000,"deliveredRate":1988.3237950975736,"loss":0.47189189189189196,"srttMs":857.8746256958868,"minRttMs":3664,"sentPackets":297,"sentWireBytes":98646,"receivedPackets":193,"receivedWireBytes":106998,"lastHeardMs":201461,"unsyncedBytes":4262,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 550/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166512080,"silences":24,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":1397.5677070868694,"loss":0.6053243243243243,"srttMs":831.515297483901,"minRttMs":647,"sentPackets":317,"sentWireBytes":104286,"receivedPackets":205,"receivedWireBytes":112401,"lastHeardMs":230108,"unsyncedBytes":4882,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166522878,"silences":25,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":1397.5677070868694,"loss":0.6053243243243243,"srttMs":831.515297483901,"minRttMs":647,"sentPackets":317,"sentWireBytes":104286,"receivedPackets":224,"receivedWireBytes":119925,"lastHeardMs":240906,"unsyncedBytes":4882,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340 38/38","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166532424,"silences":27,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":992.1860613604882,"loss":0.7006501039501039,"srttMs":1182.5758852984134,"minRttMs":3640,"sentPackets":337,"sentWireBytes":113789,"receivedPackets":231,"receivedWireBytes":122415,"lastHeardMs":250452,"unsyncedBytes":4944,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340 169/494","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166541625,"silences":28,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":992.1860613604882,"loss":0.7006501039501039,"srttMs":1182.5758852984134,"minRttMs":3640,"sentPackets":337,"sentWireBytes":113789,"receivedPackets":236,"receivedWireBytes":124981,"lastHeardMs":259653,"unsyncedBytes":4944,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340 340/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166556375,"silences":30,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":992.1860613604882,"loss":0.7006501039501039,"srttMs":1182.5758852984134,"minRttMs":3640,"sentPackets":348,"sentWireBytes":120137,"receivedPackets":240,"receivedWireBytes":127028,"lastHeardMs":274403,"unsyncedBytes":4944,"inflightPackets":59,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340 340/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166564517,"silences":31,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":992.1860613604882,"loss":0.7006501039501039,"srttMs":1182.5758852984134,"minRttMs":3640,"sentPackets":357,"sentWireBytes":125321,"receivedPackets":244,"receivedWireBytes":129066,"lastHeardMs":282545,"unsyncedBytes":5442,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1006/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21 1340/1340 438/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"288/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":255,"lastDataAt":1794166576863,"silences":33,"longestSilenceMs":35244,"longestRecoveredGapMs":35244},"stats":{"sendRate":2000,"deliveredRate":992.1860613604882,"loss":0.7006501039501039,"srttMs":1496.6288996361118,"minRttMs":3640,"sentPackets":360,"sentWireBytes":127049,"receivedPackets":246,"receivedWireBytes":129969,"lastHeardMs":294891,"unsyncedBytes":4626,"inflightPackets":39,"duplicateBytes":0}}
- shim trace
19:31:22.245 fetch GET /api/auth/session
19:31:22.246 fetched GET /api/auth/session 200
19:31:22.381 fetch GET /.well-known/t3/environment
19:31:22.382 fetched GET /.well-known/t3/environment 200
19:31:22.393 fetch GET /.well-known/t3/environment
19:31:22.394 fetched GET /.well-known/t3/environment 200
19:31:22.408 rpc 0 subscribeServerConfig
19:31:22.408 chunk 0 38714 bytes
19:31:22.428 in Ack 0
19:31:22.429 rpc 1 subscribeServerLifecycle
19:31:22.430 rpc 2 subscribeServerLifecycle
19:31:22.433 fetch GET /api/auth/session
19:31:22.434 fetched GET /api/auth/session 200
19:31:22.435 rpc 3 subscribeDeviceState
19:31:22.436 rpc 4 preview.reportProfiles
19:31:22.469 rpc 5 subscribeProjectClones
19:31:22.486 fetch GET /api/orchestration/threads/mcp%3Ad3609334-432e-4b63-9d1d-61caeb18e7e1/bounded
19:31:22.490 fetch GET /api/orchestration/shell
19:31:22.490 fetched GET /api/orchestration/shell 200
19:31:22.492 rpc 6 orchestration.subscribeShell
19:31:22.492 chunk 6 2788 bytes
19:31:22.513 rpc 7 server.refreshProviders
19:31:22.549 rpc 8 orchestration.dispatchCommand thread.visit
19:31:22.549 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:31:22.562 rpc 9 subscribeTerminalMetadata
19:31:22.562 rpc 10 subscribeVcsStatus
19:31:22.563 rpc 11 subscribePreviewEvents
19:31:22.564 rpc 12 assets.createUrl
19:31:22.565 rpc 13 preview.list
19:31:22.565 rpc 14 projects.readFile
19:31:22.567 in Ack 6
19:31:22.728 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3eec90db-c7d2-4f4c-a089-aff15b01c685","sessions":[]}}
19:31:22.728 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:31:22.744 chunk 6 2764 bytes
19:31:22.745 in Ack 6
19:31:22.814 rpc 15 server.reportClientActivity
19:31:22.830 chunk 1 3870 bytes
19:31:22.831 in Ack 1
19:31:23.190 chunk 2 3870 bytes
19:31:23.190 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:31:10.886Z","configurableProviders":false,"con
19:31:23.190 chunk 10 1659 bytes
19:31:23.190 in Ack 2
19:31:23.193 in Ack 10
19:31:23.300 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:31:23.380 fetch POST /api/observability/v1/traces
19:31:23.380 fetched POST /api/observability/v1/traces 204
19:31:23.498 chunk 3 876 bytes
19:31:23.498 exit 4 {"_tag":"Success","value":null}
19:31:23.498 chunk 5 4 bytes
19:31:23.498 chunk 9 36 bytes
19:31:23.498 exit 15 {"_tag":"Success","value":null}
19:31:23.498 in Ack 3
19:31:23.502 in Ack 5
19:31:23.502 in Ack 9
19:31:23.551 chunk 6 2807 bytes
19:31:23.551 in Ack 6
19:31:23.649 fetched GET /api/orchestration/threads/mcp%3Ad3609334-432e-4b63-9d1d-61caeb18e7e1/bounded 200
19:31:23.652 rpc 16 orchestration.subscribeThread
19:31:23.653 chunk 16 8283 bytes
19:31:23.662 in Interrupt 10
19:31:23.664 rpc 17 subscribeVcsStatus
19:31:23.665 in Ack 16
19:31:23.769 chunk 17 1659 bytes
19:31:23.769 in Ack 17
19:31:23.904 chunk 16 11920 bytes
19:31:23.905 in Ack 16
19:31:23.957 rpc 18 server.reportClientActivity
19:31:24.164 chunk 16 13502 bytes
19:31:24.166 in Ack 16
19:31:24.317 rpc 19 attachments.createUploadUrl
19:31:24.474 exit 18 {"_tag":"Success","value":null}
19:31:24.474 chunk 16 19070 bytes
19:31:24.482 in Ack 16
19:31:24.504 fetch POST /api/observability/v1/traces
19:31:24.504 fetched POST /api/observability/v1/traces 204
19:31:24.529 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-3b98b1f7-42f4-4ca1-9671-aa915d44fe07","expiresAt":1794166884518,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:31:24.530 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTNiOThiMWY3LTQyZjQtNGNhMS05NjcxLWFhOTE1ZDQ0ZmUwNyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2Njg4NDUxOH0.y3yPNnUBLeNPejgnlUPjkkOT8aZrt_cElLBeHXfwTn0
19:31:24.724 chunk 16 21195 bytes
19:31:24.725 in Ack 16
19:31:24.732 in Interrupt 17
19:31:24.733 rpc 20 subscribeVcsStatus
19:31:24.782 chunk 20 1659 bytes
19:31:24.783 in Ack 20
19:31:24.997 rpc 21 server.reportClientActivity
19:31:25.530 fetch POST /api/observability/v1/traces
19:31:25.530 fetched POST /api/observability/v1/traces 204
19:31:25.737 exit 21 {"_tag":"Success","value":null}
19:31:26.739 fetch POST /api/observability/v1/traces
19:31:26.739 fetched POST /api/observability/v1/traces 204
19:31:27.143 rpc 22 orchestration.dispatchCommand message.dispatch
19:31:27.665 in Ping 
19:31:27.730 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:31:27.732 rpc 23 attachments.delete
19:31:27.829 exit 23 {"_tag":"Success","value":null}
19:31:27.981 chunk 6 2893 bytes
19:31:27.982 chunk 16 25465 bytes
19:31:27.982 in Ack 6
19:31:28.035 in Ack 16
19:31:28.038 rpc 24 assets.createUrl
19:31:28.148 fetch POST /api/observability/v1/traces
19:31:28.148 fetched POST /api/observability/v1/traces 204
19:31:28.232 chunk 16 29304 bytes
19:31:28.233 in Ack 16
19:31:28.482 chunk 16 30289 bytes
19:31:28.483 in Ack 16
19:31:28.734 chunk 6 2829 bytes
19:31:28.735 chunk 16 30445 bytes
19:31:28.735 in Ack 6
19:31:28.736 rpc 25 orchestration.dispatchCommand thread.visit
19:31:28.736 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:31:28.742 in Ack 16
19:31:28.984 chunk 6 2829 bytes
19:31:28.984 chunk 16 32868 bytes
19:31:28.985 in Ack 6
19:31:28.990 in Ack 16
19:31:29.234 fetch POST /api/observability/v1/traces
19:31:29.234 fetched POST /api/observability/v1/traces 204
19:31:29.235 chunk 16 33506 bytes
19:31:29.236 in Ack 16
19:31:29.242 in Interrupt 20
19:31:29.244 rpc 26 subscribeVcsStatus
19:31:29.288 chunk 26 1659 bytes
19:31:29.288 in Ack 26
19:31:29.485 chunk 16 35210 bytes
19:31:29.486 in Ack 16
19:31:29.494 rpc 27 server.reportClientActivity
19:31:29.560 exit 27 {"_tag":"Success","value":null}
19:31:30.239 fetch POST /api/observability/v1/traces
19:31:30.239 fetched POST /api/observability/v1/traces 204
19:31:30.711 rpc 28 orchestration.dispatchCommand message.dispatch
19:31:31.711 fetch POST /api/observability/v1/traces
19:31:31.711 fetched POST /api/observability/v1/traces 204
19:31:32.665 in Ping 
19:31:33.999 fetch POST /api/observability/v1/traces
19:31:33.999 fetched POST /api/observability/v1/traces 204
19:31:35.181 fetch POST /api/observability/v1/traces
19:31:35.181 fetched POST /api/observability/v1/traces 204
19:31:37.665 in Ping 
19:31:42.666 in Ping 
19:31:47.633 rpc 29 server.reportClientActivity
19:31:47.666 in Ping 
19:31:48.633 fetch POST /api/observability/v1/traces
19:31:48.633 fetched POST /api/observability/v1/traces 204
19:31:52.666 in Ping 
19:31:57.667 in Ping 
19:32:02.635 in Interrupt 29
19:32:02.667 in Ping 
19:32:03.655 fetch POST /api/observability/v1/traces
19:32:03.655 fetched POST /api/observability/v1/traces 204
19:32:07.667 in Ping 
19:32:10.076 chunk 16 37666 bytes
19:32:10.079 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:32:10.082 in Ack 16
19:32:10.362 chunk 16 37686 bytes
19:32:10.363 in Ack 16
19:32:11.083 fetch POST /api/observability/v1/traces
19:32:11.083 fetched POST /api/observability/v1/traces 204
19:32:11.796 chunk 16 37718 bytes
19:32:11.797 in Ack 16
19:32:12.047 chunk 16 37738 bytes
19:32:12.048 in Ack 16
19:32:12.300 chunk 16 37768 bytes
19:32:12.302 in Ack 16
19:32:12.548 chunk 16 37798 bytes
19:32:12.549 in Ack 16
19:32:12.634 rpc 30 server.reportClientActivity
19:32:12.668 in Ping 
19:32:12.799 fetch POST /api/observability/v1/traces
19:32:12.799 fetched POST /api/observability/v1/traces 204
19:32:14.209 chunk 16 37820 bytes
19:32:14.210 in Ack 16
19:32:15.212 fetch POST /api/observability/v1/traces
19:32:15.212 fetched POST /api/observability/v1/traces 204
19:32:17.243 chunk 16 37870 bytes
19:32:17.245 in Ack 16
19:32:17.672 in Ping 
19:32:18.245 fetch POST /api/observability/v1/traces
19:32:18.245 fetched POST /api/observability/v1/traces 204
19:32:21.159 fetch POST /api/observability/v1/traces
19:32:21.160 fetched POST /api/observability/v1/traces 204
19:32:22.676 in Ping 
19:32:27.635 in Interrupt 30
19:32:27.680 in Ping 
19:32:28.636 fetch POST /api/observability/v1/traces
19:32:28.636 fetched POST /api/observability/v1/traces 204
19:32:32.684 in Ping 
19:32:37.634 rpc 31 server.reportClientActivity
19:32:37.696 in Ping 
19:32:38.634 fetch POST /api/observability/v1/traces
19:32:38.634 fetched POST /api/observability/v1/traces 204
19:32:42.700 in Ping 
19:32:45.737 chunk 16 38146 bytes
19:32:45.738 in Ack 16
19:32:46.739 fetch POST /api/observability/v1/traces
19:32:46.739 fetched POST /api/observability/v1/traces 204
19:32:47.704 in Ping 
19:32:52.634 in Interrupt 31
19:32:52.708 in Ping 
19:32:53.635 fetch POST /api/observability/v1/traces
19:32:53.635 fetched POST /api/observability/v1/traces 204
19:32:57.712 in Ping 
19:33:00.346 chunk 16 38554 bytes
19:33:00.347 in Ack 16
19:33:01.348 fetch POST /api/observability/v1/traces
19:33:01.348 fetched POST /api/observability/v1/traces 204
19:33:02.634 rpc 32 server.reportClientActivity
19:33:02.716 in Ping 
19:33:03.634 fetch POST /api/observability/v1/traces
19:33:03.634 fetched POST /api/observability/v1/traces 204
19:33:07.720 in Ping 
19:33:12.724 in Ping 
19:33:17.635 in Interrupt 32
19:33:17.728 in Ping 
19:33:18.635 fetch POST /api/observability/v1/traces
19:33:18.635 fetched POST /api/observability/v1/traces 204
19:33:22.732 in Ping 
19:33:27.635 rpc 33 server.reportClientActivity
19:33:27.736 in Ping 
19:33:28.634 fetch POST /api/observability/v1/traces
19:33:28.634 fetched POST /api/observability/v1/traces 204
19:33:32.740 in Ping 
19:33:37.744 in Ping 
19:33:42.636 in Interrupt 33
19:33:42.748 in Ping 
19:33:43.637 fetch POST /api/observability/v1/traces
19:33:43.637 fetched POST /api/observability/v1/traces 204
19:33:47.752 in Ping 
19:33:52.634 rpc 34 server.reportClientActivity
19:33:52.756 in Ping 
19:33:53.635 fetch POST /api/observability/v1/traces
19:33:53.635 fetched POST /api/observability/v1/traces 204
19:33:57.760 in Ping 
19:34:01.667 chunk 16 38864 bytes
19:34:01.669 in Ack 16
19:34:02.671 fetch POST /api/observability/v1/traces
19:34:02.671 fetched POST /api/observability/v1/traces 204
19:34:02.764 in Ping 
19:34:07.635 in Interrupt 34
19:34:07.768 in Ping 
19:34:07.888 rpc 35 server.reportClientActivity
19:34:08.636 fetch POST /api/observability/v1/traces
19:34:08.636 fetched POST /api/observability/v1/traces 204
19:34:09.329 chunk 16 38868 bytes
19:34:09.331 in Ack 16
19:34:10.332 fetch POST /api/observability/v1/traces
19:34:10.332 fetched POST /api/observability/v1/traces 204
19:34:12.717 fetch POST /api/observability/v1/traces
19:34:12.717 fetched POST /api/observability/v1/traces 204
19:34:12.772 in Ping 
19:34:17.776 in Ping 
19:34:22.780 in Ping 
19:34:22.889 in Interrupt 35
19:34:23.145 rpc 36 server.reportClientActivity
19:34:23.893 fetch POST /api/observability/v1/traces
19:34:23.893 fetched POST /api/observability/v1/traces 204
19:34:27.784 in Ping 
19:34:32.788 in Ping 
19:34:37.792 in Ping 
19:34:38.148 in Interrupt 36
19:34:39.152 fetch POST /api/observability/v1/traces
19:34:39.152 fetched POST /api/observability/v1/traces 204
19:34:40.606 chunk 16 39338 bytes
19:34:40.607 in Ack 16
19:34:41.308 exit 24 {"_tag":"Success","value":{"expiresAt":1794170072847,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
19:34:41.608 fetch POST /api/observability/v1/traces
19:34:41.608 fetched POST /api/observability/v1/traces 204
19:34:42.635 rpc 37 server.reportClientActivity
19:34:42.796 in Ping 
19:34:43.635 fetch POST /api/observability/v1/traces
19:34:43.635 fetched POST /api/observability/v1/traces 204
19:34:47.800 in Ping 
19:34:52.804 in Ping 
19:34:57.636 in Interrupt 37
19:34:57.808 in Ping 
19:34:58.640 fetch POST /api/observability/v1/traces
19:34:58.640 fetched POST /api/observability/v1/traces 204
19:35:02.812 in Ping 
19:35:06.035 chunk 16 39842 bytes
19:35:06.036 in Ack 16
19:35:07.037 fetch POST /api/observability/v1/traces
19:35:07.037 fetched POST /api/observability/v1/traces 204
19:35:07.553 chunk 16 40322 bytes
19:35:07.554 in Ack 16
19:35:07.635 rpc 38 server.reportClientActivity
19:35:07.908 in Ping 
19:35:08.555 fetch POST /api/observability/v1/traces
19:35:08.555 fetched POST /api/observability/v1/traces 204
19:35:10.016 chunk 16 41327 bytes
19:35:10.017 in Ack 16
19:35:10.259 rpc 39 orchestration.dispatchCommand runtime-request.respond
19:35:11.018 fetch POST /api/observability/v1/traces
19:35:11.018 fetched POST /api/observability/v1/traces 204
19:35:12.912 in Ping 
19:35:17.072 fetch POST /api/observability/v1/traces
19:35:17.072 fetched POST /api/observability/v1/traces 204
19:35:17.916 in Ping 
19:35:22.636 in Interrupt 38
19:35:22.920 in Ping 
19:35:23.640 fetch POST /api/observability/v1/traces
19:35:23.640 fetched POST /api/observability/v1/traces 204
19:35:27.925 in Ping 
19:35:32.425 chunk 16 43827 bytes
19:35:32.425 exit 39 {"_tag":"Success","value":{"sequence":95}}
19:35:32.425 exit 29 {"_tag":"Success","value":null}
19:35:32.426 in Ack 16
19:35:32.637 rpc 40 server.reportClientActivity
19:35:32.928 in Ping 
19:35:33.317 rpc 41 orchestration.dispatchCommand runtime-request.respond
19:35:33.428 fetch POST /api/observability/v1/traces
19:35:33.428 fetched POST /api/observability/v1/traces 204
19:35:37.932 in Ping 
19:35:39.122 chunk 16 43929 bytes
19:35:39.123 in Ack 16
19:35:39.397 chunk 16 43959 bytes
19:35:39.398 in Ack 16
19:35:40.124 fetch POST /api/observability/v1/traces
19:35:40.124 fetched POST /api/observability/v1/traces 204
19:35:40.649 chunk 16 44059 bytes
19:35:40.650 in Ack 16
19:35:41.334 chunk 16 44081 bytes
19:35:41.335 in Ack 16
19:35:41.626 chunk 16 44101 bytes
19:35:41.628 in Ack 16
19:35:41.659 fetch POST /api/observability/v1/traces
19:35:41.659 fetched POST /api/observability/v1/traces 204
19:35:42.936 in Ping 
19:35:43.507 fetch POST /api/observability/v1/traces
19:35:43.507 fetched POST /api/observability/v1/traces 204
19:35:47.640 in Interrupt 40
19:35:47.940 in Ping 
19:35:48.644 fetch POST /api/observability/v1/traces
19:35:48.644 fetched POST /api/observability/v1/traces 204
19:35:52.944 in Ping 
19:35:53.087 chunk 16 44157 bytes
19:35:53.089 in Ack 16
19:35:53.599 chunk 16 44481 bytes
19:35:53.600 in Ack 16
19:35:54.090 fetch POST /api/observability/v1/traces
19:35:54.090 fetched POST /api/observability/v1/traces 204
19:35:57.641 rpc 42 server.reportClientActivity
19:35:57.948 in Ping 
19:35:58.644 fetch POST /api/observability/v1/traces
19:35:58.644 fetched POST /api/observability/v1/traces 204
19:36:02.953 in Ping 
19:36:04.135 fetch POST /api/observability/v1/traces
19:36:04.135 fetched POST /api/observability/v1/traces 204
19:36:07.956 in Ping 
19:36:08.921 chunk 16 44683 bytes
19:36:08.923 in Ack 16
19:36:09.924 fetch POST /api/observability/v1/traces
19:36:09.924 fetched POST /api/observability/v1/traces 204
19:36:12.644 in Interrupt 42
19:36:12.960 in Ping 
19:36:13.649 fetch POST /api/observability/v1/traces
19:36:13.649 fetched POST /api/observability/v1/traces 204
19:36:17.964 in Ping 
19:36:18.705 chunk 16 44819 bytes
19:36:18.706 exit 41 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:36:18.707 in Ack 16
19:36:19.355 rpc 43 orchestration.dispatchCommand runtime-request.respond
19:36:19.708 fetch POST /api/observability/v1/traces
19:36:19.709 fetched POST /api/observability/v1/traces 204
19:36:20.916 chunk 16 45251 bytes
19:36:20.917 in Ack 16
19:36:21.959 fetch POST /api/observability/v1/traces
19:36:21.959 fetched POST /api/observability/v1/traces 204
