# case50, webapp, warm start, seed 8

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.9s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.4s |
| Whole reply shown | 44.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.9s / 0.9s / 1.7s |
| Longest silence | 2.9s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 394353 / 167880 |
| Bytes the app received / sent | 312502 / 72920 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.7 | 307 | 373 |
| 20 | 0.8 | 742 | 774 |
| 30 | 0.7 | 1077 | 1109 |
| 40 | 0.9 | 1452 | 1483 |
| 44 | 1.3 | 1575 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":174,"lastDataAt":1794165922371,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":7298.207654103475,"deliveredRate":3787.823643067326,"loss":0,"srttMs":3.6848417173055608,"minRttMs":1,"sentPackets":95,"sentWireBytes":30721,"receivedPackets":127,"receivedWireBytes":68546,"lastHeardMs":8972,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"868/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 409/451","assistant":"32/32 21/21 455/455","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 626/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 570/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":174,"lastDataAt":1794165932335,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.2961988636363636,"srttMs":435.9625203826877,"minRttMs":651,"sentPackets":151,"sentWireBytes":41756,"receivedPackets":198,"receivedWireBytes":101526,"lastHeardMs":18936,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 451/451 545/545","assistant":"32/32 21/21 953/953","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":174,"lastDataAt":1794165942473,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":4500,"deliveredRate":2218.4210526315787,"loss":0.2142857142857143,"srttMs":725.7224957242221,"minRttMs":651,"sentPackets":215,"sentWireBytes":54653,"receivedPackets":434,"receivedWireBytes":171983,"lastHeardMs":29074,"unsyncedBytes":0,"inflightPackets":17,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 1098/1098 849/849 926/926 412/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 451/451 545/545","assistant":"32/32 21/21 1340/1340 53/53","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 973/1371","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"522/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":174,"lastDataAt":1794165952483,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.4376493506493506,"srttMs":746.5687857888574,"minRttMs":651,"sentPackets":270,"sentWireBytes":65230,"receivedPackets":779,"receivedWireBytes":256835,"lastHeardMs":39085,"unsyncedBytes":399,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 1098/1098 849/849 950/950 1570/1570 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 451/451 546/546 545/545","assistant":"32/32 21/21 1340/1340 599/599","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":174,"lastDataAt":1794165963535,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":10125,"deliveredRate":1645.1377291022875,"loss":0.4102948717948718,"srttMs":822.4569290887429,"minRttMs":651,"sentPackets":366,"sentWireBytes":92466,"receivedPackets":979,"receivedWireBytes":344987,"lastHeardMs":50137,"unsyncedBytes":0,"inflightPackets":48,"duplicateBytes":0}}
- shim trace
19:25:13.663 fetch GET /api/auth/session
19:25:13.664 fetched GET /api/auth/session 200
19:25:13.801 fetch GET /.well-known/t3/environment
19:25:13.802 fetched GET /.well-known/t3/environment 200
19:25:13.810 fetch GET /.well-known/t3/environment
19:25:13.810 fetched GET /.well-known/t3/environment 200
19:25:13.814 rpc 0 subscribeServerConfig
19:25:13.814 chunk 0 38722 bytes
19:25:13.836 in Ack 0
19:25:13.837 rpc 1 subscribeServerLifecycle
19:25:13.838 rpc 2 subscribeServerLifecycle
19:25:13.840 fetch GET /api/auth/session
19:25:13.841 fetched GET /api/auth/session 200
19:25:13.842 rpc 3 subscribeDeviceState
19:25:13.843 rpc 4 preview.reportProfiles
19:25:13.878 rpc 5 subscribeProjectClones
19:25:13.896 fetch GET /api/orchestration/threads/mcp%3A146a6bae-5501-4502-b6de-c54ed691454c/bounded
19:25:13.901 fetch GET /api/orchestration/shell
19:25:13.921 rpc 6 server.refreshProviders
19:25:13.957 rpc 7 orchestration.dispatchCommand thread.visit
19:25:13.957 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:25:13.967 fetched GET /api/orchestration/shell 200
19:25:13.967 rpc 8 subscribeTerminalMetadata
19:25:13.968 rpc 9 subscribeVcsStatus
19:25:13.969 rpc 10 subscribePreviewEvents
19:25:13.971 rpc 11 orchestration.subscribeShell
19:25:13.971 chunk 11 2790 bytes
19:25:13.972 rpc 12 assets.createUrl
19:25:13.973 rpc 13 preview.list
19:25:13.973 rpc 14 projects.readFile
19:25:13.975 in Ack 11
19:25:14.091 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"b20654c2-61f6-4856-a623-b6bd9a46157a","sessions":[]}}
19:25:14.091 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:25:14.209 chunk 1 3872 bytes
19:25:14.211 in Ack 1
19:25:14.220 rpc 15 server.reportClientActivity
19:25:14.569 chunk 2 3872 bytes
19:25:14.569 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:25:02.382Z","configurableProviders":false,"con
19:25:14.569 chunk 9 1324 bytes
19:25:14.569 in Ack 2
19:25:14.572 in Ack 9
19:25:14.669 chunk 11 2766 bytes
19:25:14.669 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:25:14.669 in Ack 11
19:25:14.800 fetch POST /api/observability/v1/traces
19:25:14.800 fetched POST /api/observability/v1/traces 204
19:25:14.868 chunk 3 876 bytes
19:25:14.868 exit 4 {"_tag":"Success","value":null}
19:25:14.868 chunk 5 4 bytes
19:25:14.868 chunk 8 36 bytes
19:25:14.868 exit 15 {"_tag":"Success","value":null}
19:25:14.869 in Ack 3
19:25:14.872 rpc 16 preview.reportProfiles
19:25:14.872 in Ack 5
19:25:14.872 in Ack 8
19:25:14.919 chunk 11 2809 bytes
19:25:14.922 in Ack 11
19:25:14.938 exit 16 {"_tag":"Success","value":null}
19:25:15.027 fetched GET /api/orchestration/threads/mcp%3A146a6bae-5501-4502-b6de-c54ed691454c/bounded 200
19:25:15.030 rpc 17 orchestration.subscribeThread
19:25:15.030 chunk 17 8283 bytes
19:25:15.040 in Interrupt 9
19:25:15.042 rpc 18 subscribeVcsStatus
19:25:15.042 in Ack 17
19:25:15.148 chunk 18 1324 bytes
19:25:15.148 in Ack 18
19:25:15.281 chunk 17 11920 bytes
19:25:15.282 in Ack 17
19:25:15.312 rpc 19 server.reportClientActivity
19:25:15.532 chunk 17 13502 bytes
19:25:15.533 in Ack 17
19:25:15.691 rpc 20 attachments.createUploadUrl
19:25:15.849 exit 19 {"_tag":"Success","value":null}
19:25:15.850 chunk 17 19851 bytes
19:25:15.866 in Ack 17
19:25:15.877 in Interrupt 18
19:25:15.879 rpc 21 subscribeVcsStatus
19:25:15.880 fetch POST /api/observability/v1/traces
19:25:15.880 fetched POST /api/observability/v1/traces 204
19:25:15.908 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-5d705b18-4035-47c7-a93e-bb987c2fa7e2","expiresAt":1794166515878,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:25:15.909 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTVkNzA1YjE4LTQwMzUtNDdjNy1hOTNlLWJiOTg3YzJmYTdlMiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjUxNTg3OH0.vMz1PQRRp5ZZjxSTkULseyr_HLOpXxVSy7OD3RVFJPs
19:25:15.977 chunk 21 1324 bytes
19:25:15.977 in Ack 21
19:25:16.100 chunk 17 21199 bytes
19:25:16.101 in Ack 17
19:25:16.131 rpc 22 server.reportClientActivity
19:25:16.909 fetch POST /api/observability/v1/traces
19:25:16.909 fetched POST /api/observability/v1/traces 204
19:25:17.548 exit 22 {"_tag":"Success","value":null}
19:25:18.549 fetch POST /api/observability/v1/traces
19:25:18.549 fetched POST /api/observability/v1/traces 204
19:25:18.805 rpc 23 orchestration.dispatchCommand message.dispatch
19:25:19.326 in Ping 
19:25:19.378 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:25:19.380 rpc 24 attachments.delete
19:25:19.642 chunk 11 2895 bytes
19:25:19.642 chunk 17 24270 bytes
19:25:19.643 in Ack 11
19:25:19.668 in Ack 17
19:25:19.757 exit 24 {"_tag":"Success","value":null}
19:25:19.804 fetch POST /api/observability/v1/traces
19:25:19.804 fetched POST /api/observability/v1/traces 204
19:25:19.893 chunk 17 29318 bytes
19:25:19.894 in Ack 17
19:25:19.897 rpc 25 assets.createUrl
19:25:20.143 chunk 17 29382 bytes
19:25:20.144 in Ack 17
19:25:20.393 chunk 11 2831 bytes
19:25:20.393 chunk 17 29538 bytes
19:25:20.393 in Ack 11
19:25:20.395 rpc 26 orchestration.dispatchCommand thread.visit
19:25:20.395 exit 26 {"_tag":"Success","value":{"sequence":0}}
19:25:20.400 in Ack 17
19:25:20.643 chunk 11 2831 bytes
19:25:20.643 chunk 17 33676 bytes
19:25:20.643 in Ack 11
19:25:20.648 in Ack 17
19:25:20.655 in Interrupt 21
19:25:20.657 rpc 27 subscribeVcsStatus
19:25:20.699 chunk 27 1324 bytes
19:25:20.699 in Ack 27
19:25:20.894 chunk 17 33508 bytes
19:25:20.895 in Ack 17
19:25:20.902 fetch POST /api/observability/v1/traces
19:25:20.902 fetched POST /api/observability/v1/traces 204
19:25:20.907 rpc 28 server.reportClientActivity
19:25:20.958 exit 28 {"_tag":"Success","value":null}
19:25:21.144 chunk 17 35214 bytes
19:25:21.145 in Ack 17
19:25:21.615 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/4a879b4a-001a-41ae-b693-45aff9ab93b7","expiresAt":1794169521615}}
19:25:21.909 fetch POST /api/observability/v1/traces
19:25:21.909 fetched POST /api/observability/v1/traces 204
19:25:22.355 rpc 29 orchestration.dispatchCommand message.dispatch
19:25:23.057 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:25:23.355 fetch POST /api/observability/v1/traces
19:25:23.355 fetched POST /api/observability/v1/traces 204
19:25:23.557 chunk 17 37658 bytes
19:25:23.559 in Ack 17
19:25:23.809 chunk 17 37668 bytes
19:25:23.810 in Ack 17
19:25:24.059 chunk 17 37690 bytes
19:25:24.060 in Ack 17
19:25:24.309 chunk 17 37712 bytes
19:25:24.310 in Ack 17
19:25:24.329 in Ping 
19:25:24.560 chunk 17 37732 bytes
19:25:24.560 fetch POST /api/observability/v1/traces
19:25:24.560 fetched POST /api/observability/v1/traces 204
19:25:24.561 in Ack 17
19:25:24.810 chunk 17 37770 bytes
19:25:24.811 in Ack 17
19:25:25.208 chunk 17 37744 bytes
19:25:25.209 in Ack 17
19:25:25.459 chunk 17 37782 bytes
19:25:25.460 in Ack 17
19:25:25.562 fetch POST /api/observability/v1/traces
19:25:25.562 fetched POST /api/observability/v1/traces 204
19:25:25.709 chunk 17 39807 bytes
19:25:25.711 in Ack 17
19:25:25.960 chunk 17 39835 bytes
19:25:25.961 in Ack 17
19:25:26.210 chunk 17 39845 bytes
19:25:26.211 in Ack 17
19:25:26.460 chunk 17 40530 bytes
19:25:26.461 in Ack 17
19:25:26.710 chunk 17 40552 bytes
19:25:26.711 in Ack 17
19:25:26.720 fetch POST /api/observability/v1/traces
19:25:26.720 fetched POST /api/observability/v1/traces 204
19:25:27.057 chunk 17 40574 bytes
19:25:27.058 in Ack 17
19:25:27.307 chunk 11 2895 bytes
19:25:27.307 chunk 17 40604 bytes
19:25:27.308 in Ack 11
19:25:27.350 in Ack 17
19:25:27.558 chunk 17 40652 bytes
19:25:27.559 in Ack 17
19:25:27.808 chunk 17 40674 bytes
19:25:27.810 in Ack 17
19:25:28.060 fetch POST /api/observability/v1/traces
19:25:28.060 fetched POST /api/observability/v1/traces 204
19:25:28.125 chunk 17 40712 bytes
19:25:28.126 in Ack 17
19:25:28.684 chunk 17 40744 bytes
19:25:28.686 in Ack 17
19:25:28.935 chunk 17 40764 bytes
19:25:28.936 in Ack 17
19:25:29.127 fetch POST /api/observability/v1/traces
19:25:29.127 fetched POST /api/observability/v1/traces 204
19:25:29.185 chunk 17 41644 bytes
19:25:29.186 in Ack 17
19:25:29.329 in Ping 
19:25:29.530 chunk 17 41656 bytes
19:25:29.531 in Ack 17
19:25:29.780 chunk 17 41668 bytes
19:25:29.781 in Ack 17
19:25:30.030 chunk 17 41732 bytes
19:25:30.031 in Ack 17
19:25:30.187 fetch POST /api/observability/v1/traces
19:25:30.187 fetched POST /api/observability/v1/traces 204
19:25:30.280 chunk 17 41752 bytes
19:25:30.281 in Ack 17
19:25:30.397 rpc 30 orchestration.dispatchCommand thread.visit
19:25:30.397 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:25:30.531 chunk 17 41792 bytes
19:25:30.532 in Ack 17
19:25:30.781 chunk 17 41812 bytes
19:25:30.782 in Ack 17
19:25:31.031 chunk 17 41832 bytes
19:25:31.032 in Ack 17
19:25:31.281 chunk 17 41844 bytes
19:25:31.282 in Ack 17
19:25:31.306 fetch POST /api/observability/v1/traces
19:25:31.306 fetched POST /api/observability/v1/traces 204
19:25:31.531 chunk 17 41856 bytes
19:25:31.532 in Ack 17
19:25:31.781 chunk 17 41878 bytes
19:25:31.783 in Ack 17
19:25:32.032 chunk 17 41888 bytes
19:25:32.033 in Ack 17
19:25:32.284 chunk 17 41962 bytes
19:25:32.285 in Ack 17
19:25:32.531 chunk 17 41992 bytes
19:25:32.532 in Ack 17
19:25:32.541 fetch POST /api/observability/v1/traces
19:25:32.541 fetched POST /api/observability/v1/traces 204
19:25:32.781 chunk 17 42002 bytes
19:25:32.782 in Ack 17
19:25:33.040 chunk 17 42032 bytes
19:25:33.043 in Ack 17
19:25:33.290 chunk 11 2895 bytes
19:25:33.290 chunk 17 42709 bytes
19:25:33.290 in Ack 11
19:25:33.298 in Ack 17
19:25:33.540 chunk 17 42731 bytes
19:25:33.541 in Ack 17
19:25:33.784 fetch POST /api/observability/v1/traces
19:25:33.784 fetched POST /api/observability/v1/traces 204
19:25:33.790 chunk 17 42753 bytes
19:25:33.791 in Ack 17
19:25:34.040 chunk 17 42773 bytes
19:25:34.041 in Ack 17
19:25:34.290 chunk 17 42809 bytes
19:25:34.290 in Ack 17
19:25:34.329 in Ping 
19:25:34.540 chunk 17 42839 bytes
19:25:34.541 in Ack 17
19:25:34.790 chunk 11 2895 bytes
19:25:34.790 chunk 17 42869 bytes
19:25:34.791 in Ack 11
19:25:34.793 in Ack 17
19:25:34.802 fetch POST /api/observability/v1/traces
19:25:34.802 fetched POST /api/observability/v1/traces 204
19:25:35.041 chunk 17 42889 bytes
19:25:35.042 in Ack 17
19:25:35.291 chunk 17 42909 bytes
19:25:35.293 in Ack 17
19:25:35.542 chunk 17 44105 bytes
19:25:35.543 in Ack 17
19:25:35.793 chunk 17 44120 bytes
19:25:35.794 in Ack 17
19:25:36.043 fetch POST /api/observability/v1/traces
19:25:36.043 fetched POST /api/observability/v1/traces 204
19:25:36.043 chunk 17 44165 bytes
19:25:36.044 in Ack 17
19:25:36.294 chunk 17 44210 bytes
19:25:36.295 in Ack 17
19:25:36.544 chunk 17 45527 bytes
19:25:36.545 in Ack 17
19:25:36.794 chunk 17 45587 bytes
19:25:36.795 in Ack 17
19:25:37.044 chunk 17 45602 bytes
19:25:37.045 in Ack 17
19:25:37.054 fetch POST /api/observability/v1/traces
19:25:37.054 fetched POST /api/observability/v1/traces 204
19:25:37.293 chunk 17 45662 bytes
19:25:37.295 in Ack 17
19:25:37.544 chunk 17 45692 bytes
19:25:37.545 in Ack 17
19:25:37.794 chunk 17 45722 bytes
19:25:37.795 in Ack 17
19:25:38.045 chunk 17 45767 bytes
19:25:38.046 in Ack 17
19:25:38.295 chunk 17 45797 bytes
19:25:38.296 in Ack 17
19:25:38.315 fetch POST /api/observability/v1/traces
19:25:38.315 fetched POST /api/observability/v1/traces 204
19:25:38.545 chunk 17 45827 bytes
19:25:38.546 in Ack 17
19:25:38.795 chunk 17 45872 bytes
19:25:38.796 in Ack 17
19:25:39.045 chunk 17 45917 bytes
19:25:39.046 in Ack 17
19:25:39.080 rpc 31 server.reportClientActivity
19:25:39.295 chunk 17 45947 bytes
19:25:39.296 in Ack 17
19:25:39.330 in Ping 
19:25:39.544 chunk 17 45992 bytes
19:25:39.545 in Ack 17
19:25:39.553 fetch POST /api/observability/v1/traces
19:25:39.553 fetched POST /api/observability/v1/traces 204
19:25:39.795 chunk 17 46007 bytes
19:25:39.796 in Ack 17
19:25:40.045 chunk 17 46067 bytes
19:25:40.046 in Ack 17
19:25:40.295 chunk 17 46112 bytes
19:25:40.296 in Ack 17
19:25:40.397 rpc 32 orchestration.dispatchCommand thread.visit
19:25:40.397 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:25:40.545 chunk 17 46157 bytes
19:25:40.546 in Ack 17
19:25:40.757 exit 31 {"_tag":"Success","value":null}
19:25:40.795 chunk 17 46172 bytes
19:25:40.796 in Ack 17
19:25:40.805 fetch POST /api/observability/v1/traces
19:25:40.805 fetched POST /api/observability/v1/traces 204
19:25:41.046 chunk 17 46217 bytes
19:25:41.047 in Ack 17
19:25:41.296 chunk 17 46247 bytes
19:25:41.297 in Ack 17
19:25:41.546 chunk 17 46262 bytes
19:25:41.546 in Ack 17
19:25:41.796 chunk 11 2895 bytes
19:25:41.796 chunk 17 46322 bytes
19:25:41.796 in Ack 11
19:25:41.813 in Ack 17
19:25:42.046 chunk 17 46352 bytes
19:25:42.047 in Ack 17
19:25:42.056 fetch POST /api/observability/v1/traces
19:25:42.056 fetched POST /api/observability/v1/traces 204
19:25:42.295 chunk 17 46397 bytes
19:25:42.296 in Ack 17
19:25:42.546 chunk 17 46442 bytes
19:25:42.547 in Ack 17
19:25:42.796 chunk 17 46487 bytes
19:25:42.798 in Ack 17
19:25:43.047 chunk 17 46502 bytes
19:25:43.048 in Ack 17
19:25:43.298 fetch POST /api/observability/v1/traces
19:25:43.298 fetched POST /api/observability/v1/traces 204
19:25:43.298 chunk 17 46547 bytes
19:25:43.299 in Ack 17
19:25:43.549 chunk 17 46592 bytes
19:25:43.550 in Ack 17
19:25:43.799 chunk 17 46637 bytes
19:25:43.800 in Ack 17
19:25:44.049 chunk 17 46652 bytes
19:25:44.050 in Ack 17
19:25:44.299 chunk 17 46697 bytes
19:25:44.300 in Ack 17
19:25:44.308 fetch POST /api/observability/v1/traces
19:25:44.308 fetched POST /api/observability/v1/traces 204
19:25:44.330 in Ping 
19:25:44.550 chunk 17 46742 bytes
19:25:44.551 in Ack 17
19:25:44.800 chunk 17 46772 bytes
19:25:44.801 in Ack 17
19:25:45.050 chunk 17 46817 bytes
19:25:45.051 in Ack 17
19:25:45.300 chunk 17 46847 bytes
19:25:45.301 in Ack 17
19:25:45.486 fetch POST /api/observability/v1/traces
19:25:45.486 fetched POST /api/observability/v1/traces 204
19:25:45.550 chunk 17 46877 bytes
19:25:45.551 in Ack 17
19:25:45.800 chunk 17 46922 bytes
19:25:45.801 in Ack 17
19:25:46.051 chunk 17 46952 bytes
19:25:46.052 in Ack 17
19:25:46.301 chunk 17 46997 bytes
19:25:46.302 in Ack 17
19:25:46.551 chunk 17 47027 bytes
19:25:46.552 in Ack 17
19:25:46.560 fetch POST /api/observability/v1/traces
19:25:46.560 fetched POST /api/observability/v1/traces 204
19:25:46.802 chunk 17 47057 bytes
19:25:46.803 in Ack 17
19:25:47.052 chunk 17 47087 bytes
19:25:47.053 in Ack 17
19:25:47.303 chunk 17 47117 bytes
19:25:47.303 in Ack 17
19:25:47.553 chunk 17 47162 bytes
19:25:47.554 in Ack 17
19:25:47.803 chunk 17 47192 bytes
19:25:47.804 in Ack 17
19:25:47.825 fetch POST /api/observability/v1/traces
19:25:47.825 fetched POST /api/observability/v1/traces 204
19:25:48.054 chunk 17 47222 bytes
19:25:48.055 in Ack 17
19:25:48.304 chunk 17 47267 bytes
19:25:48.305 in Ack 17
19:25:48.555 chunk 17 47297 bytes
19:25:48.556 in Ack 17
19:25:48.805 chunk 17 47342 bytes
19:25:48.806 in Ack 17
19:25:49.057 chunk 17 47387 bytes
19:25:49.057 fetch POST /api/observability/v1/traces
19:25:49.057 fetched POST /api/observability/v1/traces 204
19:25:49.059 in Ack 17
19:25:49.308 chunk 17 47417 bytes
19:25:49.309 in Ack 17
19:25:49.330 in Ping 
19:25:49.557 chunk 17 47462 bytes
19:25:49.558 in Ack 17
19:25:49.808 chunk 17 47507 bytes
19:25:49.809 in Ack 17
19:25:50.065 chunk 17 47537 bytes
19:25:50.066 fetch POST /api/observability/v1/traces
19:25:50.066 fetched POST /api/observability/v1/traces 204
19:25:50.067 in Ack 17
19:25:50.316 chunk 17 47567 bytes
19:25:50.317 in Ack 17
19:25:50.567 chunk 17 47609 bytes
19:25:50.568 in Ack 17
19:25:50.817 chunk 17 48614 bytes
19:25:50.818 in Ack 17
19:25:51.076 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:25:51.112 fetch POST /api/observability/v1/traces
19:25:51.112 fetched POST /api/observability/v1/traces 204
19:25:51.364 chunk 17 50689 bytes
19:25:51.366 in Ack 17
19:25:51.616 chunk 17 50739 bytes
19:25:51.631 in Ack 17
19:25:52.035 exit 33 {"_tag":"Success","value":{"sequence":97}}
19:25:52.116 chunk 17 53210 bytes
19:25:52.117 in Ack 17
19:25:52.363 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:25:52.371 chunk 17 53240 bytes
19:25:52.372 fetch POST /api/observability/v1/traces
19:25:52.372 fetched POST /api/observability/v1/traces 204
19:25:52.391 in Ack 17
19:25:52.621 chunk 17 53270 bytes
19:25:52.622 in Ack 17
19:25:52.871 chunk 17 53290 bytes
19:25:52.872 in Ack 17
19:25:53.123 chunk 17 53272 bytes
19:25:53.124 in Ack 17
19:25:53.392 fetch POST /api/observability/v1/traces
19:25:53.392 fetched POST /api/observability/v1/traces 204
19:25:53.532 chunk 17 53284 bytes
19:25:53.533 in Ack 17
19:25:53.782 chunk 17 54366 bytes
19:25:53.783 in Ack 17
19:25:53.791 in Interrupt 27
19:25:53.815 rpc 35 subscribeVcsStatus
19:25:54.046 chunk 17 54430 bytes
19:25:54.047 in Ack 17
19:25:54.079 rpc 36 server.reportClientActivity
19:25:54.123 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:25:54.407 chunk 17 55500 bytes
19:25:54.408 in Ping 
19:25:54.409 in Ack 17
19:25:54.534 fetch POST /api/observability/v1/traces
19:25:54.534 fetched POST /api/observability/v1/traces 204
19:25:54.658 chunk 17 55520 bytes
19:25:54.659 in Ack 17
19:25:54.849 chunk 35 1324 bytes
19:25:54.850 in Ack 35
19:25:54.908 chunk 17 55540 bytes
19:25:54.909 in Ack 17
19:25:55.158 chunk 17 55560 bytes
19:25:55.159 in Ack 17
19:25:55.409 chunk 17 56986 bytes
19:25:55.410 in Ack 17
19:25:55.659 chunk 17 57006 bytes
19:25:55.660 fetch POST /api/observability/v1/traces
19:25:55.660 fetched POST /api/observability/v1/traces 204
19:25:55.661 in Ack 17
19:25:55.909 chunk 11 2897 bytes
19:25:55.909 in Ack 11
19:25:55.911 rpc 37 orchestration.dispatchCommand thread.visit
19:25:55.911 exit 37 {"_tag":"Success","value":{"sequence":0}}
19:25:55.912 chunk 17 60571 bytes
19:25:55.913 in Ack 17
19:25:55.981 exit 36 {"_tag":"Success","value":null}
19:25:56.162 chunk 17 60621 bytes
19:25:56.163 in Ack 17
19:25:56.412 chunk 17 60641 bytes
19:25:56.413 in Ack 17
19:25:56.662 chunk 17 60681 bytes
19:25:56.662 fetch POST /api/observability/v1/traces
19:25:56.662 fetched POST /api/observability/v1/traces 204
19:25:56.663 in Ack 17
19:25:56.912 chunk 17 60681 bytes
19:25:56.913 in Ack 17
19:25:57.163 chunk 17 60711 bytes
19:25:57.164 in Ack 17
19:25:57.409 chunk 11 2897 bytes
19:25:57.409 in Ack 11
19:25:57.416 chunk 17 60723 bytes
19:25:57.417 in Ack 17
19:25:57.664 fetch POST /api/observability/v1/traces
19:25:57.664 fetched POST /api/observability/v1/traces 204
19:25:57.677 chunk 17 60771 bytes
19:25:57.678 in Ack 17
19:25:57.927 chunk 17 60801 bytes
19:25:57.928 in Ack 17
19:25:58.177 chunk 17 60831 bytes
19:25:58.178 in Ack 17
19:25:58.427 chunk 17 60851 bytes
19:25:58.428 in Ack 17
19:25:58.677 chunk 17 60881 bytes
19:25:58.678 in Ack 17
19:25:58.709 fetch POST /api/observability/v1/traces
19:25:58.709 fetched POST /api/observability/v1/traces 204
19:25:58.927 chunk 17 60901 bytes
19:25:58.928 in Ack 17
19:25:59.178 chunk 17 60921 bytes
19:25:59.179 in Ack 17
19:25:59.408 in Ping 
19:25:59.428 chunk 17 60951 bytes
19:25:59.429 in Ack 17
19:25:59.678 chunk 17 60973 bytes
19:25:59.679 in Ack 17
19:25:59.929 chunk 17 60989 bytes
19:25:59.929 fetch POST /api/observability/v1/traces
19:25:59.929 fetched POST /api/observability/v1/traces 204
19:25:59.931 in Ack 17
19:26:00.179 chunk 17 61009 bytes
19:26:00.180 in Ack 17
19:26:00.429 chunk 17 61039 bytes
19:26:00.431 in Ack 17
19:26:00.679 chunk 17 61049 bytes
19:26:00.680 in Ack 17
19:26:00.929 chunk 17 61069 bytes
19:26:00.931 in Ack 17
19:26:00.987 fetch POST /api/observability/v1/traces
19:26:00.987 fetched POST /api/observability/v1/traces 204
19:26:01.179 chunk 11 2897 bytes
19:26:01.179 chunk 17 61089 bytes
19:26:01.180 in Ack 11
19:26:01.183 in Ack 17
19:26:01.430 chunk 17 62168 bytes
19:26:01.431 in Ack 17
19:26:01.680 chunk 17 62213 bytes
19:26:01.681 in Ack 17
19:26:01.930 chunk 17 62264 bytes
19:26:01.931 in Ack 17
19:26:02.180 chunk 17 62294 bytes
19:26:02.181 in Ack 17
19:26:02.224 fetch POST /api/observability/v1/traces
19:26:02.224 fetched POST /api/observability/v1/traces 204
19:26:02.430 chunk 17 62345 bytes
19:26:02.431 in Ack 17
19:26:02.680 chunk 17 62375 bytes
19:26:02.681 in Ack 17
19:26:02.930 chunk 17 62435 bytes
19:26:02.931 in Ack 17
19:26:03.180 chunk 17 62465 bytes
19:26:03.182 in Ack 17
19:26:03.430 chunk 17 62519 bytes
19:26:03.431 in Ack 17
19:26:03.534 fetch POST /api/observability/v1/traces
19:26:03.534 fetched POST /api/observability/v1/traces 204
19:26:03.680 chunk 17 62549 bytes
19:26:03.682 in Ack 17
19:26:03.930 chunk 17 62579 bytes
19:26:03.932 in Ack 17
19:26:04.059 rpc 38 server.reportClientActivity
19:26:04.181 chunk 17 62636 bytes
19:26:04.183 in Ack 17
19:26:04.415 in Ping 
19:26:04.431 chunk 17 62675 bytes
19:26:04.433 in Ack 17
19:26:04.682 fetch POST /api/observability/v1/traces
19:26:04.682 fetched POST /api/observability/v1/traces 204
19:26:04.683 chunk 17 62705 bytes
19:26:04.684 in Ack 17
19:26:04.933 chunk 17 62735 bytes
19:26:04.934 in Ack 17
19:26:05.183 chunk 17 62771 bytes
19:26:05.184 in Ack 17
19:26:05.433 chunk 17 62801 bytes
19:26:05.435 in Ack 17
19:26:05.683 chunk 17 62855 bytes
19:26:05.684 in Ack 17
19:26:05.774 fetch POST /api/observability/v1/traces
19:26:05.774 fetched POST /api/observability/v1/traces 204
19:26:05.911 rpc 39 orchestration.dispatchCommand thread.visit
19:26:05.911 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:26:05.933 chunk 17 62888 bytes
19:26:05.934 in Ack 17
19:26:06.183 chunk 17 62918 bytes
19:26:06.185 in Ack 17
19:26:06.434 chunk 17 63133 bytes
19:26:06.435 in Ack 17
19:26:06.912 fetch POST /api/observability/v1/traces
19:26:06.912 fetched POST /api/observability/v1/traces 204
19:26:06.933 chunk 11 2833 bytes
19:26:06.934 in Ack 11
19:26:06.935 rpc 40 orchestration.dispatchCommand thread.visit
19:26:06.935 exit 40 {"_tag":"Success","value":{"sequence":0}}
19:26:06.964 exit 38 {"_tag":"Success","value":null}
