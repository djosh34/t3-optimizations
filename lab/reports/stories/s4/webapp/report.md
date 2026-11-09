# story, webapp, warm start, seed 4

Profile story (graded): seed 4's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 54 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.3s |
| Prompt reached the agent | 5.1s |
| Time to first text | 16.2s |
| Whole reply shown | 51.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.8s / 5.0s / 14.8s |
| Longest silence | 17.0s |
| Approval shown after it was asked | 2.4s |
| Bytes on the wire down / up | 359406 / 129529 |
| Bytes the app received / sent | 253159 / 70897 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.8 | 0 | 193 |
| 20 | 4.8 | 397 | 593 |
| 30 | 14.8 | 397 | 996 |
| 40 | 4.0 | 1077 | 1237 |
| 50 | 6.3 | 1378 | 1596 |
| 51 | 7.3 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","messages":"456/456 544/544 901/901 544/544","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":731,"lastDataAt":1794216984639,"silences":0,"longestSilenceMs":1328,"longestRecoveredGapMs":1328},"stats":{"sendRate":3000,"deliveredRate":1680.7445937090429,"loss":0,"srttMs":254.18686141946705,"minRttMs":1,"sentPackets":154,"sentWireBytes":41011,"receivedPackets":205,"receivedWireBytes":113173,"lastHeardMs":19704,"unsyncedBytes":1107,"inflightPackets":15,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","messages":"456/456 544/544 901/901 544/544 451/451","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":731,"lastDataAt":1794216994818,"silences":0,"longestSilenceMs":1328,"longestRecoveredGapMs":1328},"stats":{"sendRate":843.75,"deliveredRate":1429.0780141843973,"loss":0,"srttMs":1569.7029058224198,"minRttMs":1,"sentPackets":203,"sentWireBytes":52847,"receivedPackets":312,"receivedWireBytes":160882,"lastHeardMs":29883,"unsyncedBytes":45,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","messages":"456/456 544/544 901/901 544/544 451/451 543/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":731,"lastDataAt":1794217003787,"silences":0,"longestSilenceMs":1328,"longestRecoveredGapMs":1328},"stats":{"sendRate":2143.617021276596,"deliveredRate":1486.5211810012836,"loss":0,"srttMs":1741.835884147583,"minRttMs":1652,"sentPackets":253,"sentWireBytes":60943,"receivedPackets":445,"receivedWireBytes":239881,"lastHeardMs":38852,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","messages":"456/456 544/544 901/901 544/544 451/451 543/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":731,"lastDataAt":1794217014935,"silences":1,"longestSilenceMs":2288,"longestRecoveredGapMs":2288},"stats":{"sendRate":2143.617021276596,"deliveredRate":944.4851044736818,"loss":0.09000000000000001,"srttMs":1811.8563986291351,"minRttMs":1652,"sentPackets":268,"sentWireBytes":62785,"receivedPackets":470,"receivedWireBytes":246857,"lastHeardMs":50000,"unsyncedBytes":0,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","messages":"456/456 544/544 901/901 544/544 451/451 544/544","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 983/983 1392/1392 788/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":731,"lastDataAt":1794217025996,"silences":1,"longestSilenceMs":2288,"longestRecoveredGapMs":2288},"stats":{"sendRate":3976.940543025366,"deliveredRate":2651.293695350244,"loss":0,"srttMs":1792.6593383742452,"minRttMs":1652,"sentPackets":358,"sentWireBytes":90495,"receivedPackets":555,"receivedWireBytes":285998,"lastHeardMs":61062,"unsyncedBytes":589,"inflightPackets":27,"duplicateBytes":0}}
- shim trace
09:36:05.789 fetch GET /api/auth/session
09:36:05.792 fetched GET /api/auth/session 200
09:36:06.441 fetch GET /.well-known/t3/environment
09:36:06.443 fetched GET /.well-known/t3/environment 200
09:36:06.470 fetch GET /.well-known/t3/environment
09:36:06.472 fetched GET /.well-known/t3/environment 200
09:36:06.480 rpc 0 subscribeServerConfig
09:36:06.481 chunk 0 38458 bytes
09:36:06.551 in Ack 0
09:36:06.556 rpc 1 subscribeServerLifecycle
09:36:06.558 rpc 2 subscribeServerLifecycle
09:36:06.563 fetch GET /api/auth/session
09:36:06.566 fetched GET /api/auth/session 200
09:36:06.566 rpc 3 subscribeDeviceState
09:36:06.572 rpc 4 preview.reportProfiles
09:36:06.691 rpc 5 subscribeProjectClones
09:36:06.802 rpc 6 server.reportClientActivity
09:36:06.813 fetch GET /api/orchestration/threads/mcp%3A41edb4a9-ca49-4c43-a9b0-ac6315dd6487/bounded
09:36:06.824 fetch GET /api/orchestration/shell
09:36:06.883 rpc 7 server.refreshProviders
09:36:07.008 rpc 8 orchestration.dispatchCommand thread.visit
09:36:07.008 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:36:07.044 fetched GET /api/orchestration/shell 200
09:36:07.045 rpc 9 subscribeTerminalMetadata
09:36:07.046 rpc 10 subscribeVcsStatus
09:36:07.048 rpc 11 subscribePreviewEvents
09:36:07.053 rpc 12 orchestration.subscribeShell
09:36:07.053 chunk 12 4550 bytes
09:36:07.058 rpc 13 assets.createUrl
09:36:07.059 rpc 14 preview.list
09:36:07.060 rpc 15 projects.readFile
09:36:07.079 in Ack 12
09:36:07.487 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"6df664a6-b260-46b4-86d1-a5815af5eaac","sessions":[]}}
09:36:07.488 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:36:07.488 chunk 12 4526 bytes
09:36:07.509 fetch POST /api/observability/v1/traces
09:36:07.510 fetched POST /api/observability/v1/traces 204
09:36:07.592 chunk 1 3814 bytes
09:36:07.604 in Ack 12
09:36:07.618 in Ack 1
09:36:07.993 chunk 2 3814 bytes
09:36:07.994 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:35:36.323Z","configurableProviders":false,"con
09:36:07.994 chunk 10 1632 bytes
09:36:07.994 in Ack 2
09:36:08.001 in Ack 10
09:36:08.187 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:36:08.437 chunk 12 4551 bytes
09:36:08.438 in Ack 12
09:36:08.594 fetch POST /api/observability/v1/traces
09:36:08.594 fetched POST /api/observability/v1/traces 204
09:36:08.687 chunk 12 4594 bytes
09:36:08.688 in Ack 12
09:36:08.848 chunk 3 876 bytes
09:36:08.848 exit 4 {"_tag":"Success","value":null}
09:36:08.848 chunk 5 4 bytes
09:36:08.848 exit 6 {"_tag":"Success","value":null}
09:36:08.848 chunk 9 36 bytes
09:36:08.849 in Ack 3
09:36:08.855 rpc 16 preview.reportProfiles
09:36:08.856 in Ack 5
09:36:08.856 in Ack 9
09:36:08.928 exit 16 {"_tag":"Success","value":null}
09:36:09.107 rpc 17 server.reportClientActivity
09:36:09.172 exit 17 {"_tag":"Success","value":null}
09:36:09.689 fetch POST /api/observability/v1/traces
09:36:09.689 fetched POST /api/observability/v1/traces 204
09:36:09.690 chunk 12 4591 bytes
09:36:09.690 in Ack 12
09:36:09.940 chunk 12 4591 bytes
09:36:09.941 in Ack 12
09:36:10.691 fetch POST /api/observability/v1/traces
09:36:10.691 fetched POST /api/observability/v1/traces 204
09:36:11.410 fetched GET /api/orchestration/threads/mcp%3A41edb4a9-ca49-4c43-a9b0-ac6315dd6487/bounded 200
09:36:11.415 rpc 18 orchestration.subscribeThread
09:36:11.415 chunk 18 8282 bytes
09:36:11.434 in Interrupt 10
09:36:11.438 rpc 19 subscribeVcsStatus
09:36:11.439 in Ack 18
09:36:11.522 in Ping 
09:36:11.650 chunk 19 1632 bytes
09:36:11.651 in Ack 19
09:36:11.666 chunk 18 9336 bytes
09:36:11.668 in Ack 18
09:36:11.735 rpc 20 server.reportClientActivity
09:36:11.864 exit 20 {"_tag":"Success","value":null}
09:36:11.916 chunk 18 12690 bytes
09:36:11.921 in Ack 18
09:36:12.178 chunk 18 13497 bytes
09:36:12.179 in Ack 18
09:36:12.413 fetch POST /api/observability/v1/traces
09:36:12.413 fetched POST /api/observability/v1/traces 204
09:36:12.428 chunk 18 17975 bytes
09:36:12.430 in Ack 18
09:36:12.678 chunk 18 20407 bytes
09:36:12.681 in Ack 18
09:36:12.696 in Interrupt 19
09:36:12.700 rpc 21 subscribeVcsStatus
09:36:12.803 rpc 22 attachments.createUploadUrl
09:36:12.817 chunk 21 1632 bytes
09:36:12.917 in Ack 21
09:36:12.931 chunk 18 21097 bytes
09:36:12.937 in Ack 18
09:36:12.957 rpc 23 server.reportClientActivity
09:36:13.004 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-cd346ab6-210f-4478-bf8e-0b20de4f0267","expiresAt":1794217572938,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:36:13.005 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWNkMzQ2YWI2LTIxMGYtNDQ3OC1iZjhlLTBiMjBkZTRmMDI2NyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzU3MjkzOH0.5158N6r0G9O9B-Gp1XFgvJShnfimmbPDIL_BUUX6-I0
09:36:13.163 exit 23 {"_tag":"Success","value":null}
09:36:13.436 fetch POST /api/observability/v1/traces
09:36:13.436 fetched POST /api/observability/v1/traces 204
09:36:16.522 in Ping 
09:36:18.126 rpc 24 orchestration.dispatchCommand message.dispatch
09:36:18.590 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:36:18.594 rpc 25 attachments.delete
09:36:18.629 fetch POST /api/observability/v1/traces
09:36:18.629 fetched POST /api/observability/v1/traces 204
09:36:19.121 exit 25 {"_tag":"Success","value":null}
09:36:19.415 chunk 18 23554 bytes
09:36:19.417 in Ack 18
09:36:19.664 chunk 12 4616 bytes
09:36:19.666 in Ack 12
09:36:19.671 rpc 26 orchestration.dispatchCommand thread.visit
09:36:19.671 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:36:19.915 chunk 18 24394 bytes
09:36:19.917 in Ack 18
09:36:20.123 fetch POST /api/observability/v1/traces
09:36:20.123 fetched POST /api/observability/v1/traces 204
09:36:20.165 chunk 12 4616 bytes
09:36:20.166 chunk 18 27847 bytes
09:36:20.166 in Ack 12
09:36:20.192 in Ack 18
09:36:20.195 rpc 27 assets.createUrl
09:36:20.417 chunk 18 27847 bytes
09:36:20.426 in Ack 18
09:36:20.668 chunk 18 29972 bytes
09:36:20.671 in Ack 18
09:36:20.916 chunk 12 4613 bytes
09:36:20.917 chunk 18 30104 bytes
09:36:20.917 in Ack 12
09:36:20.919 rpc 28 orchestration.dispatchCommand thread.visit
09:36:20.919 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:36:20.927 in Ack 18
09:36:21.466 chunk 12 4613 bytes
09:36:21.467 chunk 18 30104 bytes
09:36:21.468 fetch POST /api/observability/v1/traces
09:36:21.468 fetched POST /api/observability/v1/traces 204
09:36:21.472 in Ack 12
09:36:21.481 in Ack 18
09:36:21.527 in Ping 
09:36:21.739 chunk 18 32527 bytes
09:36:21.757 in Ack 18
09:36:22.257 rpc 29 server.refreshProviders
09:36:22.257 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:35:36.323Z","configurableProviders":false,"con
09:36:22.492 fetch POST /api/observability/v1/traces
09:36:22.492 fetched POST /api/observability/v1/traces 204
09:36:23.344 chunk 18 32551 bytes
09:36:23.432 in Ack 18
09:36:23.611 chunk 18 33329 bytes
09:36:23.624 in Ack 18
09:36:23.637 in Interrupt 21
09:36:23.641 rpc 30 subscribeVcsStatus
09:36:23.887 chunk 18 34346 bytes
09:36:23.891 in Ack 18
09:36:23.949 rpc 31 server.reportClientActivity
09:36:24.141 chunk 18 35257 bytes
09:36:24.177 in Ack 18
09:36:24.565 rpc 32 orchestration.dispatchCommand message.dispatch
09:36:24.640 chunk 18 35257 bytes
09:36:24.641 fetch POST /api/observability/v1/traces
09:36:24.641 fetched POST /api/observability/v1/traces 204
09:36:24.645 in Ack 18
09:36:25.645 fetch POST /api/observability/v1/traces
09:36:25.645 fetched POST /api/observability/v1/traces 204
09:36:26.149 chunk 12 4677 bytes
09:36:26.152 in Ack 12
09:36:26.529 in Ping 
09:36:26.634 exit 32 {"_tag":"Success","value":{"sequence":152}}
09:36:26.634 chunk 30 1713 bytes
09:36:26.635 in Ack 30
09:36:27.161 fetch POST /api/observability/v1/traces
09:36:27.161 fetched POST /api/observability/v1/traces 204
09:36:28.644 fetch POST /api/observability/v1/traces
09:36:28.644 fetched POST /api/observability/v1/traces 204
09:36:28.779 chunk 12 4721 bytes
09:36:28.779 in Ack 12
09:36:28.789 chunk 18 37278 bytes
09:36:28.792 in Ack 18
09:36:28.947 exit 31 {"_tag":"Success","value":null}
09:36:29.045 chunk 18 37862 bytes
09:36:29.050 in Ack 18
09:36:29.781 fetch POST /api/observability/v1/traces
09:36:29.781 fetched POST /api/observability/v1/traces 204
09:36:30.926 rpc 33 orchestration.dispatchCommand thread.visit
09:36:30.926 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:36:31.050 chunk 12 4741 bytes
09:36:31.051 in Ack 12
09:36:31.354 chunk 18 37914 bytes
09:36:31.357 in Ack 18
09:36:31.545 in Ping 
09:36:31.604 chunk 18 38754 bytes
09:36:31.606 in Ack 18
09:36:31.697 rpc 34 server.reportClientActivity
09:36:31.854 chunk 18 39409 bytes
09:36:31.856 in Ack 18
09:36:31.930 fetch POST /api/observability/v1/traces
09:36:31.930 fetched POST /api/observability/v1/traces 204
09:36:32.355 chunk 18 40195 bytes
09:36:32.357 in Ack 18
09:36:32.856 chunk 18 40197 bytes
09:36:32.858 in Ack 18
09:36:33.013 fetch POST /api/observability/v1/traces
09:36:33.013 fetched POST /api/observability/v1/traces 204
09:36:33.304 chunk 12 4741 bytes
09:36:33.305 in Ack 12
09:36:33.359 chunk 18 40197 bytes
09:36:33.366 in Ack 18
09:36:33.609 chunk 18 40722 bytes
09:36:33.611 in Ack 18
09:36:34.306 fetch POST /api/observability/v1/traces
09:36:34.306 fetched POST /api/observability/v1/traces 204
09:36:34.363 chunk 18 40722 bytes
09:36:34.365 in Ack 18
09:36:34.806 chunk 12 4741 bytes
09:36:34.807 in Ack 12
09:36:35.054 exit 34 {"_tag":"Success","value":null}
09:36:35.409 fetch POST /api/observability/v1/traces
09:36:35.409 fetched POST /api/observability/v1/traces 204
09:36:36.408 chunk 12 4741 bytes
09:36:36.409 in Ack 12
09:36:36.545 in Ping 
09:36:37.412 fetch POST /api/observability/v1/traces
09:36:37.412 fetched POST /api/observability/v1/traces 204
09:36:37.644 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/1ddd419b-602f-4486-bd98-3cf27229069c","expiresAt":1794220597644}}
09:36:38.176 chunk 12 4741 bytes
09:36:38.177 in Ack 12
09:36:38.646 fetch POST /api/observability/v1/traces
09:36:38.646 fetched POST /api/observability/v1/traces 204
09:36:40.426 chunk 18 44135 bytes
09:36:40.429 in Ack 18
09:36:40.675 chunk 12 4741 bytes
09:36:40.675 chunk 18 44087 bytes
09:36:40.676 in Ack 12
09:36:40.680 in Ack 18
09:36:40.928 chunk 18 45131 bytes
09:36:40.929 rpc 35 orchestration.dispatchCommand thread.visit
09:36:40.929 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:36:40.933 in Ack 18
09:36:41.430 fetch POST /api/observability/v1/traces
09:36:41.430 fetched POST /api/observability/v1/traces 204
09:36:41.545 in Ping 
09:36:42.787 chunk 18 45131 bytes
09:36:42.790 in Ack 18
09:36:43.041 chunk 12 4741 bytes
09:36:43.042 in Ack 12
09:36:43.791 fetch POST /api/observability/v1/traces
09:36:43.791 fetched POST /api/observability/v1/traces 204
09:36:46.550 in Ping 
09:36:48.686 fetch POST /api/observability/v1/traces
09:36:48.686 fetched POST /api/observability/v1/traces 204
09:36:51.551 in Ping 
09:36:54.939 chunk 12 5048 bytes
09:36:54.941 in Ack 12
09:36:55.942 fetch POST /api/observability/v1/traces
09:36:55.942 fetched POST /api/observability/v1/traces 204
09:36:56.551 in Ping 
09:36:56.696 rpc 36 server.reportClientActivity
09:36:57.456 chunk 18 46553 bytes
09:36:57.460 in Ack 18
09:36:57.696 fetch POST /api/observability/v1/traces
09:36:57.696 fetched POST /api/observability/v1/traces 204
09:36:57.705 chunk 18 47663 bytes
09:36:57.707 in Ack 18
09:36:58.489 chunk 18 48665 bytes
09:36:58.497 in Ack 18
09:36:58.710 fetch POST /api/observability/v1/traces
09:36:58.710 fetched POST /api/observability/v1/traces 204
09:36:59.399 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:36:59.500 chunk 18 52152 bytes
09:36:59.565 in Ack 18
09:37:00.017 chunk 18 54221 bytes
09:37:00.019 in Ack 18
09:37:00.398 fetch POST /api/observability/v1/traces
09:37:00.398 fetched POST /api/observability/v1/traces 204
09:37:00.915 chunk 18 54271 bytes
09:37:00.918 in Ack 18
09:37:01.169 exit 37 {"_tag":"Success","value":{"sequence":187}}
09:37:01.557 in Ping 
09:37:01.919 fetch POST /api/observability/v1/traces
09:37:01.919 fetched POST /api/observability/v1/traces 204
09:37:01.957 exit 36 {"_tag":"Success","value":null}
09:37:02.136 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:37:02.166 chunk 18 54314 bytes
09:37:02.169 in Ack 18
09:37:02.959 fetch POST /api/observability/v1/traces
09:37:02.959 fetched POST /api/observability/v1/traces 204
09:37:04.141 exit 38 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:37:04.216 chunk 12 5050 bytes
09:37:04.218 in Ack 12
09:37:04.220 rpc 39 orchestration.dispatchCommand thread.visit
09:37:04.220 exit 39 {"_tag":"Success","value":{"sequence":0}}
09:37:04.467 chunk 18 54362 bytes
09:37:04.469 in Ack 18
09:37:04.718 chunk 18 54410 bytes
09:37:04.732 in Ack 18
09:37:04.744 in Interrupt 30
09:37:04.797 rpc 40 subscribeVcsStatus
09:37:05.049 rpc 41 server.reportClientActivity
09:37:05.145 fetch POST /api/observability/v1/traces
09:37:05.145 fetched POST /api/observability/v1/traces 204
09:37:05.218 chunk 18 55394 bytes
09:37:05.221 in Ack 18
09:37:05.720 chunk 18 56787 bytes
09:37:05.723 in Ack 18
09:37:05.970 chunk 18 56788 bytes
09:37:05.972 in Ack 18
09:37:06.222 fetch POST /api/observability/v1/traces
09:37:06.222 fetched POST /api/observability/v1/traces 204
09:37:06.559 in Ping 
09:37:06.602 chunk 40 1713 bytes
09:37:06.602 in Ack 40
09:37:07.229 chunk 12 5050 bytes
09:37:07.230 in Ack 12
09:37:07.729 chunk 18 56788 bytes
09:37:07.731 in Ack 18
09:37:08.230 fetch POST /api/observability/v1/traces
09:37:08.230 fetched POST /api/observability/v1/traces 204
09:37:08.493 chunk 12 4745 bytes
09:37:08.494 in Ack 12
09:37:09.002 chunk 18 59428 bytes
09:37:09.004 in Ack 18
09:37:09.252 chunk 18 60024 bytes
09:37:09.254 in Ack 18
09:37:09.495 fetch POST /api/observability/v1/traces
09:37:09.496 fetched POST /api/observability/v1/traces 204
09:37:10.252 chunk 18 59976 bytes
09:37:10.257 in Ack 18
09:37:10.501 chunk 12 4745 bytes
09:37:10.502 in Ack 12
09:37:10.753 chunk 18 61948 bytes
09:37:10.755 in Ack 18
09:37:10.871 exit 41 {"_tag":"Success","value":null}
09:37:11.258 fetch POST /api/observability/v1/traces
09:37:11.258 fetched POST /api/observability/v1/traces 204
09:37:11.563 in Ping 
09:37:11.753 chunk 12 4745 bytes
09:37:11.757 in Ack 12
09:37:12.759 fetch POST /api/observability/v1/traces
09:37:12.759 fetched POST /api/observability/v1/traces 204
09:37:14.221 rpc 42 orchestration.dispatchCommand thread.visit
09:37:14.221 exit 42 {"_tag":"Success","value":{"sequence":0}}
09:37:15.189 chunk 18 62965 bytes
09:37:15.202 in Ack 18
09:37:15.450 fetch POST /api/observability/v1/traces
09:37:15.450 fetched POST /api/observability/v1/traces 204
09:37:16.005 chunk 12 4684 bytes
09:37:16.006 chunk 18 63155 bytes
09:37:16.185 in Ack 12
09:37:16.679 in Ack 18
