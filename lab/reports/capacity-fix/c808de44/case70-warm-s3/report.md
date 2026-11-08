# case70, webapp, warm start, seed 3

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 65 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.5s |
| Prompt reached the agent | 1.3s |
| Time to first text | 3.3s |
| Whole reply shown | 64.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 9.7s / 5.1s / 8.7s |
| Longest silence | 8.9s |
| Approval shown after it was asked | 10.3s |
| Bytes on the wire down / up | 74284 / 34169 |
| Bytes the app received / sent | 56831 / 87775 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.5 | 165 | 348 |
| 20 | 6.8 | 473 | 750 |
| 30 | 7.4 | 852 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 6.3 | 1155 | 1402 |
| 60 | 5.7 | 1547 | 1596 |
| 64 | 9.7 | 1547 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 424/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456","checkpointScopes":"723/723","checkpoints":"613/613 371/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172614899,"silences":0,"longestSilenceMs":1631,"longestRecoveredGapMs":1631},"stats":{"sendRate":3375,"deliveredRate":3111.6744063820197,"loss":0,"srttMs":144.85225871607827,"minRttMs":1,"sentPackets":118,"sentWireBytes":33678,"receivedPackets":119,"receivedWireBytes":51385,"lastHeardMs":11023,"unsyncedBytes":509,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 352/849 910/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 285/395","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172624776,"silences":0,"longestSilenceMs":1631,"longestRecoveredGapMs":1631},"stats":{"sendRate":2333.755804786515,"deliveredRate":1759.2615950062236,"loss":0.33222506393861895,"srttMs":880.9381342570603,"minRttMs":1,"sentPackets":162,"sentWireBytes":42659,"receivedPackets":143,"receivedWireBytes":61455,"lastHeardMs":20899,"unsyncedBytes":0,"inflightPackets":17,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 748/858","user":"10/10 9/9","attempts":"661/661 661/661 500/637","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172634652,"silences":0,"longestSilenceMs":1631,"longestRecoveredGapMs":1631},"stats":{"sendRate":2498.125000000001,"deliveredRate":356.875,"loss":0.8571428571428572,"srttMs":1354.7650049685553,"minRttMs":1787,"sentPackets":233,"sentWireBytes":59423,"receivedPackets":175,"receivedWireBytes":71031,"lastHeardMs":30776,"unsyncedBytes":131,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 531/839","assistant":"32/32 21/21 1248/1293","user":"10/10 9/9 53/64","attempts":"661/661 661/661 603/637","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548 219/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172644845,"silences":0,"longestSilenceMs":1631,"longestRecoveredGapMs":1631},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.7676470588235293,"srttMs":2361.1723258588268,"minRttMs":1552,"sentPackets":292,"sentWireBytes":71291,"receivedPackets":207,"receivedWireBytes":81090,"lastHeardMs":40968,"unsyncedBytes":0,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 707/1550","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 600/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 53/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548 219/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172655903,"silences":1,"longestSilenceMs":2180,"longestRecoveredGapMs":2180},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.7731142857142856,"srttMs":2450.4007851264732,"minRttMs":1552,"sentPackets":349,"sentWireBytes":83620,"receivedPackets":226,"receivedWireBytes":91416,"lastHeardMs":52027,"unsyncedBytes":889,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 707/1550 81/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 726/839","assistant":"32/32 21/21 1340/1340 387/408","user":"10/10 9/9 53/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548 219/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779","runtimeRequests":"639/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172666602,"silences":2,"longestSilenceMs":2704,"longestRecoveredGapMs":2704},"stats":{"sendRate":4500,"deliveredRate":349.1019466185029,"loss":0.8858620689655172,"srttMs":2172.9902952765087,"minRttMs":1552,"sentPackets":398,"sentWireBytes":102660,"receivedPackets":241,"receivedWireBytes":100058,"lastHeardMs":62725,"unsyncedBytes":0,"inflightPackets":106,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1103/1103 870/870 1299/1299 1103/1103 1058/1058 849/849 348/1103 707/1550 1103/1103","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 726/839","assistant":"32/32 21/21 1340/1340 649/723","user":"10/10 9/9 53/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","nodes":"1022/1022 809/809 809/809","providerThreads":"412/809","providerSessions":"2685/2685","messages":"548/548 456/456 245/548 219/547","checkpointScopes":"723/723","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":224,"lastDataAt":1794172675534,"silences":3,"longestSilenceMs":3045,"longestRecoveredGapMs":3045},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.8202542319749215,"srttMs":2172.9902952765087,"minRttMs":1552,"sentPackets":448,"sentWireBytes":117081,"receivedPackets":255,"receivedWireBytes":106282,"lastHeardMs":71657,"unsyncedBytes":400,"inflightPackets":118,"duplicateBytes":0}}
- shim trace
21:16:44.181 fetch GET /api/auth/session
21:16:44.183 fetched GET /api/auth/session 200
21:16:44.388 fetch GET /.well-known/t3/environment
21:16:44.389 fetched GET /.well-known/t3/environment 200
21:16:44.399 fetch GET /.well-known/t3/environment
21:16:44.399 fetched GET /.well-known/t3/environment 200
21:16:44.405 rpc 0 subscribeServerConfig
21:16:44.406 chunk 0 38678 bytes
21:16:44.439 in Ack 0
21:16:44.441 rpc 1 subscribeServerLifecycle
21:16:44.441 rpc 2 subscribeServerLifecycle
21:16:44.444 fetch GET /api/auth/session
21:16:44.446 fetched GET /api/auth/session 200
21:16:44.446 rpc 3 subscribeDeviceState
21:16:44.451 rpc 4 preview.reportProfiles
21:16:44.495 rpc 5 subscribeProjectClones
21:16:44.504 fetch GET /api/orchestration/threads/mcp%3Aa0df8cc7-25c7-4fe3-81e9-fb88f6a98a7d/bounded
21:16:44.524 fetch GET /api/orchestration/shell
21:16:44.525 fetched GET /api/orchestration/shell 200
21:16:44.527 rpc 6 orchestration.subscribeShell
21:16:44.527 chunk 6 2779 bytes
21:16:44.549 rpc 7 server.refreshProviders
21:16:44.593 rpc 8 orchestration.dispatchCommand thread.visit
21:16:44.593 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:16:44.610 rpc 9 subscribeTerminalMetadata
21:16:44.611 rpc 10 subscribeVcsStatus
21:16:44.612 rpc 11 subscribePreviewEvents
21:16:44.614 rpc 12 assets.createUrl
21:16:44.614 rpc 13 preview.list
21:16:44.615 rpc 14 projects.readFile
21:16:44.616 in Ack 6
21:16:44.728 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"60926251-8f76-4176-8ffa-5d645c933ac4","sessions":[]}}
21:16:44.728 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:16:44.866 rpc 15 server.reportClientActivity
21:16:44.986 chunk 1 3861 bytes
21:16:45.016 in Ack 1
21:16:45.215 chunk 2 3861 bytes
21:16:45.215 in Ack 2
21:16:45.389 fetch POST /api/observability/v1/traces
21:16:45.389 fetched POST /api/observability/v1/traces 204
21:16:45.415 chunk 6 2755 bytes
21:16:45.415 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:16:31.528Z","configurableProviders":false,"con
21:16:45.415 chunk 10 472 bytes
21:16:45.415 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:16:45.417 in Ack 6
21:16:45.424 in Ack 10
21:16:45.911 chunk 6 2798 bytes
21:16:45.912 in Ack 6
21:16:45.919 chunk 3 876 bytes
21:16:45.919 exit 4 {"_tag":"Success","value":null}
21:16:45.919 chunk 5 4 bytes
21:16:45.919 chunk 9 36 bytes
21:16:45.919 exit 15 {"_tag":"Success","value":null}
21:16:45.920 in Ack 3
21:16:45.923 in Ack 5
21:16:45.923 in Ack 9
21:16:46.348 fetched GET /api/orchestration/threads/mcp%3Aa0df8cc7-25c7-4fe3-81e9-fb88f6a98a7d/bounded 200
21:16:46.354 rpc 16 orchestration.subscribeThread
21:16:46.354 chunk 16 8294 bytes
21:16:46.365 in Interrupt 10
21:16:46.367 rpc 17 subscribeVcsStatus
21:16:46.368 in Ack 16
21:16:46.430 fetch POST /api/observability/v1/traces
21:16:46.430 fetched POST /api/observability/v1/traces 204
21:16:46.539 chunk 17 472 bytes
21:16:46.541 in Ack 17
21:16:46.605 chunk 16 9348 bytes
21:16:46.606 in Ack 16
21:16:46.631 rpc 18 server.reportClientActivity
21:16:46.857 chunk 16 11932 bytes
21:16:46.859 in Ack 16
21:16:47.059 exit 18 {"_tag":"Success","value":null}
21:16:47.108 chunk 16 13517 bytes
21:16:47.109 in Ack 16
21:16:47.378 chunk 16 16202 bytes
21:16:47.380 in Ack 16
21:16:47.427 rpc 19 attachments.createUploadUrl
21:16:47.612 fetch POST /api/observability/v1/traces
21:16:47.612 fetched POST /api/observability/v1/traces 204
21:16:47.628 chunk 16 19081 bytes
21:16:47.629 in Ack 16
21:16:47.668 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-0bbe07b3-b807-4379-b870-1830f868c7dc","expiresAt":1794173207629,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:16:47.670 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTBiYmUwN2IzLWI4MDctNDM3OS1iODcwLTE4MzBmODY4YzdkYyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzIwNzYyOX0.4aLKqSzbNLZnslPmV1ulJx8XYYi2iCqh47ofHX4kSmc
21:16:47.878 chunk 16 20474 bytes
21:16:47.879 in Ack 16
21:16:47.886 in Interrupt 17
21:16:47.888 rpc 20 subscribeVcsStatus
21:16:47.938 chunk 20 472 bytes
21:16:47.938 in Ack 20
21:16:48.128 chunk 16 21197 bytes
21:16:48.129 in Ack 16
21:16:48.139 rpc 21 server.reportClientActivity
21:16:48.654 fetch POST /api/observability/v1/traces
21:16:48.654 fetched POST /api/observability/v1/traces 204
21:16:49.403 in Ping 
21:16:49.638 exit 21 {"_tag":"Success","value":null}
21:16:50.639 fetch POST /api/observability/v1/traces
21:16:50.639 fetched POST /api/observability/v1/traces 204
21:16:50.826 rpc 22 orchestration.dispatchCommand message.dispatch
21:16:51.480 exit 22 {"_tag":"Success","value":{"sequence":45}}
21:16:51.482 rpc 23 attachments.delete
21:16:51.825 fetch POST /api/observability/v1/traces
21:16:51.825 fetched POST /api/observability/v1/traces 204
21:16:51.981 chunk 16 23652 bytes
21:16:51.982 in Ack 16
21:16:52.088 exit 23 {"_tag":"Success","value":null}
21:16:52.231 chunk 6 2884 bytes
21:16:52.232 chunk 16 23626 bytes
21:16:52.258 in Ack 6
21:16:52.292 in Ack 16
21:16:52.481 chunk 16 23676 bytes
21:16:52.482 in Ack 16
21:16:52.732 chunk 6 2820 bytes
21:16:52.732 in Ack 6
21:16:52.734 rpc 24 orchestration.dispatchCommand thread.visit
21:16:52.734 exit 24 {"_tag":"Success","value":{"sequence":0}}
21:16:52.982 chunk 6 2820 bytes
21:16:52.982 chunk 16 23676 bytes
21:16:52.982 in Ack 6
21:16:52.988 fetch POST /api/observability/v1/traces
21:16:52.988 fetched POST /api/observability/v1/traces 204
21:16:52.989 in Ack 16
21:16:53.232 chunk 16 30526 bytes
21:16:53.237 in Ack 16
21:16:53.239 rpc 25 assets.createUrl
21:16:53.483 chunk 16 32116 bytes
21:16:53.484 in Ack 16
21:16:53.492 in Interrupt 20
21:16:53.494 rpc 26 subscribeVcsStatus
21:16:53.787 chunk 26 472 bytes
21:16:53.788 rpc 27 server.reportClientActivity
21:16:53.788 in Ack 26
21:16:53.993 fetch POST /api/observability/v1/traces
21:16:53.993 fetched POST /api/observability/v1/traces 204
21:16:54.038 chunk 16 31950 bytes
21:16:54.053 in Ack 16
21:16:54.408 in Ping 
21:16:54.855 rpc 28 orchestration.dispatchCommand message.dispatch
21:16:55.054 fetch POST /api/observability/v1/traces
21:16:55.055 fetched POST /api/observability/v1/traces 204
21:16:56.423 chunk 16 32727 bytes
21:16:56.424 in Ack 16
21:16:57.227 exit 28 {"_tag":"Success","value":{"sequence":75}}
21:16:57.425 fetch POST /api/observability/v1/traces
21:16:57.425 fetched POST /api/observability/v1/traces 204
21:16:57.556 chunk 16 35172 bytes
21:16:57.558 in Ack 16
21:16:57.993 chunk 16 35182 bytes
21:16:57.994 in Ack 16
21:16:58.345 chunk 16 35194 bytes
21:16:58.346 in Ack 16
21:16:58.559 fetch POST /api/observability/v1/traces
21:16:58.559 fetched POST /api/observability/v1/traces 204
21:16:58.774 chunk 16 35206 bytes
21:16:58.776 in Ack 16
21:16:59.125 chunk 16 35218 bytes
21:16:59.127 in Ack 16
21:16:59.409 in Ping 
21:16:59.567 chunk 16 35230 bytes
21:16:59.568 in Ack 16
21:16:59.776 fetch POST /api/observability/v1/traces
21:16:59.776 fetched POST /api/observability/v1/traces 204
21:17:00.170 chunk 16 35260 bytes
21:17:00.172 in Ack 16
21:17:00.694 chunk 16 35292 bytes
21:17:00.695 in Ack 16
21:17:01.172 fetch POST /api/observability/v1/traces
21:17:01.173 fetched POST /api/observability/v1/traces 204
21:17:01.379 chunk 16 35324 bytes
21:17:01.381 in Ack 16
21:17:01.710 chunk 16 35494 bytes
21:17:01.712 in Ack 16
21:17:02.141 chunk 16 35552 bytes
21:17:02.143 in Ack 16
21:17:02.382 fetch POST /api/observability/v1/traces
21:17:02.382 fetched POST /api/observability/v1/traces 204
21:17:02.570 chunk 16 35572 bytes
21:17:02.571 in Ack 16
21:17:02.905 chunk 16 35614 bytes
21:17:02.906 in Ack 16
21:17:03.155 chunk 16 35636 bytes
21:17:03.156 in Ack 16
21:17:03.414 chunk 16 35666 bytes
21:17:03.415 in Ack 16
21:17:03.574 fetch POST /api/observability/v1/traces
21:17:03.574 fetched POST /api/observability/v1/traces 204
21:17:03.767 chunk 16 35686 bytes
21:17:03.769 in Ack 16
21:17:04.019 chunk 16 35698 bytes
21:17:04.020 in Ack 16
21:17:04.409 in Ping 
21:17:04.518 chunk 16 35710 bytes
21:17:04.519 in Ack 16
21:17:04.770 fetch POST /api/observability/v1/traces
21:17:04.770 fetched POST /api/observability/v1/traces 204
21:17:04.777 chunk 16 35740 bytes
21:17:04.778 in Ack 16
21:17:05.035 chunk 16 35750 bytes
21:17:05.036 in Ack 16
21:17:05.294 chunk 16 35760 bytes
21:17:05.295 in Ack 16
21:17:05.593 chunk 16 35810 bytes
21:17:05.594 in Ack 16
21:17:05.779 fetch POST /api/observability/v1/traces
21:17:05.779 fetched POST /api/observability/v1/traces 204
21:17:05.849 chunk 16 35850 bytes
21:17:05.850 in Ack 16
21:17:06.450 chunk 16 35862 bytes
21:17:06.453 in Ack 16
21:17:06.851 fetch POST /api/observability/v1/traces
21:17:06.851 fetched POST /api/observability/v1/traces 204
21:17:06.960 chunk 16 35872 bytes
21:17:06.961 in Ack 16
21:17:07.301 chunk 16 35934 bytes
21:17:07.302 in Ack 16
21:17:07.559 chunk 16 35956 bytes
21:17:07.560 in Ack 16
21:17:07.816 chunk 16 35998 bytes
21:17:07.817 in Ack 16
21:17:07.926 fetch POST /api/observability/v1/traces
21:17:07.926 fetched POST /api/observability/v1/traces 204
21:17:08.070 chunk 16 36020 bytes
21:17:08.072 in Ack 16
21:17:08.349 chunk 16 36032 bytes
21:17:08.350 in Ack 16
21:17:08.788 in Interrupt 27
21:17:09.074 fetch POST /api/observability/v1/traces
21:17:09.074 fetched POST /api/observability/v1/traces 204
21:17:09.099 chunk 16 36196 bytes
21:17:09.100 in Ack 16
21:17:09.358 chunk 16 36264 bytes
21:17:09.359 in Ack 16
21:17:09.409 in Ping 
21:17:09.609 chunk 16 36282 bytes
21:17:09.610 in Ack 16
21:17:09.640 rpc 29 server.reportClientActivity
21:17:09.860 chunk 16 36292 bytes
21:17:09.861 in Ack 16
21:17:10.101 fetch POST /api/observability/v1/traces
21:17:10.101 fetched POST /api/observability/v1/traces 204
21:17:10.112 chunk 16 36302 bytes
21:17:10.113 in Ack 16
21:17:10.458 chunk 16 36312 bytes
21:17:10.460 in Ack 16
21:17:10.710 chunk 16 36350 bytes
21:17:10.711 in Ack 16
21:17:11.114 fetch POST /api/observability/v1/traces
21:17:11.114 fetched POST /api/observability/v1/traces 204
21:17:11.217 chunk 16 36360 bytes
21:17:11.218 in Ack 16
21:17:11.473 chunk 16 36402 bytes
21:17:11.474 in Ack 16
21:17:11.784 chunk 16 36424 bytes
21:17:11.785 in Ack 16
21:17:12.219 fetch POST /api/observability/v1/traces
21:17:12.219 fetched POST /api/observability/v1/traces 204
21:17:12.373 chunk 16 36436 bytes
21:17:12.374 in Ack 16
21:17:12.623 chunk 16 36456 bytes
21:17:12.624 in Ack 16
21:17:13.124 chunk 16 36466 bytes
21:17:13.126 in Ack 16
21:17:13.375 fetch POST /api/observability/v1/traces
21:17:13.375 fetched POST /api/observability/v1/traces 204
21:17:13.544 chunk 16 36486 bytes
21:17:13.546 in Ack 16
21:17:13.972 chunk 16 36538 bytes
21:17:13.973 in Ack 16
21:17:14.396 chunk 16 36616 bytes
21:17:14.397 in Ack 16
21:17:14.426 in Ping 
21:17:14.548 fetch POST /api/observability/v1/traces
21:17:14.548 fetched POST /api/observability/v1/traces 204
21:17:14.653 chunk 16 36628 bytes
21:17:14.655 in Ack 16
21:17:14.988 chunk 16 36658 bytes
21:17:14.989 in Ack 16
21:17:15.240 chunk 16 36688 bytes
21:17:15.241 in Ack 16
21:17:15.581 chunk 16 36710 bytes
21:17:15.583 in Ack 16
21:17:15.655 fetch POST /api/observability/v1/traces
21:17:15.655 fetched POST /api/observability/v1/traces 204
21:17:15.831 chunk 16 36790 bytes
21:17:15.832 in Ack 16
21:17:16.219 chunk 16 36820 bytes
21:17:16.220 in Ack 16
21:17:16.648 chunk 16 36942 bytes
21:17:16.649 in Ack 16
21:17:16.833 fetch POST /api/observability/v1/traces
21:17:16.833 fetched POST /api/observability/v1/traces 204
21:17:16.903 chunk 16 36954 bytes
21:17:16.904 in Ack 16
21:17:17.157 chunk 16 37002 bytes
21:17:17.160 in Ack 16
21:17:17.407 chunk 16 37050 bytes
21:17:17.408 in Ack 16
21:17:17.729 chunk 16 37080 bytes
21:17:17.730 in Ack 16
21:17:17.905 fetch POST /api/observability/v1/traces
21:17:17.906 fetched POST /api/observability/v1/traces 204
21:17:17.984 chunk 16 37090 bytes
21:17:17.985 in Ack 16
21:17:18.234 chunk 16 37118 bytes
21:17:18.235 in Ack 16
21:17:18.484 chunk 16 37148 bytes
21:17:18.485 in Ack 16
21:17:18.743 chunk 16 37196 bytes
21:17:18.744 in Ack 16
21:17:18.986 fetch POST /api/observability/v1/traces
21:17:18.986 fetched POST /api/observability/v1/traces 204
21:17:18.998 chunk 16 37208 bytes
21:17:18.999 in Ack 16
21:17:19.333 chunk 6 2884 bytes
21:17:19.334 chunk 16 37238 bytes
21:17:19.334 in Ack 6
21:17:19.336 rpc 30 orchestration.dispatchCommand thread.visit
21:17:19.336 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:17:19.379 in Ack 16
21:17:19.426 in Ping 
21:17:19.584 chunk 16 37278 bytes
21:17:19.585 in Ack 16
21:17:19.835 chunk 16 37288 bytes
21:17:19.836 in Ack 16
21:17:20.000 fetch POST /api/observability/v1/traces
21:17:20.000 fetched POST /api/observability/v1/traces 204
21:17:20.086 chunk 16 37298 bytes
21:17:20.087 in Ack 16
21:17:20.341 chunk 16 37346 bytes
21:17:20.342 in Ack 16
21:17:20.712 chunk 16 37356 bytes
21:17:20.715 in Ack 16
21:17:20.963 chunk 16 37366 bytes
21:17:20.964 in Ack 16
21:17:21.088 fetch POST /api/observability/v1/traces
21:17:21.088 fetched POST /api/observability/v1/traces 204
21:17:21.262 chunk 16 37406 bytes
21:17:21.263 in Ack 16
21:17:21.671 chunk 16 37448 bytes
21:17:21.672 in Ack 16
21:17:22.130 chunk 16 37468 bytes
21:17:22.132 in Ack 16
21:17:22.265 fetch POST /api/observability/v1/traces
21:17:22.265 fetched POST /api/observability/v1/traces 204
21:17:22.389 chunk 16 37488 bytes
21:17:22.390 in Ack 16
21:17:22.796 chunk 16 37500 bytes
21:17:22.797 in Ack 16
21:17:23.219 chunk 16 37550 bytes
21:17:23.220 in Ack 16
21:17:23.391 fetch POST /api/observability/v1/traces
21:17:23.391 fetched POST /api/observability/v1/traces 204
21:17:23.651 chunk 16 37570 bytes
21:17:23.652 in Ack 16
21:17:23.902 chunk 16 37590 bytes
21:17:23.903 in Ack 16
21:17:24.406 chunk 16 37600 bytes
21:17:24.407 in Ack 16
21:17:24.436 in Ping 
21:17:24.642 in Interrupt 29
21:17:24.653 fetch POST /api/observability/v1/traces
21:17:24.653 fetched POST /api/observability/v1/traces 204
21:17:24.846 chunk 16 37620 bytes
21:17:24.847 in Ack 16
21:17:25.267 chunk 16 37652 bytes
21:17:25.268 in Ack 16
21:17:25.848 fetch POST /api/observability/v1/traces
21:17:25.848 fetched POST /api/observability/v1/traces 204
21:17:25.883 chunk 16 37670 bytes
21:17:25.884 in Ack 16
21:17:26.502 chunk 16 37678 bytes
21:17:26.503 in Ack 16
21:17:26.887 fetch POST /api/observability/v1/traces
21:17:26.887 fetched POST /api/observability/v1/traces 204
21:17:27.777 chunk 16 37734 bytes
21:17:27.778 in Ack 16
21:17:27.941 fetch POST /api/observability/v1/traces
21:17:27.941 fetched POST /api/observability/v1/traces 204
21:17:29.100 chunk 16 37802 bytes
21:17:29.101 in Ack 16
21:17:29.436 in Ping 
21:17:30.103 fetch POST /api/observability/v1/traces
21:17:30.103 fetched POST /api/observability/v1/traces 204
21:17:33.155 chunk 16 38810 bytes
21:17:33.156 in Ack 16
21:17:33.314 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:17:34.157 fetch POST /api/observability/v1/traces
21:17:34.157 fetched POST /api/observability/v1/traces 204
21:17:34.436 in Ping 
21:17:34.642 rpc 32 server.reportClientActivity
21:17:35.335 chunk 16 39448 bytes
21:17:35.336 in Ack 16
21:17:35.642 fetch POST /api/observability/v1/traces
21:17:35.642 fetched POST /api/observability/v1/traces 204
21:17:35.903 exit 27 {"_tag":"Success","value":null}
21:17:37.571 chunk 16 41959 bytes
21:17:37.572 exit 31 {"_tag":"Success","value":{"sequence":96}}
21:17:37.573 in Ack 16
21:17:37.945 fetch POST /api/observability/v1/traces
21:17:37.945 fetched POST /api/observability/v1/traces 204
21:17:38.088 chunk 16 42009 bytes
21:17:38.089 in Ack 16
21:17:38.242 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:17:38.773 chunk 16 42081 bytes
21:17:38.774 in Ack 16
21:17:39.090 fetch POST /api/observability/v1/traces
21:17:39.090 fetched POST /api/observability/v1/traces 204
21:17:39.213 chunk 16 42091 bytes
21:17:39.214 in Ack 16
21:17:39.440 in Ping 
21:17:40.215 fetch POST /api/observability/v1/traces
21:17:40.215 fetched POST /api/observability/v1/traces 204
21:17:40.564 chunk 16 42103 bytes
21:17:40.565 in Ack 16
21:17:41.094 chunk 16 42123 bytes
21:17:41.095 in Ack 16
21:17:41.567 fetch POST /api/observability/v1/traces
21:17:41.567 fetched POST /api/observability/v1/traces 204
21:17:43.798 chunk 16 42283 bytes
21:17:43.799 in Ack 16
21:17:44.329 chunk 16 42303 bytes
21:17:44.330 in Ack 16
21:17:44.443 in Ping 
21:17:44.583 chunk 16 42411 bytes
21:17:44.584 in Ack 16
21:17:44.801 fetch POST /api/observability/v1/traces
21:17:44.801 fetched POST /api/observability/v1/traces 204
21:17:45.104 chunk 16 42451 bytes
21:17:45.105 in Ack 16
21:17:45.631 chunk 16 42481 bytes
21:17:45.633 in Ack 16
21:17:46.066 chunk 16 42621 bytes
21:17:46.067 in Ack 16
21:17:46.106 fetch POST /api/observability/v1/traces
21:17:46.106 fetched POST /api/observability/v1/traces 204
21:17:46.603 chunk 16 42663 bytes
21:17:46.604 in Ack 16
21:17:47.285 chunk 16 42777 bytes
21:17:47.286 in Ack 16
21:17:47.535 chunk 16 42787 bytes
21:17:47.536 in Ack 16
21:17:47.605 fetch POST /api/observability/v1/traces
21:17:47.605 fetched POST /api/observability/v1/traces 204
21:17:48.016 chunk 16 42843 bytes
21:17:48.017 in Ack 16
21:17:48.454 chunk 16 42879 bytes
21:17:48.455 in Ack 16
21:17:48.967 chunk 16 42955 bytes
21:17:48.969 in Ack 16
21:17:49.053 fetch POST /api/observability/v1/traces
21:17:49.053 fetched POST /api/observability/v1/traces 204
21:17:49.223 chunk 16 42975 bytes
21:17:49.224 in Ack 16
21:17:49.447 in Ping 
21:17:49.475 chunk 16 43023 bytes
21:17:49.476 in Ack 16
21:17:49.642 in Interrupt 32
21:17:50.226 fetch POST /api/observability/v1/traces
21:17:50.226 fetched POST /api/observability/v1/traces 204
21:17:52.519 chunk 16 43035 bytes
21:17:52.519 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:17:52.520 in Ack 16
21:17:52.996 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:17:53.485 chunk 16 43071 bytes
21:17:53.486 in Ack 16
21:17:53.527 fetch POST /api/observability/v1/traces
21:17:53.527 fetched POST /api/observability/v1/traces 204
21:17:53.992 chunk 16 43223 bytes
21:17:53.993 in Ack 16
21:17:54.451 in Ping 
21:17:54.994 fetch POST /api/observability/v1/traces
21:17:54.994 fetched POST /api/observability/v1/traces 204
21:17:55.535 chunk 16 43266 bytes
21:17:55.536 in Ack 16
21:17:56.536 fetch POST /api/observability/v1/traces
21:17:56.536 fetched POST /api/observability/v1/traces 204
21:17:57.951 fetch POST /api/observability/v1/traces
21:17:57.951 fetched POST /api/observability/v1/traces 204
21:17:58.231 chunk 16 43428 bytes
21:17:58.232 in Ack 16
21:17:59.255 fetch POST /api/observability/v1/traces
21:17:59.255 fetched POST /api/observability/v1/traces 204
