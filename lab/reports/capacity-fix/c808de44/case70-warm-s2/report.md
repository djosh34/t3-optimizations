# case70, webapp, warm start, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 64 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.5s |
| Prompt reached the agent | 5.4s |
| Time to first text | 10.9s |
| Whole reply shown | 62.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 11.5s / 4.5s / 9.3s |
| Longest silence | 10.9s |
| Approval shown after it was asked | 9.9s |
| Bytes on the wire down / up | 61944 / 33256 |
| Bytes the app received / sent | 45138 / 80785 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.5 | 0 | 186 |
| 20 | 1.3 | 532 | 585 |
| 30 | 7.8 | 667 | 988 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.0 | 1097 | 1258 |
| 60 | 8.7 | 1304 | 1596 |
| 62 | 10.3 | 1323 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 470/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172523467,"silences":0,"longestSilenceMs":1302,"longestRecoveredGapMs":1302},"stats":{"sendRate":3000,"deliveredRate":1963.3911368015413,"loss":0,"srttMs":6.900229226292759,"minRttMs":1,"sentPackets":80,"sentWireBytes":26599,"receivedPackets":96,"receivedWireBytes":48675,"lastHeardMs":8464,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 99/849","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 40/100","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172532634,"silences":1,"longestSilenceMs":2257,"longestRecoveredGapMs":2257},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":710.3923630013804,"minRttMs":2147,"sentPackets":128,"sentWireBytes":42461,"receivedPackets":111,"receivedWireBytes":55502,"lastHeardMs":17631,"unsyncedBytes":68,"inflightPackets":44,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 819/849 1050/1078","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 673/673","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172544437,"silences":2,"longestSilenceMs":2327,"longestRecoveredGapMs":2327},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.5315151515151515,"srttMs":1925.7726573005896,"minRttMs":1356,"sentPackets":191,"sentWireBytes":55397,"receivedPackets":141,"receivedWireBytes":65411,"lastHeardMs":29433,"unsyncedBytes":308,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 988/1123","user":"10/10 9/9","attempts":"661/661 661/661 97/637","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172554326,"silences":2,"longestSilenceMs":2327,"longestRecoveredGapMs":2327},"stats":{"sendRate":1644.4185609074984,"deliveredRate":350.01330715319705,"loss":0.8237499999999999,"srttMs":3632.803037230115,"minRttMs":1356,"sentPackets":272,"sentWireBytes":77395,"receivedPackets":174,"receivedWireBytes":75488,"lastHeardMs":39323,"unsyncedBytes":122,"inflightPackets":124,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"661/661 661/661 269/637","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779","runtimeRequests":"879/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172564098,"silences":2,"longestSilenceMs":2327,"longestRecoveredGapMs":2327},"stats":{"sendRate":1233.3139206806238,"deliveredRate":286.857458309826,"loss":0.7467946160130718,"srttMs":3505.3179503793067,"minRttMs":1356,"sentPackets":318,"sentWireBytes":85244,"receivedPackets":195,"receivedWireBytes":82866,"lastHeardMs":49094,"unsyncedBytes":0,"inflightPackets":131,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 891/1550","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 1340/1340 188/213","user":"10/10 9/9","attempts":"661/661 661/661 327/637","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172574040,"silences":2,"longestSilenceMs":2327,"longestRecoveredGapMs":2327},"stats":{"sendRate":2098.628277906381,"deliveredRate":358.51566414234014,"loss":0.7437499999999999,"srttMs":2692.3653317531043,"minRttMs":1356,"sentPackets":363,"sentWireBytes":100418,"receivedPackets":216,"receivedWireBytes":90169,"lastHeardMs":59037,"unsyncedBytes":0,"inflightPackets":158,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","thread":"951/951","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 891/1550 306/1078","runs":"1014/1014 971/971","messages":"456/456 548/548","assistant":"32/32 21/21 1340/1340 667/723","user":"10/10 9/9","attempts":"661/661 661/661 327/637","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpointScopes":"723/723","checkpoints":"779/779 613/613 779/779","runtimeRequests":"143/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":264,"lastDataAt":1794172585365,"silences":3,"longestSilenceMs":4958,"longestRecoveredGapMs":4958},"stats":{"sendRate":2098.628277906381,"deliveredRate":280.998774730194,"loss":0.7372916666666666,"srttMs":2692.3653317531043,"minRttMs":1356,"sentPackets":396,"sentWireBytes":105134,"receivedPackets":225,"receivedWireBytes":93303,"lastHeardMs":70362,"unsyncedBytes":0,"inflightPackets":147,"duplicateBytes":0}}
- shim trace
21:15:15.276 fetch GET /api/auth/session
21:15:15.277 fetched GET /api/auth/session 200
21:15:15.403 fetch GET /.well-known/t3/environment
21:15:15.404 fetched GET /.well-known/t3/environment 200
21:15:15.413 fetch GET /.well-known/t3/environment
21:15:15.414 fetched GET /.well-known/t3/environment 200
21:15:15.418 rpc 0 subscribeServerConfig
21:15:15.418 chunk 0 38678 bytes
21:15:15.440 in Ack 0
21:15:15.441 rpc 1 subscribeServerLifecycle
21:15:15.442 rpc 2 subscribeServerLifecycle
21:15:15.444 fetch GET /api/auth/session
21:15:15.445 fetched GET /api/auth/session 200
21:15:15.445 rpc 3 subscribeDeviceState
21:15:15.447 rpc 4 preview.reportProfiles
21:15:15.483 rpc 5 subscribeProjectClones
21:15:15.502 fetch GET /api/orchestration/threads/mcp%3A925c555c-a772-4a98-9792-1f9911de6d3b/bounded
21:15:15.507 fetch GET /api/orchestration/shell
21:15:15.527 rpc 6 server.refreshProviders
21:15:15.568 rpc 7 orchestration.dispatchCommand thread.visit
21:15:15.568 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:15:15.579 fetched GET /api/orchestration/shell 200
21:15:15.579 rpc 8 subscribeTerminalMetadata
21:15:15.580 rpc 9 subscribeVcsStatus
21:15:15.581 rpc 10 subscribePreviewEvents
21:15:15.582 rpc 11 orchestration.subscribeShell
21:15:15.582 chunk 11 2779 bytes
21:15:15.583 rpc 12 assets.createUrl
21:15:15.584 rpc 13 preview.list
21:15:15.584 rpc 14 projects.readFile
21:15:15.586 in Ack 11
21:15:15.651 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c90ce613-9612-4df0-a0ab-57e8debc6961","sessions":[]}}
21:15:15.651 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:15:15.759 chunk 1 3861 bytes
21:15:15.759 in Ack 1
21:15:15.833 rpc 15 server.reportClientActivity
21:15:15.833 chunk 11 2755 bytes
21:15:15.833 in Ack 11
21:15:16.240 chunk 2 3861 bytes
21:15:16.240 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:15:03.277Z","configurableProviders":false,"con
21:15:16.240 chunk 9 472 bytes
21:15:16.240 in Ack 2
21:15:16.243 in Ack 9
21:15:16.368 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:15:16.402 fetch POST /api/observability/v1/traces
21:15:16.402 fetched POST /api/observability/v1/traces 204
21:15:16.508 chunk 3 876 bytes
21:15:16.508 exit 4 {"_tag":"Success","value":null}
21:15:16.508 chunk 5 4 bytes
21:15:16.508 chunk 8 36 bytes
21:15:16.508 exit 15 {"_tag":"Success","value":null}
21:15:16.509 in Ack 3
21:15:16.513 rpc 16 preview.reportProfiles
21:15:16.513 in Ack 5
21:15:16.514 in Ack 8
21:15:16.587 exit 16 {"_tag":"Success","value":null}
21:15:16.587 fetched GET /api/orchestration/threads/mcp%3A925c555c-a772-4a98-9792-1f9911de6d3b/bounded 200
21:15:16.590 rpc 17 orchestration.subscribeThread
21:15:16.590 chunk 17 8294 bytes
21:15:16.599 in Interrupt 9
21:15:16.601 rpc 18 subscribeVcsStatus
21:15:16.646 in Ack 17
21:15:16.660 chunk 11 2798 bytes
21:15:16.661 in Ack 11
21:15:16.733 chunk 18 472 bytes
21:15:16.733 in Ack 18
21:15:16.840 chunk 17 11932 bytes
21:15:16.841 in Ack 17
21:15:16.893 rpc 19 server.reportClientActivity
21:15:17.110 chunk 17 13517 bytes
21:15:17.113 in Ack 17
21:15:17.198 exit 19 {"_tag":"Success","value":null}
21:15:17.386 rpc 20 attachments.createUploadUrl
21:15:17.555 chunk 17 19081 bytes
21:15:17.562 fetch POST /api/observability/v1/traces
21:15:17.562 fetched POST /api/observability/v1/traces 204
21:15:17.566 in Ack 17
21:15:17.668 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-2f287155-75e8-467e-87c0-0391468013cd","expiresAt":1794173117588,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:15:17.669 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTJmMjg3MTU1LTc1ZTgtNDY3ZS04N2MwLTAzOTE0NjgwMTNjZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzExNzU4OH0.eFKymrr0dgjAH4J34uX7qMtg55ejG_PH14KTlB8JRLA
21:15:17.807 chunk 17 21197 bytes
21:15:17.808 in Ack 17
21:15:17.815 in Interrupt 18
21:15:17.816 rpc 21 subscribeVcsStatus
21:15:17.901 chunk 21 472 bytes
21:15:17.902 in Ack 21
21:15:18.068 rpc 22 server.reportClientActivity
21:15:18.568 fetch POST /api/observability/v1/traces
21:15:18.568 fetched POST /api/observability/v1/traces 204
21:15:18.657 exit 22 {"_tag":"Success","value":null}
21:15:19.658 fetch POST /api/observability/v1/traces
21:15:19.658 fetched POST /api/observability/v1/traces 204
21:15:20.416 in Ping 
21:15:20.572 rpc 23 orchestration.dispatchCommand message.dispatch
21:15:21.159 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:15:21.161 rpc 24 attachments.delete
21:15:21.571 fetch POST /api/observability/v1/traces
21:15:21.571 fetched POST /api/observability/v1/traces 204
21:15:21.662 chunk 17 23662 bytes
21:15:21.663 in Ack 17
21:15:21.913 chunk 17 23674 bytes
21:15:21.914 in Ack 17
21:15:22.164 chunk 17 23676 bytes
21:15:22.165 in Ack 17
21:15:22.408 exit 24 {"_tag":"Success","value":null}
21:15:22.414 chunk 11 2820 bytes
21:15:22.415 in Ack 11
21:15:22.416 rpc 25 orchestration.dispatchCommand thread.visit
21:15:22.416 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:15:22.664 fetch POST /api/observability/v1/traces
21:15:22.664 fetched POST /api/observability/v1/traces 204
21:15:22.665 chunk 17 23676 bytes
21:15:22.666 in Ack 17
21:15:22.914 chunk 11 2820 bytes
21:15:22.914 chunk 17 24648 bytes
21:15:22.915 in Ack 11
21:15:22.919 in Ack 17
21:15:23.165 chunk 17 30526 bytes
21:15:23.167 in Ack 17
21:15:23.169 rpc 26 assets.createUrl
21:15:23.459 chunk 17 31336 bytes
21:15:23.460 in Ack 17
21:15:23.671 fetch POST /api/observability/v1/traces
21:15:23.671 fetched POST /api/observability/v1/traces 204
21:15:24.236 rpc 27 orchestration.dispatchCommand message.dispatch
21:15:25.236 fetch POST /api/observability/v1/traces
21:15:25.236 fetched POST /api/observability/v1/traces 204
21:15:25.417 in Ping 
21:15:26.379 chunk 17 32116 bytes
21:15:26.380 in Ack 17
21:15:26.387 in Interrupt 21
21:15:26.395 rpc 28 subscribeVcsStatus
21:15:26.646 rpc 29 server.reportClientActivity
21:15:27.088 fetch POST /api/observability/v1/traces
21:15:27.088 fetched POST /api/observability/v1/traces 204
21:15:28.129 fetch POST /api/observability/v1/traces
21:15:28.129 fetched POST /api/observability/v1/traces 204
21:15:30.417 in Ping 
21:15:30.541 exit 27 {"_tag":"Success","value":{"sequence":75}}
21:15:30.541 chunk 28 472 bytes
21:15:30.542 in Ack 28
21:15:31.138 chunk 17 34583 bytes
21:15:31.140 in Ack 17
21:15:31.543 fetch POST /api/observability/v1/traces
21:15:31.543 fetched POST /api/observability/v1/traces 204
21:15:32.203 chunk 17 34615 bytes
21:15:32.204 in Ack 17
21:15:32.635 chunk 17 34635 bytes
21:15:32.636 in Ack 17
21:15:33.205 fetch POST /api/observability/v1/traces
21:15:33.205 fetched POST /api/observability/v1/traces 204
21:15:34.963 chunk 17 34725 bytes
21:15:34.964 in Ack 17
21:15:35.390 chunk 17 34747 bytes
21:15:35.391 in Ack 17
21:15:35.422 in Ping 
21:15:35.828 chunk 17 34767 bytes
21:15:35.829 in Ack 17
21:15:35.965 fetch POST /api/observability/v1/traces
21:15:35.965 fetched POST /api/observability/v1/traces 204
21:15:36.348 chunk 17 34777 bytes
21:15:36.349 in Ack 17
21:15:36.772 chunk 17 34797 bytes
21:15:36.773 in Ack 17
21:15:37.295 chunk 17 34849 bytes
21:15:37.297 in Ack 17
21:15:37.349 fetch POST /api/observability/v1/traces
21:15:37.350 fetched POST /api/observability/v1/traces 204
21:15:37.719 chunk 17 35109 bytes
21:15:37.720 in Ack 17
21:15:37.974 chunk 17 35137 bytes
21:15:37.975 in Ack 17
21:15:38.287 chunk 17 35169 bytes
21:15:38.289 in Ack 17
21:15:38.538 chunk 17 35191 bytes
21:15:38.539 in Ack 17
21:15:38.721 fetch POST /api/observability/v1/traces
21:15:38.721 fetched POST /api/observability/v1/traces 204
21:15:38.788 chunk 17 35203 bytes
21:15:38.789 in Ack 17
21:15:39.075 chunk 17 35223 bytes
21:15:39.076 in Ack 17
21:15:39.325 chunk 17 35245 bytes
21:15:39.326 in Ack 17
21:15:39.730 chunk 17 35267 bytes
21:15:39.731 in Ack 17
21:15:39.790 fetch POST /api/observability/v1/traces
21:15:39.790 fetched POST /api/observability/v1/traces 204
21:15:40.067 chunk 17 35413 bytes
21:15:40.068 in Ack 17
21:15:40.316 chunk 17 35423 bytes
21:15:40.317 in Ack 17
21:15:40.423 in Ping 
21:15:40.823 chunk 17 35433 bytes
21:15:40.825 in Ack 17
21:15:41.069 fetch POST /api/observability/v1/traces
21:15:41.069 fetched POST /api/observability/v1/traces 204
21:15:41.631 chunk 17 35455 bytes
21:15:41.632 in Ack 17
21:15:41.648 in Interrupt 29
21:15:41.893 chunk 17 35497 bytes
21:15:41.894 in Ack 17
21:15:41.903 rpc 30 server.reportClientActivity
21:15:42.146 chunk 17 35517 bytes
21:15:42.147 in Ack 17
21:15:42.480 chunk 17 35529 bytes
21:15:42.481 in Ack 17
21:15:42.633 fetch POST /api/observability/v1/traces
21:15:42.633 fetched POST /api/observability/v1/traces 204
21:15:42.735 chunk 17 35539 bytes
21:15:42.736 in Ack 17
21:15:42.990 chunk 17 35673 bytes
21:15:42.991 in Ack 17
21:15:43.245 chunk 17 35715 bytes
21:15:43.246 in Ack 17
21:15:43.577 chunk 17 35845 bytes
21:15:43.579 in Ack 17
21:15:43.737 fetch POST /api/observability/v1/traces
21:15:43.737 fetched POST /api/observability/v1/traces 204
21:15:43.835 chunk 17 35871 bytes
21:15:43.835 in Ack 17
21:15:44.094 chunk 17 35881 bytes
21:15:44.095 in Ack 17
21:15:44.438 chunk 17 35901 bytes
21:15:44.439 in Ack 17
21:15:44.694 chunk 17 35931 bytes
21:15:44.696 in Ack 17
21:15:44.837 fetch POST /api/observability/v1/traces
21:15:44.837 fetched POST /api/observability/v1/traces 204
21:15:45.206 chunk 17 35941 bytes
21:15:45.208 in Ack 17
21:15:45.423 in Ping 
21:15:45.716 chunk 17 35961 bytes
21:15:45.717 in Ack 17
21:15:45.968 chunk 17 35971 bytes
21:15:45.969 in Ack 17
21:15:46.210 fetch POST /api/observability/v1/traces
21:15:46.210 fetched POST /api/observability/v1/traces 204
21:15:46.476 chunk 17 35983 bytes
21:15:46.478 in Ack 17
21:15:46.727 chunk 17 36003 bytes
21:15:46.728 in Ack 17
21:15:47.227 chunk 17 36015 bytes
21:15:47.228 in Ack 17
21:15:47.478 chunk 17 36027 bytes
21:15:47.480 in Ack 17
21:15:47.489 fetch POST /api/observability/v1/traces
21:15:47.489 fetched POST /api/observability/v1/traces 204
21:15:47.980 chunk 17 36049 bytes
21:15:47.981 in Ack 17
21:15:48.313 chunk 17 36011 bytes
21:15:48.314 in Ack 17
21:15:48.567 chunk 17 36021 bytes
21:15:48.568 in Ack 17
21:15:48.982 fetch POST /api/observability/v1/traces
21:15:48.982 fetched POST /api/observability/v1/traces 204
21:15:49.231 chunk 17 36033 bytes
21:15:49.232 in Ack 17
21:15:49.562 chunk 17 36053 bytes
21:15:49.563 in Ack 17
21:15:49.817 chunk 17 36063 bytes
21:15:49.818 in Ack 17
21:15:50.071 chunk 17 36091 bytes
21:15:50.073 in Ack 17
21:15:50.234 fetch POST /api/observability/v1/traces
21:15:50.234 fetched POST /api/observability/v1/traces 204
21:15:50.414 chunk 17 36111 bytes
21:15:50.415 in Ack 17
21:15:50.424 in Ping 
21:15:51.181 chunk 17 36123 bytes
21:15:51.182 in Ack 17
21:15:51.416 fetch POST /api/observability/v1/traces
21:15:51.417 fetched POST /api/observability/v1/traces 204
21:15:51.528 chunk 17 36163 bytes
21:15:51.530 in Ack 17
21:15:52.115 chunk 17 36243 bytes
21:15:52.116 in Ack 17
21:15:52.531 fetch POST /api/observability/v1/traces
21:15:52.531 fetched POST /api/observability/v1/traces 204
21:15:52.622 chunk 17 36265 bytes
21:15:52.623 in Ack 17
21:15:52.872 chunk 17 36297 bytes
21:15:52.873 in Ack 17
21:15:53.299 chunk 17 36335 bytes
21:15:53.300 in Ack 17
21:15:53.624 fetch POST /api/observability/v1/traces
21:15:53.624 fetched POST /api/observability/v1/traces 204
21:15:53.727 chunk 17 36345 bytes
21:15:53.728 in Ack 17
21:15:54.067 chunk 17 36445 bytes
21:15:54.068 in Ack 17
21:15:54.327 chunk 17 36495 bytes
21:15:54.329 in Ack 17
21:15:54.584 chunk 17 36505 bytes
21:15:54.585 in Ack 17
21:15:54.729 fetch POST /api/observability/v1/traces
21:15:54.729 fetched POST /api/observability/v1/traces 204
21:15:55.029 chunk 17 36535 bytes
21:15:55.030 in Ack 17
21:15:55.425 in Ping 
21:15:55.434 chunk 17 36595 bytes
21:15:55.435 in Ack 17
21:15:55.915 chunk 17 36779 bytes
21:15:55.916 in Ack 17
21:15:56.031 fetch POST /api/observability/v1/traces
21:15:56.031 fetched POST /api/observability/v1/traces 204
21:15:56.344 chunk 17 36789 bytes
21:15:56.346 in Ack 17
21:15:56.595 chunk 17 36799 bytes
21:15:56.596 in Ack 17
21:15:56.847 chunk 17 36809 bytes
21:15:56.848 in Ack 17
21:15:56.903 in Interrupt 30
21:15:57.099 chunk 17 36819 bytes
21:15:57.101 in Ack 17
21:15:57.346 fetch POST /api/observability/v1/traces
21:15:57.346 fetched POST /api/observability/v1/traces 204
21:15:57.443 chunk 17 36851 bytes
21:15:57.444 in Ack 17
21:15:57.698 chunk 17 36871 bytes
21:15:57.699 in Ack 17
21:15:57.953 chunk 17 36883 bytes
21:15:57.954 in Ack 17
21:15:58.287 chunk 17 36935 bytes
21:15:58.288 in Ack 17
21:15:58.445 fetch POST /api/observability/v1/traces
21:15:58.445 fetched POST /api/observability/v1/traces 204
21:15:58.538 chunk 17 36945 bytes
21:15:58.539 in Ack 17
21:15:58.789 chunk 17 36977 bytes
21:15:58.790 in Ack 17
21:15:59.163 chunk 17 37073 bytes
21:15:59.164 in Ack 17
21:15:59.540 fetch POST /api/observability/v1/traces
21:15:59.540 fetched POST /api/observability/v1/traces 204
21:15:59.785 chunk 17 37099 bytes
21:15:59.786 in Ack 17
21:16:00.425 in Ping 
21:16:00.641 chunk 17 37191 bytes
21:16:00.642 in Ack 17
21:16:00.787 fetch POST /api/observability/v1/traces
21:16:00.787 fetched POST /api/observability/v1/traces 204
21:16:05.425 in Ping 
21:16:05.656 rpc 31 server.reportClientActivity
21:16:06.211 chunk 17 38199 bytes
21:16:06.212 in Ack 17
21:16:06.394 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:16:06.657 fetch POST /api/observability/v1/traces
21:16:06.657 fetched POST /api/observability/v1/traces 204
21:16:08.140 fetch POST /api/observability/v1/traces
21:16:08.140 fetched POST /api/observability/v1/traces 204
21:16:10.393 chunk 17 40680 bytes
21:16:10.393 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:16:10.394 in Ack 17
21:16:10.452 in Ping 
21:16:10.828 chunk 17 40700 bytes
21:16:10.829 in Ack 17
21:16:11.267 chunk 17 40730 bytes
21:16:11.268 in Ack 17
21:16:11.294 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:16:11.395 fetch POST /api/observability/v1/traces
21:16:11.395 fetched POST /api/observability/v1/traces 204
21:16:12.160 chunk 17 40792 bytes
21:16:12.161 in Ack 17
21:16:13.162 fetch POST /api/observability/v1/traces
21:16:13.162 fetched POST /api/observability/v1/traces 204
21:16:14.041 chunk 17 41012 bytes
21:16:14.042 in Ack 17
21:16:15.043 fetch POST /api/observability/v1/traces
21:16:15.043 fetched POST /api/observability/v1/traces 204
21:16:15.452 in Ping 
21:16:15.606 chunk 17 41112 bytes
21:16:15.608 in Ack 17
21:16:16.243 chunk 17 41202 bytes
21:16:16.245 in Ack 17
21:16:16.610 fetch POST /api/observability/v1/traces
21:16:16.610 fetched POST /api/observability/v1/traces 204
21:16:18.143 fetch POST /api/observability/v1/traces
21:16:18.143 fetched POST /api/observability/v1/traces 204
21:16:20.457 in Ping 
21:16:20.657 in Interrupt 31
21:16:21.201 chunk 17 41326 bytes
21:16:21.202 in Ack 17
21:16:21.629 chunk 17 41356 bytes
21:16:21.630 in Ack 17
21:16:21.666 fetch POST /api/observability/v1/traces
21:16:21.666 fetched POST /api/observability/v1/traces 204
21:16:22.017 chunk 17 41570 bytes
21:16:22.018 in Ack 17
21:16:23.019 fetch POST /api/observability/v1/traces
21:16:23.019 fetched POST /api/observability/v1/traces 204
21:16:23.930 chunk 17 41778 bytes
21:16:23.932 in Ack 17
21:16:24.188 chunk 17 41790 bytes
21:16:24.189 in Ack 17
21:16:24.932 fetch POST /api/observability/v1/traces
21:16:24.932 fetched POST /api/observability/v1/traces 204
21:16:25.075 chunk 17 42024 bytes
21:16:25.077 in Ack 17
21:16:25.462 in Ping 
21:16:26.078 fetch POST /api/observability/v1/traces
21:16:26.078 fetched POST /api/observability/v1/traces 204
21:16:26.493 chunk 17 42134 bytes
21:16:26.494 in Ack 17
