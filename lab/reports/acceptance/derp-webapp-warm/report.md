# derp, webapp, warm start, seed 1

Profile derp (fault): all traffic in one TCP connection over a 10% loss link, like a DERP relay. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar, its UDP and the app's HTTPS carried in one TCP connection across the shaped link (MTU 1280), like a Tailscale DERP relay.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.5s |
| Time to first text | 1.2s |
| Whole reply shown | 46.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.4s / 1.3s / 2.9s |
| Longest silence | 1.6s |
| Approval shown after it was asked | 0.9s |
| Bytes on the wire down / up | 419764 / 194590 |
| Bytes the app received / sent | 323054 / 52991 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.4 | 361 | 377 |
| 20 | 2.6 | 672 | 779 |
| 30 | 1.0 | 1086 | 1124 |
| 40 | 2.0 | 1429 | 1499 |
| 46 | 3.4 | 1587 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"776/776 776/776","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","nodes":"809/809 1022/1022 809/809 1022/1022","providerThreads":"809/809","providerSessions":"2691/2691","messages":"548/548 456/456 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":211,"lastDataAt":1794168305567,"silences":0,"longestSilenceMs":1496,"longestRecoveredGapMs":1496},"stats":{"sendRate":6750,"deliveredRate":3115.2017013401546,"loss":0,"srttMs":8.364340428068042,"minRttMs":2,"sentPackets":93,"sentWireBytes":28211,"receivedPackets":130,"receivedWireBytes":61730,"lastHeardMs":8324,"unsyncedBytes":509,"inflightPackets":5,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 460/460","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","nodes":"809/809 1022/1022 809/809 1022/1022 785/785","providerThreads":"811/811","providerSessions":"2691/2691","messages":"548/548 456/456 901/901 548/548 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":211,"lastDataAt":1794168316369,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.012250000000000006,"srttMs":572.407101515774,"minRttMs":535,"sentPackets":147,"sentWireBytes":38376,"receivedPackets":320,"receivedWireBytes":126468,"lastHeardMs":19126,"unsyncedBytes":0,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 873/873","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","nodes":"809/809 1022/1022 809/809 1022/1022 785/785","providerThreads":"811/811","providerSessions":"2691/2691","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":211,"lastDataAt":1794168326771,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1034.7023120980143,"minRttMs":535,"sentPackets":200,"sentWireBytes":49529,"receivedPackets":760,"receivedWireBytes":224145,"lastHeardMs":29528,"unsyncedBytes":167,"inflightPackets":21,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 756/1103 1550/1550 72/929 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 138/138","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","nodes":"809/809 1022/1022 809/809 1022/1022 785/785","providerThreads":"811/811","providerSessions":"2691/2691","messages":"548/548 456/456 901/901 548/548 451/451 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":211,"lastDataAt":1794168337706,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1687.5,"deliveredRate":1517.391304347826,"loss":0,"srttMs":1492.8978960666823,"minRttMs":369,"sentPackets":244,"sentWireBytes":58999,"receivedPackets":934,"receivedWireBytes":290892,"lastHeardMs":40463,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 589/589","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","nodes":"809/809 1022/1022 809/809 1022/1022 785/785 987/987 1398/1398 1022/1022","providerThreads":"811/811","providerSessions":"2691/2691","messages":"548/548 456/456 901/901 548/548 451/451 548/548 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":211,"lastDataAt":1794168347670,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":2448.5674784933735,"deliveredRate":1679.6834568372801,"loss":0,"srttMs":1046.509941550331,"minRttMs":369,"sentPackets":300,"sentWireBytes":70933,"receivedPackets":1111,"receivedWireBytes":356008,"lastHeardMs":50427,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- shim trace
20:04:57.544 fetch GET /api/auth/session
20:04:57.545 fetched GET /api/auth/session 200
20:04:57.658 fetch GET /.well-known/t3/environment
20:04:57.659 fetched GET /.well-known/t3/environment 200
20:04:57.668 fetch GET /.well-known/t3/environment
20:04:57.669 fetched GET /.well-known/t3/environment 200
20:04:57.673 rpc 0 subscribeServerConfig
20:04:57.673 chunk 0 38702 bytes
20:04:57.704 in Ack 0
20:04:57.705 rpc 1 subscribeServerLifecycle
20:04:57.706 rpc 2 subscribeServerLifecycle
20:04:57.709 fetch GET /api/auth/session
20:04:57.710 fetched GET /api/auth/session 200
20:04:57.711 rpc 3 subscribeDeviceState
20:04:57.713 rpc 4 preview.reportProfiles
20:04:57.786 rpc 5 subscribeProjectClones
20:04:57.804 fetch GET /api/orchestration/threads/mcp%3A7781ec6b-205c-42d2-81d6-e1e7944aea1d/bounded
20:04:57.806 fetched GET /api/orchestration/threads/mcp%3A7781ec6b-205c-42d2-81d6-e1e7944aea1d/bounded 200
20:04:57.809 rpc 6 orchestration.subscribeThread
20:04:57.809 chunk 6 9252 bytes
20:04:57.814 fetch GET /api/orchestration/shell
20:04:57.842 rpc 7 server.refreshProviders
20:04:57.910 rpc 8 orchestration.dispatchCommand thread.visit
20:04:57.910 exit 8 {"_tag":"Success","value":{"sequence":0}}
20:04:57.979 fetched GET /api/orchestration/shell 200
20:04:57.979 rpc 9 subscribeTerminalMetadata
20:04:57.980 rpc 10 subscribePreviewEvents
20:04:57.981 rpc 11 subscribeVcsStatus
20:04:57.983 rpc 12 orchestration.subscribeShell
20:04:57.983 chunk 12 2785 bytes
20:04:57.985 rpc 13 assets.createUrl
20:04:57.985 rpc 14 preview.list
20:04:57.986 rpc 15 projects.readFile
20:04:57.988 in Ack 6
20:04:58.003 in Ack 12
20:04:58.059 chunk 6 9228 bytes
20:04:58.060 in Ack 6
20:04:58.100 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c1b79c74-152d-49c7-9e66-e01cefa14b33","sessions":[]}}
20:04:58.231 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
20:04:58.231 chunk 1 3867 bytes
20:04:58.233 in Ack 1
20:04:58.234 rpc 16 server.reportClientActivity
20:04:58.235 chunk 12 2761 bytes
20:04:58.235 in Ack 12
20:04:58.310 chunk 6 9292 bytes
20:04:58.310 in Ack 6
20:04:58.364 chunk 2 3867 bytes
20:04:58.365 in Ack 2
20:04:58.479 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T20:04:40.826Z","configurableProviders":false,"con
20:04:58.479 chunk 11 638 bytes
20:04:58.479 exit 13 {"_tag":"Success","value":{"expiresAt":1794171600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
20:04:58.482 in Ack 11
20:04:58.656 fetch POST /api/observability/v1/traces
20:04:58.656 fetched POST /api/observability/v1/traces 204
20:04:58.758 chunk 12 2804 bytes
20:04:58.763 in Ack 12
20:04:58.827 chunk 6 12047 bytes
20:04:58.830 in Ack 6
20:04:59.077 chunk 6 13517 bytes
20:04:59.079 in Ack 6
20:04:59.359 rpc 17 attachments.createUploadUrl
20:04:59.530 chunk 6 18040 bytes
20:04:59.546 in Ack 6
20:04:59.620 exit 17 {"_tag":"Success","value":{"attachmentId":"pending-9862ca3b-50c1-48be-9310-1387a6905bca","expiresAt":1794168899588,"relativeUrl":"/api/attachments/upload/eyJ2ZX
20:04:59.620 chunk 3 876 bytes
20:04:59.620 exit 4 {"_tag":"Success","value":null}
20:04:59.621 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTk4NjJjYTNiLTUwYzEtNDhiZS05MzEwLTEzODdhNjkwNWJjYSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODg5OTU4OH0.nI_5JL0fkWQBFLVEY4aFnuSm6GAbOxgrWByQl8g5nZo
20:04:59.623 in Ack 3
20:04:59.642 rpc 18 preview.reportProfiles
20:04:59.688 chunk 5 4 bytes
20:04:59.688 chunk 9 36 bytes
20:04:59.688 exit 16 {"_tag":"Success","value":null}
20:04:59.689 in Ack 5
20:04:59.690 in Ack 9
20:04:59.764 fetch POST /api/observability/v1/traces
20:04:59.764 fetched POST /api/observability/v1/traces 204
20:04:59.781 chunk 6 21209 bytes
20:04:59.782 in Ack 6
20:04:59.790 in Interrupt 11
20:04:59.792 rpc 19 subscribeVcsStatus
20:04:59.893 chunk 19 638 bytes
20:04:59.893 in Ack 19
20:05:00.043 rpc 20 server.reportClientActivity
20:05:00.783 fetch POST /api/observability/v1/traces
20:05:00.783 fetched POST /api/observability/v1/traces 204
20:05:01.505 exit 18 {"_tag":"Success","value":null}
20:05:01.587 exit 20 {"_tag":"Success","value":null}
20:05:02.507 fetch POST /api/observability/v1/traces
20:05:02.507 fetched POST /api/observability/v1/traces 204
20:05:02.637 rpc 21 orchestration.dispatchCommand message.dispatch
20:05:03.213 in Ping 
20:05:03.280 exit 21 {"_tag":"Success","value":{"sequence":45}}
20:05:03.281 rpc 22 attachments.delete
20:05:03.318 exit 22 {"_tag":"Success","value":null}
20:05:03.529 chunk 6 27925 bytes
20:05:03.529 chunk 12 2890 bytes
20:05:03.531 in Ack 6
20:05:03.533 rpc 23 assets.createUrl
20:05:03.613 in Ack 12
20:05:03.636 fetch POST /api/observability/v1/traces
20:05:03.636 fetched POST /api/observability/v1/traces 204
20:05:03.779 chunk 6 29332 bytes
20:05:03.780 in Ack 6
20:05:04.030 chunk 6 30267 bytes
20:05:04.031 in Ack 6
20:05:04.279 chunk 6 30449 bytes
20:05:04.280 in Ack 6
20:05:04.530 chunk 6 30473 bytes
20:05:04.531 in Ack 6
20:05:04.630 chunk 12 2826 bytes
20:05:04.631 in Ack 12
20:05:04.632 rpc 24 orchestration.dispatchCommand thread.visit
20:05:04.632 exit 24 {"_tag":"Success","value":{"sequence":0}}
20:05:04.780 chunk 6 32920 bytes
20:05:04.782 in Ack 6
20:05:04.792 fetch POST /api/observability/v1/traces
20:05:04.792 fetched POST /api/observability/v1/traces 204
20:05:04.881 chunk 12 2826 bytes
20:05:04.882 in Ack 12
20:05:05.031 chunk 6 33534 bytes
20:05:05.032 in Ack 6
20:05:05.040 in Interrupt 19
20:05:05.042 rpc 25 subscribeVcsStatus
20:05:05.070 chunk 25 638 bytes
20:05:05.070 in Ack 25
20:05:05.529 chunk 6 35243 bytes
20:05:05.531 rpc 26 server.reportClientActivity
20:05:05.533 in Ack 6
20:05:05.569 exit 26 {"_tag":"Success","value":null}
20:05:05.883 fetch POST /api/observability/v1/traces
20:05:05.883 fetched POST /api/observability/v1/traces 204
20:05:06.383 rpc 27 orchestration.dispatchCommand message.dispatch
20:05:07.180 exit 23 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.1.1:8443/827570e4-f2e8-4e2c-8cac-3cf3c536023e","expiresAt":1794171907180}}
20:05:07.181 exit 27 {"_tag":"Success","value":{"sequence":75}}
20:05:07.383 fetch POST /api/observability/v1/traces
20:05:07.383 fetched POST /api/observability/v1/traces 204
20:05:07.423 chunk 6 37698 bytes
20:05:07.424 in Ack 6
20:05:07.673 chunk 6 37728 bytes
20:05:07.674 in Ack 6
20:05:08.140 chunk 6 37738 bytes
20:05:08.144 in Ack 6
20:05:08.214 in Ping 
20:05:08.390 chunk 6 37748 bytes
20:05:08.391 in Ack 6
20:05:08.430 fetch POST /api/observability/v1/traces
20:05:08.430 fetched POST /api/observability/v1/traces 204
20:05:08.642 chunk 6 41241 bytes
20:05:08.643 in Ack 6
20:05:08.730 chunk 12 2890 bytes
20:05:08.734 in Ack 12
20:05:08.938 chunk 6 41251 bytes
20:05:08.942 in Ack 6
20:05:09.190 chunk 6 41301 bytes
20:05:09.193 in Ack 6
20:05:09.441 chunk 6 41331 bytes
20:05:09.442 in Ack 6
20:05:09.490 fetch POST /api/observability/v1/traces
20:05:09.490 fetched POST /api/observability/v1/traces 204
20:05:09.692 chunk 6 41351 bytes
20:05:09.693 in Ack 6
20:05:09.941 chunk 6 42030 bytes
20:05:09.943 in Ack 6
20:05:10.272 chunk 6 42050 bytes
20:05:10.275 in Ack 6
20:05:10.522 chunk 6 42100 bytes
20:05:10.524 in Ack 6
20:05:10.694 fetch POST /api/observability/v1/traces
20:05:10.694 fetched POST /api/observability/v1/traces 204
20:05:10.773 chunk 6 42130 bytes
20:05:10.774 in Ack 6
20:05:11.031 chunk 6 42946 bytes
20:05:11.032 in Ack 6
20:05:11.281 chunk 6 42958 bytes
20:05:11.282 in Ack 6
20:05:11.673 chunk 6 42978 bytes
20:05:11.675 in Ack 6
20:05:11.776 fetch POST /api/observability/v1/traces
20:05:11.776 fetched POST /api/observability/v1/traces 204
20:05:11.923 chunk 6 43544 bytes
20:05:11.924 in Ack 6
20:05:12.173 chunk 6 43574 bytes
20:05:12.175 in Ack 6
20:05:12.424 chunk 6 43594 bytes
20:05:12.425 in Ack 6
20:05:12.674 chunk 6 43604 bytes
20:05:12.675 in Ack 6
20:05:12.925 fetch POST /api/observability/v1/traces
20:05:12.925 fetched POST /api/observability/v1/traces 204
20:05:12.978 chunk 6 43614 bytes
20:05:12.979 in Ack 6
20:05:13.214 in Ping 
20:05:13.227 chunk 6 43634 bytes
20:05:13.228 in Ack 6
20:05:13.478 chunk 6 43684 bytes
20:05:13.481 in Ack 6
20:05:13.728 chunk 6 43704 bytes
20:05:13.729 in Ack 6
20:05:13.978 chunk 6 43744 bytes
20:05:13.980 in Ack 6
20:05:13.992 fetch POST /api/observability/v1/traces
20:05:13.992 fetched POST /api/observability/v1/traces 204
20:05:14.228 chunk 6 43754 bytes
20:05:14.230 in Ack 6
20:05:14.627 chunk 6 43764 bytes
20:05:14.628 in Ack 6
20:05:14.646 rpc 28 orchestration.dispatchCommand thread.visit
20:05:14.646 exit 28 {"_tag":"Success","value":{"sequence":0}}
20:05:14.877 chunk 6 43824 bytes
20:05:14.878 in Ack 6
20:05:15.127 chunk 6 43844 bytes
20:05:15.128 in Ack 6
20:05:15.231 fetch POST /api/observability/v1/traces
20:05:15.231 fetched POST /api/observability/v1/traces 204
20:05:15.377 chunk 6 43894 bytes
20:05:15.383 in Ack 6
20:05:15.435 chunk 12 2890 bytes
20:05:15.437 in Ack 12
20:05:15.634 chunk 6 43904 bytes
20:05:15.635 in Ack 6
20:05:15.884 chunk 6 43934 bytes
20:05:15.885 in Ack 6
20:05:16.134 chunk 6 43974 bytes
20:05:16.135 in Ack 6
20:05:16.384 fetch POST /api/observability/v1/traces
20:05:16.384 fetched POST /api/observability/v1/traces 204
20:05:16.385 chunk 6 43994 bytes
20:05:16.386 in Ack 6
20:05:16.764 chunk 6 44004 bytes
20:05:16.766 in Ack 6
20:05:17.021 chunk 6 44044 bytes
20:05:17.022 in Ack 6
20:05:17.273 chunk 6 44074 bytes
20:05:17.273 chunk 12 2890 bytes
20:05:17.275 in Ack 6
20:05:17.317 in Ack 12
20:05:17.387 fetch POST /api/observability/v1/traces
20:05:17.387 fetched POST /api/observability/v1/traces 204
20:05:17.525 chunk 6 45167 bytes
20:05:17.527 in Ack 6
20:05:17.775 chunk 6 45197 bytes
20:05:17.776 in Ack 6
20:05:18.032 chunk 6 45242 bytes
20:05:18.033 in Ack 6
20:05:18.215 in Ping 
20:05:18.282 chunk 6 45272 bytes
20:05:18.283 in Ack 6
20:05:18.528 fetch POST /api/observability/v1/traces
20:05:18.528 fetched POST /api/observability/v1/traces 204
20:05:18.532 chunk 6 45317 bytes
20:05:18.534 in Ack 6
20:05:18.806 chunk 6 45332 bytes
20:05:18.807 in Ack 6
20:05:19.278 chunk 6 45347 bytes
20:05:19.280 in Ack 6
20:05:19.529 chunk 6 45392 bytes
20:05:19.531 in Ack 6
20:05:19.541 fetch POST /api/observability/v1/traces
20:05:19.541 fetched POST /api/observability/v1/traces 204
20:05:19.780 chunk 6 45437 bytes
20:05:19.781 in Ack 6
20:05:20.035 chunk 6 45452 bytes
20:05:20.036 in Ack 6
20:05:20.285 chunk 6 45512 bytes
20:05:20.286 in Ack 6
20:05:20.535 chunk 6 45557 bytes
20:05:20.537 in Ack 6
20:05:20.782 fetch POST /api/observability/v1/traces
20:05:20.783 fetched POST /api/observability/v1/traces 204
20:05:20.968 chunk 6 45587 bytes
20:05:20.970 in Ack 6
20:05:21.219 chunk 6 45692 bytes
20:05:21.220 in Ack 6
20:05:21.472 chunk 6 45707 bytes
20:05:21.474 in Ack 6
20:05:21.723 chunk 6 45752 bytes
20:05:21.725 in Ack 6
20:05:21.971 fetch POST /api/observability/v1/traces
20:05:21.971 fetched POST /api/observability/v1/traces 204
20:05:22.217 chunk 6 45767 bytes
20:05:22.219 in Ack 6
20:05:22.467 chunk 6 45842 bytes
20:05:22.469 in Ack 6
20:05:22.718 chunk 6 45872 bytes
20:05:22.719 in Ack 6
20:05:22.913 rpc 29 server.reportClientActivity
20:05:23.213 chunk 6 45887 bytes
20:05:23.214 in Ack 6
20:05:23.226 in Ping 
20:05:23.226 fetch POST /api/observability/v1/traces
20:05:23.227 fetched POST /api/observability/v1/traces 204
20:05:23.463 chunk 6 45932 bytes
20:05:23.464 in Ack 6
20:05:23.713 chunk 6 45962 bytes
20:05:23.714 in Ack 6
20:05:23.964 chunk 6 45977 bytes
20:05:23.965 in Ack 6
20:05:24.401 chunk 6 45992 bytes
20:05:24.403 in Ack 6
20:05:24.465 fetch POST /api/observability/v1/traces
20:05:24.465 fetched POST /api/observability/v1/traces 204
20:05:24.646 rpc 30 orchestration.dispatchCommand thread.visit
20:05:24.646 exit 30 {"_tag":"Success","value":{"sequence":0}}
20:05:24.652 chunk 6 46022 bytes
20:05:24.654 in Ack 6
20:05:24.903 chunk 6 46052 bytes
20:05:24.904 in Ack 6
20:05:25.571 chunk 6 46067 bytes
20:05:25.572 in Ack 6
20:05:25.647 fetch POST /api/observability/v1/traces
20:05:25.647 fetched POST /api/observability/v1/traces 204
20:05:25.821 chunk 6 46112 bytes
20:05:25.822 in Ack 6
20:05:26.072 chunk 6 46142 bytes
20:05:26.083 in Ack 6
20:05:26.323 chunk 6 46187 bytes
20:05:26.327 in Ack 6
20:05:26.574 chunk 6 46202 bytes
20:05:26.575 in Ack 6
20:05:26.632 exit 29 {"_tag":"Success","value":null}
20:05:26.823 fetch POST /api/observability/v1/traces
20:05:26.823 fetched POST /api/observability/v1/traces 204
20:05:26.825 chunk 6 46247 bytes
20:05:26.828 in Ack 6
20:05:27.102 chunk 6 46262 bytes
20:05:27.103 in Ack 6
20:05:27.352 chunk 6 46322 bytes
20:05:27.353 in Ack 6
20:05:27.830 fetch POST /api/observability/v1/traces
20:05:27.830 fetched POST /api/observability/v1/traces 204
20:05:27.887 chunk 6 46337 bytes
20:05:27.888 in Ack 6
20:05:28.137 chunk 6 46427 bytes
20:05:28.139 in Ack 6
20:05:28.226 in Ping 
20:05:28.388 chunk 6 46472 bytes
20:05:28.390 in Ack 6
20:05:28.639 chunk 6 46532 bytes
20:05:28.640 in Ack 6
20:05:28.889 fetch POST /api/observability/v1/traces
20:05:28.889 fetched POST /api/observability/v1/traces 204
20:05:29.023 chunk 6 46547 bytes
20:05:29.025 in Ack 6
20:05:29.273 chunk 6 46562 bytes
20:05:29.275 in Ack 6
20:05:29.524 chunk 6 46607 bytes
20:05:29.526 in Ack 6
20:05:29.963 chunk 6 46622 bytes
20:05:29.969 in Ack 6
20:05:30.068 fetch POST /api/observability/v1/traces
20:05:30.068 fetched POST /api/observability/v1/traces 204
20:05:30.213 chunk 6 46712 bytes
20:05:30.214 in Ack 6
20:05:30.225 chunk 12 2890 bytes
20:05:30.226 in Ack 12
20:05:30.464 chunk 6 46742 bytes
20:05:30.465 in Ack 6
20:05:30.718 chunk 6 46832 bytes
20:05:30.719 in Ack 6
20:05:30.976 chunk 6 46907 bytes
20:05:30.977 in Ack 6
20:05:31.216 fetch POST /api/observability/v1/traces
20:05:31.216 fetched POST /api/observability/v1/traces 204
20:05:31.227 chunk 6 47012 bytes
20:05:31.228 in Ack 6
20:05:31.922 chunk 6 47027 bytes
20:05:31.924 in Ack 6
20:05:32.173 chunk 6 47162 bytes
20:05:32.174 in Ack 6
20:05:32.229 fetch POST /api/observability/v1/traces
20:05:32.229 fetched POST /api/observability/v1/traces 204
20:05:32.428 chunk 6 47207 bytes
20:05:32.429 in Ack 6
20:05:32.683 chunk 6 47222 bytes
20:05:32.684 in Ack 6
20:05:32.933 chunk 6 47327 bytes
20:05:32.935 in Ack 6
20:05:33.184 chunk 6 47387 bytes
20:05:33.185 in Ack 6
20:05:33.267 in Ping 
20:05:33.430 fetch POST /api/observability/v1/traces
20:05:33.430 fetched POST /api/observability/v1/traces 204
20:05:33.434 chunk 6 47537 bytes
20:05:33.464 in Ack 6
20:05:34.008 chunk 6 47552 bytes
20:05:34.010 in Ack 6
20:05:34.258 chunk 6 47627 bytes
20:05:34.267 in Ack 6
20:05:34.466 fetch POST /api/observability/v1/traces
20:05:34.466 fetched POST /api/observability/v1/traces 204
20:05:34.509 chunk 6 48662 bytes
20:05:34.510 in Ack 6
20:05:34.907 rpc 31 orchestration.dispatchCommand runtime-request.respond
20:05:35.259 chunk 6 52167 bytes
20:05:35.260 in Ack 6
20:05:35.413 exit 31 {"_tag":"Success","value":{"sequence":97}}
20:05:35.509 chunk 6 54649 bytes
20:05:35.510 in Ack 6
20:05:35.590 fetch POST /api/observability/v1/traces
20:05:35.590 fetched POST /api/observability/v1/traces 204
20:05:35.760 chunk 6 54669 bytes
20:05:35.764 in Ack 6
20:05:36.271 rpc 32 orchestration.dispatchCommand runtime-request.respond
20:05:36.296 chunk 6 54679 bytes
20:05:36.301 in Ack 6
20:05:36.546 chunk 6 54802 bytes
20:05:36.548 in Ack 6
20:05:36.765 fetch POST /api/observability/v1/traces
20:05:36.765 fetched POST /api/observability/v1/traces 204
20:05:36.797 chunk 6 54812 bytes
20:05:36.798 in Ack 6
20:05:37.046 exit 32 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
20:05:37.049 chunk 6 54784 bytes
20:05:37.052 in Ack 6
20:05:37.360 chunk 6 54804 bytes
20:05:37.361 in Ack 6
20:05:37.616 chunk 6 54864 bytes
20:05:37.618 in Ack 6
20:05:37.808 fetch POST /api/observability/v1/traces
20:05:37.808 fetched POST /api/observability/v1/traces 204
20:05:37.868 chunk 6 54944 bytes
20:05:37.868 chunk 12 2892 bytes
20:05:37.870 in Ack 6
20:05:37.884 in Ack 12
20:05:37.885 rpc 33 orchestration.dispatchCommand thread.visit
20:05:37.885 exit 33 {"_tag":"Success","value":{"sequence":0}}
20:05:38.134 chunk 6 54964 bytes
20:05:38.135 in Ack 6
20:05:38.285 in Ping 
20:05:38.397 chunk 6 54974 bytes
20:05:38.399 in Ack 6
20:05:38.780 chunk 6 54994 bytes
20:05:38.781 in Ack 6
20:05:38.870 fetch POST /api/observability/v1/traces
20:05:38.870 fetched POST /api/observability/v1/traces 204
20:05:39.030 chunk 6 55092 bytes
20:05:39.031 in Ack 6
20:05:39.125 chunk 12 2892 bytes
20:05:39.125 in Ack 12
20:05:39.280 chunk 6 55112 bytes
20:05:39.281 in Ack 6
20:05:39.532 chunk 6 58271 bytes
20:05:39.536 in Ack 6
20:05:39.547 in Interrupt 25
20:05:39.577 rpc 34 subscribeVcsStatus
20:05:39.829 rpc 35 server.reportClientActivity
20:05:39.876 chunk 6 58281 bytes
20:05:39.877 in Ack 6
20:05:40.032 fetch POST /api/observability/v1/traces
20:05:40.032 fetched POST /api/observability/v1/traces 204
20:05:40.127 chunk 6 58311 bytes
20:05:40.128 in Ack 6
20:05:40.377 chunk 34 638 bytes
20:05:40.377 in Ack 34
20:05:40.380 chunk 6 58351 bytes
20:05:40.381 in Ack 6
20:05:40.631 chunk 6 59760 bytes
20:05:40.632 in Ack 6
20:05:40.887 chunk 6 60834 bytes
20:05:40.888 in Ack 6
20:05:41.129 fetch POST /api/observability/v1/traces
20:05:41.129 fetched POST /api/observability/v1/traces 204
20:05:41.136 chunk 6 60854 bytes
20:05:41.138 in Ack 6
20:05:41.481 chunk 6 60884 bytes
20:05:41.484 in Ack 6
20:05:41.731 chunk 6 60924 bytes
20:05:41.733 in Ack 6
20:05:42.017 chunk 6 60944 bytes
20:05:42.019 in Ack 6
20:05:42.144 fetch POST /api/observability/v1/traces
20:05:42.144 fetched POST /api/observability/v1/traces 204
20:05:42.364 chunk 6 60954 bytes
20:05:42.365 exit 35 {"_tag":"Success","value":null}
20:05:42.367 in Ack 6
20:05:42.614 chunk 6 60974 bytes
20:05:42.615 in Ack 6
20:05:42.865 chunk 6 61024 bytes
20:05:42.866 in Ack 6
20:05:43.147 chunk 6 61034 bytes
20:05:43.148 in Ack 6
20:05:43.286 in Ping 
20:05:43.368 fetch POST /api/observability/v1/traces
20:05:43.368 fetched POST /api/observability/v1/traces 204
20:05:43.398 chunk 6 61060 bytes
20:05:43.399 in Ack 6
20:05:43.648 chunk 6 62067 bytes
20:05:43.651 in Ack 6
20:05:43.823 chunk 12 2892 bytes
20:05:43.844 in Ack 12
20:05:43.899 chunk 6 62097 bytes
20:05:43.901 in Ack 6
20:05:44.448 fetch POST /api/observability/v1/traces
20:05:44.448 fetched POST /api/observability/v1/traces 204
20:05:44.483 chunk 6 62112 bytes
20:05:44.485 in Ack 6
20:05:44.733 chunk 6 62142 bytes
20:05:44.735 in Ack 6
20:05:44.983 chunk 6 62238 bytes
20:05:44.985 in Ack 6
20:05:45.487 fetch POST /api/observability/v1/traces
20:05:45.488 fetched POST /api/observability/v1/traces 204
20:05:45.743 chunk 6 62253 bytes
20:05:45.744 in Ack 6
20:05:45.993 chunk 6 62334 bytes
20:05:45.994 in Ack 6
20:05:46.243 chunk 6 62409 bytes
20:05:46.245 in Ack 6
20:05:46.494 chunk 6 62430 bytes
20:05:46.498 in Ack 6
20:05:46.745 fetch POST /api/observability/v1/traces
20:05:46.745 fetched POST /api/observability/v1/traces 204
20:05:47.011 chunk 6 62445 bytes
20:05:47.013 in Ack 6
20:05:47.262 chunk 6 62535 bytes
20:05:47.263 in Ack 6
20:05:47.513 chunk 6 62550 bytes
20:05:47.515 in Ack 6
20:05:47.763 chunk 6 62589 bytes
20:05:47.764 in Ack 6
20:05:47.887 rpc 36 orchestration.dispatchCommand thread.visit
20:05:47.887 exit 36 {"_tag":"Success","value":{"sequence":0}}
20:05:47.914 rpc 37 server.reportClientActivity
20:05:48.013 chunk 6 62604 bytes
20:05:48.014 in Ack 6
20:05:48.090 fetch POST /api/observability/v1/traces
20:05:48.090 fetched POST /api/observability/v1/traces 204
20:05:48.263 chunk 6 62619 bytes
20:05:48.264 in Ack 6
20:05:48.322 in Ping 
20:05:48.514 chunk 6 62649 bytes
20:05:48.515 in Ack 6
20:05:48.768 chunk 6 62688 bytes
20:05:48.770 in Ack 6
20:05:49.018 chunk 6 62721 bytes
20:05:49.019 in Ack 6
20:05:49.266 fetch POST /api/observability/v1/traces
20:05:49.266 fetched POST /api/observability/v1/traces 204
20:05:49.340 chunk 6 62736 bytes
20:05:49.342 in Ack 6
20:05:49.605 chunk 6 62835 bytes
20:05:49.607 in Ack 6
20:05:49.856 chunk 6 62886 bytes
20:05:49.865 in Ack 6
20:05:50.151 chunk 6 62907 bytes
20:05:50.152 in Ack 6
20:05:50.343 fetch POST /api/observability/v1/traces
20:05:50.343 fetched POST /api/observability/v1/traces 204
20:05:50.402 chunk 6 62940 bytes
20:05:50.404 in Ack 6
20:05:50.652 chunk 6 62988 bytes
20:05:50.655 in Ack 6
20:05:51.404 fetch POST /api/observability/v1/traces
20:05:51.404 fetched POST /api/observability/v1/traces 204
20:05:51.457 chunk 6 63003 bytes
20:05:51.459 in Ack 6
20:05:51.707 chunk 6 63036 bytes
20:05:51.708 in Ack 6
20:05:51.957 chunk 6 63086 bytes
20:05:51.959 in Ack 6
20:05:52.460 fetch POST /api/observability/v1/traces
20:05:52.460 fetched POST /api/observability/v1/traces 204
