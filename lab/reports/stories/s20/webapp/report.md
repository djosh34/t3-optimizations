# story, webapp, warm start, seed 20

Profile story (graded): seed 20's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 96 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.9s |
| Prompt reached the agent | 5.0s |
| Time to first text | 17.0s |
| Whole reply shown | 94.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 32.5s / 10.9s / 30.4s |
| Longest silence | 27.9s |
| Approval shown after it was asked | 22.3s |
| Bytes on the wire down / up | 254363 / 77259 |
| Bytes the app received / sent | 127168 / 86574 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.9 | 0 | 202 |
| 20 | 4.9 | 397 | 601 |
| 30 | 14.9 | 397 | 1004 |
| 40 | 24.9 | 397 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 1.4 | 1077 | 1136 |
| 70 | 11.4 | 1077 | 1507 |
| 80 | 19.5 | 1154 | 1596 |
| 90 | 29.5 | 1154 | 1596 |
| 94 | 25.1 | 1469 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217054621,"silences":0,"longestSilenceMs":983,"longestRecoveredGapMs":983},"stats":{"sendRate":6750,"deliveredRate":3626.305296032935,"loss":0,"srttMs":157.02852958277754,"minRttMs":1,"sentPackets":166,"sentWireBytes":43981,"receivedPackets":133,"receivedWireBytes":101696,"lastHeardMs":14722,"unsyncedBytes":736,"inflightPackets":27,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 222/635","providerTurns":"772/772 772/772 469/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"433/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217064647,"silences":0,"longestSilenceMs":983,"longestRecoveredGapMs":983},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.19799999999999998,"srttMs":3010.743072744393,"minRttMs":1,"sentPackets":214,"sentWireBytes":59893,"receivedPackets":190,"receivedWireBytes":140101,"lastHeardMs":24748,"unsyncedBytes":167,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 279/451 141/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"640/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217074828,"silences":0,"longestSilenceMs":983,"longestRecoveredGapMs":983},"stats":{"sendRate":2311.6197183098598,"deliveredRate":559.4542708447065,"loss":0.65,"srttMs":2324.525748637977,"minRttMs":1988,"sentPackets":276,"sentWireBytes":73800,"receivedPackets":256,"receivedWireBytes":164784,"lastHeardMs":34930,"unsyncedBytes":0,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 279/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"640/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217080598,"silences":1,"longestSilenceMs":2939,"longestRecoveredGapMs":2939},"stats":{"sendRate":2311.6197183098598,"deliveredRate":443.7793935567609,"loss":0.7028749999999999,"srttMs":2324.525748637977,"minRttMs":1988,"sentPackets":303,"sentWireBytes":77645,"receivedPackets":292,"receivedWireBytes":176077,"lastHeardMs":40699,"unsyncedBytes":0,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 279/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"640/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217080598,"silences":1,"longestSilenceMs":2939,"longestRecoveredGapMs":2939},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.7028749999999999,"srttMs":2324.525748637977,"minRttMs":1988,"sentPackets":318,"sentWireBytes":81436,"receivedPackets":292,"receivedWireBytes":176077,"lastHeardMs":40699,"unsyncedBytes":457,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 66/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 279/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"640/691","runtimeRequests":"797/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217104088,"silences":2,"longestSilenceMs":18609,"longestRecoveredGapMs":18609},"stats":{"sendRate":3000,"deliveredRate":895.3617027838314,"loss":0.1771428571428571,"srttMs":2780.8436538753867,"minRttMs":1988,"sentPackets":359,"sentWireBytes":89657,"receivedPackets":322,"receivedWireBytes":185649,"lastHeardMs":64190,"unsyncedBytes":0,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 59/1093 1382/1542 923/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 310/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"640/691","runtimeRequests":"857/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217116063,"silences":3,"longestSilenceMs":18609,"longestRecoveredGapMs":18609},"stats":{"sendRate":2000,"deliveredRate":208.73998543335762,"loss":0,"srttMs":3870.68627593605,"minRttMs":1988,"sentPackets":394,"sentWireBytes":100364,"receivedPackets":371,"receivedWireBytes":207190,"lastHeardMs":76164,"unsyncedBytes":398,"inflightPackets":94,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1382/1542 923/923 277/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 122/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 375/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"673/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217125290,"silences":4,"longestSilenceMs":18609,"longestRecoveredGapMs":18609},"stats":{"sendRate":1059.7208263802272,"deliveredRate":544.0900183788008,"loss":0.3582153846153846,"srttMs":3787.2754011274824,"minRttMs":1839,"sentPackets":429,"sentWireBytes":111421,"receivedPackets":381,"receivedWireBytes":212954,"lastHeardMs":85391,"unsyncedBytes":167,"inflightPackets":118,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1382/1542 923/923 277/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 122/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 901/901 544/544 449/451 276/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"673/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":718,"lastDataAt":1794217129296,"silences":5,"longestSilenceMs":18609,"longestRecoveredGapMs":18609},"stats":{"sendRate":1059.7208263802272,"deliveredRate":431.3019593934925,"loss":0.5078936263736265,"srttMs":3814.1159759865473,"minRttMs":1839,"sentPackets":443,"sentWireBytes":113866,"receivedPackets":383,"receivedWireBytes":213980,"lastHeardMs":89398,"unsyncedBytes":0,"inflightPackets":120,"duplicateBytes":0}}
- shim trace
09:37:20.746 fetch GET /api/auth/session
09:37:20.753 fetched GET /api/auth/session 200
09:37:21.171 fetch GET /.well-known/t3/environment
09:37:21.172 fetched GET /.well-known/t3/environment 200
09:37:21.215 fetch GET /.well-known/t3/environment
09:37:21.217 fetched GET /.well-known/t3/environment 200
09:37:21.244 rpc 0 subscribeServerConfig
09:37:21.246 chunk 0 38462 bytes
09:37:21.311 in Ack 0
09:37:21.316 rpc 1 subscribeServerLifecycle
09:37:21.317 rpc 2 subscribeServerLifecycle
09:37:21.321 fetch GET /api/auth/session
09:37:21.327 fetched GET /api/auth/session 200
09:37:21.328 rpc 3 subscribeDeviceState
09:37:21.330 rpc 4 preview.reportProfiles
09:37:21.635 rpc 5 server.reportClientActivity
09:37:21.666 rpc 6 subscribeProjectClones
09:37:21.700 fetch GET /api/orchestration/threads/mcp%3Ad99a1ef6-d5f4-4fa2-9543-f571b81bd0be/bounded
09:37:21.708 fetch GET /api/orchestration/shell
09:37:21.709 fetched GET /api/orchestration/shell 200
09:37:21.712 rpc 7 orchestration.subscribeShell
09:37:21.712 chunk 7 2791 bytes
09:37:21.715 in Ack 7
09:37:22.015 chunk 1 3815 bytes
09:37:22.016 in Ack 1
09:37:22.161 fetch POST /api/observability/v1/traces
09:37:22.161 fetched POST /api/observability/v1/traces 204
09:37:22.259 chunk 7 4592 bytes
09:37:22.259 chunk 2 3815 bytes
09:37:22.260 in Ack 7
09:37:22.290 rpc 8 server.refreshProviders
09:37:22.342 rpc 9 orchestration.dispatchCommand thread.visit
09:37:22.342 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:22.357 rpc 10 subscribeTerminalMetadata
09:37:22.358 rpc 11 subscribeVcsStatus
09:37:22.358 rpc 12 subscribePreviewEvents
09:37:22.360 rpc 13 assets.createUrl
09:37:22.361 rpc 14 preview.list
09:37:22.361 rpc 15 projects.readFile
09:37:22.371 in Ack 2
09:37:22.445 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c8d4645d-912a-45e8-9766-cf293a34a76d","sessions":[]}}
09:37:22.445 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:22.445 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:56.011Z","configurableProviders":false,"con
09:37:22.661 chunk 11 1632 bytes
09:37:22.661 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:22.662 in Ack 11
09:37:22.911 chunk 7 4635 bytes
09:37:22.911 in Ack 7
09:37:23.218 fetch POST /api/observability/v1/traces
09:37:23.219 fetched POST /api/observability/v1/traces 204
09:37:23.372 chunk 3 876 bytes
09:37:23.373 in Ack 3
09:37:23.455 exit 4 {"_tag":"Success","value":null}
09:37:23.455 exit 5 {"_tag":"Success","value":null}
09:37:23.455 chunk 6 4 bytes
09:37:23.455 chunk 10 36 bytes
09:37:23.457 in Ack 6
09:37:23.458 in Ack 10
09:37:23.708 rpc 16 server.reportClientActivity
09:37:23.798 exit 16 {"_tag":"Success","value":null}
09:37:24.457 fetch POST /api/observability/v1/traces
09:37:24.457 fetched POST /api/observability/v1/traces 204
09:37:24.942 fetched GET /api/orchestration/threads/mcp%3Ad99a1ef6-d5f4-4fa2-9543-f571b81bd0be/bounded 200
09:37:24.947 rpc 17 orchestration.subscribeThread
09:37:24.947 chunk 17 8283 bytes
09:37:24.964 in Interrupt 11
09:37:24.967 rpc 18 subscribeVcsStatus
09:37:25.036 in Ack 17
09:37:25.099 chunk 18 1632 bytes
09:37:25.099 in Ack 18
09:37:25.198 chunk 17 8323 bytes
09:37:25.198 in Ack 17
09:37:25.260 rpc 19 server.reportClientActivity
09:37:25.458 chunk 17 11919 bytes
09:37:25.461 in Ack 17
09:37:25.707 chunk 17 13498 bytes
09:37:25.708 in Ack 17
09:37:25.730 exit 19 {"_tag":"Success","value":null}
09:37:25.944 fetch POST /api/observability/v1/traces
09:37:25.944 fetched POST /api/observability/v1/traces 204
09:37:25.957 chunk 17 16151 bytes
09:37:25.959 in Ack 17
09:37:26.258 rpc 20 attachments.createUploadUrl
09:37:26.331 chunk 17 17977 bytes
09:37:26.331 in Ping 
09:37:26.350 in Ack 17
09:37:26.450 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-771318f4-5790-48dd-b6c3-464e3b111801","expiresAt":1794217646388,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:26.452 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTc3MTMxOGY0LTU3OTAtNDhkZC1iNmMzLTQ2NGUzYjExMTgwMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0NjM4OH0.117LPgtLEKv-oRMHHu0zqrO5u4kXhN6wx8BFFPTpnB0
09:37:26.581 chunk 17 20409 bytes
09:37:26.583 in Ack 17
09:37:26.599 in Interrupt 18
09:37:26.603 rpc 21 subscribeVcsStatus
09:37:26.690 chunk 21 1632 bytes
09:37:26.690 in Ack 21
09:37:26.832 chunk 17 21100 bytes
09:37:26.833 in Ack 17
09:37:26.856 rpc 22 server.reportClientActivity
09:37:26.960 fetch POST /api/observability/v1/traces
09:37:26.960 fetched POST /api/observability/v1/traces 204
09:37:28.389 exit 22 {"_tag":"Success","value":null}
09:37:29.553 rpc 23 orchestration.dispatchCommand message.dispatch
09:37:29.818 fetch POST /api/observability/v1/traces
09:37:29.818 fetched POST /api/observability/v1/traces 204
09:37:29.852 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:37:29.856 rpc 24 attachments.delete
09:37:30.102 chunk 17 25348 bytes
09:37:30.104 in Ack 17
09:37:30.107 rpc 25 assets.createUrl
09:37:30.352 chunk 7 4721 bytes
09:37:30.352 chunk 17 26024 bytes
09:37:30.353 in Ack 7
09:37:30.363 in Ack 17
09:37:30.542 exit 24 {"_tag":"Success","value":null}
09:37:30.603 chunk 17 27723 bytes
09:37:30.605 in Ack 17
09:37:30.853 chunk 7 4660 bytes
09:37:30.853 chunk 17 30204 bytes
09:37:30.854 in Ack 7
09:37:30.863 in Ack 17
09:37:30.954 fetch POST /api/observability/v1/traces
09:37:30.954 fetched POST /api/observability/v1/traces 204
09:37:31.104 chunk 17 30320 bytes
09:37:31.107 in Ack 17
09:37:31.331 in Ping 
09:37:31.355 chunk 17 32035 bytes
09:37:31.357 in Ack 17
09:37:31.605 chunk 7 4657 bytes
09:37:31.605 chunk 17 34482 bytes
09:37:31.606 in Ack 7
09:37:31.609 rpc 26 orchestration.dispatchCommand thread.visit
09:37:31.609 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:31.619 in Ack 17
09:37:31.860 chunk 7 4657 bytes
09:37:31.860 chunk 17 35260 bytes
09:37:31.863 in Ack 7
09:37:31.888 in Ack 17
09:37:31.900 in Interrupt 21
09:37:31.903 rpc 27 subscribeVcsStatus
09:37:32.329 chunk 27 1713 bytes
09:37:32.330 fetch POST /api/observability/v1/traces
09:37:32.330 fetched POST /api/observability/v1/traces 204
09:37:32.331 chunk 17 35260 bytes
09:37:32.332 rpc 28 server.reportClientActivity
09:37:32.334 in Ack 27
09:37:32.340 in Ack 17
09:37:32.494 rpc 29 server.refreshProviders
09:37:32.494 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:56.011Z","configurableProviders":false,"con
09:37:33.337 exit 28 {"_tag":"Success","value":null}
09:37:33.338 fetch POST /api/observability/v1/traces
09:37:33.338 fetched POST /api/observability/v1/traces 204
09:37:33.690 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/9fd060ce-5e03-412a-b7bf-091251343aa3","expiresAt":1794220653690}}
09:37:34.558 rpc 30 orchestration.dispatchCommand message.dispatch
09:37:34.623 fetch POST /api/observability/v1/traces
09:37:34.623 fetched POST /api/observability/v1/traces 204
09:37:36.331 in Ping 
09:37:36.572 fetch POST /api/observability/v1/traces
09:37:36.572 fetched POST /api/observability/v1/traces 204
09:37:40.228 exit 30 {"_tag":"Success","value":{"sequence":152}}
09:37:41.182 chunk 17 36100 bytes
09:37:41.185 in Ack 17
09:37:41.280 fetch POST /api/observability/v1/traces
09:37:41.280 fetched POST /api/observability/v1/traces 204
09:37:41.334 in Ping 
09:37:41.433 chunk 17 38121 bytes
09:37:41.436 in Ack 17
09:37:41.936 chunk 17 38907 bytes
09:37:41.938 in Ack 17
09:37:42.187 chunk 17 38909 bytes
09:37:42.189 in Ack 17
09:37:42.440 fetch POST /api/observability/v1/traces
09:37:42.440 fetched POST /api/observability/v1/traces 204
09:37:43.940 chunk 7 4721 bytes
09:37:43.945 in Ack 7
09:37:43.949 rpc 31 orchestration.dispatchCommand thread.visit
09:37:43.949 exit 31 {"_tag":"Success","value":{"sequence":0}}
09:37:44.946 fetch POST /api/observability/v1/traces
09:37:44.946 fetched POST /api/observability/v1/traces 204
09:37:45.075 chunk 17 39564 bytes
09:37:45.077 in Ack 17
09:37:46.079 fetch POST /api/observability/v1/traces
09:37:46.079 fetched POST /api/observability/v1/traces 204
09:37:46.334 in Ping 
09:37:46.430 rpc 32 server.reportClientActivity
09:37:47.433 fetch POST /api/observability/v1/traces
09:37:47.433 fetched POST /api/observability/v1/traces 204
09:37:48.079 chunk 17 40200 bytes
09:37:48.080 in Ack 17
09:37:49.081 fetch POST /api/observability/v1/traces
09:37:49.081 fetched POST /api/observability/v1/traces 204
09:37:50.832 chunk 17 43293 bytes
09:37:50.835 in Ack 17
09:37:51.111 chunk 17 43297 bytes
09:37:51.113 in Ack 17
09:37:51.336 in Ping 
09:37:51.361 chunk 17 43613 bytes
09:37:51.365 in Ack 17
09:37:51.837 fetch POST /api/observability/v1/traces
09:37:51.837 fetched POST /api/observability/v1/traces 204
09:37:51.941 chunk 17 43565 bytes
09:37:51.943 in Ack 17
09:37:52.946 fetch POST /api/observability/v1/traces
09:37:52.947 fetched POST /api/observability/v1/traces 204
09:37:53.694 chunk 17 43565 bytes
09:37:53.696 in Ack 17
09:37:54.697 fetch POST /api/observability/v1/traces
09:37:54.697 fetched POST /api/observability/v1/traces 204
09:37:56.336 in Ping 
09:38:01.337 in Ping 
09:38:01.436 in Interrupt 32
09:38:02.438 fetch POST /api/observability/v1/traces
09:38:02.438 fetched POST /api/observability/v1/traces 204
09:38:06.338 in Ping 
09:38:11.338 in Ping 
09:38:11.430 rpc 33 server.reportClientActivity
09:38:12.431 fetch POST /api/observability/v1/traces
09:38:12.431 fetched POST /api/observability/v1/traces 204
09:38:16.342 in Ping 
09:38:19.209 chunk 17 43623 bytes
09:38:19.211 in Ack 17
09:38:20.090 chunk 17 44045 bytes
09:38:20.092 in Ack 17
09:38:20.212 fetch POST /api/observability/v1/traces
09:38:20.212 fetched POST /api/observability/v1/traces 204
09:38:20.522 chunk 17 44047 bytes
09:38:20.523 in Ack 17
09:38:20.906 chunk 17 44483 bytes
09:38:20.908 in Ack 17
09:38:21.156 chunk 17 44507 bytes
09:38:21.158 in Ack 17
09:38:21.346 in Ping 
09:38:21.407 chunk 17 44511 bytes
09:38:21.409 in Ack 17
09:38:21.525 fetch POST /api/observability/v1/traces
09:38:21.525 fetched POST /api/observability/v1/traces 204
09:38:21.842 chunk 17 44903 bytes
09:38:21.844 in Ack 17
09:38:22.669 chunk 17 45253 bytes
09:38:22.671 in Ack 17
09:38:23.160 fetch POST /api/observability/v1/traces
09:38:23.160 fetched POST /api/observability/v1/traces 204
09:38:26.350 in Ping 
09:38:26.431 in Interrupt 33
09:38:27.432 fetch POST /api/observability/v1/traces
09:38:27.433 fetched POST /api/observability/v1/traces 204
09:38:28.265 chunk 17 46255 bytes
09:38:28.275 in Ack 17
09:38:28.591 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:38:29.276 fetch POST /api/observability/v1/traces
09:38:29.276 fetched POST /api/observability/v1/traces 204
09:38:31.354 in Ping 
09:38:34.863 chunk 17 48323 bytes
09:38:34.866 in Ack 17
09:38:34.985 exit 34 {"_tag":"Success","value":{"sequence":208}}
09:38:35.867 fetch POST /api/observability/v1/traces
09:38:35.868 fetched POST /api/observability/v1/traces 204
09:38:36.033 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:38:36.358 in Ping 
09:38:36.432 rpc 36 server.reportClientActivity
09:38:37.034 fetch POST /api/observability/v1/traces
09:38:37.034 fetched POST /api/observability/v1/traces 204
09:38:37.280 chunk 17 48366 bytes
09:38:37.282 in Ack 17
09:38:37.983 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:38.042 chunk 17 48416 bytes
09:38:38.045 in Ack 17
09:38:38.283 fetch POST /api/observability/v1/traces
09:38:38.283 fetched POST /api/observability/v1/traces 204
09:38:41.362 in Ping 
09:38:42.283 chunk 7 4723 bytes
09:38:42.284 in Ack 7
09:38:42.287 rpc 37 orchestration.dispatchCommand thread.visit
09:38:42.287 exit 37 {"_tag":"Success","value":{"sequence":0}}
09:38:43.285 fetch POST /api/observability/v1/traces
09:38:43.285 fetched POST /api/observability/v1/traces 204
09:38:43.764 chunk 17 51076 bytes
09:38:43.766 in Ack 17
09:38:44.767 fetch POST /api/observability/v1/traces
09:38:44.767 fetched POST /api/observability/v1/traces 204
09:38:46.366 in Ping 
09:38:51.370 in Ping 
09:38:51.432 in Interrupt 36
09:38:52.433 fetch POST /api/observability/v1/traces
09:38:52.433 fetched POST /api/observability/v1/traces 204
09:38:56.374 in Ping 
09:39:01.378 in Ping 
09:39:01.432 rpc 38 server.reportClientActivity
09:39:02.433 fetch POST /api/observability/v1/traces
09:39:02.433 fetched POST /api/observability/v1/traces 204
09:39:05.765 chunk 17 51372 bytes
09:39:05.768 in Ack 17
09:39:06.016 chunk 17 51838 bytes
09:39:06.018 in Ack 17
09:39:06.382 in Ping 
09:39:06.770 fetch POST /api/observability/v1/traces
09:39:06.770 fetched POST /api/observability/v1/traces 204
09:39:06.987 chunk 17 51912 bytes
09:39:06.989 in Ack 17
09:39:07.991 fetch POST /api/observability/v1/traces
09:39:07.991 fetched POST /api/observability/v1/traces 204
09:39:08.094 chunk 17 52330 bytes
09:39:08.095 in Ack 17
