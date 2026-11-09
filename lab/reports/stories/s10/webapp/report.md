# story, webapp, warm start, seed 10

Profile story (graded): seed 10's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 196 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 66.4s |
| Prompt reached the agent | 39.9s |
| Time to first text | 171.9s |
| Whole reply shown | 193.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 131.0s / 37.0s / 121.0s |
| Longest silence | 171.9s |
| Approval shown after it was asked | 109.4s |
| Bytes on the wire down / up | 509219 / 161812 |
| Bytes the app received / sent | 121905 / 98218 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.0 | 0 | 0 |
| 40 | 0.0 | 0 | 0 |
| 50 | 10.0 | 0 | 397 |
| 60 | 20.0 | 0 | 803 |
| 70 | 30.0 | 0 | 1077 |
| 80 | 40.0 | 0 | 1077 |
| 90 | 50.0 | 0 | 1077 |
| 100 | 60.0 | 0 | 1077 |
| 110 | 70.0 | 0 | 1077 |
| 120 | 80.0 | 0 | 1077 |
| 130 | 90.0 | 0 | 1077 |
| 140 | 100.0 | 0 | 1077 |
| 150 | 110.0 | 0 | 1077 |
| 160 | 120.0 | 0 | 1077 |
| 170 | 130.0 | 0 | 1077 |
| 180 | 2.3 | 1077 | 1169 |
| 190 | 4.6 | 1378 | 1538 |
| 193 | 7.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"961/961","runs":"1014/1014 779/839","assistant":"32/32 21/21","user":"10/10","attempts":"659/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216908177,"silences":1,"longestSilenceMs":2481,"longestRecoveredGapMs":2481},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":4.137121154561937,"minRttMs":null,"sentPackets":145,"sentWireBytes":37429,"receivedPackets":165,"receivedWireBytes":81562,"lastHeardMs":19926,"unsyncedBytes":736,"inflightPackets":20,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"961/961","runs":"1014/1014 181/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216916308,"silences":2,"longestSilenceMs":2481,"longestRecoveredGapMs":2481},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":444.6199810102417,"minRttMs":3528,"sentPackets":162,"sentWireBytes":47597,"receivedPackets":179,"receivedWireBytes":85656,"lastHeardMs":28057,"unsyncedBytes":975,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"961/961","runs":"1014/1014 181/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216916308,"silences":2,"longestSilenceMs":2481,"longestRecoveredGapMs":2481},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":444.6199810102417,"minRttMs":3528,"sentPackets":165,"sentWireBytes":49283,"receivedPackets":179,"receivedWireBytes":85656,"lastHeardMs":28057,"unsyncedBytes":975,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"961/961","runs":"1014/1014 226/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216937863,"silences":3,"longestSilenceMs":19433,"longestRecoveredGapMs":19433},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":2752.2924833839616,"minRttMs":18906,"sentPackets":177,"sentWireBytes":56396,"receivedPackets":182,"receivedWireBytes":86492,"lastHeardMs":49612,"unsyncedBytes":817,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216940771,"silences":3,"longestSilenceMs":19433,"longestRecoveredGapMs":19433},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":2752.2924833839616,"minRttMs":18906,"sentPackets":177,"sentWireBytes":56396,"receivedPackets":184,"receivedWireBytes":87288,"lastHeardMs":52520,"unsyncedBytes":1274,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216940771,"silences":3,"longestSilenceMs":19433,"longestRecoveredGapMs":19433},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":2752.2924833839616,"minRttMs":18906,"sentPackets":177,"sentWireBytes":56396,"receivedPackets":184,"receivedWireBytes":87288,"lastHeardMs":52520,"unsyncedBytes":1274,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216940771,"silences":3,"longestSilenceMs":19433,"longestRecoveredGapMs":19433},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":2752.2924833839616,"minRttMs":18906,"sentPackets":181,"sentWireBytes":59124,"receivedPackets":184,"receivedWireBytes":87288,"lastHeardMs":52520,"unsyncedBytes":1732,"inflightPackets":54,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216977221,"silences":4,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":187,"receivedWireBytes":88803,"lastHeardMs":88970,"unsyncedBytes":915,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":1372,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":1829,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":1829,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":2286,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":185,"sentWireBytes":61852,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":2286,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":202,"sentWireBytes":66669,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":2286,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":221,"sentWireBytes":72061,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":2744,"inflightPackets":93,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":245,"sentWireBytes":78867,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":2744,"inflightPackets":117,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 194/498","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794216979264,"silences":5,"longestSilenceMs":33276,"longestRecoveredGapMs":33276},"stats":{"sendRate":2000,"deliveredRate":26.10481032459914,"loss":0.875,"srttMs":6928.505922960967,"minRttMs":18906,"sentPackets":265,"sentWireBytes":84538,"receivedPackets":188,"receivedWireBytes":89315,"lastHeardMs":91013,"unsyncedBytes":3202,"inflightPackets":137,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 382/1068","frame":"1029/1029","thread":"465/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 653/1340","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"650/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794217080552,"silences":6,"longestSilenceMs":99763,"longestRecoveredGapMs":99763},"stats":{"sendRate":2000,"deliveredRate":24.180558039194228,"loss":0.8948529411764705,"srttMs":6371.817682590846,"minRttMs":2475,"sentPackets":293,"sentWireBytes":92459,"receivedPackets":198,"receivedWireBytes":94273,"lastHeardMs":192301,"unsyncedBytes":2383,"inflightPackets":159,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1093/1093 1542/1542 849/849","frame":"1029/1029","thread":"579/961","runs":"1014/1014 235/971","assistant":"32/32 21/21 1340/1340","user":"10/10","attempts":"659/659 51/659","messages":"456/456 544/544","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809","checkpoints":"777/777 611/611","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":9,"newConnections":0,"handshakeMs":1480,"lastDataAt":1794217091694,"silences":6,"longestSilenceMs":99763,"longestRecoveredGapMs":99763},"stats":{"sendRate":2000,"deliveredRate":1451.7934856225386,"loss":0.10527855367647057,"srttMs":2585.2346274967285,"minRttMs":2266,"sentPackets":355,"sentWireBytes":108082,"receivedPackets":285,"receivedWireBytes":135317,"lastHeardMs":203443,"unsyncedBytes":0,"inflightPackets":140,"duplicateBytes":0}}
- shim trace
09:34:49.612 fetch GET /api/auth/session
09:34:49.617 fetched GET /api/auth/session 200
09:34:50.137 fetch GET /.well-known/t3/environment
09:34:50.139 fetched GET /.well-known/t3/environment 200
09:34:50.170 fetch GET /.well-known/t3/environment
09:34:50.172 fetched GET /.well-known/t3/environment 200
09:34:50.180 rpc 0 subscribeServerConfig
09:34:50.183 chunk 0 38462 bytes
09:34:50.322 in Ack 0
09:34:50.325 rpc 1 subscribeServerLifecycle
09:34:50.327 rpc 2 subscribeServerLifecycle
09:34:50.332 fetch GET /api/auth/session
09:34:50.334 fetched GET /api/auth/session 200
09:34:50.335 rpc 3 subscribeDeviceState
09:34:50.338 rpc 4 preview.reportProfiles
09:34:50.431 rpc 5 server.reportClientActivity
09:34:50.485 rpc 6 subscribeProjectClones
09:34:50.546 fetch GET /api/orchestration/threads/mcp%3A2bb8b73f-0a75-48ba-8cbc-6903f1a49c39/bounded
09:34:50.563 fetch GET /api/orchestration/shell
09:34:50.602 rpc 7 server.refreshProviders
09:34:50.775 rpc 8 orchestration.dispatchCommand thread.visit
09:34:50.775 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:34:50.822 fetched GET /api/orchestration/shell 200
09:34:50.822 rpc 9 subscribeTerminalMetadata
09:34:50.824 rpc 10 subscribeVcsStatus
09:34:50.825 rpc 11 subscribePreviewEvents
09:34:50.866 rpc 12 orchestration.subscribeShell
09:34:50.866 chunk 12 4594 bytes
09:34:50.869 rpc 13 assets.createUrl
09:34:50.871 rpc 14 preview.list
09:34:50.872 rpc 15 projects.readFile
09:34:50.891 in Ack 12
09:34:51.248 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"91cccf8a-82ce-4a9d-8dca-02f583574f0d","sessions":[]}}
09:34:51.248 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:34:51.249 chunk 12 4570 bytes
09:34:51.251 fetch POST /api/observability/v1/traces
09:34:51.251 fetched POST /api/observability/v1/traces 204
09:34:51.402 in Ack 12
09:34:51.663 chunk 1 3815 bytes
09:34:51.664 in Ack 1
09:34:51.851 chunk 2 3815 bytes
09:34:51.851 in Ack 2
09:34:52.028 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:34:19.464Z","configurableProviders":false,"con
09:34:52.028 chunk 10 1632 bytes
09:34:52.028 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:34:52.035 in Ack 10
09:34:52.382 fetch POST /api/observability/v1/traces
09:34:52.382 fetched POST /api/observability/v1/traces 204
09:34:52.528 chunk 12 4638 bytes
09:34:52.528 in Ack 12
09:34:52.847 chunk 3 876 bytes
09:34:52.847 exit 4 {"_tag":"Success","value":null}
09:34:52.847 exit 5 {"_tag":"Success","value":null}
09:34:52.847 chunk 6 4 bytes
09:34:52.848 in Ack 3
09:34:52.855 rpc 16 preview.reportProfiles
09:34:52.856 in Ack 6
09:34:52.934 chunk 9 36 bytes
09:34:52.934 in Ack 9
09:34:53.017 exit 16 {"_tag":"Success","value":null}
09:34:53.027 chunk 12 4635 bytes
09:34:53.028 in Ack 12
09:34:53.107 rpc 17 server.reportClientActivity
09:34:53.175 exit 17 {"_tag":"Success","value":null}
09:34:53.530 fetch POST /api/observability/v1/traces
09:34:53.530 fetched POST /api/observability/v1/traces 204
09:34:54.842 fetched GET /api/orchestration/threads/mcp%3A2bb8b73f-0a75-48ba-8cbc-6903f1a49c39/bounded 200
09:34:54.849 rpc 18 orchestration.subscribeThread
09:34:54.850 chunk 18 8282 bytes
09:34:54.868 in Interrupt 10
09:34:54.871 rpc 19 subscribeVcsStatus
09:34:54.872 in Ack 18
09:34:55.077 chunk 19 1632 bytes
09:34:55.078 in Ack 19
09:34:55.100 chunk 18 11259 bytes
09:34:55.102 in Ack 18
09:34:55.184 rpc 20 server.reportClientActivity
09:34:55.184 in Ping 
09:34:55.366 chunk 18 13497 bytes
09:34:55.379 in Ack 18
09:34:55.660 chunk 18 17166 bytes
09:34:55.662 in Ack 18
09:34:55.843 fetch POST /api/observability/v1/traces
09:34:55.843 fetched POST /api/observability/v1/traces 204
09:34:55.921 chunk 18 19796 bytes
09:34:55.987 in Ack 18
09:34:56.002 in Interrupt 19
09:34:56.008 rpc 21 subscribeVcsStatus
09:34:56.009 exit 20 {"_tag":"Success","value":null}
09:34:56.105 rpc 22 attachments.createUploadUrl
09:34:56.163 chunk 21 1632 bytes
09:34:56.208 in Ack 21
09:34:56.215 chunk 18 21099 bytes
09:34:56.221 in Ack 18
09:34:56.261 rpc 23 server.reportClientActivity
09:34:56.279 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-3305e996-4bae-4336-ad15-0d0fa1b371a1","expiresAt":1794217496230,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:34:56.280 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTMzMDVlOTk2LTRiYWUtNDMzNi1hZDE1LTBkMGZhMWIzNzFhMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzQ5NjIzMH0.T6ph4OskPHevgEwEVLL6dTOseYuHBMjF03nDWleErUo
09:34:56.591 exit 23 {"_tag":"Success","value":null}
09:34:56.989 fetch POST /api/observability/v1/traces
09:34:56.989 fetched POST /api/observability/v1/traces 204
09:35:00.184 in Ping 
09:35:02.350 rpc 24 orchestration.dispatchCommand message.dispatch
09:35:02.681 fetch POST /api/observability/v1/traces
09:35:02.682 fetched POST /api/observability/v1/traces 204
09:35:02.729 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:35:02.733 rpc 25 attachments.delete
09:35:03.541 chunk 18 23554 bytes
09:35:03.543 in Ack 18
09:35:03.750 fetch POST /api/observability/v1/traces
09:35:03.750 fetched POST /api/observability/v1/traces 204
09:35:04.309 chunk 18 23556 bytes
09:35:04.310 in Ack 18
09:35:05.063 chunk 12 4660 bytes
09:35:05.064 in Ack 12
09:35:05.073 rpc 26 orchestration.dispatchCommand thread.visit
09:35:05.073 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:35:05.184 in Ping 
09:35:05.331 fetch POST /api/observability/v1/traces
09:35:05.331 fetched POST /api/observability/v1/traces 204
09:35:05.942 rpc 27 server.refreshProviders
09:35:05.942 exit 27 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:34:19.464Z","configurableProviders":false,"con
09:35:06.683 fetch POST /api/observability/v1/traces
09:35:06.683 fetched POST /api/observability/v1/traces 204
09:35:08.090 rpc 28 orchestration.dispatchCommand message.dispatch
09:35:09.090 fetch POST /api/observability/v1/traces
09:35:09.090 fetched POST /api/observability/v1/traces 204
09:35:10.185 in Ping 
09:35:12.488 fetch POST /api/observability/v1/traces
09:35:12.488 fetched POST /api/observability/v1/traces 204
09:35:15.185 in Ping 
09:35:15.397 rpc 29 server.reportClientActivity
09:35:16.397 fetch POST /api/observability/v1/traces
09:35:16.397 fetched POST /api/observability/v1/traces 204
09:35:20.185 in Ping 
09:35:25.185 in Ping 
09:35:30.185 in Ping 
09:35:30.401 in Interrupt 29
09:35:31.402 fetch POST /api/observability/v1/traces
09:35:31.402 fetched POST /api/observability/v1/traces 204
09:35:35.186 in Ping 
09:35:40.186 in Ping 
09:35:40.397 rpc 30 server.reportClientActivity
09:35:41.397 fetch POST /api/observability/v1/traces
09:35:41.397 fetched POST /api/observability/v1/traces 204
09:35:45.186 in Ping 
09:35:50.191 in Ping 
09:35:55.194 in Ping 
09:35:55.398 in Interrupt 30
09:35:56.401 fetch POST /api/observability/v1/traces
09:35:56.401 fetched POST /api/observability/v1/traces 204
09:36:00.198 in Ping 
09:36:05.202 in Ping 
09:36:05.399 rpc 31 server.reportClientActivity
09:36:06.400 fetch POST /api/observability/v1/traces
09:36:06.400 fetched POST /api/observability/v1/traces 204
09:36:10.206 in Ping 
09:36:14.050 chunk 18 26363 bytes
09:36:14.051 exit 28 {"_tag":"Success","value":{"sequence":138}}
09:36:14.054 in Ack 18
09:36:15.058 fetch POST /api/observability/v1/traces
09:36:15.058 fetched POST /api/observability/v1/traces 204
09:36:15.211 in Ping 
09:36:20.214 in Ping 
09:36:20.400 in Interrupt 31
09:36:20.652 rpc 32 server.reportClientActivity
09:36:21.409 fetch POST /api/observability/v1/traces
09:36:21.409 fetched POST /api/observability/v1/traces 204
09:36:25.218 in Ping 
09:36:30.222 in Ping 
09:36:35.226 in Ping 
09:36:35.653 in Interrupt 32
09:36:35.905 rpc 33 server.reportClientActivity
09:36:36.654 fetch POST /api/observability/v1/traces
09:36:36.654 fetched POST /api/observability/v1/traces 204
09:36:40.230 in Ping 
09:36:45.234 in Ping 
09:36:50.238 in Ping 
09:36:50.906 in Interrupt 33
09:36:51.907 fetch POST /api/observability/v1/traces
09:36:51.907 fetched POST /api/observability/v1/traces 204
09:36:55.242 in Ping 
09:36:55.399 rpc 34 server.reportClientActivity
09:36:56.399 fetch POST /api/observability/v1/traces
09:36:56.399 fetched POST /api/observability/v1/traces 204
09:37:00.246 in Ping 
09:37:05.250 in Ping 
09:37:10.254 in Ping 
09:37:10.399 in Interrupt 34
09:37:11.401 fetch POST /api/observability/v1/traces
09:37:11.401 fetched POST /api/observability/v1/traces 204
09:37:15.258 in Ping 
09:37:20.262 in Ping 
09:37:20.399 rpc 35 server.reportClientActivity
09:37:21.399 fetch POST /api/observability/v1/traces
09:37:21.399 fetched POST /api/observability/v1/traces 204
09:37:25.267 in Ping 
09:37:30.271 in Ping 
09:37:35.274 in Ping 
09:37:35.399 in Interrupt 35
09:37:36.401 fetch POST /api/observability/v1/traces
09:37:36.401 fetched POST /api/observability/v1/traces 204
09:37:40.279 in Ping 
09:37:45.282 in Ping 
09:37:45.400 rpc 36 server.reportClientActivity
09:37:46.399 fetch POST /api/observability/v1/traces
09:37:46.400 fetched POST /api/observability/v1/traces 204
09:37:50.287 in Ping 
09:37:55.290 in Ping 
09:37:59.029 chunk 18 26725 bytes
09:37:59.032 in Ack 18
09:37:59.279 chunk 18 27147 bytes
09:37:59.281 in Ack 18
09:37:59.529 chunk 18 27281 bytes
09:37:59.531 in Ack 18
09:38:00.033 fetch POST /api/observability/v1/traces
09:38:00.033 fetched POST /api/observability/v1/traces 204
09:38:00.294 in Ping 
09:38:00.400 in Interrupt 36
09:38:00.652 rpc 37 server.reportClientActivity
09:38:01.292 exit 25 {"_tag":"Success","value":null}
09:38:01.401 fetch POST /api/observability/v1/traces
09:38:01.401 fetched POST /api/observability/v1/traces 204
09:38:01.835 chunk 18 27689 bytes
09:38:01.836 in Ack 18
09:38:02.085 chunk 18 27749 bytes
09:38:02.086 in Ack 18
09:38:02.334 chunk 18 28189 bytes
09:38:02.336 in Ack 18
09:38:02.584 chunk 18 28657 bytes
09:38:02.585 in Ack 18
09:38:02.869 fetch POST /api/observability/v1/traces
09:38:02.869 fetched POST /api/observability/v1/traces 204
09:38:03.899 chunk 18 29659 bytes
09:38:03.901 in Ack 18
09:38:04.111 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:04.903 fetch POST /api/observability/v1/traces
09:38:04.903 fetched POST /api/observability/v1/traces 204
09:38:05.299 in Ping 
09:38:06.174 chunk 18 33146 bytes
09:38:06.175 in Ack 18
09:38:06.700 exit 38 {"_tag":"Success","value":{"sequence":158}}
09:38:07.177 fetch POST /api/observability/v1/traces
09:38:07.177 fetched POST /api/observability/v1/traces 204
09:38:07.812 chunk 18 33189 bytes
09:38:07.813 in Ack 18
09:38:08.814 fetch POST /api/observability/v1/traces
09:38:08.814 fetched POST /api/observability/v1/traces 204
09:38:09.312 chunk 18 33191 bytes
09:38:09.314 in Ack 18
09:38:10.100 fetch POST /api/observability/v1/traces
09:38:10.100 fetched POST /api/observability/v1/traces 204
09:38:10.303 in Ping 
09:38:11.972 chunk 12 4723 bytes
09:38:11.973 in Ack 12
09:38:11.976 rpc 39 orchestration.dispatchCommand thread.visit
09:38:11.976 exit 39 {"_tag":"Success","value":{"sequence":0}}
09:38:12.223 chunk 18 33827 bytes
09:38:12.224 in Ack 18
09:38:12.974 fetch POST /api/observability/v1/traces
09:38:12.974 fetched POST /api/observability/v1/traces 204
09:38:12.975 chunk 18 36688 bytes
09:38:12.976 in Ack 18
09:38:13.225 chunk 18 37660 bytes
09:38:13.227 in Ack 18
09:38:13.475 chunk 18 38320 bytes
09:38:13.477 in Ack 18
09:38:13.977 chunk 18 38368 bytes
09:38:13.977 fetch POST /api/observability/v1/traces
09:38:13.977 fetched POST /api/observability/v1/traces 204
09:38:13.979 in Ack 18
09:38:14.727 chunk 18 40859 bytes
09:38:14.729 in Ack 18
09:38:14.977 chunk 18 40883 bytes
09:38:14.979 in Ack 18
09:38:15.007 fetch POST /api/observability/v1/traces
09:38:15.007 fetched POST /api/observability/v1/traces 204
09:38:15.228 chunk 18 40955 bytes
09:38:15.232 in Ack 18
09:38:15.313 in Ping 
09:38:15.478 chunk 18 41207 bytes
09:38:15.481 in Ack 18
09:38:15.655 in Interrupt 37
09:38:15.728 chunk 18 41573 bytes
09:38:15.730 in Ack 18
09:38:15.908 rpc 40 server.reportClientActivity
09:38:15.979 chunk 18 41603 bytes
09:38:15.981 in Ack 18
09:38:16.234 fetch POST /api/observability/v1/traces
09:38:16.234 fetched POST /api/observability/v1/traces 204
09:38:16.983 chunk 18 41555 bytes
09:38:16.985 in Ack 18
09:38:17.232 chunk 12 4723 bytes
09:38:17.233 in Ack 12
09:38:17.244 chunk 18 41555 bytes
09:38:17.246 in Ack 18
09:38:17.495 chunk 18 43671 bytes
09:38:17.496 in Ack 18
09:38:17.510 in Interrupt 21
09:38:17.559 rpc 41 subscribeVcsStatus
09:38:17.989 fetch POST /api/observability/v1/traces
09:38:17.989 fetched POST /api/observability/v1/traces 204
09:38:18.245 chunk 18 46095 bytes
09:38:18.247 in Ack 18
09:38:18.494 chunk 18 47050 bytes
09:38:18.496 in Ack 18
09:38:18.994 chunk 18 49427 bytes
09:38:18.996 in Ack 18
09:38:19.244 chunk 18 51316 bytes
09:38:19.246 in Ack 18
09:38:19.266 fetch POST /api/observability/v1/traces
09:38:19.267 fetched POST /api/observability/v1/traces 204
09:38:19.750 chunk 18 52333 bytes
09:38:19.753 in Ack 18
09:38:20.006 chunk 18 53513 bytes
09:38:20.035 in Ack 18
09:38:20.252 chunk 18 54301 bytes
09:38:20.254 in Ack 18
09:38:20.351 chunk 41 1713 bytes
09:38:20.351 in Ping 
09:38:20.355 in Ack 41
09:38:20.502 chunk 18 55111 bytes
09:38:20.506 in Ack 18
09:38:20.752 chunk 18 56662 bytes
09:38:20.756 in Ack 18
09:38:20.794 in Interrupt 41
09:38:20.799 rpc 42 subscribeVcsStatus
09:38:20.804 fetch POST /api/observability/v1/traces
09:38:20.804 fetched POST /api/observability/v1/traces 204
09:38:21.003 chunk 12 4662 bytes
09:38:21.008 chunk 18 57729 bytes
09:38:21.010 in Ack 12
09:38:21.090 in Ack 18
09:38:21.511 chunk 18 57729 bytes
09:38:21.662 in Ack 18
09:38:22.293 exit 29 {"_tag":"Success","value":null}
09:38:22.293 exit 30 {"_tag":"Success","value":null}
09:38:22.293 exit 31 {"_tag":"Success","value":null}
09:38:22.296 chunk 18 64175 bytes
09:38:22.314 rpc 43 orchestration.dispatchCommand thread.visit
09:38:22.314 exit 43 {"_tag":"Success","value":{"sequence":0}}
09:38:22.321 fetch POST /api/observability/v1/traces
09:38:22.321 fetched POST /api/observability/v1/traces 204
09:38:22.333 in Ack 18
09:38:22.337 rpc 44 assets.createUrl
