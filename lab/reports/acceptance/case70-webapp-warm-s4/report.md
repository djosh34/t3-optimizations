# case70, webapp, warm start, seed 4

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 64 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.2s |
| Prompt reached the agent | 3.6s |
| Time to first text | 11.1s |
| Whole reply shown | 63.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 12.3s / 4.8s / 10.5s |
| Longest silence | 11.2s |
| Approval shown after it was asked | 13.7s |
| Bytes on the wire down / up | 68746 / 29196 |
| Bytes the app received / sent | 56337 / 65993 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.3 | 0 | 258 |
| 20 | 5.7 | 424 | 655 |
| 30 | 11.3 | 601 | 1061 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 2.8 | 1158 | 1274 |
| 60 | 8.1 | 1344 | 1596 |
| 63 | 11.1 | 1344 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","user":"10/10 9/9","assistant":"32/32 21/21","attempts":"660/660 660/660","providerTurns":"774/774 508/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","messages":"456/456 546/546 437/901","checkpointScopes":"734/734","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164844323,"silences":0,"longestSilenceMs":1223,"longestRecoveredGapMs":1223},"stats":{"sendRate":4500,"deliveredRate":1844.2844284428443,"loss":0,"srttMs":6.080590447974225,"minRttMs":1,"sentPackets":97,"sentWireBytes":31816,"receivedPackets":89,"receivedWireBytes":50170,"lastHeardMs":8314,"unsyncedBytes":1009,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 817/849 281/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","user":"10/10 9/9","assistant":"32/32 21/21 75/270","attempts":"660/660 660/660","providerTurns":"774/774 540/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164854899,"silences":1,"longestSilenceMs":2039,"longestRecoveredGapMs":2039},"stats":{"sendRate":1314.068786784108,"deliveredRate":1134.9774091854124,"loss":0.33865263157894737,"srttMs":1043.4894556780732,"minRttMs":730,"sentPackets":136,"sentWireBytes":44539,"receivedPackets":102,"receivedWireBytes":58734,"lastHeardMs":18890,"unsyncedBytes":0,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 742/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 49/839","user":"10/10 9/9","assistant":"32/32 21/21 583/613","attempts":"660/660 660/660 389/636","providerTurns":"774/774 596/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164865039,"silences":1,"longestSilenceMs":2039,"longestRecoveredGapMs":2039},"stats":{"sendRate":1971.103180176162,"deliveredRate":0,"loss":0.479913984962406,"srttMs":1043.4894556780732,"minRttMs":730,"sentPackets":197,"sentWireBytes":57367,"receivedPackets":135,"receivedWireBytes":69854,"lastHeardMs":29031,"unsyncedBytes":500,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 102/839","user":"10/10 9/9","assistant":"32/32 21/21 1033/1088","attempts":"660/660 660/660 479/636","providerTurns":"774/774 596/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164875355,"silences":1,"longestSilenceMs":2039,"longestRecoveredGapMs":2039},"stats":{"sendRate":1478.3273851321214,"deliveredRate":178.81619937694705,"loss":0.8461538461538461,"srttMs":2805.1970376905842,"minRttMs":730,"sentPackets":243,"sentWireBytes":70299,"receivedPackets":172,"receivedWireBytes":80942,"lastHeardMs":39347,"unsyncedBytes":319,"inflightPackets":100,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 225/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 102/839","user":"10/10 9/9 49/64","assistant":"32/32 21/21 1340/1340","attempts":"660/660 660/660 479/636","providerTurns":"774/774 596/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779","runtimeRequests":"840/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164885474,"silences":1,"longestSilenceMs":2039,"longestRecoveredGapMs":2039},"stats":{"sendRate":831.5591541368184,"deliveredRate":362.97120350885314,"loss":0.704845954045954,"srttMs":4183.79762730475,"minRttMs":730,"sentPackets":292,"sentWireBytes":79039,"receivedPackets":200,"receivedWireBytes":90921,"lastHeardMs":49465,"unsyncedBytes":0,"inflightPackets":97,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1259/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 102/839","user":"10/10 9/9 49/64","assistant":"32/32 21/21 1340/1340 228/278","attempts":"660/660 660/660 554/636","providerTurns":"774/774 596/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779","runtimeRequests":"283/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164895933,"silences":2,"longestSilenceMs":2198,"longestRecoveredGapMs":2198},"stats":{"sendRate":623.6693656026138,"deliveredRate":0,"loss":0.6549306293706292,"srttMs":3217.4436287008557,"minRttMs":730,"sentPackets":318,"sentWireBytes":87867,"receivedPackets":218,"receivedWireBytes":99960,"lastHeardMs":59924,"unsyncedBytes":483,"inflightPackets":112,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1259/1546 429/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 118/971","user":"10/10 9/9 49/64","assistant":"32/32 21/21 1340/1340 703/723","attempts":"660/660 660/660 554/636","providerTurns":"774/774 596/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 434/1019","messages":"456/456 546/546 557/901 546/546","checkpointScopes":"138/734","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":114,"lastDataAt":1794164906340,"silences":3,"longestSilenceMs":3195,"longestRecoveredGapMs":3195},"stats":{"sendRate":623.6693656026138,"deliveredRate":138.7821030528186,"loss":0.7657894736842106,"srttMs":2965.2631751132485,"minRttMs":730,"sentPackets":359,"sentWireBytes":94077,"receivedPackets":232,"receivedWireBytes":105999,"lastHeardMs":70331,"unsyncedBytes":0,"inflightPackets":98,"duplicateBytes":0}}
- shim trace
19:07:16.305 fetch GET /api/auth/session
19:07:16.306 fetched GET /api/auth/session 200
19:07:16.417 fetch GET /.well-known/t3/environment
19:07:16.418 fetched GET /.well-known/t3/environment 200
19:07:16.426 fetch GET /.well-known/t3/environment
19:07:16.427 fetched GET /.well-known/t3/environment 200
19:07:16.430 rpc 0 subscribeServerConfig
19:07:16.430 chunk 0 38722 bytes
19:07:16.445 in Ack 0
19:07:16.447 rpc 1 subscribeServerLifecycle
19:07:16.447 rpc 2 subscribeServerLifecycle
19:07:16.449 fetch GET /api/auth/session
19:07:16.450 fetched GET /api/auth/session 200
19:07:16.450 rpc 3 subscribeDeviceState
19:07:16.451 rpc 4 preview.reportProfiles
19:07:16.493 rpc 5 subscribeProjectClones
19:07:16.510 fetch GET /api/orchestration/threads/mcp%3A032cd927-5bc3-46be-991c-9422f0475f03/bounded
19:07:16.513 fetch GET /api/orchestration/shell
19:07:16.514 fetched GET /api/orchestration/shell 200
19:07:16.516 rpc 6 orchestration.subscribeShell
19:07:16.516 chunk 6 2790 bytes
19:07:16.543 rpc 7 server.refreshProviders
19:07:16.579 rpc 8 orchestration.dispatchCommand thread.visit
19:07:16.579 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:07:16.591 rpc 9 subscribeTerminalMetadata
19:07:16.592 rpc 10 subscribeVcsStatus
19:07:16.592 rpc 11 subscribePreviewEvents
19:07:16.593 rpc 12 assets.createUrl
19:07:16.594 rpc 13 preview.list
19:07:16.594 rpc 14 projects.readFile
19:07:16.597 in Ack 6
19:07:16.752 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"17f83099-7fea-467e-9330-450f5b0d1f75","sessions":[]}}
19:07:16.752 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:07:16.844 rpc 15 server.reportClientActivity
19:07:16.879 chunk 1 3872 bytes
19:07:16.880 in Ack 1
19:07:16.989 chunk 2 3872 bytes
19:07:16.990 in Ack 2
19:07:17.152 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:07:04.661Z","configurableProviders":false,"con
19:07:17.152 chunk 10 968 bytes
19:07:17.152 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:07:17.155 in Ack 10
19:07:17.240 chunk 6 2766 bytes
19:07:17.240 in Ack 6
19:07:17.388 chunk 3 876 bytes
19:07:17.388 exit 4 {"_tag":"Success","value":null}
19:07:17.388 chunk 5 4 bytes
19:07:17.388 chunk 9 36 bytes
19:07:17.388 exit 15 {"_tag":"Success","value":null}
19:07:17.389 in Ack 3
19:07:17.392 rpc 16 preview.reportProfiles
19:07:17.392 in Ack 5
19:07:17.393 in Ack 9
19:07:17.416 fetch POST /api/observability/v1/traces
19:07:17.416 fetched POST /api/observability/v1/traces 204
19:07:17.468 exit 16 {"_tag":"Success","value":null}
19:07:17.490 chunk 6 2809 bytes
19:07:17.490 in Ack 6
19:07:17.668 fetched GET /api/orchestration/threads/mcp%3A032cd927-5bc3-46be-991c-9422f0475f03/bounded 200
19:07:17.671 rpc 17 orchestration.subscribeThread
19:07:17.671 chunk 17 8283 bytes
19:07:17.680 in Interrupt 10
19:07:17.682 rpc 18 subscribeVcsStatus
19:07:17.727 in Ack 17
19:07:17.800 chunk 18 968 bytes
19:07:17.800 in Ack 18
19:07:17.921 chunk 17 11920 bytes
19:07:17.922 in Ack 17
19:07:17.974 rpc 19 server.reportClientActivity
19:07:18.168 exit 19 {"_tag":"Success","value":null}
19:07:18.171 chunk 17 13502 bytes
19:07:18.172 in Ack 17
19:07:18.351 rpc 20 attachments.createUploadUrl
19:07:18.515 chunk 17 18605 bytes
19:07:18.516 fetch POST /api/observability/v1/traces
19:07:18.516 fetched POST /api/observability/v1/traces 204
19:07:18.525 in Ack 17
19:07:18.566 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-12ac51ed-a981-44a8-8327-f2bbaca8a34f","expiresAt":1794165438548,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:07:18.567 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTEyYWM1MWVkLWE5ODEtNDRhOC04MzI3LWYyYmJhY2E4YTM0ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NTQzODU0OH0.oQQT8ExTw3jU0hxnSZJLldrFZ-LNAFp9bKhQi2dfHpQ
19:07:18.764 chunk 17 21199 bytes
19:07:18.765 in Ack 17
19:07:18.772 in Interrupt 18
19:07:18.773 rpc 21 subscribeVcsStatus
19:07:18.829 chunk 21 968 bytes
19:07:18.829 in Ack 21
19:07:19.024 rpc 22 server.reportClientActivity
19:07:19.526 fetch POST /api/observability/v1/traces
19:07:19.526 fetched POST /api/observability/v1/traces 204
19:07:19.627 exit 22 {"_tag":"Success","value":null}
19:07:20.632 fetch POST /api/observability/v1/traces
19:07:20.632 fetched POST /api/observability/v1/traces 204
19:07:21.482 rpc 23 orchestration.dispatchCommand message.dispatch
19:07:22.020 in Ping 
19:07:22.089 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:07:22.091 rpc 24 attachments.delete
19:07:22.482 fetch POST /api/observability/v1/traces
19:07:22.482 fetched POST /api/observability/v1/traces 204
19:07:22.590 chunk 17 23653 bytes
19:07:22.591 in Ack 17
19:07:22.840 chunk 17 23665 bytes
19:07:22.841 in Ack 17
19:07:23.090 chunk 17 23667 bytes
19:07:23.091 in Ack 17
19:07:23.337 exit 24 {"_tag":"Success","value":null}
19:07:23.339 chunk 6 2831 bytes
19:07:23.339 in Ack 6
19:07:23.340 rpc 25 orchestration.dispatchCommand thread.visit
19:07:23.340 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:07:23.376 chunk 17 27432 bytes
19:07:23.381 in Ack 17
19:07:23.383 rpc 26 assets.createUrl
19:07:23.589 chunk 6 2831 bytes
19:07:23.589 in Ack 6
19:07:23.594 fetch POST /api/observability/v1/traces
19:07:23.594 fetched POST /api/observability/v1/traces 204
19:07:23.626 chunk 17 28093 bytes
19:07:23.627 in Ack 17
19:07:23.877 chunk 17 31326 bytes
19:07:23.878 in Ack 17
19:07:24.300 chunk 17 31940 bytes
19:07:24.302 in Ack 17
19:07:24.308 in Interrupt 21
19:07:24.310 rpc 27 subscribeVcsStatus
19:07:24.564 rpc 28 server.reportClientActivity
19:07:24.631 fetch POST /api/observability/v1/traces
19:07:24.631 fetched POST /api/observability/v1/traces 204
19:07:24.953 rpc 29 orchestration.dispatchCommand message.dispatch
19:07:25.953 fetch POST /api/observability/v1/traces
19:07:25.955 fetched POST /api/observability/v1/traces 204
19:07:26.462 chunk 17 32508 bytes
19:07:26.463 chunk 27 968 bytes
19:07:26.464 in Ack 17
19:07:26.473 in Ack 27
19:07:27.021 in Ping 
19:07:27.465 fetch POST /api/observability/v1/traces
19:07:27.465 fetched POST /api/observability/v1/traces 204
19:07:29.187 fetch POST /api/observability/v1/traces
19:07:29.187 fetched POST /api/observability/v1/traces 204
19:07:29.930 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:07:30.933 fetch POST /api/observability/v1/traces
19:07:30.933 fetched POST /api/observability/v1/traces 204
19:07:31.969 chunk 17 34974 bytes
19:07:31.970 in Ack 17
19:07:32.023 in Ping 
19:07:32.606 chunk 17 34986 bytes
19:07:32.608 in Ack 17
19:07:32.972 fetch POST /api/observability/v1/traces
19:07:32.973 fetched POST /api/observability/v1/traces 204
19:07:33.170 chunk 17 35028 bytes
19:07:33.171 in Ack 17
19:07:33.604 chunk 17 35048 bytes
19:07:33.605 in Ack 17
19:07:34.172 fetch POST /api/observability/v1/traces
19:07:34.172 fetched POST /api/observability/v1/traces 204
19:07:34.456 chunk 17 35070 bytes
19:07:34.457 in Ack 17
19:07:34.900 chunk 17 35090 bytes
19:07:34.901 in Ack 17
19:07:35.240 chunk 17 35210 bytes
19:07:35.241 in Ack 17
19:07:35.458 fetch POST /api/observability/v1/traces
19:07:35.458 fetched POST /api/observability/v1/traces 204
19:07:35.499 chunk 17 35220 bytes
19:07:35.500 in Ack 17
19:07:35.757 chunk 17 35486 bytes
19:07:35.758 in Ack 17
19:07:36.270 chunk 17 35498 bytes
19:07:36.271 in Ack 17
19:07:36.501 fetch POST /api/observability/v1/traces
19:07:36.501 fetched POST /api/observability/v1/traces 204
19:07:36.774 chunk 17 35520 bytes
19:07:36.776 in Ack 17
19:07:37.030 in Ping 
19:07:37.032 chunk 17 35532 bytes
19:07:37.033 in Ack 17
19:07:37.284 chunk 17 35544 bytes
19:07:37.286 in Ack 17
19:07:37.538 chunk 17 35556 bytes
19:07:37.539 in Ack 17
19:07:37.777 fetch POST /api/observability/v1/traces
19:07:37.777 fetched POST /api/observability/v1/traces 204
19:07:37.787 chunk 17 35566 bytes
19:07:37.788 in Ack 17
19:07:38.038 chunk 17 35576 bytes
19:07:38.039 in Ack 17
19:07:38.288 chunk 17 35588 bytes
19:07:38.289 in Ack 17
19:07:38.790 fetch POST /api/observability/v1/traces
19:07:38.790 fetched POST /api/observability/v1/traces 204
19:07:39.033 chunk 17 35620 bytes
19:07:39.034 in Ack 17
19:07:39.457 chunk 17 35630 bytes
19:07:39.459 in Ack 17
19:07:39.565 in Interrupt 28
19:07:39.879 chunk 17 35650 bytes
19:07:39.880 in Ack 17
19:07:40.035 fetch POST /api/observability/v1/traces
19:07:40.035 fetched POST /api/observability/v1/traces 204
19:07:40.310 chunk 17 35660 bytes
19:07:40.311 in Ack 17
19:07:40.560 chunk 17 35780 bytes
19:07:40.561 in Ack 17
19:07:40.984 chunk 17 35792 bytes
19:07:40.985 in Ack 17
19:07:41.312 fetch POST /api/observability/v1/traces
19:07:41.312 fetched POST /api/observability/v1/traces 204
19:07:41.415 chunk 17 35906 bytes
19:07:41.416 in Ack 17
19:07:41.678 rpc 30 server.reportClientActivity
19:07:41.846 chunk 17 35916 bytes
19:07:41.847 in Ack 17
19:07:42.030 in Ping 
19:07:42.174 chunk 17 35928 bytes
19:07:42.175 in Ack 17
19:07:42.417 fetch POST /api/observability/v1/traces
19:07:42.417 fetched POST /api/observability/v1/traces 204
19:07:42.433 chunk 17 35940 bytes
19:07:42.434 in Ack 17
19:07:42.691 chunk 17 35950 bytes
19:07:42.692 in Ack 17
19:07:43.205 chunk 17 35982 bytes
19:07:43.206 in Ack 17
19:07:43.434 fetch POST /api/observability/v1/traces
19:07:43.434 fetched POST /api/observability/v1/traces 204
19:07:43.461 chunk 17 35992 bytes
19:07:43.462 in Ack 17
19:07:43.715 chunk 17 36014 bytes
19:07:43.716 in Ack 17
19:07:44.225 chunk 17 36026 bytes
19:07:44.226 in Ack 17
19:07:44.463 fetch POST /api/observability/v1/traces
19:07:44.463 fetched POST /api/observability/v1/traces 204
19:07:44.477 chunk 17 36110 bytes
19:07:44.478 in Ack 17
19:07:45.478 fetch POST /api/observability/v1/traces
19:07:45.478 fetched POST /api/observability/v1/traces 204
19:07:45.521 chunk 17 36120 bytes
19:07:45.522 in Ack 17
19:07:45.771 chunk 17 36130 bytes
19:07:45.772 in Ack 17
19:07:46.251 chunk 17 36152 bytes
19:07:46.252 in Ack 17
19:07:46.503 chunk 17 36192 bytes
19:07:46.505 in Ack 17
19:07:46.523 fetch POST /api/observability/v1/traces
19:07:46.523 fetched POST /api/observability/v1/traces 204
19:07:46.836 chunk 17 36194 bytes
19:07:46.838 in Ack 17
19:07:47.030 in Ping 
19:07:47.092 chunk 17 36232 bytes
19:07:47.093 in Ack 17
19:07:47.428 chunk 17 36252 bytes
19:07:47.429 in Ack 17
19:07:47.772 chunk 17 36272 bytes
19:07:47.773 in Ack 17
19:07:47.838 fetch POST /api/observability/v1/traces
19:07:47.838 fetched POST /api/observability/v1/traces 204
19:07:48.112 chunk 17 36292 bytes
19:07:48.113 in Ack 17
19:07:48.622 chunk 17 36344 bytes
19:07:48.623 in Ack 17
19:07:49.018 chunk 17 36346 bytes
19:07:49.019 in Ack 17
19:07:49.114 fetch POST /api/observability/v1/traces
19:07:49.114 fetched POST /api/observability/v1/traces 204
19:07:49.274 chunk 17 36386 bytes
19:07:49.275 in Ack 17
19:07:49.523 chunk 17 36482 bytes
19:07:49.525 in Ack 17
19:07:50.029 chunk 17 36502 bytes
19:07:50.030 in Ack 17
19:07:50.276 fetch POST /api/observability/v1/traces
19:07:50.276 fetched POST /api/observability/v1/traces 204
19:07:50.285 chunk 17 36534 bytes
19:07:50.286 in Ack 17
19:07:50.795 chunk 17 36566 bytes
19:07:50.796 in Ack 17
19:07:51.050 chunk 17 36586 bytes
19:07:51.051 in Ack 17
19:07:51.287 fetch POST /api/observability/v1/traces
19:07:51.287 fetched POST /api/observability/v1/traces 204
19:07:51.305 chunk 17 36624 bytes
19:07:51.306 in Ack 17
19:07:51.564 chunk 17 36636 bytes
19:07:51.565 in Ack 17
19:07:51.960 chunk 17 36674 bytes
19:07:51.961 in Ack 17
19:07:52.030 in Ping 
19:07:52.215 chunk 17 36684 bytes
19:07:52.217 in Ack 17
19:07:52.307 fetch POST /api/observability/v1/traces
19:07:52.307 fetched POST /api/observability/v1/traces 204
19:07:52.465 chunk 17 36704 bytes
19:07:52.467 in Ack 17
19:07:52.715 chunk 17 36734 bytes
19:07:52.716 in Ack 17
19:07:52.968 chunk 17 36802 bytes
19:07:52.969 in Ack 17
19:07:53.223 chunk 17 36822 bytes
19:07:53.224 in Ack 17
19:07:53.468 fetch POST /api/observability/v1/traces
19:07:53.468 fetched POST /api/observability/v1/traces 204
19:07:53.475 chunk 17 36794 bytes
19:07:53.476 in Ack 17
19:07:53.726 chunk 17 36804 bytes
19:07:53.727 in Ack 17
19:07:53.982 chunk 17 36822 bytes
19:07:53.983 in Ack 17
19:07:54.240 chunk 17 36852 bytes
19:07:54.241 in Ack 17
19:07:54.477 fetch POST /api/observability/v1/traces
19:07:54.477 fetched POST /api/observability/v1/traces 204
19:07:54.499 chunk 17 36910 bytes
19:07:54.500 in Ack 17
19:07:54.759 chunk 17 36932 bytes
19:07:54.760 in Ack 17
19:07:55.101 chunk 17 36942 bytes
19:07:55.102 in Ack 17
19:07:55.356 chunk 17 36962 bytes
19:07:55.357 in Ack 17
19:07:55.501 fetch POST /api/observability/v1/traces
19:07:55.501 fetched POST /api/observability/v1/traces 204
19:07:55.694 chunk 17 37004 bytes
19:07:55.695 in Ack 17
19:07:56.032 chunk 17 37066 bytes
19:07:56.033 in Ack 17
19:07:56.290 chunk 17 37114 bytes
19:07:56.291 in Ack 17
19:07:56.545 chunk 17 37134 bytes
19:07:56.546 in Ack 17
19:07:56.678 in Interrupt 30
19:07:56.696 fetch POST /api/observability/v1/traces
19:07:56.696 fetched POST /api/observability/v1/traces 204
19:07:56.804 chunk 17 37156 bytes
19:07:56.805 in Ack 17
19:07:57.031 in Ping 
19:07:57.062 chunk 17 37166 bytes
19:07:57.063 in Ack 17
19:07:57.488 chunk 17 37206 bytes
19:07:57.490 in Ack 17
19:07:57.806 fetch POST /api/observability/v1/traces
19:07:57.806 fetched POST /api/observability/v1/traces 204
19:07:57.861 chunk 17 37234 bytes
19:07:57.862 in Ack 17
19:07:58.192 chunk 17 37274 bytes
19:07:58.194 in Ack 17
19:07:58.443 chunk 17 37352 bytes
19:07:58.444 in Ack 17
19:07:58.863 fetch POST /api/observability/v1/traces
19:07:58.863 fetched POST /api/observability/v1/traces 204
19:07:59.137 chunk 17 37392 bytes
19:07:59.142 in Ack 17
19:07:59.760 chunk 17 37442 bytes
19:07:59.762 in Ack 17
19:08:00.010 chunk 17 37460 bytes
19:08:00.011 in Ack 17
19:08:00.142 fetch POST /api/observability/v1/traces
19:08:00.143 fetched POST /api/observability/v1/traces 204
19:08:00.791 chunk 17 37488 bytes
19:08:00.792 in Ack 17
19:08:01.051 chunk 17 37516 bytes
19:08:01.052 in Ack 17
19:08:01.383 chunk 17 37572 bytes
19:08:01.384 in Ack 17
19:08:01.793 fetch POST /api/observability/v1/traces
19:08:01.793 fetched POST /api/observability/v1/traces 204
19:08:02.031 in Ping 
19:08:06.678 rpc 31 server.reportClientActivity
19:08:07.031 in Ping 
19:08:07.679 fetch POST /api/observability/v1/traces
19:08:07.679 fetched POST /api/observability/v1/traces 204
19:08:08.881 chunk 17 38577 bytes
19:08:08.882 in Ack 17
19:08:09.092 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:08:09.224 fetch POST /api/observability/v1/traces
19:08:09.224 fetched POST /api/observability/v1/traces 204
19:08:11.763 chunk 17 41147 bytes
19:08:11.763 exit 32 {"_tag":"Success","value":{"sequence":95}}
19:08:11.764 in Ack 17
19:08:12.035 in Ping 
19:08:12.765 fetch POST /api/observability/v1/traces
19:08:12.765 fetched POST /api/observability/v1/traces 204
19:08:12.772 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:08:12.799 chunk 17 41257 bytes
19:08:12.800 in Ack 17
19:08:13.774 fetch POST /api/observability/v1/traces
19:08:13.774 fetched POST /api/observability/v1/traces 204
19:08:14.997 chunk 17 41359 bytes
19:08:14.998 in Ack 17
19:08:15.934 chunk 17 41459 bytes
19:08:15.935 in Ack 17
19:08:16.000 fetch POST /api/observability/v1/traces
19:08:16.000 fetched POST /api/observability/v1/traces 204
19:08:17.039 in Ping 
19:08:17.144 chunk 17 41707 bytes
19:08:17.145 in Ack 17
19:08:17.397 chunk 17 41727 bytes
19:08:17.398 in Ack 17
19:08:18.146 fetch POST /api/observability/v1/traces
19:08:18.146 fetched POST /api/observability/v1/traces 204
19:08:19.226 fetch POST /api/observability/v1/traces
19:08:19.226 fetched POST /api/observability/v1/traces 204
19:08:20.592 chunk 17 41783 bytes
19:08:20.593 in Ack 17
19:08:21.030 chunk 17 41803 bytes
19:08:21.031 in Ack 17
19:08:21.595 fetch POST /api/observability/v1/traces
19:08:21.595 fetched POST /api/observability/v1/traces 204
19:08:21.679 in Interrupt 31
19:08:22.043 in Ping 
19:08:22.556 chunk 17 41891 bytes
19:08:22.558 in Ack 17
19:08:22.679 fetch POST /api/observability/v1/traces
19:08:22.679 fetched POST /api/observability/v1/traces 204
19:08:22.990 chunk 17 41917 bytes
19:08:22.991 in Ack 17
19:08:23.425 chunk 17 41939 bytes
19:08:23.426 in Ack 17
19:08:23.680 chunk 17 42207 bytes
19:08:23.681 in Ack 17
19:08:23.936 chunk 17 42217 bytes
19:08:23.937 in Ack 17
19:08:24.028 fetch POST /api/observability/v1/traces
19:08:24.029 fetched POST /api/observability/v1/traces 204
19:08:24.194 chunk 17 42227 bytes
19:08:24.196 in Ack 17
19:08:24.452 chunk 17 42237 bytes
19:08:24.454 in Ack 17
19:08:24.790 chunk 17 42251 bytes
19:08:24.791 in Ack 17
19:08:25.201 fetch POST /api/observability/v1/traces
19:08:25.201 fetched POST /api/observability/v1/traces 204
19:08:25.979 chunk 17 42506 bytes
19:08:25.979 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:08:25.980 in Ack 17
19:08:26.983 fetch POST /api/observability/v1/traces
19:08:26.983 fetched POST /api/observability/v1/traces 204
19:08:27.047 in Ping 
19:08:27.794 chunk 17 42544 bytes
19:08:27.796 in Ack 17
