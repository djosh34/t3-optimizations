# case50, webapp, warm start, seed 6

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.2s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.8s |
| Whole reply shown | 43.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.1s / 1.1s / 1.9s |
| Longest silence | 3.0s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 345525 / 138592 |
| Bytes the app received / sent | 278394 / 69145 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 315 | 373 |
| 20 | 1.3 | 722 | 774 |
| 30 | 0.8 | 1077 | 1109 |
| 40 | 1.8 | 1424 | 1483 |
| 43 | 0.9 | 1558 | 1587 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":239,"lastDataAt":1794174873462,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":7522.006831983805,"deliveredRate":3690.4081146582403,"loss":0,"srttMs":47.08235296784795,"minRttMs":1,"sentPackets":97,"sentWireBytes":30754,"receivedPackets":125,"receivedWireBytes":64401,"lastHeardMs":8983,"unsyncedBytes":509,"inflightPackets":8,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 425/435","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","messages":"456/456 548/548 901/901 548/548 43/451","providerTurns":"776/776 776/776 658/658","providerThreads":"616/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 590/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":239,"lastDataAt":1794174883413,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":2000,"deliveredRate":915.2542372881356,"loss":0.6,"srttMs":460.20357618818304,"minRttMs":616,"sentPackets":158,"sentWireBytes":43867,"receivedPackets":178,"receivedWireBytes":90074,"lastHeardMs":18933,"unsyncedBytes":0,"inflightPackets":18,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849","thread":"843/951","frame":"1031/1031","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 953/953","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","messages":"456/456 548/548 901/901 548/548 451/451 547/547","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":239,"lastDataAt":1794174893743,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":3709.608208955224,"deliveredRate":1041.2935323383085,"loss":0.5789473684210527,"srttMs":604.6162753886915,"minRttMs":594,"sentPackets":221,"sentWireBytes":56629,"receivedPackets":308,"receivedWireBytes":133601,"lastHeardMs":29264,"unsyncedBytes":0,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1103/1103 849/849 929/929 978/1550","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 33/53","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","messages":"456/456 548/548 901/901 548/548 451/451 547/547","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"463/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":239,"lastDataAt":1794174903768,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.480578947368421,"srttMs":647.721835844467,"minRttMs":594,"sentPackets":277,"sentWireBytes":67633,"receivedPackets":622,"receivedWireBytes":218154,"lastHeardMs":39289,"unsyncedBytes":400,"inflightPackets":42,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1103/1103 849/849 953/953 1574/1574 1078/1078","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 609/609","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","messages":"456/456 548/548 901/901 548/548 451/451 548/548 547/547","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1022/1022 987/987 1398/1398","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":239,"lastDataAt":1794174915137,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.49628412087912077,"srttMs":878.1613689913561,"minRttMs":594,"sentPackets":372,"sentWireBytes":91424,"receivedPackets":887,"receivedWireBytes":318722,"lastHeardMs":50658,"unsyncedBytes":0,"inflightPackets":76,"duplicateBytes":0}}
- shim trace
21:54:24.781 fetch GET /api/auth/session
21:54:24.783 fetched GET /api/auth/session 200
21:54:24.906 fetch GET /.well-known/t3/environment
21:54:24.907 fetched GET /.well-known/t3/environment 200
21:54:24.917 fetch GET /.well-known/t3/environment
21:54:24.927 fetched GET /.well-known/t3/environment 200
21:54:24.931 rpc 0 subscribeServerConfig
21:54:24.931 chunk 0 38678 bytes
21:54:24.952 in Ack 0
21:54:24.954 rpc 1 subscribeServerLifecycle
21:54:24.955 rpc 2 subscribeServerLifecycle
21:54:24.957 fetch GET /api/auth/session
21:54:24.959 fetched GET /api/auth/session 200
21:54:24.960 rpc 3 subscribeDeviceState
21:54:24.962 rpc 4 preview.reportProfiles
21:54:24.993 rpc 5 subscribeProjectClones
21:54:24.998 fetch GET /api/orchestration/threads/mcp%3A4a61db5e-e3fa-4694-afe5-39035576eab2/bounded
21:54:25.015 fetch GET /api/orchestration/shell
21:54:25.033 rpc 6 server.refreshProviders
21:54:25.072 rpc 7 orchestration.dispatchCommand thread.visit
21:54:25.072 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:54:25.084 fetched GET /api/orchestration/shell 200
21:54:25.084 rpc 8 subscribeTerminalMetadata
21:54:25.085 rpc 9 subscribeVcsStatus
21:54:25.086 rpc 10 subscribePreviewEvents
21:54:25.088 rpc 11 orchestration.subscribeShell
21:54:25.088 chunk 11 2779 bytes
21:54:25.089 rpc 12 assets.createUrl
21:54:25.090 rpc 13 preview.list
21:54:25.091 rpc 14 projects.readFile
21:54:25.094 in Ack 11
21:54:25.181 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"912d7373-78d6-41dd-a119-d12834239c99","sessions":[]}}
21:54:25.181 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:54:25.319 chunk 1 3861 bytes
21:54:25.319 in Ack 1
21:54:25.339 rpc 15 server.reportClientActivity
21:54:25.339 chunk 11 2755 bytes
21:54:25.339 in Ack 11
21:54:25.699 chunk 2 3861 bytes
21:54:25.699 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:54:12.773Z","configurableProviders":false,"con
21:54:25.699 chunk 9 472 bytes
21:54:25.700 in Ack 2
21:54:25.703 in Ack 9
21:54:25.829 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:54:25.905 fetch POST /api/observability/v1/traces
21:54:25.905 fetched POST /api/observability/v1/traces 204
21:54:26.048 chunk 3 876 bytes
21:54:26.048 exit 4 {"_tag":"Success","value":null}
21:54:26.048 chunk 5 4 bytes
21:54:26.048 chunk 8 36 bytes
21:54:26.048 exit 15 {"_tag":"Success","value":null}
21:54:26.049 in Ack 3
21:54:26.053 in Ack 5
21:54:26.053 in Ack 8
21:54:26.079 chunk 11 2798 bytes
21:54:26.080 in Ack 11
21:54:26.128 fetched GET /api/orchestration/threads/mcp%3A4a61db5e-e3fa-4694-afe5-39035576eab2/bounded 200
21:54:26.131 rpc 16 orchestration.subscribeThread
21:54:26.132 chunk 16 8294 bytes
21:54:26.141 in Interrupt 9
21:54:26.143 rpc 17 subscribeVcsStatus
21:54:26.188 in Ack 16
21:54:26.248 chunk 17 472 bytes
21:54:26.249 in Ack 17
21:54:26.382 chunk 16 11932 bytes
21:54:26.383 in Ack 16
21:54:26.435 rpc 18 server.reportClientActivity
21:54:26.634 chunk 16 13517 bytes
21:54:26.635 in Ack 16
21:54:26.853 exit 18 {"_tag":"Success","value":null}
21:54:26.901 rpc 19 attachments.createUploadUrl
21:54:27.057 chunk 16 19081 bytes
21:54:27.063 fetch POST /api/observability/v1/traces
21:54:27.063 fetched POST /api/observability/v1/traces 204
21:54:27.065 in Ack 16
21:54:27.138 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-6db5267a-fc19-48bf-9824-bdc0c43fadb7","expiresAt":1794175467108,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:54:27.139 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTZkYjUyNjdhLWZjMTktNDhiZi05ODI0LWJkYzBjNDNmYWRiNyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTQ2NzEwOH0.h7VJkjxqPbsw0oHkKnIXnyQSqo-_bZxwivXdBnG1zII
21:54:27.307 chunk 16 21197 bytes
21:54:27.308 in Ack 16
21:54:27.315 in Interrupt 17
21:54:27.317 rpc 20 subscribeVcsStatus
21:54:27.348 chunk 20 472 bytes
21:54:27.348 in Ack 20
21:54:27.569 rpc 21 server.reportClientActivity
21:54:28.068 fetch POST /api/observability/v1/traces
21:54:28.068 fetched POST /api/observability/v1/traces 204
21:54:28.817 exit 21 {"_tag":"Success","value":null}
21:54:29.818 fetch POST /api/observability/v1/traces
21:54:29.818 fetched POST /api/observability/v1/traces 204
21:54:29.929 in Ping 
21:54:30.116 rpc 22 orchestration.dispatchCommand message.dispatch
21:54:30.739 exit 22 {"_tag":"Success","value":{"sequence":45}}
21:54:30.741 rpc 23 attachments.delete
21:54:30.808 exit 23 {"_tag":"Success","value":null}
21:54:30.989 chunk 11 2884 bytes
21:54:30.989 chunk 16 25468 bytes
21:54:30.990 in Ack 11
21:54:31.016 in Ack 16
21:54:31.018 rpc 24 assets.createUrl
21:54:31.156 fetch POST /api/observability/v1/traces
21:54:31.156 fetched POST /api/observability/v1/traces 204
21:54:31.241 chunk 16 29320 bytes
21:54:31.242 in Ack 16
21:54:31.492 chunk 16 29394 bytes
21:54:31.493 in Ack 16
21:54:31.741 chunk 11 2820 bytes
21:54:31.742 in Ack 11
21:54:31.743 rpc 25 orchestration.dispatchCommand thread.visit
21:54:31.743 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:54:31.992 chunk 11 2820 bytes
21:54:31.992 chunk 16 30461 bytes
21:54:31.993 in Ack 11
21:54:32.000 in Ack 16
21:54:32.244 chunk 16 32908 bytes
21:54:32.244 fetch POST /api/observability/v1/traces
21:54:32.244 fetched POST /api/observability/v1/traces 204
21:54:32.246 in Ack 16
21:54:32.494 chunk 16 33520 bytes
21:54:32.495 in Ack 16
21:54:32.501 in Interrupt 20
21:54:32.503 rpc 26 subscribeVcsStatus
21:54:32.556 chunk 26 472 bytes
21:54:32.556 in Ack 26
21:54:32.985 chunk 16 35231 bytes
21:54:32.986 rpc 27 server.reportClientActivity
21:54:32.987 in Ack 16
21:54:33.251 fetch POST /api/observability/v1/traces
21:54:33.251 fetched POST /api/observability/v1/traces 204
21:54:33.393 exit 27 {"_tag":"Success","value":null}
21:54:33.649 rpc 28 orchestration.dispatchCommand message.dispatch
21:54:34.394 fetch POST /api/observability/v1/traces
21:54:34.394 fetched POST /api/observability/v1/traces 204
21:54:34.599 exit 28 {"_tag":"Success","value":{"sequence":75}}
21:54:34.849 chunk 16 37676 bytes
21:54:34.851 in Ack 16
21:54:34.930 in Ping 
21:54:35.100 chunk 16 37688 bytes
21:54:35.101 in Ack 16
21:54:35.350 chunk 16 37736 bytes
21:54:35.351 in Ack 16
21:54:35.600 fetch POST /api/observability/v1/traces
21:54:35.600 fetched POST /api/observability/v1/traces 204
21:54:35.601 chunk 16 37756 bytes
21:54:35.602 in Ack 16
21:54:35.852 chunk 16 37786 bytes
21:54:35.853 in Ack 16
21:54:36.166 chunk 16 37798 bytes
21:54:36.167 in Ack 16
21:54:36.424 chunk 16 37810 bytes
21:54:36.425 in Ack 16
21:54:36.603 fetch POST /api/observability/v1/traces
21:54:36.603 fetched POST /api/observability/v1/traces 204
21:54:36.673 chunk 16 37822 bytes
21:54:36.675 in Ack 16
21:54:37.149 chunk 16 37842 bytes
21:54:37.150 in Ack 16
21:54:37.399 chunk 16 37946 bytes
21:54:37.400 in Ack 16
21:54:37.647 fetch POST /api/observability/v1/traces
21:54:37.647 fetched POST /api/observability/v1/traces 204
21:54:37.649 chunk 16 37908 bytes
21:54:37.650 in Ack 16
21:54:38.218 chunk 16 39923 bytes
21:54:38.220 in Ack 16
21:54:38.489 chunk 16 39935 bytes
21:54:38.491 in Ack 16
21:54:38.651 fetch POST /api/observability/v1/traces
21:54:38.651 fetched POST /api/observability/v1/traces 204
21:54:38.740 chunk 11 2884 bytes
21:54:38.741 chunk 16 39975 bytes
21:54:38.742 in Ack 11
21:54:38.766 in Ack 16
21:54:39.245 chunk 16 40025 bytes
21:54:39.246 in Ack 16
21:54:39.495 chunk 16 40055 bytes
21:54:39.496 in Ack 16
21:54:39.743 fetch POST /api/observability/v1/traces
21:54:39.743 fetched POST /api/observability/v1/traces 204
21:54:39.746 chunk 16 40065 bytes
21:54:39.747 in Ack 16
21:54:39.930 in Ping 
21:54:39.996 chunk 16 40945 bytes
21:54:39.997 in Ack 16
21:54:40.247 chunk 16 41649 bytes
21:54:40.248 in Ack 16
21:54:40.497 chunk 16 42328 bytes
21:54:40.499 in Ack 16
21:54:40.748 fetch POST /api/observability/v1/traces
21:54:40.748 fetched POST /api/observability/v1/traces 204
21:54:40.748 chunk 16 42340 bytes
21:54:40.749 in Ack 16
21:54:40.998 chunk 16 42370 bytes
21:54:40.999 in Ack 16
21:54:41.249 chunk 16 42382 bytes
21:54:41.250 in Ack 16
21:54:41.499 chunk 16 42392 bytes
21:54:41.500 in Ack 16
21:54:41.746 rpc 29 orchestration.dispatchCommand thread.visit
21:54:41.746 exit 29 {"_tag":"Success","value":{"sequence":0}}
21:54:41.752 fetch POST /api/observability/v1/traces
21:54:41.752 fetched POST /api/observability/v1/traces 204
21:54:41.807 chunk 16 42442 bytes
21:54:41.808 in Ack 16
21:54:42.120 chunk 16 42462 bytes
21:54:42.121 in Ack 16
21:54:42.440 chunk 16 42474 bytes
21:54:42.441 in Ack 16
21:54:42.713 chunk 16 42514 bytes
21:54:42.715 in Ack 16
21:54:42.809 fetch POST /api/observability/v1/traces
21:54:42.809 fetched POST /api/observability/v1/traces 204
21:54:42.963 chunk 16 42542 bytes
21:54:42.964 in Ack 16
21:54:43.213 chunk 16 42552 bytes
21:54:43.214 in Ack 16
21:54:43.464 chunk 16 42610 bytes
21:54:43.465 in Ack 16
21:54:43.850 chunk 16 42650 bytes
21:54:43.851 in Ack 16
21:54:43.965 fetch POST /api/observability/v1/traces
21:54:43.965 fetched POST /api/observability/v1/traces 204
21:54:44.100 chunk 16 42670 bytes
21:54:44.101 in Ack 16
21:54:44.352 chunk 16 42690 bytes
21:54:44.353 in Ack 16
21:54:44.603 chunk 16 42730 bytes
21:54:44.604 in Ack 16
21:54:44.853 chunk 16 42762 bytes
21:54:44.854 in Ack 16
21:54:44.930 in Ping 
21:54:45.102 fetch POST /api/observability/v1/traces
21:54:45.102 fetched POST /api/observability/v1/traces 204
21:54:45.104 chunk 16 42772 bytes
21:54:45.106 in Ack 16
21:54:45.355 chunk 16 42812 bytes
21:54:45.364 in Ack 16
21:54:45.873 chunk 16 42870 bytes
21:54:45.875 in Ack 16
21:54:46.109 fetch POST /api/observability/v1/traces
21:54:46.109 fetched POST /api/observability/v1/traces 204
21:54:46.197 chunk 16 42882 bytes
21:54:46.199 in Ack 16
21:54:46.448 chunk 16 42902 bytes
21:54:46.450 in Ack 16
21:54:46.700 chunk 16 42932 bytes
21:54:46.704 in Ack 16
21:54:46.951 chunk 16 42972 bytes
21:54:46.952 in Ack 16
21:54:47.203 fetch POST /api/observability/v1/traces
21:54:47.203 fetched POST /api/observability/v1/traces 204
21:54:47.204 chunk 16 42982 bytes
21:54:47.206 in Ack 16
21:54:47.454 chunk 16 43012 bytes
21:54:47.455 in Ack 16
21:54:47.705 chunk 11 2884 bytes
21:54:47.706 chunk 16 43032 bytes
21:54:47.706 in Ack 11
21:54:47.715 in Ack 16
21:54:47.956 chunk 16 43864 bytes
21:54:47.957 in Ack 16
21:54:48.206 chunk 16 43874 bytes
21:54:48.207 in Ack 16
21:54:48.216 fetch POST /api/observability/v1/traces
21:54:48.216 fetched POST /api/observability/v1/traces 204
21:54:48.456 chunk 16 43886 bytes
21:54:48.458 in Ack 16
21:54:48.708 chunk 16 43908 bytes
21:54:48.710 in Ack 16
21:54:48.957 chunk 16 43930 bytes
21:54:48.958 in Ack 16
21:54:49.207 chunk 16 43952 bytes
21:54:49.208 in Ack 16
21:54:49.457 chunk 16 45282 bytes
21:54:49.458 in Ack 16
21:54:49.489 fetch POST /api/observability/v1/traces
21:54:49.489 fetched POST /api/observability/v1/traces 204
21:54:49.707 chunk 16 45327 bytes
21:54:49.708 in Ack 16
21:54:49.930 in Ping 
21:54:49.958 chunk 16 45372 bytes
21:54:49.959 in Ack 16
21:54:50.159 rpc 30 server.reportClientActivity
21:54:50.209 chunk 16 45432 bytes
21:54:50.210 in Ack 16
21:54:50.459 chunk 16 45449 bytes
21:54:50.460 in Ack 16
21:54:50.709 fetch POST /api/observability/v1/traces
21:54:50.709 fetched POST /api/observability/v1/traces 204
21:54:50.709 chunk 16 46010 bytes
21:54:50.710 in Ack 16
21:54:50.960 chunk 16 46025 bytes
21:54:50.961 in Ack 16
21:54:51.210 chunk 16 46070 bytes
21:54:51.211 in Ack 16
21:54:51.460 chunk 16 46115 bytes
21:54:51.461 in Ack 16
21:54:51.710 chunk 16 46160 bytes
21:54:51.711 in Ack 16
21:54:51.721 fetch POST /api/observability/v1/traces
21:54:51.721 fetched POST /api/observability/v1/traces 204
21:54:51.747 rpc 31 orchestration.dispatchCommand thread.visit
21:54:51.747 exit 31 {"_tag":"Success","value":{"sequence":0}}
21:54:51.957 exit 30 {"_tag":"Success","value":null}
21:54:51.959 chunk 16 46205 bytes
21:54:51.960 in Ack 16
21:54:52.210 chunk 16 46235 bytes
21:54:52.211 in Ack 16
21:54:52.460 chunk 16 46265 bytes
21:54:52.461 in Ack 16
21:54:52.710 chunk 16 46310 bytes
21:54:52.711 in Ack 16
21:54:52.747 fetch POST /api/observability/v1/traces
21:54:52.748 fetched POST /api/observability/v1/traces 204
21:54:52.961 chunk 16 46328 bytes
21:54:52.962 in Ack 16
21:54:53.211 chunk 16 46346 bytes
21:54:53.212 in Ack 16
21:54:53.461 chunk 16 46415 bytes
21:54:53.462 in Ack 16
21:54:53.710 chunk 16 46460 bytes
21:54:53.712 in Ack 16
21:54:53.960 chunk 16 46490 bytes
21:54:53.961 in Ack 16
21:54:53.969 fetch POST /api/observability/v1/traces
21:54:53.969 fetched POST /api/observability/v1/traces 204
21:54:54.211 chunk 16 46520 bytes
21:54:54.212 in Ack 16
21:54:54.315 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/6f1bd0cc-ef84-4aed-af1d-a03c2cd05993","expiresAt":1794178494315}}
21:54:54.459 chunk 11 2884 bytes
21:54:54.459 in Ack 11
21:54:54.464 chunk 16 46565 bytes
21:54:54.464 in Ack 16
21:54:54.713 chunk 16 46610 bytes
21:54:54.714 in Ack 16
21:54:55.057 in Ping 
21:54:55.058 chunk 16 46655 bytes
21:54:55.059 in Ack 16
21:54:55.213 fetch POST /api/observability/v1/traces
21:54:55.213 fetched POST /api/observability/v1/traces 204
21:54:55.308 chunk 16 46700 bytes
21:54:55.309 in Ack 16
21:54:55.559 chunk 16 46730 bytes
21:54:55.560 in Ack 16
21:54:55.810 chunk 16 46775 bytes
21:54:55.811 in Ack 16
21:54:56.060 chunk 16 46805 bytes
21:54:56.061 in Ack 16
21:54:56.310 chunk 16 46850 bytes
21:54:56.311 fetch POST /api/observability/v1/traces
21:54:56.311 fetched POST /api/observability/v1/traces 204
21:54:56.312 in Ack 16
21:54:56.561 chunk 16 46880 bytes
21:54:56.562 in Ack 16
21:54:56.811 chunk 16 46925 bytes
21:54:56.812 in Ack 16
21:54:57.062 chunk 16 46955 bytes
21:54:57.063 in Ack 16
21:54:57.311 chunk 16 46985 bytes
21:54:57.312 in Ack 16
21:54:57.320 fetch POST /api/observability/v1/traces
21:54:57.320 fetched POST /api/observability/v1/traces 204
21:54:57.563 chunk 16 47030 bytes
21:54:57.564 in Ack 16
21:54:57.813 chunk 16 47060 bytes
21:54:57.814 in Ack 16
21:54:58.064 chunk 16 47090 bytes
21:54:58.065 in Ack 16
21:54:58.315 chunk 16 47135 bytes
21:54:58.316 in Ack 16
21:54:58.564 fetch POST /api/observability/v1/traces
21:54:58.565 fetched POST /api/observability/v1/traces 204
21:54:58.566 chunk 16 47180 bytes
21:54:58.567 in Ack 16
21:54:58.816 chunk 16 47210 bytes
21:54:58.817 in Ack 16
21:54:59.067 chunk 16 47255 bytes
21:54:59.068 in Ack 16
21:54:59.318 chunk 16 47270 bytes
21:54:59.319 in Ack 16
21:54:59.568 fetch POST /api/observability/v1/traces
21:54:59.568 fetched POST /api/observability/v1/traces 204
21:54:59.568 chunk 16 47315 bytes
21:54:59.569 in Ack 16
21:54:59.819 chunk 16 47345 bytes
21:54:59.820 in Ack 16
21:55:00.057 in Ping 
21:55:00.070 chunk 16 47375 bytes
21:55:00.071 in Ack 16
21:55:00.322 chunk 16 47420 bytes
21:55:00.323 in Ack 16
21:55:00.570 fetch POST /api/observability/v1/traces
21:55:00.570 fetched POST /api/observability/v1/traces 204
21:55:00.572 chunk 16 47450 bytes
21:55:00.573 in Ack 16
21:55:00.823 chunk 16 47480 bytes
21:55:00.824 in Ack 16
21:55:01.073 chunk 16 47510 bytes
21:55:01.075 in Ack 16
21:55:01.323 chunk 16 47540 bytes
21:55:01.324 in Ack 16
21:55:01.574 fetch POST /api/observability/v1/traces
21:55:01.574 fetched POST /api/observability/v1/traces 204
21:55:01.574 chunk 16 47585 bytes
21:55:01.575 in Ack 16
21:55:01.824 chunk 16 47642 bytes
21:55:01.825 in Ack 16
21:55:02.075 chunk 16 48650 bytes
21:55:02.077 in Ack 16
21:55:02.319 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:55:02.577 fetch POST /api/observability/v1/traces
21:55:02.577 fetched POST /api/observability/v1/traces 204
21:55:02.871 chunk 16 50782 bytes
21:55:02.872 in Ack 16
21:55:03.252 exit 32 {"_tag":"Success","value":{"sequence":97}}
21:55:03.372 chunk 16 53264 bytes
21:55:03.373 in Ack 16
21:55:03.645 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:55:03.672 chunk 16 53276 bytes
21:55:03.674 in Ack 16
21:55:03.873 fetch POST /api/observability/v1/traces
21:55:03.873 fetched POST /api/observability/v1/traces 204
21:55:03.922 chunk 16 53296 bytes
21:55:03.924 in Ack 16
21:55:04.172 chunk 11 2886 bytes
21:55:04.172 chunk 16 53306 bytes
21:55:04.172 in Ack 11
21:55:04.175 in Ack 16
21:55:04.421 chunk 16 53316 bytes
21:55:04.423 in Ack 16
21:55:04.672 chunk 16 53384 bytes
21:55:04.673 in Ack 16
21:55:04.680 in Interrupt 26
21:55:04.703 rpc 34 subscribeVcsStatus
21:55:04.922 chunk 16 53434 bytes
21:55:04.923 in Ack 16
21:55:04.931 fetch POST /api/observability/v1/traces
21:55:04.931 fetched POST /api/observability/v1/traces 204
21:55:04.951 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:55:04.954 rpc 35 orchestration.dispatchCommand thread.visit
21:55:04.954 exit 35 {"_tag":"Success","value":{"sequence":0}}
21:55:04.955 rpc 36 server.reportClientActivity
21:55:05.246 chunk 16 53487 bytes
21:55:05.249 in Ping 
21:55:05.252 in Ack 16
21:55:05.496 chunk 16 53527 bytes
21:55:05.497 in Ack 16
21:55:05.620 chunk 34 472 bytes
21:55:05.621 in Ack 34
21:55:05.746 chunk 16 53537 bytes
21:55:05.747 in Ack 16
21:55:05.953 fetch POST /api/observability/v1/traces
21:55:05.953 fetched POST /api/observability/v1/traces 204
21:55:05.997 chunk 16 53577 bytes
21:55:05.998 in Ack 16
21:55:06.247 chunk 16 54585 bytes
21:55:06.248 in Ack 16
21:55:06.497 chunk 16 54607 bytes
21:55:06.498 in Ack 16
21:55:06.747 chunk 16 55659 bytes
21:55:06.748 in Ack 16
21:55:06.998 chunk 16 55689 bytes
21:55:06.999 in Ack 16
21:55:07.007 fetch POST /api/observability/v1/traces
21:55:07.007 fetched POST /api/observability/v1/traces 204
21:55:07.250 chunk 16 55709 bytes
21:55:07.251 in Ack 16
21:55:07.500 chunk 16 57128 bytes
21:55:07.501 in Ack 16
21:55:07.751 chunk 11 2886 bytes
21:55:07.752 chunk 16 57148 bytes
21:55:07.752 in Ack 11
21:55:07.758 in Ack 16
21:55:08.003 chunk 16 60722 bytes
21:55:08.004 in Ack 16
21:55:08.252 fetch POST /api/observability/v1/traces
21:55:08.252 fetched POST /api/observability/v1/traces 204
21:55:08.254 chunk 16 60742 bytes
21:55:08.255 in Ack 16
21:55:08.504 chunk 16 60762 bytes
21:55:08.505 in Ack 16
21:55:08.626 exit 36 {"_tag":"Success","value":null}
21:55:08.754 chunk 16 60792 bytes
21:55:08.755 in Ack 16
21:55:09.004 chunk 16 60842 bytes
21:55:09.005 in Ack 16
21:55:09.254 chunk 16 60862 bytes
21:55:09.255 in Ack 16
21:55:09.285 fetch POST /api/observability/v1/traces
21:55:09.285 fetched POST /api/observability/v1/traces 204
21:55:09.504 chunk 16 60882 bytes
21:55:09.505 in Ack 16
21:55:09.755 chunk 16 60922 bytes
21:55:09.756 in Ack 16
21:55:10.004 chunk 16 60942 bytes
21:55:10.005 in Ack 16
21:55:10.267 in Ping 
21:55:10.268 chunk 16 60972 bytes
21:55:10.269 in Ack 16
21:55:10.506 fetch POST /api/observability/v1/traces
21:55:10.506 fetched POST /api/observability/v1/traces 204
21:55:10.519 chunk 16 61002 bytes
21:55:10.520 in Ack 16
21:55:10.770 chunk 16 61012 bytes
21:55:10.771 in Ack 16
21:55:11.023 chunk 16 61044 bytes
21:55:11.024 in Ack 16
21:55:11.272 chunk 11 2886 bytes
21:55:11.273 chunk 16 61070 bytes
21:55:11.273 in Ack 11
21:55:11.275 in Ack 16
21:55:11.521 fetch POST /api/observability/v1/traces
21:55:11.521 fetched POST /api/observability/v1/traces 204
21:55:11.523 chunk 16 62070 bytes
21:55:11.525 in Ack 16
21:55:11.774 chunk 16 62100 bytes
21:55:11.777 in Ack 16
21:55:12.025 chunk 16 62130 bytes
21:55:12.027 in Ack 16
21:55:12.286 chunk 16 62190 bytes
21:55:12.288 in Ack 16
21:55:12.526 fetch POST /api/observability/v1/traces
21:55:12.526 fetched POST /api/observability/v1/traces 204
21:55:12.537 chunk 16 62241 bytes
21:55:12.538 in Ack 16
21:55:12.802 chunk 16 62271 bytes
21:55:12.804 in Ack 16
21:55:13.053 chunk 16 62322 bytes
21:55:13.054 in Ack 16
21:55:13.314 chunk 16 62352 bytes
21:55:13.315 in Ack 16
21:55:13.539 fetch POST /api/observability/v1/traces
21:55:13.539 fetched POST /api/observability/v1/traces 204
21:55:13.564 chunk 16 62397 bytes
21:55:13.566 in Ack 16
21:55:13.815 chunk 16 62433 bytes
21:55:13.816 in Ack 16
21:55:14.065 chunk 16 62478 bytes
21:55:14.067 in Ack 16
21:55:14.175 rpc 37 orchestration.dispatchCommand thread.visit
21:55:14.175 exit 37 {"_tag":"Success","value":{"sequence":0}}
21:55:14.327 chunk 16 62523 bytes
21:55:14.328 in Ack 16
21:55:14.567 fetch POST /api/observability/v1/traces
21:55:14.567 fetched POST /api/observability/v1/traces 204
21:55:14.577 chunk 16 62538 bytes
21:55:14.578 in Ack 16
21:55:14.836 chunk 16 62592 bytes
21:55:14.838 in Ack 16
21:55:15.085 chunk 16 62622 bytes
21:55:15.087 in Ack 16
21:55:15.188 rpc 38 server.reportClientActivity
21:55:15.268 in Ping 
21:55:15.342 chunk 11 2886 bytes
21:55:15.342 chunk 16 62652 bytes
21:55:15.343 in Ack 11
21:55:15.351 in Ack 16
21:55:15.579 fetch POST /api/observability/v1/traces
21:55:15.580 fetched POST /api/observability/v1/traces 204
21:55:15.593 chunk 16 62709 bytes
21:55:15.595 in Ack 16
21:55:15.854 chunk 16 62748 bytes
21:55:15.856 in Ack 16
21:55:16.106 chunk 16 62823 bytes
21:55:16.107 in Ack 16
21:55:16.356 chunk 16 62859 bytes
21:55:16.358 in Ack 16
21:55:16.596 fetch POST /api/observability/v1/traces
21:55:16.596 fetched POST /api/observability/v1/traces 204
21:55:16.607 chunk 16 62895 bytes
21:55:16.608 in Ack 16
21:55:16.870 chunk 16 62928 bytes
21:55:16.872 in Ack 16
21:55:16.891 exit 38 {"_tag":"Success","value":null}
21:55:17.121 chunk 16 62961 bytes
21:55:17.122 in Ack 16
21:55:17.371 chunk 16 63074 bytes
21:55:17.372 in Ack 16
21:55:17.686 fetch POST /api/observability/v1/traces
21:55:17.686 fetched POST /api/observability/v1/traces 204
