# story, webapp, warm start, seed 21

Profile story (graded): seed 21's survival story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 300 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 33.9s |
| Prompt reached the agent | 28.0s |
| Time to first text | 294.9s |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 1077 / 1136 / 1596 |
| Newest-text lag: max / median / p95 | 265.9s / 116.9s / 250.9s |
| Longest silence | 294.9s |
| Approval shown after it was asked | 241.6s |
| Bytes on the wire down / up | 72057 / 59649 |
| Bytes the app received / sent | 16839 / 62438 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 1.9 | 0 | 82 |
| 40 | 11.9 | 0 | 482 |
| 50 | 21.9 | 0 | 886 |
| 60 | 31.9 | 0 | 1077 |
| 70 | 41.9 | 0 | 1077 |
| 80 | 51.9 | 0 | 1077 |
| 90 | 61.9 | 0 | 1077 |
| 100 | 71.9 | 0 | 1077 |
| 110 | 81.9 | 0 | 1077 |
| 120 | 91.9 | 0 | 1077 |
| 130 | 101.9 | 0 | 1077 |
| 140 | 111.9 | 0 | 1077 |
| 150 | 121.9 | 0 | 1077 |
| 160 | 131.9 | 0 | 1077 |
| 170 | 141.9 | 0 | 1077 |
| 180 | 151.9 | 0 | 1077 |
| 190 | 161.9 | 0 | 1077 |
| 200 | 171.9 | 0 | 1077 |
| 210 | 181.9 | 0 | 1077 |
| 220 | 191.9 | 0 | 1077 |
| 230 | 201.9 | 0 | 1077 |
| 240 | 211.9 | 0 | 1077 |
| 250 | 221.9 | 0 | 1077 |
| 260 | 231.9 | 0 | 1077 |
| 270 | 241.9 | 0 | 1077 |
| 280 | 251.9 | 0 | 1077 |
| 290 | 261.9 | 0 | 1077 |
| 298 | 0.4 | 1077 | 1097 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217071283,"silences":0,"longestSilenceMs":801,"longestRecoveredGapMs":801},"stats":{"sendRate":2000,"deliveredRate":1986.3636363636363,"loss":0,"srttMs":8.119365279849113,"minRttMs":null,"sentPackets":138,"sentWireBytes":38850,"receivedPackets":110,"receivedWireBytes":87646,"lastHeardMs":12231,"unsyncedBytes":736,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217071283,"silences":0,"longestSilenceMs":801,"longestRecoveredGapMs":801},"stats":{"sendRate":50,"deliveredRate":1986.3636363636363,"loss":0,"srttMs":8.119365279849113,"minRttMs":null,"sentPackets":148,"sentWireBytes":44855,"receivedPackets":110,"receivedWireBytes":87646,"lastHeardMs":12231,"unsyncedBytes":736,"inflightPackets":21,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217071283,"silences":0,"longestSilenceMs":801,"longestRecoveredGapMs":801},"stats":{"sendRate":50,"deliveredRate":1986.3636363636363,"loss":0,"srttMs":8.119365279849113,"minRttMs":null,"sentPackets":149,"sentWireBytes":45456,"receivedPackets":110,"receivedWireBytes":87646,"lastHeardMs":12231,"unsyncedBytes":1193,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217093608,"silences":1,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2000,"deliveredRate":1427.0604167275733,"loss":0,"srttMs":8.119365279849113,"minRttMs":null,"sentPackets":166,"sentWireBytes":54981,"receivedPackets":111,"receivedWireBytes":87923,"lastHeardMs":34556,"unsyncedBytes":1193,"inflightPackets":39,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2000,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":166,"sentWireBytes":54981,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":851,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2000,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":166,"sentWireBytes":54981,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":851,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":185,"sentWireBytes":60432,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":851,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":202,"sentWireBytes":65286,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":1309,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":226,"sentWireBytes":72124,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":1309,"inflightPackets":98,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":249,"sentWireBytes":78676,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":1767,"inflightPackets":121,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":1767,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":1767,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":2391,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":2391,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":2849,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":2849,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":2849,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":3307,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":3307,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":3765,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":3765,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":4223,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":4223,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":4223,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":4681,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":4681,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":5139,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":4,"newConnections":1,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":5139,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":4,"newConnections":2,"handshakeMs":1202,"lastDataAt":1794217110658,"silences":3,"longestSilenceMs":22325,"longestRecoveredGapMs":22325},"stats":{"sendRate":2140.9989306675434,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1486.729444619868,"minRttMs":11837,"sentPackets":262,"sentWireBytes":82383,"receivedPackets":113,"receivedWireBytes":88835,"lastHeardMs":51607,"unsyncedBytes":5139,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 214/849","thread":"961/961","assistant":"32/32 21/21 778/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":4,"iceRestarts":4,"newConnections":2,"handshakeMs":12126,"lastDataAt":1794217367035,"silences":4,"longestSilenceMs":255115,"longestRecoveredGapMs":22325},"stats":{"sendRate":2000,"deliveredRate":1016.974492067083,"loss":0.2875,"srttMs":1697.9999321205482,"minRttMs":1894,"sentPackets":282,"sentWireBytes":88640,"receivedPackets":119,"receivedWireBytes":91861,"lastHeardMs":307984,"unsyncedBytes":5317,"inflightPackets":11,"duplicateBytes":0}}
- shim trace
09:37:39.973 fetch GET /api/auth/session
09:37:39.975 fetched GET /api/auth/session 200
09:37:40.405 fetch GET /.well-known/t3/environment
09:37:40.429 fetched GET /.well-known/t3/environment 200
09:37:40.509 fetch GET /.well-known/t3/environment
09:37:40.510 fetched GET /.well-known/t3/environment 200
09:37:40.555 rpc 0 subscribeServerConfig
09:37:40.556 chunk 0 38462 bytes
09:37:40.693 in Ack 0
09:37:40.698 rpc 1 subscribeServerLifecycle
09:37:40.700 rpc 2 subscribeServerLifecycle
09:37:40.712 fetch GET /api/auth/session
09:37:40.714 fetched GET /api/auth/session 200
09:37:40.714 rpc 3 subscribeDeviceState
09:37:40.719 rpc 4 preview.reportProfiles
09:37:40.754 rpc 5 server.reportClientActivity
09:37:40.823 rpc 6 subscribeProjectClones
09:37:40.869 fetch GET /api/orchestration/threads/mcp%3Adc888d68-5fc4-4182-a96f-e5deca1d171c/bounded
09:37:40.876 fetch GET /api/orchestration/shell
09:37:40.878 fetched GET /api/orchestration/shell 200
09:37:40.881 rpc 7 orchestration.subscribeShell
09:37:40.881 chunk 7 2736 bytes
09:37:40.922 rpc 8 server.refreshProviders
09:37:41.012 rpc 9 orchestration.dispatchCommand thread.visit
09:37:41.012 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:41.036 rpc 10 subscribeTerminalMetadata
09:37:41.037 rpc 11 subscribeVcsStatus
09:37:41.039 rpc 12 subscribePreviewEvents
09:37:41.041 rpc 13 assets.createUrl
09:37:41.042 rpc 14 preview.list
09:37:41.042 rpc 15 projects.readFile
09:37:41.046 in Ack 7
09:37:41.175 chunk 7 2712 bytes
09:37:41.253 in Ack 7
09:37:41.434 fetch POST /api/observability/v1/traces
09:37:41.434 fetched POST /api/observability/v1/traces 204
09:37:41.839 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"a68dc1d3-6d2e-4110-8a93-69c5b0a47929","sessions":[]}}
09:37:41.839 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:41.839 chunk 1 3815 bytes
09:37:41.848 in Ack 1
09:37:42.063 chunk 2 3815 bytes
09:37:42.064 in Ack 2
09:37:42.250 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:17.635Z","configurableProviders":false,"con
09:37:42.250 chunk 11 1632 bytes
09:37:42.250 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:42.254 in Ack 11
09:37:42.519 fetch POST /api/observability/v1/traces
09:37:42.519 fetched POST /api/observability/v1/traces 204
09:37:42.750 chunk 7 4635 bytes
09:37:42.750 in Ack 7
09:37:42.882 chunk 3 876 bytes
09:37:42.882 exit 4 {"_tag":"Success","value":null}
09:37:42.882 exit 5 {"_tag":"Success","value":null}
09:37:42.882 chunk 6 4 bytes
09:37:42.882 chunk 10 36 bytes
09:37:42.883 in Ack 3
09:37:42.889 rpc 16 preview.reportProfiles
09:37:42.890 in Ack 6
09:37:42.891 in Ack 10
09:37:42.964 exit 16 {"_tag":"Success","value":null}
09:37:43.141 rpc 17 server.reportClientActivity
09:37:43.214 exit 17 {"_tag":"Success","value":null}
09:37:43.751 fetch POST /api/observability/v1/traces
09:37:43.752 fetched POST /api/observability/v1/traces 204
09:37:44.171 fetched GET /api/orchestration/threads/mcp%3Adc888d68-5fc4-4182-a96f-e5deca1d171c/bounded 200
09:37:44.178 rpc 18 orchestration.subscribeThread
09:37:44.178 chunk 18 8283 bytes
09:37:44.197 in Interrupt 11
09:37:44.200 rpc 19 subscribeVcsStatus
09:37:44.201 in Ack 18
09:37:44.336 chunk 19 1632 bytes
09:37:44.336 in Ack 19
09:37:44.430 chunk 18 8323 bytes
09:37:44.431 in Ack 18
09:37:44.500 rpc 20 server.reportClientActivity
09:37:44.635 exit 20 {"_tag":"Success","value":null}
09:37:44.683 chunk 18 12691 bytes
09:37:44.687 in Ack 18
09:37:44.976 chunk 18 13498 bytes
09:37:44.980 in Ack 18
09:37:45.175 fetch POST /api/observability/v1/traces
09:37:45.176 fetched POST /api/observability/v1/traces 204
09:37:45.230 chunk 18 17977 bytes
09:37:45.233 in Ack 18
09:37:45.320 rpc 21 attachments.createUploadUrl
09:37:45.480 chunk 18 19797 bytes
09:37:45.486 in Ack 18
09:37:45.500 in Interrupt 19
09:37:45.504 rpc 22 subscribeVcsStatus
09:37:45.516 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-a9d9b0e2-3457-46a4-8235-106f36d7b9e8","expiresAt":1794217665436,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:45.517 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWE5ZDliMGUyLTM0NTctNDZhNC04MjM1LTEwNmYzNmQ3YjllOCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY2NTQzNn0.eEgzAaGwgtMjUghl2ciUlr3DuI_NcrQUIAmW0Uairjs
09:37:45.521 in Ping 
09:37:45.601 chunk 22 1632 bytes
09:37:45.601 in Ack 22
09:37:45.730 chunk 18 20409 bytes
09:37:45.732 in Ack 18
09:37:45.755 rpc 23 server.reportClientActivity
09:37:45.980 chunk 18 21100 bytes
09:37:45.982 in Ack 18
09:37:46.237 fetch POST /api/observability/v1/traces
09:37:46.237 fetched POST /api/observability/v1/traces 204
09:37:46.745 exit 23 {"_tag":"Success","value":null}
09:37:47.747 fetch POST /api/observability/v1/traces
09:37:47.747 fetched POST /api/observability/v1/traces 204
09:37:48.582 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:48.938 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:48.944 rpc 25 attachments.delete
09:37:49.189 chunk 18 25348 bytes
09:37:49.191 in Ack 18
09:37:49.195 rpc 26 assets.createUrl
09:37:49.438 chunk 7 4721 bytes
09:37:49.439 in Ack 7
09:37:49.447 chunk 18 26810 bytes
09:37:49.449 in Ack 18
09:37:49.568 exit 25 {"_tag":"Success","value":null}
09:37:49.581 fetch POST /api/observability/v1/traces
09:37:49.581 fetched POST /api/observability/v1/traces 204
09:37:49.697 chunk 18 30178 bytes
09:37:49.701 in Ack 18
09:37:49.939 chunk 7 4660 bytes
09:37:49.940 in Ack 7
09:37:49.981 chunk 18 30336 bytes
09:37:49.987 in Ack 18
09:37:50.206 chunk 7 4657 bytes
09:37:50.206 in Ack 7
09:37:50.208 rpc 27 orchestration.dispatchCommand thread.visit
09:37:50.208 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:50.231 chunk 18 32783 bytes
09:37:50.233 in Ack 18
09:37:50.466 chunk 7 4657 bytes
09:37:50.468 in Ack 7
09:37:50.483 chunk 18 33559 bytes
09:37:50.489 in Ack 18
09:37:50.511 in Interrupt 22
09:37:50.514 rpc 28 subscribeVcsStatus
09:37:50.523 in Ping 
09:37:50.542 chunk 28 1713 bytes
09:37:50.542 in Ack 28
09:37:50.702 fetch POST /api/observability/v1/traces
09:37:50.702 fetched POST /api/observability/v1/traces 204
09:37:50.733 chunk 18 35260 bytes
09:37:50.735 in Ack 18
09:37:50.767 rpc 29 server.reportClientActivity
09:37:50.807 exit 29 {"_tag":"Success","value":null}
09:37:51.741 fetch POST /api/observability/v1/traces
09:37:51.741 fetched POST /api/observability/v1/traces 204
09:37:52.262 rpc 30 server.refreshProviders
09:37:52.262 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:17.635Z","configurableProviders":false,"con
09:37:53.057 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:53.262 fetch POST /api/observability/v1/traces
09:37:53.262 fetched POST /api/observability/v1/traces 204
09:37:55.523 in Ping 
09:37:55.781 fetch POST /api/observability/v1/traces
09:37:55.781 fetched POST /api/observability/v1/traces 204
09:38:00.524 in Ping 
09:38:05.525 in Ping 
09:38:05.680 rpc 32 server.reportClientActivity
09:38:06.683 fetch POST /api/observability/v1/traces
09:38:06.683 fetched POST /api/observability/v1/traces 204
09:38:10.526 in Ping 
09:38:15.527 in Ping 
09:38:20.527 in Ping 
09:38:20.688 in Interrupt 32
09:38:21.689 fetch POST /api/observability/v1/traces
09:38:21.689 fetched POST /api/observability/v1/traces 204
09:38:25.528 in Ping 
09:38:26.563 exit 31 {"_tag":"Success","value":{"sequence":138}}
09:38:27.568 fetch POST /api/observability/v1/traces
09:38:27.568 fetched POST /api/observability/v1/traces 204
09:38:30.528 in Ping 
09:38:30.682 rpc 33 server.reportClientActivity
09:38:31.682 fetch POST /api/observability/v1/traces
09:38:31.682 fetched POST /api/observability/v1/traces 204
09:38:35.531 in Ping 
09:38:40.536 in Ping 
09:38:45.539 in Ping 
09:38:45.682 in Interrupt 33
09:38:46.684 fetch POST /api/observability/v1/traces
09:38:46.684 fetched POST /api/observability/v1/traces 204
09:38:50.543 in Ping 
09:38:55.548 in Ping 
09:38:55.681 rpc 34 server.reportClientActivity
09:38:56.682 fetch POST /api/observability/v1/traces
09:38:56.682 fetched POST /api/observability/v1/traces 204
09:39:00.551 in Ping 
09:39:05.555 in Ping 
09:39:10.559 in Ping 
09:39:10.682 in Interrupt 34
09:39:11.685 fetch POST /api/observability/v1/traces
09:39:11.685 fetched POST /api/observability/v1/traces 204
09:39:15.563 in Ping 
09:39:20.567 in Ping 
09:39:20.684 rpc 35 server.reportClientActivity
09:39:21.684 fetch POST /api/observability/v1/traces
09:39:21.684 fetched POST /api/observability/v1/traces 204
09:39:25.572 in Ping 
09:39:30.575 in Ping 
09:39:35.579 in Ping 
09:39:35.684 in Interrupt 35
09:39:36.686 fetch POST /api/observability/v1/traces
09:39:36.686 fetched POST /api/observability/v1/traces 204
09:39:40.583 in Ping 
09:39:45.587 in Ping 
09:39:45.684 rpc 36 server.reportClientActivity
09:39:46.684 fetch POST /api/observability/v1/traces
09:39:46.684 fetched POST /api/observability/v1/traces 204
09:39:50.593 in Ping 
09:39:55.595 in Ping 
09:40:00.599 in Ping 
09:40:00.685 in Interrupt 36
09:40:01.690 fetch POST /api/observability/v1/traces
09:40:01.690 fetched POST /api/observability/v1/traces 204
09:40:05.603 in Ping 
09:40:10.609 in Ping 
09:40:10.685 rpc 37 server.reportClientActivity
09:40:11.686 fetch POST /api/observability/v1/traces
09:40:11.686 fetched POST /api/observability/v1/traces 204
09:40:15.611 in Ping 
09:40:20.615 in Ping 
09:40:25.619 in Ping 
09:40:25.687 in Interrupt 37
09:40:26.688 fetch POST /api/observability/v1/traces
09:40:26.688 fetched POST /api/observability/v1/traces 204
09:40:30.623 in Ping 
09:40:35.627 in Ping 
09:40:35.694 rpc 38 server.reportClientActivity
09:40:36.694 fetch POST /api/observability/v1/traces
09:40:36.694 fetched POST /api/observability/v1/traces 204
09:40:40.631 in Ping 
09:40:45.635 in Ping 
09:40:50.640 in Ping 
09:40:50.696 in Interrupt 38
09:40:51.698 fetch POST /api/observability/v1/traces
09:40:51.698 fetched POST /api/observability/v1/traces 204
09:40:55.643 in Ping 
09:41:00.647 in Ping 
09:41:00.687 rpc 39 server.reportClientActivity
09:41:01.688 fetch POST /api/observability/v1/traces
09:41:01.688 fetched POST /api/observability/v1/traces 204
09:41:05.651 in Ping 
09:41:10.655 in Ping 
09:41:15.660 in Ping 
09:41:15.689 in Interrupt 39
09:41:16.694 fetch POST /api/observability/v1/traces
09:41:16.694 fetched POST /api/observability/v1/traces 204
09:41:20.663 in Ping 
09:41:25.667 in Ping 
09:41:25.689 rpc 40 server.reportClientActivity
09:41:26.692 fetch POST /api/observability/v1/traces
09:41:26.692 fetched POST /api/observability/v1/traces 204
09:41:30.671 in Ping 
09:41:35.676 in Ping 
09:41:40.679 in Ping 
09:41:40.692 in Interrupt 40
09:41:41.697 fetch POST /api/observability/v1/traces
09:41:41.697 fetched POST /api/observability/v1/traces 204
09:41:45.683 in Ping 
09:41:50.687 in Ping 
09:41:50.689 rpc 41 server.reportClientActivity
09:41:51.692 fetch POST /api/observability/v1/traces
09:41:51.693 fetched POST /api/observability/v1/traces 204
09:41:55.691 in Ping 
09:42:00.695 in Ping 
09:42:05.692 in Interrupt 41
09:42:05.700 in Ping 
09:42:06.696 fetch POST /api/observability/v1/traces
09:42:06.697 fetched POST /api/observability/v1/traces 204
09:42:10.706 in Ping 
09:42:15.693 rpc 42 server.reportClientActivity
09:42:15.708 in Ping 
09:42:16.696 fetch POST /api/observability/v1/traces
09:42:16.696 fetched POST /api/observability/v1/traces 204
09:42:20.711 in Ping 
09:42:25.715 in Ping 
09:42:30.695 in Interrupt 42
09:42:30.719 in Ping 
09:42:31.700 fetch POST /api/observability/v1/traces
09:42:31.700 fetched POST /api/observability/v1/traces 204
09:42:35.723 in Ping 
09:42:40.697 rpc 43 server.reportClientActivity
09:42:40.727 in Ping 
09:42:41.700 fetch POST /api/observability/v1/traces
09:42:41.700 fetched POST /api/observability/v1/traces 204
09:42:45.734 in Ping 
09:42:45.776 chunk 18 38427 bytes
09:42:45.779 in Ack 18
09:42:46.026 chunk 18 38459 bytes
09:42:46.028 in Ack 18
09:42:46.564 chunk 18 39187 bytes
09:42:46.567 in Ack 18
09:42:46.781 fetch POST /api/observability/v1/traces
09:42:46.781 fetched POST /api/observability/v1/traces 204
09:42:46.815 chunk 18 39235 bytes
09:42:46.817 in Ack 18
09:42:47.308 chunk 18 39981 bytes
09:42:47.309 in Ack 18
09:42:47.557 chunk 18 40361 bytes
09:42:47.559 in Ack 18
09:42:47.838 fetch POST /api/observability/v1/traces
09:42:47.838 fetched POST /api/observability/v1/traces 204
09:42:48.093 exit 26 {"_tag":"Success","value":{"expiresAt":1794220964684,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
09:42:49.095 fetch POST /api/observability/v1/traces
09:42:49.096 fetched POST /api/observability/v1/traces 204
09:42:49.151 chunk 18 41363 bytes
09:42:49.154 in Ack 18
09:42:49.354 rpc 44 orchestration.dispatchCommand runtime-request.respond
09:42:50.154 fetch POST /api/observability/v1/traces
09:42:50.154 fetched POST /api/observability/v1/traces 204
09:42:50.735 in Ping 
09:42:51.652 exit 44 {"_tag":"Success","value":{"sequence":158}}
