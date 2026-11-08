# case50, webapp, warm start, seed 3

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.8s |
| Prompt reached the agent | 1.4s |
| Time to first text | 2.2s |
| Whole reply shown | 45.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.4s / 0.9s / 1.9s |
| Longest silence | 2.2s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 454705 / 163534 |
| Bytes the app received / sent | 371531 / 91897 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 283 | 341 |
| 20 | 0.8 | 710 | 742 |
| 30 | 0.1 | 1077 | 1086 |
| 40 | 1.4 | 1418 | 1466 |
| 45 | 1.4 | 1587 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 16/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"548/548 456/456 48/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174663179,"silences":0,"longestSilenceMs":1125,"longestRecoveredGapMs":1125},"stats":{"sendRate":3375,"deliveredRate":3601.857363443895,"loss":0,"srttMs":55.106682350961755,"minRttMs":1,"sentPackets":120,"sentWireBytes":35929,"receivedPackets":120,"receivedWireBytes":53970,"lastHeardMs":9747,"unsyncedBytes":998,"inflightPackets":16,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 395/405","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"734/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 901/1022 587/785","messages":"548/548 456/456 835/901 548/548 232/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174673220,"silences":0,"longestSilenceMs":1125,"longestRecoveredGapMs":1125},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.39677918552036195,"srttMs":511.5426231225174,"minRttMs":646,"sentPackets":177,"sentWireBytes":48841,"receivedPackets":216,"receivedWireBytes":101455,"lastHeardMs":19787,"unsyncedBytes":0,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 898/898","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174683174,"silences":0,"longestSilenceMs":1125,"longestRecoveredGapMs":1125},"stats":{"sendRate":2250,"deliveredRate":1097.3684210526317,"loss":0.625,"srttMs":826.194671881981,"minRttMs":633,"sentPackets":244,"sentWireBytes":63034,"receivedPackets":464,"receivedWireBytes":183738,"lastHeardMs":29741,"unsyncedBytes":0,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 929/929 1550/1550 709/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 53/53","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 316/1374 963/963","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"938/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174694129,"silences":0,"longestSilenceMs":1125,"longestRecoveredGapMs":1125},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.5642091063348416,"srttMs":836.2953378967334,"minRttMs":633,"sentPackets":303,"sentWireBytes":74443,"receivedPackets":815,"receivedWireBytes":275136,"lastHeardMs":40696,"unsyncedBytes":400,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 953/953 1574/1574 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 604/604","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1398/1398 987/987 1022/1022","messages":"548/548 456/456 901/901 548/548 451/451 548/548 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174705253,"silences":0,"longestSilenceMs":1125,"longestRecoveredGapMs":1125},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.5325206812180954,"srttMs":1034.773720358426,"minRttMs":633,"sentPackets":413,"sentWireBytes":104781,"receivedPackets":1006,"receivedWireBytes":389001,"lastHeardMs":51820,"unsyncedBytes":308,"inflightPackets":106,"duplicateBytes":0}}
- shim trace
21:50:53.738 fetch GET /api/auth/session
21:50:53.740 fetched GET /api/auth/session 200
21:50:53.873 fetch GET /.well-known/t3/environment
21:50:53.873 fetched GET /.well-known/t3/environment 200
21:50:53.887 fetch GET /.well-known/t3/environment
21:50:53.888 fetched GET /.well-known/t3/environment 200
21:50:53.892 rpc 0 subscribeServerConfig
21:50:53.892 chunk 0 38678 bytes
21:50:53.908 in Ack 0
21:50:53.910 rpc 1 subscribeServerLifecycle
21:50:53.910 rpc 2 subscribeServerLifecycle
21:50:53.913 fetch GET /api/auth/session
21:50:53.914 fetched GET /api/auth/session 200
21:50:53.914 rpc 3 subscribeDeviceState
21:50:53.916 rpc 4 preview.reportProfiles
21:50:53.951 rpc 5 subscribeProjectClones
21:50:53.967 fetch GET /api/orchestration/threads/mcp%3Af815f4c6-9470-4ecd-bc3a-2da9477304b0/bounded
21:50:53.972 fetch GET /api/orchestration/shell
21:50:53.988 rpc 6 server.refreshProviders
21:50:54.025 rpc 7 orchestration.dispatchCommand thread.visit
21:50:54.025 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:50:54.035 fetched GET /api/orchestration/shell 200
21:50:54.035 rpc 8 subscribeTerminalMetadata
21:50:54.036 rpc 9 subscribeVcsStatus
21:50:54.037 rpc 10 subscribePreviewEvents
21:50:54.039 rpc 11 orchestration.subscribeShell
21:50:54.039 chunk 11 2779 bytes
21:50:54.040 rpc 12 assets.createUrl
21:50:54.041 rpc 13 preview.list
21:50:54.042 rpc 14 projects.readFile
21:50:54.045 in Ack 11
21:50:54.288 rpc 15 server.reportClientActivity
21:50:54.301 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"b354b2ff-9400-42bb-b2bc-649749427a48","sessions":[]}}
21:50:54.301 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:50:54.791 chunk 1 3861 bytes
21:50:54.793 in Ack 1
21:50:54.872 fetch POST /api/observability/v1/traces
21:50:54.872 fetched POST /api/observability/v1/traces 204
21:50:55.019 chunk 2 3861 bytes
21:50:55.019 in Ack 2
21:50:55.209 chunk 11 2755 bytes
21:50:55.209 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:50:42.193Z","configurableProviders":false,"con
21:50:55.209 chunk 9 472 bytes
21:50:55.209 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:50:55.210 in Ack 11
21:50:55.216 in Ack 9
21:50:55.691 chunk 3 876 bytes
21:50:55.691 exit 4 {"_tag":"Success","value":null}
21:50:55.691 chunk 5 4 bytes
21:50:55.691 chunk 8 36 bytes
21:50:55.691 exit 15 {"_tag":"Success","value":null}
21:50:55.691 in Ack 3
21:50:55.695 rpc 16 preview.reportProfiles
21:50:55.695 in Ack 5
21:50:55.695 in Ack 8
21:50:55.710 chunk 11 2798 bytes
21:50:55.710 in Ack 11
21:50:55.778 exit 16 {"_tag":"Success","value":null}
21:50:56.021 fetch POST /api/observability/v1/traces
21:50:56.021 fetched POST /api/observability/v1/traces 204
21:50:56.049 fetched GET /api/orchestration/threads/mcp%3Af815f4c6-9470-4ecd-bc3a-2da9477304b0/bounded 200
21:50:56.052 rpc 17 orchestration.subscribeThread
21:50:56.052 chunk 17 8294 bytes
21:50:56.061 in Interrupt 9
21:50:56.063 rpc 18 subscribeVcsStatus
21:50:56.064 in Ack 17
21:50:56.197 chunk 18 472 bytes
21:50:56.198 in Ack 18
21:50:56.303 chunk 17 11271 bytes
21:50:56.303 in Ack 17
21:50:56.333 rpc 19 server.reportClientActivity
21:50:56.555 chunk 17 12708 bytes
21:50:56.556 in Ack 17
21:50:56.567 exit 19 {"_tag":"Success","value":null}
21:50:56.740 rpc 20 attachments.createUploadUrl
21:50:56.901 chunk 17 13517 bytes
21:50:56.911 in Ack 17
21:50:56.947 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-48de1097-834b-4ce0-a47d-3223753a1430","expiresAt":1794175256918,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:50:56.949 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTQ4ZGUxMDk3LTgzNGItNGNlMC1hNDdkLTMyMjM3NTNhMTQzMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTI1NjkxOH0.wNU_HNnGUEL5iCRyhdHiOk4GufvIaL7NiN6H45Yg7lU
21:50:57.049 fetch POST /api/observability/v1/traces
21:50:57.049 fetched POST /api/observability/v1/traces 204
21:50:57.152 chunk 17 17224 bytes
21:50:57.153 in Ack 17
21:50:57.446 chunk 17 20474 bytes
21:50:57.450 in Ack 17
21:50:57.457 in Interrupt 18
21:50:57.459 rpc 21 subscribeVcsStatus
21:50:57.537 chunk 21 472 bytes
21:50:57.538 in Ack 21
21:50:57.699 chunk 17 21197 bytes
21:50:57.699 in Ack 17
21:50:57.710 rpc 22 server.reportClientActivity
21:50:58.154 fetch POST /api/observability/v1/traces
21:50:58.154 fetched POST /api/observability/v1/traces 204
21:50:58.847 exit 22 {"_tag":"Success","value":null}
21:50:58.890 in Ping 
21:50:59.607 rpc 23 orchestration.dispatchCommand message.dispatch
21:51:00.143 fetch POST /api/observability/v1/traces
21:51:00.144 fetched POST /api/observability/v1/traces 204
21:51:00.177 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:51:00.179 rpc 24 attachments.delete
21:51:00.456 chunk 17 23642 bytes
21:51:00.457 in Ack 17
21:51:00.706 chunk 17 23662 bytes
21:51:00.707 in Ack 17
21:51:00.957 chunk 17 23626 bytes
21:51:00.958 in Ack 17
21:51:01.179 fetch POST /api/observability/v1/traces
21:51:01.179 fetched POST /api/observability/v1/traces 204
21:51:01.207 chunk 17 23676 bytes
21:51:01.208 in Ack 17
21:51:01.278 exit 24 {"_tag":"Success","value":null}
21:51:01.457 chunk 11 2820 bytes
21:51:01.457 chunk 17 28103 bytes
21:51:01.458 in Ack 11
21:51:01.459 rpc 25 orchestration.dispatchCommand thread.visit
21:51:01.459 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:51:01.497 in Ack 17
21:51:01.499 rpc 26 assets.createUrl
21:51:01.957 chunk 11 2820 bytes
21:51:01.958 chunk 17 30526 bytes
21:51:01.958 in Ack 11
21:51:01.963 in Ack 17
21:51:02.252 chunk 17 32116 bytes
21:51:02.253 fetch POST /api/observability/v1/traces
21:51:02.253 fetched POST /api/observability/v1/traces 204
21:51:02.452 in Ack 17
21:51:02.462 in Interrupt 21
21:51:02.464 rpc 27 subscribeVcsStatus
21:51:02.503 chunk 17 31950 bytes
21:51:02.504 in Ack 17
21:51:02.715 rpc 28 server.reportClientActivity
21:51:03.114 chunk 27 472 bytes
21:51:03.119 in Ack 27
21:51:03.277 rpc 29 orchestration.dispatchCommand message.dispatch
21:51:03.312 chunk 17 31950 bytes
21:51:03.314 in Ack 17
21:51:03.453 fetch POST /api/observability/v1/traces
21:51:03.453 fetched POST /api/observability/v1/traces 204
21:51:03.814 chunk 17 32520 bytes
21:51:03.815 in Ack 17
21:51:03.890 in Ping 
21:51:03.906 exit 28 {"_tag":"Success","value":null}
21:51:04.816 fetch POST /api/observability/v1/traces
21:51:04.816 fetched POST /api/observability/v1/traces 204
21:51:04.904 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:51:05.315 chunk 17 34975 bytes
21:51:05.316 in Ack 17
21:51:05.565 chunk 17 34985 bytes
21:51:05.566 in Ack 17
21:51:05.815 chunk 17 35015 bytes
21:51:05.815 in Ack 17
21:51:05.905 fetch POST /api/observability/v1/traces
21:51:05.905 fetched POST /api/observability/v1/traces 204
21:51:06.064 chunk 17 35045 bytes
21:51:06.066 in Ack 17
21:51:06.315 chunk 17 35057 bytes
21:51:06.316 in Ack 17
21:51:06.565 chunk 17 35079 bytes
21:51:06.566 in Ack 17
21:51:06.814 chunk 17 35089 bytes
21:51:06.816 in Ack 17
21:51:07.065 chunk 17 35101 bytes
21:51:07.067 in Ack 17
21:51:07.079 fetch POST /api/observability/v1/traces
21:51:07.079 fetched POST /api/observability/v1/traces 204
21:51:07.315 chunk 17 35103 bytes
21:51:07.316 in Ack 17
21:51:07.565 chunk 17 35123 bytes
21:51:07.567 in Ack 17
21:51:07.815 chunk 17 35179 bytes
21:51:07.816 in Ack 17
21:51:08.066 chunk 17 35171 bytes
21:51:08.068 in Ack 17
21:51:08.315 chunk 17 35183 bytes
21:51:08.317 in Ack 17
21:51:08.326 fetch POST /api/observability/v1/traces
21:51:08.326 fetched POST /api/observability/v1/traces 204
21:51:08.565 chunk 17 35257 bytes
21:51:08.566 in Ack 17
21:51:08.815 chunk 17 35277 bytes
21:51:08.816 in Ack 17
21:51:08.891 in Ping 
21:51:09.065 chunk 17 35297 bytes
21:51:09.066 in Ack 17
21:51:09.315 chunk 17 35319 bytes
21:51:09.316 in Ack 17
21:51:09.565 chunk 17 35339 bytes
21:51:09.566 in Ack 17
21:51:09.576 fetch POST /api/observability/v1/traces
21:51:09.576 fetched POST /api/observability/v1/traces 204
21:51:09.814 chunk 11 2884 bytes
21:51:09.815 chunk 17 36007 bytes
21:51:09.815 in Ack 11
21:51:09.851 in Ack 17
21:51:10.065 chunk 17 36027 bytes
21:51:10.066 in Ack 17
21:51:10.315 chunk 17 36065 bytes
21:51:10.316 in Ack 17
21:51:10.565 chunk 17 36085 bytes
21:51:10.566 in Ack 17
21:51:10.815 chunk 17 38118 bytes
21:51:10.816 in Ack 17
21:51:10.827 fetch POST /api/observability/v1/traces
21:51:10.827 fetched POST /api/observability/v1/traces 204
21:51:11.065 chunk 17 38128 bytes
21:51:11.066 in Ack 17
21:51:11.316 chunk 17 38980 bytes
21:51:11.317 in Ack 17
21:51:11.459 rpc 30 orchestration.dispatchCommand thread.visit
21:51:11.459 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:51:11.567 chunk 17 39002 bytes
21:51:11.568 in Ack 17
21:51:11.818 chunk 17 39022 bytes
21:51:11.819 in Ack 17
21:51:12.067 fetch POST /api/observability/v1/traces
21:51:12.068 fetched POST /api/observability/v1/traces 204
21:51:12.069 chunk 17 39080 bytes
21:51:12.070 in Ack 17
21:51:12.320 chunk 17 39749 bytes
21:51:12.321 in Ack 17
21:51:12.570 chunk 17 39759 bytes
21:51:12.571 in Ack 17
21:51:12.820 chunk 17 39809 bytes
21:51:12.821 in Ack 17
21:51:13.071 fetch POST /api/observability/v1/traces
21:51:13.072 fetched POST /api/observability/v1/traces 204
21:51:13.320 chunk 17 40616 bytes
21:51:13.321 in Ack 17
21:51:13.570 chunk 11 2884 bytes
21:51:13.570 chunk 17 40666 bytes
21:51:13.571 in Ack 11
21:51:13.575 in Ack 17
21:51:13.820 chunk 17 40694 bytes
21:51:13.821 in Ack 17
21:51:13.891 in Ping 
21:51:14.071 chunk 17 40714 bytes
21:51:14.072 in Ack 17
21:51:14.321 chunk 17 40734 bytes
21:51:14.322 fetch POST /api/observability/v1/traces
21:51:14.322 fetched POST /api/observability/v1/traces 204
21:51:14.323 in Ack 17
21:51:14.572 chunk 17 42698 bytes
21:51:14.574 in Ack 17
21:51:14.734 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/bef60095-a247-4d1d-8025-b2b7a1c1036b","expiresAt":1794178274734}}
21:51:14.822 chunk 17 43506 bytes
21:51:14.824 in Ack 17
21:51:15.072 chunk 17 43546 bytes
21:51:15.073 in Ack 17
21:51:15.324 fetch POST /api/observability/v1/traces
21:51:15.324 fetched POST /api/observability/v1/traces 204
21:51:15.572 chunk 17 43596 bytes
21:51:15.573 in Ack 17
21:51:15.822 chunk 17 43606 bytes
21:51:15.824 in Ack 17
21:51:16.072 chunk 17 43626 bytes
21:51:16.075 in Ack 17
21:51:16.322 chunk 17 43656 bytes
21:51:16.323 in Ack 17
21:51:16.571 chunk 11 2884 bytes
21:51:16.572 chunk 17 43686 bytes
21:51:16.572 in Ack 11
21:51:16.574 in Ack 17
21:51:16.583 fetch POST /api/observability/v1/traces
21:51:16.583 fetched POST /api/observability/v1/traces 204
21:51:16.822 chunk 17 45350 bytes
21:51:16.824 in Ack 17
21:51:17.072 chunk 17 45395 bytes
21:51:17.073 in Ack 17
21:51:17.322 chunk 17 45425 bytes
21:51:17.323 in Ack 17
21:51:17.588 chunk 17 45470 bytes
21:51:17.589 fetch POST /api/observability/v1/traces
21:51:17.589 fetched POST /api/observability/v1/traces 204
21:51:17.591 in Ack 17
21:51:17.838 chunk 17 45500 bytes
21:51:17.839 in Ack 17
21:51:18.088 chunk 17 45545 bytes
21:51:18.089 in Ack 17
21:51:18.338 chunk 17 45590 bytes
21:51:18.339 in Ack 17
21:51:18.589 chunk 17 45620 bytes
21:51:18.590 in Ack 17
21:51:18.599 fetch POST /api/observability/v1/traces
21:51:18.599 fetched POST /api/observability/v1/traces 204
21:51:18.841 chunk 17 45665 bytes
21:51:18.842 in Ack 17
21:51:18.891 in Ping 
21:51:19.091 chunk 17 45710 bytes
21:51:19.093 in Ack 17
21:51:19.125 rpc 31 server.reportClientActivity
21:51:19.342 chunk 17 45725 bytes
21:51:19.343 in Ack 17
21:51:19.592 chunk 17 45755 bytes
21:51:19.593 in Ack 17
21:51:19.842 chunk 17 45785 bytes
21:51:19.843 in Ack 17
21:51:19.862 fetch POST /api/observability/v1/traces
21:51:19.862 fetched POST /api/observability/v1/traces 204
21:51:20.092 chunk 17 45815 bytes
21:51:20.093 in Ack 17
21:51:20.345 chunk 17 45845 bytes
21:51:20.346 in Ack 17
21:51:20.595 chunk 17 45875 bytes
21:51:20.596 in Ack 17
21:51:20.845 chunk 17 45920 bytes
21:51:20.846 in Ack 17
21:51:21.099 fetch POST /api/observability/v1/traces
21:51:21.099 fetched POST /api/observability/v1/traces 204
21:51:21.100 chunk 17 45950 bytes
21:51:21.101 in Ack 17
21:51:21.349 chunk 17 45980 bytes
21:51:21.350 in Ack 17
21:51:21.459 rpc 32 orchestration.dispatchCommand thread.visit
21:51:21.459 exit 32 {"_tag":"Success","value":{"sequence":0}}
21:51:21.536 exit 31 {"_tag":"Success","value":null}
21:51:21.601 chunk 17 46010 bytes
21:51:21.602 in Ack 17
21:51:21.872 chunk 17 46070 bytes
21:51:21.874 in Ack 17
21:51:22.102 fetch POST /api/observability/v1/traces
21:51:22.102 fetched POST /api/observability/v1/traces 204
21:51:22.122 chunk 17 46130 bytes
21:51:22.124 in Ack 17
21:51:22.372 chunk 17 46175 bytes
21:51:22.374 in Ack 17
21:51:22.623 chunk 11 2884 bytes
21:51:22.623 chunk 17 46205 bytes
21:51:22.624 in Ack 11
21:51:22.630 in Ack 17
21:51:22.874 chunk 17 46265 bytes
21:51:22.875 in Ack 17
21:51:23.129 fetch POST /api/observability/v1/traces
21:51:23.130 fetched POST /api/observability/v1/traces 204
21:51:23.175 chunk 17 46310 bytes
21:51:23.176 in Ack 17
21:51:23.556 chunk 17 46355 bytes
21:51:23.557 in Ack 17
21:51:23.806 chunk 17 46400 bytes
21:51:23.807 in Ack 17
21:51:23.891 in Ping 
21:51:24.057 chunk 17 46445 bytes
21:51:24.058 in Ack 17
21:51:24.180 fetch POST /api/observability/v1/traces
21:51:24.180 fetched POST /api/observability/v1/traces 204
21:51:24.307 chunk 17 46475 bytes
21:51:24.308 in Ack 17
21:51:24.557 chunk 17 46520 bytes
21:51:24.558 in Ack 17
21:51:24.809 chunk 17 46550 bytes
21:51:24.810 in Ack 17
21:51:25.059 chunk 17 46595 bytes
21:51:25.061 in Ack 17
21:51:25.450 fetch POST /api/observability/v1/traces
21:51:25.450 fetched POST /api/observability/v1/traces 204
21:51:25.450 chunk 17 46625 bytes
21:51:25.452 in Ack 17
21:51:25.702 chunk 17 46670 bytes
21:51:25.703 in Ack 17
21:51:25.952 chunk 17 46745 bytes
21:51:25.953 in Ack 17
21:51:26.202 chunk 17 46760 bytes
21:51:26.203 in Ack 17
21:51:26.452 chunk 17 46790 bytes
21:51:26.453 fetch POST /api/observability/v1/traces
21:51:26.453 fetched POST /api/observability/v1/traces 204
21:51:26.454 in Ack 17
21:51:26.703 chunk 17 46820 bytes
21:51:26.704 in Ack 17
21:51:26.953 chunk 17 46865 bytes
21:51:26.954 in Ack 17
21:51:27.203 chunk 17 46910 bytes
21:51:27.211 in Ack 17
21:51:27.453 chunk 17 46940 bytes
21:51:27.453 in Ack 17
21:51:27.473 fetch POST /api/observability/v1/traces
21:51:27.473 fetched POST /api/observability/v1/traces 204
21:51:27.703 chunk 17 46970 bytes
21:51:27.704 in Ack 17
21:51:27.953 chunk 17 47000 bytes
21:51:27.954 in Ack 17
21:51:28.203 chunk 17 47030 bytes
21:51:28.204 in Ack 17
21:51:28.453 chunk 17 47090 bytes
21:51:28.454 in Ack 17
21:51:28.703 chunk 17 47135 bytes
21:51:28.704 in Ack 17
21:51:28.724 fetch POST /api/observability/v1/traces
21:51:28.724 fetched POST /api/observability/v1/traces 204
21:51:28.892 in Ping 
21:51:28.953 chunk 17 47165 bytes
21:51:28.954 in Ack 17
21:51:29.203 chunk 17 47195 bytes
21:51:29.205 in Ack 17
21:51:29.453 chunk 17 47240 bytes
21:51:29.454 in Ack 17
21:51:29.703 chunk 17 47270 bytes
21:51:29.704 in Ack 17
21:51:29.953 chunk 17 47315 bytes
21:51:29.954 in Ack 17
21:51:29.963 fetch POST /api/observability/v1/traces
21:51:29.963 fetched POST /api/observability/v1/traces 204
21:51:30.204 chunk 17 47345 bytes
21:51:30.205 in Ack 17
21:51:30.454 chunk 17 47375 bytes
21:51:30.455 in Ack 17
21:51:30.705 chunk 17 47405 bytes
21:51:30.708 in Ack 17
21:51:30.955 chunk 17 47450 bytes
21:51:30.956 in Ack 17
21:51:31.204 chunk 17 47480 bytes
21:51:31.205 in Ack 17
21:51:31.232 fetch POST /api/observability/v1/traces
21:51:31.232 fetched POST /api/observability/v1/traces 204
21:51:31.455 chunk 17 47510 bytes
21:51:31.456 in Ack 17
21:51:31.710 chunk 17 47555 bytes
21:51:31.711 in Ack 17
21:51:31.960 chunk 17 47585 bytes
21:51:31.961 in Ack 17
21:51:32.210 chunk 17 47642 bytes
21:51:32.211 in Ack 17
21:51:32.458 fetch POST /api/observability/v1/traces
21:51:32.458 fetched POST /api/observability/v1/traces 204
21:51:32.460 chunk 17 48650 bytes
21:51:32.461 in Ack 17
21:51:32.676 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:51:32.725 chunk 17 52155 bytes
21:51:32.730 in Ack 17
21:51:32.975 chunk 17 54238 bytes
21:51:32.976 in Ack 17
21:51:33.225 chunk 17 54288 bytes
21:51:33.226 in Ack 17
21:51:33.462 fetch POST /api/observability/v1/traces
21:51:33.462 fetched POST /api/observability/v1/traces 204
21:51:33.475 chunk 17 55252 bytes
21:51:33.477 in Ack 17
21:51:33.750 chunk 17 57754 bytes
21:51:33.750 exit 33 {"_tag":"Success","value":{"sequence":97}}
21:51:33.751 in Ack 17
21:51:33.892 in Ping 
21:51:33.991 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:51:34.001 chunk 17 57774 bytes
21:51:34.002 in Ack 17
21:51:34.251 chunk 17 57746 bytes
21:51:34.252 in Ack 17
21:51:34.478 fetch POST /api/observability/v1/traces
21:51:34.478 fetched POST /api/observability/v1/traces 204
21:51:34.501 chunk 17 57766 bytes
21:51:34.502 in Ack 17
21:51:34.751 chunk 17 57786 bytes
21:51:34.752 in Ack 17
21:51:35.001 chunk 17 57854 bytes
21:51:35.002 in Ack 17
21:51:35.009 in Interrupt 27
21:51:35.033 rpc 35 subscribeVcsStatus
21:51:35.251 chunk 17 57864 bytes
21:51:35.252 in Ack 17
21:51:35.284 rpc 36 server.reportClientActivity
21:51:35.352 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:51:35.567 chunk 17 57967 bytes
21:51:35.567 fetch POST /api/observability/v1/traces
21:51:35.567 fetched POST /api/observability/v1/traces 204
21:51:35.569 in Ack 17
21:51:35.817 chunk 17 58007 bytes
21:51:35.818 in Ack 17
21:51:36.067 chunk 17 58017 bytes
21:51:36.068 in Ack 17
21:51:36.141 chunk 35 472 bytes
21:51:36.141 in Ack 35
21:51:36.318 chunk 17 58037 bytes
21:51:36.319 in Ack 17
21:51:36.569 chunk 17 58057 bytes
21:51:36.570 in Ack 17
21:51:36.579 fetch POST /api/observability/v1/traces
21:51:36.579 fetched POST /api/observability/v1/traces 204
21:51:36.818 chunk 17 58067 bytes
21:51:36.820 in Ack 17
21:51:37.069 chunk 17 58155 bytes
21:51:37.070 in Ack 17
21:51:37.139 chunk 11 2886 bytes
21:51:37.140 in Ack 11
21:51:37.142 rpc 37 orchestration.dispatchCommand thread.visit
21:51:37.142 exit 37 {"_tag":"Success","value":{"sequence":0}}
21:51:37.320 chunk 17 58185 bytes
21:51:37.321 in Ack 17
21:51:37.570 chunk 17 58215 bytes
21:51:37.571 in Ack 17
21:51:37.821 chunk 17 58259 bytes
21:51:37.822 fetch POST /api/observability/v1/traces
21:51:37.822 fetched POST /api/observability/v1/traces 204
21:51:37.824 in Ack 17
21:51:38.071 chunk 17 58269 bytes
21:51:38.073 in Ack 17
21:51:38.322 chunk 17 58289 bytes
21:51:38.323 in Ack 17
21:51:38.572 chunk 17 59718 bytes
21:51:38.573 in Ack 17
21:51:38.822 chunk 17 59748 bytes
21:51:38.823 in Ack 17
21:51:38.842 fetch POST /api/observability/v1/traces
21:51:38.842 fetched POST /api/observability/v1/traces 204
21:51:38.892 in Ping 
21:51:38.901 chunk 11 2886 bytes
21:51:38.901 in Ack 11
21:51:39.072 chunk 17 59778 bytes
21:51:39.073 in Ack 17
21:51:39.330 chunk 17 59788 bytes
21:51:39.331 in Ack 17
21:51:39.581 chunk 17 59808 bytes
21:51:39.582 in Ack 17
21:51:39.864 chunk 17 59818 bytes
21:51:39.866 in Ack 17
21:51:39.902 fetch POST /api/observability/v1/traces
21:51:39.902 fetched POST /api/observability/v1/traces 204
21:51:40.116 chunk 17 59828 bytes
21:51:40.117 in Ack 17
21:51:40.366 chunk 17 59838 bytes
21:51:40.367 in Ack 17
21:51:40.617 chunk 17 60892 bytes
21:51:40.618 in Ack 17
21:51:40.881 chunk 17 60932 bytes
21:51:40.882 in Ack 17
21:51:41.117 fetch POST /api/observability/v1/traces
21:51:41.117 fetched POST /api/observability/v1/traces 204
21:51:41.131 chunk 17 60982 bytes
21:51:41.133 in Ack 17
21:51:41.381 chunk 17 61022 bytes
21:51:41.382 in Ack 17
21:51:41.631 chunk 17 61032 bytes
21:51:41.632 in Ack 17
21:51:41.725 exit 36 {"_tag":"Success","value":null}
21:51:41.897 chunk 17 62040 bytes
21:51:41.898 in Ack 17
21:51:41.974 chunk 11 2886 bytes
21:51:42.003 in Ack 11
21:51:42.138 fetch POST /api/observability/v1/traces
21:51:42.138 fetched POST /api/observability/v1/traces 204
21:51:42.147 chunk 17 62085 bytes
21:51:42.148 in Ack 17
21:51:42.397 chunk 17 62160 bytes
21:51:42.398 in Ack 17
21:51:42.647 chunk 17 62175 bytes
21:51:42.648 in Ack 17
21:51:42.909 chunk 17 62256 bytes
21:51:42.910 in Ack 17
21:51:43.149 fetch POST /api/observability/v1/traces
21:51:43.150 fetched POST /api/observability/v1/traces 204
21:51:43.159 chunk 17 62307 bytes
21:51:43.160 in Ack 17
21:51:43.418 chunk 17 62337 bytes
21:51:43.419 in Ack 17
21:51:43.668 chunk 17 62382 bytes
21:51:43.670 in Ack 17
21:51:43.892 in Ping 
21:51:43.918 chunk 17 62418 bytes
21:51:43.919 in Ack 17
21:51:44.125 rpc 38 server.reportClientActivity
21:51:44.160 fetch POST /api/observability/v1/traces
21:51:44.161 fetched POST /api/observability/v1/traces 204
21:51:44.168 chunk 17 62448 bytes
21:51:44.169 in Ack 17
21:51:44.420 chunk 17 62493 bytes
21:51:44.421 in Ack 17
21:51:44.670 chunk 17 62538 bytes
21:51:44.671 in Ack 17
21:51:44.922 chunk 17 62577 bytes
21:51:44.923 in Ack 17
21:51:45.170 fetch POST /api/observability/v1/traces
21:51:45.170 fetched POST /api/observability/v1/traces 204
21:51:45.173 chunk 17 62607 bytes
21:51:45.174 in Ack 17
21:51:45.424 chunk 17 62652 bytes
21:51:45.425 in Ack 17
21:51:45.674 chunk 17 62709 bytes
21:51:45.675 in Ack 17
21:51:45.930 chunk 17 62748 bytes
21:51:45.931 in Ack 17
21:51:46.175 fetch POST /api/observability/v1/traces
21:51:46.175 fetched POST /api/observability/v1/traces 204
21:51:46.180 chunk 17 62778 bytes
21:51:46.181 in Ack 17
21:51:46.430 chunk 17 62808 bytes
21:51:46.431 in Ack 17
21:51:46.680 chunk 17 62859 bytes
21:51:46.682 in Ack 17
21:51:46.930 chunk 17 62874 bytes
21:51:46.932 in Ack 17
21:51:47.008 exit 38 {"_tag":"Success","value":null}
21:51:47.143 rpc 39 orchestration.dispatchCommand thread.visit
21:51:47.143 exit 39 {"_tag":"Success","value":{"sequence":0}}
21:51:47.180 chunk 17 62943 bytes
21:51:47.182 in Ack 17
21:51:47.277 fetch POST /api/observability/v1/traces
21:51:47.277 fetched POST /api/observability/v1/traces 204
21:51:47.440 chunk 17 62976 bytes
21:51:47.441 in Ack 17
21:51:47.690 chunk 17 63074 bytes
21:51:47.691 in Ack 17
21:51:47.943 chunk 17 63206 bytes
21:51:47.945 in Ack 17
21:51:48.293 chunk 11 2822 bytes
21:51:48.293 in Ack 11
21:51:48.295 rpc 40 orchestration.dispatchCommand thread.visit
21:51:48.295 exit 40 {"_tag":"Success","value":{"sequence":0}}
21:51:48.442 fetch POST /api/observability/v1/traces
21:51:48.442 fetched POST /api/observability/v1/traces 204
21:51:48.443 chunk 17 63206 bytes
21:51:48.444 in Ack 17
21:51:48.824 chunk 11 2822 bytes
21:51:48.936 in Ack 11
21:51:48.941 in Ping 
21:51:48.944 chunk 17 65654 bytes
21:51:48.945 in Ack 17
