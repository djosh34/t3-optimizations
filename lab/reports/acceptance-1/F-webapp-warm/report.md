# F, webapp, warm start, seed 1

Profile F (diagnostic): 64 bit/s, 75% bursty loss, down 20 s of every 30 s. Commit 0f01ba755fce-dirty.

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
| Longest silence | 898.5s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 6904 / 8452 |
| Bytes the app received / sent | 0 / 443000 |

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

- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2250,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":110,"sentWireBytes":36072,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":1170,"inflightPackets":21,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":122,"sentWireBytes":42838,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":1170,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":133,"sentWireBytes":48825,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":1662,"inflightPackets":44,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":145,"sentWireBytes":54168,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":1662,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":161,"sentWireBytes":61292,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":2154,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":173,"sentWireBytes":66635,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":2154,"inflightPackets":84,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":185,"sentWireBytes":71978,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":2154,"inflightPackets":96,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":201,"sentWireBytes":79102,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":2647,"inflightPackets":79,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":213,"sentWireBytes":84445,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":2647,"inflightPackets":81,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":225,"sentWireBytes":89788,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":3140,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 12","open":true,"generation":2,"iceRestarts":12,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":241,"sentWireBytes":96912,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":3140,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":253,"sentWireBytes":102255,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":3140,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":265,"sentWireBytes":108643,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":3799,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":271,"sentWireBytes":112882,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":3799,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 17","open":true,"generation":2,"iceRestarts":17,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":277,"sentWireBytes":117121,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":4292,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 18","open":true,"generation":2,"iceRestarts":18,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":285,"sentWireBytes":122773,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":4292,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 20","open":true,"generation":2,"iceRestarts":20,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":291,"sentWireBytes":127012,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":4292,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 21","open":true,"generation":2,"iceRestarts":21,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":297,"sentWireBytes":131251,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":4785,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 22","open":true,"generation":2,"iceRestarts":22,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":305,"sentWireBytes":136903,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":4785,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 23","open":true,"generation":2,"iceRestarts":23,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":311,"sentWireBytes":141142,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":5278,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 25","open":true,"generation":2,"iceRestarts":25,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":317,"sentWireBytes":145381,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":5278,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 26","open":true,"generation":2,"iceRestarts":26,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":325,"sentWireBytes":151033,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":5278,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 27","open":true,"generation":2,"iceRestarts":27,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":331,"sentWireBytes":155272,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":5771,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 28","open":true,"generation":2,"iceRestarts":28,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":337,"sentWireBytes":159511,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":5771,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 30","open":true,"generation":2,"iceRestarts":30,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":345,"sentWireBytes":165163,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":6264,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 31","open":true,"generation":2,"iceRestarts":31,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":351,"sentWireBytes":169402,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":6264,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 32","open":true,"generation":2,"iceRestarts":32,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":359,"sentWireBytes":175054,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":6264,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 34","open":true,"generation":2,"iceRestarts":34,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":365,"sentWireBytes":179293,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":6757,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 35","open":true,"generation":2,"iceRestarts":35,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":371,"sentWireBytes":183532,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":6757,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 36","open":true,"generation":2,"iceRestarts":36,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":379,"sentWireBytes":189184,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":7250,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 37","open":true,"generation":2,"iceRestarts":37,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":385,"sentWireBytes":193423,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":7250,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 39","open":true,"generation":2,"iceRestarts":39,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":391,"sentWireBytes":197662,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":7250,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 40","open":true,"generation":2,"iceRestarts":40,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":399,"sentWireBytes":203314,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":7743,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 41","open":true,"generation":2,"iceRestarts":41,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":405,"sentWireBytes":207553,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":7743,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 42","open":true,"generation":2,"iceRestarts":42,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":411,"sentWireBytes":211792,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":8236,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 44","open":true,"generation":2,"iceRestarts":44,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":419,"sentWireBytes":217444,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":8236,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 45","open":true,"generation":2,"iceRestarts":45,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":425,"sentWireBytes":221683,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":8236,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 46","open":true,"generation":2,"iceRestarts":46,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":433,"sentWireBytes":227335,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":8729,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 47","open":true,"generation":2,"iceRestarts":47,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":439,"sentWireBytes":231574,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":8729,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 48","open":true,"generation":2,"iceRestarts":48,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":445,"sentWireBytes":235813,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":9222,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 50","open":true,"generation":2,"iceRestarts":50,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":453,"sentWireBytes":241465,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":9222,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 51","open":true,"generation":2,"iceRestarts":51,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":459,"sentWireBytes":245704,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":9222,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 52","open":true,"generation":2,"iceRestarts":52,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":465,"sentWireBytes":249943,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":9715,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 54","open":true,"generation":2,"iceRestarts":54,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":473,"sentWireBytes":255595,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":9715,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 55","open":true,"generation":2,"iceRestarts":55,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":479,"sentWireBytes":259834,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":10208,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 56","open":true,"generation":2,"iceRestarts":56,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":487,"sentWireBytes":265486,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":10208,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 57","open":true,"generation":2,"iceRestarts":57,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":493,"sentWireBytes":269725,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":10701,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 59","open":true,"generation":2,"iceRestarts":59,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":499,"sentWireBytes":273964,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":10701,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 60","open":true,"generation":2,"iceRestarts":60,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":507,"sentWireBytes":279616,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":10701,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 61","open":true,"generation":2,"iceRestarts":61,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":513,"sentWireBytes":283855,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":11194,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 62","open":true,"generation":2,"iceRestarts":62,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":519,"sentWireBytes":288094,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":11194,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 64","open":true,"generation":2,"iceRestarts":64,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":527,"sentWireBytes":293746,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":11687,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 65","open":true,"generation":2,"iceRestarts":65,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":533,"sentWireBytes":297985,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":11687,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 66","open":true,"generation":2,"iceRestarts":66,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":540,"sentWireBytes":303167,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":11687,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 67","open":true,"generation":2,"iceRestarts":67,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":546,"sentWireBytes":307406,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":12180,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 68","open":true,"generation":2,"iceRestarts":68,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":554,"sentWireBytes":313058,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":12180,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 69","open":true,"generation":2,"iceRestarts":69,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":560,"sentWireBytes":317297,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":12673,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 71","open":true,"generation":2,"iceRestarts":71,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":568,"sentWireBytes":322949,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":12673,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 72","open":true,"generation":2,"iceRestarts":72,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":574,"sentWireBytes":327188,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":12673,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 73","open":true,"generation":2,"iceRestarts":73,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":580,"sentWireBytes":331427,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":13166,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 74","open":true,"generation":2,"iceRestarts":74,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":588,"sentWireBytes":337079,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":13166,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 76","open":true,"generation":2,"iceRestarts":76,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":594,"sentWireBytes":341318,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":13659,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 77","open":true,"generation":2,"iceRestarts":77,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":600,"sentWireBytes":345557,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":13659,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 78","open":true,"generation":2,"iceRestarts":78,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":608,"sentWireBytes":351209,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":13659,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 79","open":true,"generation":2,"iceRestarts":79,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":614,"sentWireBytes":355448,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":14152,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 81","open":true,"generation":2,"iceRestarts":81,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":622,"sentWireBytes":361100,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":14152,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 82","open":true,"generation":2,"iceRestarts":82,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":628,"sentWireBytes":365339,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":14645,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 83","open":true,"generation":2,"iceRestarts":83,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":634,"sentWireBytes":369578,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":14645,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 84","open":true,"generation":2,"iceRestarts":84,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":642,"sentWireBytes":375230,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":14645,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 86","open":true,"generation":2,"iceRestarts":86,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":648,"sentWireBytes":379469,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":15138,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 87","open":true,"generation":2,"iceRestarts":87,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":654,"sentWireBytes":383708,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":15138,"inflightPackets":42,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 88","open":true,"generation":2,"iceRestarts":88,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":662,"sentWireBytes":389360,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":15631,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 89","open":true,"generation":2,"iceRestarts":89,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":668,"sentWireBytes":393599,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":15631,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 91","open":true,"generation":2,"iceRestarts":91,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":674,"sentWireBytes":397838,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":15631,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 92","open":true,"generation":2,"iceRestarts":92,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":682,"sentWireBytes":403490,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":16124,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 93","open":true,"generation":2,"iceRestarts":93,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":688,"sentWireBytes":407729,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":16124,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 94","open":true,"generation":2,"iceRestarts":94,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":696,"sentWireBytes":413381,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":16617,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 96","open":true,"generation":2,"iceRestarts":96,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":702,"sentWireBytes":417620,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":16617,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 97","open":true,"generation":2,"iceRestarts":97,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":708,"sentWireBytes":421859,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":17110,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 98","open":true,"generation":2,"iceRestarts":98,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":716,"sentWireBytes":427511,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":17110,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 99","open":true,"generation":2,"iceRestarts":99,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":722,"sentWireBytes":431750,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":17110,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 101","open":true,"generation":2,"iceRestarts":101,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":728,"sentWireBytes":435989,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":17603,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 102","open":true,"generation":2,"iceRestarts":102,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":736,"sentWireBytes":441641,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":17603,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 103","open":true,"generation":2,"iceRestarts":103,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":742,"sentWireBytes":445880,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":18096,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 104","open":true,"generation":2,"iceRestarts":104,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":750,"sentWireBytes":451532,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":18096,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 106","open":true,"generation":2,"iceRestarts":106,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":756,"sentWireBytes":455771,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":18096,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 107","open":true,"generation":2,"iceRestarts":107,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":762,"sentWireBytes":460010,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":18589,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 108","open":true,"generation":2,"iceRestarts":108,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":770,"sentWireBytes":465662,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":18589,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809","messages":"550/550 456/456","checkpoints":"613/613 526/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 109","open":true,"generation":2,"iceRestarts":109,"newConnections":0,"handshakeMs":374,"lastDataAt":1794151568374,"silences":0,"longestSilenceMs":1515,"longestRecoveredGapMs":1515},"stats":{"sendRate":2000,"deliveredRate":2265.4088050314463,"loss":0,"srttMs":13.46357316524901,"minRttMs":null,"sentPackets":776,"sentWireBytes":469901,"receivedPackets":120,"receivedWireBytes":58094,"lastHeardMs":9954,"unsyncedBytes":19082,"inflightPackets":40,"duplicateBytes":0}}
- shim trace
15:25:58.848 fetch GET /api/auth/session
15:25:58.849 fetched GET /api/auth/session 200
15:25:58.959 fetch GET /.well-known/t3/environment
15:25:58.960 fetched GET /.well-known/t3/environment 200
15:25:58.989 fetch GET /.well-known/t3/environment
15:25:58.990 fetched GET /.well-known/t3/environment 200
15:25:58.994 rpc 0 subscribeServerConfig
15:25:58.994 chunk 0 38690 bytes
15:25:59.030 in Ack 0
15:25:59.033 rpc 1 subscribeServerLifecycle
15:25:59.035 rpc 2 subscribeServerLifecycle
15:25:59.038 fetch GET /api/auth/session
15:25:59.039 fetched GET /api/auth/session 200
15:25:59.040 rpc 3 subscribeDeviceState
15:25:59.044 rpc 4 preview.reportProfiles
15:25:59.109 rpc 5 subscribeProjectClones
15:25:59.115 fetch GET /api/orchestration/threads/mcp%3Aa46a5b7d-28a8-4082-acf6-0c14b611dd41/bounded
15:25:59.134 fetch GET /api/orchestration/shell
15:25:59.157 rpc 6 server.refreshProviders
15:25:59.202 rpc 7 orchestration.dispatchCommand thread.visit
15:25:59.226 fetched GET /api/orchestration/shell 200
15:25:59.226 rpc 8 subscribeTerminalMetadata
15:25:59.227 rpc 9 subscribeVcsStatus
15:25:59.229 rpc 10 subscribePreviewEvents
15:25:59.231 rpc 11 orchestration.subscribeShell
15:25:59.231 chunk 11 2782 bytes
15:25:59.233 rpc 12 assets.createUrl
15:25:59.234 rpc 13 preview.list
15:25:59.235 rpc 14 projects.readFile
15:25:59.237 in Ack 11
15:25:59.401 exit 7 {"_tag":"Success","value":{"sequence":38}}
15:25:59.480 rpc 15 server.reportClientActivity
15:25:59.917 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"9432bd12-1451-4b4b-9719-df759c431328","sessions":[]}}
15:25:59.917 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:25:59.917 chunk 1 3864 bytes
15:25:59.923 in Ack 1
15:25:59.955 fetch POST /api/observability/v1/traces
15:25:59.955 fetched POST /api/observability/v1/traces 204
15:26:00.141 chunk 2 3864 bytes
15:26:00.141 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:25:43.597Z","configurableProviders":false,"con
15:26:00.141 chunk 9 534 bytes
15:26:00.142 in Ack 2
15:26:00.146 in Ack 9
15:26:00.264 chunk 11 944 bytes
15:26:00.264 exit 12 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:26:00.264 in Ack 11
15:26:00.470 chunk 3 876 bytes
15:26:00.470 exit 4 {"_tag":"Success","value":null}
15:26:00.470 chunk 5 4 bytes
15:26:00.470 chunk 8 36 bytes
15:26:00.470 exit 15 {"_tag":"Success","value":null}
15:26:00.471 in Ack 3
15:26:00.472 rpc 16 preview.reportProfiles
15:26:00.472 in Ack 5
15:26:00.472 in Ack 8
15:26:00.514 chunk 11 2801 bytes
15:26:00.515 in Ack 11
15:26:00.523 rpc 17 server.refreshProviders
15:26:00.523 exit 17 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:25:43.597Z","configurableProviders":false,"con
15:26:00.592 exit 16 {"_tag":"Success","value":null}
15:26:00.813 fetched GET /api/orchestration/threads/mcp%3Aa46a5b7d-28a8-4082-acf6-0c14b611dd41/bounded 200
15:26:00.827 rpc 18 orchestration.subscribeThread
15:26:00.827 chunk 18 8305 bytes
15:26:00.850 in Interrupt 9
15:26:00.896 rpc 19 subscribeVcsStatus
15:26:00.947 in Ack 18
15:26:00.999 chunk 19 534 bytes
15:26:00.999 in Ack 19
15:26:01.087 chunk 18 11944 bytes
15:26:01.089 in Ack 18
15:26:01.147 fetch POST /api/observability/v1/traces
15:26:01.147 fetched POST /api/observability/v1/traces 204
15:26:01.148 rpc 20 server.reportClientActivity
15:26:01.338 chunk 18 13532 bytes
15:26:01.339 in Ack 18
15:26:01.347 exit 20 {"_tag":"Success","value":null}
15:26:01.589 chunk 18 18055 bytes
15:26:01.590 in Ack 18
15:26:01.839 chunk 18 21223 bytes
15:26:01.840 in Ack 18
15:26:01.854 in Interrupt 19
15:26:01.858 rpc 21 subscribeVcsStatus
15:26:01.929 chunk 21 534 bytes
15:26:01.930 in Ack 21
15:26:01.988 rpc 22 attachments.createUploadUrl
15:26:02.191 rpc 23 server.reportClientActivity
15:26:02.192 fetch POST /api/observability/v1/traces
15:26:02.192 fetched POST /api/observability/v1/traces 204
15:26:02.233 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-9944d2d5-d501-4625-befe-441acce5494f","expiresAt":1794152162219,"relativeUrl":"/api/attachments/upload/eyJ2ZX
15:26:02.233 exit 23 {"_tag":"Success","value":null}
15:26:02.235 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTk5NDRkMmQ1LWQ1MDEtNDYyNS1iZWZlLTQ0MWFjY2U1NDk0ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1MjE2MjIxOX0.WDYxHGEBnIt2pk_wQLe1OVij2dfIcPEIveZPaQViWkw
15:26:03.262 fetch POST /api/observability/v1/traces
15:26:03.262 fetched POST /api/observability/v1/traces 204
15:26:03.992 in Ping 
15:26:05.410 rpc 24 orchestration.dispatchCommand message.dispatch
15:26:06.048 exit 24 {"_tag":"Success","value":{"sequence":45}}
15:26:06.051 rpc 25 attachments.delete
15:26:06.374 chunk 18 23679 bytes
15:26:06.375 in Ack 18
15:26:06.473 fetch POST /api/observability/v1/traces
15:26:06.473 fetched POST /api/observability/v1/traces 204
15:26:06.627 chunk 18 23699 bytes
15:26:06.628 in Ack 18
15:26:06.876 chunk 18 23711 bytes
15:26:06.877 in Ack 18
15:26:07.125 chunk 11 944 bytes
15:26:07.126 in Ack 11
15:26:07.134 chunk 18 23713 bytes
15:26:07.135 in Ack 18
15:26:07.188 exit 25 {"_tag":"Success","value":null}
15:26:07.375 chunk 11 2823 bytes
15:26:07.376 in Ack 11
15:26:07.378 rpc 26 orchestration.dispatchCommand thread.visit
15:26:07.412 chunk 18 28141 bytes
15:26:07.416 in Ack 18
15:26:07.419 rpc 27 assets.createUrl
15:26:07.538 exit 26 {"_tag":"Success","value":{"sequence":68}}
15:26:07.626 chunk 11 944 bytes
15:26:07.626 in Ack 11
15:26:07.633 fetch POST /api/observability/v1/traces
15:26:07.633 fetched POST /api/observability/v1/traces 204
15:26:07.875 chunk 11 2823 bytes
15:26:07.876 in Ack 11
15:26:07.913 chunk 18 30564 bytes
15:26:07.915 in Ack 18
15:26:08.354 chunk 18 31988 bytes
15:26:08.356 in Ack 18
15:26:08.368 in Interrupt 21
15:26:08.373 rpc 28 subscribeVcsStatus
15:26:08.621 chunk 18 31374 bytes
15:26:08.695 in Ack 18
15:26:08.759 rpc 29 server.reportClientActivity
15:26:08.943 fetch POST /api/observability/v1/traces
15:26:08.943 fetched POST /api/observability/v1/traces 204
15:26:09.008 in Ping 
15:26:10.040 rpc 30 orchestration.dispatchCommand message.dispatch
15:26:10.890 fetch POST /api/observability/v1/traces
15:26:10.890 fetched POST /api/observability/v1/traces 204
15:26:12.340 fetch POST /api/observability/v1/traces
15:26:12.340 fetched POST /api/observability/v1/traces 204
15:26:14.032 in Ping 
15:26:19.035 in Ping 
15:26:23.763 in Interrupt 29
15:26:24.039 in Ping 
15:26:24.216 rpc 31 server.reportClientActivity
15:26:24.764 fetch POST /api/observability/v1/traces
15:26:24.764 fetched POST /api/observability/v1/traces 204
15:26:29.056 in Ping 
15:26:34.078 in Ping 
15:26:39.078 in Ping 
15:26:39.218 in Interrupt 31
15:26:40.223 fetch POST /api/observability/v1/traces
15:26:40.223 fetched POST /api/observability/v1/traces 204
15:26:44.085 in Ping 
15:26:49.085 in Ping 
15:26:49.218 rpc 32 server.reportClientActivity
15:26:50.218 fetch POST /api/observability/v1/traces
15:26:50.218 fetched POST /api/observability/v1/traces 204
15:26:54.088 in Ping 
15:26:59.092 in Ping 
15:27:04.096 in Ping 
15:27:04.219 in Interrupt 32
15:27:05.221 fetch POST /api/observability/v1/traces
15:27:05.221 fetched POST /api/observability/v1/traces 204
15:27:09.100 in Ping 
15:27:14.104 in Ping 
15:27:14.217 rpc 33 server.reportClientActivity
15:27:15.218 fetch POST /api/observability/v1/traces
15:27:15.219 fetched POST /api/observability/v1/traces 204
15:27:19.108 in Ping 
15:27:24.112 in Ping 
15:27:29.116 in Ping 
15:27:29.220 in Interrupt 33
15:27:30.222 fetch POST /api/observability/v1/traces
15:27:30.222 fetched POST /api/observability/v1/traces 204
15:27:34.120 in Ping 
15:27:39.124 in Ping 
15:27:39.218 rpc 34 server.reportClientActivity
15:27:40.220 fetch POST /api/observability/v1/traces
15:27:40.220 fetched POST /api/observability/v1/traces 204
15:27:44.128 in Ping 
15:27:49.132 in Ping 
15:27:54.136 in Ping 
15:27:54.219 in Interrupt 34
15:27:55.220 fetch POST /api/observability/v1/traces
15:27:55.220 fetched POST /api/observability/v1/traces 204
15:27:59.140 in Ping 
15:28:04.144 in Ping 
15:28:04.221 rpc 35 server.reportClientActivity
15:28:05.221 fetch POST /api/observability/v1/traces
15:28:05.221 fetched POST /api/observability/v1/traces 204
15:28:09.152 in Ping 
15:28:14.158 in Ping 
15:28:19.160 in Ping 
15:28:19.221 in Interrupt 35
15:28:20.232 fetch POST /api/observability/v1/traces
15:28:20.232 fetched POST /api/observability/v1/traces 204
15:28:24.164 in Ping 
15:28:29.168 in Ping 
15:28:29.225 rpc 36 server.reportClientActivity
15:28:30.225 fetch POST /api/observability/v1/traces
15:28:30.225 fetched POST /api/observability/v1/traces 204
15:28:34.175 in Ping 
15:28:39.176 in Ping 
15:28:44.180 in Ping 
15:28:44.228 in Interrupt 36
15:28:45.229 fetch POST /api/observability/v1/traces
15:28:45.229 fetched POST /api/observability/v1/traces 204
15:28:49.185 in Ping 
15:28:54.188 in Ping 
15:28:54.221 rpc 37 server.reportClientActivity
15:28:55.221 fetch POST /api/observability/v1/traces
15:28:55.221 fetched POST /api/observability/v1/traces 204
15:28:59.192 in Ping 
15:29:04.196 in Ping 
15:29:09.200 in Ping 
15:29:09.221 in Interrupt 37
15:29:10.223 fetch POST /api/observability/v1/traces
15:29:10.223 fetched POST /api/observability/v1/traces 204
15:29:14.204 in Ping 
15:29:19.208 in Ping 
15:29:19.226 rpc 38 server.reportClientActivity
15:29:20.227 fetch POST /api/observability/v1/traces
15:29:20.227 fetched POST /api/observability/v1/traces 204
15:29:24.213 in Ping 
15:29:29.224 in Ping 
15:29:34.227 in Interrupt 38
15:29:34.228 in Ping 
15:29:35.228 fetch POST /api/observability/v1/traces
15:29:35.229 fetched POST /api/observability/v1/traces 204
15:29:39.232 in Ping 
15:29:44.227 rpc 39 server.reportClientActivity
15:29:44.236 in Ping 
15:29:45.229 fetch POST /api/observability/v1/traces
15:29:45.229 fetched POST /api/observability/v1/traces 204
15:29:49.240 in Ping 
15:29:54.248 in Ping 
15:29:59.228 in Interrupt 39
15:29:59.252 in Ping 
15:30:00.234 fetch POST /api/observability/v1/traces
15:30:00.234 fetched POST /api/observability/v1/traces 204
15:30:04.257 in Ping 
15:30:09.230 rpc 40 server.reportClientActivity
15:30:09.260 in Ping 
15:30:10.233 fetch POST /api/observability/v1/traces
15:30:10.233 fetched POST /api/observability/v1/traces 204
15:30:14.264 in Ping 
15:30:19.268 in Ping 
15:30:24.233 in Interrupt 40
15:30:24.272 in Ping 
15:30:25.239 fetch POST /api/observability/v1/traces
15:30:25.239 fetched POST /api/observability/v1/traces 204
15:30:29.277 in Ping 
15:30:34.233 rpc 41 server.reportClientActivity
15:30:34.284 in Ping 
15:30:35.233 fetch POST /api/observability/v1/traces
15:30:35.233 fetched POST /api/observability/v1/traces 204
15:30:39.290 in Ping 
15:30:44.292 in Ping 
15:30:49.236 in Interrupt 41
15:30:49.296 in Ping 
15:30:50.241 fetch POST /api/observability/v1/traces
15:30:50.241 fetched POST /api/observability/v1/traces 204
15:30:54.301 in Ping 
15:30:59.233 rpc 42 server.reportClientActivity
15:30:59.305 in Ping 
15:31:00.236 fetch POST /api/observability/v1/traces
15:31:00.236 fetched POST /api/observability/v1/traces 204
15:31:04.308 in Ping 
15:31:09.312 in Ping 
15:31:14.237 in Interrupt 42
15:31:14.317 in Ping 
15:31:15.241 fetch POST /api/observability/v1/traces
15:31:15.241 fetched POST /api/observability/v1/traces 204
15:31:19.321 in Ping 
15:31:24.239 rpc 43 server.reportClientActivity
15:31:24.324 in Ping 
15:31:25.241 fetch POST /api/observability/v1/traces
15:31:25.241 fetched POST /api/observability/v1/traces 204
15:31:29.328 in Ping 
15:31:34.335 in Ping 
15:31:39.244 in Interrupt 43
15:31:39.337 in Ping 
15:31:40.249 fetch POST /api/observability/v1/traces
15:31:40.249 fetched POST /api/observability/v1/traces 204
15:31:44.340 in Ping 
15:31:49.241 rpc 44 server.reportClientActivity
15:31:49.344 in Ping 
15:31:50.244 fetch POST /api/observability/v1/traces
15:31:50.244 fetched POST /api/observability/v1/traces 204
15:31:54.348 in Ping 
15:31:59.352 in Ping 
15:32:04.244 in Interrupt 44
15:32:04.356 in Ping 
15:32:05.249 fetch POST /api/observability/v1/traces
15:32:05.249 fetched POST /api/observability/v1/traces 204
15:32:09.360 in Ping 
15:32:14.256 rpc 45 server.reportClientActivity
15:32:14.364 in Ping 
15:32:15.357 fetch POST /api/observability/v1/traces
15:32:15.357 fetched POST /api/observability/v1/traces 204
15:32:19.396 in Ping 
15:32:24.400 in Ping 
15:32:29.458 in Interrupt 45
15:32:29.650 in Ping 
15:32:30.568 fetch POST /api/observability/v1/traces
15:32:30.571 fetched POST /api/observability/v1/traces 204
15:32:34.652 in Ping 
15:32:39.420 rpc 46 server.reportClientActivity
15:32:39.685 in Ping 
15:32:40.354 fetch POST /api/observability/v1/traces
15:32:40.354 fetched POST /api/observability/v1/traces 204
15:32:44.688 in Ping 
15:32:49.694 in Ping 
15:32:54.440 in Interrupt 46
15:32:54.697 in Ping 
15:32:55.501 fetch POST /api/observability/v1/traces
15:32:55.501 fetched POST /api/observability/v1/traces 204
15:32:59.700 in Ping 
15:33:04.261 rpc 47 server.reportClientActivity
15:33:04.704 in Ping 
15:33:05.265 fetch POST /api/observability/v1/traces
15:33:05.265 fetched POST /api/observability/v1/traces 204
15:33:09.708 in Ping 
15:33:14.713 in Ping 
15:33:19.265 in Interrupt 47
15:33:19.716 in Ping 
15:33:20.269 fetch POST /api/observability/v1/traces
15:33:20.269 fetched POST /api/observability/v1/traces 204
15:33:24.721 in Ping 
15:33:29.275 rpc 48 server.reportClientActivity
15:33:29.726 in Ping 
15:33:30.273 fetch POST /api/observability/v1/traces
15:33:30.273 fetched POST /api/observability/v1/traces 204
15:33:34.737 in Ping 
15:33:39.740 in Ping 
15:33:44.277 in Interrupt 48
15:33:44.744 in Ping 
15:33:45.282 fetch POST /api/observability/v1/traces
15:33:45.282 fetched POST /api/observability/v1/traces 204
15:33:49.748 in Ping 
15:33:54.274 rpc 49 server.reportClientActivity
15:33:54.757 in Ping 
15:33:55.271 fetch POST /api/observability/v1/traces
15:33:55.271 fetched POST /api/observability/v1/traces 204
15:33:59.761 in Ping 
15:34:04.764 in Ping 
15:34:09.277 in Interrupt 49
15:34:09.768 in Ping 
15:34:10.281 fetch POST /api/observability/v1/traces
15:34:10.281 fetched POST /api/observability/v1/traces 204
15:34:14.775 in Ping 
15:34:19.270 rpc 50 server.reportClientActivity
15:34:19.776 in Ping 
15:34:20.273 fetch POST /api/observability/v1/traces
15:34:20.273 fetched POST /api/observability/v1/traces 204
15:34:24.780 in Ping 
15:34:29.784 in Ping 
15:34:34.277 in Interrupt 50
15:34:34.797 in Ping 
15:34:35.289 fetch POST /api/observability/v1/traces
15:34:35.289 fetched POST /api/observability/v1/traces 204
15:34:39.800 in Ping 
15:34:44.286 rpc 51 server.reportClientActivity
15:34:44.809 in Ping 
15:34:45.285 fetch POST /api/observability/v1/traces
15:34:45.285 fetched POST /api/observability/v1/traces 204
15:34:49.812 in Ping 
15:34:54.832 in Ping 
15:34:59.301 in Interrupt 51
15:34:59.836 in Ping 
15:35:06.693 fetch POST /api/observability/v1/traces
15:35:06.694 fetched POST /api/observability/v1/traces 204
15:35:06.742 in Ping 
15:35:09.465 rpc 52 server.reportClientActivity
15:35:10.403 fetch POST /api/observability/v1/traces
15:35:10.403 fetched POST /api/observability/v1/traces 204
15:35:11.744 in Ping 
15:35:16.748 in Ping 
15:35:21.753 in Ping 
15:35:24.477 in Interrupt 52
15:35:25.480 fetch POST /api/observability/v1/traces
15:35:25.480 fetched POST /api/observability/v1/traces 204
15:35:26.756 in Ping 
15:35:31.761 in Ping 
15:35:34.323 rpc 53 server.reportClientActivity
15:35:35.335 fetch POST /api/observability/v1/traces
15:35:35.341 fetched POST /api/observability/v1/traces 204
15:35:36.766 in Ping 
15:35:41.768 in Ping 
15:35:46.898 in Ping 
15:35:49.329 in Interrupt 53
15:35:50.333 fetch POST /api/observability/v1/traces
15:35:50.334 fetched POST /api/observability/v1/traces 204
15:35:51.900 in Ping 
15:35:56.904 in Ping 
15:35:59.364 rpc 54 server.reportClientActivity
15:36:00.395 fetch POST /api/observability/v1/traces
15:36:00.395 fetched POST /api/observability/v1/traces 204
15:36:01.909 in Ping 
15:36:06.917 in Ping 
15:36:11.920 in Ping 
15:36:14.372 in Interrupt 54
15:36:15.381 fetch POST /api/observability/v1/traces
15:36:15.381 fetched POST /api/observability/v1/traces 204
15:36:16.924 in Ping 
15:36:21.931 in Ping 
15:36:24.361 rpc 55 server.reportClientActivity
15:36:25.383 fetch POST /api/observability/v1/traces
15:36:25.383 fetched POST /api/observability/v1/traces 204
15:36:26.932 in Ping 
15:36:31.939 in Ping 
15:36:36.944 in Ping 
15:36:39.368 in Interrupt 55
15:36:40.372 fetch POST /api/observability/v1/traces
15:36:40.372 fetched POST /api/observability/v1/traces 204
15:36:41.967 in Ping 
15:36:46.968 in Ping 
15:36:49.367 rpc 56 server.reportClientActivity
15:36:50.369 fetch POST /api/observability/v1/traces
15:36:50.369 fetched POST /api/observability/v1/traces 204
15:36:51.974 in Ping 
15:36:56.976 in Ping 
15:37:01.980 in Ping 
15:37:04.369 in Interrupt 56
15:37:05.373 fetch POST /api/observability/v1/traces
15:37:05.373 fetched POST /api/observability/v1/traces 204
15:37:06.984 in Ping 
15:37:11.988 in Ping 
15:37:14.376 rpc 57 server.reportClientActivity
15:37:15.387 fetch POST /api/observability/v1/traces
15:37:15.388 fetched POST /api/observability/v1/traces 204
15:37:16.992 in Ping 
15:37:21.996 in Ping 
15:37:27.000 in Ping 
15:37:29.381 in Interrupt 57
15:37:30.385 fetch POST /api/observability/v1/traces
15:37:30.385 fetched POST /api/observability/v1/traces 204
15:37:32.004 in Ping 
15:37:37.009 in Ping 
15:37:39.621 rpc 58 server.reportClientActivity
15:37:40.622 fetch POST /api/observability/v1/traces
15:37:40.622 fetched POST /api/observability/v1/traces 204
15:37:42.012 in Ping 
15:37:47.017 in Ping 
15:37:52.020 in Ping 
15:37:54.627 in Interrupt 58
15:37:55.629 fetch POST /api/observability/v1/traces
15:37:55.629 fetched POST /api/observability/v1/traces 204
15:37:57.033 in Ping 
15:38:02.076 in Ping 
15:38:04.616 rpc 59 server.reportClientActivity
15:38:05.616 fetch POST /api/observability/v1/traces
15:38:05.616 fetched POST /api/observability/v1/traces 204
15:38:07.080 in Ping 
15:38:12.084 in Ping 
15:38:17.088 in Ping 
15:38:19.621 in Interrupt 59
15:38:20.626 fetch POST /api/observability/v1/traces
15:38:20.626 fetched POST /api/observability/v1/traces 204
15:38:22.092 in Ping 
15:38:27.096 in Ping 
15:38:29.614 rpc 60 server.reportClientActivity
15:38:30.637 fetch POST /api/observability/v1/traces
15:38:30.638 fetched POST /api/observability/v1/traces 204
15:38:32.100 in Ping 
15:38:37.104 in Ping 
15:38:42.108 in Ping 
15:38:44.635 in Interrupt 60
15:38:45.648 fetch POST /api/observability/v1/traces
15:38:45.649 fetched POST /api/observability/v1/traces 204
15:38:47.112 in Ping 
15:38:52.116 in Ping 
15:38:54.619 rpc 61 server.reportClientActivity
15:38:55.620 fetch POST /api/observability/v1/traces
15:38:55.620 fetched POST /api/observability/v1/traces 204
15:38:57.142 in Ping 
15:39:02.144 in Ping 
15:39:07.148 in Ping 
15:39:09.620 in Interrupt 61
15:39:10.624 fetch POST /api/observability/v1/traces
15:39:10.624 fetched POST /api/observability/v1/traces 204
15:39:12.152 in Ping 
15:39:17.156 in Ping 
15:39:19.622 rpc 62 server.reportClientActivity
15:39:20.625 fetch POST /api/observability/v1/traces
15:39:20.625 fetched POST /api/observability/v1/traces 204
15:39:22.160 in Ping 
15:39:27.165 in Ping 
15:39:32.168 in Ping 
15:39:34.624 in Interrupt 62
15:39:35.629 fetch POST /api/observability/v1/traces
15:39:35.629 fetched POST /api/observability/v1/traces 204
15:39:37.172 in Ping 
15:39:42.176 in Ping 
15:39:44.626 rpc 63 server.reportClientActivity
15:39:45.630 fetch POST /api/observability/v1/traces
15:39:45.630 fetched POST /api/observability/v1/traces 204
15:39:47.180 in Ping 
15:39:52.184 in Ping 
15:39:57.188 in Ping 
15:39:59.629 in Interrupt 63
15:40:00.633 fetch POST /api/observability/v1/traces
15:40:00.633 fetched POST /api/observability/v1/traces 204
15:40:02.195 in Ping 
15:40:07.196 in Ping 
15:40:09.629 rpc 64 server.reportClientActivity
15:40:10.632 fetch POST /api/observability/v1/traces
15:40:10.632 fetched POST /api/observability/v1/traces 204
15:40:12.200 in Ping 
15:40:17.208 in Ping 
15:40:22.212 in Ping 
15:40:24.636 in Interrupt 64
15:40:25.641 fetch POST /api/observability/v1/traces
15:40:25.641 fetched POST /api/observability/v1/traces 204
15:40:27.216 in Ping 
15:40:32.220 in Ping 
15:40:34.635 rpc 65 server.reportClientActivity
15:40:35.637 fetch POST /api/observability/v1/traces
15:40:35.637 fetched POST /api/observability/v1/traces 204
15:40:37.224 in Ping 
15:40:42.228 in Ping 
15:40:47.232 in Ping 
15:40:49.636 in Interrupt 65
15:40:50.640 fetch POST /api/observability/v1/traces
15:40:50.640 fetched POST /api/observability/v1/traces 204
15:40:52.236 in Ping 
15:40:57.240 in Ping 
15:40:59.638 rpc 66 server.reportClientActivity
15:41:00.640 fetch POST /api/observability/v1/traces
15:41:00.640 fetched POST /api/observability/v1/traces 204
15:41:02.244 in Ping 
15:41:07.248 in Ping 
