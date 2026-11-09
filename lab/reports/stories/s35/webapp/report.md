# story, webapp, warm start, seed 35

Profile story (graded): seed 35's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 188 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 51.1s |
| Prompt reached the agent | 50.0s |
| Time to first text | 61.7s |
| Whole reply shown | 185.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 106.9s / 13.9s / 96.9s |
| Longest silence | 105.6s |
| Approval shown after it was asked | 92.9s |
| Bytes on the wire down / up | 231701 / 120650 |
| Bytes the app received / sent | 81399 / 91565 |

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
| 60 | 9.9 | 0 | 397 |
| 70 | 9.9 | 397 | 799 |
| 80 | 19.9 | 397 | 1077 |
| 90 | 29.9 | 397 | 1077 |
| 100 | 39.9 | 397 | 1077 |
| 110 | 49.9 | 397 | 1077 |
| 120 | 59.9 | 397 | 1077 |
| 130 | 69.9 | 397 | 1077 |
| 140 | 79.9 | 397 | 1077 |
| 150 | 89.9 | 397 | 1077 |
| 160 | 99.9 | 397 | 1077 |
| 170 | 0.0 | 1077 | 1077 |
| 180 | 9.1 | 1077 | 1429 |
| 185 | 6.4 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217069100,"silences":0,"longestSilenceMs":1313,"longestRecoveredGapMs":1313},"stats":{"sendRate":3375,"deliveredRate":3024.1506368607206,"loss":0,"srttMs":10.713087218623528,"minRttMs":null,"sentPackets":154,"sentWireBytes":42397,"receivedPackets":146,"receivedWireBytes":103799,"lastHeardMs":14860,"unsyncedBytes":1193,"inflightPackets":29,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217069100,"silences":0,"longestSilenceMs":1313,"longestRecoveredGapMs":1313},"stats":{"sendRate":125,"deliveredRate":3024.1506368607206,"loss":0,"srttMs":10.713087218623528,"minRttMs":null,"sentPackets":166,"sentWireBytes":48539,"receivedPackets":146,"receivedWireBytes":103799,"lastHeardMs":14860,"unsyncedBytes":1193,"inflightPackets":41,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217069100,"silences":0,"longestSilenceMs":1313,"longestRecoveredGapMs":1313},"stats":{"sendRate":50,"deliveredRate":3024.1506368607206,"loss":0,"srttMs":10.713087218623528,"minRttMs":null,"sentPackets":166,"sentWireBytes":48539,"receivedPackets":146,"receivedWireBytes":103799,"lastHeardMs":14860,"unsyncedBytes":1650,"inflightPackets":41,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217069100,"silences":0,"longestSilenceMs":1313,"longestRecoveredGapMs":1313},"stats":{"sendRate":50,"deliveredRate":3024.1506368607206,"loss":0,"srttMs":10.713087218623528,"minRttMs":null,"sentPackets":167,"sentWireBytes":49211,"receivedPackets":146,"receivedWireBytes":103799,"lastHeardMs":14860,"unsyncedBytes":1650,"inflightPackets":42,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217069100,"silences":0,"longestSilenceMs":1313,"longestRecoveredGapMs":1313},"stats":{"sendRate":50,"deliveredRate":3024.1506368607206,"loss":0,"srttMs":10.713087218623528,"minRttMs":null,"sentPackets":168,"sentWireBytes":49883,"receivedPackets":146,"receivedWireBytes":103799,"lastHeardMs":14860,"unsyncedBytes":2107,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 167/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217122296,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":2000,"deliveredRate":2133.119118650553,"loss":0.2357142857142857,"srttMs":202.12395131629557,"minRttMs":1542,"sentPackets":169,"sentWireBytes":50555,"receivedPackets":147,"receivedWireBytes":104256,"lastHeardMs":68055,"unsyncedBytes":1258,"inflightPackets":43,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"654/961","assistant":"32/32 21/21 344/498","runs":"1014/1014 971/971 616/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217132494,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":2531.7073170731705,"deliveredRate":1808.7495090390155,"loss":0,"srttMs":1583.0541777068174,"minRttMs":1421,"sentPackets":236,"sentWireBytes":65911,"receivedPackets":211,"receivedWireBytes":126565,"lastHeardMs":78254,"unsyncedBytes":0,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":1421,"sentPackets":266,"sentWireBytes":72371,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":167,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":1421,"sentPackets":282,"sentWireBytes":77605,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":624,"inflightPackets":79,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":292,"sentWireBytes":80888,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1081,"inflightPackets":89,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":293,"sentWireBytes":81218,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1081,"inflightPackets":90,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":295,"sentWireBytes":81878,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1539,"inflightPackets":92,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 6","open":true,"generation":2,"iceRestarts":6,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":296,"sentWireBytes":82208,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1539,"inflightPackets":93,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":298,"sentWireBytes":82868,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1539,"inflightPackets":95,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":299,"sentWireBytes":83198,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1997,"inflightPackets":96,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 7","open":true,"generation":2,"iceRestarts":7,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":301,"sentWireBytes":83858,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":1997,"inflightPackets":98,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"679/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 8","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217135431,"silences":1,"longestSilenceMs":53196,"longestRecoveredGapMs":53196},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":1583.0541777068174,"minRttMs":null,"sentPackets":302,"sentWireBytes":84188,"receivedPackets":220,"receivedWireBytes":130028,"lastHeardMs":81191,"unsyncedBytes":2455,"inflightPackets":99,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1139/1542","thread":"679/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 684/839","user":"10/10 9/9 57/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"435/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217243830,"silences":2,"longestSilenceMs":100367,"longestRecoveredGapMs":100367},"stats":{"sendRate":3000,"deliveredRate":1742.7157778999451,"loss":0,"srttMs":1541.7920202110622,"minRttMs":1432,"sentPackets":353,"sentWireBytes":98609,"receivedPackets":245,"receivedWireBytes":142265,"lastHeardMs":189589,"unsyncedBytes":398,"inflightPackets":108,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 1068/1068 947/947","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 256/983","messages":"544/544 456/456 901/901 544/544 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":8,"newConnections":0,"handshakeMs":1264,"lastDataAt":1794217254836,"silences":2,"longestSilenceMs":100367,"longestRecoveredGapMs":100367},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1597.6016433582906,"minRttMs":1432,"sentPackets":438,"sentWireBytes":119108,"receivedPackets":323,"receivedWireBytes":173122,"lastHeardMs":200595,"unsyncedBytes":134,"inflightPackets":107,"duplicateBytes":0}}
- shim trace
09:37:35.496 fetch GET /api/auth/session
09:37:35.498 fetched GET /api/auth/session 200
09:37:36.174 fetch GET /.well-known/t3/environment
09:37:36.175 fetched GET /.well-known/t3/environment 200
09:37:36.199 fetch GET /.well-known/t3/environment
09:37:36.200 fetched GET /.well-known/t3/environment 200
09:37:36.208 rpc 0 subscribeServerConfig
09:37:36.208 chunk 0 38462 bytes
09:37:36.292 in Ack 0
09:37:36.296 rpc 1 subscribeServerLifecycle
09:37:36.297 rpc 2 subscribeServerLifecycle
09:37:36.304 fetch GET /api/auth/session
09:37:36.306 fetched GET /api/auth/session 200
09:37:36.307 rpc 3 subscribeDeviceState
09:37:36.310 rpc 4 preview.reportProfiles
09:37:36.418 rpc 5 subscribeProjectClones
09:37:36.429 fetch GET /api/orchestration/threads/mcp%3Ade8037da-3dee-4993-8f79-ef5618578bd5/bounded
09:37:36.513 fetch GET /api/orchestration/shell
09:37:36.519 fetched GET /api/orchestration/shell 200
09:37:36.598 rpc 6 orchestration.subscribeShell
09:37:36.600 chunk 6 4594 bytes
09:37:36.788 rpc 7 server.refreshProviders
09:37:36.968 rpc 8 orchestration.dispatchCommand thread.visit
09:37:36.968 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:37.052 rpc 9 subscribeTerminalMetadata
09:37:37.053 rpc 10 subscribeVcsStatus
09:37:37.054 rpc 11 subscribePreviewEvents
09:37:37.056 rpc 12 assets.createUrl
09:37:37.057 rpc 13 preview.list
09:37:37.058 rpc 14 projects.readFile
09:37:37.066 in Ack 6
09:37:37.066 chunk 6 4570 bytes
09:37:37.091 in Ack 6
09:37:37.237 fetch POST /api/observability/v1/traces
09:37:37.237 fetched POST /api/observability/v1/traces 204
09:37:37.307 rpc 15 server.reportClientActivity
09:37:37.438 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"0841d54e-5b13-472f-8eb7-777cd4cee0ee","sessions":[]}}
09:37:37.610 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:37.846 chunk 1 3815 bytes
09:37:37.847 in Ack 1
09:37:38.076 chunk 2 3815 bytes
09:37:38.076 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:05.262Z","configurableProviders":false,"con
09:37:38.077 in Ack 2
09:37:38.249 chunk 10 1632 bytes
09:37:38.249 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:38.249 in Ack 10
09:37:38.294 fetch POST /api/observability/v1/traces
09:37:38.294 fetched POST /api/observability/v1/traces 204
09:37:38.521 chunk 6 4613 bytes
09:37:38.522 in Ack 6
09:37:38.772 chunk 6 4638 bytes
09:37:38.772 in Ack 6
09:37:39.022 chunk 6 4635 bytes
09:37:39.023 in Ack 6
09:37:39.382 chunk 3 876 bytes
09:37:39.382 exit 4 {"_tag":"Success","value":null}
09:37:39.382 chunk 5 4 bytes
09:37:39.382 chunk 9 36 bytes
09:37:39.382 exit 15 {"_tag":"Success","value":null}
09:37:39.383 in Ack 3
09:37:39.391 rpc 16 preview.reportProfiles
09:37:39.391 in Ack 5
09:37:39.392 in Ack 9
09:37:39.459 exit 16 {"_tag":"Success","value":null}
09:37:39.524 fetch POST /api/observability/v1/traces
09:37:39.524 fetched POST /api/observability/v1/traces 204
09:37:41.175 fetched GET /api/orchestration/threads/mcp%3Ade8037da-3dee-4993-8f79-ef5618578bd5/bounded 200
09:37:41.181 rpc 17 orchestration.subscribeThread
09:37:41.181 chunk 17 8282 bytes
09:37:41.203 in Interrupt 10
09:37:41.206 rpc 18 subscribeVcsStatus
09:37:41.208 in Ack 17
09:37:41.299 in Ping 
09:37:41.417 chunk 18 1632 bytes
09:37:41.417 in Ack 18
09:37:41.431 chunk 17 9336 bytes
09:37:41.432 in Ack 17
09:37:41.476 rpc 19 server.reportClientActivity
09:37:41.634 exit 19 {"_tag":"Success","value":null}
09:37:41.683 chunk 17 12690 bytes
09:37:41.687 in Ack 17
09:37:41.933 chunk 17 13497 bytes
09:37:41.935 in Ack 17
09:37:42.191 fetch POST /api/observability/v1/traces
09:37:42.191 fetched POST /api/observability/v1/traces 204
09:37:42.193 chunk 17 17976 bytes
09:37:42.195 in Ack 17
09:37:42.300 rpc 20 attachments.createUploadUrl
09:37:42.409 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e07fa329-97ac-47ac-833c-3b21a7fc4047","expiresAt":1794217662380,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:42.411 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWUwN2ZhMzI5LTk3YWMtNDdhYy04MzNjLTNiMjFhN2ZjNDA0NyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY2MjM4MH0.htjrSCkUGc4FZ9EIDN5tFqQAkcYIU5hH57f4YkhtXHE
09:37:42.444 chunk 17 19019 bytes
09:37:42.445 in Ack 17
09:37:42.694 chunk 17 21099 bytes
09:37:42.698 in Ack 17
09:37:42.715 in Interrupt 18
09:37:42.717 rpc 21 subscribeVcsStatus
09:37:42.818 chunk 21 1632 bytes
09:37:42.818 in Ack 21
09:37:42.996 rpc 22 server.reportClientActivity
09:37:43.240 fetch POST /api/observability/v1/traces
09:37:43.240 fetched POST /api/observability/v1/traces 204
09:37:44.421 exit 22 {"_tag":"Success","value":null}
09:37:45.634 rpc 23 orchestration.dispatchCommand message.dispatch
09:37:46.059 fetch POST /api/observability/v1/traces
09:37:46.059 fetched POST /api/observability/v1/traces 204
09:37:46.112 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:37:46.115 rpc 24 attachments.delete
09:37:46.231 exit 24 {"_tag":"Success","value":null}
09:37:46.299 in Ping 
09:37:46.405 chunk 6 4721 bytes
09:37:46.405 chunk 17 23892 bytes
09:37:46.406 in Ack 6
09:37:46.462 in Ack 17
09:37:46.466 rpc 25 assets.createUrl
09:37:46.655 chunk 17 26023 bytes
09:37:46.658 in Ack 17
09:37:46.906 chunk 17 29268 bytes
09:37:46.909 in Ack 17
09:37:46.964 rpc 26 orchestration.dispatchCommand thread.visit
09:37:46.964 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:47.114 fetch POST /api/observability/v1/traces
09:37:47.114 fetched POST /api/observability/v1/traces 204
09:37:47.155 chunk 6 4660 bytes
09:37:47.156 chunk 17 29268 bytes
09:37:47.156 in Ack 6
09:37:47.167 in Ack 17
09:37:47.405 chunk 6 4660 bytes
09:37:47.406 in Ack 6
09:37:47.421 chunk 17 29290 bytes
09:37:47.422 in Ack 17
09:37:47.676 chunk 17 31902 bytes
09:37:47.678 in Ack 17
09:37:47.925 chunk 17 31902 bytes
09:37:47.928 in Ack 17
09:37:48.159 fetch POST /api/observability/v1/traces
09:37:48.159 fetched POST /api/observability/v1/traces 204
09:37:48.427 chunk 17 32034 bytes
09:37:48.429 in Ack 17
09:37:48.952 chunk 6 4657 bytes
09:37:48.953 chunk 17 35259 bytes
09:37:48.957 in Ack 6
09:37:48.959 rpc 27 orchestration.dispatchCommand thread.visit
09:37:48.959 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:48.967 in Ack 17
09:37:48.981 in Interrupt 21
09:37:48.986 rpc 28 subscribeVcsStatus
09:37:48.987 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/5e34d497-6a65-4ea0-9611-0ee5a8089d12","expiresAt":1794220668987}}
09:37:49.103 chunk 28 1632 bytes
09:37:49.104 in Ack 28
09:37:49.203 chunk 17 35259 bytes
09:37:49.206 in Ack 17
09:37:49.254 rpc 29 server.reportClientActivity
09:37:49.463 rpc 30 server.refreshProviders
09:37:49.463 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:05.262Z","configurableProviders":false,"con
09:37:49.504 fetch POST /api/observability/v1/traces
09:37:49.504 fetched POST /api/observability/v1/traces 204
09:37:50.523 fetch POST /api/observability/v1/traces
09:37:50.523 fetched POST /api/observability/v1/traces 204
09:37:51.299 in Ping 
09:37:51.546 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:52.546 fetch POST /api/observability/v1/traces
09:37:52.546 fetched POST /api/observability/v1/traces 204
09:37:56.299 in Ping 
09:37:58.309 fetch POST /api/observability/v1/traces
09:37:58.309 fetched POST /api/observability/v1/traces 204
09:38:01.300 in Ping 
09:38:04.255 in Interrupt 29
09:38:04.507 rpc 32 server.reportClientActivity
09:38:05.256 fetch POST /api/observability/v1/traces
09:38:05.256 fetched POST /api/observability/v1/traces 204
09:38:06.300 in Ping 
09:38:11.300 in Ping 
09:38:16.301 in Ping 
09:38:19.508 in Interrupt 32
09:38:20.509 fetch POST /api/observability/v1/traces
09:38:20.509 fetched POST /api/observability/v1/traces 204
09:38:21.302 in Ping 
09:38:26.302 in Ping 
09:38:26.440 rpc 33 server.reportClientActivity
09:38:27.440 fetch POST /api/observability/v1/traces
09:38:27.440 fetched POST /api/observability/v1/traces 204
09:38:31.308 in Ping 
09:38:36.311 in Ping 
09:38:41.314 in Ping 
09:38:41.441 in Interrupt 33
09:38:42.297 exit 31 {"_tag":"Success","value":{"sequence":139}}
09:38:42.442 fetch POST /api/observability/v1/traces
09:38:42.442 fetched POST /api/observability/v1/traces 204
09:38:46.319 in Ping 
09:38:47.219 chunk 6 4721 bytes
09:38:47.221 in Ack 6
09:38:47.224 rpc 34 orchestration.dispatchCommand thread.visit
09:38:47.224 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:38:47.618 rpc 35 server.reportClientActivity
09:38:47.772 exit 29 {"_tag":"Success","value":null}
09:38:47.900 chunk 17 35895 bytes
09:38:47.901 in Ack 17
09:38:48.222 fetch POST /api/observability/v1/traces
09:38:48.222 fetched POST /api/observability/v1/traces 204
09:38:51.322 in Ping 
09:38:51.742 chunk 6 4721 bytes
09:38:51.743 in Ack 6
09:38:52.496 chunk 17 39002 bytes
09:38:52.498 in Ack 17
09:38:52.744 fetch POST /api/observability/v1/traces
09:38:52.744 fetched POST /api/observability/v1/traces 204
09:38:52.804 chunk 17 39308 bytes
09:38:52.806 in Ack 17
09:38:53.807 fetch POST /api/observability/v1/traces
09:38:53.807 fetched POST /api/observability/v1/traces 204
09:38:54.001 chunk 17 39260 bytes
09:38:54.003 in Ack 17
09:38:55.004 fetch POST /api/observability/v1/traces
09:38:55.004 fetched POST /api/observability/v1/traces 204
09:38:56.326 in Ping 
09:38:57.223 rpc 36 orchestration.dispatchCommand thread.visit
09:38:57.223 exit 36 {"_tag":"Success","value":{"sequence":0}}
09:38:58.224 fetch POST /api/observability/v1/traces
09:38:58.224 fetched POST /api/observability/v1/traces 204
09:39:01.330 in Ping 
09:39:02.619 in Interrupt 35
09:39:02.870 rpc 37 server.reportClientActivity
09:39:03.619 fetch POST /api/observability/v1/traces
09:39:03.619 fetched POST /api/observability/v1/traces 204
09:39:06.334 in Ping 
09:39:11.338 in Ping 
09:39:16.342 in Ping 
09:39:17.871 in Interrupt 37
09:39:18.122 rpc 38 server.reportClientActivity
09:39:18.871 fetch POST /api/observability/v1/traces
09:39:18.871 fetched POST /api/observability/v1/traces 204
09:39:21.346 in Ping 
09:39:26.350 in Ping 
09:39:31.354 in Ping 
09:39:33.122 in Interrupt 38
09:39:34.123 fetch POST /api/observability/v1/traces
09:39:34.123 fetched POST /api/observability/v1/traces 204
09:39:36.358 in Ping 
09:39:41.362 in Ping 
09:39:41.438 rpc 39 server.reportClientActivity
09:39:42.438 fetch POST /api/observability/v1/traces
09:39:42.438 fetched POST /api/observability/v1/traces 204
09:39:46.366 in Ping 
09:39:51.370 in Ping 
09:39:56.374 in Ping 
09:39:56.438 in Interrupt 39
09:39:57.441 fetch POST /api/observability/v1/traces
09:39:57.441 fetched POST /api/observability/v1/traces 204
09:40:01.378 in Ping 
09:40:06.382 in Ping 
09:40:06.438 rpc 40 server.reportClientActivity
09:40:07.438 fetch POST /api/observability/v1/traces
09:40:07.438 fetched POST /api/observability/v1/traces 204
09:40:11.386 in Ping 
09:40:16.391 in Ping 
09:40:21.394 in Ping 
09:40:21.438 in Interrupt 40
09:40:22.439 fetch POST /api/observability/v1/traces
09:40:22.439 fetched POST /api/observability/v1/traces 204
09:40:26.399 in Ping 
09:40:31.402 in Ping 
09:40:31.439 rpc 41 server.reportClientActivity
09:40:32.440 fetch POST /api/observability/v1/traces
09:40:32.440 fetched POST /api/observability/v1/traces 204
09:40:35.800 chunk 17 39462 bytes
09:40:35.804 in Ack 17
09:40:36.180 chunk 17 39710 bytes
09:40:36.182 in Ack 17
09:40:36.409 in Ping 
09:40:36.430 chunk 17 39970 bytes
09:40:36.432 in Ack 17
09:40:36.750 chunk 17 40216 bytes
09:40:36.752 in Ack 17
09:40:36.806 fetch POST /api/observability/v1/traces
09:40:36.806 fetched POST /api/observability/v1/traces 204
09:40:37.001 chunk 17 40406 bytes
09:40:37.003 in Ack 17
09:40:37.251 chunk 17 40644 bytes
09:40:37.253 in Ack 17
09:40:38.004 fetch POST /api/observability/v1/traces
09:40:38.004 fetched POST /api/observability/v1/traces 204
09:40:38.210 chunk 17 40898 bytes
09:40:38.212 in Ack 17
09:40:38.545 chunk 17 40948 bytes
09:40:38.547 in Ack 17
09:40:39.213 fetch POST /api/observability/v1/traces
09:40:39.213 fetched POST /api/observability/v1/traces 204
09:40:40.974 chunk 17 41950 bytes
09:40:40.975 in Ack 17
09:40:41.185 rpc 42 orchestration.dispatchCommand runtime-request.respond
09:40:41.410 in Ping 
09:40:41.976 fetch POST /api/observability/v1/traces
09:40:41.976 fetched POST /api/observability/v1/traces 204
09:40:43.049 exit 42 {"_tag":"Success","value":{"sequence":161}}
09:40:43.735 rpc 43 orchestration.dispatchCommand runtime-request.respond
09:40:44.050 fetch POST /api/observability/v1/traces
09:40:44.051 fetched POST /api/observability/v1/traces 204
09:40:45.007 chunk 17 41993 bytes
09:40:45.009 in Ack 17
09:40:45.737 chunk 17 42043 bytes
09:40:45.737 exit 43 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:40:45.739 in Ack 17
09:40:46.010 fetch POST /api/observability/v1/traces
09:40:46.010 fetched POST /api/observability/v1/traces 204
09:40:46.240 chunk 6 4723 bytes
09:40:46.241 in Ack 6
09:40:46.244 rpc 44 orchestration.dispatchCommand thread.visit
09:40:46.244 exit 44 {"_tag":"Success","value":{"sequence":0}}
09:40:46.414 in Ping 
09:40:46.440 in Interrupt 41
09:40:46.691 rpc 45 server.reportClientActivity
09:40:47.242 fetch POST /api/observability/v1/traces
09:40:47.242 fetched POST /api/observability/v1/traces 204
09:40:48.189 chunk 17 44904 bytes
09:40:48.191 in Ack 17
09:40:49.192 fetch POST /api/observability/v1/traces
09:40:49.192 fetched POST /api/observability/v1/traces 204
09:40:50.964 chunk 17 47883 bytes
09:40:50.966 in Ack 17
09:40:51.217 chunk 17 48139 bytes
09:40:51.219 in Ack 17
09:40:51.420 in Ping 
09:40:51.491 chunk 17 48091 bytes
09:40:51.493 in Ack 17
09:40:51.968 fetch POST /api/observability/v1/traces
09:40:51.968 fetched POST /api/observability/v1/traces 204
09:40:53.242 chunk 6 4723 bytes
09:40:53.243 in Ack 6
09:40:53.996 chunk 17 48091 bytes
09:40:53.997 in Ack 17
09:40:54.244 fetch POST /api/observability/v1/traces
09:40:54.245 fetched POST /api/observability/v1/traces 204
09:40:54.246 chunk 17 51627 bytes
09:40:54.249 in Ack 17
09:40:54.497 chunk 17 53744 bytes
09:40:54.500 in Ack 17
09:40:54.514 in Interrupt 28
09:40:54.571 rpc 46 subscribeVcsStatus
09:40:54.755 chunk 17 54699 bytes
09:40:54.758 in Ack 17
09:40:55.250 fetch POST /api/observability/v1/traces
09:40:55.250 fetched POST /api/observability/v1/traces 204
09:40:55.255 chunk 17 55683 bytes
09:40:55.260 in Ack 17
09:40:56.246 rpc 47 orchestration.dispatchCommand thread.visit
09:40:56.246 exit 47 {"_tag":"Success","value":{"sequence":0}}
09:40:56.261 fetch POST /api/observability/v1/traces
09:40:56.261 fetched POST /api/observability/v1/traces 204
09:40:56.277 chunk 17 57076 bytes
09:40:56.280 in Ack 17
09:40:56.406 chunk 46 1713 bytes
09:40:56.407 in Ack 46
09:40:56.423 in Ping 
09:40:56.528 chunk 17 58965 bytes
09:40:56.530 in Ack 17
09:40:57.027 chunk 17 59982 bytes
09:40:57.030 in Ack 17
09:40:57.847 fetch POST /api/observability/v1/traces
09:40:57.847 fetched POST /api/observability/v1/traces 204
