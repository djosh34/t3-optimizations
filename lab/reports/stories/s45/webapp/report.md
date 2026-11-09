# story, webapp, warm start, seed 45

Profile story (graded): seed 45's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 54 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.6s |
| Prompt reached the agent | 3.5s |
| Time to first text | 17.1s |
| Whole reply shown | 52.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.4s / 7.4s / 15.4s |
| Longest silence | 17.1s |
| Approval shown after it was asked | 2.7s |
| Bytes on the wire down / up | 325036 / 154002 |
| Bytes the app received / sent | 194094 / 45139 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.4 | 0 | 258 |
| 20 | 6.4 | 397 | 655 |
| 30 | 16.4 | 397 | 1061 |
| 40 | 5.3 | 1077 | 1288 |
| 50 | 7.6 | 1378 | 1596 |
| 52 | 9.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 33/901","attempts":"659/659 659/659","providerTurns":"772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809","checkpoints":"777/777 611/611 566/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":907,"lastDataAt":1794217193066,"silences":0,"longestSilenceMs":1222,"longestRecoveredGapMs":1222},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":12.75629018360489,"minRttMs":1,"sentPackets":102,"sentWireBytes":31842,"receivedPackets":137,"receivedWireBytes":93704,"lastHeardMs":13212,"unsyncedBytes":736,"inflightPackets":6,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 279/654","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 20/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":907,"lastDataAt":1794217202922,"silences":1,"longestSilenceMs":2257,"longestRecoveredGapMs":2257},"stats":{"sendRate":1500,"deliveredRate":741.6378316032295,"loss":0.16666666666666663,"srttMs":1508.415247248739,"minRttMs":1,"sentPackets":128,"sentWireBytes":41054,"receivedPackets":197,"receivedWireBytes":122002,"lastHeardMs":23068,"unsyncedBytes":331,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544 451/451","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":907,"lastDataAt":1794217213102,"silences":1,"longestSilenceMs":2257,"longestRecoveredGapMs":2257},"stats":{"sendRate":750,"deliveredRate":555.0007465440106,"loss":0.369298245614035,"srttMs":3116.659042590464,"minRttMs":1,"sentPackets":143,"sentWireBytes":45905,"receivedPackets":296,"receivedWireBytes":168003,"lastHeardMs":33248,"unsyncedBytes":457,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544 451/451 543/543","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":907,"lastDataAt":1794217223338,"silences":2,"longestSilenceMs":2360,"longestRecoveredGapMs":2360},"stats":{"sendRate":748.1988809300242,"deliveredRate":535.8792506464581,"loss":0.28377432216905896,"srttMs":4715.404118922557,"minRttMs":5631,"sentPackets":163,"sentWireBytes":51934,"receivedPackets":409,"receivedWireBytes":217611,"lastHeardMs":43484,"unsyncedBytes":0,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 861/947","thread":"634/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544 451/451 543/543","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 54/1392 496/983","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"397/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":907,"lastDataAt":1794217233958,"silences":2,"longestSilenceMs":2360,"longestRecoveredGapMs":2360},"stats":{"sendRate":1122.2983213950363,"deliveredRate":720.9719407788227,"loss":0.1734265734265734,"srttMs":2656.8693611716913,"minRttMs":2198,"sentPackets":200,"sentWireBytes":61869,"receivedPackets":493,"receivedWireBytes":258181,"lastHeardMs":54104,"unsyncedBytes":802,"inflightPackets":31,"duplicateBytes":0}}
- shim trace
09:39:40.909 fetch GET /api/auth/session
09:39:40.912 fetched GET /api/auth/session 200
09:39:41.226 fetch GET /.well-known/t3/environment
09:39:41.231 fetched GET /.well-known/t3/environment 200
09:39:41.254 fetch GET /.well-known/t3/environment
09:39:41.255 fetched GET /.well-known/t3/environment 200
09:39:41.262 rpc 0 subscribeServerConfig
09:39:41.262 chunk 0 38462 bytes
09:39:41.341 in Ack 0
09:39:41.345 rpc 1 subscribeServerLifecycle
09:39:41.364 rpc 2 subscribeServerLifecycle
09:39:41.377 fetch GET /api/auth/session
09:39:41.379 fetched GET /api/auth/session 200
09:39:41.380 rpc 3 subscribeDeviceState
09:39:41.399 rpc 4 preview.reportProfiles
09:39:41.570 rpc 5 server.reportClientActivity
09:39:41.610 rpc 6 subscribeProjectClones
09:39:41.628 fetch GET /api/orchestration/shell
09:39:41.634 fetched GET /api/orchestration/shell 200
09:39:41.644 rpc 7 orchestration.subscribeShell
09:39:41.644 chunk 7 4573 bytes
09:39:41.832 rpc 8 server.refreshProviders
09:39:41.926 rpc 9 orchestration.dispatchCommand thread.visit
09:39:41.926 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:39:41.964 rpc 10 subscribeTerminalMetadata
09:39:41.966 rpc 11 subscribeVcsStatus
09:39:41.967 rpc 12 subscribePreviewEvents
09:39:41.969 rpc 13 assets.createUrl
09:39:41.970 rpc 14 preview.list
09:39:41.971 rpc 15 projects.readFile
09:39:42.013 fetch GET /api/orchestration/threads/mcp%3A41c71895-3227-47bd-bb0f-b0c506abdc04/bounded
09:39:42.021 in Ack 7
09:39:42.245 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"293df6bc-8d0d-46e2-8377-4eb8f125383e","sessions":[]}}
09:39:42.245 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:39:42.249 fetch POST /api/observability/v1/traces
09:39:42.249 fetched POST /api/observability/v1/traces 204
09:39:42.362 chunk 1 3815 bytes
09:39:42.363 in Ack 1
09:39:42.476 chunk 2 3815 bytes
09:39:42.477 in Ack 2
09:39:42.582 chunk 7 4549 bytes
09:39:42.582 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:39:17.794Z","configurableProviders":false,"con
09:39:42.582 chunk 11 1632 bytes
09:39:42.582 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:39:42.583 in Ack 7
09:39:42.592 in Ack 11
09:39:42.832 chunk 7 4592 bytes
09:39:42.833 in Ack 7
09:39:42.876 chunk 3 876 bytes
09:39:42.876 exit 4 {"_tag":"Success","value":null}
09:39:42.877 in Ack 3
09:39:42.882 rpc 16 preview.reportProfiles
09:39:42.943 exit 5 {"_tag":"Success","value":null}
09:39:42.943 chunk 6 4 bytes
09:39:42.943 chunk 10 36 bytes
09:39:42.943 exit 16 {"_tag":"Success","value":null}
09:39:42.944 in Ack 6
09:39:42.944 in Ack 10
09:39:43.195 rpc 17 server.reportClientActivity
09:39:43.207 exit 17 {"_tag":"Success","value":null}
09:39:43.252 fetch POST /api/observability/v1/traces
09:39:43.252 fetched POST /api/observability/v1/traces 204
09:39:43.669 fetched GET /api/orchestration/threads/mcp%3A41c71895-3227-47bd-bb0f-b0c506abdc04/bounded 200
09:39:43.673 rpc 18 orchestration.subscribeThread
09:39:43.673 chunk 18 8283 bytes
09:39:43.688 in Interrupt 11
09:39:43.691 rpc 19 subscribeVcsStatus
09:39:43.757 in Ack 18
09:39:43.804 chunk 19 1632 bytes
09:39:43.804 in Ack 19
09:39:43.924 chunk 18 11260 bytes
09:39:43.926 in Ack 18
09:39:44.005 rpc 20 server.reportClientActivity
09:39:44.174 chunk 18 13498 bytes
09:39:44.178 in Ack 18
09:39:44.425 chunk 18 19020 bytes
09:39:44.428 in Ack 18
09:39:44.615 exit 20 {"_tag":"Success","value":null}
09:39:44.704 rpc 21 attachments.createUploadUrl
09:39:44.761 fetch POST /api/observability/v1/traces
09:39:44.761 fetched POST /api/observability/v1/traces 204
09:39:44.761 chunk 18 21100 bytes
09:39:44.782 in Ack 18
09:39:44.804 in Interrupt 19
09:39:44.808 rpc 22 subscribeVcsStatus
09:39:44.879 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-263bf981-c568-49b8-998d-2397a3cb3325","expiresAt":1794217784835,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:39:44.879 chunk 22 1632 bytes
09:39:44.881 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTI2M2JmOTgxLWM1NjgtNDliOC05OThkLTIzOTdhM2NiMzMyNSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzc4NDgzNX0.KTpMk_5q-sfQD_zX14uYIl9ByyuPdK5cLsHQ42WY6LQ
09:39:44.882 in Ack 22
09:39:45.059 rpc 23 server.reportClientActivity
09:39:45.796 fetch POST /api/observability/v1/traces
09:39:45.797 fetched POST /api/observability/v1/traces 204
09:39:46.027 exit 23 {"_tag":"Success","value":null}
09:39:46.258 in Ping 
09:39:47.033 fetch POST /api/observability/v1/traces
09:39:47.033 fetched POST /api/observability/v1/traces 204
09:39:48.131 rpc 24 orchestration.dispatchCommand message.dispatch
09:39:48.455 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:39:48.458 rpc 25 attachments.delete
09:39:49.129 fetch POST /api/observability/v1/traces
09:39:49.129 fetched POST /api/observability/v1/traces 204
09:39:49.301 chunk 18 23555 bytes
09:39:49.302 in Ack 18
09:39:49.552 chunk 18 23557 bytes
09:39:49.553 in Ack 18
09:39:49.645 exit 25 {"_tag":"Success","value":null}
09:39:49.802 chunk 7 4617 bytes
09:39:49.803 chunk 18 27322 bytes
09:39:49.804 in Ack 7
09:39:49.903 in Ack 18
09:39:49.911 rpc 26 assets.createUrl
09:39:50.053 chunk 7 4614 bytes
09:39:50.053 chunk 18 27982 bytes
09:39:50.053 in Ack 7
09:39:50.055 rpc 27 orchestration.dispatchCommand thread.visit
09:39:50.055 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:39:50.067 in Ack 18
09:39:50.303 fetch POST /api/observability/v1/traces
09:39:50.303 fetched POST /api/observability/v1/traces 204
09:39:50.801 chunk 7 4614 bytes
09:39:50.802 chunk 18 30405 bytes
09:39:50.802 in Ack 7
09:39:50.811 in Ack 18
09:39:51.091 chunk 18 31215 bytes
09:39:51.094 in Ack 18
09:39:51.292 in Ping 
09:39:51.819 fetch POST /api/observability/v1/traces
09:39:51.819 fetched POST /api/observability/v1/traces 204
09:39:52.598 rpc 28 server.refreshProviders
09:39:52.598 exit 28 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:39:17.794Z","configurableProviders":false,"con
09:39:53.012 rpc 29 orchestration.dispatchCommand message.dispatch
09:39:53.307 chunk 18 31993 bytes
09:39:53.311 in Ack 18
09:39:53.328 in Interrupt 22
09:39:53.337 rpc 30 subscribeVcsStatus
09:39:53.338 fetch POST /api/observability/v1/traces
09:39:53.338 fetched POST /api/observability/v1/traces 204
09:39:53.587 rpc 31 server.reportClientActivity
09:39:54.563 chunk 18 32559 bytes
09:39:54.566 in Ack 18
09:39:54.588 fetch POST /api/observability/v1/traces
09:39:54.588 fetched POST /api/observability/v1/traces 204
09:39:55.066 chunk 18 33576 bytes
09:39:55.067 in Ack 18
09:39:55.314 chunk 18 34487 bytes
09:39:55.316 in Ack 18
09:39:56.067 chunk 7 4678 bytes
09:39:56.068 in Ack 7
09:39:56.083 fetch POST /api/observability/v1/traces
09:39:56.083 fetched POST /api/observability/v1/traces 204
09:39:56.292 in Ping 
09:39:56.672 chunk 18 34487 bytes
09:39:56.673 in Ack 18
09:39:56.921 chunk 18 35260 bytes
09:39:56.923 in Ack 18
09:39:57.372 exit 29 {"_tag":"Success","value":{"sequence":151}}
09:39:57.500 chunk 30 1713 bytes
09:39:57.501 in Ack 30
09:39:57.676 fetch POST /api/observability/v1/traces
09:39:57.676 fetched POST /api/observability/v1/traces 204
09:39:58.177 chunk 18 36736 bytes
09:39:58.180 in Ack 18
09:39:58.427 chunk 18 38757 bytes
09:39:58.430 in Ack 18
09:39:59.182 fetch POST /api/observability/v1/traces
09:39:59.182 fetched POST /api/observability/v1/traces 204
09:40:01.292 in Ping 
09:40:03.335 fetch POST /api/observability/v1/traces
09:40:03.335 fetched POST /api/observability/v1/traces 204
09:40:05.207 fetch POST /api/observability/v1/traces
09:40:05.207 fetched POST /api/observability/v1/traces 204
09:40:05.604 chunk 7 4742 bytes
09:40:05.604 in Ack 7
09:40:05.606 rpc 32 orchestration.dispatchCommand thread.visit
09:40:05.606 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:40:06.292 in Ping 
09:40:06.606 fetch POST /api/observability/v1/traces
09:40:06.606 fetched POST /api/observability/v1/traces 204
09:40:06.856 chunk 18 40198 bytes
09:40:06.858 in Ack 18
09:40:07.107 chunk 18 40200 bytes
09:40:07.109 in Ack 18
09:40:07.357 chunk 18 40725 bytes
09:40:07.359 in Ack 18
09:40:07.608 chunk 7 4742 bytes
09:40:07.609 in Ack 7
09:40:07.859 fetch POST /api/observability/v1/traces
09:40:07.859 fetched POST /api/observability/v1/traces 204
09:40:08.358 chunk 7 4742 bytes
09:40:08.359 in Ack 7
09:40:08.588 in Interrupt 31
09:40:08.840 rpc 33 server.reportClientActivity
09:40:09.360 fetch POST /api/observability/v1/traces
09:40:09.360 fetched POST /api/observability/v1/traces 204
09:40:09.613 chunk 18 43768 bytes
09:40:09.618 in Ack 18
09:40:09.863 chunk 18 44138 bytes
09:40:09.865 in Ack 18
09:40:10.619 fetch POST /api/observability/v1/traces
09:40:10.619 fetched POST /api/observability/v1/traces 204
09:40:11.118 chunk 18 44090 bytes
09:40:11.122 in Ack 18
09:40:11.293 in Ping 
09:40:11.861 chunk 7 4742 bytes
09:40:11.861 in Ack 7
09:40:12.123 fetch POST /api/observability/v1/traces
09:40:12.123 fetched POST /api/observability/v1/traces 204
09:40:13.343 fetch POST /api/observability/v1/traces
09:40:13.343 fetched POST /api/observability/v1/traces 204
09:40:15.610 rpc 34 orchestration.dispatchCommand thread.visit
09:40:15.610 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:40:15.722 exit 31 {"_tag":"Success","value":null}
09:40:15.936 chunk 7 4742 bytes
09:40:15.937 in Ack 7
09:40:16.194 chunk 18 44090 bytes
09:40:16.196 in Ack 18
09:40:16.293 in Ping 
09:40:16.610 fetch POST /api/observability/v1/traces
09:40:16.610 fetched POST /api/observability/v1/traces 204
09:40:18.800 chunk 18 45134 bytes
09:40:18.803 in Ack 18
09:40:19.300 chunk 7 4742 bytes
09:40:19.301 in Ack 7
09:40:19.804 fetch POST /api/observability/v1/traces
09:40:19.804 fetched POST /api/observability/v1/traces 204
09:40:21.293 in Ping 
09:40:23.015 exit 33 {"_tag":"Success","value":null}
09:40:23.153 chunk 7 4742 bytes
09:40:23.154 chunk 18 45134 bytes
09:40:23.155 in Ack 7
09:40:23.164 in Ack 18
09:40:23.346 fetch POST /api/observability/v1/traces
09:40:23.346 fetched POST /api/observability/v1/traces 204
09:40:24.473 chunk 18 46007 bytes
09:40:24.486 in Ack 18
09:40:24.721 chunk 18 47540 bytes
09:40:24.723 in Ack 18
09:40:24.972 chunk 18 47666 bytes
09:40:24.974 in Ack 18
09:40:25.500 fetch POST /api/observability/v1/traces
09:40:25.501 fetched POST /api/observability/v1/traces 204
09:40:25.750 chunk 18 48668 bytes
09:40:25.752 in Ack 18
09:40:26.003 chunk 18 52155 bytes
09:40:26.006 in Ack 18
09:40:26.247 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:40:26.307 in Ping 
09:40:26.753 fetch POST /api/observability/v1/traces
09:40:26.753 fetched POST /api/observability/v1/traces 204
09:40:27.264 chunk 18 54224 bytes
09:40:27.265 in Ack 18
09:40:28.017 chunk 18 54274 bytes
09:40:28.021 in Ack 18
09:40:28.266 fetch POST /api/observability/v1/traces
09:40:28.266 fetched POST /api/observability/v1/traces 204
09:40:28.864 exit 35 {"_tag":"Success","value":{"sequence":191}}
09:40:29.614 chunk 7 4744 bytes
09:40:29.615 in Ack 7
09:40:29.617 rpc 36 orchestration.dispatchCommand thread.visit
09:40:29.617 exit 36 {"_tag":"Success","value":{"sequence":0}}
09:40:29.865 fetch POST /api/observability/v1/traces
09:40:29.865 fetched POST /api/observability/v1/traces 204
09:40:29.996 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:40:30.118 chunk 18 54322 bytes
09:40:30.121 in Ack 18
09:40:30.997 fetch POST /api/observability/v1/traces
09:40:30.997 fetched POST /api/observability/v1/traces 204
09:40:31.308 in Ping 
09:40:31.483 rpc 38 server.reportClientActivity
09:40:32.484 fetch POST /api/observability/v1/traces
09:40:32.484 fetched POST /api/observability/v1/traces 204
09:40:32.975 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:40:33.179 chunk 7 4744 bytes
09:40:33.180 in Ack 7
09:40:33.750 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:40:33.979 fetch POST /api/observability/v1/traces
09:40:33.980 fetched POST /api/observability/v1/traces 204
09:40:34.357 chunk 18 54322 bytes
09:40:34.360 in Ack 18
09:40:34.614 chunk 18 54370 bytes
09:40:34.617 in Ack 18
09:40:34.628 in Interrupt 30
09:40:34.664 rpc 40 subscribeVcsStatus
09:40:35.361 fetch POST /api/observability/v1/traces
09:40:35.361 fetched POST /api/observability/v1/traces 204
09:40:36.310 in Ping 
09:40:38.196 chunk 18 54413 bytes
09:40:38.198 in Ack 18
09:40:38.480 chunk 40 1713 bytes
09:40:38.481 in Ack 40
09:40:38.696 chunk 18 56779 bytes
09:40:38.698 in Ack 18
09:40:39.199 fetch POST /api/observability/v1/traces
09:40:39.199 fetched POST /api/observability/v1/traces 204
09:40:39.642 chunk 18 57327 bytes
09:40:39.644 in Ack 18
09:40:39.905 chunk 18 57359 bytes
09:40:39.909 in Ack 18
09:40:40.182 chunk 18 57601 bytes
09:40:40.184 in Ack 18
09:40:40.406 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:40:40.431 chunk 7 4744 bytes
09:40:40.432 chunk 18 58556 bytes
09:40:40.433 in Ack 7
09:40:40.436 rpc 41 orchestration.dispatchCommand thread.visit
09:40:40.436 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:40:40.440 in Ack 18
09:40:40.646 fetch POST /api/observability/v1/traces
09:40:40.646 fetched POST /api/observability/v1/traces 204
09:40:41.091 chunk 18 59540 bytes
09:40:41.093 in Ack 18
09:40:41.314 in Ping 
09:40:41.846 chunk 18 60933 bytes
09:40:41.848 in Ack 18
09:40:42.095 fetch POST /api/observability/v1/traces
09:40:42.095 fetched POST /api/observability/v1/traces 204
09:40:42.096 chunk 18 60934 bytes
09:40:42.098 in Ack 18
09:40:42.847 chunk 18 61570 bytes
09:40:42.850 in Ack 18
09:40:43.099 fetch POST /api/observability/v1/traces
09:40:43.099 fetched POST /api/observability/v1/traces 204
09:40:43.174 chunk 18 61690 bytes
09:40:43.177 in Ack 18
09:40:43.425 chunk 18 61915 bytes
09:40:43.427 in Ack 18
09:40:43.677 chunk 18 61951 bytes
09:40:43.679 in Ack 18
09:40:44.194 fetch POST /api/observability/v1/traces
09:40:44.194 fetched POST /api/observability/v1/traces 204
