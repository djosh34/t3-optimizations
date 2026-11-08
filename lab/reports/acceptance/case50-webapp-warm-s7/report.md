# case50, webapp, warm start, seed 7

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.6s |
| Whole reply shown | 44.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.1s / 1.5s / 2.6s |
| Longest silence | 4.1s |
| Approval shown after it was asked | 2.0s |
| Bytes on the wire down / up | 273200 / 102133 |
| Bytes the app received / sent | 217116 / 68979 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.6 | 311 | 373 |
| 20 | 1.8 | 701 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.9 | 1424 | 1452 |
| 44 | 1.1 | 1555 | 1590 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","messages":"546/546 456/456 215/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":154,"lastDataAt":1794165853230,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":5682.531482641118,"deliveredRate":3067.6403828863713,"loss":0,"srttMs":2.75398048923766,"minRttMs":1,"sentPackets":113,"sentWireBytes":31735,"receivedPackets":122,"receivedWireBytes":50345,"lastHeardMs":9627,"unsyncedBytes":1009,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/435","runs":"1014/1014 971/971 442/839","user":"10/10 9/9","attempts":"660/660 660/660 636/636","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","messages":"546/546 456/456 610/901 546/546","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":154,"lastDataAt":1794165863361,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.32916666666666666,"srttMs":424.8907404827081,"minRttMs":1,"sentPackets":172,"sentWireBytes":45882,"receivedPackets":162,"receivedWireBytes":69872,"lastHeardMs":19758,"unsyncedBytes":0,"inflightPackets":18,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"925/951","assistant":"32/32 21/21 933/943","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 512/656","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 505/785","messages":"546/546 456/456 610/901 546/546 292/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":154,"lastDataAt":1794165873695,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":1125,"deliveredRate":0,"loss":0.6633333333333333,"srttMs":532.13077829565,"minRttMs":609,"sentPackets":222,"sentWireBytes":56285,"receivedPackets":224,"receivedWireBytes":89473,"lastHeardMs":30092,"unsyncedBytes":167,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 299/1098 1546/1546 875/926","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 43/43","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 715/785 965/1019","messages":"546/546 456/456 875/901 546/546 545/545 254/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"842/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":154,"lastDataAt":1794165884750,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":2268.3139534883717,"deliveredRate":0,"loss":0.5913368983957218,"srttMs":638.7985487410311,"minRttMs":609,"sentPackets":278,"sentWireBytes":66503,"receivedPackets":357,"receivedWireBytes":145721,"lastHeardMs":41148,"unsyncedBytes":399,"inflightPackets":41,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 554/554","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 785/785 1019/1019 1395/1395 1019/1019 985/985","messages":"546/546 456/456 901/901 546/546 546/546 451/451 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":154,"lastDataAt":1794165894916,"silences":0,"longestSilenceMs":1106,"longestRecoveredGapMs":1106},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.42224170040485826,"srttMs":886.4832913140207,"minRttMs":609,"sentPackets":364,"sentWireBytes":89884,"receivedPackets":504,"receivedWireBytes":235939,"lastHeardMs":51313,"unsyncedBytes":500,"inflightPackets":74,"duplicateBytes":0}}
- shim trace
19:24:03.870 fetch GET /api/auth/session
19:24:03.871 fetched GET /api/auth/session 200
19:24:03.974 fetch GET /.well-known/t3/environment
19:24:03.974 fetched GET /.well-known/t3/environment 200
19:24:03.982 fetch GET /.well-known/t3/environment
19:24:03.982 fetched GET /.well-known/t3/environment 200
19:24:03.986 rpc 0 subscribeServerConfig
19:24:03.986 chunk 0 38722 bytes
19:24:04.020 in Ack 0
19:24:04.022 rpc 1 subscribeServerLifecycle
19:24:04.022 rpc 2 subscribeServerLifecycle
19:24:04.025 fetch GET /api/auth/session
19:24:04.025 fetched GET /api/auth/session 200
19:24:04.026 rpc 3 subscribeDeviceState
19:24:04.028 rpc 4 preview.reportProfiles
19:24:04.087 rpc 5 subscribeProjectClones
19:24:04.103 fetch GET /api/orchestration/threads/mcp%3A323e545f-d75c-484d-9536-903aec9dc626/bounded
19:24:04.107 fetch GET /api/orchestration/shell
19:24:04.126 rpc 6 server.refreshProviders
19:24:04.162 rpc 7 orchestration.dispatchCommand thread.visit
19:24:04.162 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:24:04.172 fetched GET /api/orchestration/shell 200
19:24:04.172 rpc 8 subscribeTerminalMetadata
19:24:04.173 rpc 9 subscribeVcsStatus
19:24:04.173 rpc 10 subscribePreviewEvents
19:24:04.175 rpc 11 orchestration.subscribeShell
19:24:04.175 chunk 11 2790 bytes
19:24:04.176 rpc 12 assets.createUrl
19:24:04.177 rpc 13 preview.list
19:24:04.177 rpc 14 projects.readFile
19:24:04.179 in Ack 11
19:24:04.426 rpc 15 server.reportClientActivity
19:24:04.450 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"a061b713-8909-438d-9e02-37e84a69368e","sessions":[]}}
19:24:04.450 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:24:04.689 chunk 1 3872 bytes
19:24:04.689 in Ack 1
19:24:04.939 chunk 2 3872 bytes
19:24:04.939 in Ack 2
19:24:04.971 fetch POST /api/observability/v1/traces
19:24:04.972 fetched POST /api/observability/v1/traces 204
19:24:05.119 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:23:52.207Z","configurableProviders":false,"con
19:24:05.119 chunk 9 1238 bytes
19:24:05.119 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:24:05.122 in Ack 9
19:24:05.333 chunk 11 2766 bytes
19:24:05.334 in Ack 11
19:24:05.587 chunk 11 2809 bytes
19:24:05.587 in Ack 11
19:24:05.628 chunk 3 876 bytes
19:24:05.628 exit 4 {"_tag":"Success","value":null}
19:24:05.628 chunk 5 4 bytes
19:24:05.628 chunk 8 36 bytes
19:24:05.628 in Ack 3
19:24:05.632 rpc 16 preview.reportProfiles
19:24:05.632 in Ack 5
19:24:05.632 in Ack 8
19:24:05.728 exit 15 {"_tag":"Success","value":null}
19:24:05.728 exit 16 {"_tag":"Success","value":null}
19:24:05.889 fetched GET /api/orchestration/threads/mcp%3A323e545f-d75c-484d-9536-903aec9dc626/bounded 200
19:24:05.892 rpc 17 orchestration.subscribeThread
19:24:05.892 chunk 17 8283 bytes
19:24:05.901 in Interrupt 9
19:24:05.903 rpc 18 subscribeVcsStatus
19:24:05.904 in Ack 17
19:24:06.048 chunk 18 1238 bytes
19:24:06.048 in Ack 18
19:24:06.122 fetch POST /api/observability/v1/traces
19:24:06.122 fetched POST /api/observability/v1/traces 204
19:24:06.143 chunk 17 9337 bytes
19:24:06.144 in Ack 17
19:24:06.167 rpc 19 server.reportClientActivity
19:24:06.278 exit 19 {"_tag":"Success","value":null}
19:24:06.395 chunk 17 11920 bytes
19:24:06.395 in Ack 17
19:24:06.645 chunk 17 13502 bytes
19:24:06.646 in Ack 17
19:24:06.788 rpc 20 attachments.createUploadUrl
19:24:06.986 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-98ed9fc0-1616-4dfc-98fc-488ced36d7ad","expiresAt":1794166446979,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:24:06.987 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTk4ZWQ5ZmMwLTE2MTYtNGRmYy05OGZjLTQ4OGNlZDM2ZDdhZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjQ0Njk3OX0.PybPf3df6BRhIhfKdo4HJ_dg7b63jFRaxZ38Mj-oAo4
19:24:07.145 fetch POST /api/observability/v1/traces
19:24:07.145 fetched POST /api/observability/v1/traces 204
19:24:07.209 chunk 17 16198 bytes
19:24:07.210 in Ack 17
19:24:07.459 chunk 17 19072 bytes
19:24:07.460 in Ack 17
19:24:07.710 chunk 17 21199 bytes
19:24:07.711 in Ack 17
19:24:07.718 in Interrupt 18
19:24:07.719 rpc 21 subscribeVcsStatus
19:24:07.782 chunk 21 1238 bytes
19:24:07.783 in Ack 21
19:24:07.970 rpc 22 server.reportClientActivity
19:24:08.211 fetch POST /api/observability/v1/traces
19:24:08.211 fetched POST /api/observability/v1/traces 204
19:24:08.984 in Ping 
19:24:09.667 exit 22 {"_tag":"Success","value":null}
19:24:10.390 rpc 23 orchestration.dispatchCommand message.dispatch
19:24:10.924 fetch POST /api/observability/v1/traces
19:24:10.924 fetched POST /api/observability/v1/traces 204
19:24:10.987 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:24:10.989 rpc 24 attachments.delete
19:24:11.277 exit 24 {"_tag":"Success","value":null}
19:24:11.509 chunk 17 23605 bytes
19:24:11.510 in Ack 17
19:24:11.761 chunk 17 23617 bytes
19:24:11.762 in Ack 17
19:24:11.989 fetch POST /api/observability/v1/traces
19:24:11.989 fetched POST /api/observability/v1/traces 204
19:24:12.023 chunk 17 23667 bytes
19:24:12.024 in Ack 17
19:24:12.273 chunk 11 2831 bytes
19:24:12.274 chunk 17 27432 bytes
19:24:12.274 in Ack 11
19:24:12.275 rpc 25 orchestration.dispatchCommand thread.visit
19:24:12.275 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:24:12.310 in Ack 17
19:24:12.312 rpc 26 assets.createUrl
19:24:12.774 chunk 11 2831 bytes
19:24:12.774 chunk 17 30516 bytes
19:24:12.774 in Ack 11
19:24:12.779 in Ack 17
19:24:13.214 chunk 17 31940 bytes
19:24:13.214 fetch POST /api/observability/v1/traces
19:24:13.214 fetched POST /api/observability/v1/traces 204
19:24:13.216 in Ack 17
19:24:13.222 in Interrupt 21
19:24:13.224 rpc 27 subscribeVcsStatus
19:24:13.469 chunk 17 31940 bytes
19:24:13.470 in Ack 17
19:24:13.482 rpc 28 server.reportClientActivity
19:24:13.785 rpc 29 orchestration.dispatchCommand message.dispatch
19:24:13.985 in Ping 
19:24:14.092 chunk 17 32508 bytes
19:24:14.094 in Ack 17
19:24:14.217 fetch POST /api/observability/v1/traces
19:24:14.217 fetched POST /api/observability/v1/traces 204
19:24:14.628 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:24:14.628 chunk 27 1238 bytes
19:24:14.632 in Ack 27
19:24:14.717 chunk 17 32508 bytes
19:24:14.719 in Ack 17
19:24:14.980 chunk 17 34942 bytes
19:24:14.981 in Ack 17
19:24:15.231 chunk 17 34972 bytes
19:24:15.232 in Ack 17
19:24:15.419 exit 28 {"_tag":"Success","value":null}
19:24:15.483 chunk 17 35002 bytes
19:24:15.484 in Ack 17
19:24:15.639 fetch POST /api/observability/v1/traces
19:24:15.639 fetched POST /api/observability/v1/traces 204
19:24:15.733 chunk 17 35022 bytes
19:24:15.734 in Ack 17
19:24:15.984 chunk 17 35042 bytes
19:24:15.986 in Ack 17
19:24:16.235 chunk 17 35024 bytes
19:24:16.236 in Ack 17
19:24:16.486 chunk 17 35036 bytes
19:24:16.487 in Ack 17
19:24:16.736 fetch POST /api/observability/v1/traces
19:24:16.736 fetched POST /api/observability/v1/traces 204
19:24:16.736 chunk 17 35056 bytes
19:24:16.737 in Ack 17
19:24:16.986 chunk 17 35088 bytes
19:24:16.987 in Ack 17
19:24:17.236 chunk 17 35118 bytes
19:24:17.237 in Ack 17
19:24:17.547 chunk 11 2895 bytes
19:24:17.547 chunk 17 35146 bytes
19:24:17.548 in Ack 11
19:24:17.590 in Ack 17
19:24:17.739 fetch POST /api/observability/v1/traces
19:24:17.739 fetched POST /api/observability/v1/traces 204
19:24:17.798 chunk 17 35164 bytes
19:24:17.799 in Ack 17
19:24:18.049 chunk 17 35196 bytes
19:24:18.050 in Ack 17
19:24:18.299 chunk 17 35206 bytes
19:24:18.300 in Ack 17
19:24:18.587 chunk 17 35228 bytes
19:24:18.588 in Ack 17
19:24:18.799 fetch POST /api/observability/v1/traces
19:24:18.799 fetched POST /api/observability/v1/traces 204
19:24:18.837 chunk 17 35238 bytes
19:24:18.838 in Ack 17
19:24:18.985 in Ping 
19:24:19.556 chunk 17 35270 bytes
19:24:19.557 in Ack 17
19:24:19.839 fetch POST /api/observability/v1/traces
19:24:19.839 fetched POST /api/observability/v1/traces 204
19:24:20.038 chunk 17 35342 bytes
19:24:20.039 in Ack 17
19:24:20.289 chunk 17 35416 bytes
19:24:20.290 in Ack 17
19:24:20.781 chunk 17 35456 bytes
19:24:20.783 in Ack 17
19:24:21.031 chunk 17 36113 bytes
19:24:21.032 in Ack 17
19:24:21.042 fetch POST /api/observability/v1/traces
19:24:21.042 fetched POST /api/observability/v1/traces 204
19:24:21.772 chunk 17 36165 bytes
19:24:21.773 in Ack 17
19:24:22.247 chunk 17 36271 bytes
19:24:22.248 in Ack 17
19:24:22.279 rpc 30 orchestration.dispatchCommand thread.visit
19:24:22.279 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:24:22.497 chunk 17 36291 bytes
19:24:22.498 in Ack 17
19:24:22.776 fetch POST /api/observability/v1/traces
19:24:22.776 fetched POST /api/observability/v1/traces 204
19:24:23.188 chunk 17 36333 bytes
19:24:23.189 in Ack 17
19:24:23.438 chunk 17 36363 bytes
19:24:23.439 in Ack 17
19:24:23.985 in Ping 
19:24:24.154 chunk 17 36385 bytes
19:24:24.155 in Ack 17
19:24:24.190 fetch POST /api/observability/v1/traces
19:24:24.190 fetched POST /api/observability/v1/traces 204
19:24:24.405 chunk 17 36405 bytes
19:24:24.406 in Ack 17
19:24:24.655 chunk 17 36435 bytes
19:24:24.656 in Ack 17
19:24:24.905 chunk 17 36455 bytes
19:24:24.906 in Ack 17
19:24:25.274 chunk 17 36535 bytes
19:24:25.275 in Ack 17
19:24:25.408 fetch POST /api/observability/v1/traces
19:24:25.408 fetched POST /api/observability/v1/traces 204
19:24:25.789 chunk 17 36575 bytes
19:24:25.790 in Ack 17
19:24:26.040 chunk 17 36595 bytes
19:24:26.041 in Ack 17
19:24:26.289 chunk 17 36643 bytes
19:24:26.290 in Ack 17
19:24:26.540 chunk 17 36681 bytes
19:24:26.541 in Ack 17
19:24:26.791 chunk 17 36701 bytes
19:24:26.791 fetch POST /api/observability/v1/traces
19:24:26.791 fetched POST /api/observability/v1/traces 204
19:24:26.792 in Ack 17
19:24:27.041 chunk 17 36723 bytes
19:24:27.042 in Ack 17
19:24:27.291 chunk 17 36753 bytes
19:24:27.292 in Ack 17
19:24:27.541 chunk 17 36763 bytes
19:24:27.542 in Ack 17
19:24:27.791 chunk 17 36783 bytes
19:24:27.792 in Ack 17
19:24:27.800 fetch POST /api/observability/v1/traces
19:24:27.801 fetched POST /api/observability/v1/traces 204
19:24:28.041 chunk 17 36793 bytes
19:24:28.042 in Ack 17
19:24:28.291 chunk 17 36825 bytes
19:24:28.293 in Ack 17
19:24:28.541 chunk 17 36835 bytes
19:24:28.542 in Ack 17
19:24:28.792 chunk 17 36895 bytes
19:24:28.793 in Ack 17
19:24:28.985 in Ping 
19:24:29.042 chunk 17 36933 bytes
19:24:29.043 in Ack 17
19:24:29.050 fetch POST /api/observability/v1/traces
19:24:29.050 fetched POST /api/observability/v1/traces 204
19:24:29.226 rpc 31 server.reportClientActivity
19:24:29.292 chunk 17 36943 bytes
19:24:29.293 in Ack 17
19:24:29.542 chunk 17 36965 bytes
19:24:29.543 in Ack 17
19:24:29.793 chunk 17 36985 bytes
19:24:29.794 in Ack 17
19:24:30.044 chunk 17 37015 bytes
19:24:30.044 in Ack 17
19:24:30.227 fetch POST /api/observability/v1/traces
19:24:30.227 fetched POST /api/observability/v1/traces 204
19:24:30.293 chunk 11 2895 bytes
19:24:30.294 chunk 17 39066 bytes
19:24:30.294 in Ack 11
19:24:30.300 in Ack 17
19:24:30.544 chunk 17 39068 bytes
19:24:30.545 in Ack 17
19:24:30.795 chunk 17 39926 bytes
19:24:30.796 in Ack 17
19:24:31.045 chunk 17 39948 bytes
19:24:31.046 in Ack 17
19:24:31.295 fetch POST /api/observability/v1/traces
19:24:31.295 fetched POST /api/observability/v1/traces 204
19:24:31.400 chunk 17 39960 bytes
19:24:31.401 in Ack 17
19:24:31.650 chunk 17 39982 bytes
19:24:31.651 in Ack 17
19:24:31.901 chunk 17 40030 bytes
19:24:31.902 in Ack 17
19:24:32.151 chunk 17 40060 bytes
19:24:32.152 in Ack 17
19:24:32.280 rpc 32 orchestration.dispatchCommand thread.visit
19:24:32.280 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:24:32.401 chunk 17 40090 bytes
19:24:32.402 in Ack 17
19:24:32.409 fetch POST /api/observability/v1/traces
19:24:32.409 fetched POST /api/observability/v1/traces 204
19:24:32.652 chunk 17 40120 bytes
19:24:32.653 in Ack 17
19:24:32.903 chunk 17 40140 bytes
19:24:32.904 in Ack 17
19:24:33.153 chunk 17 40178 bytes
19:24:33.154 in Ack 17
19:24:33.404 chunk 17 40216 bytes
19:24:33.405 in Ack 17
19:24:33.655 fetch POST /api/observability/v1/traces
19:24:33.655 fetched POST /api/observability/v1/traces 204
19:24:33.655 chunk 17 40234 bytes
19:24:33.656 in Ack 17
19:24:33.905 chunk 17 40236 bytes
19:24:33.906 in Ack 17
19:24:33.987 in Ping 
19:24:34.155 chunk 17 40246 bytes
19:24:34.156 in Ack 17
19:24:34.406 chunk 17 40268 bytes
19:24:34.407 in Ack 17
19:24:34.656 chunk 17 40288 bytes
19:24:34.657 fetch POST /api/observability/v1/traces
19:24:34.657 fetched POST /api/observability/v1/traces 204
19:24:34.658 in Ack 17
19:24:34.906 chunk 17 40338 bytes
19:24:34.907 in Ack 17
19:24:35.157 chunk 17 40386 bytes
19:24:35.158 in Ack 17
19:24:35.657 chunk 17 40418 bytes
19:24:35.658 in Ack 17
19:24:35.666 fetch POST /api/observability/v1/traces
19:24:35.666 fetched POST /api/observability/v1/traces 204
19:24:35.907 chunk 17 40440 bytes
19:24:35.908 in Ack 17
19:24:36.157 chunk 17 40460 bytes
19:24:36.158 in Ack 17
19:24:36.407 chunk 17 40498 bytes
19:24:36.409 in Ack 17
19:24:36.657 chunk 17 40526 bytes
19:24:36.658 in Ack 17
19:24:36.991 chunk 17 40536 bytes
19:24:36.991 fetch POST /api/observability/v1/traces
19:24:36.991 fetched POST /api/observability/v1/traces 204
19:24:36.993 in Ack 17
19:24:37.242 chunk 17 40586 bytes
19:24:37.243 in Ack 17
19:24:37.493 chunk 11 2895 bytes
19:24:37.493 chunk 17 40598 bytes
19:24:37.493 in Ack 11
19:24:37.499 in Ack 17
19:24:37.743 chunk 17 40620 bytes
19:24:37.744 in Ack 17
19:24:37.994 fetch POST /api/observability/v1/traces
19:24:37.994 fetched POST /api/observability/v1/traces 204
19:24:38.029 chunk 17 40660 bytes
19:24:38.030 in Ack 17
19:24:38.279 chunk 17 40670 bytes
19:24:38.281 in Ack 17
19:24:38.529 chunk 17 40708 bytes
19:24:38.530 in Ack 17
19:24:38.779 chunk 17 40748 bytes
19:24:38.780 in Ack 17
19:24:38.987 in Ping 
19:24:39.029 chunk 17 40768 bytes
19:24:39.030 in Ack 17
19:24:39.038 fetch POST /api/observability/v1/traces
19:24:39.038 fetched POST /api/observability/v1/traces 204
19:24:39.280 chunk 17 40780 bytes
19:24:39.281 in Ack 17
19:24:39.530 chunk 17 40826 bytes
19:24:39.531 in Ack 17
19:24:39.780 chunk 17 40858 bytes
19:24:39.781 in Ack 17
19:24:40.033 chunk 17 40878 bytes
19:24:40.035 in Ack 17
19:24:40.280 chunk 17 40890 bytes
19:24:40.281 in Ack 17
19:24:40.289 fetch POST /api/observability/v1/traces
19:24:40.289 fetched POST /api/observability/v1/traces 204
19:24:40.530 chunk 17 41687 bytes
19:24:40.531 in Ack 17
19:24:40.780 chunk 17 44220 bytes
19:24:40.781 in Ack 17
19:24:41.031 chunk 17 44250 bytes
19:24:41.032 in Ack 17
19:24:41.122 exit 31 {"_tag":"Success","value":null}
19:24:41.281 chunk 17 44319 bytes
19:24:41.282 in Ack 17
19:24:41.531 chunk 17 44349 bytes
19:24:41.532 in Ack 17
19:24:41.553 fetch POST /api/observability/v1/traces
19:24:41.553 fetched POST /api/observability/v1/traces 204
19:24:41.782 chunk 17 44376 bytes
19:24:41.783 in Ack 17
19:24:42.784 fetch POST /api/observability/v1/traces
19:24:42.784 fetched POST /api/observability/v1/traces 204
19:24:43.134 chunk 17 45381 bytes
19:24:43.135 in Ack 17
19:24:43.354 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:24:43.988 in Ping 
19:24:44.138 fetch POST /api/observability/v1/traces
19:24:44.138 fetched POST /api/observability/v1/traces 204
19:24:44.154 chunk 17 48877 bytes
19:24:44.155 in Ack 17
19:24:44.318 exit 33 {"_tag":"Success","value":{"sequence":97}}
19:24:44.405 chunk 17 51348 bytes
19:24:44.406 in Ack 17
19:24:44.623 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:24:44.655 chunk 17 51368 bytes
19:24:44.663 in Ack 17
19:24:44.906 chunk 17 51408 bytes
19:24:44.907 in Ack 17
19:24:45.157 fetch POST /api/observability/v1/traces
19:24:45.157 fetched POST /api/observability/v1/traces 204
19:24:45.158 chunk 17 51428 bytes
19:24:45.159 in Ack 17
19:24:45.408 chunk 17 51440 bytes
19:24:45.409 in Ack 17
19:24:45.658 chunk 17 51450 bytes
19:24:45.659 in Ack 17
19:24:45.747 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:24:45.962 chunk 17 51550 bytes
19:24:45.965 in Ack 17
19:24:46.165 fetch POST /api/observability/v1/traces
19:24:46.165 fetched POST /api/observability/v1/traces 204
19:24:46.212 chunk 17 51570 bytes
19:24:46.214 in Ack 17
19:24:46.248 rpc 35 orchestration.dispatchCommand runtime-request.respond
19:24:46.463 chunk 17 54510 bytes
19:24:46.464 in Ack 17
19:24:46.471 in Interrupt 27
19:24:46.494 rpc 36 subscribeVcsStatus
19:24:46.714 chunk 17 54585 bytes
19:24:46.715 in Ack 17
19:24:46.756 rpc 37 server.reportClientActivity
19:24:46.964 chunk 17 54557 bytes
19:24:46.965 in Ack 17
19:24:47.186 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:24:47.225 chunk 11 2897 bytes
19:24:47.226 chunk 17 54569 bytes
19:24:47.227 fetch POST /api/observability/v1/traces
19:24:47.227 fetched POST /api/observability/v1/traces 204
19:24:47.227 in Ack 11
19:24:47.228 rpc 38 orchestration.dispatchCommand thread.visit
19:24:47.228 exit 38 {"_tag":"Success","value":{"sequence":0}}
19:24:47.231 in Ack 17
19:24:47.476 chunk 17 55480 bytes
19:24:47.477 in Ack 17
19:24:47.727 chunk 17 55500 bytes
19:24:47.728 in Ack 17
19:24:47.978 chunk 17 55510 bytes
19:24:47.979 in Ack 17
19:24:48.189 chunk 36 1238 bytes
19:24:48.189 in Ack 36
19:24:48.230 fetch POST /api/observability/v1/traces
19:24:48.230 fetched POST /api/observability/v1/traces 204
19:24:48.230 chunk 17 55604 bytes
19:24:48.232 in Ack 17
19:24:48.481 chunk 17 55614 bytes
19:24:48.482 in Ack 17
19:24:48.732 chunk 17 55664 bytes
19:24:48.733 in Ack 17
19:24:48.988 in Ping 
19:24:49.049 chunk 17 55694 bytes
19:24:49.050 in Ack 17
19:24:49.232 fetch POST /api/observability/v1/traces
19:24:49.232 fetched POST /api/observability/v1/traces 204
19:24:49.300 chunk 17 55724 bytes
19:24:49.301 in Ack 17
19:24:49.550 chunk 17 55734 bytes
19:24:49.551 in Ack 17
19:24:49.800 chunk 17 55754 bytes
19:24:49.802 in Ack 17
19:24:50.050 chunk 17 55784 bytes
19:24:50.051 in Ack 17
19:24:50.275 exit 37 {"_tag":"Success","value":null}
19:24:50.300 chunk 17 57307 bytes
19:24:50.301 in Ack 17
19:24:50.362 fetch POST /api/observability/v1/traces
19:24:50.362 fetched POST /api/observability/v1/traces 204
19:24:50.550 chunk 11 2897 bytes
19:24:50.550 chunk 17 58771 bytes
19:24:50.551 in Ack 11
19:24:50.556 in Ack 17
19:24:50.828 chunk 17 58791 bytes
19:24:50.829 in Ack 17
19:24:51.078 chunk 17 58821 bytes
19:24:51.079 in Ack 17
19:24:51.328 chunk 17 58841 bytes
19:24:51.329 in Ack 17
19:24:51.554 fetch POST /api/observability/v1/traces
19:24:51.554 fetched POST /api/observability/v1/traces 204
19:24:51.579 chunk 17 60901 bytes
19:24:51.580 in Ack 17
19:24:51.830 chunk 17 60911 bytes
19:24:51.830 in Ack 17
19:24:52.080 chunk 17 60941 bytes
19:24:52.081 in Ack 17
19:24:52.332 chunk 17 60973 bytes
19:24:52.333 in Ack 17
19:24:52.581 fetch POST /api/observability/v1/traces
19:24:52.581 fetched POST /api/observability/v1/traces 204
19:24:52.582 chunk 11 2897 bytes
19:24:52.583 chunk 17 61967 bytes
19:24:52.583 in Ack 11
19:24:52.585 in Ack 17
19:24:52.856 chunk 17 61997 bytes
19:24:52.857 in Ack 17
19:24:53.106 chunk 17 62057 bytes
19:24:53.108 in Ack 17
19:24:53.144 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/99cd7173-2fb9-4a81-851e-af73f2e4af01","expiresAt":1794169493144}}
19:24:53.356 chunk 17 62117 bytes
19:24:53.358 in Ack 17
19:24:53.584 fetch POST /api/observability/v1/traces
19:24:53.584 fetched POST /api/observability/v1/traces 204
19:24:53.607 chunk 17 62168 bytes
19:24:53.608 in Ack 17
19:24:53.858 chunk 17 62198 bytes
19:24:53.860 in Ack 17
19:24:53.988 in Ping 
19:24:54.108 chunk 17 62234 bytes
19:24:54.110 in Ack 17
19:24:54.232 rpc 39 server.reportClientActivity
19:24:54.359 chunk 17 62294 bytes
19:24:54.360 in Ack 17
19:24:54.609 fetch POST /api/observability/v1/traces
19:24:54.609 fetched POST /api/observability/v1/traces 204
19:24:54.609 chunk 17 62345 bytes
19:24:54.610 in Ack 17
19:24:54.859 chunk 17 62375 bytes
19:24:54.860 in Ack 17
19:24:55.109 chunk 17 62420 bytes
19:24:55.110 in Ack 17
19:24:55.359 chunk 17 62450 bytes
19:24:55.360 in Ack 17
19:24:55.609 chunk 17 62489 bytes
19:24:55.610 in Ack 17
19:24:55.686 fetch POST /api/observability/v1/traces
19:24:55.686 fetched POST /api/observability/v1/traces 204
19:24:55.859 chunk 17 62534 bytes
19:24:55.860 in Ack 17
19:24:56.003 exit 39 {"_tag":"Success","value":null}
19:24:56.109 chunk 17 62564 bytes
19:24:56.110 in Ack 17
19:24:56.359 chunk 17 62621 bytes
19:24:56.361 in Ack 17
19:24:56.616 chunk 17 62651 bytes
19:24:56.617 in Ack 17
19:24:56.861 fetch POST /api/observability/v1/traces
19:24:56.861 fetched POST /api/observability/v1/traces 204
19:24:56.866 chunk 17 62705 bytes
19:24:56.867 in Ack 17
19:24:57.116 chunk 17 62735 bytes
19:24:57.117 in Ack 17
19:24:57.229 rpc 40 orchestration.dispatchCommand thread.visit
19:24:57.229 exit 40 {"_tag":"Success","value":{"sequence":0}}
19:24:57.366 chunk 17 62771 bytes
19:24:57.367 in Ack 17
19:24:57.619 chunk 17 62801 bytes
19:24:57.621 in Ack 17
19:24:57.868 fetch POST /api/observability/v1/traces
19:24:57.868 fetched POST /api/observability/v1/traces 204
19:24:57.870 chunk 17 62855 bytes
19:24:57.871 in Ack 17
19:24:58.120 chunk 17 62888 bytes
19:24:58.122 in Ack 17
19:24:58.371 chunk 17 63001 bytes
19:24:58.372 in Ack 17
19:24:58.678 chunk 17 63133 bytes
19:24:58.680 in Ack 17
