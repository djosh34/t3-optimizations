# story, webapp, warm start, seed 2

Profile story (graded): seed 2's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 92 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.8s |
| Prompt reached the agent | 5.7s |
| Time to first text | 22.1s |
| Whole reply shown | 89.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 20.2s / 3.2s / 16.2s |
| Longest silence | 43.0s |
| Approval shown after it was asked | 33.3s |
| Bytes on the wire down / up | 182690 / 156303 |
| Bytes the app received / sent | 55905 / 86509 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.2 | 0 | 169 |
| 20 | 14.2 | 0 | 569 |
| 30 | 14.2 | 397 | 974 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 2.0 | 1077 | 1158 |
| 80 | 4.3 | 1378 | 1528 |
| 89 | 13.2 | 1379 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217017972,"silences":0,"longestSilenceMs":1186,"longestRecoveredGapMs":1186},"stats":{"sendRate":2250,"deliveredRate":1749.9493754044825,"loss":0,"srttMs":1171.0505798193922,"minRttMs":2202,"sentPackets":149,"sentWireBytes":43066,"receivedPackets":174,"receivedWireBytes":117128,"lastHeardMs":16137,"unsyncedBytes":518,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 52/818","user":"10/10 9/9 73/73","attempts":"659/659 659/659 51/635","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217027850,"silences":0,"longestSilenceMs":1676,"longestRecoveredGapMs":1676},"stats":{"sendRate":2777.730901105677,"deliveredRate":1341.111050983724,"loss":0.13594545454545454,"srttMs":2108.1757553987236,"minRttMs":2095,"sentPackets":191,"sentWireBytes":53379,"receivedPackets":221,"receivedWireBytes":132594,"lastHeardMs":26016,"unsyncedBytes":0,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 256/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 364/498","runs":"1014/1014 971/971 426/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217037798,"silences":1,"longestSilenceMs":2126,"longestRecoveredGapMs":2126},"stats":{"sendRate":1000,"deliveredRate":1431.6986505221716,"loss":0.01875,"srttMs":3400.8941485821197,"minRttMs":2095,"sentPackets":245,"sentWireBytes":68815,"receivedPackets":247,"receivedWireBytes":142424,"lastHeardMs":35964,"unsyncedBytes":0,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"619/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 426/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217048104,"silences":1,"longestSilenceMs":2126,"longestRecoveredGapMs":2126},"stats":{"sendRate":1000,"deliveredRate":0,"loss":0.013125,"srttMs":3400.8941485821197,"minRttMs":2095,"sentPackets":271,"sentWireBytes":73748,"receivedPackets":266,"receivedWireBytes":148518,"lastHeardMs":46269,"unsyncedBytes":167,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"675/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 705/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"668/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217058099,"silences":2,"longestSilenceMs":2126,"longestRecoveredGapMs":2126},"stats":{"sendRate":1549.323017408124,"deliveredRate":1201.782468364755,"loss":0,"srttMs":2631.661388304598,"minRttMs":2095,"sentPackets":303,"sentWireBytes":81477,"receivedPackets":278,"receivedWireBytes":152637,"lastHeardMs":56264,"unsyncedBytes":0,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"675/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 705/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"850/1002"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217058969,"silences":2,"longestSilenceMs":2126,"longestRecoveredGapMs":2126},"stats":{"sendRate":1549.323017408124,"deliveredRate":1201.782468364755,"loss":0,"srttMs":2572.2037147665233,"minRttMs":2095,"sentPackets":308,"sentWireBytes":82111,"receivedPackets":279,"receivedWireBytes":152919,"lastHeardMs":57134,"unsyncedBytes":0,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"675/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 705/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"850/1002"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217058969,"silences":2,"longestSilenceMs":2126,"longestRecoveredGapMs":2126},"stats":{"sendRate":1549.323017408124,"deliveredRate":0,"loss":0,"srttMs":2572.2037147665233,"minRttMs":2095,"sentPackets":320,"sentWireBytes":84758,"receivedPackets":279,"receivedWireBytes":152919,"lastHeardMs":57134,"unsyncedBytes":456,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 621/1542","frame":"1029/1029","thread":"675/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 705/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"221/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217087721,"silences":3,"longestSilenceMs":23580,"longestRecoveredGapMs":23580},"stats":{"sendRate":2323.9845261121864,"deliveredRate":819.7183098591549,"loss":0.8148148148148149,"srttMs":2629.827844118119,"minRttMs":2098,"sentPackets":353,"sentWireBytes":97147,"receivedPackets":284,"receivedWireBytes":155478,"lastHeardMs":85886,"unsyncedBytes":398,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 180/1093 621/1542 664/1068","frame":"1029/1029","thread":"675/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 705/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 65/543","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2652/2652","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":849,"lastDataAt":1794217099649,"silences":4,"longestSilenceMs":23580,"longestRecoveredGapMs":23580},"stats":{"sendRate":3976.4087111510908,"deliveredRate":1313.7474121929224,"loss":0.23521396011396012,"srttMs":2464.2291635077163,"minRttMs":2066,"sentPackets":404,"sentWireBytes":111098,"receivedPackets":293,"receivedWireBytes":160962,"lastHeardMs":97814,"unsyncedBytes":0,"inflightPackets":21,"duplicateBytes":0}}
- shim trace
09:36:42.730 fetch GET /api/auth/session
09:36:42.732 fetched GET /api/auth/session 200
09:36:42.980 fetch GET /.well-known/t3/environment
09:36:42.981 fetched GET /.well-known/t3/environment 200
09:36:43.009 fetch GET /.well-known/t3/environment
09:36:43.011 fetched GET /.well-known/t3/environment 200
09:36:43.022 rpc 0 subscribeServerConfig
09:36:43.026 chunk 0 38458 bytes
09:36:43.132 in Ack 0
09:36:43.135 rpc 1 subscribeServerLifecycle
09:36:43.137 rpc 2 subscribeServerLifecycle
09:36:43.142 fetch GET /api/auth/session
09:36:43.143 fetched GET /api/auth/session 200
09:36:43.144 rpc 3 subscribeDeviceState
09:36:43.147 rpc 4 preview.reportProfiles
09:36:43.270 rpc 5 subscribeProjectClones
09:36:43.318 rpc 6 server.reportClientActivity
09:36:43.329 fetch GET /api/orchestration/threads/mcp%3A89e4f80d-f327-4877-95b3-af0a7a3fd268/bounded
09:36:43.338 fetch GET /api/orchestration/shell
09:36:43.389 rpc 7 server.refreshProviders
09:36:43.509 rpc 8 orchestration.dispatchCommand thread.visit
09:36:43.509 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:36:43.539 fetched GET /api/orchestration/shell 200
09:36:43.540 rpc 9 subscribeTerminalMetadata
09:36:43.541 rpc 10 subscribeVcsStatus
09:36:43.542 rpc 11 subscribePreviewEvents
09:36:43.549 rpc 12 orchestration.subscribeShell
09:36:43.549 chunk 12 4550 bytes
09:36:43.552 rpc 13 assets.createUrl
09:36:43.555 rpc 14 preview.list
09:36:43.568 rpc 15 projects.readFile
09:36:43.615 in Ack 12
09:36:43.937 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"0523ee90-d163-4fb3-b573-4e78890f0bf9","sessions":[]}}
09:36:43.938 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:36:43.938 chunk 12 4526 bytes
09:36:44.032 in Ack 12
09:36:44.058 fetch POST /api/observability/v1/traces
09:36:44.058 fetched POST /api/observability/v1/traces 204
09:36:44.070 chunk 1 3814 bytes
09:36:44.073 in Ack 1
09:36:44.179 chunk 2 3814 bytes
09:36:44.179 in Ack 2
09:36:44.294 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:12.924Z","configurableProviders":false,"con
09:36:44.294 chunk 10 1632 bytes
09:36:44.294 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:36:44.305 in Ack 10
09:36:44.518 chunk 3 876 bytes
09:36:44.519 exit 4 {"_tag":"Success","value":null}
09:36:44.519 chunk 5 4 bytes
09:36:44.519 exit 6 {"_tag":"Success","value":null}
09:36:44.519 chunk 9 36 bytes
09:36:44.519 in Ack 3
09:36:44.529 rpc 16 preview.reportProfiles
09:36:44.530 in Ack 5
09:36:44.531 in Ack 9
09:36:44.544 chunk 12 4569 bytes
09:36:44.545 in Ack 12
09:36:44.663 exit 16 {"_tag":"Success","value":null}
09:36:44.779 fetched GET /api/orchestration/threads/mcp%3A89e4f80d-f327-4877-95b3-af0a7a3fd268/bounded 200
09:36:44.791 rpc 17 orchestration.subscribeThread
09:36:44.792 chunk 17 8282 bytes
09:36:44.823 in Interrupt 10
09:36:44.827 rpc 18 subscribeVcsStatus
09:36:44.829 in Ack 17
09:36:45.041 chunk 18 1632 bytes
09:36:45.042 chunk 17 8322 bytes
09:36:45.042 in Ack 18
09:36:45.049 in Ack 17
09:36:45.190 fetch POST /api/observability/v1/traces
09:36:45.190 fetched POST /api/observability/v1/traces 204
09:36:45.193 rpc 19 server.reportClientActivity
09:36:45.310 chunk 17 12690 bytes
09:36:45.319 in Ack 17
09:36:45.811 chunk 17 17165 bytes
09:36:45.813 in Ack 17
09:36:45.939 exit 19 {"_tag":"Success","value":null}
09:36:46.178 chunk 12 4655 bytes
09:36:46.178 chunk 17 21097 bytes
09:36:46.186 in Ack 12
09:36:46.252 in Ack 17
09:36:46.293 in Interrupt 18
09:36:46.296 rpc 20 subscribeVcsStatus
09:36:46.298 fetch POST /api/observability/v1/traces
09:36:46.299 fetched POST /api/observability/v1/traces 204
09:36:46.404 chunk 20 1632 bytes
09:36:46.405 in Ack 20
09:36:46.430 chunk 12 4594 bytes
09:36:46.431 in Ack 12
09:36:46.547 rpc 21 server.reportClientActivity
09:36:46.628 exit 21 {"_tag":"Success","value":null}
09:36:47.122 rpc 22 attachments.createUploadUrl
09:36:47.363 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-952b2f9a-d386-4bdf-9a07-f663147acb73","expiresAt":1794217607312,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:36:47.365 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTk1MmIyZjlhLWQzODYtNGJkZi05YTA3LWY2NjMxNDdhY2I3MyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzYwNzMxMn0.ANU5eQfnpdpq9_-pnnTUHa8QR4AE1Z7TqcKjvFnkA9k
09:36:47.433 fetch POST /api/observability/v1/traces
09:36:47.433 fetched POST /api/observability/v1/traces 204
09:36:48.153 in Ping 
09:36:48.496 chunk 12 4591 bytes
09:36:48.498 in Ack 12
09:36:49.500 fetch POST /api/observability/v1/traces
09:36:49.500 fetched POST /api/observability/v1/traces 204
09:36:50.779 rpc 23 orchestration.dispatchCommand message.dispatch
09:36:51.256 exit 23 {"_tag":"Success","value":{"sequence":109}}
09:36:51.262 rpc 24 attachments.delete
09:36:51.778 fetch POST /api/observability/v1/traces
09:36:51.778 fetched POST /api/observability/v1/traces 204
09:36:51.828 exit 24 {"_tag":"Success","value":null}
09:36:52.091 chunk 17 23552 bytes
09:36:52.092 in Ack 17
09:36:52.341 chunk 12 4616 bytes
09:36:52.341 chunk 17 23554 bytes
09:36:52.342 in Ack 12
09:36:52.451 in Ack 17
09:36:52.592 chunk 17 27847 bytes
09:36:52.595 in Ack 17
09:36:52.602 rpc 25 assets.createUrl
09:36:52.844 fetch POST /api/observability/v1/traces
09:36:52.844 fetched POST /api/observability/v1/traces 204
09:36:53.100 chunk 17 28633 bytes
09:36:53.102 in Ack 17
09:36:53.157 in Ping 
09:36:53.351 chunk 17 29972 bytes
09:36:53.354 in Ack 17
09:36:53.508 rpc 26 orchestration.dispatchCommand thread.visit
09:36:53.508 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:36:53.601 chunk 17 31900 bytes
09:36:53.603 in Ack 17
09:36:53.850 chunk 12 4613 bytes
09:36:53.851 in Ack 12
09:36:53.853 rpc 27 orchestration.dispatchCommand thread.visit
09:36:53.853 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:36:53.860 chunk 17 31900 bytes
09:36:53.862 in Ack 17
09:36:54.100 chunk 12 4613 bytes
09:36:54.100 in Ack 12
09:36:54.108 fetch POST /api/observability/v1/traces
09:36:54.108 fetched POST /api/observability/v1/traces 204
09:36:54.109 chunk 17 32032 bytes
09:36:54.110 in Ack 17
09:36:54.647 chunk 17 35257 bytes
09:36:54.651 in Ack 17
09:36:54.663 in Interrupt 20
09:36:54.666 rpc 28 subscribeVcsStatus
09:36:54.690 chunk 28 1632 bytes
09:36:54.691 in Ack 28
09:36:54.918 rpc 29 server.reportClientActivity
09:36:55.078 rpc 30 server.refreshProviders
09:36:55.078 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:12.924Z","configurableProviders":false,"con
09:36:55.123 fetch POST /api/observability/v1/traces
09:36:55.123 fetched POST /api/observability/v1/traces 204
09:36:56.544 fetch POST /api/observability/v1/traces
09:36:56.544 fetched POST /api/observability/v1/traces 204
09:36:57.251 exit 29 {"_tag":"Success","value":null}
09:36:57.895 rpc 31 orchestration.dispatchCommand message.dispatch
09:36:58.158 in Ping 
09:36:58.302 fetch POST /api/observability/v1/traces
09:36:58.302 fetched POST /api/observability/v1/traces 204
09:37:00.327 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:37:01.330 fetch POST /api/observability/v1/traces
09:37:01.330 fetched POST /api/observability/v1/traces 204
09:37:03.158 in Ping 
09:37:05.082 fetch POST /api/observability/v1/traces
09:37:05.082 fetched POST /api/observability/v1/traces 204
09:37:06.309 chunk 17 37278 bytes
09:37:06.312 in Ack 17
09:37:07.317 fetch POST /api/observability/v1/traces
09:37:07.318 fetched POST /api/observability/v1/traces 204
09:37:08.160 in Ping 
09:37:08.232 rpc 32 server.reportClientActivity
09:37:09.233 fetch POST /api/observability/v1/traces
09:37:09.233 fetched POST /api/observability/v1/traces 204
09:37:09.871 chunk 12 4677 bytes
09:37:09.872 in Ack 12
09:37:09.874 rpc 33 orchestration.dispatchCommand thread.visit
09:37:09.874 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:10.876 fetch POST /api/observability/v1/traces
09:37:10.877 fetched POST /api/observability/v1/traces 204
09:37:13.160 in Ping 
09:37:14.131 chunk 17 37914 bytes
09:37:14.134 in Ack 17
09:37:15.095 fetch POST /api/observability/v1/traces
09:37:15.095 fetched POST /api/observability/v1/traces 204
09:37:15.429 chunk 17 40673 bytes
09:37:15.431 in Ack 17
09:37:15.681 chunk 17 41061 bytes
09:37:15.684 in Ack 17
09:37:16.432 fetch POST /api/observability/v1/traces
09:37:16.433 fetched POST /api/observability/v1/traces 204
09:37:18.160 in Ping 
09:37:19.509 chunk 17 41327 bytes
09:37:19.511 in Ack 17
09:37:20.512 fetch POST /api/observability/v1/traces
09:37:20.512 fetched POST /api/observability/v1/traces 204
09:37:21.204 chunk 17 41279 bytes
09:37:21.206 in Ack 17
09:37:22.207 fetch POST /api/observability/v1/traces
09:37:22.207 fetched POST /api/observability/v1/traces 204
09:37:23.161 in Ping 
09:37:23.238 in Interrupt 32
09:37:24.239 fetch POST /api/observability/v1/traces
09:37:24.239 fetched POST /api/observability/v1/traces 204
09:37:26.593 chunk 12 4677 bytes
09:37:26.607 in Ack 12
09:37:26.618 rpc 34 orchestration.dispatchCommand thread.visit
09:37:26.618 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:37:27.424 fetch POST /api/observability/v1/traces
09:37:27.424 fetched POST /api/observability/v1/traces 204
09:37:28.161 in Ping 
09:37:31.506 chunk 17 41675 bytes
09:37:31.508 in Ack 17
09:37:32.152 chunk 17 42109 bytes
09:37:32.155 in Ack 17
09:37:32.509 fetch POST /api/observability/v1/traces
09:37:32.509 fetched POST /api/observability/v1/traces 204
09:37:32.827 chunk 17 42475 bytes
09:37:32.830 in Ack 17
09:37:33.161 in Ping 
09:37:33.233 rpc 35 server.reportClientActivity
09:37:33.534 chunk 17 42861 bytes
09:37:33.536 in Ack 17
09:37:33.831 fetch POST /api/observability/v1/traces
09:37:33.831 fetched POST /api/observability/v1/traces 204
09:37:34.197 chunk 17 42967 bytes
09:37:34.200 in Ack 17
09:37:35.207 fetch POST /api/observability/v1/traces
09:37:35.207 fetched POST /api/observability/v1/traces 204
09:37:37.427 fetch POST /api/observability/v1/traces
09:37:37.427 fetched POST /api/observability/v1/traces 204
09:37:38.164 in Ping 
09:37:43.168 in Ping 
09:37:48.172 in Ping 
09:37:48.234 in Interrupt 35
09:37:49.235 fetch POST /api/observability/v1/traces
09:37:49.236 fetched POST /api/observability/v1/traces 204
09:37:53.177 in Ping 
09:37:58.180 in Ping 
09:37:58.234 rpc 36 server.reportClientActivity
09:37:59.234 fetch POST /api/observability/v1/traces
09:37:59.234 fetched POST /api/observability/v1/traces 204
09:38:03.184 in Ping 
09:38:03.426 chunk 17 43969 bytes
09:38:03.429 in Ack 17
09:38:03.701 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:04.430 fetch POST /api/observability/v1/traces
09:38:04.430 fetched POST /api/observability/v1/traces 204
09:38:07.722 exit 37 {"_tag":"Success","value":{"sequence":208}}
09:38:08.188 in Ping 
09:38:08.754 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:08.770 fetch POST /api/observability/v1/traces
09:38:08.770 fetched POST /api/observability/v1/traces 204
09:38:12.559 chunk 17 44012 bytes
09:38:12.559 exit 38 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:12.561 in Ack 17
09:38:13.192 in Ping 
09:38:13.239 in Interrupt 36
09:38:13.563 fetch POST /api/observability/v1/traces
09:38:13.563 fetched POST /api/observability/v1/traces 204
09:38:14.788 chunk 17 46973 bytes
09:38:14.790 in Ack 17
09:38:15.791 fetch POST /api/observability/v1/traces
09:38:15.791 fetched POST /api/observability/v1/traces 204
09:38:16.880 chunk 17 47247 bytes
09:38:16.883 in Ack 17
09:38:17.884 fetch POST /api/observability/v1/traces
09:38:17.884 fetched POST /api/observability/v1/traces 204
09:38:18.196 in Ping 
09:38:21.169 chunk 17 47591 bytes
09:38:21.171 in Ack 17
09:38:21.630 chunk 17 47669 bytes
09:38:21.632 in Ack 17
09:38:22.173 fetch POST /api/observability/v1/traces
09:38:22.173 fetched POST /api/observability/v1/traces 204
09:38:23.200 in Ping 
09:38:23.234 rpc 39 server.reportClientActivity
09:38:24.234 fetch POST /api/observability/v1/traces
09:38:24.234 fetched POST /api/observability/v1/traces 204
09:38:26.420 chunk 17 47925 bytes
09:38:26.424 in Ack 17
09:38:27.588 fetch POST /api/observability/v1/traces
09:38:27.588 fetched POST /api/observability/v1/traces 204
