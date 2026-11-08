# E, webapp, warm start, seed 1

Profile E (graded): owner extreme: 1 kbit/s, 75% bursty loss. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | 60.3s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 838.6s / 388.6s / 793.6s |
| Longest silence | 899.1s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 79238 / 29469 |
| Bytes the app received / sent | 1131 / 68422 |

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
| 70 | 9.6 | 0 | 389 |
| 80 | 19.6 | 0 | 792 |
| 90 | 29.6 | 0 | 1077 |
| 100 | 39.6 | 0 | 1077 |
| 110 | 49.6 | 0 | 1077 |
| 120 | 59.6 | 0 | 1077 |
| 130 | 69.6 | 0 | 1077 |
| 140 | 79.6 | 0 | 1077 |
| 150 | 89.6 | 0 | 1077 |
| 160 | 99.6 | 0 | 1077 |
| 170 | 109.6 | 0 | 1077 |
| 180 | 119.6 | 0 | 1077 |
| 190 | 129.6 | 0 | 1077 |
| 200 | 139.6 | 0 | 1077 |
| 210 | 149.6 | 0 | 1077 |
| 220 | 159.6 | 0 | 1077 |
| 230 | 169.6 | 0 | 1077 |
| 240 | 179.6 | 0 | 1077 |
| 250 | 189.6 | 0 | 1077 |
| 260 | 199.6 | 0 | 1077 |
| 270 | 209.6 | 0 | 1077 |
| 280 | 219.6 | 0 | 1077 |
| 290 | 229.6 | 0 | 1077 |
| 300 | 239.6 | 0 | 1077 |
| 310 | 249.6 | 0 | 1077 |
| 320 | 259.6 | 0 | 1077 |
| 330 | 269.6 | 0 | 1077 |
| 340 | 279.6 | 0 | 1077 |
| 350 | 289.6 | 0 | 1077 |
| 360 | 299.6 | 0 | 1077 |
| 370 | 309.6 | 0 | 1077 |
| 380 | 319.6 | 0 | 1077 |
| 390 | 329.6 | 0 | 1077 |
| 400 | 339.6 | 0 | 1077 |
| 410 | 349.6 | 0 | 1077 |
| 420 | 359.6 | 0 | 1077 |
| 430 | 369.6 | 0 | 1077 |
| 440 | 379.6 | 0 | 1077 |
| 450 | 389.6 | 0 | 1077 |
| 460 | 399.6 | 0 | 1077 |
| 470 | 409.6 | 0 | 1077 |
| 480 | 419.6 | 0 | 1077 |
| 490 | 429.6 | 0 | 1077 |
| 500 | 439.6 | 0 | 1077 |
| 510 | 449.6 | 0 | 1077 |
| 520 | 459.6 | 0 | 1077 |
| 530 | 469.6 | 0 | 1077 |
| 540 | 479.6 | 0 | 1077 |
| 550 | 489.6 | 0 | 1077 |
| 560 | 499.6 | 0 | 1077 |
| 570 | 509.6 | 0 | 1077 |
| 580 | 519.6 | 0 | 1077 |
| 590 | 529.6 | 0 | 1077 |
| 600 | 539.6 | 0 | 1077 |
| 610 | 549.6 | 0 | 1077 |
| 620 | 559.6 | 0 | 1077 |
| 630 | 569.6 | 0 | 1077 |
| 640 | 579.6 | 0 | 1077 |
| 650 | 589.6 | 0 | 1077 |
| 660 | 599.6 | 0 | 1077 |
| 670 | 609.6 | 0 | 1077 |
| 680 | 619.6 | 0 | 1077 |
| 690 | 629.6 | 0 | 1077 |
| 700 | 639.6 | 0 | 1077 |
| 710 | 649.6 | 0 | 1077 |
| 720 | 659.6 | 0 | 1077 |
| 730 | 669.6 | 0 | 1077 |
| 740 | 679.6 | 0 | 1077 |
| 750 | 689.6 | 0 | 1077 |
| 760 | 699.6 | 0 | 1077 |
| 770 | 709.6 | 0 | 1077 |
| 780 | 719.6 | 0 | 1077 |
| 790 | 729.6 | 0 | 1077 |
| 800 | 739.6 | 0 | 1077 |
| 810 | 749.6 | 0 | 1077 |
| 820 | 759.6 | 0 | 1077 |
| 830 | 769.6 | 0 | 1077 |
| 840 | 779.6 | 0 | 1077 |
| 850 | 789.6 | 0 | 1077 |
| 860 | 799.6 | 0 | 1077 |
| 870 | 809.6 | 0 | 1077 |
| 880 | 819.6 | 0 | 1077 |
| 890 | 829.6 | 0 | 1077 |
| 899 | 838.6 | 0 | 1077 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"810/810","providerSessions":"2688/2688","messages":"550/550 456/456 330/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159915634,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":7196.398063186745,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":3.930129042518299,"minRttMs":1,"sentPackets":127,"sentWireBytes":37918,"receivedPackets":120,"receivedWireBytes":53725,"lastHeardMs":9067,"unsyncedBytes":1001,"inflightPackets":21,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 330/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159921944,"silences":1,"longestSilenceMs":6310,"longestRecoveredGapMs":6310},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":3.930129042518299,"minRttMs":null,"sentPackets":139,"sentWireBytes":44253,"receivedPackets":121,"receivedWireBytes":54363,"lastHeardMs":15377,"unsyncedBytes":1001,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":142,"sentWireBytes":45819,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":1173,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":147,"sentWireBytes":48400,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":1173,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":159,"sentWireBytes":54916,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":1173,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":172,"sentWireBytes":60152,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":1665,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":183,"sentWireBytes":65215,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":1665,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":201,"sentWireBytes":72261,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":2158,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":217,"sentWireBytes":77960,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":2158,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":2000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":10307,"sentPackets":231,"sentWireBytes":83100,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":2158,"inflightPackets":84,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":1000,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":251,"sentWireBytes":89897,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":2651,"inflightPackets":92,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":500,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":265,"sentWireBytes":95008,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":2651,"inflightPackets":94,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":266,"sentWireBytes":95680,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":3310,"inflightPackets":83,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":267,"sentWireBytes":96498,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":3310,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":267,"sentWireBytes":96498,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":3803,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":268,"sentWireBytes":97316,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":3803,"inflightPackets":37,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":269,"sentWireBytes":98134,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":3803,"inflightPackets":18,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":269,"sentWireBytes":98134,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":4296,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":270,"sentWireBytes":98952,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":4296,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":270,"sentWireBytes":98952,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":4789,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":271,"sentWireBytes":99770,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":4789,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":4789,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":5282,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":5282,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":5775,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":5775,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":5775,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":6268,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":6268,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":3,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":6761,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":6761,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":6761,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":7254,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":7254,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":7747,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":7747,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":4,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":7747,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":8240,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":8240,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":8733,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":8733,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":5,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":8733,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":9226,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":9226,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":9719,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":9719,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":9719,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":10212,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":10212,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":10705,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":10705,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":10705,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":11198,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":11198,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":11691,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":11691,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":11691,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":12184,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":12184,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":12677,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":12677,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":12677,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":13170,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":13170,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":13663,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":13663,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":13663,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":14156,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":14156,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":14649,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":14649,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":14649,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":15142,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":15142,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":15635,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":15635,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":15635,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket closed","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":16128,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":16128,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":16621,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":16621,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":16621,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":17114,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":17114,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":17607,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":17607,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":18100,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":18100,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":18100,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","nodes":"809/809 1025/1025 809/809","providerTurns":"778/778","providerThreads":"415/810","providerSessions":"2688/2688","messages":"550/550 456/456 371/901 174/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"websocket","trying":"websocket connecting","open":false,"generation":6,"iceRestarts":8,"newConnections":1,"handshakeMs":231,"lastDataAt":1794159928271,"silences":2,"longestSilenceMs":6327,"longestRecoveredGapMs":6327},"stats":{"sendRate":50,"deliveredRate":3593.6119885701282,"loss":0,"srttMs":1291.8138629122036,"minRttMs":null,"sentPackets":272,"sentWireBytes":100588,"receivedPackets":122,"receivedWireBytes":54856,"lastHeardMs":21704,"unsyncedBytes":18593,"inflightPackets":3,"duplicateBytes":0}}
- shim trace
17:45:06.839 fetch GET /api/auth/session
17:45:06.840 fetched GET /api/auth/session 200
17:45:06.984 fetch GET /.well-known/t3/environment
17:45:06.984 fetched GET /.well-known/t3/environment 200
17:45:06.992 fetch GET /.well-known/t3/environment
17:45:06.993 fetched GET /.well-known/t3/environment 200
17:45:06.996 rpc 0 subscribeServerConfig
17:45:06.996 chunk 0 38690 bytes
17:45:07.011 in Ack 0
17:45:07.013 rpc 1 subscribeServerLifecycle
17:45:07.013 rpc 2 subscribeServerLifecycle
17:45:07.016 fetch GET /api/auth/session
17:45:07.017 fetched GET /api/auth/session 200
17:45:07.017 rpc 3 subscribeDeviceState
17:45:07.019 rpc 4 preview.reportProfiles
17:45:07.054 rpc 5 subscribeProjectClones
17:45:07.070 fetch GET /api/orchestration/threads/mcp%3Afb216675-a091-4702-b87c-bd6550aa50af/bounded
17:45:07.074 fetch GET /api/orchestration/shell
17:45:07.092 rpc 6 server.refreshProviders
17:45:07.127 rpc 7 orchestration.dispatchCommand thread.visit
17:45:07.127 exit 7 {"_tag":"Success","value":{"sequence":0}}
17:45:07.137 fetched GET /api/orchestration/shell 200
17:45:07.137 rpc 8 subscribeTerminalMetadata
17:45:07.138 rpc 9 subscribeVcsStatus
17:45:07.138 rpc 10 subscribePreviewEvents
17:45:07.140 rpc 11 orchestration.subscribeShell
17:45:07.140 chunk 11 2782 bytes
17:45:07.141 rpc 12 assets.createUrl
17:45:07.142 rpc 13 preview.list
17:45:07.142 rpc 14 projects.readFile
17:45:07.145 in Ack 11
17:45:07.390 rpc 15 server.reportClientActivity
17:45:07.422 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"0112d72b-babc-4117-a5de-e51b4939447a","sessions":[]}}
17:45:07.422 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:45:07.899 chunk 1 3864 bytes
17:45:07.900 in Ack 1
17:45:07.982 fetch POST /api/observability/v1/traces
17:45:07.982 fetched POST /api/observability/v1/traces 204
17:45:08.079 chunk 2 3864 bytes
17:45:08.079 in Ack 2
17:45:08.289 chunk 11 2758 bytes
17:45:08.289 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:44:55.254Z","configurableProviders":false,"con
17:45:08.289 chunk 9 534 bytes
17:45:08.289 exit 12 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:45:08.290 in Ack 11
17:45:08.296 in Ack 9
17:45:08.539 chunk 11 2801 bytes
17:45:08.539 in Ack 11
17:45:08.633 chunk 3 876 bytes
17:45:08.633 exit 4 {"_tag":"Success","value":null}
17:45:08.633 chunk 5 4 bytes
17:45:08.633 chunk 8 36 bytes
17:45:08.633 exit 15 {"_tag":"Success","value":null}
17:45:08.633 in Ack 3
17:45:08.637 rpc 16 preview.reportProfiles
17:45:08.637 in Ack 5
17:45:08.637 in Ack 8
17:45:08.712 exit 16 {"_tag":"Success","value":null}
17:45:09.049 fetched GET /api/orchestration/threads/mcp%3Afb216675-a091-4702-b87c-bd6550aa50af/bounded 200
17:45:09.051 rpc 17 orchestration.subscribeThread
17:45:09.052 chunk 17 8305 bytes
17:45:09.060 in Interrupt 9
17:45:09.062 rpc 18 subscribeVcsStatus
17:45:09.063 in Ack 17
17:45:09.117 fetch POST /api/observability/v1/traces
17:45:09.117 fetched POST /api/observability/v1/traces 204
17:45:09.209 chunk 18 534 bytes
17:45:09.209 in Ack 18
17:45:09.302 chunk 17 9359 bytes
17:45:09.303 in Ack 17
17:45:09.325 rpc 19 server.reportClientActivity
17:45:09.552 chunk 17 12722 bytes
17:45:09.554 in Ack 17
17:45:09.802 chunk 17 13532 bytes
17:45:09.803 in Ack 17
17:45:09.924 rpc 20 attachments.createUploadUrl
17:45:10.084 chunk 17 17245 bytes
17:45:10.092 in Ack 17
17:45:10.178 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-b1c27909-9c41-4e70-90a8-5092c11f7745","expiresAt":1794160510098,"relativeUrl":"/api/attachments/upload/eyJ2ZX
17:45:10.179 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWIxYzI3OTA5LTljNDEtNGU3MC05MGE4LTUwOTJjMTFmNzc0NSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2MDUxMDA5OH0.oJDZzVQutgiYZ-yJEE82baXEz6zH_oZlPVC85jm4WLg
17:45:10.248 exit 19 {"_tag":"Success","value":null}
17:45:10.304 fetch POST /api/observability/v1/traces
17:45:10.304 fetched POST /api/observability/v1/traces 204
17:45:10.334 chunk 17 19883 bytes
17:45:10.335 in Ack 17
17:45:10.342 in Interrupt 18
17:45:10.343 rpc 21 subscribeVcsStatus
17:45:10.388 chunk 21 534 bytes
17:45:10.388 in Ack 21
17:45:10.626 chunk 17 21223 bytes
17:45:10.627 rpc 22 server.reportClientActivity
17:45:10.629 in Ack 17
17:45:11.340 fetch POST /api/observability/v1/traces
17:45:11.340 fetched POST /api/observability/v1/traces 204
17:45:11.995 in Ping 
17:45:12.016 exit 22 {"_tag":"Success","value":null}
17:45:12.769 rpc 23 orchestration.dispatchCommand message.dispatch
17:45:13.315 fetch POST /api/observability/v1/traces
17:45:13.315 fetched POST /api/observability/v1/traces 204
17:45:13.369 exit 23 {"_tag":"Success","value":{"sequence":45}}
17:45:13.371 rpc 24 attachments.delete
17:45:13.652 chunk 17 23679 bytes
17:45:13.654 in Ack 17
17:45:13.792 exit 24 {"_tag":"Success","value":null}
17:45:13.902 chunk 17 23651 bytes
17:45:13.903 in Ack 17
17:45:14.153 chunk 17 23663 bytes
17:45:14.154 in Ack 17
17:45:14.370 fetch POST /api/observability/v1/traces
17:45:14.370 fetched POST /api/observability/v1/traces 204
17:45:14.415 chunk 17 23713 bytes
17:45:14.416 in Ack 17
17:45:14.670 chunk 11 2823 bytes
17:45:14.671 chunk 17 28141 bytes
17:45:14.671 in Ack 11
17:45:14.672 rpc 25 orchestration.dispatchCommand thread.visit
17:45:14.672 exit 25 {"_tag":"Success","value":{"sequence":0}}
17:45:14.704 in Ack 17
17:45:14.707 rpc 26 assets.createUrl
17:45:14.921 chunk 17 30564 bytes
17:45:14.923 in Ack 17
17:45:15.171 chunk 11 2823 bytes
17:45:15.171 chunk 17 32154 bytes
17:45:15.172 in Ack 11
17:45:15.177 in Ack 17
17:45:15.182 in Interrupt 21
17:45:15.185 rpc 27 subscribeVcsStatus
17:45:15.206 chunk 27 534 bytes
17:45:15.206 in Ack 27
17:45:15.631 fetch POST /api/observability/v1/traces
17:45:15.631 fetched POST /api/observability/v1/traces 204
17:45:15.632 chunk 17 31988 bytes
17:45:15.633 rpc 28 server.reportClientActivity
17:45:15.635 in Ack 17
17:45:15.886 chunk 17 31988 bytes
17:45:15.887 in Ack 17
17:45:16.599 rpc 29 orchestration.dispatchCommand message.dispatch
17:45:16.634 fetch POST /api/observability/v1/traces
17:45:16.634 fetched POST /api/observability/v1/traces 204
17:45:16.995 in Ping 
17:45:18.646 fetch POST /api/observability/v1/traces
17:45:18.646 fetched POST /api/observability/v1/traces 204
17:45:20.609 fetch POST /api/observability/v1/traces
17:45:20.609 fetched POST /api/observability/v1/traces 204
17:45:21.995 in Ping 
17:45:26.996 in Ping 
17:45:30.633 in Interrupt 28
17:45:31.634 fetch POST /api/observability/v1/traces
17:45:31.634 fetched POST /api/observability/v1/traces 204
17:45:31.996 in Ping 
17:45:32.236 rpc 30 server.reportClientActivity
17:45:33.236 fetch POST /api/observability/v1/traces
17:45:33.236 fetched POST /api/observability/v1/traces 204
17:45:36.996 in Ping 
17:45:41.996 in Ping 
17:45:46.997 in Ping 
17:45:47.236 in Interrupt 30
17:45:48.237 fetch POST /api/observability/v1/traces
17:45:48.237 fetched POST /api/observability/v1/traces 204
17:45:51.997 in Ping 
17:45:56.997 in Ping 
17:45:57.236 rpc 31 server.reportClientActivity
17:45:58.237 fetch POST /api/observability/v1/traces
17:45:58.237 fetched POST /api/observability/v1/traces 204
17:46:01.999 in Ping 
17:46:07.003 in Ping 
17:46:12.007 in Ping 
17:46:12.237 in Interrupt 31
17:46:13.240 fetch POST /api/observability/v1/traces
17:46:13.240 fetched POST /api/observability/v1/traces 204
17:46:17.021 in Ping 
17:46:22.023 in Ping 
17:46:22.237 rpc 32 server.reportClientActivity
17:46:23.237 fetch POST /api/observability/v1/traces
17:46:23.237 fetched POST /api/observability/v1/traces 204
17:46:27.027 in Ping 
17:46:32.031 in Ping 
17:46:37.042 in Ping 
17:46:37.238 in Interrupt 32
17:46:38.239 fetch POST /api/observability/v1/traces
17:46:38.239 fetched POST /api/observability/v1/traces 204
17:46:42.043 in Ping 
17:46:47.047 in Ping 
17:46:47.238 rpc 33 server.reportClientActivity
17:46:48.238 fetch POST /api/observability/v1/traces
17:46:48.238 fetched POST /api/observability/v1/traces 204
17:46:52.051 in Ping 
17:46:57.055 in Ping 
17:47:02.059 in Ping 
17:47:02.238 in Interrupt 33
17:47:03.239 fetch POST /api/observability/v1/traces
17:47:03.239 fetched POST /api/observability/v1/traces 204
17:47:07.063 in Ping 
17:47:12.067 in Ping 
17:47:12.238 rpc 34 server.reportClientActivity
17:47:13.238 fetch POST /api/observability/v1/traces
17:47:13.238 fetched POST /api/observability/v1/traces 204
17:47:17.071 in Ping 
17:47:22.075 in Ping 
17:47:27.079 in Ping 
17:47:27.239 in Interrupt 34
17:47:28.240 fetch POST /api/observability/v1/traces
17:47:28.240 fetched POST /api/observability/v1/traces 204
17:47:32.083 in Ping 
17:47:37.087 in Ping 
17:47:37.238 rpc 35 server.reportClientActivity
17:47:38.239 fetch POST /api/observability/v1/traces
17:47:38.239 fetched POST /api/observability/v1/traces 204
17:47:42.091 in Ping 
17:47:47.095 in Ping 
17:47:52.104 in Ping 
17:47:52.239 in Interrupt 35
17:47:53.240 fetch POST /api/observability/v1/traces
17:47:53.240 fetched POST /api/observability/v1/traces 204
17:47:57.107 in Ping 
17:48:02.111 in Ping 
17:48:02.238 rpc 36 server.reportClientActivity
17:48:03.239 fetch POST /api/observability/v1/traces
17:48:03.239 fetched POST /api/observability/v1/traces 204
17:48:07.115 in Ping 
17:48:12.119 in Ping 
17:48:17.123 in Ping 
17:48:17.239 in Interrupt 36
17:48:18.240 fetch POST /api/observability/v1/traces
17:48:18.240 fetched POST /api/observability/v1/traces 204
17:48:22.127 in Ping 
17:48:27.131 in Ping 
17:48:27.239 rpc 37 server.reportClientActivity
17:48:28.239 fetch POST /api/observability/v1/traces
17:48:28.240 fetched POST /api/observability/v1/traces 204
17:48:32.135 in Ping 
17:48:37.139 in Ping 
17:48:42.143 in Ping 
17:48:42.240 in Interrupt 37
17:48:43.243 fetch POST /api/observability/v1/traces
17:48:43.243 fetched POST /api/observability/v1/traces 204
17:48:47.147 in Ping 
17:48:52.151 in Ping 
17:48:52.239 rpc 38 server.reportClientActivity
17:48:53.240 fetch POST /api/observability/v1/traces
17:48:53.240 fetched POST /api/observability/v1/traces 204
17:48:57.155 in Ping 
17:49:02.159 in Ping 
17:49:07.163 in Ping 
17:49:07.243 in Interrupt 38
17:49:08.247 fetch POST /api/observability/v1/traces
17:49:08.247 fetched POST /api/observability/v1/traces 204
17:49:12.167 in Ping 
17:49:17.171 in Ping 
17:49:17.241 rpc 39 server.reportClientActivity
17:49:18.243 fetch POST /api/observability/v1/traces
17:49:18.243 fetched POST /api/observability/v1/traces 204
17:49:22.175 in Ping 
17:49:27.179 in Ping 
17:49:32.183 in Ping 
17:49:32.243 in Interrupt 39
17:49:33.248 fetch POST /api/observability/v1/traces
17:49:33.248 fetched POST /api/observability/v1/traces 204
17:49:37.187 in Ping 
17:49:42.191 in Ping 
17:49:42.244 rpc 40 server.reportClientActivity
17:49:43.247 fetch POST /api/observability/v1/traces
17:49:43.247 fetched POST /api/observability/v1/traces 204
17:49:47.195 in Ping 
17:49:52.199 in Ping 
17:49:57.203 in Ping 
17:49:57.248 in Interrupt 40
17:49:58.252 fetch POST /api/observability/v1/traces
17:49:58.252 fetched POST /api/observability/v1/traces 204
17:50:02.207 in Ping 
17:50:07.211 in Ping 
17:50:07.248 rpc 41 server.reportClientActivity
17:50:08.251 fetch POST /api/observability/v1/traces
17:50:08.252 fetched POST /api/observability/v1/traces 204
17:50:12.215 in Ping 
17:50:17.219 in Ping 
17:50:22.223 in Ping 
17:50:22.251 in Interrupt 41
17:50:23.255 fetch POST /api/observability/v1/traces
17:50:23.256 fetched POST /api/observability/v1/traces 204
17:50:27.227 in Ping 
17:50:32.231 in Ping 
17:50:32.252 rpc 42 server.reportClientActivity
17:50:33.255 fetch POST /api/observability/v1/traces
17:50:33.255 fetched POST /api/observability/v1/traces 204
17:50:37.235 in Ping 
17:50:42.239 in Ping 
17:50:47.243 in Ping 
17:50:47.255 in Interrupt 42
17:50:48.259 fetch POST /api/observability/v1/traces
17:50:48.259 fetched POST /api/observability/v1/traces 204
17:50:52.247 in Ping 
17:50:57.251 in Ping 
17:50:57.256 rpc 43 server.reportClientActivity
17:50:58.259 fetch POST /api/observability/v1/traces
17:50:58.259 fetched POST /api/observability/v1/traces 204
17:51:02.255 in Ping 
17:51:07.259 in Ping 
17:51:12.259 in Interrupt 43
17:51:12.263 in Ping 
17:51:13.263 fetch POST /api/observability/v1/traces
17:51:13.263 fetched POST /api/observability/v1/traces 204
17:51:17.267 in Ping 
17:51:22.260 rpc 44 server.reportClientActivity
17:51:22.271 in Ping 
17:51:23.263 fetch POST /api/observability/v1/traces
17:51:23.263 fetched POST /api/observability/v1/traces 204
17:51:27.275 in Ping 
17:51:32.279 in Ping 
17:51:37.263 in Interrupt 44
17:51:37.283 in Ping 
17:51:38.267 fetch POST /api/observability/v1/traces
17:51:38.268 fetched POST /api/observability/v1/traces 204
17:51:42.287 in Ping 
17:51:47.264 rpc 45 server.reportClientActivity
17:51:47.291 in Ping 
17:51:48.267 fetch POST /api/observability/v1/traces
17:51:48.267 fetched POST /api/observability/v1/traces 204
17:51:52.295 in Ping 
17:51:57.299 in Ping 
17:52:02.267 in Interrupt 45
17:52:02.303 in Ping 
17:52:03.272 fetch POST /api/observability/v1/traces
17:52:03.272 fetched POST /api/observability/v1/traces 204
17:52:07.307 in Ping 
17:52:12.268 rpc 46 server.reportClientActivity
17:52:12.311 in Ping 
17:52:13.272 fetch POST /api/observability/v1/traces
17:52:13.272 fetched POST /api/observability/v1/traces 204
17:52:17.315 in Ping 
17:52:22.319 in Ping 
17:52:27.271 in Interrupt 46
17:52:27.323 in Ping 
17:52:28.275 fetch POST /api/observability/v1/traces
17:52:28.275 fetched POST /api/observability/v1/traces 204
17:52:32.327 in Ping 
17:52:37.276 rpc 47 server.reportClientActivity
17:52:37.331 in Ping 
17:52:38.280 fetch POST /api/observability/v1/traces
17:52:38.280 fetched POST /api/observability/v1/traces 204
17:52:42.335 in Ping 
17:52:47.339 in Ping 
17:52:52.279 in Interrupt 47
17:52:52.343 in Ping 
17:52:53.283 fetch POST /api/observability/v1/traces
17:52:53.283 fetched POST /api/observability/v1/traces 204
17:52:57.347 in Ping 
17:53:02.276 rpc 48 server.reportClientActivity
17:53:02.352 in Ping 
17:53:03.279 fetch POST /api/observability/v1/traces
17:53:03.279 fetched POST /api/observability/v1/traces 204
17:53:07.355 in Ping 
17:53:12.359 in Ping 
17:53:17.279 in Interrupt 48
17:53:17.363 in Ping 
17:53:18.284 fetch POST /api/observability/v1/traces
17:53:18.284 fetched POST /api/observability/v1/traces 204
17:53:22.367 in Ping 
17:53:27.280 rpc 49 server.reportClientActivity
17:53:27.371 in Ping 
17:53:28.283 fetch POST /api/observability/v1/traces
17:53:28.283 fetched POST /api/observability/v1/traces 204
17:53:32.375 in Ping 
17:53:37.379 in Ping 
17:53:42.283 in Interrupt 49
17:53:42.383 in Ping 
17:53:43.287 fetch POST /api/observability/v1/traces
17:53:43.287 fetched POST /api/observability/v1/traces 204
17:53:47.387 in Ping 
17:53:52.284 rpc 50 server.reportClientActivity
17:53:52.391 in Ping 
17:53:53.288 fetch POST /api/observability/v1/traces
17:53:53.288 fetched POST /api/observability/v1/traces 204
17:53:57.395 in Ping 
17:54:02.399 in Ping 
17:54:07.287 in Interrupt 50
17:54:07.403 in Ping 
17:54:08.291 fetch POST /api/observability/v1/traces
17:54:08.291 fetched POST /api/observability/v1/traces 204
17:54:12.407 in Ping 
17:54:17.288 rpc 51 server.reportClientActivity
17:54:17.411 in Ping 
17:54:18.291 fetch POST /api/observability/v1/traces
17:54:18.291 fetched POST /api/observability/v1/traces 204
17:54:22.415 in Ping 
17:54:27.419 in Ping 
17:54:32.291 in Interrupt 51
17:54:32.423 in Ping 
17:54:33.297 fetch POST /api/observability/v1/traces
17:54:33.297 fetched POST /api/observability/v1/traces 204
17:54:37.427 in Ping 
17:54:42.292 rpc 52 server.reportClientActivity
17:54:42.431 in Ping 
17:54:43.296 fetch POST /api/observability/v1/traces
17:54:43.296 fetched POST /api/observability/v1/traces 204
17:54:47.435 in Ping 
17:54:52.439 in Ping 
17:54:57.295 in Interrupt 52
17:54:57.443 in Ping 
17:54:58.300 fetch POST /api/observability/v1/traces
17:54:58.300 fetched POST /api/observability/v1/traces 204
17:55:02.447 in Ping 
17:55:07.296 rpc 53 server.reportClientActivity
17:55:07.451 in Ping 
17:55:08.299 fetch POST /api/observability/v1/traces
17:55:08.299 fetched POST /api/observability/v1/traces 204
17:55:12.455 in Ping 
17:55:17.459 in Ping 
17:55:22.299 in Interrupt 53
17:55:22.463 in Ping 
17:55:23.303 fetch POST /api/observability/v1/traces
17:55:23.304 fetched POST /api/observability/v1/traces 204
17:55:27.467 in Ping 
17:55:32.300 rpc 54 server.reportClientActivity
17:55:32.471 in Ping 
17:55:33.303 fetch POST /api/observability/v1/traces
17:55:33.304 fetched POST /api/observability/v1/traces 204
17:55:37.475 in Ping 
17:55:42.479 in Ping 
17:55:47.303 in Interrupt 54
17:55:47.483 in Ping 
17:55:48.307 fetch POST /api/observability/v1/traces
17:55:48.307 fetched POST /api/observability/v1/traces 204
17:55:52.487 in Ping 
17:55:57.304 rpc 55 server.reportClientActivity
17:55:57.491 in Ping 
17:55:58.308 fetch POST /api/observability/v1/traces
17:55:58.308 fetched POST /api/observability/v1/traces 204
17:56:02.495 in Ping 
17:56:07.499 in Ping 
17:56:12.307 in Interrupt 55
17:56:12.503 in Ping 
17:56:13.311 fetch POST /api/observability/v1/traces
17:56:13.311 fetched POST /api/observability/v1/traces 204
17:56:17.507 in Ping 
17:56:22.308 rpc 56 server.reportClientActivity
17:56:22.511 in Ping 
17:56:23.311 fetch POST /api/observability/v1/traces
17:56:23.311 fetched POST /api/observability/v1/traces 204
17:56:27.515 in Ping 
17:56:32.519 in Ping 
17:56:37.311 in Interrupt 56
17:56:37.523 in Ping 
17:56:38.316 fetch POST /api/observability/v1/traces
17:56:38.316 fetched POST /api/observability/v1/traces 204
17:56:42.527 in Ping 
17:56:47.312 rpc 57 server.reportClientActivity
17:56:47.531 in Ping 
17:56:48.315 fetch POST /api/observability/v1/traces
17:56:48.315 fetched POST /api/observability/v1/traces 204
17:56:52.535 in Ping 
17:56:57.539 in Ping 
17:57:02.316 in Interrupt 57
17:57:02.543 in Ping 
17:57:03.320 fetch POST /api/observability/v1/traces
17:57:03.320 fetched POST /api/observability/v1/traces 204
17:57:07.547 in Ping 
17:57:12.317 rpc 58 server.reportClientActivity
17:57:12.551 in Ping 
17:57:13.319 fetch POST /api/observability/v1/traces
17:57:13.319 fetched POST /api/observability/v1/traces 204
17:57:17.555 in Ping 
17:57:22.559 in Ping 
17:57:27.319 in Interrupt 58
17:57:27.563 in Ping 
17:57:28.323 fetch POST /api/observability/v1/traces
17:57:28.323 fetched POST /api/observability/v1/traces 204
17:57:32.567 in Ping 
17:57:37.320 rpc 59 server.reportClientActivity
17:57:37.571 in Ping 
17:57:38.323 fetch POST /api/observability/v1/traces
17:57:38.323 fetched POST /api/observability/v1/traces 204
17:57:42.575 in Ping 
17:57:47.579 in Ping 
17:57:52.323 in Interrupt 59
17:57:52.583 in Ping 
17:57:53.329 fetch POST /api/observability/v1/traces
17:57:53.329 fetched POST /api/observability/v1/traces 204
17:57:57.587 in Ping 
17:58:02.324 rpc 60 server.reportClientActivity
17:58:02.591 in Ping 
17:58:03.327 fetch POST /api/observability/v1/traces
17:58:03.327 fetched POST /api/observability/v1/traces 204
17:58:07.595 in Ping 
17:58:12.599 in Ping 
17:58:17.327 in Interrupt 60
17:58:17.603 in Ping 
17:58:18.331 fetch POST /api/observability/v1/traces
17:58:18.331 fetched POST /api/observability/v1/traces 204
17:58:22.607 in Ping 
17:58:27.328 rpc 61 server.reportClientActivity
17:58:27.611 in Ping 
17:58:28.331 fetch POST /api/observability/v1/traces
17:58:28.331 fetched POST /api/observability/v1/traces 204
17:58:32.615 in Ping 
17:58:37.619 in Ping 
17:58:42.331 in Interrupt 61
17:58:42.623 in Ping 
17:58:43.335 fetch POST /api/observability/v1/traces
17:58:43.335 fetched POST /api/observability/v1/traces 204
17:58:47.627 in Ping 
17:58:52.331 rpc 62 server.reportClientActivity
17:58:52.631 in Ping 
17:58:53.335 fetch POST /api/observability/v1/traces
17:58:53.335 fetched POST /api/observability/v1/traces 204
17:58:57.635 in Ping 
17:59:02.639 in Ping 
17:59:07.335 in Interrupt 62
17:59:07.643 in Ping 
17:59:08.339 fetch POST /api/observability/v1/traces
17:59:08.339 fetched POST /api/observability/v1/traces 204
17:59:12.647 in Ping 
17:59:17.336 rpc 63 server.reportClientActivity
17:59:17.651 in Ping 
17:59:18.340 fetch POST /api/observability/v1/traces
17:59:18.340 fetched POST /api/observability/v1/traces 204
17:59:22.655 in Ping 
17:59:27.659 in Ping 
17:59:32.339 in Interrupt 63
17:59:32.663 in Ping 
17:59:33.343 fetch POST /api/observability/v1/traces
17:59:33.343 fetched POST /api/observability/v1/traces 204
17:59:37.667 in Ping 
17:59:42.340 rpc 64 server.reportClientActivity
17:59:42.671 in Ping 
17:59:43.343 fetch POST /api/observability/v1/traces
17:59:43.344 fetched POST /api/observability/v1/traces 204
17:59:47.675 in Ping 
17:59:52.679 in Ping 
17:59:57.344 in Interrupt 64
17:59:57.683 in Ping 
17:59:58.347 fetch POST /api/observability/v1/traces
17:59:58.347 fetched POST /api/observability/v1/traces 204
18:00:02.687 in Ping 
18:00:07.344 rpc 65 server.reportClientActivity
18:00:07.691 in Ping 
18:00:08.347 fetch POST /api/observability/v1/traces
18:00:08.347 fetched POST /api/observability/v1/traces 204
18:00:12.695 in Ping 
