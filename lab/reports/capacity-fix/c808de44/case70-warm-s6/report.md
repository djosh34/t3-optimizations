# case70, webapp, warm start, seed 6

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 69 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.4s |
| Prompt reached the agent | 2.1s |
| Time to first text | 12.2s |
| Whole reply shown | 68.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.1s / 7.7s / 13.1s |
| Longest silence | 12.2s |
| Approval shown after it was asked | 16.0s |
| Bytes on the wire down / up | 86833 / 34238 |
| Bytes the app received / sent | 71557 / 84894 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.8 | 0 | 318 |
| 20 | 8.2 | 385 | 718 |
| 30 | 9.1 | 750 | 1077 |
| 40 | 12.6 | 1013 | 1077 |
| 50 | 3.8 | 1077 | 1229 |
| 60 | 10.1 | 1220 | 1590 |
| 68 | 8.9 | 1558 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 856/1022","messages":"456/456 548/548 197/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172912007,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":10125,"deliveredRate":3662.012114703485,"loss":0,"srttMs":5.034629575226619,"minRttMs":1,"sentPackets":126,"sentWireBytes":39170,"receivedPackets":113,"receivedWireBytes":54167,"lastHeardMs":9090,"unsyncedBytes":998,"inflightPackets":25,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 676/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 115/260","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172922844,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":2690.556149889588,"deliveredRate":2414.8780813834724,"loss":0.3007177033492823,"srttMs":2057.4330304285086,"minRttMs":1,"sentPackets":171,"sentWireBytes":50175,"receivedPackets":138,"receivedWireBytes":64946,"lastHeardMs":19926,"unsyncedBytes":0,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 938/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 708/778","user":"10/10 9/9","attempts":"661/661 661/661 446/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172933033,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":2000,"deliveredRate":1223.8095238095239,"loss":0.6666666666666667,"srttMs":2450.4326746815987,"minRttMs":3209,"sentPackets":234,"sentWireBytes":64196,"receivedPackets":168,"receivedWireBytes":75784,"lastHeardMs":30116,"unsyncedBytes":174,"inflightPackets":58,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 271/839","assistant":"32/32 21/21 1088/1188","user":"10/10 9/9","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172942921,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":3000,"deliveredRate":1036.4974141984014,"loss":0.6666666666666667,"srttMs":3816.731242686958,"minRttMs":3209,"sentPackets":292,"sentWireBytes":76282,"receivedPackets":203,"receivedWireBytes":86129,"lastHeardMs":40004,"unsyncedBytes":0,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078 219/1550","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 499/839","assistant":"32/32 21/21 1320/1340","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"888/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172952812,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":3000,"deliveredRate":443.1891465639001,"loss":0.7861916666666666,"srttMs":3816.731242686958,"minRttMs":3209,"sentPackets":342,"sentWireBytes":83423,"receivedPackets":226,"receivedWireBytes":96898,"lastHeardMs":49894,"unsyncedBytes":0,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078 1257/1550 210/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 603/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172963712,"silences":0,"longestSilenceMs":1020,"longestRecoveredGapMs":1020},"stats":{"sendRate":1000,"deliveredRate":350.6933190016713,"loss":0.7831666666666666,"srttMs":5627.41968431289,"minRttMs":3209,"sentPackets":393,"sentWireBytes":100620,"receivedPackets":252,"receivedWireBytes":108980,"lastHeardMs":60795,"unsyncedBytes":400,"inflightPackets":117,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078 1257/1550 210/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 491/524","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":210,"lastDataAt":1794172974298,"silences":0,"longestSilenceMs":1549,"longestRecoveredGapMs":1549},"stats":{"sendRate":750,"deliveredRate":443.2125960284426,"loss":0.7282166666666665,"srttMs":5966.464466377159,"minRttMs":3209,"sentPackets":429,"sentWireBytes":110297,"receivedPackets":275,"receivedWireBytes":119166,"lastHeardMs":71381,"unsyncedBytes":0,"inflightPackets":104,"duplicateBytes":0}}
- shim trace
21:21:43.227 fetch GET /api/auth/session
21:21:43.229 fetched GET /api/auth/session 200
21:21:43.355 fetch GET /.well-known/t3/environment
21:21:43.355 fetched GET /.well-known/t3/environment 200
21:21:43.370 fetch GET /.well-known/t3/environment
21:21:43.371 fetched GET /.well-known/t3/environment 200
21:21:43.375 rpc 0 subscribeServerConfig
21:21:43.375 chunk 0 38678 bytes
21:21:43.395 in Ack 0
21:21:43.397 rpc 1 subscribeServerLifecycle
21:21:43.397 rpc 2 subscribeServerLifecycle
21:21:43.400 fetch GET /api/auth/session
21:21:43.402 fetched GET /api/auth/session 200
21:21:43.402 rpc 3 subscribeDeviceState
21:21:43.404 rpc 4 preview.reportProfiles
21:21:43.434 rpc 5 subscribeProjectClones
21:21:43.451 fetch GET /api/orchestration/threads/mcp%3A62fb3e2b-f3f6-4470-a7dc-8d94f72d2b96/bounded
21:21:43.455 fetch GET /api/orchestration/shell
21:21:43.456 fetched GET /api/orchestration/shell 200
21:21:43.458 rpc 6 orchestration.subscribeShell
21:21:43.458 chunk 6 2779 bytes
21:21:43.476 rpc 7 server.refreshProviders
21:21:43.511 rpc 8 orchestration.dispatchCommand thread.visit
21:21:43.511 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:21:43.524 rpc 9 subscribeTerminalMetadata
21:21:43.525 rpc 10 subscribeVcsStatus
21:21:43.525 rpc 11 subscribePreviewEvents
21:21:43.526 rpc 12 assets.createUrl
21:21:43.527 rpc 13 preview.list
21:21:43.528 rpc 14 projects.readFile
21:21:43.530 in Ack 6
21:21:43.777 rpc 15 server.reportClientActivity
21:21:43.791 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"0464b9ad-4233-4a55-8213-e6562fff68e4","sessions":[]}}
21:21:43.791 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:21:44.030 chunk 1 3861 bytes
21:21:44.030 in Ack 1
21:21:44.300 chunk 2 3861 bytes
21:21:44.301 in Ack 2
21:21:44.354 fetch POST /api/observability/v1/traces
21:21:44.354 fetched POST /api/observability/v1/traces 204
21:21:44.461 chunk 6 2755 bytes
21:21:44.461 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:21:31.465Z","configurableProviders":false,"con
21:21:44.461 chunk 10 472 bytes
21:21:44.461 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:21:44.462 in Ack 6
21:21:44.472 in Ack 10
21:21:44.918 chunk 3 876 bytes
21:21:44.918 exit 4 {"_tag":"Success","value":null}
21:21:44.918 chunk 5 4 bytes
21:21:44.918 chunk 9 36 bytes
21:21:44.918 exit 15 {"_tag":"Success","value":null}
21:21:44.919 in Ack 3
21:21:44.922 in Ack 5
21:21:44.922 in Ack 9
21:21:44.962 chunk 6 2798 bytes
21:21:44.963 in Ack 6
21:21:45.337 fetched GET /api/orchestration/threads/mcp%3A62fb3e2b-f3f6-4470-a7dc-8d94f72d2b96/bounded 200
21:21:45.340 rpc 16 orchestration.subscribeThread
21:21:45.340 chunk 16 8294 bytes
21:21:45.349 in Interrupt 10
21:21:45.351 rpc 17 subscribeVcsStatus
21:21:45.393 in Ack 16
21:21:45.463 fetch POST /api/observability/v1/traces
21:21:45.463 fetched POST /api/observability/v1/traces 204
21:21:45.488 chunk 17 472 bytes
21:21:45.488 in Ack 17
21:21:45.591 chunk 16 11932 bytes
21:21:45.592 in Ack 16
21:21:45.622 rpc 18 server.reportClientActivity
21:21:45.837 exit 18 {"_tag":"Success","value":null}
21:21:45.843 chunk 16 13517 bytes
21:21:45.844 in Ack 16
21:21:46.227 rpc 19 attachments.createUploadUrl
21:21:46.396 chunk 16 18034 bytes
21:21:46.409 in Ack 16
21:21:46.450 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-0351580b-88f8-401f-89d2-360a4148a6f2","expiresAt":1794173506413,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:21:46.451 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTAzNTE1ODBiLTg4ZjgtNDAxZi04OWQyLTM2MGE0MTQ4YTZmMiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzUwNjQxM30.8s03hdT6NeG6Nld5t2HdZpMBTtCFueVzRIQrhHICfLE
21:21:46.595 fetch POST /api/observability/v1/traces
21:21:46.595 fetched POST /api/observability/v1/traces 204
21:21:46.646 chunk 16 19081 bytes
21:21:46.647 in Ack 16
21:21:46.900 chunk 16 21197 bytes
21:21:46.901 in Ack 16
21:21:46.908 in Interrupt 17
21:21:46.909 rpc 20 subscribeVcsStatus
21:21:47.025 chunk 20 472 bytes
21:21:47.026 in Ack 20
21:21:47.161 rpc 21 server.reportClientActivity
21:21:47.648 fetch POST /api/observability/v1/traces
21:21:47.648 fetched POST /api/observability/v1/traces 204
21:21:47.788 exit 21 {"_tag":"Success","value":null}
21:21:48.373 in Ping 
21:21:48.789 fetch POST /api/observability/v1/traces
21:21:48.789 fetched POST /api/observability/v1/traces 204
21:21:49.161 rpc 22 orchestration.dispatchCommand message.dispatch
21:21:49.778 exit 22 {"_tag":"Success","value":{"sequence":45}}
21:21:49.779 rpc 23 attachments.delete
21:21:50.049 chunk 16 23642 bytes
21:21:50.050 in Ack 16
21:21:50.160 fetch POST /api/observability/v1/traces
21:21:50.160 fetched POST /api/observability/v1/traces 204
21:21:50.300 chunk 16 23614 bytes
21:21:50.302 in Ack 16
21:21:50.358 exit 23 {"_tag":"Success","value":null}
21:21:50.552 chunk 6 2884 bytes
21:21:50.552 chunk 16 26419 bytes
21:21:50.554 in Ack 6
21:21:50.588 in Ack 16
21:21:50.590 rpc 24 assets.createUrl
21:21:50.803 chunk 16 26469 bytes
21:21:50.804 in Ack 16
21:21:51.053 chunk 16 28103 bytes
21:21:51.053 chunk 6 2820 bytes
21:21:51.054 in Ack 16
21:21:51.068 in Ack 6
21:21:51.069 rpc 25 orchestration.dispatchCommand thread.visit
21:21:51.069 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:21:51.303 chunk 16 31336 bytes
21:21:51.304 fetch POST /api/observability/v1/traces
21:21:51.304 fetched POST /api/observability/v1/traces 204
21:21:51.304 chunk 6 2820 bytes
21:21:51.305 in Ack 16
21:21:51.313 in Ack 6
21:21:51.553 chunk 16 31950 bytes
21:21:51.554 in Ack 16
21:21:51.560 in Interrupt 20
21:21:51.562 rpc 26 subscribeVcsStatus
21:21:51.603 chunk 26 472 bytes
21:21:51.604 in Ack 26
21:21:52.008 chunk 16 33297 bytes
21:21:52.008 rpc 27 server.reportClientActivity
21:21:52.010 in Ack 16
21:21:52.308 fetch POST /api/observability/v1/traces
21:21:52.308 fetched POST /api/observability/v1/traces 204
21:21:52.801 rpc 28 orchestration.dispatchCommand message.dispatch
21:21:53.373 in Ping 
21:21:53.801 fetch POST /api/observability/v1/traces
21:21:53.801 fetched POST /api/observability/v1/traces 204
21:21:54.871 chunk 16 35231 bytes
21:21:54.872 in Ack 16
21:21:54.991 fetch POST /api/observability/v1/traces
21:21:54.991 fetched POST /api/observability/v1/traces 204
21:21:55.858 chunk 16 35231 bytes
21:21:55.859 in Ack 16
21:21:56.855 fetch POST /api/observability/v1/traces
21:21:56.855 fetched POST /api/observability/v1/traces 204
21:21:56.895 exit 28 {"_tag":"Success","value":{"sequence":75}}
21:21:57.312 chunk 16 37678 bytes
21:21:57.313 in Ack 16
21:21:57.648 chunk 16 37688 bytes
21:21:57.649 in Ack 16
21:21:57.898 fetch POST /api/observability/v1/traces
21:21:57.898 fetched POST /api/observability/v1/traces 204
21:21:58.070 chunk 16 37708 bytes
21:21:58.071 in Ack 16
21:21:58.374 in Ping 
21:21:58.753 chunk 16 37738 bytes
21:21:58.755 in Ack 16
21:21:59.072 fetch POST /api/observability/v1/traces
21:21:59.072 fetched POST /api/observability/v1/traces 204
21:21:59.089 chunk 16 37748 bytes
21:21:59.091 in Ack 16
21:21:59.513 chunk 16 37770 bytes
21:21:59.514 in Ack 16
21:21:59.851 chunk 16 37792 bytes
21:21:59.853 in Ack 16
21:22:00.091 fetch POST /api/observability/v1/traces
21:22:00.091 fetched POST /api/observability/v1/traces 204
21:22:00.607 chunk 16 37804 bytes
21:22:00.609 in Ack 16
21:22:01.376 chunk 16 37824 bytes
21:22:01.377 in Ack 16
21:22:01.609 fetch POST /api/observability/v1/traces
21:22:01.610 fetched POST /api/observability/v1/traces 204
21:22:01.718 chunk 16 37844 bytes
21:22:01.719 in Ack 16
21:22:02.322 chunk 16 37886 bytes
21:22:02.323 in Ack 16
21:22:02.587 chunk 16 37896 bytes
21:22:02.588 in Ack 16
21:22:02.720 fetch POST /api/observability/v1/traces
21:22:02.720 fetched POST /api/observability/v1/traces 204
21:22:02.845 chunk 16 37906 bytes
21:22:02.846 in Ack 16
21:22:03.103 chunk 16 37916 bytes
21:22:03.104 in Ack 16
21:22:03.362 chunk 16 37946 bytes
21:22:03.363 in Ack 16
21:22:03.374 in Ping 
21:22:03.626 chunk 16 37966 bytes
21:22:03.628 in Ack 16
21:22:03.847 fetch POST /api/observability/v1/traces
21:22:03.847 fetched POST /api/observability/v1/traces 204
21:22:03.888 chunk 16 38150 bytes
21:22:03.889 in Ack 16
21:22:04.308 chunk 16 38180 bytes
21:22:04.309 in Ack 16
21:22:04.751 chunk 16 38250 bytes
21:22:04.753 in Ack 16
21:22:04.890 fetch POST /api/observability/v1/traces
21:22:04.890 fetched POST /api/observability/v1/traces 204
21:22:05.197 chunk 16 38290 bytes
21:22:05.198 in Ack 16
21:22:05.721 chunk 16 38322 bytes
21:22:05.722 in Ack 16
21:22:06.180 chunk 16 38342 bytes
21:22:06.181 in Ack 16
21:22:06.201 fetch POST /api/observability/v1/traces
21:22:06.201 fetched POST /api/observability/v1/traces 204
21:22:06.512 chunk 16 38482 bytes
21:22:06.513 in Ack 16
21:22:06.950 chunk 16 38542 bytes
21:22:06.951 in Ack 16
21:22:07.021 in Interrupt 27
21:22:07.381 chunk 16 38612 bytes
21:22:07.382 in Ack 16
21:22:07.514 fetch POST /api/observability/v1/traces
21:22:07.514 fetched POST /api/observability/v1/traces 204
21:22:07.828 chunk 16 38684 bytes
21:22:07.829 in Ack 16
21:22:08.079 chunk 16 38704 bytes
21:22:08.080 in Ack 16
21:22:08.330 chunk 16 38724 bytes
21:22:08.331 in Ack 16
21:22:08.374 in Ping 
21:22:08.580 chunk 16 38774 bytes
21:22:08.581 in Ack 16
21:22:08.617 rpc 29 server.reportClientActivity
21:22:08.830 chunk 16 38784 bytes
21:22:08.831 in Ack 16
21:22:08.841 fetch POST /api/observability/v1/traces
21:22:08.841 fetched POST /api/observability/v1/traces 204
21:22:09.080 chunk 16 38794 bytes
21:22:09.081 in Ack 16
21:22:09.476 chunk 16 38838 bytes
21:22:09.488 in Ack 16
21:22:09.726 chunk 16 38858 bytes
21:22:09.727 in Ack 16
21:22:10.082 fetch POST /api/observability/v1/traces
21:22:10.082 fetched POST /api/observability/v1/traces 204
21:22:10.218 chunk 16 38908 bytes
21:22:10.219 in Ack 16
21:22:10.470 chunk 16 38940 bytes
21:22:10.471 in Ack 16
21:22:10.721 chunk 16 38960 bytes
21:22:10.723 in Ack 16
21:22:10.980 chunk 16 38992 bytes
21:22:10.981 in Ack 16
21:22:11.220 fetch POST /api/observability/v1/traces
21:22:11.220 fetched POST /api/observability/v1/traces 204
21:22:11.235 chunk 16 39002 bytes
21:22:11.236 in Ack 16
21:22:11.491 chunk 16 39012 bytes
21:22:11.492 in Ack 16
21:22:12.086 chunk 16 39034 bytes
21:22:12.088 in Ack 16
21:22:12.237 fetch POST /api/observability/v1/traces
21:22:12.237 fetched POST /api/observability/v1/traces 204
21:22:12.515 chunk 16 39056 bytes
21:22:12.516 in Ack 16
21:22:12.773 chunk 16 39088 bytes
21:22:12.774 in Ack 16
21:22:13.034 chunk 16 39098 bytes
21:22:13.035 in Ack 16
21:22:13.375 in Ping 
21:22:13.428 chunk 16 39100 bytes
21:22:13.429 in Ack 16
21:22:13.517 fetch POST /api/observability/v1/traces
21:22:13.517 fetched POST /api/observability/v1/traces 204
21:22:13.910 chunk 16 39178 bytes
21:22:13.912 in Ack 16
21:22:14.329 chunk 16 39216 bytes
21:22:14.330 in Ack 16
21:22:14.589 chunk 16 39226 bytes
21:22:14.591 in Ack 16
21:22:14.839 chunk 16 39274 bytes
21:22:14.840 in Ack 16
21:22:14.912 fetch POST /api/observability/v1/traces
21:22:14.912 fetched POST /api/observability/v1/traces 204
21:22:15.090 chunk 16 39284 bytes
21:22:15.091 in Ack 16
21:22:15.346 chunk 16 39294 bytes
21:22:15.348 in Ack 16
21:22:15.605 chunk 16 39304 bytes
21:22:15.606 in Ack 16
21:22:15.867 chunk 16 39324 bytes
21:22:15.868 in Ack 16
21:22:16.092 fetch POST /api/observability/v1/traces
21:22:16.092 fetched POST /api/observability/v1/traces 204
21:22:16.117 chunk 16 39362 bytes
21:22:16.118 in Ack 16
21:22:16.368 chunk 16 39372 bytes
21:22:16.370 in Ack 16
21:22:16.620 chunk 16 39382 bytes
21:22:16.621 in Ack 16
21:22:16.935 chunk 16 39404 bytes
21:22:16.936 in Ack 16
21:22:17.119 fetch POST /api/observability/v1/traces
21:22:17.119 fetched POST /api/observability/v1/traces 204
21:22:17.266 chunk 16 39426 bytes
21:22:17.267 in Ack 16
21:22:17.516 chunk 16 39438 bytes
21:22:17.517 in Ack 16
21:22:17.769 chunk 16 39450 bytes
21:22:17.771 in Ack 16
21:22:18.025 chunk 16 39460 bytes
21:22:18.027 in Ack 16
21:22:18.267 fetch POST /api/observability/v1/traces
21:22:18.267 fetched POST /api/observability/v1/traces 204
21:22:18.277 chunk 16 39470 bytes
21:22:18.278 in Ack 16
21:22:18.375 in Ping 
21:22:18.532 chunk 16 39500 bytes
21:22:18.533 in Ack 16
21:22:18.942 chunk 16 39530 bytes
21:22:18.943 in Ack 16
21:22:19.277 chunk 16 40198 bytes
21:22:19.278 in Ack 16
21:22:19.285 fetch POST /api/observability/v1/traces
21:22:19.285 fetched POST /api/observability/v1/traces 204
21:22:19.535 chunk 16 40208 bytes
21:22:19.536 in Ack 16
21:22:19.787 chunk 16 40284 bytes
21:22:19.788 in Ack 16
21:22:20.048 chunk 16 40284 bytes
21:22:20.049 in Ack 16
21:22:20.299 chunk 16 40294 bytes
21:22:20.300 in Ack 16
21:22:20.537 fetch POST /api/observability/v1/traces
21:22:20.537 fetched POST /api/observability/v1/traces 204
21:22:20.548 chunk 16 40314 bytes
21:22:20.549 in Ack 16
21:22:20.798 chunk 16 40364 bytes
21:22:20.801 in Ack 16
21:22:21.069 chunk 16 40394 bytes
21:22:21.071 in Ack 16
21:22:21.391 chunk 16 40404 bytes
21:22:21.392 in Ack 16
21:22:21.550 fetch POST /api/observability/v1/traces
21:22:21.550 fetched POST /api/observability/v1/traces 204
21:22:21.905 chunk 16 40416 bytes
21:22:21.906 in Ack 16
21:22:22.160 chunk 16 40428 bytes
21:22:22.161 in Ack 16
21:22:22.418 chunk 16 40440 bytes
21:22:22.420 in Ack 16
21:22:22.673 chunk 16 40452 bytes
21:22:22.674 in Ack 16
21:22:22.907 fetch POST /api/observability/v1/traces
21:22:22.907 fetched POST /api/observability/v1/traces 204
21:22:23.178 chunk 16 40474 bytes
21:22:23.179 in Ack 16
21:22:23.375 in Ping 
21:22:23.430 chunk 16 40506 bytes
21:22:23.431 in Ack 16
21:22:23.618 in Interrupt 29
21:22:24.189 fetch POST /api/observability/v1/traces
21:22:24.189 fetched POST /api/observability/v1/traces 204
21:22:24.190 chunk 16 40518 bytes
21:22:24.191 in Ack 16
21:22:24.556 chunk 16 42541 bytes
21:22:24.560 in Ack 16
21:22:24.933 exit 29 {"_tag":"Success","value":null}
21:22:25.192 fetch POST /api/observability/v1/traces
21:22:25.192 fetched POST /api/observability/v1/traces 204
21:22:25.519 chunk 16 42731 bytes
21:22:25.521 in Ack 16
21:22:26.018 chunk 16 42741 bytes
21:22:26.019 in Ack 16
21:22:26.269 chunk 16 42817 bytes
21:22:26.270 in Ack 16
21:22:26.546 fetch POST /api/observability/v1/traces
21:22:26.546 fetched POST /api/observability/v1/traces 204
21:22:26.882 chunk 16 42837 bytes
21:22:26.884 in Ack 16
21:22:27.886 fetch POST /api/observability/v1/traces
21:22:27.886 fetched POST /api/observability/v1/traces 204
21:22:28.375 in Ping 
21:22:29.733 chunk 16 42845 bytes
21:22:29.734 in Ack 16
21:22:29.983 chunk 16 42871 bytes
21:22:29.984 in Ack 16
21:22:30.735 fetch POST /api/observability/v1/traces
21:22:30.735 fetched POST /api/observability/v1/traces 204
21:22:31.745 chunk 16 42909 bytes
21:22:31.746 in Ack 16
21:22:32.747 fetch POST /api/observability/v1/traces
21:22:32.747 fetched POST /api/observability/v1/traces 204
21:22:33.375 in Ping 
21:22:33.425 chunk 16 42947 bytes
21:22:33.426 in Ack 16
21:22:33.607 rpc 30 server.reportClientActivity
21:22:34.427 fetch POST /api/observability/v1/traces
21:22:34.427 fetched POST /api/observability/v1/traces 204
21:22:36.906 fetch POST /api/observability/v1/traces
21:22:36.906 fetched POST /api/observability/v1/traces 204
21:22:37.477 chunk 16 43955 bytes
21:22:37.478 in Ack 16
21:22:37.701 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:22:38.376 in Ping 
21:22:38.479 fetch POST /api/observability/v1/traces
21:22:38.479 fetched POST /api/observability/v1/traces 204
21:22:43.382 in Ping 
21:22:44.236 chunk 16 46466 bytes
21:22:44.236 exit 31 {"_tag":"Success","value":{"sequence":95}}
21:22:44.237 in Ack 16
21:22:44.760 chunk 16 46506 bytes
21:22:44.761 in Ack 16
21:22:45.028 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:22:45.196 chunk 16 46556 bytes
21:22:45.197 in Ack 16
21:22:45.239 fetch POST /api/observability/v1/traces
21:22:45.239 fetched POST /api/observability/v1/traces 204
21:22:45.630 chunk 16 46648 bytes
21:22:45.632 in Ack 16
21:22:46.068 chunk 16 46658 bytes
21:22:46.070 in Ack 16
21:22:46.591 chunk 16 46678 bytes
21:22:46.592 in Ack 16
21:22:46.633 fetch POST /api/observability/v1/traces
21:22:46.633 fetched POST /api/observability/v1/traces 204
21:22:47.022 chunk 16 46688 bytes
21:22:47.024 in Ack 16
21:22:47.457 chunk 16 46708 bytes
21:22:47.459 in Ack 16
21:22:47.882 chunk 16 46718 bytes
21:22:47.882 exit 30 {"_tag":"Success","value":null}
21:22:47.883 in Ack 16
21:22:48.024 fetch POST /api/observability/v1/traces
21:22:48.024 fetched POST /api/observability/v1/traces 204
21:22:48.316 chunk 16 46738 bytes
21:22:48.317 in Ack 16
21:22:48.384 in Ping 
21:22:48.742 chunk 16 46768 bytes
21:22:48.743 in Ack 16
21:22:49.258 chunk 16 46800 bytes
21:22:49.259 in Ack 16
21:22:49.318 fetch POST /api/observability/v1/traces
21:22:49.318 fetched POST /api/observability/v1/traces 204
21:22:49.695 chunk 16 46840 bytes
21:22:49.696 in Ack 16
21:22:50.133 chunk 16 46860 bytes
21:22:50.134 in Ack 16
21:22:50.571 chunk 16 46920 bytes
21:22:50.572 in Ack 16
21:22:50.697 fetch POST /api/observability/v1/traces
21:22:50.697 fetched POST /api/observability/v1/traces 204
21:22:51.008 chunk 16 47790 bytes
21:22:51.010 in Ack 16
21:22:51.407 chunk 16 47868 bytes
21:22:51.408 in Ack 16
21:22:51.657 chunk 16 47878 bytes
21:22:51.658 in Ack 16
21:22:51.908 chunk 16 47890 bytes
21:22:51.909 in Ack 16
21:22:52.010 fetch POST /api/observability/v1/traces
21:22:52.010 fetched POST /api/observability/v1/traces 204
21:22:53.388 in Ping 
21:22:53.451 chunk 16 48048 bytes
21:22:53.453 in Ack 16
21:22:53.801 chunk 16 48176 bytes
21:22:53.803 in Ack 16
21:22:54.052 chunk 16 48216 bytes
21:22:54.053 in Ack 16
21:22:54.302 chunk 16 48226 bytes
21:22:54.303 in Ack 16
21:22:54.459 fetch POST /api/observability/v1/traces
21:22:54.459 fetched POST /api/observability/v1/traces 204
21:22:55.485 chunk 16 48330 bytes
21:22:55.487 in Ack 16
21:22:56.022 chunk 16 48340 bytes
21:22:56.024 in Ack 16
21:22:56.272 chunk 16 48416 bytes
21:22:56.273 in Ack 16
21:22:56.488 fetch POST /api/observability/v1/traces
21:22:56.488 fetched POST /api/observability/v1/traces 204
21:22:56.529 chunk 16 48484 bytes
21:22:56.531 in Ack 16
21:22:56.787 chunk 16 48504 bytes
21:22:56.789 in Ack 16
21:22:57.047 chunk 16 48588 bytes
21:22:57.048 in Ack 16
21:22:57.305 chunk 16 48624 bytes
21:22:57.306 in Ack 16
21:22:57.532 fetch POST /api/observability/v1/traces
21:22:57.532 fetched POST /api/observability/v1/traces 204
21:22:58.359 chunk 16 48634 bytes
21:22:58.359 exit 32 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:22:58.360 in Ack 16
21:22:58.608 in Ping 
21:22:58.610 rpc 33 server.reportClientActivity
21:22:58.743 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:22:58.788 chunk 16 48640 bytes
21:22:58.789 in Ack 16
21:22:59.046 chunk 16 48666 bytes
21:22:59.048 in Ack 16
21:22:59.361 fetch POST /api/observability/v1/traces
21:22:59.361 fetched POST /api/observability/v1/traces 204
21:23:01.177 chunk 16 48773 bytes
21:23:01.178 in Ack 16
