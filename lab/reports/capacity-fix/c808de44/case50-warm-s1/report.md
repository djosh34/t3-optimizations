# case50, webapp, warm start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.4s |
| Whole reply shown | 43.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.0s / 1.0s / 2.0s |
| Longest silence | 3.0s |
| Approval shown after it was asked | 0.7s |
| Bytes on the wire down / up | 384956 / 153219 |
| Bytes the app received / sent | 307905 / 68125 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.9 | 299 | 377 |
| 20 | 0.8 | 742 | 779 |
| 30 | 1.0 | 1086 | 1128 |
| 40 | 0.7 | 1473 | 1500 |
| 43 | 1.3 | 1558 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"776/776 776/776","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"548/548 456/456 901/901 548/548","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794173202263,"silences":0,"longestSilenceMs":1443,"longestRecoveredGapMs":1443},"stats":{"sendRate":7152.190596136591,"deliveredRate":3679.0734454505387,"loss":0,"srttMs":5.346880489994243,"minRttMs":1,"sentPackets":98,"sentWireBytes":31001,"receivedPackets":130,"receivedWireBytes":70039,"lastHeardMs":9080,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 440/455","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 641/785","messages":"548/548 456/456 901/901 548/548 250/451","checkpointScopes":"577/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794173212487,"silences":0,"longestSilenceMs":1443,"longestRecoveredGapMs":1443},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.3414025974025974,"srttMs":417.0278859135491,"minRttMs":544,"sentPackets":152,"sentWireBytes":42468,"receivedPackets":189,"receivedWireBytes":98902,"lastHeardMs":19304,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 958/958","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794173222676,"silences":0,"longestSilenceMs":1443,"longestRecoveredGapMs":1443},"stats":{"sendRate":3375,"deliveredRate":1874.1935483870968,"loss":0.4375,"srttMs":758.8649619520337,"minRttMs":544,"sentPackets":227,"sentWireBytes":58832,"receivedPackets":438,"receivedWireBytes":168532,"lastHeardMs":29493,"unsyncedBytes":0,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1103/1103 567/1574 953/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 128/128","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 987/987 451/1398","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794173233733,"silences":0,"longestSilenceMs":1443,"longestRecoveredGapMs":1443},"stats":{"sendRate":3452.8301886792456,"deliveredRate":767.2955974842766,"loss":0.6666666666666667,"srttMs":740.7872425474005,"minRttMs":544,"sentPackets":290,"sentWireBytes":72847,"receivedPackets":792,"receivedWireBytes":262604,"lastHeardMs":40550,"unsyncedBytes":0,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 674/674","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 987/987 1398/1398 1022/1022","messages":"548/548 456/456 901/901 548/548 451/451 548/548 547/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":246,"lastDataAt":1794173244923,"silences":0,"longestSilenceMs":1443,"longestRecoveredGapMs":1443},"stats":{"sendRate":3000,"deliveredRate":572.5490196078431,"loss":0.7272727272727273,"srttMs":840.3963601604805,"minRttMs":544,"sentPackets":384,"sentWireBytes":95271,"receivedPackets":1078,"receivedWireBytes":363419,"lastHeardMs":51740,"unsyncedBytes":174,"inflightPackets":83,"duplicateBytes":0}}
- shim trace
21:26:33.481 fetch GET /api/auth/session
21:26:33.482 fetched GET /api/auth/session 200
21:26:33.629 fetch GET /.well-known/t3/environment
21:26:33.630 fetched GET /.well-known/t3/environment 200
21:26:33.642 fetch GET /.well-known/t3/environment
21:26:33.642 fetched GET /.well-known/t3/environment 200
21:26:33.646 rpc 0 subscribeServerConfig
21:26:33.646 chunk 0 38678 bytes
21:26:33.664 in Ack 0
21:26:33.666 rpc 1 subscribeServerLifecycle
21:26:33.666 rpc 2 subscribeServerLifecycle
21:26:33.669 fetch GET /api/auth/session
21:26:33.670 fetched GET /api/auth/session 200
21:26:33.670 rpc 3 subscribeDeviceState
21:26:33.672 rpc 4 preview.reportProfiles
21:26:33.712 rpc 5 subscribeProjectClones
21:26:33.731 fetch GET /api/orchestration/threads/mcp%3A5502570c-e362-4e56-9555-6e753fe92ab4/bounded
21:26:33.735 fetch GET /api/orchestration/shell
21:26:33.755 rpc 6 server.refreshProviders
21:26:33.794 rpc 7 orchestration.dispatchCommand thread.visit
21:26:33.794 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:26:33.805 fetched GET /api/orchestration/shell 200
21:26:33.805 rpc 8 subscribeTerminalMetadata
21:26:33.806 rpc 9 subscribeVcsStatus
21:26:33.807 rpc 10 subscribePreviewEvents
21:26:33.809 rpc 11 orchestration.subscribeShell
21:26:33.809 chunk 11 2779 bytes
21:26:33.811 rpc 12 assets.createUrl
21:26:33.811 rpc 13 preview.list
21:26:33.812 rpc 14 projects.readFile
21:26:33.818 in Ack 11
21:26:33.931 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"87961c08-03e9-45d6-a3e5-703510bd0090","sessions":[]}}
21:26:33.931 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:26:34.058 rpc 15 server.reportClientActivity
21:26:34.059 chunk 11 2755 bytes
21:26:34.059 in Ack 11
21:26:34.070 chunk 1 3861 bytes
21:26:34.071 in Ack 1
21:26:34.549 chunk 2 3861 bytes
21:26:34.549 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:26:22.033Z","configurableProviders":false,"con
21:26:34.549 chunk 9 539 bytes
21:26:34.549 in Ack 2
21:26:34.552 in Ack 9
21:26:34.628 fetch POST /api/observability/v1/traces
21:26:34.628 fetched POST /api/observability/v1/traces 204
21:26:34.640 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:26:34.818 chunk 3 876 bytes
21:26:34.818 exit 4 {"_tag":"Success","value":null}
21:26:34.818 chunk 5 4 bytes
21:26:34.818 chunk 8 36 bytes
21:26:34.818 exit 15 {"_tag":"Success","value":null}
21:26:34.819 in Ack 3
21:26:34.822 rpc 16 preview.reportProfiles
21:26:34.822 in Ack 5
21:26:34.823 in Ack 8
21:26:34.870 exit 16 {"_tag":"Success","value":null}
21:26:34.870 fetched GET /api/orchestration/threads/mcp%3A5502570c-e362-4e56-9555-6e753fe92ab4/bounded 200
21:26:34.873 rpc 17 orchestration.subscribeThread
21:26:34.874 chunk 17 8294 bytes
21:26:34.882 in Interrupt 9
21:26:34.884 rpc 18 subscribeVcsStatus
21:26:34.932 in Ack 17
21:26:34.945 chunk 11 2798 bytes
21:26:34.946 in Ack 11
21:26:35.058 chunk 18 539 bytes
21:26:35.059 in Ack 18
21:26:35.135 chunk 17 11932 bytes
21:26:35.136 rpc 19 server.reportClientActivity
21:26:35.137 in Ack 17
21:26:35.339 exit 19 {"_tag":"Success","value":null}
21:26:35.385 chunk 17 13517 bytes
21:26:35.386 in Ack 17
21:26:35.643 chunk 17 18034 bytes
21:26:35.643 fetch POST /api/observability/v1/traces
21:26:35.643 fetched POST /api/observability/v1/traces 204
21:26:35.645 in Ack 17
21:26:35.699 rpc 20 attachments.createUploadUrl
21:26:35.894 chunk 17 21197 bytes
21:26:35.895 in Ack 17
21:26:35.902 in Interrupt 18
21:26:35.904 rpc 21 subscribeVcsStatus
21:26:35.917 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-342500ad-25f0-4031-948f-6ac855ae69fb","expiresAt":1794173795878,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:26:35.918 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTM0MjUwMGFkLTI1ZjAtNDAzMS05NDhmLTZhYzg1NWFlNjlmYiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3Mzc5NTg3OH0.0CA6nGRsO5zIBvl7KbmDPvRDFver69sDhqU2uIoE6gQ
21:26:35.958 chunk 21 539 bytes
21:26:35.959 in Ack 21
21:26:36.156 rpc 22 server.reportClientActivity
21:26:36.646 fetch POST /api/observability/v1/traces
21:26:36.646 fetched POST /api/observability/v1/traces 204
21:26:37.677 exit 22 {"_tag":"Success","value":null}
21:26:38.736 rpc 23 orchestration.dispatchCommand message.dispatch
21:26:39.280 in Ping 
21:26:39.281 fetch POST /api/observability/v1/traces
21:26:39.281 fetched POST /api/observability/v1/traces 204
21:26:39.340 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:26:39.342 rpc 24 attachments.delete
21:26:39.408 exit 24 {"_tag":"Success","value":null}
21:26:39.591 chunk 11 2884 bytes
21:26:39.591 chunk 17 25468 bytes
21:26:39.592 in Ack 11
21:26:39.626 in Ack 17
21:26:39.629 rpc 25 assets.createUrl
21:26:39.842 chunk 17 28534 bytes
21:26:39.843 in Ack 17
21:26:40.093 chunk 17 30255 bytes
21:26:40.094 in Ack 17
21:26:40.342 fetch POST /api/observability/v1/traces
21:26:40.342 fetched POST /api/observability/v1/traces 204
21:26:40.343 chunk 11 2820 bytes
21:26:40.344 chunk 17 30305 bytes
21:26:40.344 in Ack 11
21:26:40.345 rpc 26 orchestration.dispatchCommand thread.visit
21:26:40.345 exit 26 {"_tag":"Success","value":{"sequence":0}}
21:26:40.351 in Ack 17
21:26:40.594 chunk 11 2820 bytes
21:26:40.595 chunk 17 30461 bytes
21:26:40.595 in Ack 11
21:26:40.601 in Ack 17
21:26:40.845 chunk 17 33688 bytes
21:26:40.846 in Ack 17
21:26:40.852 in Interrupt 21
21:26:40.854 rpc 27 subscribeVcsStatus
21:26:40.899 chunk 27 539 bytes
21:26:40.900 in Ack 27
21:26:41.095 chunk 17 34208 bytes
21:26:41.096 in Ack 17
21:26:41.104 rpc 28 server.reportClientActivity
21:26:41.209 exit 28 {"_tag":"Success","value":null}
21:26:41.599 fetch POST /api/observability/v1/traces
21:26:41.600 fetched POST /api/observability/v1/traces 204
21:26:41.600 chunk 17 35231 bytes
21:26:41.602 in Ack 17
21:26:42.449 rpc 29 orchestration.dispatchCommand message.dispatch
21:26:42.603 fetch POST /api/observability/v1/traces
21:26:42.603 fetched POST /api/observability/v1/traces 204
21:26:43.201 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:26:43.704 chunk 17 37686 bytes
21:26:43.705 in Ack 17
21:26:43.954 chunk 17 37698 bytes
21:26:43.955 in Ack 17
21:26:44.203 fetch POST /api/observability/v1/traces
21:26:44.203 fetched POST /api/observability/v1/traces 204
21:26:44.204 chunk 17 37708 bytes
21:26:44.206 in Ack 17
21:26:44.281 in Ping 
21:26:44.455 chunk 17 37748 bytes
21:26:44.456 in Ack 17
21:26:44.708 chunk 17 37760 bytes
21:26:44.709 in Ack 17
21:26:44.983 chunk 11 2884 bytes
21:26:44.983 chunk 17 37724 bytes
21:26:44.984 in Ack 11
21:26:45.025 in Ack 17
21:26:45.207 fetch POST /api/observability/v1/traces
21:26:45.207 fetched POST /api/observability/v1/traces 204
21:26:45.234 chunk 17 38382 bytes
21:26:45.235 in Ack 17
21:26:45.484 chunk 17 39232 bytes
21:26:45.486 in Ack 17
21:26:45.734 chunk 17 39254 bytes
21:26:45.736 in Ack 17
21:26:45.995 chunk 17 39276 bytes
21:26:45.996 in Ack 17
21:26:46.237 fetch POST /api/observability/v1/traces
21:26:46.237 fetched POST /api/observability/v1/traces 204
21:26:46.245 chunk 17 39304 bytes
21:26:46.246 in Ack 17
21:26:46.495 chunk 17 39370 bytes
21:26:46.497 in Ack 17
21:26:46.746 chunk 17 41421 bytes
21:26:46.748 in Ack 17
21:26:46.997 chunk 17 41431 bytes
21:26:46.998 in Ack 17
21:26:47.247 chunk 17 41481 bytes
21:26:47.247 fetch POST /api/observability/v1/traces
21:26:47.247 fetched POST /api/observability/v1/traces 204
21:26:47.249 in Ack 17
21:26:47.497 chunk 17 41501 bytes
21:26:47.499 in Ack 17
21:26:48.250 fetch POST /api/observability/v1/traces
21:26:48.250 fetched POST /api/observability/v1/traces 204
21:26:48.253 chunk 17 41523 bytes
21:26:48.254 in Ack 17
21:26:48.503 chunk 17 41543 bytes
21:26:48.504 in Ack 17
21:26:49.248 chunk 17 42338 bytes
21:26:49.249 in Ack 17
21:26:49.258 fetch POST /api/observability/v1/traces
21:26:49.258 fetched POST /api/observability/v1/traces 204
21:26:49.281 in Ping 
21:26:49.499 chunk 17 42348 bytes
21:26:49.500 in Ack 17
21:26:49.749 chunk 17 42388 bytes
21:26:49.750 in Ack 17
21:26:50.052 chunk 17 42418 bytes
21:26:50.053 in Ack 17
21:26:50.302 chunk 17 42430 bytes
21:26:50.303 in Ack 17
21:26:50.347 rpc 30 orchestration.dispatchCommand thread.visit
21:26:50.347 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:26:50.503 fetch POST /api/observability/v1/traces
21:26:50.503 fetched POST /api/observability/v1/traces 204
21:26:50.552 chunk 17 42460 bytes
21:26:50.553 in Ack 17
21:26:50.802 chunk 17 42488 bytes
21:26:50.803 in Ack 17
21:26:51.052 chunk 17 42510 bytes
21:26:51.053 in Ack 17
21:26:51.303 chunk 17 42532 bytes
21:26:51.304 in Ack 17
21:26:51.553 chunk 17 42544 bytes
21:26:51.554 in Ack 17
21:26:51.564 fetch POST /api/observability/v1/traces
21:26:51.564 fetched POST /api/observability/v1/traces 204
21:26:51.804 chunk 17 42554 bytes
21:26:51.805 in Ack 17
21:26:52.053 chunk 17 42574 bytes
21:26:52.055 in Ack 17
21:26:52.304 chunk 17 42606 bytes
21:26:52.305 in Ack 17
21:26:52.554 chunk 17 42644 bytes
21:26:52.555 in Ack 17
21:26:52.804 chunk 17 42664 bytes
21:26:52.805 in Ack 17
21:26:52.815 fetch POST /api/observability/v1/traces
21:26:52.815 fetched POST /api/observability/v1/traces 204
21:26:53.058 chunk 17 42684 bytes
21:26:53.059 in Ack 17
21:26:53.308 chunk 17 42696 bytes
21:26:53.309 in Ack 17
21:26:53.558 chunk 17 42720 bytes
21:26:53.560 in Ack 17
21:26:53.809 chunk 17 42730 bytes
21:26:53.810 in Ack 17
21:26:54.060 chunk 17 42778 bytes
21:26:54.060 fetch POST /api/observability/v1/traces
21:26:54.060 fetched POST /api/observability/v1/traces 204
21:26:54.062 in Ack 17
21:26:54.281 in Ping 
21:26:54.311 chunk 17 42798 bytes
21:26:54.312 in Ack 17
21:26:54.560 chunk 17 42826 bytes
21:26:54.561 in Ack 17
21:26:54.810 chunk 17 42876 bytes
21:26:54.811 in Ack 17
21:26:55.060 chunk 17 42886 bytes
21:26:55.061 in Ack 17
21:26:55.070 fetch POST /api/observability/v1/traces
21:26:55.070 fetched POST /api/observability/v1/traces 204
21:26:55.310 chunk 17 44234 bytes
21:26:55.311 in Ack 17
21:26:55.561 chunk 17 44272 bytes
21:26:55.561 in Ack 17
21:26:55.810 chunk 17 44292 bytes
21:26:55.811 in Ack 17
21:26:56.060 chunk 11 2884 bytes
21:26:56.060 chunk 17 44312 bytes
21:26:56.061 in Ack 11
21:26:56.066 in Ack 17
21:26:56.311 chunk 17 44342 bytes
21:26:56.312 in Ack 17
21:26:56.320 fetch POST /api/observability/v1/traces
21:26:56.320 fetched POST /api/observability/v1/traces 204
21:26:56.492 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/55afb91e-3bff-4e39-820c-b0de6340abf8","expiresAt":1794176816492}}
21:26:56.561 chunk 17 44362 bytes
21:26:56.562 in Ack 17
21:26:56.816 chunk 17 45605 bytes
21:26:56.817 in Ack 17
21:26:57.122 chunk 17 45650 bytes
21:26:57.123 in Ack 17
21:26:57.373 chunk 17 45680 bytes
21:26:57.374 in Ack 17
21:26:57.493 fetch POST /api/observability/v1/traces
21:26:57.493 fetched POST /api/observability/v1/traces 204
21:26:57.623 chunk 17 45725 bytes
21:26:57.624 in Ack 17
21:26:57.873 chunk 17 45770 bytes
21:26:57.874 in Ack 17
21:26:58.123 chunk 17 45815 bytes
21:26:58.124 in Ack 17
21:26:58.373 chunk 17 45845 bytes
21:26:58.376 in Ack 17
21:26:58.624 chunk 17 45890 bytes
21:26:58.625 in Ack 17
21:26:58.634 fetch POST /api/observability/v1/traces
21:26:58.634 fetched POST /api/observability/v1/traces 204
21:26:58.875 chunk 17 45920 bytes
21:26:58.876 in Ack 17
21:26:58.885 rpc 31 server.reportClientActivity
21:26:59.126 chunk 17 45965 bytes
21:26:59.128 in Ack 17
21:26:59.281 in Ping 
21:26:59.377 chunk 17 45995 bytes
21:26:59.378 in Ack 17
21:26:59.627 chunk 17 46025 bytes
21:26:59.628 in Ack 17
21:26:59.877 fetch POST /api/observability/v1/traces
21:26:59.877 fetched POST /api/observability/v1/traces 204
21:26:59.877 chunk 17 46055 bytes
21:26:59.878 in Ack 17
21:27:00.127 chunk 17 46100 bytes
21:27:00.128 in Ack 17
21:27:00.347 rpc 32 orchestration.dispatchCommand thread.visit
21:27:00.347 exit 32 {"_tag":"Success","value":{"sequence":0}}
21:27:00.377 chunk 17 46130 bytes
21:27:00.378 in Ack 17
21:27:00.628 chunk 17 46190 bytes
21:27:00.629 in Ack 17
21:27:00.879 chunk 17 46220 bytes
21:27:00.879 fetch POST /api/observability/v1/traces
21:27:00.879 fetched POST /api/observability/v1/traces 204
21:27:00.880 in Ack 17
21:27:01.130 chunk 17 46250 bytes
21:27:01.131 in Ack 17
21:27:01.380 chunk 17 46280 bytes
21:27:01.381 in Ack 17
21:27:01.634 chunk 11 2884 bytes
21:27:01.635 chunk 17 46295 bytes
21:27:01.636 in Ack 11
21:27:01.641 in Ack 17
21:27:01.881 fetch POST /api/observability/v1/traces
21:27:01.881 fetched POST /api/observability/v1/traces 204
21:27:01.885 chunk 17 46340 bytes
21:27:01.886 in Ack 17
21:27:02.135 chunk 17 46385 bytes
21:27:02.136 in Ack 17
21:27:02.336 exit 31 {"_tag":"Success","value":null}
21:27:02.386 chunk 17 46430 bytes
21:27:02.387 in Ack 17
21:27:02.637 chunk 17 46475 bytes
21:27:02.638 in Ack 17
21:27:02.887 chunk 17 46505 bytes
21:27:02.888 fetch POST /api/observability/v1/traces
21:27:02.888 fetched POST /api/observability/v1/traces 204
21:27:02.889 in Ack 17
21:27:03.137 chunk 17 46535 bytes
21:27:03.138 in Ack 17
21:27:03.387 chunk 17 46580 bytes
21:27:03.388 in Ack 17
21:27:03.637 chunk 17 46610 bytes
21:27:03.638 in Ack 17
21:27:03.951 chunk 17 46655 bytes
21:27:03.952 fetch POST /api/observability/v1/traces
21:27:03.952 fetched POST /api/observability/v1/traces 204
21:27:03.953 in Ack 17
21:27:04.202 chunk 17 46700 bytes
21:27:04.203 in Ack 17
21:27:04.282 in Ping 
21:27:04.452 chunk 17 46745 bytes
21:27:04.453 in Ack 17
21:27:04.702 chunk 17 46790 bytes
21:27:04.703 in Ack 17
21:27:04.952 chunk 17 46820 bytes
21:27:04.954 in Ack 17
21:27:04.962 fetch POST /api/observability/v1/traces
21:27:04.962 fetched POST /api/observability/v1/traces 204
21:27:05.202 chunk 17 46850 bytes
21:27:05.203 in Ack 17
21:27:05.453 chunk 17 46865 bytes
21:27:05.454 in Ack 17
21:27:05.703 chunk 17 46895 bytes
21:27:05.705 in Ack 17
21:27:05.954 chunk 17 46925 bytes
21:27:05.955 in Ack 17
21:27:06.204 fetch POST /api/observability/v1/traces
21:27:06.204 fetched POST /api/observability/v1/traces 204
21:27:06.204 chunk 17 46955 bytes
21:27:06.205 in Ack 17
21:27:06.454 chunk 17 46985 bytes
21:27:06.455 in Ack 17
21:27:06.704 chunk 17 47030 bytes
21:27:06.705 in Ack 17
21:27:06.955 chunk 17 47075 bytes
21:27:06.956 in Ack 17
21:27:07.204 chunk 17 47105 bytes
21:27:07.205 in Ack 17
21:27:07.214 fetch POST /api/observability/v1/traces
21:27:07.214 fetched POST /api/observability/v1/traces 204
21:27:07.455 chunk 17 47135 bytes
21:27:07.456 in Ack 17
21:27:07.706 chunk 17 47165 bytes
21:27:07.707 in Ack 17
21:27:07.956 chunk 17 47195 bytes
21:27:07.957 in Ack 17
21:27:08.206 chunk 17 47225 bytes
21:27:08.207 in Ack 17
21:27:08.457 fetch POST /api/observability/v1/traces
21:27:08.457 fetched POST /api/observability/v1/traces 204
21:27:08.457 chunk 17 47285 bytes
21:27:08.458 in Ack 17
21:27:08.707 chunk 17 47330 bytes
21:27:08.708 in Ack 17
21:27:08.957 chunk 17 47360 bytes
21:27:08.958 in Ack 17
21:27:09.210 chunk 17 47405 bytes
21:27:09.211 in Ack 17
21:27:09.284 in Ping 
21:27:09.459 fetch POST /api/observability/v1/traces
21:27:09.459 fetched POST /api/observability/v1/traces 204
21:27:09.460 chunk 17 47450 bytes
21:27:09.461 in Ack 17
21:27:09.711 chunk 17 47495 bytes
21:27:09.712 in Ack 17
21:27:09.961 chunk 17 47540 bytes
21:27:09.962 in Ack 17
21:27:10.211 chunk 17 47585 bytes
21:27:10.212 in Ack 17
21:27:10.461 chunk 17 48650 bytes
21:27:10.462 in Ack 17
21:27:10.533 fetch POST /api/observability/v1/traces
21:27:10.533 fetched POST /api/observability/v1/traces 204
21:27:10.718 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:27:11.206 chunk 17 52155 bytes
21:27:11.207 in Ack 17
21:27:11.456 chunk 11 3193 bytes
21:27:11.456 chunk 17 54238 bytes
21:27:11.456 in Ack 11
21:27:11.464 in Ack 17
21:27:11.667 exit 33 {"_tag":"Success","value":{"sequence":97}}
21:27:11.668 rpc 34 orchestration.dispatchCommand thread.visit
21:27:11.668 exit 34 {"_tag":"Success","value":{"sequence":0}}
21:27:11.707 chunk 17 56740 bytes
21:27:11.708 in Ack 17
21:27:11.738 fetch POST /api/observability/v1/traces
21:27:11.738 fetched POST /api/observability/v1/traces 204
21:27:11.959 chunk 17 56750 bytes
21:27:11.960 in Ack 17
21:27:12.120 rpc 35 orchestration.dispatchCommand runtime-request.respond
21:27:12.209 chunk 17 56770 bytes
21:27:12.210 in Ack 17
21:27:12.459 chunk 17 56792 bytes
21:27:12.461 in Ack 17
21:27:12.710 chunk 17 56824 bytes
21:27:12.712 in Ack 17
21:27:12.961 fetch POST /api/observability/v1/traces
21:27:12.961 fetched POST /api/observability/v1/traces 204
21:27:12.962 chunk 17 56846 bytes
21:27:12.964 in Ack 17
21:27:13.212 chunk 17 56856 bytes
21:27:13.214 in Ack 17
21:27:13.328 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:27:13.545 chunk 17 56919 bytes
21:27:13.549 in Ack 17
21:27:13.798 chunk 17 58011 bytes
21:27:13.799 in Ack 17
21:27:13.808 in Interrupt 27
21:27:13.831 rpc 36 subscribeVcsStatus
21:27:13.965 fetch POST /api/observability/v1/traces
21:27:13.965 fetched POST /api/observability/v1/traces 204
21:27:14.095 rpc 37 server.reportClientActivity
21:27:14.284 in Ping 
21:27:14.308 chunk 11 2886 bytes
21:27:14.308 chunk 17 58061 bytes
21:27:14.308 in Ack 11
21:27:14.315 in Ack 17
21:27:14.566 chunk 17 58081 bytes
21:27:14.569 in Ack 17
21:27:14.755 chunk 36 539 bytes
21:27:14.756 in Ack 36
21:27:14.824 chunk 17 58121 bytes
21:27:14.826 in Ack 17
21:27:15.075 chunk 17 58131 bytes
21:27:15.076 in Ack 17
21:27:15.096 fetch POST /api/observability/v1/traces
21:27:15.096 fetched POST /api/observability/v1/traces 204
21:27:15.325 chunk 17 59570 bytes
21:27:15.327 in Ack 17
21:27:15.575 chunk 17 59590 bytes
21:27:15.577 in Ack 17
21:27:15.827 chunk 17 59658 bytes
21:27:15.829 in Ack 17
21:27:16.077 chunk 17 59688 bytes
21:27:16.079 in Ack 17
21:27:16.327 chunk 17 59698 bytes
21:27:16.327 fetch POST /api/observability/v1/traces
21:27:16.327 fetched POST /api/observability/v1/traces 204
21:27:16.329 in Ack 17
21:27:16.577 chunk 17 60762 bytes
21:27:16.578 in Ack 17
21:27:16.827 chunk 17 60792 bytes
21:27:16.829 in Ack 17
21:27:17.062 exit 37 {"_tag":"Success","value":null}
21:27:17.085 chunk 17 60812 bytes
21:27:17.087 in Ack 17
21:27:17.371 fetch POST /api/observability/v1/traces
21:27:17.371 fetched POST /api/observability/v1/traces 204
21:27:17.372 chunk 17 60842 bytes
21:27:17.373 in Ack 17
21:27:17.627 chunk 17 60862 bytes
21:27:17.628 in Ack 17
21:27:17.877 chunk 17 60892 bytes
21:27:17.878 in Ack 17
21:27:18.128 chunk 17 60922 bytes
21:27:18.129 in Ack 17
21:27:18.374 fetch POST /api/observability/v1/traces
21:27:18.374 fetched POST /api/observability/v1/traces 204
21:27:18.379 chunk 17 60942 bytes
21:27:18.380 in Ack 17
21:27:18.630 chunk 17 60972 bytes
21:27:18.631 in Ack 17
21:27:18.881 chunk 17 60992 bytes
21:27:18.883 in Ack 17
21:27:19.133 chunk 17 61012 bytes
21:27:19.136 in Ack 17
21:27:19.284 in Ping 
21:27:19.383 fetch POST /api/observability/v1/traces
21:27:19.383 fetched POST /api/observability/v1/traces 204
21:27:19.383 chunk 17 61032 bytes
21:27:19.384 in Ack 17
21:27:19.634 chunk 11 2886 bytes
21:27:19.635 chunk 17 61060 bytes
21:27:19.635 in Ack 11
21:27:19.637 in Ack 17
21:27:19.887 chunk 17 62040 bytes
21:27:19.888 in Ack 17
21:27:20.138 chunk 17 62085 bytes
21:27:20.140 in Ack 17
21:27:20.438 fetch POST /api/observability/v1/traces
21:27:20.438 fetched POST /api/observability/v1/traces 204
21:27:20.439 chunk 17 62115 bytes
21:27:20.441 in Ack 17
21:27:20.691 chunk 17 62160 bytes
21:27:20.707 in Ack 17
21:27:20.941 chunk 17 62226 bytes
21:27:20.943 in Ack 17
21:27:21.193 chunk 17 62286 bytes
21:27:21.195 in Ack 17
21:27:21.442 fetch POST /api/observability/v1/traces
21:27:21.442 fetched POST /api/observability/v1/traces 204
21:27:21.444 chunk 17 62322 bytes
21:27:21.445 in Ack 17
21:27:21.481 rpc 38 orchestration.dispatchCommand thread.visit
21:27:21.481 exit 38 {"_tag":"Success","value":{"sequence":0}}
21:27:21.694 chunk 17 62367 bytes
21:27:21.695 in Ack 17
21:27:21.944 chunk 17 62397 bytes
21:27:21.946 in Ack 17
21:27:22.194 chunk 17 62433 bytes
21:27:22.196 in Ack 17
21:27:22.444 chunk 17 62463 bytes
21:27:22.445 in Ack 17
21:27:22.482 fetch POST /api/observability/v1/traces
21:27:22.482 fetched POST /api/observability/v1/traces 204
21:27:22.694 chunk 11 2886 bytes
21:27:22.694 chunk 17 62493 bytes
21:27:22.695 in Ack 11
21:27:22.702 in Ack 17
21:27:22.944 chunk 17 62562 bytes
21:27:22.946 in Ack 17
21:27:23.195 chunk 17 62592 bytes
21:27:23.196 in Ack 17
21:27:23.445 chunk 17 62622 bytes
21:27:23.447 in Ack 17
21:27:23.696 chunk 17 62676 bytes
21:27:23.696 fetch POST /api/observability/v1/traces
21:27:23.696 fetched POST /api/observability/v1/traces 204
21:27:23.698 in Ack 17
21:27:23.886 rpc 39 server.reportClientActivity
21:27:23.946 chunk 17 62724 bytes
21:27:23.948 in Ack 17
21:27:24.198 chunk 17 62763 bytes
21:27:24.200 in Ack 17
21:27:24.285 in Ping 
21:27:24.449 chunk 17 62793 bytes
21:27:24.450 in Ack 17
21:27:24.699 fetch POST /api/observability/v1/traces
21:27:24.699 fetched POST /api/observability/v1/traces 204
21:27:24.700 chunk 17 62823 bytes
21:27:24.701 in Ack 17
21:27:24.951 chunk 17 62859 bytes
21:27:24.952 in Ack 17
21:27:25.201 chunk 17 62913 bytes
21:27:25.203 in Ack 17
21:27:25.451 chunk 17 62943 bytes
21:27:25.454 in Ack 17
21:27:25.709 fetch POST /api/observability/v1/traces
21:27:25.709 fetched POST /api/observability/v1/traces 204
21:27:25.709 chunk 17 62976 bytes
21:27:25.710 in Ack 17
21:27:25.960 chunk 17 63074 bytes
21:27:25.961 in Ack 17
21:27:26.264 chunk 17 63206 bytes
21:27:26.265 exit 39 {"_tag":"Success","value":null}
21:27:26.267 in Ack 17
