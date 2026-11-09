# story, webapp, warm start, seed 49

Profile story (graded): seed 49's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 181 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 44.2s |
| Prompt reached the agent | 35.5s |
| Time to first text | 124.7s |
| Whole reply shown | 178.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 88.4s / 9.3s / 83.4s |
| Longest silence | 124.7s |
| Approval shown after it was asked | 97.0s |
| Bytes on the wire down / up | 324212 / 130722 |
| Bytes the app received / sent | 73614 / 98789 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.0 | 0 | 0 |
| 40 | 4.4 | 0 | 181 |
| 50 | 14.4 | 0 | 581 |
| 60 | 24.4 | 0 | 984 |
| 70 | 34.4 | 0 | 1077 |
| 80 | 44.4 | 0 | 1077 |
| 90 | 54.4 | 0 | 1077 |
| 100 | 64.4 | 0 | 1077 |
| 110 | 74.4 | 0 | 1077 |
| 120 | 84.4 | 0 | 1077 |
| 130 | 84.4 | 397 | 1077 |
| 140 | 0.0 | 1077 | 1077 |
| 150 | 0.0 | 1077 | 1077 |
| 160 | 0.0 | 1077 | 1077 |
| 170 | 7.3 | 1077 | 1371 |
| 178 | 7.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216743804,"silences":0,"longestSilenceMs":1103,"longestRecoveredGapMs":1103},"stats":{"sendRate":6922.292165167666,"deliveredRate":3265.339227544562,"loss":0,"srttMs":35.34452086268893,"minRttMs":1,"sentPackets":97,"sentWireBytes":29530,"receivedPackets":176,"receivedWireBytes":104030,"lastHeardMs":9207,"unsyncedBytes":518,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216755625,"silences":2,"longestSilenceMs":8120,"longestRecoveredGapMs":8120},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":35.34452086268893,"minRttMs":null,"sentPackets":116,"sentWireBytes":37875,"receivedPackets":178,"receivedWireBytes":104272,"lastHeardMs":21029,"unsyncedBytes":518,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216760082,"silences":3,"longestSilenceMs":8120,"longestRecoveredGapMs":8120},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":35.34452086268893,"minRttMs":null,"sentPackets":116,"sentWireBytes":37875,"receivedPackets":179,"receivedWireBytes":104393,"lastHeardMs":25485,"unsyncedBytes":975,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216767114,"silences":4,"longestSilenceMs":8120,"longestRecoveredGapMs":8120},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":35.34452086268893,"minRttMs":null,"sentPackets":126,"sentWireBytes":43779,"receivedPackets":180,"receivedWireBytes":104515,"lastHeardMs":32517,"unsyncedBytes":975,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216767114,"silences":4,"longestSilenceMs":8120,"longestRecoveredGapMs":8120},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":35.34452086268893,"minRttMs":null,"sentPackets":136,"sentWireBytes":49691,"receivedPackets":180,"receivedWireBytes":104515,"lastHeardMs":32517,"unsyncedBytes":1432,"inflightPackets":39,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216793340,"silences":6,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":920.4264557548528,"minRttMs":7116,"sentPackets":145,"sentWireBytes":52606,"receivedPackets":182,"receivedWireBytes":105430,"lastHeardMs":58743,"unsyncedBytes":457,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 409/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216805547,"silences":9,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":153,"sentWireBytes":54881,"receivedPackets":187,"receivedWireBytes":107908,"lastHeardMs":70950,"unsyncedBytes":914,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 612/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216816336,"silences":10,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":153,"sentWireBytes":54881,"receivedPackets":192,"receivedWireBytes":110444,"lastHeardMs":81739,"unsyncedBytes":1371,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 821/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216821215,"silences":11,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":153,"sentWireBytes":54881,"receivedPackets":193,"receivedWireBytes":110946,"lastHeardMs":86619,"unsyncedBytes":1371,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 821/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216821215,"silences":11,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":177,"sentWireBytes":61731,"receivedPackets":193,"receivedWireBytes":110946,"lastHeardMs":86619,"unsyncedBytes":1828,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 821/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216821215,"silences":11,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":200,"sentWireBytes":68272,"receivedPackets":193,"receivedWireBytes":110946,"lastHeardMs":86619,"unsyncedBytes":1828,"inflightPackets":100,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 821/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216821215,"silences":11,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":213,"sentWireBytes":71979,"receivedPackets":193,"receivedWireBytes":110946,"lastHeardMs":86619,"unsyncedBytes":1828,"inflightPackets":113,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 821/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216821215,"silences":11,"longestSilenceMs":22617,"longestRecoveredGapMs":22617},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":2118.873148785496,"minRttMs":7116,"sentPackets":237,"sentWireBytes":78806,"receivedPackets":193,"receivedWireBytes":110946,"lastHeardMs":86619,"unsyncedBytes":2285,"inflightPackets":137,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1128/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"161/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216876202,"silences":13,"longestSilenceMs":49070,"longestRecoveredGapMs":49070},"stats":{"sendRate":2000,"deliveredRate":24.357861410406986,"loss":0.935483870967742,"srttMs":4009.6856519125918,"minRttMs":5162,"sentPackets":271,"sentWireBytes":88456,"receivedPackets":198,"receivedWireBytes":112721,"lastHeardMs":141605,"unsyncedBytes":2173,"inflightPackets":167,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849 171/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"836/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216886213,"silences":13,"longestSilenceMs":49070,"longestRecoveredGapMs":49070},"stats":{"sendRate":2000,"deliveredRate":23.53145993342399,"loss":0.9385596399099774,"srttMs":4636.806202245578,"minRttMs":5162,"sentPackets":288,"sentWireBytes":93249,"receivedPackets":211,"receivedWireBytes":116841,"lastHeardMs":151616,"unsyncedBytes":1871,"inflightPackets":182,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849 180/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"836/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216889350,"silences":13,"longestSilenceMs":49070,"longestRecoveredGapMs":49070},"stats":{"sendRate":2000,"deliveredRate":23.53145993342399,"loss":0.9385596399099774,"srttMs":4636.806202245578,"minRttMs":5162,"sentPackets":288,"sentWireBytes":93249,"receivedPackets":215,"receivedWireBytes":118370,"lastHeardMs":154753,"unsyncedBytes":2328,"inflightPackets":182,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 341/849 1194/1542 50/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216907671,"silences":14,"longestSilenceMs":49070,"longestRecoveredGapMs":49070},"stats":{"sendRate":2000,"deliveredRate":438.9559670110614,"loss":0.6665158451775105,"srttMs":3781.059138218427,"minRttMs":2364,"sentPackets":314,"sentWireBytes":102685,"receivedPackets":235,"receivedWireBytes":124682,"lastHeardMs":173074,"unsyncedBytes":954,"inflightPackets":192,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1355/1542 50/923 1093/1093 1031/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 53/839","user":"10/10 9/9","attempts":"659/659 659/659 132/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":462,"lastDataAt":1794216918564,"silences":14,"longestSilenceMs":49070,"longestRecoveredGapMs":49070},"stats":{"sendRate":2639.963846922461,"deliveredRate":1316.9397363322987,"loss":0.16003045442712022,"srttMs":2460.2051398972035,"minRttMs":2263,"sentPackets":367,"sentWireBytes":117978,"receivedPackets":305,"receivedWireBytes":153694,"lastHeardMs":183967,"unsyncedBytes":0,"inflightPackets":183,"duplicateBytes":0}}
- shim trace
09:32:15.111 fetch GET /api/auth/session
09:32:15.113 fetched GET /api/auth/session 200
09:32:15.423 fetch GET /.well-known/t3/environment
09:32:15.425 fetched GET /.well-known/t3/environment 200
09:32:15.454 fetch GET /.well-known/t3/environment
09:32:15.455 fetched GET /.well-known/t3/environment 200
09:32:15.466 rpc 0 subscribeServerConfig
09:32:15.466 chunk 0 38462 bytes
09:32:15.553 in Ack 0
09:32:15.555 rpc 1 subscribeServerLifecycle
09:32:15.556 rpc 2 subscribeServerLifecycle
09:32:15.559 fetch GET /api/auth/session
09:32:15.560 fetched GET /api/auth/session 200
09:32:15.561 rpc 3 subscribeDeviceState
09:32:15.601 rpc 4 preview.reportProfiles
09:32:15.659 rpc 5 subscribeProjectClones
09:32:15.686 fetch GET /api/orchestration/threads/mcp%3Aa7e8da12-594c-4ad6-95e6-0dadef871721/bounded
09:32:15.690 fetch GET /api/orchestration/shell
09:32:15.691 fetched GET /api/orchestration/shell 200
09:32:15.694 rpc 6 orchestration.subscribeShell
09:32:15.694 chunk 6 4616 bytes
09:32:15.719 rpc 7 server.refreshProviders
09:32:15.780 rpc 8 orchestration.dispatchCommand thread.visit
09:32:15.780 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:32:15.807 rpc 9 subscribeTerminalMetadata
09:32:15.808 rpc 10 subscribeVcsStatus
09:32:15.809 rpc 11 subscribePreviewEvents
09:32:15.810 rpc 12 assets.createUrl
09:32:15.811 rpc 13 preview.list
09:32:15.811 rpc 14 projects.readFile
09:32:15.820 in Ack 6
09:32:15.932 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"0095a0a0-9d93-4e0f-8ee4-9f89e191f973","sessions":[]}}
09:32:15.932 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:32:15.944 chunk 6 4592 bytes
09:32:15.945 in Ack 6
09:32:16.058 chunk 1 3815 bytes
09:32:16.059 in Ack 1
09:32:16.061 rpc 15 server.reportClientActivity
09:32:16.196 chunk 2 3815 bytes
09:32:16.205 in Ack 2
09:32:16.319 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:31:54.865Z","configurableProviders":false,"con
09:32:16.319 chunk 10 1632 bytes
09:32:16.319 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:32:16.324 in Ack 10
09:32:16.396 fetch POST /api/observability/v1/traces
09:32:16.397 fetched POST /api/observability/v1/traces 204
09:32:16.692 chunk 6 4635 bytes
09:32:16.692 in Ack 6
09:32:16.716 chunk 3 876 bytes
09:32:16.716 exit 4 {"_tag":"Success","value":null}
09:32:16.716 chunk 5 4 bytes
09:32:16.716 chunk 9 36 bytes
09:32:16.716 exit 15 {"_tag":"Success","value":null}
09:32:16.717 in Ack 3
09:32:16.722 rpc 16 preview.reportProfiles
09:32:16.722 in Ack 5
09:32:16.722 in Ack 9
09:32:16.782 exit 16 {"_tag":"Success","value":null}
09:32:17.497 fetched GET /api/orchestration/threads/mcp%3Aa7e8da12-594c-4ad6-95e6-0dadef871721/bounded 200
09:32:17.501 rpc 17 orchestration.subscribeThread
09:32:17.501 chunk 17 8283 bytes
09:32:17.512 in Interrupt 10
09:32:17.514 rpc 18 subscribeVcsStatus
09:32:17.515 in Ack 17
09:32:17.628 chunk 18 1632 bytes
09:32:17.628 in Ack 18
09:32:17.694 fetch POST /api/observability/v1/traces
09:32:17.694 fetched POST /api/observability/v1/traces 204
09:32:17.752 chunk 17 11260 bytes
09:32:17.753 in Ack 17
09:32:17.800 rpc 19 server.reportClientActivity
09:32:17.950 exit 19 {"_tag":"Success","value":null}
09:32:18.001 chunk 17 13498 bytes
09:32:18.006 in Ack 17
09:32:18.232 rpc 20 attachments.createUploadUrl
09:32:18.292 chunk 17 18553 bytes
09:32:18.301 in Ack 17
09:32:18.384 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-4f7ddb99-86bd-4de6-b948-4b2c5aadb7af","expiresAt":1794217338318,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:32:18.385 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTRmN2RkYjk5LTg2YmQtNGRlNi1iOTQ4LTRiMmM1YWFkYjdhZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzMzODMxOH0.m7e0xuTxhE5DYw4L8ZpAfs3UexmsGkFjJWKgc_BIuY0
09:32:18.542 chunk 17 21100 bytes
09:32:18.544 in Ack 17
09:32:18.554 in Interrupt 18
09:32:18.556 rpc 21 subscribeVcsStatus
09:32:18.665 chunk 21 1632 bytes
09:32:18.665 in Ack 21
09:32:18.754 fetch POST /api/observability/v1/traces
09:32:18.754 fetched POST /api/observability/v1/traces 204
09:32:18.818 rpc 22 server.reportClientActivity
09:32:19.817 fetch POST /api/observability/v1/traces
09:32:19.817 fetched POST /api/observability/v1/traces 204
09:32:20.097 exit 22 {"_tag":"Success","value":null}
09:32:20.460 in Ping 
09:32:21.170 rpc 23 orchestration.dispatchCommand message.dispatch
09:32:21.376 fetch POST /api/observability/v1/traces
09:32:21.376 fetched POST /api/observability/v1/traces 204
09:32:21.404 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:32:21.408 rpc 24 attachments.delete
09:32:21.528 exit 24 {"_tag":"Success","value":null}
09:32:21.666 chunk 6 4721 bytes
09:32:21.666 chunk 17 26024 bytes
09:32:21.667 in Ack 6
09:32:21.732 in Ack 17
09:32:21.735 rpc 25 assets.createUrl
09:32:21.920 chunk 17 27723 bytes
09:32:21.931 in Ack 17
09:32:22.169 chunk 17 30180 bytes
09:32:22.174 in Ack 17
09:32:22.405 fetch POST /api/observability/v1/traces
09:32:22.405 fetched POST /api/observability/v1/traces 204
09:32:22.419 chunk 6 4660 bytes
09:32:22.420 chunk 17 30320 bytes
09:32:22.425 in Ack 6
09:32:22.446 in Ack 17
09:32:22.670 chunk 17 31903 bytes
09:32:22.672 in Ack 17
09:32:22.722 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/996934f7-9172-4fc8-a947-c6bee03a324e","expiresAt":1794220342722}}
09:32:23.172 chunk 6 4657 bytes
09:32:23.173 chunk 17 35260 bytes
09:32:23.173 in Ack 6
09:32:23.176 rpc 26 orchestration.dispatchCommand thread.visit
09:32:23.176 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:32:23.185 in Ack 17
09:32:23.201 in Interrupt 21
09:32:23.232 rpc 27 subscribeVcsStatus
09:32:23.299 chunk 27 1713 bytes
09:32:23.299 in Ack 27
09:32:23.424 chunk 6 4657 bytes
09:32:23.424 chunk 17 35260 bytes
09:32:23.425 in Ack 6
09:32:23.438 in Ack 17
09:32:23.450 fetch POST /api/observability/v1/traces
09:32:23.452 fetched POST /api/observability/v1/traces 204
09:32:23.483 rpc 28 server.reportClientActivity
09:32:23.587 exit 28 {"_tag":"Success","value":null}
09:32:24.504 fetch POST /api/observability/v1/traces
09:32:24.505 fetched POST /api/observability/v1/traces 204
09:32:25.465 in Ping 
09:32:25.794 rpc 29 orchestration.dispatchCommand message.dispatch
09:32:26.796 fetch POST /api/observability/v1/traces
09:32:26.796 fetched POST /api/observability/v1/traces 204
09:32:29.030 fetch POST /api/observability/v1/traces
09:32:29.030 fetched POST /api/observability/v1/traces 204
09:32:30.467 in Ping 
09:32:35.467 in Ping 
09:32:40.467 in Ping 
09:32:40.683 rpc 30 server.reportClientActivity
09:32:41.685 fetch POST /api/observability/v1/traces
09:32:41.685 fetched POST /api/observability/v1/traces 204
09:32:45.467 in Ping 
09:32:50.467 in Ping 
09:32:55.468 in Ping 
09:32:55.688 in Interrupt 30
09:32:56.689 fetch POST /api/observability/v1/traces
09:32:56.689 fetched POST /api/observability/v1/traces 204
09:33:00.468 in Ping 
09:33:05.468 in Ping 
09:33:05.683 rpc 31 server.reportClientActivity
09:33:06.683 fetch POST /api/observability/v1/traces
09:33:06.683 fetched POST /api/observability/v1/traces 204
09:33:09.734 exit 29 {"_tag":"Success","value":{"sequence":138}}
09:33:10.470 in Ping 
09:33:10.736 fetch POST /api/observability/v1/traces
09:33:10.736 fetched POST /api/observability/v1/traces 204
09:33:15.474 in Ping 
09:33:17.982 chunk 17 38069 bytes
09:33:17.985 in Ack 17
09:33:18.987 fetch POST /api/observability/v1/traces
09:33:18.987 fetched POST /api/observability/v1/traces 204
09:33:20.479 in Ping 
09:33:20.684 in Interrupt 31
09:33:20.936 rpc 32 server.reportClientActivity
09:33:21.685 fetch POST /api/observability/v1/traces
09:33:21.685 fetched POST /api/observability/v1/traces 204
09:33:23.729 chunk 17 38497 bytes
09:33:23.731 in Ack 17
09:33:24.732 fetch POST /api/observability/v1/traces
09:33:24.732 fetched POST /api/observability/v1/traces 204
09:33:25.482 in Ping 
09:33:29.083 fetch POST /api/observability/v1/traces
09:33:29.083 fetched POST /api/observability/v1/traces 204
09:33:30.486 in Ping 
09:33:34.788 chunk 17 38863 bytes
09:33:34.790 in Ack 17
09:33:35.493 in Ping 
09:33:35.792 fetch POST /api/observability/v1/traces
09:33:35.792 fetched POST /api/observability/v1/traces 204
09:33:35.936 in Interrupt 32
09:33:36.191 rpc 33 server.reportClientActivity
09:33:36.337 chunk 17 38909 bytes
09:33:36.340 in Ack 17
09:33:36.937 fetch POST /api/observability/v1/traces
09:33:36.938 fetched POST /api/observability/v1/traces 204
09:33:39.090 fetch POST /api/observability/v1/traces
09:33:39.090 fetched POST /api/observability/v1/traces 204
09:33:40.496 in Ping 
09:33:41.217 chunk 17 39327 bytes
09:33:41.220 in Ack 17
09:33:42.221 fetch POST /api/observability/v1/traces
09:33:42.221 fetched POST /api/observability/v1/traces 204
09:33:45.498 in Ping 
09:33:49.114 fetch POST /api/observability/v1/traces
09:33:49.114 fetched POST /api/observability/v1/traces 204
09:33:50.502 in Ping 
09:33:51.194 in Interrupt 33
09:33:52.195 fetch POST /api/observability/v1/traces
09:33:52.195 fetched POST /api/observability/v1/traces 204
09:33:55.506 in Ping 
09:33:55.683 rpc 34 server.reportClientActivity
09:33:56.684 fetch POST /api/observability/v1/traces
09:33:56.684 fetched POST /api/observability/v1/traces 204
09:34:00.510 in Ping 
09:34:05.515 in Ping 
09:34:10.520 in Ping 
09:34:10.684 in Interrupt 34
09:34:11.686 fetch POST /api/observability/v1/traces
09:34:11.686 fetched POST /api/observability/v1/traces 204
09:34:15.524 in Ping 
09:34:20.526 in Ping 
09:34:20.685 rpc 35 server.reportClientActivity
09:34:21.685 fetch POST /api/observability/v1/traces
09:34:21.685 fetched POST /api/observability/v1/traces 204
09:34:25.530 in Ping 
09:34:30.287 chunk 17 39503 bytes
09:34:30.291 in Ack 17
09:34:30.534 in Ping 
09:34:31.292 fetch POST /api/observability/v1/traces
09:34:31.292 fetched POST /api/observability/v1/traces 204
09:34:31.323 chunk 17 39939 bytes
09:34:31.324 in Ack 17
09:34:32.327 fetch POST /api/observability/v1/traces
09:34:32.327 fetched POST /api/observability/v1/traces 204
09:34:35.540 in Ping 
09:34:35.686 in Interrupt 35
09:34:35.939 rpc 36 server.reportClientActivity
09:34:36.687 fetch POST /api/observability/v1/traces
09:34:36.687 fetched POST /api/observability/v1/traces 204
09:34:38.649 chunk 17 40361 bytes
09:34:38.650 in Ack 17
09:34:39.652 fetch POST /api/observability/v1/traces
09:34:39.652 fetched POST /api/observability/v1/traces 204
09:34:40.543 in Ping 
09:34:41.330 fetch POST /api/observability/v1/traces
09:34:41.330 fetched POST /api/observability/v1/traces 204
09:34:45.546 in Ping 
09:34:50.552 in Ping 
09:34:50.940 in Interrupt 36
09:34:51.191 rpc 37 server.reportClientActivity
09:34:51.941 fetch POST /api/observability/v1/traces
09:34:51.941 fetched POST /api/observability/v1/traces 204
09:34:55.554 in Ping 
09:35:00.559 in Ping 
09:35:04.796 chunk 17 41363 bytes
09:35:04.798 in Ack 17
09:35:05.094 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:35:05.562 in Ping 
09:35:05.799 fetch POST /api/observability/v1/traces
09:35:05.799 fetched POST /api/observability/v1/traces 204
09:35:06.194 in Interrupt 37
09:35:07.199 fetch POST /api/observability/v1/traces
09:35:07.199 fetched POST /api/observability/v1/traces 204
09:35:09.623 exit 38 {"_tag":"Success","value":{"sequence":158}}
09:35:10.024 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:35:10.566 in Ping 
09:35:10.624 fetch POST /api/observability/v1/traces
09:35:10.624 fetched POST /api/observability/v1/traces 204
09:35:10.684 rpc 40 server.reportClientActivity
09:35:11.684 fetch POST /api/observability/v1/traces
09:35:11.684 fetched POST /api/observability/v1/traces 204
09:35:12.729 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:35:12.771 chunk 17 41406 bytes
09:35:12.773 in Ack 17
09:35:13.465 chunk 17 41408 bytes
09:35:13.467 in Ack 17
09:35:13.734 fetch POST /api/observability/v1/traces
09:35:13.734 fetched POST /api/observability/v1/traces 204
09:35:15.570 in Ping 
09:35:15.972 fetch POST /api/observability/v1/traces
09:35:15.972 fetched POST /api/observability/v1/traces 204
09:35:17.511 chunk 17 44401 bytes
09:35:17.512 in Ack 17
09:35:17.763 chunk 17 44643 bytes
09:35:17.767 in Ack 17
09:35:18.513 fetch POST /api/observability/v1/traces
09:35:18.513 fetched POST /api/observability/v1/traces 204
09:35:18.760 chunk 17 44595 bytes
09:35:18.762 in Ack 17
09:35:19.762 fetch POST /api/observability/v1/traces
09:35:19.762 fetched POST /api/observability/v1/traces 204
09:35:20.574 in Ping 
09:35:21.276 chunk 6 4723 bytes
09:35:21.277 in Ack 6
09:35:21.279 rpc 41 orchestration.dispatchCommand thread.visit
09:35:21.279 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:35:21.527 chunk 17 45231 bytes
09:35:21.529 in Ack 17
09:35:21.739 exit 30 {"_tag":"Success","value":null}
09:35:21.739 exit 31 {"_tag":"Success","value":null}
09:35:21.777 chunk 17 48092 bytes
09:35:21.778 in Ack 17
09:35:22.278 fetch POST /api/observability/v1/traces
09:35:22.278 fetched POST /api/observability/v1/traces 204
09:35:22.529 chunk 17 51628 bytes
09:35:22.530 in Ack 17
09:35:22.952 exit 32 {"_tag":"Success","value":null}
09:35:23.029 chunk 17 53745 bytes
09:35:23.031 in Ack 17
09:35:23.043 in Interrupt 27
09:35:23.072 rpc 42 subscribeVcsStatus
09:35:23.176 exit 33 {"_tag":"Success","value":null}
09:35:23.281 chunk 17 54700 bytes
09:35:23.283 in Ack 17
09:35:23.531 fetch POST /api/observability/v1/traces
09:35:23.531 fetched POST /api/observability/v1/traces 204
09:35:23.782 chunk 17 55717 bytes
09:35:23.784 in Ack 17
09:35:24.227 exit 34 {"_tag":"Success","value":null}
09:35:24.296 chunk 17 55767 bytes
09:35:24.298 in Ack 17
09:35:24.792 fetch POST /api/observability/v1/traces
09:35:24.792 fetched POST /api/observability/v1/traces 204
