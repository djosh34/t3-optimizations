# story, webapp, warm start, seed 24

Profile story (graded): seed 24's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 59 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.8s |
| Prompt reached the agent | 7.3s |
| Time to first text | 18.8s |
| Whole reply shown | 56.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.6s / 6.3s / 15.4s |
| Longest silence | 18.8s |
| Approval shown after it was asked | 3.1s |
| Bytes on the wire down / up | 175059 / 113335 |
| Bytes the app received / sent | 88315 / 74133 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.6 | 0 | 109 |
| 20 | 2.6 | 397 | 510 |
| 30 | 12.6 | 397 | 914 |
| 40 | 1.3 | 1077 | 1132 |
| 50 | 11.3 | 1077 | 1505 |
| 56 | 15.4 | 1154 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 424/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809","messages":"544/544 456/456 119/901","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217062195,"silences":0,"longestSilenceMs":1346,"longestRecoveredGapMs":1346},"stats":{"sendRate":4500,"deliveredRate":1459.7911717637212,"loss":0,"srttMs":755.311835756143,"minRttMs":2,"sentPackets":133,"sentWireBytes":39136,"receivedPackets":175,"receivedWireBytes":105114,"lastHeardMs":18450,"unsyncedBytes":518,"inflightPackets":16,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9 73/73","attempts":"659/659 659/659 122/583","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 105/1016","messages":"544/544 456/456 206/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217071185,"silences":1,"longestSilenceMs":3619,"longestRecoveredGapMs":3619},"stats":{"sendRate":1999.5362821854537,"deliveredRate":1792.8569410177392,"loss":0.10336363636363638,"srttMs":2232.8643500607836,"minRttMs":1804,"sentPackets":180,"sentWireBytes":49851,"receivedPackets":215,"receivedWireBytes":122388,"lastHeardMs":27440,"unsyncedBytes":0,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 304/818","assistant":"32/32 21/21 498/498","user":"10/10 9/9 73/73","attempts":"659/659 659/659 122/583","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 195/1016","messages":"544/544 456/456 206/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217082375,"silences":2,"longestSilenceMs":3619,"longestRecoveredGapMs":3619},"stats":{"sendRate":2999.3044232781804,"deliveredRate":129.44839857651243,"loss":0.9285714285714286,"srttMs":2253.3333262929546,"minRttMs":1804,"sentPackets":226,"sentWireBytes":61757,"receivedPackets":260,"receivedWireBytes":137359,"lastHeardMs":38631,"unsyncedBytes":38,"inflightPackets":34,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","runs":"1014/1014 971/971 779/839","assistant":"32/32 21/21 498/498","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 301/1016","messages":"544/544 456/456 206/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217092449,"silences":3,"longestSilenceMs":3619,"longestRecoveredGapMs":3619},"stats":{"sendRate":1505.4550653972362,"deliveredRate":884.8485696764527,"loss":0.5251338235294116,"srttMs":3573.142030562327,"minRttMs":1804,"sentPackets":317,"sentWireBytes":79326,"receivedPackets":323,"receivedWireBytes":158743,"lastHeardMs":48705,"unsyncedBytes":0,"inflightPackets":62,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1214/1542","frame":"1029/1029","thread":"736/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 301/1016","messages":"544/544 456/456 206/901 544/544 254/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"742/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217103037,"silences":3,"longestSilenceMs":3619,"longestRecoveredGapMs":3619},"stats":{"sendRate":2873.756320857935,"deliveredRate":1512.7496814758608,"loss":0.21039772727272724,"srttMs":2311.1012190950037,"minRttMs":1804,"sentPackets":393,"sentWireBytes":100241,"receivedPackets":361,"receivedWireBytes":174751,"lastHeardMs":59293,"unsyncedBytes":398,"inflightPackets":78,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1214/1542 421/1068","frame":"1029/1029","thread":"815/961","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 122/409","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 301/1016","messages":"544/544 456/456 206/901 544/544 254/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":296,"lastDataAt":1794217114142,"silences":4,"longestSilenceMs":5568,"longestRecoveredGapMs":5568},"stats":{"sendRate":3080.1590595843923,"deliveredRate":1237.3380452855238,"loss":0.23601649431818178,"srttMs":2247.6342785847023,"minRttMs":1804,"sentPackets":420,"sentWireBytes":105161,"receivedPackets":374,"receivedWireBytes":180380,"lastHeardMs":70398,"unsyncedBytes":0,"inflightPackets":67,"duplicateBytes":0}}
- shim trace
09:37:24.764 fetch GET /api/auth/session
09:37:24.766 fetched GET /api/auth/session 200
09:37:25.141 fetch GET /.well-known/t3/environment
09:37:25.143 fetched GET /.well-known/t3/environment 200
09:37:25.166 fetch GET /.well-known/t3/environment
09:37:25.168 fetched GET /.well-known/t3/environment 200
09:37:25.179 rpc 0 subscribeServerConfig
09:37:25.180 chunk 0 38462 bytes
09:37:25.264 in Ack 0
09:37:25.271 rpc 1 subscribeServerLifecycle
09:37:25.271 rpc 2 subscribeServerLifecycle
09:37:25.276 fetch GET /api/auth/session
09:37:25.280 fetched GET /api/auth/session 200
09:37:25.281 rpc 3 subscribeDeviceState
09:37:25.284 rpc 4 preview.reportProfiles
09:37:25.388 rpc 5 subscribeProjectClones
09:37:25.440 rpc 6 server.reportClientActivity
09:37:25.448 fetch GET /api/orchestration/threads/mcp%3A6abb5729-572f-4af3-b908-1b45530c771a/bounded
09:37:25.461 fetch GET /api/orchestration/shell
09:37:25.590 rpc 7 server.refreshProviders
09:37:25.793 rpc 8 orchestration.dispatchCommand thread.visit
09:37:25.793 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:25.824 fetched GET /api/orchestration/shell 200
09:37:25.825 rpc 9 subscribeTerminalMetadata
09:37:25.826 rpc 10 subscribeVcsStatus
09:37:25.827 rpc 11 subscribePreviewEvents
09:37:25.832 rpc 12 orchestration.subscribeShell
09:37:25.832 chunk 12 4551 bytes
09:37:25.834 rpc 13 assets.createUrl
09:37:25.835 rpc 14 preview.list
09:37:25.837 rpc 15 projects.readFile
09:37:25.860 in Ack 12
09:37:26.143 chunk 12 4527 bytes
09:37:26.146 fetch POST /api/observability/v1/traces
09:37:26.147 fetched POST /api/observability/v1/traces 204
09:37:26.150 in Ack 12
09:37:26.252 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"02340ec8-2bfe-4ac7-a9d5-09ef1dc119f4","sessions":[]}}
09:37:26.252 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:26.491 chunk 1 3815 bytes
09:37:26.492 in Ack 1
09:37:26.732 chunk 2 3815 bytes
09:37:26.733 in Ack 2
09:37:27.012 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:54.544Z","configurableProviders":false,"con
09:37:27.012 chunk 10 1632 bytes
09:37:27.012 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:27.027 in Ack 10
09:37:27.157 fetch POST /api/observability/v1/traces
09:37:27.157 fetched POST /api/observability/v1/traces 204
09:37:27.416 chunk 12 4552 bytes
09:37:27.417 in Ack 12
09:37:27.667 chunk 12 4595 bytes
09:37:27.669 in Ack 12
09:37:27.775 chunk 3 876 bytes
09:37:27.775 exit 4 {"_tag":"Success","value":null}
09:37:27.775 chunk 5 4 bytes
09:37:27.775 exit 6 {"_tag":"Success","value":null}
09:37:27.776 in Ack 3
09:37:27.782 rpc 16 preview.reportProfiles
09:37:27.783 in Ack 5
09:37:27.851 chunk 9 36 bytes
09:37:27.851 exit 16 {"_tag":"Success","value":null}
09:37:27.851 in Ack 9
09:37:28.035 rpc 17 server.reportClientActivity
09:37:28.092 exit 17 {"_tag":"Success","value":null}
09:37:28.421 fetch POST /api/observability/v1/traces
09:37:28.421 fetched POST /api/observability/v1/traces 204
09:37:28.671 chunk 12 4592 bytes
09:37:28.672 in Ack 12
09:37:28.922 chunk 12 4592 bytes
09:37:28.923 in Ack 12
09:37:29.673 fetch POST /api/observability/v1/traces
09:37:29.673 fetched POST /api/observability/v1/traces 204
09:37:30.175 in Ping 
09:37:30.403 fetched GET /api/orchestration/threads/mcp%3A6abb5729-572f-4af3-b908-1b45530c771a/bounded 200
09:37:30.408 rpc 18 orchestration.subscribeThread
09:37:30.409 chunk 18 8282 bytes
09:37:30.426 in Interrupt 10
09:37:30.429 rpc 19 subscribeVcsStatus
09:37:30.431 in Ack 18
09:37:30.625 chunk 19 1632 bytes
09:37:30.625 in Ack 19
09:37:30.661 chunk 18 11918 bytes
09:37:30.665 in Ack 18
09:37:30.742 rpc 20 server.reportClientActivity
09:37:30.919 chunk 18 13497 bytes
09:37:30.921 in Ack 18
09:37:31.182 chunk 18 16150 bytes
09:37:31.184 in Ack 18
09:37:31.405 fetch POST /api/observability/v1/traces
09:37:31.405 fetched POST /api/observability/v1/traces 204
09:37:31.432 chunk 18 18552 bytes
09:37:31.433 in Ack 18
09:37:31.497 exit 20 {"_tag":"Success","value":null}
09:37:31.683 rpc 21 attachments.createUploadUrl
09:37:31.758 chunk 18 19796 bytes
09:37:31.773 in Ack 18
09:37:31.786 in Interrupt 19
09:37:31.789 rpc 22 subscribeVcsStatus
09:37:31.876 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-69ff2a64-9a62-4299-93b1-b7624f99759e","expiresAt":1794217651818,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:31.876 chunk 22 1632 bytes
09:37:31.879 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTY5ZmYyYTY0LTlhNjItNDI5OS05M2IxLWI3NjI0Zjk5NzU5ZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1MTgxOH0.KqBq5iPWuiANMJ-hD6AWpA8FTPUbfoYFqtxh5VU7O-8
09:37:31.880 in Ack 22
09:37:32.009 chunk 18 21099 bytes
09:37:32.011 in Ack 18
09:37:32.040 rpc 23 server.reportClientActivity
09:37:32.438 fetch POST /api/observability/v1/traces
09:37:32.438 fetched POST /api/observability/v1/traces 204
09:37:33.097 exit 23 {"_tag":"Success","value":null}
09:37:34.129 fetch POST /api/observability/v1/traces
09:37:34.129 fetched POST /api/observability/v1/traces 204
09:37:35.328 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:35.765 in Ping 
09:37:35.806 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:35.809 rpc 25 attachments.delete
09:37:36.326 fetch POST /api/observability/v1/traces
09:37:36.326 fetched POST /api/observability/v1/traces 204
09:37:36.892 chunk 18 23554 bytes
09:37:36.894 in Ack 18
09:37:37.045 exit 25 {"_tag":"Success","value":null}
09:37:37.145 chunk 18 23556 bytes
09:37:37.147 in Ack 18
09:37:37.348 fetch POST /api/observability/v1/traces
09:37:37.348 fetched POST /api/observability/v1/traces 204
09:37:37.393 chunk 12 4617 bytes
09:37:37.394 in Ack 12
09:37:37.396 rpc 26 orchestration.dispatchCommand thread.visit
09:37:37.396 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:37.767 chunk 18 24396 bytes
09:37:37.768 in Ack 18
09:37:37.953 chunk 12 4617 bytes
09:37:37.956 in Ack 12
09:37:38.017 chunk 18 25056 bytes
09:37:38.021 in Ack 18
09:37:38.270 chunk 18 27849 bytes
09:37:38.272 in Ack 18
09:37:38.276 rpc 27 assets.createUrl
09:37:38.396 fetch POST /api/observability/v1/traces
09:37:38.396 fetched POST /api/observability/v1/traces 204
09:37:38.707 chunk 12 4614 bytes
09:37:38.708 in Ack 12
09:37:38.710 rpc 28 orchestration.dispatchCommand thread.visit
09:37:38.710 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:37:38.771 chunk 18 27981 bytes
09:37:38.772 in Ack 18
09:37:38.961 chunk 12 4614 bytes
09:37:38.962 in Ack 12
09:37:39.021 chunk 18 27981 bytes
09:37:39.023 in Ack 18
09:37:39.578 chunk 18 31214 bytes
09:37:39.580 in Ack 18
09:37:39.710 fetch POST /api/observability/v1/traces
09:37:39.710 fetched POST /api/observability/v1/traces 204
09:37:39.787 rpc 29 server.refreshProviders
09:37:39.787 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:54.544Z","configurableProviders":false,"con
09:37:40.926 in Ping 
09:37:40.931 fetch POST /api/observability/v1/traces
09:37:40.931 fetched POST /api/observability/v1/traces 204
09:37:42.106 rpc 30 orchestration.dispatchCommand message.dispatch
09:37:42.194 chunk 18 31992 bytes
09:37:42.201 fetch POST /api/observability/v1/traces
09:37:42.201 fetched POST /api/observability/v1/traces 204
09:37:42.220 in Ack 18
09:37:42.259 in Interrupt 22
09:37:42.267 rpc 31 subscribeVcsStatus
09:37:42.520 rpc 32 server.reportClientActivity
09:37:43.221 fetch POST /api/observability/v1/traces
09:37:43.221 fetched POST /api/observability/v1/traces 204
09:37:43.233 chunk 18 32765 bytes
09:37:43.235 in Ack 18
09:37:44.237 fetch POST /api/observability/v1/traces
09:37:44.237 fetched POST /api/observability/v1/traces 204
09:37:44.446 exit 30 {"_tag":"Success","value":{"sequence":152}}
09:37:44.446 chunk 31 1713 bytes
09:37:44.447 in Ack 31
09:37:44.492 chunk 18 33331 bytes
09:37:44.494 in Ack 18
09:37:45.449 fetch POST /api/observability/v1/traces
09:37:45.449 fetched POST /api/observability/v1/traces 204
09:37:45.928 in Ping 
09:37:46.071 chunk 18 35352 bytes
09:37:46.109 in Ack 18
09:37:47.110 fetch POST /api/observability/v1/traces
09:37:47.110 fetched POST /api/observability/v1/traces 204
09:37:50.928 in Ping 
09:37:52.142 fetch POST /api/observability/v1/traces
09:37:52.142 fetched POST /api/observability/v1/traces 204
09:37:55.929 in Ping 
09:37:57.525 in Interrupt 32
09:37:57.779 rpc 33 server.reportClientActivity
09:37:58.528 fetch POST /api/observability/v1/traces
09:37:58.528 fetched POST /api/observability/v1/traces 204
09:38:00.160 chunk 18 38437 bytes
09:38:00.165 in Ack 18
09:38:00.532 chunk 18 38765 bytes
09:38:00.535 in Ack 18
09:38:00.933 in Ping 
09:38:01.167 fetch POST /api/observability/v1/traces
09:38:01.167 fetched POST /api/observability/v1/traces 204
09:38:01.877 chunk 18 38717 bytes
09:38:01.879 in Ack 18
09:38:02.880 fetch POST /api/observability/v1/traces
09:38:02.880 fetched POST /api/observability/v1/traces 204
09:38:05.935 in Ping 
09:38:09.132 chunk 12 4678 bytes
09:38:09.133 in Ack 12
09:38:09.138 rpc 34 orchestration.dispatchCommand thread.visit
09:38:09.138 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:38:09.384 chunk 18 39353 bytes
09:38:09.386 in Ack 18
09:38:10.135 fetch POST /api/observability/v1/traces
09:38:10.135 fetched POST /api/observability/v1/traces 204
09:38:10.936 in Ping 
09:38:12.164 fetch POST /api/observability/v1/traces
09:38:12.164 fetched POST /api/observability/v1/traces 204
09:38:12.744 chunk 18 40193 bytes
09:38:12.746 in Ack 18
09:38:12.929 in Interrupt 33
09:38:13.748 fetch POST /api/observability/v1/traces
09:38:13.748 fetched POST /api/observability/v1/traces 204
09:38:15.412 rpc 35 server.reportClientActivity
09:38:15.938 in Ping 
09:38:16.213 chunk 12 4678 bytes
09:38:16.215 in Ack 12
09:38:16.414 fetch POST /api/observability/v1/traces
09:38:16.414 fetched POST /api/observability/v1/traces 204
09:38:16.974 chunk 18 40619 bytes
09:38:16.984 in Ack 18
09:38:17.236 chunk 18 41079 bytes
09:38:17.239 in Ack 18
09:38:17.486 chunk 18 41881 bytes
09:38:17.493 in Ack 18
09:38:18.370 fetch POST /api/observability/v1/traces
09:38:18.370 fetched POST /api/observability/v1/traces 204
09:38:18.866 chunk 18 42883 bytes
09:38:18.870 in Ack 18
09:38:19.303 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:19.872 fetch POST /api/observability/v1/traces
09:38:19.872 fetched POST /api/observability/v1/traces 204
09:38:20.941 in Ping 
09:38:21.500 exit 36 {"_tag":"Success","value":{"sequence":187}}
09:38:21.880 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:22.502 fetch POST /api/observability/v1/traces
09:38:22.503 fetched POST /api/observability/v1/traces 204
09:38:24.087 chunk 18 42926 bytes
09:38:24.087 exit 37 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:24.090 in Ack 18
09:38:25.091 fetch POST /api/observability/v1/traces
09:38:25.091 fetched POST /api/observability/v1/traces 204
09:38:25.391 chunk 18 42976 bytes
09:38:25.394 in Ack 18
09:38:25.945 in Ping 
09:38:26.396 fetch POST /api/observability/v1/traces
09:38:26.396 fetched POST /api/observability/v1/traces 204
09:38:30.413 in Interrupt 35
09:38:30.949 in Ping 
09:38:31.414 fetch POST /api/observability/v1/traces
09:38:31.414 fetched POST /api/observability/v1/traces 204
09:38:33.284 chunk 18 45635 bytes
09:38:33.287 in Ack 18
09:38:34.290 fetch POST /api/observability/v1/traces
09:38:34.290 fetched POST /api/observability/v1/traces 204
09:38:35.512 chunk 18 45985 bytes
09:38:35.515 in Ack 18
09:38:35.762 chunk 18 46011 bytes
09:38:35.769 in Ack 18
09:38:35.953 in Ping 
09:38:36.138 chunk 18 46323 bytes
09:38:36.141 in Ack 18
09:38:36.564 chunk 18 46353 bytes
09:38:36.567 fetch POST /api/observability/v1/traces
09:38:36.567 fetched POST /api/observability/v1/traces 204
09:38:36.635 in Ack 18
09:38:37.113 chunk 18 46757 bytes
09:38:37.122 in Ack 18
09:38:37.645 fetch POST /api/observability/v1/traces
09:38:37.645 fetched POST /api/observability/v1/traces 204
09:38:38.092 chunk 18 46889 bytes
09:38:38.095 in Ack 18
