# case70, webapp, warm start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 76 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 12.1s |
| Prompt reached the agent | 11.4s |
| Time to first text | 26.0s |
| Whole reply shown | 75.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 22.6s / 6.5s / 18.6s |
| Longest silence | 26.0s |
| Approval shown after it was asked | 17.3s |
| Bytes on the wire down / up | 70997 / 39474 |
| Bytes the app received / sent | 56552 / 82784 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 8.5 | 0 | 341 |
| 30 | 17.6 | 37 | 742 |
| 40 | 7.7 | 835 | 1077 |
| 50 | 13.0 | 1023 | 1077 |
| 60 | 3.4 | 1077 | 1208 |
| 70 | 8.5 | 1270 | 1571 |
| 75 | 13.5 | 1270 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"658/658 658/658","providerTurns":"770/770","providerThreads":"450/808","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809","messages":"456/456 542/542 63/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163614204,"silences":0,"longestSilenceMs":1101,"longestRecoveredGapMs":1101},"stats":{"sendRate":5816.812569484811,"deliveredRate":3120.0500382955192,"loss":0,"srttMs":3.792235202499054,"minRttMs":1,"sentPackets":124,"sentWireBytes":35854,"receivedPackets":132,"receivedWireBytes":54402,"lastHeardMs":10154,"unsyncedBytes":1180,"inflightPackets":19,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"658/658 658/658","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163620368,"silences":0,"longestSilenceMs":1101,"longestRecoveredGapMs":1101},"stats":{"sendRate":2908.4062847424057,"deliveredRate":2373.3207410925775,"loss":0.15,"srttMs":476.9213606922991,"minRttMs":1,"sentPackets":135,"sentWireBytes":42801,"receivedPackets":141,"receivedWireBytes":60457,"lastHeardMs":16318,"unsyncedBytes":891,"inflightPackets":23,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 160/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 220/400","user":"10/10 9/9","attempts":"658/658 658/658","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163635165,"silences":1,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":3000,"deliveredRate":1683.396887185857,"loss":0.38357142857142856,"srttMs":705.1585417800414,"minRttMs":1349,"sentPackets":163,"sentWireBytes":53562,"receivedPackets":150,"receivedWireBytes":64535,"lastHeardMs":31115,"unsyncedBytes":299,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 511/849 526/1063","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 781/863","user":"10/10 9/9","attempts":"658/658 658/658 71/634","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163645236,"silences":2,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":4096.330779942409,"deliveredRate":1174.5542764207864,"loss":0.42628333333333335,"srttMs":2473.321037734458,"minRttMs":1349,"sentPackets":241,"sentWireBytes":74280,"receivedPackets":171,"receivedWireBytes":72092,"lastHeardMs":41186,"unsyncedBytes":0,"inflightPackets":97,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 801/849 988/1063","thread":"951/951","runs":"1014/1014 971/971 49/839","assistant":"32/32 21/21 1143/1193","user":"10/10 9/9","attempts":"658/658 658/658 338/634","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163655120,"silences":2,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.5674586725766717,"srttMs":2974.7040555665117,"minRttMs":1349,"sentPackets":296,"sentWireBytes":83405,"receivedPackets":206,"receivedWireBytes":82743,"lastHeardMs":51070,"unsyncedBytes":497,"inflightPackets":90,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 849/849 1063/1063 224/1538","thread":"951/951","runs":"1014/1014 971/971 49/839","assistant":"32/32 21/21 1328/1340","user":"10/10 9/9 48/64","attempts":"658/658 658/658 338/634","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"801/999"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163665223,"silences":2,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":1000,"deliveredRate":275.2895752895753,"loss":0.5454545454545454,"srttMs":4547.774032572069,"minRttMs":1349,"sentPackets":342,"sentWireBytes":96457,"receivedPackets":228,"receivedWireBytes":93722,"lastHeardMs":61173,"unsyncedBytes":316,"inflightPackets":131,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 849/849 1063/1063 1129/1538 48/920","thread":"951/951","runs":"1014/1014 971/971 49/839","assistant":"32/32 21/21 1340/1340 98/158","user":"10/10 9/9 48/64","attempts":"658/658 658/658 519/634","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"188/1042"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163675148,"silences":2,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":529.7395517570156,"deliveredRate":270.77829871004116,"loss":0.6788113038277512,"srttMs":3727.9929780694924,"minRttMs":1349,"sentPackets":368,"sentWireBytes":104096,"receivedPackets":253,"receivedWireBytes":103822,"lastHeardMs":71098,"unsyncedBytes":397,"inflightPackets":129,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1028/1028","turnItems":"1058/1058 1088/1088 870/870 860/860 1299/1299 1088/1088 1058/1058 849/849 1063/1063 1129/1538 48/920","thread":"951/951","runs":"1014/1014 971/971 49/839","assistant":"32/32 21/21 1340/1340 630/723","user":"10/10 9/9 48/64","attempts":"658/658 658/658 519/634","providerTurns":"770/770","providerThreads":"532/806","providerSessions":"2693/2693","nodes":"1013/1013 809/809 809/809 946/1013","messages":"456/456 542/542 145/901 478/542","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"410/1042"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":106,"lastDataAt":1794163686448,"silences":3,"longestSilenceMs":6623,"longestRecoveredGapMs":6623},"stats":{"sendRate":662.1744396962696,"deliveredRate":217.71511979898509,"loss":0.7108821983937115,"srttMs":2658.244907310755,"minRttMs":1009,"sentPackets":389,"sentWireBytes":110547,"receivedPackets":265,"receivedWireBytes":108290,"lastHeardMs":82398,"unsyncedBytes":0,"inflightPackets":132,"duplicateBytes":0}}
- Scroll to end shows
- shim trace
18:46:44.380 fetch GET /api/auth/session
18:46:44.382 fetched GET /api/auth/session 200
18:46:44.491 fetch GET /.well-known/t3/environment
18:46:44.491 fetched GET /.well-known/t3/environment 200
18:46:44.500 fetch GET /.well-known/t3/environment
18:46:44.500 fetched GET /.well-known/t3/environment 200
18:46:44.504 rpc 0 subscribeServerConfig
18:46:44.504 chunk 0 38710 bytes
18:46:44.519 in Ack 0
18:46:44.520 rpc 1 subscribeServerLifecycle
18:46:44.521 rpc 2 subscribeServerLifecycle
18:46:44.524 fetch GET /api/auth/session
18:46:44.524 fetched GET /api/auth/session 200
18:46:44.525 rpc 3 subscribeDeviceState
18:46:44.526 rpc 4 preview.reportProfiles
18:46:44.563 rpc 5 subscribeProjectClones
18:46:44.580 fetch GET /api/orchestration/threads/mcp%3A77bfe0f6-eba3-44d6-ac4c-29825f75bcae/bounded
18:46:44.583 fetch GET /api/orchestration/shell
18:46:44.584 fetched GET /api/orchestration/shell 200
18:46:44.586 rpc 6 orchestration.subscribeShell
18:46:44.586 chunk 6 2787 bytes
18:46:44.603 rpc 7 server.refreshProviders
18:46:44.643 rpc 8 orchestration.dispatchCommand thread.visit
18:46:44.643 exit 8 {"_tag":"Success","value":{"sequence":0}}
18:46:44.657 rpc 9 subscribeTerminalMetadata
18:46:44.658 rpc 10 subscribeVcsStatus
18:46:44.658 rpc 11 subscribePreviewEvents
18:46:44.659 rpc 12 assets.createUrl
18:46:44.660 rpc 13 preview.list
18:46:44.661 rpc 14 projects.readFile
18:46:44.662 in Ack 6
18:46:44.891 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"29adf4c0-1d21-4097-bc3d-e3abd8eb701e","sessions":[]}}
18:46:44.892 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
18:46:44.911 rpc 15 server.reportClientActivity
18:46:45.139 chunk 1 3869 bytes
18:46:45.140 in Ack 1
18:46:45.368 chunk 2 3869 bytes
18:46:45.369 in Ack 2
18:46:45.489 fetch POST /api/observability/v1/traces
18:46:45.489 fetched POST /api/observability/v1/traces 204
18:46:45.589 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T18:46:32.254Z","configurableProviders":false,"con
18:46:45.589 chunk 10 630 bytes
18:46:45.589 exit 12 {"_tag":"Success","value":{"expiresAt":1794166200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
18:46:45.592 in Ack 10
18:46:45.789 chunk 6 2763 bytes
18:46:45.790 in Ack 6
18:46:46.039 chunk 6 2806 bytes
18:46:46.039 in Ack 6
18:46:46.177 chunk 3 876 bytes
18:46:46.177 exit 4 {"_tag":"Success","value":null}
18:46:46.177 chunk 5 4 bytes
18:46:46.177 chunk 9 36 bytes
18:46:46.178 in Ack 3
18:46:46.182 rpc 16 preview.reportProfiles
18:46:46.182 in Ack 5
18:46:46.183 in Ack 9
18:46:46.258 exit 15 {"_tag":"Success","value":null}
18:46:46.258 exit 16 {"_tag":"Success","value":null}
18:46:46.591 fetch POST /api/observability/v1/traces
18:46:46.591 fetched POST /api/observability/v1/traces 204
18:46:46.837 fetched GET /api/orchestration/threads/mcp%3A77bfe0f6-eba3-44d6-ac4c-29825f75bcae/bounded 200
18:46:46.841 rpc 17 orchestration.subscribeThread
18:46:46.841 chunk 17 8261 bytes
18:46:46.852 in Interrupt 10
18:46:46.853 rpc 18 subscribeVcsStatus
18:46:46.854 in Ack 17
18:46:46.978 chunk 18 630 bytes
18:46:46.978 in Ack 18
18:46:47.092 chunk 17 11238 bytes
18:46:47.093 in Ack 17
18:46:47.122 rpc 19 server.reportClientActivity
18:46:47.341 chunk 17 12666 bytes
18:46:47.344 in Ack 17
18:46:47.448 exit 19 {"_tag":"Success","value":null}
18:46:47.643 chunk 17 13472 bytes
18:46:47.645 in Ack 17
18:46:47.865 rpc 20 attachments.createUploadUrl
18:46:47.872 fetch POST /api/observability/v1/traces
18:46:47.872 fetched POST /api/observability/v1/traces 204
18:46:48.026 chunk 17 18562 bytes
18:46:48.034 in Ack 17
18:46:48.078 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-69e525d0-fc24-4394-8a96-23beeedcbd3e","expiresAt":1794164208058,"relativeUrl":"/api/attachments/upload/eyJ2ZX
18:46:48.079 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTY5ZTUyNWQwLWZjMjQtNDM5NC04YTk2LTIzYmVlZWRjYmQzZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NDIwODA1OH0.waedgvqucOnsDnIxPYiJvxkkyl7vghp9V6vjQmwmisA
18:46:48.276 chunk 17 19808 bytes
18:46:48.277 in Ack 17
18:46:48.284 in Interrupt 18
18:46:48.285 rpc 21 subscribeVcsStatus
18:46:48.389 chunk 21 630 bytes
18:46:48.389 in Ack 21
18:46:48.526 chunk 17 21153 bytes
18:46:48.528 in Ack 17
18:46:48.537 rpc 22 server.reportClientActivity
18:46:49.037 fetch POST /api/observability/v1/traces
18:46:49.037 fetched POST /api/observability/v1/traces 204
18:46:49.503 in Ping 
18:46:50.627 exit 22 {"_tag":"Success","value":null}
18:46:51.348 rpc 23 orchestration.dispatchCommand message.dispatch
18:46:51.909 fetch POST /api/observability/v1/traces
18:46:51.909 fetched POST /api/observability/v1/traces 204
18:46:51.967 exit 23 {"_tag":"Success","value":{"sequence":45}}
18:46:51.969 rpc 24 attachments.delete
18:46:52.253 chunk 17 23565 bytes
18:46:52.254 in Ack 17
18:46:52.503 chunk 17 23585 bytes
18:46:52.504 in Ack 17
18:46:52.755 chunk 17 23549 bytes
18:46:52.756 in Ack 17
18:46:52.969 fetch POST /api/observability/v1/traces
18:46:52.969 fetched POST /api/observability/v1/traces 204
18:46:53.021 chunk 17 23599 bytes
18:46:53.022 in Ack 17
18:46:53.271 chunk 6 2828 bytes
18:46:53.272 in Ack 6
18:46:53.273 rpc 25 orchestration.dispatchCommand thread.visit
18:46:53.273 exit 25 {"_tag":"Success","value":{"sequence":0}}
18:46:53.522 chunk 17 27364 bytes
18:46:53.523 in Ack 17
18:46:53.526 rpc 26 assets.createUrl
18:46:53.772 chunk 6 2828 bytes
18:46:53.772 chunk 17 28023 bytes
18:46:53.773 in Ack 6
18:46:53.778 in Ack 17
18:46:53.789 exit 24 {"_tag":"Success","value":null}
18:46:54.205 fetch POST /api/observability/v1/traces
18:46:54.205 fetched POST /api/observability/v1/traces 204
18:46:54.206 chunk 17 31870 bytes
18:46:54.207 in Ack 17
18:46:54.215 in Interrupt 21
18:46:54.217 rpc 27 subscribeVcsStatus
18:46:54.470 rpc 28 server.reportClientActivity
18:46:54.503 in Ping 
18:46:55.002 rpc 29 orchestration.dispatchCommand message.dispatch
18:46:55.208 fetch POST /api/observability/v1/traces
18:46:55.208 fetched POST /api/observability/v1/traces 204
18:46:56.840 chunk 17 31870 bytes
18:46:56.841 chunk 27 630 bytes
18:46:56.842 in Ack 17
18:46:56.851 in Ack 27
18:46:57.843 fetch POST /api/observability/v1/traces
18:46:57.843 fetched POST /api/observability/v1/traces 204
18:46:59.503 in Ping 
18:47:04.503 in Ping 
18:47:06.992 exit 29 {"_tag":"Success","value":{"sequence":75}}
18:47:07.840 chunk 17 34304 bytes
18:47:07.842 in Ack 17
18:47:07.995 fetch POST /api/observability/v1/traces
18:47:07.996 fetched POST /api/observability/v1/traces 204
18:47:08.274 chunk 17 34334 bytes
18:47:08.275 in Ack 17
18:47:09.306 fetch POST /api/observability/v1/traces
18:47:09.306 fetched POST /api/observability/v1/traces 204
18:47:09.471 in Interrupt 28
18:47:09.504 in Ping 
18:47:09.742 rpc 30 server.reportClientActivity
18:47:10.065 chunk 17 34386 bytes
18:47:10.069 in Ack 17
18:47:10.472 fetch POST /api/observability/v1/traces
18:47:10.472 fetched POST /api/observability/v1/traces 204
18:47:10.499 chunk 17 34406 bytes
18:47:10.500 in Ack 17
18:47:11.501 fetch POST /api/observability/v1/traces
18:47:11.501 fetched POST /api/observability/v1/traces 204
18:47:11.642 chunk 17 34458 bytes
18:47:11.644 in Ack 17
18:47:12.644 fetch POST /api/observability/v1/traces
18:47:12.644 fetched POST /api/observability/v1/traces 204
18:47:13.100 chunk 17 34510 bytes
18:47:13.101 in Ack 17
18:47:14.102 fetch POST /api/observability/v1/traces
18:47:14.102 fetched POST /api/observability/v1/traces 204
18:47:14.504 in Ping 
18:47:14.917 chunk 17 34708 bytes
18:47:14.918 in Ack 17
18:47:15.167 chunk 17 34718 bytes
18:47:15.168 in Ack 17
18:47:15.424 chunk 17 34834 bytes
18:47:15.425 in Ack 17
18:47:15.919 fetch POST /api/observability/v1/traces
18:47:15.919 fetched POST /api/observability/v1/traces 204
18:47:17.454 chunk 17 34994 bytes
18:47:17.455 in Ack 17
18:47:17.924 chunk 17 35168 bytes
18:47:17.925 in Ack 17
18:47:18.456 fetch POST /api/observability/v1/traces
18:47:18.456 fetched POST /api/observability/v1/traces 204
18:47:19.013 chunk 17 35208 bytes
18:47:19.014 in Ack 17
18:47:19.277 chunk 17 35250 bytes
18:47:19.278 in Ack 17
18:47:19.505 in Ping 
18:47:19.538 chunk 17 35260 bytes
18:47:19.539 in Ack 17
18:47:19.797 chunk 17 35308 bytes
18:47:19.799 in Ack 17
18:47:20.018 fetch POST /api/observability/v1/traces
18:47:20.018 fetched POST /api/observability/v1/traces 204
18:47:20.049 chunk 17 35348 bytes
18:47:20.050 in Ack 17
18:47:20.387 chunk 17 35462 bytes
18:47:20.388 in Ack 17
18:47:20.815 chunk 17 35520 bytes
18:47:20.816 in Ack 17
18:47:21.051 fetch POST /api/observability/v1/traces
18:47:21.051 fetched POST /api/observability/v1/traces 204
18:47:21.338 chunk 17 35540 bytes
18:47:21.339 in Ack 17
18:47:21.773 chunk 17 35580 bytes
18:47:21.774 in Ack 17
18:47:22.211 chunk 17 35590 bytes
18:47:22.212 in Ack 17
18:47:22.340 fetch POST /api/observability/v1/traces
18:47:22.340 fetched POST /api/observability/v1/traces 204
18:47:22.644 chunk 17 35612 bytes
18:47:22.645 in Ack 17
18:47:22.972 chunk 17 35710 bytes
18:47:22.973 in Ack 17
18:47:23.646 fetch POST /api/observability/v1/traces
18:47:23.646 fetched POST /api/observability/v1/traces 204
18:47:23.717 chunk 17 35772 bytes
18:47:23.719 in Ack 17
18:47:23.972 chunk 17 35802 bytes
18:47:23.973 in Ack 17
18:47:24.223 chunk 17 35812 bytes
18:47:24.224 in Ack 17
18:47:24.505 in Ping 
18:47:24.720 fetch POST /api/observability/v1/traces
18:47:24.720 fetched POST /api/observability/v1/traces 204
18:47:24.743 in Interrupt 30
18:47:24.812 chunk 17 35834 bytes
18:47:24.814 in Ack 17
18:47:25.237 chunk 17 35844 bytes
18:47:25.238 in Ack 17
18:47:25.575 chunk 17 35898 bytes
18:47:25.576 in Ack 17
18:47:25.744 fetch POST /api/observability/v1/traces
18:47:25.744 fetched POST /api/observability/v1/traces 204
18:47:25.825 chunk 17 35968 bytes
18:47:25.826 in Ack 17
18:47:26.076 chunk 17 35978 bytes
18:47:26.078 in Ack 17
18:47:26.497 chunk 17 35988 bytes
18:47:26.498 in Ack 17
18:47:26.825 chunk 17 36020 bytes
18:47:26.826 in Ack 17
18:47:26.835 fetch POST /api/observability/v1/traces
18:47:26.835 fetched POST /api/observability/v1/traces 204
18:47:27.253 chunk 17 36030 bytes
18:47:27.254 in Ack 17
18:47:27.597 chunk 17 36062 bytes
18:47:27.598 in Ack 17
18:47:27.850 chunk 17 36072 bytes
18:47:27.853 in Ack 17
18:47:28.104 chunk 17 36122 bytes
18:47:28.105 in Ack 17
18:47:28.255 fetch POST /api/observability/v1/traces
18:47:28.255 fetched POST /api/observability/v1/traces 204
18:47:28.359 chunk 17 36132 bytes
18:47:28.360 in Ack 17
18:47:28.610 chunk 17 36162 bytes
18:47:28.611 in Ack 17
18:47:28.860 chunk 17 36182 bytes
18:47:28.861 in Ack 17
18:47:29.195 chunk 17 36194 bytes
18:47:29.196 in Ack 17
18:47:29.361 fetch POST /api/observability/v1/traces
18:47:29.361 fetched POST /api/observability/v1/traces 204
18:47:29.506 in Ping 
18:47:29.591 chunk 17 36296 bytes
18:47:29.592 in Ack 17
18:47:29.849 chunk 17 36342 bytes
18:47:29.850 in Ack 17
18:47:30.166 chunk 17 36352 bytes
18:47:30.174 in Ack 17
18:47:30.414 chunk 17 36364 bytes
18:47:30.416 in Ack 17
18:47:30.593 fetch POST /api/observability/v1/traces
18:47:30.593 fetched POST /api/observability/v1/traces 204
18:47:30.665 chunk 17 36376 bytes
18:47:30.666 in Ack 17
18:47:31.128 chunk 17 36386 bytes
18:47:31.129 in Ack 17
18:47:31.383 chunk 17 36408 bytes
18:47:31.384 in Ack 17
18:47:31.667 fetch POST /api/observability/v1/traces
18:47:31.667 fetched POST /api/observability/v1/traces 204
18:47:31.894 chunk 17 36420 bytes
18:47:31.895 in Ack 17
18:47:32.149 chunk 17 36430 bytes
18:47:32.150 in Ack 17
18:47:32.404 chunk 17 36440 bytes
18:47:32.405 in Ack 17
18:47:32.659 chunk 17 36470 bytes
18:47:32.661 in Ack 17
18:47:32.896 fetch POST /api/observability/v1/traces
18:47:32.896 fetched POST /api/observability/v1/traces 204
18:47:33.249 chunk 17 36492 bytes
18:47:33.250 in Ack 17
18:47:33.504 chunk 17 36514 bytes
18:47:33.505 in Ack 17
18:47:33.759 chunk 17 36524 bytes
18:47:33.760 in Ack 17
18:47:34.103 chunk 17 36534 bytes
18:47:34.104 in Ack 17
18:47:34.251 fetch POST /api/observability/v1/traces
18:47:34.251 fetched POST /api/observability/v1/traces 204
18:47:34.355 chunk 17 36544 bytes
18:47:34.356 in Ack 17
18:47:34.506 in Ping 
18:47:34.611 chunk 17 36554 bytes
18:47:34.612 in Ack 17
18:47:34.742 rpc 31 server.reportClientActivity
18:47:34.869 chunk 17 36564 bytes
18:47:34.870 in Ack 17
18:47:35.121 chunk 17 36574 bytes
18:47:35.122 in Ack 17
18:47:35.357 fetch POST /api/observability/v1/traces
18:47:35.357 fetched POST /api/observability/v1/traces 204
18:47:35.458 chunk 17 36634 bytes
18:47:35.459 in Ack 17
18:47:35.716 chunk 17 36616 bytes
18:47:35.718 in Ack 17
18:47:35.966 chunk 17 36644 bytes
18:47:35.967 in Ack 17
18:47:36.217 chunk 17 36684 bytes
18:47:36.219 in Ack 17
18:47:36.460 fetch POST /api/observability/v1/traces
18:47:36.460 fetched POST /api/observability/v1/traces 204
18:47:36.591 chunk 17 36706 bytes
18:47:36.592 in Ack 17
18:47:36.841 chunk 17 36736 bytes
18:47:36.842 in Ack 17
18:47:37.092 chunk 17 36746 bytes
18:47:37.093 in Ack 17
18:47:37.343 chunk 17 36766 bytes
18:47:37.344 in Ack 17
18:47:37.592 chunk 17 36786 bytes
18:47:37.593 fetch POST /api/observability/v1/traces
18:47:37.593 fetched POST /api/observability/v1/traces 204
18:47:37.594 in Ack 17
18:47:38.544 chunk 17 36806 bytes
18:47:38.545 in Ack 17
18:47:38.595 fetch POST /api/observability/v1/traces
18:47:38.595 fetched POST /api/observability/v1/traces 204
18:47:39.509 in Ping 
18:47:39.695 chunk 17 36814 bytes
18:47:39.696 in Ack 17
18:47:40.697 fetch POST /api/observability/v1/traces
18:47:40.697 fetched POST /api/observability/v1/traces 204
18:47:41.886 chunk 17 36842 bytes
18:47:41.887 in Ack 17
18:47:42.887 fetch POST /api/observability/v1/traces
18:47:42.888 fetched POST /api/observability/v1/traces 204
18:47:43.541 chunk 17 36868 bytes
18:47:43.543 in Ack 17
18:47:44.511 in Ping 
18:47:44.544 fetch POST /api/observability/v1/traces
18:47:44.544 fetched POST /api/observability/v1/traces 204
18:47:45.224 chunk 17 36886 bytes
18:47:45.225 in Ack 17
18:47:46.226 fetch POST /api/observability/v1/traces
18:47:46.226 fetched POST /api/observability/v1/traces 204
18:47:46.821 chunk 17 36912 bytes
18:47:46.822 in Ack 17
18:47:47.823 fetch POST /api/observability/v1/traces
18:47:47.823 fetched POST /api/observability/v1/traces 204
18:47:48.971 fetch POST /api/observability/v1/traces
18:47:48.971 fetched POST /api/observability/v1/traces 204
18:47:49.515 in Ping 
18:47:49.743 in Interrupt 31
18:47:50.343 chunk 17 37911 bytes
18:47:50.344 in Ack 17
18:47:50.553 rpc 32 orchestration.dispatchCommand runtime-request.respond
18:47:50.747 fetch POST /api/observability/v1/traces
18:47:50.747 fetched POST /api/observability/v1/traces 204
18:47:53.361 chunk 17 40359 bytes
18:47:53.361 exit 32 {"_tag":"Success","value":{"sequence":95}}
18:47:53.362 in Ack 17
18:47:54.118 chunk 17 40421 bytes
18:47:54.120 in Ack 17
18:47:54.240 rpc 33 orchestration.dispatchCommand runtime-request.respond
18:47:54.363 fetch POST /api/observability/v1/traces
18:47:54.363 fetched POST /api/observability/v1/traces 204
18:47:54.519 in Ping 
18:47:54.539 chunk 17 40451 bytes
18:47:54.540 in Ack 17
18:47:55.149 chunk 17 40511 bytes
18:47:55.150 in Ack 17
18:47:55.541 fetch POST /api/observability/v1/traces
18:47:55.541 fetched POST /api/observability/v1/traces 204
18:47:55.680 chunk 17 40551 bytes
18:47:55.681 in Ack 17
18:47:56.308 chunk 17 40621 bytes
18:47:56.309 in Ack 17
18:47:56.682 fetch POST /api/observability/v1/traces
18:47:56.682 fetched POST /api/observability/v1/traces 204
18:47:57.377 chunk 17 40761 bytes
18:47:57.378 in Ack 17
18:47:58.379 fetch POST /api/observability/v1/traces
18:47:58.379 fetched POST /api/observability/v1/traces 204
18:47:59.526 in Ping 
18:47:59.745 rpc 34 server.reportClientActivity
18:48:00.745 fetch POST /api/observability/v1/traces
18:48:00.745 fetched POST /api/observability/v1/traces 204
18:48:00.943 chunk 17 40989 bytes
18:48:00.944 in Ack 17
18:48:01.194 chunk 17 40999 bytes
18:48:01.196 in Ack 17
18:48:01.453 chunk 17 41043 bytes
18:48:01.455 in Ack 17
18:48:01.843 chunk 17 41055 bytes
18:48:01.844 in Ack 17
18:48:01.946 fetch POST /api/observability/v1/traces
18:48:01.946 fetched POST /api/observability/v1/traces 204
18:48:02.628 chunk 17 41287 bytes
18:48:02.630 in Ack 17
18:48:03.631 fetch POST /api/observability/v1/traces
18:48:03.631 fetched POST /api/observability/v1/traces 204
18:48:04.371 chunk 17 41433 bytes
18:48:04.372 in Ack 17
18:48:04.527 in Ping 
18:48:04.624 chunk 17 41449 bytes
18:48:04.625 in Ack 17
18:48:04.899 chunk 17 41499 bytes
18:48:04.900 in Ack 17
18:48:05.373 fetch POST /api/observability/v1/traces
18:48:05.374 fetched POST /api/observability/v1/traces 204
18:48:06.449 chunk 17 41623 bytes
18:48:06.451 in Ack 17
18:48:07.452 fetch POST /api/observability/v1/traces
18:48:07.452 fetched POST /api/observability/v1/traces 204
18:48:07.808 chunk 17 43628 bytes
18:48:07.808 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
18:48:07.810 in Ack 17
18:48:08.810 fetch POST /api/observability/v1/traces
18:48:08.810 fetched POST /api/observability/v1/traces 204
18:48:08.962 chunk 17 43818 bytes
18:48:08.963 in Ack 17
18:48:09.244 rpc 35 orchestration.dispatchCommand runtime-request.respond
18:48:09.531 in Ping 
18:48:09.965 fetch POST /api/observability/v1/traces
18:48:09.965 fetched POST /api/observability/v1/traces 204
