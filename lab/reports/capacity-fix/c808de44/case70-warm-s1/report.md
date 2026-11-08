# case70, webapp, warm start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 76 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 14.0s |
| Prompt reached the agent | 12.9s |
| Time to first text | 18.2s |
| Whole reply shown | 74.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 12.8s / 3.0s / 9.8s |
| Longest silence | 18.2s |
| Approval shown after it was asked | 10.7s |
| Bytes on the wire down / up | 63211 / 42503 |
| Bytes the app received / sent | 45975 / 109059 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 1.0 | 243 | 287 |
| 30 | 6.6 | 414 | 685 |
| 40 | 3.1 | 962 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 7.8 | 1077 | 1379 |
| 70 | 8.8 | 1424 | 1596 |
| 74 | 12.8 | 1424 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172423241,"silences":0,"longestSilenceMs":1305,"longestRecoveredGapMs":1305},"stats":{"sendRate":5044.387123392222,"deliveredRate":2842.6496490582126,"loss":0,"srttMs":4.2778176700891635,"minRttMs":1,"sentPackets":95,"sentWireBytes":29157,"receivedPackets":125,"receivedWireBytes":52221,"lastHeardMs":9642,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172427379,"silences":0,"longestSilenceMs":1305,"longestRecoveredGapMs":1305},"stats":{"sendRate":1261.0967808480555,"deliveredRate":0,"loss":0,"srttMs":4.2778176700891635,"minRttMs":null,"sentPackets":114,"sentWireBytes":37351,"receivedPackets":130,"receivedWireBytes":54632,"lastHeardMs":13780,"unsyncedBytes":509,"inflightPackets":22,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 373/1078","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 300/300","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172443674,"silences":1,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":2956.117021276596,"deliveredRate":303.19148936170217,"loss":0.8461538461538461,"srttMs":941.382607470129,"minRttMs":1229,"sentPackets":165,"sentWireBytes":51876,"receivedPackets":137,"receivedWireBytes":57770,"lastHeardMs":30074,"unsyncedBytes":106,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 1078/1078 836/849","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 713/788","user":"10/10 9/9","attempts":"661/661 661/661 99/637","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172454309,"silences":1,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":2956.117021276596,"deliveredRate":315.5250486437151,"loss":0.7568316946211682,"srttMs":1043.6215514887776,"minRttMs":1229,"sentPackets":224,"sentWireBytes":61903,"receivedPackets":162,"receivedWireBytes":66814,"lastHeardMs":40710,"unsyncedBytes":0,"inflightPackets":70,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 1078/1078 849/849","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1293/1303","user":"10/10 9/9 63/64","attempts":"661/661 661/661 412/637","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172464334,"silences":1,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.7276794269160966,"srttMs":1043.6215514887776,"minRttMs":1229,"sentPackets":279,"sentWireBytes":70563,"receivedPackets":188,"receivedWireBytes":74379,"lastHeardMs":50735,"unsyncedBytes":489,"inflightPackets":76,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 1078/1078 849/849 652/1550 53/929","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 63/64","attempts":"661/661 661/661 412/637","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172474340,"silences":1,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":3000,"deliveredRate":560.171130952381,"loss":0.7518518518518518,"srttMs":2388.654006283793,"minRttMs":1092,"sentPackets":380,"sentWireBytes":98272,"receivedPackets":214,"receivedWireBytes":84408,"lastHeardMs":60741,"unsyncedBytes":400,"inflightPackets":163,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 68/1103 849/849 1319/1550 103/929","thread":"951/951","runs":"1014/1014 971/971 85/839","assistant":"32/32 21/21 1340/1340 306/353","user":"10/10 9/9 63/64","attempts":"661/661 661/661 603/637","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172485645,"silences":1,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":1693.054537579957,"deliveredRate":570.1749918820875,"loss":0.7254662309368191,"srttMs":4865.165854087898,"minRttMs":1092,"sentPackets":429,"sentWireBytes":116765,"receivedPackets":241,"receivedWireBytes":95057,"lastHeardMs":72046,"unsyncedBytes":0,"inflightPackets":175,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1058/1058 1103/1103 860/860 1299/1299 1103/1103 269/1058 68/1103 849/849 1319/1550 103/929","thread":"951/951","runs":"1014/1014 971/971 85/839","assistant":"32/32 21/21 1340/1340 645/723","user":"10/10 9/9 63/64","attempts":"661/661 661/661 603/637","messages":"456/456 548/548 16/901","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809","checkpoints":"779/779 613/613","checkpointScopes":"723/723","runtimeRequests":"258/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":232,"lastDataAt":1794172492245,"silences":3,"longestSilenceMs":10456,"longestRecoveredGapMs":10456},"stats":{"sendRate":1904.6863547774515,"deliveredRate":275,"loss":0.8333333333333334,"srttMs":3837.351343656541,"minRttMs":1090,"sentPackets":489,"sentWireBytes":131661,"receivedPackets":246,"receivedWireBytes":96826,"lastHeardMs":78645,"unsyncedBytes":308,"inflightPackets":204,"duplicateBytes":0}}
- shim trace
21:13:33.902 fetch GET /api/auth/session
21:13:33.904 fetched GET /api/auth/session 200
21:13:34.029 fetch GET /.well-known/t3/environment
21:13:34.030 fetched GET /.well-known/t3/environment 200
21:13:34.058 fetch GET /.well-known/t3/environment
21:13:34.059 fetched GET /.well-known/t3/environment 200
21:13:34.063 rpc 0 subscribeServerConfig
21:13:34.063 chunk 0 38678 bytes
21:13:34.081 in Ack 0
21:13:34.083 rpc 1 subscribeServerLifecycle
21:13:34.084 rpc 2 subscribeServerLifecycle
21:13:34.087 fetch GET /api/auth/session
21:13:34.088 fetched GET /api/auth/session 200
21:13:34.088 rpc 3 subscribeDeviceState
21:13:34.091 rpc 4 preview.reportProfiles
21:13:34.134 rpc 5 subscribeProjectClones
21:13:34.152 fetch GET /api/orchestration/threads/mcp%3Af26ebb81-d3b4-44ad-971d-a279da1a6b85/bounded
21:13:34.160 fetch GET /api/orchestration/shell
21:13:34.161 fetched GET /api/orchestration/shell 200
21:13:34.164 rpc 6 orchestration.subscribeShell
21:13:34.164 chunk 6 2779 bytes
21:13:34.193 rpc 7 server.refreshProviders
21:13:34.237 rpc 8 orchestration.dispatchCommand thread.visit
21:13:34.237 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:13:34.254 rpc 9 subscribeTerminalMetadata
21:13:34.256 rpc 10 subscribeVcsStatus
21:13:34.256 rpc 11 subscribePreviewEvents
21:13:34.258 rpc 12 assets.createUrl
21:13:34.258 rpc 13 preview.list
21:13:34.259 rpc 14 projects.readFile
21:13:34.268 in Ack 6
21:13:34.411 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"b7720dc6-118b-4b99-8921-3ff9ae79d8f8","sessions":[]}}
21:13:34.411 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:13:34.415 chunk 6 2755 bytes
21:13:34.416 in Ack 6
21:13:34.509 rpc 15 server.reportClientActivity
21:13:34.549 chunk 1 3861 bytes
21:13:34.550 in Ack 1
21:13:34.919 chunk 2 3861 bytes
21:13:34.919 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:13:19.577Z","configurableProviders":false,"con
21:13:34.919 chunk 10 472 bytes
21:13:34.920 in Ack 2
21:13:34.923 in Ack 10
21:13:35.028 fetch POST /api/observability/v1/traces
21:13:35.028 fetched POST /api/observability/v1/traces 204
21:13:35.032 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:13:35.228 chunk 3 876 bytes
21:13:35.228 exit 4 {"_tag":"Success","value":null}
21:13:35.228 chunk 5 4 bytes
21:13:35.228 chunk 9 36 bytes
21:13:35.228 exit 15 {"_tag":"Success","value":null}
21:13:35.229 in Ack 3
21:13:35.232 rpc 16 preview.reportProfiles
21:13:35.233 in Ack 5
21:13:35.233 in Ack 9
21:13:35.282 chunk 6 2798 bytes
21:13:35.282 in Ack 6
21:13:35.298 exit 16 {"_tag":"Success","value":null}
21:13:35.388 fetched GET /api/orchestration/threads/mcp%3Af26ebb81-d3b4-44ad-971d-a279da1a6b85/bounded 200
21:13:35.392 rpc 17 orchestration.subscribeThread
21:13:35.392 chunk 17 8294 bytes
21:13:35.402 in Interrupt 10
21:13:35.404 rpc 18 subscribeVcsStatus
21:13:35.450 in Ack 17
21:13:35.528 chunk 18 472 bytes
21:13:35.528 in Ack 18
21:13:35.643 chunk 17 11932 bytes
21:13:35.644 in Ack 17
21:13:35.720 rpc 19 server.reportClientActivity
21:13:35.895 chunk 17 13517 bytes
21:13:35.896 in Ack 17
21:13:36.033 fetch POST /api/observability/v1/traces
21:13:36.033 fetched POST /api/observability/v1/traces 204
21:13:36.129 exit 19 {"_tag":"Success","value":null}
21:13:36.146 chunk 17 19081 bytes
21:13:36.147 in Ack 17
21:13:36.473 chunk 17 21197 bytes
21:13:36.474 in Ack 17
21:13:36.481 in Interrupt 18
21:13:36.483 rpc 20 subscribeVcsStatus
21:13:36.525 rpc 21 attachments.createUploadUrl
21:13:36.737 chunk 20 472 bytes
21:13:36.752 rpc 22 server.reportClientActivity
21:13:36.753 in Ack 20
21:13:36.787 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-47a324dd-07ee-41da-895c-60e9750bcc1f","expiresAt":1794173016765,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:13:36.789 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTQ3YTMyNGRkLTA3ZWUtNDFkYS04OTVjLTYwZTk3NTBiY2MxZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzAxNjc2NX0.DDVTkVA-dIwLoYnWAu3JgdaXeZtiJuezOVW1weIb3Hs
21:13:36.877 exit 22 {"_tag":"Success","value":null}
21:13:37.130 fetch POST /api/observability/v1/traces
21:13:37.130 fetched POST /api/observability/v1/traces 204
21:13:39.061 in Ping 
21:13:40.333 rpc 23 orchestration.dispatchCommand message.dispatch
21:13:40.949 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:13:40.951 rpc 24 attachments.delete
21:13:41.097 exit 24 {"_tag":"Success","value":null}
21:13:41.332 fetch POST /api/observability/v1/traces
21:13:41.332 fetched POST /api/observability/v1/traces 204
21:13:41.451 chunk 17 23652 bytes
21:13:41.452 in Ack 17
21:13:41.701 chunk 17 23626 bytes
21:13:41.702 in Ack 17
21:13:41.952 chunk 17 23676 bytes
21:13:41.953 in Ack 17
21:13:42.453 fetch POST /api/observability/v1/traces
21:13:42.453 fetched POST /api/observability/v1/traces 204
21:13:42.453 chunk 6 2820 bytes
21:13:42.453 in Ack 6
21:13:42.454 rpc 25 orchestration.dispatchCommand thread.visit
21:13:42.454 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:13:42.704 chunk 17 24648 bytes
21:13:42.704 in Ack 17
21:13:42.954 chunk 6 2820 bytes
21:13:42.954 chunk 17 24648 bytes
21:13:42.954 in Ack 6
21:13:42.960 in Ack 17
21:13:43.232 chunk 17 25310 bytes
21:13:43.233 in Ack 17
21:13:43.458 fetch POST /api/observability/v1/traces
21:13:43.458 fetched POST /api/observability/v1/traces 204
21:13:43.484 chunk 17 28103 bytes
21:13:43.485 in Ack 17
21:13:43.488 rpc 26 assets.createUrl
21:13:44.048 rpc 27 orchestration.dispatchCommand message.dispatch
21:13:44.084 in Ping 
21:13:44.486 fetch POST /api/observability/v1/traces
21:13:44.486 fetched POST /api/observability/v1/traces 204
21:13:45.671 fetch POST /api/observability/v1/traces
21:13:45.671 fetched POST /api/observability/v1/traces 204
21:13:46.909 fetch POST /api/observability/v1/traces
21:13:46.909 fetched POST /api/observability/v1/traces 204
21:13:49.085 in Ping 
21:13:54.085 in Ping 
21:13:57.835 chunk 17 30548 bytes
21:13:57.836 exit 27 {"_tag":"Success","value":{"sequence":75}}
21:13:57.837 in Ack 17
21:13:58.839 fetch POST /api/observability/v1/traces
21:13:58.839 fetched POST /api/observability/v1/traces 204
21:13:59.086 in Ping 
21:13:59.234 chunk 17 30580 bytes
21:13:59.236 in Ack 17
21:13:59.285 rpc 28 server.reportClientActivity
21:14:00.082 chunk 17 30632 bytes
21:14:00.084 in Ack 17
21:14:00.237 fetch POST /api/observability/v1/traces
21:14:00.237 fetched POST /api/observability/v1/traces 204
21:14:01.963 chunk 17 30790 bytes
21:14:01.964 in Ack 17
21:14:02.965 fetch POST /api/observability/v1/traces
21:14:02.965 fetched POST /api/observability/v1/traces 204
21:14:03.084 chunk 17 31020 bytes
21:14:03.086 in Ack 17
21:14:03.416 chunk 17 31070 bytes
21:14:03.417 in Ack 17
21:14:03.674 chunk 17 31138 bytes
21:14:03.675 in Ack 17
21:14:04.086 in Ping 
21:14:04.086 fetch POST /api/observability/v1/traces
21:14:04.086 fetched POST /api/observability/v1/traces 204
21:14:04.450 chunk 17 31228 bytes
21:14:04.451 in Ack 17
21:14:04.709 chunk 17 31238 bytes
21:14:04.714 in Ack 17
21:14:05.453 fetch POST /api/observability/v1/traces
21:14:05.453 fetched POST /api/observability/v1/traces 204
21:14:05.569 chunk 17 31250 bytes
21:14:05.570 in Ack 17
21:14:06.571 fetch POST /api/observability/v1/traces
21:14:06.573 fetched POST /api/observability/v1/traces 204
21:14:06.684 chunk 17 31292 bytes
21:14:06.685 in Ack 17
21:14:07.073 chunk 17 31312 bytes
21:14:07.074 in Ack 17
21:14:07.324 chunk 17 31468 bytes
21:14:07.326 in Ack 17
21:14:07.686 fetch POST /api/observability/v1/traces
21:14:07.686 fetched POST /api/observability/v1/traces 204
21:14:07.991 chunk 17 31478 bytes
21:14:07.993 in Ack 17
21:14:08.384 chunk 17 31528 bytes
21:14:08.385 in Ack 17
21:14:08.776 chunk 17 31538 bytes
21:14:08.777 in Ack 17
21:14:08.929 fetch POST /api/observability/v1/traces
21:14:08.929 fetched POST /api/observability/v1/traces 204
21:14:09.086 in Ping 
21:14:09.169 chunk 17 31548 bytes
21:14:09.170 in Ack 17
21:14:09.507 chunk 17 31578 bytes
21:14:09.508 in Ack 17
21:14:10.171 fetch POST /api/observability/v1/traces
21:14:10.171 fetched POST /api/observability/v1/traces 204
21:14:10.190 chunk 17 31630 bytes
21:14:10.191 in Ack 17
21:14:10.533 chunk 17 31682 bytes
21:14:10.534 in Ack 17
21:14:10.789 chunk 17 31692 bytes
21:14:10.790 in Ack 17
21:14:11.126 chunk 17 31714 bytes
21:14:11.127 in Ack 17
21:14:11.192 fetch POST /api/observability/v1/traces
21:14:11.192 fetched POST /api/observability/v1/traces 204
21:14:11.638 chunk 17 31756 bytes
21:14:11.639 in Ack 17
21:14:11.895 chunk 17 31794 bytes
21:14:11.896 in Ack 17
21:14:12.406 chunk 17 31806 bytes
21:14:12.407 in Ack 17
21:14:12.640 fetch POST /api/observability/v1/traces
21:14:12.640 fetched POST /api/observability/v1/traces 204
21:14:12.837 chunk 17 31826 bytes
21:14:12.840 in Ack 17
21:14:13.318 chunk 17 31866 bytes
21:14:13.320 in Ack 17
21:14:13.577 chunk 17 31914 bytes
21:14:13.578 in Ack 17
21:14:13.835 chunk 17 31932 bytes
21:14:13.836 in Ack 17
21:14:13.846 fetch POST /api/observability/v1/traces
21:14:13.846 fetched POST /api/observability/v1/traces 204
21:14:14.086 in Ping 
21:14:14.288 in Interrupt 28
21:14:14.310 chunk 17 31926 bytes
21:14:14.311 in Ack 17
21:14:14.567 chunk 17 31994 bytes
21:14:14.568 in Ack 17
21:14:15.289 fetch POST /api/observability/v1/traces
21:14:15.289 fetched POST /api/observability/v1/traces 204
21:14:15.302 chunk 17 32056 bytes
21:14:15.303 in Ack 17
21:14:15.554 chunk 17 32144 bytes
21:14:15.555 in Ack 17
21:14:15.804 chunk 17 32154 bytes
21:14:15.806 in Ack 17
21:14:16.297 chunk 17 32186 bytes
21:14:16.298 in Ack 17
21:14:16.319 fetch POST /api/observability/v1/traces
21:14:16.319 fetched POST /api/observability/v1/traces 204
21:14:17.095 chunk 17 32256 bytes
21:14:17.097 in Ack 17
21:14:17.351 chunk 17 32266 bytes
21:14:17.352 in Ack 17
21:14:17.608 chunk 17 32404 bytes
21:14:17.609 in Ack 17
21:14:18.098 fetch POST /api/observability/v1/traces
21:14:18.098 fetched POST /api/observability/v1/traces 204
21:14:18.190 chunk 17 32494 bytes
21:14:18.191 in Ack 17
21:14:18.821 chunk 17 32546 bytes
21:14:18.822 in Ack 17
21:14:19.087 in Ping 
21:14:19.192 fetch POST /api/observability/v1/traces
21:14:19.192 fetched POST /api/observability/v1/traces 204
21:14:19.391 chunk 17 32646 bytes
21:14:19.393 in Ack 17
21:14:19.977 chunk 17 32686 bytes
21:14:19.978 in Ack 17
21:14:20.231 chunk 17 32706 bytes
21:14:20.232 in Ack 17
21:14:20.393 fetch POST /api/observability/v1/traces
21:14:20.393 fetched POST /api/observability/v1/traces 204
21:14:20.489 chunk 17 32734 bytes
21:14:20.491 in Ack 17
21:14:20.742 chunk 17 32744 bytes
21:14:20.743 in Ack 17
21:14:21.000 chunk 17 32802 bytes
21:14:21.001 in Ack 17
21:14:21.491 fetch POST /api/observability/v1/traces
21:14:21.491 fetched POST /api/observability/v1/traces 204
21:14:21.522 chunk 17 32852 bytes
21:14:21.523 in Ack 17
21:14:22.145 chunk 17 32932 bytes
21:14:22.146 in Ack 17
21:14:22.395 chunk 17 32944 bytes
21:14:22.396 in Ack 17
21:14:22.524 fetch POST /api/observability/v1/traces
21:14:22.524 fetched POST /api/observability/v1/traces 204
21:14:22.721 chunk 17 32964 bytes
21:14:22.722 in Ack 17
21:14:22.973 chunk 17 32984 bytes
21:14:22.974 in Ack 17
21:14:23.311 chunk 17 33004 bytes
21:14:23.312 in Ack 17
21:14:23.569 chunk 17 33014 bytes
21:14:23.570 in Ack 17
21:14:23.723 fetch POST /api/observability/v1/traces
21:14:23.723 fetched POST /api/observability/v1/traces 204
21:14:23.828 chunk 17 33034 bytes
21:14:23.829 in Ack 17
21:14:24.078 chunk 17 33074 bytes
21:14:24.079 in Ack 17
21:14:24.089 in Ping 
21:14:24.286 rpc 29 server.reportClientActivity
21:14:24.335 chunk 17 33084 bytes
21:14:24.336 in Ack 17
21:14:24.672 chunk 17 33114 bytes
21:14:24.673 in Ack 17
21:14:24.830 fetch POST /api/observability/v1/traces
21:14:24.830 fetched POST /api/observability/v1/traces 204
21:14:25.206 chunk 17 33152 bytes
21:14:25.208 in Ack 17
21:14:26.208 fetch POST /api/observability/v1/traces
21:14:26.208 fetched POST /api/observability/v1/traces 204
21:14:28.938 fetch POST /api/observability/v1/traces
21:14:28.938 fetched POST /api/observability/v1/traces 204
21:14:29.093 in Ping 
21:14:29.293 chunk 17 33178 bytes
21:14:29.294 in Ack 17
21:14:30.295 fetch POST /api/observability/v1/traces
21:14:30.295 fetched POST /api/observability/v1/traces 204
21:14:34.097 in Ping 
21:14:34.348 chunk 17 34186 bytes
21:14:34.349 in Ack 17
21:14:34.524 rpc 30 orchestration.dispatchCommand runtime-request.respond
21:14:35.350 fetch POST /api/observability/v1/traces
21:14:35.351 fetched POST /api/observability/v1/traces 204
21:14:38.941 fetch POST /api/observability/v1/traces
21:14:38.941 fetched POST /api/observability/v1/traces 204
21:14:39.101 in Ping 
21:14:39.287 in Interrupt 29
21:14:40.287 fetch POST /api/observability/v1/traces
21:14:40.288 fetched POST /api/observability/v1/traces 204
21:14:41.232 chunk 17 36693 bytes
21:14:41.232 exit 30 {"_tag":"Success","value":{"sequence":95}}
21:14:41.234 in Ack 17
21:14:41.841 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:14:41.924 chunk 17 36795 bytes
21:14:41.925 in Ack 17
21:14:42.236 fetch POST /api/observability/v1/traces
21:14:42.236 fetched POST /api/observability/v1/traces 204
21:14:42.443 chunk 17 36875 bytes
21:14:42.445 in Ack 17
21:14:43.446 fetch POST /api/observability/v1/traces
21:14:43.446 fetched POST /api/observability/v1/traces 204
21:14:43.640 chunk 17 36977 bytes
21:14:43.641 in Ack 17
21:14:44.078 chunk 17 36997 bytes
21:14:44.079 in Ack 17
21:14:44.105 in Ping 
21:14:44.513 chunk 17 37057 bytes
21:14:44.514 in Ack 17
21:14:44.642 fetch POST /api/observability/v1/traces
21:14:44.642 fetched POST /api/observability/v1/traces 204
21:14:44.951 chunk 17 37077 bytes
21:14:44.952 in Ack 17
21:14:45.388 chunk 17 37107 bytes
21:14:45.389 in Ack 17
21:14:45.646 chunk 17 37239 bytes
21:14:45.647 in Ack 17
21:14:45.953 fetch POST /api/observability/v1/traces
21:14:45.953 fetched POST /api/observability/v1/traces 204
21:14:47.088 chunk 17 37495 bytes
21:14:47.089 in Ack 17
21:14:48.091 fetch POST /api/observability/v1/traces
21:14:48.091 fetched POST /api/observability/v1/traces 204
21:14:49.109 in Ping 
21:14:49.201 chunk 17 37525 bytes
21:14:49.202 in Ack 17
21:14:49.288 rpc 32 server.reportClientActivity
21:14:49.470 chunk 17 37661 bytes
21:14:49.471 in Ack 17
21:14:50.203 fetch POST /api/observability/v1/traces
21:14:50.203 fetched POST /api/observability/v1/traces 204
21:14:51.991 chunk 17 37903 bytes
21:14:51.992 in Ack 17
21:14:52.246 chunk 17 37951 bytes
21:14:52.247 in Ack 17
21:14:52.993 fetch POST /api/observability/v1/traces
21:14:52.993 fetched POST /api/observability/v1/traces 204
21:14:54.113 in Ping 
21:14:57.466 chunk 17 38039 bytes
21:14:57.467 in Ack 17
21:14:57.926 chunk 17 38121 bytes
21:14:57.928 in Ack 17
21:14:58.469 fetch POST /api/observability/v1/traces
21:14:58.469 fetched POST /api/observability/v1/traces 204
