# duplicate, webapp, warm start, seed 1

Profile duplicate (fault): 20% of packets duplicated, 5% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.7s |
| Time to first text | 1.7s |
| Whole reply shown | 46.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.3s / 1.0s / 1.9s |
| Longest silence | 3.6s |
| Approval shown after it was asked | 1.8s |
| Bytes on the wire down / up | 513569 / 440554 |
| Bytes the app received / sent | 222234 / 56324 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.9 | 333 | 369 |
| 20 | 1.1 | 727 | 770 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 2.0 | 1371 | 1440 |
| 46 | 1.6 | 1590 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 870/870 1058/1058 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774","nodes":"1019/1019 809/809 809/809","providerThreads":"808/808","providerSessions":"2696/2696","messages":"546/546 456/456 463/901","checkpoints":"146/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794167695890,"silences":0,"longestSilenceMs":1437,"longestRecoveredGapMs":1437},"stats":{"sendRate":7027.383290266593,"deliveredRate":3062.2680640083686,"loss":0,"srttMs":4.418476305077693,"minRttMs":1,"sentPackets":96,"sentWireBytes":28045,"receivedPackets":118,"receivedWireBytes":54214,"lastHeardMs":10296,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 870/870 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"834/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 445/455","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 618/774","nodes":"1019/1019 809/809 809/809 785/785","providerThreads":"810/810","providerSessions":"2696/2696","messages":"546/546 456/456 764/901 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794167706005,"silences":0,"longestSilenceMs":1437,"longestRecoveredGapMs":1437},"stats":{"sendRate":2000,"deliveredRate":3066.346153846154,"loss":0,"srttMs":787.8182275684682,"minRttMs":712,"sentPackets":171,"sentWireBytes":45247,"receivedPackets":206,"receivedWireBytes":89459,"lastHeardMs":20411,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 870/870 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 938/938","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","nodes":"1019/1019 809/809 809/809 785/785 1019/1019","providerThreads":"810/810","providerSessions":"2696/2696","messages":"546/546 456/456 901/901 451/451 546/546 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794167716204,"silences":0,"longestSilenceMs":1437,"longestRecoveredGapMs":1437},"stats":{"sendRate":3000,"deliveredRate":2009.0980019080553,"loss":0.11615384615384614,"srttMs":1167.5116440288457,"minRttMs":712,"sentPackets":233,"sentWireBytes":58026,"receivedPackets":417,"receivedWireBytes":159173,"lastHeardMs":30610,"unsyncedBytes":0,"inflightPackets":6,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 870/870 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 328/1098 1546/1546 59/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","nodes":"1019/1019 809/809 809/809 785/785 1019/1019","providerThreads":"810/810","providerSessions":"2696/2696","messages":"546/546 456/456 901/901 451/451 546/546 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794167726652,"silences":0,"longestSilenceMs":1437,"longestRecoveredGapMs":1437},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.15750899300699295,"srttMs":1180.57268852524,"minRttMs":712,"sentPackets":288,"sentWireBytes":68598,"receivedPackets":540,"receivedWireBytes":196716,"lastHeardMs":41058,"unsyncedBytes":399,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 870/870 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 539/539","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","nodes":"1019/1019 809/809 809/809 785/785 1019/1019 1395/1395 985/985 1019/1019","providerThreads":"810/810","providerSessions":"2696/2696","messages":"546/546 456/456 901/901 451/451 546/546 546/546 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794167737414,"silences":0,"longestSilenceMs":1437,"longestRecoveredGapMs":1437},"stats":{"sendRate":632.8125,"deliveredRate":1258.6495357684364,"loss":0.17245454545454542,"srttMs":1270.3175277147811,"minRttMs":712,"sentPackets":330,"sentWireBytes":80001,"receivedPackets":646,"receivedWireBytes":250029,"lastHeardMs":51820,"unsyncedBytes":317,"inflightPackets":11,"duplicateBytes":0}}
- shim trace
19:54:45.888 fetch GET /api/auth/session
19:54:45.889 fetched GET /api/auth/session 200
19:54:46.028 fetch GET /.well-known/t3/environment
19:54:46.028 fetched GET /.well-known/t3/environment 200
19:54:46.036 fetch GET /.well-known/t3/environment
19:54:46.037 fetched GET /.well-known/t3/environment 200
19:54:46.042 rpc 0 subscribeServerConfig
19:54:46.043 chunk 0 38722 bytes
19:54:46.093 in Ack 0
19:54:46.095 rpc 1 subscribeServerLifecycle
19:54:46.096 rpc 2 subscribeServerLifecycle
19:54:46.099 fetch GET /api/auth/session
19:54:46.102 fetched GET /api/auth/session 200
19:54:46.102 rpc 3 subscribeDeviceState
19:54:46.104 rpc 4 preview.reportProfiles
19:54:46.140 rpc 5 subscribeProjectClones
19:54:46.159 fetch GET /api/orchestration/threads/mcp%3A47b6f360-fbe7-44de-9481-f3a85b7ebfdb/bounded
19:54:46.163 fetch GET /api/orchestration/shell
19:54:46.184 rpc 6 server.refreshProviders
19:54:46.221 rpc 7 orchestration.dispatchCommand thread.visit
19:54:46.221 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:54:46.234 fetched GET /api/orchestration/shell 200
19:54:46.234 rpc 8 subscribeTerminalMetadata
19:54:46.235 rpc 9 subscribeVcsStatus
19:54:46.236 rpc 10 subscribePreviewEvents
19:54:46.238 rpc 11 orchestration.subscribeShell
19:54:46.239 chunk 11 2790 bytes
19:54:46.240 rpc 12 assets.createUrl
19:54:46.241 rpc 13 preview.list
19:54:46.241 rpc 14 projects.readFile
19:54:46.245 in Ack 11
19:54:46.461 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"36c8949b-2e44-4401-bd09-68e51d9dd09e","sessions":[]}}
19:54:46.461 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:54:46.488 rpc 15 server.reportClientActivity
19:54:46.709 chunk 1 3872 bytes
19:54:46.710 in Ack 1
19:54:47.026 fetch POST /api/observability/v1/traces
19:54:47.026 fetched POST /api/observability/v1/traces 204
19:54:47.139 chunk 2 3872 bytes
19:54:47.139 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:54:34.315Z","configurableProviders":false,"con
19:54:47.139 chunk 9 1995 bytes
19:54:47.140 in Ack 2
19:54:47.143 in Ack 9
19:54:47.329 chunk 11 2766 bytes
19:54:47.329 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:54:47.329 in Ack 11
19:54:47.579 chunk 11 2809 bytes
19:54:47.580 in Ack 11
19:54:47.688 chunk 3 876 bytes
19:54:47.688 exit 4 {"_tag":"Success","value":null}
19:54:47.688 chunk 5 4 bytes
19:54:47.688 chunk 8 36 bytes
19:54:47.688 exit 15 {"_tag":"Success","value":null}
19:54:47.688 in Ack 3
19:54:47.691 in Ack 5
19:54:47.692 in Ack 8
19:54:48.108 fetched GET /api/orchestration/threads/mcp%3A47b6f360-fbe7-44de-9481-f3a85b7ebfdb/bounded 200
19:54:48.111 rpc 16 orchestration.subscribeThread
19:54:48.112 chunk 16 8283 bytes
19:54:48.123 in Interrupt 9
19:54:48.124 rpc 17 subscribeVcsStatus
19:54:48.126 in Ack 16
19:54:48.177 fetch POST /api/observability/v1/traces
19:54:48.177 fetched POST /api/observability/v1/traces 204
19:54:48.268 chunk 17 1995 bytes
19:54:48.269 in Ack 17
19:54:48.372 chunk 16 11260 bytes
19:54:48.373 in Ack 16
19:54:48.405 rpc 18 server.reportClientActivity
19:54:48.622 chunk 16 12694 bytes
19:54:48.622 in Ack 16
19:54:48.688 exit 18 {"_tag":"Success","value":null}
19:54:48.899 chunk 16 13502 bytes
19:54:48.900 in Ack 16
19:54:48.948 rpc 19 attachments.createUploadUrl
19:54:49.158 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-bf2e9b6c-9b6c-401d-9ff3-91d7b6af8489","expiresAt":1794168289148,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:54:49.159 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWJmMmU5YjZjLTliNmMtNDAxZC05ZmYzLTkxZDdiNmFmODQ4OSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODI4OTE0OH0.NF_fzGwIHJ460Q80RF6d72vNNA6pjrm7IrCMbhecpEY
19:54:49.375 fetch POST /api/observability/v1/traces
19:54:49.375 fetched POST /api/observability/v1/traces 204
19:54:49.402 chunk 16 18605 bytes
19:54:49.403 in Ack 16
19:54:49.653 chunk 16 20465 bytes
19:54:49.654 in Ack 16
19:54:49.663 in Interrupt 17
19:54:49.666 rpc 20 subscribeVcsStatus
19:54:49.738 chunk 20 1995 bytes
19:54:49.738 in Ack 20
19:54:49.909 chunk 16 21199 bytes
19:54:49.911 in Ack 16
19:54:49.949 rpc 21 server.reportClientActivity
19:54:50.407 fetch POST /api/observability/v1/traces
19:54:50.407 fetched POST /api/observability/v1/traces 204
19:54:51.039 in Ping 
19:54:51.117 exit 21 {"_tag":"Success","value":null}
19:54:52.182 rpc 22 orchestration.dispatchCommand message.dispatch
19:54:52.734 fetch POST /api/observability/v1/traces
19:54:52.734 fetched POST /api/observability/v1/traces 204
19:54:52.789 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:54:52.791 rpc 23 attachments.delete
19:54:53.290 chunk 16 23605 bytes
19:54:53.291 in Ack 16
19:54:53.358 exit 23 {"_tag":"Success","value":null}
19:54:53.539 chunk 11 2895 bytes
19:54:53.539 chunk 16 23617 bytes
19:54:53.540 in Ack 11
19:54:53.593 in Ack 16
19:54:53.789 chunk 16 23667 bytes
19:54:53.790 in Ack 16
19:54:53.837 fetch POST /api/observability/v1/traces
19:54:53.837 fetched POST /api/observability/v1/traces 204
19:54:54.087 chunk 11 2831 bytes
19:54:54.087 in Ack 11
19:54:54.088 rpc 24 orchestration.dispatchCommand thread.visit
19:54:54.088 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:54:54.289 chunk 16 27432 bytes
19:54:54.290 in Ack 16
19:54:54.293 rpc 25 assets.createUrl
19:54:54.337 chunk 11 2831 bytes
19:54:54.337 in Ack 11
19:54:54.540 chunk 16 28093 bytes
19:54:54.541 in Ack 16
19:54:55.031 chunk 16 30516 bytes
19:54:55.032 in Ack 16
19:54:55.089 fetch POST /api/observability/v1/traces
19:54:55.089 fetched POST /api/observability/v1/traces 204
19:54:55.283 chunk 16 31326 bytes
19:54:55.284 in Ack 16
19:54:55.857 rpc 26 orchestration.dispatchCommand message.dispatch
19:54:55.891 chunk 16 32106 bytes
19:54:55.896 in Ack 16
19:54:55.902 in Interrupt 20
19:54:55.906 rpc 27 subscribeVcsStatus
19:54:56.039 in Ping 
19:54:56.157 rpc 28 server.reportClientActivity
19:54:56.285 fetch POST /api/observability/v1/traces
19:54:56.285 fetched POST /api/observability/v1/traces 204
19:54:56.643 chunk 16 32106 bytes
19:54:56.644 in Ack 16
19:54:56.732 exit 26 {"_tag":"Success","value":{"sequence":75}}
19:54:57.144 chunk 16 34540 bytes
19:54:57.146 in Ack 16
19:54:57.394 chunk 16 34570 bytes
19:54:57.395 in Ack 16
19:54:57.627 chunk 27 1995 bytes
19:54:57.627 in Ack 27
19:54:57.644 chunk 16 34590 bytes
19:54:57.645 in Ack 16
19:54:57.654 fetch POST /api/observability/v1/traces
19:54:57.654 fetched POST /api/observability/v1/traces 204
19:54:57.937 chunk 16 34602 bytes
19:54:57.938 in Ack 16
19:54:58.187 chunk 16 34612 bytes
19:54:58.188 in Ack 16
19:54:58.437 chunk 16 34642 bytes
19:54:58.439 in Ack 16
19:54:58.938 chunk 16 34644 bytes
19:54:58.939 fetch POST /api/observability/v1/traces
19:54:58.939 fetched POST /api/observability/v1/traces 204
19:54:58.940 in Ack 16
19:54:59.189 chunk 16 34664 bytes
19:54:59.190 in Ack 16
19:54:59.439 chunk 16 34694 bytes
19:54:59.440 in Ack 16
19:54:59.689 chunk 16 34762 bytes
19:54:59.691 in Ack 16
19:54:59.940 chunk 16 34772 bytes
19:54:59.941 in Ack 16
19:54:59.949 fetch POST /api/observability/v1/traces
19:54:59.949 fetched POST /api/observability/v1/traces 204
19:55:00.190 chunk 16 34812 bytes
19:55:00.191 in Ack 16
19:55:00.440 chunk 16 34822 bytes
19:55:00.442 in Ack 16
19:55:00.698 chunk 16 34862 bytes
19:55:00.699 in Ack 16
19:55:00.948 chunk 16 34872 bytes
19:55:00.949 in Ack 16
19:55:01.039 in Ping 
19:55:01.192 fetch POST /api/observability/v1/traces
19:55:01.192 fetched POST /api/observability/v1/traces 204
19:55:01.198 chunk 11 2895 bytes
19:55:01.198 in Ack 11
19:55:01.240 chunk 16 34912 bytes
19:55:01.242 in Ack 16
19:55:01.490 chunk 16 35579 bytes
19:55:01.491 in Ack 16
19:55:01.804 chunk 16 35609 bytes
19:55:01.805 in Ack 16
19:55:02.098 chunk 16 35629 bytes
19:55:02.099 in Ack 16
19:55:02.199 fetch POST /api/observability/v1/traces
19:55:02.199 fetched POST /api/observability/v1/traces 204
19:55:02.348 chunk 16 35659 bytes
19:55:02.350 in Ack 16
19:55:02.599 chunk 16 36519 bytes
19:55:02.600 in Ack 16
19:55:02.852 chunk 16 38542 bytes
19:55:02.853 in Ack 16
19:55:03.098 chunk 16 39229 bytes
19:55:03.107 in Ack 16
19:55:03.348 chunk 16 39249 bytes
19:55:03.350 in Ack 16
19:55:03.360 fetch POST /api/observability/v1/traces
19:55:03.360 fetched POST /api/observability/v1/traces 204
19:55:03.598 chunk 16 39271 bytes
19:55:03.600 in Ack 16
19:55:03.849 chunk 16 39301 bytes
19:55:03.850 in Ack 16
19:55:04.090 rpc 29 orchestration.dispatchCommand thread.visit
19:55:04.090 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:55:04.099 chunk 16 39311 bytes
19:55:04.100 in Ack 16
19:55:04.349 chunk 16 39353 bytes
19:55:04.351 in Ack 16
19:55:04.599 chunk 16 39897 bytes
19:55:04.600 in Ack 16
19:55:04.694 fetch POST /api/observability/v1/traces
19:55:04.694 fetched POST /api/observability/v1/traces 204
19:55:04.854 chunk 16 39751 bytes
19:55:04.855 in Ack 16
19:55:05.322 chunk 16 40557 bytes
19:55:05.324 in Ack 16
19:55:05.466 chunk 16 40587 bytes
19:55:05.467 in Ack 16
19:55:05.716 chunk 16 40619 bytes
19:55:05.737 in Ack 16
19:55:05.864 fetch POST /api/observability/v1/traces
19:55:05.865 fetched POST /api/observability/v1/traces 204
19:55:05.967 chunk 16 40639 bytes
19:55:05.968 in Ack 16
19:55:06.040 in Ping 
19:55:06.217 chunk 16 41436 bytes
19:55:06.219 in Ack 16
19:55:06.469 chunk 16 41466 bytes
19:55:06.474 in Ack 16
19:55:06.527 exit 28 {"_tag":"Success","value":null}
19:55:06.601 chunk 11 2895 bytes
19:55:06.604 in Ack 11
19:55:07.115 chunk 16 43001 bytes
19:55:07.142 fetch POST /api/observability/v1/traces
19:55:07.142 fetched POST /api/observability/v1/traces 204
19:55:07.153 in Ack 16
19:55:07.418 chunk 16 43001 bytes
19:55:07.421 in Ack 16
19:55:07.681 chunk 11 2895 bytes
19:55:07.681 chunk 16 43071 bytes
19:55:07.682 in Ack 11
19:55:07.700 in Ack 16
19:55:07.932 chunk 16 43101 bytes
19:55:07.934 in Ack 16
19:55:08.155 fetch POST /api/observability/v1/traces
19:55:08.156 fetched POST /api/observability/v1/traces 204
19:55:08.198 chunk 16 44222 bytes
19:55:08.209 in Ack 16
19:55:08.438 chunk 16 44252 bytes
19:55:08.439 in Ack 16
19:55:08.689 chunk 16 44297 bytes
19:55:08.690 in Ack 16
19:55:08.938 chunk 16 45362 bytes
19:55:08.940 in Ack 16
19:55:09.189 chunk 16 45407 bytes
19:55:09.190 in Ack 16
19:55:09.244 fetch POST /api/observability/v1/traces
19:55:09.244 fetched POST /api/observability/v1/traces 204
19:55:09.590 chunk 16 45422 bytes
19:55:09.596 in Ack 16
19:55:09.843 chunk 16 45497 bytes
19:55:09.849 in Ack 16
19:55:10.093 chunk 16 45542 bytes
19:55:10.095 in Ack 16
19:55:10.349 chunk 16 45572 bytes
19:55:10.350 in Ack 16
19:55:10.446 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/01e9d773-241a-4a0f-a24b-fd3e8b7b998a","expiresAt":1794171310446}}
19:55:10.639 fetch POST /api/observability/v1/traces
19:55:10.639 fetched POST /api/observability/v1/traces 204
19:55:10.640 chunk 16 45617 bytes
19:55:10.642 in Ack 16
19:55:10.891 chunk 16 45647 bytes
19:55:10.892 in Ack 16
19:55:11.040 in Ping 
19:55:11.159 chunk 16 45677 bytes
19:55:11.162 in Ack 16
19:55:11.296 rpc 30 server.reportClientActivity
19:55:11.410 chunk 16 45722 bytes
19:55:11.411 in Ack 16
19:55:11.643 fetch POST /api/observability/v1/traces
19:55:11.643 fetched POST /api/observability/v1/traces 204
19:55:11.659 chunk 16 45737 bytes
19:55:11.661 in Ack 16
19:55:11.909 chunk 16 45782 bytes
19:55:11.910 in Ack 16
19:55:12.160 chunk 16 45827 bytes
19:55:12.161 in Ack 16
19:55:12.373 exit 30 {"_tag":"Success","value":null}
19:55:12.411 chunk 16 45872 bytes
19:55:12.446 in Ack 16
19:55:12.713 chunk 16 45902 bytes
19:55:12.714 fetch POST /api/observability/v1/traces
19:55:12.714 fetched POST /api/observability/v1/traces 204
19:55:12.717 in Ack 16
19:55:12.963 chunk 16 45947 bytes
19:55:12.965 in Ack 16
19:55:13.213 chunk 16 45977 bytes
19:55:13.215 in Ack 16
19:55:13.464 chunk 16 45992 bytes
19:55:13.465 in Ack 16
19:55:13.716 chunk 16 46022 bytes
19:55:13.717 in Ack 16
19:55:13.729 fetch POST /api/observability/v1/traces
19:55:13.729 fetched POST /api/observability/v1/traces 204
19:55:13.966 chunk 16 46052 bytes
19:55:13.967 in Ack 16
19:55:14.102 rpc 31 orchestration.dispatchCommand thread.visit
19:55:14.102 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:55:14.217 chunk 16 46097 bytes
19:55:14.218 in Ack 16
19:55:14.471 chunk 16 46142 bytes
19:55:14.472 in Ack 16
19:55:14.724 chunk 16 46187 bytes
19:55:14.725 in Ack 16
19:55:14.968 fetch POST /api/observability/v1/traces
19:55:14.969 fetched POST /api/observability/v1/traces 204
19:55:14.974 chunk 16 46232 bytes
19:55:14.975 in Ack 16
19:55:15.224 chunk 16 46262 bytes
19:55:15.226 in Ack 16
19:55:15.473 chunk 16 46292 bytes
19:55:15.474 in Ack 16
19:55:15.587 chunk 11 2895 bytes
19:55:15.587 in Ack 11
19:55:15.724 chunk 16 46322 bytes
19:55:15.725 in Ack 16
19:55:15.973 chunk 16 46337 bytes
19:55:15.974 in Ack 16
19:55:15.983 fetch POST /api/observability/v1/traces
19:55:15.983 fetched POST /api/observability/v1/traces 204
19:55:16.040 in Ping 
19:55:16.223 chunk 16 46397 bytes
19:55:16.225 in Ack 16
19:55:16.475 chunk 16 46412 bytes
19:55:16.476 in Ack 16
19:55:16.725 chunk 16 46487 bytes
19:55:16.727 in Ack 16
19:55:16.975 chunk 16 46517 bytes
19:55:16.977 in Ack 16
19:55:17.225 chunk 16 46562 bytes
19:55:17.226 fetch POST /api/observability/v1/traces
19:55:17.226 fetched POST /api/observability/v1/traces 204
19:55:17.229 in Ack 16
19:55:17.552 chunk 16 46592 bytes
19:55:17.553 in Ack 16
19:55:17.802 chunk 16 46622 bytes
19:55:17.804 in Ack 16
19:55:18.053 chunk 16 46667 bytes
19:55:18.055 in Ack 16
19:55:18.231 fetch POST /api/observability/v1/traces
19:55:18.231 fetched POST /api/observability/v1/traces 204
19:55:18.303 chunk 16 46727 bytes
19:55:18.305 in Ack 16
19:55:18.553 chunk 16 46787 bytes
19:55:18.554 in Ack 16
19:55:18.803 chunk 16 46817 bytes
19:55:18.804 in Ack 16
19:55:19.053 chunk 16 46862 bytes
19:55:19.054 in Ack 16
19:55:19.303 chunk 16 46892 bytes
19:55:19.306 in Ack 16
19:55:19.315 fetch POST /api/observability/v1/traces
19:55:19.315 fetched POST /api/observability/v1/traces 204
19:55:19.553 chunk 16 46937 bytes
19:55:19.555 in Ack 16
19:55:19.804 chunk 16 46982 bytes
19:55:19.805 in Ack 16
19:55:20.054 chunk 16 47012 bytes
19:55:20.055 in Ack 16
19:55:20.304 chunk 16 47042 bytes
19:55:20.305 in Ack 16
19:55:20.554 chunk 16 47087 bytes
19:55:20.556 in Ack 16
19:55:20.564 fetch POST /api/observability/v1/traces
19:55:20.564 fetched POST /api/observability/v1/traces 204
19:55:20.805 chunk 16 47132 bytes
19:55:20.807 in Ack 16
19:55:21.040 in Ping 
19:55:21.054 chunk 16 47162 bytes
19:55:21.055 in Ack 16
19:55:21.304 chunk 16 47207 bytes
19:55:21.305 in Ack 16
19:55:21.554 chunk 16 47252 bytes
19:55:21.555 in Ack 16
19:55:21.804 chunk 16 47282 bytes
19:55:21.806 in Ack 16
19:55:21.816 fetch POST /api/observability/v1/traces
19:55:21.816 fetched POST /api/observability/v1/traces 204
19:55:22.055 chunk 16 47327 bytes
19:55:22.056 in Ack 16
19:55:22.305 chunk 16 47342 bytes
19:55:22.306 in Ack 16
19:55:22.557 chunk 16 47372 bytes
19:55:22.559 in Ack 16
19:55:22.808 chunk 16 47432 bytes
19:55:22.809 in Ack 16
19:55:23.057 fetch POST /api/observability/v1/traces
19:55:23.057 fetched POST /api/observability/v1/traces 204
19:55:23.058 chunk 16 47462 bytes
19:55:23.059 in Ack 16
19:55:23.308 chunk 16 47507 bytes
19:55:23.309 in Ack 16
19:55:23.558 chunk 16 47522 bytes
19:55:23.559 in Ack 16
19:55:23.809 chunk 16 47582 bytes
19:55:23.810 in Ack 16
19:55:24.059 chunk 16 47609 bytes
19:55:24.061 fetch POST /api/observability/v1/traces
19:55:24.061 fetched POST /api/observability/v1/traces 204
19:55:24.062 in Ack 16
19:55:25.061 chunk 16 48614 bytes
19:55:25.062 in Ack 16
19:55:25.154 fetch POST /api/observability/v1/traces
19:55:25.154 fetched POST /api/observability/v1/traces 204
19:55:25.301 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:55:26.041 in Ping 
19:55:26.301 fetch POST /api/observability/v1/traces
19:55:26.301 fetched POST /api/observability/v1/traces 204
19:55:26.319 chunk 16 52110 bytes
19:55:26.320 in Ack 16
19:55:26.783 exit 32 {"_tag":"Success","value":{"sequence":97}}
19:55:27.099 chunk 16 54601 bytes
19:55:27.100 in Ack 16
19:55:27.383 chunk 16 54621 bytes
19:55:27.385 fetch POST /api/observability/v1/traces
19:55:27.385 fetched POST /api/observability/v1/traces 204
19:55:27.386 in Ack 16
19:55:27.634 chunk 16 54661 bytes
19:55:27.635 in Ack 16
19:55:27.891 chunk 16 54731 bytes
19:55:27.947 in Ack 16
19:55:28.193 chunk 16 54751 bytes
19:55:28.196 in Ack 16
19:55:28.361 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:55:28.389 fetch POST /api/observability/v1/traces
19:55:28.389 fetched POST /api/observability/v1/traces 204
19:55:28.443 chunk 16 54834 bytes
19:55:28.444 in Ack 16
19:55:28.694 chunk 16 54844 bytes
19:55:28.695 in Ack 16
19:55:28.968 chunk 16 54836 bytes
19:55:28.970 in Ack 16
19:55:29.218 chunk 16 54846 bytes
19:55:29.220 in Ack 16
19:55:29.466 fetch POST /api/observability/v1/traces
19:55:29.466 fetched POST /api/observability/v1/traces 204
19:55:29.469 chunk 16 54858 bytes
19:55:29.470 in Ack 16
19:55:29.719 chunk 16 54888 bytes
19:55:29.721 in Ack 16
19:55:29.969 chunk 16 54918 bytes
19:55:29.970 in Ack 16
19:55:30.226 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:55:30.851 fetch POST /api/observability/v1/traces
19:55:30.851 fetched POST /api/observability/v1/traces 204
19:55:30.852 chunk 16 55046 bytes
19:55:30.854 in Ack 16
19:55:31.041 in Ping 
19:55:31.101 chunk 16 55076 bytes
19:55:31.103 in Ack 16
19:55:31.351 chunk 11 2897 bytes
19:55:31.355 chunk 16 57228 bytes
19:55:31.355 in Ack 11
19:55:31.362 rpc 34 orchestration.dispatchCommand thread.visit
19:55:31.362 exit 34 {"_tag":"Success","value":{"sequence":0}}
19:55:31.364 in Ack 16
19:55:31.392 in Interrupt 27
19:55:31.477 rpc 35 subscribeVcsStatus
19:55:31.602 chunk 16 57248 bytes
19:55:31.603 in Ack 16
19:55:31.765 rpc 36 server.reportClientActivity
19:55:31.873 chunk 16 58254 bytes
19:55:31.874 fetch POST /api/observability/v1/traces
19:55:31.874 fetched POST /api/observability/v1/traces 204
19:55:31.876 in Ack 16
19:55:32.124 chunk 16 59670 bytes
19:55:32.125 in Ack 16
19:55:32.374 chunk 16 59701 bytes
19:55:32.376 in Ack 16
19:55:32.625 chunk 16 59731 bytes
19:55:32.630 in Ack 16
19:55:32.875 chunk 16 59751 bytes
19:55:32.880 in Ack 16
19:55:32.936 fetch POST /api/observability/v1/traces
19:55:32.936 fetched POST /api/observability/v1/traces 204
19:55:33.007 chunk 35 1995 bytes
19:55:33.009 in Ack 35
19:55:33.125 chunk 16 59761 bytes
19:55:33.127 in Ack 16
19:55:33.375 chunk 16 59791 bytes
19:55:33.377 in Ack 16
19:55:33.626 chunk 16 59813 bytes
19:55:33.641 in Ack 16
19:55:33.875 chunk 16 59861 bytes
19:55:33.887 in Ack 16
19:55:34.127 chunk 16 59901 bytes
19:55:34.148 in Ack 16
19:55:34.235 fetch POST /api/observability/v1/traces
19:55:34.235 fetched POST /api/observability/v1/traces 204
19:55:34.477 chunk 16 59931 bytes
19:55:34.481 in Ack 16
19:55:34.727 chunk 16 60973 bytes
19:55:34.729 in Ack 16
19:55:34.876 chunk 11 2897 bytes
19:55:34.876 in Ack 11
19:55:34.977 chunk 16 61009 bytes
19:55:34.979 in Ack 16
19:55:35.269 chunk 16 61031 bytes
19:55:35.272 in Ack 16
19:55:35.581 fetch POST /api/observability/v1/traces
19:55:35.581 fetched POST /api/observability/v1/traces 204
19:55:35.583 chunk 16 61061 bytes
19:55:35.613 in Ack 16
19:55:35.832 chunk 11 2897 bytes
19:55:35.832 chunk 16 61099 bytes
19:55:35.833 in Ack 11
19:55:35.836 in Ack 16
19:55:36.002 exit 36 {"_tag":"Success","value":null}
19:55:36.042 in Ping 
19:55:36.097 chunk 16 61123 bytes
19:55:36.098 in Ack 16
19:55:36.282 rpc 37 server.reportClientActivity
19:55:36.348 chunk 16 62198 bytes
19:55:36.349 in Ack 16
19:55:36.598 chunk 16 62249 bytes
19:55:36.599 in Ack 16
19:55:36.684 fetch POST /api/observability/v1/traces
19:55:36.684 fetched POST /api/observability/v1/traces 204
19:55:36.847 chunk 16 62279 bytes
19:55:36.848 in Ack 16
19:55:37.112 chunk 16 62294 bytes
19:55:37.114 in Ack 16
19:55:37.363 chunk 16 62324 bytes
19:55:37.364 in Ack 16
19:55:37.613 chunk 16 62375 bytes
19:55:37.614 in Ack 16
19:55:37.850 fetch POST /api/observability/v1/traces
19:55:37.850 fetched POST /api/observability/v1/traces 204
19:55:37.864 chunk 16 62420 bytes
19:55:37.865 in Ack 16
19:55:37.999 exit 37 {"_tag":"Success","value":null}
19:55:38.113 chunk 16 62465 bytes
19:55:38.115 in Ack 16
19:55:38.364 chunk 16 62519 bytes
19:55:38.366 in Ack 16
19:55:38.614 chunk 16 62549 bytes
19:55:38.615 in Ack 16
19:55:38.865 chunk 16 62579 bytes
19:55:38.866 in Ack 16
19:55:38.906 fetch POST /api/observability/v1/traces
19:55:38.906 fetched POST /api/observability/v1/traces 204
19:55:39.117 chunk 16 62603 bytes
19:55:39.118 in Ack 16
19:55:39.368 chunk 16 62675 bytes
19:55:39.370 in Ack 16
19:55:39.624 chunk 16 62720 bytes
19:55:39.625 in Ack 16
19:55:39.875 chunk 16 62771 bytes
19:55:39.876 in Ack 16
19:55:40.119 fetch POST /api/observability/v1/traces
19:55:40.119 fetched POST /api/observability/v1/traces 204
19:55:40.124 chunk 16 62801 bytes
19:55:40.125 in Ack 16
19:55:40.375 chunk 16 62840 bytes
19:55:40.376 in Ack 16
19:55:40.626 chunk 16 62888 bytes
19:55:40.627 in Ack 16
19:55:40.875 chunk 16 62918 bytes
19:55:40.876 in Ack 16
19:55:41.053 in Ping 
19:55:41.126 chunk 16 62951 bytes
19:55:41.126 fetch POST /api/observability/v1/traces
19:55:41.126 fetched POST /api/observability/v1/traces 204
19:55:41.128 in Ack 16
19:55:41.358 rpc 38 orchestration.dispatchCommand thread.visit
19:55:41.358 exit 38 {"_tag":"Success","value":{"sequence":0}}
19:55:42.129 fetch POST /api/observability/v1/traces
19:55:42.129 fetched POST /api/observability/v1/traces 204
