# case50, webapp, warm start, seed 4

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 3.3s |
| Whole reply shown | 44.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.8s / 1.2s / 2.1s |
| Longest silence | 3.3s |
| Approval shown after it was asked | 0.9s |
| Bytes on the wire down / up | 324260 / 115473 |
| Bytes the app received / sent | 260804 / 49658 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.8 | 303 | 373 |
| 20 | 0.8 | 742 | 774 |
| 30 | 0.7 | 1090 | 1116 |
| 40 | 0.9 | 1461 | 1492 |
| 44 | 1.1 | 1590 | 1596 |

## Driver notes

- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","runs":"1014/1014 971/971","providerTurns":"774/774 774/774","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerThreads":"808/808","providerSessions":"2696/2696","messages":"456/456 546/546 399/901 546/546","nodes":"1019/1019 809/809 809/809 365/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"653/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":243,"lastDataAt":1794165642214,"silences":0,"longestSilenceMs":1148,"longestRecoveredGapMs":1148},"stats":{"sendRate":4500,"deliveredRate":2010.0229357798162,"loss":0,"srttMs":48.49468092325638,"minRttMs":1,"sentPackets":98,"sentWireBytes":31476,"receivedPackets":109,"receivedWireBytes":53534,"lastHeardMs":9034,"unsyncedBytes":509,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 622/839","providerTurns":"774/774 774/774 71/656","assistant":"32/32 21/21 400/445","user":"10/10 9/9","attempts":"660/660 660/660 636/636","providerThreads":"808/808","providerSessions":"2696/2696","messages":"456/456 546/546 703/901 546/546","nodes":"1019/1019 809/809 809/809 443/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"653/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":243,"lastDataAt":1794165652037,"silences":0,"longestSilenceMs":1148,"longestRecoveredGapMs":1148},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.40573303167420816,"srttMs":411.16323362709113,"minRttMs":1,"sentPackets":156,"sentWireBytes":43477,"receivedPackets":156,"receivedWireBytes":72845,"lastHeardMs":18856,"unsyncedBytes":0,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 656/656","assistant":"32/32 21/21 948/953","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 545/545 451/451","nodes":"1019/1019 809/809 809/809 917/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"716/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":243,"lastDataAt":1794165662335,"silences":0,"longestSilenceMs":1148,"longestRecoveredGapMs":1148},"stats":{"sendRate":1125,"deliveredRate":1287.741935483871,"loss":0.3846153846153846,"srttMs":561.3939962027137,"minRttMs":1,"sentPackets":211,"sentWireBytes":53339,"receivedPackets":283,"receivedWireBytes":117941,"lastHeardMs":29154,"unsyncedBytes":0,"inflightPackets":26,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 477/1546 926/926","frame":"1030/1030","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 656/656","assistant":"32/32 21/21 1340/1340 98/113","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 545/545 451/451","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"344/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":243,"lastDataAt":1794165673395,"silences":0,"longestSilenceMs":1148,"longestRecoveredGapMs":1148},"stats":{"sendRate":843.75,"deliveredRate":0,"loss":0.5293706293706293,"srttMs":629.1741029248649,"minRttMs":1,"sentPackets":262,"sentWireBytes":62827,"receivedPackets":500,"receivedWireBytes":196227,"lastHeardMs":40214,"unsyncedBytes":399,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 656/656","assistant":"32/32 21/21 1340/1340 664/664","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 546/546 451/451 545/545","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 985/985 1395/1395 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":243,"lastDataAt":1794165684502,"silences":0,"longestSilenceMs":1148,"longestRecoveredGapMs":1148},"stats":{"sendRate":1431.5593821447662,"deliveredRate":411.51492542212713,"loss":0.5688111888111889,"srttMs":740.4459024630978,"minRttMs":676,"sentPackets":309,"sentWireBytes":74446,"receivedPackets":729,"receivedWireBytes":290401,"lastHeardMs":51321,"unsyncedBytes":500,"inflightPackets":51,"duplicateBytes":0}}
- shim trace
19:20:33.464 fetch GET /api/auth/session
19:20:33.465 fetched GET /api/auth/session 200
19:20:33.585 fetch GET /.well-known/t3/environment
19:20:33.585 fetched GET /.well-known/t3/environment 200
19:20:33.593 fetch GET /.well-known/t3/environment
19:20:33.594 fetched GET /.well-known/t3/environment 200
19:20:33.597 rpc 0 subscribeServerConfig
19:20:33.598 chunk 0 38722 bytes
19:20:33.612 in Ack 0
19:20:33.613 rpc 1 subscribeServerLifecycle
19:20:33.614 rpc 2 subscribeServerLifecycle
19:20:33.616 fetch GET /api/auth/session
19:20:33.617 fetched GET /api/auth/session 200
19:20:33.617 rpc 3 subscribeDeviceState
19:20:33.618 rpc 4 preview.reportProfiles
19:20:33.687 rpc 5 subscribeProjectClones
19:20:33.704 fetch GET /api/orchestration/threads/mcp%3Aca6975bb-9870-4a3e-b939-2bcab5d0436c/bounded
19:20:33.709 fetch GET /api/orchestration/shell
19:20:33.728 rpc 6 server.refreshProviders
19:20:33.766 rpc 7 orchestration.dispatchCommand thread.visit
19:20:33.766 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:20:33.777 fetched GET /api/orchestration/shell 200
19:20:33.777 rpc 8 subscribeTerminalMetadata
19:20:33.778 rpc 9 subscribeVcsStatus
19:20:33.779 rpc 10 subscribePreviewEvents
19:20:33.780 rpc 11 orchestration.subscribeShell
19:20:33.780 chunk 11 2790 bytes
19:20:33.782 rpc 12 assets.createUrl
19:20:33.782 rpc 13 preview.list
19:20:33.783 rpc 14 projects.readFile
19:20:33.787 in Ack 11
19:20:33.881 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"ac9c80be-27b9-460d-ac87-6474510c6ecc","sessions":[]}}
19:20:33.881 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:20:33.999 chunk 1 3872 bytes
19:20:33.999 in Ack 1
19:20:34.031 rpc 15 server.reportClientActivity
19:20:34.031 chunk 11 2766 bytes
19:20:34.032 in Ack 11
19:20:34.399 chunk 2 3872 bytes
19:20:34.399 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:20:21.841Z","configurableProviders":false,"con
19:20:34.399 chunk 9 980 bytes
19:20:34.400 in Ack 2
19:20:34.403 in Ack 9
19:20:34.508 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:20:34.583 fetch POST /api/observability/v1/traces
19:20:34.583 fetched POST /api/observability/v1/traces 204
19:20:34.740 chunk 3 876 bytes
19:20:34.740 exit 4 {"_tag":"Success","value":null}
19:20:34.740 chunk 5 4 bytes
19:20:34.740 chunk 8 36 bytes
19:20:34.740 exit 15 {"_tag":"Success","value":null}
19:20:34.741 in Ack 3
19:20:34.745 rpc 16 preview.reportProfiles
19:20:34.745 in Ack 5
19:20:34.745 in Ack 8
19:20:34.758 chunk 11 2809 bytes
19:20:34.758 in Ack 11
19:20:34.819 exit 16 {"_tag":"Success","value":null}
19:20:34.819 fetched GET /api/orchestration/threads/mcp%3Aca6975bb-9870-4a3e-b939-2bcab5d0436c/bounded 200
19:20:34.822 rpc 17 orchestration.subscribeThread
19:20:34.822 chunk 17 8283 bytes
19:20:34.832 in Interrupt 9
19:20:34.834 rpc 18 subscribeVcsStatus
19:20:34.835 in Ack 17
19:20:34.926 chunk 18 980 bytes
19:20:34.926 in Ack 18
19:20:35.074 chunk 17 11920 bytes
19:20:35.075 in Ack 17
19:20:35.125 rpc 19 server.reportClientActivity
19:20:35.325 chunk 17 13502 bytes
19:20:35.326 in Ack 17
19:20:35.428 exit 19 {"_tag":"Success","value":null}
19:20:35.558 rpc 20 attachments.createUploadUrl
19:20:35.718 chunk 17 18605 bytes
19:20:35.724 in Ack 17
19:20:35.745 fetch POST /api/observability/v1/traces
19:20:35.745 fetched POST /api/observability/v1/traces 204
19:20:35.768 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-acc4e032-143a-4a7d-b737-a7a8554d79b0","expiresAt":1794166235738,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:20:35.769 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWFjYzRlMDMyLTE0M2EtNGE3ZC1iNzM3LWE3YTg1NTRkNzliMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjIzNTczOH0.CZKGS4oLQ4xRzGwly0uPBKvO1s2ZVLOTRL8JrbBP3B0
19:20:35.968 chunk 17 20465 bytes
19:20:35.969 in Ack 17
19:20:35.976 in Interrupt 18
19:20:35.977 rpc 21 subscribeVcsStatus
19:20:36.017 chunk 21 980 bytes
19:20:36.018 in Ack 21
19:20:36.218 chunk 17 21199 bytes
19:20:36.219 in Ack 17
19:20:36.229 rpc 22 server.reportClientActivity
19:20:36.772 fetch POST /api/observability/v1/traces
19:20:36.772 fetched POST /api/observability/v1/traces 204
19:20:36.823 exit 22 {"_tag":"Success","value":null}
19:20:37.826 fetch POST /api/observability/v1/traces
19:20:37.826 fetched POST /api/observability/v1/traces 204
19:20:38.676 rpc 23 orchestration.dispatchCommand message.dispatch
19:20:39.215 in Ping 
19:20:39.259 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:20:39.261 rpc 24 attachments.delete
19:20:39.528 chunk 17 23633 bytes
19:20:39.529 in Ack 17
19:20:39.675 fetch POST /api/observability/v1/traces
19:20:39.675 fetched POST /api/observability/v1/traces 204
19:20:39.779 chunk 17 23643 bytes
19:20:39.779 in Ack 17
19:20:40.035 chunk 17 23665 bytes
19:20:40.036 in Ack 17
19:20:40.289 exit 24 {"_tag":"Success","value":null}
19:20:40.290 chunk 17 23667 bytes
19:20:40.291 in Ack 17
19:20:40.539 chunk 11 2831 bytes
19:20:40.539 chunk 17 30516 bytes
19:20:40.539 in Ack 11
19:20:40.541 rpc 25 orchestration.dispatchCommand thread.visit
19:20:40.541 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:20:40.578 in Ack 17
19:20:40.580 rpc 26 assets.createUrl
19:20:40.781 fetch POST /api/observability/v1/traces
19:20:40.781 fetched POST /api/observability/v1/traces 204
19:20:41.039 chunk 11 2831 bytes
19:20:41.039 chunk 17 32106 bytes
19:20:41.040 in Ack 11
19:20:41.045 in Ack 17
19:20:41.050 in Interrupt 21
19:20:41.052 rpc 27 subscribeVcsStatus
19:20:41.138 chunk 27 980 bytes
19:20:41.139 in Ack 27
19:20:41.503 chunk 17 32715 bytes
19:20:41.503 rpc 28 server.reportClientActivity
19:20:41.505 in Ack 17
19:20:41.755 chunk 17 33283 bytes
19:20:41.756 in Ack 17
19:20:42.057 fetch POST /api/observability/v1/traces
19:20:42.057 fetched POST /api/observability/v1/traces 204
19:20:42.181 rpc 29 orchestration.dispatchCommand message.dispatch
19:20:42.973 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:20:43.182 fetch POST /api/observability/v1/traces
19:20:43.182 fetched POST /api/observability/v1/traces 204
19:20:43.512 chunk 17 35719 bytes
19:20:43.513 in Ack 17
19:20:43.764 chunk 17 35729 bytes
19:20:43.766 in Ack 17
19:20:44.023 chunk 17 35759 bytes
19:20:44.024 in Ack 17
19:20:44.215 in Ping 
19:20:44.466 chunk 17 35771 bytes
19:20:44.468 in Ack 17
19:20:44.514 fetch POST /api/observability/v1/traces
19:20:44.514 fetched POST /api/observability/v1/traces 204
19:20:44.717 chunk 17 35781 bytes
19:20:44.718 in Ack 17
19:20:45.005 chunk 17 35793 bytes
19:20:45.006 in Ack 17
19:20:45.255 chunk 17 35891 bytes
19:20:45.256 in Ack 17
19:20:45.719 fetch POST /api/observability/v1/traces
19:20:45.719 fetched POST /api/observability/v1/traces 204
19:20:46.092 chunk 17 35913 bytes
19:20:46.094 in Ack 17
19:20:46.344 chunk 17 35963 bytes
19:20:46.345 in Ack 17
19:20:46.594 chunk 17 35973 bytes
19:20:46.595 in Ack 17
19:20:46.846 chunk 17 36023 bytes
19:20:46.847 in Ack 17
19:20:47.094 fetch POST /api/observability/v1/traces
19:20:47.094 fetched POST /api/observability/v1/traces 204
19:20:47.096 chunk 17 36107 bytes
19:20:47.097 in Ack 17
19:20:47.347 chunk 17 36069 bytes
19:20:47.348 in Ack 17
19:20:47.664 chunk 17 36091 bytes
19:20:47.666 in Ack 17
19:20:47.914 chunk 17 36103 bytes
19:20:47.915 in Ack 17
19:20:48.098 fetch POST /api/observability/v1/traces
19:20:48.098 fetched POST /api/observability/v1/traces 204
19:20:48.165 chunk 17 36133 bytes
19:20:48.166 in Ack 17
19:20:48.635 chunk 17 36181 bytes
19:20:48.636 in Ack 17
19:20:48.886 chunk 17 36193 bytes
19:20:48.887 in Ack 17
19:20:49.137 chunk 17 36203 bytes
19:20:49.139 in Ack 17
19:20:49.168 fetch POST /api/observability/v1/traces
19:20:49.168 fetched POST /api/observability/v1/traces 204
19:20:49.217 in Ping 
19:20:49.387 chunk 17 36263 bytes
19:20:49.388 in Ack 17
19:20:49.637 chunk 17 36283 bytes
19:20:49.638 in Ack 17
19:20:49.886 chunk 11 2895 bytes
19:20:49.886 in Ack 11
19:20:49.923 chunk 17 36313 bytes
19:20:49.925 in Ack 17
19:20:50.173 chunk 17 36351 bytes
19:20:50.174 in Ack 17
19:20:50.390 fetch POST /api/observability/v1/traces
19:20:50.390 fetched POST /api/observability/v1/traces 204
19:20:50.424 chunk 17 36371 bytes
19:20:50.425 in Ack 17
19:20:50.543 rpc 30 orchestration.dispatchCommand thread.visit
19:20:50.543 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:20:50.673 chunk 17 36409 bytes
19:20:50.673 in Ack 17
19:20:51.040 chunk 17 37058 bytes
19:20:51.041 in Ack 17
19:20:51.427 fetch POST /api/observability/v1/traces
19:20:51.427 fetched POST /api/observability/v1/traces 204
19:20:51.996 chunk 17 37088 bytes
19:20:51.997 in Ack 17
19:20:52.247 chunk 17 37098 bytes
19:20:52.248 in Ack 17
19:20:52.497 chunk 17 37170 bytes
19:20:52.498 in Ack 17
19:20:52.747 chunk 17 37200 bytes
19:20:52.749 in Ack 17
19:20:52.998 chunk 17 37268 bytes
19:20:52.998 fetch POST /api/observability/v1/traces
19:20:52.998 fetched POST /api/observability/v1/traces 204
19:20:52.999 in Ack 17
19:20:53.248 chunk 17 39291 bytes
19:20:53.249 in Ack 17
19:20:53.499 chunk 17 39301 bytes
19:20:53.500 in Ack 17
19:20:53.750 chunk 17 39335 bytes
19:20:53.751 in Ack 17
19:20:54.000 chunk 17 39355 bytes
19:20:54.000 fetch POST /api/observability/v1/traces
19:20:54.000 fetched POST /api/observability/v1/traces 204
19:20:54.002 in Ack 17
19:20:54.218 in Ping 
19:20:54.249 chunk 17 39385 bytes
19:20:54.250 in Ack 17
19:20:54.500 chunk 17 39395 bytes
19:20:54.501 in Ack 17
19:20:54.750 chunk 17 39423 bytes
19:20:54.751 in Ack 17
19:20:55.000 chunk 17 39463 bytes
19:20:55.001 in Ack 17
19:20:55.010 fetch POST /api/observability/v1/traces
19:20:55.010 fetched POST /api/observability/v1/traces 204
19:20:55.250 chunk 17 39473 bytes
19:20:55.251 in Ack 17
19:20:55.501 chunk 17 39505 bytes
19:20:55.502 in Ack 17
19:20:55.751 chunk 17 39533 bytes
19:20:55.752 in Ack 17
19:20:56.001 chunk 17 39553 bytes
19:20:56.002 in Ack 17
19:20:56.252 chunk 17 39585 bytes
19:20:56.252 fetch POST /api/observability/v1/traces
19:20:56.252 fetched POST /api/observability/v1/traces 204
19:20:56.253 in Ack 17
19:20:56.502 chunk 17 39595 bytes
19:20:56.503 in Ack 17
19:20:56.512 in Interrupt 28
19:20:56.753 chunk 17 39617 bytes
19:20:56.753 in Ack 17
19:20:57.003 chunk 17 39663 bytes
19:20:57.004 in Ack 17
19:20:57.253 chunk 17 40533 bytes
19:20:57.254 fetch POST /api/observability/v1/traces
19:20:57.254 fetched POST /api/observability/v1/traces 204
19:20:57.255 in Ack 17
19:20:57.504 chunk 11 2895 bytes
19:20:57.504 chunk 17 40543 bytes
19:20:57.505 in Ack 11
19:20:57.510 in Ack 17
19:20:57.755 chunk 17 40581 bytes
19:20:57.756 in Ack 17
19:20:58.013 chunk 17 40591 bytes
19:20:58.014 in Ack 17
19:20:58.256 fetch POST /api/observability/v1/traces
19:20:58.256 fetched POST /api/observability/v1/traces 204
19:20:58.263 chunk 17 40621 bytes
19:20:58.264 in Ack 17
19:20:58.513 chunk 17 42496 bytes
19:20:58.514 in Ack 17
19:20:58.757 exit 28 {"_tag":"Success","value":null}
19:20:58.765 chunk 17 43437 bytes
19:20:58.766 in Ack 17
19:20:58.837 rpc 31 server.reportClientActivity
19:20:59.018 chunk 17 43482 bytes
19:20:59.019 in Ack 17
19:20:59.218 in Ping 
19:20:59.265 fetch POST /api/observability/v1/traces
19:20:59.265 fetched POST /api/observability/v1/traces 204
19:20:59.269 chunk 17 43500 bytes
19:20:59.270 in Ack 17
19:20:59.770 chunk 17 43551 bytes
19:20:59.771 in Ack 17
19:21:00.020 chunk 17 43584 bytes
19:21:00.021 in Ack 17
19:21:00.270 chunk 17 43647 bytes
19:21:00.271 in Ack 17
19:21:00.278 fetch POST /api/observability/v1/traces
19:21:00.278 fetched POST /api/observability/v1/traces 204
19:21:00.520 chunk 17 44495 bytes
19:21:00.521 in Ack 17
19:21:00.543 rpc 32 orchestration.dispatchCommand thread.visit
19:21:00.543 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:21:00.770 chunk 17 44540 bytes
19:21:00.771 in Ack 17
19:21:00.937 exit 31 {"_tag":"Success","value":null}
19:21:01.020 chunk 17 45227 bytes
19:21:01.021 in Ack 17
19:21:01.270 chunk 17 45272 bytes
19:21:01.271 in Ack 17
19:21:01.521 chunk 17 45317 bytes
19:21:01.522 fetch POST /api/observability/v1/traces
19:21:01.522 fetched POST /api/observability/v1/traces 204
19:21:01.523 in Ack 17
19:21:01.772 chunk 17 45332 bytes
19:21:01.775 in Ack 17
19:21:02.027 chunk 17 45365 bytes
19:21:02.028 in Ack 17
19:21:02.277 chunk 17 45395 bytes
19:21:02.278 in Ack 17
19:21:02.524 fetch POST /api/observability/v1/traces
19:21:02.524 fetched POST /api/observability/v1/traces 204
19:21:02.527 chunk 17 45452 bytes
19:21:02.528 in Ack 17
19:21:02.777 chunk 11 2895 bytes
19:21:02.778 chunk 17 45482 bytes
19:21:02.778 in Ack 11
19:21:02.783 in Ack 17
19:21:03.028 chunk 17 45527 bytes
19:21:03.030 in Ack 17
19:21:03.278 chunk 17 45557 bytes
19:21:03.279 in Ack 17
19:21:03.705 chunk 17 46637 bytes
19:21:03.705 fetch POST /api/observability/v1/traces
19:21:03.706 fetched POST /api/observability/v1/traces 204
19:21:03.710 in Ack 17
19:21:03.955 chunk 17 46667 bytes
19:21:03.956 in Ack 17
19:21:04.206 chunk 17 46712 bytes
19:21:04.207 in Ack 17
19:21:04.223 in Ping 
19:21:04.457 chunk 17 46742 bytes
19:21:04.458 in Ack 17
19:21:04.707 chunk 17 46787 bytes
19:21:04.707 in Ack 17
19:21:04.728 fetch POST /api/observability/v1/traces
19:21:04.728 fetched POST /api/observability/v1/traces 204
19:21:04.959 chunk 17 46817 bytes
19:21:04.961 in Ack 17
19:21:05.209 chunk 17 46847 bytes
19:21:05.210 in Ack 17
19:21:05.458 chunk 17 46907 bytes
19:21:05.459 in Ack 17
19:21:05.573 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/d718a4eb-de6e-4143-a1c0-8b7ed4d8e452","expiresAt":1794169265573}}
19:21:05.709 chunk 17 46952 bytes
19:21:05.710 in Ack 17
19:21:05.959 chunk 17 46982 bytes
19:21:05.960 in Ack 17
19:21:05.969 fetch POST /api/observability/v1/traces
19:21:05.969 fetched POST /api/observability/v1/traces 204
19:21:06.209 chunk 17 47012 bytes
19:21:06.210 in Ack 17
19:21:06.459 chunk 17 47042 bytes
19:21:06.460 in Ack 17
19:21:06.711 chunk 17 47102 bytes
19:21:06.712 in Ack 17
19:21:06.961 chunk 17 47132 bytes
19:21:06.962 in Ack 17
19:21:07.211 chunk 17 47162 bytes
19:21:07.211 fetch POST /api/observability/v1/traces
19:21:07.211 fetched POST /api/observability/v1/traces 204
19:21:07.213 in Ack 17
19:21:07.461 chunk 17 47207 bytes
19:21:07.462 in Ack 17
19:21:07.711 chunk 17 47252 bytes
19:21:07.712 in Ack 17
19:21:07.961 chunk 17 47267 bytes
19:21:07.962 in Ack 17
19:21:08.211 chunk 17 47297 bytes
19:21:08.212 in Ack 17
19:21:08.223 fetch POST /api/observability/v1/traces
19:21:08.223 fetched POST /api/observability/v1/traces 204
19:21:08.462 chunk 17 47327 bytes
19:21:08.463 in Ack 17
19:21:08.712 chunk 17 47387 bytes
19:21:08.713 in Ack 17
19:21:08.962 chunk 17 47417 bytes
19:21:08.963 in Ack 17
19:21:09.212 chunk 17 47447 bytes
19:21:09.213 in Ack 17
19:21:09.223 in Ping 
19:21:09.462 chunk 17 47492 bytes
19:21:09.463 in Ack 17
19:21:09.471 fetch POST /api/observability/v1/traces
19:21:09.471 fetched POST /api/observability/v1/traces 204
19:21:09.712 chunk 17 47537 bytes
19:21:09.713 in Ack 17
19:21:09.962 chunk 17 47582 bytes
19:21:09.963 in Ack 17
19:21:10.212 chunk 17 47609 bytes
19:21:10.213 in Ack 17
19:21:10.462 chunk 17 48614 bytes
19:21:10.463 in Ack 17
19:21:10.696 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:21:10.751 fetch POST /api/observability/v1/traces
19:21:10.751 fetched POST /api/observability/v1/traces 204
19:21:11.176 chunk 17 50689 bytes
19:21:11.178 in Ack 17
19:21:11.555 chunk 17 50739 bytes
19:21:11.557 in Ack 17
19:21:11.644 exit 33 {"_tag":"Success","value":{"sequence":97}}
19:21:11.806 chunk 17 53240 bytes
19:21:11.808 in Ack 17
19:21:11.992 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:21:12.060 chunk 17 53250 bytes
19:21:12.061 in Ack 17
19:21:12.179 fetch POST /api/observability/v1/traces
19:21:12.179 fetched POST /api/observability/v1/traces 204
19:21:12.310 chunk 17 53272 bytes
19:21:12.311 in Ack 17
19:21:12.560 chunk 17 53292 bytes
19:21:12.561 in Ack 17
19:21:12.810 chunk 17 53312 bytes
19:21:12.811 in Ack 17
19:21:13.060 chunk 17 53332 bytes
19:21:13.061 in Ack 17
19:21:13.311 chunk 17 53352 bytes
19:21:13.312 in Ack 17
19:21:13.331 fetch POST /api/observability/v1/traces
19:21:13.331 fetched POST /api/observability/v1/traces 204
19:21:13.562 chunk 17 53372 bytes
19:21:13.563 in Ack 17
19:21:13.811 chunk 17 53404 bytes
19:21:13.812 in Ack 17
19:21:14.155 chunk 17 53426 bytes
19:21:14.157 in Ack 17
19:21:14.223 in Ping 
19:21:14.407 chunk 17 53474 bytes
19:21:14.408 in Ack 17
19:21:14.564 fetch POST /api/observability/v1/traces
19:21:14.564 fetched POST /api/observability/v1/traces 204
19:21:14.657 chunk 17 53484 bytes
19:21:14.659 in Ack 17
19:21:14.907 chunk 17 53524 bytes
19:21:14.908 in Ack 17
19:21:15.106 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:21:15.316 chunk 17 53605 bytes
19:21:15.318 in Ack 17
19:21:15.567 chunk 17 53637 bytes
19:21:15.568 in Ack 17
19:21:15.661 fetch POST /api/observability/v1/traces
19:21:15.661 fetched POST /api/observability/v1/traces 204
19:21:15.822 chunk 17 53665 bytes
19:21:15.823 in Ack 17
19:21:16.073 chunk 17 53687 bytes
19:21:16.074 in Ack 17
19:21:16.322 chunk 17 54693 bytes
19:21:16.323 in Ack 17
19:21:16.573 chunk 17 54691 bytes
19:21:16.574 in Ack 17
19:21:16.823 chunk 11 2897 bytes
19:21:16.824 chunk 17 54721 bytes
19:21:16.824 in Ack 11
19:21:16.826 rpc 35 orchestration.dispatchCommand thread.visit
19:21:16.826 exit 35 {"_tag":"Success","value":{"sequence":0}}
19:21:16.826 fetch POST /api/observability/v1/traces
19:21:16.826 fetched POST /api/observability/v1/traces 204
19:21:16.827 in Ack 17
19:21:17.083 chunk 17 54741 bytes
19:21:17.085 in Ack 17
19:21:17.334 chunk 17 58316 bytes
19:21:17.335 in Ack 17
19:21:17.585 chunk 17 59770 bytes
19:21:17.586 in Ack 17
19:21:17.593 in Interrupt 27
19:21:17.619 rpc 36 subscribeVcsStatus
19:21:17.832 fetch POST /api/observability/v1/traces
19:21:17.832 fetched POST /api/observability/v1/traces 204
19:21:17.837 chunk 17 60831 bytes
19:21:17.839 in Ack 17
19:21:17.875 rpc 37 server.reportClientActivity
19:21:18.087 chunk 17 60851 bytes
19:21:18.088 in Ack 17
19:21:18.338 chunk 17 60871 bytes
19:21:18.339 in Ack 17
19:21:18.587 chunk 11 2897 bytes
19:21:18.588 in Ack 11
19:21:18.592 chunk 17 60911 bytes
19:21:18.594 in Ack 17
19:21:18.840 fetch POST /api/observability/v1/traces
19:21:18.840 fetched POST /api/observability/v1/traces 204
19:21:18.932 chunk 17 60931 bytes
19:21:18.932 chunk 36 980 bytes
19:21:18.934 in Ack 17
19:21:18.942 in Ack 36
19:21:19.182 chunk 17 60961 bytes
19:21:19.183 in Ack 17
19:21:19.223 in Ping 
19:21:19.433 chunk 11 2897 bytes
19:21:19.433 chunk 17 60989 bytes
19:21:19.434 in Ack 11
19:21:19.435 in Ack 17
19:21:19.683 chunk 17 61982 bytes
19:21:19.684 in Ack 17
19:21:19.934 chunk 17 61997 bytes
19:21:19.935 fetch POST /api/observability/v1/traces
19:21:19.935 fetched POST /api/observability/v1/traces 204
19:21:19.938 in Ack 17
19:21:20.184 chunk 17 62027 bytes
19:21:20.185 in Ack 17
19:21:20.434 chunk 17 62087 bytes
19:21:20.436 in Ack 17
19:21:20.685 chunk 17 62117 bytes
19:21:20.686 in Ack 17
19:21:20.935 chunk 17 62168 bytes
19:21:20.937 in Ack 17
19:21:21.014 fetch POST /api/observability/v1/traces
19:21:21.014 fetched POST /api/observability/v1/traces 204
19:21:21.186 chunk 17 62213 bytes
19:21:21.187 in Ack 17
19:21:21.436 chunk 17 62264 bytes
19:21:21.437 in Ack 17
19:21:21.686 chunk 17 62294 bytes
19:21:21.687 in Ack 17
19:21:21.936 chunk 17 62324 bytes
19:21:21.938 in Ack 17
19:21:22.186 chunk 17 62360 bytes
19:21:22.188 in Ack 17
19:21:22.236 fetch POST /api/observability/v1/traces
19:21:22.236 fetched POST /api/observability/v1/traces 204
19:21:22.237 exit 37 {"_tag":"Success","value":null}
19:21:22.436 chunk 17 62405 bytes
19:21:22.437 in Ack 17
19:21:22.686 chunk 17 62435 bytes
19:21:22.688 in Ack 17
19:21:22.936 chunk 17 62489 bytes
19:21:22.937 in Ack 17
19:21:23.186 chunk 17 62519 bytes
19:21:23.187 in Ack 17
19:21:23.238 fetch POST /api/observability/v1/traces
19:21:23.238 fetched POST /api/observability/v1/traces 204
19:21:23.436 chunk 17 62564 bytes
19:21:23.437 in Ack 17
19:21:23.686 chunk 17 62603 bytes
19:21:23.687 in Ack 17
19:21:23.851 rpc 38 server.reportClientActivity
19:21:23.937 chunk 17 62636 bytes
19:21:23.939 in Ack 17
19:21:24.187 chunk 17 62690 bytes
19:21:24.188 in Ack 17
19:21:24.303 in Ping 
19:21:24.437 chunk 17 62735 bytes
19:21:24.438 in Ack 17
19:21:24.470 fetch POST /api/observability/v1/traces
19:21:24.470 fetched POST /api/observability/v1/traces 204
19:21:24.687 chunk 17 62771 bytes
19:21:24.688 in Ack 17
19:21:24.938 chunk 17 62822 bytes
19:21:24.940 in Ack 17
19:21:25.187 chunk 17 62870 bytes
19:21:25.188 in Ack 17
19:21:25.445 chunk 17 62918 bytes
19:21:25.447 in Ack 17
19:21:25.690 fetch POST /api/observability/v1/traces
19:21:25.690 fetched POST /api/observability/v1/traces 204
19:21:25.696 chunk 17 63001 bytes
19:21:25.697 in Ack 17
19:21:26.196 chunk 17 63157 bytes
19:21:26.198 in Ack 17
19:21:26.446 chunk 17 63181 bytes
19:21:26.447 in Ack 17
19:21:26.699 fetch POST /api/observability/v1/traces
19:21:26.699 fetched POST /api/observability/v1/traces 204
19:21:26.848 rpc 39 orchestration.dispatchCommand thread.visit
19:21:26.848 exit 39 {"_tag":"Success","value":{"sequence":0}}
