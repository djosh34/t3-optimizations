# case50, webapp, cold start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 70 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.2s |
| Prompt confirmation | 23.2s |
| Prompt reached the agent | 22.8s |
| Time to first text | 24.3s |
| Whole reply shown | 67.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.0s / 0.9s / 2.4s |
| Longest silence | 24.3s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 291494 / 118918 |
| Bytes the app received / sent | 221934 / 95588 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 1.9 | 211 | 291 |
| 40 | 0.7 | 655 | 689 |
| 50 | 0.6 | 1065 | 1077 |
| 60 | 0.8 | 1378 | 1414 |
| 67 | 1.8 | 1587 | 1596 |

## Driver notes

- webapp {"objects":{"nodes":"809/809 809/809 1019/1019","checkpoints":"779/779 613/613 531/613","user":"10/10 9/9","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870","thread":"951/951","runs":"1014/1014 971/971","attempts":"660/660 660/660","messages":"456/456 451/901 546/546","assistant":"32/32 21/21","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774","frame":"1030/1030","providerThreads":"808/808"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2700,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019","checkpoints":"779/779 613/613 531/613","user":"10/10 9/9","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870","thread":"951/951","runs":"1014/1014 971/971","attempts":"660/660 660/660","messages":"456/456 451/901 546/546","assistant":"32/32 21/21","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774","frame":"1030/1030","providerThreads":"808/808"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2700,"inflightPackets":0,"duplicateBytes":0}}
- prompt sent
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019","checkpoints":"779/779 613/613 531/613","user":"10/10 9/9","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870","thread":"951/951","runs":"1014/1014 971/971","attempts":"660/660 660/660","messages":"456/456 451/901 546/546","assistant":"32/32 21/21","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774","frame":"1030/1030","providerThreads":"808/808"},"link":{"transport":"webrtc","trying":"webrtc open","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":20268,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":5,"sentWireBytes":5910,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2700,"inflightPackets":5,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019","checkpoints":"779/779 613/613 531/613","user":"10/10 9/9","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870 1073/1073 849/849","thread":"951/951","runs":"1014/1014 971/971","attempts":"660/660 660/660","messages":"456/456 451/901 546/546","assistant":"32/32 21/21 330/345","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774","frame":"1030/1030","providerThreads":"808/808"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":20268,"lastDataAt":1794165439137,"silences":0,"longestSilenceMs":1024,"longestRecoveredGapMs":1024},"stats":{"sendRate":4761.582899054005,"deliveredRate":1662.082821553939,"loss":0.41947089947089944,"srttMs":1153.9497920257847,"minRttMs":589,"sentPackets":73,"sentWireBytes":25564,"receivedPackets":39,"receivedWireBytes":19887,"lastHeardMs":32496,"unsyncedBytes":0,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019","checkpoints":"779/779 613/613 531/613","user":"10/10 9/9 54/64","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870 1073/1073 849/849","thread":"486/951","runs":"1014/1014 971/971 85/839","attempts":"660/660 660/660 630/636","messages":"456/456 451/901 546/546","assistant":"32/32 21/21 853/858","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774","frame":"1030/1030","providerThreads":"808/808"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":20268,"lastDataAt":1794165449460,"silences":0,"longestSilenceMs":1024,"longestRecoveredGapMs":1024},"stats":{"sendRate":3000,"deliveredRate":2157.528285465622,"loss":0.3076923076923077,"srttMs":930.0098313774831,"minRttMs":589,"sentPackets":129,"sentWireBytes":34895,"receivedPackets":109,"receivedWireBytes":41173,"lastHeardMs":42819,"unsyncedBytes":0,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019 785/785 948/1019","checkpoints":"779/779 613/613 613/613","user":"10/10 9/9 64/64","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870 1073/1073 849/849","thread":"951/951","runs":"1014/1014 971/971 839/839","attempts":"660/660 660/660 636/636","messages":"456/456 865/901 546/546 545/545 451/451 546/546","assistant":"32/32 21/21 1333/1333","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774 656/656 774/774","frame":"1030/1030","providerThreads":"810/810","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":20268,"lastDataAt":1794165459590,"silences":0,"longestSilenceMs":1024,"longestRecoveredGapMs":1024},"stats":{"sendRate":3000,"deliveredRate":704.5025022078303,"loss":0.6390029325513196,"srttMs":798.0784456174315,"minRttMs":589,"sentPackets":200,"sentWireBytes":49075,"receivedPackets":226,"receivedWireBytes":85407,"lastHeardMs":52949,"unsyncedBytes":0,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"nodes":"809/809 809/809 1019/1019 785/785 1019/1019 985/985 1395/1395 1019/1019","checkpoints":"779/779 613/613 613/613","user":"10/10 9/9 64/64","turnItems":"1058/1058 1058/1058 1098/1098 1299/1299 1098/1098 860/860 870/870 1098/1098 849/849 1570/1570 950/950 1073/1073","thread":"951/951","runs":"1014/1014 971/971 839/839","attempts":"660/660 660/660 636/636","messages":"456/456 901/901 546/546 546/546 451/451 546/546 142/545","assistant":"32/32 21/21 1340/1340 464/464","checkpointScopes":"731/731","providerSessions":"2693/2693","providerTurns":"774/774 656/656 774/774","frame":"1030/1030","providerThreads":"810/810","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":20268,"lastDataAt":1794165470166,"silences":0,"longestSilenceMs":1024,"longestRecoveredGapMs":1024},"stats":{"sendRate":4165.3179895773455,"deliveredRate":1100.5672567836548,"loss":0.6036675016634219,"srttMs":904.793409026611,"minRttMs":589,"sentPackets":308,"sentWireBytes":84553,"receivedPackets":366,"receivedWireBytes":168229,"lastHeardMs":63525,"unsyncedBytes":239,"inflightPackets":112,"duplicateBytes":0}}
- shim trace
19:16:47.126 fetch GET /api/auth/session
19:16:47.129 fetched GET /api/auth/session 200
19:16:47.232 fetch GET /.well-known/t3/environment
19:16:47.233 fetched GET /.well-known/t3/environment 200
19:16:47.252 fetch GET /.well-known/t3/environment
19:16:47.253 fetched GET /.well-known/t3/environment 200
19:16:47.260 rpc 0 subscribeServerConfig
19:16:47.260 chunk 0 38710 bytes
19:16:47.281 in Ack 0
19:16:47.285 rpc 1 subscribeServerLifecycle
19:16:47.286 rpc 2 subscribeServerLifecycle
19:16:47.290 fetch GET /api/auth/session
19:16:47.291 fetched GET /api/auth/session 200
19:16:47.291 rpc 3 subscribeDeviceState
19:16:47.294 rpc 4 preview.reportProfiles
19:16:47.396 rpc 5 subscribeProjectClones
19:16:47.439 rpc 6 orchestration.subscribeThread
19:16:47.440 chunk 6 31344 bytes
19:16:47.444 in Ack 6
19:16:47.453 fetch GET /api/orchestration/shell
19:16:47.510 rpc 7 server.refreshProviders
19:16:47.607 rpc 8 orchestration.dispatchCommand thread.visit
19:16:47.607 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:16:47.691 fetched GET /api/orchestration/shell 200
19:16:47.692 rpc 9 subscribeTerminalMetadata
19:16:47.693 rpc 10 subscribePreviewEvents
19:16:47.694 rpc 11 subscribeVcsStatus
19:16:47.697 rpc 12 orchestration.subscribeShell
19:16:47.697 chunk 12 2852 bytes
19:16:47.699 rpc 13 assets.createUrl
19:16:47.699 rpc 14 preview.list
19:16:47.700 rpc 15 projects.readFile
19:16:47.700 rpc 16 assets.createUrl
19:16:47.751 in Ack 12
19:16:47.957 rpc 17 server.reportClientActivity
19:16:48.228 fetch POST /api/observability/v1/traces
19:16:48.228 fetched POST /api/observability/v1/traces 204
19:16:49.118 rpc 18 orchestration.dispatchCommand message.dispatch
19:16:50.117 fetch POST /api/observability/v1/traces
19:16:50.117 fetched POST /api/observability/v1/traces 204
19:16:52.257 in Ping 
19:16:57.257 in Ping 
19:17:02.257 in Ping 
19:17:02.958 in Interrupt 17
19:17:03.959 fetch POST /api/observability/v1/traces
19:17:03.959 fetched POST /api/observability/v1/traces 204
19:17:07.258 in Ping 
19:17:11.170 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"03a703e9-4951-4b91-9507-b36feb6d3a23","sessions":[]}}
19:17:11.170 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:17:12.171 fetch POST /api/observability/v1/traces
19:17:12.171 fetched POST /api/observability/v1/traces 204
19:17:12.192 exit 18 {"_tag":"Success","value":{"sequence":76}}
19:17:12.258 in Ping 
19:17:12.487 rpc 19 server.reportClientActivity
19:17:12.919 chunk 6 33768 bytes
19:17:12.919 chunk 12 2828 bytes
19:17:12.921 in Ack 6
19:17:12.998 in Ack 12
19:17:13.053 chunk 1 3869 bytes
19:17:13.053 in Ack 1
19:17:13.169 chunk 6 33814 bytes
19:17:13.170 in Ack 6
19:17:13.197 chunk 2 3869 bytes
19:17:13.198 fetch POST /api/observability/v1/traces
19:17:13.198 fetched POST /api/observability/v1/traces 204
19:17:13.198 in Ack 2
19:17:13.312 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:16:26.464Z","configurableProviders":false,"con
19:17:13.312 chunk 11 725 bytes
19:17:13.316 in Ack 11
19:17:13.419 chunk 6 33844 bytes
19:17:13.420 in Ack 6
19:17:13.444 exit 13 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:17:13.670 chunk 6 33854 bytes
19:17:13.671 in Ack 6
19:17:13.962 chunk 6 33866 bytes
19:17:13.963 in Ack 6
19:17:14.199 fetch POST /api/observability/v1/traces
19:17:14.199 fetched POST /api/observability/v1/traces 204
19:17:14.212 chunk 6 33876 bytes
19:17:14.214 in Ack 6
19:17:14.463 chunk 6 33898 bytes
19:17:14.464 in Ack 6
19:17:14.714 chunk 6 33930 bytes
19:17:14.715 in Ack 6
19:17:14.964 chunk 6 33940 bytes
19:17:14.965 in Ack 6
19:17:15.214 fetch POST /api/observability/v1/traces
19:17:15.214 fetched POST /api/observability/v1/traces 204
19:17:15.315 chunk 6 33972 bytes
19:17:15.316 in Ack 6
19:17:15.565 chunk 6 33992 bytes
19:17:15.566 in Ack 6
19:17:15.848 chunk 6 33982 bytes
19:17:15.850 in Ack 6
19:17:16.099 chunk 6 34010 bytes
19:17:16.100 in Ack 6
19:17:16.317 fetch POST /api/observability/v1/traces
19:17:16.317 fetched POST /api/observability/v1/traces 204
19:17:16.349 chunk 6 34088 bytes
19:17:16.350 in Ack 6
19:17:16.599 chunk 6 34100 bytes
19:17:16.600 in Ack 6
19:17:16.850 chunk 6 34110 bytes
19:17:16.852 in Ack 6
19:17:17.100 chunk 6 34132 bytes
19:17:17.101 in Ack 6
19:17:17.258 in Ping 
19:17:17.350 chunk 6 34162 bytes
19:17:17.351 in Ack 6
19:17:17.361 fetch POST /api/observability/v1/traces
19:17:17.361 fetched POST /api/observability/v1/traces 204
19:17:17.600 chunk 6 34200 bytes
19:17:17.601 in Ack 6
19:17:17.850 chunk 6 34232 bytes
19:17:17.851 in Ack 6
19:17:18.101 chunk 6 34242 bytes
19:17:18.102 in Ack 6
19:17:18.351 chunk 6 34272 bytes
19:17:18.352 in Ack 6
19:17:18.601 chunk 6 34302 bytes
19:17:18.602 fetch POST /api/observability/v1/traces
19:17:18.602 fetched POST /api/observability/v1/traces 204
19:17:18.603 in Ack 6
19:17:18.852 chunk 6 34332 bytes
19:17:18.853 in Ack 6
19:17:19.102 chunk 6 34352 bytes
19:17:19.104 in Ack 6
19:17:19.352 chunk 6 34362 bytes
19:17:19.354 in Ack 6
19:17:19.604 fetch POST /api/observability/v1/traces
19:17:19.604 fetched POST /api/observability/v1/traces 204
19:17:19.759 chunk 6 34394 bytes
19:17:19.761 in Ack 6
19:17:20.010 chunk 6 34414 bytes
19:17:20.011 in Ack 6
19:17:20.259 chunk 6 34426 bytes
19:17:20.260 in Ack 6
19:17:20.509 chunk 6 34446 bytes
19:17:20.510 in Ack 6
19:17:20.760 chunk 6 34526 bytes
19:17:20.761 in Ack 6
19:17:20.775 fetch POST /api/observability/v1/traces
19:17:20.775 fetched POST /api/observability/v1/traces 204
19:17:21.023 chunk 6 34566 bytes
19:17:21.024 in Ack 6
19:17:21.273 chunk 6 34586 bytes
19:17:21.274 in Ack 6
19:17:21.780 chunk 6 34628 bytes
19:17:21.781 in Ack 6
19:17:22.025 fetch POST /api/observability/v1/traces
19:17:22.025 fetched POST /api/observability/v1/traces 204
19:17:22.030 chunk 6 34648 bytes
19:17:22.031 in Ack 6
19:17:22.258 in Ping 
19:17:22.280 chunk 6 34668 bytes
19:17:22.281 in Ack 6
19:17:22.531 chunk 6 34716 bytes
19:17:22.532 in Ack 6
19:17:22.781 chunk 6 34736 bytes
19:17:22.782 in Ack 6
19:17:23.032 fetch POST /api/observability/v1/traces
19:17:23.032 fetched POST /api/observability/v1/traces 204
19:17:23.389 chunk 6 34806 bytes
19:17:23.390 in Ack 6
19:17:23.819 chunk 6 34846 bytes
19:17:23.820 in Ack 6
19:17:24.069 chunk 6 34858 bytes
19:17:24.070 in Ack 6
19:17:24.319 chunk 6 34868 bytes
19:17:24.320 in Ack 6
19:17:24.391 fetch POST /api/observability/v1/traces
19:17:24.391 fetched POST /api/observability/v1/traces 204
19:17:24.569 chunk 6 34898 bytes
19:17:24.570 in Ack 6
19:17:24.820 chunk 6 34918 bytes
19:17:24.821 in Ack 6
19:17:25.070 chunk 6 34928 bytes
19:17:25.071 in Ack 6
19:17:25.320 chunk 6 34988 bytes
19:17:25.321 in Ack 6
19:17:25.570 fetch POST /api/observability/v1/traces
19:17:25.571 fetched POST /api/observability/v1/traces 204
19:17:25.571 chunk 6 35010 bytes
19:17:25.572 in Ack 6
19:17:25.821 chunk 6 35030 bytes
19:17:25.822 in Ack 6
19:17:25.856 chunk 12 2892 bytes
19:17:25.856 in Ack 12
19:17:25.858 rpc 20 orchestration.dispatchCommand thread.visit
19:17:25.858 exit 20 {"_tag":"Success","value":{"sequence":0}}
19:17:26.071 chunk 6 35040 bytes
19:17:26.072 in Ack 6
19:17:26.321 chunk 6 35080 bytes
19:17:26.322 in Ack 6
19:17:26.574 fetch POST /api/observability/v1/traces
19:17:26.574 fetched POST /api/observability/v1/traces 204
19:17:26.705 chunk 6 35092 bytes
19:17:26.707 in Ack 6
19:17:26.955 chunk 6 35102 bytes
19:17:26.956 in Ack 6
19:17:27.205 chunk 6 35142 bytes
19:17:27.207 in Ack 6
19:17:27.259 in Ping 
19:17:27.462 chunk 6 35172 bytes
19:17:27.463 in Ack 6
19:17:27.488 in Interrupt 19
19:17:27.707 fetch POST /api/observability/v1/traces
19:17:27.707 fetched POST /api/observability/v1/traces 204
19:17:27.712 chunk 6 35218 bytes
19:17:27.713 in Ack 6
19:17:27.962 chunk 6 35256 bytes
19:17:27.963 in Ack 6
19:17:28.213 chunk 6 35286 bytes
19:17:28.214 in Ack 6
19:17:28.463 chunk 6 35316 bytes
19:17:28.464 in Ack 6
19:17:28.713 chunk 6 35336 bytes
19:17:28.714 in Ack 6
19:17:28.732 fetch POST /api/observability/v1/traces
19:17:28.732 fetched POST /api/observability/v1/traces 204
19:17:28.965 chunk 6 35348 bytes
19:17:28.966 in Ack 6
19:17:29.215 chunk 6 35378 bytes
19:17:29.216 in Ack 6
19:17:29.466 chunk 6 35408 bytes
19:17:29.467 in Ack 6
19:17:29.716 chunk 6 35418 bytes
19:17:29.719 in Ack 6
19:17:29.969 fetch POST /api/observability/v1/traces
19:17:29.969 fetched POST /api/observability/v1/traces 204
19:17:29.970 chunk 6 36065 bytes
19:17:29.971 in Ack 6
19:17:30.276 chunk 6 36075 bytes
19:17:30.278 in Ack 6
19:17:30.527 chunk 6 36115 bytes
19:17:30.528 in Ack 6
19:17:30.777 chunk 6 36125 bytes
19:17:30.778 in Ack 6
19:17:30.971 fetch POST /api/observability/v1/traces
19:17:30.971 fetched POST /api/observability/v1/traces 204
19:17:31.028 chunk 6 36147 bytes
19:17:31.029 in Ack 6
19:17:31.278 chunk 6 38210 bytes
19:17:31.279 in Ack 6
19:17:31.309 chunk 12 2892 bytes
19:17:31.309 in Ack 12
19:17:31.528 chunk 6 38230 bytes
19:17:31.529 in Ack 6
19:17:31.778 chunk 6 39110 bytes
19:17:31.779 in Ack 6
19:17:32.028 chunk 6 39138 bytes
19:17:32.029 in Ack 6
19:17:32.038 fetch POST /api/observability/v1/traces
19:17:32.038 fetched POST /api/observability/v1/traces 204
19:17:32.259 in Ping 
19:17:32.277 chunk 6 39160 bytes
19:17:32.279 in Ack 6
19:17:32.707 chunk 6 39182 bytes
19:17:32.708 in Ack 6
19:17:32.958 chunk 6 39210 bytes
19:17:32.959 in Ack 6
19:17:33.208 chunk 6 40034 bytes
19:17:33.209 in Ack 6
19:17:33.280 fetch POST /api/observability/v1/traces
19:17:33.280 fetched POST /api/observability/v1/traces 204
19:17:33.554 chunk 6 40046 bytes
19:17:33.556 in Ack 6
19:17:33.805 chunk 6 40084 bytes
19:17:33.806 in Ack 6
19:17:34.055 chunk 6 40106 bytes
19:17:34.056 in Ack 6
19:17:34.558 fetch POST /api/observability/v1/traces
19:17:34.558 fetched POST /api/observability/v1/traces 204
19:17:34.642 chunk 6 40128 bytes
19:17:34.643 in Ack 6
19:17:34.842 exit 4 {"_tag":"Success","value":null}
19:17:34.842 chunk 5 4 bytes
19:17:34.842 chunk 9 36 bytes
19:17:34.846 rpc 21 preview.reportProfiles
19:17:34.847 in Ack 5
19:17:34.847 in Ack 9
19:17:34.893 chunk 6 40158 bytes
19:17:34.894 in Ack 6
19:17:35.143 chunk 6 40242 bytes
19:17:35.144 in Ack 6
19:17:35.393 chunk 6 40262 bytes
19:17:35.394 in Ack 6
19:17:35.643 chunk 6 40282 bytes
19:17:35.644 in Ack 6
19:17:35.653 fetch POST /api/observability/v1/traces
19:17:35.653 fetched POST /api/observability/v1/traces 204
19:17:35.893 chunk 6 40294 bytes
19:17:35.894 in Ack 6
19:17:35.974 exit 17 {"_tag":"Success","value":null}
19:17:35.974 exit 19 {"_tag":"Success","value":null}
19:17:36.145 chunk 6 40975 bytes
19:17:36.146 in Ack 6
19:17:36.396 chunk 6 41609 bytes
19:17:36.397 in Ack 6
19:17:36.645 chunk 6 42197 bytes
19:17:36.646 in Ack 6
19:17:36.888 exit 21 {"_tag":"Success","value":null}
19:17:36.895 fetch POST /api/observability/v1/traces
19:17:36.895 fetched POST /api/observability/v1/traces 204
19:17:36.896 chunk 6 42245 bytes
19:17:36.897 in Ack 6
19:17:37.109 chunk 3 876 bytes
19:17:37.110 in Ack 3
19:17:37.146 chunk 6 42255 bytes
19:17:37.147 in Ack 6
19:17:37.259 in Ping 
19:17:37.396 chunk 6 43060 bytes
19:17:37.397 in Ack 6
19:17:37.486 rpc 22 server.reportClientActivity
19:17:37.647 chunk 6 43070 bytes
19:17:37.648 in Ack 6
19:17:37.897 chunk 6 43626 bytes
19:17:37.898 in Ack 6
19:17:37.905 fetch POST /api/observability/v1/traces
19:17:37.906 fetched POST /api/observability/v1/traces 204
19:17:38.147 chunk 6 43638 bytes
19:17:38.148 in Ack 6
19:17:38.397 chunk 6 43686 bytes
19:17:38.398 in Ack 6
19:17:38.648 chunk 6 45585 bytes
19:17:38.649 in Ack 6
19:17:38.897 chunk 6 45615 bytes
19:17:38.898 in Ack 6
19:17:39.030 exit 22 {"_tag":"Success","value":null}
19:17:39.148 chunk 6 45645 bytes
19:17:39.149 in Ack 6
19:17:39.157 fetch POST /api/observability/v1/traces
19:17:39.157 fetched POST /api/observability/v1/traces 204
19:17:39.648 chunk 6 46650 bytes
19:17:39.649 in Ack 6
19:17:39.895 rpc 23 orchestration.dispatchCommand runtime-request.respond
19:17:39.917 chunk 6 50146 bytes
19:17:39.944 in Ack 6
19:17:40.167 chunk 6 50149 bytes
19:17:40.168 in Ack 6
19:17:40.417 chunk 6 50173 bytes
19:17:40.418 in Ack 6
19:17:40.651 fetch POST /api/observability/v1/traces
19:17:40.651 fetched POST /api/observability/v1/traces 204
19:17:40.847 exit 23 {"_tag":"Success","value":{"sequence":97}}
19:17:40.918 chunk 6 52644 bytes
19:17:40.919 in Ack 6
19:17:41.169 chunk 6 52656 bytes
19:17:41.170 in Ack 6
19:17:41.225 rpc 24 orchestration.dispatchCommand runtime-request.respond
19:17:41.420 chunk 6 52676 bytes
19:17:41.421 in Ack 6
19:17:41.668 chunk 6 52708 bytes
19:17:41.670 in Ack 6
19:17:41.849 fetch POST /api/observability/v1/traces
19:17:41.849 fetched POST /api/observability/v1/traces 204
19:17:41.919 chunk 6 52718 bytes
19:17:41.920 in Ack 6
19:17:42.169 chunk 6 52730 bytes
19:17:42.170 in Ack 6
19:17:42.263 in Ping 
19:17:42.375 exit 24 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:17:42.580 chunk 6 52838 bytes
19:17:42.583 in Ack 6
19:17:42.691 rpc 25 orchestration.dispatchCommand runtime-request.respond
19:17:42.830 chunk 6 52878 bytes
19:17:42.831 in Ack 6
19:17:42.921 fetch POST /api/observability/v1/traces
19:17:42.921 fetched POST /api/observability/v1/traces 204
19:17:43.081 chunk 6 52896 bytes
19:17:43.095 in Ack 6
19:17:43.330 chunk 6 52926 bytes
19:17:43.332 in Ack 6
19:17:43.737 chunk 6 52959 bytes
19:17:43.737 exit 25 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:17:43.738 in Ack 6
19:17:43.948 rpc 26 orchestration.dispatchCommand runtime-request.respond
19:17:43.987 chunk 6 52989 bytes
19:17:43.987 chunk 12 2894 bytes
19:17:43.988 in Ack 6
19:17:43.996 in Ack 12
19:17:44.096 fetch POST /api/observability/v1/traces
19:17:44.096 fetched POST /api/observability/v1/traces 204
19:17:44.236 chunk 6 53025 bytes
19:17:44.237 in Ack 6
19:17:44.487 chunk 6 53055 bytes
19:17:44.488 in Ack 6
19:17:44.738 chunk 6 53060 bytes
19:17:44.739 in Ack 6
19:17:44.988 chunk 6 53138 bytes
19:17:44.989 in Ack 6
19:17:45.214 exit 26 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:17:45.215 rpc 27 orchestration.dispatchCommand thread.visit
19:17:45.216 exit 27 {"_tag":"Success","value":{"sequence":0}}
19:17:45.239 fetch POST /api/observability/v1/traces
19:17:45.239 fetched POST /api/observability/v1/traces 204
19:17:45.240 chunk 6 53178 bytes
19:17:45.241 in Ack 6
19:17:45.490 chunk 6 53208 bytes
19:17:45.491 in Ack 6
19:17:45.740 chunk 6 54119 bytes
19:17:45.741 in Ack 6
19:17:45.990 chunk 6 54169 bytes
19:17:45.991 in Ack 6
19:17:46.240 chunk 6 54179 bytes
19:17:46.241 in Ack 6
19:17:46.250 fetch POST /api/observability/v1/traces
19:17:46.250 fetched POST /api/observability/v1/traces 204
19:17:46.464 chunk 12 2894 bytes
19:17:46.464 in Ack 12
19:17:46.490 chunk 6 54209 bytes
19:17:46.491 in Ack 6
19:17:46.739 chunk 6 54229 bytes
19:17:46.740 in Ack 6
19:17:46.989 chunk 6 54249 bytes
19:17:46.991 in Ack 6
19:17:47.240 chunk 6 56403 bytes
19:17:47.241 in Ack 6
19:17:47.249 in Interrupt 11
19:17:47.273 rpc 28 subscribeVcsStatus
19:17:47.298 in Ping 
19:17:47.465 fetch POST /api/observability/v1/traces
19:17:47.465 fetched POST /api/observability/v1/traces 204
19:17:47.490 chunk 6 56443 bytes
19:17:47.491 in Ack 6
19:17:47.526 rpc 29 server.reportClientActivity
19:17:47.740 chunk 6 56453 bytes
19:17:47.741 in Ack 6
19:17:47.990 chunk 6 57869 bytes
19:17:47.991 in Ack 6
19:17:48.239 chunk 6 57899 bytes
19:17:48.241 in Ack 6
19:17:48.490 chunk 6 57909 bytes
19:17:48.492 in Ack 6
19:17:48.502 fetch POST /api/observability/v1/traces
19:17:48.502 fetched POST /api/observability/v1/traces 204
19:17:48.745 chunk 6 57941 bytes
19:17:48.747 in Ack 6
19:17:48.995 chunk 6 57957 bytes
19:17:48.997 in Ack 6
19:17:49.090 chunk 28 725 bytes
19:17:49.099 in Ack 28
19:17:49.170 exit 16 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/4fb8a32f-d5c9-43d0-adec-5a639bbaa0de","expiresAt":1794169069170}}
19:17:49.276 chunk 6 60027 bytes
19:17:49.278 in Ack 6
19:17:49.526 chunk 6 60047 bytes
19:17:49.528 in Ack 6
19:17:49.748 fetch POST /api/observability/v1/traces
19:17:49.748 fetched POST /api/observability/v1/traces 204
19:17:49.777 chunk 6 60067 bytes
19:17:49.778 in Ack 6
19:17:50.027 chunk 6 61073 bytes
19:17:50.028 in Ack 6
19:17:50.277 chunk 6 61093 bytes
19:17:50.278 in Ack 6
19:17:50.528 chunk 6 62177 bytes
19:17:50.529 in Ack 6
19:17:50.618 chunk 12 2894 bytes
19:17:50.618 in Ack 12
19:17:50.778 chunk 6 62192 bytes
19:17:50.779 fetch POST /api/observability/v1/traces
19:17:50.779 fetched POST /api/observability/v1/traces 204
19:17:50.781 in Ack 6
19:17:51.028 chunk 6 62243 bytes
19:17:51.029 in Ack 6
19:17:51.279 chunk 6 62273 bytes
19:17:51.280 in Ack 6
19:17:51.396 exit 29 {"_tag":"Success","value":null}
19:17:51.529 chunk 6 62354 bytes
19:17:51.530 in Ack 6
19:17:51.779 chunk 6 62414 bytes
19:17:51.781 in Ack 6
19:17:51.837 fetch POST /api/observability/v1/traces
19:17:51.837 fetched POST /api/observability/v1/traces 204
19:17:52.029 chunk 6 62459 bytes
19:17:52.031 in Ack 6
19:17:52.280 chunk 6 62498 bytes
19:17:52.281 in Ack 6
19:17:52.351 in Ping 
19:17:52.530 chunk 6 62543 bytes
19:17:52.531 in Ack 6
19:17:52.780 chunk 6 62597 bytes
19:17:52.781 in Ack 6
19:17:53.029 chunk 6 62630 bytes
19:17:53.030 in Ack 6
19:17:53.089 fetch POST /api/observability/v1/traces
19:17:53.089 fetched POST /api/observability/v1/traces 204
19:17:53.280 chunk 6 62669 bytes
19:17:53.281 in Ack 6
19:17:53.530 chunk 6 62714 bytes
19:17:53.531 in Ack 6
19:17:53.780 chunk 6 62744 bytes
19:17:53.781 in Ack 6
19:17:54.000 rpc 30 orchestration.dispatchCommand thread.visit
19:17:54.000 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:17:54.030 chunk 6 62795 bytes
19:17:54.031 in Ack 6
19:17:54.280 chunk 6 62849 bytes
19:17:54.281 in Ack 6
19:17:54.351 fetch POST /api/observability/v1/traces
19:17:54.351 fetched POST /api/observability/v1/traces 204
19:17:54.535 chunk 6 62882 bytes
19:17:54.536 in Ack 6
19:17:54.785 chunk 6 62897 bytes
19:17:54.787 in Ack 6
19:17:55.036 chunk 6 62995 bytes
19:17:55.037 in Ack 6
19:17:55.542 fetch POST /api/observability/v1/traces
19:17:55.542 fetched POST /api/observability/v1/traces 204
19:17:55.684 chunk 12 2830 bytes
19:17:55.685 in Ack 12
19:17:55.686 rpc 31 orchestration.dispatchCommand thread.visit
19:17:55.686 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:17:55.792 chunk 6 63127 bytes
19:17:55.794 in Ack 6
19:17:56.041 chunk 6 63151 bytes
19:17:56.042 in Ack 6
