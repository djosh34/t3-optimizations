# case70, webapp, warm start, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 78 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.6s |
| Prompt reached the agent | 2.8s |
| Time to first text | 20.6s |
| Whole reply shown | 77.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.9s / 9.1s / 17.9s |
| Longest silence | 20.6s |
| Approval shown after it was asked | 18.1s |
| Bytes on the wire down / up | 65119 / 35805 |
| Bytes the app received / sent | 48074 / 88635 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.1 | 0 | 287 |
| 20 | 17.1 | 0 | 685 |
| 30 | 21.9 | 211 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 1.0 | 1077 | 1120 |
| 60 | 10.1 | 1117 | 1497 |
| 70 | 7.7 | 1571 | 1596 |
| 77 | 14.7 | 1571 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164640083,"silences":0,"longestSilenceMs":1230,"longestRecoveredGapMs":1230},"stats":{"sendRate":4500,"deliveredRate":2086.996336996337,"loss":0,"srttMs":6.585981181441657,"minRttMs":1,"sentPackets":95,"sentWireBytes":31593,"receivedPackets":108,"receivedWireBytes":51247,"lastHeardMs":8445,"unsyncedBytes":1186,"inflightPackets":14,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 70/260","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164650146,"silences":0,"longestSilenceMs":1527,"longestRecoveredGapMs":1527},"stats":{"sendRate":2250,"deliveredRate":1229.7785265194343,"loss":0.30818181818181817,"srttMs":741.4726397367862,"minRttMs":1,"sentPackets":108,"sentWireBytes":39358,"receivedPackets":123,"receivedWireBytes":60294,"lastHeardMs":18507,"unsyncedBytes":225,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 566/828","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164660876,"silences":2,"longestSilenceMs":4764,"longestRecoveredGapMs":4764},"stats":{"sendRate":1666.5034237492332,"deliveredRate":1229.7785265194343,"loss":0.30818181818181817,"srttMs":816.5385597696879,"minRttMs":1,"sentPackets":127,"sentWireBytes":44735,"receivedPackets":128,"receivedWireBytes":62152,"lastHeardMs":29238,"unsyncedBytes":390,"inflightPackets":37,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058 913/1073 99/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 1167/1253","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164670958,"silences":2,"longestSilenceMs":4764,"longestRecoveredGapMs":4764},"stats":{"sendRate":2055.635597783623,"deliveredRate":588.8768814232549,"loss":0.6077832867132866,"srttMs":926.909621095709,"minRttMs":1,"sentPackets":179,"sentWireBytes":54311,"receivedPackets":154,"receivedWireBytes":70333,"lastHeardMs":39319,"unsyncedBytes":0,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058 1040/1073 399/849 638/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734","runtimeRequests":"897/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164681033,"silences":2,"longestSilenceMs":4764,"longestRecoveredGapMs":4764},"stats":{"sendRate":2055.635597783623,"deliveredRate":443.15357603242296,"loss":0.6825911578421577,"srttMs":926.909621095709,"minRttMs":1,"sentPackets":229,"sentWireBytes":61445,"receivedPackets":179,"receivedWireBytes":81431,"lastHeardMs":49394,"unsyncedBytes":0,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058 1040/1073 399/849 1468/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164690805,"silences":2,"longestSilenceMs":4764,"longestRecoveredGapMs":4764},"stats":{"sendRate":1125,"deliveredRate":0,"loss":0.7201215027972027,"srttMs":2653.248634385258,"minRttMs":1,"sentPackets":279,"sentWireBytes":76849,"receivedPackets":207,"receivedWireBytes":91224,"lastHeardMs":59167,"unsyncedBytes":543,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058 1040/1073 640/849 1505/1546 45/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 1340/1340 369/604","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164701917,"silences":4,"longestSilenceMs":4764,"longestRecoveredGapMs":4764},"stats":{"sendRate":1406.25,"deliveredRate":331.38020513603954,"loss":0.7526666666666666,"srttMs":2207.0347228428045,"minRttMs":1093,"sentPackets":314,"sentWireBytes":89989,"receivedPackets":217,"receivedWireBytes":95407,"lastHeardMs":70279,"unsyncedBytes":0,"inflightPackets":88,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 860/860 870/870 1299/1299 1098/1098 1058/1058 1040/1073 640/849 1505/1546 45/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 546/546 224/901","assistant":"32/32 21/21 1340/1340 713/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019 809/809 348/1019","providerTurns":"774/774 518/774","providerThreads":"808/808","providerSessions":"2696/2696","checkpoints":"613/613 525/613 779/779","checkpointScopes":"112/734","runtimeRequests":"169/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":240,"lastDataAt":1794164710140,"silences":5,"longestSilenceMs":5162,"longestRecoveredGapMs":5162},"stats":{"sendRate":3164.0625,"deliveredRate":0,"loss":0.7526666666666666,"srttMs":2164.155382487454,"minRttMs":1093,"sentPackets":388,"sentWireBytes":108198,"receivedPackets":222,"receivedWireBytes":97187,"lastHeardMs":78501,"unsyncedBytes":173,"inflightPackets":130,"duplicateBytes":0}}
- shim trace
19:03:51.935 fetch GET /api/auth/session
19:03:51.937 fetched GET /api/auth/session 200
19:03:52.076 fetch GET /.well-known/t3/environment
19:03:52.076 fetched GET /.well-known/t3/environment 200
19:03:52.085 fetch GET /.well-known/t3/environment
19:03:52.086 fetched GET /.well-known/t3/environment 200
19:03:52.089 rpc 0 subscribeServerConfig
19:03:52.090 chunk 0 38722 bytes
19:03:52.106 in Ack 0
19:03:52.107 rpc 1 subscribeServerLifecycle
19:03:52.108 rpc 2 subscribeServerLifecycle
19:03:52.110 fetch GET /api/auth/session
19:03:52.111 fetched GET /api/auth/session 200
19:03:52.111 rpc 3 subscribeDeviceState
19:03:52.113 rpc 4 preview.reportProfiles
19:03:52.147 rpc 5 subscribeProjectClones
19:03:52.164 fetch GET /api/orchestration/threads/mcp%3Af6c3e0eb-59d3-4f33-8d7f-b39944e02993/bounded
19:03:52.168 fetch GET /api/orchestration/shell
19:03:52.187 rpc 6 server.refreshProviders
19:03:52.223 rpc 7 orchestration.dispatchCommand thread.visit
19:03:52.223 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:03:52.232 fetched GET /api/orchestration/shell 200
19:03:52.232 rpc 8 subscribeTerminalMetadata
19:03:52.233 rpc 9 subscribeVcsStatus
19:03:52.234 rpc 10 subscribePreviewEvents
19:03:52.235 rpc 11 orchestration.subscribeShell
19:03:52.236 chunk 11 2790 bytes
19:03:52.236 rpc 12 assets.createUrl
19:03:52.237 rpc 13 preview.list
19:03:52.237 rpc 14 projects.readFile
19:03:52.241 in Ack 11
19:03:52.330 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"afb77d8c-430a-4e0c-a5a6-8158d5b42140","sessions":[]}}
19:03:52.330 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:03:52.448 chunk 1 3872 bytes
19:03:52.449 in Ack 1
19:03:52.485 rpc 15 server.reportClientActivity
19:03:52.486 chunk 11 2766 bytes
19:03:52.486 in Ack 11
19:03:52.817 chunk 2 3872 bytes
19:03:52.817 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:03:39.692Z","configurableProviders":false,"con
19:03:52.817 chunk 9 796 bytes
19:03:52.818 in Ack 2
19:03:52.821 in Ack 9
19:03:52.938 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:03:53.074 fetch POST /api/observability/v1/traces
19:03:53.074 fetched POST /api/observability/v1/traces 204
19:03:53.168 chunk 3 876 bytes
19:03:53.168 exit 4 {"_tag":"Success","value":null}
19:03:53.168 chunk 5 4 bytes
19:03:53.168 chunk 8 36 bytes
19:03:53.168 exit 15 {"_tag":"Success","value":null}
19:03:53.168 in Ack 3
19:03:53.171 rpc 16 preview.reportProfiles
19:03:53.172 in Ack 5
19:03:53.172 in Ack 8
19:03:53.188 chunk 11 2809 bytes
19:03:53.189 in Ack 11
19:03:53.245 exit 16 {"_tag":"Success","value":null}
19:03:53.298 fetched GET /api/orchestration/threads/mcp%3Af6c3e0eb-59d3-4f33-8d7f-b39944e02993/bounded 200
19:03:53.302 rpc 17 orchestration.subscribeThread
19:03:53.302 chunk 17 8283 bytes
19:03:53.311 in Interrupt 9
19:03:53.314 rpc 18 subscribeVcsStatus
19:03:53.357 in Ack 17
19:03:53.448 chunk 18 796 bytes
19:03:53.449 in Ack 18
19:03:53.553 chunk 17 11920 bytes
19:03:53.554 in Ack 17
19:03:53.604 rpc 19 server.reportClientActivity
19:03:53.804 chunk 17 13502 bytes
19:03:53.805 in Ack 17
19:03:54.048 rpc 20 attachments.createUploadUrl
19:03:54.208 chunk 17 18027 bytes
19:03:54.213 exit 19 {"_tag":"Success","value":null}
19:03:54.214 fetch POST /api/observability/v1/traces
19:03:54.214 fetched POST /api/observability/v1/traces 204
19:03:54.217 in Ack 17
19:03:54.338 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-d71ae871-4366-4d1b-a63c-6a84574153bb","expiresAt":1794165234258,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:03:54.339 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWQ3MWFlODcxLTQzNjYtNGQxYi1hNjNjLTZhODQ1NzQxNTNiYiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NTIzNDI1OH0.I5ZcNbWkX0IlKSTmNYy2advC5O0y99qdwXfNU_A_DlA
19:03:54.458 chunk 17 21199 bytes
19:03:54.459 in Ack 17
19:03:54.466 in Interrupt 18
19:03:54.467 rpc 21 subscribeVcsStatus
19:03:54.529 chunk 21 796 bytes
19:03:54.530 in Ack 21
19:03:54.718 rpc 22 server.reportClientActivity
19:03:55.218 fetch POST /api/observability/v1/traces
19:03:55.218 fetched POST /api/observability/v1/traces 204
19:03:55.355 exit 22 {"_tag":"Success","value":null}
19:03:56.356 fetch POST /api/observability/v1/traces
19:03:56.356 fetched POST /api/observability/v1/traces 204
19:03:57.089 in Ping 
19:03:57.243 rpc 23 orchestration.dispatchCommand message.dispatch
19:03:57.839 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:03:57.840 rpc 24 attachments.delete
19:03:58.242 fetch POST /api/observability/v1/traces
19:03:58.242 fetched POST /api/observability/v1/traces 204
19:03:58.343 chunk 17 23653 bytes
19:03:58.344 in Ack 17
19:03:58.596 chunk 17 23665 bytes
19:03:58.598 in Ack 17
19:03:58.846 chunk 17 23667 bytes
19:03:58.847 in Ack 17
19:03:59.055 exit 24 {"_tag":"Success","value":null}
19:03:59.096 chunk 11 2831 bytes
19:03:59.097 chunk 17 24639 bytes
19:03:59.097 in Ack 11
19:03:59.098 rpc 25 orchestration.dispatchCommand thread.visit
19:03:59.098 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:03:59.137 in Ack 17
19:03:59.345 fetch POST /api/observability/v1/traces
19:03:59.345 fetched POST /api/observability/v1/traces 204
19:03:59.347 chunk 17 28093 bytes
19:03:59.349 in Ack 17
19:03:59.351 rpc 26 assets.createUrl
19:03:59.598 chunk 11 2831 bytes
19:03:59.598 chunk 17 30516 bytes
19:03:59.599 in Ack 11
19:03:59.603 in Ack 17
19:04:00.071 chunk 17 32106 bytes
19:04:00.075 in Ack 17
19:04:00.081 in Interrupt 21
19:04:00.083 rpc 27 subscribeVcsStatus
19:04:00.324 chunk 17 31940 bytes
19:04:00.325 in Ack 17
19:04:00.337 rpc 28 server.reportClientActivity
19:04:00.350 fetch POST /api/observability/v1/traces
19:04:00.350 fetched POST /api/observability/v1/traces 204
19:04:00.754 rpc 29 orchestration.dispatchCommand message.dispatch
19:04:01.754 fetch POST /api/observability/v1/traces
19:04:01.754 fetched POST /api/observability/v1/traces 204
19:04:02.089 in Ping 
19:04:02.564 chunk 17 32508 bytes
19:04:02.565 in Ack 17
19:04:03.353 chunk 27 796 bytes
19:04:03.354 in Ack 27
19:04:03.567 fetch POST /api/observability/v1/traces
19:04:03.567 fetched POST /api/observability/v1/traces 204
19:04:04.815 fetch POST /api/observability/v1/traces
19:04:04.815 fetched POST /api/observability/v1/traces 204
19:04:07.102 in Ping 
19:04:07.168 chunk 17 34964 bytes
19:04:07.168 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:04:07.170 in Ack 17
19:04:07.596 chunk 17 34984 bytes
19:04:07.597 in Ack 17
19:04:08.171 fetch POST /api/observability/v1/traces
19:04:08.171 fetched POST /api/observability/v1/traces 204
19:04:08.189 chunk 17 35016 bytes
19:04:08.190 in Ack 17
19:04:08.620 chunk 17 35048 bytes
19:04:08.621 in Ack 17
19:04:09.191 fetch POST /api/observability/v1/traces
19:04:09.191 fetched POST /api/observability/v1/traces 204
19:04:10.147 chunk 17 35080 bytes
19:04:10.148 in Ack 17
19:04:11.149 fetch POST /api/observability/v1/traces
19:04:11.149 fetched POST /api/observability/v1/traces 204
19:04:11.403 chunk 17 35102 bytes
19:04:11.404 in Ack 17
19:04:12.103 in Ping 
19:04:12.405 fetch POST /api/observability/v1/traces
19:04:12.405 fetched POST /api/observability/v1/traces 204
19:04:14.819 fetch POST /api/observability/v1/traces
19:04:14.819 fetched POST /api/observability/v1/traces 204
19:04:15.337 in Interrupt 28
19:04:16.167 chunk 17 35508 bytes
19:04:16.169 in Ack 17
19:04:16.338 fetch POST /api/observability/v1/traces
19:04:16.338 fetched POST /api/observability/v1/traces 204
19:04:17.104 in Ping 
19:04:17.328 rpc 30 server.reportClientActivity
19:04:18.328 fetch POST /api/observability/v1/traces
19:04:18.328 fetched POST /api/observability/v1/traces 204
19:04:20.375 chunk 17 35708 bytes
19:04:20.381 in Ack 17
19:04:20.632 chunk 17 36060 bytes
19:04:20.633 in Ack 17
19:04:20.881 chunk 17 36080 bytes
19:04:20.882 in Ack 17
19:04:21.133 chunk 17 36326 bytes
19:04:21.134 in Ack 17
19:04:21.382 fetch POST /api/observability/v1/traces
19:04:21.382 fetched POST /api/observability/v1/traces 204
19:04:21.391 chunk 17 36456 bytes
19:04:21.392 in Ack 17
19:04:21.873 chunk 17 36488 bytes
19:04:21.874 in Ack 17
19:04:22.104 in Ping 
19:04:22.123 chunk 17 36498 bytes
19:04:22.124 in Ack 17
19:04:22.393 fetch POST /api/observability/v1/traces
19:04:22.393 fetched POST /api/observability/v1/traces 204
19:04:22.549 chunk 17 36522 bytes
19:04:22.550 in Ack 17
19:04:23.551 fetch POST /api/observability/v1/traces
19:04:23.551 fetched POST /api/observability/v1/traces 204
19:04:23.736 chunk 17 36670 bytes
19:04:23.737 in Ack 17
19:04:24.036 chunk 17 36700 bytes
19:04:24.037 in Ack 17
19:04:24.455 chunk 17 36776 bytes
19:04:24.456 in Ack 17
19:04:24.713 chunk 17 36786 bytes
19:04:24.714 in Ack 17
19:04:24.744 fetch POST /api/observability/v1/traces
19:04:24.744 fetched POST /api/observability/v1/traces 204
19:04:24.977 chunk 17 36796 bytes
19:04:24.978 in Ack 17
19:04:25.692 chunk 17 36936 bytes
19:04:25.694 in Ack 17
19:04:25.979 fetch POST /api/observability/v1/traces
19:04:25.979 fetched POST /api/observability/v1/traces 204
19:04:26.322 chunk 17 36988 bytes
19:04:26.323 in Ack 17
19:04:26.974 chunk 17 37030 bytes
19:04:26.975 in Ack 17
19:04:27.104 in Ping 
19:04:27.232 chunk 17 37040 bytes
19:04:27.233 in Ack 17
19:04:27.324 fetch POST /api/observability/v1/traces
19:04:27.324 fetched POST /api/observability/v1/traces 204
19:04:27.484 chunk 17 37086 bytes
19:04:27.485 in Ack 17
19:04:27.735 chunk 17 37118 bytes
19:04:27.736 in Ack 17
19:04:28.486 fetch POST /api/observability/v1/traces
19:04:28.486 fetched POST /api/observability/v1/traces 204
19:04:28.491 chunk 17 37158 bytes
19:04:28.492 in Ack 17
19:04:28.750 chunk 17 37168 bytes
19:04:28.751 in Ack 17
19:04:29.140 chunk 17 37178 bytes
19:04:29.141 in Ack 17
19:04:29.493 fetch POST /api/observability/v1/traces
19:04:29.493 fetched POST /api/observability/v1/traces 204
19:04:29.816 chunk 17 37218 bytes
19:04:29.817 in Ack 17
19:04:30.068 chunk 17 37228 bytes
19:04:30.069 in Ack 17
19:04:30.461 chunk 17 37240 bytes
19:04:30.462 in Ack 17
19:04:30.711 chunk 17 37250 bytes
19:04:30.712 in Ack 17
19:04:30.818 fetch POST /api/observability/v1/traces
19:04:30.818 fetched POST /api/observability/v1/traces 204
19:04:30.961 chunk 17 37284 bytes
19:04:30.962 in Ack 17
19:04:31.214 chunk 17 37354 bytes
19:04:31.215 in Ack 17
19:04:31.567 chunk 17 37364 bytes
19:04:31.569 in Ack 17
19:04:31.817 chunk 17 37384 bytes
19:04:31.818 in Ack 17
19:04:31.964 fetch POST /api/observability/v1/traces
19:04:31.964 fetched POST /api/observability/v1/traces 204
19:04:32.105 in Ping 
19:04:32.329 in Interrupt 30
19:04:32.396 chunk 17 37396 bytes
19:04:32.398 in Ack 17
19:04:32.649 chunk 17 37406 bytes
19:04:32.650 in Ack 17
19:04:32.903 chunk 17 37438 bytes
19:04:32.904 in Ack 17
19:04:33.330 fetch POST /api/observability/v1/traces
19:04:33.330 fetched POST /api/observability/v1/traces 204
19:04:33.410 chunk 17 37450 bytes
19:04:33.411 in Ack 17
19:04:33.921 chunk 17 37460 bytes
19:04:33.922 in Ack 17
19:04:34.171 chunk 17 37560 bytes
19:04:34.172 in Ack 17
19:04:34.412 fetch POST /api/observability/v1/traces
19:04:34.412 fetched POST /api/observability/v1/traces 204
19:04:34.939 chunk 17 37568 bytes
19:04:34.940 in Ack 17
19:04:35.470 chunk 17 37586 bytes
19:04:35.471 in Ack 17
19:04:35.941 fetch POST /api/observability/v1/traces
19:04:35.941 fetched POST /api/observability/v1/traces 204
19:04:37.105 in Ping 
19:04:39.816 chunk 17 37612 bytes
19:04:39.817 in Ack 17
19:04:40.421 chunk 17 37620 bytes
19:04:40.422 in Ack 17
19:04:40.818 fetch POST /api/observability/v1/traces
19:04:40.818 fetched POST /api/observability/v1/traces 204
19:04:42.105 in Ping 
19:04:42.329 rpc 31 server.reportClientActivity
19:04:43.329 fetch POST /api/observability/v1/traces
19:04:43.329 fetched POST /api/observability/v1/traces 204
19:04:44.830 fetch POST /api/observability/v1/traces
19:04:44.830 fetched POST /api/observability/v1/traces 204
19:04:47.108 in Ping 
19:04:48.362 chunk 17 38625 bytes
19:04:48.363 in Ack 17
19:04:48.549 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:04:49.367 fetch POST /api/observability/v1/traces
19:04:49.367 fetched POST /api/observability/v1/traces 204
19:04:52.082 chunk 17 41195 bytes
19:04:52.082 exit 32 {"_tag":"Success","value":{"sequence":95}}
19:04:52.083 in Ack 17
19:04:52.133 in Ping 
19:04:52.278 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:04:52.947 chunk 17 41287 bytes
19:04:52.948 in Ack 17
19:04:53.084 fetch POST /api/observability/v1/traces
19:04:53.084 fetched POST /api/observability/v1/traces 204
19:04:54.832 fetch POST /api/observability/v1/traces
19:04:54.832 fetched POST /api/observability/v1/traces 204
19:04:55.894 chunk 17 41319 bytes
19:04:55.895 in Ack 17
19:04:56.320 chunk 17 41349 bytes
19:04:56.321 in Ack 17
19:04:56.897 fetch POST /api/observability/v1/traces
19:04:56.897 fetched POST /api/observability/v1/traces 204
19:04:57.136 in Ping 
19:04:57.330 in Interrupt 31
19:04:57.984 chunk 17 41411 bytes
19:04:57.985 in Ack 17
19:04:58.330 fetch POST /api/observability/v1/traces
19:04:58.330 fetched POST /api/observability/v1/traces 204
19:05:00.875 chunk 17 41609 bytes
19:05:00.876 in Ack 17
19:05:01.133 chunk 17 41639 bytes
19:05:01.135 in Ack 17
19:05:01.877 fetch POST /api/observability/v1/traces
19:05:01.877 fetched POST /api/observability/v1/traces 204
19:05:01.918 chunk 17 41805 bytes
19:05:01.919 in Ack 17
19:05:02.140 in Ping 
19:05:02.902 chunk 17 41919 bytes
19:05:02.903 in Ack 17
19:05:02.974 fetch POST /api/observability/v1/traces
19:05:02.974 fetched POST /api/observability/v1/traces 204
19:05:03.213 chunk 17 41929 bytes
19:05:03.215 in Ack 17
19:05:03.631 chunk 17 42179 bytes
19:05:03.632 in Ack 17
19:05:04.216 fetch POST /api/observability/v1/traces
19:05:04.216 fetched POST /api/observability/v1/traces 204
19:05:04.979 chunk 17 42441 bytes
19:05:04.980 in Ack 17
19:05:05.981 fetch POST /api/observability/v1/traces
19:05:05.981 fetched POST /api/observability/v1/traces 204
19:05:07.144 in Ping 
19:05:07.329 rpc 34 server.reportClientActivity
19:05:08.329 fetch POST /api/observability/v1/traces
19:05:08.329 fetched POST /api/observability/v1/traces 204
19:05:10.141 chunk 17 42531 bytes
19:05:10.143 in Ack 17
19:05:11.143 fetch POST /api/observability/v1/traces
19:05:11.143 fetched POST /api/observability/v1/traces 204
19:05:12.148 in Ping 
19:05:14.838 fetch POST /api/observability/v1/traces
19:05:14.838 fetched POST /api/observability/v1/traces 204
19:05:17.153 in Ping 
19:05:17.492 chunk 17 42549 bytes
19:05:17.493 in Ack 17
