# rate100, webapp, warm start, seed 1

Profile rate100 (fault): 100 bytes/s, no random loss. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 899.4s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 90375 / 72209 |
| Bytes the app received / sent | 2327 / 262738 |

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
| 899 | 0.0 | 0 | 0 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 586/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 600/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166605439,"silences":0,"longestSilenceMs":1274,"longestRecoveredGapMs":1274},"stats":{"sendRate":3000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":5.67697344375578,"minRttMs":1,"sentPackets":95,"sentWireBytes":31825,"receivedPackets":101,"receivedWireBytes":53380,"lastHeardMs":8803,"unsyncedBytes":1007,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 639/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166611974,"silences":1,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":112,"sentWireBytes":41378,"receivedPackets":102,"receivedWireBytes":53980,"lastHeardMs":15338,"unsyncedBytes":509,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":123,"sentWireBytes":46360,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":1007,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":135,"sentWireBytes":52682,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":1007,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":144,"sentWireBytes":57434,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":1007,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":159,"sentWireBytes":63118,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":1505,"inflightPackets":71,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":168,"sentWireBytes":67508,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":1505,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":184,"sentWireBytes":73861,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":2004,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":2000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":6483,"sentPackets":198,"sentWireBytes":78910,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":2004,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":1000,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":null,"sentPackets":211,"sentWireBytes":83680,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":2004,"inflightPackets":79,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166617282,"silences":2,"longestSilenceMs":6535,"longestRecoveredGapMs":6535},"stats":{"sendRate":50,"deliveredRate":1789.2857142857142,"loss":0,"srttMs":815.3423517632863,"minRttMs":null,"sentPackets":219,"sentWireBytes":86679,"receivedPackets":103,"receivedWireBytes":54481,"lastHeardMs":20646,"unsyncedBytes":2503,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166709722,"silences":3,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":815.3423517632863,"minRttMs":null,"sentPackets":239,"sentWireBytes":93826,"receivedPackets":104,"receivedWireBytes":54773,"lastHeardMs":113086,"unsyncedBytes":2503,"inflightPackets":85,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":239,"sentWireBytes":93826,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":2447,"inflightPackets":81,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":239,"sentWireBytes":93826,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":2447,"inflightPackets":81,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":255,"sentWireBytes":100177,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":2447,"inflightPackets":58,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":267,"sentWireBytes":105193,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":2946,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":286,"sentWireBytes":112879,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":2946,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":298,"sentWireBytes":117566,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":3445,"inflightPackets":59,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":310,"sentWireBytes":122582,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":3445,"inflightPackets":71,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":326,"sentWireBytes":129270,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":3445,"inflightPackets":87,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":339,"sentWireBytes":134623,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":3944,"inflightPackets":84,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":353,"sentWireBytes":139971,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":3944,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":369,"sentWireBytes":146660,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":4443,"inflightPackets":84,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":384,"sentWireBytes":152674,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":4443,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":396,"sentWireBytes":157361,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":4942,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":412,"sentWireBytes":164050,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":4942,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":424,"sentWireBytes":169068,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":4942,"inflightPackets":85,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":434,"sentWireBytes":173079,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":5441,"inflightPackets":89,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":454,"sentWireBytes":181111,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":5441,"inflightPackets":85,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":466,"sentWireBytes":186127,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":5940,"inflightPackets":83,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":478,"sentWireBytes":191142,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":5940,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":494,"sentWireBytes":197835,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":5940,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":506,"sentWireBytes":202852,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":6439,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":522,"sentWireBytes":209541,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":6439,"inflightPackets":84,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":534,"sentWireBytes":214561,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":6938,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":546,"sentWireBytes":219580,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":6938,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":560,"sentWireBytes":225264,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":6938,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":576,"sentWireBytes":231623,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":7437,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":588,"sentWireBytes":236639,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":7437,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":604,"sentWireBytes":243327,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":7936,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":2000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":26463,"sentPackets":615,"sentWireBytes":247669,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":7936,"inflightPackets":85,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":1000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":634,"sentWireBytes":255362,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":7936,"inflightPackets":85,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":1000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":646,"sentWireBytes":260040,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":8435,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":1000,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":662,"sentWireBytes":266396,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":8435,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":667,"sentWireBytes":268741,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":8934,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":668,"sentWireBytes":269412,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":8934,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":669,"sentWireBytes":270083,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":8934,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":670,"sentWireBytes":270754,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":9433,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":670,"sentWireBytes":270754,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":9433,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":671,"sentWireBytes":271425,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":9932,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":672,"sentWireBytes":272096,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":9932,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":673,"sentWireBytes":272767,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":9932,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":673,"sentWireBytes":272767,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":10431,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":674,"sentWireBytes":273438,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":10431,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":675,"sentWireBytes":274109,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":10930,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":676,"sentWireBytes":274780,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":10930,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":676,"sentWireBytes":274780,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":10930,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":677,"sentWireBytes":275451,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":11429,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":678,"sentWireBytes":276122,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":11429,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":679,"sentWireBytes":276793,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":11928,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":679,"sentWireBytes":276793,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":11928,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":680,"sentWireBytes":277464,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":11928,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":681,"sentWireBytes":278135,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":12427,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":682,"sentWireBytes":278806,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":12427,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":682,"sentWireBytes":278806,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":12926,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":683,"sentWireBytes":279477,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":12926,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":684,"sentWireBytes":280148,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":12926,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":685,"sentWireBytes":280819,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":13425,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":685,"sentWireBytes":280819,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":13425,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":686,"sentWireBytes":281490,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":13924,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":687,"sentWireBytes":282161,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":13924,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":688,"sentWireBytes":282832,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":13924,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":688,"sentWireBytes":282832,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":14423,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":689,"sentWireBytes":283503,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":14423,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":690,"sentWireBytes":284174,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":14922,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":691,"sentWireBytes":284845,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":14922,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":691,"sentWireBytes":284845,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":14922,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":692,"sentWireBytes":285516,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":15421,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":693,"sentWireBytes":286187,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":15421,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":694,"sentWireBytes":286858,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":15920,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":694,"sentWireBytes":286858,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":15920,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":695,"sentWireBytes":287529,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":15920,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":696,"sentWireBytes":288200,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":16419,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":697,"sentWireBytes":288871,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":16419,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":697,"sentWireBytes":288871,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":16918,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":698,"sentWireBytes":289542,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":16918,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":699,"sentWireBytes":290213,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":16918,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":700,"sentWireBytes":290884,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":17417,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":700,"sentWireBytes":290884,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":17417,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"196/732"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":284,"lastDataAt":1794166724732,"silences":5,"longestSilenceMs":92440,"longestRecoveredGapMs":92440},"stats":{"sendRate":50,"deliveredRate":1263.6021745606195,"loss":0,"srttMs":4021.2995577928755,"minRttMs":null,"sentPackets":701,"sentWireBytes":291555,"receivedPackets":106,"receivedWireBytes":55707,"lastHeardMs":128096,"unsyncedBytes":17916,"inflightPackets":4,"duplicateBytes":0}}
- shim trace
19:36:36.927 fetch GET /api/auth/session
19:36:36.928 fetched GET /api/auth/session 200
19:36:37.025 fetch GET /.well-known/t3/environment
19:36:37.026 fetched GET /.well-known/t3/environment 200
19:36:37.033 fetch GET /.well-known/t3/environment
19:36:37.034 fetched GET /.well-known/t3/environment 200
19:36:37.037 rpc 0 subscribeServerConfig
19:36:37.038 chunk 0 38714 bytes
19:36:37.064 in Ack 0
19:36:37.065 rpc 1 subscribeServerLifecycle
19:36:37.066 rpc 2 subscribeServerLifecycle
19:36:37.069 fetch GET /api/auth/session
19:36:37.070 fetched GET /api/auth/session 200
19:36:37.070 rpc 3 subscribeDeviceState
19:36:37.072 rpc 4 preview.reportProfiles
19:36:37.094 rpc 5 subscribeProjectClones
19:36:37.110 fetch GET /api/orchestration/threads/mcp%3Ad46ba3df-f0f0-479b-a28a-31a83a686259/bounded
19:36:37.114 fetch GET /api/orchestration/shell
19:36:37.114 fetched GET /api/orchestration/shell 200
19:36:37.116 rpc 6 orchestration.subscribeShell
19:36:37.116 chunk 6 2788 bytes
19:36:37.134 rpc 7 server.refreshProviders
19:36:37.168 rpc 8 orchestration.dispatchCommand thread.visit
19:36:37.168 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:36:37.180 rpc 9 subscribeTerminalMetadata
19:36:37.181 rpc 10 subscribeVcsStatus
19:36:37.182 rpc 11 subscribePreviewEvents
19:36:37.183 rpc 12 assets.createUrl
19:36:37.184 rpc 13 preview.list
19:36:37.184 rpc 14 projects.readFile
19:36:37.185 in Ack 6
19:36:37.367 chunk 6 2764 bytes
19:36:37.367 in Ack 6
19:36:37.435 rpc 15 server.reportClientActivity
19:36:37.729 chunk 1 3870 bytes
19:36:37.730 in Ack 1
19:36:37.857 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"505d07ca-69d9-468c-9ae5-8219b9840d27","sessions":[]}}
19:36:37.857 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:36:37.857 chunk 2 3870 bytes
19:36:37.861 in Ack 2
19:36:38.024 fetch POST /api/observability/v1/traces
19:36:38.024 fetched POST /api/observability/v1/traces 204
19:36:38.097 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:36:25.531Z","configurableProviders":false,"con
19:36:38.097 chunk 10 1743 bytes
19:36:38.100 in Ack 10
19:36:38.229 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:36:38.428 chunk 3 876 bytes
19:36:38.428 exit 4 {"_tag":"Success","value":null}
19:36:38.428 chunk 5 4 bytes
19:36:38.428 chunk 9 36 bytes
19:36:38.428 exit 15 {"_tag":"Success","value":null}
19:36:38.429 in Ack 3
19:36:38.432 rpc 16 preview.reportProfiles
19:36:38.432 in Ack 5
19:36:38.432 in Ack 9
19:36:38.480 chunk 6 2807 bytes
19:36:38.481 in Ack 6
19:36:38.503 exit 16 {"_tag":"Success","value":null}
19:36:38.639 fetched GET /api/orchestration/threads/mcp%3Ad46ba3df-f0f0-479b-a28a-31a83a686259/bounded 200
19:36:38.642 rpc 17 orchestration.subscribeThread
19:36:38.642 chunk 17 8283 bytes
19:36:38.651 in Interrupt 10
19:36:38.653 rpc 18 subscribeVcsStatus
19:36:38.654 in Ack 17
19:36:38.758 chunk 18 1743 bytes
19:36:38.759 in Ack 18
19:36:38.891 chunk 17 11920 bytes
19:36:38.892 in Ack 17
19:36:38.924 rpc 19 server.reportClientActivity
19:36:39.098 fetch POST /api/observability/v1/traces
19:36:39.098 fetched POST /api/observability/v1/traces 204
19:36:39.168 chunk 17 13502 bytes
19:36:39.171 in Ack 17
19:36:39.257 exit 19 {"_tag":"Success","value":null}
19:36:39.376 rpc 20 attachments.createUploadUrl
19:36:39.532 chunk 17 18491 bytes
19:36:39.544 in Ack 17
19:36:39.588 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-89ee9ba3-a572-480a-bd15-52cb85527b39","expiresAt":1794167199578,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:36:39.589 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTg5ZWU5YmEzLWE1NzItNDgwYS1iZDE1LTUyY2I4NTUyN2IzOSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NzE5OTU3OH0.0jdr6-IxtJ72f75gdTLvr64idL15FBJH1TxlGC5Nbu8
19:36:39.786 chunk 17 20463 bytes
19:36:39.787 in Ack 17
19:36:39.793 in Interrupt 18
19:36:39.795 rpc 21 subscribeVcsStatus
19:36:39.859 chunk 21 1743 bytes
19:36:39.860 in Ack 21
19:36:40.036 chunk 17 21195 bytes
19:36:40.037 in Ack 17
19:36:40.046 rpc 22 server.reportClientActivity
19:36:40.172 fetch POST /api/observability/v1/traces
19:36:40.172 fetched POST /api/observability/v1/traces 204
19:36:40.701 exit 22 {"_tag":"Success","value":null}
19:36:41.702 fetch POST /api/observability/v1/traces
19:36:41.702 fetched POST /api/observability/v1/traces 204
19:36:42.036 in Ping 
19:36:42.604 rpc 23 orchestration.dispatchCommand message.dispatch
19:36:43.210 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:36:43.212 rpc 24 attachments.delete
19:36:43.603 fetch POST /api/observability/v1/traces
19:36:43.603 fetched POST /api/observability/v1/traces 204
19:36:43.710 chunk 17 23639 bytes
19:36:43.711 in Ack 17
19:36:43.961 chunk 17 23661 bytes
19:36:43.962 in Ack 17
19:36:44.211 chunk 17 23663 bytes
19:36:44.211 in Ack 17
19:36:44.460 chunk 6 2829 bytes
19:36:44.461 chunk 17 28089 bytes
19:36:44.461 in Ack 6
19:36:44.463 rpc 25 orchestration.dispatchCommand thread.visit
19:36:44.463 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:36:44.499 in Ack 17
19:36:44.501 rpc 26 assets.createUrl
19:36:44.711 chunk 6 2829 bytes
19:36:44.711 chunk 17 30512 bytes
19:36:44.711 in Ack 6
19:36:44.716 in Ack 17
19:36:44.723 fetch POST /api/observability/v1/traces
19:36:44.723 fetched POST /api/observability/v1/traces 204
19:36:44.728 exit 24 {"_tag":"Success","value":null}
19:36:44.962 chunk 17 32102 bytes
19:36:44.963 in Ack 17
19:36:44.969 in Interrupt 21
19:36:44.971 rpc 27 subscribeVcsStatus
19:36:45.078 chunk 27 1743 bytes
19:36:45.078 in Ack 27
19:36:45.440 chunk 17 33279 bytes
19:36:45.440 rpc 28 server.reportClientActivity
19:36:45.442 in Ack 17
19:36:45.729 fetch POST /api/observability/v1/traces
19:36:45.729 fetched POST /api/observability/v1/traces 204
19:36:46.136 rpc 29 orchestration.dispatchCommand message.dispatch
19:36:47.036 in Ping 
19:36:47.136 fetch POST /api/observability/v1/traces
19:36:47.136 fetched POST /api/observability/v1/traces 204
19:36:48.622 fetch POST /api/observability/v1/traces
19:36:48.622 fetched POST /api/observability/v1/traces 204
19:36:50.183 fetch POST /api/observability/v1/traces
19:36:50.183 fetched POST /api/observability/v1/traces 204
19:36:51.975 chunk 17 34299 bytes
19:36:51.976 in Ack 17
19:36:52.036 in Ping 
19:36:52.977 fetch POST /api/observability/v1/traces
19:36:52.977 fetched POST /api/observability/v1/traces 204
19:36:57.037 in Ping 
19:36:57.283 chunk 17 35210 bytes
19:36:57.285 in Ack 17
19:36:58.286 fetch POST /api/observability/v1/traces
19:36:58.286 fetched POST /api/observability/v1/traces 204
19:37:00.188 fetch POST /api/observability/v1/traces
19:37:00.188 fetched POST /api/observability/v1/traces 204
19:37:00.442 in Interrupt 28
19:37:01.443 fetch POST /api/observability/v1/traces
19:37:01.443 fetched POST /api/observability/v1/traces 204
19:37:02.037 in Ping 
19:37:02.277 rpc 30 server.reportClientActivity
19:37:03.277 fetch POST /api/observability/v1/traces
19:37:03.277 fetched POST /api/observability/v1/traces 204
19:37:07.037 in Ping 
19:37:12.037 in Ping 
19:37:17.038 in Ping 
19:37:17.278 in Interrupt 30
19:37:18.279 fetch POST /api/observability/v1/traces
19:37:18.279 fetched POST /api/observability/v1/traces 204
19:37:22.039 in Ping 
19:37:27.039 in Ping 
19:37:27.278 rpc 31 server.reportClientActivity
19:37:28.278 fetch POST /api/observability/v1/traces
19:37:28.279 fetched POST /api/observability/v1/traces 204
19:37:32.043 in Ping 
19:37:37.053 in Ping 
19:37:42.055 in Ping 
19:37:42.278 in Interrupt 31
19:37:43.279 fetch POST /api/observability/v1/traces
19:37:43.279 fetched POST /api/observability/v1/traces 204
19:37:47.059 in Ping 
19:37:52.063 in Ping 
19:37:52.280 rpc 32 server.reportClientActivity
19:37:53.280 fetch POST /api/observability/v1/traces
19:37:53.280 fetched POST /api/observability/v1/traces 204
19:37:57.067 in Ping 
19:38:02.071 in Ping 
19:38:07.075 in Ping 
19:38:07.280 in Interrupt 32
19:38:08.281 fetch POST /api/observability/v1/traces
19:38:08.281 fetched POST /api/observability/v1/traces 204
19:38:12.079 in Ping 
19:38:17.083 in Ping 
19:38:17.279 rpc 33 server.reportClientActivity
19:38:18.279 fetch POST /api/observability/v1/traces
19:38:18.279 fetched POST /api/observability/v1/traces 204
19:38:22.087 in Ping 
19:38:27.091 in Ping 
19:38:32.095 in Ping 
19:38:32.280 in Interrupt 33
19:38:33.282 fetch POST /api/observability/v1/traces
19:38:33.282 fetched POST /api/observability/v1/traces 204
19:38:37.099 in Ping 
19:38:42.103 in Ping 
19:38:42.309 rpc 34 server.reportClientActivity
19:38:43.309 fetch POST /api/observability/v1/traces
19:38:43.309 fetched POST /api/observability/v1/traces 204
19:38:47.107 in Ping 
19:38:52.111 in Ping 
19:38:57.115 in Ping 
19:38:57.310 in Interrupt 34
19:38:58.311 fetch POST /api/observability/v1/traces
19:38:58.311 fetched POST /api/observability/v1/traces 204
19:39:02.119 in Ping 
19:39:07.123 in Ping 
19:39:07.308 rpc 35 server.reportClientActivity
19:39:08.308 fetch POST /api/observability/v1/traces
19:39:08.308 fetched POST /api/observability/v1/traces 204
19:39:12.127 in Ping 
19:39:17.131 in Ping 
19:39:22.135 in Ping 
19:39:22.309 in Interrupt 35
19:39:23.309 fetch POST /api/observability/v1/traces
19:39:23.309 fetched POST /api/observability/v1/traces 204
19:39:27.139 in Ping 
19:39:32.145 in Ping 
19:39:32.309 rpc 36 server.reportClientActivity
19:39:33.309 fetch POST /api/observability/v1/traces
19:39:33.309 fetched POST /api/observability/v1/traces 204
19:39:37.147 in Ping 
19:39:42.151 in Ping 
19:39:47.155 in Ping 
19:39:47.312 in Interrupt 36
19:39:48.313 fetch POST /api/observability/v1/traces
19:39:48.313 fetched POST /api/observability/v1/traces 204
19:39:52.159 in Ping 
19:39:57.163 in Ping 
19:39:57.309 rpc 37 server.reportClientActivity
19:39:58.310 fetch POST /api/observability/v1/traces
19:39:58.310 fetched POST /api/observability/v1/traces 204
19:40:02.167 in Ping 
19:40:07.171 in Ping 
19:40:12.175 in Ping 
19:40:12.310 in Interrupt 37
19:40:13.312 fetch POST /api/observability/v1/traces
19:40:13.312 fetched POST /api/observability/v1/traces 204
19:40:17.179 in Ping 
19:40:22.183 in Ping 
19:40:22.308 rpc 38 server.reportClientActivity
19:40:23.312 fetch POST /api/observability/v1/traces
19:40:23.312 fetched POST /api/observability/v1/traces 204
19:40:27.187 in Ping 
19:40:32.191 in Ping 
19:40:37.195 in Ping 
19:40:37.312 in Interrupt 38
19:40:38.317 fetch POST /api/observability/v1/traces
19:40:38.317 fetched POST /api/observability/v1/traces 204
19:40:42.199 in Ping 
19:40:47.203 in Ping 
19:40:47.313 rpc 39 server.reportClientActivity
19:40:48.316 fetch POST /api/observability/v1/traces
19:40:48.316 fetched POST /api/observability/v1/traces 204
19:40:52.207 in Ping 
19:40:57.211 in Ping 
19:41:02.215 in Ping 
19:41:02.315 in Interrupt 39
19:41:03.320 fetch POST /api/observability/v1/traces
19:41:03.320 fetched POST /api/observability/v1/traces 204
19:41:07.219 in Ping 
19:41:12.223 in Ping 
19:41:12.313 rpc 40 server.reportClientActivity
19:41:13.316 fetch POST /api/observability/v1/traces
19:41:13.316 fetched POST /api/observability/v1/traces 204
19:41:17.227 in Ping 
19:41:22.231 in Ping 
19:41:27.235 in Ping 
19:41:27.316 in Interrupt 40
19:41:28.320 fetch POST /api/observability/v1/traces
19:41:28.320 fetched POST /api/observability/v1/traces 204
19:41:32.239 in Ping 
19:41:37.243 in Ping 
19:41:37.321 rpc 41 server.reportClientActivity
19:41:38.324 fetch POST /api/observability/v1/traces
19:41:38.324 fetched POST /api/observability/v1/traces 204
19:41:42.247 in Ping 
19:41:47.251 in Ping 
19:41:52.255 in Ping 
19:41:52.323 in Interrupt 41
19:41:53.328 fetch POST /api/observability/v1/traces
19:41:53.328 fetched POST /api/observability/v1/traces 204
19:41:57.259 in Ping 
19:42:02.263 in Ping 
19:42:02.321 rpc 42 server.reportClientActivity
19:42:03.324 fetch POST /api/observability/v1/traces
19:42:03.324 fetched POST /api/observability/v1/traces 204
19:42:07.267 in Ping 
19:42:12.271 in Ping 
19:42:17.275 in Ping 
19:42:17.323 in Interrupt 42
19:42:18.328 fetch POST /api/observability/v1/traces
19:42:18.328 fetched POST /api/observability/v1/traces 204
19:42:22.279 in Ping 
19:42:27.283 in Ping 
19:42:27.328 rpc 43 server.reportClientActivity
19:42:28.332 fetch POST /api/observability/v1/traces
19:42:28.332 fetched POST /api/observability/v1/traces 204
19:42:32.287 in Ping 
19:42:37.291 in Ping 
19:42:42.295 in Ping 
19:42:42.332 in Interrupt 43
19:42:43.336 fetch POST /api/observability/v1/traces
19:42:43.336 fetched POST /api/observability/v1/traces 204
19:42:47.299 in Ping 
19:42:52.303 in Ping 
19:42:52.329 rpc 44 server.reportClientActivity
19:42:53.332 fetch POST /api/observability/v1/traces
19:42:53.332 fetched POST /api/observability/v1/traces 204
19:42:57.307 in Ping 
19:43:02.311 in Ping 
19:43:07.315 in Ping 
19:43:07.331 in Interrupt 44
19:43:08.338 fetch POST /api/observability/v1/traces
19:43:08.338 fetched POST /api/observability/v1/traces 204
19:43:12.319 in Ping 
19:43:17.323 in Ping 
19:43:17.332 rpc 45 server.reportClientActivity
19:43:18.337 fetch POST /api/observability/v1/traces
19:43:18.337 fetched POST /api/observability/v1/traces 204
19:43:22.327 in Ping 
19:43:27.331 in Ping 
19:43:32.335 in Interrupt 45
19:43:32.336 in Ping 
19:43:33.340 fetch POST /api/observability/v1/traces
19:43:33.340 fetched POST /api/observability/v1/traces 204
19:43:37.339 in Ping 
19:43:42.337 rpc 46 server.reportClientActivity
19:43:42.343 in Ping 
19:43:43.340 fetch POST /api/observability/v1/traces
19:43:43.340 fetched POST /api/observability/v1/traces 204
19:43:47.347 in Ping 
19:43:52.352 in Ping 
19:43:57.339 in Interrupt 46
19:43:57.355 in Ping 
19:43:58.344 fetch POST /api/observability/v1/traces
19:43:58.344 fetched POST /api/observability/v1/traces 204
19:44:02.359 in Ping 
19:44:07.341 rpc 47 server.reportClientActivity
19:44:07.363 in Ping 
19:44:08.344 fetch POST /api/observability/v1/traces
19:44:08.344 fetched POST /api/observability/v1/traces 204
19:44:12.367 in Ping 
19:44:17.371 in Ping 
19:44:22.344 in Interrupt 47
19:44:22.375 in Ping 
19:44:23.348 fetch POST /api/observability/v1/traces
19:44:23.348 fetched POST /api/observability/v1/traces 204
19:44:27.379 in Ping 
19:44:32.344 rpc 48 server.reportClientActivity
19:44:32.383 in Ping 
19:44:33.348 fetch POST /api/observability/v1/traces
19:44:33.348 fetched POST /api/observability/v1/traces 204
19:44:37.387 in Ping 
19:44:42.391 in Ping 
19:44:47.347 in Interrupt 48
19:44:47.395 in Ping 
19:44:48.352 fetch POST /api/observability/v1/traces
19:44:48.352 fetched POST /api/observability/v1/traces 204
19:44:52.399 in Ping 
19:44:57.349 rpc 49 server.reportClientActivity
19:44:57.403 in Ping 
19:44:58.352 fetch POST /api/observability/v1/traces
19:44:58.352 fetched POST /api/observability/v1/traces 204
19:45:02.407 in Ping 
19:45:07.411 in Ping 
19:45:12.351 in Interrupt 49
19:45:12.415 in Ping 
19:45:13.356 fetch POST /api/observability/v1/traces
19:45:13.356 fetched POST /api/observability/v1/traces 204
19:45:17.419 in Ping 
19:45:22.352 rpc 50 server.reportClientActivity
19:45:22.423 in Ping 
19:45:23.356 fetch POST /api/observability/v1/traces
19:45:23.356 fetched POST /api/observability/v1/traces 204
19:45:27.427 in Ping 
19:45:32.431 in Ping 
19:45:37.355 in Interrupt 50
19:45:37.435 in Ping 
19:45:38.360 fetch POST /api/observability/v1/traces
19:45:38.360 fetched POST /api/observability/v1/traces 204
19:45:42.439 in Ping 
19:45:47.356 rpc 51 server.reportClientActivity
19:45:47.443 in Ping 
19:45:48.360 fetch POST /api/observability/v1/traces
19:45:48.360 fetched POST /api/observability/v1/traces 204
19:45:52.447 in Ping 
19:45:57.451 in Ping 
19:46:02.359 in Interrupt 51
19:46:02.455 in Ping 
19:46:03.364 fetch POST /api/observability/v1/traces
19:46:03.364 fetched POST /api/observability/v1/traces 204
19:46:07.463 in Ping 
19:46:12.361 rpc 52 server.reportClientActivity
19:46:12.467 in Ping 
19:46:13.364 fetch POST /api/observability/v1/traces
19:46:13.364 fetched POST /api/observability/v1/traces 204
19:46:17.471 in Ping 
19:46:22.475 in Ping 
19:46:27.363 in Interrupt 52
19:46:27.479 in Ping 
19:46:28.368 fetch POST /api/observability/v1/traces
19:46:28.368 fetched POST /api/observability/v1/traces 204
19:46:32.483 in Ping 
19:46:37.365 rpc 53 server.reportClientActivity
19:46:37.487 in Ping 
19:46:38.368 fetch POST /api/observability/v1/traces
19:46:38.368 fetched POST /api/observability/v1/traces 204
19:46:42.491 in Ping 
19:46:47.495 in Ping 
19:46:52.367 in Interrupt 53
19:46:52.499 in Ping 
19:46:53.372 fetch POST /api/observability/v1/traces
19:46:53.372 fetched POST /api/observability/v1/traces 204
19:46:57.503 in Ping 
19:47:02.368 rpc 54 server.reportClientActivity
19:47:02.507 in Ping 
19:47:03.372 fetch POST /api/observability/v1/traces
19:47:03.372 fetched POST /api/observability/v1/traces 204
19:47:07.511 in Ping 
19:47:12.515 in Ping 
19:47:17.372 in Interrupt 54
19:47:17.519 in Ping 
19:47:18.376 fetch POST /api/observability/v1/traces
19:47:18.376 fetched POST /api/observability/v1/traces 204
19:47:22.523 in Ping 
19:47:27.373 rpc 55 server.reportClientActivity
19:47:27.529 in Ping 
19:47:28.376 fetch POST /api/observability/v1/traces
19:47:28.376 fetched POST /api/observability/v1/traces 204
19:47:32.531 in Ping 
19:47:37.535 in Ping 
19:47:42.376 in Interrupt 55
19:47:42.539 in Ping 
19:47:43.380 fetch POST /api/observability/v1/traces
19:47:43.380 fetched POST /api/observability/v1/traces 204
19:47:47.543 in Ping 
19:47:52.376 rpc 56 server.reportClientActivity
19:47:52.547 in Ping 
19:47:53.380 fetch POST /api/observability/v1/traces
19:47:53.380 fetched POST /api/observability/v1/traces 204
19:47:57.551 in Ping 
19:48:02.555 in Ping 
19:48:07.380 in Interrupt 56
19:48:07.559 in Ping 
19:48:08.384 fetch POST /api/observability/v1/traces
19:48:08.384 fetched POST /api/observability/v1/traces 204
19:48:12.563 in Ping 
19:48:17.381 rpc 57 server.reportClientActivity
19:48:17.567 in Ping 
19:48:18.384 fetch POST /api/observability/v1/traces
19:48:18.384 fetched POST /api/observability/v1/traces 204
19:48:22.572 in Ping 
19:48:27.575 in Ping 
19:48:32.384 in Interrupt 57
19:48:32.579 in Ping 
19:48:33.388 fetch POST /api/observability/v1/traces
19:48:33.388 fetched POST /api/observability/v1/traces 204
19:48:37.583 in Ping 
19:48:42.385 rpc 58 server.reportClientActivity
19:48:42.587 in Ping 
19:48:43.388 fetch POST /api/observability/v1/traces
19:48:43.388 fetched POST /api/observability/v1/traces 204
19:48:47.591 in Ping 
19:48:52.595 in Ping 
19:48:57.388 in Interrupt 58
19:48:57.599 in Ping 
19:48:58.392 fetch POST /api/observability/v1/traces
19:48:58.392 fetched POST /api/observability/v1/traces 204
19:49:02.603 in Ping 
19:49:07.389 rpc 59 server.reportClientActivity
19:49:07.607 in Ping 
19:49:08.392 fetch POST /api/observability/v1/traces
19:49:08.392 fetched POST /api/observability/v1/traces 204
19:49:12.611 in Ping 
19:49:17.615 in Ping 
19:49:22.392 in Interrupt 59
19:49:22.619 in Ping 
19:49:23.395 fetch POST /api/observability/v1/traces
19:49:23.396 fetched POST /api/observability/v1/traces 204
19:49:27.623 in Ping 
19:49:32.393 rpc 60 server.reportClientActivity
19:49:32.627 in Ping 
19:49:33.397 fetch POST /api/observability/v1/traces
19:49:33.397 fetched POST /api/observability/v1/traces 204
19:49:37.631 in Ping 
19:49:42.635 in Ping 
19:49:47.395 in Interrupt 60
19:49:47.639 in Ping 
19:49:48.400 fetch POST /api/observability/v1/traces
19:49:48.400 fetched POST /api/observability/v1/traces 204
19:49:52.643 in Ping 
19:49:57.397 rpc 61 server.reportClientActivity
19:49:57.647 in Ping 
19:49:58.400 fetch POST /api/observability/v1/traces
19:49:58.400 fetched POST /api/observability/v1/traces 204
19:50:02.651 in Ping 
19:50:07.655 in Ping 
19:50:12.399 in Interrupt 61
19:50:12.659 in Ping 
19:50:13.404 fetch POST /api/observability/v1/traces
19:50:13.404 fetched POST /api/observability/v1/traces 204
19:50:17.663 in Ping 
19:50:22.400 rpc 62 server.reportClientActivity
19:50:22.667 in Ping 
19:50:23.404 fetch POST /api/observability/v1/traces
19:50:23.404 fetched POST /api/observability/v1/traces 204
19:50:27.671 in Ping 
19:50:32.675 in Ping 
19:50:37.404 in Interrupt 62
19:50:37.691 in Ping 
19:50:38.408 fetch POST /api/observability/v1/traces
19:50:38.408 fetched POST /api/observability/v1/traces 204
19:50:42.695 in Ping 
19:50:47.405 rpc 63 server.reportClientActivity
19:50:47.699 in Ping 
19:50:48.408 fetch POST /api/observability/v1/traces
19:50:48.408 fetched POST /api/observability/v1/traces 204
19:50:52.703 in Ping 
19:50:57.707 in Ping 
19:51:02.408 in Interrupt 63
19:51:02.711 in Ping 
19:51:03.412 fetch POST /api/observability/v1/traces
19:51:03.412 fetched POST /api/observability/v1/traces 204
19:51:07.716 in Ping 
19:51:12.409 rpc 64 server.reportClientActivity
19:51:12.719 in Ping 
19:51:13.412 fetch POST /api/observability/v1/traces
19:51:13.412 fetched POST /api/observability/v1/traces 204
19:51:17.723 in Ping 
19:51:22.727 in Ping 
19:51:27.412 in Interrupt 64
19:51:27.732 in Ping 
19:51:28.416 fetch POST /api/observability/v1/traces
19:51:28.416 fetched POST /api/observability/v1/traces 204
19:51:32.735 in Ping 
19:51:37.412 rpc 65 server.reportClientActivity
19:51:37.739 in Ping 
19:51:38.416 fetch POST /api/observability/v1/traces
19:51:38.416 fetched POST /api/observability/v1/traces 204
19:51:42.743 in Ping 
