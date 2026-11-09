# story, webapp, warm start, seed 44

Profile story (graded): seed 44's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 98 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.5s |
| Prompt reached the agent | 3.4s |
| Time to first text | 21.6s |
| Whole reply shown | 95.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 33.5s / 9.5s / 28.5s |
| Longest silence | 25.5s |
| Approval shown after it was asked | 30.7s |
| Bytes on the wire down / up | 228709 / 154267 |
| Bytes the app received / sent | 81085 / 76872 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.5 | 0 | 266 |
| 20 | 16.5 | 0 | 663 |
| 30 | 16.5 | 397 | 1069 |
| 40 | 26.5 | 397 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 7.5 | 1077 | 1378 |
| 80 | 9.8 | 1378 | 1596 |
| 90 | 19.8 | 1378 | 1596 |
| 95 | 24.8 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217081452,"silences":0,"longestSilenceMs":1339,"longestRecoveredGapMs":1339},"stats":{"sendRate":6750,"deliveredRate":2899.0795255460794,"loss":0,"srttMs":14.920979155808137,"minRttMs":1,"sentPackets":152,"sentWireBytes":42192,"receivedPackets":142,"receivedWireBytes":107041,"lastHeardMs":14998,"unsyncedBytes":1193,"inflightPackets":23,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 586/654","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217091423,"silences":0,"longestSilenceMs":1339,"longestRecoveredGapMs":1339},"stats":{"sendRate":2262.1524126830127,"deliveredRate":1782.8282588238123,"loss":0.06407624999999999,"srttMs":2730.263375893495,"minRttMs":1,"sentPackets":204,"sentWireBytes":55676,"receivedPackets":212,"receivedWireBytes":132740,"lastHeardMs":24969,"unsyncedBytes":0,"inflightPackets":6,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 277/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217098484,"silences":1,"longestSilenceMs":6462,"longestRecoveredGapMs":6462},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.06407624999999999,"srttMs":2730.263375893495,"minRttMs":1,"sentPackets":251,"sentWireBytes":68732,"receivedPackets":215,"receivedWireBytes":134277,"lastHeardMs":32029,"unsyncedBytes":457,"inflightPackets":51,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 632/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217102602,"silences":2,"longestSilenceMs":6462,"longestRecoveredGapMs":6462},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.06407624999999999,"srttMs":2730.263375893495,"minRttMs":1,"sentPackets":263,"sentWireBytes":72190,"receivedPackets":222,"receivedWireBytes":137776,"lastHeardMs":36147,"unsyncedBytes":457,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 632/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217102602,"silences":2,"longestSilenceMs":6462,"longestRecoveredGapMs":6462},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.06407624999999999,"srttMs":2730.263375893495,"minRttMs":1,"sentPackets":287,"sentWireBytes":79094,"receivedPackets":222,"receivedWireBytes":137776,"lastHeardMs":36147,"unsyncedBytes":914,"inflightPackets":87,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 632/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"268/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217128875,"silences":3,"longestSilenceMs":23840,"longestRecoveredGapMs":23840},"stats":{"sendRate":1000,"deliveredRate":202.1209099972809,"loss":0.7333333333333334,"srttMs":2647.8735221684574,"minRttMs":1,"sentPackets":321,"sentWireBytes":86179,"receivedPackets":231,"receivedWireBytes":140666,"lastHeardMs":62421,"unsyncedBytes":0,"inflightPackets":73,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 632/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"922/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217141791,"silences":4,"longestSilenceMs":23840,"longestRecoveredGapMs":23840},"stats":{"sendRate":1000,"deliveredRate":177.16562156566215,"loss":0.645047619047619,"srttMs":2802.2643318974,"minRttMs":3,"sentPackets":346,"sentWireBytes":89444,"receivedPackets":247,"receivedWireBytes":145929,"lastHeardMs":75336,"unsyncedBytes":0,"inflightPackets":70,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 184/1566","thread":"961/961","assistant":"32/32 21/21 1340/1340 287/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 632/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217152436,"silences":4,"longestSilenceMs":23840,"longestRecoveredGapMs":23840},"stats":{"sendRate":1322.0708353221958,"deliveredRate":0,"loss":0,"srttMs":2399.420440610998,"minRttMs":2182,"sentPackets":386,"sentWireBytes":100404,"receivedPackets":284,"receivedWireBytes":162599,"lastHeardMs":85982,"unsyncedBytes":167,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1395/1566 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 695/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217157660,"silences":4,"longestSilenceMs":23840,"longestRecoveredGapMs":23840},"stats":{"sendRate":1793.5762584522915,"deliveredRate":1235.3629096821078,"loss":0,"srttMs":2241.5275368766124,"minRttMs":2075,"sentPackets":414,"sentWireBytes":105521,"receivedPackets":315,"receivedWireBytes":173003,"lastHeardMs":91206,"unsyncedBytes":0,"inflightPackets":70,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1093/1093 1058/1058 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1395/1566 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"617/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 695/785","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":763,"lastDataAt":1794217157660,"silences":4,"longestSilenceMs":23840,"longestRecoveredGapMs":23840},"stats":{"sendRate":1793.5762584522915,"deliveredRate":0,"loss":0,"srttMs":2241.5275368766124,"minRttMs":2075,"sentPackets":441,"sentWireBytes":112600,"receivedPackets":315,"receivedWireBytes":173003,"lastHeardMs":91206,"unsyncedBytes":457,"inflightPackets":94,"duplicateBytes":0}}
- shim trace
09:37:47.216 fetch GET /api/auth/session
09:37:47.219 fetched GET /api/auth/session 200
09:37:47.524 fetch GET /.well-known/t3/environment
09:37:47.525 fetched GET /.well-known/t3/environment 200
09:37:47.544 fetch GET /.well-known/t3/environment
09:37:47.545 fetched GET /.well-known/t3/environment 200
09:37:47.553 rpc 0 subscribeServerConfig
09:37:47.554 chunk 0 38462 bytes
09:37:47.696 in Ack 0
09:37:47.699 rpc 1 subscribeServerLifecycle
09:37:47.700 rpc 2 subscribeServerLifecycle
09:37:47.703 fetch GET /api/auth/session
09:37:47.705 fetched GET /api/auth/session 200
09:37:47.705 rpc 3 subscribeDeviceState
09:37:47.708 rpc 4 preview.reportProfiles
09:37:47.912 rpc 5 server.reportClientActivity
09:37:48.025 rpc 6 subscribeProjectClones
09:37:48.143 fetch GET /api/orchestration/threads/mcp%3A7d899d33-4445-4e9c-9551-f4265b1777bd/bounded
09:37:48.153 fetch GET /api/orchestration/shell
09:37:48.154 fetched GET /api/orchestration/shell 200
09:37:48.158 rpc 7 orchestration.subscribeShell
09:37:48.158 chunk 7 2751 bytes
09:37:48.163 in Ack 7
09:37:48.448 chunk 7 2724 bytes
09:37:48.449 in Ack 7
09:37:48.520 fetch POST /api/observability/v1/traces
09:37:48.520 fetched POST /api/observability/v1/traces 204
09:37:48.616 chunk 1 3815 bytes
09:37:48.617 in Ack 1
09:37:48.812 chunk 7 4549 bytes
09:37:48.812 chunk 2 3815 bytes
09:37:48.813 in Ack 7
09:37:48.846 rpc 8 server.refreshProviders
09:37:48.912 rpc 9 orchestration.dispatchCommand thread.visit
09:37:48.912 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:48.931 rpc 10 subscribeTerminalMetadata
09:37:48.932 rpc 11 subscribeVcsStatus
09:37:48.933 rpc 12 subscribePreviewEvents
09:37:48.934 rpc 13 assets.createUrl
09:37:48.936 rpc 14 preview.list
09:37:48.936 rpc 15 projects.readFile
09:37:48.952 in Ack 2
09:37:49.038 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3109bdd5-17a9-4000-8d13-28313a1836e1","sessions":[]}}
09:37:49.038 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:49.038 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:21.286Z","configurableProviders":false,"con
09:37:49.318 chunk 11 1632 bytes
09:37:49.318 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:49.319 in Ack 11
09:37:49.573 chunk 7 4592 bytes
09:37:49.574 in Ack 7
09:37:49.663 fetch POST /api/observability/v1/traces
09:37:49.663 fetched POST /api/observability/v1/traces 204
09:37:49.760 chunk 3 876 bytes
09:37:49.760 exit 4 {"_tag":"Success","value":null}
09:37:49.760 exit 5 {"_tag":"Success","value":null}
09:37:49.760 chunk 6 4 bytes
09:37:49.760 chunk 10 36 bytes
09:37:49.761 in Ack 3
09:37:49.767 rpc 16 preview.reportProfiles
09:37:49.768 in Ack 6
09:37:49.768 in Ack 10
09:37:49.841 exit 16 {"_tag":"Success","value":null}
09:37:50.019 rpc 17 server.reportClientActivity
09:37:50.093 exit 17 {"_tag":"Success","value":null}
09:37:50.767 fetch POST /api/observability/v1/traces
09:37:50.767 fetched POST /api/observability/v1/traces 204
09:37:51.216 fetched GET /api/orchestration/threads/mcp%3A7d899d33-4445-4e9c-9551-f4265b1777bd/bounded 200
09:37:51.221 rpc 18 orchestration.subscribeThread
09:37:51.221 chunk 18 8283 bytes
09:37:51.235 in Interrupt 11
09:37:51.237 rpc 19 subscribeVcsStatus
09:37:51.284 in Ack 18
09:37:51.376 chunk 19 1632 bytes
09:37:51.377 in Ack 19
09:37:51.471 chunk 18 8323 bytes
09:37:51.472 in Ack 18
09:37:51.521 rpc 20 server.reportClientActivity
09:37:51.682 exit 20 {"_tag":"Success","value":null}
09:37:51.721 chunk 18 12691 bytes
09:37:51.726 in Ack 18
09:37:51.971 chunk 18 13498 bytes
09:37:51.973 in Ack 18
09:37:52.217 fetch POST /api/observability/v1/traces
09:37:52.218 fetched POST /api/observability/v1/traces 204
09:37:52.221 chunk 18 17167 bytes
09:37:52.223 in Ack 18
09:37:52.470 rpc 21 attachments.createUploadUrl
09:37:52.533 chunk 18 19797 bytes
09:37:52.545 in Ack 18
09:37:52.556 in Interrupt 19
09:37:52.559 rpc 22 subscribeVcsStatus
09:37:52.560 in Ping 
09:37:52.655 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-ad933512-57e1-42bb-922e-613faf20508e","expiresAt":1794217672580,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:52.655 chunk 22 1632 bytes
09:37:52.657 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWFkOTMzNTEyLTU3ZTEtNDJiYi05MjJlLTYxM2ZhZjIwNTA4ZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY3MjU4MH0.yZRmptwasd3QhukdN04Xdj4kZg48RTh6cVMDzFHVKG8
09:37:52.657 in Ack 22
09:37:52.784 chunk 18 21100 bytes
09:37:52.785 in Ack 18
09:37:52.810 rpc 23 server.reportClientActivity
09:37:53.225 fetch POST /api/observability/v1/traces
09:37:53.225 fetched POST /api/observability/v1/traces 204
09:37:54.430 exit 23 {"_tag":"Success","value":null}
09:37:55.438 fetch POST /api/observability/v1/traces
09:37:55.438 fetched POST /api/observability/v1/traces 204
09:37:55.934 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:56.241 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:56.245 rpc 25 attachments.delete
09:37:56.507 chunk 7 4678 bytes
09:37:56.508 in Ack 7
09:37:56.585 exit 25 {"_tag":"Success","value":null}
09:37:56.759 chunk 18 26810 bytes
09:37:56.761 in Ack 18
09:37:56.764 rpc 26 assets.createUrl
09:37:56.933 fetch POST /api/observability/v1/traces
09:37:56.933 fetched POST /api/observability/v1/traces 204
09:37:57.010 chunk 18 30180 bytes
09:37:57.015 in Ack 18
09:37:57.260 chunk 7 4617 bytes
09:37:57.261 chunk 18 30202 bytes
09:37:57.262 in Ack 7
09:37:57.291 in Ack 18
09:37:57.511 chunk 18 30886 bytes
09:37:57.522 in Ack 18
09:37:57.576 in Ping 
09:37:57.761 chunk 18 31903 bytes
09:37:57.763 in Ack 18
09:37:58.016 chunk 18 32035 bytes
09:37:58.018 fetch POST /api/observability/v1/traces
09:37:58.018 fetched POST /api/observability/v1/traces 204
09:37:58.020 in Ack 18
09:37:58.265 chunk 7 4614 bytes
09:37:58.266 chunk 18 34482 bytes
09:37:58.267 in Ack 7
09:37:58.268 rpc 27 orchestration.dispatchCommand thread.visit
09:37:58.268 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:58.299 in Ack 18
09:37:58.810 chunk 7 4614 bytes
09:37:58.811 chunk 18 35260 bytes
09:37:58.815 in Ack 7
09:37:58.827 in Ack 18
09:37:58.845 in Interrupt 22
09:37:58.849 rpc 28 subscribeVcsStatus
09:37:58.935 chunk 28 1713 bytes
09:37:58.936 in Ack 28
09:37:59.073 fetch POST /api/observability/v1/traces
09:37:59.073 fetched POST /api/observability/v1/traces 204
09:37:59.088 rpc 29 server.refreshProviders
09:37:59.088 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:21.286Z","configurableProviders":false,"con
09:37:59.131 rpc 30 server.reportClientActivity
09:38:00.100 fetch POST /api/observability/v1/traces
09:38:00.101 fetched POST /api/observability/v1/traces 204
09:38:00.274 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/f3b4f8f8-618e-4dc1-bc31-ff025b401d04","expiresAt":1794220680274}}
09:38:01.396 rpc 31 orchestration.dispatchCommand message.dispatch
09:38:01.453 fetch POST /api/observability/v1/traces
09:38:01.454 fetched POST /api/observability/v1/traces 204
09:38:02.271 exit 30 {"_tag":"Success","value":null}
09:38:02.579 in Ping 
09:38:02.792 fetch POST /api/observability/v1/traces
09:38:02.792 fetched POST /api/observability/v1/traces 204
09:38:04.581 exit 31 {"_tag":"Success","value":{"sequence":151}}
09:38:05.559 chunk 18 37281 bytes
09:38:05.561 in Ack 18
09:38:05.626 fetch POST /api/observability/v1/traces
09:38:05.626 fetched POST /api/observability/v1/traces 204
09:38:07.581 in Ping 
09:38:09.861 fetch POST /api/observability/v1/traces
09:38:09.861 fetched POST /api/observability/v1/traces 204
09:38:10.520 chunk 18 37917 bytes
09:38:10.524 in Ack 18
09:38:11.273 chunk 18 38757 bytes
09:38:11.274 in Ack 18
09:38:11.525 fetch POST /api/observability/v1/traces
09:38:11.525 fetched POST /api/observability/v1/traces 204
09:38:11.773 chunk 18 39412 bytes
09:38:11.775 in Ack 18
09:38:12.581 in Ping 
09:38:12.777 rpc 32 server.reportClientActivity
09:38:12.777 fetch POST /api/observability/v1/traces
09:38:12.778 fetched POST /api/observability/v1/traces 204
09:38:17.581 in Ping 
09:38:22.582 in Ping 
09:38:22.623 chunk 18 42777 bytes
09:38:22.625 in Ack 18
09:38:23.626 fetch POST /api/observability/v1/traces
09:38:23.626 fetched POST /api/observability/v1/traces 204
09:38:27.585 in Ping 
09:38:27.784 in Interrupt 32
09:38:28.785 fetch POST /api/observability/v1/traces
09:38:28.785 fetched POST /api/observability/v1/traces 204
09:38:32.586 in Ping 
09:38:37.587 in Ping 
09:38:37.776 rpc 33 server.reportClientActivity
09:38:38.776 fetch POST /api/observability/v1/traces
09:38:38.777 fetched POST /api/observability/v1/traces 204
09:38:42.587 in Ping 
09:38:46.445 chunk 18 43165 bytes
09:38:46.447 in Ack 18
09:38:46.730 chunk 18 43203 bytes
09:38:46.733 in Ack 18
09:38:47.064 chunk 18 43633 bytes
09:38:47.067 in Ack 18
09:38:47.349 chunk 18 43671 bytes
09:38:47.351 in Ack 18
09:38:47.449 fetch POST /api/observability/v1/traces
09:38:47.449 fetched POST /api/observability/v1/traces 204
09:38:47.591 in Ping 
09:38:47.642 chunk 18 44025 bytes
09:38:47.644 in Ack 18
09:38:47.931 chunk 18 44061 bytes
09:38:47.933 in Ack 18
09:38:48.182 chunk 18 44465 bytes
09:38:48.185 in Ack 18
09:38:48.666 fetch POST /api/observability/v1/traces
09:38:48.666 fetched POST /api/observability/v1/traces 204
09:38:52.595 in Ping 
09:38:52.777 in Interrupt 33
09:38:53.778 fetch POST /api/observability/v1/traces
09:38:53.778 fetched POST /api/observability/v1/traces 204
09:38:57.599 in Ping 
09:39:02.089 chunk 18 45467 bytes
09:39:02.091 in Ack 18
09:39:02.367 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:39:02.652 in Ping 
09:39:02.778 rpc 35 server.reportClientActivity
09:39:03.093 fetch POST /api/observability/v1/traces
09:39:03.093 fetched POST /api/observability/v1/traces 204
09:39:04.874 exit 34 {"_tag":"Success","value":{"sequence":183}}
09:39:04.989 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:39:05.875 fetch POST /api/observability/v1/traces
09:39:05.875 fetched POST /api/observability/v1/traces 204
09:39:07.539 chunk 18 45510 bytes
09:39:07.540 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:39:07.542 in Ack 18
09:39:07.658 in Ping 
09:39:08.344 chunk 18 45560 bytes
09:39:08.347 in Ack 18
09:39:08.543 fetch POST /api/observability/v1/traces
09:39:08.543 fetched POST /api/observability/v1/traces 204
09:39:11.959 chunk 7 4680 bytes
09:39:11.960 in Ack 7
09:39:11.966 rpc 37 orchestration.dispatchCommand thread.visit
09:39:11.966 exit 37 {"_tag":"Success","value":{"sequence":0}}
09:39:12.461 chunk 18 48553 bytes
09:39:12.463 in Ack 18
09:39:12.660 in Ping 
09:39:12.963 fetch POST /api/observability/v1/traces
09:39:12.963 fetched POST /api/observability/v1/traces 204
09:39:13.028 chunk 18 48795 bytes
09:39:13.031 in Ack 18
09:39:14.036 fetch POST /api/observability/v1/traces
09:39:14.036 fetched POST /api/observability/v1/traces 204
09:39:16.682 chunk 18 48747 bytes
09:39:16.685 in Ack 18
09:39:17.667 in Ping 
09:39:17.686 fetch POST /api/observability/v1/traces
09:39:17.686 fetched POST /api/observability/v1/traces 204
09:39:17.779 in Interrupt 35
09:39:18.780 fetch POST /api/observability/v1/traces
09:39:18.780 fetched POST /api/observability/v1/traces 204
09:39:22.668 in Ping 
09:39:27.672 in Ping 
09:39:27.778 rpc 38 server.reportClientActivity
09:39:28.779 fetch POST /api/observability/v1/traces
09:39:28.779 fetched POST /api/observability/v1/traces 204
09:39:32.677 in Ping 
09:39:35.379 chunk 18 49091 bytes
09:39:35.383 in Ack 18
09:39:35.879 chunk 18 49425 bytes
09:39:35.880 in Ack 18
09:39:36.384 fetch POST /api/observability/v1/traces
09:39:36.384 fetched POST /api/observability/v1/traces 204
09:39:36.642 chunk 18 49475 bytes
09:39:36.645 in Ack 18
