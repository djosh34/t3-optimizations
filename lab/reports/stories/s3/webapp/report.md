# story, webapp, warm start, seed 3

Profile story (graded): seed 3's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 182 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 93.2s |
| Prompt reached the agent | 90.8s |
| Time to first text | 162.7s |
| Whole reply shown | 180.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 71.1s / 0.0s / 62.1s |
| Longest silence | 162.7s |
| Approval shown after it was asked | 46.5s |
| Bytes on the wire down / up | 287021 / 111661 |
| Bytes the app received / sent | 114308 / 81564 |

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
| 100 | 9.1 | 0 | 365 |
| 110 | 19.1 | 0 | 766 |
| 120 | 29.1 | 0 | 1077 |
| 130 | 39.1 | 0 | 1077 |
| 140 | 49.1 | 0 | 1077 |
| 150 | 59.1 | 0 | 1077 |
| 160 | 69.1 | 0 | 1077 |
| 170 | 5.1 | 1077 | 1281 |
| 180 | 7.4 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216509141,"silences":0,"longestSilenceMs":1002,"longestRecoveredGapMs":1002},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":3.2192230984634165,"minRttMs":1,"sentPackets":100,"sentWireBytes":31097,"receivedPackets":165,"receivedWireBytes":98344,"lastHeardMs":9971,"unsyncedBytes":518,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216509141,"silences":0,"longestSilenceMs":1002,"longestRecoveredGapMs":1002},"stats":{"sendRate":50,"deliveredRate":0,"loss":0,"srttMs":3.2192230984634165,"minRttMs":null,"sentPackets":108,"sentWireBytes":34634,"receivedPackets":165,"receivedWireBytes":98344,"lastHeardMs":9971,"unsyncedBytes":518,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216525441,"silences":1,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":3.2192230984634165,"minRttMs":null,"sentPackets":119,"sentWireBytes":41010,"receivedPackets":166,"receivedWireBytes":98465,"lastHeardMs":26272,"unsyncedBytes":974,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216538162,"silences":2,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":3.2192230984634165,"minRttMs":null,"sentPackets":125,"sentWireBytes":44644,"receivedPackets":167,"receivedWireBytes":98587,"lastHeardMs":38992,"unsyncedBytes":974,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216549203,"silences":4,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":137,"sentWireBytes":51682,"receivedPackets":169,"receivedWireBytes":98832,"lastHeardMs":50033,"unsyncedBytes":1240,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216560236,"silences":5,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":137,"sentWireBytes":51682,"receivedPackets":172,"receivedWireBytes":99202,"lastHeardMs":61066,"unsyncedBytes":1240,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216570768,"silences":8,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":141,"sentWireBytes":54134,"receivedPackets":175,"receivedWireBytes":99574,"lastHeardMs":71599,"unsyncedBytes":1240,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216573766,"silences":9,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":151,"sentWireBytes":60278,"receivedPackets":176,"receivedWireBytes":99698,"lastHeardMs":74596,"unsyncedBytes":1697,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216588810,"silences":10,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":160,"sentWireBytes":65801,"receivedPackets":177,"receivedWireBytes":99822,"lastHeardMs":89640,"unsyncedBytes":1697,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216595780,"silences":11,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":743.0668202111555,"minRttMs":5922,"sentPackets":166,"sentWireBytes":69504,"receivedPackets":178,"receivedWireBytes":99946,"lastHeardMs":96611,"unsyncedBytes":2154,"inflightPackets":68,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216612325,"silences":15,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":12.501081408426336,"loss":0.96,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":172,"sentWireBytes":72871,"receivedPackets":185,"receivedWireBytes":102553,"lastHeardMs":113155,"unsyncedBytes":1241,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 198/498","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216621120,"silences":18,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":2000,"deliveredRate":14.688823863396708,"loss":0.9573658536585365,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":172,"sentWireBytes":72871,"receivedPackets":190,"receivedWireBytes":105017,"lastHeardMs":121950,"unsyncedBytes":1697,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 198/498","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216632013,"silences":20,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":1000,"deliveredRate":14.688823863396708,"loss":0.9573658536585365,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":181,"sentWireBytes":75416,"receivedPackets":192,"receivedWireBytes":106021,"lastHeardMs":132843,"unsyncedBytes":2153,"inflightPackets":81,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 386/1340","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216637771,"silences":21,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":1000,"deliveredRate":14.688823863396708,"loss":0.9573658536585365,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":192,"sentWireBytes":78529,"receivedPackets":195,"receivedWireBytes":107533,"lastHeardMs":138601,"unsyncedBytes":2153,"inflightPackets":92,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 386/1340","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216637771,"silences":21,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":1000,"deliveredRate":14.688823863396708,"loss":0.9573658536585365,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":192,"sentWireBytes":78529,"receivedPackets":195,"receivedWireBytes":107533,"lastHeardMs":138601,"unsyncedBytes":2609,"inflightPackets":92,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 386/1340","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216637771,"silences":21,"longestSilenceMs":16300,"longestRecoveredGapMs":16300},"stats":{"sendRate":1000,"deliveredRate":14.688823863396708,"loss":0.9573658536585365,"srttMs":2109.558467684761,"minRttMs":5922,"sentPackets":204,"sentWireBytes":81921,"receivedPackets":195,"receivedWireBytes":107533,"lastHeardMs":138601,"unsyncedBytes":2609,"inflightPackets":104,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 188/849","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"805/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216673632,"silences":22,"longestSilenceMs":34799,"longestRecoveredGapMs":34799},"stats":{"sendRate":1000,"deliveredRate":17.09361839575816,"loss":0.9413099437148216,"srttMs":2169.488659224166,"minRttMs":2589,"sentPackets":225,"sentWireBytes":87843,"receivedPackets":204,"receivedWireBytes":111519,"lastHeardMs":174463,"unsyncedBytes":1815,"inflightPackets":120,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1566/1566 1093/1093","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":965,"lastDataAt":1794216684083,"silences":22,"longestSilenceMs":34799,"longestRecoveredGapMs":34799},"stats":{"sendRate":2241.6933999983085,"deliveredRate":2248.315688161694,"loss":0,"srttMs":1165.9767342604114,"minRttMs":1023,"sentPackets":272,"sentWireBytes":99269,"receivedPackets":308,"receivedWireBytes":154953,"lastHeardMs":184913,"unsyncedBytes":0,"inflightPackets":110,"duplicateBytes":0}}
- shim trace
09:28:19.872 fetch GET /api/auth/session
09:28:19.873 fetched GET /api/auth/session 200
09:28:20.286 fetch GET /.well-known/t3/environment
09:28:20.287 fetched GET /.well-known/t3/environment 200
09:28:20.308 fetch GET /.well-known/t3/environment
09:28:20.308 fetched GET /.well-known/t3/environment 200
09:28:20.316 rpc 0 subscribeServerConfig
09:28:20.316 chunk 0 38458 bytes
09:28:20.375 in Ack 0
09:28:20.378 rpc 1 subscribeServerLifecycle
09:28:20.380 rpc 2 subscribeServerLifecycle
09:28:20.386 fetch GET /api/auth/session
09:28:20.387 fetched GET /api/auth/session 200
09:28:20.387 rpc 3 subscribeDeviceState
09:28:20.392 rpc 4 preview.reportProfiles
09:28:20.496 rpc 5 subscribeProjectClones
09:28:20.523 fetch GET /api/orchestration/threads/mcp%3A32a35bc5-561a-4bd3-b863-1b12181a8675/bounded
09:28:20.529 fetch GET /api/orchestration/shell
09:28:20.531 fetched GET /api/orchestration/shell 200
09:28:20.533 rpc 6 orchestration.subscribeShell
09:28:20.533 chunk 6 4618 bytes
09:28:20.560 rpc 7 server.refreshProviders
09:28:20.612 rpc 8 orchestration.dispatchCommand thread.visit
09:28:20.612 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:28:20.632 rpc 9 subscribeTerminalMetadata
09:28:20.633 rpc 10 subscribeVcsStatus
09:28:20.634 rpc 11 subscribePreviewEvents
09:28:20.635 rpc 12 assets.createUrl
09:28:20.637 rpc 13 preview.list
09:28:20.642 rpc 14 projects.readFile
09:28:20.659 in Ack 6
09:28:20.783 chunk 6 4594 bytes
09:28:20.784 in Ack 6
09:28:20.886 rpc 15 server.reportClientActivity
09:28:21.278 fetch POST /api/observability/v1/traces
09:28:21.278 fetched POST /api/observability/v1/traces 204
09:28:21.294 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"d559befd-7c02-47cb-938b-8c490fe9432d","sessions":[]}}
09:28:21.294 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:21.294 chunk 1 3814 bytes
09:28:21.300 in Ack 1
09:28:21.547 chunk 2 3814 bytes
09:28:21.547 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:59.667Z","configurableProviders":false,"con
09:28:21.547 chunk 10 1632 bytes
09:28:21.547 in Ack 2
09:28:21.551 in Ack 10
09:28:21.671 exit 12 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:21.921 chunk 6 4634 bytes
09:28:21.922 in Ack 6
09:28:22.065 chunk 3 876 bytes
09:28:22.065 exit 4 {"_tag":"Success","value":null}
09:28:22.065 chunk 5 4 bytes
09:28:22.065 chunk 9 36 bytes
09:28:22.065 exit 15 {"_tag":"Success","value":null}
09:28:22.065 in Ack 3
09:28:22.070 rpc 16 preview.reportProfiles
09:28:22.070 in Ack 5
09:28:22.071 in Ack 9
09:28:22.130 exit 16 {"_tag":"Success","value":null}
09:28:22.295 fetch POST /api/observability/v1/traces
09:28:22.295 fetched POST /api/observability/v1/traces 204
09:28:22.842 fetched GET /api/orchestration/threads/mcp%3A32a35bc5-561a-4bd3-b863-1b12181a8675/bounded 200
09:28:22.846 rpc 17 orchestration.subscribeThread
09:28:22.846 chunk 17 8283 bytes
09:28:22.863 in Interrupt 10
09:28:22.865 rpc 18 subscribeVcsStatus
09:28:22.866 in Ack 17
09:28:23.043 chunk 18 1632 bytes
09:28:23.043 in Ack 18
09:28:23.097 chunk 17 11260 bytes
09:28:23.098 in Ack 17
09:28:23.149 rpc 19 server.reportClientActivity
09:28:23.346 chunk 17 13498 bytes
09:28:23.348 in Ack 17
09:28:23.479 exit 19 {"_tag":"Success","value":null}
09:28:23.641 chunk 17 19019 bytes
09:28:23.644 in Ack 17
09:28:23.855 fetch POST /api/observability/v1/traces
09:28:23.855 fetched POST /api/observability/v1/traces 204
09:28:23.924 rpc 20 attachments.createUploadUrl
09:28:23.969 chunk 17 21098 bytes
09:28:23.983 in Ack 17
09:28:23.998 in Interrupt 18
09:28:23.999 rpc 21 subscribeVcsStatus
09:28:24.070 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e4ff4da9-daca-4cd7-b3d2-51eb80cd36d6","expiresAt":1794217104061,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:24.070 chunk 21 1632 bytes
09:28:24.071 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWU0ZmY0ZGE5LWRhY2EtNGNkNy1iM2QyLTUxZWI4MGNkMzZkNiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzEwNDA2MX0.UBbvDAXVaU1NNFGQeu7YMV0JRIxK2kYqVfyGxbsV2Kk
09:28:24.073 in Ack 21
09:28:24.251 rpc 22 server.reportClientActivity
09:28:24.924 fetch POST /api/observability/v1/traces
09:28:24.924 fetched POST /api/observability/v1/traces 204
09:28:25.310 in Ping 
09:28:25.914 exit 22 {"_tag":"Success","value":null}
09:28:26.919 rpc 23 orchestration.dispatchCommand message.dispatch
09:28:27.096 fetch POST /api/observability/v1/traces
09:28:27.096 fetched POST /api/observability/v1/traces 204
09:28:27.130 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:28:27.132 rpc 24 attachments.delete
09:28:27.140 chunk 17 23891 bytes
09:28:27.142 in Ack 17
09:28:27.145 rpc 25 assets.createUrl
09:28:27.246 exit 24 {"_tag":"Success","value":null}
09:28:27.390 chunk 6 4720 bytes
09:28:27.390 chunk 17 25367 bytes
09:28:27.391 in Ack 6
09:28:27.415 in Ack 17
09:28:27.646 chunk 17 27721 bytes
09:28:27.648 in Ack 17
09:28:27.896 chunk 6 4659 bytes
09:28:27.897 chunk 17 30178 bytes
09:28:27.897 in Ack 6
09:28:27.907 in Ack 17
09:28:28.132 fetch POST /api/observability/v1/traces
09:28:28.132 fetched POST /api/observability/v1/traces 204
09:28:28.148 chunk 17 31901 bytes
09:28:28.150 in Ack 17
09:28:28.336 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/957956d2-76e7-450d-a06e-fff8b995a019","expiresAt":1794220108336}}
09:28:28.648 chunk 6 4656 bytes
09:28:28.649 chunk 17 35258 bytes
09:28:28.649 in Ack 6
09:28:28.651 rpc 26 orchestration.dispatchCommand thread.visit
09:28:28.651 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:28:28.659 in Ack 17
09:28:28.670 in Interrupt 21
09:28:28.696 rpc 27 subscribeVcsStatus
09:28:28.752 chunk 27 1713 bytes
09:28:28.752 in Ack 27
09:28:28.898 chunk 6 4656 bytes
09:28:28.898 in Ack 6
09:28:28.906 chunk 17 35258 bytes
09:28:28.909 in Ack 17
09:28:28.946 rpc 28 server.reportClientActivity
09:28:28.981 exit 28 {"_tag":"Success","value":null}
09:28:29.151 fetch POST /api/observability/v1/traces
09:28:29.151 fetched POST /api/observability/v1/traces 204
09:28:30.311 in Ping 
09:28:31.113 rpc 29 orchestration.dispatchCommand message.dispatch
09:28:32.113 fetch POST /api/observability/v1/traces
09:28:32.114 fetched POST /api/observability/v1/traces 204
09:28:34.372 fetch POST /api/observability/v1/traces
09:28:34.372 fetched POST /api/observability/v1/traces 204
09:28:35.312 in Ping 
09:28:40.313 in Ping 
09:28:45.314 in Ping 
09:28:45.547 rpc 30 server.reportClientActivity
09:28:46.547 fetch POST /api/observability/v1/traces
09:28:46.547 fetched POST /api/observability/v1/traces 204
09:28:50.314 in Ping 
09:28:55.314 in Ping 
09:29:00.314 in Ping 
09:29:00.552 in Interrupt 30
09:29:01.553 fetch POST /api/observability/v1/traces
09:29:01.553 fetched POST /api/observability/v1/traces 204
09:29:05.315 in Ping 
09:29:10.315 in Ping 
09:29:10.545 rpc 31 server.reportClientActivity
09:29:11.545 fetch POST /api/observability/v1/traces
09:29:11.545 fetched POST /api/observability/v1/traces 204
09:29:15.316 in Ping 
09:29:20.320 in Ping 
09:29:25.324 in Ping 
09:29:25.547 in Interrupt 31
09:29:26.548 fetch POST /api/observability/v1/traces
09:29:26.548 fetched POST /api/observability/v1/traces 204
09:29:30.328 in Ping 
09:29:35.332 in Ping 
09:29:35.545 rpc 32 server.reportClientActivity
09:29:36.545 fetch POST /api/observability/v1/traces
09:29:36.545 fetched POST /api/observability/v1/traces 204
09:29:40.336 in Ping 
09:29:45.340 in Ping 
09:29:50.344 in Ping 
09:29:50.546 in Interrupt 32
09:29:51.548 fetch POST /api/observability/v1/traces
09:29:51.548 fetched POST /api/observability/v1/traces 204
09:29:55.348 in Ping 
09:30:00.352 in Ping 
09:30:00.546 rpc 33 server.reportClientActivity
09:30:01.546 fetch POST /api/observability/v1/traces
09:30:01.546 fetched POST /api/observability/v1/traces 204
09:30:04.097 exit 29 {"_tag":"Success","value":{"sequence":138}}
09:30:05.100 fetch POST /api/observability/v1/traces
09:30:05.100 fetched POST /api/observability/v1/traces 204
09:30:05.358 in Ping 
09:30:10.360 in Ping 
09:30:15.364 in Ping 
09:30:15.547 in Interrupt 33
09:30:15.594 chunk 17 38043 bytes
09:30:15.597 in Ack 17
09:30:16.129 rpc 34 server.reportClientActivity
09:30:16.549 fetch POST /api/observability/v1/traces
09:30:16.550 fetched POST /api/observability/v1/traces 204
09:30:17.852 chunk 17 38073 bytes
09:30:17.854 in Ack 17
09:30:18.855 fetch POST /api/observability/v1/traces
09:30:18.855 fetched POST /api/observability/v1/traces 204
09:30:20.368 in Ping 
09:30:25.372 in Ping 
09:30:26.683 fetch POST /api/observability/v1/traces
09:30:26.683 fetched POST /api/observability/v1/traces 204
09:30:30.376 in Ping 
09:30:31.131 in Interrupt 34
09:30:31.382 rpc 35 server.reportClientActivity
09:30:32.133 fetch POST /api/observability/v1/traces
09:30:32.133 fetched POST /api/observability/v1/traces 204
09:30:35.380 in Ping 
09:30:36.931 chunk 17 38455 bytes
09:30:36.936 in Ack 17
09:30:37.937 fetch POST /api/observability/v1/traces
09:30:37.937 fetched POST /api/observability/v1/traces 204
09:30:40.384 in Ping 
09:30:45.388 in Ping 
09:30:46.383 in Interrupt 35
09:30:47.384 fetch POST /api/observability/v1/traces
09:30:47.385 fetched POST /api/observability/v1/traces 204
09:30:50.393 in Ping 
09:30:50.547 rpc 36 server.reportClientActivity
09:30:51.549 fetch POST /api/observability/v1/traces
09:30:51.549 fetched POST /api/observability/v1/traces 204
09:30:55.396 in Ping 
09:31:00.400 in Ping 
09:31:05.404 in Ping 
09:31:05.548 in Interrupt 36
09:31:06.549 fetch POST /api/observability/v1/traces
09:31:06.549 fetched POST /api/observability/v1/traces 204
09:31:10.408 in Ping 
09:31:12.822 chunk 17 38999 bytes
09:31:12.826 in Ack 17
09:31:13.072 chunk 17 39457 bytes
09:31:13.074 in Ack 17
09:31:13.323 chunk 17 39809 bytes
09:31:13.325 in Ack 17
09:31:13.577 chunk 17 40359 bytes
09:31:13.579 in Ack 17
09:31:13.891 fetch POST /api/observability/v1/traces
09:31:13.891 fetched POST /api/observability/v1/traces 204
09:31:15.122 chunk 17 41361 bytes
09:31:15.124 in Ack 17
09:31:15.286 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:31:15.412 in Ping 
09:31:15.589 rpc 38 server.reportClientActivity
09:31:16.125 fetch POST /api/observability/v1/traces
09:31:16.125 fetched POST /api/observability/v1/traces 204
09:31:16.454 exit 37 {"_tag":"Success","value":{"sequence":158}}
09:31:16.577 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:31:17.455 fetch POST /api/observability/v1/traces
09:31:17.455 fetched POST /api/observability/v1/traces 204
09:31:17.941 chunk 17 41404 bytes
09:31:17.941 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:31:17.943 in Ack 17
09:31:18.292 chunk 17 41406 bytes
09:31:18.293 in Ack 17
09:31:18.944 fetch POST /api/observability/v1/traces
09:31:18.944 fetched POST /api/observability/v1/traces 204
09:31:20.416 in Ping 
09:31:20.990 chunk 6 4722 bytes
09:31:20.991 in Ack 6
09:31:20.993 rpc 40 orchestration.dispatchCommand thread.visit
09:31:20.993 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:31:21.250 chunk 17 42042 bytes
09:31:21.251 in Ack 17
09:31:21.309 exit 30 {"_tag":"Success","value":null}
09:31:21.992 fetch POST /api/observability/v1/traces
09:31:21.992 fetched POST /api/observability/v1/traces 204
09:31:23.492 chunk 6 4722 bytes
09:31:23.493 in Ack 6
09:31:23.749 chunk 17 42882 bytes
09:31:23.750 in Ack 17
09:31:23.895 fetch POST /api/observability/v1/traces
09:31:23.895 fetched POST /api/observability/v1/traces 204
09:31:24.000 chunk 17 44903 bytes
09:31:24.001 in Ack 17
09:31:24.250 chunk 17 51294 bytes
09:31:24.253 in Ack 17
09:31:24.499 chunk 17 51674 bytes
09:31:24.501 in Ack 17
09:31:25.000 chunk 17 51626 bytes
09:31:25.001 in Ack 17
09:31:25.019 fetch POST /api/observability/v1/traces
09:31:25.019 fetched POST /api/observability/v1/traces 204
09:31:25.420 in Ping 
09:31:25.494 chunk 6 4722 bytes
09:31:25.494 in Ack 6
09:31:25.750 chunk 17 54698 bytes
09:31:25.751 in Ack 17
09:31:25.763 in Interrupt 27
09:31:25.796 rpc 41 subscribeVcsStatus
09:31:26.000 chunk 17 55682 bytes
09:31:26.001 in Ack 17
09:31:26.135 exit 31 {"_tag":"Success","value":null}
09:31:26.220 exit 32 {"_tag":"Success","value":null}
09:31:26.220 exit 33 {"_tag":"Success","value":null}
09:31:26.249 chunk 17 58964 bytes
09:31:26.251 in Ack 17
09:31:26.497 fetch POST /api/observability/v1/traces
09:31:26.497 fetched POST /api/observability/v1/traces 204
09:31:26.499 chunk 17 59981 bytes
09:31:26.501 in Ack 17
09:31:26.545 exit 34 {"_tag":"Success","value":null}
09:31:26.730 exit 35 {"_tag":"Success","value":null}
09:31:26.749 chunk 17 61422 bytes
09:31:26.751 in Ack 17
09:31:26.831 exit 36 {"_tag":"Success","value":null}
09:31:26.876 exit 38 {"_tag":"Success","value":null}
09:31:27.000 chunk 17 61949 bytes
09:31:27.002 in Ack 17
09:31:27.040 chunk 41 1713 bytes
09:31:27.040 in Ack 41
09:31:27.127 rpc 42 server.reportClientActivity
09:31:27.250 chunk 17 61949 bytes
09:31:27.251 in Ack 17
09:31:27.502 fetch POST /api/observability/v1/traces
09:31:27.502 fetched POST /api/observability/v1/traces 204
09:31:28.492 exit 42 {"_tag":"Success","value":null}
09:31:29.493 fetch POST /api/observability/v1/traces
09:31:29.493 fetched POST /api/observability/v1/traces 204
09:31:30.381 chunk 17 62966 bytes
09:31:30.383 in Ack 17
09:31:30.466 in Ping 
09:31:30.631 chunk 6 4661 bytes
09:31:30.631 chunk 17 63016 bytes
09:31:30.632 in Ack 6
09:31:30.652 in Ack 17
09:31:30.881 chunk 17 64174 bytes
09:31:30.885 in Ack 17
09:31:30.998 rpc 43 orchestration.dispatchCommand thread.visit
09:31:30.998 exit 43 {"_tag":"Success","value":{"sequence":0}}
09:31:31.132 chunk 17 64306 bytes
09:31:31.134 in Ack 17
09:31:31.481 chunk 6 4658 bytes
09:31:31.482 chunk 17 66754 bytes
09:31:31.483 fetch POST /api/observability/v1/traces
09:31:31.483 fetched POST /api/observability/v1/traces 204
09:31:31.485 in Ack 6
09:31:31.486 rpc 44 orchestration.dispatchCommand thread.visit
09:31:31.486 exit 44 {"_tag":"Success","value":{"sequence":0}}
09:31:31.495 in Ack 17
