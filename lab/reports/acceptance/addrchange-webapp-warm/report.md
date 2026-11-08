# addrchange, webapp, warm start, seed 1

Profile addrchange (fault): the phone's address changes once, on a weak link. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.6s |
| Prompt reached the agent | 0.7s |
| Time to first text | 1.4s |
| Whole reply shown | 43.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.9s / 0.6s / 1.5s |
| Longest silence | 1.9s |
| Approval shown after it was asked | 0.6s |
| Bytes on the wire down / up | 890201 / 777127 |
| Bytes the app received / sent | 446504 / 48487 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.6 | 344 | 373 |
| 20 | 0.6 | 746 | 774 |
| 30 | 0.9 | 1094 | 1132 |
| 40 | 0.7 | 1477 | 1505 |
| 43 | 1.2 | 1564 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 548/548 205/901 548/548","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2697/2697","nodes":"1022/1022 809/809 809/809 979/1022","checkpointScopes":"735/735","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":108,"lastDataAt":1794168231243,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":4500,"deliveredRate":2276.453024453024,"loss":0,"srttMs":98.67494010892544,"minRttMs":2,"sentPackets":90,"sentWireBytes":30147,"receivedPackets":110,"receivedWireBytes":54926,"lastHeardMs":9285,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 548/548 901/901 548/548 451/451","assistant":"32/32 21/21 465/465","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2697/2697","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","checkpointScopes":"735/735","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":108,"lastDataAt":1794168241342,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":1500,"deliveredRate":2185.080264400378,"loss":0,"srttMs":315.7288441260231,"minRttMs":2,"sentPackets":151,"sentWireBytes":40641,"receivedPackets":471,"receivedWireBytes":139330,"lastHeardMs":19384,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 548/548 901/901 548/548 451/451 547/547","assistant":"32/32 21/21 958/958","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2697/2697","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","checkpointScopes":"735/735","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":108,"lastDataAt":1794168251340,"silences":0,"longestSilenceMs":1818,"longestRecoveredGapMs":1818},"stats":{"sendRate":1125,"deliveredRate":1771.2871287128712,"loss":0,"srttMs":396.8029704896535,"minRttMs":2,"sentPackets":198,"sentWireBytes":48712,"receivedPackets":897,"receivedWireBytes":228911,"lastHeardMs":29383,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 548/548 901/901 548/548 451/451 548/548","assistant":"32/32 21/21 1340/1340 123/123","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2697/2697","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1398/1398 987/987 1022/1022","checkpointScopes":"735/735","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":108,"lastDataAt":1794168262206,"silences":0,"longestSilenceMs":1818,"longestRecoveredGapMs":1818},"stats":{"sendRate":3796.875,"deliveredRate":0,"loss":0.20905594405594408,"srttMs":476.40561360121194,"minRttMs":414,"sentPackets":258,"sentWireBytes":60903,"receivedPackets":1328,"receivedWireBytes":348786,"lastHeardMs":40249,"unsyncedBytes":178,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 548/548 901/901 548/548 451/451 548/548 547/547","assistant":"32/32 21/21 1340/1340 689/689","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2697/2697","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1398/1398 987/987 1022/1022","checkpointScopes":"735/735","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":108,"lastDataAt":1794168273426,"silences":0,"longestSilenceMs":1818,"longestRecoveredGapMs":1818},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.03208333333333332,"srttMs":494.19615473917236,"minRttMs":414,"sentPackets":329,"sentWireBytes":76044,"receivedPackets":1895,"receivedWireBytes":487923,"lastHeardMs":51468,"unsyncedBytes":143,"inflightPackets":17,"duplicateBytes":0}}
- shim trace
20:03:42.324 fetch GET /api/auth/session
20:03:42.325 fetched GET /api/auth/session 200
20:03:42.449 fetch GET /.well-known/t3/environment
20:03:42.450 fetched GET /.well-known/t3/environment 200
20:03:42.458 fetch GET /.well-known/t3/environment
20:03:42.458 fetched GET /.well-known/t3/environment 200
20:03:42.461 rpc 0 subscribeServerConfig
20:03:42.462 chunk 0 38726 bytes
20:03:42.489 in Ack 0
20:03:42.491 rpc 1 subscribeServerLifecycle
20:03:42.492 rpc 2 subscribeServerLifecycle
20:03:42.495 fetch GET /api/auth/session
20:03:42.496 fetched GET /api/auth/session 200
20:03:42.496 rpc 3 subscribeDeviceState
20:03:42.498 rpc 4 preview.reportProfiles
20:03:42.521 rpc 5 subscribeProjectClones
20:03:42.538 fetch GET /api/orchestration/threads/mcp%3Adebe1438-cb92-4128-b1e7-281786937a13/bounded
20:03:42.542 fetch GET /api/orchestration/shell
20:03:42.561 rpc 6 server.refreshProviders
20:03:42.600 rpc 7 orchestration.dispatchCommand thread.visit
20:03:42.600 exit 7 {"_tag":"Success","value":{"sequence":0}}
20:03:42.611 fetched GET /api/orchestration/shell 200
20:03:42.611 rpc 8 subscribeTerminalMetadata
20:03:42.612 rpc 9 subscribeVcsStatus
20:03:42.613 rpc 10 subscribePreviewEvents
20:03:42.615 rpc 11 orchestration.subscribeShell
20:03:42.615 chunk 11 2791 bytes
20:03:42.616 rpc 12 assets.createUrl
20:03:42.617 rpc 13 preview.list
20:03:42.618 rpc 14 projects.readFile
20:03:42.620 in Ack 11
20:03:42.741 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"037926c2-9189-4021-abbf-758592485266","sessions":[]}}
20:03:42.741 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
20:03:42.866 rpc 15 server.reportClientActivity
20:03:42.866 chunk 11 2767 bytes
20:03:42.867 in Ack 11
20:03:42.872 chunk 1 3873 bytes
20:03:42.874 in Ack 1
20:03:43.109 chunk 2 3873 bytes
20:03:43.109 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T20:03:28.900Z","configurableProviders":false,"con
20:03:43.109 chunk 9 551 bytes
20:03:43.109 in Ack 2
20:03:43.112 in Ack 9
20:03:43.254 exit 12 {"_tag":"Success","value":{"expiresAt":1794171600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
20:03:43.438 chunk 3 876 bytes
20:03:43.438 exit 4 {"_tag":"Success","value":null}
20:03:43.438 chunk 5 4 bytes
20:03:43.438 chunk 8 36 bytes
20:03:43.438 exit 15 {"_tag":"Success","value":null}
20:03:43.439 in Ack 3
20:03:43.443 rpc 16 preview.reportProfiles
20:03:43.443 in Ack 5
20:03:43.443 in Ack 8
20:03:43.448 fetch POST /api/observability/v1/traces
20:03:43.448 fetched POST /api/observability/v1/traces 204
20:03:43.505 chunk 11 2810 bytes
20:03:43.506 in Ack 11
20:03:43.519 exit 16 {"_tag":"Success","value":null}
20:03:43.639 fetched GET /api/orchestration/threads/mcp%3Adebe1438-cb92-4128-b1e7-281786937a13/bounded 200
20:03:43.642 rpc 17 orchestration.subscribeThread
20:03:43.643 chunk 17 8294 bytes
20:03:43.651 in Interrupt 9
20:03:43.653 rpc 18 subscribeVcsStatus
20:03:43.654 in Ack 17
20:03:43.758 chunk 18 551 bytes
20:03:43.758 in Ack 18
20:03:43.893 chunk 17 11932 bytes
20:03:43.895 in Ack 17
20:03:43.926 rpc 19 server.reportClientActivity
20:03:44.146 chunk 17 13983 bytes
20:03:44.147 in Ack 17
20:03:44.258 exit 19 {"_tag":"Success","value":null}
20:03:44.396 chunk 17 19093 bytes
20:03:44.397 in Ack 17
20:03:44.507 fetch POST /api/observability/v1/traces
20:03:44.507 fetched POST /api/observability/v1/traces 204
20:03:44.676 rpc 20 attachments.createUploadUrl
20:03:44.846 chunk 17 21221 bytes
20:03:44.867 in Ack 17
20:03:44.880 in Interrupt 18
20:03:44.883 rpc 21 subscribeVcsStatus
20:03:44.903 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-91db010b-c780-4105-91e6-0f7c87486f07","expiresAt":1794168824899,"relativeUrl":"/api/attachments/upload/eyJ2ZX
20:03:44.906 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTkxZGIwMTBiLWM3ODAtNDEwNS05MWU2LTBmN2M4NzQ4NmYwNyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODgyNDg5OX0.h4HKicEK3ZRCRgHP7C1B97-luHo1jyks08IL3P9yZMM
20:03:44.928 chunk 21 551 bytes
20:03:44.929 in Ack 21
20:03:45.135 rpc 22 server.reportClientActivity
20:03:45.676 fetch POST /api/observability/v1/traces
20:03:45.676 fetched POST /api/observability/v1/traces 204
20:03:45.967 exit 22 {"_tag":"Success","value":null}
20:03:46.968 fetch POST /api/observability/v1/traces
20:03:46.968 fetched POST /api/observability/v1/traces 204
20:03:47.460 in Ping 
20:03:47.611 rpc 23 orchestration.dispatchCommand message.dispatch
20:03:48.290 exit 23 {"_tag":"Success","value":{"sequence":45}}
20:03:48.292 rpc 24 attachments.delete
20:03:48.610 fetch POST /api/observability/v1/traces
20:03:48.610 fetched POST /api/observability/v1/traces 204
20:03:48.791 chunk 17 23676 bytes
20:03:48.792 in Ack 17
20:03:48.988 exit 24 {"_tag":"Success","value":null}
20:03:49.054 chunk 17 23650 bytes
20:03:49.055 in Ack 17
20:03:49.305 chunk 11 2832 bytes
20:03:49.305 chunk 17 23700 bytes
20:03:49.305 in Ack 11
20:03:49.307 rpc 25 orchestration.dispatchCommand thread.visit
20:03:49.307 exit 25 {"_tag":"Success","value":{"sequence":0}}
20:03:49.368 in Ack 17
20:03:49.556 chunk 17 25334 bytes
20:03:49.557 in Ack 17
20:03:49.793 fetch POST /api/observability/v1/traces
20:03:49.793 fetched POST /api/observability/v1/traces 204
20:03:49.806 chunk 11 2832 bytes
20:03:49.807 chunk 17 28127 bytes
20:03:49.807 in Ack 11
20:03:49.813 in Ack 17
20:03:49.815 rpc 26 assets.createUrl
20:03:50.057 chunk 17 32140 bytes
20:03:50.059 in Ack 17
20:03:50.065 in Interrupt 21
20:03:50.067 rpc 27 subscribeVcsStatus
20:03:50.170 chunk 27 551 bytes
20:03:50.171 in Ack 27
20:03:50.467 chunk 17 31974 bytes
20:03:50.468 rpc 28 server.reportClientActivity
20:03:50.469 in Ack 17
20:03:50.721 chunk 17 31974 bytes
20:03:50.723 in Ack 17
20:03:50.810 fetch POST /api/observability/v1/traces
20:03:50.810 fetched POST /api/observability/v1/traces 204
20:03:50.976 chunk 17 33321 bytes
20:03:50.977 in Ack 17
20:03:51.207 rpc 29 orchestration.dispatchCommand message.dispatch
20:03:51.406 exit 28 {"_tag":"Success","value":null}
20:03:51.495 chunk 17 35255 bytes
20:03:51.498 in Ack 17
20:03:51.632 exit 29 {"_tag":"Success","value":{"sequence":75}}
20:03:51.978 fetch POST /api/observability/v1/traces
20:03:51.978 fetched POST /api/observability/v1/traces 204
20:03:52.245 chunk 17 37700 bytes
20:03:52.246 in Ack 17
20:03:52.461 in Ping 
20:03:52.495 chunk 17 37730 bytes
20:03:52.496 in Ack 17
20:03:52.745 chunk 17 37750 bytes
20:03:52.746 in Ack 17
20:03:52.995 chunk 17 37770 bytes
20:03:52.996 in Ack 17
20:03:53.245 chunk 17 37800 bytes
20:03:53.246 in Ack 17
20:03:53.256 fetch POST /api/observability/v1/traces
20:03:53.256 fetched POST /api/observability/v1/traces 204
20:03:53.495 chunk 17 37782 bytes
20:03:53.496 in Ack 17
20:03:53.745 chunk 17 37802 bytes
20:03:53.747 in Ack 17
20:03:53.996 chunk 17 37822 bytes
20:03:53.997 in Ack 17
20:03:54.249 chunk 11 2896 bytes
20:03:54.249 chunk 17 38490 bytes
20:03:54.250 in Ack 11
20:03:54.287 in Ack 17
20:03:54.497 fetch POST /api/observability/v1/traces
20:03:54.497 fetched POST /api/observability/v1/traces 204
20:03:54.500 chunk 17 41365 bytes
20:03:54.501 in Ack 17
20:03:54.750 chunk 17 42054 bytes
20:03:54.751 in Ack 17
20:03:55.000 chunk 17 43366 bytes
20:03:55.001 in Ack 17
20:03:55.251 chunk 17 43406 bytes
20:03:55.252 in Ack 17
20:03:55.500 chunk 17 43426 bytes
20:03:55.501 in Ack 17
20:03:55.512 fetch POST /api/observability/v1/traces
20:03:55.512 fetched POST /api/observability/v1/traces 204
20:03:55.750 chunk 17 43446 bytes
20:03:55.751 in Ack 17
20:03:56.000 chunk 17 43476 bytes
20:03:56.002 in Ack 17
20:03:56.250 chunk 17 43496 bytes
20:03:56.251 in Ack 17
20:03:56.341 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/1136d228-8a8a-41cf-9604-69ca3439b688","expiresAt":1794171836341}}
20:03:56.500 chunk 17 43526 bytes
20:03:56.501 in Ack 17
20:03:56.750 chunk 17 43546 bytes
20:03:56.751 in Ack 17
20:03:56.760 fetch POST /api/observability/v1/traces
20:03:56.760 fetched POST /api/observability/v1/traces 204
20:03:57.000 chunk 17 43576 bytes
20:03:57.001 in Ack 17
20:03:57.250 chunk 17 43596 bytes
20:03:57.251 in Ack 17
20:03:57.461 in Ping 
20:03:57.500 chunk 17 43626 bytes
20:03:57.502 in Ack 17
20:03:57.750 chunk 17 43646 bytes
20:03:57.751 in Ack 17
20:03:58.001 chunk 17 43676 bytes
20:03:58.002 in Ack 17
20:03:58.026 fetch POST /api/observability/v1/traces
20:03:58.026 fetched POST /api/observability/v1/traces 204
20:03:58.251 chunk 17 43696 bytes
20:03:58.252 in Ack 17
20:03:58.501 chunk 17 43726 bytes
20:03:58.502 in Ack 17
20:03:58.751 chunk 17 43746 bytes
20:03:58.752 in Ack 17
20:03:59.001 chunk 17 43776 bytes
20:03:59.002 in Ack 17
20:03:59.251 chunk 17 43796 bytes
20:03:59.252 in Ack 17
20:03:59.284 fetch POST /api/observability/v1/traces
20:03:59.284 fetched POST /api/observability/v1/traces 204
20:03:59.307 rpc 30 orchestration.dispatchCommand thread.visit
20:03:59.307 exit 30 {"_tag":"Success","value":{"sequence":0}}
20:03:59.501 chunk 17 43826 bytes
20:03:59.503 in Ack 17
20:03:59.752 chunk 17 43846 bytes
20:03:59.754 in Ack 17
20:04:00.003 chunk 11 2896 bytes
20:04:00.004 chunk 17 43876 bytes
20:04:00.005 in Ack 11
20:04:00.010 in Ack 17
20:04:00.254 chunk 17 43906 bytes
20:04:00.255 in Ack 17
20:04:00.308 fetch POST /api/observability/v1/traces
20:04:00.308 fetched POST /api/observability/v1/traces 204
20:04:00.505 chunk 17 43926 bytes
20:04:00.506 in Ack 17
20:04:00.755 chunk 17 43946 bytes
20:04:00.756 in Ack 17
20:04:01.005 chunk 17 43976 bytes
20:04:01.006 in Ack 17
20:04:01.255 chunk 17 43996 bytes
20:04:01.256 in Ack 17
20:04:01.505 chunk 17 44026 bytes
20:04:01.506 in Ack 17
20:04:01.516 fetch POST /api/observability/v1/traces
20:04:01.516 fetched POST /api/observability/v1/traces 204
20:04:01.756 chunk 17 44046 bytes
20:04:01.757 in Ack 17
20:04:02.006 chunk 17 44076 bytes
20:04:02.008 in Ack 17
20:04:02.255 chunk 11 2896 bytes
20:04:02.256 in Ack 11
20:04:02.257 chunk 17 45149 bytes
20:04:02.258 in Ack 17
20:04:02.461 in Ping 
20:04:02.508 chunk 17 45194 bytes
20:04:02.509 in Ack 17
20:04:02.758 fetch POST /api/observability/v1/traces
20:04:02.758 fetched POST /api/observability/v1/traces 204
20:04:02.758 chunk 17 45224 bytes
20:04:02.761 in Ack 17
20:04:03.009 chunk 17 45269 bytes
20:04:03.010 in Ack 17
20:04:03.260 chunk 17 45299 bytes
20:04:03.262 in Ack 17
20:04:03.510 chunk 17 45344 bytes
20:04:03.512 in Ack 17
20:04:03.760 chunk 17 45374 bytes
20:04:03.761 in Ack 17
20:04:03.770 fetch POST /api/observability/v1/traces
20:04:03.771 fetched POST /api/observability/v1/traces 204
20:04:04.010 chunk 17 45419 bytes
20:04:04.011 in Ack 17
20:04:04.260 chunk 17 45449 bytes
20:04:04.261 in Ack 17
20:04:04.510 chunk 17 45479 bytes
20:04:04.511 in Ack 17
20:04:04.760 chunk 17 45524 bytes
20:04:04.761 in Ack 17
20:04:05.010 chunk 17 45569 bytes
20:04:05.011 in Ack 17
20:04:05.040 fetch POST /api/observability/v1/traces
20:04:05.040 fetched POST /api/observability/v1/traces 204
20:04:05.260 chunk 17 45599 bytes
20:04:05.261 in Ack 17
20:04:05.510 chunk 17 45629 bytes
20:04:05.511 in Ack 17
20:04:06.262 fetch POST /api/observability/v1/traces
20:04:06.262 fetched POST /api/observability/v1/traces 204
20:04:07.317 chunk 17 45899 bytes
20:04:07.318 in Ack 17
20:04:07.462 in Ping 
20:04:07.567 chunk 17 45929 bytes
20:04:07.568 in Ack 17
20:04:07.704 rpc 31 server.reportClientActivity
20:04:07.817 chunk 17 45959 bytes
20:04:07.819 in Ack 17
20:04:08.067 chunk 17 46019 bytes
20:04:08.068 in Ack 17
20:04:08.164 exit 31 {"_tag":"Success","value":null}
20:04:08.317 chunk 17 46064 bytes
20:04:08.318 in Ack 17
20:04:08.327 fetch POST /api/observability/v1/traces
20:04:08.327 fetched POST /api/observability/v1/traces 204
20:04:08.567 chunk 17 46094 bytes
20:04:08.569 in Ack 17
20:04:08.818 chunk 17 46139 bytes
20:04:08.820 in Ack 17
20:04:09.069 chunk 17 46169 bytes
20:04:09.071 in Ack 17
20:04:09.310 rpc 32 orchestration.dispatchCommand thread.visit
20:04:09.310 exit 32 {"_tag":"Success","value":{"sequence":0}}
20:04:09.318 chunk 17 46199 bytes
20:04:09.319 in Ack 17
20:04:09.568 chunk 17 46244 bytes
20:04:09.569 in Ack 17
20:04:09.579 fetch POST /api/observability/v1/traces
20:04:09.579 fetched POST /api/observability/v1/traces 204
20:04:09.818 chunk 17 46274 bytes
20:04:09.820 in Ack 17
20:04:10.069 chunk 11 2896 bytes
20:04:10.070 chunk 17 46304 bytes
20:04:10.070 in Ack 11
20:04:10.080 in Ack 17
20:04:10.320 chunk 17 46349 bytes
20:04:10.321 in Ack 17
20:04:10.570 chunk 17 46394 bytes
20:04:10.571 in Ack 17
20:04:10.820 chunk 17 46424 bytes
20:04:10.821 in Ack 17
20:04:10.830 fetch POST /api/observability/v1/traces
20:04:10.830 fetched POST /api/observability/v1/traces 204
20:04:11.070 chunk 17 46469 bytes
20:04:11.071 in Ack 17
20:04:11.320 chunk 17 46499 bytes
20:04:11.321 in Ack 17
20:04:11.570 chunk 17 46544 bytes
20:04:11.571 in Ack 17
20:04:11.820 chunk 17 46589 bytes
20:04:11.822 in Ack 17
20:04:12.070 chunk 17 46619 bytes
20:04:12.071 in Ack 17
20:04:12.081 fetch POST /api/observability/v1/traces
20:04:12.081 fetched POST /api/observability/v1/traces 204
20:04:12.565 chunk 17 46694 bytes
20:04:12.565 in Ping 
20:04:12.567 in Ack 17
20:04:12.816 chunk 17 46724 bytes
20:04:12.817 in Ack 17
20:04:13.066 chunk 17 46769 bytes
20:04:13.067 in Ack 17
20:04:13.317 chunk 17 46799 bytes
20:04:13.318 in Ack 17
20:04:13.568 chunk 17 46844 bytes
20:04:13.570 fetch POST /api/observability/v1/traces
20:04:13.570 fetched POST /api/observability/v1/traces 204
20:04:13.571 in Ack 17
20:04:13.818 chunk 17 46874 bytes
20:04:13.819 in Ack 17
20:04:14.068 chunk 17 46919 bytes
20:04:14.069 in Ack 17
20:04:14.319 chunk 17 46949 bytes
20:04:14.321 in Ack 17
20:04:14.569 chunk 17 46994 bytes
20:04:14.570 in Ack 17
20:04:14.601 fetch POST /api/observability/v1/traces
20:04:14.601 fetched POST /api/observability/v1/traces 204
20:04:14.821 chunk 17 47024 bytes
20:04:14.822 in Ack 17
20:04:15.070 chunk 17 47069 bytes
20:04:15.071 in Ack 17
20:04:15.320 chunk 17 47099 bytes
20:04:15.321 in Ack 17
20:04:15.571 chunk 17 47144 bytes
20:04:15.572 in Ack 17
20:04:15.823 chunk 17 47174 bytes
20:04:15.823 fetch POST /api/observability/v1/traces
20:04:15.823 fetched POST /api/observability/v1/traces 204
20:04:15.824 in Ack 17
20:04:16.073 chunk 17 47219 bytes
20:04:16.074 in Ack 17
20:04:16.324 chunk 17 47249 bytes
20:04:16.325 in Ack 17
20:04:16.574 chunk 17 47294 bytes
20:04:16.575 in Ack 17
20:04:16.825 chunk 17 47324 bytes
20:04:16.825 fetch POST /api/observability/v1/traces
20:04:16.825 fetched POST /api/observability/v1/traces 204
20:04:16.826 in Ack 17
20:04:17.074 chunk 17 47369 bytes
20:04:17.075 in Ack 17
20:04:17.334 chunk 17 47399 bytes
20:04:17.336 in Ack 17
20:04:17.567 in Ping 
20:04:17.585 chunk 17 47444 bytes
20:04:17.586 in Ack 17
20:04:17.827 fetch POST /api/observability/v1/traces
20:04:17.827 fetched POST /api/observability/v1/traces 204
20:04:17.835 chunk 17 47489 bytes
20:04:17.837 in Ack 17
20:04:18.086 chunk 17 47534 bytes
20:04:18.088 in Ack 17
20:04:18.336 chunk 17 47564 bytes
20:04:18.337 in Ack 17
20:04:18.586 chunk 17 47609 bytes
20:04:18.587 in Ack 17
20:04:18.835 chunk 17 47624 bytes
20:04:18.837 in Ack 17
20:04:18.860 fetch POST /api/observability/v1/traces
20:04:18.860 fetched POST /api/observability/v1/traces 204
20:04:19.086 chunk 17 48674 bytes
20:04:19.093 in Ack 17
20:04:19.556 rpc 33 orchestration.dispatchCommand runtime-request.respond
20:04:19.594 chunk 17 54262 bytes
20:04:19.626 in Ack 17
20:04:19.844 chunk 11 3205 bytes
20:04:19.845 chunk 17 54312 bytes
20:04:19.845 in Ack 11
20:04:19.854 in Ack 17
20:04:20.094 fetch POST /api/observability/v1/traces
20:04:20.094 fetched POST /api/observability/v1/traces 204
20:04:20.095 chunk 17 55277 bytes
20:04:20.096 in Ack 17
20:04:20.261 exit 33 {"_tag":"Success","value":{"sequence":97}}
20:04:20.262 rpc 34 orchestration.dispatchCommand thread.visit
20:04:20.262 exit 34 {"_tag":"Success","value":{"sequence":0}}
20:04:20.346 chunk 17 57769 bytes
20:04:20.347 in Ack 17
20:04:20.597 chunk 17 57799 bytes
20:04:20.598 in Ack 17
20:04:20.835 rpc 35 orchestration.dispatchCommand runtime-request.respond
20:04:20.861 chunk 17 57771 bytes
20:04:20.862 in Ack 17
20:04:21.098 fetch POST /api/observability/v1/traces
20:04:21.098 fetched POST /api/observability/v1/traces 204
20:04:21.111 chunk 11 2898 bytes
20:04:21.112 chunk 17 57801 bytes
20:04:21.112 in Ack 11
20:04:21.120 in Ack 17
20:04:21.363 chunk 17 57864 bytes
20:04:21.364 in Ack 17
20:04:21.518 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
20:04:21.761 chunk 17 60360 bytes
20:04:21.763 in Ack 17
20:04:22.013 chunk 17 60476 bytes
20:04:22.014 in Ack 17
20:04:22.024 in Interrupt 27
20:04:22.051 rpc 36 subscribeVcsStatus
20:04:22.113 fetch POST /api/observability/v1/traces
20:04:22.113 fetched POST /api/observability/v1/traces 204
20:04:22.263 chunk 17 60496 bytes
20:04:22.265 in Ack 17
20:04:22.312 rpc 37 server.reportClientActivity
20:04:22.513 chunk 17 60526 bytes
20:04:22.514 in Ack 17
20:04:22.589 in Ping 
20:04:22.667 chunk 36 551 bytes
20:04:22.668 in Ack 36
20:04:22.763 chunk 17 60556 bytes
20:04:22.765 in Ack 17
20:04:23.013 chunk 17 60586 bytes
20:04:23.014 in Ack 17
20:04:23.202 exit 37 {"_tag":"Success","value":null}
20:04:23.263 chunk 17 60606 bytes
20:04:23.264 in Ack 17
20:04:23.273 fetch POST /api/observability/v1/traces
20:04:23.273 fetched POST /api/observability/v1/traces 204
20:04:23.513 chunk 17 60636 bytes
20:04:23.514 in Ack 17
20:04:23.765 chunk 17 60656 bytes
20:04:23.766 in Ack 17
20:04:24.020 chunk 17 60686 bytes
20:04:24.023 in Ack 17
20:04:24.271 chunk 17 60716 bytes
20:04:24.272 in Ack 17
20:04:24.515 fetch POST /api/observability/v1/traces
20:04:24.515 fetched POST /api/observability/v1/traces 204
20:04:24.521 chunk 17 60736 bytes
20:04:24.523 in Ack 17
20:04:24.771 chunk 17 60766 bytes
20:04:24.772 in Ack 17
20:04:25.028 chunk 17 60786 bytes
20:04:25.029 in Ack 17
20:04:25.279 chunk 17 60816 bytes
20:04:25.281 in Ack 17
20:04:25.529 fetch POST /api/observability/v1/traces
20:04:25.529 fetched POST /api/observability/v1/traces 204
20:04:25.529 chunk 17 60836 bytes
20:04:25.530 in Ack 17
20:04:25.780 chunk 17 60866 bytes
20:04:25.781 in Ack 17
20:04:26.030 chunk 17 60886 bytes
20:04:26.032 in Ack 17
20:04:26.281 chunk 17 60906 bytes
20:04:26.282 in Ack 17
20:04:26.531 fetch POST /api/observability/v1/traces
20:04:26.531 fetched POST /api/observability/v1/traces 204
20:04:26.532 chunk 17 60946 bytes
20:04:26.533 in Ack 17
20:04:26.783 chunk 17 60966 bytes
20:04:26.784 in Ack 17
20:04:27.058 chunk 17 61006 bytes
20:04:27.059 in Ack 17
20:04:27.307 chunk 17 61026 bytes
20:04:27.308 in Ack 17
20:04:27.534 fetch POST /api/observability/v1/traces
20:04:27.534 fetched POST /api/observability/v1/traces 204
20:04:27.596 chunk 17 61056 bytes
20:04:27.596 in Ping 
20:04:27.598 in Ack 17
20:04:27.846 chunk 17 61072 bytes
20:04:27.847 in Ack 17
20:04:28.097 chunk 11 2898 bytes
20:04:28.097 chunk 17 61084 bytes
20:04:28.097 in Ack 11
20:04:28.099 in Ack 17
20:04:28.347 chunk 17 61114 bytes
20:04:28.348 in Ack 17
20:04:28.631 chunk 17 62124 bytes
20:04:28.633 fetch POST /api/observability/v1/traces
20:04:28.633 fetched POST /api/observability/v1/traces 204
20:04:28.636 in Ack 17
20:04:28.882 chunk 17 62184 bytes
20:04:28.884 in Ack 17
20:04:29.132 chunk 17 62214 bytes
20:04:29.133 in Ack 17
20:04:29.382 chunk 17 62265 bytes
20:04:29.383 in Ack 17
20:04:29.632 chunk 17 62310 bytes
20:04:29.641 in Ack 17
20:04:29.691 fetch POST /api/observability/v1/traces
20:04:29.691 fetched POST /api/observability/v1/traces 204
20:04:29.848 rpc 38 orchestration.dispatchCommand thread.visit
20:04:29.848 exit 38 {"_tag":"Success","value":{"sequence":0}}
20:04:29.882 chunk 17 62346 bytes
20:04:29.883 in Ack 17
20:04:30.133 chunk 17 62391 bytes
20:04:30.134 in Ack 17
20:04:30.412 chunk 17 62442 bytes
20:04:30.414 in Ack 17
20:04:30.662 chunk 17 62472 bytes
20:04:30.664 in Ack 17
20:04:30.848 fetch POST /api/observability/v1/traces
20:04:30.848 fetched POST /api/observability/v1/traces 204
20:04:30.912 chunk 11 2898 bytes
20:04:30.913 chunk 17 62487 bytes
20:04:30.913 in Ack 11
20:04:30.920 in Ack 17
20:04:31.163 chunk 17 62517 bytes
20:04:31.164 in Ack 17
20:04:31.413 chunk 17 62562 bytes
20:04:31.414 in Ack 17
20:04:31.663 chunk 17 62601 bytes
20:04:31.665 in Ack 17
20:04:31.913 chunk 17 62661 bytes
20:04:31.914 in Ack 17
20:04:31.973 fetch POST /api/observability/v1/traces
20:04:31.973 fetched POST /api/observability/v1/traces 204
20:04:32.163 chunk 17 62718 bytes
20:04:32.165 in Ack 17
20:04:32.413 chunk 17 62748 bytes
20:04:32.415 in Ack 17
20:04:32.596 in Ping 
20:04:32.663 chunk 17 62802 bytes
20:04:32.665 in Ack 17
20:04:32.731 rpc 39 server.reportClientActivity
20:04:32.913 chunk 17 62847 bytes
20:04:32.914 in Ack 17
20:04:33.163 chunk 17 62883 bytes
20:04:33.165 in Ack 17
20:04:33.284 fetch POST /api/observability/v1/traces
20:04:33.284 fetched POST /api/observability/v1/traces 204
20:04:33.429 chunk 17 62937 bytes
20:04:33.431 in Ack 17
20:04:33.679 chunk 17 62967 bytes
20:04:33.681 in Ack 17
20:04:33.844 exit 39 {"_tag":"Success","value":null}
20:04:33.929 chunk 17 63015 bytes
20:04:33.931 in Ack 17
20:04:34.179 chunk 17 63098 bytes
20:04:34.181 in Ack 17
20:04:34.435 fetch POST /api/observability/v1/traces
20:04:34.435 fetched POST /api/observability/v1/traces 204
20:04:34.795 chunk 17 63122 bytes
20:04:34.798 in Ack 17
