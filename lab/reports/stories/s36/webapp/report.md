# story, webapp, warm start, seed 36

Profile story (graded): seed 36's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 88 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.9s |
| Prompt reached the agent | 6.5s |
| Time to first text | 30.7s |
| Whole reply shown | 85.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 38.9s / 15.9s / 33.9s |
| Longest silence | 36.6s |
| Approval shown after it was asked | 3.5s |
| Bytes on the wire down / up | 318215 / 148516 |
| Bytes the app received / sent | 215329 / 67967 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 3.4 | 0 | 137 |
| 20 | 13.4 | 0 | 538 |
| 30 | 23.4 | 0 | 942 |
| 40 | 23.4 | 397 | 1144 |
| 50 | 3.9 | 1378 | 1516 |
| 60 | 13.9 | 1378 | 1596 |
| 70 | 23.9 | 1378 | 1596 |
| 80 | 33.9 | 1378 | 1596 |
| 85 | 38.9 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217048108,"silences":0,"longestSilenceMs":1560,"longestRecoveredGapMs":1560},"stats":{"sendRate":5070.534320481732,"deliveredRate":2198.8430602957383,"loss":0,"srttMs":1759.3773037199112,"minRttMs":1,"sentPackets":164,"sentWireBytes":44109,"receivedPackets":216,"receivedWireBytes":131120,"lastHeardMs":19895,"unsyncedBytes":518,"inflightPackets":17,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 818/818","user":"10/10 9/9 73/73","attempts":"659/659 659/659 81/635","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"1016/1016 809/809 809/809 1016/1016 710/763","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217058249,"silences":0,"longestSilenceMs":1560,"longestRecoveredGapMs":1560},"stats":{"sendRate":1648.7657193194188,"deliveredRate":0,"loss":0.013125,"srttMs":3347.2336018365677,"minRttMs":1,"sentPackets":212,"sentWireBytes":59079,"receivedPackets":277,"receivedWireBytes":173354,"lastHeardMs":30036,"unsyncedBytes":457,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 626/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 539/635","providerTurns":"772/772 772/772 229/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 413/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217067045,"silences":1,"longestSilenceMs":3396,"longestRecoveredGapMs":3396},"stats":{"sendRate":1236.574289489564,"deliveredRate":1005.2539404553414,"loss":0.625,"srttMs":4531.589010721507,"minRttMs":1,"sentPackets":249,"sentWireBytes":66996,"receivedPackets":306,"receivedWireBytes":184482,"lastHeardMs":38832,"unsyncedBytes":0,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1035/1068","thread":"961/961","assistant":"32/32 21/21 488/498","runs":"1014/1014 971/971 626/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 582/635","providerTurns":"772/772 772/772 229/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 413/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217078411,"silences":3,"longestSilenceMs":4583,"longestRecoveredGapMs":4583},"stats":{"sendRate":1236.574289489564,"deliveredRate":759.8622787443054,"loss":0.655,"srttMs":4531.589010721507,"minRttMs":1,"sentPackets":275,"sentWireBytes":70700,"receivedPackets":352,"receivedWireBytes":204647,"lastHeardMs":50198,"unsyncedBytes":0,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 262/1093 1542/1542 348/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 442/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 413/785 312/1368","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 150/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217088848,"silences":3,"longestSilenceMs":4583,"longestRecoveredGapMs":4583},"stats":{"sendRate":1854.8614342343462,"deliveredRate":931.1696272165229,"loss":0.1636363636363636,"srttMs":4624.875095203696,"minRttMs":1,"sentPackets":324,"sentWireBytes":85193,"receivedPackets":454,"receivedWireBytes":247977,"lastHeardMs":60635,"unsyncedBytes":398,"inflightPackets":68,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 623/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 413/785 966/1392 762/983","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 150/543 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217099261,"silences":3,"longestSilenceMs":4583,"longestRecoveredGapMs":4583},"stats":{"sendRate":521.6797783784099,"deliveredRate":508.0405002977963,"loss":0.4545454545454546,"srttMs":3419.5885871541627,"minRttMs":1,"sentPackets":353,"sentWireBytes":93278,"receivedPackets":568,"receivedWireBytes":311774,"lastHeardMs":71048,"unsyncedBytes":758,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 578/785 1053/1392 762/983 609/1016","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 544/544 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217101848,"silences":3,"longestSilenceMs":4583,"longestRecoveredGapMs":4583},"stats":{"sendRate":391.2598337838074,"deliveredRate":508.0405002977963,"loss":0.4545454545454546,"srttMs":3186.5622761599175,"minRttMs":2,"sentPackets":367,"sentWireBytes":97309,"receivedPackets":588,"receivedWireBytes":322963,"lastHeardMs":73635,"unsyncedBytes":398,"inflightPackets":63,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 578/785 1053/1392 762/983 609/1016","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 544/544 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217111609,"silences":4,"longestSilenceMs":9761,"longestRecoveredGapMs":9761},"stats":{"sendRate":391.2598337838074,"deliveredRate":508.0405002977963,"loss":0.4545454545454546,"srttMs":4139.991991639928,"minRttMs":2127,"sentPackets":382,"sentWireBytes":101545,"receivedPackets":589,"receivedWireBytes":323908,"lastHeardMs":83396,"unsyncedBytes":794,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947 1068/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"519/809","nodes":"1016/1016 809/809 809/809 1016/1016 578/785 1053/1392 762/983 609/1016","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 544/544 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":947,"lastDataAt":1794217111609,"silences":4,"longestSilenceMs":9761,"longestRecoveredGapMs":9761},"stats":{"sendRate":391.2598337838074,"deliveredRate":508.0405002977963,"loss":0.4545454545454546,"srttMs":4139.991991639928,"minRttMs":2127,"sentPackets":391,"sentWireBytes":104090,"receivedPackets":589,"receivedWireBytes":323908,"lastHeardMs":83396,"unsyncedBytes":1251,"inflightPackets":86,"duplicateBytes":0}}
- shim trace
09:37:09.120 fetch GET /api/auth/session
09:37:09.130 fetched GET /api/auth/session 200
09:37:09.576 fetch GET /.well-known/t3/environment
09:37:09.577 fetched GET /.well-known/t3/environment 200
09:37:09.600 fetch GET /.well-known/t3/environment
09:37:09.601 fetched GET /.well-known/t3/environment 200
09:37:09.612 rpc 0 subscribeServerConfig
09:37:09.613 chunk 0 38462 bytes
09:37:09.732 in Ack 0
09:37:09.735 rpc 1 subscribeServerLifecycle
09:37:09.736 rpc 2 subscribeServerLifecycle
09:37:09.740 fetch GET /api/auth/session
09:37:09.742 fetched GET /api/auth/session 200
09:37:09.742 rpc 3 subscribeDeviceState
09:37:09.746 rpc 4 preview.reportProfiles
09:37:09.889 rpc 5 server.reportClientActivity
09:37:09.974 rpc 6 subscribeProjectClones
09:37:09.983 fetch GET /api/orchestration/shell
09:37:09.985 fetched GET /api/orchestration/shell 200
09:37:09.988 rpc 7 orchestration.subscribeShell
09:37:09.988 chunk 7 4551 bytes
09:37:10.126 rpc 8 server.refreshProviders
09:37:10.272 rpc 9 orchestration.dispatchCommand thread.visit
09:37:10.272 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:10.312 rpc 10 subscribeTerminalMetadata
09:37:10.316 rpc 11 subscribeVcsStatus
09:37:10.317 rpc 12 subscribePreviewEvents
09:37:10.319 rpc 13 assets.createUrl
09:37:10.320 rpc 14 preview.list
09:37:10.321 rpc 15 projects.readFile
09:37:10.341 fetch GET /api/orchestration/threads/mcp%3A2ee5853d-b99c-49b2-865b-0d85e6ae2676/bounded
09:37:10.350 in Ack 7
09:37:10.370 chunk 7 4527 bytes
09:37:10.375 in Ack 7
09:37:10.638 fetch POST /api/observability/v1/traces
09:37:10.638 fetched POST /api/observability/v1/traces 204
09:37:10.769 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"2c6135b8-e3f9-40e4-8b52-e7fa8ba7e810","sessions":[]}}
09:37:10.769 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:10.994 chunk 1 3815 bytes
09:37:10.995 in Ack 1
09:37:11.236 chunk 2 3815 bytes
09:37:11.237 in Ack 2
09:37:11.426 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:38.810Z","configurableProviders":false,"con
09:37:11.426 chunk 11 1632 bytes
09:37:11.426 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:11.431 in Ack 11
09:37:11.646 fetch POST /api/observability/v1/traces
09:37:11.646 fetched POST /api/observability/v1/traces 204
09:37:11.850 chunk 7 4613 bytes
09:37:11.851 in Ack 7
09:37:12.099 chunk 7 4656 bytes
09:37:12.100 in Ack 7
09:37:12.351 chunk 7 4595 bytes
09:37:12.352 in Ack 7
09:37:12.431 chunk 3 876 bytes
09:37:12.431 exit 4 {"_tag":"Success","value":null}
09:37:12.431 exit 5 {"_tag":"Success","value":null}
09:37:12.431 chunk 6 4 bytes
09:37:12.431 chunk 10 36 bytes
09:37:12.433 in Ack 3
09:37:12.440 rpc 16 preview.reportProfiles
09:37:12.443 in Ack 6
09:37:12.443 in Ack 10
09:37:12.509 exit 16 {"_tag":"Success","value":null}
09:37:12.694 rpc 17 server.reportClientActivity
09:37:12.741 exit 17 {"_tag":"Success","value":null}
09:37:12.853 fetch POST /api/observability/v1/traces
09:37:12.853 fetched POST /api/observability/v1/traces 204
09:37:13.602 chunk 7 4592 bytes
09:37:13.603 in Ack 7
09:37:13.853 chunk 7 4592 bytes
09:37:13.854 in Ack 7
09:37:14.606 fetch POST /api/observability/v1/traces
09:37:14.606 fetched POST /api/observability/v1/traces 204
09:37:14.608 in Ping 
09:37:15.233 fetched GET /api/orchestration/threads/mcp%3A2ee5853d-b99c-49b2-865b-0d85e6ae2676/bounded 200
09:37:15.239 rpc 18 orchestration.subscribeThread
09:37:15.239 chunk 18 8282 bytes
09:37:15.259 in Interrupt 11
09:37:15.262 rpc 19 subscribeVcsStatus
09:37:15.264 in Ack 18
09:37:15.392 chunk 19 1632 bytes
09:37:15.392 in Ack 19
09:37:15.492 chunk 18 8322 bytes
09:37:15.493 in Ack 18
09:37:15.537 rpc 20 server.reportClientActivity
09:37:15.685 exit 20 {"_tag":"Success","value":null}
09:37:15.745 chunk 18 12690 bytes
09:37:15.747 in Ack 18
09:37:15.995 chunk 18 13497 bytes
09:37:15.997 in Ack 18
09:37:16.235 fetch POST /api/observability/v1/traces
09:37:16.235 fetched POST /api/observability/v1/traces 204
09:37:16.246 chunk 18 17976 bytes
09:37:16.248 in Ack 18
09:37:16.502 rpc 21 attachments.createUploadUrl
09:37:16.549 chunk 18 19796 bytes
09:37:16.567 in Ack 18
09:37:16.586 in Interrupt 19
09:37:16.591 rpc 22 subscribeVcsStatus
09:37:16.643 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-2fbbea1c-5356-4416-9156-d34b31be9366","expiresAt":1794217636628,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:16.643 chunk 22 1632 bytes
09:37:16.644 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTJmYmJlYTFjLTUzNTYtNDQxNi05MTU2LWQzNGIzMWJlOTM2NiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzYzNjYyOH0.qlt-Zycl5Pz37aApzhP8kCnNYa3BzdF6B8i09etYdDI
09:37:16.650 in Ack 22
09:37:16.799 chunk 18 21099 bytes
09:37:16.801 in Ack 18
09:37:16.843 rpc 23 server.reportClientActivity
09:37:17.249 fetch POST /api/observability/v1/traces
09:37:17.249 fetched POST /api/observability/v1/traces 204
09:37:19.587 exit 23 {"_tag":"Success","value":null}
09:37:19.609 in Ping 
09:37:20.588 fetch POST /api/observability/v1/traces
09:37:20.588 fetched POST /api/observability/v1/traces 204
09:37:20.897 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:21.465 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:21.470 rpc 25 attachments.delete
09:37:21.659 fetch POST /api/observability/v1/traces
09:37:21.659 fetched POST /api/observability/v1/traces 204
09:37:21.991 chunk 7 4678 bytes
09:37:21.991 chunk 18 25368 bytes
09:37:21.992 in Ack 7
09:37:22.015 rpc 26 orchestration.dispatchCommand thread.visit
09:37:22.016 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:22.189 in Ack 18
09:37:22.194 rpc 27 assets.createUrl
09:37:22.285 exit 25 {"_tag":"Success","value":null}
09:37:22.536 chunk 18 28480 bytes
09:37:22.541 in Ack 18
09:37:22.785 chunk 7 4617 bytes
09:37:22.786 chunk 18 28480 bytes
09:37:22.787 in Ack 7
09:37:22.799 in Ack 18
09:37:22.994 fetch POST /api/observability/v1/traces
09:37:22.994 fetched POST /api/observability/v1/traces 204
09:37:23.038 chunk 18 29408 bytes
09:37:23.047 in Ack 18
09:37:23.292 chunk 18 31902 bytes
09:37:23.296 in Ack 18
09:37:23.543 chunk 18 31902 bytes
09:37:23.545 in Ack 18
09:37:23.791 chunk 7 4614 bytes
09:37:23.791 in Ack 7
09:37:23.793 rpc 28 orchestration.dispatchCommand thread.visit
09:37:23.793 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:37:24.040 chunk 7 4614 bytes
09:37:24.041 in Ack 7
09:37:24.050 fetch POST /api/observability/v1/traces
09:37:24.050 fetched POST /api/observability/v1/traces 204
09:37:24.051 chunk 18 32034 bytes
09:37:24.053 in Ack 18
09:37:24.327 chunk 18 35259 bytes
09:37:24.330 in Ack 18
09:37:24.344 in Interrupt 22
09:37:24.348 rpc 29 subscribeVcsStatus
09:37:24.771 rpc 30 server.reportClientActivity
09:37:24.772 in Ping 
09:37:24.847 chunk 29 1632 bytes
09:37:24.847 in Ack 29
09:37:25.095 rpc 31 server.refreshProviders
09:37:25.095 exit 31 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:38.810Z","configurableProviders":false,"con
09:37:25.132 fetch POST /api/observability/v1/traces
09:37:25.132 fetched POST /api/observability/v1/traces 204
09:37:25.892 exit 30 {"_tag":"Success","value":null}
09:37:26.062 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/e7442916-0277-4587-8295-072e86daf2c7","expiresAt":1794220646062}}
09:37:26.218 fetch POST /api/observability/v1/traces
09:37:26.218 fetched POST /api/observability/v1/traces 204
09:37:28.019 rpc 32 orchestration.dispatchCommand message.dispatch
09:37:29.025 fetch POST /api/observability/v1/traces
09:37:29.025 fetched POST /api/observability/v1/traces 204
09:37:29.773 in Ping 
09:37:31.392 exit 32 {"_tag":"Success","value":{"sequence":152}}
09:37:31.663 fetch POST /api/observability/v1/traces
09:37:31.663 fetched POST /api/observability/v1/traces 204
09:37:31.762 chunk 18 37864 bytes
09:37:31.765 in Ack 18
09:37:32.013 chunk 18 39208 bytes
09:37:32.015 in Ack 18
09:37:32.516 chunk 18 39208 bytes
09:37:32.522 in Ack 18
09:37:32.770 chunk 7 4678 bytes
09:37:32.774 fetch POST /api/observability/v1/traces
09:37:32.774 fetched POST /api/observability/v1/traces 204
09:37:32.776 in Ack 7
09:37:33.778 fetch POST /api/observability/v1/traces
09:37:33.778 fetched POST /api/observability/v1/traces 204
09:37:34.773 in Ping 
09:37:34.830 rpc 33 server.reportClientActivity
09:37:35.831 fetch POST /api/observability/v1/traces
09:37:35.831 fetched POST /api/observability/v1/traces 204
09:37:37.091 fetch POST /api/observability/v1/traces
09:37:37.091 fetched POST /api/observability/v1/traces 204
09:37:39.774 in Ping 
09:37:41.666 fetch POST /api/observability/v1/traces
09:37:41.666 fetched POST /api/observability/v1/traces 204
09:37:44.775 in Ping 
09:37:49.775 in Ping 
09:37:49.837 in Interrupt 33
09:37:50.839 fetch POST /api/observability/v1/traces
09:37:50.839 fetched POST /api/observability/v1/traces 204
09:37:54.467 chunk 18 41753 bytes
09:37:54.470 in Ack 18
09:37:54.755 chunk 18 41881 bytes
09:37:54.758 in Ack 18
09:37:54.829 in Ping 
09:37:55.327 chunk 18 42009 bytes
09:37:55.333 in Ack 18
09:37:55.476 fetch POST /api/observability/v1/traces
09:37:55.476 fetched POST /api/observability/v1/traces 204
09:37:55.875 chunk 18 42137 bytes
09:37:55.880 in Ack 18
09:37:56.172 chunk 18 42269 bytes
09:37:56.175 in Ack 18
09:37:56.881 fetch POST /api/observability/v1/traces
09:37:56.881 fetched POST /api/observability/v1/traces 204
09:37:56.886 chunk 18 42377 bytes
09:37:56.889 in Ack 18
09:37:57.891 fetch POST /api/observability/v1/traces
09:37:57.891 fetched POST /api/observability/v1/traces 204
09:37:58.138 chunk 18 42603 bytes
09:37:58.145 in Ack 18
09:37:59.148 fetch POST /api/observability/v1/traces
09:37:59.149 fetched POST /api/observability/v1/traces 204
09:37:59.832 rpc 34 server.reportClientActivity
09:37:59.833 in Ping 
09:38:00.114 chunk 18 42555 bytes
09:38:00.131 in Ack 18
09:38:00.839 fetch POST /api/observability/v1/traces
09:38:00.839 fetched POST /api/observability/v1/traces 204
09:38:00.982 chunk 18 42573 bytes
09:38:00.989 in Ack 18
09:38:01.990 fetch POST /api/observability/v1/traces
09:38:01.990 fetched POST /api/observability/v1/traces 204
09:38:02.246 chunk 7 4742 bytes
09:38:02.249 in Ack 7
09:38:02.260 rpc 35 orchestration.dispatchCommand thread.visit
09:38:02.261 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:02.498 chunk 18 42646 bytes
09:38:02.501 in Ack 18
09:38:03.260 fetch POST /api/observability/v1/traces
09:38:03.260 fetched POST /api/observability/v1/traces 204
09:38:04.365 chunk 18 43648 bytes
09:38:04.370 in Ack 18
09:38:04.836 in Ping 
09:38:04.906 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:05.373 fetch POST /api/observability/v1/traces
09:38:05.373 fetched POST /api/observability/v1/traces 204
09:38:06.479 exit 33 {"_tag":"Success","value":null}
09:38:06.873 chunk 18 47135 bytes
09:38:06.876 in Ack 18
09:38:07.880 fetch POST /api/observability/v1/traces
09:38:07.880 fetched POST /api/observability/v1/traces 204
09:38:08.500 chunk 18 47949 bytes
09:38:08.503 in Ack 18
09:38:09.001 chunk 18 48823 bytes
09:38:09.007 in Ack 18
09:38:09.507 fetch POST /api/observability/v1/traces
09:38:09.507 fetched POST /api/observability/v1/traces 204
09:38:09.838 in Ping 
09:38:10.313 exit 36 {"_tag":"Success","value":{"sequence":187}}
09:38:10.436 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:11.314 fetch POST /api/observability/v1/traces
09:38:11.314 fetched POST /api/observability/v1/traces 204
09:38:12.498 chunk 18 48866 bytes
09:38:12.500 in Ack 18
09:38:13.372 chunk 18 48916 bytes
09:38:13.375 in Ack 18
09:38:13.504 fetch POST /api/observability/v1/traces
09:38:13.504 fetched POST /api/observability/v1/traces 204
09:38:13.903 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:14.402 chunk 18 48916 bytes
09:38:14.405 in Ack 18
09:38:14.833 in Interrupt 34
09:38:14.851 in Ping 
09:38:14.920 fetch POST /api/observability/v1/traces
09:38:14.920 fetched POST /api/observability/v1/traces 204
09:38:15.425 chunk 18 52152 bytes
09:38:15.427 in Ack 18
09:38:16.023 chunk 18 52104 bytes
09:38:16.027 in Ack 18
09:38:16.428 fetch POST /api/observability/v1/traces
09:38:16.428 fetched POST /api/observability/v1/traces 204
09:38:17.022 chunk 7 4744 bytes
09:38:17.024 in Ack 7
09:38:17.028 rpc 38 orchestration.dispatchCommand thread.visit
09:38:17.028 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:38:17.776 chunk 18 55176 bytes
09:38:17.779 in Ack 18
09:38:17.795 in Interrupt 29
09:38:17.837 rpc 39 subscribeVcsStatus
09:38:18.031 fetch POST /api/observability/v1/traces
09:38:18.031 fetched POST /api/observability/v1/traces 204
09:38:18.094 rpc 40 server.reportClientActivity
09:38:18.285 chunk 18 55224 bytes
09:38:18.288 in Ack 18
09:38:19.096 fetch POST /api/observability/v1/traces
09:38:19.096 fetched POST /api/observability/v1/traces 204
09:38:19.538 chunk 18 57768 bytes
09:38:19.541 in Ack 18
09:38:19.696 exit 34 {"_tag":"Success","value":null}
09:38:19.854 in Ping 
09:38:20.289 chunk 7 4744 bytes
09:38:20.290 in Ack 7
09:38:20.540 chunk 18 57768 bytes
09:38:20.545 in Ack 18
09:38:20.574 fetch POST /api/observability/v1/traces
09:38:20.574 fetched POST /api/observability/v1/traces 204
09:38:20.791 chunk 18 57768 bytes
09:38:20.792 in Ack 18
09:38:21.795 fetch POST /api/observability/v1/traces
09:38:21.795 fetched POST /api/observability/v1/traces 204
09:38:24.858 in Ping 
09:38:28.060 fetch POST /api/observability/v1/traces
09:38:28.060 fetched POST /api/observability/v1/traces 204
09:38:29.862 in Ping 
09:38:33.095 in Interrupt 40
09:38:33.347 rpc 41 server.reportClientActivity
09:38:34.096 fetch POST /api/observability/v1/traces
09:38:34.096 fetched POST /api/observability/v1/traces 204
09:38:34.866 in Ping 
09:38:39.870 in Ping 
09:38:44.873 in Ping 
09:38:48.347 in Interrupt 41
09:38:49.348 fetch POST /api/observability/v1/traces
09:38:49.348 fetched POST /api/observability/v1/traces 204
09:38:49.831 rpc 42 server.reportClientActivity
09:38:49.878 in Ping 
09:38:50.833 fetch POST /api/observability/v1/traces
09:38:50.833 fetched POST /api/observability/v1/traces 204
09:38:52.409 chunk 18 58296 bytes
09:38:52.413 in Ack 18
09:38:52.661 chunk 18 58392 bytes
09:38:52.664 in Ack 18
09:38:52.912 chunk 18 58785 bytes
09:38:52.914 in Ack 18
09:38:53.719 chunk 39 1713 bytes
09:38:53.721 fetch POST /api/observability/v1/traces
09:38:53.722 fetched POST /api/observability/v1/traces 204
09:38:53.725 in Ack 39
