# E-iid, webapp, warm start, seed 1

Profile E-iid (fault): E with independent 75% loss instead of bursts. Commit 0f01ba755fce-dirty.

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
| Longest silence | 899.0s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 103558 / 39342 |
| Bytes the app received / sent | 3501 / 82457 |

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

- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151072431,"silences":0,"longestSilenceMs":685,"longestRecoveredGapMs":685},"stats":{"sendRate":15187.5,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":1,"sentPackets":99,"sentWireBytes":33059,"receivedPackets":107,"receivedWireBytes":69414,"lastHeardMs":7450,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151082966,"silences":1,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":122,"sentWireBytes":42769,"receivedPackets":108,"receivedWireBytes":70530,"lastHeardMs":17985,"unsyncedBytes":509,"inflightPackets":23,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151091854,"silences":2,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":137,"sentWireBytes":49455,"receivedPackets":109,"receivedWireBytes":71608,"lastHeardMs":26873,"unsyncedBytes":1005,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151102285,"silences":5,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":137,"sentWireBytes":49455,"receivedPackets":112,"receivedWireBytes":72623,"lastHeardMs":37304,"unsyncedBytes":1005,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":137,"sentWireBytes":49455,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1005,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":147,"sentWireBytes":53064,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1501,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":162,"sentWireBytes":58855,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1501,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":176,"sentWireBytes":65115,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1998,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":183,"sentWireBytes":68245,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1998,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 9","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":197,"sentWireBytes":74505,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":1998,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 10","open":true,"generation":2,"iceRestarts":10,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":211,"sentWireBytes":80765,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":2495,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 11","open":true,"generation":2,"iceRestarts":11,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":218,"sentWireBytes":83895,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":2495,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 13","open":true,"generation":2,"iceRestarts":13,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":232,"sentWireBytes":90500,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":3158,"inflightPackets":70,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":243,"sentWireBytes":96099,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":3158,"inflightPackets":71,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 14","open":true,"generation":2,"iceRestarts":14,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":255,"sentWireBytes":102039,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":3158,"inflightPackets":69,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 15","open":true,"generation":2,"iceRestarts":15,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":264,"sentWireBytes":106494,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":3655,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 16","open":true,"generation":2,"iceRestarts":16,"newConnections":0,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":273,"sentWireBytes":110949,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":3655,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":4152,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":4152,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":4649,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":4649,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":4649,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":5146,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":5146,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":5643,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":5643,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":5643,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":6140,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":6140,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":6637,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":6637,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":6637,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":7134,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":7134,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":7631,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":7631,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":7631,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":8128,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":8128,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":8625,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":8625,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":8625,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":9122,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":9122,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":9619,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":9619,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":9619,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":10116,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":10116,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":10613,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":10613,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":10613,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":11110,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":11110,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":11607,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":11607,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":11607,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":12104,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":12104,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":12601,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":12601,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":12601,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":13098,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":13098,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":13595,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":13595,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":14092,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":14092,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":14092,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":14589,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":14589,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":15086,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":15086,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":15086,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":15583,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":15583,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":16080,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":16080,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":16080,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":16577,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":16577,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":17074,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":17074,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":17074,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":17571,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":17571,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":18068,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":18068,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":17,"newConnections":1,"handshakeMs":412,"lastDataAt":1794151104867,"silences":6,"longestSilenceMs":10535,"longestRecoveredGapMs":10535},"stats":{"sendRate":2000,"deliveredRate":5087.834685691366,"loss":0,"srttMs":5.540202179452919,"minRttMs":null,"sentPackets":282,"sentWireBytes":115404,"receivedPackets":113,"receivedWireBytes":72915,"lastHeardMs":39886,"unsyncedBytes":18068,"inflightPackets":64,"duplicateBytes":0}}
- shim trace
15:17:45.533 fetch GET /api/auth/session
15:17:45.534 fetched GET /api/auth/session 200
15:17:45.687 fetch GET /.well-known/t3/environment
15:17:45.688 fetched GET /.well-known/t3/environment 200
15:17:45.699 fetch GET /.well-known/t3/environment
15:17:45.700 fetched GET /.well-known/t3/environment 200
15:17:45.703 rpc 0 subscribeServerConfig
15:17:45.703 chunk 0 38706 bytes
15:17:45.726 in Ack 0
15:17:45.728 rpc 1 subscribeServerLifecycle
15:17:45.729 rpc 2 subscribeServerLifecycle
15:17:45.733 fetch GET /api/auth/session
15:17:45.735 fetched GET /api/auth/session 200
15:17:45.735 rpc 3 subscribeDeviceState
15:17:45.738 rpc 4 preview.reportProfiles
15:17:45.785 rpc 5 subscribeProjectClones
15:17:45.791 fetch GET /api/orchestration/shell
15:17:45.814 rpc 6 server.refreshProviders
15:17:45.902 rpc 7 orchestration.dispatchCommand thread.visit
15:17:45.914 fetched GET /api/orchestration/shell 200
15:17:45.915 rpc 8 subscribeTerminalMetadata
15:17:45.915 rpc 9 subscribeVcsStatus
15:17:45.916 rpc 10 subscribePreviewEvents
15:17:45.920 rpc 11 orchestration.subscribeShell
15:17:45.920 chunk 11 2786 bytes
15:17:45.921 rpc 12 assets.createUrl
15:17:45.922 rpc 13 preview.list
15:17:45.922 rpc 14 projects.readFile
15:17:45.926 fetch GET /api/orchestration/threads/mcp%3Af6b24930-fd9b-4fa6-bc5a-a8fcff29bbfd/bounded
15:17:45.930 in Ack 11
15:17:46.110 exit 7 {"_tag":"Success","value":{"sequence":38}}
15:17:46.169 rpc 15 server.reportClientActivity
15:17:46.614 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f6c0eac6-bbde-4f2f-96c2-2d4f7c6413ea","sessions":[]}}
15:17:46.614 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:17:46.614 chunk 1 3868 bytes
15:17:46.620 in Ack 1
15:17:46.687 fetch POST /api/observability/v1/traces
15:17:46.687 fetched POST /api/observability/v1/traces 204
15:17:46.752 chunk 2 3868 bytes
15:17:46.753 in Ack 2
15:17:46.880 chunk 11 2762 bytes
15:17:46.880 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:17:31.414Z","configurableProviders":false,"con
15:17:46.880 chunk 9 1427 bytes
15:17:46.880 exit 12 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:17:46.880 in Ack 11
15:17:46.889 in Ack 9
15:17:47.129 chunk 11 2805 bytes
15:17:47.130 in Ack 11
15:17:47.198 chunk 3 876 bytes
15:17:47.198 exit 4 {"_tag":"Success","value":null}
15:17:47.198 chunk 5 4 bytes
15:17:47.198 chunk 8 36 bytes
15:17:47.198 exit 15 {"_tag":"Success","value":null}
15:17:47.198 in Ack 3
15:17:47.202 in Ack 5
15:17:47.202 in Ack 8
15:17:47.469 fetched GET /api/orchestration/threads/mcp%3Af6b24930-fd9b-4fa6-bc5a-a8fcff29bbfd/bounded 200
15:17:47.472 rpc 16 orchestration.subscribeThread
15:17:47.473 chunk 16 8305 bytes
15:17:47.484 in Interrupt 9
15:17:47.486 rpc 17 subscribeVcsStatus
15:17:47.487 in Ack 16
15:17:47.600 chunk 17 1427 bytes
15:17:47.600 in Ack 17
15:17:47.725 chunk 16 11282 bytes
15:17:47.727 in Ack 16
15:17:47.792 rpc 18 server.reportClientActivity
15:17:47.792 fetch POST /api/observability/v1/traces
15:17:47.793 fetched POST /api/observability/v1/traces 204
15:17:47.980 chunk 16 13532 bytes
15:17:47.982 in Ack 16
15:17:48.288 chunk 16 18059 bytes
15:17:48.289 in Ack 16
15:17:48.539 chunk 16 20501 bytes
15:17:48.540 in Ack 16
15:17:48.548 in Interrupt 17
15:17:48.550 rpc 19 subscribeVcsStatus
15:17:48.559 exit 18 {"_tag":"Success","value":null}
15:17:48.669 chunk 19 1427 bytes
15:17:48.671 in Ack 19
15:17:48.719 rpc 20 attachments.createUploadUrl
15:17:48.915 chunk 16 21231 bytes
15:17:48.916 rpc 21 server.reportClientActivity
15:17:48.924 in Ack 16
15:17:48.978 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e0c2d781-f80b-4457-a9d8-12560c830054","expiresAt":1794151668969,"relativeUrl":"/api/attachments/upload/eyJ2ZX
15:17:48.978 exit 21 {"_tag":"Success","value":null}
15:17:48.980 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWUwYzJkNzgxLWY4MGItNDQ1Ny1hOWQ4LTEyNTYwYzgzMDA1NCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1MTY2ODk2OX0.yYXKYI8in89560Cz1oSVLsV9E6tG0J56ymVMcGvF8zk
15:17:48.983 fetch POST /api/observability/v1/traces
15:17:48.983 fetched POST /api/observability/v1/traces 204
15:17:50.041 rpc 22 orchestration.dispatchCommand message.dispatch
15:17:50.130 exit 22 {"_tag":"Success","value":{"sequence":45}}
15:17:50.131 rpc 23 attachments.delete
15:17:50.238 exit 23 {"_tag":"Success","value":null}
15:17:50.975 chunk 11 948 bytes
15:17:50.975 chunk 16 27993 bytes
15:17:50.977 in Ping 
15:17:50.978 in Ack 11
15:17:50.985 in Ack 16
15:17:50.988 rpc 24 assets.createUrl
15:17:51.044 fetch POST /api/observability/v1/traces
15:17:51.044 fetched POST /api/observability/v1/traces 204
15:17:51.226 chunk 11 2827 bytes
15:17:51.227 in Ack 11
15:17:51.228 rpc 25 orchestration.dispatchCommand thread.visit
15:17:51.328 exit 25 {"_tag":"Success","value":{"sequence":68}}
15:17:51.478 chunk 11 2827 bytes
15:17:51.479 chunk 16 27486 bytes
15:17:51.480 in Ack 11
15:17:51.492 in Ack 16
15:17:51.728 chunk 16 32162 bytes
15:17:51.729 in Ack 16
15:17:51.737 in Interrupt 19
15:17:51.744 rpc 26 subscribeVcsStatus
15:17:51.778 chunk 26 1427 bytes
15:17:51.787 in Ack 26
15:17:51.978 chunk 16 32775 bytes
15:17:51.979 in Ack 16
15:17:51.996 rpc 27 server.reportClientActivity
15:17:52.192 exit 27 {"_tag":"Success","value":null}
15:17:52.433 fetch POST /api/observability/v1/traces
15:17:52.433 fetched POST /api/observability/v1/traces 204
15:17:52.433 chunk 16 35284 bytes
15:17:52.435 in Ack 16
15:17:53.455 fetch POST /api/observability/v1/traces
15:17:53.455 fetched POST /api/observability/v1/traces 204
15:17:53.607 rpc 28 orchestration.dispatchCommand message.dispatch
15:17:54.607 fetch POST /api/observability/v1/traces
15:17:54.607 fetched POST /api/observability/v1/traces 204
15:17:55.977 in Ping 
15:17:57.498 fetch POST /api/observability/v1/traces
15:17:57.498 fetched POST /api/observability/v1/traces 204
15:17:59.010 fetch POST /api/observability/v1/traces
15:17:59.010 fetched POST /api/observability/v1/traces 204
15:18:00.978 in Ping 
15:18:05.981 in Ping 
15:18:10.940 rpc 29 server.reportClientActivity
15:18:10.982 in Ping 
15:18:11.940 fetch POST /api/observability/v1/traces
15:18:11.940 fetched POST /api/observability/v1/traces 204
15:18:15.982 in Ping 
15:18:20.983 in Ping 
15:18:25.944 in Interrupt 29
15:18:25.984 in Ping 
15:18:26.945 fetch POST /api/observability/v1/traces
15:18:26.945 fetched POST /api/observability/v1/traces 204
15:18:30.985 in Ping 
15:18:35.941 rpc 30 server.reportClientActivity
15:18:35.985 in Ping 
15:18:36.942 fetch POST /api/observability/v1/traces
15:18:36.942 fetched POST /api/observability/v1/traces 204
15:18:40.988 in Ping 
15:18:45.992 in Ping 
15:18:50.942 in Interrupt 30
15:18:50.996 in Ping 
15:18:51.944 fetch POST /api/observability/v1/traces
15:18:51.944 fetched POST /api/observability/v1/traces 204
15:18:56.000 in Ping 
15:19:00.942 rpc 31 server.reportClientActivity
15:19:01.005 in Ping 
15:19:01.944 fetch POST /api/observability/v1/traces
15:19:01.944 fetched POST /api/observability/v1/traces 204
15:19:06.008 in Ping 
15:19:11.031 in Ping 
15:19:15.943 in Interrupt 31
15:19:16.039 in Ping 
15:19:16.945 fetch POST /api/observability/v1/traces
15:19:16.945 fetched POST /api/observability/v1/traces 204
15:19:21.055 in Ping 
15:19:25.941 rpc 32 server.reportClientActivity
15:19:26.060 in Ping 
15:19:26.945 fetch POST /api/observability/v1/traces
15:19:26.945 fetched POST /api/observability/v1/traces 204
15:19:31.064 in Ping 
15:19:36.068 in Ping 
15:19:40.942 in Interrupt 32
15:19:41.072 in Ping 
15:19:41.944 fetch POST /api/observability/v1/traces
15:19:41.944 fetched POST /api/observability/v1/traces 204
15:19:46.098 in Ping 
15:19:50.944 rpc 33 server.reportClientActivity
15:19:51.100 in Ping 
15:19:51.945 fetch POST /api/observability/v1/traces
15:19:51.945 fetched POST /api/observability/v1/traces 204
15:19:56.104 in Ping 
15:20:01.108 in Ping 
15:20:05.946 in Interrupt 33
15:20:06.112 in Ping 
15:20:06.947 fetch POST /api/observability/v1/traces
15:20:06.947 fetched POST /api/observability/v1/traces 204
15:20:11.118 in Ping 
15:20:15.945 rpc 34 server.reportClientActivity
15:20:16.122 in Ping 
15:20:16.946 fetch POST /api/observability/v1/traces
15:20:16.946 fetched POST /api/observability/v1/traces 204
15:20:21.126 in Ping 
15:20:26.128 in Ping 
15:20:30.946 in Interrupt 34
15:20:31.132 in Ping 
15:20:31.947 fetch POST /api/observability/v1/traces
15:20:31.947 fetched POST /api/observability/v1/traces 204
15:20:36.136 in Ping 
15:20:40.943 rpc 35 server.reportClientActivity
15:20:41.140 in Ping 
15:20:41.943 fetch POST /api/observability/v1/traces
15:20:41.943 fetched POST /api/observability/v1/traces 204
15:20:46.144 in Ping 
15:20:51.148 in Ping 
15:20:55.944 in Interrupt 35
15:20:56.152 in Ping 
15:20:56.947 fetch POST /api/observability/v1/traces
15:20:56.947 fetched POST /api/observability/v1/traces 204
15:21:01.156 in Ping 
15:21:05.943 rpc 36 server.reportClientActivity
15:21:06.160 in Ping 
15:21:06.943 fetch POST /api/observability/v1/traces
15:21:06.943 fetched POST /api/observability/v1/traces 204
15:21:11.166 in Ping 
15:21:16.168 in Ping 
15:21:20.943 in Interrupt 36
15:21:21.173 in Ping 
15:21:21.949 fetch POST /api/observability/v1/traces
15:21:21.949 fetched POST /api/observability/v1/traces 204
15:21:26.176 in Ping 
15:21:30.944 rpc 37 server.reportClientActivity
15:21:31.180 in Ping 
15:21:31.944 fetch POST /api/observability/v1/traces
15:21:31.944 fetched POST /api/observability/v1/traces 204
15:21:36.184 in Ping 
15:21:41.188 in Ping 
15:21:45.955 in Interrupt 37
15:21:46.193 in Ping 
15:21:46.958 fetch POST /api/observability/v1/traces
15:21:46.958 fetched POST /api/observability/v1/traces 204
15:21:51.196 in Ping 
15:21:55.945 rpc 38 server.reportClientActivity
15:21:56.200 in Ping 
15:21:56.950 fetch POST /api/observability/v1/traces
15:21:56.950 fetched POST /api/observability/v1/traces 204
15:22:01.204 in Ping 
15:22:06.208 in Ping 
15:22:10.949 in Interrupt 38
15:22:11.213 in Ping 
15:22:11.953 fetch POST /api/observability/v1/traces
15:22:11.953 fetched POST /api/observability/v1/traces 204
15:22:16.217 in Ping 
15:22:20.954 rpc 39 server.reportClientActivity
15:22:21.220 in Ping 
15:22:21.957 fetch POST /api/observability/v1/traces
15:22:21.957 fetched POST /api/observability/v1/traces 204
15:22:26.228 in Ping 
15:22:31.232 in Ping 
15:22:35.956 in Interrupt 39
15:22:36.236 in Ping 
15:22:36.961 fetch POST /api/observability/v1/traces
15:22:36.961 fetched POST /api/observability/v1/traces 204
15:22:41.240 in Ping 
15:22:45.958 rpc 40 server.reportClientActivity
15:22:46.244 in Ping 
15:22:46.960 fetch POST /api/observability/v1/traces
15:22:46.960 fetched POST /api/observability/v1/traces 204
15:22:51.248 in Ping 
15:22:56.252 in Ping 
15:23:00.962 in Interrupt 40
15:23:01.256 in Ping 
15:23:01.966 fetch POST /api/observability/v1/traces
15:23:01.966 fetched POST /api/observability/v1/traces 204
15:23:06.261 in Ping 
15:23:10.959 rpc 41 server.reportClientActivity
15:23:11.266 in Ping 
15:23:11.961 fetch POST /api/observability/v1/traces
15:23:11.962 fetched POST /api/observability/v1/traces 204
15:23:16.268 in Ping 
15:23:21.272 in Ping 
15:23:25.960 in Interrupt 41
15:23:26.278 in Ping 
15:23:26.969 fetch POST /api/observability/v1/traces
15:23:26.969 fetched POST /api/observability/v1/traces 204
15:23:31.286 in Ping 
15:23:35.965 rpc 42 server.reportClientActivity
15:23:36.289 in Ping 
15:23:36.968 fetch POST /api/observability/v1/traces
15:23:36.968 fetched POST /api/observability/v1/traces 204
15:23:41.292 in Ping 
15:23:46.296 in Ping 
15:23:50.968 in Interrupt 42
15:23:51.300 in Ping 
15:23:51.972 fetch POST /api/observability/v1/traces
15:23:51.972 fetched POST /api/observability/v1/traces 204
15:23:56.304 in Ping 
15:24:00.971 rpc 43 server.reportClientActivity
15:24:01.308 in Ping 
15:24:01.973 fetch POST /api/observability/v1/traces
15:24:01.973 fetched POST /api/observability/v1/traces 204
15:24:06.312 in Ping 
15:24:11.316 in Ping 
15:24:15.977 in Interrupt 43
15:24:16.320 in Ping 
15:24:16.982 fetch POST /api/observability/v1/traces
15:24:16.982 fetched POST /api/observability/v1/traces 204
15:24:21.326 in Ping 
15:24:25.973 rpc 44 server.reportClientActivity
15:24:26.328 in Ping 
15:24:26.978 fetch POST /api/observability/v1/traces
15:24:26.978 fetched POST /api/observability/v1/traces 204
15:24:31.332 in Ping 
15:24:36.336 in Ping 
15:24:40.987 in Interrupt 44
15:24:41.340 in Ping 
15:24:41.994 fetch POST /api/observability/v1/traces
15:24:41.994 fetched POST /api/observability/v1/traces 204
15:24:46.344 in Ping 
15:24:50.990 rpc 45 server.reportClientActivity
15:24:51.348 in Ping 
15:24:51.998 fetch POST /api/observability/v1/traces
15:24:51.998 fetched POST /api/observability/v1/traces 204
15:24:56.352 in Ping 
15:25:01.356 in Ping 
15:25:05.993 in Interrupt 45
15:25:06.366 in Ping 
15:25:06.996 fetch POST /api/observability/v1/traces
15:25:06.996 fetched POST /api/observability/v1/traces 204
15:25:11.371 in Ping 
15:25:15.981 rpc 46 server.reportClientActivity
15:25:16.377 in Ping 
15:25:16.984 fetch POST /api/observability/v1/traces
15:25:16.984 fetched POST /api/observability/v1/traces 204
15:25:21.380 in Ping 
15:25:26.384 in Ping 
15:25:30.984 in Interrupt 46
15:25:31.388 in Ping 
15:25:31.989 fetch POST /api/observability/v1/traces
15:25:31.989 fetched POST /api/observability/v1/traces 204
15:25:36.395 in Ping 
15:25:40.985 rpc 47 server.reportClientActivity
15:25:41.400 in Ping 
15:25:41.988 fetch POST /api/observability/v1/traces
15:25:41.988 fetched POST /api/observability/v1/traces 204
15:25:46.404 in Ping 
15:25:51.413 in Ping 
15:25:55.988 in Interrupt 47
15:25:56.416 in Ping 
15:25:56.992 fetch POST /api/observability/v1/traces
15:25:56.992 fetched POST /api/observability/v1/traces 204
15:26:01.420 in Ping 
15:26:05.989 rpc 48 server.reportClientActivity
15:26:06.424 in Ping 
15:26:06.992 fetch POST /api/observability/v1/traces
15:26:06.992 fetched POST /api/observability/v1/traces 204
15:26:11.428 in Ping 
15:26:16.432 in Ping 
15:26:20.992 in Interrupt 48
15:26:21.436 in Ping 
15:26:21.997 fetch POST /api/observability/v1/traces
15:26:21.997 fetched POST /api/observability/v1/traces 204
15:26:26.444 in Ping 
15:26:30.993 rpc 49 server.reportClientActivity
15:26:31.448 in Ping 
15:26:31.997 fetch POST /api/observability/v1/traces
15:26:31.997 fetched POST /api/observability/v1/traces 204
15:26:36.452 in Ping 
15:26:41.456 in Ping 
15:26:45.996 in Interrupt 49
15:26:46.461 in Ping 
15:26:47.000 fetch POST /api/observability/v1/traces
15:26:47.000 fetched POST /api/observability/v1/traces 204
15:26:51.464 in Ping 
15:26:55.997 rpc 50 server.reportClientActivity
15:26:56.468 in Ping 
15:26:57.000 fetch POST /api/observability/v1/traces
15:26:57.000 fetched POST /api/observability/v1/traces 204
15:27:01.472 in Ping 
15:27:06.478 in Ping 
15:27:11.000 in Interrupt 50
15:27:11.480 in Ping 
15:27:12.005 fetch POST /api/observability/v1/traces
15:27:12.005 fetched POST /api/observability/v1/traces 204
15:27:16.485 in Ping 
15:27:21.003 rpc 51 server.reportClientActivity
15:27:21.488 in Ping 
15:27:22.005 fetch POST /api/observability/v1/traces
15:27:22.005 fetched POST /api/observability/v1/traces 204
15:27:26.493 in Ping 
15:27:31.496 in Ping 
15:27:36.011 in Interrupt 51
15:27:36.500 in Ping 
15:27:37.018 fetch POST /api/observability/v1/traces
15:27:37.018 fetched POST /api/observability/v1/traces 204
15:27:41.504 in Ping 
15:27:46.005 rpc 52 server.reportClientActivity
15:27:46.508 in Ping 
15:27:47.008 fetch POST /api/observability/v1/traces
15:27:47.008 fetched POST /api/observability/v1/traces 204
15:27:51.512 in Ping 
15:27:56.518 in Ping 
15:28:01.008 in Interrupt 52
15:28:01.520 in Ping 
15:28:02.014 fetch POST /api/observability/v1/traces
15:28:02.014 fetched POST /api/observability/v1/traces 204
15:28:06.524 in Ping 
15:28:11.012 rpc 53 server.reportClientActivity
15:28:11.528 in Ping 
15:28:12.016 fetch POST /api/observability/v1/traces
15:28:12.016 fetched POST /api/observability/v1/traces 204
15:28:16.534 in Ping 
15:28:21.536 in Ping 
15:28:26.016 in Interrupt 53
15:28:26.540 in Ping 
15:28:27.020 fetch POST /api/observability/v1/traces
15:28:27.021 fetched POST /api/observability/v1/traces 204
15:28:31.544 in Ping 
15:28:36.022 rpc 54 server.reportClientActivity
15:28:36.548 in Ping 
15:28:37.051 fetch POST /api/observability/v1/traces
15:28:37.051 fetched POST /api/observability/v1/traces 204
15:28:41.552 in Ping 
15:28:46.556 in Ping 
15:28:51.050 in Interrupt 54
15:28:51.560 in Ping 
15:28:52.092 fetch POST /api/observability/v1/traces
15:28:52.092 fetched POST /api/observability/v1/traces 204
15:28:56.564 in Ping 
15:29:01.038 rpc 55 server.reportClientActivity
15:29:01.568 in Ping 
15:29:02.040 fetch POST /api/observability/v1/traces
15:29:02.040 fetched POST /api/observability/v1/traces 204
15:29:06.572 in Ping 
15:29:11.576 in Ping 
15:29:16.081 in Interrupt 55
15:29:16.580 in Ping 
15:29:17.085 fetch POST /api/observability/v1/traces
15:29:17.085 fetched POST /api/observability/v1/traces 204
15:29:21.584 in Ping 
15:29:26.044 rpc 56 server.reportClientActivity
15:29:26.590 in Ping 
15:29:27.052 fetch POST /api/observability/v1/traces
15:29:27.052 fetched POST /api/observability/v1/traces 204
15:29:31.593 in Ping 
15:29:36.596 in Ping 
15:29:41.048 in Interrupt 56
15:29:41.600 in Ping 
15:29:42.052 fetch POST /api/observability/v1/traces
15:29:42.052 fetched POST /api/observability/v1/traces 204
15:29:46.606 in Ping 
15:29:51.029 rpc 57 server.reportClientActivity
15:29:51.608 in Ping 
15:29:52.055 fetch POST /api/observability/v1/traces
15:29:52.055 fetched POST /api/observability/v1/traces 204
15:29:56.612 in Ping 
15:30:01.616 in Ping 
15:30:06.032 in Interrupt 57
15:30:06.620 in Ping 
15:30:07.037 fetch POST /api/observability/v1/traces
15:30:07.037 fetched POST /api/observability/v1/traces 204
15:30:11.624 in Ping 
15:30:16.056 rpc 58 server.reportClientActivity
15:30:16.628 in Ping 
15:30:17.060 fetch POST /api/observability/v1/traces
15:30:17.060 fetched POST /api/observability/v1/traces 204
15:30:21.632 in Ping 
15:30:26.636 in Ping 
15:30:31.065 in Interrupt 58
15:30:31.640 in Ping 
15:30:32.102 fetch POST /api/observability/v1/traces
15:30:32.102 fetched POST /api/observability/v1/traces 204
15:30:36.644 in Ping 
15:30:41.038 rpc 59 server.reportClientActivity
15:30:41.648 in Ping 
15:30:42.044 fetch POST /api/observability/v1/traces
15:30:42.044 fetched POST /api/observability/v1/traces 204
15:30:46.652 in Ping 
15:30:51.656 in Ping 
15:30:56.041 in Interrupt 59
15:30:56.660 in Ping 
15:30:57.063 fetch POST /api/observability/v1/traces
15:30:57.063 fetched POST /api/observability/v1/traces 204
15:31:01.664 in Ping 
15:31:06.043 rpc 60 server.reportClientActivity
15:31:06.668 in Ping 
15:31:07.045 fetch POST /api/observability/v1/traces
15:31:07.045 fetched POST /api/observability/v1/traces 204
15:31:11.672 in Ping 
15:31:16.676 in Ping 
15:31:21.048 in Interrupt 60
15:31:21.680 in Ping 
15:31:22.053 fetch POST /api/observability/v1/traces
15:31:22.053 fetched POST /api/observability/v1/traces 204
15:31:26.684 in Ping 
15:31:31.045 rpc 61 server.reportClientActivity
15:31:31.688 in Ping 
15:31:32.048 fetch POST /api/observability/v1/traces
15:31:32.048 fetched POST /api/observability/v1/traces 204
15:31:36.692 in Ping 
15:31:41.696 in Ping 
15:31:46.050 in Interrupt 61
15:31:46.700 in Ping 
15:31:47.056 fetch POST /api/observability/v1/traces
15:31:47.056 fetched POST /api/observability/v1/traces 204
15:31:51.704 in Ping 
15:31:56.051 rpc 62 server.reportClientActivity
15:31:56.708 in Ping 
15:31:57.067 fetch POST /api/observability/v1/traces
15:31:57.068 fetched POST /api/observability/v1/traces 204
15:32:01.712 in Ping 
15:32:06.716 in Ping 
15:32:11.090 in Interrupt 62
15:32:11.720 in Ping 
15:32:12.237 fetch POST /api/observability/v1/traces
15:32:12.237 fetched POST /api/observability/v1/traces 204
15:32:16.726 in Ping 
15:32:21.118 rpc 63 server.reportClientActivity
15:32:21.728 in Ping 
15:32:22.168 fetch POST /api/observability/v1/traces
15:32:22.168 fetched POST /api/observability/v1/traces 204
15:32:26.775 in Ping 
15:32:31.820 in Ping 
15:32:36.132 in Interrupt 63
15:32:36.825 in Ping 
15:32:37.137 fetch POST /api/observability/v1/traces
15:32:37.138 fetched POST /api/observability/v1/traces 204
15:32:41.828 in Ping 
15:32:46.081 rpc 64 server.reportClientActivity
15:32:46.834 in Ping 
15:32:47.085 fetch POST /api/observability/v1/traces
15:32:47.085 fetched POST /api/observability/v1/traces 204
15:32:51.837 in Ping 
