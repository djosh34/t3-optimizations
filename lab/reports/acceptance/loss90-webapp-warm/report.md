# loss90, webapp, warm start, seed 1

Profile loss90 (fault): 90% random loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 135 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 24.9s |
| Prompt reached the agent | 22.5s |
| Time to first text | 24.9s |
| Whole reply shown | 134.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 51.5s / 7.5s / 44.5s |
| Longest silence | 49.6s |
| Approval shown after it was asked | 24.6s |
| Bytes on the wire down / up | 44078 / 26301 |
| Bytes the app received / sent | 29105 / 93099 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.7 | 271 | 303 |
| 40 | 8.0 | 377 | 701 |
| 50 | 18.0 | 377 | 1077 |
| 60 | 28.0 | 377 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 0.0 | 1077 | 1077 |
| 90 | 7.5 | 1106 | 1398 |
| 100 | 17.5 | 1106 | 1596 |
| 110 | 27.5 | 1106 | 1596 |
| 120 | 37.5 | 1106 | 1596 |
| 130 | 47.5 | 1106 | 1596 |
| 134 | 51.5 | 1106 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 149/1058","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166131770,"silences":0,"longestSilenceMs":1154,"longestRecoveredGapMs":1154},"stats":{"sendRate":4500,"deliveredRate":1337.4193548387098,"loss":0,"srttMs":8.777249526699954,"minRttMs":1,"sentPackets":89,"sentWireBytes":28826,"receivedPackets":107,"receivedWireBytes":48399,"lastHeardMs":8883,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 412/1058","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166135099,"silences":0,"longestSilenceMs":1441,"longestRecoveredGapMs":1441},"stats":{"sendRate":1000,"deliveredRate":1337.4193548387098,"loss":0,"srttMs":8.777249526699954,"minRttMs":null,"sentPackets":116,"sentWireBytes":40451,"receivedPackets":110,"receivedWireBytes":49307,"lastHeardMs":12212,"unsyncedBytes":509,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 412/1058","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166135099,"silences":0,"longestSilenceMs":1441,"longestRecoveredGapMs":1441},"stats":{"sendRate":50,"deliveredRate":1337.4193548387098,"loss":0,"srttMs":8.777249526699954,"minRttMs":null,"sentPackets":120,"sentWireBytes":42187,"receivedPackets":110,"receivedWireBytes":49307,"lastHeardMs":12212,"unsyncedBytes":1006,"inflightPackets":34,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 335/335","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166162067,"silences":2,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":2000,"deliveredRate":946.5505234927801,"loss":0.2763157894736842,"srttMs":164.98570666887966,"minRttMs":647,"sentPackets":140,"sentWireBytes":47403,"receivedPackets":113,"receivedWireBytes":50612,"lastHeardMs":39180,"unsyncedBytes":0,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 470/470","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166164801,"silences":3,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":2000,"deliveredRate":946.5505234927801,"loss":0.2763157894736842,"srttMs":164.98570666887966,"minRttMs":647,"sentPackets":146,"sentWireBytes":48180,"receivedPackets":114,"receivedWireBytes":50890,"lastHeardMs":41914,"unsyncedBytes":0,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 689/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166182328,"silences":4,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.2763157894736842,"srttMs":229.3624933352697,"minRttMs":647,"sentPackets":183,"sentWireBytes":58570,"receivedPackets":118,"receivedWireBytes":52043,"lastHeardMs":59441,"unsyncedBytes":297,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1288/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731","runtimeRequests":"49/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166192455,"silences":4,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.2763157894736842,"srttMs":229.3624933352697,"minRttMs":647,"sentPackets":183,"sentWireBytes":58570,"receivedPackets":141,"receivedWireBytes":58941,"lastHeardMs":69568,"unsyncedBytes":297,"inflightPackets":82,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731","runtimeRequests":"962/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166202870,"silences":4,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":6750,"deliveredRate":49.85388661601403,"loss":0.9302325581395349,"srttMs":425.9362015898389,"minRttMs":647,"sentPackets":241,"sentWireBytes":74178,"receivedPackets":159,"receivedWireBytes":64384,"lastHeardMs":79983,"unsyncedBytes":497,"inflightPackets":109,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 460/1058 109/1073 214/1546","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 53/779","checkpointScopes":"731/731","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166212759,"silences":5,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":6750,"deliveredRate":49.85388661601403,"loss":0.9302325581395349,"srttMs":425.9362015898389,"minRttMs":647,"sentPackets":264,"sentWireBytes":83995,"receivedPackets":179,"receivedWireBytes":70853,"lastHeardMs":89872,"unsyncedBytes":896,"inflightPackets":124,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 568/1058 109/1073 214/1546","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340 188/454","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 186/779","checkpointScopes":"731/731","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166223319,"silences":7,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":3375,"deliveredRate":58.21553978014599,"loss":0.9400516795865632,"srttMs":841.8191763911091,"minRttMs":647,"sentPackets":283,"sentWireBytes":91603,"receivedPackets":185,"receivedWireBytes":73690,"lastHeardMs":100431,"unsyncedBytes":896,"inflightPackets":124,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 568/1058 109/1073 214/1546","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340 599/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 186/779","checkpointScopes":"731/731","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166234198,"silences":10,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":1687.5,"deliveredRate":59.62030459769455,"loss":0.9106677546579627,"srttMs":1217.5917793422204,"minRttMs":680,"sentPackets":308,"sentWireBytes":99605,"receivedPackets":190,"receivedWireBytes":75115,"lastHeardMs":111311,"unsyncedBytes":497,"inflightPackets":134,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 568/1058 109/1073 214/1546","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340 681/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 186/779","checkpointScopes":"731/731","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166239713,"silences":10,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":1687.5,"deliveredRate":59.62030459769455,"loss":0.9106677546579627,"srttMs":1217.5917793422204,"minRttMs":680,"sentPackets":341,"sentWireBytes":109019,"receivedPackets":194,"receivedWireBytes":76247,"lastHeardMs":116825,"unsyncedBytes":497,"inflightPackets":144,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 568/1058 109/1073 214/1546","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546","assistant":"32/32 21/21 1340/1340 681/723","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"809/809 1019/1019","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","checkpoints":"71/613 613/613 186/779","checkpointScopes":"731/731","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":241,"lastDataAt":1794166239713,"silences":10,"longestSilenceMs":22110,"longestRecoveredGapMs":22110},"stats":{"sendRate":1687.5,"deliveredRate":59.62030459769455,"loss":0.9106677546579627,"srttMs":1217.5917793422204,"minRttMs":680,"sentPackets":366,"sentWireBytes":117012,"receivedPackets":194,"receivedWireBytes":76247,"lastHeardMs":116825,"unsyncedBytes":1160,"inflightPackets":126,"duplicateBytes":0}}
- shim trace
19:28:43.164 fetch GET /api/auth/session
19:28:43.165 fetched GET /api/auth/session 200
19:28:43.266 fetch GET /.well-known/t3/environment
19:28:43.267 fetched GET /.well-known/t3/environment 200
19:28:43.274 fetch GET /.well-known/t3/environment
19:28:43.275 fetched GET /.well-known/t3/environment 200
19:28:43.278 rpc 0 subscribeServerConfig
19:28:43.279 chunk 0 38710 bytes
19:28:43.336 in Ack 0
19:28:43.338 rpc 1 subscribeServerLifecycle
19:28:43.338 rpc 2 subscribeServerLifecycle
19:28:43.340 fetch GET /api/auth/session
19:28:43.341 fetched GET /api/auth/session 200
19:28:43.342 rpc 3 subscribeDeviceState
19:28:43.343 rpc 4 preview.reportProfiles
19:28:43.384 rpc 5 subscribeProjectClones
19:28:43.400 fetch GET /api/orchestration/threads/mcp%3A03ad388c-2d84-40f9-b1ed-7391c5d274da/bounded
19:28:43.404 fetch GET /api/orchestration/shell
19:28:43.405 fetched GET /api/orchestration/shell 200
19:28:43.406 rpc 6 orchestration.subscribeShell
19:28:43.406 chunk 6 2787 bytes
19:28:43.425 rpc 7 server.refreshProviders
19:28:43.464 rpc 8 orchestration.dispatchCommand thread.visit
19:28:43.464 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:28:43.477 rpc 9 subscribeTerminalMetadata
19:28:43.478 rpc 10 subscribeVcsStatus
19:28:43.478 rpc 11 subscribePreviewEvents
19:28:43.479 rpc 12 assets.createUrl
19:28:43.480 rpc 13 preview.list
19:28:43.480 rpc 14 projects.readFile
19:28:43.483 in Ack 6
19:28:43.621 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"aa772965-84d4-4a61-aac5-ab36efe11873","sessions":[]}}
19:28:43.621 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:28:43.731 rpc 15 server.reportClientActivity
19:28:43.738 chunk 1 3869 bytes
19:28:43.739 in Ack 1
19:28:44.220 chunk 2 3869 bytes
19:28:44.220 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:28:31.724Z","configurableProviders":false,"con
19:28:44.220 chunk 10 1576 bytes
19:28:44.221 in Ack 2
19:28:44.224 in Ack 10
19:28:44.266 fetch POST /api/observability/v1/traces
19:28:44.266 fetched POST /api/observability/v1/traces 204
19:28:44.359 chunk 6 2763 bytes
19:28:44.360 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:28:44.360 in Ack 6
19:28:44.548 chunk 3 876 bytes
19:28:44.548 exit 4 {"_tag":"Success","value":null}
19:28:44.548 chunk 5 4 bytes
19:28:44.548 chunk 9 36 bytes
19:28:44.548 exit 15 {"_tag":"Success","value":null}
19:28:44.549 in Ack 3
19:28:44.555 rpc 16 preview.reportProfiles
19:28:44.555 in Ack 5
19:28:44.555 in Ack 9
19:28:44.607 exit 16 {"_tag":"Success","value":null}
19:28:44.610 chunk 6 2806 bytes
19:28:44.610 in Ack 6
19:28:44.807 fetched GET /api/orchestration/threads/mcp%3A03ad388c-2d84-40f9-b1ed-7391c5d274da/bounded 200
19:28:44.809 rpc 17 orchestration.subscribeThread
19:28:44.810 chunk 17 8283 bytes
19:28:44.818 in Interrupt 10
19:28:44.820 rpc 18 subscribeVcsStatus
19:28:44.821 in Ack 17
19:28:44.950 chunk 18 1576 bytes
19:28:44.950 in Ack 18
19:28:45.072 chunk 17 11920 bytes
19:28:45.072 rpc 19 server.reportClientActivity
19:28:45.073 in Ack 17
19:28:45.322 chunk 17 13502 bytes
19:28:45.323 in Ack 17
19:28:45.361 fetch POST /api/observability/v1/traces
19:28:45.361 fetched POST /api/observability/v1/traces 204
19:28:45.569 exit 19 {"_tag":"Success","value":null}
19:28:45.573 chunk 17 19069 bytes
19:28:45.574 in Ack 17
19:28:45.680 rpc 20 attachments.createUploadUrl
19:28:45.829 chunk 17 19848 bytes
19:28:45.837 in Ack 17
19:28:45.845 in Interrupt 18
19:28:45.847 rpc 21 subscribeVcsStatus
19:28:45.914 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-b128c6bd-302b-4b28-b4a9-7c848755d273","expiresAt":1794166725878,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:28:45.914 chunk 21 1576 bytes
19:28:45.915 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWIxMjhjNmJkLTMwMmItNGIyOC1iNGE5LTdjODQ4NzU1ZDI3MyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjcyNTg3OH0.lncHBIEt82NsnusvmThTf29XA8IaHMOG4HaEhE2lWrU
19:28:45.915 in Ack 21
19:28:46.079 chunk 17 21193 bytes
19:28:46.080 in Ack 17
19:28:46.099 rpc 22 server.reportClientActivity
19:28:46.572 fetch POST /api/observability/v1/traces
19:28:46.572 fetched POST /api/observability/v1/traces 204
19:28:47.097 exit 22 {"_tag":"Success","value":null}
19:28:48.098 fetch POST /api/observability/v1/traces
19:28:48.098 fetched POST /api/observability/v1/traces 204
19:28:48.277 in Ping 
19:28:48.876 rpc 23 orchestration.dispatchCommand message.dispatch
19:28:49.467 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:28:49.469 rpc 24 attachments.delete
19:28:49.809 chunk 17 23627 bytes
19:28:49.810 in Ack 17
19:28:49.894 fetch POST /api/observability/v1/traces
19:28:49.894 fetched POST /api/observability/v1/traces 204
19:28:50.060 chunk 17 23637 bytes
19:28:50.061 in Ack 17
19:28:50.310 chunk 17 23659 bytes
19:28:50.321 in Ack 17
19:28:50.561 chunk 17 23661 bytes
19:28:50.561 in Ack 17
19:28:51.061 chunk 6 2828 bytes
19:28:51.061 chunk 17 24633 bytes
19:28:51.062 in Ack 6
19:28:51.063 rpc 25 orchestration.dispatchCommand thread.visit
19:28:51.063 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:28:51.105 fetch POST /api/observability/v1/traces
19:28:51.106 fetched POST /api/observability/v1/traces 204
19:28:51.107 in Ack 17
19:28:51.567 chunk 6 2828 bytes
19:28:51.765 in Ack 6
19:28:51.818 chunk 17 28087 bytes
19:28:51.819 in Ack 17
19:28:51.821 rpc 26 assets.createUrl
19:28:52.108 fetch POST /api/observability/v1/traces
19:28:52.108 fetched POST /api/observability/v1/traces 204
19:28:52.570 rpc 27 orchestration.dispatchCommand message.dispatch
19:28:53.277 in Ping 
19:28:53.571 fetch POST /api/observability/v1/traces
19:28:53.571 fetched POST /api/observability/v1/traces 204
19:28:54.917 fetch POST /api/observability/v1/traces
19:28:54.918 fetched POST /api/observability/v1/traces 204
19:28:56.318 fetch POST /api/observability/v1/traces
19:28:56.318 fetched POST /api/observability/v1/traces 204
19:28:58.277 in Ping 
19:29:03.277 in Ping 
19:29:08.278 in Ping 
19:29:08.518 rpc 28 server.reportClientActivity
19:29:09.518 fetch POST /api/observability/v1/traces
19:29:09.518 fetched POST /api/observability/v1/traces 204
19:29:13.278 in Ping 
19:29:17.210 chunk 17 30691 bytes
19:29:17.211 exit 27 {"_tag":"Success","value":{"sequence":75}}
19:29:17.212 in Ack 17
19:29:18.213 fetch POST /api/observability/v1/traces
19:29:18.213 fetched POST /api/observability/v1/traces 204
19:29:18.278 in Ping 
19:29:19.061 chunk 17 30871 bytes
19:29:19.062 in Ack 17
19:29:20.063 fetch POST /api/observability/v1/traces
19:29:20.064 fetched POST /api/observability/v1/traces 204
19:29:22.068 chunk 17 31181 bytes
19:29:22.069 in Ack 17
19:29:23.071 fetch POST /api/observability/v1/traces
19:29:23.071 fetched POST /api/observability/v1/traces 204
19:29:23.278 in Ping 
19:29:23.520 in Interrupt 28
19:29:24.521 fetch POST /api/observability/v1/traces
19:29:24.521 fetched POST /api/observability/v1/traces 204
19:29:24.802 chunk 17 31451 bytes
19:29:24.803 in Ack 17
19:29:25.804 fetch POST /api/observability/v1/traces
19:29:25.804 fetched POST /api/observability/v1/traces 204
19:29:28.279 fetch POST /api/observability/v1/traces
19:29:28.279 fetched POST /api/observability/v1/traces 204
19:29:28.280 in Ping 
19:29:33.280 in Ping 
19:29:33.518 rpc 29 server.reportClientActivity
19:29:34.518 fetch POST /api/observability/v1/traces
19:29:34.518 fetched POST /api/observability/v1/traces 204
19:29:38.283 in Ping 
19:29:41.169 chunk 17 31635 bytes
19:29:41.172 in Ack 17
19:29:41.420 chunk 17 31729 bytes
19:29:41.421 in Ack 17
19:29:41.982 chunk 17 31791 bytes
19:29:41.984 in Ack 17
19:29:42.174 fetch POST /api/observability/v1/traces
19:29:42.174 fetched POST /api/observability/v1/traces 204
19:29:42.329 chunk 17 31897 bytes
19:29:42.330 in Ack 17
19:29:43.138 chunk 17 31935 bytes
19:29:43.139 in Ack 17
19:29:43.287 in Ping 
19:29:43.331 fetch POST /api/observability/v1/traces
19:29:43.331 fetched POST /api/observability/v1/traces 204
19:29:43.388 chunk 17 31971 bytes
19:29:43.389 in Ack 17
19:29:43.798 chunk 17 32487 bytes
19:29:43.799 in Ack 17
19:29:44.273 chunk 17 32855 bytes
19:29:44.274 in Ack 17
19:29:44.390 fetch POST /api/observability/v1/traces
19:29:44.390 fetched POST /api/observability/v1/traces 204
19:29:47.630 chunk 17 32931 bytes
19:29:47.631 in Ack 17
19:29:47.992 chunk 17 33025 bytes
19:29:47.993 in Ack 17
19:29:48.291 in Ping 
19:29:48.519 in Interrupt 29
19:29:48.632 fetch POST /api/observability/v1/traces
19:29:48.632 fetched POST /api/observability/v1/traces 204
19:29:50.396 chunk 17 33093 bytes
19:29:50.398 in Ack 17
19:29:51.399 fetch POST /api/observability/v1/traces
19:29:51.399 fetched POST /api/observability/v1/traces 204
19:29:53.295 in Ping 
19:29:54.533 chunk 17 33199 bytes
19:29:54.534 in Ack 17
19:29:55.535 fetch POST /api/observability/v1/traces
19:29:55.535 fetched POST /api/observability/v1/traces 204
19:29:58.299 in Ping 
19:29:58.519 rpc 30 server.reportClientActivity
19:29:59.519 fetch POST /api/observability/v1/traces
19:29:59.519 fetched POST /api/observability/v1/traces 204
19:30:02.238 fetch POST /api/observability/v1/traces
19:30:02.238 fetched POST /api/observability/v1/traces 204
19:30:03.303 in Ping 
19:30:06.281 chunk 17 34204 bytes
19:30:06.283 in Ack 17
19:30:06.533 rpc 31 orchestration.dispatchCommand runtime-request.respond
19:30:07.285 fetch POST /api/observability/v1/traces
19:30:07.285 fetched POST /api/observability/v1/traces 204
19:30:08.307 in Ping 
19:30:12.241 fetch POST /api/observability/v1/traces
19:30:12.241 fetched POST /api/observability/v1/traces 204
19:30:13.313 in Ping 
19:30:13.519 in Interrupt 30
19:30:14.520 fetch POST /api/observability/v1/traces
19:30:14.520 fetched POST /api/observability/v1/traces 204
19:30:15.355 chunk 17 36744 bytes
19:30:15.355 exit 31 {"_tag":"Success","value":{"sequence":95}}
19:30:15.356 in Ack 17
19:30:16.267 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:30:16.357 fetch POST /api/observability/v1/traces
19:30:16.357 fetched POST /api/observability/v1/traces 204
19:30:17.272 chunk 17 36816 bytes
19:30:17.274 in Ack 17
19:30:18.274 fetch POST /api/observability/v1/traces
19:30:18.275 fetched POST /api/observability/v1/traces 204
19:30:18.315 in Ping 
19:30:18.445 chunk 17 36898 bytes
19:30:18.446 in Ack 17
19:30:19.448 fetch POST /api/observability/v1/traces
19:30:19.448 fetched POST /api/observability/v1/traces 204
19:30:20.834 chunk 17 36990 bytes
19:30:20.835 in Ack 17
19:30:21.836 fetch POST /api/observability/v1/traces
19:30:21.836 fetched POST /api/observability/v1/traces 204
19:30:23.320 chunk 17 37012 bytes
19:30:23.320 in Ping 
19:30:23.321 in Ack 17
19:30:23.519 rpc 33 server.reportClientActivity
19:30:24.323 fetch POST /api/observability/v1/traces
19:30:24.323 fetched POST /api/observability/v1/traces 204
19:30:25.994 chunk 17 37280 bytes
19:30:25.995 in Ack 17
19:30:27.000 fetch POST /api/observability/v1/traces
19:30:27.000 fetched POST /api/observability/v1/traces 204
19:30:28.189 chunk 17 37480 bytes
19:30:28.190 in Ack 17
19:30:28.323 in Ping 
19:30:29.003 chunk 17 37560 bytes
19:30:29.004 in Ack 17
19:30:29.192 fetch POST /api/observability/v1/traces
19:30:29.192 fetched POST /api/observability/v1/traces 204
19:30:32.247 fetch POST /api/observability/v1/traces
19:30:32.247 fetched POST /api/observability/v1/traces 204
19:30:33.328 in Ping 
19:30:33.433 chunk 17 37660 bytes
19:30:33.435 in Ack 17
19:30:34.200 chunk 17 37872 bytes
19:30:34.201 in Ack 17
19:30:34.435 fetch POST /api/observability/v1/traces
19:30:34.435 fetched POST /api/observability/v1/traces 204
19:30:38.331 in Ping 
19:30:38.521 in Interrupt 33
19:30:39.524 fetch POST /api/observability/v1/traces
19:30:39.524 fetched POST /api/observability/v1/traces 204
19:30:39.714 chunk 17 38046 bytes
19:30:39.715 in Ack 17
19:30:40.715 fetch POST /api/observability/v1/traces
19:30:40.715 fetched POST /api/observability/v1/traces 204
19:30:42.250 fetch POST /api/observability/v1/traces
19:30:42.251 fetched POST /api/observability/v1/traces 204
19:30:43.335 in Ping 
19:30:48.339 in Ping 
19:30:48.520 rpc 34 server.reportClientActivity
19:30:49.520 fetch POST /api/observability/v1/traces
19:30:49.520 fetched POST /api/observability/v1/traces 204
19:30:53.343 in Ping 
19:30:58.347 in Ping 
19:31:03.352 in Ping 
19:31:03.521 in Interrupt 34
19:31:04.522 fetch POST /api/observability/v1/traces
19:31:04.522 fetched POST /api/observability/v1/traces 204
19:31:06.075 chunk 17 38128 bytes
19:31:06.076 in Ack 17
19:31:07.123 fetch POST /api/observability/v1/traces
19:31:07.123 fetched POST /api/observability/v1/traces 204
