# story, webapp, warm start, seed 46

Profile story (graded): seed 46's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 74 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.7s |
| Prompt reached the agent | 2.5s |
| Time to first text | 14.7s |
| Whole reply shown | 72.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 26.4s / 9.4s / 22.4s |
| Longest silence | 29.3s |
| Approval shown after it was asked | 10.6s |
| Bytes on the wire down / up | 235944 / 115446 |
| Bytes the app received / sent | 174509 / 61402 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 303 |
| 20 | 7.4 | 397 | 701 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 4.0 | 1077 | 1242 |
| 60 | 14.0 | 1077 | 1596 |
| 70 | 14.9 | 1429 | 1596 |
| 72 | 16.9 | 1429 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217076456,"silences":0,"longestSilenceMs":974,"longestRecoveredGapMs":974},"stats":{"sendRate":6998.806215149262,"deliveredRate":3284.930367459512,"loss":0,"srttMs":38.98455700696174,"minRttMs":1,"sentPackets":141,"sentWireBytes":38979,"receivedPackets":122,"receivedWireBytes":97699,"lastHeardMs":14578,"unsyncedBytes":736,"inflightPackets":12,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 20/73","attempts":"659/659 659/659","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 370/451","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 15/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"90/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217086382,"silences":0,"longestSilenceMs":974,"longestRecoveredGapMs":974},"stats":{"sendRate":1962.9657386035242,"deliveredRate":2556.88870566818,"loss":0.02307692307692306,"srttMs":4072.6115734828636,"minRttMs":1,"sentPackets":181,"sentWireBytes":52661,"receivedPackets":186,"receivedWireBytes":142710,"lastHeardMs":24505,"unsyncedBytes":167,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217096358,"silences":0,"longestSilenceMs":974,"longestRecoveredGapMs":974},"stats":{"sendRate":828.1261709733618,"deliveredRate":0,"loss":0.048637397975708485,"srttMs":1742.8719966647054,"minRttMs":1,"sentPackets":238,"sentWireBytes":65255,"receivedPackets":279,"receivedWireBytes":183584,"lastHeardMs":34480,"unsyncedBytes":0,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217105549,"silences":1,"longestSilenceMs":2977,"longestRecoveredGapMs":2977},"stats":{"sendRate":828.1261709733618,"deliveredRate":1117.9407486669627,"loss":0.044545454545454555,"srttMs":1572.4322910136614,"minRttMs":1,"sentPackets":271,"sentWireBytes":69942,"receivedPackets":355,"receivedWireBytes":210865,"lastHeardMs":43672,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 923/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217116769,"silences":4,"longestSilenceMs":3258,"longestRecoveredGapMs":3258},"stats":{"sendRate":828.1261709733618,"deliveredRate":0,"loss":0.044545454545454555,"srttMs":1572.4322910136614,"minRttMs":1,"sentPackets":290,"sentWireBytes":74884,"receivedPackets":388,"receivedWireBytes":228734,"lastHeardMs":54892,"unsyncedBytes":855,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 114/1566 923/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217125427,"silences":4,"longestSilenceMs":3258,"longestRecoveredGapMs":3258},"stats":{"sendRate":828.1261709733618,"deliveredRate":204.92339924053948,"loss":0.5652173913043479,"srttMs":1976.8175911158228,"minRttMs":1560,"sentPackets":313,"sentWireBytes":82677,"receivedPackets":431,"receivedWireBytes":252815,"lastHeardMs":63550,"unsyncedBytes":167,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 114/1566 923/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1187,"lastDataAt":1794217125427,"silences":4,"longestSilenceMs":3258,"longestRecoveredGapMs":3258},"stats":{"sendRate":828.1261709733618,"deliveredRate":204.92339924053948,"loss":0.5652173913043479,"srttMs":1976.8175911158228,"minRttMs":1560,"sentPackets":338,"sentWireBytes":89977,"receivedPackets":431,"receivedWireBytes":252815,"lastHeardMs":63550,"unsyncedBytes":624,"inflightPackets":55,"duplicateBytes":0}}
- shim trace
09:37:42.949 fetch GET /api/auth/session
09:37:42.953 fetched GET /api/auth/session 200
09:37:43.179 fetch GET /.well-known/t3/environment
09:37:43.182 fetched GET /.well-known/t3/environment 200
09:37:43.205 fetch GET /.well-known/t3/environment
09:37:43.206 fetched GET /.well-known/t3/environment 200
09:37:43.212 rpc 0 subscribeServerConfig
09:37:43.213 chunk 0 38462 bytes
09:37:43.342 in Ack 0
09:37:43.348 rpc 1 subscribeServerLifecycle
09:37:43.349 rpc 2 subscribeServerLifecycle
09:37:43.354 fetch GET /api/auth/session
09:37:43.355 fetched GET /api/auth/session 200
09:37:43.356 rpc 3 subscribeDeviceState
09:37:43.359 rpc 4 preview.reportProfiles
09:37:43.494 rpc 5 subscribeProjectClones
09:37:43.501 rpc 6 server.reportClientActivity
09:37:43.514 fetch GET /api/orchestration/shell
09:37:43.544 rpc 7 server.refreshProviders
09:37:43.635 rpc 8 orchestration.dispatchCommand thread.visit
09:37:43.635 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:43.654 fetched GET /api/orchestration/shell 200
09:37:43.655 rpc 9 subscribeTerminalMetadata
09:37:43.657 rpc 10 subscribeVcsStatus
09:37:43.658 rpc 11 subscribePreviewEvents
09:37:43.661 rpc 12 orchestration.subscribeShell
09:37:43.661 chunk 12 2736 bytes
09:37:43.663 rpc 13 assets.createUrl
09:37:43.663 rpc 14 preview.list
09:37:43.664 rpc 15 projects.readFile
09:37:43.674 fetch GET /api/orchestration/threads/mcp%3A2dd2843c-055b-4265-849e-bd5e2f6d7192/bounded
09:37:43.681 in Ack 12
09:37:43.920 chunk 12 2712 bytes
09:37:44.092 in Ack 12
09:37:44.176 fetch POST /api/observability/v1/traces
09:37:44.176 fetched POST /api/observability/v1/traces 204
09:37:44.253 chunk 1 3815 bytes
09:37:44.253 in Ack 1
09:37:44.503 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"29fdf7f5-6fca-47ae-bccb-bc493bc96b1e","sessions":[]}}
09:37:44.503 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:44.503 chunk 2 3815 bytes
09:37:44.516 in Ack 2
09:37:44.731 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:21.485Z","configurableProviders":false,"con
09:37:44.731 chunk 10 1632 bytes
09:37:44.739 in Ack 10
09:37:44.930 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:45.180 chunk 12 4635 bytes
09:37:45.180 in Ack 12
09:37:45.423 chunk 3 876 bytes
09:37:45.424 in Ack 3
09:37:45.505 fetch POST /api/observability/v1/traces
09:37:45.505 fetched POST /api/observability/v1/traces 204
09:37:45.522 exit 4 {"_tag":"Success","value":null}
09:37:45.522 chunk 5 4 bytes
09:37:45.522 exit 6 {"_tag":"Success","value":null}
09:37:45.522 chunk 9 36 bytes
09:37:45.523 rpc 16 preview.reportProfiles
09:37:45.523 in Ack 5
09:37:45.524 in Ack 9
09:37:45.609 exit 16 {"_tag":"Success","value":null}
09:37:45.775 rpc 17 server.reportClientActivity
09:37:45.856 exit 17 {"_tag":"Success","value":null}
09:37:46.523 fetch POST /api/observability/v1/traces
09:37:46.523 fetched POST /api/observability/v1/traces 204
09:37:47.001 fetched GET /api/orchestration/threads/mcp%3A2dd2843c-055b-4265-849e-bd5e2f6d7192/bounded 200
09:37:47.006 rpc 18 orchestration.subscribeThread
09:37:47.007 chunk 18 8283 bytes
09:37:47.021 in Interrupt 10
09:37:47.024 rpc 19 subscribeVcsStatus
09:37:47.026 in Ack 18
09:37:47.170 chunk 19 1632 bytes
09:37:47.170 in Ack 19
09:37:47.258 chunk 18 8323 bytes
09:37:47.259 in Ack 18
09:37:47.308 rpc 20 server.reportClientActivity
09:37:47.321 exit 20 {"_tag":"Success","value":null}
09:37:47.513 chunk 18 12691 bytes
09:37:47.515 in Ack 18
09:37:47.763 chunk 18 13498 bytes
09:37:47.765 in Ack 18
09:37:48.016 fetch POST /api/observability/v1/traces
09:37:48.016 fetched POST /api/observability/v1/traces 204
09:37:48.109 rpc 21 attachments.createUploadUrl
09:37:48.200 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-31732bdb-e24f-4dcf-9037-2ff54fc3f520","expiresAt":1794217668194,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:48.202 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTMxNzMyYmRiLWUyNGYtNGRjZi05MDM3LTJmZjU0ZmMzZjUyMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY2ODE5NH0.a7yMxbL68H5przM-aACTc0AaULz5IN61YAvUkcXo_uk
09:37:48.209 in Ping 
09:37:48.267 chunk 18 17167 bytes
09:37:48.269 in Ack 18
09:37:48.517 chunk 18 19797 bytes
09:37:48.518 in Ack 18
09:37:48.530 in Interrupt 19
09:37:48.533 rpc 22 subscribeVcsStatus
09:37:48.600 chunk 22 1632 bytes
09:37:48.601 in Ack 22
09:37:48.767 chunk 18 21100 bytes
09:37:48.768 in Ack 18
09:37:48.783 rpc 23 server.reportClientActivity
09:37:49.110 fetch POST /api/observability/v1/traces
09:37:49.110 fetched POST /api/observability/v1/traces 204
09:37:50.170 exit 23 {"_tag":"Success","value":null}
09:37:51.258 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:51.497 fetch POST /api/observability/v1/traces
09:37:51.497 fetched POST /api/observability/v1/traces 204
09:37:51.559 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:51.561 rpc 25 attachments.delete
09:37:51.639 exit 25 {"_tag":"Success","value":null}
09:37:51.748 chunk 18 21736 bytes
09:37:51.748 chunk 12 4721 bytes
09:37:51.749 in Ack 18
09:37:51.761 in Ack 12
09:37:51.999 chunk 18 26810 bytes
09:37:52.001 in Ack 18
09:37:52.004 rpc 26 assets.createUrl
09:37:52.249 chunk 18 30180 bytes
09:37:52.251 in Ack 18
09:37:52.499 chunk 12 4660 bytes
09:37:52.499 chunk 18 30336 bytes
09:37:52.500 in Ack 12
09:37:52.529 in Ack 18
09:37:52.623 fetch POST /api/observability/v1/traces
09:37:52.623 fetched POST /api/observability/v1/traces 204
09:37:52.749 chunk 12 4657 bytes
09:37:52.749 chunk 18 32759 bytes
09:37:52.750 in Ack 12
09:37:52.755 rpc 27 orchestration.dispatchCommand thread.visit
09:37:52.755 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:52.774 in Ack 18
09:37:52.998 chunk 12 4657 bytes
09:37:52.999 chunk 18 32783 bytes
09:37:53.000 in Ack 12
09:37:53.008 in Ack 18
09:37:53.209 in Ping 
09:37:53.250 chunk 18 35260 bytes
09:37:53.251 in Ack 18
09:37:53.261 in Interrupt 22
09:37:53.263 rpc 28 subscribeVcsStatus
09:37:53.324 chunk 28 1713 bytes
09:37:53.324 in Ack 28
09:37:53.515 rpc 29 server.reportClientActivity
09:37:53.883 exit 29 {"_tag":"Success","value":null}
09:37:53.885 fetch POST /api/observability/v1/traces
09:37:53.885 fetched POST /api/observability/v1/traces 204
09:37:53.927 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/571e516d-2b2e-4ea4-9558-f5300561ca4d","expiresAt":1794220673927}}
09:37:54.775 rpc 30 server.refreshProviders
09:37:54.775 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:21.485Z","configurableProviders":false,"con
09:37:54.904 fetch POST /api/observability/v1/traces
09:37:54.905 fetched POST /api/observability/v1/traces 204
09:37:56.407 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:57.407 fetch POST /api/observability/v1/traces
09:37:57.407 fetched POST /api/observability/v1/traces 204
09:37:58.209 in Ping 
09:37:58.544 chunk 12 4721 bytes
09:37:58.549 in Ack 12
09:37:58.578 fetch POST /api/observability/v1/traces
09:37:58.578 fetched POST /api/observability/v1/traces 204
09:38:01.701 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:38:01.955 chunk 18 35915 bytes
09:38:01.957 in Ack 18
09:38:02.205 chunk 18 35917 bytes
09:38:02.208 in Ack 18
09:38:02.702 fetch POST /api/observability/v1/traces
09:38:02.702 fetched POST /api/observability/v1/traces 204
09:38:03.210 in Ping 
09:38:03.779 chunk 18 36757 bytes
09:38:03.781 in Ack 18
09:38:04.587 chunk 12 4785 bytes
09:38:04.588 in Ack 12
09:38:04.590 rpc 32 orchestration.dispatchCommand thread.visit
09:38:04.590 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:38:04.782 fetch POST /api/observability/v1/traces
09:38:04.782 fetched POST /api/observability/v1/traces 204
09:38:08.210 in Ping 
09:38:08.436 rpc 33 server.reportClientActivity
09:38:08.575 chunk 18 36757 bytes
09:38:08.577 in Ack 18
09:38:08.614 fetch POST /api/observability/v1/traces
09:38:08.614 fetched POST /api/observability/v1/traces 204
09:38:09.074 chunk 12 4785 bytes
09:38:09.075 chunk 18 38778 bytes
09:38:09.076 in Ack 12
09:38:09.088 in Ack 18
09:38:09.327 chunk 18 38778 bytes
09:38:09.329 in Ack 18
09:38:10.077 fetch POST /api/observability/v1/traces
09:38:10.077 fetched POST /api/observability/v1/traces 204
09:38:10.328 chunk 18 41619 bytes
09:38:10.330 in Ack 18
09:38:10.578 chunk 18 42069 bytes
09:38:10.580 in Ack 18
09:38:10.829 chunk 18 42191 bytes
09:38:10.831 in Ack 18
09:38:11.334 fetch POST /api/observability/v1/traces
09:38:11.334 fetched POST /api/observability/v1/traces 204
09:38:12.585 chunk 18 42143 bytes
09:38:12.587 in Ack 18
09:38:13.210 in Ping 
09:38:13.588 fetch POST /api/observability/v1/traces
09:38:13.588 fetched POST /api/observability/v1/traces 204
09:38:14.582 chunk 12 4785 bytes
09:38:14.583 in Ack 12
09:38:14.595 rpc 34 orchestration.dispatchCommand thread.visit
09:38:14.595 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:38:15.096 chunk 18 42779 bytes
09:38:15.098 in Ack 18
09:38:15.324 fetch POST /api/observability/v1/traces
09:38:15.324 fetched POST /api/observability/v1/traces 204
09:38:15.596 chunk 18 43823 bytes
09:38:15.599 in Ack 18
09:38:15.847 chunk 18 45134 bytes
09:38:15.848 in Ack 18
09:38:16.600 fetch POST /api/observability/v1/traces
09:38:16.600 fetched POST /api/observability/v1/traces 204
09:38:18.096 chunk 12 4785 bytes
09:38:18.097 in Ack 12
09:38:18.211 in Ping 
09:38:18.593 chunk 12 4785 bytes
09:38:18.594 in Ack 12
09:38:18.607 chunk 18 45134 bytes
09:38:18.609 in Ack 18
09:38:18.967 exit 33 {"_tag":"Success","value":null}
09:38:19.098 fetch POST /api/observability/v1/traces
09:38:19.098 fetched POST /api/observability/v1/traces 204
09:38:19.845 chunk 12 4785 bytes
09:38:19.846 in Ack 12
09:38:20.847 fetch POST /api/observability/v1/traces
09:38:20.847 fetched POST /api/observability/v1/traces 204
09:38:23.211 in Ping 
09:38:25.331 fetch POST /api/observability/v1/traces
09:38:25.331 fetched POST /api/observability/v1/traces 204
09:38:28.212 in Ping 
09:38:33.215 in Ping 
09:38:33.441 rpc 35 server.reportClientActivity
09:38:34.439 fetch POST /api/observability/v1/traces
09:38:34.439 fetched POST /api/observability/v1/traces 204
09:38:34.476 chunk 18 47202 bytes
09:38:34.478 in Ack 18
09:38:34.977 chunk 18 49676 bytes
09:38:34.981 in Ack 18
09:38:35.230 chunk 18 49784 bytes
09:38:35.233 in Ack 18
09:38:35.598 fetch POST /api/observability/v1/traces
09:38:35.598 fetched POST /api/observability/v1/traces 204
09:38:36.121 chunk 18 50786 bytes
09:38:36.123 in Ack 18
09:38:36.376 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:36.893 chunk 18 54274 bytes
09:38:36.895 in Ack 18
09:38:37.125 fetch POST /api/observability/v1/traces
09:38:37.125 fetched POST /api/observability/v1/traces 204
09:38:38.216 in Ping 
09:38:42.915 exit 36 {"_tag":"Success","value":{"sequence":187}}
09:38:43.221 in Ping 
09:38:43.803 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:43.922 fetch POST /api/observability/v1/traces
09:38:43.922 fetched POST /api/observability/v1/traces 204
09:38:44.885 chunk 18 54317 bytes
09:38:44.887 in Ack 18
09:38:45.430 chunk 12 4787 bytes
09:38:45.431 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:45.432 in Ack 12
09:38:45.439 rpc 38 orchestration.dispatchCommand thread.visit
09:38:45.439 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:38:45.889 fetch POST /api/observability/v1/traces
09:38:45.889 fetched POST /api/observability/v1/traces 204
09:38:48.224 in Ping 
09:38:48.442 in Interrupt 35
09:38:49.443 fetch POST /api/observability/v1/traces
09:38:49.443 fetched POST /api/observability/v1/traces 204
09:38:53.229 in Ping 
09:38:58.232 in Ping 
09:38:58.443 rpc 39 server.reportClientActivity
09:38:59.443 fetch POST /api/observability/v1/traces
09:38:59.443 fetched POST /api/observability/v1/traces 204
09:39:02.283 chunk 18 56733 bytes
09:39:02.285 in Ack 18
09:39:03.236 in Ping 
09:39:03.287 fetch POST /api/observability/v1/traces
09:39:03.287 fetched POST /api/observability/v1/traces 204
09:39:03.892 chunk 18 57233 bytes
09:39:03.894 in Ack 18
09:39:04.253 chunk 18 57303 bytes
09:39:04.260 in Ack 18
09:39:04.507 chunk 18 57701 bytes
09:39:04.510 in Ack 18
09:39:04.896 fetch POST /api/observability/v1/traces
09:39:04.896 fetched POST /api/observability/v1/traces 204
09:39:06.716 chunk 18 58063 bytes
09:39:06.718 in Ack 18
09:39:07.143 chunk 18 58097 bytes
09:39:07.146 in Ack 18
09:39:07.393 chunk 18 58233 bytes
09:39:07.395 in Ack 18
09:39:07.720 fetch POST /api/observability/v1/traces
09:39:07.720 fetched POST /api/observability/v1/traces 204
09:39:08.240 in Ping 
