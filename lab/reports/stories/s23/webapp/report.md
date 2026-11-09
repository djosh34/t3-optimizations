# story, webapp, warm start, seed 23

Profile story (graded): seed 23's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 87 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 12.5s |
| Prompt reached the agent | 11.1s |
| Time to first text | 23.1s |
| Whole reply shown | 85.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 25.8s / 6.8s / 22.6s |
| Longest silence | 24.9s |
| Approval shown after it was asked | 13.1s |
| Bytes on the wire down / up | 311743 / 136644 |
| Bytes the app received / sent | 135593 / 62479 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 8.8 | 0 | 357 |
| 30 | 8.8 | 397 | 758 |
| 40 | 18.8 | 397 | 1077 |
| 50 | 23.6 | 609 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 9.9 | 1077 | 1461 |
| 80 | 12.2 | 1378 | 1596 |
| 85 | 14.6 | 1472 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217174702,"silences":0,"longestSilenceMs":468,"longestRecoveredGapMs":468},"stats":{"sendRate":2000,"deliveredRate":3136.9591527875455,"loss":0,"srttMs":3.0749274467930148,"minRttMs":null,"sentPackets":131,"sentWireBytes":34676,"receivedPackets":121,"receivedWireBytes":91586,"lastHeardMs":11123,"unsyncedBytes":736,"inflightPackets":9,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217184499,"silences":1,"longestSilenceMs":3002,"longestRecoveredGapMs":3002},"stats":{"sendRate":2000,"deliveredRate":2960.7462911032744,"loss":0,"srttMs":3.0749274467930148,"minRttMs":null,"sentPackets":144,"sentWireBytes":42482,"receivedPackets":129,"receivedWireBytes":96431,"lastHeardMs":20920,"unsyncedBytes":736,"inflightPackets":22,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659 498/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217196509,"silences":2,"longestSilenceMs":3951,"longestRecoveredGapMs":3951},"stats":{"sendRate":3000,"deliveredRate":2104.942796446768,"loss":0.24705882352941175,"srttMs":1303.065561515944,"minRttMs":10403,"sentPackets":169,"sentWireBytes":49629,"receivedPackets":158,"receivedWireBytes":108517,"lastHeardMs":32931,"unsyncedBytes":457,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 304/654","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217206721,"silences":2,"longestSilenceMs":3951,"longestRecoveredGapMs":3951},"stats":{"sendRate":2000,"deliveredRate":520.7394048692515,"loss":0.18181818181818177,"srttMs":3233.0592167464156,"minRttMs":2924,"sentPackets":233,"sentWireBytes":63682,"receivedPackets":232,"receivedWireBytes":139244,"lastHeardMs":43143,"unsyncedBytes":167,"inflightPackets":50,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 543/543 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217211809,"silences":2,"longestSilenceMs":3951,"longestRecoveredGapMs":3951},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.2690681818181817,"srttMs":2692.52558182767,"minRttMs":2524,"sentPackets":274,"sentWireBytes":71000,"receivedPackets":298,"receivedWireBytes":165062,"lastHeardMs":48230,"unsyncedBytes":457,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 633/1093 234/923 696/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1205/1368 959/959 250/1016","messages":"544/544 456/456 901/901 544/544 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217226990,"silences":3,"longestSilenceMs":8432,"longestRecoveredGapMs":8432},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.2690681818181817,"srttMs":2692.52558182767,"minRttMs":2524,"sentPackets":281,"sentWireBytes":72995,"receivedPackets":327,"receivedWireBytes":181768,"lastHeardMs":63411,"unsyncedBytes":624,"inflightPackets":55,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 595/923 1186/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1205/1368 959/959 250/1016","messages":"544/544 456/456 901/901 544/544 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217236216,"silences":4,"longestSilenceMs":8432,"longestRecoveredGapMs":8432},"stats":{"sendRate":2000,"deliveredRate":155.7816431462641,"loss":0.8214285714285714,"srttMs":4009.334884099211,"minRttMs":2524,"sentPackets":293,"sentWireBytes":78754,"receivedPackets":353,"receivedWireBytes":195455,"lastHeardMs":72637,"unsyncedBytes":565,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 595/923 1186/1542 283/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1205/1368 959/959 250/1016","messages":"544/544 456/456 901/901 544/544 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217248145,"silences":5,"longestSilenceMs":8432,"longestRecoveredGapMs":8432},"stats":{"sendRate":1000,"deliveredRate":692.6116844721129,"loss":0.48977272727272725,"srttMs":4512.717727672459,"minRttMs":2524,"sentPackets":333,"sentWireBytes":89693,"receivedPackets":391,"receivedWireBytes":214280,"lastHeardMs":84566,"unsyncedBytes":0,"inflightPackets":69,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 595/923 1186/1542 64/1093","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 540/723","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1205/1368 959/959 250/1016","messages":"544/544 456/456 901/901 544/544 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":627,"lastDataAt":1794217259212,"silences":6,"longestSilenceMs":8432,"longestRecoveredGapMs":8432},"stats":{"sendRate":750,"deliveredRate":545.933165883482,"loss":0.2399886363636363,"srttMs":5628.479233685341,"minRttMs":2524,"sentPackets":355,"sentWireBytes":92852,"receivedPackets":414,"receivedWireBytes":223276,"lastHeardMs":95633,"unsyncedBytes":0,"inflightPackets":54,"duplicateBytes":0}}
- shim trace
09:39:24.112 fetch GET /api/auth/session
09:39:24.124 fetched GET /api/auth/session 200
09:39:24.463 fetch GET /.well-known/t3/environment
09:39:24.465 fetched GET /.well-known/t3/environment 200
09:39:24.476 fetch GET /.well-known/t3/environment
09:39:24.477 fetched GET /.well-known/t3/environment 200
09:39:24.482 rpc 0 subscribeServerConfig
09:39:24.482 chunk 0 38462 bytes
09:39:24.528 in Ack 0
09:39:24.530 rpc 1 subscribeServerLifecycle
09:39:24.531 rpc 2 subscribeServerLifecycle
09:39:24.535 fetch GET /api/auth/session
09:39:24.536 fetched GET /api/auth/session 200
09:39:24.537 rpc 3 subscribeDeviceState
09:39:24.539 rpc 4 preview.reportProfiles
09:39:24.594 rpc 5 subscribeProjectClones
09:39:24.601 fetch GET /api/orchestration/threads/mcp%3Ae04122a6-10e8-47c4-95d5-80a77c70d061/bounded
09:39:24.628 fetch GET /api/orchestration/shell
09:39:24.630 fetched GET /api/orchestration/shell 200
09:39:24.633 rpc 6 orchestration.subscribeShell
09:39:24.633 chunk 6 4619 bytes
09:39:24.666 rpc 7 server.refreshProviders
09:39:24.732 rpc 8 orchestration.dispatchCommand thread.visit
09:39:24.732 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:39:24.761 rpc 9 subscribeTerminalMetadata
09:39:24.762 rpc 10 subscribeVcsStatus
09:39:24.762 rpc 11 subscribePreviewEvents
09:39:24.764 rpc 12 assets.createUrl
09:39:24.765 rpc 13 preview.list
09:39:24.766 rpc 14 projects.readFile
09:39:24.769 in Ack 6
09:39:24.943 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"bb87c478-51df-43a1-88d1-1cc0d2d99d5a","sessions":[]}}
09:39:24.943 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:39:25.017 rpc 15 server.reportClientActivity
09:39:25.211 chunk 1 3815 bytes
09:39:25.211 in Ack 1
09:39:25.405 chunk 2 3815 bytes
09:39:25.405 in Ack 2
09:39:25.460 fetch POST /api/observability/v1/traces
09:39:25.460 fetched POST /api/observability/v1/traces 204
09:39:25.617 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:39:03.634Z","configurableProviders":false,"con
09:39:25.617 chunk 10 1632 bytes
09:39:25.621 in Ack 10
09:39:25.797 chunk 6 4595 bytes
09:39:25.797 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:39:25.798 in Ack 6
09:39:26.047 chunk 6 4638 bytes
09:39:26.048 in Ack 6
09:39:26.297 chunk 6 4635 bytes
09:39:26.298 in Ack 6
09:39:26.523 chunk 3 876 bytes
09:39:26.523 exit 4 {"_tag":"Success","value":null}
09:39:26.523 chunk 5 4 bytes
09:39:26.523 chunk 9 36 bytes
09:39:26.523 exit 15 {"_tag":"Success","value":null}
09:39:26.524 in Ack 3
09:39:26.529 rpc 16 preview.reportProfiles
09:39:26.529 in Ack 5
09:39:26.529 in Ack 9
09:39:26.605 exit 16 {"_tag":"Success","value":null}
09:39:26.618 fetch POST /api/observability/v1/traces
09:39:26.618 fetched POST /api/observability/v1/traces 204
09:39:27.960 fetched GET /api/orchestration/threads/mcp%3Ae04122a6-10e8-47c4-95d5-80a77c70d061/bounded 200
09:39:27.964 rpc 17 orchestration.subscribeThread
09:39:27.965 chunk 17 8283 bytes
09:39:27.976 in Interrupt 10
09:39:27.978 rpc 18 subscribeVcsStatus
09:39:27.979 in Ack 17
09:39:28.118 chunk 18 1632 bytes
09:39:28.119 in Ack 18
09:39:28.216 chunk 17 9337 bytes
09:39:28.217 in Ack 17
09:39:28.255 rpc 19 server.reportClientActivity
09:39:28.433 exit 19 {"_tag":"Success","value":null}
09:39:28.477 chunk 17 12691 bytes
09:39:28.478 in Ack 17
09:39:28.726 chunk 17 13498 bytes
09:39:28.729 in Ack 17
09:39:28.770 rpc 20 attachments.createUploadUrl
09:39:28.896 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e425552e-7e8b-43b7-8818-736944e46b77","expiresAt":1794217768873,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:39:28.898 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWU0MjU1NTJlLTdlOGItNDNiNy04ODE4LTczNjk0NGU0NmI3NyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzc2ODg3M30.bipSAc8RyIKWw3dA0uyEJZLrqJAD4C5wj4-4pd0pILE
09:39:28.962 fetch POST /api/observability/v1/traces
09:39:28.962 fetched POST /api/observability/v1/traces 204
09:39:28.977 chunk 17 16151 bytes
09:39:28.978 in Ack 17
09:39:29.227 chunk 17 19020 bytes
09:39:29.228 in Ack 17
09:39:29.496 chunk 17 21100 bytes
09:39:29.496 in Ping 
09:39:29.521 in Ack 17
09:39:29.541 in Interrupt 18
09:39:29.543 rpc 21 subscribeVcsStatus
09:39:29.613 chunk 21 1632 bytes
09:39:29.614 in Ack 21
09:39:29.794 rpc 22 server.reportClientActivity
09:39:29.979 fetch POST /api/observability/v1/traces
09:39:29.979 fetched POST /api/observability/v1/traces 204
09:39:31.709 exit 22 {"_tag":"Success","value":null}
09:39:32.150 rpc 23 orchestration.dispatchCommand message.dispatch
09:39:32.369 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:39:32.371 rpc 24 attachments.delete
09:39:32.459 exit 24 {"_tag":"Success","value":null}
09:39:32.635 chunk 6 4721 bytes
09:39:32.636 in Ack 6
09:39:32.697 chunk 17 25369 bytes
09:39:32.702 in Ack 17
09:39:32.705 rpc 25 assets.createUrl
09:39:32.745 fetch POST /api/observability/v1/traces
09:39:32.745 fetched POST /api/observability/v1/traces 204
09:39:32.947 chunk 17 26812 bytes
09:39:32.949 in Ack 17
09:39:33.200 chunk 17 29269 bytes
09:39:33.201 in Ack 17
09:39:33.386 chunk 6 4660 bytes
09:39:33.389 in Ack 6
09:39:33.456 chunk 17 29409 bytes
09:39:33.458 in Ack 17
09:39:33.705 chunk 17 31903 bytes
09:39:33.707 in Ack 17
09:39:33.890 chunk 6 4657 bytes
09:39:33.891 in Ack 6
09:39:33.895 rpc 26 orchestration.dispatchCommand thread.visit
09:39:33.895 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:39:33.950 fetch POST /api/observability/v1/traces
09:39:33.950 fetched POST /api/observability/v1/traces 204
09:39:33.956 chunk 17 32035 bytes
09:39:33.957 in Ack 17
09:39:34.142 chunk 6 4657 bytes
09:39:34.143 in Ack 6
09:39:34.206 chunk 17 35260 bytes
09:39:34.208 in Ack 17
09:39:34.220 in Interrupt 21
09:39:34.225 rpc 27 subscribeVcsStatus
09:39:34.258 chunk 27 1713 bytes
09:39:34.259 in Ack 27
09:39:34.693 rpc 28 server.reportClientActivity
09:39:34.693 in Ping 
09:39:34.702 exit 28 {"_tag":"Success","value":null}
09:39:35.103 fetch POST /api/observability/v1/traces
09:39:35.103 fetched POST /api/observability/v1/traces 204
09:39:35.633 rpc 29 server.refreshProviders
09:39:35.633 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:39:03.634Z","configurableProviders":false,"con
09:39:36.278 rpc 30 orchestration.dispatchCommand message.dispatch
09:39:36.329 fetch POST /api/observability/v1/traces
09:39:36.329 fetched POST /api/observability/v1/traces 204
09:39:39.571 fetch POST /api/observability/v1/traces
09:39:39.571 fetched POST /api/observability/v1/traces 204
09:39:39.693 in Ping 
09:39:41.245 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/a53f71cb-2998-472c-9e40-2b0c0bd335ec","expiresAt":1794220781245}}
09:39:42.254 fetch POST /api/observability/v1/traces
09:39:42.254 fetched POST /api/observability/v1/traces 204
09:39:44.694 in Ping 
09:39:48.451 exit 30 {"_tag":"Success","value":{"sequence":153}}
09:39:49.452 fetch POST /api/observability/v1/traces
09:39:49.452 fetched POST /api/observability/v1/traces 204
09:39:49.696 in Ping 
09:39:49.722 rpc 31 server.reportClientActivity
09:39:50.741 fetch POST /api/observability/v1/traces
09:39:50.741 fetched POST /api/observability/v1/traces 204
09:39:54.696 in Ping 
09:39:58.658 chunk 17 38345 bytes
09:39:58.660 in Ack 17
09:39:59.155 chunk 17 38673 bytes
09:39:59.160 in Ack 17
09:39:59.662 fetch POST /api/observability/v1/traces
09:39:59.662 fetched POST /api/observability/v1/traces 204
09:39:59.696 in Ping 
09:40:00.541 chunk 17 38625 bytes
09:40:00.542 in Ack 17
09:40:01.543 fetch POST /api/observability/v1/traces
09:40:01.543 fetched POST /api/observability/v1/traces 204
09:40:04.071 chunk 6 4721 bytes
09:40:04.071 chunk 17 39261 bytes
09:40:04.072 in Ack 6
09:40:04.075 rpc 32 orchestration.dispatchCommand thread.visit
09:40:04.075 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:40:04.119 in Ack 17
09:40:04.696 in Ping 
09:40:04.724 in Interrupt 31
09:40:05.073 fetch POST /api/observability/v1/traces
09:40:05.073 fetched POST /api/observability/v1/traces 204
09:40:06.326 chunk 17 40101 bytes
09:40:06.329 in Ack 17
09:40:06.577 chunk 17 43166 bytes
09:40:06.579 in Ack 17
09:40:07.330 fetch POST /api/observability/v1/traces
09:40:07.330 fetched POST /api/observability/v1/traces 204
09:40:08.835 chunk 6 4721 bytes
09:40:08.836 in Ack 6
09:40:09.336 chunk 17 43166 bytes
09:40:09.338 in Ack 17
09:40:09.587 chunk 17 43821 bytes
09:40:09.589 in Ack 17
09:40:09.697 in Ping 
09:40:09.759 fetch POST /api/observability/v1/traces
09:40:09.759 fetched POST /api/observability/v1/traces 204
09:40:10.587 chunk 17 44607 bytes
09:40:10.589 in Ack 17
09:40:10.837 chunk 17 44609 bytes
09:40:10.839 in Ack 17
09:40:11.088 chunk 17 45134 bytes
09:40:11.091 in Ack 17
09:40:11.337 chunk 17 45134 bytes
09:40:11.339 in Ack 17
09:40:11.590 fetch POST /api/observability/v1/traces
09:40:11.590 fetched POST /api/observability/v1/traces 204
09:40:11.838 chunk 6 5092 bytes
09:40:11.838 in Ack 6
09:40:12.839 fetch POST /api/observability/v1/traces
09:40:12.839 fetched POST /api/observability/v1/traces 204
09:40:14.697 in Ping 
09:40:14.719 rpc 33 server.reportClientActivity
09:40:15.167 fetch POST /api/observability/v1/traces
09:40:15.167 fetched POST /api/observability/v1/traces 204
09:40:19.697 in Ping 
09:40:24.029 chunk 17 45917 bytes
09:40:24.033 in Ack 17
09:40:24.701 in Ping 
09:40:25.035 fetch POST /api/observability/v1/traces
09:40:25.035 fetched POST /api/observability/v1/traces 204
09:40:25.675 chunk 6 5399 bytes
09:40:25.676 in Ack 6
09:40:25.679 rpc 34 orchestration.dispatchCommand thread.visit
09:40:25.679 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:40:26.176 chunk 17 46878 bytes
09:40:26.181 in Ack 17
09:40:26.425 chunk 17 48627 bytes
09:40:26.427 in Ack 17
09:40:26.785 fetch POST /api/observability/v1/traces
09:40:26.785 fetched POST /api/observability/v1/traces 204
09:40:27.045 chunk 17 49629 bytes
09:40:27.047 in Ack 17
09:40:27.335 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:40:27.797 chunk 17 49679 bytes
09:40:27.798 in Ack 17
09:40:28.048 fetch POST /api/observability/v1/traces
09:40:28.048 fetched POST /api/observability/v1/traces 204
09:40:29.705 in Ping 
09:40:29.720 in Interrupt 33
09:40:30.722 fetch POST /api/observability/v1/traces
09:40:30.722 fetched POST /api/observability/v1/traces 204
09:40:34.709 in Ping 
09:40:37.806 exit 35 {"_tag":"Success","value":{"sequence":187}}
09:40:38.301 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:40:38.808 fetch POST /api/observability/v1/traces
09:40:38.808 fetched POST /api/observability/v1/traces 204
09:40:39.713 in Ping 
09:40:39.719 rpc 37 server.reportClientActivity
09:40:40.720 fetch POST /api/observability/v1/traces
09:40:40.720 fetched POST /api/observability/v1/traces 204
09:40:41.432 chunk 17 49722 bytes
09:40:41.432 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:40:41.435 in Ack 17
09:40:42.436 fetch POST /api/observability/v1/traces
09:40:42.436 fetched POST /api/observability/v1/traces 204
09:40:44.717 in Ping 
09:40:47.344 chunk 17 52683 bytes
09:40:47.346 in Ack 17
09:40:47.594 chunk 17 52715 bytes
09:40:47.596 in Ack 17
09:40:47.849 chunk 17 52957 bytes
09:40:47.851 in Ack 17
09:40:48.348 fetch POST /api/observability/v1/traces
09:40:48.348 fetched POST /api/observability/v1/traces 204
09:40:49.223 chunk 17 52909 bytes
09:40:49.225 in Ack 17
09:40:49.721 in Ping 
09:40:50.228 fetch POST /api/observability/v1/traces
09:40:50.228 fetched POST /api/observability/v1/traces 204
09:40:54.720 in Interrupt 37
09:40:54.725 in Ping 
09:40:55.721 fetch POST /api/observability/v1/traces
09:40:55.721 fetched POST /api/observability/v1/traces 204
09:40:58.590 chunk 17 53187 bytes
09:40:58.593 in Ack 17
09:40:59.594 fetch POST /api/observability/v1/traces
09:40:59.594 fetched POST /api/observability/v1/traces 204
09:40:59.729 in Ping 
09:41:00.871 chunk 17 53237 bytes
09:41:00.873 in Ack 17
09:41:01.267 chunk 17 53637 bytes
09:41:01.271 in Ack 17
