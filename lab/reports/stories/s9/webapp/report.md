# story, webapp, warm start, seed 9

Profile story (graded): seed 9's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 158 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 8.0s |
| Prompt reached the agent | 11.0s |
| Time to first text | 23.1s |
| Whole reply shown | 156.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 111.5s / 33.5s / 103.5s |
| Longest silence | 116.0s |
| Approval shown after it was asked | 3.0s |
| Bytes on the wire down / up | 421774 / 169218 |
| Bytes the app received / sent | 212222 / 125875 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 8.9 | 0 | 357 |
| 30 | 8.9 | 397 | 758 |
| 40 | 18.9 | 397 | 1077 |
| 50 | 5.5 | 1077 | 1297 |
| 60 | 15.5 | 1077 | 1596 |
| 70 | 25.5 | 1077 | 1596 |
| 80 | 35.5 | 1077 | 1596 |
| 90 | 45.5 | 1077 | 1596 |
| 100 | 55.5 | 1077 | 1596 |
| 110 | 65.5 | 1077 | 1596 |
| 120 | 75.5 | 1077 | 1596 |
| 130 | 85.5 | 1077 | 1596 |
| 140 | 95.5 | 1077 | 1596 |
| 150 | 105.5 | 1077 | 1596 |
| 156 | 111.5 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","runs":"1014/1014 971/971","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"605/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216513309,"silences":0,"longestSilenceMs":1223,"longestRecoveredGapMs":1223},"stats":{"sendRate":2250,"deliveredRate":1021.5596330275229,"loss":0,"srttMs":8.244534029827538,"minRttMs":null,"sentPackets":140,"sentWireBytes":39314,"receivedPackets":190,"receivedWireBytes":117180,"lastHeardMs":16205,"unsyncedBytes":868,"inflightPackets":18,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","runs":"1014/1014 971/971","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9 73/73","attempts":"659/659 659/659 168/583","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216523205,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":2250,"deliveredRate":991.6998049599612,"loss":0.3355263157894737,"srttMs":2818.3188039544807,"minRttMs":2154,"sentPackets":186,"sentWireBytes":56733,"receivedPackets":241,"receivedWireBytes":140833,"lastHeardMs":26101,"unsyncedBytes":133,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 339/654","assistant":"32/32 21/21","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216533342,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":1406.25,"deliveredRate":1092.8595890673014,"loss":0.2594738309716599,"srttMs":2531.6068576057805,"minRttMs":2103,"sentPackets":240,"sentWireBytes":67608,"receivedPackets":327,"receivedWireBytes":173189,"lastHeardMs":36239,"unsyncedBytes":167,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 498/498","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216543402,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":1938.1193554196238,"deliveredRate":0,"loss":0.21972012136175004,"srttMs":2274.5901771471954,"minRttMs":2103,"sentPackets":297,"sentWireBytes":78107,"receivedPackets":426,"receivedWireBytes":208586,"lastHeardMs":46298,"unsyncedBytes":167,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 661/1542","thread":"696/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216553953,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":2000,"deliveredRate":274.65271238730486,"loss":0.7025974025974027,"srttMs":2485.181826953224,"minRttMs":2103,"sentPackets":367,"sentWireBytes":95717,"receivedPackets":481,"receivedWireBytes":234868,"lastHeardMs":56849,"unsyncedBytes":284,"inflightPackets":91,"duplicateBytes":0}}
- sample failed: locator.click: Timeout 2000ms exceeded.
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216564026,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":3108.3786256994254,"deliveredRate":1688.1107988736233,"loss":0.2463850636363636,"srttMs":2282.343414920746,"minRttMs":2103,"sentPackets":439,"sentWireBytes":119777,"receivedPackets":581,"receivedWireBytes":291595,"lastHeardMs":66922,"unsyncedBytes":623,"inflightPackets":107,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":3744.6613179481706,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":2103,"sentPackets":503,"sentWireBytes":137392,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":100,"inflightPackets":156,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":2000,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":527,"sentWireBytes":144284,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":556,"inflightPackets":180,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":250,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":546,"sentWireBytes":149728,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":556,"inflightPackets":199,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":548,"sentWireBytes":150304,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":1012,"inflightPackets":201,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":550,"sentWireBytes":150880,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":1012,"inflightPackets":203,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":552,"sentWireBytes":151456,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":1635,"inflightPackets":205,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":553,"sentWireBytes":151808,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":1635,"inflightPackets":206,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":555,"sentWireBytes":152512,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":2092,"inflightPackets":208,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","runs":"1014/1014 971/971 839/839","providerTurns":"772/772 772/772 654/654","assistant":"32/32 21/21 1340/1340 333/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216566502,"silences":1,"longestSilenceMs":3375,"longestRecoveredGapMs":3375},"stats":{"sendRate":50,"deliveredRate":1783.1361187321804,"loss":0.28572868118181816,"srttMs":2290.65221593953,"minRttMs":null,"sentPackets":556,"sentWireBytes":152864,"receivedPackets":610,"receivedWireBytes":309981,"lastHeardMs":69398,"unsyncedBytes":2092,"inflightPackets":209,"duplicateBytes":0}}
- shim trace
09:28:17.989 fetch GET /api/auth/session
09:28:17.991 fetched GET /api/auth/session 200
09:28:18.522 fetch GET /.well-known/t3/environment
09:28:18.523 fetched GET /.well-known/t3/environment 200
09:28:18.546 fetch GET /.well-known/t3/environment
09:28:18.547 fetched GET /.well-known/t3/environment 200
09:28:18.557 rpc 0 subscribeServerConfig
09:28:18.558 chunk 0 38458 bytes
09:28:18.689 in Ack 0
09:28:18.693 rpc 1 subscribeServerLifecycle
09:28:18.700 rpc 2 subscribeServerLifecycle
09:28:18.707 fetch GET /api/auth/session
09:28:18.709 fetched GET /api/auth/session 200
09:28:18.710 rpc 3 subscribeDeviceState
09:28:18.716 rpc 4 preview.reportProfiles
09:28:18.799 rpc 5 server.reportClientActivity
09:28:18.858 rpc 6 subscribeProjectClones
09:28:18.928 fetch GET /api/orchestration/threads/mcp%3Aa62d2d0c-f9fe-4168-98f9-8cfbe54ed79f/bounded
09:28:18.955 fetch GET /api/orchestration/shell
09:28:18.958 fetched GET /api/orchestration/shell 200
09:28:18.963 rpc 7 orchestration.subscribeShell
09:28:18.963 chunk 7 4593 bytes
09:28:19.085 rpc 8 server.refreshProviders
09:28:19.203 rpc 9 orchestration.dispatchCommand thread.visit
09:28:19.203 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:28:19.253 rpc 10 subscribeTerminalMetadata
09:28:19.254 rpc 11 subscribeVcsStatus
09:28:19.255 rpc 12 subscribePreviewEvents
09:28:19.259 rpc 13 assets.createUrl
09:28:19.262 rpc 14 preview.list
09:28:19.264 rpc 15 projects.readFile
09:28:19.268 in Ack 7
09:28:19.268 chunk 7 4569 bytes
09:28:19.310 in Ack 7
09:28:19.517 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c001eece-04f8-4229-bb54-5848baff1a53","sessions":[]}}
09:28:19.517 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:19.603 fetch POST /api/observability/v1/traces
09:28:19.603 fetched POST /api/observability/v1/traces 204
09:28:19.605 chunk 1 3814 bytes
09:28:19.622 in Ack 1
09:28:19.843 chunk 2 3814 bytes
09:28:19.843 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:48.560Z","configurableProviders":false,"con
09:28:19.843 chunk 11 1632 bytes
09:28:19.844 in Ack 2
09:28:19.850 in Ack 11
09:28:19.950 exit 13 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:20.201 chunk 7 4698 bytes
09:28:20.203 in Ack 7
09:28:20.245 chunk 3 876 bytes
09:28:20.245 exit 4 {"_tag":"Success","value":null}
09:28:20.245 exit 5 {"_tag":"Success","value":null}
09:28:20.245 chunk 6 4 bytes
09:28:20.245 chunk 10 36 bytes
09:28:20.251 in Ack 3
09:28:20.268 rpc 16 preview.reportProfiles
09:28:20.270 in Ack 6
09:28:20.271 in Ack 10
09:28:20.356 exit 16 {"_tag":"Success","value":null}
09:28:20.587 rpc 17 server.reportClientActivity
09:28:20.608 fetch POST /api/observability/v1/traces
09:28:20.608 fetched POST /api/observability/v1/traces 204
09:28:20.675 exit 17 {"_tag":"Success","value":null}
09:28:20.836 chunk 7 4637 bytes
09:28:20.837 in Ack 7
09:28:21.564 fetched GET /api/orchestration/threads/mcp%3Aa62d2d0c-f9fe-4168-98f9-8cfbe54ed79f/bounded 200
09:28:21.571 rpc 18 orchestration.subscribeThread
09:28:21.571 chunk 18 8282 bytes
09:28:21.591 in Interrupt 11
09:28:21.595 rpc 19 subscribeVcsStatus
09:28:21.596 in Ack 18
09:28:21.730 fetch POST /api/observability/v1/traces
09:28:21.730 fetched POST /api/observability/v1/traces 204
09:28:21.824 chunk 18 11259 bytes
09:28:21.826 in Ack 18
09:28:21.942 chunk 19 1632 bytes
09:28:21.943 rpc 20 server.reportClientActivity
09:28:21.987 in Ack 19
09:28:22.076 chunk 18 13497 bytes
09:28:22.077 in Ack 18
09:28:22.325 chunk 18 17975 bytes
09:28:22.328 in Ack 18
09:28:22.565 exit 20 {"_tag":"Success","value":null}
09:28:22.581 chunk 18 21097 bytes
09:28:22.586 in Ack 18
09:28:22.633 in Interrupt 19
09:28:22.642 rpc 21 subscribeVcsStatus
09:28:22.819 chunk 7 4634 bytes
09:28:22.820 in Ack 7
09:28:22.879 chunk 21 1632 bytes
09:28:22.880 fetch POST /api/observability/v1/traces
09:28:22.880 fetched POST /api/observability/v1/traces 204
09:28:22.890 in Ack 21
09:28:22.896 rpc 22 server.reportClientActivity
09:28:22.955 exit 22 {"_tag":"Success","value":null}
09:28:23.278 rpc 23 attachments.createUploadUrl
09:28:23.463 exit 23 {"_tag":"Success","value":{"attachmentId":"pending-e6d6ee55-ad4b-4d61-bade-b709cc7f736f","expiresAt":1794217103410,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:23.470 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWU2ZDZlZTU1LWFkNGItNGQ2MS1iYWRlLWI3MDljYzdmNzM2ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzEwMzQxMH0.CvXZ2t61MKe3PrS0S8jH0vX16QmrJM2ZLVFI0KJfKmE
09:28:23.553 in Ping 
09:28:23.896 fetch POST /api/observability/v1/traces
09:28:23.896 fetched POST /api/observability/v1/traces 204
09:28:27.105 rpc 24 orchestration.dispatchCommand message.dispatch
09:28:27.558 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:28:27.562 rpc 25 attachments.delete
09:28:28.083 exit 25 {"_tag":"Success","value":null}
09:28:28.105 fetch POST /api/observability/v1/traces
09:28:28.105 fetched POST /api/observability/v1/traces 204
09:28:28.408 chunk 18 23552 bytes
09:28:28.415 in Ack 18
09:28:28.665 chunk 18 23554 bytes
09:28:28.667 in Ping 
09:28:28.670 in Ack 18
09:28:28.915 chunk 7 4659 bytes
09:28:28.916 in Ack 7
09:28:29.020 chunk 18 24394 bytes
09:28:29.022 in Ack 18
09:28:29.390 chunk 18 27847 bytes
09:28:29.399 rpc 26 orchestration.dispatchCommand thread.visit
09:28:29.399 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:28:29.412 in Ack 18
09:28:29.424 rpc 27 assets.createUrl
09:28:29.512 fetch POST /api/observability/v1/traces
09:28:29.512 fetched POST /api/observability/v1/traces 204
09:28:29.640 chunk 18 29972 bytes
09:28:29.642 in Ack 18
09:28:29.890 chunk 7 4659 bytes
09:28:29.890 chunk 18 30989 bytes
09:28:29.891 in Ack 7
09:28:29.922 in Ack 18
09:28:30.140 chunk 7 4656 bytes
09:28:30.140 chunk 18 31121 bytes
09:28:30.141 in Ack 7
09:28:30.143 rpc 28 orchestration.dispatchCommand thread.visit
09:28:30.143 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:28:30.153 in Ack 18
09:28:30.756 chunk 18 33544 bytes
09:28:30.758 fetch POST /api/observability/v1/traces
09:28:30.758 fetched POST /api/observability/v1/traces 204
09:28:30.763 in Ack 18
09:28:31.006 chunk 18 33568 bytes
09:28:31.009 in Ack 18
09:28:31.113 rpc 29 server.refreshProviders
09:28:31.113 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:48.560Z","configurableProviders":false,"con
09:28:31.791 fetch POST /api/observability/v1/traces
09:28:31.791 fetched POST /api/observability/v1/traces 204
09:28:32.301 chunk 7 4656 bytes
09:28:32.324 in Ack 7
09:28:33.242 rpc 30 orchestration.dispatchCommand message.dispatch
09:28:33.309 chunk 18 34346 bytes
09:28:33.310 fetch POST /api/observability/v1/traces
09:28:33.310 fetched POST /api/observability/v1/traces 204
09:28:33.313 in Ack 18
09:28:33.325 in Interrupt 21
09:28:33.333 rpc 31 subscribeVcsStatus
09:28:33.593 chunk 18 35257 bytes
09:28:33.594 rpc 32 server.reportClientActivity
09:28:33.604 in Ack 18
09:28:33.667 in Ping 
09:28:34.361 fetch POST /api/observability/v1/traces
09:28:34.361 fetched POST /api/observability/v1/traces 204
09:28:36.037 chunk 31 1632 bytes
09:28:36.039 in Ack 31
09:28:37.754 chunk 7 4720 bytes
09:28:37.757 in Ack 7
09:28:38.673 in Ping 
09:28:38.759 fetch POST /api/observability/v1/traces
09:28:38.759 fetched POST /api/observability/v1/traces 204
09:28:40.641 fetch POST /api/observability/v1/traces
09:28:40.641 fetched POST /api/observability/v1/traces 204
09:28:41.028 exit 30 {"_tag":"Success","value":{"sequence":154}}
09:28:42.031 fetch POST /api/observability/v1/traces
09:28:42.031 fetched POST /api/observability/v1/traces 204
09:28:42.294 chunk 18 37278 bytes
09:28:42.296 in Ack 18
09:28:43.155 fetch POST /api/observability/v1/traces
09:28:43.155 fetched POST /api/observability/v1/traces 204
09:28:43.673 in Ping 
09:28:46.487 exit 32 {"_tag":"Success","value":null}
09:28:46.605 chunk 18 37862 bytes
09:28:46.608 in Ack 18
09:28:46.740 rpc 33 server.reportClientActivity
09:28:47.489 fetch POST /api/observability/v1/traces
09:28:47.489 fetched POST /api/observability/v1/traces 204
09:28:48.673 in Ping 
09:28:49.787 chunk 18 37914 bytes
09:28:49.789 in Ack 18
09:28:50.790 fetch POST /api/observability/v1/traces
09:28:50.791 fetched POST /api/observability/v1/traces 204
09:28:51.577 chunk 18 38700 bytes
09:28:51.579 in Ack 18
09:28:51.827 chunk 18 38702 bytes
09:28:51.829 in Ack 18
09:28:52.580 fetch POST /api/observability/v1/traces
09:28:52.580 fetched POST /api/observability/v1/traces 204
09:28:53.073 chunk 7 4784 bytes
09:28:53.074 in Ack 7
09:28:53.080 rpc 34 orchestration.dispatchCommand thread.visit
09:28:53.080 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:28:53.154 chunk 18 39542 bytes
09:28:53.157 in Ack 18
09:28:53.343 exit 33 {"_tag":"Success","value":null}
09:28:53.465 chunk 18 40067 bytes
09:28:53.470 in Ack 18
09:28:53.674 in Ping 
09:28:54.076 fetch POST /api/observability/v1/traces
09:28:54.076 fetched POST /api/observability/v1/traces 204
09:28:55.334 chunk 7 4784 bytes
09:28:55.335 in Ack 7
09:28:55.527 chunk 18 43122 bytes
09:28:55.532 in Ack 18
09:28:55.776 chunk 18 43428 bytes
09:28:55.778 in Ack 18
09:28:56.026 chunk 18 43480 bytes
09:28:56.040 in Ack 18
09:28:56.339 fetch POST /api/observability/v1/traces
09:28:56.339 fetched POST /api/observability/v1/traces 204
09:28:57.029 chunk 18 43432 bytes
09:28:57.031 in Ack 18
09:28:58.034 fetch POST /api/observability/v1/traces
09:28:58.034 fetched POST /api/observability/v1/traces 204
09:28:58.674 in Ping 
09:28:59.803 chunk 7 4784 bytes
09:28:59.804 in Ack 7
09:29:00.306 chunk 18 43432 bytes
09:29:00.308 in Ack 18
09:29:00.557 chunk 18 45131 bytes
09:29:00.561 in Ack 18
09:29:00.805 fetch POST /api/observability/v1/traces
09:29:00.805 fetched POST /api/observability/v1/traces 204
09:29:00.808 chunk 18 45131 bytes
09:29:00.810 in Ack 18
09:29:01.808 chunk 7 4784 bytes
09:29:01.808 in Ack 7
09:29:01.811 fetch POST /api/observability/v1/traces
09:29:01.811 fetched POST /api/observability/v1/traces 204
09:29:03.081 rpc 35 orchestration.dispatchCommand thread.visit
09:29:03.081 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:29:03.562 chunk 7 5091 bytes
09:29:03.563 in Ack 7
09:29:03.675 in Ping 
09:29:04.081 fetch POST /api/observability/v1/traces
09:29:04.081 fetched POST /api/observability/v1/traces 204
09:29:08.675 in Ping 
09:29:08.861 rpc 36 server.reportClientActivity
09:29:09.861 fetch POST /api/observability/v1/traces
09:29:09.861 fetched POST /api/observability/v1/traces 204
09:29:12.641 chunk 18 47189 bytes
09:29:12.643 in Ack 18
09:29:12.930 chunk 18 47663 bytes
09:29:12.946 in Ack 18
09:29:13.742 chunk 18 48665 bytes
09:29:13.742 fetch POST /api/observability/v1/traces
09:29:13.742 fetched POST /api/observability/v1/traces 204
09:29:13.743 in Ping 
09:29:13.753 in Ack 18
09:29:14.754 fetch POST /api/observability/v1/traces
09:29:14.754 fetched POST /api/observability/v1/traces 204
09:29:16.291 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:29:16.635 chunk 18 52152 bytes
09:29:16.636 in Ack 18
09:29:17.290 fetch POST /api/observability/v1/traces
09:29:17.290 fetched POST /api/observability/v1/traces 204
09:29:18.674 exit 37 {"_tag":"Success","value":{"sequence":195}}
09:29:18.747 in Ping 
09:29:18.854 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:29:19.142 chunk 18 52195 bytes
09:29:19.144 in Ack 18
09:29:19.675 fetch POST /api/observability/v1/traces
09:29:19.675 fetched POST /api/observability/v1/traces 204
09:29:20.827 exit 36 {"_tag":"Success","value":null}
09:29:20.937 chunk 18 52243 bytes
09:29:20.940 in Ack 18
09:29:21.212 exit 38 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:29:21.445 chunk 18 55344 bytes
09:29:21.450 in Ack 18
09:29:21.465 in Interrupt 31
09:29:21.528 rpc 39 subscribeVcsStatus
09:29:21.779 rpc 40 server.reportClientActivity
09:29:21.828 fetch POST /api/observability/v1/traces
09:29:21.829 fetched POST /api/observability/v1/traces 204
09:29:22.699 chunk 18 55394 bytes
09:29:22.705 in Ack 18
09:29:22.951 chunk 18 56787 bytes
09:29:22.954 in Ack 18
09:29:23.201 chunk 7 5093 bytes
09:29:23.203 in Ack 7
09:29:23.207 rpc 41 orchestration.dispatchCommand thread.visit
09:29:23.207 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:29:23.452 chunk 18 56788 bytes
09:29:23.455 in Ack 18
09:29:23.718 chunk 18 57805 bytes
09:29:23.719 fetch POST /api/observability/v1/traces
09:29:23.719 fetched POST /api/observability/v1/traces 204
09:29:23.722 in Ack 18
09:29:23.801 in Ping 
09:29:24.027 chunk 39 1713 bytes
09:29:24.027 in Ack 39
09:29:24.222 chunk 7 4788 bytes
09:29:24.233 in Ack 7
09:29:24.726 fetch POST /api/observability/v1/traces
09:29:24.726 fetched POST /api/observability/v1/traces 204
09:29:26.229 chunk 7 4788 bytes
09:29:26.230 in Ack 7
09:29:26.485 chunk 18 57805 bytes
09:29:26.487 in Ack 18
09:29:26.737 chunk 18 60891 bytes
09:29:26.741 in Ack 18
09:29:27.232 fetch POST /api/observability/v1/traces
09:29:27.232 fetched POST /api/observability/v1/traces 204
09:29:28.803 in Ping 
09:29:33.808 in Ping 
09:29:34.874 fetch POST /api/observability/v1/traces
09:29:34.874 fetched POST /api/observability/v1/traces 204
09:29:36.781 in Interrupt 40
09:29:37.036 rpc 42 server.reportClientActivity
09:29:37.782 fetch POST /api/observability/v1/traces
09:29:37.782 fetched POST /api/observability/v1/traces 204
09:29:38.812 in Ping 
09:29:43.814 in Ping 
09:29:48.818 in Ping 
09:29:52.038 in Interrupt 42
09:29:53.043 fetch POST /api/observability/v1/traces
09:29:53.043 fetched POST /api/observability/v1/traces 204
09:29:53.823 in Ping 
09:29:58.788 rpc 43 server.reportClientActivity
09:29:58.832 in Ping 
09:29:59.790 fetch POST /api/observability/v1/traces
09:29:59.791 fetched POST /api/observability/v1/traces 204
09:30:03.835 in Ping 
09:30:08.839 in Ping 
09:30:13.789 in Interrupt 43
09:30:13.842 in Ping 
09:30:14.791 fetch POST /api/observability/v1/traces
09:30:14.791 fetched POST /api/observability/v1/traces 204
09:30:18.846 in Ping 
09:30:23.788 rpc 44 server.reportClientActivity
09:30:23.851 in Ping 
09:30:24.789 fetch POST /api/observability/v1/traces
09:30:24.789 fetched POST /api/observability/v1/traces 204
09:30:28.854 in Ping 
09:30:33.858 in Ping 
09:30:38.789 in Interrupt 44
09:30:38.862 in Ping 
09:30:39.789 fetch POST /api/observability/v1/traces
09:30:39.789 fetched POST /api/observability/v1/traces 204
09:30:43.866 in Ping 
09:30:48.788 rpc 45 server.reportClientActivity
09:30:48.870 in Ping 
09:30:49.788 fetch POST /api/observability/v1/traces
09:30:49.788 fetched POST /api/observability/v1/traces 204
09:30:53.874 in Ping 
09:30:58.878 in Ping 
09:31:03.789 in Interrupt 45
09:31:03.882 in Ping 
09:31:04.790 fetch POST /api/observability/v1/traces
09:31:04.790 fetched POST /api/observability/v1/traces 204
09:31:08.382 chunk 18 61235 bytes
09:31:08.388 in Ack 18
09:31:08.633 chunk 18 61533 bytes
09:31:08.636 in Ack 18
09:31:08.743 exit 27 {"_tag":"Success","value":{"expiresAt":1794220264339,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
09:31:08.883 chunk 18 61719 bytes
09:31:08.886 in Ack 18
09:31:09.084 in Ping 
09:31:09.088 rpc 46 server.reportClientActivity
09:31:09.791 fetch POST /api/observability/v1/traces
09:31:09.791 fetched POST /api/observability/v1/traces 204
