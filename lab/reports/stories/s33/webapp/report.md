# story, webapp, warm start, seed 33

Profile story (graded): seed 33's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 183 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 9.0s |
| Prompt reached the agent | 6.0s |
| Time to first text | 162.1s |
| Whole reply shown | 181.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 155.9s / 66.9s / 146.9s |
| Longest silence | 162.1s |
| Approval shown after it was asked | 131.6s |
| Bytes on the wire down / up | 225951 / 89763 |
| Bytes the app received / sent | 84299 / 65735 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 3.9 | 0 | 161 |
| 20 | 13.9 | 0 | 561 |
| 30 | 23.9 | 0 | 965 |
| 40 | 33.9 | 0 | 1077 |
| 50 | 43.9 | 0 | 1077 |
| 60 | 53.9 | 0 | 1077 |
| 70 | 63.9 | 0 | 1077 |
| 80 | 73.9 | 0 | 1077 |
| 90 | 83.9 | 0 | 1077 |
| 100 | 93.9 | 0 | 1077 |
| 110 | 103.9 | 0 | 1077 |
| 120 | 113.9 | 0 | 1077 |
| 130 | 123.9 | 0 | 1077 |
| 140 | 133.9 | 0 | 1077 |
| 150 | 143.9 | 0 | 1077 |
| 160 | 153.9 | 0 | 1077 |
| 170 | 4.4 | 1077 | 1254 |
| 180 | 6.6 | 1378 | 1596 |
| 181 | 7.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217286052,"silences":0,"longestSilenceMs":1355,"longestRecoveredGapMs":1355},"stats":{"sendRate":3375,"deliveredRate":3080.9117663805882,"loss":0,"srttMs":169.51776276568305,"minRttMs":1308,"sentPackets":125,"sentWireBytes":38405,"receivedPackets":146,"receivedWireBytes":102440,"lastHeardMs":12110,"unsyncedBytes":518,"inflightPackets":25,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 368/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217295844,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":2131.8090855529085,"deliveredRate":2340.80997629339,"loss":0.17647058823529413,"srttMs":1763.4952981027916,"minRttMs":1308,"sentPackets":143,"sentWireBytes":45325,"receivedPackets":159,"receivedWireBytes":105203,"lastHeardMs":21902,"unsyncedBytes":0,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 497/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 325/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217305521,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":2000,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":1763.4952981027916,"minRttMs":1308,"sentPackets":169,"sentWireBytes":51805,"receivedPackets":168,"receivedWireBytes":109268,"lastHeardMs":31579,"unsyncedBytes":457,"inflightPackets":55,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":2000,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":1308,"sentPackets":169,"sentWireBytes":51805,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":293,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":1000,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":1308,"sentPackets":174,"sentWireBytes":53225,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":750,"inflightPackets":59,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":1000,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":1308,"sentPackets":193,"sentWireBytes":58582,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":750,"inflightPackets":78,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":1000,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":1308,"sentPackets":217,"sentWireBytes":65364,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":750,"inflightPackets":102,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":225,"sentWireBytes":67639,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":1208,"inflightPackets":110,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":227,"sentWireBytes":68201,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":1208,"inflightPackets":112,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":228,"sentWireBytes":68482,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":1666,"inflightPackets":113,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":230,"sentWireBytes":69044,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":1666,"inflightPackets":115,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":232,"sentWireBytes":69606,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":1666,"inflightPackets":117,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":234,"sentWireBytes":70168,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":2124,"inflightPackets":119,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":236,"sentWireBytes":70730,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":2124,"inflightPackets":121,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":237,"sentWireBytes":71011,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":2582,"inflightPackets":122,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":239,"sentWireBytes":71573,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":2582,"inflightPackets":124,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 730/849 680/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 351/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217310663,"silences":0,"longestSilenceMs":1925,"longestRecoveredGapMs":1925},"stats":{"sendRate":50,"deliveredRate":67.93349168646081,"loss":0.9444444444444444,"srttMs":2443.5583858399427,"minRttMs":null,"sentPackets":241,"sentWireBytes":72135,"receivedPackets":171,"receivedWireBytes":110817,"lastHeardMs":36721,"unsyncedBytes":2582,"inflightPackets":126,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 341/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 421/839","user":"10/10 9/9","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":469,"lastDataAt":1794217457829,"silences":1,"longestSilenceMs":137065,"longestRecoveredGapMs":137065},"stats":{"sendRate":1336.0840752968588,"deliveredRate":475.4206179671368,"loss":0.4662529550827422,"srttMs":1541.6908167102988,"minRttMs":1322,"sentPackets":278,"sentWireBytes":83308,"receivedPackets":238,"receivedWireBytes":141401,"lastHeardMs":183887,"unsyncedBytes":167,"inflightPackets":133,"duplicateBytes":0}}
- shim trace
09:41:14.474 fetch GET /api/auth/session
09:41:14.476 fetched GET /api/auth/session 200
09:41:14.671 fetch GET /.well-known/t3/environment
09:41:14.672 fetched GET /.well-known/t3/environment 200
09:41:14.682 fetch GET /.well-known/t3/environment
09:41:14.683 fetched GET /.well-known/t3/environment 200
09:41:14.688 rpc 0 subscribeServerConfig
09:41:14.688 chunk 0 38462 bytes
09:41:14.734 in Ack 0
09:41:14.737 rpc 1 subscribeServerLifecycle
09:41:14.738 rpc 2 subscribeServerLifecycle
09:41:14.742 fetch GET /api/auth/session
09:41:14.746 fetched GET /api/auth/session 200
09:41:14.747 rpc 3 subscribeDeviceState
09:41:14.750 rpc 4 preview.reportProfiles
09:41:14.813 rpc 5 subscribeProjectClones
09:41:14.839 fetch GET /api/orchestration/threads/mcp%3A96d19ecf-5186-4ac0-a129-85d974082b7e/bounded
09:41:14.847 fetch GET /api/orchestration/shell
09:41:14.848 fetched GET /api/orchestration/shell 200
09:41:14.851 rpc 6 orchestration.subscribeShell
09:41:14.852 chunk 6 2794 bytes
09:41:14.854 in Ack 6
09:41:14.934 rpc 7 server.reportClientActivity
09:41:15.236 chunk 6 2770 bytes
09:41:15.243 in Ack 6
09:41:15.335 chunk 1 3815 bytes
09:41:15.337 in Ack 1
09:41:15.505 chunk 2 3815 bytes
09:41:15.506 in Ack 2
09:41:15.668 fetch POST /api/observability/v1/traces
09:41:15.668 fetched POST /api/observability/v1/traces 204
09:41:15.755 chunk 6 4592 bytes
09:41:15.756 in Ack 6
09:41:15.784 rpc 8 server.refreshProviders
09:41:15.836 rpc 9 orchestration.dispatchCommand thread.visit
09:41:15.836 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:41:15.854 rpc 10 subscribeTerminalMetadata
09:41:15.854 rpc 11 subscribeVcsStatus
09:41:15.855 rpc 12 subscribePreviewEvents
09:41:15.857 rpc 13 assets.createUrl
09:41:15.858 rpc 14 preview.list
09:41:15.859 rpc 15 projects.readFile
09:41:15.914 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f9ecbdac-17f9-475f-aefc-abc8a3047b70","sessions":[]}}
09:41:15.914 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:41:15.914 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:40:52.336Z","configurableProviders":false,"con
09:41:16.010 chunk 11 1632 bytes
09:41:16.011 in Ack 11
09:41:16.128 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:41:16.255 chunk 6 4635 bytes
09:41:16.256 in Ack 6
09:41:16.267 chunk 3 876 bytes
09:41:16.267 exit 4 {"_tag":"Success","value":null}
09:41:16.267 chunk 5 4 bytes
09:41:16.267 exit 7 {"_tag":"Success","value":null}
09:41:16.267 chunk 10 36 bytes
09:41:16.269 in Ack 3
09:41:16.273 in Ack 5
09:41:16.274 in Ack 10
09:41:16.525 rpc 16 server.reportClientActivity
09:41:16.587 exit 16 {"_tag":"Success","value":null}
09:41:16.757 fetch POST /api/observability/v1/traces
09:41:16.757 fetched POST /api/observability/v1/traces 204
09:41:17.115 fetched GET /api/orchestration/threads/mcp%3A96d19ecf-5186-4ac0-a129-85d974082b7e/bounded 200
09:41:17.120 rpc 17 orchestration.subscribeThread
09:41:17.120 chunk 17 8283 bytes
09:41:17.135 in Interrupt 11
09:41:17.138 rpc 18 subscribeVcsStatus
09:41:17.139 in Ack 17
09:41:17.245 chunk 18 1632 bytes
09:41:17.246 in Ack 18
09:41:17.372 chunk 17 11260 bytes
09:41:17.373 in Ack 17
09:41:17.436 rpc 19 server.reportClientActivity
09:41:17.624 chunk 17 13498 bytes
09:41:17.625 in Ack 17
09:41:17.874 chunk 17 19020 bytes
09:41:17.875 in Ack 17
09:41:18.098 exit 19 {"_tag":"Success","value":null}
09:41:18.165 rpc 20 attachments.createUploadUrl
09:41:18.219 fetch POST /api/observability/v1/traces
09:41:18.219 fetched POST /api/observability/v1/traces 204
09:41:18.219 chunk 17 21100 bytes
09:41:18.236 in Ack 17
09:41:18.262 in Interrupt 18
09:41:18.267 rpc 21 subscribeVcsStatus
09:41:18.337 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e5eaade7-1489-452d-9ec6-efdd4c8a9d11","expiresAt":1794217878300,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:41:18.337 chunk 21 1632 bytes
09:41:18.339 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWU1ZWFhZGU3LTE0ODktNDUyZC05ZWM2LWVmZGQ0YzhhOWQxMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzg3ODMwMH0.X8PFrI-8DIqfhxEyXAT3blp9AnHGn0S5PmXHmTFwSHs
09:41:18.339 in Ack 21
09:41:18.518 rpc 22 server.reportClientActivity
09:41:19.237 fetch POST /api/observability/v1/traces
09:41:19.237 fetched POST /api/observability/v1/traces 204
09:41:19.685 in Ping 
09:41:20.051 exit 22 {"_tag":"Success","value":null}
09:41:21.195 rpc 23 orchestration.dispatchCommand message.dispatch
09:41:21.421 fetch POST /api/observability/v1/traces
09:41:21.421 fetched POST /api/observability/v1/traces 204
09:41:21.480 chunk 17 23893 bytes
09:41:21.481 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:41:21.486 in Ack 17
09:41:21.493 rpc 24 assets.createUrl
09:41:21.561 rpc 25 attachments.delete
09:41:21.674 exit 25 {"_tag":"Success","value":null}
09:41:21.730 chunk 6 4721 bytes
09:41:21.731 chunk 17 26810 bytes
09:41:21.732 in Ack 6
09:41:21.771 in Ack 17
09:41:21.981 chunk 17 27723 bytes
09:41:21.982 in Ack 17
09:41:22.231 chunk 6 4660 bytes
09:41:22.231 chunk 17 30180 bytes
09:41:22.231 in Ack 6
09:41:22.241 in Ack 17
09:41:22.482 chunk 17 31903 bytes
09:41:22.484 in Ack 17
09:41:22.552 fetch POST /api/observability/v1/traces
09:41:22.552 fetched POST /api/observability/v1/traces 204
09:41:22.572 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/167c6b98-8cf3-41b2-8867-ea5820b65100","expiresAt":1794220882572}}
09:41:23.182 chunk 17 32035 bytes
09:41:23.185 in Ack 17
09:41:23.436 chunk 6 4657 bytes
09:41:23.437 chunk 17 35260 bytes
09:41:23.437 in Ack 6
09:41:23.441 rpc 26 orchestration.dispatchCommand thread.visit
09:41:23.441 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:41:23.467 in Ack 17
09:41:23.481 in Interrupt 21
09:41:23.527 rpc 27 subscribeVcsStatus
09:41:23.575 fetch POST /api/observability/v1/traces
09:41:23.575 fetched POST /api/observability/v1/traces 204
09:41:23.591 chunk 27 1713 bytes
09:41:23.592 in Ack 27
09:41:23.694 chunk 6 4657 bytes
09:41:23.695 chunk 17 35260 bytes
09:41:23.965 in Ack 6
09:41:23.995 in Ack 17
09:41:24.028 rpc 28 server.reportClientActivity
09:41:24.688 in Ping 
09:41:24.974 fetch POST /api/observability/v1/traces
09:41:24.975 fetched POST /api/observability/v1/traces 204
09:41:25.384 exit 28 {"_tag":"Success","value":null}
09:41:26.004 rpc 29 orchestration.dispatchCommand message.dispatch
09:41:26.387 fetch POST /api/observability/v1/traces
09:41:26.387 fetched POST /api/observability/v1/traces 204
09:41:28.653 fetch POST /api/observability/v1/traces
09:41:28.653 fetched POST /api/observability/v1/traces 204
09:41:29.688 in Ping 
09:41:34.494 exit 29 {"_tag":"Success","value":{"sequence":138}}
09:41:34.688 in Ping 
09:41:35.497 fetch POST /api/observability/v1/traces
09:41:35.497 fetched POST /api/observability/v1/traces 204
09:41:39.688 in Ping 
09:41:39.930 rpc 30 server.reportClientActivity
09:41:40.930 fetch POST /api/observability/v1/traces
09:41:40.930 fetched POST /api/observability/v1/traces 204
09:41:44.440 chunk 17 38329 bytes
09:41:44.444 in Ack 17
09:41:44.689 in Ping 
09:41:45.445 fetch POST /api/observability/v1/traces
09:41:45.446 fetched POST /api/observability/v1/traces 204
09:41:48.959 chunk 17 38361 bytes
09:41:48.963 in Ack 17
09:41:49.689 in Ping 
09:41:49.965 fetch POST /api/observability/v1/traces
09:41:49.965 fetched POST /api/observability/v1/traces 204
09:41:50.664 chunk 17 38381 bytes
09:41:50.666 in Ack 17
09:41:51.668 fetch POST /api/observability/v1/traces
09:41:51.668 fetched POST /api/observability/v1/traces 204
09:41:54.689 in Ping 
09:41:54.939 in Interrupt 30
09:41:55.559 fetch POST /api/observability/v1/traces
09:41:55.559 fetched POST /api/observability/v1/traces 204
09:41:59.692 in Ping 
09:42:04.692 in Ping 
09:42:04.930 rpc 31 server.reportClientActivity
09:42:05.931 fetch POST /api/observability/v1/traces
09:42:05.931 fetched POST /api/observability/v1/traces 204
09:42:09.696 in Ping 
09:42:14.699 in Ping 
09:42:19.702 in Ping 
09:42:19.932 in Interrupt 31
09:42:20.933 fetch POST /api/observability/v1/traces
09:42:20.933 fetched POST /api/observability/v1/traces 204
09:42:24.706 in Ping 
09:42:29.711 in Ping 
09:42:29.932 rpc 32 server.reportClientActivity
09:42:30.932 fetch POST /api/observability/v1/traces
09:42:30.932 fetched POST /api/observability/v1/traces 204
09:42:34.714 in Ping 
09:42:39.718 in Ping 
09:42:44.723 in Ping 
09:42:44.933 in Interrupt 32
09:42:45.934 fetch POST /api/observability/v1/traces
09:42:45.934 fetched POST /api/observability/v1/traces 204
09:42:49.726 in Ping 
09:42:54.730 in Ping 
09:42:54.934 rpc 33 server.reportClientActivity
09:42:55.935 fetch POST /api/observability/v1/traces
09:42:55.935 fetched POST /api/observability/v1/traces 204
09:42:59.734 in Ping 
09:43:04.738 in Ping 
09:43:09.744 in Ping 
09:43:09.936 in Interrupt 33
09:43:10.941 fetch POST /api/observability/v1/traces
09:43:10.941 fetched POST /api/observability/v1/traces 204
09:43:14.746 in Ping 
09:43:19.752 in Ping 
09:43:19.935 rpc 34 server.reportClientActivity
09:43:20.936 fetch POST /api/observability/v1/traces
09:43:20.936 fetched POST /api/observability/v1/traces 204
09:43:24.754 in Ping 
09:43:29.758 in Ping 
09:43:34.764 in Ping 
09:43:34.936 in Interrupt 34
09:43:35.937 fetch POST /api/observability/v1/traces
09:43:35.937 fetched POST /api/observability/v1/traces 204
09:43:39.767 in Ping 
09:43:44.772 in Ping 
09:43:44.935 rpc 35 server.reportClientActivity
09:43:45.935 fetch POST /api/observability/v1/traces
09:43:45.935 fetched POST /api/observability/v1/traces 204
09:43:49.776 in Ping 
09:43:54.778 in Ping 
09:43:59.783 in Ping 
09:43:59.936 in Interrupt 35
09:44:00.937 fetch POST /api/observability/v1/traces
09:44:00.937 fetched POST /api/observability/v1/traces 204
09:44:04.791 in Ping 
09:44:07.731 chunk 17 38675 bytes
09:44:07.734 in Ack 17
09:44:08.234 rpc 36 server.reportClientActivity
09:44:08.736 fetch POST /api/observability/v1/traces
09:44:08.736 fetched POST /api/observability/v1/traces 204
09:44:09.342 chunk 17 39143 bytes
09:44:09.343 in Ack 17
09:44:09.592 chunk 17 40031 bytes
09:44:09.594 in Ack 17
09:44:09.794 in Ping 
09:44:09.843 chunk 17 40361 bytes
09:44:09.845 in Ack 17
09:44:10.173 chunk 17 41363 bytes
09:44:10.177 in Ack 17
09:44:10.346 fetch POST /api/observability/v1/traces
09:44:10.346 fetched POST /api/observability/v1/traces 204
09:44:10.431 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:44:11.431 fetch POST /api/observability/v1/traces
09:44:11.431 fetched POST /api/observability/v1/traces 204
09:44:12.216 exit 37 {"_tag":"Success","value":{"sequence":158}}
09:44:12.923 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:44:13.217 fetch POST /api/observability/v1/traces
09:44:13.217 fetched POST /api/observability/v1/traces 204
09:44:14.205 chunk 17 41406 bytes
09:44:14.207 in Ack 17
09:44:14.798 in Ping 
09:44:14.897 chunk 17 41408 bytes
09:44:14.898 exit 38 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:44:14.899 in Ack 17
09:44:15.129 exit 30 {"_tag":"Success","value":null}
09:44:15.208 fetch POST /api/observability/v1/traces
09:44:15.208 fetched POST /api/observability/v1/traces 204
09:44:15.437 exit 31 {"_tag":"Success","value":null}
09:44:16.839 chunk 6 4723 bytes
09:44:16.839 in Ack 6
09:44:16.842 rpc 39 orchestration.dispatchCommand thread.visit
09:44:16.842 exit 39 {"_tag":"Success","value":{"sequence":0}}
09:44:17.098 chunk 17 42044 bytes
09:44:17.099 in Ack 17
09:44:17.841 fetch POST /api/observability/v1/traces
09:44:17.841 fetched POST /api/observability/v1/traces 204
09:44:18.464 exit 32 {"_tag":"Success","value":null}
09:44:18.570 exit 33 {"_tag":"Success","value":null}
09:44:18.570 exit 34 {"_tag":"Success","value":null}
09:44:18.600 chunk 17 42884 bytes
09:44:18.601 in Ack 17
09:44:18.682 exit 35 {"_tag":"Success","value":null}
09:44:18.849 chunk 17 44905 bytes
09:44:18.851 in Ack 17
09:44:19.602 fetch POST /api/observability/v1/traces
09:44:19.602 fetched POST /api/observability/v1/traces 204
09:44:19.802 in Ping 
09:44:20.099 chunk 17 47928 bytes
09:44:20.101 in Ack 17
09:44:20.350 chunk 17 48140 bytes
09:44:20.352 in Ack 17
09:44:21.103 fetch POST /api/observability/v1/traces
09:44:21.103 fetched POST /api/observability/v1/traces 204
09:44:22.603 chunk 17 48140 bytes
09:44:22.605 in Ack 17
09:44:23.236 in Interrupt 36
09:44:23.488 rpc 40 server.reportClientActivity
09:44:23.606 fetch POST /api/observability/v1/traces
09:44:23.606 fetched POST /api/observability/v1/traces 204
09:44:24.092 chunk 6 4723 bytes
09:44:24.092 in Ack 6
09:44:24.806 in Ping 
09:44:25.093 fetch POST /api/observability/v1/traces
09:44:25.093 fetched POST /api/observability/v1/traces 204
09:44:25.103 chunk 17 51676 bytes
09:44:25.105 in Ack 17
09:44:25.854 chunk 17 51628 bytes
09:44:25.856 in Ack 17
09:44:26.107 fetch POST /api/observability/v1/traces
09:44:26.107 fetched POST /api/observability/v1/traces 204
09:44:26.603 chunk 17 52306 bytes
09:44:26.608 in Ack 17
09:44:26.848 rpc 41 orchestration.dispatchCommand thread.visit
09:44:26.848 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:44:27.779 fetch POST /api/observability/v1/traces
09:44:27.779 fetched POST /api/observability/v1/traces 204
