# story, webapp, warm start, seed 31

Profile story (graded): seed 31's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 80 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 4.0s |
| Time to first text | 16.1s |
| Whole reply shown | 78.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 39.9s / 7.9s / 35.9s |
| Longest silence | 37.9s |
| Approval shown after it was asked | 29.4s |
| Bytes on the wire down / up | 248660 / 146734 |
| Bytes the app received / sent | 96194 / 71686 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.9 | 0 | 239 |
| 20 | 5.9 | 397 | 635 |
| 30 | 15.9 | 397 | 1041 |
| 40 | 25.9 | 397 | 1077 |
| 50 | 35.9 | 397 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 8.4 | 1077 | 1406 |
| 78 | 8.6 | 1379 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 43/772","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 106/901","nodes":"1016/1016 809/809 809/809 14/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217056328,"silences":0,"longestSilenceMs":1473,"longestRecoveredGapMs":1473},"stats":{"sendRate":3000,"deliveredRate":1621.6448764484462,"loss":0,"srttMs":250.9222747832767,"minRttMs":2,"sentPackets":128,"sentWireBytes":38757,"receivedPackets":178,"receivedWireBytes":111758,"lastHeardMs":16895,"unsyncedBytes":1109,"inflightPackets":22,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 525/654","providerThreads":"528/807","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 12/451","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217066207,"silences":0,"longestSilenceMs":1473,"longestRecoveredGapMs":1473},"stats":{"sendRate":843.75,"deliveredRate":1348.5453508271535,"loss":0.08333333333333337,"srttMs":1844.7860351460213,"minRttMs":2,"sentPackets":171,"sentWireBytes":48637,"receivedPackets":246,"receivedWireBytes":141694,"lastHeardMs":26774,"unsyncedBytes":0,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"528/807","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451","nodes":"1016/1016 809/809 809/809 1016/1016 719/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217076605,"silences":0,"longestSilenceMs":1737,"longestRecoveredGapMs":1737},"stats":{"sendRate":1122.2910216718267,"deliveredRate":635.9649122807018,"loss":0.15000000000000002,"srttMs":2351.218145847875,"minRttMs":1866,"sentPackets":210,"sentWireBytes":55798,"receivedPackets":288,"receivedWireBytes":158408,"lastHeardMs":37172,"unsyncedBytes":0,"inflightPackets":7,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"737/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"498/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451 327/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217085945,"silences":0,"longestSilenceMs":1737,"longestRecoveredGapMs":1737},"stats":{"sendRate":1122.2910216718267,"deliveredRate":1234.1519131924615,"loss":0.08333333333333337,"srttMs":2164.788718800432,"minRttMs":1709,"sentPackets":256,"sentWireBytes":64024,"receivedPackets":315,"receivedWireBytes":166790,"lastHeardMs":46511,"unsyncedBytes":0,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"737/961","assistant":"32/32 21/21 689/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"622/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451 327/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217093628,"silences":1,"longestSilenceMs":4775,"longestRecoveredGapMs":4775},"stats":{"sendRate":1683.4365325077401,"deliveredRate":0,"loss":0.08333333333333337,"srttMs":2561.315128950378,"minRttMs":1709,"sentPackets":271,"sentWireBytes":67825,"receivedPackets":318,"receivedWireBytes":167895,"lastHeardMs":54200,"unsyncedBytes":457,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"737/961","assistant":"32/32 21/21 689/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"622/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451 327/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217093628,"silences":1,"longestSilenceMs":4775,"longestRecoveredGapMs":4775},"stats":{"sendRate":1683.4365325077401,"deliveredRate":0,"loss":0.08333333333333337,"srttMs":2561.315128950378,"minRttMs":1709,"sentPackets":283,"sentWireBytes":71198,"receivedPackets":318,"receivedWireBytes":167895,"lastHeardMs":54200,"unsyncedBytes":457,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 626/1542","thread":"737/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"622/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451 327/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217118266,"silences":2,"longestSilenceMs":14549,"longestRecoveredGapMs":14549},"stats":{"sendRate":2000,"deliveredRate":1877.9585798816568,"loss":0,"srttMs":2199.7618109421273,"minRttMs":1709,"sentPackets":347,"sentWireBytes":87805,"receivedPackets":342,"receivedWireBytes":176624,"lastHeardMs":78833,"unsyncedBytes":398,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 342/1566 1012/1068","thread":"737/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"622/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 114/451 327/543","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":595,"lastDataAt":1794217129287,"silences":2,"longestSilenceMs":14549,"longestRecoveredGapMs":14549},"stats":{"sendRate":2926.058189548008,"deliveredRate":1688.148373419179,"loss":0,"srttMs":2015.1679635375563,"minRttMs":1664,"sentPackets":393,"sentWireBytes":100306,"receivedPackets":368,"receivedWireBytes":189649,"lastHeardMs":89853,"unsyncedBytes":0,"inflightPackets":21,"duplicateBytes":0}}
- shim trace
09:37:20.269 fetch GET /api/auth/session
09:37:20.272 fetched GET /api/auth/session 200
09:37:20.689 fetch GET /.well-known/t3/environment
09:37:20.690 fetched GET /.well-known/t3/environment 200
09:37:20.716 fetch GET /.well-known/t3/environment
09:37:20.717 fetched GET /.well-known/t3/environment 200
09:37:20.732 rpc 0 subscribeServerConfig
09:37:20.733 chunk 0 38462 bytes
09:37:20.803 in Ack 0
09:37:20.807 rpc 1 subscribeServerLifecycle
09:37:20.808 rpc 2 subscribeServerLifecycle
09:37:20.814 fetch GET /api/auth/session
09:37:20.816 fetched GET /api/auth/session 200
09:37:20.816 rpc 3 subscribeDeviceState
09:37:20.820 rpc 4 preview.reportProfiles
09:37:20.947 rpc 5 subscribeProjectClones
09:37:21.148 rpc 6 server.reportClientActivity
09:37:21.155 fetch GET /api/orchestration/threads/mcp%3A03ce1fe6-86b4-4cbd-a585-701fe5f2e31d/bounded
09:37:21.177 fetch GET /api/orchestration/shell
09:37:21.188 fetched GET /api/orchestration/shell 200
09:37:21.213 rpc 7 orchestration.subscribeShell
09:37:21.213 chunk 7 4594 bytes
09:37:21.282 rpc 8 server.refreshProviders
09:37:21.463 rpc 9 orchestration.dispatchCommand thread.visit
09:37:21.463 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:21.543 rpc 10 subscribeTerminalMetadata
09:37:21.544 rpc 11 subscribeVcsStatus
09:37:21.548 rpc 12 subscribePreviewEvents
09:37:21.550 rpc 13 assets.createUrl
09:37:21.554 rpc 14 preview.list
09:37:21.554 rpc 15 projects.readFile
09:37:21.560 in Ack 7
09:37:21.895 fetch POST /api/observability/v1/traces
09:37:21.896 fetched POST /api/observability/v1/traces 204
09:37:22.076 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"1c2c1a2d-9c38-4fc8-8e51-f11a4413c09f","sessions":[]}}
09:37:22.076 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:22.528 chunk 1 3815 bytes
09:37:22.528 in Ack 1
09:37:22.841 chunk 2 3815 bytes
09:37:22.841 in Ack 2
09:37:22.978 fetch POST /api/observability/v1/traces
09:37:22.979 fetched POST /api/observability/v1/traces 204
09:37:23.059 chunk 7 4570 bytes
09:37:23.060 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:53.077Z","configurableProviders":false,"con
09:37:23.060 chunk 11 1632 bytes
09:37:23.060 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:23.061 in Ack 7
09:37:23.074 in Ack 11
09:37:23.310 chunk 7 4656 bytes
09:37:23.311 in Ack 7
09:37:23.560 chunk 7 4595 bytes
09:37:23.561 in Ack 7
09:37:23.810 chunk 7 4638 bytes
09:37:23.811 in Ack 7
09:37:23.832 chunk 3 876 bytes
09:37:23.832 exit 4 {"_tag":"Success","value":null}
09:37:23.832 chunk 5 4 bytes
09:37:23.832 exit 6 {"_tag":"Success","value":null}
09:37:23.833 in Ack 3
09:37:23.842 rpc 16 preview.reportProfiles
09:37:23.843 in Ack 5
09:37:23.898 chunk 10 36 bytes
09:37:23.898 exit 16 {"_tag":"Success","value":null}
09:37:23.900 in Ack 10
09:37:24.063 fetch POST /api/observability/v1/traces
09:37:24.063 fetched POST /api/observability/v1/traces 204
09:37:24.095 rpc 17 server.reportClientActivity
09:37:24.163 exit 17 {"_tag":"Success","value":null}
09:37:24.311 chunk 7 4635 bytes
09:37:24.312 in Ack 7
09:37:25.095 fetch POST /api/observability/v1/traces
09:37:25.095 fetched POST /api/observability/v1/traces 204
09:37:25.721 in Ping 
09:37:26.106 fetched GET /api/orchestration/threads/mcp%3A03ce1fe6-86b4-4cbd-a585-701fe5f2e31d/bounded 200
09:37:26.112 rpc 18 orchestration.subscribeThread
09:37:26.112 chunk 18 8282 bytes
09:37:26.129 in Interrupt 11
09:37:26.132 rpc 19 subscribeVcsStatus
09:37:26.134 in Ack 18
09:37:26.237 chunk 19 1632 bytes
09:37:26.237 in Ack 19
09:37:26.363 chunk 18 11259 bytes
09:37:26.365 in Ack 18
09:37:26.483 rpc 20 server.reportClientActivity
09:37:26.618 chunk 18 12690 bytes
09:37:26.620 in Ack 18
09:37:26.643 exit 20 {"_tag":"Success","value":null}
09:37:26.869 chunk 18 16150 bytes
09:37:26.876 in Ack 18
09:37:27.148 fetch POST /api/observability/v1/traces
09:37:27.148 fetched POST /api/observability/v1/traces 204
09:37:27.149 chunk 18 19019 bytes
09:37:27.151 in Ack 18
09:37:27.264 rpc 21 attachments.createUploadUrl
09:37:27.399 chunk 18 21099 bytes
09:37:27.401 in Ack 18
09:37:27.433 in Interrupt 19
09:37:27.436 rpc 22 subscribeVcsStatus
09:37:27.440 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-7ec3b31f-abc0-4d48-bb67-1721f432f67f","expiresAt":1794217647386,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:27.441 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTdlYzNiMzFmLWFiYzAtNGQ0OC1iYjY3LTE3MjFmNDMyZjY3ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0NzM4Nn0.BGpHPNdsIpjUttb2OG76HtCS3WTavZCu2nj-l0Wh6QA
09:37:27.482 chunk 22 1632 bytes
09:37:27.483 in Ack 22
09:37:27.688 rpc 23 server.reportClientActivity
09:37:28.152 fetch POST /api/observability/v1/traces
09:37:28.152 fetched POST /api/observability/v1/traces 204
09:37:28.674 exit 23 {"_tag":"Success","value":null}
09:37:29.676 fetch POST /api/observability/v1/traces
09:37:29.676 fetched POST /api/observability/v1/traces 204
09:37:30.922 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:31.307 in Ping 
09:37:31.475 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:31.478 rpc 25 attachments.delete
09:37:31.921 fetch POST /api/observability/v1/traces
09:37:31.921 fetched POST /api/observability/v1/traces 204
09:37:32.185 exit 25 {"_tag":"Success","value":null}
09:37:32.227 chunk 7 4721 bytes
09:37:32.227 chunk 18 21735 bytes
09:37:32.228 in Ack 7
09:37:32.231 rpc 26 orchestration.dispatchCommand thread.visit
09:37:32.231 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:32.372 in Ack 18
09:37:32.485 chunk 18 25032 bytes
09:37:32.488 in Ack 18
09:37:32.735 chunk 7 4660 bytes
09:37:32.735 chunk 18 25032 bytes
09:37:32.736 in Ack 7
09:37:32.745 in Ack 18
09:37:32.997 fetch POST /api/observability/v1/traces
09:37:32.997 fetched POST /api/observability/v1/traces 204
09:37:33.235 chunk 7 4657 bytes
09:37:33.236 in Ack 7
09:37:33.237 rpc 27 orchestration.dispatchCommand thread.visit
09:37:33.237 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:33.245 chunk 18 27981 bytes
09:37:33.248 in Ack 18
09:37:33.251 rpc 28 assets.createUrl
09:37:33.738 chunk 7 4657 bytes
09:37:34.112 in Ack 7
09:37:34.127 chunk 18 31214 bytes
09:37:34.133 in Ack 18
09:37:34.239 fetch POST /api/observability/v1/traces
09:37:34.239 fetched POST /api/observability/v1/traces 204
09:37:34.376 rpc 29 server.refreshProviders
09:37:34.376 exit 29 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:53.077Z","configurableProviders":false,"con
09:37:34.418 chunk 18 31992 bytes
09:37:34.445 in Ack 18
09:37:34.456 in Interrupt 22
09:37:34.460 rpc 30 subscribeVcsStatus
09:37:34.720 rpc 31 server.reportClientActivity
09:37:35.394 fetch POST /api/observability/v1/traces
09:37:35.394 fetched POST /api/observability/v1/traces 204
09:37:36.308 rpc 32 orchestration.dispatchCommand message.dispatch
09:37:36.330 chunk 18 31992 bytes
09:37:36.330 in Ping 
09:37:36.334 in Ack 18
09:37:36.576 chunk 30 1713 bytes
09:37:36.580 in Ack 30
09:37:36.585 chunk 18 32765 bytes
09:37:36.591 in Ack 18
09:37:37.088 chunk 18 33331 bytes
09:37:37.095 in Ack 18
09:37:37.308 fetch POST /api/observability/v1/traces
09:37:37.308 fetched POST /api/observability/v1/traces 204
09:37:37.338 chunk 18 35259 bytes
09:37:37.340 in Ack 18
09:37:37.589 chunk 18 35259 bytes
09:37:37.593 in Ack 18
09:37:38.342 fetch POST /api/observability/v1/traces
09:37:38.343 fetched POST /api/observability/v1/traces 204
09:37:38.468 exit 32 {"_tag":"Success","value":{"sequence":153}}
09:37:39.472 fetch POST /api/observability/v1/traces
09:37:39.472 fetched POST /api/observability/v1/traces 204
09:37:40.098 chunk 7 4701 bytes
09:37:40.099 in Ack 7
09:37:40.598 chunk 18 37280 bytes
09:37:40.603 in Ack 18
09:37:41.100 fetch POST /api/observability/v1/traces
09:37:41.100 fetched POST /api/observability/v1/traces 204
09:37:41.331 in Ping 
09:37:43.003 fetch POST /api/observability/v1/traces
09:37:43.003 fetched POST /api/observability/v1/traces 204
09:37:43.245 rpc 33 orchestration.dispatchCommand thread.visit
09:37:43.245 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:37:43.542 chunk 18 37916 bytes
09:37:43.544 in Ack 18
09:37:44.247 fetch POST /api/observability/v1/traces
09:37:44.247 fetched POST /api/observability/v1/traces 204
09:37:45.669 chunk 18 38756 bytes
09:37:45.676 in Ack 18
09:37:46.341 in Ping 
09:37:46.678 fetch POST /api/observability/v1/traces
09:37:46.678 fetched POST /api/observability/v1/traces 204
09:37:47.701 fetch POST /api/observability/v1/traces
09:37:47.701 fetched POST /api/observability/v1/traces 204
09:37:49.453 chunk 18 39411 bytes
09:37:49.455 in Ack 18
09:37:49.720 in Interrupt 31
09:37:49.974 rpc 34 server.reportClientActivity
09:37:50.457 fetch POST /api/observability/v1/traces
09:37:50.457 fetched POST /api/observability/v1/traces 204
09:37:51.341 in Ping 
09:37:51.900 chunk 18 42474 bytes
09:37:51.903 in Ack 18
09:37:52.149 chunk 18 42824 bytes
09:37:52.151 in Ack 18
09:37:52.904 fetch POST /api/observability/v1/traces
09:37:52.905 fetched POST /api/observability/v1/traces 204
09:37:54.377 chunk 18 42776 bytes
09:37:54.379 in Ack 18
09:37:55.380 fetch POST /api/observability/v1/traces
09:37:55.380 fetched POST /api/observability/v1/traces 204
09:37:56.341 in Ping 
09:38:01.341 in Ping 
09:38:03.519 chunk 7 4721 bytes
09:38:03.520 in Ack 7
09:38:03.524 rpc 35 orchestration.dispatchCommand thread.visit
09:38:03.524 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:04.521 fetch POST /api/observability/v1/traces
09:38:04.521 fetched POST /api/observability/v1/traces 204
09:38:04.975 in Interrupt 34
09:38:05.717 chunk 18 43562 bytes
09:38:05.726 in Ack 18
09:38:05.976 fetch POST /api/observability/v1/traces
09:38:05.976 fetched POST /api/observability/v1/traces 204
09:38:06.344 in Ping 
09:38:11.055 rpc 36 server.reportClientActivity
09:38:11.344 in Ping 
09:38:12.051 fetch POST /api/observability/v1/traces
09:38:12.051 fetched POST /api/observability/v1/traces 204
09:38:12.189 chunk 18 43758 bytes
09:38:12.193 in Ack 18
09:38:13.194 fetch POST /api/observability/v1/traces
09:38:13.194 fetched POST /api/observability/v1/traces 204
09:38:13.637 chunk 18 43950 bytes
09:38:13.641 in Ack 18
09:38:14.642 fetch POST /api/observability/v1/traces
09:38:14.642 fetched POST /api/observability/v1/traces 204
09:38:16.347 in Ping 
09:38:21.351 in Ping 
09:38:26.057 in Interrupt 36
09:38:26.355 in Ping 
09:38:27.059 fetch POST /api/observability/v1/traces
09:38:27.059 fetched POST /api/observability/v1/traces 204
09:38:28.179 chunk 18 44300 bytes
09:38:28.183 in Ack 18
09:38:28.605 chunk 18 44340 bytes
09:38:28.608 in Ack 18
09:38:29.184 fetch POST /api/observability/v1/traces
09:38:29.184 fetched POST /api/observability/v1/traces 204
09:38:29.454 chunk 18 44704 bytes
09:38:29.459 in Ack 18
09:38:29.975 chunk 18 44782 bytes
09:38:29.986 in Ack 18
09:38:30.460 fetch POST /api/observability/v1/traces
09:38:30.460 fetched POST /api/observability/v1/traces 204
09:38:30.942 chunk 18 45250 bytes
09:38:30.946 in Ack 18
09:38:31.643 in Ping 
09:38:31.947 fetch POST /api/observability/v1/traces
09:38:31.948 fetched POST /api/observability/v1/traces 204
09:38:36.051 rpc 37 server.reportClientActivity
09:38:36.327 chunk 18 46252 bytes
09:38:36.331 in Ack 18
09:38:36.647 in Ping 
09:38:36.755 rpc 38 orchestration.dispatchCommand runtime-request.respond
09:38:37.063 fetch POST /api/observability/v1/traces
09:38:37.063 fetched POST /api/observability/v1/traces 204
09:38:38.970 exit 38 {"_tag":"Success","value":{"sequence":209}}
09:38:39.360 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:38:39.972 fetch POST /api/observability/v1/traces
09:38:39.972 fetched POST /api/observability/v1/traces 204
09:38:41.651 in Ping 
09:38:41.743 chunk 18 46295 bytes
09:38:41.743 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:41.746 in Ack 18
09:38:42.748 fetch POST /api/observability/v1/traces
09:38:42.748 fetched POST /api/observability/v1/traces 204
09:38:44.914 chunk 18 46345 bytes
09:38:44.917 in Ack 18
09:38:45.918 fetch POST /api/observability/v1/traces
09:38:45.918 fetched POST /api/observability/v1/traces 204
09:38:46.655 in Ping 
09:38:46.676 chunk 18 49334 bytes
09:38:46.678 in Ack 18
09:38:47.242 chunk 18 49580 bytes
09:38:47.246 in Ack 18
09:38:47.679 fetch POST /api/observability/v1/traces
09:38:47.680 fetched POST /api/observability/v1/traces 204
09:38:49.721 chunk 18 49532 bytes
09:38:49.724 in Ack 18
09:38:50.726 fetch POST /api/observability/v1/traces
09:38:50.726 fetched POST /api/observability/v1/traces 204
09:38:51.052 in Interrupt 37
09:38:51.659 in Ping 
09:38:52.053 fetch POST /api/observability/v1/traces
09:38:52.053 fetched POST /api/observability/v1/traces 204
09:38:52.723 chunk 18 49952 bytes
09:38:52.727 in Ack 18
09:38:53.141 chunk 18 50210 bytes
09:38:53.143 in Ack 18
09:38:53.745 fetch POST /api/observability/v1/traces
09:38:53.745 fetched POST /api/observability/v1/traces 204
09:38:54.402 chunk 18 50260 bytes
09:38:54.408 in Ack 18
