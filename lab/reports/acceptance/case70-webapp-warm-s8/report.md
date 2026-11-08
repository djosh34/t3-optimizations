# case70, webapp, warm start, seed 8

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 70 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.4s |
| Prompt reached the agent | 3.0s |
| Time to first text | 8.1s |
| Whole reply shown | 69.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.7s / 9.5s / 20.6s |
| Longest silence | 17.1s |
| Approval shown after it was asked | 14.0s |
| Bytes on the wire down / up | 68571 / 47075 |
| Bytes the app received / sent | 56192 / 108847 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.5 | 17 | 279 |
| 20 | 15.8 | 46 | 677 |
| 30 | 20.6 | 255 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 3.3 | 1148 | 1277 |
| 60 | 13.0 | 1158 | 1596 |
| 69 | 16.7 | 1367 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2696/2696","messages":"456/456 546/546 209/901","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165241149,"silences":0,"longestSilenceMs":1416,"longestRecoveredGapMs":1416},"stats":{"sendRate":6978.379702719909,"deliveredRate":3244.6991387046896,"loss":0,"srttMs":4.006997856163004,"minRttMs":1,"sentPackets":120,"sentWireBytes":35709,"receivedPackets":114,"receivedWireBytes":53091,"lastHeardMs":9668,"unsyncedBytes":1186,"inflightPackets":20,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 89/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 55/110","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 47/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 375/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165251821,"silences":0,"longestSilenceMs":1416,"longestRecoveredGapMs":1416},"stats":{"sendRate":2063.220619945425,"deliveredRate":2085.8755914585513,"loss":0.2417647058823529,"srttMs":3740.237136838864,"minRttMs":1279,"sentPackets":144,"sentWireBytes":46371,"receivedPackets":132,"receivedWireBytes":63987,"lastHeardMs":20339,"unsyncedBytes":0,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 910/1073 712/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 315/548","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 81/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 429/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165262006,"silences":0,"longestSilenceMs":1416,"longestRecoveredGapMs":1416},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.2783262032085561,"srttMs":4939.759682892255,"minRttMs":1279,"sentPackets":194,"sentWireBytes":57930,"receivedPackets":157,"receivedWireBytes":75000,"lastHeardMs":30524,"unsyncedBytes":500,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 990/1073 712/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 48/839","assistant":"32/32 21/21 1086/1113","user":"10/10 9/9","attempts":"660/660 660/660 47/636","providerTurns":"774/774 81/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 429/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165271932,"silences":0,"longestSilenceMs":1416,"longestRecoveredGapMs":1416},"stats":{"sendRate":2522.6964342372885,"deliveredRate":485.40865809257195,"loss":0.7113751075011943,"srttMs":5300.401952312451,"minRttMs":1279,"sentPackets":248,"sentWireBytes":70515,"receivedPackets":191,"receivedWireBytes":85527,"lastHeardMs":40450,"unsyncedBytes":0,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1049/1073 712/849 170/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 93/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660 47/636","providerTurns":"774/774 81/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 429/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"752/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165280450,"silences":0,"longestSilenceMs":1416,"longestRecoveredGapMs":1416},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.6605393999968545,"srttMs":5377.726708273394,"minRttMs":1279,"sentPackets":300,"sentWireBytes":78785,"receivedPackets":210,"receivedWireBytes":93444,"lastHeardMs":48968,"unsyncedBytes":500,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1049/1073 848/849 1226/1546 48/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 93/839","assistant":"32/32 21/21 1340/1340 218/233","user":"10/10 9/9","attempts":"660/660 660/660 47/636","providerTurns":"774/774 81/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 429/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"739/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165292965,"silences":0,"longestSilenceMs":1879,"longestRecoveredGapMs":1879},"stats":{"sendRate":3315.9051210696693,"deliveredRate":764.017136563806,"loss":0.6299416666666666,"srttMs":2770.467573539303,"minRttMs":1032,"sentPackets":393,"sentWireBytes":110726,"receivedPackets":237,"receivedWireBytes":103961,"lastHeardMs":61484,"unsyncedBytes":399,"inflightPackets":141,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1049/1073 848/849 1226/1546 48/926 300/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 93/839","assistant":"32/32 21/21 1340/1340 497/723","user":"10/10 9/9","attempts":"660/660 660/660 47/636","providerTurns":"774/774 81/774","providerThreads":"396/808","providerSessions":"2696/2696","messages":"456/456 546/546 429/901 546/546","nodes":"1019/1019 809/809 809/809 935/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":288,"lastDataAt":1794165304418,"silences":3,"longestSilenceMs":4845,"longestRecoveredGapMs":4845},"stats":{"sendRate":3315.9051210696693,"deliveredRate":570.1591426048731,"loss":0.7077080516734509,"srttMs":2550.4673609910287,"minRttMs":1032,"sentPackets":441,"sentWireBytes":126588,"receivedPackets":240,"receivedWireBytes":105718,"lastHeardMs":72936,"unsyncedBytes":0,"inflightPackets":149,"duplicateBytes":0}}
- shim trace
19:13:51.761 fetch GET /api/auth/session
19:13:51.762 fetched GET /api/auth/session 200
19:13:51.861 fetch GET /.well-known/t3/environment
19:13:51.862 fetched GET /.well-known/t3/environment 200
19:13:51.870 fetch GET /.well-known/t3/environment
19:13:51.870 fetched GET /.well-known/t3/environment 200
19:13:51.874 rpc 0 subscribeServerConfig
19:13:51.874 chunk 0 38722 bytes
19:13:51.900 in Ack 0
19:13:51.901 rpc 1 subscribeServerLifecycle
19:13:51.902 rpc 2 subscribeServerLifecycle
19:13:51.904 fetch GET /api/auth/session
19:13:51.905 fetched GET /api/auth/session 200
19:13:51.905 rpc 3 subscribeDeviceState
19:13:51.907 rpc 4 preview.reportProfiles
19:13:51.928 rpc 5 subscribeProjectClones
19:13:51.944 fetch GET /api/orchestration/threads/mcp%3A35afd645-02e6-4df6-abea-878d4b973c79/bounded
19:13:51.948 fetch GET /api/orchestration/shell
19:13:51.948 fetched GET /api/orchestration/shell 200
19:13:51.950 rpc 6 orchestration.subscribeShell
19:13:51.950 chunk 6 2790 bytes
19:13:51.969 rpc 7 server.refreshProviders
19:13:52.006 rpc 8 orchestration.dispatchCommand thread.visit
19:13:52.006 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:13:52.020 rpc 9 subscribeTerminalMetadata
19:13:52.021 rpc 10 subscribeVcsStatus
19:13:52.022 rpc 11 subscribePreviewEvents
19:13:52.023 rpc 12 assets.createUrl
19:13:52.023 rpc 13 preview.list
19:13:52.024 rpc 14 projects.readFile
19:13:52.024 in Ack 6
19:13:52.274 rpc 15 server.reportClientActivity
19:13:52.802 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"9782e381-9b60-496a-9b74-67010e60a34a","sessions":[]}}
19:13:52.802 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:13:52.860 fetch POST /api/observability/v1/traces
19:13:52.860 fetched POST /api/observability/v1/traces 204
19:13:53.032 chunk 1 3872 bytes
19:13:53.033 in Ack 1
19:13:53.240 chunk 2 3872 bytes
19:13:53.240 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:13:40.153Z","configurableProviders":false,"con
19:13:53.240 in Ack 2
19:13:53.419 chunk 6 2766 bytes
19:13:53.419 chunk 10 556 bytes
19:13:53.419 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:13:53.419 in Ack 6
19:13:53.424 in Ack 10
19:13:53.669 chunk 6 2809 bytes
19:13:53.669 in Ack 6
19:13:53.818 chunk 3 876 bytes
19:13:53.818 exit 4 {"_tag":"Success","value":null}
19:13:53.818 chunk 5 4 bytes
19:13:53.818 chunk 9 36 bytes
19:13:53.818 exit 15 {"_tag":"Success","value":null}
19:13:53.819 in Ack 3
19:13:53.822 rpc 16 preview.reportProfiles
19:13:53.823 in Ack 5
19:13:53.824 in Ack 9
19:13:53.888 exit 16 {"_tag":"Success","value":null}
19:13:54.238 fetched GET /api/orchestration/threads/mcp%3A35afd645-02e6-4df6-abea-878d4b973c79/bounded 200
19:13:54.241 rpc 17 orchestration.subscribeThread
19:13:54.241 chunk 17 8283 bytes
19:13:54.251 in Interrupt 10
19:13:54.252 rpc 18 subscribeVcsStatus
19:13:54.253 fetch POST /api/observability/v1/traces
19:13:54.253 fetched POST /api/observability/v1/traces 204
19:13:54.254 in Ack 17
19:13:54.388 chunk 18 556 bytes
19:13:54.388 in Ack 18
19:13:54.492 chunk 17 11920 bytes
19:13:54.492 in Ack 17
19:13:54.523 rpc 19 server.reportClientActivity
19:13:54.729 exit 19 {"_tag":"Success","value":null}
19:13:54.745 chunk 17 13502 bytes
19:13:54.746 in Ack 17
19:13:55.190 rpc 20 attachments.createUploadUrl
19:13:55.352 chunk 17 18027 bytes
19:13:55.352 fetch POST /api/observability/v1/traces
19:13:55.352 fetched POST /api/observability/v1/traces 204
19:13:55.358 in Ack 17
19:13:55.447 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-f11444f6-27fd-4c4d-92ba-6576dd298ec0","expiresAt":1794165835368,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:13:55.448 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWYxMTQ0NGY2LTI3ZmQtNGM0ZC05MmJhLTY1NzZkZDI5OGVjMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NTgzNTM2OH0.eKoJbWLNDtfIFbL-5uym3gRF97bDfhPRb8kkHjxWYo0
19:13:55.602 chunk 17 19851 bytes
19:13:55.603 in Ack 17
19:13:55.608 in Interrupt 18
19:13:55.610 rpc 21 subscribeVcsStatus
19:13:55.701 chunk 21 556 bytes
19:13:55.701 in Ack 21
19:13:55.852 chunk 17 21199 bytes
19:13:55.853 in Ack 17
19:13:55.861 rpc 22 server.reportClientActivity
19:13:56.360 fetch POST /api/observability/v1/traces
19:13:56.360 fetched POST /api/observability/v1/traces 204
19:13:56.872 in Ping 
19:13:57.267 exit 22 {"_tag":"Success","value":null}
19:13:58.320 rpc 23 orchestration.dispatchCommand message.dispatch
19:13:58.853 fetch POST /api/observability/v1/traces
19:13:58.854 fetched POST /api/observability/v1/traces 204
19:13:58.939 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:13:58.941 rpc 24 attachments.delete
19:13:58.997 exit 24 {"_tag":"Success","value":null}
19:13:59.189 chunk 17 23633 bytes
19:13:59.189 in Ack 17
19:13:59.439 chunk 17 23595 bytes
19:13:59.440 in Ack 17
19:13:59.700 chunk 6 2895 bytes
19:13:59.701 chunk 17 23617 bytes
19:13:59.703 in Ack 6
19:13:59.735 in Ack 17
19:13:59.940 fetch POST /api/observability/v1/traces
19:13:59.940 fetched POST /api/observability/v1/traces 204
19:13:59.951 chunk 17 23667 bytes
19:13:59.951 in Ack 17
19:14:00.201 chunk 6 2831 bytes
19:14:00.201 chunk 17 24639 bytes
19:14:00.201 in Ack 6
19:14:00.202 rpc 25 orchestration.dispatchCommand thread.visit
19:14:00.202 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:14:00.207 in Ack 17
19:14:00.451 chunk 17 25300 bytes
19:14:00.452 in Ack 17
19:14:00.701 chunk 6 2831 bytes
19:14:00.701 in Ack 6
19:14:00.707 chunk 17 31326 bytes
19:14:00.709 in Ack 17
19:14:00.711 rpc 26 assets.createUrl
19:14:01.150 fetch POST /api/observability/v1/traces
19:14:01.150 fetched POST /api/observability/v1/traces 204
19:14:01.151 chunk 17 31940 bytes
19:14:01.152 in Ack 17
19:14:01.160 in Interrupt 21
19:14:01.162 rpc 27 subscribeVcsStatus
19:14:01.413 rpc 28 server.reportClientActivity
19:14:01.874 rpc 29 orchestration.dispatchCommand message.dispatch
19:14:01.906 in Ping 
19:14:02.153 fetch POST /api/observability/v1/traces
19:14:02.153 fetched POST /api/observability/v1/traces 204
19:14:03.141 chunk 27 556 bytes
19:14:03.141 in Ack 27
19:14:03.456 fetch POST /api/observability/v1/traces
19:14:03.456 fetched POST /api/observability/v1/traces 204
19:14:05.756 fetch POST /api/observability/v1/traces
19:14:05.756 fetched POST /api/observability/v1/traces 204
19:14:06.906 in Ping 
19:14:08.151 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:14:08.662 chunk 17 34386 bytes
19:14:08.664 in Ack 17
19:14:09.152 fetch POST /api/observability/v1/traces
19:14:09.152 fetched POST /api/observability/v1/traces 204
19:14:09.183 chunk 17 34396 bytes
19:14:09.184 in Ack 17
19:14:09.707 chunk 17 34416 bytes
19:14:09.708 in Ack 17
19:14:10.184 fetch POST /api/observability/v1/traces
19:14:10.184 fetched POST /api/observability/v1/traces 204
19:14:10.227 chunk 17 34438 bytes
19:14:10.230 in Ack 17
19:14:10.627 chunk 17 35018 bytes
19:14:10.629 in Ack 17
19:14:11.029 chunk 17 35030 bytes
19:14:11.030 in Ack 17
19:14:11.231 fetch POST /api/observability/v1/traces
19:14:11.231 fetched POST /api/observability/v1/traces 204
19:14:11.426 chunk 17 35042 bytes
19:14:11.427 in Ack 17
19:14:11.822 chunk 17 35054 bytes
19:14:11.823 in Ack 17
19:14:11.906 in Ping 
19:14:12.223 chunk 17 35076 bytes
19:14:12.224 in Ack 17
19:14:12.428 fetch POST /api/observability/v1/traces
19:14:12.428 fetched POST /api/observability/v1/traces 204
19:14:12.626 chunk 17 35088 bytes
19:14:12.627 in Ack 17
19:14:13.628 fetch POST /api/observability/v1/traces
19:14:13.628 fetched POST /api/observability/v1/traces 204
19:14:14.031 chunk 17 35110 bytes
19:14:14.032 in Ack 17
19:14:14.529 chunk 17 35130 bytes
19:14:14.531 in Ack 17
19:14:15.032 fetch POST /api/observability/v1/traces
19:14:15.032 fetched POST /api/observability/v1/traces 204
19:14:15.103 chunk 17 35152 bytes
19:14:15.105 in Ack 17
19:14:15.678 chunk 17 35174 bytes
19:14:15.679 in Ack 17
19:14:16.106 fetch POST /api/observability/v1/traces
19:14:16.106 fetched POST /api/observability/v1/traces 204
19:14:16.176 chunk 17 35196 bytes
19:14:16.178 in Ack 17
19:14:16.413 in Interrupt 28
19:14:16.675 chunk 17 35206 bytes
19:14:16.676 in Ack 17
19:14:16.908 in Ping 
19:14:17.113 rpc 30 server.reportClientActivity
19:14:17.173 chunk 17 35216 bytes
19:14:17.174 in Ack 17
19:14:17.196 fetch POST /api/observability/v1/traces
19:14:17.196 fetched POST /api/observability/v1/traces 204
19:14:17.672 chunk 17 35238 bytes
19:14:17.673 in Ack 17
19:14:18.170 chunk 17 35260 bytes
19:14:18.171 in Ack 17
19:14:18.674 fetch POST /api/observability/v1/traces
19:14:18.674 fetched POST /api/observability/v1/traces 204
19:14:19.143 chunk 17 35376 bytes
19:14:19.144 in Ack 17
19:14:19.398 chunk 17 35424 bytes
19:14:19.399 in Ack 17
19:14:19.653 chunk 17 35434 bytes
19:14:19.654 in Ack 17
19:14:20.145 fetch POST /api/observability/v1/traces
19:14:20.145 fetched POST /api/observability/v1/traces 204
19:14:20.157 chunk 17 35444 bytes
19:14:20.158 in Ack 17
19:14:20.919 chunk 17 35466 bytes
19:14:20.919 in Ack 17
19:14:21.160 fetch POST /api/observability/v1/traces
19:14:21.160 fetched POST /api/observability/v1/traces 204
19:14:21.175 chunk 17 35498 bytes
19:14:21.176 in Ack 17
19:14:21.503 chunk 17 35510 bytes
19:14:21.504 in Ack 17
19:14:21.755 chunk 17 35540 bytes
19:14:21.756 in Ack 17
19:14:21.908 in Ping 
19:14:22.007 chunk 17 35590 bytes
19:14:22.008 in Ack 17
19:14:22.177 fetch POST /api/observability/v1/traces
19:14:22.177 fetched POST /api/observability/v1/traces 204
19:14:22.766 chunk 17 35658 bytes
19:14:22.767 in Ack 17
19:14:23.082 chunk 17 35680 bytes
19:14:23.083 in Ack 17
19:14:23.332 chunk 17 35702 bytes
19:14:23.333 in Ack 17
19:14:23.591 chunk 17 35722 bytes
19:14:23.592 in Ack 17
19:14:23.768 fetch POST /api/observability/v1/traces
19:14:23.768 fetched POST /api/observability/v1/traces 204
19:14:23.849 chunk 17 35762 bytes
19:14:23.850 in Ack 17
19:14:24.363 chunk 17 35792 bytes
19:14:24.364 in Ack 17
19:14:24.694 chunk 17 35952 bytes
19:14:24.695 in Ack 17
19:14:24.851 fetch POST /api/observability/v1/traces
19:14:24.851 fetched POST /api/observability/v1/traces 204
19:14:25.038 chunk 17 35990 bytes
19:14:25.039 in Ack 17
19:14:25.296 chunk 17 36020 bytes
19:14:25.298 in Ack 17
19:14:25.798 chunk 17 36040 bytes
19:14:25.799 in Ack 17
19:14:26.040 fetch POST /api/observability/v1/traces
19:14:26.040 fetched POST /api/observability/v1/traces 204
19:14:26.214 chunk 17 36072 bytes
19:14:26.215 in Ack 17
19:14:26.472 chunk 17 36102 bytes
19:14:26.473 in Ack 17
19:14:26.723 chunk 17 36112 bytes
19:14:26.725 in Ack 17
19:14:26.909 in Ping 
19:14:26.973 chunk 17 36294 bytes
19:14:26.974 in Ack 17
19:14:27.217 fetch POST /api/observability/v1/traces
19:14:27.217 fetched POST /api/observability/v1/traces 204
19:14:27.224 chunk 17 36344 bytes
19:14:27.226 in Ack 17
19:14:27.565 chunk 17 36402 bytes
19:14:27.566 in Ack 17
19:14:27.993 chunk 17 36472 bytes
19:14:27.994 in Ack 17
19:14:28.227 fetch POST /api/observability/v1/traces
19:14:28.227 fetched POST /api/observability/v1/traces 204
19:14:28.252 chunk 17 36492 bytes
19:14:28.253 in Ack 17
19:14:28.510 chunk 17 36558 bytes
19:14:28.511 in Ack 17
19:14:28.762 chunk 17 36694 bytes
19:14:28.763 in Ack 17
19:14:29.020 chunk 17 36734 bytes
19:14:29.021 in Ack 17
19:14:29.254 fetch POST /api/observability/v1/traces
19:14:29.254 fetched POST /api/observability/v1/traces 204
19:14:29.285 chunk 17 36856 bytes
19:14:29.286 in Ack 17
19:14:29.537 chunk 17 36904 bytes
19:14:29.538 in Ack 17
19:14:29.790 chunk 17 36936 bytes
19:14:29.791 in Ack 17
19:14:30.185 chunk 17 36994 bytes
19:14:30.186 in Ack 17
19:14:30.287 fetch POST /api/observability/v1/traces
19:14:30.287 fetched POST /api/observability/v1/traces 204
19:14:30.444 chunk 17 37014 bytes
19:14:30.445 in Ack 17
19:14:30.778 chunk 17 37034 bytes
19:14:30.779 in Ack 17
19:14:31.039 chunk 17 37044 bytes
19:14:31.043 in Ack 17
19:14:31.287 chunk 17 37054 bytes
19:14:31.288 in Ack 17
19:14:31.446 fetch POST /api/observability/v1/traces
19:14:31.446 fetched POST /api/observability/v1/traces 204
19:14:31.537 chunk 17 37064 bytes
19:14:31.541 in Ack 17
19:14:31.909 in Ping 
19:14:31.933 chunk 17 37112 bytes
19:14:31.934 in Ack 17
19:14:32.114 in Interrupt 30
19:14:32.329 chunk 17 37132 bytes
19:14:32.330 in Ack 17
19:14:32.542 fetch POST /api/observability/v1/traces
19:14:32.542 fetched POST /api/observability/v1/traces 204
19:14:32.722 chunk 17 37144 bytes
19:14:32.723 in Ack 17
19:14:32.982 chunk 17 37184 bytes
19:14:32.983 in Ack 17
19:14:33.324 chunk 17 37216 bytes
19:14:33.325 in Ack 17
19:14:33.652 chunk 17 37278 bytes
19:14:33.654 in Ack 17
19:14:33.903 chunk 17 37308 bytes
19:14:33.904 fetch POST /api/observability/v1/traces
19:14:33.904 fetched POST /api/observability/v1/traces 204
19:14:33.906 in Ack 17
19:14:34.235 chunk 17 37370 bytes
19:14:34.237 in Ack 17
19:14:34.852 chunk 17 37440 bytes
19:14:34.853 in Ack 17
19:14:34.906 fetch POST /api/observability/v1/traces
19:14:34.906 fetched POST /api/observability/v1/traces 204
19:14:35.975 chunk 17 37490 bytes
19:14:35.976 in Ack 17
19:14:36.910 in Ping 
19:14:36.977 fetch POST /api/observability/v1/traces
19:14:36.977 fetched POST /api/observability/v1/traces 204
19:14:37.212 chunk 17 37558 bytes
19:14:37.213 in Ack 17
19:14:38.215 fetch POST /api/observability/v1/traces
19:14:38.215 fetched POST /api/observability/v1/traces 204
19:14:39.936 chunk 17 37620 bytes
19:14:39.938 in Ack 17
19:14:40.940 fetch POST /api/observability/v1/traces
19:14:40.940 fetched POST /api/observability/v1/traces 204
19:14:41.910 in Ping 
19:14:42.113 rpc 31 server.reportClientActivity
19:14:43.114 fetch POST /api/observability/v1/traces
19:14:43.114 fetched POST /api/observability/v1/traces 204
19:14:45.607 chunk 17 38625 bytes
19:14:45.608 in Ack 17
19:14:45.803 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:14:45.839 fetch POST /api/observability/v1/traces
19:14:45.839 fetched POST /api/observability/v1/traces 204
19:14:46.912 in Ping 
19:14:49.076 chunk 17 41155 bytes
19:14:49.076 exit 32 {"_tag":"Success","value":{"sequence":95}}
19:14:49.077 in Ack 17
19:14:49.510 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:14:49.516 chunk 17 41205 bytes
19:14:49.532 in Ack 17
19:14:49.988 chunk 17 41255 bytes
19:14:49.989 in Ack 17
19:14:50.079 fetch POST /api/observability/v1/traces
19:14:50.079 fetched POST /api/observability/v1/traces 204
19:14:50.410 chunk 17 41275 bytes
19:14:50.411 in Ack 17
19:14:50.847 chunk 17 41305 bytes
19:14:50.848 in Ack 17
19:14:51.285 chunk 17 41367 bytes
19:14:51.287 in Ack 17
19:14:51.413 fetch POST /api/observability/v1/traces
19:14:51.413 fetched POST /api/observability/v1/traces 204
19:14:51.902 chunk 17 41447 bytes
19:14:51.903 in Ack 17
19:14:51.923 in Ping 
19:14:52.904 fetch POST /api/observability/v1/traces
19:14:52.904 fetched POST /api/observability/v1/traces 204
19:14:52.966 chunk 17 41487 bytes
19:14:52.966 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:14:52.967 in Ack 17
19:14:53.327 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:14:53.968 fetch POST /api/observability/v1/traces
19:14:53.968 fetched POST /api/observability/v1/traces 204
19:14:55.773 fetch POST /api/observability/v1/traces
19:14:55.773 fetched POST /api/observability/v1/traces 204
19:14:56.927 in Ping 
19:14:57.114 in Interrupt 31
19:14:57.811 chunk 17 41843 bytes
19:14:57.812 in Ack 17
19:14:58.115 fetch POST /api/observability/v1/traces
19:14:58.116 fetched POST /api/observability/v1/traces 204
19:14:59.871 chunk 17 41908 bytes
19:14:59.871 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:14:59.872 in Ack 17
19:15:00.873 fetch POST /api/observability/v1/traces
19:15:00.873 fetched POST /api/observability/v1/traces 204
19:15:01.928 in Ping 
19:15:04.419 chunk 17 42112 bytes
19:15:04.420 in Ack 17
19:15:05.421 fetch POST /api/observability/v1/traces
19:15:05.421 fetched POST /api/observability/v1/traces 204
19:15:06.043 chunk 17 42378 bytes
19:15:06.044 in Ack 17
19:15:06.934 in Ping 
19:15:07.047 fetch POST /api/observability/v1/traces
19:15:07.047 fetched POST /api/observability/v1/traces 204
19:15:07.118 rpc 35 server.reportClientActivity
19:15:07.914 chunk 17 42460 bytes
19:15:07.918 in Ack 17
19:15:08.127 fetch POST /api/observability/v1/traces
19:15:08.127 fetched POST /api/observability/v1/traces 204
19:15:10.332 chunk 17 42592 bytes
19:15:10.333 in Ack 17
19:15:11.334 fetch POST /api/observability/v1/traces
19:15:11.334 fetched POST /api/observability/v1/traces 204
