# case70, webapp, warm start, seed 4

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 69 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 12.4s |
| Prompt reached the agent | 10.2s |
| Time to first text | 24.5s |
| Whole reply shown | 67.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 13.7s / 4.4s / 11.3s |
| Longest silence | 24.5s |
| Approval shown after it was asked | 10.6s |
| Bytes on the wire down / up | 62889 / 27963 |
| Bytes the app received / sent | 48508 / 72707 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 9.7 | 0 | 389 |
| 30 | 4.8 | 597 | 792 |
| 40 | 9.7 | 803 | 1077 |
| 50 | 0.7 | 1077 | 1105 |
| 60 | 6.2 | 1254 | 1479 |
| 67 | 6.0 | 1516 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","messages":"456/456 548/548","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"39/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174022043,"silences":0,"longestSilenceMs":1109,"longestRecoveredGapMs":1109},"stats":{"sendRate":4500,"deliveredRate":2071.559633027523,"loss":0,"srttMs":5.658724458104468,"minRttMs":1,"sentPackets":101,"sentWireBytes":30772,"receivedPackets":118,"receivedWireBytes":57652,"lastHeardMs":9091,"unsyncedBytes":1164,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 360/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174031384,"silences":0,"longestSilenceMs":1109,"longestRecoveredGapMs":1109},"stats":{"sendRate":2250,"deliveredRate":1167.150166832803,"loss":0.39249999999999996,"srttMs":485.2074609132362,"minRttMs":1,"sentPackets":118,"sentWireBytes":40512,"receivedPackets":133,"receivedWireBytes":66543,"lastHeardMs":18431,"unsyncedBytes":949,"inflightPackets":27,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 360/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21 180/385","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174041518,"silences":1,"longestSilenceMs":3697,"longestRecoveredGapMs":3697},"stats":{"sendRate":2250,"deliveredRate":835.8032819205768,"loss":0.5474772727272726,"srttMs":1269.3065282990817,"minRttMs":1,"sentPackets":122,"sentWireBytes":43270,"receivedPackets":141,"receivedWireBytes":70642,"lastHeardMs":28565,"unsyncedBytes":597,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 971/1078 303/849","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 360/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21 848/913","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174053273,"silences":2,"longestSilenceMs":3697,"longestRecoveredGapMs":3697},"stats":{"sendRate":2308.732886627076,"deliveredRate":659.2209840749098,"loss":0.6082340909090909,"srttMs":1259.30006472141,"minRttMs":1,"sentPackets":187,"sentWireBytes":59262,"receivedPackets":165,"receivedWireBytes":78358,"lastHeardMs":40321,"unsyncedBytes":0,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 812/849","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 360/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21 1275/1340","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"154/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174063385,"silences":2,"longestSilenceMs":3697,"longestRecoveredGapMs":3697},"stats":{"sendRate":2308.732886627076,"deliveredRate":362.5676979017484,"loss":0.6449879295454545,"srttMs":1259.30006472141,"minRttMs":1,"sentPackets":236,"sentWireBytes":66245,"receivedPackets":199,"receivedWireBytes":88380,"lastHeardMs":50433,"unsyncedBytes":0,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 839/849 911/1550","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 360/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21 1340/1340 93/93","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174074162,"silences":2,"longestSilenceMs":3697,"longestRecoveredGapMs":3697},"stats":{"sendRate":1250,"deliveredRate":222.1368833797116,"loss":0.8480147058823528,"srttMs":1937.3195630077666,"minRttMs":1,"sentPackets":276,"sentWireBytes":79453,"receivedPackets":223,"receivedWireBytes":98334,"lastHeardMs":61209,"unsyncedBytes":69,"inflightPackets":98,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 1058/1058 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 911/1550","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776 517/776","messages":"456/456 548/548 142/901","assistant":"32/32 21/21 1340/1340 564/604","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"665/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"721/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":252,"lastDataAt":1794174084279,"silences":3,"longestSilenceMs":3697,"longestRecoveredGapMs":3697},"stats":{"sendRate":1632.2494633195643,"deliveredRate":0,"loss":0.7558129201680672,"srttMs":1657.4644133139507,"minRttMs":1,"sentPackets":322,"sentWireBytes":92070,"receivedPackets":234,"receivedWireBytes":103895,"lastHeardMs":71327,"unsyncedBytes":400,"inflightPackets":98,"duplicateBytes":0}}
- shim trace
21:40:13.245 fetch GET /api/auth/session
21:40:13.246 fetched GET /api/auth/session 200
21:40:13.366 fetch GET /.well-known/t3/environment
21:40:13.367 fetched GET /.well-known/t3/environment 200
21:40:13.391 fetch GET /.well-known/t3/environment
21:40:13.392 fetched GET /.well-known/t3/environment 200
21:40:13.396 rpc 0 subscribeServerConfig
21:40:13.397 chunk 0 38678 bytes
21:40:13.417 in Ack 0
21:40:13.419 rpc 1 subscribeServerLifecycle
21:40:13.419 rpc 2 subscribeServerLifecycle
21:40:13.422 fetch GET /api/auth/session
21:40:13.424 fetched GET /api/auth/session 200
21:40:13.424 rpc 3 subscribeDeviceState
21:40:13.426 rpc 4 preview.reportProfiles
21:40:13.456 rpc 5 subscribeProjectClones
21:40:13.474 fetch GET /api/orchestration/threads/mcp%3A102587a3-1a6f-4c2a-b6fa-cd34ea7a0f25/bounded
21:40:13.479 fetch GET /api/orchestration/shell
21:40:13.480 fetched GET /api/orchestration/shell 200
21:40:13.481 rpc 6 orchestration.subscribeShell
21:40:13.481 chunk 6 2779 bytes
21:40:13.502 rpc 7 server.refreshProviders
21:40:13.541 rpc 8 orchestration.dispatchCommand thread.visit
21:40:13.541 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:40:13.555 rpc 9 subscribeTerminalMetadata
21:40:13.556 rpc 10 subscribeVcsStatus
21:40:13.557 rpc 11 subscribePreviewEvents
21:40:13.558 rpc 12 assets.createUrl
21:40:13.562 rpc 13 preview.list
21:40:13.563 rpc 14 projects.readFile
21:40:13.565 in Ack 6
21:40:13.633 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"ad7c03cb-ade2-428f-8e09-e3c381f2fc75","sessions":[]}}
21:40:13.633 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:40:13.809 rpc 15 server.reportClientActivity
21:40:13.990 chunk 1 3861 bytes
21:40:13.991 in Ack 1
21:40:14.109 chunk 2 3861 bytes
21:40:14.110 in Ack 2
21:40:14.229 chunk 6 2755 bytes
21:40:14.229 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:40:01.096Z","configurableProviders":false,"con
21:40:14.229 chunk 10 472 bytes
21:40:14.229 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:40:14.230 in Ack 6
21:40:14.237 in Ack 10
21:40:14.366 fetch POST /api/observability/v1/traces
21:40:14.366 fetched POST /api/observability/v1/traces 204
21:40:14.469 chunk 3 876 bytes
21:40:14.469 exit 4 {"_tag":"Success","value":null}
21:40:14.469 in Ack 3
21:40:14.480 chunk 6 2798 bytes
21:40:14.480 in Ack 6
21:40:14.548 chunk 5 4 bytes
21:40:14.548 chunk 9 36 bytes
21:40:14.548 exit 15 {"_tag":"Success","value":null}
21:40:14.548 in Ack 5
21:40:14.548 in Ack 9
21:40:14.739 fetched GET /api/orchestration/threads/mcp%3A102587a3-1a6f-4c2a-b6fa-cd34ea7a0f25/bounded 200
21:40:14.742 rpc 16 orchestration.subscribeThread
21:40:14.742 chunk 16 8294 bytes
21:40:14.751 in Interrupt 10
21:40:14.753 rpc 17 subscribeVcsStatus
21:40:14.754 in Ack 16
21:40:15.090 chunk 16 9284 bytes
21:40:15.091 rpc 18 server.reportClientActivity
21:40:15.092 in Ack 16
21:40:15.280 chunk 17 472 bytes
21:40:15.284 in Ack 17
21:40:15.340 chunk 16 13517 bytes
21:40:15.341 in Ack 16
21:40:15.473 fetch POST /api/observability/v1/traces
21:40:15.473 fetched POST /api/observability/v1/traces 204
21:40:15.861 chunk 16 18034 bytes
21:40:15.862 in Ack 16
21:40:15.919 rpc 19 attachments.createUploadUrl
21:40:16.079 exit 18 {"_tag":"Success","value":null}
21:40:16.111 chunk 16 19860 bytes
21:40:16.111 in Ack 16
21:40:16.118 in Interrupt 17
21:40:16.120 rpc 20 subscribeVcsStatus
21:40:16.124 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-65fef04f-2870-467f-a9f1-022f815f0c56","expiresAt":1794174616108,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:40:16.125 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTY1ZmVmMDRmLTI4NzAtNDY3Zi1hOWYxLTAyMmY4MTVmMGM1NiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NDYxNjEwOH0.g2XKvMTblmsKHmiLBPa9vsinY6UtdOazKL53dbMzEbQ
21:40:16.182 chunk 20 472 bytes
21:40:16.182 in Ack 20
21:40:16.361 chunk 16 21197 bytes
21:40:16.362 in Ack 16
21:40:16.371 rpc 21 server.reportClientActivity
21:40:16.867 fetch POST /api/observability/v1/traces
21:40:16.867 fetched POST /api/observability/v1/traces 204
21:40:17.412 exit 21 {"_tag":"Success","value":null}
21:40:18.395 in Ping 
21:40:18.413 fetch POST /api/observability/v1/traces
21:40:18.413 fetched POST /api/observability/v1/traces 204
21:40:19.099 rpc 22 orchestration.dispatchCommand message.dispatch
21:40:19.790 exit 22 {"_tag":"Success","value":{"sequence":45}}
21:40:19.791 rpc 23 attachments.delete
21:40:20.098 fetch POST /api/observability/v1/traces
21:40:20.098 fetched POST /api/observability/v1/traces 204
21:40:20.293 chunk 16 23642 bytes
21:40:20.301 in Ack 16
21:40:20.542 chunk 16 23674 bytes
21:40:20.543 in Ack 16
21:40:21.054 chunk 6 2820 bytes
21:40:21.054 chunk 16 23676 bytes
21:40:21.054 in Ack 6
21:40:21.056 rpc 24 orchestration.dispatchCommand thread.visit
21:40:21.056 exit 24 {"_tag":"Success","value":{"sequence":0}}
21:40:21.080 in Ack 16
21:40:21.114 exit 23 {"_tag":"Success","value":null}
21:40:21.302 fetch POST /api/observability/v1/traces
21:40:21.302 fetched POST /api/observability/v1/traces 204
21:40:21.304 chunk 16 25310 bytes
21:40:21.305 in Ack 16
21:40:21.554 chunk 6 2820 bytes
21:40:21.554 chunk 16 28103 bytes
21:40:21.555 in Ack 6
21:40:21.560 in Ack 16
21:40:21.562 rpc 25 assets.createUrl
21:40:22.035 chunk 16 31336 bytes
21:40:22.036 in Ack 16
21:40:22.285 chunk 16 32116 bytes
21:40:22.287 in Ack 16
21:40:22.293 in Interrupt 20
21:40:22.295 rpc 26 subscribeVcsStatus
21:40:22.310 fetch POST /api/observability/v1/traces
21:40:22.310 fetched POST /api/observability/v1/traces 204
21:40:22.550 rpc 27 server.reportClientActivity
21:40:22.810 rpc 28 orchestration.dispatchCommand message.dispatch
21:40:23.395 in Ping 
21:40:23.550 fetch POST /api/observability/v1/traces
21:40:23.550 fetched POST /api/observability/v1/traces 204
21:40:24.987 fetch POST /api/observability/v1/traces
21:40:24.987 fetched POST /api/observability/v1/traces 204
21:40:26.290 fetch POST /api/observability/v1/traces
21:40:26.290 fetched POST /api/observability/v1/traces 204
21:40:26.627 chunk 26 472 bytes
21:40:26.628 in Ack 26
21:40:28.395 in Ping 
21:40:29.444 chunk 16 31950 bytes
21:40:29.445 in Ack 16
21:40:30.446 fetch POST /api/observability/v1/traces
21:40:30.446 fetched POST /api/observability/v1/traces 204
21:40:31.385 chunk 16 31950 bytes
21:40:31.386 in Ack 16
21:40:32.386 fetch POST /api/observability/v1/traces
21:40:32.386 fetched POST /api/observability/v1/traces 204
21:40:33.396 in Ping 
21:40:35.082 chunk 16 34457 bytes
21:40:35.083 exit 28 {"_tag":"Success","value":{"sequence":75}}
21:40:35.084 in Ack 16
21:40:36.085 fetch POST /api/observability/v1/traces
21:40:36.085 fetched POST /api/observability/v1/traces 204
21:40:36.658 chunk 16 34539 bytes
21:40:36.659 in Ack 16
21:40:37.027 chunk 16 34559 bytes
21:40:37.028 in Ack 16
21:40:37.464 chunk 16 34589 bytes
21:40:37.466 in Ack 16
21:40:37.551 in Interrupt 27
21:40:37.660 fetch POST /api/observability/v1/traces
21:40:37.660 fetched POST /api/observability/v1/traces 204
21:40:37.902 chunk 16 34609 bytes
21:40:37.903 in Ack 16
21:40:38.396 in Ping 
21:40:38.618 rpc 29 server.reportClientActivity
21:40:38.904 fetch POST /api/observability/v1/traces
21:40:38.904 fetched POST /api/observability/v1/traces 204
21:40:39.671 chunk 16 34681 bytes
21:40:39.672 in Ack 16
21:40:40.673 fetch POST /api/observability/v1/traces
21:40:40.673 fetched POST /api/observability/v1/traces 204
21:40:40.986 chunk 16 34733 bytes
21:40:40.987 in Ack 16
21:40:41.519 chunk 16 34755 bytes
21:40:41.520 in Ack 16
21:40:41.988 fetch POST /api/observability/v1/traces
21:40:41.988 fetched POST /api/observability/v1/traces 204
21:40:43.396 in Ping 
21:40:44.559 chunk 16 34787 bytes
21:40:44.560 in Ack 16
21:40:44.916 chunk 16 35141 bytes
21:40:44.921 in Ack 16
21:40:45.213 chunk 16 35317 bytes
21:40:45.214 in Ack 16
21:40:45.561 fetch POST /api/observability/v1/traces
21:40:45.561 fetched POST /api/observability/v1/traces 204
21:40:45.671 chunk 16 35527 bytes
21:40:45.672 in Ack 16
21:40:45.929 chunk 16 35547 bytes
21:40:45.931 in Ack 16
21:40:46.364 chunk 16 35557 bytes
21:40:46.366 in Ack 16
21:40:46.615 chunk 16 35585 bytes
21:40:46.616 in Ack 16
21:40:46.673 fetch POST /api/observability/v1/traces
21:40:46.673 fetched POST /api/observability/v1/traces 204
21:40:47.044 chunk 16 35609 bytes
21:40:47.045 in Ack 16
21:40:48.046 fetch POST /api/observability/v1/traces
21:40:48.046 fetched POST /api/observability/v1/traces 204
21:40:48.397 in Ping 
21:40:48.591 chunk 16 35797 bytes
21:40:48.592 in Ack 16
21:40:49.283 chunk 16 35887 bytes
21:40:49.284 in Ack 16
21:40:49.538 chunk 16 35907 bytes
21:40:49.539 in Ack 16
21:40:49.593 fetch POST /api/observability/v1/traces
21:40:49.593 fetched POST /api/observability/v1/traces 204
21:40:50.105 chunk 16 35929 bytes
21:40:50.106 in Ack 16
21:40:50.357 chunk 16 35939 bytes
21:40:50.359 in Ack 16
21:40:50.741 chunk 16 35981 bytes
21:40:50.742 in Ack 16
21:40:50.996 chunk 16 36001 bytes
21:40:50.998 in Ack 16
21:40:51.107 fetch POST /api/observability/v1/traces
21:40:51.107 fetched POST /api/observability/v1/traces 204
21:40:51.251 chunk 16 36021 bytes
21:40:51.253 in Ack 16
21:40:51.762 chunk 16 36033 bytes
21:40:51.763 in Ack 16
21:40:52.014 chunk 16 36053 bytes
21:40:52.015 in Ack 16
21:40:52.253 fetch POST /api/observability/v1/traces
21:40:52.253 fetched POST /api/observability/v1/traces 204
21:40:52.269 chunk 16 36063 bytes
21:40:52.271 in Ack 16
21:40:52.521 chunk 16 36073 bytes
21:40:52.522 in Ack 16
21:40:53.027 chunk 16 36083 bytes
21:40:53.028 in Ack 16
21:40:53.272 fetch POST /api/observability/v1/traces
21:40:53.272 fetched POST /api/observability/v1/traces 204
21:40:53.277 chunk 16 36093 bytes
21:40:53.278 in Ack 16
21:40:53.397 in Ping 
21:40:53.529 chunk 16 36103 bytes
21:40:53.530 in Ack 16
21:40:53.618 in Interrupt 29
21:40:53.779 chunk 16 36113 bytes
21:40:53.780 in Ack 16
21:40:54.030 chunk 16 36125 bytes
21:40:54.031 in Ack 16
21:40:54.279 fetch POST /api/observability/v1/traces
21:40:54.279 fetched POST /api/observability/v1/traces 204
21:40:54.282 chunk 16 36135 bytes
21:40:54.283 in Ack 16
21:40:54.537 chunk 16 36259 bytes
21:40:54.538 in Ack 16
21:40:54.787 chunk 16 36289 bytes
21:40:54.789 in Ack 16
21:40:55.041 chunk 16 36261 bytes
21:40:55.043 in Ack 16
21:40:55.283 fetch POST /api/observability/v1/traces
21:40:55.284 fetched POST /api/observability/v1/traces 204
21:40:55.296 chunk 16 36271 bytes
21:40:55.298 in Ack 16
21:40:55.551 chunk 16 36283 bytes
21:40:55.553 in Ack 16
21:40:55.802 chunk 16 36293 bytes
21:40:55.803 in Ack 16
21:40:56.053 chunk 16 36321 bytes
21:40:56.055 in Ack 16
21:40:56.300 fetch POST /api/observability/v1/traces
21:40:56.300 fetched POST /api/observability/v1/traces 204
21:40:56.390 chunk 16 36331 bytes
21:40:56.391 in Ack 16
21:40:56.642 chunk 16 36351 bytes
21:40:56.644 in Ack 16
21:40:56.953 chunk 16 36361 bytes
21:40:56.954 in Ack 16
21:40:57.206 chunk 16 36393 bytes
21:40:57.207 in Ack 16
21:40:57.392 fetch POST /api/observability/v1/traces
21:40:57.392 fetched POST /api/observability/v1/traces 204
21:40:57.805 chunk 16 36431 bytes
21:40:57.806 in Ack 16
21:40:58.055 chunk 16 36443 bytes
21:40:58.057 in Ack 16
21:40:58.309 chunk 16 36499 bytes
21:40:58.310 in Ack 16
21:40:58.397 in Ping 
21:40:58.561 chunk 16 36539 bytes
21:40:58.562 in Ack 16
21:40:58.807 fetch POST /api/observability/v1/traces
21:40:58.807 fetched POST /api/observability/v1/traces 204
21:40:58.898 chunk 16 36609 bytes
21:40:58.899 in Ack 16
21:40:59.510 chunk 16 36629 bytes
21:40:59.511 in Ack 16
21:40:59.763 chunk 16 36671 bytes
21:40:59.766 in Ack 16
21:40:59.900 fetch POST /api/observability/v1/traces
21:40:59.900 fetched POST /api/observability/v1/traces 204
21:41:00.021 chunk 16 36691 bytes
21:41:00.022 in Ack 16
21:41:00.272 chunk 16 36703 bytes
21:41:00.272 in Ack 16
21:41:00.908 chunk 16 36725 bytes
21:41:00.909 in Ack 16
21:41:01.023 fetch POST /api/observability/v1/traces
21:41:01.023 fetched POST /api/observability/v1/traces 204
21:41:01.244 chunk 16 36735 bytes
21:41:01.246 in Ack 16
21:41:01.754 chunk 16 36765 bytes
21:41:01.756 in Ack 16
21:41:02.019 chunk 16 36785 bytes
21:41:02.020 in Ack 16
21:41:02.246 fetch POST /api/observability/v1/traces
21:41:02.247 fetched POST /api/observability/v1/traces 204
21:41:02.595 chunk 16 36807 bytes
21:41:02.596 in Ack 16
21:41:02.852 chunk 16 36827 bytes
21:41:02.853 in Ack 16
21:41:03.386 chunk 16 36903 bytes
21:41:03.388 in Ack 16
21:41:03.408 in Ping 
21:41:03.597 fetch POST /api/observability/v1/traces
21:41:03.597 fetched POST /api/observability/v1/traces 204
21:41:03.618 rpc 30 server.reportClientActivity
21:41:04.171 chunk 16 36911 bytes
21:41:04.172 in Ack 16
21:41:04.618 fetch POST /api/observability/v1/traces
21:41:04.618 fetched POST /api/observability/v1/traces 204
21:41:04.877 chunk 16 36929 bytes
21:41:04.879 in Ack 16
21:41:05.661 chunk 16 36949 bytes
21:41:05.662 in Ack 16
21:41:05.879 fetch POST /api/observability/v1/traces
21:41:05.879 fetched POST /api/observability/v1/traces 204
21:41:06.196 chunk 16 36987 bytes
21:41:06.198 in Ack 16
21:41:07.199 fetch POST /api/observability/v1/traces
21:41:07.199 fetched POST /api/observability/v1/traces 204
21:41:08.411 in Ping 
21:41:08.887 chunk 16 37025 bytes
21:41:08.889 in Ack 16
21:41:09.890 fetch POST /api/observability/v1/traces
21:41:09.890 fetched POST /api/observability/v1/traces 204
21:41:10.254 chunk 16 38033 bytes
21:41:10.255 in Ack 16
21:41:10.467 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:41:11.256 fetch POST /api/observability/v1/traces
21:41:11.256 fetched POST /api/observability/v1/traces 204
21:41:13.415 in Ping 
21:41:14.163 chunk 16 40654 bytes
21:41:14.163 exit 31 {"_tag":"Success","value":{"sequence":95}}
21:41:14.164 in Ack 16
21:41:14.929 chunk 16 40714 bytes
21:41:14.930 in Ack 16
21:41:15.165 fetch POST /api/observability/v1/traces
21:41:15.165 fetched POST /api/observability/v1/traces 204
21:41:15.366 chunk 16 40734 bytes
21:41:15.367 in Ack 16
21:41:15.399 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:41:16.317 fetch POST /api/observability/v1/traces
21:41:16.317 fetched POST /api/observability/v1/traces 204
21:41:16.843 chunk 16 40806 bytes
21:41:16.844 in Ack 16
21:41:17.479 chunk 16 40876 bytes
21:41:17.480 in Ack 16
21:41:17.845 fetch POST /api/observability/v1/traces
21:41:17.845 fetched POST /api/observability/v1/traces 204
21:41:18.420 in Ping 
21:41:18.619 in Interrupt 30
21:41:19.620 fetch POST /api/observability/v1/traces
21:41:19.620 fetched POST /api/observability/v1/traces 204
21:41:19.964 chunk 16 41136 bytes
21:41:19.965 in Ack 16
21:41:20.966 fetch POST /api/observability/v1/traces
21:41:20.966 fetched POST /api/observability/v1/traces 204
21:41:21.241 chunk 16 41170 bytes
21:41:21.241 exit 32 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:41:21.243 in Ack 16
21:41:21.584 chunk 16 41364 bytes
21:41:21.587 in Ack 16
21:41:21.875 chunk 16 41394 bytes
21:41:21.877 in Ack 16
21:41:22.152 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:41:22.244 fetch POST /api/observability/v1/traces
21:41:22.244 fetched POST /api/observability/v1/traces 204
21:41:22.747 chunk 16 41480 bytes
21:41:22.748 in Ack 16
21:41:23.424 in Ping 
21:41:23.750 fetch POST /api/observability/v1/traces
21:41:23.750 fetched POST /api/observability/v1/traces 204
21:41:24.032 chunk 16 41596 bytes
21:41:24.033 in Ack 16
21:41:24.282 chunk 16 41618 bytes
21:41:24.284 in Ack 16
21:41:25.034 fetch POST /api/observability/v1/traces
21:41:25.034 fetched POST /api/observability/v1/traces 204
21:41:26.319 fetch POST /api/observability/v1/traces
21:41:26.319 fetched POST /api/observability/v1/traces 204
21:41:28.427 in Ping 
21:41:28.552 chunk 16 41767 bytes
21:41:28.552 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:41:28.555 in Ack 16
21:41:28.669 rpc 34 server.reportClientActivity
21:41:29.556 fetch POST /api/observability/v1/traces
21:41:29.556 fetched POST /api/observability/v1/traces 204
21:41:30.350 chunk 16 42011 bytes
21:41:30.352 in Ack 16
