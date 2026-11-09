# story, webapp, warm start, seed 41

Profile story (graded): seed 41's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 180 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 12.4s |
| Prompt reached the agent | 9.9s |
| Time to first text | 53.5s |
| Whole reply shown | 179.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 43.0s / 0.0s / 34.0s |
| Longest silence | 119.5s |
| Approval shown after it was asked | 122.7s |
| Bytes on the wire down / up | 505892 / 143868 |
| Bytes the app received / sent | 168375 / 73427 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 10.0 | 0 | 397 |
| 30 | 20.0 | 0 | 803 |
| 40 | 30.0 | 0 | 1077 |
| 50 | 40.0 | 0 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 0.0 | 1077 | 1077 |
| 90 | 0.0 | 1077 | 1077 |
| 100 | 0.0 | 1077 | 1077 |
| 110 | 0.0 | 1077 | 1077 |
| 120 | 0.0 | 1077 | 1077 |
| 130 | 0.0 | 1077 | 1077 |
| 140 | 0.0 | 1077 | 1077 |
| 150 | 0.0 | 1077 | 1077 |
| 160 | 0.0 | 1077 | 1077 |
| 170 | 7.4 | 1077 | 1367 |
| 179 | 8.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1030/1030","thread":"506/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 361/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217107945,"silences":0,"longestSilenceMs":1082,"longestRecoveredGapMs":1082},"stats":{"sendRate":3000,"deliveredRate":1382.389937106918,"loss":0,"srttMs":9.54261071316508,"minRttMs":1,"sentPackets":101,"sentWireBytes":30111,"receivedPackets":110,"receivedWireBytes":75870,"lastHeardMs":10617,"unsyncedBytes":518,"inflightPackets":0,"duplicateBytes":0}}
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217119289,"silences":1,"longestSilenceMs":2161,"longestRecoveredGapMs":2161},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":9.54261071316508,"minRttMs":1,"sentPackets":116,"sentWireBytes":36755,"receivedPackets":123,"receivedWireBytes":79824,"lastHeardMs":21961,"unsyncedBytes":518,"inflightPackets":15,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217124537,"silences":2,"longestSilenceMs":2161,"longestRecoveredGapMs":2161},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1369.3497843740195,"minRttMs":1,"sentPackets":116,"sentWireBytes":36755,"receivedPackets":127,"receivedWireBytes":81668,"lastHeardMs":27209,"unsyncedBytes":457,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217124537,"silences":2,"longestSilenceMs":2161,"longestRecoveredGapMs":2161},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0,"srttMs":1369.3497843740195,"minRttMs":1,"sentPackets":128,"sentWireBytes":40194,"receivedPackets":127,"receivedWireBytes":81668,"lastHeardMs":27209,"unsyncedBytes":457,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217124537,"silences":2,"longestSilenceMs":2161,"longestRecoveredGapMs":2161},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0,"srttMs":1369.3497843740195,"minRttMs":1,"sentPackets":151,"sentWireBytes":46709,"receivedPackets":127,"receivedWireBytes":81668,"lastHeardMs":27209,"unsyncedBytes":914,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1027/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217159383,"silences":3,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":1778.8060613272671,"minRttMs":4645,"sentPackets":164,"sentWireBytes":50426,"receivedPackets":132,"receivedWireBytes":84177,"lastHeardMs":62055,"unsyncedBytes":779,"inflightPackets":59,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217163645,"silences":3,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":1778.8060613272671,"minRttMs":4645,"sentPackets":164,"sentWireBytes":50426,"receivedPackets":136,"receivedWireBytes":86187,"lastHeardMs":66318,"unsyncedBytes":779,"inflightPackets":59,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217163645,"silences":3,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":1778.8060613272671,"minRttMs":4645,"sentPackets":172,"sentWireBytes":52676,"receivedPackets":136,"receivedWireBytes":86187,"lastHeardMs":66318,"unsyncedBytes":1236,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217190055,"silences":4,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":190,"sentWireBytes":57742,"receivedPackets":137,"receivedWireBytes":86693,"lastHeardMs":92727,"unsyncedBytes":1069,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217197152,"silences":6,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":196,"sentWireBytes":59426,"receivedPackets":140,"receivedWireBytes":88323,"lastHeardMs":99824,"unsyncedBytes":1526,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217209942,"silences":9,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":196,"sentWireBytes":59426,"receivedPackets":145,"receivedWireBytes":90964,"lastHeardMs":112614,"unsyncedBytes":1526,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217209942,"silences":9,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":196,"sentWireBytes":59426,"receivedPackets":145,"receivedWireBytes":90964,"lastHeardMs":112614,"unsyncedBytes":1526,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217209942,"silences":9,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":196,"sentWireBytes":59426,"receivedPackets":145,"receivedWireBytes":90964,"lastHeardMs":112614,"unsyncedBytes":1984,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217209942,"silences":9,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":196,"sentWireBytes":59426,"receivedPackets":145,"receivedWireBytes":90964,"lastHeardMs":112614,"unsyncedBytes":1984,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217209942,"silences":9,"longestSilenceMs":28924,"longestRecoveredGapMs":28924},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":208,"sentWireBytes":62843,"receivedPackets":145,"receivedWireBytes":90964,"lastHeardMs":112614,"unsyncedBytes":2442,"inflightPackets":98,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 372/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"313/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217257753,"silences":10,"longestSilenceMs":45585,"longestRecoveredGapMs":45585},"stats":{"sendRate":1500,"deliveredRate":41.46651779918469,"loss":0.9047619047619048,"srttMs":3570.080303661359,"minRttMs":16109,"sentPackets":232,"sentWireBytes":69675,"receivedPackets":149,"receivedWireBytes":92995,"lastHeardMs":160425,"unsyncedBytes":2442,"inflightPackets":122,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 406/849 1542/1542 397/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10","messages":"456/456 544/544","attempts":"659/659 378/659","providerTurns":"772/772","nodes":"1016/1016 809/809","providerThreads":"807/807","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217271279,"silences":12,"longestSilenceMs":45585,"longestRecoveredGapMs":45585},"stats":{"sendRate":1500,"deliveredRate":135.3084411195156,"loss":0.6977777777777777,"srttMs":5247.894781667539,"minRttMs":2390,"sentPackets":247,"sentWireBytes":75708,"receivedPackets":221,"receivedWireBytes":128061,"lastHeardMs":173952,"unsyncedBytes":1458,"inflightPackets":125,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 849/849 1566/1566 947/947 1093/1093 1058/1058 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 73/73","messages":"456/456 544/544 451/451 544/544 543/543","attempts":"659/659 659/659 635/635","providerTurns":"772/772 654/654","nodes":"1016/1016 809/809 1392/1392 983/983 1016/1016 785/785 20/809","providerThreads":"809/809","providerSessions":"2653/2653","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":672,"lastDataAt":1794217282274,"silences":12,"longestSilenceMs":45585,"longestRecoveredGapMs":45585},"stats":{"sendRate":1500,"deliveredRate":1356.4007334470193,"loss":0.019999999999999993,"srttMs":2279.9956779505064,"minRttMs":2023,"sentPackets":309,"sentWireBytes":90994,"receivedPackets":338,"receivedWireBytes":189438,"lastHeardMs":184946,"unsyncedBytes":0,"inflightPackets":129,"duplicateBytes":0}}
- shim trace
09:38:17.961 fetch GET /api/auth/session
09:38:17.963 fetched GET /api/auth/session 200
09:38:18.135 fetch GET /.well-known/t3/environment
09:38:18.137 fetched GET /.well-known/t3/environment 200
09:38:18.162 fetch GET /.well-known/t3/environment
09:38:18.163 fetched GET /.well-known/t3/environment 200
09:38:18.173 rpc 0 subscribeServerConfig
09:38:18.177 chunk 0 38462 bytes
09:38:18.257 in Ack 0
09:38:18.260 rpc 1 subscribeServerLifecycle
09:38:18.261 rpc 2 subscribeServerLifecycle
09:38:18.264 fetch GET /api/auth/session
09:38:18.265 fetched GET /api/auth/session 200
09:38:18.266 rpc 3 subscribeDeviceState
09:38:18.268 rpc 4 preview.reportProfiles
09:38:18.372 rpc 5 subscribeProjectClones
09:38:18.382 fetch GET /api/orchestration/threads/mcp%3A6d78fbc7-623a-4260-8c19-be48ede8b3f7/bounded
09:38:18.414 fetch GET /api/orchestration/shell
09:38:18.416 fetched GET /api/orchestration/shell 200
09:38:18.418 rpc 6 orchestration.subscribeShell
09:38:18.418 chunk 6 2736 bytes
09:38:18.459 rpc 7 server.refreshProviders
09:38:18.521 rpc 8 orchestration.dispatchCommand thread.visit
09:38:18.521 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:38:18.546 rpc 9 subscribeTerminalMetadata
09:38:18.548 rpc 10 subscribeVcsStatus
09:38:18.549 rpc 11 subscribePreviewEvents
09:38:18.551 rpc 12 assets.createUrl
09:38:18.551 rpc 13 preview.list
09:38:18.552 rpc 14 projects.readFile
09:38:18.560 in Ack 6
09:38:18.669 chunk 6 2712 bytes
09:38:18.670 in Ack 6
09:38:18.801 rpc 15 server.reportClientActivity
09:38:19.159 fetch POST /api/observability/v1/traces
09:38:19.159 fetched POST /api/observability/v1/traces 204
09:38:19.173 chunk 1 3815 bytes
09:38:19.174 in Ack 1
09:38:19.219 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"54603e66-54ca-4436-9ace-84b3b0e1674c","sessions":[]}}
09:38:19.445 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:38:19.445 chunk 2 3815 bytes
09:38:19.449 in Ack 2
09:38:19.460 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:55.714Z","configurableProviders":false,"con
09:38:19.460 chunk 10 1632 bytes
09:38:19.464 in Ack 10
09:38:19.583 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:38:19.835 chunk 6 4635 bytes
09:38:19.836 in Ack 6
09:38:19.980 chunk 3 876 bytes
09:38:19.980 exit 4 {"_tag":"Success","value":null}
09:38:19.980 in Ack 3
09:38:19.986 rpc 16 preview.reportProfiles
09:38:20.046 chunk 5 4 bytes
09:38:20.046 chunk 9 36 bytes
09:38:20.046 exit 15 {"_tag":"Success","value":null}
09:38:20.046 exit 16 {"_tag":"Success","value":null}
09:38:20.046 in Ack 5
09:38:20.047 in Ack 9
09:38:20.212 fetch POST /api/observability/v1/traces
09:38:20.212 fetched POST /api/observability/v1/traces 204
09:38:20.807 fetched GET /api/orchestration/threads/mcp%3A6d78fbc7-623a-4260-8c19-be48ede8b3f7/bounded 200
09:38:20.810 rpc 17 orchestration.subscribeThread
09:38:20.811 chunk 17 8283 bytes
09:38:20.825 in Interrupt 10
09:38:20.828 rpc 18 subscribeVcsStatus
09:38:20.829 in Ack 17
09:38:20.941 chunk 18 1632 bytes
09:38:20.941 in Ack 18
09:38:21.074 chunk 17 11260 bytes
09:38:21.076 in Ack 17
09:38:21.130 rpc 19 server.reportClientActivity
09:38:21.325 chunk 17 13498 bytes
09:38:21.326 in Ack 17
09:38:21.563 exit 19 {"_tag":"Success","value":null}
09:38:21.578 chunk 17 19020 bytes
09:38:21.579 in Ack 17
09:38:21.808 fetch POST /api/observability/v1/traces
09:38:21.808 fetched POST /api/observability/v1/traces 204
09:38:21.828 chunk 17 21100 bytes
09:38:21.829 in Ack 17
09:38:21.838 in Interrupt 18
09:38:21.842 rpc 20 subscribeVcsStatus
09:38:21.929 chunk 20 1632 bytes
09:38:21.930 in Ack 20
09:38:22.053 rpc 21 attachments.createUploadUrl
09:38:22.110 rpc 22 server.reportClientActivity
09:38:22.198 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-6326a7f3-c364-4c37-87e3-bf184b95aa8f","expiresAt":1794217702128,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:38:22.198 exit 22 {"_tag":"Success","value":null}
09:38:22.199 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTYzMjZhN2YzLWMzNjQtNGMzNy04N2UzLWJmMTg0Yjk1YWE4ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzcwMjEyOH0.PeLtkkwlnOyfkYz7vDR1eQRf8F08-w2glsQJrIFq5-k
09:38:22.846 fetch POST /api/observability/v1/traces
09:38:22.846 fetched POST /api/observability/v1/traces 204
09:38:23.169 in Ping 
09:38:25.215 rpc 23 orchestration.dispatchCommand message.dispatch
09:38:25.531 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:38:25.534 rpc 24 attachments.delete
09:38:26.214 fetch POST /api/observability/v1/traces
09:38:26.214 fetched POST /api/observability/v1/traces 204
09:38:26.296 chunk 17 23555 bytes
09:38:26.298 in Ack 17
09:38:26.713 exit 24 {"_tag":"Success","value":null}
09:38:26.799 chunk 17 23557 bytes
09:38:26.801 in Ack 17
09:38:27.061 chunk 17 24529 bytes
09:38:27.064 in Ack 17
09:38:27.292 chunk 6 4657 bytes
09:38:27.293 in Ack 6
09:38:27.294 rpc 25 orchestration.dispatchCommand thread.visit
09:38:27.294 exit 25 {"_tag":"Success","value":{"sequence":0}}
09:38:27.312 fetch POST /api/observability/v1/traces
09:38:27.312 fetched POST /api/observability/v1/traces 204
09:38:28.171 in Ping 
09:38:29.290 rpc 26 orchestration.dispatchCommand message.dispatch
09:38:30.293 fetch POST /api/observability/v1/traces
09:38:30.293 fetched POST /api/observability/v1/traces 204
09:38:31.167 chunk 6 4657 bytes
09:38:31.168 in Ack 6
09:38:32.169 fetch POST /api/observability/v1/traces
09:38:32.169 fetched POST /api/observability/v1/traces 204
09:38:33.176 in Ping 
09:38:37.013 chunk 17 24529 bytes
09:38:37.015 in Ack 17
09:38:38.018 fetch POST /api/observability/v1/traces
09:38:38.018 fetched POST /api/observability/v1/traces 204
09:38:38.178 in Ping 
09:38:40.381 fetch POST /api/observability/v1/traces
09:38:40.381 fetched POST /api/observability/v1/traces 204
09:38:41.361 exit 26 {"_tag":"Success","value":{"sequence":138}}
09:38:42.345 fetch POST /api/observability/v1/traces
09:38:42.345 fetched POST /api/observability/v1/traces 204
09:38:43.179 in Ping 
09:38:43.397 rpc 27 server.reportClientActivity
09:38:44.397 fetch POST /api/observability/v1/traces
09:38:44.397 fetched POST /api/observability/v1/traces 204
09:38:48.180 in Ping 
09:38:53.180 in Ping 
09:38:58.181 in Ping 
09:38:58.414 in Interrupt 27
09:38:59.415 fetch POST /api/observability/v1/traces
09:38:59.415 fetched POST /api/observability/v1/traces 204
09:39:03.182 in Ping 
09:39:08.182 in Ping 
09:39:08.396 rpc 28 server.reportClientActivity
09:39:09.397 fetch POST /api/observability/v1/traces
09:39:09.397 fetched POST /api/observability/v1/traces 204
09:39:13.185 in Ping 
09:39:13.462 chunk 17 27690 bytes
09:39:13.464 in Ack 17
09:39:14.466 fetch POST /api/observability/v1/traces
09:39:14.466 fetched POST /api/observability/v1/traces 204
09:39:15.068 chunk 17 28352 bytes
09:39:15.070 in Ack 17
09:39:16.071 fetch POST /api/observability/v1/traces
09:39:16.071 fetched POST /api/observability/v1/traces 204
09:39:18.190 in Ping 
09:39:19.384 chunk 17 29006 bytes
09:39:19.386 in Ack 17
09:39:20.387 fetch POST /api/observability/v1/traces
09:39:20.387 fetched POST /api/observability/v1/traces 204
09:39:22.563 chunk 17 29630 bytes
09:39:22.565 in Ack 17
09:39:23.193 in Ping 
09:39:23.397 in Interrupt 28
09:39:23.566 fetch POST /api/observability/v1/traces
09:39:23.566 fetched POST /api/observability/v1/traces 204
09:39:24.610 fetch POST /api/observability/v1/traces
09:39:24.610 fetched POST /api/observability/v1/traces 204
09:39:28.198 in Ping 
09:39:33.201 in Ping 
09:39:33.398 rpc 29 server.reportClientActivity
09:39:34.398 fetch POST /api/observability/v1/traces
09:39:34.398 fetched POST /api/observability/v1/traces 204
09:39:38.205 in Ping 
09:39:43.209 in Ping 
09:39:48.213 in Ping 
09:39:48.398 in Interrupt 29
09:39:49.399 fetch POST /api/observability/v1/traces
09:39:49.399 fetched POST /api/observability/v1/traces 204
09:39:53.217 in Ping 
09:39:58.221 in Ping 
09:39:58.406 rpc 30 server.reportClientActivity
09:39:59.406 fetch POST /api/observability/v1/traces
09:39:59.406 fetched POST /api/observability/v1/traces 204
09:40:03.225 in Ping 
09:40:08.229 in Ping 
09:40:13.233 in Ping 
09:40:13.407 in Interrupt 30
09:40:14.409 fetch POST /api/observability/v1/traces
09:40:14.409 fetched POST /api/observability/v1/traces 204
09:40:18.238 in Ping 
09:40:23.241 in Ping 
09:40:23.400 rpc 31 server.reportClientActivity
09:40:24.400 fetch POST /api/observability/v1/traces
09:40:24.400 fetched POST /api/observability/v1/traces 204
09:40:28.245 in Ping 
09:40:33.249 in Ping 
09:40:38.253 in Ping 
09:40:38.401 in Interrupt 31
09:40:39.402 fetch POST /api/observability/v1/traces
09:40:39.402 fetched POST /api/observability/v1/traces 204
09:40:43.258 in Ping 
09:40:48.261 in Ping 
09:40:48.401 rpc 32 server.reportClientActivity
09:40:49.401 fetch POST /api/observability/v1/traces
09:40:49.401 fetched POST /api/observability/v1/traces 204
09:40:53.265 in Ping 
09:40:58.269 in Ping 
09:41:03.274 in Ping 
09:41:03.403 in Interrupt 32
09:41:04.404 fetch POST /api/observability/v1/traces
09:41:04.404 fetched POST /api/observability/v1/traces 204
09:41:08.278 in Ping 
09:41:08.465 chunk 17 30632 bytes
09:41:08.467 in Ack 17
09:41:08.679 rpc 33 orchestration.dispatchCommand runtime-request.respond
09:41:08.896 rpc 34 server.reportClientActivity
09:41:09.468 fetch POST /api/observability/v1/traces
09:41:09.468 fetched POST /api/observability/v1/traces 204
09:41:09.597 chunk 17 34119 bytes
09:41:09.598 in Ack 17
09:41:10.599 fetch POST /api/observability/v1/traces
09:41:10.600 fetched POST /api/observability/v1/traces 204
09:41:11.883 chunk 17 36187 bytes
09:41:11.885 in Ack 17
09:41:12.886 fetch POST /api/observability/v1/traces
09:41:12.886 fetched POST /api/observability/v1/traces 204
09:41:12.919 exit 33 {"_tag":"Success","value":{"sequence":182}}
09:41:13.281 in Ping 
09:41:13.450 chunk 17 36230 bytes
09:41:13.451 in Ack 17
09:41:13.920 fetch POST /api/observability/v1/traces
09:41:13.920 fetched POST /api/observability/v1/traces 204
09:41:14.949 chunk 17 36232 bytes
09:41:14.951 in Ack 17
09:41:15.950 chunk 6 4723 bytes
09:41:15.951 in Ack 6
09:41:15.954 rpc 35 orchestration.dispatchCommand thread.visit
09:41:15.954 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:41:15.990 fetch POST /api/observability/v1/traces
09:41:15.990 fetched POST /api/observability/v1/traces 204
09:41:16.750 chunk 17 36868 bytes
09:41:16.752 in Ack 17
09:41:16.991 fetch POST /api/observability/v1/traces
09:41:16.991 fetched POST /api/observability/v1/traces 204
09:41:17.501 chunk 17 37708 bytes
09:41:17.502 in Ack 17
09:41:17.751 chunk 17 40389 bytes
09:41:17.752 in Ack 17
09:41:18.001 chunk 17 40437 bytes
09:41:18.003 in Ack 17
09:41:18.285 in Ping 
09:41:18.503 fetch POST /api/observability/v1/traces
09:41:18.503 fetched POST /api/observability/v1/traces 204
09:41:18.751 chunk 6 4723 bytes
09:41:18.751 chunk 17 40437 bytes
09:41:18.752 in Ack 6
09:41:18.760 in Ack 17
09:41:19.252 chunk 17 40485 bytes
09:41:19.253 in Ack 17
09:41:19.262 in Interrupt 20
09:41:19.293 rpc 36 subscribeVcsStatus
09:41:19.585 fetch POST /api/observability/v1/traces
09:41:19.585 fetched POST /api/observability/v1/traces 204
09:41:19.754 chunk 17 42908 bytes
09:41:19.755 in Ack 17
09:41:20.004 chunk 17 43892 bytes
09:41:20.005 in Ack 17
09:41:20.086 exit 27 {"_tag":"Success","value":null}
09:41:20.086 exit 28 {"_tag":"Success","value":null}
09:41:20.254 chunk 17 45285 bytes
09:41:20.256 in Ack 17
09:41:20.755 chunk 17 50062 bytes
09:41:20.757 in Ack 17
09:41:20.819 fetch POST /api/observability/v1/traces
09:41:20.819 fetched POST /api/observability/v1/traces 204
09:41:21.005 chunk 17 50410 bytes
09:41:21.007 in Ack 17
09:41:21.256 chunk 17 50362 bytes
09:41:21.258 in Ack 17
09:41:21.505 chunk 6 4723 bytes
09:41:21.505 chunk 17 51317 bytes
09:41:21.506 in Ack 6
09:41:21.509 in Ack 17
09:41:21.586 chunk 36 1713 bytes
09:41:21.586 in Ack 36
09:41:21.756 chunk 17 52334 bytes
09:41:21.758 in Ack 17
09:41:22.007 chunk 17 53775 bytes
09:41:22.008 fetch POST /api/observability/v1/traces
09:41:22.008 fetched POST /api/observability/v1/traces 204
09:41:22.009 in Ack 17
09:41:22.258 chunk 17 54302 bytes
09:41:22.259 in Ack 17
09:41:22.507 chunk 17 55112 bytes
09:41:22.508 in Ack 17
09:41:22.758 chunk 17 56663 bytes
09:41:22.760 in Ack 17
09:41:22.771 in Interrupt 36
09:41:22.774 rpc 37 subscribeVcsStatus
09:41:23.007 chunk 17 58246 bytes
09:41:23.009 in Ack 17
09:41:23.027 fetch POST /api/observability/v1/traces
09:41:23.027 fetched POST /api/observability/v1/traces 204
09:41:23.257 chunk 6 5094 bytes
09:41:23.257 chunk 17 61951 bytes
09:41:23.258 in Ack 6
09:41:23.261 in Ack 17
09:41:23.264 rpc 38 assets.createUrl
09:41:23.320 exit 29 {"_tag":"Success","value":null}
09:41:23.321 in Ping 
09:41:23.515 exit 30 {"_tag":"Success","value":null}
09:41:23.515 exit 31 {"_tag":"Success","value":null}
09:41:23.515 exit 32 {"_tag":"Success","value":null}
09:41:23.515 exit 34 {"_tag":"Success","value":null}
09:41:23.768 rpc 39 server.reportClientActivity
09:41:24.261 fetch POST /api/observability/v1/traces
09:41:24.261 fetched POST /api/observability/v1/traces 204
09:41:25.109 chunk 37 1713 bytes
09:41:25.110 in Ack 37
09:41:25.954 rpc 40 orchestration.dispatchCommand thread.visit
09:41:25.954 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:41:26.954 fetch POST /api/observability/v1/traces
09:41:26.954 fetched POST /api/observability/v1/traces 204
09:41:27.129 chunk 17 63018 bytes
09:41:27.132 in Ack 17
09:41:27.379 chunk 6 5033 bytes
09:41:27.380 chunk 17 63018 bytes
09:41:27.380 in Ack 6
09:41:27.391 in Ack 17
09:41:27.581 exit 39 {"_tag":"Success","value":null}
09:41:27.630 chunk 17 63172 bytes
09:41:27.632 in Ack 17
09:41:27.879 chunk 6 5030 bytes
09:41:27.880 in Ack 6
09:41:27.882 rpc 41 orchestration.dispatchCommand thread.visit
09:41:27.882 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:41:28.137 fetch POST /api/observability/v1/traces
09:41:28.138 fetched POST /api/observability/v1/traces 204
09:41:28.158 chunk 17 66516 bytes
09:41:28.160 in Ack 17
09:41:28.170 in Interrupt 37
09:41:28.173 rpc 42 subscribeVcsStatus
09:41:28.495 in Ping 
09:41:28.496 chunk 17 67534 bytes
09:41:28.497 rpc 43 server.reportClientActivity
09:41:28.499 in Ack 17
