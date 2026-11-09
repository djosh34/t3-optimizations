# story, webapp, warm start, seed 42

Profile story (graded): seed 42's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 90 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.8s |
| Prompt reached the agent | 6.3s |
| Time to first text | 56.3s |
| Whole reply shown | 87.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 51.1s / 16.3s / 48.1s |
| Longest silence | 56.3s |
| Approval shown after it was asked | 29.1s |
| Bytes on the wire down / up | 241015 / 127351 |
| Bytes the app received / sent | 107717 / 99662 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 3.6 | 0 | 145 |
| 20 | 13.6 | 0 | 547 |
| 30 | 23.6 | 0 | 950 |
| 40 | 33.6 | 0 | 1077 |
| 50 | 43.6 | 0 | 1077 |
| 60 | 51.1 | 101 | 1077 |
| 70 | 5.3 | 1077 | 1288 |
| 80 | 15.3 | 1077 | 1596 |
| 87 | 22.3 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217066291,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":7085.06105295894,"deliveredRate":2897.8452533441086,"loss":0,"srttMs":14.432655486290857,"minRttMs":1,"sentPackets":136,"sentWireBytes":38582,"receivedPackets":158,"receivedWireBytes":109856,"lastHeardMs":16931,"unsyncedBytes":1193,"inflightPackets":20,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 185/818","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 451/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"133/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217076238,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1771.265263239735,"deliveredRate":0,"loss":0.09230769230769231,"srttMs":4788.005678273399,"minRttMs":1,"sentPackets":183,"sentWireBytes":51609,"receivedPackets":240,"receivedWireBytes":154782,"lastHeardMs":26878,"unsyncedBytes":457,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 675/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 599/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217078128,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1771.265263239735,"deliveredRate":1455.1665243381724,"loss":0.5714285714285714,"srttMs":5021.598097428071,"minRttMs":1,"sentPackets":212,"sentWireBytes":61286,"receivedPackets":257,"receivedWireBytes":162323,"lastHeardMs":28768,"unsyncedBytes":624,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 675/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 599/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217078128,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1771.265263239735,"deliveredRate":1455.1665243381724,"loss":0.5714285714285714,"srttMs":5021.598097428071,"minRttMs":1,"sentPackets":232,"sentWireBytes":68366,"receivedPackets":257,"receivedWireBytes":162323,"lastHeardMs":28768,"unsyncedBytes":624,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 675/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 599/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217078128,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1771.265263239735,"deliveredRate":1455.1665243381724,"loss":0.5714285714285714,"srttMs":5021.598097428071,"minRttMs":1,"sentPackets":247,"sentWireBytes":73675,"receivedPackets":257,"receivedWireBytes":162323,"lastHeardMs":28768,"unsyncedBytes":1081,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 675/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659 599/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217078128,"silences":0,"longestSilenceMs":1604,"longestRecoveredGapMs":1604},"stats":{"sendRate":1771.265263239735,"deliveredRate":1455.1665243381724,"loss":0.5714285714285714,"srttMs":5021.598097428071,"minRttMs":1,"sentPackets":266,"sentWireBytes":80409,"receivedPackets":257,"receivedWireBytes":162323,"lastHeardMs":28768,"unsyncedBytes":1081,"inflightPackets":99,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 675/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"659/659 659/659 623/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691","runtimeRequests":"592/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217127111,"silences":1,"longestSilenceMs":43926,"longestRecoveredGapMs":43926},"stats":{"sendRate":3033.035472873543,"deliveredRate":1516.4775552611775,"loss":0.32239252336448593,"srttMs":2905.376033401203,"minRttMs":1940,"sentPackets":323,"sentWireBytes":97179,"receivedPackets":282,"receivedWireBytes":172956,"lastHeardMs":77751,"unsyncedBytes":457,"inflightPackets":125,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 68/1566 923/923 1093/1093","frame":"1029/1029","thread":"589/961","runs":"1014/1014 971/971 827/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"659/659 659/659 635/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217136087,"silences":1,"longestSilenceMs":43926,"longestRecoveredGapMs":43926},"stats":{"sendRate":4304.752352669633,"deliveredRate":1858.0606140897457,"loss":0.335841167246025,"srttMs":2923.4978280409896,"minRttMs":1940,"sentPackets":400,"sentWireBytes":125735,"receivedPackets":328,"receivedWireBytes":198010,"lastHeardMs":86727,"unsyncedBytes":0,"inflightPackets":146,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 68/1566 923/923 1093/1093","frame":"1029/1029","thread":"589/961","runs":"1014/1014 971/971 827/839","providerTurns":"772/772 772/772","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"659/659 659/659 635/635","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 40/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"254/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":710,"lastDataAt":1794217136087,"silences":1,"longestSilenceMs":43926,"longestRecoveredGapMs":43926},"stats":{"sendRate":4304.752352669633,"deliveredRate":1858.0606140897457,"loss":0.335841167246025,"srttMs":2923.4978280409896,"minRttMs":1940,"sentPackets":405,"sentWireBytes":126455,"receivedPackets":328,"receivedWireBytes":198010,"lastHeardMs":86727,"unsyncedBytes":0,"inflightPackets":146,"duplicateBytes":0}}
- shim trace
09:37:30.298 fetch GET /api/auth/session
09:37:30.301 fetched GET /api/auth/session 200
09:37:30.759 fetch GET /.well-known/t3/environment
09:37:30.761 fetched GET /.well-known/t3/environment 200
09:37:30.783 fetch GET /.well-known/t3/environment
09:37:30.784 fetched GET /.well-known/t3/environment 200
09:37:30.791 rpc 0 subscribeServerConfig
09:37:30.794 chunk 0 38462 bytes
09:37:30.847 in Ack 0
09:37:30.850 rpc 1 subscribeServerLifecycle
09:37:30.851 rpc 2 subscribeServerLifecycle
09:37:30.855 fetch GET /api/auth/session
09:37:30.857 fetched GET /api/auth/session 200
09:37:30.857 rpc 3 subscribeDeviceState
09:37:30.862 rpc 4 preview.reportProfiles
09:37:30.966 rpc 5 subscribeProjectClones
09:37:31.017 fetch GET /api/orchestration/threads/mcp%3A5405730c-62ee-4750-a8b8-841a9d279551/bounded
09:37:31.038 fetch GET /api/orchestration/shell
09:37:31.040 fetched GET /api/orchestration/shell 200
09:37:31.044 rpc 6 orchestration.subscribeShell
09:37:31.044 chunk 6 4594 bytes
09:37:31.101 rpc 7 server.refreshProviders
09:37:31.200 rpc 8 orchestration.dispatchCommand thread.visit
09:37:31.200 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:31.241 rpc 9 subscribeTerminalMetadata
09:37:31.242 rpc 10 subscribeVcsStatus
09:37:31.243 rpc 11 subscribePreviewEvents
09:37:31.246 rpc 12 assets.createUrl
09:37:31.247 rpc 13 preview.list
09:37:31.248 rpc 14 projects.readFile
09:37:31.252 in Ack 6
09:37:31.300 chunk 6 4570 bytes
09:37:31.303 in Ack 6
09:37:31.441 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"153bd25d-4e1d-4734-96ce-151a6bdb81a2","sessions":[]}}
09:37:31.441 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:31.611 rpc 15 server.reportClientActivity
09:37:31.679 chunk 1 3815 bytes
09:37:31.685 in Ack 1
09:37:31.750 fetch POST /api/observability/v1/traces
09:37:31.750 fetched POST /api/observability/v1/traces 204
09:37:32.076 chunk 2 3815 bytes
09:37:32.076 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:02.951Z","configurableProviders":false,"con
09:37:32.076 chunk 10 1632 bytes
09:37:32.077 in Ack 2
09:37:32.084 in Ack 10
09:37:32.272 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:32.556 chunk 6 4656 bytes
09:37:32.557 in Ack 6
09:37:32.806 chunk 6 4699 bytes
09:37:32.806 in Ack 6
09:37:33.055 chunk 6 4638 bytes
09:37:33.056 in Ack 6
09:37:33.074 chunk 3 876 bytes
09:37:33.074 exit 4 {"_tag":"Success","value":null}
09:37:33.074 chunk 5 4 bytes
09:37:33.074 chunk 9 36 bytes
09:37:33.074 exit 15 {"_tag":"Success","value":null}
09:37:33.076 in Ack 3
09:37:33.082 rpc 16 preview.reportProfiles
09:37:33.082 in Ack 5
09:37:33.083 in Ack 9
09:37:33.088 fetch POST /api/observability/v1/traces
09:37:33.088 fetched POST /api/observability/v1/traces 204
09:37:33.154 exit 16 {"_tag":"Success","value":null}
09:37:33.806 chunk 6 4635 bytes
09:37:33.807 in Ack 6
09:37:34.057 chunk 6 4635 bytes
09:37:34.057 in Ack 6
09:37:34.155 fetch POST /api/observability/v1/traces
09:37:34.155 fetched POST /api/observability/v1/traces 204
09:37:35.787 in Ping 
09:37:35.806 fetched GET /api/orchestration/threads/mcp%3A5405730c-62ee-4750-a8b8-841a9d279551/bounded 200
09:37:35.812 rpc 17 orchestration.subscribeThread
09:37:35.813 chunk 17 8282 bytes
09:37:35.830 in Interrupt 10
09:37:35.833 rpc 18 subscribeVcsStatus
09:37:35.835 in Ack 17
09:37:35.978 chunk 18 1632 bytes
09:37:35.978 in Ack 18
09:37:36.063 chunk 17 11259 bytes
09:37:36.064 in Ack 17
09:37:36.134 rpc 19 server.reportClientActivity
09:37:36.313 chunk 17 13497 bytes
09:37:36.319 in Ack 17
09:37:36.468 exit 19 {"_tag":"Success","value":null}
09:37:36.565 chunk 17 16150 bytes
09:37:36.567 in Ack 17
09:37:36.826 rpc 20 attachments.createUploadUrl
09:37:36.922 fetch POST /api/observability/v1/traces
09:37:36.922 fetched POST /api/observability/v1/traces 204
09:37:36.923 chunk 17 18552 bytes
09:37:36.940 in Ack 17
09:37:36.985 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-5a513664-1a18-4aa1-b2b1-c44c85150d2a","expiresAt":1794217656967,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:36.987 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTVhNTEzNjY0LTFhMTgtNGFhMS1iMmIxLWM0NGM4NTE1MGQyYSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1Njk2N30.oKdGZxJ3squL-EiDWrc-iso6VneUEY1W2UOBau3eBJQ
09:37:37.173 chunk 17 21099 bytes
09:37:37.175 in Ack 17
09:37:37.187 in Interrupt 18
09:37:37.191 rpc 21 subscribeVcsStatus
09:37:37.230 chunk 21 1632 bytes
09:37:37.230 in Ack 21
09:37:37.442 rpc 22 server.reportClientActivity
09:37:37.941 fetch POST /api/observability/v1/traces
09:37:37.941 fetched POST /api/observability/v1/traces 204
09:37:38.710 exit 22 {"_tag":"Success","value":null}
09:37:39.729 fetch POST /api/observability/v1/traces
09:37:39.729 fetched POST /api/observability/v1/traces 204
09:37:40.070 rpc 23 orchestration.dispatchCommand message.dispatch
09:37:40.550 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:37:40.555 rpc 24 attachments.delete
09:37:40.788 in Ping 
09:37:40.821 chunk 6 4721 bytes
09:37:40.822 chunk 17 23892 bytes
09:37:40.823 in Ack 6
09:37:40.892 in Ack 17
09:37:40.899 rpc 25 assets.createUrl
09:37:40.950 exit 24 {"_tag":"Success","value":null}
09:37:41.078 fetch POST /api/observability/v1/traces
09:37:41.078 fetched POST /api/observability/v1/traces 204
09:37:41.080 chunk 17 26023 bytes
09:37:41.082 in Ack 17
09:37:41.194 rpc 26 orchestration.dispatchCommand thread.visit
09:37:41.194 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:41.330 chunk 17 29266 bytes
09:37:41.334 in Ack 17
09:37:41.580 chunk 6 4660 bytes
09:37:41.581 chunk 17 29268 bytes
09:37:41.582 in Ack 6
09:37:41.601 in Ack 17
09:37:41.832 chunk 17 29290 bytes
09:37:41.835 in Ack 17
09:37:42.111 chunk 17 30991 bytes
09:37:42.113 fetch POST /api/observability/v1/traces
09:37:42.113 fetched POST /api/observability/v1/traces 204
09:37:42.116 in Ack 17
09:37:42.365 chunk 17 32034 bytes
09:37:42.369 in Ack 17
09:37:42.617 chunk 6 4657 bytes
09:37:42.617 chunk 17 34481 bytes
09:37:42.618 in Ack 6
09:37:42.620 rpc 27 orchestration.dispatchCommand thread.visit
09:37:42.620 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:42.633 in Ack 17
09:37:43.165 chunk 6 4657 bytes
09:37:43.166 chunk 17 35259 bytes
09:37:43.167 fetch POST /api/observability/v1/traces
09:37:43.167 fetched POST /api/observability/v1/traces 204
09:37:43.169 in Ack 6
09:37:43.178 in Ack 17
09:37:43.188 in Interrupt 21
09:37:43.190 rpc 28 subscribeVcsStatus
09:37:43.246 chunk 28 1632 bytes
09:37:43.246 in Ack 28
09:37:43.400 rpc 29 server.refreshProviders
09:37:43.400 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:02.951Z","configurableProviders":false,"con
09:37:43.452 rpc 30 server.reportClientActivity
09:37:44.186 fetch POST /api/observability/v1/traces
09:37:44.186 fetched POST /api/observability/v1/traces 204
09:37:45.829 in Ping 
09:37:46.193 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:47.050 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/f65d66ff-e9bb-45b6-a182-6e23bfab0ac7","expiresAt":1794220667050}}
09:37:47.193 fetch POST /api/observability/v1/traces
09:37:47.193 fetched POST /api/observability/v1/traces 204
09:37:47.499 exit 30 {"_tag":"Success","value":null}
09:37:48.500 fetch POST /api/observability/v1/traces
09:37:48.500 fetched POST /api/observability/v1/traces 204
09:37:49.822 chunk 6 4721 bytes
09:37:49.824 in Ack 6
09:37:50.826 fetch POST /api/observability/v1/traces
09:37:50.826 fetched POST /api/observability/v1/traces 204
09:37:50.830 in Ping 
09:37:51.616 exit 31 {"_tag":"Success","value":{"sequence":153}}
09:37:52.121 chunk 17 35843 bytes
09:37:52.128 in Ack 17
09:37:52.620 fetch POST /api/observability/v1/traces
09:37:52.620 fetched POST /api/observability/v1/traces 204
09:37:55.831 in Ping 
09:37:56.022 rpc 32 server.reportClientActivity
09:37:57.023 fetch POST /api/observability/v1/traces
09:37:57.023 fetched POST /api/observability/v1/traces 204
09:37:57.626 chunk 6 4785 bytes
09:37:57.628 in Ack 6
09:37:57.633 rpc 33 orchestration.dispatchCommand thread.visit
09:37:57.633 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:58.630 fetch POST /api/observability/v1/traces
09:37:58.630 fetched POST /api/observability/v1/traces 204
09:38:00.832 in Ping 
09:38:02.715 fetch POST /api/observability/v1/traces
09:38:02.715 fetched POST /api/observability/v1/traces 204
09:38:05.833 in Ping 
09:38:10.833 in Ping 
09:38:11.029 in Interrupt 32
09:38:12.033 fetch POST /api/observability/v1/traces
09:38:12.033 fetched POST /api/observability/v1/traces 204
09:38:15.833 in Ping 
09:38:20.833 in Ping 
09:38:21.019 rpc 34 server.reportClientActivity
09:38:22.020 fetch POST /api/observability/v1/traces
09:38:22.021 fetched POST /api/observability/v1/traces 204
09:38:25.837 in Ping 
09:38:30.842 in Ping 
09:38:35.846 in Ping 
09:38:36.020 in Interrupt 34
09:38:37.025 fetch POST /api/observability/v1/traces
09:38:37.025 fetched POST /api/observability/v1/traces 204
09:38:40.849 in Ping 
09:38:42.056 chunk 17 38508 bytes
09:38:42.059 in Ack 17
09:38:43.061 fetch POST /api/observability/v1/traces
09:38:43.061 fetched POST /api/observability/v1/traces 204
09:38:43.450 chunk 17 38880 bytes
09:38:43.452 in Ack 17
09:38:44.017 chunk 17 39292 bytes
09:38:44.020 in Ack 17
09:38:44.268 chunk 17 39610 bytes
09:38:44.270 in Ack 17
09:38:44.454 fetch POST /api/observability/v1/traces
09:38:44.454 fetched POST /api/observability/v1/traces 204
09:38:44.551 chunk 17 39650 bytes
09:38:44.555 in Ack 17
09:38:44.802 chunk 17 39974 bytes
09:38:44.804 in Ack 17
09:38:45.052 chunk 17 40010 bytes
09:38:45.055 in Ack 17
09:38:45.302 chunk 17 40352 bytes
09:38:45.305 in Ack 17
09:38:45.552 chunk 17 40386 bytes
09:38:45.554 in Ack 17
09:38:45.573 fetch POST /api/observability/v1/traces
09:38:45.573 fetched POST /api/observability/v1/traces 204
09:38:45.853 in Ping 
09:38:46.023 rpc 35 server.reportClientActivity
09:38:46.269 chunk 17 40714 bytes
09:38:46.273 in Ack 17
09:38:46.520 chunk 17 40944 bytes
09:38:46.522 in Ack 17
09:38:47.046 fetch POST /api/observability/v1/traces
09:38:47.046 fetched POST /api/observability/v1/traces 204
09:38:48.046 chunk 17 41946 bytes
09:38:48.048 in Ack 17
09:38:48.367 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:49.050 fetch POST /api/observability/v1/traces
09:38:49.050 fetched POST /api/observability/v1/traces 204
09:38:49.707 chunk 17 45433 bytes
09:38:49.709 in Ack 17
09:38:50.710 chunk 17 47501 bytes
09:38:50.711 fetch POST /api/observability/v1/traces
09:38:50.711 fetched POST /api/observability/v1/traces 204
09:38:50.713 in Ack 17
09:38:50.857 in Ping 
09:38:51.458 exit 36 {"_tag":"Success","value":{"sequence":185}}
09:38:51.717 fetch POST /api/observability/v1/traces
09:38:51.717 fetched POST /api/observability/v1/traces 204
09:38:52.366 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:53.133 chunk 17 47596 bytes
09:38:53.135 in Ack 17
09:38:53.224 fetch POST /api/observability/v1/traces
09:38:53.224 fetched POST /api/observability/v1/traces 204
09:38:53.957 chunk 17 47598 bytes
09:38:53.959 in Ack 17
09:38:54.960 fetch POST /api/observability/v1/traces
09:38:54.960 fetched POST /api/observability/v1/traces 204
09:38:55.737 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:55.861 in Ping 
09:38:56.741 fetch POST /api/observability/v1/traces
09:38:56.741 fetched POST /api/observability/v1/traces 204
09:39:00.866 in Ping 
09:39:01.024 in Interrupt 35
09:39:02.026 fetch POST /api/observability/v1/traces
09:39:02.026 fetched POST /api/observability/v1/traces 204
09:39:03.170 fetch POST /api/observability/v1/traces
09:39:03.170 fetched POST /api/observability/v1/traces 204
09:39:05.869 in Ping 
09:39:10.874 in Ping 
09:39:11.021 rpc 38 server.reportClientActivity
09:39:11.384 chunk 17 50518 bytes
09:39:11.388 in Ack 17
09:39:11.634 chunk 17 51024 bytes
09:39:11.637 in Ack 17
09:39:11.885 chunk 17 51512 bytes
09:39:11.889 in Ack 17
09:39:12.023 fetch POST /api/observability/v1/traces
09:39:12.023 fetched POST /api/observability/v1/traces 204
09:39:12.650 exit 32 {"_tag":"Success","value":null}
09:39:12.733 chunk 6 4723 bytes
09:39:12.734 in Ack 6
09:39:12.739 rpc 39 orchestration.dispatchCommand thread.visit
09:39:12.739 exit 39 {"_tag":"Success","value":{"sequence":0}}
09:39:12.891 chunk 17 51514 bytes
09:39:12.894 in Ack 17
09:39:13.444 fetch POST /api/observability/v1/traces
09:39:13.445 fetched POST /api/observability/v1/traces 204
