# story, webapp, warm start, seed 50

Profile story (graded): seed 50's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 56 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.0s |
| Prompt reached the agent | 4.3s |
| Time to first text | 16.0s |
| Whole reply shown | 54.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.6s / 4.6s / 14.6s |
| Longest silence | 16.7s |
| Approval shown after it was asked | 2.1s |
| Bytes on the wire down / up | 443168 / 166845 |
| Bytes the app received / sent | 253911 / 59209 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.6 | 0 | 231 |
| 20 | 5.6 | 397 | 629 |
| 30 | 15.6 | 397 | 1032 |
| 40 | 1.9 | 1077 | 1155 |
| 50 | 4.1 | 1378 | 1524 |
| 53 | 7.1 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 445/901","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217056789,"silences":0,"longestSilenceMs":1081,"longestRecoveredGapMs":1081},"stats":{"sendRate":2000,"deliveredRate":1642.9463171036205,"loss":0,"srttMs":8.603911013459008,"minRttMs":null,"sentPackets":142,"sentWireBytes":40204,"receivedPackets":146,"receivedWireBytes":92400,"lastHeardMs":14871,"unsyncedBytes":1193,"inflightPackets":26,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 130/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 901/901","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217069484,"silences":1,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":2333.584432045421,"deliveredRate":1059.6879981342643,"loss":0.11977777777777776,"srttMs":1821.5573046557613,"minRttMs":2258,"sentPackets":184,"sentWireBytes":50719,"receivedPackets":195,"receivedWireBytes":111554,"lastHeardMs":27566,"unsyncedBytes":0,"inflightPackets":21,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"587/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 498/498","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 205/654","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 901/901 55/543","nodes":"1016/1016 809/809 809/809 1016/1016 516/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217079751,"silences":1,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":2447.909067323427,"minRttMs":2258,"sentPackets":246,"sentWireBytes":65288,"receivedPackets":274,"receivedWireBytes":140338,"lastHeardMs":37833,"unsyncedBytes":167,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 498/498","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 901/901 543/543 451/451","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217089940,"silences":1,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":1000,"deliveredRate":375.1064437028499,"loss":0.5284615384615384,"srttMs":4442.634619470731,"minRttMs":2258,"sentPackets":289,"sentWireBytes":73657,"receivedPackets":406,"receivedWireBytes":183546,"lastHeardMs":48022,"unsyncedBytes":0,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 923/923","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 901/901 544/544 451/451","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1368/1368 959/959 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217100770,"silences":2,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":1000,"deliveredRate":641.3718187461204,"loss":0,"srttMs":4638.05529203689,"minRttMs":2258,"sentPackets":321,"sentWireBytes":83837,"receivedPackets":527,"receivedWireBytes":262217,"lastHeardMs":58852,"unsyncedBytes":167,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1093/1093 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947 1068/1068","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 409/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 544/544 901/901 544/544 451/451 543/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1198/1392 983/983 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1024,"lastDataAt":1794217110914,"silences":2,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":625,"deliveredRate":794.3054282837788,"loss":0.04285714285714287,"srttMs":2841.694715288967,"minRttMs":2202,"sentPackets":339,"sentWireBytes":90247,"receivedPackets":617,"receivedWireBytes":320687,"lastHeardMs":68996,"unsyncedBytes":378,"inflightPackets":39,"duplicateBytes":0}}
- shim trace
09:37:22.925 fetch GET /api/auth/session
09:37:22.931 fetched GET /api/auth/session 200
09:37:23.317 fetch GET /.well-known/t3/environment
09:37:23.318 fetched GET /.well-known/t3/environment 200
09:37:23.337 fetch GET /.well-known/t3/environment
09:37:23.338 fetched GET /.well-known/t3/environment 200
09:37:23.347 rpc 0 subscribeServerConfig
09:37:23.347 chunk 0 38462 bytes
09:37:23.448 in Ack 0
09:37:23.451 rpc 1 subscribeServerLifecycle
09:37:23.452 rpc 2 subscribeServerLifecycle
09:37:23.458 fetch GET /api/auth/session
09:37:23.459 fetched GET /api/auth/session 200
09:37:23.460 rpc 3 subscribeDeviceState
09:37:23.463 rpc 4 preview.reportProfiles
09:37:23.576 rpc 5 subscribeProjectClones
09:37:23.591 fetch GET /api/orchestration/threads/mcp%3A65f99766-b556-4992-acef-00ddd22e408b/bounded
09:37:23.649 rpc 6 server.reportClientActivity
09:37:23.657 fetch GET /api/orchestration/shell
09:37:23.703 rpc 7 server.refreshProviders
09:37:23.813 rpc 8 orchestration.dispatchCommand thread.visit
09:37:23.813 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:23.842 fetched GET /api/orchestration/shell 200
09:37:23.843 rpc 9 subscribeTerminalMetadata
09:37:23.844 rpc 10 subscribeVcsStatus
09:37:23.845 rpc 11 subscribePreviewEvents
09:37:23.851 rpc 12 orchestration.subscribeShell
09:37:23.851 chunk 12 4551 bytes
09:37:23.853 rpc 13 assets.createUrl
09:37:23.854 rpc 14 preview.list
09:37:23.855 rpc 15 projects.readFile
09:37:23.860 in Ack 12
09:37:24.040 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"d9e662f4-db0b-42df-a3fc-e74844c800f8","sessions":[]}}
09:37:24.040 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:24.274 chunk 12 4527 bytes
09:37:24.340 in Ack 12
09:37:24.350 fetch POST /api/observability/v1/traces
09:37:24.350 fetched POST /api/observability/v1/traces 204
09:37:24.513 chunk 1 3815 bytes
09:37:24.514 in Ack 1
09:37:24.733 chunk 2 3815 bytes
09:37:24.734 in Ack 2
09:37:25.059 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:54.201Z","configurableProviders":false,"con
09:37:25.059 chunk 10 1632 bytes
09:37:25.059 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:25.064 in Ack 10
09:37:25.309 chunk 12 4552 bytes
09:37:25.310 in Ack 12
09:37:25.363 fetch POST /api/observability/v1/traces
09:37:25.363 fetched POST /api/observability/v1/traces 204
09:37:25.560 chunk 12 4595 bytes
09:37:25.561 in Ack 12
09:37:25.693 chunk 3 876 bytes
09:37:25.693 exit 4 {"_tag":"Success","value":null}
09:37:25.693 chunk 5 4 bytes
09:37:25.693 exit 6 {"_tag":"Success","value":null}
09:37:25.693 chunk 9 36 bytes
09:37:25.694 in Ack 3
09:37:25.704 rpc 16 preview.reportProfiles
09:37:25.704 in Ack 5
09:37:25.706 in Ack 9
09:37:25.855 exit 16 {"_tag":"Success","value":null}
09:37:25.957 rpc 17 server.reportClientActivity
09:37:26.102 exit 17 {"_tag":"Success","value":null}
09:37:26.314 chunk 12 4592 bytes
09:37:26.315 in Ack 12
09:37:26.562 fetch POST /api/observability/v1/traces
09:37:26.562 fetched POST /api/observability/v1/traces 204
09:37:28.341 in Ping 
09:37:28.438 fetched GET /api/orchestration/threads/mcp%3A65f99766-b556-4992-acef-00ddd22e408b/bounded 200
09:37:28.443 rpc 18 orchestration.subscribeThread
09:37:28.443 chunk 18 8282 bytes
09:37:28.461 in Interrupt 10
09:37:28.465 rpc 19 subscribeVcsStatus
09:37:28.467 in Ack 18
09:37:28.600 chunk 19 1632 bytes
09:37:28.601 in Ack 19
09:37:28.694 chunk 18 11259 bytes
09:37:28.695 in Ack 18
09:37:28.773 rpc 20 server.reportClientActivity
09:37:28.962 chunk 18 12690 bytes
09:37:28.966 in Ack 18
09:37:29.213 chunk 18 16150 bytes
09:37:29.217 in Ack 18
09:37:29.274 exit 20 {"_tag":"Success","value":null}
09:37:29.484 rpc 21 attachments.createUploadUrl
09:37:29.534 fetch POST /api/observability/v1/traces
09:37:29.534 fetched POST /api/observability/v1/traces 204
09:37:29.535 chunk 18 17976 bytes
09:37:29.582 in Ack 18
09:37:29.670 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-d3c72a7c-7b15-4970-9e5c-83f8053fffe0","expiresAt":1794217649622,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:29.671 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWQzYzcyYTdjLTdiMTUtNDk3MC05ZTVjLTgzZjgwNTNmZmZlMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0OTYyMn0.NhulVJdeKoKErqjOQiZgwLVGm6mk1oUFjW9X1XoJIyc
09:37:29.786 chunk 18 20408 bytes
09:37:29.787 in Ack 18
09:37:29.802 in Interrupt 19
09:37:29.805 rpc 22 subscribeVcsStatus
09:37:29.907 chunk 22 1632 bytes
09:37:29.907 in Ack 22
09:37:30.035 chunk 18 21099 bytes
09:37:30.037 in Ack 18
09:37:30.056 rpc 23 server.reportClientActivity
09:37:30.587 fetch POST /api/observability/v1/traces
09:37:30.587 fetched POST /api/observability/v1/traces 204
09:37:31.423 exit 23 {"_tag":"Success","value":null}
09:37:32.425 fetch POST /api/observability/v1/traces
09:37:32.425 fetched POST /api/observability/v1/traces 204
09:37:33.262 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:33.668 in Ping 
09:37:33.733 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:33.736 rpc 25 attachments.delete
09:37:34.262 fetch POST /api/observability/v1/traces
09:37:34.262 fetched POST /api/observability/v1/traces 204
09:37:34.564 chunk 18 23556 bytes
09:37:34.566 in Ack 18
09:37:34.844 exit 25 {"_tag":"Success","value":null}
09:37:35.065 chunk 12 4617 bytes
09:37:35.066 chunk 18 27849 bytes
09:37:35.067 in Ack 12
09:37:35.072 rpc 26 orchestration.dispatchCommand thread.visit
09:37:35.072 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:35.151 in Ack 18
09:37:35.155 rpc 27 assets.createUrl
09:37:35.410 fetch POST /api/observability/v1/traces
09:37:35.410 fetched POST /api/observability/v1/traces 204
09:37:35.572 chunk 12 4614 bytes
09:37:35.573 in Ack 12
09:37:35.577 rpc 28 orchestration.dispatchCommand thread.visit
09:37:35.577 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:37:35.823 chunk 18 27849 bytes
09:37:35.828 in Ack 18
09:37:36.072 chunk 12 4614 bytes
09:37:36.072 in Ack 12
09:37:36.081 chunk 18 30404 bytes
09:37:36.084 in Ack 18
09:37:36.673 chunk 18 33331 bytes
09:37:36.674 fetch POST /api/observability/v1/traces
09:37:36.675 fetched POST /api/observability/v1/traces 204
09:37:36.679 in Ack 18
09:37:36.693 in Interrupt 22
09:37:36.696 rpc 29 subscribeVcsStatus
09:37:36.790 chunk 29 1713 bytes
09:37:36.790 in Ack 29
09:37:36.924 chunk 18 34348 bytes
09:37:36.934 in Ack 18
09:37:36.994 rpc 30 server.reportClientActivity
09:37:37.154 rpc 31 server.refreshProviders
09:37:37.154 exit 31 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:54.201Z","configurableProviders":false,"con
09:37:37.683 fetch POST /api/observability/v1/traces
09:37:37.683 fetched POST /api/observability/v1/traces 204
09:37:38.717 in Ping 
09:37:39.455 rpc 32 orchestration.dispatchCommand message.dispatch
09:37:40.022 fetch POST /api/observability/v1/traces
09:37:40.022 fetched POST /api/observability/v1/traces 204
09:37:42.937 chunk 18 35259 bytes
09:37:42.945 in Ack 18
09:37:43.188 chunk 18 35259 bytes
09:37:43.191 in Ack 18
09:37:43.718 in Ping 
09:37:43.951 fetch POST /api/observability/v1/traces
09:37:43.951 fetched POST /api/observability/v1/traces 204
09:37:44.100 exit 32 {"_tag":"Success","value":{"sequence":152}}
09:37:45.105 fetch POST /api/observability/v1/traces
09:37:45.105 fetched POST /api/observability/v1/traces 204
09:37:48.718 in Ping 
09:37:49.733 chunk 12 4678 bytes
09:37:49.735 in Ack 12
09:37:49.738 rpc 33 orchestration.dispatchCommand thread.visit
09:37:49.738 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:50.038 fetch POST /api/observability/v1/traces
09:37:50.038 fetched POST /api/observability/v1/traces 204
09:37:50.291 chunk 18 35895 bytes
09:37:50.294 in Ack 18
09:37:51.296 fetch POST /api/observability/v1/traces
09:37:51.296 fetched POST /api/observability/v1/traces 204
09:37:51.995 in Interrupt 30
09:37:52.247 rpc 34 server.reportClientActivity
09:37:52.545 exit 30 {"_tag":"Success","value":null}
09:37:52.794 chunk 18 36735 bytes
09:37:52.795 in Ack 18
09:37:52.996 fetch POST /api/observability/v1/traces
09:37:52.996 fetched POST /api/observability/v1/traces 204
09:37:53.302 chunk 18 38756 bytes
09:37:53.315 in Ack 18
09:37:53.718 in Ping 
09:37:54.317 fetch POST /api/observability/v1/traces
09:37:54.317 fetched POST /api/observability/v1/traces 204
09:37:55.065 chunk 18 42169 bytes
09:37:55.069 in Ack 18
09:37:55.382 fetch POST /api/observability/v1/traces
09:37:55.382 fetched POST /api/observability/v1/traces 204
09:37:55.565 chunk 18 42121 bytes
09:37:55.567 in Ack 18
09:37:56.066 chunk 12 4678 bytes
09:37:56.067 in Ack 12
09:37:56.568 fetch POST /api/observability/v1/traces
09:37:56.568 fetched POST /api/observability/v1/traces 204
09:37:58.719 in Ping 
09:37:59.745 rpc 35 orchestration.dispatchCommand thread.visit
09:37:59.745 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:00.047 fetch POST /api/observability/v1/traces
09:38:00.047 fetched POST /api/observability/v1/traces 204
09:38:01.714 chunk 18 42776 bytes
09:38:01.716 in Ack 18
09:38:02.719 fetch POST /api/observability/v1/traces
09:38:02.719 fetched POST /api/observability/v1/traces 204
09:38:03.719 in Ping 
09:38:05.386 fetch POST /api/observability/v1/traces
09:38:05.386 fetched POST /api/observability/v1/traces 204
09:38:05.931 chunk 18 43562 bytes
09:38:05.933 in Ack 18
09:38:06.181 chunk 18 43564 bytes
09:38:06.186 in Ack 18
09:38:06.683 chunk 18 45133 bytes
09:38:06.687 in Ack 18
09:38:06.934 chunk 18 45133 bytes
09:38:06.935 fetch POST /api/observability/v1/traces
09:38:06.935 fetched POST /api/observability/v1/traces 204
09:38:06.940 in Ack 18
09:38:07.249 in Interrupt 34
09:38:07.951 fetch POST /api/observability/v1/traces
09:38:07.951 fetched POST /api/observability/v1/traces 204
09:38:08.691 chunk 12 4678 bytes
09:38:08.692 in Ack 12
09:38:08.719 in Ping 
09:38:09.442 chunk 18 45133 bytes
09:38:09.447 in Ack 18
09:38:09.695 fetch POST /api/observability/v1/traces
09:38:09.695 fetched POST /api/observability/v1/traces 204
09:38:09.902 exit 34 {"_tag":"Success","value":null}
09:38:10.699 chunk 12 5049 bytes
09:38:10.700 in Ack 12
09:38:11.702 fetch POST /api/observability/v1/traces
09:38:11.702 fetched POST /api/observability/v1/traces 204
09:38:11.703 chunk 18 47665 bytes
09:38:11.706 in Ack 18
09:38:12.387 chunk 18 48667 bytes
09:38:12.391 in Ack 18
09:38:12.728 fetch POST /api/observability/v1/traces
09:38:12.728 fetched POST /api/observability/v1/traces 204
09:38:13.286 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:13.591 rpc 37 server.reportClientActivity
09:38:13.720 in Ping 
09:38:14.286 fetch POST /api/observability/v1/traces
09:38:14.286 fetched POST /api/observability/v1/traces 204
09:38:15.388 fetch POST /api/observability/v1/traces
09:38:15.388 fetched POST /api/observability/v1/traces 204
09:38:16.024 chunk 18 52154 bytes
09:38:16.029 in Ack 18
09:38:16.274 chunk 18 54223 bytes
09:38:16.276 in Ack 18
09:38:16.524 chunk 18 54273 bytes
09:38:16.526 in Ack 18
09:38:16.773 chunk 12 5356 bytes
09:38:16.774 in Ack 12
09:38:17.034 chunk 18 55642 bytes
09:38:17.039 fetch POST /api/observability/v1/traces
09:38:17.039 fetched POST /api/observability/v1/traces 204
09:38:17.041 in Ack 18
09:38:17.284 chunk 18 57620 bytes
09:38:17.288 in Ack 18
09:38:18.045 fetch POST /api/observability/v1/traces
09:38:18.045 fetched POST /api/observability/v1/traces 204
09:38:18.723 in Ping 
09:38:20.659 exit 36 {"_tag":"Success","value":{"sequence":187}}
09:38:20.666 rpc 38 orchestration.dispatchCommand thread.visit
09:38:20.666 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:38:21.665 fetch POST /api/observability/v1/traces
09:38:21.665 fetched POST /api/observability/v1/traces 204
09:38:22.009 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:38:23.013 fetch POST /api/observability/v1/traces
09:38:23.014 fetched POST /api/observability/v1/traces 204
09:38:23.728 in Ping 
09:38:23.994 exit 37 {"_tag":"Success","value":null}
09:38:24.225 chunk 12 5051 bytes
09:38:24.226 in Ack 12
09:38:24.996 fetch POST /api/observability/v1/traces
09:38:24.996 fetched POST /api/observability/v1/traces 204
09:38:25.280 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:25.398 chunk 18 57620 bytes
09:38:25.401 in Ack 18
09:38:25.654 chunk 18 57668 bytes
09:38:25.657 in Ack 18
09:38:25.838 rpc 40 orchestration.dispatchCommand runtime-request.respond
09:38:25.905 chunk 18 57759 bytes
09:38:25.908 in Ack 18
09:38:25.931 in Interrupt 29
09:38:25.998 rpc 41 subscribeVcsStatus
09:38:26.249 rpc 42 server.reportClientActivity
09:38:26.283 fetch POST /api/observability/v1/traces
09:38:26.283 fetched POST /api/observability/v1/traces 204
09:38:26.405 chunk 12 5051 bytes
09:38:26.407 chunk 18 60891 bytes
09:38:26.407 in Ack 12
09:38:26.418 in Ack 18
09:38:26.656 chunk 18 60995 bytes
09:38:26.663 in Ack 18
09:38:27.409 fetch POST /api/observability/v1/traces
09:38:27.409 fetched POST /api/observability/v1/traces 204
09:38:27.415 chunk 18 60947 bytes
09:38:27.418 in Ack 18
09:38:28.166 chunk 18 61902 bytes
09:38:28.170 in Ack 18
09:38:28.366 exit 40 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:28.367 rpc 43 orchestration.dispatchCommand thread.visit
09:38:28.367 exit 43 {"_tag":"Success","value":{"sequence":0}}
09:38:28.419 fetch POST /api/observability/v1/traces
09:38:28.419 fetched POST /api/observability/v1/traces 204
09:38:28.731 in Ping 
09:38:28.919 chunk 18 61926 bytes
09:38:28.921 in Ack 18
09:38:29.036 chunk 41 1713 bytes
09:38:29.036 in Ack 41
09:38:29.922 fetch POST /api/observability/v1/traces
09:38:29.922 fetched POST /api/observability/v1/traces 204
09:38:30.927 chunk 12 5051 bytes
09:38:30.929 in Ack 12
09:38:31.178 chunk 12 4746 bytes
09:38:31.184 in Ack 12
09:38:31.429 chunk 18 61926 bytes
09:38:31.432 in Ack 18
09:38:31.678 chunk 12 4746 bytes
09:38:31.680 in Ack 12
09:38:31.929 chunk 18 61950 bytes
09:38:31.930 fetch POST /api/observability/v1/traces
09:38:31.930 fetched POST /api/observability/v1/traces 204
09:38:31.933 in Ack 18
09:38:32.679 chunk 18 62967 bytes
09:38:32.697 in Ack 18
09:38:32.980 fetch POST /api/observability/v1/traces
09:38:32.980 fetched POST /api/observability/v1/traces 204
09:38:33.483 exit 42 {"_tag":"Success","value":null}
09:38:33.484 chunk 12 4685 bytes
09:38:33.485 chunk 18 63017 bytes
09:38:33.924 in Ack 12
09:38:33.937 in Ack 18
09:38:34.048 in Ping 
