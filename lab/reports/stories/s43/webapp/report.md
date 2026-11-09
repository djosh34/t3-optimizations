# story, webapp, warm start, seed 43

Profile story (graded): seed 43's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 62 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 8.7s |
| Prompt reached the agent | 7.2s |
| Time to first text | 21.7s |
| Whole reply shown | 60.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.7s / 6.7s / 16.7s |
| Longest silence | 21.7s |
| Approval shown after it was asked | 9.0s |
| Bytes on the wire down / up | 143980 / 99621 |
| Bytes the app received / sent | 69664 / 66165 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.7 | 0 | 114 |
| 20 | 12.7 | 0 | 514 |
| 30 | 12.7 | 397 | 917 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 5.5 | 1077 | 1297 |
| 60 | 7.7 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217063514,"silences":0,"longestSilenceMs":1214,"longestRecoveredGapMs":1214},"stats":{"sendRate":7225.101628005389,"deliveredRate":3254.2092985658264,"loss":0,"srttMs":5.395719207967021,"minRttMs":1,"sentPackets":108,"sentWireBytes":32396,"receivedPackets":174,"receivedWireBytes":110427,"lastHeardMs":11888,"unsyncedBytes":518,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 693/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217073633,"silences":0,"longestSilenceMs":1214,"longestRecoveredGapMs":1214},"stats":{"sendRate":3612.5508140026946,"deliveredRate":2385.6686414678065,"loss":0.075,"srttMs":1915.0842225185997,"minRttMs":8051,"sentPackets":132,"sentWireBytes":40938,"receivedPackets":227,"receivedWireBytes":123451,"lastHeardMs":22007,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659 307/635","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217083886,"silences":1,"longestSilenceMs":3022,"longestRecoveredGapMs":3022},"stats":{"sendRate":3000,"deliveredRate":1242.6655490360436,"loss":0.0714285714285714,"srttMs":3400.8762739910053,"minRttMs":5065,"sentPackets":181,"sentWireBytes":52054,"receivedPackets":263,"receivedWireBytes":135794,"lastHeardMs":32260,"unsyncedBytes":0,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 49/839","user":"10/10 9/9","attempts":"659/659 659/659 452/635","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217093908,"silences":1,"longestSilenceMs":3022,"longestRecoveredGapMs":3022},"stats":{"sendRate":3000,"deliveredRate":634.7281354720886,"loss":0.3174195652173913,"srttMs":4671.204833064147,"minRttMs":5065,"sentPackets":225,"sentWireBytes":58320,"receivedPackets":309,"receivedWireBytes":153329,"lastHeardMs":42282,"unsyncedBytes":0,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 390/543","providerTurns":"772/772 772/772 365/654","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"329/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217103155,"silences":1,"longestSilenceMs":3022,"longestRecoveredGapMs":3022},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.1684285714285714,"srttMs":3320.1057831007533,"minRttMs":2188,"sentPackets":278,"sentWireBytes":71347,"receivedPackets":331,"receivedWireBytes":162146,"lastHeardMs":51529,"unsyncedBytes":457,"inflightPackets":34,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 125/1093 454/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 390/543","providerTurns":"772/772 772/772 365/654","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":613,"lastDataAt":1794217113101,"silences":1,"longestSilenceMs":3022,"longestRecoveredGapMs":3022},"stats":{"sendRate":3000,"deliveredRate":1640.996957121331,"loss":0.16445970695970696,"srttMs":2682.181027402543,"minRttMs":2188,"sentPackets":348,"sentWireBytes":95223,"receivedPackets":344,"receivedWireBytes":168966,"lastHeardMs":61475,"unsyncedBytes":0,"inflightPackets":54,"duplicateBytes":0}}
- shim trace
09:37:32.468 fetch GET /api/auth/session
09:37:32.471 fetched GET /api/auth/session 200
09:37:32.711 fetch GET /.well-known/t3/environment
09:37:32.712 fetched GET /.well-known/t3/environment 200
09:37:32.727 fetch GET /.well-known/t3/environment
09:37:32.728 fetched GET /.well-known/t3/environment 200
09:37:32.738 rpc 0 subscribeServerConfig
09:37:32.739 chunk 0 38462 bytes
09:37:32.790 in Ack 0
09:37:32.792 rpc 1 subscribeServerLifecycle
09:37:32.793 rpc 2 subscribeServerLifecycle
09:37:32.798 fetch GET /api/auth/session
09:37:32.799 fetched GET /api/auth/session 200
09:37:32.799 rpc 3 subscribeDeviceState
09:37:32.802 rpc 4 preview.reportProfiles
09:37:32.961 rpc 5 subscribeProjectClones
09:37:32.970 fetch GET /api/orchestration/threads/mcp%3A9993107e-02c6-4825-a1d4-8a372dd53f5a/bounded
09:37:33.035 rpc 6 server.reportClientActivity
09:37:33.039 fetch GET /api/orchestration/shell
09:37:33.040 fetched GET /api/orchestration/shell 200
09:37:33.042 rpc 7 orchestration.subscribeShell
09:37:33.042 chunk 7 2794 bytes
09:37:33.045 in Ack 7
09:37:33.293 chunk 7 2770 bytes
09:37:33.293 in Ack 7
09:37:33.318 chunk 1 3815 bytes
09:37:33.325 in Ack 1
09:37:33.456 chunk 2 3815 bytes
09:37:33.456 in Ack 2
09:37:33.708 fetch POST /api/observability/v1/traces
09:37:33.708 fetched POST /api/observability/v1/traces 204
09:37:33.792 chunk 7 4592 bytes
09:37:33.793 in Ack 7
09:37:33.824 rpc 8 server.refreshProviders
09:37:33.881 rpc 9 orchestration.dispatchCommand thread.visit
09:37:33.881 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:33.895 rpc 10 subscribeTerminalMetadata
09:37:33.896 rpc 11 subscribeVcsStatus
09:37:33.897 rpc 12 subscribePreviewEvents
09:37:33.898 rpc 13 assets.createUrl
09:37:33.899 rpc 14 preview.list
09:37:33.900 rpc 15 projects.readFile
09:37:33.996 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"4fe02511-3f69-4842-a9d7-aeea935015ff","sessions":[]}}
09:37:33.996 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:33.996 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:10.126Z","configurableProviders":false,"con
09:37:34.111 chunk 11 1632 bytes
09:37:34.111 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:34.112 in Ack 11
09:37:34.293 chunk 7 4635 bytes
09:37:34.294 in Ack 7
09:37:34.337 chunk 3 876 bytes
09:37:34.337 exit 4 {"_tag":"Success","value":null}
09:37:34.337 chunk 5 4 bytes
09:37:34.337 exit 6 {"_tag":"Success","value":null}
09:37:34.337 chunk 10 36 bytes
09:37:34.338 in Ack 3
09:37:34.346 rpc 16 preview.reportProfiles
09:37:34.347 in Ack 5
09:37:34.348 in Ack 10
09:37:34.404 exit 16 {"_tag":"Success","value":null}
09:37:34.599 rpc 17 server.reportClientActivity
09:37:34.665 exit 17 {"_tag":"Success","value":null}
09:37:34.794 fetch POST /api/observability/v1/traces
09:37:34.794 fetched POST /api/observability/v1/traces 204
09:37:35.340 fetched GET /api/orchestration/threads/mcp%3A9993107e-02c6-4825-a1d4-8a372dd53f5a/bounded 200
09:37:35.345 rpc 18 orchestration.subscribeThread
09:37:35.345 chunk 18 8283 bytes
09:37:35.362 in Interrupt 11
09:37:35.365 rpc 19 subscribeVcsStatus
09:37:35.366 in Ack 18
09:37:35.471 chunk 19 1632 bytes
09:37:35.471 in Ack 19
09:37:35.595 chunk 18 11260 bytes
09:37:35.596 in Ack 18
09:37:35.660 rpc 20 server.reportClientActivity
09:37:35.848 chunk 18 13498 bytes
09:37:35.850 in Ack 18
09:37:36.099 chunk 18 18553 bytes
09:37:36.101 in Ack 18
09:37:36.159 exit 20 {"_tag":"Success","value":null}
09:37:36.229 rpc 21 attachments.createUploadUrl
09:37:36.341 fetch POST /api/observability/v1/traces
09:37:36.341 fetched POST /api/observability/v1/traces 204
09:37:36.349 chunk 18 21100 bytes
09:37:36.350 in Ack 18
09:37:36.360 in Interrupt 19
09:37:36.363 rpc 22 subscribeVcsStatus
09:37:36.364 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-d6c58116-b3e4-4cbb-ae98-24e564d1bd7b","expiresAt":1794217656297,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:36.365 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWQ2YzU4MTE2LWIzZTQtNGNiYi1hZTk4LTI0ZTU2NGQxYmQ3YiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1NjI5N30.A2U7CTZuw6nO06gXXYZeDnx5ofuNjSoYjIz3BXmhB-U
09:37:36.424 chunk 22 1632 bytes
09:37:36.424 in Ack 22
09:37:36.615 rpc 23 server.reportClientActivity
09:37:37.351 fetch POST /api/observability/v1/traces
09:37:37.351 fetched POST /api/observability/v1/traces 204
09:37:37.734 in Ping 
09:37:37.996 exit 23 {"_tag":"Success","value":null}
09:37:39.000 fetch POST /api/observability/v1/traces
09:37:39.000 fetched POST /api/observability/v1/traces 204
09:37:39.181 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:39.438 chunk 18 23893 bytes
09:37:39.438 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:39.442 in Ack 18
09:37:39.446 rpc 25 assets.createUrl
09:37:39.499 rpc 26 attachments.delete
09:37:39.688 chunk 7 4721 bytes
09:37:39.688 chunk 18 26810 bytes
09:37:39.689 in Ack 7
09:37:39.767 in Ack 18
09:37:39.813 exit 26 {"_tag":"Success","value":null}
09:37:39.940 chunk 18 27723 bytes
09:37:39.943 in Ack 18
09:37:40.180 fetch POST /api/observability/v1/traces
09:37:40.180 fetched POST /api/observability/v1/traces 204
09:37:40.190 chunk 7 4660 bytes
09:37:40.191 in Ack 7
09:37:40.202 chunk 18 30204 bytes
09:37:40.204 in Ack 18
09:37:40.453 chunk 18 31903 bytes
09:37:40.457 in Ack 18
09:37:40.588 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/d41d7130-206f-48f5-a766-f379b9ddd769","expiresAt":1794220660588}}
09:37:41.093 chunk 18 32035 bytes
09:37:41.095 in Ack 18
09:37:41.193 fetch POST /api/observability/v1/traces
09:37:41.193 fetched POST /api/observability/v1/traces 204
09:37:41.342 chunk 7 4657 bytes
09:37:41.343 chunk 18 35260 bytes
09:37:41.343 in Ack 7
09:37:41.345 rpc 27 orchestration.dispatchCommand thread.visit
09:37:41.345 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:41.355 in Ack 18
09:37:41.365 in Interrupt 22
09:37:41.369 rpc 28 subscribeVcsStatus
09:37:41.417 chunk 28 1632 bytes
09:37:41.417 in Ack 28
09:37:41.765 chunk 7 4657 bytes
09:37:41.766 chunk 18 35260 bytes
09:37:41.767 rpc 29 server.reportClientActivity
09:37:41.772 in Ack 7
09:37:41.785 in Ack 18
09:37:41.832 exit 29 {"_tag":"Success","value":null}
09:37:42.378 fetch POST /api/observability/v1/traces
09:37:42.378 fetched POST /api/observability/v1/traces 204
09:37:42.743 in Ping 
09:37:43.465 rpc 30 orchestration.dispatchCommand message.dispatch
09:37:44.466 fetch POST /api/observability/v1/traces
09:37:44.466 fetched POST /api/observability/v1/traces 204
09:37:46.896 fetch POST /api/observability/v1/traces
09:37:46.896 fetched POST /api/observability/v1/traces 204
09:37:47.743 in Ping 
09:37:51.787 exit 30 {"_tag":"Success","value":{"sequence":153}}
09:37:52.744 in Ping 
09:37:52.789 fetch POST /api/observability/v1/traces
09:37:52.789 fetched POST /api/observability/v1/traces 204
09:37:57.744 in Ping 
09:37:57.968 rpc 31 server.reportClientActivity
09:37:58.968 fetch POST /api/observability/v1/traces
09:37:58.968 fetched POST /api/observability/v1/traces 204
09:38:02.744 in Ping 
09:38:04.669 chunk 18 38305 bytes
09:38:04.672 in Ack 18
09:38:04.919 chunk 18 38673 bytes
09:38:04.922 in Ack 18
09:38:05.676 fetch POST /api/observability/v1/traces
09:38:05.676 fetched POST /api/observability/v1/traces 204
09:38:07.745 in Ping 
09:38:09.162 chunk 18 38625 bytes
09:38:09.167 in Ack 18
09:38:10.169 fetch POST /api/observability/v1/traces
09:38:10.169 fetched POST /api/observability/v1/traces 204
09:38:12.748 in Ping 
09:38:12.974 in Interrupt 31
09:38:13.975 fetch POST /api/observability/v1/traces
09:38:13.975 fetched POST /api/observability/v1/traces 204
09:38:14.123 chunk 7 4721 bytes
09:38:14.124 in Ack 7
09:38:14.128 rpc 32 orchestration.dispatchCommand thread.visit
09:38:14.128 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:38:14.373 chunk 18 39261 bytes
09:38:14.376 in Ack 18
09:38:15.125 fetch POST /api/observability/v1/traces
09:38:15.125 fetched POST /api/observability/v1/traces 204
09:38:15.878 chunk 18 42122 bytes
09:38:15.880 in Ack 18
09:38:16.096 exit 31 {"_tag":"Success","value":null}
09:38:16.881 fetch POST /api/observability/v1/traces
09:38:16.882 fetched POST /api/observability/v1/traces 204
09:38:17.748 in Ping 
09:38:18.686 chunk 18 42334 bytes
09:38:18.688 in Ack 18
09:38:19.690 fetch POST /api/observability/v1/traces
09:38:19.690 fetched POST /api/observability/v1/traces 204
09:38:19.965 chunk 18 42738 bytes
09:38:19.968 in Ack 18
09:38:20.551 chunk 18 43116 bytes
09:38:20.554 in Ack 18
09:38:20.844 chunk 18 43172 bytes
09:38:20.847 in Ack 18
09:38:20.971 fetch POST /api/observability/v1/traces
09:38:20.971 fetched POST /api/observability/v1/traces 204
09:38:22.748 in Ping 
09:38:22.764 chunk 18 43592 bytes
09:38:22.766 in Ack 18
09:38:22.967 rpc 33 server.reportClientActivity
09:38:23.157 chunk 18 43810 bytes
09:38:23.159 in Ack 18
09:38:23.773 fetch POST /api/observability/v1/traces
09:38:23.773 fetched POST /api/observability/v1/traces 204
09:38:26.340 chunk 18 44812 bytes
09:38:26.342 in Ack 18
09:38:26.601 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:38:27.343 fetch POST /api/observability/v1/traces
09:38:27.343 fetched POST /api/observability/v1/traces 204
09:38:27.750 in Ping 
09:38:29.549 exit 34 {"_tag":"Success","value":{"sequence":193}}
09:38:30.391 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:38:30.550 fetch POST /api/observability/v1/traces
09:38:30.550 fetched POST /api/observability/v1/traces 204
09:38:32.754 in Ping 
09:38:33.102 chunk 18 44855 bytes
09:38:33.103 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:33.104 in Ack 18
09:38:34.106 fetch POST /api/observability/v1/traces
09:38:34.106 fetched POST /api/observability/v1/traces 204
09:38:35.616 chunk 18 44905 bytes
09:38:35.618 in Ack 18
09:38:36.619 fetch POST /api/observability/v1/traces
09:38:36.619 fetched POST /api/observability/v1/traces 204
09:38:37.758 in Ping 
09:38:37.968 in Interrupt 33
09:38:38.401 chunk 18 47864 bytes
09:38:38.404 in Ack 18
09:38:38.969 fetch POST /api/observability/v1/traces
09:38:38.969 fetched POST /api/observability/v1/traces 204
09:38:40.787 chunk 18 48140 bytes
09:38:40.789 in Ack 18
09:38:41.790 fetch POST /api/observability/v1/traces
09:38:41.790 fetched POST /api/observability/v1/traces 204
09:38:42.762 in Ping 
09:38:43.214 chunk 18 48560 bytes
09:38:43.216 in Ack 18
09:38:43.701 chunk 18 48818 bytes
09:38:43.704 in Ack 18
