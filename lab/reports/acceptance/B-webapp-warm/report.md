# B, webapp, warm start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.4s |
| Prompt reached the agent | 0.9s |
| Time to first text | 1.8s |
| Whole reply shown | 44.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.9s / 0.9s / 1.7s |
| Longest silence | 2.6s |
| Approval shown after it was asked | 1.0s |
| Bytes on the wire down / up | 553757 / 354595 |
| Bytes the app received / sent | 359058 / 65796 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.7 | 333 | 365 |
| 20 | 0.7 | 734 | 766 |
| 30 | 0.4 | 1077 | 1101 |
| 40 | 1.2 | 1431 | 1477 |
| 44 | 1.4 | 1564 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"860/860 1058/1058 1108/1108 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809","attempts":"662/662 662/662","providerTurns":"778/778 47/778","providerThreads":"810/810","providerSessions":"2688/2688","messages":"550/550 456/456 60/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":248,"lastDataAt":1794159247546,"silences":0,"longestSilenceMs":1150,"longestRecoveredGapMs":1150},"stats":{"sendRate":6750,"deliveredRate":3423.997523692444,"loss":0,"srttMs":39.365395805604095,"minRttMs":1,"sentPackets":116,"sentWireBytes":34277,"receivedPackets":111,"receivedWireBytes":53480,"lastHeardMs":9505,"unsyncedBytes":1001,"inflightPackets":12,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"860/860 1058/1058 1108/1108 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"614/951","assistant":"32/32 21/21 425/425","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":248,"lastDataAt":1794159257750,"silences":0,"longestSilenceMs":1150,"longestRecoveredGapMs":1150},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1115.8813367146881,"minRttMs":992,"sentPackets":177,"sentWireBytes":47142,"receivedPackets":202,"receivedWireBytes":95557,"lastHeardMs":19708,"unsyncedBytes":0,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1108/1108 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 933/933","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 549/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":248,"lastDataAt":1794159267927,"silences":0,"longestSilenceMs":1150,"longestRecoveredGapMs":1150},"stats":{"sendRate":3000,"deliveredRate":1893.8659516197415,"loss":0,"srttMs":1212.152877107689,"minRttMs":956,"sentPackets":248,"sentWireBytes":60777,"receivedPackets":472,"receivedWireBytes":169238,"lastHeardMs":29885,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1108/1108 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1578/1578 956/956 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 1340/1340 83/83","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785 989/989 1401/1401 1025/1025","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":248,"lastDataAt":1794159279047,"silences":0,"longestSilenceMs":1150,"longestRecoveredGapMs":1150},"stats":{"sendRate":3000,"deliveredRate":1755.1401869158879,"loss":0,"srttMs":1320.2378827323118,"minRttMs":956,"sentPackets":312,"sentWireBytes":74074,"receivedPackets":895,"receivedWireBytes":300013,"lastHeardMs":41005,"unsyncedBytes":336,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"860/860 1058/1058 1108/1108 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1578/1578 956/956 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 1340/1340 624/624","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785 989/989 1401/1401 1025/1025","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 550/550 549/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":248,"lastDataAt":1794159289874,"silences":0,"longestSilenceMs":1150,"longestRecoveredGapMs":1150},"stats":{"sendRate":2459.9892338416203,"deliveredRate":2308.6997193638917,"loss":0,"srttMs":1341.7399948917268,"minRttMs":956,"sentPackets":386,"sentWireBytes":92328,"receivedPackets":1100,"receivedWireBytes":385554,"lastHeardMs":51833,"unsyncedBytes":492,"inflightPackets":15,"duplicateBytes":0}}
- shim trace
17:33:58.340 fetch GET /api/auth/session
17:33:58.342 fetched GET /api/auth/session 200
17:33:58.471 fetch GET /.well-known/t3/environment
17:33:58.472 fetched GET /.well-known/t3/environment 200
17:33:58.481 fetch GET /.well-known/t3/environment
17:33:58.493 fetched GET /.well-known/t3/environment 200
17:33:58.497 rpc 0 subscribeServerConfig
17:33:58.498 chunk 0 38690 bytes
17:33:58.526 in Ack 0
17:33:58.528 rpc 1 subscribeServerLifecycle
17:33:58.529 rpc 2 subscribeServerLifecycle
17:33:58.532 fetch GET /api/auth/session
17:33:58.533 fetched GET /api/auth/session 200
17:33:58.534 rpc 3 subscribeDeviceState
17:33:58.535 rpc 4 preview.reportProfiles
17:33:58.560 rpc 5 subscribeProjectClones
17:33:58.576 fetch GET /api/orchestration/threads/mcp%3A3f0d8ed7-2d54-4c70-8bc5-5724c425f71f/bounded
17:33:58.580 fetch GET /api/orchestration/shell
17:33:58.597 rpc 6 server.refreshProviders
17:33:58.635 rpc 7 orchestration.dispatchCommand thread.visit
17:33:58.635 exit 7 {"_tag":"Success","value":{"sequence":0}}
17:33:58.647 fetched GET /api/orchestration/shell 200
17:33:58.647 rpc 8 subscribeTerminalMetadata
17:33:58.648 rpc 9 subscribeVcsStatus
17:33:58.649 rpc 10 subscribePreviewEvents
17:33:58.651 rpc 11 orchestration.subscribeShell
17:33:58.651 chunk 11 2782 bytes
17:33:58.652 rpc 12 assets.createUrl
17:33:58.653 rpc 13 preview.list
17:33:58.653 rpc 14 projects.readFile
17:33:58.655 in Ack 11
17:33:58.901 rpc 15 server.reportClientActivity
17:33:58.960 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"dfd71891-6367-4faa-bcb7-9d8d2d6715b2","sessions":[]}}
17:33:58.960 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:33:59.221 chunk 1 3864 bytes
17:33:59.222 in Ack 1
17:33:59.449 chunk 2 3864 bytes
17:33:59.450 in Ack 2
17:33:59.469 fetch POST /api/observability/v1/traces
17:33:59.469 fetched POST /api/observability/v1/traces 204
17:33:59.659 chunk 11 2758 bytes
17:33:59.659 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:33:46.176Z","configurableProviders":false,"con
17:33:59.659 chunk 9 534 bytes
17:33:59.659 exit 12 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:33:59.660 in Ack 11
17:33:59.666 in Ack 9
17:34:00.158 chunk 3 876 bytes
17:34:00.158 exit 4 {"_tag":"Success","value":null}
17:34:00.158 chunk 5 4 bytes
17:34:00.158 chunk 8 36 bytes
17:34:00.159 in Ack 3
17:34:00.162 rpc 16 preview.reportProfiles
17:34:00.163 in Ack 5
17:34:00.163 in Ack 8
17:34:00.166 chunk 11 2801 bytes
17:34:00.166 in Ack 11
17:34:00.248 exit 15 {"_tag":"Success","value":null}
17:34:00.248 exit 16 {"_tag":"Success","value":null}
17:34:00.589 fetched GET /api/orchestration/threads/mcp%3A3f0d8ed7-2d54-4c70-8bc5-5724c425f71f/bounded 200
17:34:00.592 rpc 17 orchestration.subscribeThread
17:34:00.593 chunk 17 8305 bytes
17:34:00.602 in Interrupt 9
17:34:00.604 rpc 18 subscribeVcsStatus
17:34:00.605 in Ack 17
17:34:00.662 fetch POST /api/observability/v1/traces
17:34:00.662 fetched POST /api/observability/v1/traces 204
17:34:00.768 chunk 18 534 bytes
17:34:00.768 in Ack 18
17:34:00.843 chunk 17 8345 bytes
17:34:00.844 in Ack 17
17:34:00.867 rpc 19 server.reportClientActivity
17:34:01.095 chunk 17 11944 bytes
17:34:01.097 in Ack 17
17:34:01.351 chunk 17 12722 bytes
17:34:01.352 in Ack 17
17:34:01.399 rpc 20 attachments.createUploadUrl
17:34:01.604 chunk 17 13532 bytes
17:34:01.605 in Ack 17
17:34:01.612 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-a036a905-3844-40e4-b71a-ed28c3eb5119","expiresAt":1794159841588,"relativeUrl":"/api/attachments/upload/eyJ2ZX
17:34:01.613 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWEwMzZhOTA1LTM4NDQtNDBlNC1iNzFhLWVkMjhjM2ViNTExOSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1OTg0MTU4OH0.1LwuBC1AoYHU2G8VMWj6t0m1YqD7DBinM2PPC2I4A_M
17:34:01.845 fetch POST /api/observability/v1/traces
17:34:01.845 fetched POST /api/observability/v1/traces 204
17:34:02.104 chunk 17 18055 bytes
17:34:02.105 in Ack 17
17:34:02.353 chunk 17 19883 bytes
17:34:02.354 in Ack 17
17:34:02.361 in Interrupt 18
17:34:02.362 rpc 21 subscribeVcsStatus
17:34:02.409 chunk 21 534 bytes
17:34:02.409 in Ack 21
17:34:02.577 exit 19 {"_tag":"Success","value":null}
17:34:02.604 chunk 17 21223 bytes
17:34:02.605 in Ack 17
17:34:02.829 rpc 22 server.reportClientActivity
17:34:03.106 fetch POST /api/observability/v1/traces
17:34:03.106 fetched POST /api/observability/v1/traces 204
17:34:03.407 exit 22 {"_tag":"Success","value":null}
17:34:03.496 in Ping 
17:34:04.202 rpc 23 orchestration.dispatchCommand message.dispatch
17:34:04.738 fetch POST /api/observability/v1/traces
17:34:04.738 fetched POST /api/observability/v1/traces 204
17:34:04.789 exit 23 {"_tag":"Success","value":{"sequence":45}}
17:34:04.790 rpc 24 attachments.delete
17:34:05.258 exit 24 {"_tag":"Success","value":null}
17:34:05.289 chunk 11 2887 bytes
17:34:05.289 chunk 17 23651 bytes
17:34:05.290 in Ack 11
17:34:05.344 in Ack 17
17:34:05.539 chunk 17 23663 bytes
17:34:05.540 in Ack 17
17:34:05.790 fetch POST /api/observability/v1/traces
17:34:05.790 fetched POST /api/observability/v1/traces 204
17:34:05.790 chunk 17 23713 bytes
17:34:05.791 in Ack 17
17:34:06.038 chunk 11 2823 bytes
17:34:06.039 in Ack 11
17:34:06.040 rpc 25 orchestration.dispatchCommand thread.visit
17:34:06.040 exit 25 {"_tag":"Success","value":{"sequence":0}}
17:34:06.044 chunk 17 27478 bytes
17:34:06.045 in Ack 17
17:34:06.047 rpc 26 assets.createUrl
17:34:06.288 chunk 11 2823 bytes
17:34:06.289 in Ack 11
17:34:06.295 chunk 17 28141 bytes
17:34:06.296 in Ack 17
17:34:06.546 chunk 17 30564 bytes
17:34:06.547 in Ack 17
17:34:07.027 fetch POST /api/observability/v1/traces
17:34:07.027 fetched POST /api/observability/v1/traces 204
17:34:07.028 chunk 17 31988 bytes
17:34:07.029 in Ack 17
17:34:07.036 in Interrupt 21
17:34:07.038 rpc 27 subscribeVcsStatus
17:34:07.283 chunk 17 31988 bytes
17:34:07.284 in Ack 17
17:34:07.298 rpc 28 server.reportClientActivity
17:34:07.422 chunk 27 534 bytes
17:34:07.427 in Ack 27
17:34:07.702 rpc 29 orchestration.dispatchCommand message.dispatch
17:34:07.801 chunk 17 31988 bytes
17:34:07.805 in Ack 17
17:34:08.030 fetch POST /api/observability/v1/traces
17:34:08.030 fetched POST /api/observability/v1/traces 204
17:34:08.386 chunk 17 32560 bytes
17:34:08.387 in Ack 17
17:34:08.496 in Ping 
17:34:08.772 exit 28 {"_tag":"Success","value":null}
17:34:08.886 chunk 17 33586 bytes
17:34:08.887 in Ack 17
17:34:08.931 exit 29 {"_tag":"Success","value":{"sequence":75}}
17:34:09.387 chunk 17 36052 bytes
17:34:09.388 fetch POST /api/observability/v1/traces
17:34:09.388 fetched POST /api/observability/v1/traces 204
17:34:09.389 in Ack 17
17:34:09.637 chunk 17 36024 bytes
17:34:09.638 in Ack 17
17:34:09.887 chunk 17 36034 bytes
17:34:09.888 in Ack 17
17:34:10.136 chunk 17 36084 bytes
17:34:10.137 in Ack 17
17:34:10.387 chunk 17 36114 bytes
17:34:10.388 in Ack 17
17:34:10.399 fetch POST /api/observability/v1/traces
17:34:10.399 fetched POST /api/observability/v1/traces 204
17:34:10.637 chunk 17 36134 bytes
17:34:10.638 in Ack 17
17:34:10.886 chunk 11 2887 bytes
17:34:10.887 chunk 17 36154 bytes
17:34:10.887 in Ack 11
17:34:10.927 in Ack 17
17:34:11.137 chunk 17 36164 bytes
17:34:11.138 in Ack 17
17:34:11.386 chunk 17 36843 bytes
17:34:11.388 in Ack 17
17:34:11.637 chunk 17 36853 bytes
17:34:11.637 in Ack 17
17:34:11.645 fetch POST /api/observability/v1/traces
17:34:11.646 fetched POST /api/observability/v1/traces 204
17:34:11.886 chunk 17 37713 bytes
17:34:11.887 in Ack 17
17:34:12.137 chunk 17 40417 bytes
17:34:12.139 in Ack 17
17:34:12.388 chunk 17 40447 bytes
17:34:12.389 in Ack 17
17:34:12.639 chunk 17 40457 bytes
17:34:12.640 in Ack 17
17:34:12.888 fetch POST /api/observability/v1/traces
17:34:12.888 fetched POST /api/observability/v1/traces 204
17:34:12.889 chunk 17 40487 bytes
17:34:12.890 in Ack 17
17:34:13.138 chunk 17 40537 bytes
17:34:13.140 in Ack 17
17:34:13.497 in Ping 
17:34:13.640 chunk 17 41363 bytes
17:34:13.641 in Ack 17
17:34:13.890 chunk 17 41385 bytes
17:34:13.891 in Ack 17
17:34:13.899 fetch POST /api/observability/v1/traces
17:34:13.899 fetched POST /api/observability/v1/traces 204
17:34:14.140 chunk 17 41395 bytes
17:34:14.141 in Ack 17
17:34:14.390 chunk 17 41951 bytes
17:34:14.391 in Ack 17
17:34:14.641 chunk 17 41961 bytes
17:34:14.642 in Ack 17
17:34:14.891 chunk 17 41981 bytes
17:34:14.892 in Ack 17
17:34:15.141 chunk 17 43711 bytes
17:34:15.142 fetch POST /api/observability/v1/traces
17:34:15.142 fetched POST /api/observability/v1/traces 204
17:34:15.143 in Ack 17
17:34:15.392 chunk 17 43741 bytes
17:34:15.393 in Ack 17
17:34:15.641 chunk 17 43761 bytes
17:34:15.642 in Ack 17
17:34:15.892 chunk 17 43781 bytes
17:34:15.893 in Ack 17
17:34:16.040 rpc 30 orchestration.dispatchCommand thread.visit
17:34:16.040 exit 30 {"_tag":"Success","value":{"sequence":0}}
17:34:16.143 chunk 17 43821 bytes
17:34:16.144 fetch POST /api/observability/v1/traces
17:34:16.144 fetched POST /api/observability/v1/traces 204
17:34:16.145 in Ack 17
17:34:16.461 chunk 17 43841 bytes
17:34:16.462 in Ack 17
17:34:16.712 chunk 17 43851 bytes
17:34:16.714 in Ack 17
17:34:16.963 chunk 17 43891 bytes
17:34:16.963 in Ack 17
17:34:17.146 fetch POST /api/observability/v1/traces
17:34:17.146 fetched POST /api/observability/v1/traces 204
17:34:17.213 chunk 17 43921 bytes
17:34:17.214 in Ack 17
17:34:17.464 chunk 17 43951 bytes
17:34:17.465 in Ack 17
17:34:17.714 chunk 17 43961 bytes
17:34:17.715 in Ack 17
17:34:17.965 chunk 17 43991 bytes
17:34:17.966 in Ack 17
17:34:18.215 fetch POST /api/observability/v1/traces
17:34:18.215 fetched POST /api/observability/v1/traces 204
17:34:18.215 chunk 11 2887 bytes
17:34:18.215 chunk 17 44021 bytes
17:34:18.216 in Ack 11
17:34:18.221 in Ack 17
17:34:18.467 chunk 17 44031 bytes
17:34:18.468 in Ack 17
17:34:18.497 in Ping 
17:34:18.719 chunk 17 44071 bytes
17:34:18.720 in Ack 17
17:34:18.969 chunk 17 44111 bytes
17:34:18.971 in Ack 17
17:34:19.217 fetch POST /api/observability/v1/traces
17:34:19.217 fetched POST /api/observability/v1/traces 204
17:34:19.219 chunk 17 44121 bytes
17:34:19.229 in Ack 17
17:34:19.469 chunk 17 44141 bytes
17:34:19.470 in Ack 17
17:34:19.719 chunk 11 2887 bytes
17:34:19.720 chunk 17 44181 bytes
17:34:19.720 in Ack 11
17:34:19.722 in Ack 17
17:34:19.969 chunk 17 44201 bytes
17:34:19.970 in Ack 17
17:34:20.220 chunk 17 45321 bytes
17:34:20.221 in Ack 17
17:34:20.230 fetch POST /api/observability/v1/traces
17:34:20.230 fetched POST /api/observability/v1/traces 204
17:34:20.470 chunk 17 45366 bytes
17:34:20.471 in Ack 17
17:34:20.605 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/ed7c07b2-96c2-4dea-b31f-28cf025c992e","expiresAt":1794162860605}}
17:34:20.721 chunk 17 45396 bytes
17:34:20.722 in Ack 17
17:34:20.971 chunk 17 45441 bytes
17:34:20.972 in Ack 17
17:34:21.221 chunk 17 45486 bytes
17:34:21.222 in Ack 17
17:34:21.471 chunk 17 45501 bytes
17:34:21.472 in Ack 17
17:34:21.481 fetch POST /api/observability/v1/traces
17:34:21.481 fetched POST /api/observability/v1/traces 204
17:34:21.721 chunk 17 45546 bytes
17:34:21.723 in Ack 17
17:34:21.972 chunk 17 45576 bytes
17:34:21.973 in Ack 17
17:34:22.222 chunk 17 45606 bytes
17:34:22.224 in Ack 17
17:34:22.473 chunk 17 45666 bytes
17:34:22.474 in Ack 17
17:34:22.723 chunk 17 45681 bytes
17:34:22.724 fetch POST /api/observability/v1/traces
17:34:22.724 fetched POST /api/observability/v1/traces 204
17:34:22.725 in Ack 17
17:34:22.973 chunk 17 45741 bytes
17:34:22.974 in Ack 17
17:34:23.222 chunk 17 45756 bytes
17:34:23.223 in Ack 17
17:34:23.475 chunk 17 45816 bytes
17:34:23.476 in Ack 17
17:34:23.509 in Ping 
17:34:23.729 rpc 31 server.reportClientActivity
17:34:23.730 chunk 17 45831 bytes
17:34:23.730 fetch POST /api/observability/v1/traces
17:34:23.730 fetched POST /api/observability/v1/traces 204
17:34:23.731 in Ack 17
17:34:23.980 chunk 17 45891 bytes
17:34:23.981 in Ack 17
17:34:24.231 chunk 17 45921 bytes
17:34:24.232 in Ack 17
17:34:24.480 chunk 17 45966 bytes
17:34:24.481 in Ack 17
17:34:24.730 chunk 17 45996 bytes
17:34:24.731 in Ack 17
17:34:24.740 fetch POST /api/observability/v1/traces
17:34:24.740 fetched POST /api/observability/v1/traces 204
17:34:24.980 chunk 17 46026 bytes
17:34:24.981 in Ack 17
17:34:25.130 exit 31 {"_tag":"Success","value":null}
17:34:25.238 chunk 17 46071 bytes
17:34:25.239 in Ack 17
17:34:25.488 chunk 17 46101 bytes
17:34:25.489 in Ack 17
17:34:25.739 chunk 17 46161 bytes
17:34:25.740 in Ack 17
17:34:25.983 fetch POST /api/observability/v1/traces
17:34:25.983 fetched POST /api/observability/v1/traces 204
17:34:25.989 chunk 17 46176 bytes
17:34:25.990 in Ack 17
17:34:26.041 rpc 32 orchestration.dispatchCommand thread.visit
17:34:26.041 exit 32 {"_tag":"Success","value":{"sequence":0}}
17:34:26.240 chunk 17 46221 bytes
17:34:26.241 in Ack 17
17:34:26.489 chunk 17 46251 bytes
17:34:26.490 in Ack 17
17:34:26.740 chunk 17 46296 bytes
17:34:26.741 in Ack 17
17:34:26.990 chunk 17 46326 bytes
17:34:26.991 in Ack 17
17:34:27.000 fetch POST /api/observability/v1/traces
17:34:27.000 fetched POST /api/observability/v1/traces 204
17:34:27.240 chunk 17 46356 bytes
17:34:27.242 in Ack 17
17:34:27.491 chunk 11 2887 bytes
17:34:27.491 chunk 17 46416 bytes
17:34:27.492 in Ack 11
17:34:27.497 in Ack 17
17:34:27.880 chunk 17 46431 bytes
17:34:27.882 in Ack 17
17:34:28.131 chunk 17 46491 bytes
17:34:28.132 in Ack 17
17:34:28.242 fetch POST /api/observability/v1/traces
17:34:28.242 fetched POST /api/observability/v1/traces 204
17:34:28.380 chunk 17 46551 bytes
17:34:28.381 in Ack 17
17:34:28.509 in Ping 
17:34:28.630 chunk 17 46566 bytes
17:34:28.631 in Ack 17
17:34:28.880 chunk 17 46611 bytes
17:34:28.881 in Ack 17
17:34:29.131 chunk 17 46626 bytes
17:34:29.132 in Ack 17
17:34:29.381 chunk 17 46701 bytes
17:34:29.382 in Ack 17
17:34:29.391 fetch POST /api/observability/v1/traces
17:34:29.391 fetched POST /api/observability/v1/traces 204
17:34:29.630 chunk 17 46716 bytes
17:34:29.631 in Ack 17
17:34:29.881 chunk 17 46761 bytes
17:34:29.882 in Ack 17
17:34:30.131 chunk 17 46791 bytes
17:34:30.132 in Ack 17
17:34:30.381 chunk 17 46836 bytes
17:34:30.382 in Ack 17
17:34:30.631 chunk 17 46866 bytes
17:34:30.632 in Ack 17
17:34:30.640 fetch POST /api/observability/v1/traces
17:34:30.640 fetched POST /api/observability/v1/traces 204
17:34:30.881 chunk 17 46911 bytes
17:34:30.882 in Ack 17
17:34:31.131 chunk 17 46971 bytes
17:34:31.132 in Ack 17
17:34:31.383 chunk 17 46986 bytes
17:34:31.384 in Ack 17
17:34:31.634 chunk 17 47001 bytes
17:34:31.635 in Ack 17
17:34:31.883 fetch POST /api/observability/v1/traces
17:34:31.883 fetched POST /api/observability/v1/traces 204
17:34:31.884 chunk 17 47076 bytes
17:34:31.885 in Ack 17
17:34:32.135 chunk 17 47106 bytes
17:34:32.136 in Ack 17
17:34:32.392 chunk 17 47121 bytes
17:34:32.393 in Ack 17
17:34:32.643 chunk 17 47166 bytes
17:34:32.644 in Ack 17
17:34:32.886 fetch POST /api/observability/v1/traces
17:34:32.886 fetched POST /api/observability/v1/traces 204
17:34:32.892 chunk 17 47196 bytes
17:34:32.893 in Ack 17
17:34:33.143 chunk 17 47226 bytes
17:34:33.144 in Ack 17
17:34:33.393 chunk 17 47271 bytes
17:34:33.394 in Ack 17
17:34:33.510 in Ping 
17:34:33.644 chunk 17 47331 bytes
17:34:33.645 in Ack 17
17:34:33.894 fetch POST /api/observability/v1/traces
17:34:33.894 fetched POST /api/observability/v1/traces 204
17:34:33.895 chunk 17 47376 bytes
17:34:33.896 in Ack 17
17:34:34.158 chunk 17 47391 bytes
17:34:34.159 in Ack 17
17:34:34.408 chunk 17 47436 bytes
17:34:34.409 in Ack 17
17:34:34.658 chunk 17 47451 bytes
17:34:34.659 in Ack 17
17:34:34.897 fetch POST /api/observability/v1/traces
17:34:34.897 fetched POST /api/observability/v1/traces 204
17:34:34.908 chunk 17 47511 bytes
17:34:34.909 in Ack 17
17:34:35.159 chunk 17 47556 bytes
17:34:35.160 in Ack 17
17:34:35.409 chunk 17 47601 bytes
17:34:35.410 in Ack 17
17:34:35.659 chunk 17 47631 bytes
17:34:35.660 in Ack 17
17:34:35.909 chunk 17 47676 bytes
17:34:35.910 in Ack 17
17:34:35.929 fetch POST /api/observability/v1/traces
17:34:35.929 fetched POST /api/observability/v1/traces 204
17:34:36.161 chunk 17 48714 bytes
17:34:36.162 in Ack 17
17:34:36.393 rpc 33 orchestration.dispatchCommand runtime-request.respond
17:34:36.443 chunk 17 54318 bytes
17:34:36.447 in Ack 17
17:34:36.693 chunk 17 57689 bytes
17:34:36.694 in Ack 17
17:34:37.164 fetch POST /api/observability/v1/traces
17:34:37.164 fetched POST /api/observability/v1/traces 204
17:34:37.633 chunk 11 3197 bytes
17:34:37.634 chunk 17 57739 bytes
17:34:37.634 in Ack 11
17:34:37.640 in Ack 17
17:34:37.757 exit 33 {"_tag":"Success","value":{"sequence":97}}
17:34:37.757 rpc 34 orchestration.dispatchCommand thread.visit
17:34:37.757 exit 34 {"_tag":"Success","value":{"sequence":0}}
17:34:37.884 chunk 17 60232 bytes
17:34:37.885 in Ack 17
17:34:38.135 chunk 11 2889 bytes
17:34:38.135 chunk 17 60267 bytes
17:34:38.135 in Ack 11
17:34:38.142 in Ack 17
17:34:38.386 chunk 17 60277 bytes
17:34:38.387 in Ack 17
17:34:38.510 in Ping 
17:34:38.636 fetch POST /api/observability/v1/traces
17:34:38.636 fetched POST /api/observability/v1/traces 204
17:34:38.637 chunk 17 60297 bytes
17:34:38.638 in Ack 17
17:34:38.887 chunk 17 60447 bytes
17:34:38.888 in Ack 17
17:34:38.896 in Interrupt 27
17:34:38.920 rpc 35 subscribeVcsStatus
17:34:39.137 chunk 17 60501 bytes
17:34:39.138 in Ack 17
17:34:39.171 rpc 36 server.reportClientActivity
17:34:39.385 chunk 11 2889 bytes
17:34:39.386 in Ack 11
17:34:39.390 chunk 17 60521 bytes
17:34:39.391 in Ack 17
17:34:39.638 fetch POST /api/observability/v1/traces
17:34:39.638 fetched POST /api/observability/v1/traces 204
17:34:39.640 chunk 17 60551 bytes
17:34:39.642 in Ack 17
17:34:39.890 chunk 17 60581 bytes
17:34:39.891 in Ack 17
17:34:40.141 chunk 17 60611 bytes
17:34:40.142 in Ack 17
17:34:40.390 chunk 17 60631 bytes
17:34:40.391 in Ack 17
17:34:40.400 chunk 35 534 bytes
17:34:40.400 in Ack 35
17:34:40.641 chunk 17 60651 bytes
17:34:40.642 in Ack 17
17:34:40.650 fetch POST /api/observability/v1/traces
17:34:40.650 fetched POST /api/observability/v1/traces 204
17:34:40.892 chunk 17 60681 bytes
17:34:40.893 in Ack 17
17:34:41.143 chunk 17 60701 bytes
17:34:41.144 in Ack 17
17:34:41.393 chunk 17 60711 bytes
17:34:41.394 in Ack 17
17:34:41.643 chunk 17 60751 bytes
17:34:41.644 in Ack 17
17:34:41.741 exit 36 {"_tag":"Success","value":null}
17:34:41.893 chunk 17 60791 bytes
17:34:41.894 in Ack 17
17:34:41.921 fetch POST /api/observability/v1/traces
17:34:41.921 fetched POST /api/observability/v1/traces 204
17:34:42.154 chunk 17 60811 bytes
17:34:42.155 in Ack 17
17:34:42.405 chunk 17 60831 bytes
17:34:42.406 in Ack 17
17:34:42.654 chunk 17 60861 bytes
17:34:42.656 in Ack 17
17:34:42.905 chunk 17 60871 bytes
17:34:42.907 in Ack 17
17:34:43.157 fetch POST /api/observability/v1/traces
17:34:43.157 fetched POST /api/observability/v1/traces 204
17:34:43.158 chunk 17 60901 bytes
17:34:43.159 in Ack 17
17:34:43.407 chunk 17 60941 bytes
17:34:43.409 in Ack 17
17:34:43.510 in Ping 
17:34:43.658 chunk 17 60951 bytes
17:34:43.659 in Ack 17
17:34:43.908 chunk 17 60981 bytes
17:34:43.909 in Ack 17
17:34:44.158 chunk 17 61001 bytes
17:34:44.159 in Ack 17
17:34:44.167 fetch POST /api/observability/v1/traces
17:34:44.167 fetched POST /api/observability/v1/traces 204
17:34:44.414 chunk 17 61031 bytes
17:34:44.415 in Ack 17
17:34:44.664 chunk 17 61041 bytes
17:34:44.665 in Ack 17
17:34:44.914 chunk 17 61081 bytes
17:34:44.915 in Ack 17
17:34:45.164 chunk 17 61101 bytes
17:34:45.165 in Ack 17
17:34:45.415 chunk 17 61131 bytes
17:34:45.416 in Ack 17
17:34:45.425 fetch POST /api/observability/v1/traces
17:34:45.425 fetched POST /api/observability/v1/traces 204
17:34:45.665 chunk 17 61143 bytes
17:34:45.666 in Ack 17
17:34:45.907 chunk 11 2889 bytes
17:34:45.907 in Ack 11
17:34:45.915 chunk 17 62156 bytes
17:34:45.915 in Ack 17
17:34:46.165 chunk 17 62186 bytes
17:34:46.166 in Ack 17
17:34:46.415 chunk 17 62216 bytes
17:34:46.418 in Ack 17
17:34:46.664 chunk 17 62261 bytes
17:34:46.666 in Ack 17
17:34:46.705 fetch POST /api/observability/v1/traces
17:34:46.705 fetched POST /api/observability/v1/traces 204
17:34:46.915 chunk 17 62291 bytes
17:34:46.916 in Ack 17
17:34:47.165 chunk 17 62342 bytes
17:34:47.166 in Ack 17
17:34:47.415 chunk 17 62387 bytes
17:34:47.417 in Ack 17
17:34:47.644 rpc 37 orchestration.dispatchCommand thread.visit
17:34:47.644 exit 37 {"_tag":"Success","value":{"sequence":0}}
17:34:47.665 chunk 17 62408 bytes
17:34:47.667 in Ack 17
17:34:47.916 chunk 17 62453 bytes
17:34:47.917 in Ack 17
17:34:47.961 fetch POST /api/observability/v1/traces
17:34:47.961 fetched POST /api/observability/v1/traces 204
17:34:48.166 chunk 17 62519 bytes
17:34:48.167 in Ack 17
17:34:48.415 chunk 17 62549 bytes
17:34:48.417 in Ack 17
17:34:48.512 in Ping 
17:34:48.666 chunk 17 62579 bytes
17:34:48.667 in Ack 17
17:34:48.740 rpc 38 server.reportClientActivity
17:34:48.916 chunk 17 62609 bytes
17:34:48.917 in Ack 17
17:34:49.160 chunk 11 2889 bytes
17:34:49.161 in Ack 11
17:34:49.167 chunk 17 62678 bytes
17:34:49.167 fetch POST /api/observability/v1/traces
17:34:49.168 fetched POST /api/observability/v1/traces 204
17:34:49.169 in Ack 17
17:34:49.418 chunk 17 62708 bytes
17:34:49.419 in Ack 17
17:34:49.668 chunk 17 62753 bytes
17:34:49.669 in Ack 17
17:34:49.918 chunk 17 62795 bytes
17:34:49.920 in Ack 17
17:34:50.169 chunk 17 62849 bytes
17:34:50.169 fetch POST /api/observability/v1/traces
17:34:50.169 fetched POST /api/observability/v1/traces 204
17:34:50.172 in Ack 17
17:34:50.420 chunk 17 62864 bytes
17:34:50.421 in Ack 17
17:34:50.670 chunk 17 62909 bytes
17:34:50.671 in Ack 17
17:34:50.744 exit 38 {"_tag":"Success","value":null}
17:34:50.920 chunk 17 62960 bytes
17:34:50.921 in Ack 17
17:34:51.170 chunk 17 63014 bytes
17:34:51.172 in Ack 17
17:34:51.248 fetch POST /api/observability/v1/traces
17:34:51.248 fetched POST /api/observability/v1/traces 204
17:34:51.420 chunk 17 63044 bytes
17:34:51.421 in Ack 17
17:34:51.670 chunk 17 63077 bytes
17:34:51.672 in Ack 17
17:34:51.921 chunk 17 63175 bytes
17:34:51.922 in Ack 17
17:34:52.171 chunk 17 63307 bytes
17:34:52.173 in Ack 17
17:34:52.434 chunk 11 2825 bytes
17:34:52.434 chunk 17 66559 bytes
17:34:52.434 fetch POST /api/observability/v1/traces
17:34:52.434 fetched POST /api/observability/v1/traces 204
