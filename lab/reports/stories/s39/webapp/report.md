# story, webapp, warm start, seed 39

Profile story (graded): seed 39's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 66 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.7s |
| Prompt reached the agent | 2.7s |
| Time to first text | 14.5s |
| Whole reply shown | 63.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.2s / 5.2s / 14.2s |
| Longest silence | 25.3s |
| Approval shown after it was asked | 10.6s |
| Bytes on the wire down / up | 516307 / 144821 |
| Bytes the app received / sent | 412350 / 55635 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.2 | 0 | 291 |
| 20 | 7.2 | 397 | 689 |
| 30 | 17.2 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 5.6 | 1077 | 1301 |
| 60 | 7.9 | 1378 | 1596 |
| 63 | 10.9 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217069482,"silences":0,"longestSilenceMs":1002,"longestRecoveredGapMs":1002},"stats":{"sendRate":4500,"deliveredRate":2116.5458587192898,"loss":0,"srttMs":1110.4263085007633,"minRttMs":1,"sentPackets":144,"sentWireBytes":41666,"receivedPackets":184,"receivedWireBytes":120314,"lastHeardMs":17851,"unsyncedBytes":518,"inflightPackets":18,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"470/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217079488,"silences":0,"longestSilenceMs":1002,"longestRecoveredGapMs":1002},"stats":{"sendRate":1124.0035819917857,"deliveredRate":2028.4140139986853,"loss":0,"srttMs":2032.2825247096862,"minRttMs":1,"sentPackets":202,"sentWireBytes":56651,"receivedPackets":277,"receivedWireBytes":173647,"lastHeardMs":27858,"unsyncedBytes":457,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217089675,"silences":1,"longestSilenceMs":2672,"longestRecoveredGapMs":2672},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.09409090909090909,"srttMs":1978.8376151010038,"minRttMs":1843,"sentPackets":244,"sentWireBytes":64581,"receivedPackets":368,"receivedWireBytes":210102,"lastHeardMs":38044,"unsyncedBytes":167,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217099731,"silences":1,"longestSilenceMs":2672,"longestRecoveredGapMs":2672},"stats":{"sendRate":1000,"deliveredRate":574.6044449940123,"loss":0.04932692307692306,"srttMs":3937.5937768116833,"minRttMs":1843,"sentPackets":280,"sentWireBytes":70144,"receivedPackets":565,"receivedWireBytes":286094,"lastHeardMs":48101,"unsyncedBytes":0,"inflightPackets":13,"duplicateBytes":0}}
- sample failed: locator.click: Timeout 2000ms exceeded.
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 923/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1368/1368 959/959 1016/1016","messages":"456/456 544/544 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217111615,"silences":2,"longestSilenceMs":2758,"longestRecoveredGapMs":2758},"stats":{"sendRate":517.8714323819686,"deliveredRate":456.9278739145968,"loss":0.509090909090909,"srttMs":3162.966530153619,"minRttMs":1843,"sentPackets":310,"sentWireBytes":78127,"receivedPackets":662,"receivedWireBytes":341466,"lastHeardMs":59985,"unsyncedBytes":167,"inflightPackets":25,"duplicateBytes":0}}
- Scroll to end shows
- sample failed: locator.click: Timeout 2000ms exceeded.
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1392/1392 983/983 1016/1016","messages":"456/456 544/544 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":779,"lastDataAt":1794217122014,"silences":3,"longestSilenceMs":2758,"longestRecoveredGapMs":2758},"stats":{"sendRate":647.3392904774607,"deliveredRate":525.6282291468422,"loss":0.3863636363636363,"srttMs":2736.560594945943,"minRttMs":1843,"sentPackets":329,"sentWireBytes":84252,"receivedPackets":785,"receivedWireBytes":450187,"lastHeardMs":70384,"unsyncedBytes":758,"inflightPackets":32,"duplicateBytes":0}}
- shim trace
09:37:32.691 fetch GET /api/auth/session
09:37:32.695 fetched GET /api/auth/session 200
09:37:33.270 fetch GET /.well-known/t3/environment
09:37:33.272 fetched GET /.well-known/t3/environment 200
09:37:33.336 fetch GET /.well-known/t3/environment
09:37:33.401 fetched GET /.well-known/t3/environment 200
09:37:33.457 rpc 0 subscribeServerConfig
09:37:33.458 chunk 0 38462 bytes
09:37:33.565 in Ack 0
09:37:33.569 rpc 1 subscribeServerLifecycle
09:37:33.570 rpc 2 subscribeServerLifecycle
09:37:33.578 fetch GET /api/auth/session
09:37:33.580 fetched GET /api/auth/session 200
09:37:33.581 rpc 3 subscribeDeviceState
09:37:33.584 rpc 4 preview.reportProfiles
09:37:33.628 rpc 5 server.reportClientActivity
09:37:33.712 rpc 6 subscribeProjectClones
09:37:33.774 fetch GET /api/orchestration/threads/mcp%3A7742805e-86e0-45ba-8735-5304d226dbd5/bounded
09:37:33.784 fetch GET /api/orchestration/shell
09:37:33.785 fetched GET /api/orchestration/shell 200
09:37:33.790 rpc 7 orchestration.subscribeShell
09:37:33.790 chunk 7 4551 bytes
09:37:33.848 rpc 8 server.refreshProviders
09:37:34.077 rpc 9 orchestration.dispatchCommand thread.visit
09:37:34.077 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:34.142 rpc 10 subscribeTerminalMetadata
09:37:34.143 rpc 11 subscribeVcsStatus
09:37:34.144 rpc 12 subscribePreviewEvents
09:37:34.147 rpc 13 assets.createUrl
09:37:34.149 rpc 14 preview.list
09:37:34.150 rpc 15 projects.readFile
09:37:34.160 in Ack 7
09:37:34.160 chunk 7 4527 bytes
09:37:34.193 in Ack 7
09:37:34.350 fetch POST /api/observability/v1/traces
09:37:34.350 fetched POST /api/observability/v1/traces 204
09:37:34.561 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"fb8fabce-f592-4047-8575-a5c7bea97842","sessions":[]}}
09:37:34.561 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:34.788 chunk 1 3815 bytes
09:37:34.788 in Ack 1
09:37:35.035 chunk 2 3815 bytes
09:37:35.035 in Ack 2
09:37:35.247 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:04.403Z","configurableProviders":false,"con
09:37:35.247 chunk 11 1632 bytes
09:37:35.247 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:35.254 in Ack 11
09:37:35.411 fetch POST /api/observability/v1/traces
09:37:35.411 fetched POST /api/observability/v1/traces 204
09:37:35.941 chunk 7 4549 bytes
09:37:35.942 in Ack 7
09:37:36.192 chunk 7 4592 bytes
09:37:36.193 in Ack 7
09:37:36.273 chunk 3 876 bytes
09:37:36.273 exit 4 {"_tag":"Success","value":null}
09:37:36.273 exit 5 {"_tag":"Success","value":null}
09:37:36.273 chunk 6 4 bytes
09:37:36.273 chunk 10 36 bytes
09:37:36.274 in Ack 3
09:37:36.283 rpc 16 preview.reportProfiles
09:37:36.284 in Ack 6
09:37:36.284 in Ack 10
09:37:36.352 exit 16 {"_tag":"Success","value":null}
09:37:36.535 rpc 17 server.reportClientActivity
09:37:36.585 exit 17 {"_tag":"Success","value":null}
09:37:36.944 fetch POST /api/observability/v1/traces
09:37:36.944 fetched POST /api/observability/v1/traces 204
09:37:38.411 in Ping 
09:37:38.558 fetched GET /api/orchestration/threads/mcp%3A7742805e-86e0-45ba-8735-5304d226dbd5/bounded 200
09:37:38.564 rpc 18 orchestration.subscribeThread
09:37:38.565 chunk 18 8282 bytes
09:37:38.583 in Interrupt 11
09:37:38.587 rpc 19 subscribeVcsStatus
09:37:38.588 in Ack 18
09:37:38.726 chunk 19 1632 bytes
09:37:38.727 in Ack 19
09:37:38.815 chunk 18 8322 bytes
09:37:38.816 in Ack 18
09:37:38.873 rpc 20 server.reportClientActivity
09:37:39.068 chunk 18 12690 bytes
09:37:39.076 in Ack 18
09:37:39.320 chunk 18 13497 bytes
09:37:39.322 in Ack 18
09:37:39.347 exit 20 {"_tag":"Success","value":null}
09:37:39.560 fetch POST /api/observability/v1/traces
09:37:39.560 fetched POST /api/observability/v1/traces 204
09:37:39.571 chunk 18 17976 bytes
09:37:39.575 in Ack 18
09:37:39.884 rpc 21 attachments.createUploadUrl
09:37:39.935 chunk 18 19796 bytes
09:37:39.950 in Ack 18
09:37:39.963 in Interrupt 19
09:37:39.965 rpc 22 subscribeVcsStatus
09:37:40.143 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-76754d3e-4129-45a8-a65e-879006d9e305","expiresAt":1794217660083,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:40.143 chunk 22 1632 bytes
09:37:40.146 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTc2NzU0ZDNlLTQxMjktNDVhOC1hNjVlLTg3OTAwNmQ5ZTMwNSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY2MDA4M30.LfPZvRaZlo8XxxhLrhpIeEScE1LSOJ6n8q0SL7F02w0
09:37:40.146 in Ack 22
09:37:40.186 chunk 18 21099 bytes
09:37:40.187 in Ack 18
09:37:40.217 rpc 23 server.reportClientActivity
09:37:40.576 fetch POST /api/observability/v1/traces
09:37:40.576 fetched POST /api/observability/v1/traces 204
09:37:42.078 exit 23 {"_tag":"Success","value":null}
09:37:43.080 fetch POST /api/observability/v1/traces
09:37:43.080 fetched POST /api/observability/v1/traces 204
09:37:43.411 in Ping 
09:37:43.751 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:44.272 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:44.276 rpc 25 attachments.delete
09:37:44.524 chunk 7 4678 bytes
09:37:44.524 chunk 18 24528 bytes
09:37:44.525 in Ack 7
09:37:44.527 rpc 26 orchestration.dispatchCommand thread.visit
09:37:44.527 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:44.571 in Ack 18
09:37:44.575 rpc 27 assets.createUrl
09:37:44.629 exit 25 {"_tag":"Success","value":null}
09:37:44.749 fetch POST /api/observability/v1/traces
09:37:44.750 fetched POST /api/observability/v1/traces 204
09:37:44.774 chunk 18 25368 bytes
09:37:44.776 in Ack 18
09:37:45.024 chunk 7 4678 bytes
09:37:45.025 in Ack 7
09:37:45.037 chunk 18 27825 bytes
09:37:45.039 in Ack 18
09:37:45.276 chunk 7 4617 bytes
09:37:45.277 in Ack 7
09:37:45.288 chunk 18 27849 bytes
09:37:45.293 in Ack 18
09:37:45.538 chunk 18 27981 bytes
09:37:45.540 in Ack 18
09:37:45.777 fetch POST /api/observability/v1/traces
09:37:45.777 fetched POST /api/observability/v1/traces 204
09:37:45.779 chunk 7 4614 bytes
09:37:45.780 in Ack 7
09:37:45.785 rpc 28 orchestration.dispatchCommand thread.visit
09:37:45.785 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:37:46.029 chunk 7 4614 bytes
09:37:46.030 in Ack 7
09:37:46.045 chunk 18 31214 bytes
09:37:46.048 in Ack 18
09:37:46.348 chunk 18 33331 bytes
09:37:46.351 in Ack 18
09:37:46.364 in Interrupt 22
09:37:46.368 rpc 29 subscribeVcsStatus
09:37:46.703 chunk 18 34348 bytes
09:37:46.705 rpc 30 server.reportClientActivity
09:37:46.707 in Ack 18
09:37:46.776 chunk 29 1713 bytes
09:37:46.776 in Ack 29
09:37:46.784 fetch POST /api/observability/v1/traces
09:37:46.784 fetched POST /api/observability/v1/traces 204
09:37:46.927 rpc 31 server.refreshProviders
09:37:46.927 exit 31 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:04.403Z","configurableProviders":false,"con
09:37:47.031 chunk 18 35259 bytes
09:37:47.039 in Ack 18
09:37:47.753 exit 30 {"_tag":"Success","value":null}
09:37:47.975 fetch POST /api/observability/v1/traces
09:37:47.975 fetched POST /api/observability/v1/traces 204
09:37:48.453 in Ping 
09:37:49.133 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/97e4d3f0-405b-4b01-a7e4-9408a63a7faa","expiresAt":1794220669133}}
09:37:49.408 rpc 32 orchestration.dispatchCommand message.dispatch
09:37:50.146 fetch POST /api/observability/v1/traces
09:37:50.146 fetched POST /api/observability/v1/traces 204
09:37:51.858 exit 32 {"_tag":"Success","value":{"sequence":152}}
09:37:52.109 chunk 18 35843 bytes
09:37:52.112 in Ack 18
09:37:52.362 chunk 18 37187 bytes
09:37:52.368 in Ack 18
09:37:52.612 chunk 18 37951 bytes
09:37:52.615 in Ack 18
09:37:52.860 fetch POST /api/observability/v1/traces
09:37:52.860 fetched POST /api/observability/v1/traces 204
09:37:52.862 chunk 18 37951 bytes
09:37:52.865 in Ack 18
09:37:53.116 chunk 18 38003 bytes
09:37:53.125 in Ack 18
09:37:53.368 chunk 7 4678 bytes
09:37:53.369 chunk 18 38679 bytes
09:37:53.369 in Ack 7
09:37:53.381 in Ack 18
09:37:53.453 in Ping 
09:37:53.619 chunk 18 38701 bytes
09:37:53.621 in Ack 18
09:37:53.868 fetch POST /api/observability/v1/traces
09:37:53.868 fetched POST /api/observability/v1/traces 204
09:37:53.872 chunk 18 40724 bytes
09:37:53.874 in Ack 18
09:37:54.881 fetch POST /api/observability/v1/traces
09:37:54.881 fetched POST /api/observability/v1/traces 204
09:37:55.127 chunk 7 4742 bytes
09:37:55.128 in Ack 7
09:37:55.785 rpc 33 orchestration.dispatchCommand thread.visit
09:37:55.785 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:56.129 fetch POST /api/observability/v1/traces
09:37:56.129 fetched POST /api/observability/v1/traces 204
09:37:56.681 chunk 7 4742 bytes
09:37:56.683 in Ack 7
09:37:57.686 fetch POST /api/observability/v1/traces
09:37:57.686 fetched POST /api/observability/v1/traces 204
09:37:58.453 in Ping 
09:37:58.523 rpc 34 server.reportClientActivity
09:37:59.189 chunk 7 4742 bytes
09:37:59.189 in Ack 7
09:37:59.524 fetch POST /api/observability/v1/traces
09:37:59.524 fetched POST /api/observability/v1/traces 204
09:38:00.447 chunk 18 40724 bytes
09:38:00.451 in Ack 18
09:38:01.067 exit 34 {"_tag":"Success","value":null}
09:38:01.452 fetch POST /api/observability/v1/traces
09:38:01.452 fetched POST /api/observability/v1/traces 204
09:38:02.040 chunk 7 4742 bytes
09:38:02.041 in Ack 7
09:38:03.042 fetch POST /api/observability/v1/traces
09:38:03.042 fetched POST /api/observability/v1/traces 204
09:38:03.073 chunk 18 43289 bytes
09:38:03.076 in Ack 18
09:38:03.322 chunk 18 43573 bytes
09:38:03.324 in Ack 18
09:38:03.454 in Ping 
09:38:03.571 chunk 18 44137 bytes
09:38:03.573 in Ack 18
09:38:04.078 fetch POST /api/observability/v1/traces
09:38:04.078 fetched POST /api/observability/v1/traces 204
09:38:05.438 fetch POST /api/observability/v1/traces
09:38:05.438 fetched POST /api/observability/v1/traces 204
09:38:07.535 chunk 7 4742 bytes
09:38:07.536 in Ack 7
09:38:07.538 rpc 35 orchestration.dispatchCommand thread.visit
09:38:07.538 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:08.038 chunk 18 45181 bytes
09:38:08.040 in Ack 18
09:38:08.454 in Ping 
09:38:08.538 fetch POST /api/observability/v1/traces
09:38:08.538 fetched POST /api/observability/v1/traces 204
09:38:09.587 chunk 18 45133 bytes
09:38:09.589 in Ack 18
09:38:10.590 fetch POST /api/observability/v1/traces
09:38:10.590 fetched POST /api/observability/v1/traces 204
09:38:11.455 chunk 7 4742 bytes
09:38:11.456 in Ack 7
09:38:12.457 fetch POST /api/observability/v1/traces
09:38:12.457 fetched POST /api/observability/v1/traces 204
09:38:13.455 in Ping 
09:38:13.596 chunk 18 45133 bytes
09:38:13.598 in Ack 18
09:38:14.600 fetch POST /api/observability/v1/traces
09:38:14.600 fetched POST /api/observability/v1/traces 204
09:38:15.237 chunk 7 4742 bytes
09:38:15.241 in Ack 7
09:38:16.243 fetch POST /api/observability/v1/traces
09:38:16.243 fetched POST /api/observability/v1/traces 204
09:38:18.455 in Ping 
09:38:18.866 chunk 7 5049 bytes
09:38:18.871 in Ack 7
09:38:19.879 fetch POST /api/observability/v1/traces
09:38:19.879 fetched POST /api/observability/v1/traces 204
09:38:19.989 chunk 18 47665 bytes
09:38:19.993 in Ack 18
09:38:20.994 fetch POST /api/observability/v1/traces
09:38:20.994 fetched POST /api/observability/v1/traces 204
09:38:23.456 in Ping 
09:38:23.527 rpc 36 server.reportClientActivity
09:38:24.526 fetch POST /api/observability/v1/traces
09:38:24.526 fetched POST /api/observability/v1/traces 204
09:38:28.460 in Ping 
09:38:29.184 chunk 18 48667 bytes
09:38:29.190 in Ack 18
09:38:29.449 chunk 18 54223 bytes
09:38:29.452 in Ack 18
09:38:29.702 chunk 18 54273 bytes
09:38:29.705 in Ack 18
09:38:29.952 chunk 7 5356 bytes
09:38:29.953 chunk 18 54274 bytes
09:38:29.954 in Ack 7
09:38:29.959 rpc 37 orchestration.dispatchCommand thread.visit
09:38:29.959 exit 37 {"_tag":"Success","value":{"sequence":0}}
09:38:29.974 in Ack 18
09:38:30.191 fetch POST /api/observability/v1/traces
09:38:30.192 fetched POST /api/observability/v1/traces 204
09:38:30.203 chunk 18 55291 bytes
09:38:30.205 in Ack 18
09:38:30.703 chunk 18 57620 bytes
09:38:30.705 in Ack 18
09:38:31.207 fetch POST /api/observability/v1/traces
09:38:31.207 fetched POST /api/observability/v1/traces 204
09:38:32.318 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:32.415 exit 36 {"_tag":"Success","value":null}
09:38:32.448 chunk 18 57620 bytes
09:38:32.451 in Ack 18
09:38:32.700 chunk 7 5356 bytes
09:38:32.701 in Ack 7
09:38:32.951 chunk 7 5356 bytes
09:38:32.952 in Ack 7
09:38:33.320 fetch POST /api/observability/v1/traces
09:38:33.320 fetched POST /api/observability/v1/traces 204
09:38:33.464 in Ping 
09:38:34.744 exit 38 {"_tag":"Success","value":{"sequence":188}}
09:38:34.941 chunk 18 57663 bytes
09:38:34.944 in Ack 18
09:38:35.189 chunk 7 5051 bytes
09:38:35.190 in Ack 7
09:38:35.449 chunk 18 57783 bytes
09:38:35.456 in Ack 18
09:38:35.480 in Interrupt 29
09:38:35.486 rpc 39 subscribeVcsStatus
09:38:35.488 fetch POST /api/observability/v1/traces
09:38:35.488 fetched POST /api/observability/v1/traces 204
09:38:35.699 chunk 18 57807 bytes
09:38:35.702 in Ack 18
09:38:35.738 rpc 40 server.reportClientActivity
09:38:36.704 fetch POST /api/observability/v1/traces
09:38:36.704 fetched POST /api/observability/v1/traces 204
09:38:37.840 chunk 7 4746 bytes
09:38:37.841 in Ack 7
09:38:38.468 in Ping 
09:38:38.842 fetch POST /api/observability/v1/traces
09:38:38.842 fetched POST /api/observability/v1/traces 204
09:38:39.959 rpc 41 orchestration.dispatchCommand thread.visit
09:38:39.959 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:38:40.959 fetch POST /api/observability/v1/traces
09:38:40.959 fetched POST /api/observability/v1/traces 204
09:38:42.726 chunk 39 1713 bytes
09:38:42.726 in Ack 39
09:38:43.472 in Ping 
09:38:44.828 chunk 18 61043 bytes
09:38:44.831 in Ack 18
09:38:45.092 chunk 18 60995 bytes
09:38:45.113 in Ack 18
09:38:45.342 chunk 7 4746 bytes
09:38:45.343 chunk 18 61950 bytes
09:38:45.343 in Ack 7
09:38:45.347 in Ack 18
09:38:45.462 fetch POST /api/observability/v1/traces
09:38:45.462 fetched POST /api/observability/v1/traces 204
09:38:48.478 in Ping 
09:38:49.961 rpc 42 orchestration.dispatchCommand thread.visit
09:38:49.961 exit 42 {"_tag":"Success","value":{"sequence":0}}
09:38:50.739 in Interrupt 40
09:38:50.961 fetch POST /api/observability/v1/traces
09:38:50.961 fetched POST /api/observability/v1/traces 204
09:38:50.996 rpc 43 server.reportClientActivity
09:38:51.998 fetch POST /api/observability/v1/traces
09:38:51.998 fetched POST /api/observability/v1/traces 204
09:38:52.401 chunk 18 62967 bytes
09:38:52.405 in Ack 18
09:38:52.669 chunk 7 4685 bytes
09:38:52.670 chunk 18 63041 bytes
09:38:52.728 in Ack 7
09:38:52.746 in Ack 18
09:38:52.930 chunk 7 4682 bytes
09:38:52.931 chunk 18 63173 bytes
09:38:52.931 in Ack 7
09:38:52.933 rpc 44 orchestration.dispatchCommand thread.visit
09:38:52.933 exit 44 {"_tag":"Success","value":{"sequence":0}}
09:38:52.946 in Ack 18
09:38:53.462 fetch POST /api/observability/v1/traces
09:38:53.463 fetched POST /api/observability/v1/traces 204
