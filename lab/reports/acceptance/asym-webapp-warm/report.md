# asym, webapp, warm start, seed 1

Profile asym (fault): uplink 2 kbit/s with 50% loss, downlink 200 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 99 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.3s |
| Prompt reached the agent | 4.1s |
| Time to first text | 4.8s |
| Whole reply shown | 98.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 54.8s / 8.6s / 49.8s |
| Longest silence | 67.3s |
| Approval shown after it was asked | 0.9s |
| Bytes on the wire down / up | 555278 / 24675 |
| Bytes the app received / sent | 420897 / 47973 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.6 | 8 | 234 |
| 20 | 3.3 | 502 | 632 |
| 30 | 0.6 | 1013 | 1037 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 6.8 | 1077 | 1347 |
| 60 | 16.8 | 1077 | 1596 |
| 70 | 26.8 | 1077 | 1596 |
| 80 | 36.8 | 1077 | 1596 |
| 90 | 46.8 | 1077 | 1596 |
| 98 | 54.8 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"774/774 774/774","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","messages":"456/456 546/546 901/901 546/546","providerThreads":"808/808","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167840629,"silences":0,"longestSilenceMs":1607,"longestRecoveredGapMs":1607},"stats":{"sendRate":6750,"deliveredRate":3453.351479801452,"loss":0,"srttMs":5.857149750729465,"minRttMs":1,"sentPackets":101,"sentWireBytes":30828,"receivedPackets":143,"receivedWireBytes":77589,"lastHeardMs":9733,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 417/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"774/774 774/774","assistant":"32/32 21/21 280/290","user":"10/10 9/9","attempts":"660/660 660/660","messages":"456/456 546/546 901/901 546/546","providerThreads":"808/808","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167850705,"silences":0,"longestSilenceMs":1607,"longestRecoveredGapMs":1607},"stats":{"sendRate":3375,"deliveredRate":3453.351479801452,"loss":0,"srttMs":505.3750060318883,"minRttMs":1,"sentPackets":124,"sentWireBytes":37249,"receivedPackets":219,"receivedWireBytes":108963,"lastHeardMs":19809,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"774/774 774/774","assistant":"32/32 21/21 783/788","user":"10/10 9/9","attempts":"660/660 660/660 204/636","messages":"456/456 546/546 901/901 546/546","providerThreads":"808/808","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167860749,"silences":0,"longestSilenceMs":1607,"longestRecoveredGapMs":1607},"stats":{"sendRate":2000,"deliveredRate":134.8647647276769,"loss":0.75,"srttMs":505.3750060318883,"minRttMs":1,"sentPackets":174,"sentWireBytes":47867,"receivedPackets":278,"receivedWireBytes":132682,"lastHeardMs":29853,"unsyncedBytes":495,"inflightPackets":42,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 218/656","assistant":"32/32 21/21 1293/1293","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 205/451","providerThreads":"702/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 33/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167870957,"silences":0,"longestSilenceMs":1607,"longestRecoveredGapMs":1607},"stats":{"sendRate":2000,"deliveredRate":126.18311308715161,"loss":0.8099999999999998,"srttMs":2059.958255994019,"minRttMs":4531,"sentPackets":192,"sentWireBytes":53373,"receivedPackets":375,"receivedWireBytes":172407,"lastHeardMs":40061,"unsyncedBytes":454,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1546/1546 519/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 218/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 205/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167875740,"silences":0,"longestSilenceMs":1607,"longestRecoveredGapMs":1607},"stats":{"sendRate":2000,"deliveredRate":126.18311308715161,"loss":0.8099999999999998,"srttMs":2059.958255994019,"minRttMs":4531,"sentPackets":197,"sentWireBytes":56929,"receivedPackets":416,"receivedWireBytes":194519,"lastHeardMs":44844,"unsyncedBytes":853,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 202/1570 519/926","frame":"1030/1030","thread":"561/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 218/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 205/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"774/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167892153,"silences":1,"longestSilenceMs":14336,"longestRecoveredGapMs":14336},"stats":{"sendRate":2000,"deliveredRate":109.42293867285106,"loss":0.8228823529411763,"srttMs":2837.088473994767,"minRttMs":4531,"sentPackets":205,"sentWireBytes":62632,"receivedPackets":445,"receivedWireBytes":207602,"lastHeardMs":61257,"unsyncedBytes":956,"inflightPackets":67,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 254/1570 519/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 299/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 205/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167903018,"silences":1,"longestSilenceMs":14336,"longestRecoveredGapMs":14336},"stats":{"sendRate":1000,"deliveredRate":139.35265531146496,"loss":0.7860176470588234,"srttMs":4732.645862902244,"minRttMs":4531,"sentPackets":215,"sentWireBytes":68347,"receivedPackets":590,"receivedWireBytes":272122,"lastHeardMs":72122,"unsyncedBytes":764,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 254/1570 519/926 79/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 299/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 205/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167913851,"silences":1,"longestSilenceMs":14336,"longestRecoveredGapMs":14336},"stats":{"sendRate":1000,"deliveredRate":139.35265531146496,"loss":0.7860176470588234,"srttMs":4732.645862902244,"minRttMs":4531,"sentPackets":215,"sentWireBytes":68347,"receivedPackets":743,"receivedWireBytes":337852,"lastHeardMs":82955,"unsyncedBytes":1259,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 254/1570 519/926 122/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 299/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 229/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167924728,"silences":1,"longestSilenceMs":14336,"longestRecoveredGapMs":14336},"stats":{"sendRate":1000,"deliveredRate":139.35265531146496,"loss":0.7860176470588234,"srttMs":4732.645862902244,"minRttMs":4531,"sentPackets":215,"sentWireBytes":68347,"receivedPackets":886,"receivedWireBytes":402370,"lastHeardMs":93832,"unsyncedBytes":1259,"inflightPackets":64,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 254/1570 519/926 122/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"774/774 774/774 299/656","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 230/451","providerThreads":"744/810","providerSessions":"2691/2691","nodes":"1019/1019 809/809 809/809 1019/1019 47/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":274,"lastDataAt":1794167935570,"silences":1,"longestSilenceMs":14336,"longestRecoveredGapMs":14336},"stats":{"sendRate":1000,"deliveredRate":139.35265531146496,"loss":0.7860176470588234,"srttMs":4732.645862902244,"minRttMs":4531,"sentPackets":225,"sentWireBytes":74148,"receivedPackets":1031,"receivedWireBytes":467769,"lastHeardMs":104674,"unsyncedBytes":1754,"inflightPackets":29,"duplicateBytes":0}}
- shim trace
19:57:11.195 fetch GET /api/auth/session
19:57:11.196 fetched GET /api/auth/session 200
19:57:11.335 fetch GET /.well-known/t3/environment
19:57:11.335 fetched GET /.well-known/t3/environment 200
19:57:11.345 fetch GET /.well-known/t3/environment
19:57:11.346 fetched GET /.well-known/t3/environment 200
19:57:11.362 rpc 0 subscribeServerConfig
19:57:11.362 chunk 0 38702 bytes
19:57:11.383 in Ack 0
19:57:11.384 rpc 1 subscribeServerLifecycle
19:57:11.385 rpc 2 subscribeServerLifecycle
19:57:11.388 fetch GET /api/auth/session
19:57:11.389 fetched GET /api/auth/session 200
19:57:11.390 rpc 3 subscribeDeviceState
19:57:11.391 rpc 4 preview.reportProfiles
19:57:11.425 rpc 5 subscribeProjectClones
19:57:11.445 fetch GET /api/orchestration/threads/mcp%3A24b4fdf5-031c-43da-8c02-4165cf42e9ae/bounded
19:57:11.449 fetch GET /api/orchestration/shell
19:57:11.450 fetched GET /api/orchestration/shell 200
19:57:11.452 rpc 6 orchestration.subscribeShell
19:57:11.452 chunk 6 2785 bytes
19:57:11.475 rpc 7 server.refreshProviders
19:57:11.512 rpc 8 orchestration.dispatchCommand thread.visit
19:57:11.512 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:57:11.526 rpc 9 subscribeTerminalMetadata
19:57:11.527 rpc 10 subscribeVcsStatus
19:57:11.527 rpc 11 subscribePreviewEvents
19:57:11.529 rpc 12 assets.createUrl
19:57:11.530 rpc 13 preview.list
19:57:11.530 rpc 14 projects.readFile
19:57:11.537 in Ack 6
19:57:11.681 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"59d5b7f3-0410-4cdc-a11c-b03f1d449b91","sessions":[]}}
19:57:11.681 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:57:11.779 rpc 15 server.reportClientActivity
19:57:11.780 chunk 1 3867 bytes
19:57:11.780 in Ack 1
19:57:12.251 chunk 2 3867 bytes
19:57:12.251 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:56:59.660Z","configurableProviders":false,"con
19:57:12.251 chunk 10 2165 bytes
19:57:12.251 in Ack 2
19:57:12.255 in Ack 10
19:57:12.333 fetch POST /api/observability/v1/traces
19:57:12.334 fetched POST /api/observability/v1/traces 204
19:57:12.374 chunk 6 2761 bytes
19:57:12.374 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:57:12.374 in Ack 6
19:57:12.521 chunk 3 876 bytes
19:57:12.522 in Ack 3
19:57:12.598 exit 4 {"_tag":"Success","value":null}
19:57:12.598 chunk 5 4 bytes
19:57:12.598 chunk 9 36 bytes
19:57:12.598 exit 15 {"_tag":"Success","value":null}
19:57:12.598 in Ack 5
19:57:12.599 in Ack 9
19:57:12.624 chunk 6 2804 bytes
19:57:12.624 in Ack 6
19:57:12.722 fetched GET /api/orchestration/threads/mcp%3A24b4fdf5-031c-43da-8c02-4165cf42e9ae/bounded 200
19:57:12.725 rpc 16 orchestration.subscribeThread
19:57:12.726 chunk 16 8283 bytes
19:57:12.743 in Interrupt 10
19:57:12.747 rpc 17 subscribeVcsStatus
19:57:12.748 in Ack 16
19:57:12.918 chunk 17 2165 bytes
19:57:12.919 in Ack 17
19:57:12.977 chunk 16 11920 bytes
19:57:12.979 in Ack 16
19:57:13.039 rpc 18 server.reportClientActivity
19:57:13.237 chunk 16 13502 bytes
19:57:13.240 in Ack 16
19:57:13.375 fetch POST /api/observability/v1/traces
19:57:13.376 fetched POST /api/observability/v1/traces 204
19:57:13.506 chunk 16 19067 bytes
19:57:13.508 in Ack 16
19:57:13.698 rpc 19 attachments.createUploadUrl
19:57:13.884 exit 18 {"_tag":"Success","value":null}
19:57:13.885 chunk 16 21189 bytes
19:57:13.888 in Ack 16
19:57:13.897 in Interrupt 17
19:57:13.899 rpc 20 subscribeVcsStatus
19:57:13.938 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-fb7cd53a-8526-46e2-8437-762fafd5f078","expiresAt":1794168433928,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:57:13.938 chunk 20 2165 bytes
19:57:13.939 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWZiN2NkNTNhLTg1MjYtNDZlMi04NDM3LTc2MmZhZmQ1ZjA3OCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODQzMzkyOH0.pK3kPBIJnhl0vQ_FBMk7-cpt515EjjrKbXpa09H2qGk
19:57:13.940 in Ack 20
19:57:14.149 rpc 21 server.reportClientActivity
19:57:14.509 fetch POST /api/observability/v1/traces
19:57:14.509 fetched POST /api/observability/v1/traces 204
19:57:15.817 exit 21 {"_tag":"Success","value":null}
19:57:16.348 in Ping 
19:57:16.818 fetch POST /api/observability/v1/traces
19:57:16.818 fetched POST /api/observability/v1/traces 204
19:57:17.019 rpc 22 orchestration.dispatchCommand message.dispatch
19:57:17.650 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:57:17.652 rpc 23 attachments.delete
19:57:17.687 exit 23 {"_tag":"Success","value":null}
19:57:17.900 chunk 6 2890 bytes
19:57:17.900 chunk 16 26416 bytes
19:57:17.900 in Ack 6
19:57:17.930 in Ack 16
19:57:17.933 rpc 24 assets.createUrl
19:57:18.018 fetch POST /api/observability/v1/traces
19:57:18.018 fetched POST /api/observability/v1/traces 204
19:57:18.152 chunk 16 27855 bytes
19:57:18.153 in Ack 16
19:57:18.401 chunk 16 28534 bytes
19:57:18.403 in Ack 16
19:57:18.652 chunk 6 2826 bytes
19:57:18.652 chunk 16 28716 bytes
19:57:18.652 in Ack 6
19:57:18.653 rpc 25 orchestration.dispatchCommand thread.visit
19:57:18.653 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:57:18.658 in Ack 16
19:57:18.902 chunk 6 2826 bytes
19:57:18.902 chunk 16 31163 bytes
19:57:18.903 in Ack 6
19:57:18.908 in Ack 16
19:57:19.152 chunk 16 32587 bytes
19:57:19.153 in Ack 16
19:57:19.160 in Interrupt 20
19:57:19.161 rpc 26 subscribeVcsStatus
19:57:19.162 fetch POST /api/observability/v1/traces
19:57:19.162 fetched POST /api/observability/v1/traces 204
19:57:19.218 chunk 26 2165 bytes
19:57:19.219 in Ack 26
19:57:19.403 chunk 16 35204 bytes
19:57:19.404 in Ack 16
19:57:19.412 rpc 27 server.reportClientActivity
19:57:19.448 exit 27 {"_tag":"Success","value":null}
19:57:20.281 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/38fc453a-4bf8-48ab-9cd6-9006b7c61d90","expiresAt":1794171440281}}
19:57:20.408 fetch POST /api/observability/v1/traces
19:57:20.408 fetched POST /api/observability/v1/traces 204
19:57:20.611 rpc 28 orchestration.dispatchCommand message.dispatch
19:57:21.349 in Ping 
19:57:21.611 fetch POST /api/observability/v1/traces
19:57:21.611 fetched POST /api/observability/v1/traces 204
19:57:22.959 fetch POST /api/observability/v1/traces
19:57:22.959 fetched POST /api/observability/v1/traces 204
19:57:24.255 fetch POST /api/observability/v1/traces
19:57:24.255 fetched POST /api/observability/v1/traces 204
19:57:24.667 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:57:25.168 chunk 16 37648 bytes
19:57:25.169 in Ack 16
19:57:25.465 chunk 16 37660 bytes
19:57:25.467 in Ack 16
19:57:25.668 fetch POST /api/observability/v1/traces
19:57:25.668 fetched POST /api/observability/v1/traces 204
19:57:25.717 chunk 16 37680 bytes
19:57:25.718 in Ack 16
19:57:25.967 chunk 16 37710 bytes
19:57:25.968 in Ack 16
19:57:26.218 chunk 16 37730 bytes
19:57:26.219 in Ack 16
19:57:26.350 in Ping 
19:57:26.468 chunk 16 37760 bytes
19:57:26.470 in Ack 16
19:57:26.719 chunk 16 37780 bytes
19:57:26.719 fetch POST /api/observability/v1/traces
19:57:26.719 fetched POST /api/observability/v1/traces 204
19:57:26.720 in Ack 16
19:57:26.969 chunk 16 37810 bytes
19:57:26.970 in Ack 16
19:57:27.219 chunk 16 37830 bytes
19:57:27.220 in Ack 16
19:57:27.469 chunk 16 37860 bytes
19:57:27.470 in Ack 16
19:57:27.718 chunk 16 37880 bytes
19:57:27.719 in Ack 16
19:57:27.729 fetch POST /api/observability/v1/traces
19:57:27.729 fetched POST /api/observability/v1/traces 204
19:57:27.969 chunk 16 37910 bytes
19:57:27.970 in Ack 16
19:57:28.219 chunk 16 37930 bytes
19:57:28.220 in Ack 16
19:57:28.469 chunk 16 37960 bytes
19:57:28.470 in Ack 16
19:57:28.720 chunk 16 37980 bytes
19:57:28.721 in Ack 16
19:57:28.970 chunk 16 38010 bytes
19:57:28.971 fetch POST /api/observability/v1/traces
19:57:28.971 fetched POST /api/observability/v1/traces 204
19:57:28.972 in Ack 16
19:57:29.220 chunk 16 38030 bytes
19:57:29.221 in Ack 16
19:57:29.470 chunk 16 38060 bytes
19:57:29.471 in Ack 16
19:57:29.721 chunk 16 38090 bytes
19:57:29.722 in Ack 16
19:57:29.970 chunk 16 38110 bytes
19:57:29.971 in Ack 16
19:57:29.980 fetch POST /api/observability/v1/traces
19:57:29.980 fetched POST /api/observability/v1/traces 204
19:57:30.226 chunk 16 38140 bytes
19:57:30.228 in Ack 16
19:57:30.479 chunk 16 38160 bytes
19:57:30.480 in Ack 16
19:57:30.729 chunk 16 38190 bytes
19:57:30.730 in Ack 16
19:57:30.979 chunk 16 38210 bytes
19:57:30.981 in Ack 16
19:57:31.228 fetch POST /api/observability/v1/traces
19:57:31.228 fetched POST /api/observability/v1/traces 204
19:57:31.229 chunk 16 38240 bytes
19:57:31.230 in Ack 16
19:57:31.350 in Ping 
19:57:31.504 chunk 16 38270 bytes
19:57:31.505 in Ack 16
19:57:31.764 chunk 16 38290 bytes
19:57:31.765 in Ack 16
19:57:32.014 chunk 16 38320 bytes
19:57:32.015 in Ack 16
19:57:32.231 fetch POST /api/observability/v1/traces
19:57:32.231 fetched POST /api/observability/v1/traces 204
19:57:32.265 chunk 16 38340 bytes
19:57:32.266 in Ack 16
19:57:32.515 chunk 16 38370 bytes
19:57:32.516 in Ack 16
19:57:32.765 chunk 16 38390 bytes
19:57:32.767 in Ack 16
19:57:33.041 chunk 16 38420 bytes
19:57:33.042 in Ack 16
19:57:33.267 fetch POST /api/observability/v1/traces
19:57:33.267 fetched POST /api/observability/v1/traces 204
19:57:33.292 chunk 16 38440 bytes
19:57:33.293 in Ack 16
19:57:33.558 chunk 16 38470 bytes
19:57:33.559 in Ack 16
19:57:34.064 chunk 16 38502 bytes
19:57:34.065 in Ack 16
19:57:34.294 fetch POST /api/observability/v1/traces
19:57:34.294 fetched POST /api/observability/v1/traces 204
19:57:34.314 chunk 16 38522 bytes
19:57:34.316 in Ack 16
19:57:34.565 chunk 16 38552 bytes
19:57:34.566 in Ack 16
19:57:34.814 chunk 16 38582 bytes
19:57:34.815 in Ack 16
19:57:35.076 chunk 16 38602 bytes
19:57:35.077 in Ack 16
19:57:35.317 fetch POST /api/observability/v1/traces
19:57:35.317 fetched POST /api/observability/v1/traces 204
19:57:35.330 chunk 16 38632 bytes
19:57:35.331 in Ack 16
19:57:35.581 chunk 16 38652 bytes
19:57:35.582 in Ack 16
19:57:35.837 chunk 16 38708 bytes
19:57:35.838 in Ack 16
19:57:36.086 chunk 16 38738 bytes
19:57:36.088 in Ack 16
19:57:36.332 fetch POST /api/observability/v1/traces
19:57:36.332 fetched POST /api/observability/v1/traces 204
19:57:36.337 chunk 16 38710 bytes
19:57:36.338 in Ack 16
19:57:36.394 in Ping 
19:57:36.587 rpc 29 server.reportClientActivity
19:57:36.588 chunk 16 38740 bytes
19:57:36.589 in Ack 16
19:57:36.838 chunk 16 38760 bytes
19:57:36.840 in Ack 16
19:57:37.090 chunk 16 38780 bytes
19:57:37.092 in Ack 16
19:57:37.339 fetch POST /api/observability/v1/traces
19:57:37.339 fetched POST /api/observability/v1/traces 204
19:57:37.340 chunk 16 38820 bytes
19:57:37.341 in Ack 16
19:57:37.590 chunk 16 38830 bytes
19:57:37.591 in Ack 16
19:57:37.840 chunk 16 38852 bytes
19:57:37.841 in Ack 16
19:57:38.091 chunk 16 38882 bytes
19:57:38.092 in Ack 16
19:57:38.341 chunk 16 38912 bytes
19:57:38.342 in Ack 16
19:57:38.350 fetch POST /api/observability/v1/traces
19:57:38.350 fetched POST /api/observability/v1/traces 204
19:57:38.591 chunk 16 38932 bytes
19:57:38.592 in Ack 16
19:57:38.841 chunk 16 38944 bytes
19:57:38.842 in Ack 16
19:57:39.091 chunk 16 38964 bytes
19:57:39.092 in Ack 16
19:57:39.342 chunk 16 39004 bytes
19:57:39.343 in Ack 16
19:57:39.592 chunk 16 39014 bytes
19:57:39.593 fetch POST /api/observability/v1/traces
19:57:39.593 fetched POST /api/observability/v1/traces 204
19:57:39.594 in Ack 16
19:57:39.841 chunk 16 39054 bytes
19:57:39.842 in Ack 16
19:57:40.093 chunk 16 39074 bytes
19:57:40.094 in Ack 16
19:57:40.342 chunk 16 39094 bytes
19:57:40.343 in Ack 16
19:57:40.593 chunk 16 39124 bytes
19:57:40.594 in Ack 16
19:57:40.625 fetch POST /api/observability/v1/traces
19:57:40.625 fetched POST /api/observability/v1/traces 204
19:57:40.842 chunk 16 39152 bytes
19:57:40.844 in Ack 16
19:57:41.093 chunk 16 39182 bytes
19:57:41.094 in Ack 16
19:57:41.343 chunk 16 39202 bytes
19:57:41.344 in Ack 16
19:57:41.394 in Ping 
19:57:41.593 chunk 16 39232 bytes
19:57:41.594 in Ack 16
19:57:41.843 chunk 16 39250 bytes
19:57:41.844 in Ack 16
19:57:41.852 fetch POST /api/observability/v1/traces
19:57:41.852 fetched POST /api/observability/v1/traces 204
19:57:42.094 chunk 16 39290 bytes
19:57:42.095 in Ack 16
19:57:42.344 chunk 16 39320 bytes
19:57:42.347 in Ack 16
19:57:42.594 chunk 16 39340 bytes
19:57:42.595 in Ack 16
19:57:42.844 chunk 16 39360 bytes
19:57:42.845 in Ack 16
19:57:43.094 chunk 6 2890 bytes
19:57:43.094 chunk 16 39390 bytes
19:57:43.095 in Ack 6
19:57:43.096 rpc 30 orchestration.dispatchCommand thread.visit
19:57:43.096 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:57:43.134 in Ack 16
19:57:43.159 fetch POST /api/observability/v1/traces
19:57:43.159 fetched POST /api/observability/v1/traces 204
19:57:43.344 chunk 16 39410 bytes
19:57:43.346 in Ack 16
19:57:43.594 chunk 16 40067 bytes
19:57:43.595 in Ack 16
19:57:43.845 chunk 16 40097 bytes
19:57:43.846 in Ack 16
19:57:44.096 chunk 16 40967 bytes
19:57:44.097 in Ack 16
19:57:44.161 fetch POST /api/observability/v1/traces
19:57:44.161 fetched POST /api/observability/v1/traces 204
19:57:44.347 chunk 16 42980 bytes
19:57:44.348 in Ack 16
19:57:44.597 chunk 16 43020 bytes
19:57:44.598 in Ack 16
19:57:44.847 chunk 16 43040 bytes
19:57:44.848 in Ack 16
19:57:45.097 chunk 16 44621 bytes
19:57:45.099 in Ack 16
19:57:45.475 chunk 16 44651 bytes
19:57:45.475 fetch POST /api/observability/v1/traces
19:57:45.475 fetched POST /api/observability/v1/traces 204
19:57:45.478 in Ack 16
19:57:45.725 chunk 16 44696 bytes
19:57:45.726 in Ack 16
19:57:45.976 chunk 16 44756 bytes
19:57:45.977 in Ack 16
19:57:46.225 chunk 16 44786 bytes
19:57:46.226 in Ack 16
19:57:46.394 in Ping 
19:57:46.476 chunk 16 44801 bytes
19:57:46.477 in Ack 16
19:57:46.485 fetch POST /api/observability/v1/traces
19:57:46.485 fetched POST /api/observability/v1/traces 204
19:57:46.727 chunk 16 44861 bytes
19:57:46.728 in Ack 16
19:57:46.977 chunk 16 44891 bytes
19:57:46.978 in Ack 16
19:57:47.232 chunk 16 44936 bytes
19:57:47.233 in Ack 16
19:57:47.482 chunk 16 44966 bytes
19:57:47.483 in Ack 16
19:57:47.729 fetch POST /api/observability/v1/traces
19:57:47.729 fetched POST /api/observability/v1/traces 204
19:57:47.732 chunk 16 45011 bytes
19:57:47.733 in Ack 16
19:57:47.983 chunk 16 45041 bytes
19:57:47.984 in Ack 16
19:57:48.233 chunk 16 45086 bytes
19:57:48.234 in Ack 16
19:57:48.483 chunk 16 45116 bytes
19:57:48.484 in Ack 16
19:57:48.733 chunk 16 45176 bytes
19:57:48.734 fetch POST /api/observability/v1/traces
19:57:48.734 fetched POST /api/observability/v1/traces 204
19:57:48.735 in Ack 16
19:57:48.984 chunk 16 45206 bytes
19:57:48.985 in Ack 16
19:57:49.235 chunk 16 45236 bytes
19:57:49.235 in Ack 16
19:57:49.484 chunk 16 45281 bytes
19:57:49.485 in Ack 16
19:57:49.735 chunk 16 45311 bytes
19:57:49.736 in Ack 16
19:57:49.744 fetch POST /api/observability/v1/traces
19:57:49.744 fetched POST /api/observability/v1/traces 204
19:57:49.986 chunk 16 45341 bytes
19:57:49.987 in Ack 16
19:57:50.235 chunk 16 45386 bytes
19:57:50.236 in Ack 16
19:57:50.486 chunk 16 45416 bytes
19:57:50.487 in Ack 16
19:57:50.736 chunk 16 45461 bytes
19:57:50.737 in Ack 16
19:57:50.986 chunk 16 45506 bytes
19:57:50.987 in Ack 16
19:57:51.011 fetch POST /api/observability/v1/traces
19:57:51.011 fetched POST /api/observability/v1/traces 204
19:57:51.235 chunk 16 45536 bytes
19:57:51.236 in Ack 16
19:57:51.395 in Ping 
19:57:51.486 chunk 16 45566 bytes
19:57:51.487 in Ack 16
19:57:51.591 in Interrupt 29
19:57:51.736 chunk 16 45638 bytes
19:57:51.737 in Ack 16
19:57:52.239 chunk 16 46643 bytes
19:57:52.240 fetch POST /api/observability/v1/traces
19:57:52.240 fetched POST /api/observability/v1/traces 204
19:57:52.241 in Ack 16
19:57:52.503 rpc 31 orchestration.dispatchCommand runtime-request.respond
19:57:53.061 chunk 16 50139 bytes
19:57:53.062 in Ack 16
19:57:53.242 fetch POST /api/observability/v1/traces
19:57:53.242 fetched POST /api/observability/v1/traces 204
19:57:54.263 fetch POST /api/observability/v1/traces
19:57:54.263 fetched POST /api/observability/v1/traces 204
19:57:56.395 in Ping 
19:58:01.396 in Ping 
19:58:01.591 rpc 32 server.reportClientActivity
19:58:02.591 fetch POST /api/observability/v1/traces
19:58:02.591 fetched POST /api/observability/v1/traces 204
19:58:06.399 in Ping 
19:58:10.077 exit 31 {"_tag":"Success","value":{"sequence":96}}
19:58:10.739 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:58:11.079 fetch POST /api/observability/v1/traces
19:58:11.079 fetched POST /api/observability/v1/traces 204
19:58:11.402 in Ping 
19:58:13.085 chunk 16 50182 bytes
19:58:13.086 in Ack 16
19:58:14.087 fetch POST /api/observability/v1/traces
19:58:14.087 fetched POST /api/observability/v1/traces 204
19:58:14.588 chunk 16 50182 bytes
19:58:14.589 in Ack 16
19:58:15.590 fetch POST /api/observability/v1/traces
19:58:15.590 fetched POST /api/observability/v1/traces 204
19:58:15.670 chunk 6 2892 bytes
19:58:15.671 in Ack 6
19:58:16.406 in Ping 
19:58:16.591 in Interrupt 32
19:58:16.672 fetch POST /api/observability/v1/traces
19:58:16.672 fetched POST /api/observability/v1/traces 204
19:58:21.410 in Ping 
19:58:23.270 chunk 6 2828 bytes
19:58:23.271 in Ack 6
19:58:24.271 fetch POST /api/observability/v1/traces
19:58:24.271 fetched POST /api/observability/v1/traces 204
19:58:26.414 in Ping 
19:58:26.591 rpc 34 server.reportClientActivity
19:58:26.679 fetch POST /api/observability/v1/traces
19:58:26.679 fetched POST /api/observability/v1/traces 204
19:58:31.418 in Ping 
19:58:36.422 in Ping 
19:58:41.426 in Ping 
19:58:41.592 in Interrupt 34
19:58:42.593 fetch POST /api/observability/v1/traces
19:58:42.593 fetched POST /api/observability/v1/traces 204
19:58:46.430 in Ping 
19:58:51.434 in Ping 
19:58:51.591 rpc 35 server.reportClientActivity
19:58:52.592 fetch POST /api/observability/v1/traces
19:58:52.592 fetched POST /api/observability/v1/traces 204
19:58:56.438 in Ping 
19:58:58.038 exit 29 {"_tag":"Success","value":null}
19:58:58.090 chunk 16 52759 bytes
19:58:58.091 in Ack 16
19:58:58.345 chunk 16 53257 bytes
19:58:58.347 in Ack 16
19:58:58.596 chunk 16 54005 bytes
19:58:58.597 in Ack 16
19:58:58.689 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:58:58.705 rpc 36 orchestration.dispatchCommand thread.visit
19:58:58.705 exit 36 {"_tag":"Success","value":{"sequence":0}}
19:58:58.740 rpc 37 orchestration.dispatchCommand thread.visit
19:58:58.740 exit 37 {"_tag":"Success","value":{"sequence":0}}
19:58:58.846 chunk 16 54107 bytes
19:58:58.847 in Ack 16
19:58:59.160 fetch POST /api/observability/v1/traces
19:58:59.160 fetched POST /api/observability/v1/traces 204
19:58:59.408 chunk 16 54109 bytes
19:58:59.410 in Ack 16
