# story, webapp, warm start, seed 12

Profile story (graded): seed 12's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 55 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.7s |
| Prompt reached the agent | 4.8s |
| Time to first text | 16.8s |
| Whole reply shown | 52.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.1s / 5.1s / 15.1s |
| Longest silence | 16.8s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 567288 / 197279 |
| Bytes the app received / sent | 249904 / 64707 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.1 | 0 | 207 |
| 20 | 5.1 | 397 | 604 |
| 30 | 15.1 | 397 | 1009 |
| 40 | 4.1 | 1077 | 1237 |
| 50 | 6.3 | 1378 | 1596 |
| 51 | 7.3 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","nodes":"1016/1016 809/809 809/809 1016/1016","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216513413,"silences":0,"longestSilenceMs":1213,"longestRecoveredGapMs":1213},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":16.191829561138597,"minRttMs":2,"sentPackets":140,"sentWireBytes":40545,"receivedPackets":315,"receivedWireBytes":153992,"lastHeardMs":13731,"unsyncedBytes":736,"inflightPackets":9,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 46/839","user":"10/10 9/9","attempts":"659/659 659/659 333/635","providerTurns":"772/772 772/772","nodes":"1016/1016 809/809 809/809 1016/1016","providerThreads":"807/807","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216524167,"silences":0,"longestSilenceMs":1672,"longestRecoveredGapMs":1672},"stats":{"sendRate":750,"deliveredRate":875.563591406737,"loss":0.2153846153846154,"srttMs":2228.022320022592,"minRttMs":2,"sentPackets":173,"sentWireBytes":47543,"receivedPackets":367,"receivedWireBytes":169773,"lastHeardMs":24485,"unsyncedBytes":0,"inflightPackets":2,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","nodes":"1016/1016 809/809 809/809 1016/1016 532/785","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 80/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216534428,"silences":0,"longestSilenceMs":1672,"longestRecoveredGapMs":1672},"stats":{"sendRate":816.2289247358924,"deliveredRate":277.5178344102035,"loss":0.6599999999999999,"srttMs":2432.5163037427437,"minRttMs":2509,"sentPackets":205,"sentWireBytes":54784,"receivedPackets":421,"receivedWireBytes":191476,"lastHeardMs":34747,"unsyncedBytes":132,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216544519,"silences":0,"longestSilenceMs":1672,"longestRecoveredGapMs":1672},"stats":{"sendRate":1273.9140588510043,"deliveredRate":800.0789954636147,"loss":0.1181818181818182,"srttMs":2467.567393145482,"minRttMs":2193,"sentPackets":253,"sentWireBytes":63386,"receivedPackets":530,"receivedWireBytes":225164,"lastHeardMs":44837,"unsyncedBytes":0,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 884/947","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 367/1392","providerThreads":"809/809","providerSessions":"2653/2653","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":870,"lastDataAt":1794216554784,"silences":0,"longestSilenceMs":1672,"longestRecoveredGapMs":1672},"stats":{"sendRate":1816.4675130694548,"deliveredRate":1112.8912686791818,"loss":0.05454545454545453,"srttMs":2457.017321905085,"minRttMs":2193,"sentPackets":304,"sentWireBytes":76846,"receivedPackets":636,"receivedWireBytes":277073,"lastHeardMs":55103,"unsyncedBytes":167,"inflightPackets":27,"duplicateBytes":0}}
- shim trace
09:28:20.846 fetch GET /api/auth/session
09:28:20.852 fetched GET /api/auth/session 200
09:28:21.338 fetch GET /.well-known/t3/environment
09:28:21.342 fetched GET /.well-known/t3/environment 200
09:28:21.416 fetch GET /.well-known/t3/environment
09:28:21.419 fetched GET /.well-known/t3/environment 200
09:28:21.467 rpc 0 subscribeServerConfig
09:28:21.468 chunk 0 38462 bytes
09:28:21.590 in Ack 0
09:28:21.594 rpc 1 subscribeServerLifecycle
09:28:21.596 rpc 2 subscribeServerLifecycle
09:28:21.605 fetch GET /api/auth/session
09:28:21.607 fetched GET /api/auth/session 200
09:28:21.608 rpc 3 subscribeDeviceState
09:28:21.612 rpc 4 preview.reportProfiles
09:28:21.687 rpc 5 server.reportClientActivity
09:28:22.143 rpc 6 subscribeProjectClones
09:28:22.205 fetch GET /api/orchestration/threads/mcp%3A0e720c5f-7dc9-4ca7-ab3c-cd2d3e5e2ae7/bounded
09:28:22.225 fetch GET /api/orchestration/shell
09:28:22.229 fetched GET /api/orchestration/shell 200
09:28:22.238 rpc 7 orchestration.subscribeShell
09:28:22.239 chunk 7 4680 bytes
09:28:22.296 rpc 8 server.refreshProviders
09:28:22.462 rpc 9 orchestration.dispatchCommand thread.visit
09:28:22.462 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:28:22.500 rpc 10 subscribeTerminalMetadata
09:28:22.501 rpc 11 subscribeVcsStatus
09:28:22.503 rpc 12 subscribePreviewEvents
09:28:22.505 rpc 13 assets.createUrl
09:28:22.506 rpc 14 preview.list
09:28:22.507 rpc 15 projects.readFile
09:28:22.511 in Ack 7
09:28:22.511 chunk 7 4595 bytes
09:28:22.522 fetch POST /api/observability/v1/traces
09:28:22.522 fetched POST /api/observability/v1/traces 204
09:28:22.540 in Ack 7
09:28:22.808 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"2b278056-cfa5-460a-83f3-eac23b5ae5ba","sessions":[]}}
09:28:22.808 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:22.809 chunk 1 3815 bytes
09:28:22.998 in Ack 1
09:28:23.214 chunk 2 3815 bytes
09:28:23.214 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:50.080Z","configurableProviders":false,"con
09:28:23.214 in Ack 2
09:28:23.319 chunk 11 1632 bytes
09:28:23.319 exit 13 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:23.320 in Ack 11
09:28:23.525 fetch POST /api/observability/v1/traces
09:28:23.525 fetched POST /api/observability/v1/traces 204
09:28:23.556 chunk 3 876 bytes
09:28:23.556 exit 4 {"_tag":"Success","value":null}
09:28:23.556 exit 5 {"_tag":"Success","value":null}
09:28:23.556 chunk 6 4 bytes
09:28:23.556 chunk 10 36 bytes
09:28:23.557 in Ack 3
09:28:23.568 rpc 16 preview.reportProfiles
09:28:23.569 in Ack 6
09:28:23.570 in Ack 10
09:28:23.577 chunk 7 4638 bytes
09:28:23.578 in Ack 7
09:28:23.613 exit 16 {"_tag":"Success","value":null}
09:28:23.821 rpc 17 server.reportClientActivity
09:28:23.893 exit 17 {"_tag":"Success","value":null}
09:28:24.079 chunk 7 4635 bytes
09:28:24.079 in Ack 7
09:28:24.347 chunk 7 4635 bytes
09:28:24.348 in Ack 7
09:28:24.413 fetched GET /api/orchestration/threads/mcp%3A0e720c5f-7dc9-4ca7-ab3c-cd2d3e5e2ae7/bounded 200
09:28:24.420 rpc 18 orchestration.subscribeThread
09:28:24.420 chunk 18 8282 bytes
09:28:24.444 in Interrupt 11
09:28:24.448 rpc 19 subscribeVcsStatus
09:28:24.450 in Ack 18
09:28:24.588 fetch POST /api/observability/v1/traces
09:28:24.588 fetched POST /api/observability/v1/traces 204
09:28:24.671 chunk 18 9336 bytes
09:28:24.672 in Ack 18
09:28:24.744 chunk 19 1632 bytes
09:28:24.745 rpc 20 server.reportClientActivity
09:28:24.747 in Ack 19
09:28:24.946 chunk 18 13497 bytes
09:28:24.952 in Ack 18
09:28:25.195 chunk 18 19019 bytes
09:28:25.198 in Ack 18
09:28:25.427 exit 20 {"_tag":"Success","value":null}
09:28:25.445 chunk 18 21099 bytes
09:28:25.447 in Ack 18
09:28:25.484 in Interrupt 19
09:28:25.492 rpc 21 subscribeVcsStatus
09:28:25.556 chunk 21 1632 bytes
09:28:25.561 in Ack 21
09:28:25.699 fetch POST /api/observability/v1/traces
09:28:25.699 fetched POST /api/observability/v1/traces 204
09:28:25.743 rpc 22 server.reportClientActivity
09:28:25.771 exit 22 {"_tag":"Success","value":null}
09:28:26.044 rpc 23 attachments.createUploadUrl
09:28:26.172 exit 23 {"_tag":"Success","value":{"attachmentId":"pending-4599709b-e4af-4364-94db-680bc5babd5e","expiresAt":1794217106123,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:26.174 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTQ1OTk3MDliLWU0YWYtNDM2NC05NGRiLTY4MGJjNWJhYmQ1ZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzEwNjEyM30.dtXxmuHFHOJqSfilu5Wh4RVCmj7XBJgueYy4F_-6wes
09:28:26.462 in Ping 
09:28:26.743 fetch POST /api/observability/v1/traces
09:28:26.743 fetched POST /api/observability/v1/traces 204
09:28:27.622 rpc 24 orchestration.dispatchCommand message.dispatch
09:28:28.100 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:28:28.102 rpc 25 attachments.delete
09:28:28.124 chunk 18 23892 bytes
09:28:28.127 in Ack 18
09:28:28.131 rpc 26 assets.createUrl
09:28:28.374 chunk 7 4721 bytes
09:28:28.375 chunk 18 27722 bytes
09:28:28.376 in Ack 7
09:28:28.427 in Ack 18
09:28:28.493 exit 25 {"_tag":"Success","value":null}
09:28:28.620 fetch POST /api/observability/v1/traces
09:28:28.620 fetched POST /api/observability/v1/traces 204
09:28:28.627 chunk 18 27722 bytes
09:28:28.633 in Ack 18
09:28:28.883 chunk 18 30179 bytes
09:28:28.885 in Ack 18
09:28:29.107 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/fbc128c8-91f5-499a-92f2-a0615d62079f","expiresAt":1794220109107}}
09:28:29.128 chunk 7 4660 bytes
09:28:29.129 in Ack 7
09:28:29.140 chunk 18 31902 bytes
09:28:29.145 in Ack 18
09:28:29.635 fetch POST /api/observability/v1/traces
09:28:29.635 fetched POST /api/observability/v1/traces 204
09:28:30.018 chunk 18 32034 bytes
09:28:30.022 in Ack 18
09:28:30.264 chunk 7 4657 bytes
09:28:30.264 in Ack 7
09:28:30.266 rpc 27 orchestration.dispatchCommand thread.visit
09:28:30.266 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:28:30.274 chunk 18 35259 bytes
09:28:30.277 in Ack 18
09:28:30.290 in Interrupt 21
09:28:30.293 rpc 28 subscribeVcsStatus
09:28:30.306 chunk 28 1632 bytes
09:28:30.307 in Ack 28
09:28:30.815 chunk 7 4657 bytes
09:28:30.815 chunk 18 35259 bytes
09:28:30.816 rpc 29 server.reportClientActivity
09:28:30.818 in Ack 7
09:28:30.829 in Ack 18
09:28:30.873 exit 29 {"_tag":"Success","value":null}
09:28:31.024 fetch POST /api/observability/v1/traces
09:28:31.024 fetched POST /api/observability/v1/traces 204
09:28:31.531 in Ping 
09:28:33.227 rpc 30 server.refreshProviders
09:28:33.227 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:50.080Z","configurableProviders":false,"con
09:28:34.151 rpc 31 orchestration.dispatchCommand message.dispatch
09:28:34.231 fetch POST /api/observability/v1/traces
09:28:34.231 fetched POST /api/observability/v1/traces 204
09:28:36.123 fetch POST /api/observability/v1/traces
09:28:36.123 fetched POST /api/observability/v1/traces 204
09:28:36.531 in Ping 
09:28:38.435 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:28:39.437 fetch POST /api/observability/v1/traces
09:28:39.437 fetched POST /api/observability/v1/traces 204
09:28:41.532 in Ping 
09:28:46.265 chunk 18 37280 bytes
09:28:46.269 in Ack 18
09:28:46.533 in Ping 
09:28:46.545 chunk 18 37935 bytes
09:28:46.547 in Ack 18
09:28:46.610 rpc 32 server.reportClientActivity
09:28:47.274 fetch POST /api/observability/v1/traces
09:28:47.274 fetched POST /api/observability/v1/traces 204
09:28:48.816 chunk 7 4721 bytes
09:28:48.816 in Ack 7
09:28:48.819 rpc 33 orchestration.dispatchCommand thread.visit
09:28:48.819 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:28:48.909 chunk 18 38571 bytes
09:28:48.912 in Ack 18
09:28:49.160 chunk 18 39411 bytes
09:28:49.163 in Ack 18
09:28:49.819 fetch POST /api/observability/v1/traces
09:28:49.819 fetched POST /api/observability/v1/traces 204
09:28:49.845 chunk 18 39413 bytes
09:28:49.850 in Ack 18
09:28:50.096 chunk 18 42514 bytes
09:28:50.098 in Ack 18
09:28:50.590 chunk 18 42826 bytes
09:28:50.595 in Ack 18
09:28:50.852 fetch POST /api/observability/v1/traces
09:28:50.852 fetched POST /api/observability/v1/traces 204
09:28:51.533 in Ping 
09:28:51.671 chunk 18 42778 bytes
09:28:51.673 in Ack 18
09:28:52.674 fetch POST /api/observability/v1/traces
09:28:52.674 fetched POST /api/observability/v1/traces 204
09:28:53.586 chunk 18 43822 bytes
09:28:53.587 in Ack 18
09:28:54.589 fetch POST /api/observability/v1/traces
09:28:54.589 fetched POST /api/observability/v1/traces 204
09:28:56.533 in Ping 
09:28:57.362 fetch POST /api/observability/v1/traces
09:28:57.363 fetched POST /api/observability/v1/traces 204
09:28:57.412 chunk 18 45133 bytes
09:28:57.414 in Ack 18
09:28:58.412 chunk 7 4721 bytes
09:28:58.414 in Ack 7
09:28:58.441 fetch POST /api/observability/v1/traces
09:28:58.442 fetched POST /api/observability/v1/traces 204
09:28:58.824 rpc 34 orchestration.dispatchCommand thread.visit
09:28:58.824 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:28:59.825 fetch POST /api/observability/v1/traces
09:28:59.825 fetched POST /api/observability/v1/traces 204
09:29:00.421 chunk 18 45133 bytes
09:29:00.423 in Ack 18
09:29:00.705 exit 32 {"_tag":"Success","value":null}
09:29:00.921 chunk 18 45133 bytes
09:29:00.924 in Ack 18
09:29:01.424 fetch POST /api/observability/v1/traces
09:29:01.424 fetched POST /api/observability/v1/traces 204
09:29:01.535 in Ping 
09:29:02.680 chunk 7 4785 bytes
09:29:02.681 in Ack 7
09:29:03.684 fetch POST /api/observability/v1/traces
09:29:03.684 fetched POST /api/observability/v1/traces 204
09:29:06.535 in Ping 
09:29:06.976 chunk 18 47326 bytes
09:29:06.979 in Ack 18
09:29:07.226 chunk 18 47665 bytes
09:29:07.229 in Ack 18
09:29:07.885 chunk 18 48667 bytes
09:29:07.893 in Ack 18
09:29:07.994 fetch POST /api/observability/v1/traces
09:29:07.994 fetched POST /api/observability/v1/traces 204
09:29:08.354 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:29:09.354 fetch POST /api/observability/v1/traces
09:29:09.354 fetched POST /api/observability/v1/traces 204
09:29:11.105 exit 35 {"_tag":"Success","value":{"sequence":186}}
09:29:11.535 in Ping 
09:29:11.608 rpc 36 server.reportClientActivity
09:29:11.855 chunk 18 48710 bytes
09:29:11.857 in Ack 18
09:29:12.111 fetch POST /api/observability/v1/traces
09:29:12.111 fetched POST /api/observability/v1/traces 204
09:29:13.107 chunk 7 4787 bytes
09:29:13.108 in Ack 7
09:29:13.110 rpc 37 orchestration.dispatchCommand thread.visit
09:29:13.110 exit 37 {"_tag":"Success","value":{"sequence":0}}
09:29:13.610 chunk 18 52295 bytes
09:29:13.612 in Ack 18
09:29:13.860 chunk 18 52295 bytes
09:29:13.864 in Ack 18
09:29:14.109 fetch POST /api/observability/v1/traces
09:29:14.109 fetched POST /api/observability/v1/traces 204
09:29:14.868 chunk 18 54412 bytes
09:29:14.872 in Ack 18
09:29:14.887 in Interrupt 28
09:29:14.891 rpc 38 subscribeVcsStatus
09:29:15.368 chunk 18 56789 bytes
09:29:15.376 in Ack 18
09:29:15.618 chunk 18 56790 bytes
09:29:15.629 in Ack 18
09:29:15.870 chunk 7 4787 bytes
09:29:15.872 in Ack 7
09:29:15.881 fetch POST /api/observability/v1/traces
09:29:15.881 fetched POST /api/observability/v1/traces 204
09:29:16.122 chunk 18 56790 bytes
09:29:16.125 in Ack 18
09:29:16.254 exit 36 {"_tag":"Success","value":null}
09:29:16.371 chunk 7 5094 bytes
09:29:16.372 chunk 18 57807 bytes
09:29:16.373 in Ack 7
09:29:16.393 in Ack 18
09:29:16.510 rpc 39 server.reportClientActivity
09:29:16.542 in Ping 
09:29:17.127 fetch POST /api/observability/v1/traces
09:29:17.127 fetched POST /api/observability/v1/traces 204
09:29:17.629 chunk 38 1713 bytes
09:29:17.630 in Ack 38
09:29:17.879 chunk 7 5094 bytes
09:29:17.880 in Ack 7
09:29:18.882 fetch POST /api/observability/v1/traces
09:29:18.882 fetched POST /api/observability/v1/traces 204
09:29:19.133 chunk 18 60995 bytes
09:29:19.136 in Ack 18
09:29:19.381 chunk 7 5094 bytes
09:29:19.382 in Ack 7
09:29:19.384 chunk 18 61950 bytes
09:29:19.387 in Ack 18
09:29:19.944 fetch POST /api/observability/v1/traces
09:29:19.944 fetched POST /api/observability/v1/traces 204
09:29:21.543 in Ping 
09:29:22.499 exit 39 {"_tag":"Success","value":null}
09:29:23.112 rpc 40 orchestration.dispatchCommand thread.visit
09:29:23.112 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:29:23.501 fetch POST /api/observability/v1/traces
09:29:23.501 fetched POST /api/observability/v1/traces 204
09:29:24.281 chunk 7 4789 bytes
09:29:24.284 in Ack 7
09:29:25.238 chunk 18 62967 bytes
09:29:25.242 in Ack 18
09:29:25.564 chunk 18 63017 bytes
09:29:25.575 fetch POST /api/observability/v1/traces
09:29:25.575 fetched POST /api/observability/v1/traces 204
09:29:25.737 in Ack 18
09:29:26.750 chunk 7 4728 bytes
09:29:26.751 chunk 18 64175 bytes
09:29:26.751 in Ping 
09:29:26.752 fetch POST /api/observability/v1/traces
09:29:26.752 fetched POST /api/observability/v1/traces 204
09:29:26.754 in Ack 7
09:29:26.764 in Ack 18
