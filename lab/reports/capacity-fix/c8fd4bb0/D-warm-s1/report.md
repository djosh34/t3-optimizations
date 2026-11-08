# D, webapp, warm start, seed 1

Profile D (graded): worst observed EDGE, inspired envelope. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 75 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 8.5s |
| Prompt reached the agent | 4.1s |
| Time to first text | 9.0s |
| Whole reply shown | 74.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 16.1s / 5.2s / 12.1s |
| Longest silence | 23.2s |
| Approval shown after it was asked | 15.2s |
| Bytes on the wire down / up | 244984 / 92967 |
| Bytes the app received / sent | 166361 / 48925 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.7 | 46 | 234 |
| 20 | 11.1 | 189 | 632 |
| 30 | 5.0 | 835 | 1037 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 9.2 | 1077 | 1431 |
| 70 | 9.6 | 1446 | 1596 |
| 74 | 9.4 | 1590 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 14/776","providerThreads":"668/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 192/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175082290,"silences":0,"longestSilenceMs":1184,"longestRecoveredGapMs":1184},"stats":{"sendRate":2250,"deliveredRate":1854.677565849228,"loss":0,"srttMs":43.064579348525776,"minRttMs":1,"sentPackets":94,"sentWireBytes":30245,"receivedPackets":103,"receivedWireBytes":48505,"lastHeardMs":8422,"unsyncedBytes":993,"inflightPackets":9,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 189/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 70/70","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175092941,"silences":1,"longestSilenceMs":2967,"longestRecoveredGapMs":2967},"stats":{"sendRate":1299.3822481878994,"deliveredRate":1732.5096642505325,"loss":0,"srttMs":4233.318190367793,"minRttMs":2933,"sentPackets":110,"sentWireBytes":38189,"receivedPackets":151,"receivedWireBytes":70295,"lastHeardMs":19073,"unsyncedBytes":0,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 189/849 288/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 370/608","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175103434,"silences":2,"longestSilenceMs":2967,"longestRecoveredGapMs":2967},"stats":{"sendRate":1299.3822481878994,"deliveredRate":134.29124067421938,"loss":0.6363636363636364,"srttMs":5520.743614500341,"minRttMs":2933,"sentPackets":144,"sentWireBytes":45951,"receivedPackets":171,"receivedWireBytes":80492,"lastHeardMs":29566,"unsyncedBytes":484,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1063/1063","user":"10/10 9/9","attempts":"661/661 661/661 59/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175113564,"silences":3,"longestSilenceMs":2967,"longestRecoveredGapMs":2967},"stats":{"sendRate":649.6911240939497,"deliveredRate":1217.5647454182101,"loss":0.27707272727272725,"srttMs":8599.295767005186,"minRttMs":2933,"sentPackets":172,"sentWireBytes":50987,"receivedPackets":214,"receivedWireBytes":96187,"lastHeardMs":39697,"unsyncedBytes":0,"inflightPackets":17,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078 334/1550","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"661/661 661/661 106/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718","runtimeRequests":"844/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175123194,"silences":3,"longestSilenceMs":2967,"longestRecoveredGapMs":2967},"stats":{"sendRate":649.6911240939497,"deliveredRate":857.405560698514,"loss":0.23576563636363634,"srttMs":11262.015555785538,"minRttMs":2933,"sentPackets":215,"sentWireBytes":57096,"receivedPackets":274,"receivedWireBytes":117644,"lastHeardMs":49327,"unsyncedBytes":0,"inflightPackets":10,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1550/1550 929/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"661/661 661/661 170/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175134351,"silences":3,"longestSilenceMs":2967,"longestRecoveredGapMs":2967},"stats":{"sendRate":649.6911240939497,"deliveredRate":561.1496339583136,"loss":0.1972521008403361,"srttMs":10086.586022812764,"minRttMs":2933,"sentPackets":240,"sentWireBytes":65104,"receivedPackets":331,"receivedWireBytes":149098,"lastHeardMs":60484,"unsyncedBytes":400,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1550/1550 929/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340 298/363","user":"10/10 9/9","attempts":"661/661 661/661 291/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175145384,"silences":4,"longestSilenceMs":6740,"longestRecoveredGapMs":6740},"stats":{"sendRate":487.26834307046227,"deliveredRate":527.2188062708195,"loss":0.18807647058823523,"srttMs":10016.354923716022,"minRttMs":2933,"sentPackets":257,"sentWireBytes":70805,"receivedPackets":379,"receivedWireBytes":172159,"lastHeardMs":71516,"unsyncedBytes":400,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1550/1550 929/929 1103/1103","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340 723/723","user":"10/10 9/9","attempts":"661/661 661/661 291/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2680/2680","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 423/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"718/718","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794175156331,"silences":4,"longestSilenceMs":6740,"longestRecoveredGapMs":6740},"stats":{"sendRate":487.26834307046227,"deliveredRate":345.6690810732589,"loss":0.413975652406417,"srttMs":9716.220745078655,"minRttMs":8067,"sentPackets":271,"sentWireBytes":76239,"receivedPackets":449,"receivedWireBytes":209832,"lastHeardMs":82464,"unsyncedBytes":484,"inflightPackets":28,"duplicateBytes":0}}
- shim trace
21:57:54.178 fetch GET /api/auth/session
21:57:54.179 fetched GET /api/auth/session 200
21:57:54.319 fetch GET /.well-known/t3/environment
21:57:54.319 fetched GET /.well-known/t3/environment 200
21:57:54.340 fetch GET /.well-known/t3/environment
21:57:54.348 fetched GET /.well-known/t3/environment 200
21:57:54.352 rpc 0 subscribeServerConfig
21:57:54.353 chunk 0 38658 bytes
21:57:54.369 in Ack 0
21:57:54.371 rpc 1 subscribeServerLifecycle
21:57:54.371 rpc 2 subscribeServerLifecycle
21:57:54.373 fetch GET /api/auth/session
21:57:54.374 fetched GET /api/auth/session 200
21:57:54.374 rpc 3 subscribeDeviceState
21:57:54.376 rpc 4 preview.reportProfiles
21:57:54.413 rpc 5 subscribeProjectClones
21:57:54.431 fetch GET /api/orchestration/threads/mcp%3A56388cdb-3335-4a52-882f-ce78f7855d04/bounded
21:57:54.435 fetch GET /api/orchestration/shell
21:57:54.436 fetched GET /api/orchestration/shell 200
21:57:54.438 rpc 6 orchestration.subscribeShell
21:57:54.438 chunk 6 2774 bytes
21:57:54.458 rpc 7 server.refreshProviders
21:57:54.495 rpc 8 orchestration.dispatchCommand thread.visit
21:57:54.495 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:57:54.509 rpc 9 subscribeTerminalMetadata
21:57:54.510 rpc 10 subscribeVcsStatus
21:57:54.511 rpc 11 subscribePreviewEvents
21:57:54.512 rpc 12 assets.createUrl
21:57:54.512 rpc 13 preview.list
21:57:54.513 rpc 14 projects.readFile
21:57:54.515 in Ack 6
21:57:54.661 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"a64a1985-9897-4645-a03c-f84f5987379e","sessions":[]}}
21:57:54.661 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:57:54.688 chunk 6 2750 bytes
21:57:54.688 in Ack 6
21:57:54.763 rpc 15 server.reportClientActivity
21:57:54.798 chunk 1 3856 bytes
21:57:54.799 in Ack 1
21:57:55.050 chunk 2 3856 bytes
21:57:55.050 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:57:42.048Z","configurableProviders":false,"con
21:57:55.050 chunk 10 472 bytes
21:57:55.051 in Ack 2
21:57:55.053 in Ack 10
21:57:55.180 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:57:55.317 fetch POST /api/observability/v1/traces
21:57:55.317 fetched POST /api/observability/v1/traces 204
21:57:55.378 chunk 3 876 bytes
21:57:55.378 exit 4 {"_tag":"Success","value":null}
21:57:55.378 chunk 5 4 bytes
21:57:55.378 chunk 9 36 bytes
21:57:55.378 exit 15 {"_tag":"Success","value":null}
21:57:55.379 in Ack 3
21:57:55.382 rpc 16 preview.reportProfiles
21:57:55.382 in Ack 5
21:57:55.383 in Ack 9
21:57:55.433 chunk 6 2793 bytes
21:57:55.433 in Ack 6
21:57:55.449 exit 16 {"_tag":"Success","value":null}
21:57:55.449 fetched GET /api/orchestration/threads/mcp%3A56388cdb-3335-4a52-882f-ce78f7855d04/bounded 200
21:57:55.452 rpc 17 orchestration.subscribeThread
21:57:55.452 chunk 17 8294 bytes
21:57:55.461 in Interrupt 10
21:57:55.462 rpc 18 subscribeVcsStatus
21:57:55.463 in Ack 17
21:57:55.598 chunk 18 472 bytes
21:57:55.598 in Ack 18
21:57:55.704 chunk 17 11932 bytes
21:57:55.705 in Ack 17
21:57:55.760 rpc 19 server.reportClientActivity
21:57:55.928 exit 19 {"_tag":"Success","value":null}
21:57:56.000 chunk 17 13517 bytes
21:57:56.002 in Ack 17
21:57:56.269 rpc 20 attachments.createUploadUrl
21:57:56.429 chunk 17 18029 bytes
21:57:56.434 fetch POST /api/observability/v1/traces
21:57:56.435 fetched POST /api/observability/v1/traces 204
21:57:56.438 in Ack 17
21:57:56.518 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-09ecbbcf-08e5-455e-9120-b9940cd438e4","expiresAt":1794175676459,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:57:56.519 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA5ZWNiYmNmLTA4ZTUtNDU1ZS05MTIwLWI5OTQwY2Q0MzhlNCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTY3NjQ1OX0.bxomSeOcgi99KCW_vTtf3Rg176u5lqWJhPDlT8pAgXs
21:57:56.680 chunk 17 21187 bytes
21:57:56.681 in Ack 17
21:57:56.688 in Interrupt 18
21:57:56.689 rpc 21 subscribeVcsStatus
21:57:56.737 chunk 21 472 bytes
21:57:56.738 in Ack 21
21:57:56.940 rpc 22 server.reportClientActivity
21:57:57.439 fetch POST /api/observability/v1/traces
21:57:57.439 fetched POST /api/observability/v1/traces 204
21:57:57.601 exit 22 {"_tag":"Success","value":null}
21:57:58.602 fetch POST /api/observability/v1/traces
21:57:58.602 fetched POST /api/observability/v1/traces 204
21:57:59.436 rpc 23 orchestration.dispatchCommand message.dispatch
21:57:59.981 in Ping 
21:58:00.029 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:58:00.032 rpc 24 attachments.delete
21:58:00.435 fetch POST /api/observability/v1/traces
21:58:00.435 fetched POST /api/observability/v1/traces 204
21:58:00.529 chunk 17 23632 bytes
21:58:00.530 in Ack 17
21:58:00.779 chunk 17 23664 bytes
21:58:00.780 in Ack 17
21:58:01.029 chunk 17 23666 bytes
21:58:01.030 in Ack 17
21:58:01.279 chunk 6 2815 bytes
21:58:01.280 in Ack 6
21:58:01.281 rpc 25 orchestration.dispatchCommand thread.visit
21:58:01.281 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:58:01.529 chunk 6 2815 bytes
21:58:01.530 chunk 17 27431 bytes
21:58:01.530 in Ack 6
21:58:01.536 in Ack 17
21:58:01.538 rpc 26 assets.createUrl
21:58:01.571 fetch POST /api/observability/v1/traces
21:58:01.571 fetched POST /api/observability/v1/traces 204
21:58:01.717 exit 24 {"_tag":"Success","value":null}
21:58:01.780 chunk 17 30516 bytes
21:58:01.781 in Ack 17
21:58:02.030 chunk 17 32106 bytes
21:58:02.032 in Ack 17
21:58:02.040 in Interrupt 21
21:58:02.042 rpc 27 subscribeVcsStatus
21:58:02.290 chunk 27 472 bytes
21:58:02.290 chunk 17 31940 bytes
21:58:02.291 in Ack 27
21:58:02.294 in Ack 17
21:58:02.302 rpc 28 server.reportClientActivity
21:58:02.721 fetch POST /api/observability/v1/traces
21:58:02.721 fetched POST /api/observability/v1/traces 204
21:58:03.196 rpc 29 orchestration.dispatchCommand message.dispatch
21:58:04.196 fetch POST /api/observability/v1/traces
21:58:04.196 fetched POST /api/observability/v1/traces 204
21:58:04.981 in Ping 
21:58:05.258 chunk 17 31940 bytes
21:58:05.260 in Ack 17
21:58:05.532 chunk 17 32717 bytes
21:58:05.534 in Ack 17
21:58:05.944 fetch POST /api/observability/v1/traces
21:58:05.944 fetched POST /api/observability/v1/traces 204
21:58:07.018 fetch POST /api/observability/v1/traces
21:58:07.018 fetched POST /api/observability/v1/traces 204
21:58:07.456 chunk 17 33287 bytes
21:58:07.457 in Ack 17
21:58:07.706 chunk 17 34310 bytes
21:58:07.707 in Ack 17
21:58:08.458 fetch POST /api/observability/v1/traces
21:58:08.458 fetched POST /api/observability/v1/traces 204
21:58:09.982 in Ping 
21:58:11.438 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:58:11.689 chunk 17 36755 bytes
21:58:11.690 in Ack 17
21:58:11.940 chunk 17 36785 bytes
21:58:11.941 in Ack 17
21:58:12.240 chunk 17 36805 bytes
21:58:12.241 in Ack 17
21:58:12.439 fetch POST /api/observability/v1/traces
21:58:12.439 fetched POST /api/observability/v1/traces 204
21:58:12.491 chunk 17 36835 bytes
21:58:12.491 in Ack 17
21:58:12.848 chunk 17 36855 bytes
21:58:12.849 in Ack 17
21:58:13.098 chunk 17 36885 bytes
21:58:13.099 in Ack 17
21:58:13.441 chunk 17 36905 bytes
21:58:13.442 in Ack 17
21:58:13.492 fetch POST /api/observability/v1/traces
21:58:13.492 fetched POST /api/observability/v1/traces 204
21:58:13.691 chunk 17 36935 bytes
21:58:13.692 in Ack 17
21:58:14.434 chunk 17 36965 bytes
21:58:14.435 in Ack 17
21:58:14.684 chunk 17 36985 bytes
21:58:14.686 in Ack 17
21:58:14.696 fetch POST /api/observability/v1/traces
21:58:14.696 fetched POST /api/observability/v1/traces 204
21:58:14.934 chunk 17 37065 bytes
21:58:14.935 in Ack 17
21:58:14.982 in Ping 
21:58:15.184 chunk 17 37115 bytes
21:58:15.185 in Ack 17
21:58:15.894 chunk 17 37135 bytes
21:58:15.895 in Ack 17
21:58:15.936 fetch POST /api/observability/v1/traces
21:58:15.936 fetched POST /api/observability/v1/traces 204
21:58:16.145 chunk 17 37165 bytes
21:58:16.146 in Ack 17
21:58:16.395 chunk 17 37215 bytes
21:58:16.396 in Ack 17
21:58:17.030 fetch POST /api/observability/v1/traces
21:58:17.030 fetched POST /api/observability/v1/traces 204
21:58:17.302 in Interrupt 28
21:58:18.303 fetch POST /api/observability/v1/traces
21:58:18.303 fetched POST /api/observability/v1/traces 204
21:58:19.213 chunk 17 37247 bytes
21:58:19.216 in Ack 17
21:58:19.570 rpc 30 server.reportClientActivity
21:58:19.982 in Ping 
21:58:20.001 chunk 17 37277 bytes
21:58:20.002 in Ack 17
21:58:20.217 fetch POST /api/observability/v1/traces
21:58:20.217 fetched POST /api/observability/v1/traces 204
21:58:20.690 chunk 17 37299 bytes
21:58:20.691 in Ack 17
21:58:21.612 chunk 17 37321 bytes
21:58:21.614 in Ack 17
21:58:21.692 fetch POST /api/observability/v1/traces
21:58:21.692 fetched POST /api/observability/v1/traces 204
21:58:21.863 chunk 17 37371 bytes
21:58:21.864 in Ack 17
21:58:22.544 chunk 17 37433 bytes
21:58:22.545 in Ack 17
21:58:22.865 fetch POST /api/observability/v1/traces
21:58:22.865 fetched POST /api/observability/v1/traces 204
21:58:23.436 chunk 17 37491 bytes
21:58:23.437 in Ack 17
21:58:23.751 chunk 17 37511 bytes
21:58:23.753 in Ack 17
21:58:24.001 chunk 17 37531 bytes
21:58:24.002 in Ack 17
21:58:24.252 chunk 17 37551 bytes
21:58:24.253 in Ack 17
21:58:24.438 fetch POST /api/observability/v1/traces
21:58:24.438 fetched POST /api/observability/v1/traces 204
21:58:24.982 in Ping 
21:58:26.479 chunk 17 37573 bytes
21:58:26.480 in Ack 17
21:58:27.033 fetch POST /api/observability/v1/traces
21:58:27.033 fetched POST /api/observability/v1/traces 204
21:58:27.274 chunk 17 37593 bytes
21:58:27.275 in Ack 17
21:58:27.524 chunk 17 37643 bytes
21:58:27.525 in Ack 17
21:58:27.775 chunk 17 37873 bytes
21:58:27.776 in Ack 17
21:58:28.092 chunk 17 38111 bytes
21:58:28.093 in Ack 17
21:58:28.276 fetch POST /api/observability/v1/traces
21:58:28.276 fetched POST /api/observability/v1/traces 204
21:58:28.341 chunk 17 38375 bytes
21:58:28.342 in Ack 17
21:58:28.592 chunk 17 38347 bytes
21:58:28.592 in Ack 17
21:58:28.919 chunk 17 38367 bytes
21:58:28.921 in Ack 17
21:58:29.169 chunk 17 38417 bytes
21:58:29.170 in Ack 17
21:58:29.347 fetch POST /api/observability/v1/traces
21:58:29.347 fetched POST /api/observability/v1/traces 204
21:58:29.420 chunk 17 38437 bytes
21:58:29.421 in Ack 17
21:58:29.961 chunk 17 38457 bytes
21:58:29.962 in Ack 17
21:58:29.991 in Ping 
21:58:30.211 chunk 17 38487 bytes
21:58:30.212 in Ack 17
21:58:30.422 fetch POST /api/observability/v1/traces
21:58:30.422 fetched POST /api/observability/v1/traces 204
21:58:30.461 chunk 17 38507 bytes
21:58:30.462 in Ack 17
21:58:31.288 chunk 17 38509 bytes
21:58:31.290 in Ack 17
21:58:31.463 fetch POST /api/observability/v1/traces
21:58:31.463 fetched POST /api/observability/v1/traces 204
21:58:31.675 chunk 17 38529 bytes
21:58:31.676 in Ack 17
21:58:31.925 chunk 17 38549 bytes
21:58:31.927 in Ack 17
21:58:32.176 chunk 17 38707 bytes
21:58:32.176 in Ack 17
21:58:32.565 chunk 17 38767 bytes
21:58:32.570 in Ack 17
21:58:32.677 fetch POST /api/observability/v1/traces
21:58:32.677 fetched POST /api/observability/v1/traces 204
21:58:32.815 chunk 17 38777 bytes
21:58:32.817 in Ack 17
21:58:33.231 chunk 17 38787 bytes
21:58:33.232 in Ack 17
21:58:33.481 chunk 17 38817 bytes
21:58:33.482 in Ack 17
21:58:33.731 chunk 17 38867 bytes
21:58:33.732 in Ack 17
21:58:33.818 fetch POST /api/observability/v1/traces
21:58:33.818 fetched POST /api/observability/v1/traces 204
21:58:33.981 chunk 17 38897 bytes
21:58:33.982 in Ack 17
21:58:34.231 chunk 17 38937 bytes
21:58:34.233 in Ack 17
21:58:34.552 chunk 17 38957 bytes
21:58:34.553 in Ack 17
21:58:34.581 in Interrupt 30
21:58:34.802 chunk 17 38967 bytes
21:58:34.803 in Ack 17
21:58:34.983 fetch POST /api/observability/v1/traces
21:58:34.983 fetched POST /api/observability/v1/traces 204
21:58:34.992 in Ping 
21:58:35.052 chunk 17 38987 bytes
21:58:35.053 in Ack 17
21:58:35.302 chunk 17 39027 bytes
21:58:35.303 in Ack 17
21:58:35.553 chunk 17 39047 bytes
21:58:35.554 in Ack 17
21:58:35.934 chunk 17 39099 bytes
21:58:35.935 in Ack 17
21:58:36.054 fetch POST /api/observability/v1/traces
21:58:36.054 fetched POST /api/observability/v1/traces 204
21:58:36.184 chunk 17 39109 bytes
21:58:36.185 in Ack 17
21:58:36.434 chunk 17 39139 bytes
21:58:36.435 in Ack 17
21:58:36.685 chunk 17 39149 bytes
21:58:36.686 in Ack 17
21:58:37.186 fetch POST /api/observability/v1/traces
21:58:37.186 fetched POST /api/observability/v1/traces 204
21:58:37.299 chunk 17 39159 bytes
21:58:37.300 in Ack 17
21:58:37.549 chunk 17 39219 bytes
21:58:37.551 in Ack 17
21:58:37.799 chunk 17 39271 bytes
21:58:37.800 in Ack 17
21:58:38.050 chunk 17 39281 bytes
21:58:38.051 in Ack 17
21:58:38.300 chunk 17 39301 bytes
21:58:38.301 in Ack 17
21:58:38.310 fetch POST /api/observability/v1/traces
21:58:38.310 fetched POST /api/observability/v1/traces 204
21:58:38.741 chunk 17 39347 bytes
21:58:38.742 in Ack 17
21:58:39.743 fetch POST /api/observability/v1/traces
21:58:39.743 fetched POST /api/observability/v1/traces 204
21:58:39.992 in Ping 
21:58:40.556 chunk 17 39385 bytes
21:58:40.557 in Ack 17
21:58:41.558 fetch POST /api/observability/v1/traces
21:58:41.558 fetched POST /api/observability/v1/traces 204
21:58:44.578 rpc 31 server.reportClientActivity
21:58:44.992 in Ping 
21:58:45.577 fetch POST /api/observability/v1/traces
21:58:45.577 fetched POST /api/observability/v1/traces 204
21:58:47.043 fetch POST /api/observability/v1/traces
21:58:47.043 fetched POST /api/observability/v1/traces 204
21:58:49.122 chunk 17 40393 bytes
21:58:49.123 in Ack 17
21:58:49.407 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:58:49.994 in Ping 
21:58:50.124 fetch POST /api/observability/v1/traces
21:58:50.124 fetched POST /api/observability/v1/traces 204
21:58:50.556 chunk 17 43898 bytes
21:58:50.557 in Ack 17
21:58:51.110 chunk 17 45980 bytes
21:58:51.111 in Ack 17
21:58:51.558 fetch POST /api/observability/v1/traces
21:58:51.558 fetched POST /api/observability/v1/traces 204
21:58:52.536 exit 28 {"_tag":"Success","value":null}
21:58:52.970 chunk 17 46030 bytes
21:58:52.971 in Ack 17
21:58:53.972 fetch POST /api/observability/v1/traces
21:58:53.972 fetched POST /api/observability/v1/traces 204
21:58:54.998 in Ping 
21:58:57.045 fetch POST /api/observability/v1/traces
21:58:57.045 fetched POST /api/observability/v1/traces 204
21:58:59.579 in Interrupt 31
21:59:00.012 in Ping 
21:59:00.580 fetch POST /api/observability/v1/traces
21:59:00.580 fetched POST /api/observability/v1/traces 204
21:59:02.885 chunk 17 48478 bytes
21:59:02.885 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:59:02.887 in Ack 17
21:59:03.136 chunk 17 48614 bytes
21:59:03.137 in Ack 17
21:59:03.386 chunk 17 48654 bytes
21:59:03.388 in Ack 17
21:59:03.637 chunk 17 48684 bytes
21:59:03.638 in Ack 17
21:59:03.887 chunk 17 48774 bytes
21:59:03.888 in Ack 17
21:59:03.897 fetch POST /api/observability/v1/traces
21:59:03.897 fetched POST /api/observability/v1/traces 204
21:59:03.988 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:59:04.137 chunk 17 48804 bytes
21:59:04.138 in Ack 17
21:59:04.388 chunk 17 48924 bytes
21:59:04.389 in Ack 17
21:59:04.638 chunk 17 48964 bytes
21:59:04.639 in Ack 17
21:59:04.888 chunk 17 48994 bytes
21:59:04.889 in Ack 17
21:59:04.988 fetch POST /api/observability/v1/traces
21:59:04.988 fetched POST /api/observability/v1/traces 204
21:59:05.013 in Ping 
21:59:05.139 chunk 17 49024 bytes
21:59:05.140 in Ack 17
21:59:05.389 chunk 17 49064 bytes
21:59:05.390 in Ack 17
21:59:05.638 chunk 17 49084 bytes
21:59:05.640 in Ack 17
21:59:05.889 chunk 17 49124 bytes
21:59:05.890 in Ack 17
21:59:06.143 fetch POST /api/observability/v1/traces
21:59:06.143 fetched POST /api/observability/v1/traces 204
21:59:06.294 chunk 17 49134 bytes
21:59:06.295 in Ack 17
21:59:06.545 chunk 17 49156 bytes
21:59:06.546 in Ack 17
21:59:06.795 chunk 17 49182 bytes
21:59:06.797 in Ack 17
21:59:07.046 chunk 17 49212 bytes
21:59:07.047 in Ack 17
21:59:07.296 fetch POST /api/observability/v1/traces
21:59:07.296 fetched POST /api/observability/v1/traces 204
21:59:07.390 chunk 17 49222 bytes
21:59:07.392 in Ack 17
21:59:07.640 chunk 17 49370 bytes
21:59:07.641 in Ack 17
21:59:07.891 chunk 17 49416 bytes
21:59:07.892 in Ack 17
21:59:08.140 chunk 17 49454 bytes
21:59:08.142 in Ack 17
21:59:08.391 chunk 17 49500 bytes
21:59:08.393 in Ack 17
21:59:08.459 fetch POST /api/observability/v1/traces
21:59:08.459 fetched POST /api/observability/v1/traces 204
21:59:08.642 chunk 17 49532 bytes
21:59:08.643 in Ack 17
21:59:08.892 chunk 17 49546 bytes
21:59:08.893 in Ack 17
21:59:09.270 chunk 17 49566 bytes
21:59:09.271 in Ack 17
21:59:09.572 rpc 34 server.reportClientActivity
21:59:09.644 fetch POST /api/observability/v1/traces
21:59:09.644 fetched POST /api/observability/v1/traces 204
21:59:09.758 chunk 17 49594 bytes
21:59:09.759 in Ack 17
21:59:10.009 chunk 17 49614 bytes
21:59:10.010 in Ack 17
21:59:10.023 in Ping 
21:59:10.259 chunk 17 49624 bytes
21:59:10.260 in Ack 17
21:59:10.509 chunk 17 49644 bytes
21:59:10.510 in Ack 17
21:59:10.759 chunk 17 49682 bytes
21:59:10.761 in Ack 17
21:59:10.823 fetch POST /api/observability/v1/traces
21:59:10.823 fetched POST /api/observability/v1/traces 204
21:59:12.295 chunk 17 49918 bytes
21:59:12.296 in Ack 17
21:59:12.545 chunk 17 49944 bytes
21:59:12.546 in Ack 17
21:59:13.297 fetch POST /api/observability/v1/traces
21:59:13.297 fetched POST /api/observability/v1/traces 204
21:59:14.254 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:59:14.476 chunk 17 49987 bytes
21:59:14.478 in Ack 17
21:59:14.726 chunk 17 49989 bytes
21:59:14.727 in Ack 17
21:59:15.025 in Ping 
21:59:15.260 fetch POST /api/observability/v1/traces
21:59:15.260 fetched POST /api/observability/v1/traces 204
21:59:16.166 chunk 17 49991 bytes
21:59:16.168 in Ack 17
21:59:16.416 chunk 17 50011 bytes
21:59:16.417 in Ack 17
21:59:17.051 fetch POST /api/observability/v1/traces
21:59:17.051 fetched POST /api/observability/v1/traces 204
