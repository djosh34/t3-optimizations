# story, webapp, warm start, seed 38

Profile story (graded): seed 38's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 131 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 7.5s |
| Prompt reached the agent | 11.0s |
| Time to first text | 30.1s |
| Whole reply shown | 129.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 55.6s / 16.9s / 51.1s |
| Longest silence | 48.1s |
| Approval shown after it was asked | 22.6s |
| Bytes on the wire down / up | 380278 / 129527 |
| Bytes the app received / sent | 139650 / 86224 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 8.9 | 0 | 361 |
| 30 | 18.9 | 0 | 762 |
| 40 | 18.9 | 397 | 1077 |
| 50 | 28.9 | 397 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 7.9 | 1077 | 1383 |
| 80 | 10.1 | 1378 | 1596 |
| 90 | 20.1 | 1378 | 1596 |
| 100 | 30.1 | 1378 | 1596 |
| 110 | 40.1 | 1378 | 1596 |
| 120 | 50.1 | 1378 | 1596 |
| 128 | 55.6 | 1469 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217256230,"silences":0,"longestSilenceMs":1001,"longestRecoveredGapMs":1001},"stats":{"sendRate":4500,"deliveredRate":0,"loss":0,"srttMs":8.67061720484438,"minRttMs":1,"sentPackets":124,"sentWireBytes":35186,"receivedPackets":213,"receivedWireBytes":121656,"lastHeardMs":14809,"unsyncedBytes":736,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 538/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217266339,"silences":1,"longestSilenceMs":2002,"longestRecoveredGapMs":2002},"stats":{"sendRate":2250,"deliveredRate":1114.909090909091,"loss":0,"srttMs":3366.751944466418,"minRttMs":1,"sentPackets":142,"sentWireBytes":42082,"receivedPackets":267,"receivedWireBytes":150302,"lastHeardMs":24918,"unsyncedBytes":0,"inflightPackets":17,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659 144/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217276442,"silences":1,"longestSilenceMs":2002,"longestRecoveredGapMs":2002},"stats":{"sendRate":1000,"deliveredRate":654.4830355051381,"loss":0.6046136363636363,"srttMs":4473.078240204672,"minRttMs":1,"sentPackets":188,"sentWireBytes":53031,"receivedPackets":329,"receivedWireBytes":171887,"lastHeardMs":35021,"unsyncedBytes":0,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 46/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 451/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659 164/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217286301,"silences":2,"longestSilenceMs":7613,"longestRecoveredGapMs":7613},"stats":{"sendRate":1000,"deliveredRate":504.10803618168273,"loss":0.5946581168831168,"srttMs":4452.943460179087,"minRttMs":1,"sentPackets":208,"sentWireBytes":55881,"receivedPackets":343,"receivedWireBytes":177921,"lastHeardMs":44880,"unsyncedBytes":0,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 491/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217295278,"silences":2,"longestSilenceMs":7613,"longestRecoveredGapMs":7613},"stats":{"sendRate":1000,"deliveredRate":538.340666247643,"loss":0.08333333333333337,"srttMs":4007.4732890389996,"minRttMs":3227,"sentPackets":252,"sentWireBytes":66123,"receivedPackets":404,"receivedWireBytes":202034,"lastHeardMs":53857,"unsyncedBytes":457,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 491/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217295278,"silences":2,"longestSilenceMs":7613,"longestRecoveredGapMs":7613},"stats":{"sendRate":1000,"deliveredRate":538.340666247643,"loss":0.08333333333333337,"srttMs":4007.4732890389996,"minRttMs":3227,"sentPackets":282,"sentWireBytes":74493,"receivedPackets":404,"receivedWireBytes":202034,"lastHeardMs":53857,"unsyncedBytes":457,"inflightPackets":83,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 347/1542","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 491/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217316880,"silences":3,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1000,"deliveredRate":356.73396731359776,"loss":0.3923039215686275,"srttMs":3527.8792228017783,"minRttMs":2045,"sentPackets":310,"sentWireBytes":82360,"receivedPackets":458,"receivedWireBytes":223425,"lastHeardMs":75460,"unsyncedBytes":398,"inflightPackets":94,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1224/1542","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 510/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"545/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217326179,"silences":4,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1083.2730413605586,"deliveredRate":464.77401808337385,"loss":0.356430926916221,"srttMs":3253.823098092491,"minRttMs":1872,"sentPackets":332,"sentWireBytes":92298,"receivedPackets":468,"receivedWireBytes":229998,"lastHeardMs":84758,"unsyncedBytes":501,"inflightPackets":105,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1224/1542 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 525/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217334731,"silences":5,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1083.2730413605586,"deliveredRate":583.2092608098495,"loss":0.2495016488413547,"srttMs":4221.470210830929,"minRttMs":1872,"sentPackets":332,"sentWireBytes":92298,"receivedPackets":503,"receivedWireBytes":245704,"lastHeardMs":93310,"unsyncedBytes":398,"inflightPackets":103,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1224/1542 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 525/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217334731,"silences":5,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1083.2730413605586,"deliveredRate":583.2092608098495,"loss":0.2495016488413547,"srttMs":4221.470210830929,"minRttMs":1872,"sentPackets":332,"sentWireBytes":92298,"receivedPackets":503,"receivedWireBytes":245704,"lastHeardMs":93310,"unsyncedBytes":855,"inflightPackets":103,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1224/1542 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 525/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217334731,"silences":5,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1083.2730413605586,"deliveredRate":583.2092608098495,"loss":0.2495016488413547,"srttMs":4221.470210830929,"minRttMs":1872,"sentPackets":345,"sentWireBytes":99902,"receivedPackets":503,"receivedWireBytes":245704,"lastHeardMs":93310,"unsyncedBytes":855,"inflightPackets":116,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1224/1542 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 525/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 174/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217334731,"silences":5,"longestSilenceMs":12568,"longestRecoveredGapMs":12568},"stats":{"sendRate":1083.2730413605586,"deliveredRate":583.2092608098495,"loss":0.2495016488413547,"srttMs":4221.470210830929,"minRttMs":1872,"sentPackets":359,"sentWireBytes":106882,"receivedPackets":503,"receivedWireBytes":245704,"lastHeardMs":93310,"unsyncedBytes":1312,"inflightPackets":130,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1224/1542 454/1093","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 533/723","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 525/635","providerTurns":"772/772 772/772 322/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 408/785","messages":"544/544 456/456 901/901 544/544 543/543 264/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"318/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":739,"lastDataAt":1794217382284,"silences":6,"longestSilenceMs":45932,"longestRecoveredGapMs":45932},"stats":{"sendRate":1110.1275476925134,"deliveredRate":430.43260029401273,"loss":0.4184011541889483,"srttMs":4053.5162551674302,"minRttMs":2139,"sentPackets":378,"sentWireBytes":114271,"receivedPackets":512,"receivedWireBytes":249412,"lastHeardMs":140863,"unsyncedBytes":0,"inflightPackets":137,"duplicateBytes":0}}
- shim trace
09:40:42.391 fetch GET /api/auth/session
09:40:42.394 fetched GET /api/auth/session 200
09:40:42.695 fetch GET /.well-known/t3/environment
09:40:42.696 fetched GET /.well-known/t3/environment 200
09:40:42.714 fetch GET /.well-known/t3/environment
09:40:42.715 fetched GET /.well-known/t3/environment 200
09:40:42.723 rpc 0 subscribeServerConfig
09:40:42.724 chunk 0 38462 bytes
09:40:42.804 in Ack 0
09:40:42.807 rpc 1 subscribeServerLifecycle
09:40:42.810 rpc 2 subscribeServerLifecycle
09:40:42.815 fetch GET /api/auth/session
09:40:42.816 fetched GET /api/auth/session 200
09:40:42.817 rpc 3 subscribeDeviceState
09:40:42.819 rpc 4 preview.reportProfiles
09:40:43.003 rpc 5 subscribeProjectClones
09:40:43.066 rpc 6 server.reportClientActivity
09:40:43.101 fetch GET /api/orchestration/threads/mcp%3A9407b488-361d-446a-bffd-2a846acaf051/bounded
09:40:43.130 fetch GET /api/orchestration/shell
09:40:43.216 rpc 7 server.refreshProviders
09:40:43.296 rpc 8 orchestration.dispatchCommand thread.visit
09:40:43.296 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:40:43.317 fetched GET /api/orchestration/shell 200
09:40:43.318 rpc 9 subscribeTerminalMetadata
09:40:43.319 rpc 10 subscribeVcsStatus
09:40:43.320 rpc 11 subscribePreviewEvents
09:40:43.324 rpc 12 orchestration.subscribeShell
09:40:43.324 chunk 12 4576 bytes
09:40:43.327 rpc 13 assets.createUrl
09:40:43.328 rpc 14 preview.list
09:40:43.329 rpc 15 projects.readFile
09:40:43.382 in Ack 12
09:40:43.590 chunk 12 4552 bytes
09:40:43.796 in Ack 12
09:40:43.807 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"684ede34-2e16-4322-ade8-875e05882543","sessions":[]}}
09:40:43.807 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:40:43.808 chunk 1 3815 bytes
09:40:43.821 fetch POST /api/observability/v1/traces
09:40:43.821 fetched POST /api/observability/v1/traces 204
09:40:43.912 in Ack 1
09:40:44.049 chunk 2 3815 bytes
09:40:44.049 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:40:11.551Z","configurableProviders":false,"con
09:40:44.049 in Ack 2
09:40:44.147 chunk 10 1632 bytes
09:40:44.147 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:40:44.148 in Ack 10
09:40:44.396 chunk 12 4592 bytes
09:40:44.397 in Ack 12
09:40:44.508 chunk 3 876 bytes
09:40:44.508 exit 4 {"_tag":"Success","value":null}
09:40:44.508 chunk 5 4 bytes
09:40:44.508 exit 6 {"_tag":"Success","value":null}
09:40:44.508 chunk 9 36 bytes
09:40:44.509 in Ack 3
09:40:44.516 rpc 16 preview.reportProfiles
09:40:44.517 in Ack 5
09:40:44.518 in Ack 9
09:40:44.572 exit 16 {"_tag":"Success","value":null}
09:40:44.711 fetched GET /api/orchestration/threads/mcp%3A9407b488-361d-446a-bffd-2a846acaf051/bounded 200
09:40:44.716 rpc 17 orchestration.subscribeThread
09:40:44.717 chunk 17 8282 bytes
09:40:44.745 in Interrupt 10
09:40:44.783 rpc 18 subscribeVcsStatus
09:40:44.785 in Ack 17
09:40:44.906 fetch POST /api/observability/v1/traces
09:40:44.906 fetched POST /api/observability/v1/traces 204
09:40:44.967 chunk 17 8258 bytes
09:40:44.968 in Ack 17
09:40:45.035 rpc 19 server.reportClientActivity
09:40:45.238 chunk 17 12690 bytes
09:40:45.246 in Ack 17
09:40:45.317 chunk 18 1632 bytes
09:40:45.348 in Ack 18
09:40:45.563 chunk 17 13497 bytes
09:40:45.569 exit 19 {"_tag":"Success","value":null}
09:40:45.575 in Ack 17
09:40:45.814 chunk 17 19796 bytes
09:40:45.817 in Ack 17
09:40:45.832 in Interrupt 18
09:40:45.834 rpc 20 subscribeVcsStatus
09:40:45.887 chunk 20 1632 bytes
09:40:45.887 in Ack 20
09:40:45.970 fetch POST /api/observability/v1/traces
09:40:45.970 fetched POST /api/observability/v1/traces 204
09:40:46.066 chunk 17 21099 bytes
09:40:46.067 in Ack 17
09:40:46.087 rpc 21 server.reportClientActivity
09:40:46.169 exit 21 {"_tag":"Success","value":null}
09:40:46.831 rpc 22 attachments.createUploadUrl
09:40:46.981 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-1f9ba919-d454-4e2a-8819-d2ec9c3473f7","expiresAt":1794217846945,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:40:46.982 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTFmOWJhOTE5LWQ0NTQtNGUyYS04ODE5LWQyZWM5YzM0NzNmNyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzg0Njk0NX0.DMv4CGEwndGBqO93-X6_8PCuAXtCzzCsBxBU3ULww4w
09:40:47.068 fetch POST /api/observability/v1/traces
09:40:47.068 fetched POST /api/observability/v1/traces 204
09:40:47.802 in Ping 
09:40:50.033 rpc 23 orchestration.dispatchCommand message.dispatch
09:40:50.468 exit 23 {"_tag":"Success","value":{"sequence":109}}
09:40:50.475 rpc 24 attachments.delete
09:40:50.566 exit 24 {"_tag":"Success","value":null}
09:40:50.741 chunk 12 4678 bytes
09:40:50.742 chunk 17 25368 bytes
09:40:50.743 in Ack 12
09:40:50.800 in Ack 17
09:40:50.805 rpc 25 assets.createUrl
09:40:50.995 chunk 17 27722 bytes
09:40:50.999 in Ack 17
09:40:51.046 fetch POST /api/observability/v1/traces
09:40:51.046 fetched POST /api/observability/v1/traces 204
09:40:51.246 chunk 17 30179 bytes
09:40:51.249 in Ack 17
09:40:51.495 chunk 12 4617 bytes
09:40:51.495 in Ack 12
09:40:51.508 chunk 17 30201 bytes
09:40:51.511 in Ack 17
09:40:51.761 chunk 17 31902 bytes
09:40:51.765 in Ack 17
09:40:51.899 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/4c04f027-d647-42ed-ad29-baa8f173598d","expiresAt":1794220851899}}
09:40:52.250 fetch POST /api/observability/v1/traces
09:40:52.250 fetched POST /api/observability/v1/traces 204
09:40:52.802 in Ping 
09:40:52.961 chunk 17 32034 bytes
09:40:52.964 in Ack 17
09:40:53.210 chunk 12 4614 bytes
09:40:53.211 chunk 17 35259 bytes
09:40:53.211 in Ack 12
09:40:53.213 rpc 26 orchestration.dispatchCommand thread.visit
09:40:53.213 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:40:53.223 in Ack 17
09:40:53.233 in Interrupt 20
09:40:53.238 rpc 27 subscribeVcsStatus
09:40:53.256 chunk 27 1632 bytes
09:40:53.256 in Ack 27
09:40:53.745 chunk 12 4614 bytes
09:40:53.746 chunk 17 35259 bytes
09:40:53.747 rpc 28 server.reportClientActivity
09:40:53.748 in Ack 12
09:40:53.758 in Ack 17
09:40:53.807 exit 28 {"_tag":"Success","value":null}
09:40:54.008 fetch POST /api/observability/v1/traces
09:40:54.008 fetched POST /api/observability/v1/traces 204
09:40:54.073 rpc 29 server.refreshProviders
09:40:54.073 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:40:11.551Z","configurableProviders":false,"con
09:40:55.127 fetch POST /api/observability/v1/traces
09:40:55.127 fetched POST /api/observability/v1/traces 204
09:40:56.162 rpc 30 orchestration.dispatchCommand message.dispatch
09:40:56.481 fetch POST /api/observability/v1/traces
09:40:56.481 fetched POST /api/observability/v1/traces 204
09:40:57.802 in Ping 
09:41:00.252 chunk 12 4678 bytes
09:41:00.257 in Ack 12
09:41:01.260 fetch POST /api/observability/v1/traces
09:41:01.260 fetched POST /api/observability/v1/traces 204
09:41:02.806 in Ping 
09:41:03.209 exit 30 {"_tag":"Success","value":{"sequence":151}}
09:41:04.210 fetch POST /api/observability/v1/traces
09:41:04.210 fetched POST /api/observability/v1/traces 204
09:41:07.806 in Ping 
09:41:07.953 rpc 31 server.reportClientActivity
09:41:08.954 fetch POST /api/observability/v1/traces
09:41:08.955 fetched POST /api/observability/v1/traces 204
09:41:12.807 in Ping 
09:41:17.808 in Ping 
09:41:22.808 in Ping 
09:41:22.961 in Interrupt 31
09:41:23.963 fetch POST /api/observability/v1/traces
09:41:23.963 fetched POST /api/observability/v1/traces 204
09:41:24.977 chunk 17 37864 bytes
09:41:24.982 in Ack 17
09:41:25.459 chunk 17 37958 bytes
09:41:25.462 in Ack 17
09:41:25.870 chunk 17 38576 bytes
09:41:25.872 in Ack 17
09:41:25.986 fetch POST /api/observability/v1/traces
09:41:25.987 fetched POST /api/observability/v1/traces 204
09:41:27.809 in Ping 
09:41:28.836 chunk 17 38672 bytes
09:41:28.838 in Ack 17
09:41:29.336 chunk 17 38624 bytes
09:41:29.339 in Ack 17
09:41:29.839 fetch POST /api/observability/v1/traces
09:41:29.840 fetched POST /api/observability/v1/traces 204
09:41:30.090 chunk 12 4742 bytes
09:41:30.091 in Ack 12
09:41:30.096 rpc 32 orchestration.dispatchCommand thread.visit
09:41:30.096 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:41:31.092 fetch POST /api/observability/v1/traces
09:41:31.093 fetched POST /api/observability/v1/traces 204
09:41:31.382 chunk 17 40645 bytes
09:41:31.385 in Ack 17
09:41:31.970 exit 31 {"_tag":"Success","value":null}
09:41:32.156 chunk 17 41689 bytes
09:41:32.160 in Ack 17
09:41:32.386 fetch POST /api/observability/v1/traces
09:41:32.386 fetched POST /api/observability/v1/traces 204
09:41:32.809 in Ping 
09:41:32.954 rpc 33 server.reportClientActivity
09:41:33.159 chunk 17 41691 bytes
09:41:33.161 in Ack 17
09:41:33.954 fetch POST /api/observability/v1/traces
09:41:33.954 fetched POST /api/observability/v1/traces 204
09:41:35.044 chunk 17 42531 bytes
09:41:35.047 in Ack 17
09:41:36.049 fetch POST /api/observability/v1/traces
09:41:36.049 fetched POST /api/observability/v1/traces 204
09:41:37.810 in Ping 
09:41:42.819 in Ping 
09:41:47.822 in Ping 
09:41:47.849 chunk 17 42885 bytes
09:41:47.851 in Ack 17
09:41:47.955 in Interrupt 33
09:41:48.107 chunk 17 43239 bytes
09:41:48.112 in Ack 17
09:41:48.431 chunk 17 43521 bytes
09:41:48.441 in Ack 17
09:41:48.793 chunk 17 43833 bytes
09:41:48.800 in Ack 17
09:41:48.886 fetch POST /api/observability/v1/traces
09:41:48.886 fetched POST /api/observability/v1/traces 204
09:41:49.216 chunk 17 44103 bytes
09:41:49.219 in Ack 17
09:41:49.553 chunk 17 44406 bytes
09:41:49.556 in Ack 17
09:41:50.222 fetch POST /api/observability/v1/traces
09:41:50.222 fetched POST /api/observability/v1/traces 204
09:41:51.657 chunk 17 44520 bytes
09:41:51.660 in Ack 17
09:41:52.270 chunk 17 44808 bytes
09:41:52.279 in Ack 17
09:41:52.661 fetch POST /api/observability/v1/traces
09:41:52.661 fetched POST /api/observability/v1/traces 204
09:41:52.826 in Ping 
09:41:54.242 chunk 17 45063 bytes
09:41:54.245 in Ack 17
09:41:55.261 fetch POST /api/observability/v1/traces
09:41:55.261 fetched POST /api/observability/v1/traces 204
09:41:56.186 chunk 17 46065 bytes
09:41:56.195 in Ack 17
09:41:56.689 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:41:57.197 fetch POST /api/observability/v1/traces
09:41:57.197 fetched POST /api/observability/v1/traces 204
09:41:57.830 in Ping 
09:41:57.954 rpc 35 server.reportClientActivity
09:41:58.805 exit 34 {"_tag":"Success","value":{"sequence":206}}
09:41:58.954 fetch POST /api/observability/v1/traces
09:41:58.955 fetched POST /api/observability/v1/traces 204
09:41:59.419 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:42:00.412 fetch POST /api/observability/v1/traces
09:42:00.412 fetched POST /api/observability/v1/traces 204
09:42:02.834 in Ping 
09:42:07.838 in Ping 
09:42:11.662 chunk 17 49058 bytes
09:42:11.665 in Ack 17
09:42:11.915 chunk 17 49300 bytes
09:42:11.924 in Ack 17
09:42:12.165 chunk 17 49343 bytes
09:42:12.168 in Ack 17
09:42:12.668 fetch POST /api/observability/v1/traces
09:42:12.668 fetched POST /api/observability/v1/traces 204
09:42:12.669 chunk 17 49295 bytes
09:42:12.671 in Ack 17
09:42:12.842 in Ping 
09:42:12.956 in Interrupt 35
09:42:13.671 chunk 17 49345 bytes
09:42:13.673 in Ack 17
09:42:13.713 fetch POST /api/observability/v1/traces
09:42:13.713 fetched POST /api/observability/v1/traces 204
09:42:17.847 in Ping 
09:42:22.850 in Ping 
09:42:22.954 rpc 37 server.reportClientActivity
09:42:23.955 fetch POST /api/observability/v1/traces
09:42:23.955 fetched POST /api/observability/v1/traces 204
09:42:27.854 in Ping 
09:42:32.858 in Ping 
09:42:37.866 in Ping 
09:42:37.957 in Interrupt 37
09:42:38.958 fetch POST /api/observability/v1/traces
09:42:38.958 fetched POST /api/observability/v1/traces 204
09:42:42.870 in Ping 
09:42:47.874 in Ping 
09:42:47.954 rpc 38 server.reportClientActivity
09:42:48.955 fetch POST /api/observability/v1/traces
09:42:48.955 fetched POST /api/observability/v1/traces 204
09:42:52.878 in Ping 
09:42:57.883 in Ping 
09:43:00.668 chunk 17 49605 bytes
09:43:00.671 in Ack 17
09:43:01.677 fetch POST /api/observability/v1/traces
09:43:01.678 fetched POST /api/observability/v1/traces 204
09:43:02.880 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:43:02.895 in Ping 
09:43:03.050 in Interrupt 38
09:43:03.303 rpc 39 server.reportClientActivity
09:43:03.739 chunk 17 49947 bytes
09:43:03.743 in Ack 17
09:43:03.926 fetch POST /api/observability/v1/traces
09:43:03.926 fetched POST /api/observability/v1/traces 204
09:43:04.002 chunk 17 50023 bytes
09:43:04.006 in Ack 17
09:43:04.260 chunk 17 50073 bytes
09:43:04.265 in Ack 17
09:43:05.107 fetch POST /api/observability/v1/traces
09:43:05.107 fetched POST /api/observability/v1/traces 204
