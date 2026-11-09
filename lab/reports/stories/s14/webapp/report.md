# story, webapp, warm start, seed 14

Profile story (graded): seed 14's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 76 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.9s |
| Prompt reached the agent | 5.8s |
| Time to first text | 18.1s |
| Whole reply shown | 73.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 35.3s / 10.3s / 31.3s |
| Longest silence | 38.6s |
| Approval shown after it was asked | 3.3s |
| Bytes on the wire down / up | 333290 / 94334 |
| Bytes the app received / sent | 177477 / 74062 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.1 | 0 | 165 |
| 20 | 9.3 | 193 | 566 |
| 30 | 14.1 | 397 | 969 |
| 40 | 2.3 | 1077 | 1169 |
| 50 | 12.3 | 1077 | 1538 |
| 60 | 22.3 | 1077 | 1596 |
| 70 | 32.3 | 1077 | 1596 |
| 73 | 35.3 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 544/544 901/901","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217058579,"silences":0,"longestSilenceMs":1254,"longestRecoveredGapMs":1254},"stats":{"sendRate":3796.875,"deliveredRate":3945.906217290054,"loss":0,"srttMs":63.308132346158274,"minRttMs":null,"sentPackets":133,"sentWireBytes":40129,"receivedPackets":126,"receivedWireBytes":97261,"lastHeardMs":15060,"unsyncedBytes":736,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 723/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 482/635","providerTurns":"772/772 772/772","providerThreads":"807/807","messages":"456/456 544/544 544/544 901/901","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217068649,"silences":0,"longestSilenceMs":1254,"longestRecoveredGapMs":1254},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.22276315789473686,"srttMs":2043.2991876325036,"minRttMs":2592,"sentPackets":175,"sentWireBytes":52759,"receivedPackets":214,"receivedWireBytes":131280,"lastHeardMs":25129,"unsyncedBytes":167,"inflightPackets":21,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 723/1068","thread":"961/961","assistant":"32/32 21/21 433/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 544/544 901/901 451/451","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"585/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217078595,"silences":0,"longestSilenceMs":1254,"longestRecoveredGapMs":1254},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.402390909090909,"srttMs":2637.8904157017832,"minRttMs":2345,"sentPackets":243,"sentWireBytes":68795,"receivedPackets":312,"receivedWireBytes":170322,"lastHeardMs":35076,"unsyncedBytes":167,"inflightPackets":41,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 544/544 901/901 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"671/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217088838,"silences":0,"longestSilenceMs":1254,"longestRecoveredGapMs":1254},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.8343749999999999,"srttMs":2786.34406166949,"minRttMs":2345,"sentPackets":305,"sentWireBytes":83084,"receivedPackets":415,"receivedWireBytes":212129,"lastHeardMs":45318,"unsyncedBytes":167,"inflightPackets":71,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 258/1093 1542/1542 923/923","thread":"607/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 544/544 901/901 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217096573,"silences":0,"longestSilenceMs":1254,"longestRecoveredGapMs":1254},"stats":{"sendRate":1084.5232333010645,"deliveredRate":0,"loss":0.7829999999999999,"srttMs":3039.862098970164,"minRttMs":2345,"sentPackets":350,"sentWireBytes":97107,"receivedPackets":482,"receivedWireBytes":248926,"lastHeardMs":53053,"unsyncedBytes":855,"inflightPackets":95,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 359/1093 277/1566 923/923 531/1068","thread":"881/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 544/544 901/901 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217109828,"silences":2,"longestSilenceMs":6727,"longestRecoveredGapMs":6727},"stats":{"sendRate":813.3924249757983,"deliveredRate":172.19270928741528,"loss":0.5333333333333333,"srttMs":4429.941294524032,"minRttMs":2345,"sentPackets":362,"sentWireBytes":103451,"receivedPackets":498,"receivedWireBytes":258208,"lastHeardMs":66309,"unsyncedBytes":686,"inflightPackets":103,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"860/860 1058/1058 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 359/1093 277/1566 923/923 531/1068","thread":"881/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","messages":"456/456 544/544 544/544 901/901 451/451 543/543","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":668,"lastDataAt":1794217109828,"silences":2,"longestSilenceMs":6727,"longestRecoveredGapMs":6727},"stats":{"sendRate":813.3924249757983,"deliveredRate":172.19270928741528,"loss":0.5333333333333333,"srttMs":4429.941294524032,"minRttMs":2345,"sentPackets":362,"sentWireBytes":103451,"receivedPackets":498,"receivedWireBytes":258208,"lastHeardMs":66309,"unsyncedBytes":1143,"inflightPackets":103,"duplicateBytes":0}}
- shim trace
09:37:24.523 fetch GET /api/auth/session
09:37:24.526 fetched GET /api/auth/session 200
09:37:25.452 fetch GET /.well-known/t3/environment
09:37:25.454 fetched GET /.well-known/t3/environment 200
09:37:25.471 fetch GET /.well-known/t3/environment
09:37:25.473 fetched GET /.well-known/t3/environment 200
09:37:25.602 rpc 0 subscribeServerConfig
09:37:25.632 chunk 0 38462 bytes
09:37:25.710 in Ack 0
09:37:25.713 rpc 1 subscribeServerLifecycle
09:37:25.714 rpc 2 subscribeServerLifecycle
09:37:25.719 fetch GET /api/auth/session
09:37:25.721 fetched GET /api/auth/session 200
09:37:25.721 rpc 3 subscribeDeviceState
09:37:25.724 rpc 4 preview.reportProfiles
09:37:25.753 rpc 5 server.reportClientActivity
09:37:25.832 rpc 6 subscribeProjectClones
09:37:25.883 fetch GET /api/orchestration/shell
09:37:25.885 fetched GET /api/orchestration/shell 200
09:37:25.888 rpc 7 orchestration.subscribeShell
09:37:25.891 chunk 7 4551 bytes
09:37:25.949 rpc 8 server.refreshProviders
09:37:26.064 rpc 9 orchestration.dispatchCommand thread.visit
09:37:26.064 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:26.166 rpc 10 subscribeTerminalMetadata
09:37:26.204 rpc 11 subscribeVcsStatus
09:37:26.216 rpc 12 subscribePreviewEvents
09:37:26.283 rpc 13 assets.createUrl
09:37:26.284 rpc 14 preview.list
09:37:26.285 rpc 15 projects.readFile
09:37:26.330 fetch GET /api/orchestration/threads/mcp%3A381cae5b-82fd-4c4d-9ba1-77899e3ae2b2/bounded
09:37:26.340 in Ack 7
09:37:26.717 chunk 7 4527 bytes
09:37:26.724 fetch POST /api/observability/v1/traces
09:37:26.724 fetched POST /api/observability/v1/traces 204
09:37:26.726 in Ack 7
09:37:27.034 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"58c6a964-af84-427d-afd8-fb6b05fe47ee","sessions":[]}}
09:37:27.034 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:27.185 chunk 1 3815 bytes
09:37:27.186 in Ack 1
09:37:27.462 chunk 2 3815 bytes
09:37:27.463 in Ack 2
09:37:27.660 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:57.298Z","configurableProviders":false,"con
09:37:27.660 chunk 11 1632 bytes
09:37:27.660 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:27.665 in Ack 11
09:37:27.726 fetch POST /api/observability/v1/traces
09:37:27.726 fetched POST /api/observability/v1/traces 204
09:37:28.110 chunk 7 4549 bytes
09:37:28.111 in Ack 7
09:37:28.360 chunk 7 4592 bytes
09:37:28.361 in Ack 7
09:37:28.526 chunk 3 876 bytes
09:37:28.526 exit 4 {"_tag":"Success","value":null}
09:37:28.526 exit 5 {"_tag":"Success","value":null}
09:37:28.526 chunk 6 4 bytes
09:37:28.527 in Ack 3
09:37:28.535 rpc 16 preview.reportProfiles
09:37:28.537 in Ack 6
09:37:28.605 chunk 10 36 bytes
09:37:28.605 exit 16 {"_tag":"Success","value":null}
09:37:28.605 in Ack 10
09:37:28.788 rpc 17 server.reportClientActivity
09:37:28.842 exit 17 {"_tag":"Success","value":null}
09:37:29.112 fetch POST /api/observability/v1/traces
09:37:29.113 fetched POST /api/observability/v1/traces 204
09:37:30.597 in Ping 
09:37:30.729 fetched GET /api/orchestration/threads/mcp%3A381cae5b-82fd-4c4d-9ba1-77899e3ae2b2/bounded 200
09:37:30.735 rpc 18 orchestration.subscribeThread
09:37:30.735 chunk 18 8282 bytes
09:37:30.754 in Interrupt 11
09:37:30.756 rpc 19 subscribeVcsStatus
09:37:30.816 in Ack 18
09:37:30.894 chunk 19 1632 bytes
09:37:30.895 in Ack 19
09:37:30.986 chunk 18 8322 bytes
09:37:30.987 in Ack 18
09:37:31.049 rpc 20 server.reportClientActivity
09:37:31.236 chunk 18 11918 bytes
09:37:31.243 in Ack 18
09:37:31.486 chunk 18 13497 bytes
09:37:31.488 in Ack 18
09:37:31.732 fetch POST /api/observability/v1/traces
09:37:31.732 fetched POST /api/observability/v1/traces 204
09:37:31.738 chunk 18 16150 bytes
09:37:31.740 in Ack 18
09:37:32.009 chunk 18 18442 bytes
09:37:32.026 rpc 21 attachments.createUploadUrl
09:37:32.092 exit 20 {"_tag":"Success","value":null}
09:37:32.134 in Ack 18
09:37:32.238 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-45ddc8b9-f5cb-428c-874b-e3ab30897a83","expiresAt":1794217652209,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:32.239 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTQ1ZGRjOGI5LWY1Y2ItNDI4Yy04NzRiLWUzYWIzMDg5N2E4MyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1MjIwOX0.dROHYhT1vNIIa3ixvoQsrg5sMHq5vwiAY2t4M_RpY58
09:37:32.266 chunk 18 20408 bytes
09:37:32.268 in Ack 18
09:37:32.280 in Interrupt 19
09:37:32.283 rpc 22 subscribeVcsStatus
09:37:32.318 chunk 22 1632 bytes
09:37:32.318 in Ack 22
09:37:32.516 chunk 18 21099 bytes
09:37:32.518 in Ack 18
09:37:32.535 rpc 23 server.reportClientActivity
09:37:32.741 fetch POST /api/observability/v1/traces
09:37:32.741 fetched POST /api/observability/v1/traces 204
09:37:33.456 exit 23 {"_tag":"Success","value":null}
09:37:33.679 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:33.775 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:33.777 rpc 25 attachments.delete
09:37:33.963 chunk 7 4678 bytes
09:37:33.965 in Ack 7
09:37:34.184 chunk 18 25368 bytes
09:37:34.187 in Ack 18
09:37:34.190 rpc 26 assets.createUrl
09:37:34.277 exit 25 {"_tag":"Success","value":null}
09:37:34.986 chunk 18 29266 bytes
09:37:34.987 fetch POST /api/observability/v1/traces
09:37:34.987 fetched POST /api/observability/v1/traces 204
09:37:34.989 chunk 7 4617 bytes
09:37:34.992 in Ack 18
09:37:35.021 in Ack 7
09:37:35.237 chunk 18 29398 bytes
09:37:35.239 in Ack 18
09:37:35.316 chunk 7 4614 bytes
09:37:35.317 in Ack 7
09:37:35.319 rpc 27 orchestration.dispatchCommand thread.visit
09:37:35.319 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:35.486 chunk 18 29422 bytes
09:37:35.488 in Ack 18
09:37:35.566 chunk 7 4614 bytes
09:37:35.567 in Ack 7
09:37:35.597 in Ping 
09:37:35.736 chunk 18 32647 bytes
09:37:35.738 in Ack 18
09:37:35.747 in Interrupt 22
09:37:35.750 rpc 28 subscribeVcsStatus
09:37:36.202 chunk 18 32765 bytes
09:37:36.203 chunk 28 1632 bytes
09:37:36.205 fetch POST /api/observability/v1/traces
09:37:36.205 fetched POST /api/observability/v1/traces 204
09:37:36.207 rpc 29 server.reportClientActivity
09:37:36.209 in Ack 18
09:37:36.224 in Ack 28
09:37:36.509 chunk 18 35259 bytes
09:37:36.541 in Ack 18
09:37:37.209 fetch POST /api/observability/v1/traces
09:37:37.209 fetched POST /api/observability/v1/traces 204
09:37:37.502 chunk 18 35259 bytes
09:37:37.503 exit 29 {"_tag":"Success","value":null}
09:37:37.513 in Ack 18
09:37:37.692 rpc 30 server.refreshProviders
09:37:37.692 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:57.298Z","configurableProviders":false,"con
09:37:38.505 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:38.584 fetch POST /api/observability/v1/traces
09:37:38.584 fetched POST /api/observability/v1/traces 204
09:37:40.597 in Ping 
09:37:42.353 fetch POST /api/observability/v1/traces
09:37:42.353 fetched POST /api/observability/v1/traces 204
09:37:43.860 chunk 7 4678 bytes
09:37:43.861 in Ack 7
09:37:44.863 fetch POST /api/observability/v1/traces
09:37:44.863 fetched POST /api/observability/v1/traces 204
09:37:45.049 exit 31 {"_tag":"Success","value":{"sequence":151}}
09:37:45.599 in Ping 
09:37:46.055 fetch POST /api/observability/v1/traces
09:37:46.055 fetched POST /api/observability/v1/traces 204
09:37:47.749 fetch POST /api/observability/v1/traces
09:37:47.749 fetched POST /api/observability/v1/traces 204
09:37:48.311 chunk 7 4742 bytes
09:37:48.312 in Ack 7
09:37:48.314 rpc 32 orchestration.dispatchCommand thread.visit
09:37:48.314 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:37:48.812 chunk 18 37280 bytes
09:37:48.814 in Ack 18
09:37:49.346 chunk 18 37916 bytes
09:37:49.347 fetch POST /api/observability/v1/traces
09:37:49.347 fetched POST /api/observability/v1/traces 204
09:37:49.353 in Ack 18
09:37:49.596 chunk 18 38571 bytes
09:37:49.604 in Ack 18
09:37:50.356 fetch POST /api/observability/v1/traces
09:37:50.357 fetched POST /api/observability/v1/traces 204
09:37:50.600 in Ping 
09:37:50.705 rpc 33 server.reportClientActivity
09:37:51.705 fetch POST /api/observability/v1/traces
09:37:51.705 fetched POST /api/observability/v1/traces 204
09:37:51.850 chunk 7 4742 bytes
09:37:51.851 in Ack 7
09:37:52.110 chunk 18 39411 bytes
09:37:52.112 in Ack 18
09:37:52.360 chunk 18 39936 bytes
09:37:52.363 in Ack 18
09:37:52.852 fetch POST /api/observability/v1/traces
09:37:52.852 fetched POST /api/observability/v1/traces 204
09:37:53.110 chunk 18 40722 bytes
09:37:53.114 in Ack 18
09:37:53.983 chunk 18 40724 bytes
09:37:53.985 in Ack 18
09:37:54.115 fetch POST /api/observability/v1/traces
09:37:54.115 fetched POST /api/observability/v1/traces 204
09:37:55.282 exit 33 {"_tag":"Success","value":null}
09:37:55.600 in Ping 
09:37:55.745 chunk 18 43423 bytes
09:37:55.748 in Ack 18
09:37:55.997 chunk 18 43799 bytes
09:37:55.999 in Ack 18
09:37:56.248 chunk 18 43993 bytes
09:37:56.253 in Ack 18
09:37:56.286 fetch POST /api/observability/v1/traces
09:37:56.286 fetched POST /api/observability/v1/traces 204
09:37:56.747 chunk 18 44009 bytes
09:37:56.754 in Ack 18
09:37:57.755 fetch POST /api/observability/v1/traces
09:37:57.755 fetched POST /api/observability/v1/traces 204
09:37:58.319 rpc 34 orchestration.dispatchCommand thread.visit
09:37:58.319 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:37:59.320 fetch POST /api/observability/v1/traces
09:37:59.320 fetched POST /api/observability/v1/traces 204
09:37:59.510 chunk 18 44137 bytes
09:37:59.512 in Ack 18
09:38:00.514 fetch POST /api/observability/v1/traces
09:38:00.514 fetched POST /api/observability/v1/traces 204
09:38:00.601 in Ping 
09:38:00.761 chunk 18 44089 bytes
09:38:00.766 in Ack 18
09:38:01.519 chunk 7 4742 bytes
09:38:01.521 in Ack 7
09:38:01.768 fetch POST /api/observability/v1/traces
09:38:01.768 fetched POST /api/observability/v1/traces 204
09:38:02.275 chunk 7 4742 bytes
09:38:02.278 in Ack 7
09:38:03.282 fetch POST /api/observability/v1/traces
09:38:03.282 fetched POST /api/observability/v1/traces 204
09:38:05.601 in Ping 
09:38:07.810 fetch POST /api/observability/v1/traces
09:38:07.810 fetched POST /api/observability/v1/traces 204
09:38:08.321 rpc 35 orchestration.dispatchCommand thread.visit
09:38:08.321 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:08.524 chunk 18 44089 bytes
09:38:08.531 in Ack 18
09:38:09.029 chunk 18 45133 bytes
09:38:09.032 in Ack 18
09:38:09.323 fetch POST /api/observability/v1/traces
09:38:09.323 fetched POST /api/observability/v1/traces 204
09:38:10.606 in Ping 
09:38:12.815 chunk 18 46252 bytes
09:38:12.834 in Ack 18
09:38:13.067 chunk 18 47665 bytes
09:38:13.070 in Ack 18
09:38:13.844 fetch POST /api/observability/v1/traces
09:38:13.844 fetched POST /api/observability/v1/traces 204
09:38:13.977 chunk 18 48667 bytes
09:38:13.985 in Ack 18
09:38:14.556 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:14.986 fetch POST /api/observability/v1/traces
09:38:14.986 fetched POST /api/observability/v1/traces 204
09:38:15.076 chunk 18 52154 bytes
09:38:15.079 in Ack 18
09:38:15.606 in Ping 
09:38:15.705 rpc 37 server.reportClientActivity
09:38:16.080 fetch POST /api/observability/v1/traces
09:38:16.080 fetched POST /api/observability/v1/traces 204
09:38:16.351 chunk 18 54223 bytes
09:38:16.353 in Ack 18
09:38:17.354 fetch POST /api/observability/v1/traces
09:38:17.354 fetched POST /api/observability/v1/traces 204
09:38:20.608 in Ping 
09:38:21.581 exit 36 {"_tag":"Success","value":{"sequence":187}}
09:38:22.241 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:22.582 fetch POST /api/observability/v1/traces
09:38:22.582 fetched POST /api/observability/v1/traces 204
09:38:25.612 in Ping 
09:38:29.560 chunk 18 54266 bytes
09:38:29.564 in Ack 18
09:38:30.577 fetch POST /api/observability/v1/traces
09:38:30.577 fetched POST /api/observability/v1/traces 204
09:38:30.618 in Ping 
09:38:30.706 in Interrupt 37
09:38:31.707 fetch POST /api/observability/v1/traces
09:38:31.707 fetched POST /api/observability/v1/traces 204
09:38:35.620 in Ping 
09:38:40.624 in Ping 
09:38:40.705 rpc 39 server.reportClientActivity
09:38:41.707 fetch POST /api/observability/v1/traces
09:38:41.707 fetched POST /api/observability/v1/traces 204
09:38:45.630 in Ping 
09:38:49.488 chunk 18 57182 bytes
09:38:49.492 in Ack 18
09:38:49.741 chunk 18 57270 bytes
09:38:49.747 in Ack 18
09:38:50.040 chunk 18 57800 bytes
09:38:50.049 in Ack 18
09:38:50.547 chunk 18 58180 bytes
09:38:50.549 fetch POST /api/observability/v1/traces
09:38:50.550 fetched POST /api/observability/v1/traces 204
09:38:50.560 in Ack 18
09:38:50.632 in Ping 
09:38:50.802 exit 38 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:51.562 fetch POST /api/observability/v1/traces
09:38:51.562 fetched POST /api/observability/v1/traces 204
