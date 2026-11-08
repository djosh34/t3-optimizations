# case70, webapp, warm start, seed 5

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 80 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 7.1s |
| Prompt reached the agent | 2.6s |
| Time to first text | 7.5s |
| Whole reply shown | 79.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 38.9s / 9.0s / 34.9s |
| Longest silence | 41.8s |
| Approval shown after it was asked | 9.7s |
| Bytes on the wire down / up | 98874 / 41648 |
| Bytes the app received / sent | 62021 / 79275 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.0 | 53 | 299 |
| 20 | 9.7 | 307 | 697 |
| 30 | 10.6 | 667 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 9.9 | 1077 | 1461 |
| 60 | 19.9 | 1077 | 1596 |
| 70 | 29.9 | 1077 | 1596 |
| 79 | 38.9 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 308/901 154/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172807595,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":6750,"deliveredRate":1866.3716814159293,"loss":0,"srttMs":5.027255693237108,"minRttMs":1,"sentPackets":127,"sentWireBytes":37388,"receivedPackets":102,"receivedWireBytes":50830,"lastHeardMs":9292,"unsyncedBytes":998,"inflightPackets":22,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 166/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 65/65","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172818112,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":1687.5,"deliveredRate":1613.3155650348256,"loss":0.22625,"srttMs":3520.8904646674796,"minRttMs":1,"sentPackets":155,"sentWireBytes":45649,"receivedPackets":123,"receivedWireBytes":62035,"lastHeardMs":19809,"unsyncedBytes":0,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 589/849 991/1078","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 598/733","user":"10/10 9/9","attempts":"661/661 661/661 52/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172828305,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":2531.25,"deliveredRate":617.2413793103449,"loss":0.7,"srttMs":3888.072387011039,"minRttMs":1,"sentPackets":215,"sentWireBytes":58995,"receivedPackets":150,"receivedWireBytes":72163,"lastHeardMs":30002,"unsyncedBytes":309,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 835/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 55/839","assistant":"32/32 21/21 1173/1278","user":"10/10 9/9 51/64","attempts":"661/661 661/661 342/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172838414,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":1449.9925189984554,"deliveredRate":837.4709187624278,"loss":0.6416570813397128,"srttMs":4147.642730363286,"minRttMs":1,"sentPackets":278,"sentWireBytes":71921,"receivedPackets":185,"receivedWireBytes":82659,"lastHeardMs":40111,"unsyncedBytes":0,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 966/1550","thread":"951/951","runs":"1014/1014 971/971 55/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 51/64","attempts":"661/661 661/661 416/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172848884,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":1449.9925189984554,"deliveredRate":0,"loss":0.6691009650717702,"srttMs":4147.642730363286,"minRttMs":1,"sentPackets":331,"sentWireBytes":81642,"receivedPackets":209,"receivedWireBytes":94351,"lastHeardMs":50581,"unsyncedBytes":889,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 1146/1550 648/929","thread":"951/951","runs":"1014/1014 971/971 55/839","assistant":"32/32 21/21 1340/1340 145/228","user":"10/10 9/9 51/64","attempts":"661/661 661/661 542/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172859748,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":1087.4943892488416,"deliveredRate":345.05494505494505,"loss":0.7894736842105263,"srttMs":4738.523340434391,"minRttMs":1,"sentPackets":356,"sentWireBytes":95220,"receivedPackets":237,"receivedWireBytes":106773,"lastHeardMs":61445,"unsyncedBytes":639,"inflightPackets":88,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 1146/1550 648/929","thread":"951/951","runs":"1014/1014 971/971 55/839","assistant":"32/32 21/21 1340/1340 155/238","user":"10/10 9/9 51/64","attempts":"661/661 661/661 605/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172860173,"silences":0,"longestSilenceMs":1099,"longestRecoveredGapMs":1099},"stats":{"sendRate":1087.4943892488416,"deliveredRate":345.05494505494505,"loss":0.7894736842105263,"srttMs":4738.523340434391,"minRttMs":1,"sentPackets":361,"sentWireBytes":97537,"receivedPackets":238,"receivedWireBytes":107272,"lastHeardMs":61870,"unsyncedBytes":639,"inflightPackets":81,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 849/849 1078/1078 1146/1550 648/929","thread":"951/951","runs":"1014/1014 971/971 55/839","assistant":"32/32 21/21 1340/1340 272/723","user":"10/10 9/9 51/64","attempts":"661/661 661/661 605/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 235/1022","messages":"456/456 548/548 901/901 268/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":289,"lastDataAt":1794172880991,"silences":1,"longestSilenceMs":20818,"longestRecoveredGapMs":20818},"stats":{"sendRate":1087.4943892488416,"deliveredRate":271.1804663117312,"loss":0.8071770334928229,"srttMs":4800.957922880092,"minRttMs":3128,"sentPackets":374,"sentWireBytes":104665,"receivedPackets":239,"receivedWireBytes":107562,"lastHeardMs":82688,"unsyncedBytes":0,"inflightPackets":81,"duplicateBytes":0}}
- shim trace
21:19:58.549 fetch GET /api/auth/session
21:19:58.550 fetched GET /api/auth/session 200
21:19:58.650 fetch GET /.well-known/t3/environment
21:19:58.651 fetched GET /.well-known/t3/environment 200
21:19:58.658 fetch GET /.well-known/t3/environment
21:19:58.658 fetched GET /.well-known/t3/environment 200
21:19:58.661 rpc 0 subscribeServerConfig
21:19:58.662 chunk 0 38678 bytes
21:19:58.689 in Ack 0
21:19:58.690 rpc 1 subscribeServerLifecycle
21:19:58.691 rpc 2 subscribeServerLifecycle
21:19:58.693 fetch GET /api/auth/session
21:19:58.694 fetched GET /api/auth/session 200
21:19:58.694 rpc 3 subscribeDeviceState
21:19:58.696 rpc 4 preview.reportProfiles
21:19:58.719 rpc 5 subscribeProjectClones
21:19:58.737 fetch GET /api/orchestration/threads/mcp%3A26263207-a27c-41ae-85b5-a1e58f4bdd1a/bounded
21:19:58.742 fetch GET /api/orchestration/shell
21:19:58.743 fetched GET /api/orchestration/shell 200
21:19:58.744 rpc 6 orchestration.subscribeShell
21:19:58.745 chunk 6 2779 bytes
21:19:58.763 rpc 7 server.refreshProviders
21:19:58.797 rpc 8 orchestration.dispatchCommand thread.visit
21:19:58.797 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:19:58.810 rpc 9 subscribeTerminalMetadata
21:19:58.811 rpc 10 subscribeVcsStatus
21:19:58.812 rpc 11 subscribePreviewEvents
21:19:58.813 rpc 12 assets.createUrl
21:19:58.814 rpc 13 preview.list
21:19:58.814 rpc 14 projects.readFile
21:19:58.816 in Ack 6
21:19:58.995 chunk 6 2755 bytes
21:19:58.995 in Ack 6
21:19:59.065 rpc 15 server.reportClientActivity
21:19:59.192 chunk 1 3861 bytes
21:19:59.212 in Ack 1
21:19:59.439 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"54c9af21-96db-4619-b071-60bf9efc6893","sessions":[]}}
21:19:59.439 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:19:59.650 fetch POST /api/observability/v1/traces
21:19:59.650 fetched POST /api/observability/v1/traces 204
21:19:59.689 chunk 2 3861 bytes
21:19:59.689 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:19:46.932Z","configurableProviders":false,"con
21:19:59.689 chunk 10 472 bytes
21:19:59.690 in Ack 2
21:19:59.693 in Ack 10
21:19:59.889 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:20:00.139 chunk 6 2798 bytes
21:20:00.140 in Ack 6
21:20:00.287 chunk 3 876 bytes
21:20:00.287 exit 4 {"_tag":"Success","value":null}
21:20:00.287 chunk 5 4 bytes
21:20:00.287 chunk 9 36 bytes
21:20:00.288 in Ack 3
21:20:00.291 rpc 16 preview.reportProfiles
21:20:00.292 in Ack 5
21:20:00.292 in Ack 9
21:20:00.388 exit 15 {"_tag":"Success","value":null}
21:20:00.388 exit 16 {"_tag":"Success","value":null}
21:20:00.691 fetch POST /api/observability/v1/traces
21:20:00.691 fetched POST /api/observability/v1/traces 204
21:20:00.738 fetched GET /api/orchestration/threads/mcp%3A26263207-a27c-41ae-85b5-a1e58f4bdd1a/bounded 200
21:20:00.741 rpc 17 orchestration.subscribeThread
21:20:00.741 chunk 17 8294 bytes
21:20:00.750 in Interrupt 10
21:20:00.752 rpc 18 subscribeVcsStatus
21:20:00.753 in Ack 17
21:20:00.888 chunk 18 472 bytes
21:20:00.888 in Ack 18
21:20:00.992 chunk 17 11271 bytes
21:20:00.993 in Ack 17
21:20:01.023 rpc 19 server.reportClientActivity
21:20:01.238 exit 19 {"_tag":"Success","value":null}
21:20:01.242 chunk 17 12708 bytes
21:20:01.245 in Ack 17
21:20:01.499 rpc 20 attachments.createUploadUrl
21:20:01.654 chunk 17 13517 bytes
21:20:01.662 in Ack 17
21:20:01.708 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-bbb0871f-1943-48ac-a3ee-2c44849a830e","expiresAt":1794173401688,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:20:01.709 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWJiYjA4NzFmLTE5NDMtNDhhYy1hM2VlLTJjNDQ4NDlhODMwZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzQwMTY4OH0.FHDHGn68xRMwWwf3kjwwbW0GHPDjTbsk3RTNj6YW9eM
21:20:01.740 fetch POST /api/observability/v1/traces
21:20:01.740 fetched POST /api/observability/v1/traces 204
21:20:01.905 chunk 17 17224 bytes
21:20:01.906 in Ack 17
21:20:02.156 chunk 17 19081 bytes
21:20:02.157 in Ack 17
21:20:02.412 chunk 17 21197 bytes
21:20:02.414 in Ack 17
21:20:02.425 in Interrupt 18
21:20:02.427 rpc 21 subscribeVcsStatus
21:20:02.534 chunk 21 472 bytes
21:20:02.535 in Ack 21
21:20:02.678 rpc 22 server.reportClientActivity
21:20:02.907 fetch POST /api/observability/v1/traces
21:20:02.907 fetched POST /api/observability/v1/traces 204
21:20:03.017 exit 22 {"_tag":"Success","value":null}
21:20:03.662 in Ping 
21:20:04.017 fetch POST /api/observability/v1/traces
21:20:04.017 fetched POST /api/observability/v1/traces 204
21:20:04.682 rpc 23 orchestration.dispatchCommand message.dispatch
21:20:05.389 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:20:05.391 rpc 24 attachments.delete
21:20:05.681 fetch POST /api/observability/v1/traces
21:20:05.681 fetched POST /api/observability/v1/traces 204
21:20:05.890 chunk 17 23662 bytes
21:20:05.890 in Ack 17
21:20:06.139 chunk 17 23674 bytes
21:20:06.140 in Ack 17
21:20:06.298 exit 24 {"_tag":"Success","value":null}
21:20:06.390 chunk 17 23676 bytes
21:20:06.391 in Ack 17
21:20:06.639 chunk 6 2820 bytes
21:20:06.640 in Ack 6
21:20:06.641 rpc 25 orchestration.dispatchCommand thread.visit
21:20:06.641 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:20:06.679 chunk 17 24648 bytes
21:20:06.684 in Ack 17
21:20:06.889 chunk 6 2820 bytes
21:20:06.889 in Ack 6
21:20:06.894 fetch POST /api/observability/v1/traces
21:20:06.894 fetched POST /api/observability/v1/traces 204
21:20:06.928 chunk 17 30526 bytes
21:20:06.930 in Ack 17
21:20:06.932 rpc 26 assets.createUrl
21:20:07.179 chunk 17 32116 bytes
21:20:07.179 in Ack 17
21:20:07.185 in Interrupt 21
21:20:07.187 rpc 27 subscribeVcsStatus
21:20:07.248 chunk 27 472 bytes
21:20:07.248 in Ack 27
21:20:07.594 chunk 17 31950 bytes
21:20:07.595 rpc 28 server.reportClientActivity
21:20:07.596 in Ack 17
21:20:07.848 chunk 17 32727 bytes
21:20:07.850 in Ack 17
21:20:07.931 fetch POST /api/observability/v1/traces
21:20:07.931 fetched POST /api/observability/v1/traces 204
21:20:08.330 rpc 29 orchestration.dispatchCommand message.dispatch
21:20:08.662 in Ping 
21:20:09.330 fetch POST /api/observability/v1/traces
21:20:09.330 fetched POST /api/observability/v1/traces 204
21:20:11.950 chunk 17 32727 bytes
21:20:11.951 in Ack 17
21:20:12.256 fetch POST /api/observability/v1/traces
21:20:12.256 fetched POST /api/observability/v1/traces 204
21:20:13.170 chunk 17 33638 bytes
21:20:13.171 in Ack 17
21:20:13.662 in Ping 
21:20:14.172 fetch POST /api/observability/v1/traces
21:20:14.172 fetched POST /api/observability/v1/traces 204
21:20:15.140 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:20:15.638 chunk 17 36093 bytes
21:20:15.639 in Ack 17
21:20:16.141 fetch POST /api/observability/v1/traces
21:20:16.141 fetched POST /api/observability/v1/traces 204
21:20:16.227 chunk 17 36143 bytes
21:20:16.228 in Ack 17
21:20:17.055 chunk 17 36203 bytes
21:20:17.056 in Ack 17
21:20:17.229 fetch POST /api/observability/v1/traces
21:20:17.229 fetched POST /api/observability/v1/traces 204
21:20:18.663 in Ping 
21:20:19.935 chunk 17 36315 bytes
21:20:19.936 in Ack 17
21:20:20.937 fetch POST /api/observability/v1/traces
21:20:20.937 fetched POST /api/observability/v1/traces 204
21:20:21.600 chunk 17 36387 bytes
21:20:21.602 in Ack 17
21:20:22.055 chunk 17 36427 bytes
21:20:22.058 in Ack 17
21:20:22.259 fetch POST /api/observability/v1/traces
21:20:22.259 fetched POST /api/observability/v1/traces 204
21:20:22.478 chunk 17 36457 bytes
21:20:22.480 in Ack 17
21:20:22.596 in Interrupt 28
21:20:22.913 chunk 17 36499 bytes
21:20:22.914 in Ack 17
21:20:23.348 chunk 17 36541 bytes
21:20:23.349 exit 28 {"_tag":"Success","value":null}
21:20:23.350 in Ack 17
21:20:23.484 fetch POST /api/observability/v1/traces
21:20:23.484 fetched POST /api/observability/v1/traces 204
21:20:23.663 in Ping 
21:20:23.775 chunk 17 36573 bytes
21:20:23.776 in Ack 17
21:20:23.902 rpc 30 server.reportClientActivity
21:20:24.110 chunk 17 36595 bytes
21:20:24.111 in Ack 17
21:20:24.444 chunk 17 36615 bytes
21:20:24.445 in Ack 17
21:20:24.777 fetch POST /api/observability/v1/traces
21:20:24.777 fetched POST /api/observability/v1/traces 204
21:20:25.043 chunk 17 36895 bytes
21:20:25.045 in Ack 17
21:20:25.298 chunk 17 37015 bytes
21:20:25.300 in Ack 17
21:20:25.550 chunk 17 37025 bytes
21:20:25.552 in Ack 17
21:20:25.806 chunk 17 37055 bytes
21:20:25.807 in Ack 17
21:20:26.045 fetch POST /api/observability/v1/traces
21:20:26.045 fetched POST /api/observability/v1/traces 204
21:20:26.064 chunk 17 37065 bytes
21:20:26.065 in Ack 17
21:20:26.578 chunk 17 37075 bytes
21:20:26.579 in Ack 17
21:20:26.831 chunk 17 37095 bytes
21:20:26.832 in Ack 17
21:20:27.066 fetch POST /api/observability/v1/traces
21:20:27.066 fetched POST /api/observability/v1/traces 204
21:20:27.082 chunk 17 37115 bytes
21:20:27.083 in Ack 17
21:20:27.803 chunk 17 37261 bytes
21:20:27.806 in Ack 17
21:20:28.062 chunk 17 37271 bytes
21:20:28.063 in Ack 17
21:20:28.094 fetch POST /api/observability/v1/traces
21:20:28.094 fetched POST /api/observability/v1/traces 204
21:20:28.312 chunk 17 37283 bytes
21:20:28.313 in Ack 17
21:20:28.651 chunk 17 37397 bytes
21:20:28.652 in Ack 17
21:20:28.663 in Ping 
21:20:29.044 chunk 17 37465 bytes
21:20:29.045 in Ack 17
21:20:29.299 chunk 17 37519 bytes
21:20:29.301 in Ack 17
21:20:29.334 fetch POST /api/observability/v1/traces
21:20:29.334 fetched POST /api/observability/v1/traces 204
21:20:29.555 chunk 17 37531 bytes
21:20:29.556 in Ack 17
21:20:29.806 chunk 17 37541 bytes
21:20:29.807 in Ack 17
21:20:30.314 chunk 17 37551 bytes
21:20:30.315 in Ack 17
21:20:30.557 fetch POST /api/observability/v1/traces
21:20:30.557 fetched POST /api/observability/v1/traces 204
21:20:30.564 chunk 17 37561 bytes
21:20:30.565 in Ack 17
21:20:30.890 chunk 17 37633 bytes
21:20:30.891 in Ack 17
21:20:31.146 chunk 17 37713 bytes
21:20:31.147 in Ack 17
21:20:31.401 chunk 17 37723 bytes
21:20:31.402 in Ack 17
21:20:31.566 fetch POST /api/observability/v1/traces
21:20:31.566 fetched POST /api/observability/v1/traces 204
21:20:31.659 chunk 17 37703 bytes
21:20:31.661 in Ack 17
21:20:31.910 chunk 17 37725 bytes
21:20:31.911 in Ack 17
21:20:32.301 chunk 17 37745 bytes
21:20:32.302 in Ack 17
21:20:32.645 chunk 17 37795 bytes
21:20:32.646 in Ack 17
21:20:32.662 fetch POST /api/observability/v1/traces
21:20:32.662 fetched POST /api/observability/v1/traces 204
21:20:32.903 chunk 17 37823 bytes
21:20:32.905 in Ack 17
21:20:33.247 chunk 17 37835 bytes
21:20:33.249 in Ack 17
21:20:33.664 in Ping 
21:20:33.668 chunk 17 37873 bytes
21:20:33.669 in Ack 17
21:20:33.906 fetch POST /api/observability/v1/traces
21:20:33.906 fetched POST /api/observability/v1/traces 204
21:20:33.922 chunk 17 37883 bytes
21:20:33.923 in Ack 17
21:20:34.178 chunk 17 37935 bytes
21:20:34.179 in Ack 17
21:20:34.433 chunk 17 37955 bytes
21:20:34.434 in Ack 17
21:20:34.806 chunk 17 37983 bytes
21:20:34.808 in Ack 17
21:20:34.924 fetch POST /api/observability/v1/traces
21:20:34.924 fetched POST /api/observability/v1/traces 204
21:20:35.057 chunk 17 37993 bytes
21:20:35.058 in Ack 17
21:20:35.311 chunk 17 38013 bytes
21:20:35.312 in Ack 17
21:20:35.638 chunk 17 38053 bytes
21:20:35.639 in Ack 17
21:20:35.889 chunk 17 38093 bytes
21:20:35.890 in Ack 17
21:20:36.059 fetch POST /api/observability/v1/traces
21:20:36.059 fetched POST /api/observability/v1/traces 204
21:20:36.212 chunk 17 38113 bytes
21:20:36.213 in Ack 17
21:20:36.467 chunk 17 38123 bytes
21:20:36.468 in Ack 17
21:20:36.723 chunk 17 38143 bytes
21:20:36.724 in Ack 17
21:20:36.981 chunk 17 38233 bytes
21:20:36.982 in Ack 17
21:20:37.214 fetch POST /api/observability/v1/traces
21:20:37.214 fetched POST /api/observability/v1/traces 204
21:20:37.315 chunk 17 38275 bytes
21:20:37.317 in Ack 17
21:20:37.574 chunk 17 38295 bytes
21:20:37.575 in Ack 17
21:20:37.911 chunk 17 38323 bytes
21:20:37.912 in Ack 17
21:20:38.170 chunk 17 38363 bytes
21:20:38.171 in Ack 17
21:20:38.317 fetch POST /api/observability/v1/traces
21:20:38.317 fetched POST /api/observability/v1/traces 204
21:20:38.420 chunk 17 38385 bytes
21:20:38.421 in Ack 17
21:20:38.664 in Ping 
21:20:38.671 chunk 17 38403 bytes
21:20:38.672 in Ack 17
21:20:38.903 in Interrupt 30
21:20:38.926 chunk 17 38413 bytes
21:20:38.927 in Ack 17
21:20:39.185 chunk 17 38465 bytes
21:20:39.186 in Ack 17
21:20:39.422 fetch POST /api/observability/v1/traces
21:20:39.422 fetched POST /api/observability/v1/traces 204
21:20:39.443 chunk 17 38477 bytes
21:20:39.444 in Ack 17
21:20:39.979 chunk 17 38611 bytes
21:20:39.981 in Ack 17
21:20:40.445 fetch POST /api/observability/v1/traces
21:20:40.445 fetched POST /api/observability/v1/traces 204
21:20:41.586 chunk 17 38631 bytes
21:20:41.588 in Ack 17
21:20:42.128 chunk 17 38649 bytes
21:20:42.130 in Ack 17
21:20:42.267 fetch POST /api/observability/v1/traces
21:20:42.267 fetched POST /api/observability/v1/traces 204
21:20:42.912 chunk 17 38657 bytes
21:20:42.913 in Ack 17
21:20:43.534 chunk 17 38665 bytes
21:20:43.535 in Ack 17
21:20:43.664 in Ping 
21:20:43.915 fetch POST /api/observability/v1/traces
21:20:43.915 fetched POST /api/observability/v1/traces 204
21:20:45.581 chunk 17 38713 bytes
21:20:45.582 in Ack 17
21:20:46.584 fetch POST /api/observability/v1/traces
21:20:46.584 fetched POST /api/observability/v1/traces 204
21:20:47.249 chunk 17 39721 bytes
21:20:47.251 in Ack 17
21:20:47.503 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:20:48.252 fetch POST /api/observability/v1/traces
21:20:48.252 fetched POST /api/observability/v1/traces 204
21:20:48.666 in Ping 
21:20:48.903 rpc 32 server.reportClientActivity
21:20:49.903 fetch POST /api/observability/v1/traces
21:20:49.903 fetched POST /api/observability/v1/traces 204
21:20:52.270 fetch POST /api/observability/v1/traces
21:20:52.270 fetched POST /api/observability/v1/traces 204
21:20:53.668 in Ping 
21:20:54.549 chunk 17 42188 bytes
21:20:54.549 exit 31 {"_tag":"Success","value":{"sequence":95}}
21:20:54.550 in Ack 17
21:20:54.813 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:20:54.987 chunk 17 42198 bytes
21:20:54.988 in Ack 17
21:20:55.411 chunk 17 42218 bytes
21:20:55.412 in Ack 17
21:20:55.551 fetch POST /api/observability/v1/traces
21:20:55.551 fetched POST /api/observability/v1/traces 204
21:20:55.846 chunk 17 42238 bytes
21:20:55.847 in Ack 17
21:20:56.285 chunk 17 42248 bytes
21:20:56.286 in Ack 17
21:20:56.724 chunk 17 42270 bytes
21:20:56.725 in Ack 17
21:20:56.848 fetch POST /api/observability/v1/traces
21:20:56.848 fetched POST /api/observability/v1/traces 204
21:20:57.142 chunk 17 42300 bytes
21:20:57.144 in Ack 17
21:20:57.564 chunk 17 42320 bytes
21:20:57.566 in Ack 17
21:20:58.002 chunk 17 42360 bytes
21:20:58.003 in Ack 17
21:20:58.146 fetch POST /api/observability/v1/traces
21:20:58.146 fetched POST /api/observability/v1/traces 204
21:20:58.436 chunk 17 42370 bytes
21:20:58.438 in Ack 17
21:20:58.671 in Ping 
21:20:58.874 chunk 17 42382 bytes
21:20:58.875 in Ack 17
21:20:59.312 chunk 17 42412 bytes
21:20:59.313 in Ack 17
21:20:59.438 fetch POST /api/observability/v1/traces
21:20:59.439 fetched POST /api/observability/v1/traces 204
21:20:59.749 chunk 17 42454 bytes
21:20:59.750 in Ack 17
21:21:00.174 chunk 17 42474 bytes
21:21:00.175 in Ack 17
21:21:00.751 fetch POST /api/observability/v1/traces
21:21:00.751 fetched POST /api/observability/v1/traces 204
21:21:02.273 fetch POST /api/observability/v1/traces
21:21:02.273 fetched POST /api/observability/v1/traces 204
21:21:03.676 in Ping 
21:21:03.905 in Interrupt 32
21:21:04.907 fetch POST /api/observability/v1/traces
21:21:04.907 fetched POST /api/observability/v1/traces 204
21:21:08.679 in Ping 
21:21:13.684 in Ping 
21:21:13.904 rpc 34 server.reportClientActivity
21:21:14.905 fetch POST /api/observability/v1/traces
21:21:14.905 fetched POST /api/observability/v1/traces 204
21:21:18.687 in Ping 
21:21:20.992 chunk 17 42736 bytes
21:21:20.994 in Ack 17
21:21:21.995 fetch POST /api/observability/v1/traces
21:21:21.995 fetched POST /api/observability/v1/traces 204
21:21:22.223 chunk 17 42904 bytes
21:21:22.223 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:21:22.224 in Ack 17
21:21:22.650 chunk 17 42932 bytes
21:21:22.652 in Ack 17
21:21:22.715 rpc 35 orchestration.dispatchCommand runtime-request.respond
21:21:23.225 fetch POST /api/observability/v1/traces
21:21:23.225 fetched POST /api/observability/v1/traces 204
21:21:23.369 chunk 17 43188 bytes
21:21:23.370 in Ack 17
21:21:23.692 in Ping 
21:21:23.860 chunk 17 43498 bytes
21:21:23.861 in Ack 17
21:21:24.117 chunk 17 43534 bytes
21:21:24.118 in Ack 17
21:21:24.371 fetch POST /api/observability/v1/traces
21:21:24.371 fetched POST /api/observability/v1/traces 204
21:21:24.750 chunk 17 43552 bytes
21:21:24.752 in Ack 17
21:21:25.753 fetch POST /api/observability/v1/traces
21:21:25.753 fetched POST /api/observability/v1/traces 204
21:21:26.450 chunk 17 43656 bytes
21:21:26.451 in Ack 17
21:21:27.452 fetch POST /api/observability/v1/traces
21:21:27.452 fetched POST /api/observability/v1/traces 204
