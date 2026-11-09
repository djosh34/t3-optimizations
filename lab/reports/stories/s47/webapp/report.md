# story, webapp, warm start, seed 47

Profile story (graded): seed 47's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 61 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.6s |
| Prompt reached the agent | 9.1s |
| Time to first text | 22.1s |
| Whole reply shown | 58.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 17.8s / 5.2s / 14.8s |
| Longest silence | 22.1s |
| Approval shown after it was asked | 2.4s |
| Bytes on the wire down / up | 392355 / 143307 |
| Bytes the app received / sent | 321679 / 89509 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.8 | 0 | 33 |
| 20 | 10.8 | 0 | 431 |
| 30 | 10.8 | 397 | 835 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 8.2 | 1077 | 1398 |
| 58 | 8.5 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217061830,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":6750,"deliveredRate":2674.60978317791,"loss":0,"srttMs":1258.1669390476666,"minRttMs":1,"sentPackets":164,"sentWireBytes":43687,"receivedPackets":198,"receivedWireBytes":115753,"lastHeardMs":17110,"unsyncedBytes":518,"inflightPackets":20,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 818/818","user":"10/10 9/9 73/73","attempts":"659/659 659/659 305/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 763/763","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217071842,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":1500,"deliveredRate":989.8477157360406,"loss":0.5454545454545454,"srttMs":1441.1576963254902,"minRttMs":1,"sentPackets":232,"sentWireBytes":62352,"receivedPackets":290,"receivedWireBytes":159113,"lastHeardMs":27123,"unsyncedBytes":457,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21 350/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217081759,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":3000,"deliveredRate":833.0786860198625,"loss":0.5,"srttMs":1448.1781550729795,"minRttMs":1345,"sentPackets":295,"sentWireBytes":76568,"receivedPackets":411,"receivedWireBytes":210784,"lastHeardMs":37039,"unsyncedBytes":0,"inflightPackets":57,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"882/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217092037,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":3000,"deliveredRate":1026.9741985926505,"loss":0.47619047619047616,"srttMs":1541.5952011886916,"minRttMs":1345,"sentPackets":352,"sentWireBytes":86946,"receivedPackets":518,"receivedWireBytes":258740,"lastHeardMs":47317,"unsyncedBytes":0,"inflightPackets":65,"duplicateBytes":0}}
- sample failed: locator.click: Timeout 2000ms exceeded.
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 923/923","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 293/1368 83/1016","messages":"456/456 544/544 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217102583,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":1500,"deliveredRate":942.8600568990811,"loss":0.5107212475633528,"srttMs":1995.5277645539566,"minRttMs":1345,"sentPackets":431,"sentWireBytes":104611,"receivedPackets":639,"receivedWireBytes":330908,"lastHeardMs":57863,"unsyncedBytes":398,"inflightPackets":98,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1237/1566 822/947 313/1068","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 762/1392 924/1016 983/983","messages":"456/456 544/544 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":993,"lastDataAt":1794217112643,"silences":0,"longestSilenceMs":1421,"longestRecoveredGapMs":1421},"stats":{"sendRate":2071.1332578391866,"deliveredRate":897.6370579156868,"loss":0.43073508063087007,"srttMs":1870.065763960119,"minRttMs":1345,"sentPackets":475,"sentWireBytes":115568,"receivedPackets":739,"receivedWireBytes":393948,"lastHeardMs":67924,"unsyncedBytes":0,"inflightPackets":97,"duplicateBytes":0}}
- shim trace
09:37:25.526 fetch GET /api/auth/session
09:37:25.532 fetched GET /api/auth/session 200
09:37:26.195 fetch GET /.well-known/t3/environment
09:37:26.196 fetched GET /.well-known/t3/environment 200
09:37:26.215 fetch GET /.well-known/t3/environment
09:37:26.216 fetched GET /.well-known/t3/environment 200
09:37:26.223 rpc 0 subscribeServerConfig
09:37:26.224 chunk 0 38462 bytes
09:37:26.282 in Ack 0
09:37:26.284 rpc 1 subscribeServerLifecycle
09:37:26.286 rpc 2 subscribeServerLifecycle
09:37:26.292 fetch GET /api/auth/session
09:37:26.293 fetched GET /api/auth/session 200
09:37:26.294 rpc 3 subscribeDeviceState
09:37:26.296 rpc 4 preview.reportProfiles
09:37:26.380 rpc 5 subscribeProjectClones
09:37:26.437 fetch GET /api/orchestration/threads/mcp%3A84fe4f69-03ed-43f3-b4f6-9c123805b2e4/bounded
09:37:26.443 fetch GET /api/orchestration/shell
09:37:26.445 fetched GET /api/orchestration/shell 200
09:37:26.449 rpc 6 orchestration.subscribeShell
09:37:26.449 chunk 6 4551 bytes
09:37:26.489 rpc 7 server.refreshProviders
09:37:26.593 rpc 8 orchestration.dispatchCommand thread.visit
09:37:26.593 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:26.645 rpc 9 subscribeTerminalMetadata
09:37:26.646 rpc 10 subscribeVcsStatus
09:37:26.647 rpc 11 subscribePreviewEvents
09:37:26.649 rpc 12 assets.createUrl
09:37:26.652 rpc 13 preview.list
09:37:26.653 rpc 14 projects.readFile
09:37:26.662 in Ack 6
09:37:26.699 chunk 6 4527 bytes
09:37:26.700 in Ack 6
09:37:27.015 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3118e9ac-e0f1-41b9-b4ea-d046d872149f","sessions":[]}}
09:37:27.015 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:27.019 rpc 15 server.reportClientActivity
09:37:27.188 fetch POST /api/observability/v1/traces
09:37:27.189 fetched POST /api/observability/v1/traces 204
09:37:27.200 chunk 1 3815 bytes
09:37:27.201 in Ack 1
09:37:27.369 chunk 2 3815 bytes
09:37:27.370 in Ack 2
09:37:27.577 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:55.948Z","configurableProviders":false,"con
09:37:27.577 chunk 10 1632 bytes
09:37:27.577 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:27.588 in Ack 10
09:37:28.083 chunk 6 4595 bytes
09:37:28.090 in Ack 6
09:37:28.371 fetch POST /api/observability/v1/traces
09:37:28.371 fetched POST /api/observability/v1/traces 204
09:37:28.411 chunk 3 876 bytes
09:37:28.411 exit 4 {"_tag":"Success","value":null}
09:37:28.411 chunk 5 4 bytes
09:37:28.411 chunk 9 36 bytes
09:37:28.411 exit 15 {"_tag":"Success","value":null}
09:37:28.413 in Ack 3
09:37:28.421 rpc 16 preview.reportProfiles
09:37:28.421 in Ack 5
09:37:28.422 in Ack 9
09:37:28.584 chunk 6 4592 bytes
09:37:28.585 in Ack 6
09:37:28.657 exit 16 {"_tag":"Success","value":null}
09:37:28.834 chunk 6 4592 bytes
09:37:28.835 in Ack 6
09:37:29.421 fetch POST /api/observability/v1/traces
09:37:29.421 fetched POST /api/observability/v1/traces 204
09:37:30.665 fetched GET /api/orchestration/threads/mcp%3A84fe4f69-03ed-43f3-b4f6-9c123805b2e4/bounded 200
09:37:30.671 rpc 17 orchestration.subscribeThread
09:37:30.672 chunk 17 8282 bytes
09:37:30.688 in Interrupt 10
09:37:30.691 rpc 18 subscribeVcsStatus
09:37:30.692 in Ack 17
09:37:30.824 chunk 18 1632 bytes
09:37:30.824 in Ack 18
09:37:30.921 chunk 17 8322 bytes
09:37:30.923 in Ack 17
09:37:31.012 rpc 19 server.reportClientActivity
09:37:31.172 chunk 17 11918 bytes
09:37:31.181 in Ack 17
09:37:31.250 exit 19 {"_tag":"Success","value":null}
09:37:31.250 in Ping 
09:37:31.434 chunk 17 13497 bytes
09:37:31.436 in Ack 17
09:37:31.721 fetch POST /api/observability/v1/traces
09:37:31.721 fetched POST /api/observability/v1/traces 204
09:37:31.827 rpc 20 attachments.createUploadUrl
09:37:31.938 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-67921c0b-1bb1-4e0e-b44b-cab1af541d7f","expiresAt":1794217651920,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:31.940 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTY3OTIxYzBiLTFiYjEtNGUwZS1iNDRiLWNhYjFhZjU0MWQ3ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1MTkyMH0.BtygR-DcT4B3jGNByPb09D1xSAgLqVFbQrKsAsoUVGs
09:37:31.971 chunk 17 17976 bytes
09:37:31.973 in Ack 17
09:37:32.222 chunk 17 20408 bytes
09:37:32.226 in Ack 17
09:37:32.239 in Interrupt 18
09:37:32.243 rpc 21 subscribeVcsStatus
09:37:32.339 chunk 21 1632 bytes
09:37:32.339 in Ack 21
09:37:32.475 chunk 17 21099 bytes
09:37:32.476 in Ack 17
09:37:32.513 rpc 22 server.reportClientActivity
09:37:32.838 fetch POST /api/observability/v1/traces
09:37:32.838 fetched POST /api/observability/v1/traces 204
09:37:33.822 exit 22 {"_tag":"Success","value":null}
09:37:34.823 fetch POST /api/observability/v1/traces
09:37:34.823 fetched POST /api/observability/v1/traces 204
09:37:35.380 rpc 23 orchestration.dispatchCommand message.dispatch
09:37:35.847 exit 23 {"_tag":"Success","value":{"sequence":109}}
09:37:35.893 rpc 24 attachments.delete
09:37:36.067 exit 24 {"_tag":"Success","value":null}
09:37:36.186 chunk 6 4678 bytes
09:37:36.187 chunk 17 25368 bytes
09:37:36.187 in Ack 6
09:37:36.259 in Ack 17
09:37:36.264 rpc 25 assets.createUrl
09:37:36.333 in Ping 
09:37:36.377 fetch POST /api/observability/v1/traces
09:37:36.377 fetched POST /api/observability/v1/traces 204
09:37:36.443 chunk 17 27722 bytes
09:37:36.457 in Ack 17
09:37:36.592 rpc 26 orchestration.dispatchCommand thread.visit
09:37:36.592 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:37:36.694 chunk 17 30179 bytes
09:37:36.698 in Ack 17
09:37:36.943 chunk 6 4617 bytes
09:37:36.943 chunk 17 30203 bytes
09:37:36.945 in Ack 6
09:37:36.966 in Ack 17
09:37:37.194 chunk 17 31902 bytes
09:37:37.200 in Ack 17
09:37:37.460 fetch POST /api/observability/v1/traces
09:37:37.460 fetched POST /api/observability/v1/traces 204
09:37:37.764 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/28879d57-7963-4115-a5bb-ecb396fbf60f","expiresAt":1794220657764}}
09:37:38.196 chunk 6 4614 bytes
09:37:38.197 chunk 17 34481 bytes
09:37:38.198 in Ack 6
09:37:38.199 rpc 27 orchestration.dispatchCommand thread.visit
09:37:38.199 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:38.208 in Ack 17
09:37:38.450 chunk 6 4614 bytes
09:37:38.450 chunk 17 35259 bytes
09:37:38.451 in Ack 6
09:37:38.467 in Ack 17
09:37:38.478 in Interrupt 21
09:37:38.482 rpc 28 subscribeVcsStatus
09:37:38.538 chunk 28 1632 bytes
09:37:38.538 in Ack 28
09:37:39.061 rpc 29 server.reportClientActivity
09:37:39.062 fetch POST /api/observability/v1/traces
09:37:39.062 fetched POST /api/observability/v1/traces 204
09:37:39.133 exit 29 {"_tag":"Success","value":null}
09:37:39.281 rpc 30 server.refreshProviders
09:37:39.281 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:36:55.948Z","configurableProviders":false,"con
09:37:40.176 fetch POST /api/observability/v1/traces
09:37:40.176 fetched POST /api/observability/v1/traces 204
09:37:41.335 in Ping 
09:37:41.754 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:42.317 fetch POST /api/observability/v1/traces
09:37:42.317 fetched POST /api/observability/v1/traces 204
09:37:42.578 chunk 6 4678 bytes
09:37:42.580 in Ack 6
09:37:43.581 fetch POST /api/observability/v1/traces
09:37:43.581 fetched POST /api/observability/v1/traces 204
09:37:43.981 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:37:44.689 chunk 6 4722 bytes
09:37:44.690 chunk 17 37280 bytes
09:37:44.690 in Ack 6
09:37:44.704 in Ack 17
09:37:44.941 chunk 17 37864 bytes
09:37:44.948 in Ack 17
09:37:44.985 fetch POST /api/observability/v1/traces
09:37:44.985 fetched POST /api/observability/v1/traces 204
09:37:45.444 chunk 17 38683 bytes
09:37:45.451 in Ack 17
09:37:45.944 chunk 17 39447 bytes
09:37:45.949 in Ack 17
09:37:46.337 in Ping 
09:37:46.444 chunk 17 39447 bytes
09:37:46.447 in Ack 17
09:37:46.467 fetch POST /api/observability/v1/traces
09:37:46.467 fetched POST /api/observability/v1/traces 204
09:37:47.977 chunk 17 39972 bytes
09:37:47.979 in Ack 17
09:37:48.164 fetch POST /api/observability/v1/traces
09:37:48.164 fetched POST /api/observability/v1/traces 204
09:37:48.204 rpc 32 orchestration.dispatchCommand thread.visit
09:37:48.204 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:37:48.733 chunk 6 4722 bytes
09:37:48.734 in Ack 6
09:37:49.209 fetch POST /api/observability/v1/traces
09:37:49.209 fetched POST /api/observability/v1/traces 204
09:37:51.337 in Ping 
09:37:51.453 rpc 33 server.reportClientActivity
09:37:51.989 chunk 17 39974 bytes
09:37:51.992 in Ack 17
09:37:52.326 fetch POST /api/observability/v1/traces
09:37:52.326 fetched POST /api/observability/v1/traces 204
09:37:52.740 chunk 17 39995 bytes
09:37:52.743 in Ack 17
09:37:53.242 chunk 17 40047 bytes
09:37:53.245 in Ack 17
09:37:53.494 chunk 6 4742 bytes
09:37:53.495 in Ack 6
09:37:53.747 fetch POST /api/observability/v1/traces
09:37:53.748 fetched POST /api/observability/v1/traces 204
09:37:54.004 chunk 17 40047 bytes
09:37:54.006 in Ack 17
09:37:54.754 chunk 17 40702 bytes
09:37:54.757 in Ack 17
09:37:55.007 fetch POST /api/observability/v1/traces
09:37:55.007 fetched POST /api/observability/v1/traces 204
09:37:55.047 exit 33 {"_tag":"Success","value":null}
09:37:56.007 chunk 17 40724 bytes
09:37:56.009 in Ack 17
09:37:56.049 fetch POST /api/observability/v1/traces
09:37:56.049 fetched POST /api/observability/v1/traces 204
09:37:56.338 in Ping 
09:37:57.612 chunk 6 4742 bytes
09:37:57.613 in Ack 6
09:37:58.160 fetch POST /api/observability/v1/traces
09:37:58.160 fetched POST /api/observability/v1/traces 204
09:37:58.208 rpc 34 orchestration.dispatchCommand thread.visit
09:37:58.208 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:37:59.208 fetch POST /api/observability/v1/traces
09:37:59.208 fetched POST /api/observability/v1/traces 204
09:38:00.828 chunk 6 4742 bytes
09:38:00.830 in Ack 6
09:38:01.080 chunk 17 40724 bytes
09:38:01.082 in Ack 17
09:38:01.338 in Ping 
09:38:01.647 chunk 17 43517 bytes
09:38:01.650 in Ack 17
09:38:01.832 fetch POST /api/observability/v1/traces
09:38:01.832 fetched POST /api/observability/v1/traces 204
09:38:01.896 chunk 17 43843 bytes
09:38:01.901 in Ack 17
09:38:02.728 chunk 17 43897 bytes
09:38:02.732 in Ack 17
09:38:02.903 fetch POST /api/observability/v1/traces
09:38:02.903 fetched POST /api/observability/v1/traces 204
09:38:03.508 chunk 17 44137 bytes
09:38:03.510 in Ack 17
09:38:04.512 fetch POST /api/observability/v1/traces
09:38:04.512 fetched POST /api/observability/v1/traces 204
09:38:04.630 chunk 6 4742 bytes
09:38:04.630 in Ack 6
09:38:04.881 chunk 17 45181 bytes
09:38:04.883 in Ack 17
09:38:05.477 chunk 17 45133 bytes
09:38:05.480 in Ack 17
09:38:05.633 fetch POST /api/observability/v1/traces
09:38:05.633 fetched POST /api/observability/v1/traces 204
09:38:06.338 in Ping 
09:38:08.169 fetch POST /api/observability/v1/traces
09:38:08.169 fetched POST /api/observability/v1/traces 204
09:38:08.210 rpc 35 orchestration.dispatchCommand thread.visit
09:38:08.210 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:38:09.210 fetch POST /api/observability/v1/traces
09:38:09.210 fetched POST /api/observability/v1/traces 204
09:38:11.339 in Ping 
09:38:13.226 chunk 6 4742 bytes
09:38:13.239 in Ack 6
09:38:13.988 chunk 17 45133 bytes
09:38:13.992 in Ack 17
09:38:14.240 fetch POST /api/observability/v1/traces
09:38:14.240 fetched POST /api/observability/v1/traces 204
09:38:16.002 chunk 6 4744 bytes
09:38:16.003 in Ack 6
09:38:16.339 in Ping 
09:38:16.448 rpc 36 server.reportClientActivity
09:38:17.004 fetch POST /api/observability/v1/traces
09:38:17.004 fetched POST /api/observability/v1/traces 204
09:38:18.141 chunk 6 4744 bytes
09:38:18.144 in Ack 6
09:38:18.177 fetch POST /api/observability/v1/traces
09:38:18.177 fetched POST /api/observability/v1/traces 204
09:38:19.146 chunk 17 47665 bytes
09:38:19.151 in Ack 17
09:38:19.738 chunk 17 48667 bytes
09:38:19.751 in Ack 17
09:38:20.153 fetch POST /api/observability/v1/traces
09:38:20.153 fetched POST /api/observability/v1/traces 204
09:38:20.538 chunk 17 50735 bytes
09:38:20.542 in Ack 17
09:38:20.788 chunk 17 50785 bytes
09:38:20.790 in Ack 17
09:38:21.289 chunk 17 50786 bytes
09:38:21.292 in Ack 17
09:38:21.342 in Ping 
09:38:21.543 fetch POST /api/observability/v1/traces
09:38:21.543 fetched POST /api/observability/v1/traces 204
09:38:22.095 chunk 17 54274 bytes
09:38:22.107 in Ack 17
09:38:22.429 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:23.109 fetch POST /api/observability/v1/traces
09:38:23.109 fetched POST /api/observability/v1/traces 204
09:38:23.447 chunk 6 5051 bytes
09:38:23.448 in Ack 6
09:38:23.697 chunk 17 55234 bytes
09:38:23.700 in Ack 17
09:38:23.716 exit 36 {"_tag":"Success","value":null}
09:38:24.450 fetch POST /api/observability/v1/traces
09:38:24.450 fetched POST /api/observability/v1/traces 204
09:38:24.476 exit 37 {"_tag":"Success","value":{"sequence":196}}
09:38:24.478 rpc 38 orchestration.dispatchCommand thread.visit
09:38:24.478 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:38:25.012 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:38:25.479 fetch POST /api/observability/v1/traces
09:38:25.479 fetched POST /api/observability/v1/traces 204
09:38:26.343 in Ping 
09:38:26.406 chunk 6 4746 bytes
09:38:26.406 in Ack 6
09:38:27.409 fetch POST /api/observability/v1/traces
09:38:27.409 fetched POST /api/observability/v1/traces 204
09:38:27.679 chunk 17 55277 bytes
09:38:27.683 in Ack 17
09:38:28.180 chunk 17 55277 bytes
09:38:28.184 in Ack 17
09:38:28.684 fetch POST /api/observability/v1/traces
09:38:28.684 fetched POST /api/observability/v1/traces 204
09:38:30.293 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:30.433 chunk 6 4746 bytes
09:38:30.434 in Ack 6
09:38:31.304 fetch POST /api/observability/v1/traces
09:38:31.305 fetched POST /api/observability/v1/traces 204
09:38:31.347 in Ping 
09:38:31.455 chunk 17 55301 bytes
09:38:31.457 in Ack 17
09:38:32.224 chunk 17 58537 bytes
09:38:32.228 in Ack 17
09:38:32.475 fetch POST /api/observability/v1/traces
09:38:32.475 fetched POST /api/observability/v1/traces 204
09:38:32.727 chunk 6 4746 bytes
09:38:32.728 in Ack 6
09:38:32.978 chunk 17 59492 bytes
09:38:32.980 in Ack 17
09:38:33.458 rpc 40 orchestration.dispatchCommand thread.visit
09:38:33.458 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:38:33.729 fetch POST /api/observability/v1/traces
09:38:33.729 fetched POST /api/observability/v1/traces 204
09:38:35.312 chunk 17 59492 bytes
09:38:35.314 in Ack 17
09:38:35.561 chunk 17 59492 bytes
09:38:35.563 in Ack 17
09:38:36.315 fetch POST /api/observability/v1/traces
09:38:36.315 fetched POST /api/observability/v1/traces 204
09:38:36.351 in Ping 
09:38:38.188 fetch POST /api/observability/v1/traces
09:38:38.188 fetched POST /api/observability/v1/traces 204
09:38:38.610 chunk 17 60509 bytes
09:38:38.615 in Ack 17
09:38:39.617 fetch POST /api/observability/v1/traces
09:38:39.617 fetched POST /api/observability/v1/traces 204
