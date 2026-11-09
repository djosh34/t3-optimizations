# story, webapp, warm start, seed 19

Profile story (graded): seed 19's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 77 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.0s |
| Prompt reached the agent | 2.5s |
| Time to first text | 15.8s |
| Whole reply shown | 74.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 37.4s / 9.6s / 35.4s |
| Longest silence | 31.4s |
| Approval shown after it was asked | 25.2s |
| Bytes on the wire down / up | 339372 / 159542 |
| Bytes the app received / sent | 146378 / 66846 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 303 |
| 20 | 7.4 | 397 | 701 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 27.4 | 397 | 1077 |
| 50 | 37.4 | 397 | 1077 |
| 60 | 4.3 | 1077 | 1254 |
| 70 | 6.6 | 1378 | 1596 |
| 74 | 10.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217049479,"silences":0,"longestSilenceMs":1442,"longestRecoveredGapMs":1442},"stats":{"sendRate":4500,"deliveredRate":1780.4281345565748,"loss":0,"srttMs":7.503070357963824,"minRttMs":1,"sentPackets":149,"sentWireBytes":41665,"receivedPackets":143,"receivedWireBytes":101259,"lastHeardMs":16116,"unsyncedBytes":736,"inflightPackets":23,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659 240/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217059499,"silences":0,"longestSilenceMs":1442,"longestRecoveredGapMs":1442},"stats":{"sendRate":1304.2729228908006,"deliveredRate":0,"loss":0.3318181818181818,"srttMs":4359.120233859452,"minRttMs":1,"sentPackets":192,"sentWireBytes":54408,"receivedPackets":219,"receivedWireBytes":138285,"lastHeardMs":26137,"unsyncedBytes":167,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 943/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"509/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 736/785","messages":"456/456 544/544 901/901 544/544 298/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"124/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217069633,"silences":0,"longestSilenceMs":1442,"longestRecoveredGapMs":1442},"stats":{"sendRate":1074.4908672598308,"deliveredRate":0,"loss":0.06227272727272727,"srttMs":3428.8480803834655,"minRttMs":3164,"sentPackets":239,"sentWireBytes":65391,"receivedPackets":308,"receivedWireBytes":176453,"lastHeardMs":36270,"unsyncedBytes":167,"inflightPackets":27,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1051/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"807/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"331/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217072232,"silences":0,"longestSilenceMs":1442,"longestRecoveredGapMs":1442},"stats":{"sendRate":1611.7363008897462,"deliveredRate":0,"loss":0.06227272727272727,"srttMs":3152.8064553224367,"minRttMs":2601,"sentPackets":253,"sentWireBytes":68298,"receivedPackets":328,"receivedWireBytes":184248,"lastHeardMs":38869,"unsyncedBytes":0,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1051/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 770/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"807/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"331/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217089520,"silences":1,"longestSilenceMs":14288,"longestRecoveredGapMs":14288},"stats":{"sendRate":1611.7363008897462,"deliveredRate":0,"loss":0.06227272727272727,"srttMs":3110.7056484071322,"minRttMs":2601,"sentPackets":274,"sentWireBytes":73779,"receivedPackets":332,"receivedWireBytes":185476,"lastHeardMs":56158,"unsyncedBytes":457,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1051/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1285/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"807/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"331/691","runtimeRequests":"381/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217100260,"silences":2,"longestSilenceMs":14288,"longestRecoveredGapMs":14288},"stats":{"sendRate":1611.7363008897462,"deliveredRate":98.14230634419908,"loss":0.8333333333333334,"srttMs":4169.742442356241,"minRttMs":2601,"sentPackets":284,"sentWireBytes":76616,"receivedPackets":360,"receivedWireBytes":194896,"lastHeardMs":66897,"unsyncedBytes":296,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1051/1068 1186/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"807/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"331/691","runtimeRequests":"833/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217110343,"silences":3,"longestSilenceMs":14288,"longestRecoveredGapMs":14288},"stats":{"sendRate":2278.470254957507,"deliveredRate":1518.9801699716716,"loss":0,"srttMs":2359.234629620324,"minRttMs":1353,"sentPackets":337,"sentWireBytes":92904,"receivedPackets":395,"receivedWireBytes":207765,"lastHeardMs":76981,"unsyncedBytes":398,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 314/1566 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 469/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"381/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1233,"lastDataAt":1794217120919,"silences":3,"longestSilenceMs":14288,"longestRecoveredGapMs":14288},"stats":{"sendRate":1139.2351274787536,"deliveredRate":1596.1526796668172,"loss":0,"srttMs":7710.208981624069,"minRttMs":1353,"sentPackets":362,"sentWireBytes":99928,"receivedPackets":457,"receivedWireBytes":236707,"lastHeardMs":87557,"unsyncedBytes":457,"inflightPackets":67,"duplicateBytes":0}}
- shim trace
09:37:15.024 fetch GET /api/auth/session
09:37:15.030 fetched GET /api/auth/session 200
09:37:15.567 fetch GET /.well-known/t3/environment
09:37:15.568 fetched GET /.well-known/t3/environment 200
09:37:15.590 fetch GET /.well-known/t3/environment
09:37:15.592 fetched GET /.well-known/t3/environment 200
09:37:15.601 rpc 0 subscribeServerConfig
09:37:15.602 chunk 0 38462 bytes
09:37:15.682 in Ack 0
09:37:15.689 rpc 1 subscribeServerLifecycle
09:37:15.690 rpc 2 subscribeServerLifecycle
09:37:15.695 fetch GET /api/auth/session
09:37:15.697 fetched GET /api/auth/session 200
09:37:15.697 rpc 3 subscribeDeviceState
09:37:15.700 rpc 4 preview.reportProfiles
09:37:15.844 rpc 5 subscribeProjectClones
09:37:15.865 fetch GET /api/orchestration/threads/mcp%3Aad0b54c9-f3ee-4cd0-8935-9013b1ce950a/bounded
09:37:15.871 rpc 6 server.reportClientActivity
09:37:15.929 fetch GET /api/orchestration/shell
09:37:15.997 rpc 7 server.refreshProviders
09:37:16.109 rpc 8 orchestration.dispatchCommand thread.visit
09:37:16.109 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:16.143 fetched GET /api/orchestration/shell 200
09:37:16.144 rpc 9 subscribeTerminalMetadata
09:37:16.145 rpc 10 subscribeVcsStatus
09:37:16.146 rpc 11 subscribePreviewEvents
09:37:16.151 rpc 12 orchestration.subscribeShell
09:37:16.151 chunk 12 4619 bytes
09:37:16.155 rpc 13 assets.createUrl
09:37:16.156 rpc 14 preview.list
09:37:16.158 rpc 15 projects.readFile
09:37:16.169 in Ack 12
09:37:16.316 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"dd6d6583-d283-4845-9a61-7129d0e548f8","sessions":[]}}
09:37:16.316 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:16.743 chunk 1 3815 bytes
09:37:16.744 chunk 12 4595 bytes
09:37:16.746 fetch POST /api/observability/v1/traces
09:37:16.746 fetched POST /api/observability/v1/traces 204
09:37:16.752 in Ack 1
09:37:16.753 in Ack 12
09:37:17.007 chunk 2 3815 bytes
09:37:17.008 in Ack 2
09:37:17.186 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:47.611Z","configurableProviders":false,"con
09:37:17.186 chunk 10 1632 bytes
09:37:17.186 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:17.192 in Ack 10
09:37:17.613 chunk 12 4635 bytes
09:37:17.614 in Ack 12
09:37:17.755 fetch POST /api/observability/v1/traces
09:37:17.755 fetched POST /api/observability/v1/traces 204
09:37:17.943 chunk 3 876 bytes
09:37:17.944 exit 4 {"_tag":"Success","value":null}
09:37:17.944 chunk 5 4 bytes
09:37:17.944 exit 6 {"_tag":"Success","value":null}
09:37:17.944 chunk 9 36 bytes
09:37:17.944 in Ack 3
09:37:17.951 rpc 16 preview.reportProfiles
09:37:17.952 in Ack 5
09:37:17.953 in Ack 9
09:37:18.025 exit 16 {"_tag":"Success","value":null}
09:37:18.206 rpc 17 server.reportClientActivity
09:37:18.263 exit 17 {"_tag":"Success","value":null}
09:37:18.951 fetch POST /api/observability/v1/traces
09:37:18.951 fetched POST /api/observability/v1/traces 204
09:37:19.197 fetched GET /api/orchestration/threads/mcp%3Aad0b54c9-f3ee-4cd0-8935-9013b1ce950a/bounded 200
09:37:19.204 rpc 18 orchestration.subscribeThread
09:37:19.204 chunk 18 8283 bytes
09:37:19.227 in Interrupt 10
09:37:19.230 rpc 19 subscribeVcsStatus
09:37:19.232 in Ack 18
09:37:19.356 chunk 19 1632 bytes
09:37:19.357 in Ack 19
09:37:19.455 chunk 18 8323 bytes
09:37:19.456 in Ack 18
09:37:19.524 rpc 20 server.reportClientActivity
09:37:19.658 exit 20 {"_tag":"Success","value":null}
09:37:19.757 chunk 18 12691 bytes
09:37:19.765 in Ack 18
09:37:20.025 chunk 18 13498 bytes
09:37:20.028 in Ack 18
09:37:20.198 fetch POST /api/observability/v1/traces
09:37:20.199 fetched POST /api/observability/v1/traces 204
09:37:20.312 chunk 18 17167 bytes
09:37:20.315 in Ack 18
09:37:20.439 rpc 21 attachments.createUploadUrl
09:37:20.529 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-d7f64334-57ea-4a15-a0ba-82028e6088bf","expiresAt":1794217640511,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:20.531 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWQ3ZjY0MzM0LTU3ZWEtNGExNS1hMGJhLTgyMDI4ZTYwODhiZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0MDUxMX0.pawOG8piT4zsu-tRluO2igENq-AaTLwq77pSSKUAnR4
09:37:20.562 chunk 18 19020 bytes
09:37:20.564 in Ack 18
09:37:20.595 in Ping 
09:37:20.812 chunk 18 21100 bytes
09:37:20.815 in Ack 18
09:37:20.830 in Interrupt 19
09:37:20.833 rpc 22 subscribeVcsStatus
09:37:20.918 chunk 22 1632 bytes
09:37:20.918 in Ack 22
09:37:21.086 rpc 23 server.reportClientActivity
09:37:21.316 fetch POST /api/observability/v1/traces
09:37:21.316 fetched POST /api/observability/v1/traces 204
09:37:22.538 exit 23 {"_tag":"Success","value":null}
09:37:23.539 fetch POST /api/observability/v1/traces
09:37:23.539 fetched POST /api/observability/v1/traces 204
09:37:24.176 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:24.627 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:24.631 rpc 25 attachments.delete
09:37:25.069 exit 25 {"_tag":"Success","value":null}
09:37:25.151 chunk 12 4721 bytes
09:37:25.151 chunk 18 26024 bytes
09:37:25.152 in Ack 12
09:37:25.251 in Ack 18
09:37:25.255 rpc 26 assets.createUrl
09:37:25.332 fetch POST /api/observability/v1/traces
09:37:25.332 fetched POST /api/observability/v1/traces 204
09:37:25.442 chunk 18 28481 bytes
09:37:25.445 in Ack 18
09:37:25.597 in Ping 
09:37:25.694 chunk 12 4660 bytes
09:37:25.695 chunk 18 28637 bytes
09:37:25.695 in Ack 12
09:37:25.711 in Ack 18
09:37:25.945 chunk 12 4657 bytes
09:37:25.946 chunk 18 31870 bytes
09:37:25.946 in Ack 12
09:37:25.953 rpc 27 orchestration.dispatchCommand thread.visit
09:37:25.953 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:25.965 in Ack 18
09:37:26.196 chunk 12 4657 bytes
09:37:26.196 chunk 18 32648 bytes
09:37:26.197 in Ack 12
09:37:26.214 in Ack 18
09:37:26.230 in Interrupt 22
09:37:26.233 rpc 28 subscribeVcsStatus
09:37:26.285 chunk 28 1713 bytes
09:37:26.288 in Ack 28
09:37:26.446 fetch POST /api/observability/v1/traces
09:37:26.446 fetched POST /api/observability/v1/traces 204
09:37:26.447 chunk 18 33332 bytes
09:37:26.449 in Ack 18
09:37:26.483 rpc 29 server.reportClientActivity
09:37:26.572 exit 29 {"_tag":"Success","value":null}
09:37:26.718 chunk 18 35260 bytes
09:37:27.065 in Ack 18
09:37:27.252 rpc 30 server.refreshProviders
09:37:27.252 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:47.611Z","configurableProviders":false,"con
09:37:27.480 fetch POST /api/observability/v1/traces
09:37:27.480 fetched POST /api/observability/v1/traces 204
09:37:29.382 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:30.383 fetch POST /api/observability/v1/traces
09:37:30.383 fetched POST /api/observability/v1/traces 204
09:37:30.598 in Ping 
09:37:30.981 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/941f47b2-b0a5-46a2-8f22-7ae000547745","expiresAt":1794220650981}}
09:37:31.988 fetch POST /api/observability/v1/traces
09:37:31.988 fetched POST /api/observability/v1/traces 204
09:37:33.640 chunk 12 4721 bytes
09:37:33.641 in Ack 12
09:37:34.642 fetch POST /api/observability/v1/traces
09:37:34.642 fetched POST /api/observability/v1/traces 204
09:37:34.874 exit 31 {"_tag":"Success","value":{"sequence":153}}
09:37:35.598 in Ping 
09:37:35.875 fetch POST /api/observability/v1/traces
09:37:35.875 fetched POST /api/observability/v1/traces 204
09:37:38.097 fetch POST /api/observability/v1/traces
09:37:38.097 fetched POST /api/observability/v1/traces 204
09:37:38.833 chunk 12 4785 bytes
09:37:38.834 in Ack 12
09:37:38.836 rpc 32 orchestration.dispatchCommand thread.visit
09:37:38.836 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:37:39.835 fetch POST /api/observability/v1/traces
09:37:39.835 fetched POST /api/observability/v1/traces 204
09:37:40.083 chunk 18 35896 bytes
09:37:40.086 in Ack 18
09:37:40.600 in Ping 
09:37:40.824 rpc 33 server.reportClientActivity
09:37:40.834 chunk 18 36736 bytes
09:37:40.836 in Ack 18
09:37:41.086 chunk 18 38757 bytes
09:37:41.090 in Ack 18
09:37:41.107 fetch POST /api/observability/v1/traces
09:37:41.107 fetched POST /api/observability/v1/traces 204
09:37:42.337 chunk 12 4785 bytes
09:37:42.338 in Ack 12
09:37:42.587 chunk 18 38757 bytes
09:37:42.589 in Ack 18
09:37:43.339 fetch POST /api/observability/v1/traces
09:37:43.339 fetched POST /api/observability/v1/traces 204
09:37:43.840 chunk 18 41258 bytes
09:37:43.842 in Ack 18
09:37:44.090 chunk 18 41400 bytes
09:37:44.094 in Ack 18
09:37:44.341 chunk 18 41590 bytes
09:37:44.345 in Ack 18
09:37:44.593 chunk 18 41724 bytes
09:37:44.598 in Ack 18
09:37:44.844 chunk 18 42028 bytes
09:37:44.845 fetch POST /api/observability/v1/traces
09:37:44.845 fetched POST /api/observability/v1/traces 204
09:37:44.847 in Ack 18
09:37:45.344 chunk 18 42108 bytes
09:37:45.347 in Ack 18
09:37:45.602 in Ping 
09:37:45.854 fetch POST /api/observability/v1/traces
09:37:45.854 fetched POST /api/observability/v1/traces 204
09:37:48.101 fetch POST /api/observability/v1/traces
09:37:48.101 fetched POST /api/observability/v1/traces 204
09:37:48.299 chunk 18 42170 bytes
09:37:48.302 in Ack 18
09:37:48.550 chunk 18 42825 bytes
09:37:48.553 in Ack 18
09:37:48.844 rpc 34 orchestration.dispatchCommand thread.visit
09:37:48.844 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:37:49.303 fetch POST /api/observability/v1/traces
09:37:49.303 fetched POST /api/observability/v1/traces 204
09:37:50.603 in Ping 
09:37:51.202 chunk 18 43611 bytes
09:37:51.203 in Ack 18
09:37:51.452 chunk 18 44136 bytes
09:37:51.454 in Ack 18
09:37:52.210 fetch POST /api/observability/v1/traces
09:37:52.210 fetched POST /api/observability/v1/traces 204
09:37:55.603 in Ping 
09:37:55.845 in Interrupt 33
09:37:56.847 fetch POST /api/observability/v1/traces
09:37:56.847 fetched POST /api/observability/v1/traces 204
09:38:00.605 in Ping 
09:38:05.606 in Ping 
09:38:05.826 rpc 35 server.reportClientActivity
09:38:06.522 chunk 18 44444 bytes
09:38:06.528 in Ack 18
09:38:06.827 fetch POST /api/observability/v1/traces
09:38:06.828 fetched POST /api/observability/v1/traces 204
09:38:07.875 chunk 18 44486 bytes
09:38:07.880 in Ack 18
09:38:08.688 chunk 18 44688 bytes
09:38:08.691 in Ack 18
09:38:08.881 fetch POST /api/observability/v1/traces
09:38:08.881 fetched POST /api/observability/v1/traces 204
09:38:10.607 in Ping 
09:38:13.336 chunk 18 44870 bytes
09:38:13.339 in Ack 18
09:38:14.330 chunk 18 45106 bytes
09:38:14.333 in Ack 18
09:38:14.380 fetch POST /api/observability/v1/traces
09:38:14.380 fetched POST /api/observability/v1/traces 204
09:38:15.613 in Ping 
09:38:16.772 chunk 18 45274 bytes
09:38:16.774 in Ack 18
09:38:17.586 chunk 18 45298 bytes
09:38:17.589 in Ack 18
09:38:17.776 fetch POST /api/observability/v1/traces
09:38:17.776 fetched POST /api/observability/v1/traces 204
09:38:18.670 chunk 18 45310 bytes
09:38:18.679 in Ack 18
09:38:18.922 chunk 18 45608 bytes
09:38:18.924 in Ack 18
09:38:19.175 chunk 18 45644 bytes
09:38:19.179 in Ack 18
09:38:19.663 chunk 18 45716 bytes
09:38:19.666 in Ack 18
09:38:19.713 fetch POST /api/observability/v1/traces
09:38:19.713 fetched POST /api/observability/v1/traces 204
09:38:20.615 in Ping 
09:38:20.827 in Interrupt 35
09:38:21.828 fetch POST /api/observability/v1/traces
09:38:21.828 fetched POST /api/observability/v1/traces 204
09:38:21.968 chunk 18 45824 bytes
09:38:21.970 in Ack 18
09:38:22.971 fetch POST /api/observability/v1/traces
09:38:22.971 fetched POST /api/observability/v1/traces 204
09:38:23.580 chunk 18 46826 bytes
09:38:23.585 in Ack 18
09:38:23.948 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:24.586 fetch POST /api/observability/v1/traces
09:38:24.586 fetched POST /api/observability/v1/traces 204
09:38:25.559 exit 36 {"_tag":"Success","value":{"sequence":186}}
09:38:25.619 in Ping 
09:38:26.463 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:26.561 fetch POST /api/observability/v1/traces
09:38:26.562 fetched POST /api/observability/v1/traces 204
09:38:30.623 in Ping 
09:38:30.825 rpc 38 server.reportClientActivity
09:38:30.845 chunk 18 46869 bytes
09:38:30.849 in Ack 18
09:38:31.826 fetch POST /api/observability/v1/traces
09:38:31.826 fetched POST /api/observability/v1/traces 204
09:38:32.162 exit 33 {"_tag":"Success","value":null}
09:38:32.183 chunk 18 46873 bytes
09:38:32.185 in Ack 18
09:38:33.189 fetch POST /api/observability/v1/traces
09:38:33.189 fetched POST /api/observability/v1/traces 204
09:38:34.149 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:35.153 fetch POST /api/observability/v1/traces
09:38:35.154 fetched POST /api/observability/v1/traces 204
09:38:35.627 in Ping 
09:38:36.300 chunk 18 49862 bytes
09:38:36.303 in Ack 18
09:38:36.549 chunk 18 49866 bytes
09:38:36.552 in Ack 18
09:38:36.800 chunk 18 50108 bytes
09:38:36.803 in Ack 18
09:38:37.305 fetch POST /api/observability/v1/traces
09:38:37.305 fetched POST /api/observability/v1/traces 204
09:38:40.172 chunk 18 50060 bytes
09:38:40.175 in Ack 18
09:38:40.631 in Ping 
09:38:41.179 fetch POST /api/observability/v1/traces
09:38:41.179 fetched POST /api/observability/v1/traces 204
09:38:42.938 chunk 18 50616 bytes
09:38:42.944 in Ack 18
09:38:43.202 chunk 18 50738 bytes
09:38:43.207 in Ack 18
09:38:44.150 fetch POST /api/observability/v1/traces
09:38:44.150 fetched POST /api/observability/v1/traces 204
