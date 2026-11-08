# F, webapp, warm start, seed 1

Profile F (diagnostic): 64 bit/s, 75% bursty loss, down 20 s of every 30 s. Commit fe6e6aee76ac.

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
| Longest silence | 899.6s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 5323 / 7140 |
| Bytes the app received / sent | 0 / 55837 |

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

- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":7015.118409059303,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":1,"sentPackets":92,"sentWireBytes":27523,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":54.80561257077581,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":115,"sentWireBytes":37347,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":509,"inflightPackets":26,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":116,"sentWireBytes":37775,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1001,"inflightPackets":27,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":117,"sentWireBytes":38424,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1001,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":118,"sentWireBytes":39073,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1001,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":119,"sentWireBytes":39722,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1493,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":119,"sentWireBytes":39722,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1493,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":120,"sentWireBytes":40371,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1986,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":121,"sentWireBytes":41020,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1986,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":122,"sentWireBytes":41669,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":1986,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":122,"sentWireBytes":41669,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":2479,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":123,"sentWireBytes":42318,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":2479,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":124,"sentWireBytes":42967,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":3138,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":125,"sentWireBytes":43762,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":3138,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":125,"sentWireBytes":43762,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":3138,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":126,"sentWireBytes":44557,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":3631,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":127,"sentWireBytes":45352,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":3631,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":127,"sentWireBytes":45352,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":4124,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":128,"sentWireBytes":46147,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":4124,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":129,"sentWireBytes":46943,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":4124,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":129,"sentWireBytes":46943,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":4617,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":130,"sentWireBytes":47739,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":4617,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":130,"sentWireBytes":47739,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":5110,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":131,"sentWireBytes":48535,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":5110,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":132,"sentWireBytes":49331,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":5110,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":132,"sentWireBytes":49331,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":5603,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":133,"sentWireBytes":50127,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":5603,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":134,"sentWireBytes":50923,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":6096,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":134,"sentWireBytes":50923,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":6096,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":135,"sentWireBytes":51719,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":6096,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":135,"sentWireBytes":51719,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":6589,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":136,"sentWireBytes":52515,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":6589,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":137,"sentWireBytes":53311,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":7082,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":137,"sentWireBytes":53311,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":7082,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":138,"sentWireBytes":54107,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":7082,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":139,"sentWireBytes":54903,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":7575,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":139,"sentWireBytes":54903,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":7575,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":140,"sentWireBytes":55699,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":8068,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":141,"sentWireBytes":56495,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":8068,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":141,"sentWireBytes":56495,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":8561,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":142,"sentWireBytes":57291,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":8561,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":142,"sentWireBytes":57291,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":8561,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":143,"sentWireBytes":58087,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":9054,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":144,"sentWireBytes":58883,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":9054,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":144,"sentWireBytes":58883,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":9547,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":145,"sentWireBytes":59679,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":9547,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":146,"sentWireBytes":60475,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":9547,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":146,"sentWireBytes":60475,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":10040,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":147,"sentWireBytes":61271,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":10040,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":147,"sentWireBytes":61271,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":10533,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":148,"sentWireBytes":62067,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":10533,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":149,"sentWireBytes":62863,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":10533,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":149,"sentWireBytes":62863,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":11026,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":150,"sentWireBytes":63659,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":11026,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 19","open":true,"generation":2,"iceRestarts":19,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":151,"sentWireBytes":64455,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":11519,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":151,"sentWireBytes":64455,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":11519,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":152,"sentWireBytes":65251,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":11519,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":153,"sentWireBytes":66047,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12012,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":153,"sentWireBytes":66047,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12012,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":154,"sentWireBytes":66843,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12505,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":154,"sentWireBytes":66843,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12505,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":155,"sentWireBytes":67639,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12505,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":156,"sentWireBytes":68435,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12998,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":156,"sentWireBytes":68435,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":12998,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":157,"sentWireBytes":69231,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":13491,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":158,"sentWireBytes":70027,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":13491,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":158,"sentWireBytes":70027,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":13491,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":159,"sentWireBytes":70823,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":13984,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":159,"sentWireBytes":70823,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":13984,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 24","open":true,"generation":2,"iceRestarts":24,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":160,"sentWireBytes":71619,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":14477,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":161,"sentWireBytes":72415,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":14477,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":161,"sentWireBytes":72415,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":14477,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":162,"sentWireBytes":73211,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":14970,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":163,"sentWireBytes":74007,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":14970,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":163,"sentWireBytes":74007,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":15463,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":164,"sentWireBytes":74803,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":15463,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":165,"sentWireBytes":75599,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":15463,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":165,"sentWireBytes":75599,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":15956,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":166,"sentWireBytes":76395,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":15956,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":166,"sentWireBytes":76395,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":16449,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":167,"sentWireBytes":77191,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":16449,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":168,"sentWireBytes":77987,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":16449,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":168,"sentWireBytes":77987,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":16942,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":169,"sentWireBytes":78783,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":16942,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 29","open":true,"generation":2,"iceRestarts":29,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":170,"sentWireBytes":79579,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":17435,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":170,"sentWireBytes":79579,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":17435,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":171,"sentWireBytes":80375,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":17435,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":172,"sentWireBytes":81171,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":17928,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":172,"sentWireBytes":81171,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":17928,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":240,"lastDataAt":1794162688029,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":50,"deliveredRate":3683.0583995847555,"loss":0,"srttMs":3.74801852514644,"minRttMs":null,"sentPackets":173,"sentWireBytes":81967,"receivedPackets":122,"receivedWireBytes":57476,"lastHeardMs":7843,"unsyncedBytes":18421,"inflightPackets":4,"duplicateBytes":0}}
- shim trace
18:31:20.470 fetch GET /api/auth/session
18:31:20.472 fetched GET /api/auth/session 200
18:31:20.586 fetch GET /.well-known/t3/environment
18:31:20.587 fetched GET /.well-known/t3/environment 200
18:31:20.595 fetch GET /.well-known/t3/environment
18:31:20.596 fetched GET /.well-known/t3/environment 200
18:31:20.599 rpc 0 subscribeServerConfig
18:31:20.599 chunk 0 38690 bytes
18:31:20.617 in Ack 0
18:31:20.618 rpc 1 subscribeServerLifecycle
18:31:20.619 rpc 2 subscribeServerLifecycle
18:31:20.622 fetch GET /api/auth/session
18:31:20.623 fetched GET /api/auth/session 200
18:31:20.623 rpc 3 subscribeDeviceState
18:31:20.625 rpc 4 preview.reportProfiles
18:31:20.663 rpc 5 subscribeProjectClones
18:31:20.681 fetch GET /api/orchestration/threads/mcp%3Ab925d3bd-cb6c-4e14-8e42-01223c10189b/bounded
18:31:20.686 fetch GET /api/orchestration/shell
18:31:20.687 fetched GET /api/orchestration/shell 200
18:31:20.688 rpc 6 orchestration.subscribeShell
18:31:20.688 chunk 6 2782 bytes
18:31:20.707 rpc 7 server.refreshProviders
18:31:20.760 rpc 8 orchestration.dispatchCommand thread.visit
18:31:20.760 exit 8 {"_tag":"Success","value":{"sequence":0}}
18:31:20.774 rpc 9 subscribeTerminalMetadata
18:31:20.774 rpc 10 subscribeVcsStatus
18:31:20.775 rpc 11 subscribePreviewEvents
18:31:20.777 rpc 12 assets.createUrl
18:31:20.777 rpc 13 preview.list
18:31:20.778 rpc 14 projects.readFile
18:31:20.780 in Ack 6
18:31:20.789 fetched GET /api/orchestration/threads/mcp%3Ab925d3bd-cb6c-4e14-8e42-01223c10189b/bounded 200
18:31:20.791 rpc 15 orchestration.subscribeThread
18:31:20.792 chunk 15 8249 bytes
18:31:20.819 in Interrupt 10
18:31:20.858 rpc 16 subscribeVcsStatus
18:31:20.860 in Ack 15
18:31:20.938 chunk 6 2758 bytes
18:31:20.938 in Ack 6
18:31:20.981 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"409f7dc1-96a5-49fd-ac54-5673e944b8e5","sessions":[]}}
18:31:20.981 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
18:31:21.042 chunk 15 9239 bytes
18:31:21.043 in Ack 15
18:31:21.105 chunk 1 3864 bytes
18:31:21.105 in Ack 1
18:31:21.110 rpc 17 server.reportClientActivity
18:31:21.292 chunk 15 9303 bytes
18:31:21.294 in Ack 15
18:31:21.349 chunk 2 3864 bytes
18:31:21.349 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T18:31:08.701Z","configurableProviders":false,"con
18:31:21.349 chunk 10 552 bytes
18:31:21.349 in Ack 2
18:31:21.483 exit 12 {"_tag":"Success","value":{"expiresAt":1794166200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
18:31:21.483 chunk 16 552 bytes
18:31:21.484 in Ack 16
18:31:21.585 fetch POST /api/observability/v1/traces
18:31:21.585 fetched POST /api/observability/v1/traces 204
18:31:21.754 chunk 6 2801 bytes
18:31:21.755 in Ack 6
18:31:21.792 chunk 15 11944 bytes
18:31:21.793 in Ack 15
18:31:22.047 rpc 18 attachments.createUploadUrl
18:31:22.203 chunk 15 13532 bytes
18:31:22.212 in Ack 15
18:31:22.298 exit 18 {"_tag":"Success","value":{"attachmentId":"pending-995ce66e-6144-4f87-b182-527e104b0633","expiresAt":1794163282248,"relativeUrl":"/api/attachments/upload/eyJ2ZX
18:31:22.299 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTk5NWNlNjZlLTYxNDQtNGY4Ny1iMTgyLTUyN2UxMDRiMDYzMyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2MzI4MjI0OH0.WOO9OguEioskwTOJKm7E4NBqskOxFYD3o7jHGhHB5AA
18:31:22.454 chunk 15 19883 bytes
18:31:22.455 in Ack 15
18:31:22.462 in Interrupt 16
18:31:22.464 rpc 19 subscribeVcsStatus
18:31:22.558 chunk 19 552 bytes
18:31:22.559 in Ack 19
18:31:22.627 chunk 3 876 bytes
18:31:22.627 exit 4 {"_tag":"Success","value":null}
18:31:22.627 chunk 5 4 bytes
18:31:22.627 chunk 9 36 bytes
18:31:22.627 exit 17 {"_tag":"Success","value":null}
18:31:22.628 in Ack 3
18:31:22.631 rpc 20 preview.reportProfiles
18:31:22.632 in Ack 5
18:31:22.632 in Ack 9
18:31:22.704 chunk 15 21223 bytes
18:31:22.705 in Ack 15
18:31:22.756 fetch POST /api/observability/v1/traces
18:31:22.756 fetched POST /api/observability/v1/traces 204
18:31:22.924 rpc 21 server.reportClientActivity
18:31:23.931 fetch POST /api/observability/v1/traces
18:31:23.931 fetched POST /api/observability/v1/traces 204
18:31:23.997 exit 20 {"_tag":"Success","value":null}
18:31:24.053 exit 21 {"_tag":"Success","value":null}
18:31:24.998 fetch POST /api/observability/v1/traces
18:31:24.998 fetched POST /api/observability/v1/traces 204
18:31:25.200 rpc 22 orchestration.dispatchCommand message.dispatch
18:31:25.742 in Ping 
18:31:25.809 exit 22 {"_tag":"Success","value":{"sequence":45}}
18:31:25.810 rpc 23 attachments.delete
18:31:25.977 exit 23 {"_tag":"Success","value":null}
18:31:26.058 chunk 6 2887 bytes
18:31:26.059 in Ack 6
18:31:26.202 fetch POST /api/observability/v1/traces
18:31:26.202 fetched POST /api/observability/v1/traces 204
18:31:26.310 chunk 15 27083 bytes
18:31:26.311 in Ack 15
18:31:26.314 rpc 24 assets.createUrl
18:31:26.561 chunk 15 28646 bytes
18:31:26.562 in Ack 15
18:31:26.810 chunk 6 2823 bytes
18:31:26.810 chunk 15 28778 bytes
18:31:26.811 in Ack 6
18:31:26.812 rpc 25 orchestration.dispatchCommand thread.visit
18:31:26.812 exit 25 {"_tag":"Success","value":{"sequence":0}}
18:31:26.817 in Ack 15
18:31:27.060 chunk 6 2823 bytes
18:31:27.061 chunk 15 31225 bytes
18:31:27.061 in Ack 6
18:31:27.067 in Ack 15
18:31:27.311 chunk 15 32649 bytes
18:31:27.312 in Ack 15
18:31:27.318 in Interrupt 19
18:31:27.320 rpc 26 subscribeVcsStatus
18:31:27.320 fetch POST /api/observability/v1/traces
18:31:27.321 fetched POST /api/observability/v1/traces 204
18:31:27.378 chunk 26 552 bytes
18:31:27.378 in Ack 26
18:31:27.563 chunk 15 33339 bytes
18:31:27.564 in Ack 15
18:31:27.572 rpc 27 server.reportClientActivity
18:31:27.688 exit 27 {"_tag":"Success","value":null}
18:31:28.029 chunk 15 35276 bytes
18:31:28.031 in Ack 15
18:31:28.573 rpc 28 orchestration.dispatchCommand message.dispatch
18:31:28.575 fetch POST /api/observability/v1/traces
18:31:28.575 fetched POST /api/observability/v1/traces 204
18:31:30.742 in Ping 
18:31:32.194 fetch POST /api/observability/v1/traces
18:31:32.195 fetched POST /api/observability/v1/traces 204
18:31:35.742 in Ping 
18:31:40.746 in Ping 
18:31:45.747 in Ping 
18:31:45.838 rpc 29 server.reportClientActivity
18:31:46.838 fetch POST /api/observability/v1/traces
18:31:46.838 fetched POST /api/observability/v1/traces 204
18:31:50.747 in Ping 
18:31:55.747 in Ping 
18:32:00.747 in Ping 
18:32:00.841 in Interrupt 29
18:32:01.842 fetch POST /api/observability/v1/traces
18:32:01.842 fetched POST /api/observability/v1/traces 204
18:32:05.748 in Ping 
18:32:10.748 in Ping 
18:32:10.838 rpc 30 server.reportClientActivity
18:32:11.839 fetch POST /api/observability/v1/traces
18:32:11.839 fetched POST /api/observability/v1/traces 204
18:32:15.754 in Ping 
18:32:20.758 in Ping 
18:32:25.762 in Ping 
18:32:25.839 in Interrupt 30
18:32:26.843 fetch POST /api/observability/v1/traces
18:32:26.843 fetched POST /api/observability/v1/traces 204
18:32:30.766 in Ping 
18:32:35.770 in Ping 
18:32:35.839 rpc 31 server.reportClientActivity
18:32:36.839 fetch POST /api/observability/v1/traces
18:32:36.839 fetched POST /api/observability/v1/traces 204
18:32:40.776 in Ping 
18:32:45.778 in Ping 
18:32:50.782 in Ping 
18:32:50.840 in Interrupt 31
18:32:51.843 fetch POST /api/observability/v1/traces
18:32:51.843 fetched POST /api/observability/v1/traces 204
18:32:55.786 in Ping 
18:33:00.790 in Ping 
18:33:00.839 rpc 32 server.reportClientActivity
18:33:01.839 fetch POST /api/observability/v1/traces
18:33:01.839 fetched POST /api/observability/v1/traces 204
18:33:05.794 in Ping 
18:33:10.798 in Ping 
18:33:15.802 in Ping 
18:33:15.840 in Interrupt 32
18:33:16.841 fetch POST /api/observability/v1/traces
18:33:16.841 fetched POST /api/observability/v1/traces 204
18:33:20.806 in Ping 
18:33:25.810 in Ping 
18:33:25.840 rpc 33 server.reportClientActivity
18:33:26.840 fetch POST /api/observability/v1/traces
18:33:26.840 fetched POST /api/observability/v1/traces 204
18:33:30.814 in Ping 
18:33:35.818 in Ping 
18:33:40.822 in Ping 
18:33:40.840 in Interrupt 33
18:33:41.841 fetch POST /api/observability/v1/traces
18:33:41.841 fetched POST /api/observability/v1/traces 204
18:33:45.826 in Ping 
18:33:50.830 in Ping 
18:33:50.839 rpc 34 server.reportClientActivity
18:33:51.840 fetch POST /api/observability/v1/traces
18:33:51.840 fetched POST /api/observability/v1/traces 204
18:33:55.834 in Ping 
18:34:00.839 in Ping 
18:34:05.841 in Interrupt 34
18:34:05.842 in Ping 
18:34:06.842 fetch POST /api/observability/v1/traces
18:34:06.842 fetched POST /api/observability/v1/traces 204
18:34:10.846 in Ping 
18:34:15.840 rpc 35 server.reportClientActivity
18:34:15.850 in Ping 
18:34:16.840 fetch POST /api/observability/v1/traces
18:34:16.840 fetched POST /api/observability/v1/traces 204
18:34:20.854 in Ping 
18:34:25.858 in Ping 
18:34:30.841 in Interrupt 35
18:34:30.862 in Ping 
18:34:31.842 fetch POST /api/observability/v1/traces
18:34:31.842 fetched POST /api/observability/v1/traces 204
18:34:35.866 in Ping 
18:34:40.840 rpc 36 server.reportClientActivity
18:34:40.870 in Ping 
18:34:41.840 fetch POST /api/observability/v1/traces
18:34:41.840 fetched POST /api/observability/v1/traces 204
18:34:45.874 in Ping 
18:34:50.878 in Ping 
18:34:55.841 in Interrupt 36
18:34:55.882 in Ping 
18:34:56.843 fetch POST /api/observability/v1/traces
18:34:56.843 fetched POST /api/observability/v1/traces 204
18:35:00.886 in Ping 
18:35:05.841 rpc 37 server.reportClientActivity
18:35:05.890 in Ping 
18:35:06.843 fetch POST /api/observability/v1/traces
18:35:06.843 fetched POST /api/observability/v1/traces 204
18:35:10.894 in Ping 
18:35:15.898 in Ping 
18:35:20.843 in Interrupt 37
18:35:20.902 in Ping 
18:35:21.847 fetch POST /api/observability/v1/traces
18:35:21.847 fetched POST /api/observability/v1/traces 204
18:35:25.906 in Ping 
18:35:30.843 rpc 38 server.reportClientActivity
18:35:30.910 in Ping 
18:35:31.848 fetch POST /api/observability/v1/traces
18:35:31.848 fetched POST /api/observability/v1/traces 204
18:35:35.914 in Ping 
18:35:40.918 in Ping 
18:35:45.847 in Interrupt 38
18:35:45.922 in Ping 
18:35:46.851 fetch POST /api/observability/v1/traces
18:35:46.851 fetched POST /api/observability/v1/traces 204
18:35:50.926 in Ping 
18:35:55.843 rpc 39 server.reportClientActivity
18:35:55.933 in Ping 
18:35:56.847 fetch POST /api/observability/v1/traces
18:35:56.847 fetched POST /api/observability/v1/traces 204
18:36:00.934 in Ping 
18:36:05.938 in Ping 
18:36:10.846 in Interrupt 39
18:36:10.942 in Ping 
18:36:11.851 fetch POST /api/observability/v1/traces
18:36:11.851 fetched POST /api/observability/v1/traces 204
18:36:15.946 in Ping 
18:36:20.848 rpc 40 server.reportClientActivity
18:36:20.950 in Ping 
18:36:21.851 fetch POST /api/observability/v1/traces
18:36:21.851 fetched POST /api/observability/v1/traces 204
18:36:25.954 in Ping 
18:36:30.958 in Ping 
18:36:35.851 in Interrupt 40
18:36:35.963 in Ping 
18:36:36.855 fetch POST /api/observability/v1/traces
18:36:36.855 fetched POST /api/observability/v1/traces 204
18:36:40.966 in Ping 
18:36:45.852 rpc 41 server.reportClientActivity
18:36:45.970 in Ping 
18:36:46.860 fetch POST /api/observability/v1/traces
18:36:46.860 fetched POST /api/observability/v1/traces 204
18:36:50.974 in Ping 
18:36:55.978 in Ping 
18:37:00.855 in Interrupt 41
18:37:00.982 in Ping 
18:37:01.859 fetch POST /api/observability/v1/traces
18:37:01.859 fetched POST /api/observability/v1/traces 204
18:37:05.986 in Ping 
18:37:10.856 rpc 42 server.reportClientActivity
18:37:10.991 in Ping 
18:37:11.859 fetch POST /api/observability/v1/traces
18:37:11.859 fetched POST /api/observability/v1/traces 204
18:37:15.994 in Ping 
18:37:20.998 in Ping 
18:37:25.859 in Interrupt 42
18:37:26.002 in Ping 
18:37:26.863 fetch POST /api/observability/v1/traces
18:37:26.863 fetched POST /api/observability/v1/traces 204
18:37:31.013 in Ping 
18:37:35.860 rpc 43 server.reportClientActivity
18:37:36.024 in Ping 
18:37:36.863 fetch POST /api/observability/v1/traces
18:37:36.863 fetched POST /api/observability/v1/traces 204
18:37:41.027 in Ping 
18:37:46.030 in Ping 
18:37:50.863 in Interrupt 43
18:37:51.034 in Ping 
18:37:51.867 fetch POST /api/observability/v1/traces
18:37:51.867 fetched POST /api/observability/v1/traces 204
18:37:56.038 in Ping 
18:38:00.864 rpc 44 server.reportClientActivity
18:38:01.042 in Ping 
18:38:01.867 fetch POST /api/observability/v1/traces
18:38:01.867 fetched POST /api/observability/v1/traces 204
18:38:06.046 in Ping 
18:38:11.050 in Ping 
18:38:15.866 in Interrupt 44
18:38:16.054 in Ping 
18:38:16.871 fetch POST /api/observability/v1/traces
18:38:16.871 fetched POST /api/observability/v1/traces 204
18:38:21.059 in Ping 
18:38:25.867 rpc 45 server.reportClientActivity
18:38:26.062 in Ping 
18:38:26.871 fetch POST /api/observability/v1/traces
18:38:26.871 fetched POST /api/observability/v1/traces 204
18:38:31.067 in Ping 
18:38:36.070 in Ping 
18:38:40.871 in Interrupt 45
18:38:41.074 in Ping 
18:38:41.875 fetch POST /api/observability/v1/traces
18:38:41.875 fetched POST /api/observability/v1/traces 204
18:38:46.078 in Ping 
18:38:50.872 rpc 46 server.reportClientActivity
18:38:51.082 in Ping 
18:38:51.875 fetch POST /api/observability/v1/traces
18:38:51.875 fetched POST /api/observability/v1/traces 204
18:38:56.086 in Ping 
18:39:01.091 in Ping 
18:39:05.875 in Interrupt 46
18:39:06.094 in Ping 
18:39:06.879 fetch POST /api/observability/v1/traces
18:39:06.879 fetched POST /api/observability/v1/traces 204
18:39:11.098 in Ping 
18:39:15.875 rpc 47 server.reportClientActivity
18:39:16.105 in Ping 
18:39:16.879 fetch POST /api/observability/v1/traces
18:39:16.879 fetched POST /api/observability/v1/traces 204
18:39:21.106 in Ping 
18:39:26.110 in Ping 
18:39:30.879 in Interrupt 47
18:39:31.114 in Ping 
18:39:31.883 fetch POST /api/observability/v1/traces
18:39:31.883 fetched POST /api/observability/v1/traces 204
18:39:36.118 in Ping 
18:39:40.879 rpc 48 server.reportClientActivity
18:39:41.122 in Ping 
18:39:41.885 fetch POST /api/observability/v1/traces
18:39:41.885 fetched POST /api/observability/v1/traces 204
18:39:46.126 in Ping 
18:39:51.131 in Ping 
18:39:55.883 in Interrupt 48
18:39:56.134 in Ping 
18:39:56.887 fetch POST /api/observability/v1/traces
18:39:56.887 fetched POST /api/observability/v1/traces 204
18:40:01.138 in Ping 
18:40:05.884 rpc 49 server.reportClientActivity
18:40:06.142 in Ping 
18:40:06.887 fetch POST /api/observability/v1/traces
18:40:06.887 fetched POST /api/observability/v1/traces 204
18:40:11.146 in Ping 
18:40:16.150 in Ping 
18:40:20.887 in Interrupt 49
18:40:21.154 in Ping 
18:40:21.891 fetch POST /api/observability/v1/traces
18:40:21.891 fetched POST /api/observability/v1/traces 204
18:40:26.158 in Ping 
18:40:30.887 rpc 50 server.reportClientActivity
18:40:31.162 in Ping 
18:40:31.891 fetch POST /api/observability/v1/traces
18:40:31.891 fetched POST /api/observability/v1/traces 204
18:40:36.167 in Ping 
18:40:41.170 in Ping 
18:40:45.891 in Interrupt 50
18:40:46.174 in Ping 
18:40:46.895 fetch POST /api/observability/v1/traces
18:40:46.895 fetched POST /api/observability/v1/traces 204
18:40:51.178 in Ping 
18:40:55.892 rpc 51 server.reportClientActivity
18:40:56.182 in Ping 
18:40:56.895 fetch POST /api/observability/v1/traces
18:40:56.895 fetched POST /api/observability/v1/traces 204
18:41:01.186 in Ping 
18:41:06.191 in Ping 
18:41:10.895 in Interrupt 51
18:41:11.194 in Ping 
18:41:11.899 fetch POST /api/observability/v1/traces
18:41:11.899 fetched POST /api/observability/v1/traces 204
18:41:16.198 in Ping 
18:41:20.895 rpc 52 server.reportClientActivity
18:41:21.202 in Ping 
18:41:21.899 fetch POST /api/observability/v1/traces
18:41:21.899 fetched POST /api/observability/v1/traces 204
18:41:26.206 in Ping 
18:41:31.210 in Ping 
18:41:35.899 in Interrupt 52
18:41:36.214 in Ping 
18:41:36.903 fetch POST /api/observability/v1/traces
18:41:36.903 fetched POST /api/observability/v1/traces 204
18:41:41.218 in Ping 
18:41:45.900 rpc 53 server.reportClientActivity
18:41:46.223 in Ping 
18:41:46.903 fetch POST /api/observability/v1/traces
18:41:46.903 fetched POST /api/observability/v1/traces 204
18:41:51.226 in Ping 
18:41:56.230 in Ping 
18:42:00.903 in Interrupt 53
18:42:01.234 in Ping 
18:42:01.907 fetch POST /api/observability/v1/traces
18:42:01.907 fetched POST /api/observability/v1/traces 204
18:42:06.238 in Ping 
18:42:10.903 rpc 54 server.reportClientActivity
18:42:11.242 in Ping 
18:42:11.907 fetch POST /api/observability/v1/traces
18:42:11.907 fetched POST /api/observability/v1/traces 204
18:42:16.246 in Ping 
18:42:21.250 in Ping 
18:42:25.907 in Interrupt 54
18:42:26.254 in Ping 
18:42:26.911 fetch POST /api/observability/v1/traces
18:42:26.911 fetched POST /api/observability/v1/traces 204
18:42:31.258 in Ping 
18:42:35.907 rpc 55 server.reportClientActivity
18:42:36.262 in Ping 
18:42:36.911 fetch POST /api/observability/v1/traces
18:42:36.911 fetched POST /api/observability/v1/traces 204
18:42:41.266 in Ping 
18:42:46.270 in Ping 
18:42:50.911 in Interrupt 55
18:42:51.274 in Ping 
18:42:51.915 fetch POST /api/observability/v1/traces
18:42:51.915 fetched POST /api/observability/v1/traces 204
18:42:56.278 in Ping 
18:43:00.916 rpc 56 server.reportClientActivity
18:43:01.283 in Ping 
18:43:01.919 fetch POST /api/observability/v1/traces
18:43:01.919 fetched POST /api/observability/v1/traces 204
18:43:06.286 in Ping 
18:43:11.290 in Ping 
18:43:15.919 in Interrupt 56
18:43:16.294 in Ping 
18:43:16.923 fetch POST /api/observability/v1/traces
18:43:16.923 fetched POST /api/observability/v1/traces 204
18:43:21.298 in Ping 
18:43:25.916 rpc 57 server.reportClientActivity
18:43:26.302 in Ping 
18:43:26.919 fetch POST /api/observability/v1/traces
18:43:26.919 fetched POST /api/observability/v1/traces 204
18:43:31.306 in Ping 
18:43:36.310 in Ping 
18:43:40.919 in Interrupt 57
18:43:41.314 in Ping 
18:43:41.924 fetch POST /api/observability/v1/traces
18:43:41.924 fetched POST /api/observability/v1/traces 204
18:43:46.318 in Ping 
18:43:50.920 rpc 58 server.reportClientActivity
18:43:51.322 in Ping 
18:43:51.923 fetch POST /api/observability/v1/traces
18:43:51.923 fetched POST /api/observability/v1/traces 204
18:43:56.326 in Ping 
18:44:01.330 in Ping 
18:44:05.927 in Interrupt 58
18:44:06.334 in Ping 
18:44:06.931 fetch POST /api/observability/v1/traces
18:44:06.931 fetched POST /api/observability/v1/traces 204
18:44:11.338 in Ping 
18:44:15.924 rpc 59 server.reportClientActivity
18:44:16.342 in Ping 
18:44:16.927 fetch POST /api/observability/v1/traces
18:44:16.927 fetched POST /api/observability/v1/traces 204
18:44:21.346 in Ping 
18:44:26.350 in Ping 
18:44:30.927 in Interrupt 59
18:44:31.354 in Ping 
18:44:31.931 fetch POST /api/observability/v1/traces
18:44:31.931 fetched POST /api/observability/v1/traces 204
18:44:36.358 in Ping 
18:44:40.927 rpc 60 server.reportClientActivity
18:44:41.363 in Ping 
18:44:41.931 fetch POST /api/observability/v1/traces
18:44:41.931 fetched POST /api/observability/v1/traces 204
18:44:46.366 in Ping 
18:44:51.371 in Ping 
18:44:55.931 in Interrupt 60
18:44:56.374 in Ping 
18:44:56.935 fetch POST /api/observability/v1/traces
18:44:56.936 fetched POST /api/observability/v1/traces 204
18:45:01.378 in Ping 
18:45:05.932 rpc 61 server.reportClientActivity
18:45:06.382 in Ping 
18:45:06.935 fetch POST /api/observability/v1/traces
18:45:06.935 fetched POST /api/observability/v1/traces 204
18:45:11.386 in Ping 
18:45:16.390 in Ping 
18:45:20.935 in Interrupt 61
18:45:21.394 in Ping 
18:45:21.953 fetch POST /api/observability/v1/traces
18:45:21.954 fetched POST /api/observability/v1/traces 204
18:45:26.398 in Ping 
18:45:30.935 rpc 62 server.reportClientActivity
18:45:31.403 in Ping 
18:45:31.947 fetch POST /api/observability/v1/traces
18:45:31.947 fetched POST /api/observability/v1/traces 204
18:45:36.406 in Ping 
18:45:41.410 in Ping 
18:45:45.939 in Interrupt 62
18:45:46.414 in Ping 
18:45:46.943 fetch POST /api/observability/v1/traces
18:45:46.943 fetched POST /api/observability/v1/traces 204
18:45:51.418 in Ping 
18:45:55.940 rpc 63 server.reportClientActivity
18:45:56.422 in Ping 
18:45:56.943 fetch POST /api/observability/v1/traces
18:45:56.943 fetched POST /api/observability/v1/traces 204
18:46:01.436 in Ping 
18:46:06.438 in Ping 
18:46:10.943 in Interrupt 63
18:46:11.443 in Ping 
18:46:11.948 fetch POST /api/observability/v1/traces
18:46:11.948 fetched POST /api/observability/v1/traces 204
18:46:16.447 in Ping 
18:46:20.945 rpc 64 server.reportClientActivity
18:46:21.450 in Ping 
18:46:21.947 fetch POST /api/observability/v1/traces
18:46:21.948 fetched POST /api/observability/v1/traces 204
18:46:26.454 in Ping 
