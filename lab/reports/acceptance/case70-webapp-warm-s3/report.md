# case70, webapp, warm start, seed 3

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 79 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 9.7s |
| Prompt reached the agent | 8.4s |
| Time to first text | 10.6s |
| Whole reply shown | 78.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 27.8s / 6.9s / 23.9s |
| Longest silence | 20.6s |
| Approval shown after it was asked | 8.9s |
| Bytes on the wire down / up | 85167 / 45116 |
| Bytes the app received / sent | 50953 / 109190 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 0 | 65 |
| 20 | 4.7 | 275 | 465 |
| 30 | 9.9 | 465 | 870 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.3 | 1094 | 1266 |
| 60 | 11.9 | 1185 | 1596 |
| 70 | 21.9 | 1185 | 1596 |
| 78 | 27.8 | 1270 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774","messages":"456/456 546/546","providerThreads":"708/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164742240,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":7539.762300530712,"deliveredRate":3829.874421566347,"loss":0,"srttMs":3.707959779689185,"minRttMs":1,"sentPackets":118,"sentWireBytes":35260,"receivedPackets":110,"receivedWireBytes":50545,"lastHeardMs":9325,"unsyncedBytes":1186,"inflightPackets":18,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 160/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 5/5","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 616/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164752857,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":3769.881150265356,"deliveredRate":2115.327051206914,"loss":0.3316239316239316,"srttMs":1420.5973660720297,"minRttMs":1,"sentPackets":139,"sentWireBytes":47974,"receivedPackets":124,"receivedWireBytes":61432,"lastHeardMs":19942,"unsyncedBytes":67,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 731/849 978/1073","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 481/548","user":"10/10 9/9","attempts":"660/660 660/660 53/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164763042,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":4500,"deliveredRate":0,"loss":0.4721367521367521,"srttMs":1520.0182787605063,"minRttMs":1,"sentPackets":220,"sentWireBytes":67118,"receivedPackets":146,"receivedWireBytes":69316,"lastHeardMs":30128,"unsyncedBytes":133,"inflightPackets":78,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 911/963","user":"10/10 9/9 52/64","attempts":"660/660 660/660 106/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164773040,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":3375,"deliveredRate":635.9980300375832,"loss":0.6981176470588235,"srttMs":2484.0496272811133,"minRttMs":1,"sentPackets":280,"sentWireBytes":77426,"receivedPackets":180,"receivedWireBytes":79373,"lastHeardMs":40126,"unsyncedBytes":0,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 54/64","attempts":"660/660 660/660 106/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901 209/545","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"751/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164783209,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":3375,"deliveredRate":308.6081920749131,"loss":0.7647510562378796,"srttMs":2484.0496272811133,"minRttMs":1,"sentPackets":330,"sentWireBytes":84562,"receivedPackets":206,"receivedWireBytes":89357,"lastHeardMs":50294,"unsyncedBytes":0,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1228/1546","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340 218/268","user":"10/10 9/9 54/64","attempts":"660/660 660/660 222/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901 209/545","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"264/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164793830,"silences":0,"longestSilenceMs":1490,"longestRecoveredGapMs":1490},"stats":{"sendRate":1957.0744475198578,"deliveredRate":937.1510616080752,"loss":0.6408602150537634,"srttMs":2221.8048842286375,"minRttMs":1,"sentPackets":427,"sentWireBytes":118688,"receivedPackets":224,"receivedWireBytes":96550,"lastHeardMs":60916,"unsyncedBytes":0,"inflightPackets":147,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1228/1546","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340 218/268","user":"10/10 9/9 54/64","attempts":"660/660 660/660 222/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901 209/545","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"264/1048"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164793830,"silences":0,"longestSilenceMs":1490,"longestRecoveredGapMs":1490},"stats":{"sendRate":1957.0744475198578,"deliveredRate":937.1510616080752,"loss":0.6408602150537634,"srttMs":2221.8048842286375,"minRttMs":1,"sentPackets":435,"sentWireBytes":119832,"receivedPackets":224,"receivedWireBytes":96550,"lastHeardMs":60916,"unsyncedBytes":0,"inflightPackets":128,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1228/1546 39/1098","thread":"951/951","runs":"1014/1014 971/971 54/839","assistant":"32/32 21/21 1340/1340 545/723","user":"10/10 9/9 54/64","attempts":"660/660 660/660 222/636","providerTurns":"774/774 774/774","messages":"456/456 546/546 546/546 901/901 209/545","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 708/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":232,"lastDataAt":1794164816117,"silences":2,"longestSilenceMs":17556,"longestRecoveredGapMs":17556},"stats":{"sendRate":2935.6116712797866,"deliveredRate":0,"loss":0.6408602150537634,"srttMs":2370.548975176607,"minRttMs":1019,"sentPackets":479,"sentWireBytes":132170,"receivedPackets":231,"receivedWireBytes":99478,"lastHeardMs":83202,"unsyncedBytes":317,"inflightPackets":149,"duplicateBytes":0}}
- shim trace
19:05:33.204 fetch GET /api/auth/session
19:05:33.205 fetched GET /api/auth/session 200
19:05:33.340 fetch GET /.well-known/t3/environment
19:05:33.341 fetched GET /.well-known/t3/environment 200
19:05:33.349 fetch GET /.well-known/t3/environment
19:05:33.350 fetched GET /.well-known/t3/environment 200
19:05:33.353 rpc 0 subscribeServerConfig
19:05:33.354 chunk 0 38722 bytes
19:05:33.376 in Ack 0
19:05:33.377 rpc 1 subscribeServerLifecycle
19:05:33.379 rpc 2 subscribeServerLifecycle
19:05:33.381 fetch GET /api/auth/session
19:05:33.382 fetched GET /api/auth/session 200
19:05:33.382 rpc 3 subscribeDeviceState
19:05:33.384 rpc 4 preview.reportProfiles
19:05:33.429 rpc 5 subscribeProjectClones
19:05:33.445 fetch GET /api/orchestration/threads/mcp%3Ad25befbe-3090-4aa4-acac-21f3d4e2fd5d/bounded
19:05:33.450 fetch GET /api/orchestration/shell
19:05:33.469 rpc 6 server.refreshProviders
19:05:33.508 rpc 7 orchestration.dispatchCommand thread.visit
19:05:33.508 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:05:33.518 fetched GET /api/orchestration/shell 200
19:05:33.518 rpc 8 subscribeTerminalMetadata
19:05:33.519 rpc 9 subscribeVcsStatus
19:05:33.519 rpc 10 subscribePreviewEvents
19:05:33.521 rpc 11 orchestration.subscribeShell
19:05:33.521 chunk 11 2790 bytes
19:05:33.522 rpc 12 assets.createUrl
19:05:33.523 rpc 13 preview.list
19:05:33.523 rpc 14 projects.readFile
19:05:33.527 in Ack 11
19:05:33.770 rpc 15 server.reportClientActivity
19:05:33.801 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"78b662a3-e291-4c72-ade8-8e882800a30b","sessions":[]}}
19:05:33.801 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:05:34.057 chunk 1 3872 bytes
19:05:34.057 in Ack 1
19:05:34.279 chunk 2 3872 bytes
19:05:34.279 in Ack 2
19:05:34.339 fetch POST /api/observability/v1/traces
19:05:34.339 fetched POST /api/observability/v1/traces 204
19:05:34.499 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:05:21.624Z","configurableProviders":false,"con
19:05:34.499 chunk 9 882 bytes
19:05:34.499 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:05:34.502 in Ack 9
19:05:34.699 chunk 11 2766 bytes
19:05:34.699 in Ack 11
19:05:34.950 chunk 11 2809 bytes
19:05:34.950 in Ack 11
19:05:34.977 chunk 3 876 bytes
19:05:34.977 exit 4 {"_tag":"Success","value":null}
19:05:34.977 chunk 5 4 bytes
19:05:34.977 chunk 8 36 bytes
19:05:34.977 exit 15 {"_tag":"Success","value":null}
19:05:34.978 in Ack 3
19:05:34.981 rpc 16 preview.reportProfiles
19:05:34.981 in Ack 5
19:05:34.981 in Ack 8
19:05:35.082 exit 16 {"_tag":"Success","value":null}
19:05:35.500 fetch POST /api/observability/v1/traces
19:05:35.500 fetched POST /api/observability/v1/traces 204
19:05:35.508 fetched GET /api/orchestration/threads/mcp%3Ad25befbe-3090-4aa4-acac-21f3d4e2fd5d/bounded 200
19:05:35.511 rpc 17 orchestration.subscribeThread
19:05:35.512 chunk 17 8283 bytes
19:05:35.521 in Interrupt 9
19:05:35.523 rpc 18 subscribeVcsStatus
19:05:35.524 in Ack 17
19:05:35.658 chunk 18 882 bytes
19:05:35.659 in Ack 18
19:05:35.763 chunk 17 11260 bytes
19:05:35.764 in Ack 17
19:05:35.791 rpc 19 server.reportClientActivity
19:05:36.029 chunk 17 12694 bytes
19:05:36.030 in Ack 17
19:05:36.051 exit 19 {"_tag":"Success","value":null}
19:05:36.222 rpc 20 attachments.createUploadUrl
19:05:36.377 chunk 17 13502 bytes
19:05:36.388 in Ack 17
19:05:36.462 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-021ce9b7-6209-4b89-8ba1-ae8334f394d2","expiresAt":1794165336408,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:05:36.463 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTAyMWNlOWI3LTYyMDktNGI4OS04YmExLWFlODMzNGYzOTRkMiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NTMzNjQwOH0.7Y8CeAP9VmYBkKkXe-BjUoOZH1k-dj0nOkzfk0vlOXI
19:05:36.509 fetch POST /api/observability/v1/traces
19:05:36.509 fetched POST /api/observability/v1/traces 204
19:05:36.628 chunk 17 16198 bytes
19:05:36.629 in Ack 17
19:05:36.879 chunk 17 19851 bytes
19:05:36.880 in Ack 17
19:05:36.887 in Interrupt 18
19:05:36.888 rpc 21 subscribeVcsStatus
19:05:36.940 chunk 21 882 bytes
19:05:36.940 in Ack 21
19:05:37.136 chunk 17 21199 bytes
19:05:37.139 in Ack 17
19:05:37.146 rpc 22 server.reportClientActivity
19:05:37.630 fetch POST /api/observability/v1/traces
19:05:37.630 fetched POST /api/observability/v1/traces 204
19:05:38.320 exit 22 {"_tag":"Success","value":null}
19:05:38.352 in Ping 
19:05:39.400 rpc 23 orchestration.dispatchCommand message.dispatch
19:05:39.924 fetch POST /api/observability/v1/traces
19:05:39.924 fetched POST /api/observability/v1/traces 204
19:05:39.979 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:05:39.981 rpc 24 attachments.delete
19:05:40.077 exit 24 {"_tag":"Success","value":null}
19:05:40.242 chunk 17 23633 bytes
19:05:40.244 in Ack 17
19:05:40.494 chunk 17 23653 bytes
19:05:40.495 in Ack 17
19:05:40.744 chunk 17 23667 bytes
19:05:40.745 in Ack 17
19:05:40.980 fetch POST /api/observability/v1/traces
19:05:40.980 fetched POST /api/observability/v1/traces 204
19:05:41.244 chunk 17 28093 bytes
19:05:41.246 in Ack 17
19:05:41.248 rpc 25 assets.createUrl
19:05:41.307 chunk 11 2831 bytes
19:05:41.309 in Ack 11
19:05:41.310 rpc 26 orchestration.dispatchCommand thread.visit
19:05:41.310 exit 26 {"_tag":"Success","value":{"sequence":0}}
19:05:41.494 chunk 17 30516 bytes
19:05:41.495 in Ack 17
19:05:41.745 chunk 17 31326 bytes
19:05:41.745 in Ack 17
19:05:41.808 chunk 11 2831 bytes
19:05:41.808 in Ack 11
19:05:42.220 chunk 17 31940 bytes
19:05:42.222 in Ack 17
19:05:42.229 in Interrupt 21
19:05:42.231 rpc 27 subscribeVcsStatus
19:05:42.247 fetch POST /api/observability/v1/traces
19:05:42.247 fetched POST /api/observability/v1/traces 204
19:05:42.482 rpc 28 server.reportClientActivity
19:05:42.794 rpc 29 orchestration.dispatchCommand message.dispatch
19:05:43.352 in Ping 
19:05:43.482 fetch POST /api/observability/v1/traces
19:05:43.482 fetched POST /api/observability/v1/traces 204
19:05:44.710 chunk 27 882 bytes
19:05:44.710 in Ack 27
19:05:45.026 fetch POST /api/observability/v1/traces
19:05:45.026 fetched POST /api/observability/v1/traces 204
19:05:47.041 fetch POST /api/observability/v1/traces
19:05:47.041 fetched POST /api/observability/v1/traces 204
19:05:47.288 chunk 17 32851 bytes
19:05:47.289 in Ack 17
19:05:48.290 fetch POST /api/observability/v1/traces
19:05:48.290 fetched POST /api/observability/v1/traces 204
19:05:48.352 in Ping 
19:05:49.513 chunk 17 33419 bytes
19:05:49.515 in Ack 17
19:05:50.515 fetch POST /api/observability/v1/traces
19:05:50.515 fetched POST /api/observability/v1/traces 204
19:05:50.593 chunk 17 34194 bytes
19:05:50.594 in Ack 17
19:05:51.595 fetch POST /api/observability/v1/traces
19:05:51.595 fetched POST /api/observability/v1/traces 204
19:05:52.425 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:05:52.858 chunk 17 36628 bytes
19:05:52.859 in Ack 17
19:05:53.293 chunk 17 36678 bytes
19:05:53.295 in Ack 17
19:05:53.352 in Ping 
19:05:53.426 fetch POST /api/observability/v1/traces
19:05:53.426 fetched POST /api/observability/v1/traces 204
19:05:53.810 chunk 17 36728 bytes
19:05:53.811 in Ack 17
19:05:54.729 chunk 17 36780 bytes
19:05:54.731 in Ack 17
19:05:54.812 fetch POST /api/observability/v1/traces
19:05:54.812 fetched POST /api/observability/v1/traces 204
19:05:55.167 chunk 17 36800 bytes
19:05:55.168 in Ack 17
19:05:55.604 chunk 17 36830 bytes
19:05:55.605 in Ack 17
19:05:56.124 chunk 17 36882 bytes
19:05:56.125 in Ack 17
19:05:56.169 fetch POST /api/observability/v1/traces
19:05:56.169 fetched POST /api/observability/v1/traces 204
19:05:56.677 chunk 17 36982 bytes
19:05:56.679 in Ack 17
19:05:56.957 chunk 17 37014 bytes
19:05:56.958 in Ack 17
19:05:57.482 in Interrupt 28
19:05:57.680 fetch POST /api/observability/v1/traces
19:05:57.680 fetched POST /api/observability/v1/traces 204
19:05:57.969 chunk 17 37114 bytes
19:05:57.970 in Ack 17
19:05:58.353 in Ping 
19:05:58.592 rpc 30 server.reportClientActivity
19:05:58.611 chunk 17 37212 bytes
19:05:58.612 in Ack 17
19:05:58.971 fetch POST /api/observability/v1/traces
19:05:58.971 fetched POST /api/observability/v1/traces 204
19:05:59.523 chunk 17 37264 bytes
19:05:59.524 in Ack 17
19:05:59.952 chunk 17 37274 bytes
19:05:59.954 in Ack 17
19:06:00.506 chunk 17 37324 bytes
19:06:00.507 in Ack 17
19:06:00.536 fetch POST /api/observability/v1/traces
19:06:00.536 fetched POST /api/observability/v1/traces 204
19:06:00.756 chunk 17 37334 bytes
19:06:00.757 in Ack 17
19:06:01.013 chunk 17 37344 bytes
19:06:01.015 in Ack 17
19:06:01.269 chunk 17 37364 bytes
19:06:01.270 in Ack 17
19:06:01.524 chunk 17 37384 bytes
19:06:01.527 in Ack 17
19:06:01.759 fetch POST /api/observability/v1/traces
19:06:01.759 fetched POST /api/observability/v1/traces 204
19:06:02.116 chunk 17 37396 bytes
19:06:02.117 in Ack 17
19:06:02.371 chunk 17 37408 bytes
19:06:02.372 in Ack 17
19:06:02.783 chunk 17 37430 bytes
19:06:02.784 in Ack 17
19:06:03.043 chunk 17 37588 bytes
19:06:03.045 in Ack 17
19:06:03.119 fetch POST /api/observability/v1/traces
19:06:03.119 fetched POST /api/observability/v1/traces 204
19:06:03.299 chunk 17 37598 bytes
19:06:03.300 in Ack 17
19:06:03.353 in Ping 
19:06:03.557 chunk 17 37628 bytes
19:06:03.559 in Ack 17
19:06:04.153 chunk 17 37638 bytes
19:06:04.155 in Ack 17
19:06:04.301 fetch POST /api/observability/v1/traces
19:06:04.301 fetched POST /api/observability/v1/traces 204
19:06:04.409 chunk 17 37610 bytes
19:06:04.410 in Ack 17
19:06:04.920 chunk 17 37650 bytes
19:06:04.921 in Ack 17
19:06:05.410 fetch POST /api/observability/v1/traces
19:06:05.411 fetched POST /api/observability/v1/traces 204
19:06:05.573 chunk 17 37682 bytes
19:06:05.574 in Ack 17
19:06:06.167 chunk 17 37704 bytes
19:06:06.168 in Ack 17
19:06:06.575 fetch POST /api/observability/v1/traces
19:06:06.575 fetched POST /api/observability/v1/traces 204
19:06:06.590 chunk 17 37726 bytes
19:06:06.591 in Ack 17
19:06:06.930 chunk 17 37736 bytes
19:06:06.932 in Ack 17
19:06:07.272 chunk 17 37758 bytes
19:06:07.274 in Ack 17
19:06:07.591 fetch POST /api/observability/v1/traces
19:06:07.591 fetched POST /api/observability/v1/traces 204
19:06:07.937 chunk 17 37824 bytes
19:06:07.939 in Ack 17
19:06:08.195 chunk 17 37854 bytes
19:06:08.197 in Ack 17
19:06:08.353 in Ping 
19:06:08.709 chunk 17 37876 bytes
19:06:08.711 in Ack 17
19:06:08.939 fetch POST /api/observability/v1/traces
19:06:08.939 fetched POST /api/observability/v1/traces 204
19:06:09.210 chunk 17 37916 bytes
19:06:09.211 in Ack 17
19:06:09.462 chunk 17 37936 bytes
19:06:09.463 in Ack 17
19:06:09.717 chunk 17 37948 bytes
19:06:09.718 in Ack 17
19:06:09.972 chunk 17 37958 bytes
19:06:09.973 in Ack 17
19:06:10.212 fetch POST /api/observability/v1/traces
19:06:10.212 fetched POST /api/observability/v1/traces 204
19:06:10.224 chunk 17 37998 bytes
19:06:10.226 in Ack 17
19:06:10.562 chunk 17 38020 bytes
19:06:10.563 in Ack 17
19:06:10.820 chunk 17 38060 bytes
19:06:10.821 in Ack 17
19:06:11.155 chunk 17 38080 bytes
19:06:11.156 in Ack 17
19:06:11.227 fetch POST /api/observability/v1/traces
19:06:11.227 fetched POST /api/observability/v1/traces 204
19:06:11.406 chunk 17 38120 bytes
19:06:11.407 in Ack 17
19:06:11.829 chunk 17 38222 bytes
19:06:11.831 in Ack 17
19:06:12.282 chunk 17 38318 bytes
19:06:12.283 in Ack 17
19:06:12.408 fetch POST /api/observability/v1/traces
19:06:12.408 fetched POST /api/observability/v1/traces 204
19:06:12.537 chunk 17 38348 bytes
19:06:12.538 in Ack 17
19:06:12.787 chunk 17 38376 bytes
19:06:12.789 in Ack 17
19:06:13.041 chunk 17 38406 bytes
19:06:13.043 in Ack 17
19:06:13.299 chunk 17 38416 bytes
19:06:13.300 in Ack 17
19:06:13.353 in Ping 
19:06:13.539 fetch POST /api/observability/v1/traces
19:06:13.539 fetched POST /api/observability/v1/traces 204
19:06:13.593 in Interrupt 30
19:06:13.631 chunk 17 38436 bytes
19:06:13.632 in Ack 17
19:06:13.881 chunk 17 38454 bytes
19:06:13.882 in Ack 17
19:06:14.301 chunk 17 38474 bytes
19:06:14.303 in Ack 17
19:06:14.594 fetch POST /api/observability/v1/traces
19:06:14.594 fetched POST /api/observability/v1/traces 204
19:06:14.725 chunk 17 38564 bytes
19:06:14.726 in Ack 17
19:06:15.141 chunk 17 38594 bytes
19:06:15.143 in Ack 17
19:06:15.393 chunk 17 38604 bytes
19:06:15.395 in Ack 17
19:06:15.643 chunk 17 38624 bytes
19:06:15.645 in Ack 17
19:06:15.727 fetch POST /api/observability/v1/traces
19:06:15.727 fetched POST /api/observability/v1/traces 204
19:06:16.154 chunk 17 38664 bytes
19:06:16.155 in Ack 17
19:06:17.050 fetch POST /api/observability/v1/traces
19:06:17.050 fetched POST /api/observability/v1/traces 204
19:06:17.089 chunk 17 38706 bytes
19:06:17.090 in Ack 17
19:06:17.524 chunk 17 38822 bytes
19:06:17.525 in Ack 17
19:06:17.951 chunk 17 38844 bytes
19:06:17.952 in Ack 17
19:06:18.093 fetch POST /api/observability/v1/traces
19:06:18.093 fetched POST /api/observability/v1/traces 204
19:06:18.354 in Ping 
19:06:18.382 chunk 17 38864 bytes
19:06:18.384 in Ack 17
19:06:18.817 chunk 17 38914 bytes
19:06:18.818 in Ack 17
19:06:19.072 chunk 17 38984 bytes
19:06:19.073 in Ack 17
19:06:19.327 chunk 17 39004 bytes
19:06:19.328 in Ack 17
19:06:19.384 fetch POST /api/observability/v1/traces
19:06:19.384 fetched POST /api/observability/v1/traces 204
19:06:19.586 chunk 17 39024 bytes
19:06:19.588 in Ack 17
19:06:20.114 chunk 17 39070 bytes
19:06:20.115 in Ack 17
19:06:20.590 fetch POST /api/observability/v1/traces
19:06:20.590 fetched POST /api/observability/v1/traces 204
19:06:20.778 chunk 17 39246 bytes
19:06:20.779 in Ack 17
19:06:21.027 chunk 17 39258 bytes
19:06:21.028 in Ack 17
19:06:21.779 fetch POST /api/observability/v1/traces
19:06:21.779 fetched POST /api/observability/v1/traces 204
19:06:23.354 in Ping 
19:06:23.595 rpc 31 server.reportClientActivity
19:06:24.595 fetch POST /api/observability/v1/traces
19:06:24.595 fetched POST /api/observability/v1/traces 204
19:06:26.758 chunk 17 40263 bytes
19:06:26.760 in Ack 17
19:06:26.955 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:06:27.053 fetch POST /api/observability/v1/traces
19:06:27.053 fetched POST /api/observability/v1/traces 204
19:06:28.357 in Ping 
19:06:29.050 chunk 17 42733 bytes
19:06:29.050 exit 32 {"_tag":"Success","value":{"sequence":95}}
19:06:29.051 in Ack 17
19:06:29.434 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:06:29.484 chunk 17 42773 bytes
19:06:29.486 in Ack 17
19:06:30.052 fetch POST /api/observability/v1/traces
19:06:30.052 fetched POST /api/observability/v1/traces 204
19:06:30.968 chunk 17 42815 bytes
19:06:30.970 in Ack 17
19:06:31.492 chunk 17 42855 bytes
19:06:31.493 in Ack 17
19:06:31.971 fetch POST /api/observability/v1/traces
19:06:31.971 fetched POST /api/observability/v1/traces 204
19:06:32.013 chunk 17 42885 bytes
19:06:32.014 in Ack 17
19:06:33.014 fetch POST /api/observability/v1/traces
19:06:33.015 fetched POST /api/observability/v1/traces 204
19:06:33.361 in Ping 
19:06:33.503 chunk 17 43115 bytes
19:06:33.504 in Ack 17
19:06:33.831 chunk 17 43125 bytes
19:06:33.832 in Ack 17
19:06:34.505 fetch POST /api/observability/v1/traces
19:06:34.505 fetched POST /api/observability/v1/traces 204
19:06:37.056 fetch POST /api/observability/v1/traces
19:06:37.056 fetched POST /api/observability/v1/traces 204
19:06:38.365 in Ping 
19:06:38.595 in Interrupt 31
19:06:39.597 fetch POST /api/observability/v1/traces
19:06:39.597 fetched POST /api/observability/v1/traces 204
19:06:43.369 in Ping 
19:06:48.373 in Ping 
19:06:48.595 rpc 34 server.reportClientActivity
19:06:49.595 fetch POST /api/observability/v1/traces
19:06:49.595 fetched POST /api/observability/v1/traces 204
19:06:51.387 chunk 17 43387 bytes
19:06:51.389 in Ack 17
19:06:51.640 chunk 17 43407 bytes
19:06:51.641 in Ack 17
19:06:52.063 chunk 17 43517 bytes
19:06:52.065 in Ack 17
19:06:52.391 fetch POST /api/observability/v1/traces
19:06:52.391 fetched POST /api/observability/v1/traces 204
19:06:53.377 in Ping 
19:06:54.071 chunk 17 43658 bytes
19:06:54.071 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:06:54.072 in Ack 17
19:06:55.073 fetch POST /api/observability/v1/traces
19:06:55.073 fetched POST /api/observability/v1/traces 204
19:06:55.124 chunk 17 43774 bytes
19:06:55.125 in Ack 17
19:06:55.780 chunk 17 43860 bytes
19:06:55.782 in Ack 17
19:06:56.126 fetch POST /api/observability/v1/traces
19:06:56.126 fetched POST /api/observability/v1/traces 204
19:06:57.632 chunk 17 44120 bytes
19:06:57.633 in Ack 17
19:06:58.381 in Ping 
19:06:58.634 fetch POST /api/observability/v1/traces
19:06:58.634 fetched POST /api/observability/v1/traces 204
19:07:00.597 chunk 17 44230 bytes
19:07:00.598 in Ack 17
