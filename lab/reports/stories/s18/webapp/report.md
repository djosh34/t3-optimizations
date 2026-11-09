# story, webapp, warm start, seed 18

Profile story (graded): seed 18's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 285.0s |
| Prompt reached the agent | 283.6s |
| Time to first text | 295.2s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 397 / 569 / 1596 |
| Newest-text lag: max / median / p95 | 11.3s / 0.0s / 0.0s |
| Longest silence | 295.2s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 121040 / 62762 |
| Bytes the app received / sent | 57557 / 46237 |

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
| 290 | 6.3 | 0 | 254 |
| 297 | 3.3 | 397 | 535 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"773/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456","checkpoints":"777/777 611/611","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217010925,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":6750,"deliveredRate":1340.909090909091,"loss":0,"srttMs":55.65997140045173,"minRttMs":2,"sentPackets":125,"sentWireBytes":36824,"receivedPackets":156,"receivedWireBytes":91797,"lastHeardMs":17599,"unsyncedBytes":736,"inflightPackets":21,"duplicateBytes":0}}
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":3375,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":2,"sentPackets":137,"sentWireBytes":43208,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":975,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":2000,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":140,"sentWireBytes":44859,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":975,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":1000,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":149,"sentWireBytes":49814,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":975,"inflightPackets":44,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":153,"sentWireBytes":52149,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":1432,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":154,"sentWireBytes":52831,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":1432,"inflightPackets":49,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":155,"sentWireBytes":53513,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":1890,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":155,"sentWireBytes":53513,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":1890,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":156,"sentWireBytes":54195,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":1890,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":2348,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":2348,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":2806,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":2972,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":2972,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":3430,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":3430,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":3888,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":3888,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":3888,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":4346,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":4346,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":4804,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":4804,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":4804,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":5262,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":5262,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":5720,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":4,"newConnections":2,"handshakeMs":836,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":157,"sentWireBytes":54877,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":5720,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 48/772","providerThreads":"446/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 315/809","messages":"544/544 456/456 21/901","checkpoints":"777/777 611/611 98/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc open","open":true,"generation":4,"iceRestarts":4,"newConnections":2,"handshakeMs":12452,"lastDataAt":1794217020701,"silences":1,"longestSilenceMs":2395,"longestRecoveredGapMs":2395},"stats":{"sendRate":50,"deliveredRate":1194.0837733694875,"loss":0,"srttMs":639.8274749753953,"minRttMs":null,"sentPackets":160,"sentWireBytes":56382,"receivedPackets":167,"receivedWireBytes":96463,"lastHeardMs":27375,"unsyncedBytes":6486,"inflightPackets":3,"duplicateBytes":0}}
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 48/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 785/785","messages":"544/544 456/456 21/901 350/451","checkpoints":"777/777 611/611 563/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":4,"newConnections":2,"handshakeMs":12452,"lastDataAt":1794217305298,"silences":2,"longestSilenceMs":275053,"longestRecoveredGapMs":2395},"stats":{"sendRate":2151.66873320819,"deliveredRate":1721.1092729412476,"loss":0,"srttMs":2016.372032485064,"minRttMs":1963,"sentPackets":223,"sentWireBytes":71537,"receivedPackets":250,"receivedWireBytes":128614,"lastHeardMs":311972,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
09:36:34.285 fetch GET /api/auth/session
09:36:34.288 fetched GET /api/auth/session 200
09:36:34.779 fetch GET /.well-known/t3/environment
09:36:34.780 fetched GET /.well-known/t3/environment 200
09:36:34.804 fetch GET /.well-known/t3/environment
09:36:34.806 fetched GET /.well-known/t3/environment 200
09:36:34.815 rpc 0 subscribeServerConfig
09:36:34.816 chunk 0 38462 bytes
09:36:34.907 in Ack 0
09:36:34.911 rpc 1 subscribeServerLifecycle
09:36:34.912 rpc 2 subscribeServerLifecycle
09:36:34.919 fetch GET /api/auth/session
09:36:34.929 fetched GET /api/auth/session 200
09:36:34.930 rpc 3 subscribeDeviceState
09:36:34.937 rpc 4 preview.reportProfiles
09:36:35.035 rpc 5 subscribeProjectClones
09:36:35.048 fetch GET /api/orchestration/shell
09:36:35.050 fetched GET /api/orchestration/shell 200
09:36:35.057 rpc 6 orchestration.subscribeShell
09:36:35.057 chunk 6 4551 bytes
09:36:35.124 rpc 7 server.refreshProviders
09:36:35.279 rpc 8 orchestration.dispatchCommand thread.visit
09:36:35.279 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:36:35.334 rpc 9 subscribeTerminalMetadata
09:36:35.336 rpc 10 subscribeVcsStatus
09:36:35.338 rpc 11 subscribePreviewEvents
09:36:35.340 rpc 12 assets.createUrl
09:36:35.342 rpc 13 preview.list
09:36:35.343 rpc 14 projects.readFile
09:36:35.353 fetch GET /api/orchestration/threads/mcp%3Aa93b2cd4-a1fe-4d76-9086-a1ab10803a2b/bounded
09:36:35.363 in Ack 6
09:36:35.668 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"63a474fd-ebc0-4867-938a-abc23b1d2b4b","sessions":[]}}
09:36:35.668 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:36:35.683 rpc 15 server.reportClientActivity
09:36:35.821 fetch POST /api/observability/v1/traces
09:36:35.821 fetched POST /api/observability/v1/traces 204
09:36:36.030 chunk 1 3815 bytes
09:36:36.031 in Ack 1
09:36:36.246 chunk 2 3815 bytes
09:36:36.246 in Ack 2
09:36:36.440 chunk 6 4527 bytes
09:36:36.440 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:04.573Z","configurableProviders":false,"con
09:36:36.440 chunk 10 1632 bytes
09:36:36.440 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:36:36.441 in Ack 6
09:36:36.452 in Ack 10
09:36:36.854 fetch POST /api/observability/v1/traces
09:36:36.854 fetched POST /api/observability/v1/traces 204
09:36:36.942 chunk 6 4613 bytes
09:36:36.943 in Ack 6
09:36:37.192 chunk 6 4595 bytes
09:36:37.193 in Ack 6
09:36:37.426 chunk 3 876 bytes
09:36:37.426 exit 4 {"_tag":"Success","value":null}
09:36:37.426 chunk 5 4 bytes
09:36:37.426 chunk 9 36 bytes
09:36:37.426 exit 15 {"_tag":"Success","value":null}
09:36:37.430 in Ack 3
09:36:37.452 in Ack 5
09:36:37.453 in Ack 9
09:36:37.944 fetch POST /api/observability/v1/traces
09:36:37.945 fetched POST /api/observability/v1/traces 204
09:36:37.961 chunk 6 4592 bytes
09:36:37.962 in Ack 6
09:36:38.211 chunk 6 4592 bytes
09:36:38.212 in Ack 6
09:36:38.963 fetch POST /api/observability/v1/traces
09:36:38.963 fetched POST /api/observability/v1/traces 204
09:36:39.810 in Ping 
09:36:40.245 fetched GET /api/orchestration/threads/mcp%3Aa93b2cd4-a1fe-4d76-9086-a1ab10803a2b/bounded 200
09:36:40.252 rpc 16 orchestration.subscribeThread
09:36:40.253 chunk 16 8282 bytes
09:36:40.278 in Interrupt 10
09:36:40.281 rpc 17 subscribeVcsStatus
09:36:40.283 in Ack 16
09:36:40.484 chunk 17 1632 bytes
09:36:40.484 in Ack 17
09:36:40.503 chunk 16 11259 bytes
09:36:40.505 in Ack 16
09:36:40.572 rpc 18 server.reportClientActivity
09:36:40.753 chunk 16 12690 bytes
09:36:40.760 in Ack 16
09:36:40.857 exit 18 {"_tag":"Success","value":null}
09:36:41.042 chunk 16 16150 bytes
09:36:41.044 in Ack 16
09:36:41.251 fetch POST /api/observability/v1/traces
09:36:41.251 fetched POST /api/observability/v1/traces 204
09:36:41.327 chunk 16 18552 bytes
09:36:41.328 in Ack 16
09:36:41.437 rpc 19 attachments.createUploadUrl
09:36:41.578 chunk 16 20408 bytes
09:36:41.579 in Ack 16
09:36:41.602 in Interrupt 17
09:36:41.608 rpc 20 subscribeVcsStatus
09:36:41.609 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-1c91c64d-95f1-4bb0-a22e-c010e4c417ec","expiresAt":1794217601536,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:36:41.612 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTFjOTFjNjRkLTk1ZjEtNGJiMC1hMjJlLWMwMTBlNGM0MTdlYyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzYwMTUzNn0.Ft4G_yYt9JaupJ8pi_YYa4sHPs4ka4xQRPs7H9u97B0
09:36:41.675 chunk 20 1632 bytes
09:36:41.675 in Ack 20
09:36:41.827 chunk 16 21099 bytes
09:36:41.829 in Ack 16
09:36:41.859 rpc 21 server.reportClientActivity
09:36:42.329 fetch POST /api/observability/v1/traces
09:36:42.329 fetched POST /api/observability/v1/traces 204
09:36:43.216 exit 21 {"_tag":"Success","value":null}
09:36:44.217 fetch POST /api/observability/v1/traces
09:36:44.217 fetched POST /api/observability/v1/traces 204
09:36:45.025 rpc 22 orchestration.dispatchCommand message.dispatch
09:36:45.416 in Ping 
09:36:45.510 exit 22 {"_tag":"Success","value":{"sequence":109}}
09:36:45.514 rpc 23 attachments.delete
09:36:46.022 fetch POST /api/observability/v1/traces
09:36:46.022 fetched POST /api/observability/v1/traces 204
09:36:46.053 exit 23 {"_tag":"Success","value":null}
09:36:46.340 chunk 16 23554 bytes
09:36:46.342 in Ack 16
09:36:46.595 chunk 16 23556 bytes
09:36:46.597 in Ack 16
09:36:46.853 chunk 6 4617 bytes
09:36:46.855 in Ack 6
09:36:46.856 rpc 24 orchestration.dispatchCommand thread.visit
09:36:46.856 exit 24 {"_tag":"Success","value":{"sequence":0}}
09:36:47.055 fetch POST /api/observability/v1/traces
09:36:47.055 fetched POST /api/observability/v1/traces 204
09:36:47.365 chunk 6 4617 bytes
09:36:47.365 chunk 16 24396 bytes
09:36:47.366 in Ack 6
09:36:47.375 in Ack 16
09:36:47.617 chunk 16 25056 bytes
09:36:47.621 in Ack 16
09:36:47.868 chunk 6 4614 bytes
09:36:47.869 chunk 16 27981 bytes
09:36:47.869 in Ack 6
09:36:47.872 rpc 25 orchestration.dispatchCommand thread.visit
09:36:47.872 exit 25 {"_tag":"Success","value":{"sequence":0}}
09:36:47.881 in Ack 16
09:36:47.884 rpc 26 assets.createUrl
09:36:48.428 chunk 6 4614 bytes
09:36:48.428 chunk 16 30404 bytes
09:36:48.430 fetch POST /api/observability/v1/traces
09:36:48.430 fetched POST /api/observability/v1/traces 204
09:36:48.433 in Ack 6
09:36:48.445 in Ack 16
09:36:48.737 rpc 27 server.refreshProviders
09:36:48.737 exit 27 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:04.573Z","configurableProviders":false,"con
09:36:49.463 fetch POST /api/observability/v1/traces
09:36:49.463 fetched POST /api/observability/v1/traces 204
09:36:50.437 in Ping 
09:36:51.603 rpc 28 orchestration.dispatchCommand message.dispatch
09:36:51.980 fetch POST /api/observability/v1/traces
09:36:51.980 fetched POST /api/observability/v1/traces 204
09:36:55.438 in Ping 
09:36:56.072 chunk 16 30404 bytes
09:36:56.087 in Ack 16
09:36:56.841 fetch POST /api/observability/v1/traces
09:36:56.841 fetched POST /api/observability/v1/traces 204
09:37:00.043 rpc 29 server.reportClientActivity
09:37:00.439 in Ping 
09:37:01.043 fetch POST /api/observability/v1/traces
09:37:01.043 fetched POST /api/observability/v1/traces 204
09:37:05.441 in Ping 
09:37:10.441 in Ping 
09:37:15.049 in Interrupt 29
09:37:15.442 in Ping 
09:37:16.051 fetch POST /api/observability/v1/traces
09:37:16.051 fetched POST /api/observability/v1/traces 204
09:37:20.442 in Ping 
09:37:25.041 rpc 30 server.reportClientActivity
09:37:25.442 in Ping 
09:37:26.041 fetch POST /api/observability/v1/traces
09:37:26.041 fetched POST /api/observability/v1/traces 204
09:37:30.443 in Ping 
09:37:35.447 in Ping 
09:37:40.042 in Interrupt 30
09:37:40.452 in Ping 
09:37:41.043 fetch POST /api/observability/v1/traces
09:37:41.043 fetched POST /api/observability/v1/traces 204
09:37:45.455 in Ping 
09:37:50.044 rpc 31 server.reportClientActivity
09:37:50.459 in Ping 
09:37:51.044 fetch POST /api/observability/v1/traces
09:37:51.044 fetched POST /api/observability/v1/traces 204
09:37:55.463 in Ping 
09:38:00.468 in Ping 
09:38:05.045 in Interrupt 31
09:38:05.472 in Ping 
09:38:06.046 fetch POST /api/observability/v1/traces
09:38:06.046 fetched POST /api/observability/v1/traces 204
09:38:10.475 in Ping 
09:38:15.044 rpc 32 server.reportClientActivity
09:38:15.481 in Ping 
09:38:16.044 fetch POST /api/observability/v1/traces
09:38:16.044 fetched POST /api/observability/v1/traces 204
09:38:20.486 in Ping 
09:38:25.488 in Ping 
09:38:30.046 in Interrupt 32
09:38:30.492 in Ping 
09:38:31.047 fetch POST /api/observability/v1/traces
09:38:31.047 fetched POST /api/observability/v1/traces 204
09:38:35.495 in Ping 
09:38:40.046 rpc 33 server.reportClientActivity
09:38:40.501 in Ping 
09:38:41.046 fetch POST /api/observability/v1/traces
09:38:41.046 fetched POST /api/observability/v1/traces 204
09:38:45.503 in Ping 
09:38:50.507 in Ping 
09:38:55.047 in Interrupt 33
09:38:55.512 in Ping 
09:38:56.052 fetch POST /api/observability/v1/traces
09:38:56.052 fetched POST /api/observability/v1/traces 204
09:39:00.515 in Ping 
09:39:05.045 rpc 34 server.reportClientActivity
09:39:05.519 in Ping 
09:39:06.046 fetch POST /api/observability/v1/traces
09:39:06.046 fetched POST /api/observability/v1/traces 204
09:39:10.523 in Ping 
09:39:15.527 in Ping 
09:39:20.046 in Interrupt 34
09:39:20.531 in Ping 
09:39:21.048 fetch POST /api/observability/v1/traces
09:39:21.048 fetched POST /api/observability/v1/traces 204
09:39:25.535 in Ping 
09:39:30.048 rpc 35 server.reportClientActivity
09:39:30.539 in Ping 
09:39:31.048 fetch POST /api/observability/v1/traces
09:39:31.048 fetched POST /api/observability/v1/traces 204
09:39:35.543 in Ping 
09:39:40.547 in Ping 
09:39:45.050 in Interrupt 35
09:39:45.551 in Ping 
09:39:46.051 fetch POST /api/observability/v1/traces
09:39:46.051 fetched POST /api/observability/v1/traces 204
09:39:50.555 in Ping 
09:39:55.046 rpc 36 server.reportClientActivity
09:39:55.559 in Ping 
09:39:56.046 fetch POST /api/observability/v1/traces
09:39:56.047 fetched POST /api/observability/v1/traces 204
09:40:00.563 in Ping 
09:40:05.567 in Ping 
09:40:10.047 in Interrupt 36
09:40:10.571 in Ping 
09:40:11.053 fetch POST /api/observability/v1/traces
09:40:11.053 fetched POST /api/observability/v1/traces 204
09:40:15.575 in Ping 
09:40:20.050 rpc 37 server.reportClientActivity
09:40:20.579 in Ping 
09:40:21.052 fetch POST /api/observability/v1/traces
09:40:21.052 fetched POST /api/observability/v1/traces 204
09:40:25.583 in Ping 
09:40:30.588 in Ping 
09:40:35.052 in Interrupt 37
09:40:35.591 in Ping 
09:40:36.056 fetch POST /api/observability/v1/traces
09:40:36.057 fetched POST /api/observability/v1/traces 204
09:40:40.595 in Ping 
09:40:45.049 rpc 38 server.reportClientActivity
09:40:45.599 in Ping 
09:40:46.053 fetch POST /api/observability/v1/traces
09:40:46.053 fetched POST /api/observability/v1/traces 204
09:40:50.603 in Ping 
09:40:55.607 in Ping 
09:41:00.051 in Interrupt 38
09:41:00.611 in Ping 
09:41:01.056 fetch POST /api/observability/v1/traces
09:41:01.056 fetched POST /api/observability/v1/traces 204
09:41:05.615 in Ping 
09:41:10.054 rpc 39 server.reportClientActivity
09:41:10.619 in Ping 
09:41:11.056 fetch POST /api/observability/v1/traces
09:41:11.056 fetched POST /api/observability/v1/traces 204
09:41:15.623 in Ping 
09:41:20.627 in Ping 
09:41:25.056 in Interrupt 39
09:41:25.631 in Ping 
09:41:26.060 fetch POST /api/observability/v1/traces
09:41:26.061 fetched POST /api/observability/v1/traces 204
09:41:30.635 in Ping 
09:41:35.056 rpc 40 server.reportClientActivity
09:41:35.639 in Ping 
09:41:36.008 chunk 16 31214 bytes
09:41:36.012 in Ack 16
09:41:36.060 fetch POST /api/observability/v1/traces
09:41:36.060 fetched POST /api/observability/v1/traces 204
09:41:36.064 exit 28 {"_tag":"Success","value":{"sequence":138}}
09:41:36.064 exit 26 {"_tag":"Success","value":{"expiresAt":1794220894774,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
09:41:37.065 fetch POST /api/observability/v1/traces
09:41:37.065 fetched POST /api/observability/v1/traces 204
09:41:40.164 chunk 6 4678 bytes
09:41:40.165 in Ack 6
09:41:40.169 rpc 41 orchestration.dispatchCommand thread.visit
09:41:40.169 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:41:40.415 chunk 16 31850 bytes
09:41:40.417 in Ack 16
09:41:40.643 in Ping 
09:41:41.166 fetch POST /api/observability/v1/traces
09:41:41.166 fetched POST /api/observability/v1/traces 204
09:41:42.166 chunk 16 32690 bytes
09:41:42.168 in Ack 16
09:41:42.416 chunk 16 34711 bytes
09:41:42.422 in Ack 16
09:41:43.169 fetch POST /api/observability/v1/traces
09:41:43.169 fetched POST /api/observability/v1/traces 204
09:41:43.415 chunk 6 4678 bytes
09:41:43.416 in Ack 6
09:41:43.925 chunk 16 34711 bytes
09:41:43.926 in Ack 16
09:41:44.175 chunk 16 35366 bytes
09:41:44.177 in Ack 16
09:41:44.417 fetch POST /api/observability/v1/traces
09:41:44.417 fetched POST /api/observability/v1/traces 204
09:41:44.425 chunk 16 35368 bytes
09:41:44.427 in Ack 16
09:41:45.425 chunk 16 36679 bytes
09:41:45.427 in Ack 16
09:41:45.439 fetch POST /api/observability/v1/traces
09:41:45.439 fetched POST /api/observability/v1/traces 204
09:41:45.647 in Ping 
09:41:46.176 chunk 16 39316 bytes
09:41:46.179 in Ack 16
09:41:46.426 chunk 16 40092 bytes
09:41:46.430 in Ack 16
09:41:47.059 fetch POST /api/observability/v1/traces
09:41:47.059 fetched POST /api/observability/v1/traces 204
09:41:47.426 chunk 16 40044 bytes
09:41:47.427 in Ack 16
09:41:47.670 chunk 6 4678 bytes
09:41:47.670 in Ack 6
09:41:47.926 chunk 16 41866 bytes
09:41:47.928 in Ack 16
09:41:47.938 in Interrupt 20
09:41:47.943 rpc 42 subscribeVcsStatus
09:41:48.427 chunk 16 42639 bytes
09:41:48.429 in Ack 16
09:41:48.441 fetch POST /api/observability/v1/traces
09:41:48.441 fetched POST /api/observability/v1/traces 204
09:41:49.012 chunk 16 43205 bytes
09:41:49.017 in Ack 16
