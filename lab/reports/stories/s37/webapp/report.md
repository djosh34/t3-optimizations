# story, webapp, warm start, seed 37

Profile story (graded): seed 37's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 57 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 0.9s |
| Time to first text | 13.4s |
| Whole reply shown | 55.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.0s / 7.0s / 16.0s |
| Longest silence | 21.4s |
| Approval shown after it was asked | 9.4s |
| Bytes on the wire down / up | 218171 / 112027 |
| Bytes the app received / sent | 148821 / 53005 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 365 |
| 20 | 9.0 | 397 | 766 |
| 30 | 19.0 | 397 | 1077 |
| 40 | 2.1 | 1077 | 1163 |
| 50 | 12.1 | 1077 | 1532 |
| 55 | 9.3 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217254538,"silences":0,"longestSilenceMs":702,"longestRecoveredGapMs":702},"stats":{"sendRate":6750,"deliveredRate":3289.0114496192364,"loss":0,"srttMs":3.195051076583359,"minRttMs":1,"sentPackets":125,"sentWireBytes":33273,"receivedPackets":120,"receivedWireBytes":93174,"lastHeardMs":11819,"unsyncedBytes":518,"inflightPackets":0,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217264426,"silences":0,"longestSilenceMs":702,"longestRecoveredGapMs":702},"stats":{"sendRate":2250,"deliveredRate":2128.3662477558346,"loss":0,"srttMs":1143.6377573076006,"minRttMs":1,"sentPackets":177,"sentWireBytes":43251,"receivedPackets":228,"receivedWireBytes":151228,"lastHeardMs":21707,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217274239,"silences":1,"longestSilenceMs":2805,"longestRecoveredGapMs":2805},"stats":{"sendRate":1500,"deliveredRate":1485.1632000338898,"loss":0.027272727272727282,"srttMs":1219.9020746704364,"minRttMs":1,"sentPackets":230,"sentWireBytes":55264,"receivedPackets":310,"receivedWireBytes":190952,"lastHeardMs":31520,"unsyncedBytes":167,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"462/961","assistant":"32/32 21/21 1169/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217284708,"silences":2,"longestSilenceMs":2805,"longestRecoveredGapMs":2805},"stats":{"sendRate":1125,"deliveredRate":873.7305879247779,"loss":0.27935454545454547,"srttMs":4091.2402667855094,"minRttMs":1,"sentPackets":261,"sentWireBytes":60125,"receivedPackets":360,"receivedWireBytes":204256,"lastHeardMs":41989,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1542/1542 731/923","thread":"462/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"386/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217295565,"silences":3,"longestSilenceMs":2805,"longestRecoveredGapMs":2805},"stats":{"sendRate":1687.5,"deliveredRate":1372.9662077596995,"loss":0,"srttMs":3064.039866225449,"minRttMs":1,"sentPackets":309,"sentWireBytes":70811,"receivedPackets":392,"receivedWireBytes":217277,"lastHeardMs":52846,"unsyncedBytes":398,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 731/923 1022/1068","thread":"508/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":487,"lastDataAt":1794217306560,"silences":4,"longestSilenceMs":7249,"longestRecoveredGapMs":7249},"stats":{"sendRate":2059.4493116395493,"deliveredRate":1079.6936686239794,"loss":0.027272727272727282,"srttMs":3252.5473030912694,"minRttMs":1,"sentPackets":341,"sentWireBytes":83077,"receivedPackets":410,"receivedWireBytes":226542,"lastHeardMs":63841,"unsyncedBytes":398,"inflightPackets":28,"duplicateBytes":0}}
- shim trace
09:40:43.256 fetch GET /api/auth/session
09:40:43.277 fetched GET /api/auth/session 200
09:40:43.544 fetch GET /.well-known/t3/environment
09:40:43.545 fetched GET /.well-known/t3/environment 200
09:40:43.557 fetch GET /.well-known/t3/environment
09:40:43.558 fetched GET /.well-known/t3/environment 200
09:40:43.566 rpc 0 subscribeServerConfig
09:40:43.567 chunk 0 38462 bytes
09:40:43.602 in Ack 0
09:40:43.604 rpc 1 subscribeServerLifecycle
09:40:43.605 rpc 2 subscribeServerLifecycle
09:40:43.608 fetch GET /api/auth/session
09:40:43.609 fetched GET /api/auth/session 200
09:40:43.610 rpc 3 subscribeDeviceState
09:40:43.612 rpc 4 preview.reportProfiles
09:40:43.673 rpc 5 subscribeProjectClones
09:40:43.703 fetch GET /api/orchestration/threads/mcp%3A5090dd34-9cc9-47db-b892-6db268a31ec3/bounded
09:40:43.710 fetch GET /api/orchestration/shell
09:40:43.711 fetched GET /api/orchestration/shell 200
09:40:43.715 rpc 6 orchestration.subscribeShell
09:40:43.715 chunk 6 4616 bytes
09:40:43.746 rpc 7 server.refreshProviders
09:40:43.794 rpc 8 orchestration.dispatchCommand thread.visit
09:40:43.794 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:40:43.812 rpc 9 subscribeTerminalMetadata
09:40:43.813 rpc 10 subscribeVcsStatus
09:40:43.813 rpc 11 subscribePreviewEvents
09:40:43.814 rpc 12 assets.createUrl
09:40:43.815 rpc 13 preview.list
09:40:43.815 rpc 14 projects.readFile
09:40:43.828 in Ack 6
09:40:43.950 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"2698ece0-2cc4-45b8-9695-645da3728fa9","sessions":[]}}
09:40:43.950 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:40:43.965 chunk 6 4592 bytes
09:40:43.966 in Ack 6
09:40:44.065 rpc 15 server.reportClientActivity
09:40:44.193 chunk 1 3815 bytes
09:40:44.199 in Ack 1
09:40:44.407 chunk 2 3815 bytes
09:40:44.408 in Ack 2
09:40:44.541 fetch POST /api/observability/v1/traces
09:40:44.541 fetched POST /api/observability/v1/traces 204
09:40:44.605 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:40:24.594Z","configurableProviders":false,"con
09:40:44.605 chunk 10 1632 bytes
09:40:44.605 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:40:44.608 in Ack 10
09:40:45.052 chunk 6 4635 bytes
09:40:45.053 in Ack 6
09:40:45.095 chunk 3 876 bytes
09:40:45.095 exit 4 {"_tag":"Success","value":null}
09:40:45.095 chunk 5 4 bytes
09:40:45.095 chunk 9 36 bytes
09:40:45.096 in Ack 3
09:40:45.102 rpc 16 preview.reportProfiles
09:40:45.102 in Ack 5
09:40:45.103 in Ack 9
09:40:45.194 exit 15 {"_tag":"Success","value":null}
09:40:45.194 exit 16 {"_tag":"Success","value":null}
09:40:45.606 fetch POST /api/observability/v1/traces
09:40:45.607 fetched POST /api/observability/v1/traces 204
09:40:46.502 fetched GET /api/orchestration/threads/mcp%3A5090dd34-9cc9-47db-b892-6db268a31ec3/bounded 200
09:40:46.506 rpc 17 orchestration.subscribeThread
09:40:46.506 chunk 17 8283 bytes
09:40:46.518 in Interrupt 10
09:40:46.520 rpc 18 subscribeVcsStatus
09:40:46.522 in Ack 17
09:40:46.665 chunk 18 1632 bytes
09:40:46.666 in Ack 18
09:40:46.756 chunk 17 8323 bytes
09:40:46.757 in Ack 17
09:40:46.794 rpc 19 server.reportClientActivity
09:40:46.881 exit 19 {"_tag":"Success","value":null}
09:40:47.006 chunk 17 12691 bytes
09:40:47.007 in Ack 17
09:40:47.255 chunk 17 13498 bytes
09:40:47.257 in Ack 17
09:40:47.417 rpc 20 attachments.createUploadUrl
09:40:47.503 fetch POST /api/observability/v1/traces
09:40:47.503 fetched POST /api/observability/v1/traces 204
09:40:47.505 chunk 17 16151 bytes
09:40:47.506 in Ack 17
09:40:47.520 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-561c71a2-e864-4ef5-8ff1-4619f97706fc","expiresAt":1794217847509,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:40:47.522 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTU2MWM3MWEyLWU4NjQtNGVmNS04ZmYxLTQ2MTlmOTc3MDZmYyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzg0NzUwOX0.S589SVlIax0DxOpw16CrRs4T6dJxHQdvNAdKka78PtA
09:40:47.756 chunk 17 17977 bytes
09:40:47.757 in Ack 17
09:40:48.015 chunk 17 20409 bytes
09:40:48.017 in Ack 17
09:40:48.027 in Interrupt 18
09:40:48.030 rpc 21 subscribeVcsStatus
09:40:48.147 chunk 21 1713 bytes
09:40:48.148 in Ack 21
09:40:48.265 chunk 17 21100 bytes
09:40:48.267 in Ack 17
09:40:48.281 rpc 22 server.reportClientActivity
09:40:48.507 fetch POST /api/observability/v1/traces
09:40:48.507 fetched POST /api/observability/v1/traces 204
09:40:48.561 in Ping 
09:40:49.388 exit 22 {"_tag":"Success","value":null}
09:40:50.312 rpc 23 orchestration.dispatchCommand message.dispatch
09:40:50.522 fetch POST /api/observability/v1/traces
09:40:50.522 fetched POST /api/observability/v1/traces 204
09:40:50.580 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:40:50.583 rpc 24 attachments.delete
09:40:50.772 chunk 6 4721 bytes
09:40:50.773 in Ack 6
09:40:50.973 exit 24 {"_tag":"Success","value":null}
09:40:51.022 chunk 17 26024 bytes
09:40:51.024 in Ack 17
09:40:51.027 rpc 25 assets.createUrl
09:40:51.272 chunk 17 30178 bytes
09:40:51.274 in Ack 17
09:40:51.522 chunk 17 30312 bytes
09:40:51.524 in Ack 17
09:40:51.616 fetch POST /api/observability/v1/traces
09:40:51.616 fetched POST /api/observability/v1/traces 204
09:40:51.619 chunk 6 4660 bytes
09:40:51.620 in Ack 6
09:40:51.777 chunk 17 32759 bytes
09:40:51.785 in Ack 17
09:40:51.869 chunk 6 4657 bytes
09:40:51.869 in Ack 6
09:40:51.871 rpc 26 orchestration.dispatchCommand thread.visit
09:40:51.871 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:40:52.027 chunk 17 33559 bytes
09:40:52.028 in Ack 17
09:40:52.039 in Interrupt 21
09:40:52.041 rpc 27 subscribeVcsStatus
09:40:52.084 chunk 27 1713 bytes
09:40:52.085 in Ack 27
09:40:52.119 chunk 6 4657 bytes
09:40:52.119 in Ack 6
09:40:52.277 chunk 17 35260 bytes
09:40:52.278 in Ack 17
09:40:52.293 rpc 28 server.reportClientActivity
09:40:52.350 exit 28 {"_tag":"Success","value":null}
09:40:52.528 chunk 17 35260 bytes
09:40:52.529 in Ack 17
09:40:52.831 fetch POST /api/observability/v1/traces
09:40:52.831 fetched POST /api/observability/v1/traces 204
09:40:53.571 in Ping 
09:40:54.508 rpc 29 orchestration.dispatchCommand message.dispatch
09:40:54.538 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/efcfd9aa-9f04-4657-9738-2b86009e2c6b","expiresAt":1794220854538}}
09:40:55.311 fetch POST /api/observability/v1/traces
09:40:55.311 fetched POST /api/observability/v1/traces 204
09:40:56.659 chunk 6 4721 bytes
09:40:56.660 in Ack 6
09:40:56.668 exit 29 {"_tag":"Success","value":{"sequence":152}}
09:40:57.162 chunk 17 36736 bytes
09:40:57.163 in Ack 17
09:40:57.412 chunk 17 37391 bytes
09:40:57.413 in Ack 17
09:40:57.664 fetch POST /api/observability/v1/traces
09:40:57.664 fetched POST /api/observability/v1/traces 204
09:40:57.665 chunk 17 39412 bytes
09:40:57.669 in Ack 17
09:40:57.915 chunk 6 4785 bytes
09:40:57.916 in Ack 6
09:40:58.240 chunk 17 40198 bytes
09:40:58.242 in Ack 17
09:40:58.491 chunk 17 40200 bytes
09:40:58.493 in Ack 17
09:40:58.572 in Ping 
09:40:58.670 fetch POST /api/observability/v1/traces
09:40:58.670 fetched POST /api/observability/v1/traces 204
09:40:58.740 chunk 17 40725 bytes
09:40:58.742 in Ack 17
09:40:58.992 chunk 6 4785 bytes
09:40:58.993 in Ack 6
09:40:59.743 fetch POST /api/observability/v1/traces
09:40:59.743 fetched POST /api/observability/v1/traces 204
09:41:01.872 rpc 30 orchestration.dispatchCommand thread.visit
09:41:01.872 exit 30 {"_tag":"Success","value":{"sequence":0}}
09:41:02.876 fetch POST /api/observability/v1/traces
09:41:02.876 fetched POST /api/observability/v1/traces 204
09:41:03.241 chunk 6 4785 bytes
09:41:03.242 in Ack 6
09:41:03.491 chunk 17 40725 bytes
09:41:03.493 in Ack 17
09:41:03.572 in Ping 
09:41:04.243 fetch POST /api/observability/v1/traces
09:41:04.243 fetched POST /api/observability/v1/traces 204
09:41:05.314 fetch POST /api/observability/v1/traces
09:41:05.314 fetched POST /api/observability/v1/traces 204
09:41:05.737 chunk 6 4785 bytes
09:41:05.738 in Ack 6
09:41:06.739 fetch POST /api/observability/v1/traces
09:41:06.739 fetched POST /api/observability/v1/traces 204
09:41:07.657 chunk 17 45182 bytes
09:41:07.658 in Ack 17
09:41:08.572 in Ping 
09:41:08.659 fetch POST /api/observability/v1/traces
09:41:08.659 fetched POST /api/observability/v1/traces 204
09:41:08.660 chunk 17 45134 bytes
09:41:08.662 in Ack 17
09:41:08.800 rpc 31 server.reportClientActivity
09:41:08.974 chunk 6 4785 bytes
09:41:08.975 in Ack 6
09:41:09.663 fetch POST /api/observability/v1/traces
09:41:09.663 fetched POST /api/observability/v1/traces 204
09:41:10.015 exit 31 {"_tag":"Success","value":null}
09:41:10.152 chunk 6 4785 bytes
09:41:10.152 in Ack 6
09:41:11.018 fetch POST /api/observability/v1/traces
09:41:11.018 fetched POST /api/observability/v1/traces 204
09:41:11.871 rpc 32 orchestration.dispatchCommand thread.visit
09:41:11.871 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:41:12.871 fetch POST /api/observability/v1/traces
09:41:12.871 fetched POST /api/observability/v1/traces 204
09:41:13.572 in Ping 
09:41:15.321 fetch POST /api/observability/v1/traces
09:41:15.321 fetched POST /api/observability/v1/traces 204
09:41:18.575 in Ping 
09:41:23.575 in Ping 
09:41:24.336 chunk 17 47153 bytes
09:41:24.339 in Ack 17
09:41:24.709 chunk 17 47156 bytes
09:41:24.714 in Ack 17
09:41:25.057 chunk 17 47666 bytes
09:41:25.062 in Ack 17
09:41:25.404 fetch POST /api/observability/v1/traces
09:41:25.404 fetched POST /api/observability/v1/traces 204
09:41:28.575 in Ping 
09:41:31.457 chunk 17 48668 bytes
09:41:31.459 in Ack 17
09:41:31.684 rpc 33 orchestration.dispatchCommand runtime-request.respond
09:41:31.765 chunk 17 52155 bytes
09:41:31.769 in Ack 17
09:41:32.461 fetch POST /api/observability/v1/traces
09:41:32.461 fetched POST /api/observability/v1/traces 204
09:41:33.576 in Ping 
09:41:33.801 rpc 34 server.reportClientActivity
09:41:34.801 fetch POST /api/observability/v1/traces
09:41:34.801 fetched POST /api/observability/v1/traces 204
09:41:34.961 exit 33 {"_tag":"Success","value":{"sequence":193}}
09:41:35.453 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:41:35.962 fetch POST /api/observability/v1/traces
09:41:35.962 fetched POST /api/observability/v1/traces 204
09:41:36.664 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:41:36.776 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:41:36.809 chunk 17 52198 bytes
09:41:36.811 in Ack 17
09:41:37.667 fetch POST /api/observability/v1/traces
09:41:37.667 fetched POST /api/observability/v1/traces 204
09:41:38.579 in Ping 
09:41:43.583 in Ping 
09:41:43.914 chunk 17 52248 bytes
09:41:43.916 in Ack 17
09:41:44.918 fetch POST /api/observability/v1/traces
09:41:44.918 fetched POST /api/observability/v1/traces 204
09:41:45.793 chunk 17 54892 bytes
09:41:45.800 in Ack 17
09:41:46.793 chunk 17 55484 bytes
09:41:46.797 in Ack 17
09:41:46.829 fetch POST /api/observability/v1/traces
09:41:46.829 fetched POST /api/observability/v1/traces 204
09:41:47.043 chunk 17 56042 bytes
09:41:47.045 in Ack 17
09:41:48.046 fetch POST /api/observability/v1/traces
09:41:48.047 fetched POST /api/observability/v1/traces 204
09:41:48.588 in Ping 
09:41:48.802 in Interrupt 34
09:41:49.165 chunk 17 56162 bytes
09:41:49.168 in Ack 17
09:41:49.570 chunk 17 56164 bytes
09:41:49.571 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:41:49.572 in Ack 17
09:41:49.803 fetch POST /api/observability/v1/traces
09:41:49.803 fetched POST /api/observability/v1/traces 204
