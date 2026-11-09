# story, webapp, warm start, seed 13

Profile story (graded): seed 13's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 57 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.7s |
| Prompt reached the agent | 7.0s |
| Time to first text | 18.4s |
| Whole reply shown | 54.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.9s / 4.9s / 14.9s |
| Longest silence | 18.4s |
| Approval shown after it was asked | 4.4s |
| Bytes on the wire down / up | 317804 / 200158 |
| Bytes the app received / sent | 196878 / 76111 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.9 | 0 | 122 |
| 20 | 2.9 | 397 | 523 |
| 30 | 12.9 | 397 | 925 |
| 40 | 0.5 | 1077 | 1101 |
| 50 | 2.7 | 1378 | 1477 |
| 54 | 6.7 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","messages":"456/456 544/544 901/901 544/544","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217060931,"silences":0,"longestSilenceMs":1704,"longestRecoveredGapMs":1704},"stats":{"sendRate":3375,"deliveredRate":2941.1165437299655,"loss":0,"srttMs":386.5487584899012,"minRttMs":1093,"sentPackets":149,"sentWireBytes":40479,"receivedPackets":180,"receivedWireBytes":116322,"lastHeardMs":16505,"unsyncedBytes":963,"inflightPackets":16,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 451/451","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"590/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217071069,"silences":0,"longestSilenceMs":1704,"longestRecoveredGapMs":1704},"stats":{"sendRate":3000,"deliveredRate":624.5634458672876,"loss":0.5,"srttMs":2146.4606438021356,"minRttMs":1093,"sentPackets":213,"sentWireBytes":59748,"receivedPackets":276,"receivedWireBytes":173521,"lastHeardMs":26642,"unsyncedBytes":457,"inflightPackets":37,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"892/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 451/451","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217081168,"silences":0,"longestSilenceMs":1704,"longestRecoveredGapMs":1704},"stats":{"sendRate":1186.8920759396908,"deliveredRate":517.0556552962298,"loss":0.6,"srttMs":2070.98227242704,"minRttMs":1093,"sentPackets":275,"sentWireBytes":73687,"receivedPackets":378,"receivedWireBytes":222326,"lastHeardMs":36742,"unsyncedBytes":0,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217091187,"silences":1,"longestSilenceMs":2459,"longestRecoveredGapMs":2459},"stats":{"sendRate":1186.8920759396908,"deliveredRate":0,"loss":0.27598749999999994,"srttMs":2022.9586025628228,"minRttMs":1093,"sentPackets":322,"sentWireBytes":83043,"receivedPackets":451,"receivedWireBytes":248406,"lastHeardMs":46760,"unsyncedBytes":167,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1189/1542","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"381/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217101601,"silences":1,"longestSilenceMs":2459,"longestRecoveredGapMs":2459},"stats":{"sendRate":1930.2667506826297,"deliveredRate":1057.4824633022686,"loss":0.09999999999999998,"srttMs":2154.6583849806575,"minRttMs":1093,"sentPackets":374,"sentWireBytes":95934,"receivedPackets":492,"receivedWireBytes":267390,"lastHeardMs":57175,"unsyncedBytes":398,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 1068/1068 75/947","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":770,"lastDataAt":1794217112925,"silences":1,"longestSilenceMs":2459,"longestRecoveredGapMs":2459},"stats":{"sendRate":1469.0072508005005,"deliveredRate":921.2623855512354,"loss":0.1615384615384615,"srttMs":1775.752863151352,"minRttMs":1093,"sentPackets":431,"sentWireBytes":107784,"receivedPackets":558,"receivedWireBytes":292955,"lastHeardMs":68499,"unsyncedBytes":0,"inflightPackets":44,"duplicateBytes":0}}
- shim trace
09:37:25.277 fetch GET /api/auth/session
09:37:25.280 fetched GET /api/auth/session 200
09:37:25.817 fetch GET /.well-known/t3/environment
09:37:25.819 fetched GET /.well-known/t3/environment 200
09:37:25.853 fetch GET /.well-known/t3/environment
09:37:25.854 fetched GET /.well-known/t3/environment 200
09:37:25.864 rpc 0 subscribeServerConfig
09:37:25.865 chunk 0 38462 bytes
09:37:25.974 in Ack 0
09:37:25.977 rpc 1 subscribeServerLifecycle
09:37:25.977 rpc 2 subscribeServerLifecycle
09:37:25.985 fetch GET /api/auth/session
09:37:25.987 fetched GET /api/auth/session 200
09:37:25.988 rpc 3 subscribeDeviceState
09:37:25.990 rpc 4 preview.reportProfiles
09:37:26.125 rpc 5 subscribeProjectClones
09:37:26.132 rpc 6 server.reportClientActivity
09:37:26.196 fetch GET /api/orchestration/threads/mcp%3Adcc2fb93-2b2d-4a5f-9e09-1d49e76683c2/bounded
09:37:26.221 fetch GET /api/orchestration/shell
09:37:26.265 rpc 7 server.refreshProviders
09:37:26.389 rpc 8 orchestration.dispatchCommand thread.visit
09:37:26.389 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:26.423 fetched GET /api/orchestration/shell 200
09:37:26.423 rpc 9 subscribeTerminalMetadata
09:37:26.424 rpc 10 subscribeVcsStatus
09:37:26.426 rpc 11 subscribePreviewEvents
09:37:26.430 rpc 12 orchestration.subscribeShell
09:37:26.430 chunk 12 4551 bytes
09:37:26.433 rpc 13 assets.createUrl
09:37:26.434 rpc 14 preview.list
09:37:26.435 rpc 15 projects.readFile
09:37:26.444 in Ack 12
09:37:26.568 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f82b8362-7706-400a-b506-0870b61acf13","sessions":[]}}
09:37:26.841 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:26.841 chunk 1 3815 bytes
09:37:26.843 fetch POST /api/observability/v1/traces
09:37:26.843 fetched POST /api/observability/v1/traces 204
09:37:26.849 in Ack 1
09:37:27.044 chunk 2 3815 bytes
09:37:27.046 in Ack 2
09:37:27.287 chunk 12 4527 bytes
09:37:27.288 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:57.186Z","configurableProviders":false,"con
09:37:27.288 chunk 10 1632 bytes
09:37:27.288 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:27.289 in Ack 12
09:37:27.322 in Ack 10
09:37:27.534 chunk 12 4613 bytes
09:37:27.535 in Ack 12
09:37:27.776 chunk 3 876 bytes
09:37:27.776 exit 4 {"_tag":"Success","value":null}
09:37:27.776 chunk 5 4 bytes
09:37:27.776 exit 6 {"_tag":"Success","value":null}
09:37:27.777 in Ack 3
09:37:27.784 rpc 16 preview.reportProfiles
09:37:27.785 in Ack 5
09:37:27.788 chunk 12 4656 bytes
09:37:27.789 in Ack 12
09:37:27.849 fetch POST /api/observability/v1/traces
09:37:27.849 fetched POST /api/observability/v1/traces 204
09:37:27.862 chunk 9 36 bytes
09:37:27.862 exit 16 {"_tag":"Success","value":null}
09:37:27.862 in Ack 9
09:37:28.037 rpc 17 server.reportClientActivity
09:37:28.188 exit 17 {"_tag":"Success","value":null}
09:37:28.292 chunk 12 4595 bytes
09:37:28.292 in Ack 12
09:37:28.884 fetch POST /api/observability/v1/traces
09:37:28.884 fetched POST /api/observability/v1/traces 204
09:37:29.295 chunk 12 4592 bytes
09:37:29.296 in Ack 12
09:37:29.545 chunk 12 4592 bytes
09:37:29.546 in Ack 12
09:37:30.297 fetch POST /api/observability/v1/traces
09:37:30.297 fetched POST /api/observability/v1/traces 204
09:37:30.754 fetched GET /api/orchestration/threads/mcp%3Adcc2fb93-2b2d-4a5f-9e09-1d49e76683c2/bounded 200
09:37:30.759 rpc 18 orchestration.subscribeThread
09:37:30.760 chunk 18 8282 bytes
09:37:30.777 in Interrupt 10
09:37:30.780 rpc 19 subscribeVcsStatus
09:37:30.782 in Ack 18
09:37:30.881 in Ping 
09:37:31.004 chunk 19 1632 bytes
09:37:31.004 in Ack 19
09:37:31.011 chunk 18 11259 bytes
09:37:31.013 in Ack 18
09:37:31.143 rpc 20 server.reportClientActivity
09:37:31.266 chunk 18 12690 bytes
09:37:31.268 in Ack 18
09:37:31.516 chunk 18 13497 bytes
09:37:31.518 in Ack 18
09:37:31.744 exit 20 {"_tag":"Success","value":null}
09:37:31.756 fetch POST /api/observability/v1/traces
09:37:31.756 fetched POST /api/observability/v1/traces 204
09:37:31.834 rpc 21 attachments.createUploadUrl
09:37:31.844 chunk 18 18442 bytes
09:37:31.913 in Ack 18
09:37:31.992 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-0475ac4e-42d4-453d-9b4a-9b208a2ccc67","expiresAt":1794217651962,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:31.994 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA0NzVhYzRlLTQyZDQtNDUzZC05YjRhLTliMjA4YTJjY2M2NyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1MTk2Mn0.WNld_-gmy42b8XdgRdUmX8RhL8QX0idlVMZvDVHrQSk
09:37:32.095 chunk 18 21099 bytes
09:37:32.097 in Ack 18
09:37:32.107 in Interrupt 19
09:37:32.109 rpc 22 subscribeVcsStatus
09:37:32.156 chunk 22 1632 bytes
09:37:32.157 in Ack 22
09:37:32.361 rpc 23 server.reportClientActivity
09:37:32.833 fetch POST /api/observability/v1/traces
09:37:32.834 fetched POST /api/observability/v1/traces 204
09:37:33.917 exit 23 {"_tag":"Success","value":null}
09:37:34.921 fetch POST /api/observability/v1/traces
09:37:34.921 fetched POST /api/observability/v1/traces 204
09:37:35.315 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:35.901 in Ping 
09:37:35.903 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:35.909 rpc 25 attachments.delete
09:37:36.005 exit 25 {"_tag":"Success","value":null}
09:37:36.156 chunk 12 4678 bytes
09:37:36.156 chunk 18 22575 bytes
09:37:36.157 in Ack 12
09:37:36.205 in Ack 18
09:37:36.312 fetch POST /api/observability/v1/traces
09:37:36.313 fetched POST /api/observability/v1/traces 204
09:37:36.386 rpc 26 orchestration.dispatchCommand thread.visit
09:37:36.386 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:36.408 chunk 18 27722 bytes
09:37:36.411 in Ack 18
09:37:36.415 rpc 27 assets.createUrl
09:37:36.656 chunk 12 4678 bytes
09:37:36.657 in Ack 12
09:37:36.668 chunk 18 27722 bytes
09:37:36.671 in Ack 18
09:37:36.907 chunk 12 4617 bytes
09:37:36.908 in Ack 12
09:37:36.919 chunk 18 30179 bytes
09:37:36.923 in Ack 18
09:37:37.169 chunk 18 30885 bytes
09:37:37.171 in Ack 18
09:37:37.387 fetch POST /api/observability/v1/traces
09:37:37.387 fetched POST /api/observability/v1/traces 204
09:37:37.418 chunk 18 31902 bytes
09:37:37.423 in Ack 18
09:37:37.916 chunk 12 4614 bytes
09:37:37.916 in Ack 12
09:37:37.918 rpc 28 orchestration.dispatchCommand thread.visit
09:37:37.918 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:37:37.924 chunk 18 34457 bytes
09:37:37.926 in Ack 18
09:37:38.426 chunk 12 4614 bytes
09:37:38.427 chunk 18 35259 bytes
09:37:38.427 fetch POST /api/observability/v1/traces
09:37:38.427 fetched POST /api/observability/v1/traces 204
09:37:38.430 in Ack 12
09:37:38.439 in Ack 18
09:37:38.449 in Interrupt 22
09:37:38.453 rpc 29 subscribeVcsStatus
09:37:38.488 chunk 29 1632 bytes
09:37:38.488 in Ack 29
09:37:38.753 rpc 30 server.reportClientActivity
09:37:38.869 rpc 31 server.refreshProviders
09:37:38.869 exit 31 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:57.186Z","configurableProviders":false,"con
09:37:39.462 fetch POST /api/observability/v1/traces
09:37:39.462 fetched POST /api/observability/v1/traces 204
09:37:39.689 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/8627a3b2-cf84-4f76-92bc-753eaa192e3c","expiresAt":1794220659689}}
09:37:40.851 rpc 32 orchestration.dispatchCommand message.dispatch
09:37:40.942 fetch POST /api/observability/v1/traces
09:37:40.942 fetched POST /api/observability/v1/traces 204
09:37:40.943 in Ping 
09:37:42.465 chunk 12 4658 bytes
09:37:42.468 in Ack 12
09:37:42.672 fetch POST /api/observability/v1/traces
09:37:42.672 fetched POST /api/observability/v1/traces 204
09:37:42.815 chunk 12 4678 bytes
09:37:42.818 in Ack 12
09:37:42.937 exit 30 {"_tag":"Success","value":null}
09:37:43.820 fetch POST /api/observability/v1/traces
09:37:43.820 fetched POST /api/observability/v1/traces 204
09:37:45.180 exit 32 {"_tag":"Success","value":{"sequence":152}}
09:37:45.943 in Ping 
09:37:46.182 fetch POST /api/observability/v1/traces
09:37:46.182 fetched POST /api/observability/v1/traces 204
09:37:47.685 chunk 12 4722 bytes
09:37:47.686 in Ack 12
09:37:47.919 rpc 33 orchestration.dispatchCommand thread.visit
09:37:47.919 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:48.045 fetch POST /api/observability/v1/traces
09:37:48.045 fetched POST /api/observability/v1/traces 204
09:37:49.192 chunk 12 4742 bytes
09:37:49.193 in Ack 12
09:37:49.442 chunk 18 37916 bytes
09:37:49.444 in Ack 18
09:37:49.692 chunk 18 38756 bytes
09:37:49.697 in Ack 18
09:37:49.943 chunk 18 40197 bytes
09:37:49.948 in Ack 18
09:37:50.193 chunk 18 40199 bytes
09:37:50.197 in Ack 18
09:37:50.213 fetch POST /api/observability/v1/traces
09:37:50.213 fetched POST /api/observability/v1/traces 204
09:37:50.692 chunk 12 4742 bytes
09:37:50.693 in Ack 12
09:37:50.710 chunk 18 40724 bytes
09:37:50.713 in Ack 18
09:37:50.945 in Ping 
09:37:51.072 rpc 34 server.reportClientActivity
09:37:51.694 fetch POST /api/observability/v1/traces
09:37:51.694 fetched POST /api/observability/v1/traces 204
09:37:51.964 chunk 18 40724 bytes
09:37:51.967 in Ack 18
09:37:52.970 fetch POST /api/observability/v1/traces
09:37:52.970 fetched POST /api/observability/v1/traces 204
09:37:53.025 exit 34 {"_tag":"Success","value":null}
09:37:54.026 fetch POST /api/observability/v1/traces
09:37:54.026 fetched POST /api/observability/v1/traces 204
09:37:54.650 chunk 12 4742 bytes
09:37:54.651 in Ack 12
09:37:55.652 fetch POST /api/observability/v1/traces
09:37:55.652 fetched POST /api/observability/v1/traces 204
09:37:55.945 in Ping 
09:37:57.759 chunk 12 4742 bytes
09:37:57.760 in Ack 12
09:37:57.919 rpc 35 orchestration.dispatchCommand thread.visit
09:37:57.919 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:37:58.054 fetch POST /api/observability/v1/traces
09:37:58.054 fetched POST /api/observability/v1/traces 204
09:37:58.563 chunk 18 43885 bytes
09:37:58.566 in Ack 18
09:37:58.814 chunk 18 44137 bytes
09:37:58.816 in Ack 18
09:37:59.568 fetch POST /api/observability/v1/traces
09:37:59.568 fetched POST /api/observability/v1/traces 204
09:37:59.822 chunk 18 44089 bytes
09:37:59.827 in Ack 18
09:38:00.074 chunk 12 4742 bytes
09:38:00.078 in Ack 12
09:38:00.828 fetch POST /api/observability/v1/traces
09:38:00.828 fetched POST /api/observability/v1/traces 204
09:38:00.945 in Ping 
09:38:02.149 chunk 12 4742 bytes
09:38:02.151 in Ack 12
09:38:02.962 chunk 18 44089 bytes
09:38:02.965 in Ack 18
09:38:03.154 fetch POST /api/observability/v1/traces
09:38:03.154 fetched POST /api/observability/v1/traces 204
09:38:04.312 chunk 18 45133 bytes
09:38:04.316 in Ack 18
09:38:05.317 fetch POST /api/observability/v1/traces
09:38:05.317 fetched POST /api/observability/v1/traces 204
09:38:05.946 in Ping 
09:38:07.920 rpc 36 orchestration.dispatchCommand thread.visit
09:38:07.920 exit 36 {"_tag":"Success","value":{"sequence":0}}
09:38:08.064 fetch POST /api/observability/v1/traces
09:38:08.064 fetched POST /api/observability/v1/traces 204
09:38:10.946 in Ping 
09:38:15.400 chunk 18 45769 bytes
09:38:15.405 in Ack 18
09:38:15.666 chunk 18 46978 bytes
09:38:15.673 in Ack 18
09:38:15.915 chunk 18 47584 bytes
09:38:15.931 in Ack 18
09:38:15.983 in Ping 
09:38:16.073 rpc 37 server.reportClientActivity
09:38:16.166 chunk 18 47665 bytes
09:38:16.168 in Ack 18
09:38:16.712 fetch POST /api/observability/v1/traces
09:38:16.712 fetched POST /api/observability/v1/traces 204
09:38:18.691 chunk 18 48667 bytes
09:38:18.693 in Ack 18
09:38:19.026 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:19.695 fetch POST /api/observability/v1/traces
09:38:19.695 fetched POST /api/observability/v1/traces 204
09:38:20.942 exit 38 {"_tag":"Success","value":{"sequence":193}}
09:38:20.986 in Ping 
09:38:21.610 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:38:21.944 fetch POST /api/observability/v1/traces
09:38:21.944 fetched POST /api/observability/v1/traces 204
09:38:23.341 chunk 18 48710 bytes
09:38:23.345 in Ack 18
09:38:23.539 exit 37 {"_tag":"Success","value":null}
09:38:23.931 chunk 18 48760 bytes
09:38:23.931 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:23.933 in Ack 18
09:38:24.347 fetch POST /api/observability/v1/traces
09:38:24.347 fetched POST /api/observability/v1/traces 204
09:38:25.990 in Ping 
09:38:27.481 chunk 18 48760 bytes
09:38:27.482 in Ack 18
09:38:27.769 chunk 12 4744 bytes
09:38:27.770 in Ack 12
09:38:27.773 rpc 40 orchestration.dispatchCommand thread.visit
09:38:27.773 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:38:28.489 fetch POST /api/observability/v1/traces
09:38:28.489 fetched POST /api/observability/v1/traces 204
09:38:28.880 chunk 18 51749 bytes
09:38:28.882 in Ack 18
09:38:29.131 chunk 18 51995 bytes
09:38:29.133 in Ack 18
09:38:29.883 fetch POST /api/observability/v1/traces
09:38:29.883 fetched POST /api/observability/v1/traces 204
09:38:30.430 chunk 18 51947 bytes
09:38:30.433 in Ack 18
09:38:30.996 in Ping 
09:38:31.153 chunk 18 55483 bytes
09:38:31.157 in Ack 18
09:38:31.434 fetch POST /api/observability/v1/traces
09:38:31.434 fetched POST /api/observability/v1/traces 204
09:38:33.154 chunk 12 4744 bytes
09:38:33.155 in Ack 12
09:38:34.158 fetch POST /api/observability/v1/traces
09:38:34.158 fetched POST /api/observability/v1/traces 204
09:38:34.409 chunk 18 55483 bytes
09:38:34.413 in Ack 18
09:38:34.660 chunk 18 57600 bytes
09:38:34.663 in Ack 18
09:38:34.673 in Interrupt 29
09:38:34.736 rpc 41 subscribeVcsStatus
09:38:34.913 chunk 18 58278 bytes
09:38:34.915 in Ack 18
09:38:35.118 rpc 42 server.reportClientActivity
09:38:35.735 fetch POST /api/observability/v1/traces
09:38:35.735 fetched POST /api/observability/v1/traces 204
