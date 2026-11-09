# story, webapp, warm start, seed 34

Profile story (graded): seed 34's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 54 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.5s |
| Prompt reached the agent | 2.6s |
| Time to first text | 14.1s |
| Whole reply shown | 52.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.3s / 5.3s / 14.3s |
| Longest silence | 16.9s |
| Approval shown after it was asked | 4.2s |
| Bytes on the wire down / up | 421158 / 141944 |
| Bytes the app received / sent | 267041 / 53557 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.3 | 0 | 299 |
| 20 | 7.3 | 397 | 697 |
| 30 | 17.3 | 397 | 1077 |
| 40 | 3.1 | 1077 | 1200 |
| 50 | 5.3 | 1378 | 1567 |
| 52 | 7.3 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":670,"lastDataAt":1794217047724,"silences":0,"longestSilenceMs":999,"longestRecoveredGapMs":999},"stats":{"sendRate":10125,"deliveredRate":4081.9966253784205,"loss":0,"srttMs":5.605480791162908,"minRttMs":1,"sentPackets":104,"sentWireBytes":33501,"receivedPackets":157,"receivedWireBytes":99109,"lastHeardMs":9214,"unsyncedBytes":518,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 133/839","user":"10/10 9/9","attempts":"659/659 659/659 624/635","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":670,"lastDataAt":1794217059293,"silences":1,"longestSilenceMs":3154,"longestRecoveredGapMs":3154},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.2045,"srttMs":1835.964212513949,"minRttMs":3159,"sentPackets":146,"sentWireBytes":42003,"receivedPackets":205,"receivedWireBytes":114779,"lastHeardMs":20783,"unsyncedBytes":167,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 543/543 451/451","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":670,"lastDataAt":1794217069312,"silences":1,"longestSilenceMs":3154,"longestRecoveredGapMs":3154},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1808.7132012920954,"minRttMs":1741,"sentPackets":213,"sentWireBytes":55768,"receivedPackets":274,"receivedWireBytes":141072,"lastHeardMs":30802,"unsyncedBytes":167,"inflightPackets":10,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 543/543 451/451","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":670,"lastDataAt":1794217079539,"silences":1,"longestSilenceMs":3154,"longestRecoveredGapMs":3154},"stats":{"sendRate":2230.8939323761006,"deliveredRate":1056.3075281856231,"loss":0,"srttMs":1825.9855144964097,"minRttMs":1725,"sentPackets":268,"sentWireBytes":63963,"receivedPackets":399,"receivedWireBytes":185985,"lastHeardMs":41029,"unsyncedBytes":0,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 171/947 917/1566","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 544/544 451/451","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1177/1392 959/959 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":670,"lastDataAt":1794217090536,"silences":2,"longestSilenceMs":3154,"longestRecoveredGapMs":3154},"stats":{"sendRate":1500,"deliveredRate":380.63413117811626,"loss":0.07692307692307687,"srttMs":3174.451233344274,"minRttMs":1725,"sentPackets":294,"sentWireBytes":71556,"receivedPackets":514,"receivedWireBytes":252157,"lastHeardMs":52026,"unsyncedBytes":1022,"inflightPackets":19,"duplicateBytes":0}}
- shim trace
09:37:19.110 fetch GET /api/auth/session
09:37:19.115 fetched GET /api/auth/session 200
09:37:19.363 fetch GET /.well-known/t3/environment
09:37:19.366 fetched GET /.well-known/t3/environment 200
09:37:19.376 fetch GET /.well-known/t3/environment
09:37:19.377 fetched GET /.well-known/t3/environment 200
09:37:19.381 rpc 0 subscribeServerConfig
09:37:19.382 chunk 0 38462 bytes
09:37:19.433 in Ack 0
09:37:19.435 rpc 1 subscribeServerLifecycle
09:37:19.436 rpc 2 subscribeServerLifecycle
09:37:19.439 fetch GET /api/auth/session
09:37:19.440 fetched GET /api/auth/session 200
09:37:19.440 rpc 3 subscribeDeviceState
09:37:19.442 rpc 4 preview.reportProfiles
09:37:19.608 rpc 5 subscribeProjectClones
09:37:19.636 fetch GET /api/orchestration/threads/mcp%3A27ab7524-2db7-43e6-aeff-aa3e4396e627/bounded
09:37:19.639 rpc 6 server.reportClientActivity
09:37:19.644 fetch GET /api/orchestration/shell
09:37:19.645 fetched GET /api/orchestration/shell 200
09:37:19.649 rpc 7 orchestration.subscribeShell
09:37:19.649 chunk 7 4573 bytes
09:37:19.679 rpc 8 server.refreshProviders
09:37:19.718 rpc 9 orchestration.dispatchCommand thread.visit
09:37:19.718 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:19.734 rpc 10 subscribeTerminalMetadata
09:37:19.735 rpc 11 subscribeVcsStatus
09:37:19.736 rpc 12 subscribePreviewEvents
09:37:19.737 rpc 13 assets.createUrl
09:37:19.737 rpc 14 preview.list
09:37:19.738 rpc 15 projects.readFile
09:37:19.765 in Ack 7
09:37:19.832 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"b226ca4b-870c-4ac5-a895-f6f279104af2","sessions":[]}}
09:37:19.832 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:20.130 chunk 1 3815 bytes
09:37:20.131 in Ack 1
09:37:20.204 chunk 2 3815 bytes
09:37:20.205 in Ack 2
09:37:20.323 chunk 7 4549 bytes
09:37:20.323 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:58.117Z","configurableProviders":false,"con
09:37:20.323 chunk 11 1632 bytes
09:37:20.323 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:20.324 in Ack 7
09:37:20.337 in Ack 11
09:37:20.362 fetch POST /api/observability/v1/traces
09:37:20.362 fetched POST /api/observability/v1/traces 204
09:37:20.567 chunk 3 876 bytes
09:37:20.567 exit 4 {"_tag":"Success","value":null}
09:37:20.567 chunk 5 4 bytes
09:37:20.567 exit 6 {"_tag":"Success","value":null}
09:37:20.567 chunk 10 36 bytes
09:37:20.568 in Ack 3
09:37:20.572 rpc 16 preview.reportProfiles
09:37:20.572 in Ack 5
09:37:20.573 in Ack 10
09:37:20.576 chunk 7 4592 bytes
09:37:20.577 in Ack 7
09:37:20.628 exit 16 {"_tag":"Success","value":null}
09:37:20.824 rpc 17 server.reportClientActivity
09:37:20.891 exit 17 {"_tag":"Success","value":null}
09:37:21.354 fetched GET /api/orchestration/threads/mcp%3A27ab7524-2db7-43e6-aeff-aa3e4396e627/bounded 200
09:37:21.358 rpc 18 orchestration.subscribeThread
09:37:21.358 chunk 18 8283 bytes
09:37:21.371 in Interrupt 11
09:37:21.373 rpc 19 subscribeVcsStatus
09:37:21.374 in Ack 18
09:37:21.486 chunk 19 1632 bytes
09:37:21.486 in Ack 19
09:37:21.592 fetch POST /api/observability/v1/traces
09:37:21.592 fetched POST /api/observability/v1/traces 204
09:37:21.608 chunk 18 11260 bytes
09:37:21.609 in Ack 18
09:37:21.665 rpc 20 server.reportClientActivity
09:37:21.859 chunk 18 13498 bytes
09:37:21.863 in Ack 18
09:37:22.148 rpc 21 attachments.createUploadUrl
09:37:22.194 chunk 18 17977 bytes
09:37:22.206 in Ack 18
09:37:22.256 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-b3da0d09-9489-4097-82a6-67465a70bbfd","expiresAt":1794217642234,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:22.256 exit 20 {"_tag":"Success","value":null}
09:37:22.258 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWIzZGEwZDA5LTk0ODktNDA5Ny04MmE2LTY3NDY1YTcwYmJmZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0MjIzNH0.dKS1zEP88ZMs-poS_fF88nkUrsOYWb4Mx_xOTATIGnQ
09:37:22.445 chunk 18 21100 bytes
09:37:22.446 in Ack 18
09:37:22.458 in Interrupt 19
09:37:22.460 rpc 22 subscribeVcsStatus
09:37:22.521 chunk 22 1632 bytes
09:37:22.521 in Ack 22
09:37:22.610 fetch POST /api/observability/v1/traces
09:37:22.610 fetched POST /api/observability/v1/traces 204
09:37:22.718 rpc 23 server.reportClientActivity
09:37:23.453 exit 23 {"_tag":"Success","value":null}
09:37:23.718 fetch POST /api/observability/v1/traces
09:37:23.718 fetched POST /api/observability/v1/traces 204
09:37:24.379 in Ping 
09:37:25.208 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:25.438 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:25.442 rpc 25 attachments.delete
09:37:25.448 chunk 18 23893 bytes
09:37:25.450 in Ack 18
09:37:25.453 rpc 26 assets.createUrl
09:37:25.515 exit 25 {"_tag":"Success","value":null}
09:37:25.698 chunk 7 4678 bytes
09:37:25.699 chunk 18 26024 bytes
09:37:25.699 in Ack 7
09:37:25.741 in Ack 18
09:37:26.004 chunk 18 27723 bytes
09:37:26.006 in Ack 18
09:37:26.207 fetch POST /api/observability/v1/traces
09:37:26.207 fetched POST /api/observability/v1/traces 204
09:37:26.254 chunk 7 4617 bytes
09:37:26.254 chunk 18 30202 bytes
09:37:26.255 in Ack 7
09:37:26.266 in Ack 18
09:37:26.507 chunk 18 31903 bytes
09:37:26.509 in Ack 18
09:37:26.742 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/63be9e56-2a8c-418a-bd21-e01cc3f24e11","expiresAt":1794220646742}}
09:37:27.007 chunk 7 4614 bytes
09:37:27.007 chunk 18 35260 bytes
09:37:27.008 in Ack 7
09:37:27.009 rpc 27 orchestration.dispatchCommand thread.visit
09:37:27.009 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:27.016 in Ack 18
09:37:27.030 in Interrupt 22
09:37:27.058 rpc 28 subscribeVcsStatus
09:37:27.130 chunk 28 1713 bytes
09:37:27.130 in Ack 28
09:37:27.256 fetch POST /api/observability/v1/traces
09:37:27.256 fetched POST /api/observability/v1/traces 204
09:37:27.258 chunk 7 4614 bytes
09:37:27.259 chunk 18 35260 bytes
09:37:27.259 in Ack 7
09:37:27.267 in Ack 18
09:37:27.310 rpc 29 server.reportClientActivity
09:37:27.329 exit 29 {"_tag":"Success","value":null}
09:37:28.375 fetch POST /api/observability/v1/traces
09:37:28.375 fetched POST /api/observability/v1/traces 204
09:37:29.259 rpc 30 orchestration.dispatchCommand message.dispatch
09:37:29.379 in Ping 
09:37:30.261 fetch POST /api/observability/v1/traces
09:37:30.261 fetched POST /api/observability/v1/traces 204
09:37:32.578 exit 30 {"_tag":"Success","value":{"sequence":151}}
09:37:32.894 fetch POST /api/observability/v1/traces
09:37:32.894 fetched POST /api/observability/v1/traces 204
09:37:34.381 in Ping 
09:37:39.026 chunk 7 4678 bytes
09:37:39.027 in Ack 7
09:37:39.028 rpc 31 orchestration.dispatchCommand thread.visit
09:37:39.028 exit 31 {"_tag":"Success","value":{"sequence":0}}
09:37:39.381 in Ping 
09:37:40.027 chunk 18 35896 bytes
09:37:40.029 in Ack 18
09:37:40.040 fetch POST /api/observability/v1/traces
09:37:40.040 fetched POST /api/observability/v1/traces 204
09:37:41.181 fetch POST /api/observability/v1/traces
09:37:41.181 fetched POST /api/observability/v1/traces 204
09:37:42.028 chunk 18 38757 bytes
09:37:42.029 in Ack 18
09:37:42.780 chunk 18 41554 bytes
09:37:42.782 in Ack 18
09:37:42.897 fetch POST /api/observability/v1/traces
09:37:42.897 fetched POST /api/observability/v1/traces 204
09:37:43.031 chunk 18 42170 bytes
09:37:43.034 in Ack 18
09:37:43.783 chunk 18 42122 bytes
09:37:43.785 in Ack 18
09:37:44.041 fetch POST /api/observability/v1/traces
09:37:44.041 fetched POST /api/observability/v1/traces 204
09:37:44.381 in Ping 
09:37:44.621 rpc 32 server.reportClientActivity
09:37:45.621 fetch POST /api/observability/v1/traces
09:37:45.621 fetched POST /api/observability/v1/traces 204
09:37:45.787 chunk 7 4678 bytes
09:37:45.788 chunk 18 42122 bytes
09:37:45.788 in Ack 7
09:37:45.797 in Ack 18
09:37:46.288 chunk 18 43821 bytes
09:37:46.290 in Ack 18
09:37:46.788 chunk 18 44607 bytes
09:37:46.790 in Ack 18
09:37:46.809 fetch POST /api/observability/v1/traces
09:37:46.809 fetched POST /api/observability/v1/traces 204
09:37:47.290 chunk 18 44609 bytes
09:37:47.291 in Ack 18
09:37:48.290 chunk 18 45134 bytes
09:37:48.292 in Ack 18
09:37:48.304 fetch POST /api/observability/v1/traces
09:37:48.304 fetched POST /api/observability/v1/traces 204
09:37:48.554 exit 32 {"_tag":"Success","value":null}
09:37:48.791 chunk 18 45134 bytes
09:37:48.793 in Ack 18
09:37:49.032 rpc 33 orchestration.dispatchCommand thread.visit
09:37:49.032 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:49.382 in Ping 
09:37:49.540 chunk 7 4742 bytes
09:37:49.542 in Ack 7
09:37:49.556 fetch POST /api/observability/v1/traces
09:37:49.556 fetched POST /api/observability/v1/traces 204
09:37:51.186 fetch POST /api/observability/v1/traces
09:37:51.186 fetched POST /api/observability/v1/traces 204
09:37:54.047 chunk 7 4742 bytes
09:37:54.048 in Ack 7
09:37:54.383 in Ping 
09:37:54.570 chunk 18 45134 bytes
09:37:54.572 in Ack 18
09:37:55.050 fetch POST /api/observability/v1/traces
09:37:55.050 fetched POST /api/observability/v1/traces 204
09:37:55.550 chunk 7 4742 bytes
09:37:55.550 in Ack 7
09:37:56.551 fetch POST /api/observability/v1/traces
09:37:56.551 fetched POST /api/observability/v1/traces 204
09:37:59.383 in Ping 
09:37:59.581 chunk 18 46682 bytes
09:37:59.584 in Ack 18
09:37:59.830 chunk 18 47666 bytes
09:37:59.832 in Ack 18
09:38:00.585 fetch POST /api/observability/v1/traces
09:38:00.585 fetched POST /api/observability/v1/traces 204
09:38:02.690 chunk 18 48668 bytes
09:38:02.692 in Ack 18
09:38:02.882 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:38:02.942 chunk 18 52155 bytes
09:38:02.944 in Ack 18
09:38:03.191 chunk 18 54224 bytes
09:38:03.192 in Ack 18
09:38:03.691 chunk 18 54274 bytes
09:38:03.692 in Ack 18
09:38:03.713 fetch POST /api/observability/v1/traces
09:38:03.713 fetched POST /api/observability/v1/traces 204
09:38:03.941 chunk 7 5049 bytes
09:38:03.942 in Ack 7
09:38:04.191 chunk 18 55643 bytes
09:38:04.193 in Ack 18
09:38:04.383 in Ping 
09:38:04.943 fetch POST /api/observability/v1/traces
09:38:04.943 fetched POST /api/observability/v1/traces 204
09:38:05.966 chunk 18 57621 bytes
09:38:05.968 in Ack 18
09:38:06.218 chunk 7 5356 bytes
09:38:06.219 in Ack 7
09:38:06.969 fetch POST /api/observability/v1/traces
09:38:06.969 fetched POST /api/observability/v1/traces 204
09:38:08.934 exit 34 {"_tag":"Success","value":{"sequence":186}}
09:38:08.935 rpc 35 orchestration.dispatchCommand thread.visit
09:38:08.935 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:09.141 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:09.184 chunk 18 57664 bytes
09:38:09.187 in Ack 18
09:38:09.383 in Ping 
09:38:09.633 rpc 37 server.reportClientActivity
09:38:09.936 fetch POST /api/observability/v1/traces
09:38:09.936 fetched POST /api/observability/v1/traces 204
09:38:11.269 chunk 7 5051 bytes
09:38:11.270 in Ack 7
09:38:11.360 fetch POST /api/observability/v1/traces
09:38:11.360 fetched POST /api/observability/v1/traces 204
09:38:11.520 chunk 7 5051 bytes
09:38:11.521 in Ack 7
09:38:11.771 chunk 18 57664 bytes
09:38:11.773 in Ack 18
09:38:12.023 chunk 18 57712 bytes
09:38:12.025 in Ack 18
09:38:12.102 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:12.523 fetch POST /api/observability/v1/traces
09:38:12.523 fetched POST /api/observability/v1/traces 204
09:38:13.025 chunk 18 57736 bytes
09:38:13.030 in Ack 18
09:38:13.278 chunk 18 57784 bytes
09:38:13.279 in Ack 18
09:38:13.289 in Interrupt 28
09:38:13.310 rpc 38 subscribeVcsStatus
09:38:13.527 chunk 7 4746 bytes
09:38:13.528 chunk 18 57808 bytes
09:38:13.528 in Ack 7
09:38:13.531 in Ack 18
09:38:13.944 rpc 39 orchestration.dispatchCommand thread.visit
09:38:13.944 exit 39 {"_tag":"Success","value":{"sequence":0}}
09:38:14.032 fetch POST /api/observability/v1/traces
09:38:14.032 fetched POST /api/observability/v1/traces 204
09:38:14.386 in Ping 
09:38:15.030 chunk 18 60996 bytes
09:38:15.032 in Ack 18
09:38:15.238 exit 37 {"_tag":"Success","value":null}
09:38:15.278 chunk 7 4746 bytes
09:38:15.279 in Ack 7
09:38:15.280 chunk 18 61951 bytes
09:38:15.282 in Ack 18
09:38:15.371 chunk 38 1713 bytes
09:38:15.372 in Ack 38
09:38:15.489 rpc 40 server.reportClientActivity
09:38:16.032 fetch POST /api/observability/v1/traces
09:38:16.033 fetched POST /api/observability/v1/traces 204
09:38:16.529 chunk 7 4746 bytes
09:38:16.529 in Ack 7
09:38:16.536 chunk 18 61951 bytes
09:38:16.537 in Ack 18
09:38:17.532 fetch POST /api/observability/v1/traces
09:38:17.532 fetched POST /api/observability/v1/traces 204
09:38:18.488 exit 40 {"_tag":"Success","value":null}
09:38:18.932 chunk 7 4746 bytes
09:38:18.933 in Ack 7
09:38:19.390 in Ping 
09:38:19.489 fetch POST /api/observability/v1/traces
09:38:19.489 fetched POST /api/observability/v1/traces 204
09:38:20.958 chunk 18 63018 bytes
09:38:20.960 in Ack 18
09:38:21.217 chunk 7 4685 bytes
09:38:21.217 chunk 18 63040 bytes
09:38:21.218 in Ack 7
09:38:21.236 in Ack 18
09:38:21.385 fetch POST /api/observability/v1/traces
09:38:21.385 fetched POST /api/observability/v1/traces 204
09:38:21.649 chunk 18 64176 bytes
09:38:21.651 in Ack 18
