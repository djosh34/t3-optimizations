# case50, webapp, warm start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.2s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.8s |
| Whole reply shown | 44.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.1s / 0.9s / 1.8s |
| Longest silence | 2.9s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 430004 / 152936 |
| Bytes the app received / sent | 353494 / 60884 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.7 | 307 | 377 |
| 20 | 0.9 | 738 | 779 |
| 30 | 0.7 | 1077 | 1113 |
| 40 | 1.4 | 1435 | 1487 |
| 44 | 1.1 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"776/776 776/776","messages":"456/456 548/548 901/901 548/548","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","nodes":"1022/1022 809/809 809/809 1022/1022","providerSessions":"2685/2685","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":249,"lastDataAt":1794174521292,"silences":0,"longestSilenceMs":1434,"longestRecoveredGapMs":1434},"stats":{"sendRate":7238.0511780308925,"deliveredRate":3702.5255483845012,"loss":0,"srttMs":3.594147401417583,"minRttMs":1,"sentPackets":93,"sentWireBytes":27651,"receivedPackets":142,"receivedWireBytes":66105,"lastHeardMs":8747,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"496/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 362/658","messages":"456/456 548/548 901/901 548/548 266/451","assistant":"32/32 21/21 440/455","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"809/809","nodes":"1022/1022 809/809 809/809 1022/1022 380/785","providerSessions":"2685/2685","checkpoints":"613/613 613/613 779/779","checkpointScopes":"103/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":249,"lastDataAt":1794174531326,"silences":0,"longestSilenceMs":1434,"longestRecoveredGapMs":1434},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.3286666666666666,"srttMs":390.2797464280295,"minRttMs":631,"sentPackets":149,"sentWireBytes":39209,"receivedPackets":230,"receivedWireBytes":102268,"lastHeardMs":18780,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","messages":"456/456 548/548 901/901 548/548 451/451 547/547","assistant":"32/32 21/21 948/948","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","providerSessions":"2685/2685","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":249,"lastDataAt":1794174541400,"silences":0,"longestSilenceMs":1434,"longestRecoveredGapMs":1434},"stats":{"sendRate":3154.836059963037,"deliveredRate":817.9204599904168,"loss":0.6111111111111112,"srttMs":613.0003886700106,"minRttMs":631,"sentPackets":210,"sentWireBytes":50961,"receivedPackets":501,"receivedWireBytes":184527,"lastHeardMs":28855,"unsyncedBytes":0,"inflightPackets":27,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 595/1574 901/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","messages":"456/456 548/548 901/901 548/548 451/451 548/548","assistant":"32/32 21/21 1340/1340 78/108","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 333/1374 14/963 1022/1022","providerSessions":"2685/2685","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":249,"lastDataAt":1794174552540,"silences":0,"longestSilenceMs":1434,"longestRecoveredGapMs":1434},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.5456209150326797,"srttMs":743.6311197535423,"minRttMs":631,"sentPackets":270,"sentWireBytes":63149,"receivedPackets":872,"receivedWireBytes":286885,"lastHeardMs":39994,"unsyncedBytes":167,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","messages":"456/456 548/548 901/901 548/548 451/451 548/548 547/547","assistant":"32/32 21/21 1340/1340 659/659","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1398/1398 987/987 1022/1022","providerSessions":"2685/2685","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":249,"lastDataAt":1794174563557,"silences":0,"longestSilenceMs":1434,"longestRecoveredGapMs":1434},"stats":{"sendRate":1500,"deliveredRate":889.9876390605685,"loss":0.4545454545454546,"srttMs":899.9691240006548,"minRttMs":578,"sentPackets":355,"sentWireBytes":82679,"receivedPackets":1093,"receivedWireBytes":392772,"lastHeardMs":51012,"unsyncedBytes":489,"inflightPackets":67,"duplicateBytes":0}}
- shim trace
21:48:32.875 fetch GET /api/auth/session
21:48:32.876 fetched GET /api/auth/session 200
21:48:33.004 fetch GET /.well-known/t3/environment
21:48:33.005 fetched GET /.well-known/t3/environment 200
21:48:33.020 fetch GET /.well-known/t3/environment
21:48:33.021 fetched GET /.well-known/t3/environment 200
21:48:33.024 rpc 0 subscribeServerConfig
21:48:33.024 chunk 0 38678 bytes
21:48:33.042 in Ack 0
21:48:33.044 rpc 1 subscribeServerLifecycle
21:48:33.044 rpc 2 subscribeServerLifecycle
21:48:33.046 fetch GET /api/auth/session
21:48:33.047 fetched GET /api/auth/session 200
21:48:33.048 rpc 3 subscribeDeviceState
21:48:33.049 rpc 4 preview.reportProfiles
21:48:33.090 rpc 5 subscribeProjectClones
21:48:33.107 fetch GET /api/orchestration/threads/mcp%3Ad885711e-56c9-4808-a739-580aca194900/bounded
21:48:33.108 fetched GET /api/orchestration/threads/mcp%3Ad885711e-56c9-4808-a739-580aca194900/bounded 200
21:48:33.111 rpc 6 orchestration.subscribeThread
21:48:33.111 chunk 6 8238 bytes
21:48:33.113 in Ack 6
21:48:33.117 fetch GET /api/orchestration/shell
21:48:33.118 fetched GET /api/orchestration/shell 200
21:48:33.119 rpc 7 orchestration.subscribeShell
21:48:33.119 chunk 7 2779 bytes
21:48:33.146 rpc 8 server.refreshProviders
21:48:33.211 rpc 9 orchestration.dispatchCommand thread.visit
21:48:33.211 exit 9 {"_tag":"Success","value":{"sequence":0}}
21:48:33.284 rpc 10 subscribeTerminalMetadata
21:48:33.284 rpc 11 subscribePreviewEvents
21:48:33.285 rpc 12 subscribeVcsStatus
21:48:33.287 rpc 13 assets.createUrl
21:48:33.289 rpc 14 preview.list
21:48:33.290 rpc 15 projects.readFile
21:48:33.299 in Ack 7
21:48:33.362 chunk 6 11991 bytes
21:48:33.363 in Ack 6
21:48:33.396 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"4b6f279e-b22d-46f9-bafa-b01576f58c38","sessions":[]}}
21:48:33.396 chunk 7 2755 bytes
21:48:33.430 in Ack 7
21:48:33.498 chunk 1 3861 bytes
21:48:33.499 in Ack 1
21:48:33.537 rpc 16 server.reportClientActivity
21:48:33.622 chunk 2 3861 bytes
21:48:33.623 in Ack 2
21:48:33.750 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:48:33.750 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:48:20.609Z","configurableProviders":false,"con
21:48:33.750 chunk 12 472 bytes
21:48:33.750 exit 13 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:48:33.753 in Ack 12
21:48:34.028 chunk 6 11991 bytes
21:48:34.028 chunk 7 2798 bytes
21:48:34.030 fetch POST /api/observability/v1/traces
21:48:34.030 fetched POST /api/observability/v1/traces 204
21:48:34.032 in Ack 6
21:48:34.039 in Ack 7
21:48:34.319 chunk 6 13517 bytes
21:48:34.320 in Ack 6
21:48:34.380 rpc 17 attachments.createUploadUrl
21:48:34.569 chunk 6 16202 bytes
21:48:34.570 in Ack 6
21:48:34.599 exit 17 {"_tag":"Success","value":{"attachmentId":"pending-07bc419d-e051-4d2a-88b3-6e7159c4c53b","expiresAt":1794175114568,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:48:34.601 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA3YmM0MTlkLWUwNTEtNGQyYS04OGIzLTZlNzE1OWM0YzUzYiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTExNDU2OH0.K7USXQeOt4fC07r2tvsCjlJ9oyUqF65RFc1iG2J3Xy8
21:48:34.820 chunk 6 19081 bytes
21:48:34.821 in Ack 6
21:48:35.008 chunk 3 876 bytes
21:48:35.008 exit 4 {"_tag":"Success","value":null}
21:48:35.008 chunk 5 4 bytes
21:48:35.008 chunk 10 36 bytes
21:48:35.009 in Ack 3
21:48:35.012 rpc 18 preview.reportProfiles
21:48:35.013 in Ack 5
21:48:35.013 in Ack 10
21:48:35.033 fetch POST /api/observability/v1/traces
21:48:35.033 fetched POST /api/observability/v1/traces 204
21:48:35.067 exit 16 {"_tag":"Success","value":null}
21:48:35.070 chunk 6 21197 bytes
21:48:35.071 in Ack 6
21:48:35.077 in Interrupt 12
21:48:35.079 rpc 19 subscribeVcsStatus
21:48:35.158 chunk 19 472 bytes
21:48:35.158 in Ack 19
21:48:35.331 rpc 20 server.reportClientActivity
21:48:36.068 fetch POST /api/observability/v1/traces
21:48:36.068 fetched POST /api/observability/v1/traces 204
21:48:36.517 exit 18 {"_tag":"Success","value":null}
21:48:36.586 exit 20 {"_tag":"Success","value":null}
21:48:37.624 rpc 21 orchestration.dispatchCommand message.dispatch
21:48:38.212 fetch POST /api/observability/v1/traces
21:48:38.212 fetched POST /api/observability/v1/traces 204
21:48:38.213 in Ping 
21:48:38.257 exit 21 {"_tag":"Success","value":{"sequence":45}}
21:48:38.260 rpc 22 attachments.delete
21:48:38.518 chunk 6 21835 bytes
21:48:38.518 chunk 7 2884 bytes
21:48:38.519 in Ack 6
21:48:38.529 in Ack 7
21:48:38.628 exit 22 {"_tag":"Success","value":null}
21:48:38.768 chunk 6 27875 bytes
21:48:38.769 in Ack 6
21:48:38.772 rpc 23 assets.createUrl
21:48:39.018 chunk 6 29342 bytes
21:48:39.019 in Ack 6
21:48:39.259 fetch POST /api/observability/v1/traces
21:48:39.259 fetched POST /api/observability/v1/traces 204
21:48:39.268 chunk 6 29524 bytes
21:48:39.269 in Ack 6
21:48:39.357 chunk 7 2823 bytes
21:48:39.357 in Ack 7
21:48:39.518 chunk 6 31971 bytes
21:48:39.519 in Ack 6
21:48:39.606 chunk 7 2820 bytes
21:48:39.607 in Ack 7
21:48:39.608 rpc 24 orchestration.dispatchCommand thread.visit
21:48:39.608 exit 24 {"_tag":"Success","value":{"sequence":0}}
21:48:39.767 chunk 6 32775 bytes
21:48:39.769 in Ack 6
21:48:39.775 in Interrupt 19
21:48:39.777 rpc 25 subscribeVcsStatus
21:48:39.857 chunk 7 2820 bytes
21:48:39.858 in Ack 7
21:48:39.862 chunk 25 472 bytes
21:48:39.862 in Ack 25
21:48:40.018 chunk 6 34320 bytes
21:48:40.019 in Ack 6
21:48:40.028 rpc 26 server.reportClientActivity
21:48:40.108 exit 26 {"_tag":"Success","value":null}
21:48:40.268 chunk 6 35231 bytes
21:48:40.269 in Ack 6
21:48:40.277 fetch POST /api/observability/v1/traces
21:48:40.277 fetched POST /api/observability/v1/traces 204
21:48:41.259 rpc 27 orchestration.dispatchCommand message.dispatch
21:48:42.170 exit 27 {"_tag":"Success","value":{"sequence":75}}
21:48:42.259 fetch POST /api/observability/v1/traces
21:48:42.259 fetched POST /api/observability/v1/traces 204
21:48:42.419 chunk 6 37676 bytes
21:48:42.421 in Ack 6
21:48:42.671 chunk 6 37698 bytes
21:48:42.672 in Ack 6
21:48:42.923 chunk 6 37726 bytes
21:48:42.924 in Ack 6
21:48:43.173 chunk 6 37748 bytes
21:48:43.174 in Ack 6
21:48:43.216 in Ping 
21:48:43.422 fetch POST /api/observability/v1/traces
21:48:43.422 fetched POST /api/observability/v1/traces 204
21:48:43.422 chunk 6 37760 bytes
21:48:43.425 in Ack 6
21:48:43.673 chunk 6 37772 bytes
21:48:43.674 in Ack 6
21:48:43.923 chunk 6 37836 bytes
21:48:43.924 in Ack 6
21:48:44.173 chunk 6 37846 bytes
21:48:44.174 in Ack 6
21:48:44.426 fetch POST /api/observability/v1/traces
21:48:44.426 fetched POST /api/observability/v1/traces 204
21:48:44.779 chunk 6 37888 bytes
21:48:44.780 in Ack 6
21:48:45.030 chunk 6 37908 bytes
21:48:45.031 in Ack 6
21:48:45.280 chunk 6 37948 bytes
21:48:45.281 in Ack 6
21:48:45.530 chunk 6 37960 bytes
21:48:45.531 in Ack 6
21:48:45.780 chunk 6 37980 bytes
21:48:45.781 fetch POST /api/observability/v1/traces
21:48:45.781 fetched POST /api/observability/v1/traces 204
21:48:45.782 in Ack 6
21:48:46.030 chunk 6 38012 bytes
21:48:46.031 in Ack 6
21:48:46.280 chunk 6 38024 bytes
21:48:46.281 in Ack 6
21:48:46.746 chunk 6 38036 bytes
21:48:46.746 chunk 7 2884 bytes
21:48:46.748 in Ack 6
21:48:46.757 in Ack 7
21:48:46.793 fetch POST /api/observability/v1/traces
21:48:46.793 fetched POST /api/observability/v1/traces 204
21:48:46.996 chunk 6 38046 bytes
21:48:46.997 in Ack 6
21:48:47.277 chunk 6 38058 bytes
21:48:47.278 in Ack 6
21:48:47.527 chunk 6 38094 bytes
21:48:47.528 in Ack 6
21:48:47.777 chunk 6 40765 bytes
21:48:47.778 in Ack 6
21:48:47.998 fetch POST /api/observability/v1/traces
21:48:47.998 fetched POST /api/observability/v1/traces 204
21:48:48.027 chunk 6 40839 bytes
21:48:48.028 in Ack 6
21:48:48.216 in Ping 
21:48:48.276 chunk 6 40849 bytes
21:48:48.277 in Ack 6
21:48:48.527 chunk 6 40873 bytes
21:48:48.528 in Ack 6
21:48:48.777 chunk 6 40893 bytes
21:48:48.778 in Ack 6
21:48:49.028 chunk 6 40913 bytes
21:48:49.028 fetch POST /api/observability/v1/traces
21:48:49.029 fetched POST /api/observability/v1/traces 204
21:48:49.030 in Ack 6
21:48:49.279 chunk 6 40933 bytes
21:48:49.280 in Ack 6
21:48:49.529 chunk 6 40971 bytes
21:48:49.530 in Ack 6
21:48:49.610 rpc 28 orchestration.dispatchCommand thread.visit
21:48:49.610 exit 28 {"_tag":"Success","value":{"sequence":0}}
21:48:49.780 chunk 6 40991 bytes
21:48:49.781 in Ack 6
21:48:50.031 chunk 6 41841 bytes
21:48:50.032 fetch POST /api/observability/v1/traces
21:48:50.032 fetched POST /api/observability/v1/traces 204
21:48:50.033 in Ack 6
21:48:50.282 chunk 6 41873 bytes
21:48:50.283 in Ack 6
21:48:50.532 chunk 6 41913 bytes
21:48:50.533 in Ack 6
21:48:51.032 chunk 6 41935 bytes
21:48:51.033 in Ack 6
21:48:51.042 fetch POST /api/observability/v1/traces
21:48:51.042 fetched POST /api/observability/v1/traces 204
21:48:51.282 chunk 6 41973 bytes
21:48:51.284 in Ack 6
21:48:51.532 chunk 6 41993 bytes
21:48:51.533 in Ack 6
21:48:51.783 chunk 6 42025 bytes
21:48:51.784 in Ack 6
21:48:52.034 chunk 6 42063 bytes
21:48:52.035 in Ack 6
21:48:52.103 chunk 7 2884 bytes
21:48:52.104 in Ack 7
21:48:52.285 fetch POST /api/observability/v1/traces
21:48:52.285 fetched POST /api/observability/v1/traces 204
21:48:52.286 chunk 6 42083 bytes
21:48:52.288 in Ack 6
21:48:52.537 chunk 6 42113 bytes
21:48:52.538 in Ack 6
21:48:52.604 chunk 7 2884 bytes
21:48:52.605 in Ack 7
21:48:52.787 chunk 6 42131 bytes
21:48:52.789 in Ack 6
21:48:53.037 chunk 6 42179 bytes
21:48:53.038 in Ack 6
21:48:53.217 in Ping 
21:48:53.287 chunk 6 42199 bytes
21:48:53.288 in Ack 6
21:48:53.297 fetch POST /api/observability/v1/traces
21:48:53.297 fetched POST /api/observability/v1/traces 204
21:48:53.537 chunk 6 42219 bytes
21:48:53.538 in Ack 6
21:48:53.858 chunk 6 44031 bytes
21:48:53.860 in Ack 6
21:48:54.108 chunk 6 44577 bytes
21:48:54.110 in Ack 6
21:48:54.361 chunk 6 44622 bytes
21:48:54.362 in Ack 6
21:48:54.539 fetch POST /api/observability/v1/traces
21:48:54.539 fetched POST /api/observability/v1/traces 204
21:48:54.611 chunk 6 44652 bytes
21:48:54.612 in Ack 6
21:48:54.856 exit 23 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/662af0e3-41e1-4172-9a75-63dd21092dc2","expiresAt":1794178134856}}
21:48:54.865 chunk 6 44682 bytes
21:48:54.867 in Ack 6
21:48:55.115 chunk 6 45515 bytes
21:48:55.117 in Ack 6
21:48:55.366 chunk 6 45560 bytes
21:48:55.367 in Ack 6
21:48:55.613 fetch POST /api/observability/v1/traces
21:48:55.613 fetched POST /api/observability/v1/traces 204
21:48:55.616 chunk 6 45605 bytes
21:48:55.617 in Ack 6
21:48:55.866 chunk 6 45635 bytes
21:48:55.867 in Ack 6
21:48:56.116 chunk 6 45680 bytes
21:48:56.117 in Ack 6
21:48:56.366 chunk 6 45725 bytes
21:48:56.367 in Ack 6
21:48:56.616 chunk 6 45755 bytes
21:48:56.617 in Ack 6
21:48:56.653 fetch POST /api/observability/v1/traces
21:48:56.653 fetched POST /api/observability/v1/traces 204
21:48:56.866 chunk 6 45785 bytes
21:48:56.867 in Ack 6
21:48:57.116 chunk 6 45815 bytes
21:48:57.117 in Ack 6
21:48:57.366 chunk 6 45860 bytes
21:48:57.367 in Ack 6
21:48:57.616 chunk 6 45890 bytes
21:48:57.617 in Ack 6
21:48:57.866 chunk 6 45920 bytes
21:48:57.867 in Ack 6
21:48:57.896 fetch POST /api/observability/v1/traces
21:48:57.896 fetched POST /api/observability/v1/traces 204
21:48:58.116 chunk 6 45950 bytes
21:48:58.117 in Ack 6
21:48:58.217 in Ping 
21:48:58.256 rpc 29 server.reportClientActivity
21:48:58.367 chunk 6 45995 bytes
21:48:58.368 in Ack 6
21:48:58.617 chunk 6 46025 bytes
21:48:58.617 in Ack 6
21:48:58.867 chunk 6 46055 bytes
21:48:58.868 in Ack 6
21:48:59.116 chunk 6 46115 bytes
21:48:59.118 in Ack 6
21:48:59.140 fetch POST /api/observability/v1/traces
21:48:59.140 fetched POST /api/observability/v1/traces 204
21:48:59.367 chunk 6 46160 bytes
21:48:59.368 in Ack 6
21:48:59.456 exit 29 {"_tag":"Success","value":null}
21:48:59.610 rpc 30 orchestration.dispatchCommand thread.visit
21:48:59.610 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:48:59.617 chunk 6 46190 bytes
21:48:59.618 in Ack 6
21:48:59.867 chunk 6 46250 bytes
21:48:59.868 in Ack 6
21:49:00.117 chunk 6 46280 bytes
21:49:00.118 in Ack 6
21:49:00.368 chunk 6 46310 bytes
21:49:00.368 fetch POST /api/observability/v1/traces
21:49:00.368 fetched POST /api/observability/v1/traces 204
21:49:00.369 in Ack 6
21:49:00.617 chunk 6 46340 bytes
21:49:00.618 in Ack 6
21:49:00.686 chunk 7 2884 bytes
21:49:00.687 in Ack 7
21:49:00.868 chunk 6 46370 bytes
21:49:00.869 in Ack 6
21:49:01.118 chunk 6 46415 bytes
21:49:01.119 in Ack 6
21:49:01.368 chunk 6 46430 bytes
21:49:01.369 in Ack 6
21:49:01.378 fetch POST /api/observability/v1/traces
21:49:01.378 fetched POST /api/observability/v1/traces 204
21:49:01.618 chunk 6 46505 bytes
21:49:01.619 in Ack 6
21:49:01.868 chunk 6 46550 bytes
21:49:01.869 in Ack 6
21:49:02.118 chunk 6 46580 bytes
21:49:02.119 in Ack 6
21:49:02.368 chunk 6 46610 bytes
21:49:02.369 in Ack 6
21:49:02.715 chunk 6 46670 bytes
21:49:02.716 fetch POST /api/observability/v1/traces
21:49:02.716 fetched POST /api/observability/v1/traces 204
21:49:02.718 in Ack 6
21:49:02.967 chunk 6 46700 bytes
21:49:02.968 in Ack 6
21:49:03.218 chunk 6 46730 bytes
21:49:03.218 in Ping 
21:49:03.219 in Ack 6
21:49:03.468 chunk 6 46775 bytes
21:49:03.470 in Ack 6
21:49:03.719 fetch POST /api/observability/v1/traces
21:49:03.719 fetched POST /api/observability/v1/traces 204
21:49:03.719 chunk 6 46805 bytes
21:49:03.720 in Ack 6
21:49:03.969 chunk 6 46835 bytes
21:49:03.969 in Ack 6
21:49:04.219 chunk 6 46880 bytes
21:49:04.220 in Ack 6
21:49:04.470 chunk 6 46925 bytes
21:49:04.471 in Ack 6
21:49:04.721 fetch POST /api/observability/v1/traces
21:49:04.721 fetched POST /api/observability/v1/traces 204
21:49:04.721 chunk 6 46955 bytes
21:49:04.722 in Ack 6
21:49:04.971 chunk 6 46985 bytes
21:49:04.972 in Ack 6
21:49:05.220 chunk 6 47015 bytes
21:49:05.222 in Ack 6
21:49:05.470 chunk 6 47060 bytes
21:49:05.472 in Ack 6
21:49:05.721 chunk 6 47105 bytes
21:49:05.722 in Ack 6
21:49:05.731 fetch POST /api/observability/v1/traces
21:49:05.731 fetched POST /api/observability/v1/traces 204
21:49:05.983 chunk 6 47150 bytes
21:49:05.985 in Ack 6
21:49:06.234 chunk 6 47180 bytes
21:49:06.235 in Ack 6
21:49:06.484 chunk 6 47240 bytes
21:49:06.485 in Ack 6
21:49:06.734 chunk 6 47255 bytes
21:49:06.735 in Ack 6
21:49:06.984 chunk 6 47315 bytes
21:49:06.985 in Ack 6
21:49:06.993 fetch POST /api/observability/v1/traces
21:49:06.993 fetched POST /api/observability/v1/traces 204
21:49:07.235 chunk 6 47330 bytes
21:49:07.236 in Ack 6
21:49:07.485 chunk 6 47360 bytes
21:49:07.486 in Ack 6
21:49:07.735 chunk 6 47390 bytes
21:49:07.736 in Ack 6
21:49:07.986 chunk 6 47435 bytes
21:49:07.987 in Ack 6
21:49:08.219 in Ping 
21:49:08.236 chunk 6 47465 bytes
21:49:08.236 fetch POST /api/observability/v1/traces
21:49:08.237 fetched POST /api/observability/v1/traces 204
21:49:08.238 in Ack 6
21:49:08.487 chunk 6 47495 bytes
21:49:08.488 in Ack 6
21:49:08.737 chunk 6 47540 bytes
21:49:08.738 in Ack 6
21:49:08.988 chunk 6 47570 bytes
21:49:08.988 in Ack 6
21:49:09.238 chunk 6 47600 bytes
21:49:09.239 in Ack 6
21:49:09.247 fetch POST /api/observability/v1/traces
21:49:09.247 fetched POST /api/observability/v1/traces 204
21:49:09.488 chunk 6 47630 bytes
21:49:09.488 in Ack 6
21:49:09.738 chunk 6 48650 bytes
21:49:09.739 in Ack 6
21:49:09.991 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:49:10.025 chunk 6 52155 bytes
21:49:10.029 in Ack 6
21:49:10.275 chunk 6 52205 bytes
21:49:10.276 in Ack 6
21:49:10.340 chunk 7 3193 bytes
21:49:10.341 in Ack 7
21:49:10.490 fetch POST /api/observability/v1/traces
21:49:10.490 fetched POST /api/observability/v1/traces 204
21:49:10.776 chunk 6 55311 bytes
21:49:10.777 in Ack 6
21:49:11.404 chunk 6 57813 bytes
21:49:11.405 exit 31 {"_tag":"Success","value":{"sequence":97}}
21:49:11.406 in Ack 6
21:49:11.447 rpc 32 orchestration.dispatchCommand thread.visit
21:49:11.447 exit 32 {"_tag":"Success","value":{"sequence":0}}
21:49:11.655 chunk 6 57878 bytes
21:49:11.656 in Ack 6
21:49:11.778 fetch POST /api/observability/v1/traces
21:49:11.778 fetched POST /api/observability/v1/traces 204
21:49:11.905 chunk 6 57852 bytes
21:49:11.907 in Ack 6
21:49:12.155 chunk 6 57884 bytes
21:49:12.156 in Ack 6
21:49:12.175 chunk 7 2886 bytes
21:49:12.175 in Ack 7
21:49:12.406 chunk 6 57897 bytes
21:49:12.407 in Ack 6
21:49:12.657 chunk 6 57907 bytes
21:49:12.658 in Ack 6
21:49:12.913 chunk 6 57963 bytes
21:49:12.914 fetch POST /api/observability/v1/traces
21:49:12.914 fetched POST /api/observability/v1/traces 204
21:49:12.915 in Ack 6
21:49:12.952 chunk 7 2886 bytes
21:49:12.953 in Ack 7
21:49:13.163 chunk 6 58019 bytes
21:49:13.164 in Ack 6
21:49:13.219 in Ping 
21:49:13.413 chunk 6 58039 bytes
21:49:13.415 in Ack 6
21:49:13.664 chunk 6 58079 bytes
21:49:13.665 in Ack 6
21:49:13.914 chunk 6 58089 bytes
21:49:13.915 in Ack 6
21:49:13.924 fetch POST /api/observability/v1/traces
21:49:13.924 fetched POST /api/observability/v1/traces 204
21:49:14.164 chunk 6 58119 bytes
21:49:14.165 in Ack 6
21:49:14.415 chunk 6 58187 bytes
21:49:14.416 in Ack 6
21:49:14.665 chunk 6 59205 bytes
21:49:14.666 in Ack 6
21:49:14.915 chunk 6 59283 bytes
21:49:14.916 in Ack 6
21:49:14.925 in Interrupt 25
21:49:14.950 rpc 33 subscribeVcsStatus
21:49:15.169 chunk 6 59303 bytes
21:49:15.170 fetch POST /api/observability/v1/traces
21:49:15.170 fetched POST /api/observability/v1/traces 204
21:49:15.171 in Ack 6
21:49:15.200 rpc 34 server.reportClientActivity
21:49:15.419 chunk 6 59333 bytes
21:49:15.420 in Ack 6
21:49:15.670 chunk 6 60752 bytes
21:49:15.671 in Ack 6
21:49:15.792 chunk 33 472 bytes
21:49:15.793 in Ack 33
21:49:15.920 chunk 6 60782 bytes
21:49:15.921 in Ack 6
21:49:16.178 chunk 6 60802 bytes
21:49:16.178 fetch POST /api/observability/v1/traces
21:49:16.178 fetched POST /api/observability/v1/traces 204
21:49:16.180 in Ack 6
21:49:16.428 chunk 6 60822 bytes
21:49:16.429 in Ack 6
21:49:16.678 chunk 6 60852 bytes
21:49:16.679 in Ack 6
21:49:16.928 chunk 6 60872 bytes
21:49:16.929 in Ack 6
21:49:17.179 chunk 6 60912 bytes
21:49:17.180 in Ack 6
21:49:17.206 fetch POST /api/observability/v1/traces
21:49:17.206 fetched POST /api/observability/v1/traces 204
21:49:17.429 chunk 6 60922 bytes
21:49:17.430 in Ack 6
21:49:17.679 chunk 6 60942 bytes
21:49:17.683 in Ack 6
21:49:17.871 exit 34 {"_tag":"Success","value":null}
21:49:17.929 chunk 6 60972 bytes
21:49:17.930 in Ack 6
21:49:18.179 chunk 6 60992 bytes
21:49:18.180 in Ack 6
21:49:18.223 in Ping 
21:49:18.429 chunk 6 61012 bytes
21:49:18.430 in Ack 6
21:49:18.456 fetch POST /api/observability/v1/traces
21:49:18.456 fetched POST /api/observability/v1/traces 204
21:49:18.679 chunk 6 61032 bytes
21:49:18.680 in Ack 6
21:49:18.929 chunk 6 61048 bytes
21:49:18.930 in Ack 6
21:49:18.995 chunk 7 2886 bytes
21:49:18.996 in Ack 7
21:49:19.180 chunk 6 61060 bytes
21:49:19.181 in Ack 6
21:49:19.431 chunk 6 61100 bytes
21:49:19.432 in Ack 6
21:49:19.681 fetch POST /api/observability/v1/traces
21:49:19.681 fetched POST /api/observability/v1/traces 204
21:49:19.682 chunk 6 61120 bytes
21:49:19.683 in Ack 6
21:49:19.932 chunk 6 61150 bytes
21:49:19.933 in Ack 6
21:49:20.192 chunk 6 61170 bytes
21:49:20.193 in Ack 6
21:49:20.343 rpc 35 orchestration.dispatchCommand thread.visit
21:49:20.343 exit 35 {"_tag":"Success","value":{"sequence":0}}
21:49:20.443 chunk 6 61194 bytes
21:49:20.444 in Ack 6
21:49:20.684 fetch POST /api/observability/v1/traces
21:49:20.684 fetched POST /api/observability/v1/traces 204
21:49:20.693 chunk 6 62256 bytes
21:49:20.694 in Ack 6
21:49:20.943 chunk 6 62286 bytes
21:49:20.944 in Ack 6
21:49:21.193 chunk 6 62322 bytes
21:49:21.194 in Ack 6
21:49:21.444 chunk 6 62397 bytes
21:49:21.445 in Ack 6
21:49:21.535 chunk 7 2886 bytes
21:49:21.536 in Ack 7
21:49:21.695 chunk 6 62433 bytes
21:49:21.695 fetch POST /api/observability/v1/traces
21:49:21.695 fetched POST /api/observability/v1/traces 204
21:49:21.696 in Ack 6
21:49:21.945 chunk 6 62508 bytes
21:49:21.946 in Ack 6
21:49:22.196 chunk 6 62562 bytes
21:49:22.197 in Ack 6
21:49:22.446 chunk 6 62607 bytes
21:49:22.448 in Ack 6
21:49:22.696 chunk 6 62637 bytes
21:49:22.698 in Ack 6
21:49:22.711 fetch POST /api/observability/v1/traces
21:49:22.711 fetched POST /api/observability/v1/traces 204
21:49:22.946 chunk 6 62694 bytes
21:49:22.947 in Ack 6
21:49:23.196 chunk 6 62748 bytes
21:49:23.198 in Ack 6
21:49:23.281 in Ping 
21:49:23.282 rpc 36 server.reportClientActivity
21:49:23.447 chunk 6 62778 bytes
21:49:23.449 in Ack 6
21:49:23.697 chunk 6 62808 bytes
21:49:23.699 in Ack 6
21:49:23.948 fetch POST /api/observability/v1/traces
21:49:23.948 fetched POST /api/observability/v1/traces 204
21:49:23.949 chunk 6 62859 bytes
21:49:23.950 in Ack 6
21:49:24.076 exit 36 {"_tag":"Success","value":null}
21:49:24.199 chunk 6 62913 bytes
21:49:24.200 in Ack 6
21:49:24.449 chunk 6 62961 bytes
21:49:24.451 in Ack 6
21:49:24.699 chunk 6 62976 bytes
21:49:24.700 in Ack 6
21:49:24.949 chunk 6 63074 bytes
21:49:24.951 in Ack 6
21:49:25.022 fetch POST /api/observability/v1/traces
21:49:25.022 fetched POST /api/observability/v1/traces 204
21:49:25.287 chunk 7 2822 bytes
21:49:25.288 in Ack 7
21:49:25.289 rpc 37 orchestration.dispatchCommand thread.visit
21:49:25.289 exit 37 {"_tag":"Success","value":{"sequence":0}}
21:49:25.449 chunk 6 65654 bytes
21:49:25.452 in Ack 6
21:49:25.700 chunk 6 66292 bytes
21:49:25.701 in Ack 6
21:49:25.707 in Interrupt 33
21:49:25.709 rpc 38 subscribeVcsStatus
21:49:26.002 chunk 6 66409 bytes
21:49:26.002 rpc 39 server.reportClientActivity
21:49:26.004 in Ack 6
