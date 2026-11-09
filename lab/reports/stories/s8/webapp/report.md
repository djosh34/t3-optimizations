# story, webapp, warm start, seed 8

Profile story (graded): seed 8's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 56 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.4s |
| Prompt reached the agent | 2.5s |
| Time to first text | 14.2s |
| Whole reply shown | 54.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.4s / 6.5s / 18.4s |
| Longest silence | 19.4s |
| Approval shown after it was asked | 5.9s |
| Bytes on the wire down / up | 237173 / 103430 |
| Bytes the app received / sent | 179107 / 53367 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.4 | 0 | 299 |
| 20 | 7.4 | 397 | 697 |
| 30 | 17.4 | 397 | 1077 |
| 40 | 3.5 | 1077 | 1217 |
| 50 | 5.8 | 1378 | 1579 |
| 54 | 9.8 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217056067,"silences":0,"longestSilenceMs":1044,"longestRecoveredGapMs":1044},"stats":{"sendRate":4500,"deliveredRate":1673.0589335827876,"loss":0,"srttMs":11.066446171863134,"minRttMs":1,"sentPackets":138,"sentWireBytes":39153,"receivedPackets":140,"receivedWireBytes":107647,"lastHeardMs":13511,"unsyncedBytes":735,"inflightPackets":16,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217066092,"silences":0,"longestSilenceMs":1044,"longestRecoveredGapMs":1044},"stats":{"sendRate":1136.848306543177,"deliveredRate":1262.2727272727273,"loss":0,"srttMs":1922.2447138047667,"minRttMs":1771,"sentPackets":190,"sentWireBytes":51798,"receivedPackets":261,"receivedWireBytes":164957,"lastHeardMs":23536,"unsyncedBytes":0,"inflightPackets":5,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217075820,"silences":0,"longestSilenceMs":1044,"longestRecoveredGapMs":1044},"stats":{"sendRate":1558.2973473164716,"deliveredRate":1333.6925568837144,"loss":0,"srttMs":2277.7029098983267,"minRttMs":1771,"sentPackets":247,"sentWireBytes":63107,"receivedPackets":374,"receivedWireBytes":217848,"lastHeardMs":33264,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217085247,"silences":1,"longestSilenceMs":2015,"longestRecoveredGapMs":2015},"stats":{"sendRate":2000.5388353255717,"deliveredRate":773.5432071733817,"loss":0.193,"srttMs":2277.7029098983267,"minRttMs":1771,"sentPackets":266,"sentWireBytes":65549,"receivedPackets":412,"receivedWireBytes":227466,"lastHeardMs":42691,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 843/1093 1177/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217095966,"silences":2,"longestSilenceMs":3278,"longestRecoveredGapMs":3278},"stats":{"sendRate":2000,"deliveredRate":1181.8261221619787,"loss":0,"srttMs":2123.8723474373937,"minRttMs":1771,"sentPackets":308,"sentWireBytes":78232,"receivedPackets":458,"receivedWireBytes":250286,"lastHeardMs":53410,"unsyncedBytes":456,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1298/1566 951/1068 221/947","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":688,"lastDataAt":1794217107230,"silences":2,"longestSilenceMs":3278,"longestRecoveredGapMs":3278},"stats":{"sendRate":3000,"deliveredRate":1447.2049689440994,"loss":0,"srttMs":2280.7987832812532,"minRttMs":1770,"sentPackets":351,"sentWireBytes":86721,"receivedPackets":495,"receivedWireBytes":265350,"lastHeardMs":64674,"unsyncedBytes":0,"inflightPackets":13,"duplicateBytes":0}}
- shim trace
09:37:23.327 fetch GET /api/auth/session
09:37:23.329 fetched GET /api/auth/session 200
09:37:23.653 fetch GET /.well-known/t3/environment
09:37:23.656 fetched GET /.well-known/t3/environment 200
09:37:23.674 fetch GET /.well-known/t3/environment
09:37:23.675 fetched GET /.well-known/t3/environment 200
09:37:23.680 rpc 0 subscribeServerConfig
09:37:23.680 chunk 0 38458 bytes
09:37:23.720 in Ack 0
09:37:23.723 rpc 1 subscribeServerLifecycle
09:37:23.723 rpc 2 subscribeServerLifecycle
09:37:23.726 fetch GET /api/auth/session
09:37:23.728 fetched GET /api/auth/session 200
09:37:23.728 rpc 3 subscribeDeviceState
09:37:23.730 rpc 4 preview.reportProfiles
09:37:23.806 rpc 5 subscribeProjectClones
09:37:23.843 fetch GET /api/orchestration/threads/mcp%3A2ef08d57-c930-4d60-ad55-77f2efd428fd/bounded
09:37:23.870 fetch GET /api/orchestration/shell
09:37:23.872 fetched GET /api/orchestration/shell 200
09:37:23.875 rpc 6 orchestration.subscribeShell
09:37:23.875 chunk 6 2793 bytes
09:37:23.878 in Ack 6
09:37:23.924 rpc 7 server.reportClientActivity
09:37:24.336 chunk 1 3814 bytes
09:37:24.337 in Ack 1
09:37:24.530 chunk 6 2766 bytes
09:37:24.530 chunk 2 3814 bytes
09:37:24.531 in Ack 6
09:37:24.532 in Ack 2
09:37:24.645 fetch POST /api/observability/v1/traces
09:37:24.645 fetched POST /api/observability/v1/traces 204
09:37:24.780 chunk 6 4591 bytes
09:37:24.781 in Ack 6
09:37:24.809 rpc 8 server.refreshProviders
09:37:24.853 rpc 9 orchestration.dispatchCommand thread.visit
09:37:24.853 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:24.867 rpc 10 subscribeTerminalMetadata
09:37:24.867 rpc 11 subscribeVcsStatus
09:37:24.868 rpc 12 subscribePreviewEvents
09:37:24.869 rpc 13 assets.createUrl
09:37:24.869 rpc 14 preview.list
09:37:24.870 rpc 15 projects.readFile
09:37:24.975 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"71a3a89d-0aec-4f37-a9d7-5c65a14e806c","sessions":[]}}
09:37:24.975 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:24.975 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:01.185Z","configurableProviders":false,"con
09:37:25.110 chunk 11 1632 bytes
09:37:25.110 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:25.111 in Ack 11
09:37:25.521 chunk 3 876 bytes
09:37:25.521 exit 4 {"_tag":"Success","value":null}
09:37:25.521 chunk 5 4 bytes
09:37:25.521 exit 7 {"_tag":"Success","value":null}
09:37:25.521 chunk 10 36 bytes
09:37:25.523 in Ack 3
09:37:25.530 rpc 16 preview.reportProfiles
09:37:25.530 in Ack 5
09:37:25.531 in Ack 10
09:37:25.536 chunk 6 4634 bytes
09:37:25.536 in Ack 6
09:37:25.597 exit 16 {"_tag":"Success","value":null}
09:37:25.782 fetch POST /api/observability/v1/traces
09:37:25.782 fetched POST /api/observability/v1/traces 204
09:37:25.783 rpc 17 server.reportClientActivity
09:37:25.838 exit 17 {"_tag":"Success","value":null}
09:37:26.765 fetched GET /api/orchestration/threads/mcp%3A2ef08d57-c930-4d60-ad55-77f2efd428fd/bounded 200
09:37:26.769 rpc 18 orchestration.subscribeThread
09:37:26.770 chunk 18 8283 bytes
09:37:26.783 in Interrupt 11
09:37:26.785 rpc 19 subscribeVcsStatus
09:37:26.787 in Ack 18
09:37:26.840 fetch POST /api/observability/v1/traces
09:37:26.840 fetched POST /api/observability/v1/traces 204
09:37:26.926 chunk 19 1632 bytes
09:37:26.927 in Ack 19
09:37:27.019 chunk 18 8323 bytes
09:37:27.020 in Ack 18
09:37:27.064 rpc 20 server.reportClientActivity
09:37:27.270 chunk 18 11919 bytes
09:37:27.271 in Ack 18
09:37:27.322 exit 20 {"_tag":"Success","value":null}
09:37:27.566 chunk 18 13498 bytes
09:37:27.568 in Ack 18
09:37:27.904 chunk 18 17976 bytes
09:37:27.907 in Ack 18
09:37:27.997 rpc 21 attachments.createUploadUrl
09:37:28.059 fetch POST /api/observability/v1/traces
09:37:28.059 fetched POST /api/observability/v1/traces 204
09:37:28.100 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-10c40ba3-c30f-493e-9ab0-3d9d8402e8b1","expiresAt":1794217648082,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:28.103 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTEwYzQwYmEzLWMzMGYtNDkzZS05YWIwLTNkOWQ4NDAyZThiMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY0ODA4Mn0.7uxsPA-EeGtIoo5H-nzFGahUfyD2gCtnNKltAQ63k-k
09:37:28.154 chunk 18 19019 bytes
09:37:28.157 in Ack 18
09:37:28.404 chunk 18 21098 bytes
09:37:28.406 in Ack 18
09:37:28.419 in Interrupt 19
09:37:28.426 rpc 22 subscribeVcsStatus
09:37:28.492 chunk 22 1632 bytes
09:37:28.493 in Ack 22
09:37:28.679 rpc 23 server.reportClientActivity
09:37:28.680 in Ping 
09:37:29.102 fetch POST /api/observability/v1/traces
09:37:29.102 fetched POST /api/observability/v1/traces 204
09:37:29.237 exit 23 {"_tag":"Success","value":null}
09:37:30.246 fetch POST /api/observability/v1/traces
09:37:30.246 fetched POST /api/observability/v1/traces 204
09:37:31.155 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:31.432 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:31.435 rpc 25 attachments.delete
09:37:31.596 exit 25 {"_tag":"Success","value":null}
09:37:31.700 chunk 6 4720 bytes
09:37:31.700 chunk 18 25367 bytes
09:37:31.701 in Ack 6
09:37:31.760 in Ack 18
09:37:31.763 rpc 26 assets.createUrl
09:37:31.950 chunk 18 26810 bytes
09:37:31.952 in Ack 18
09:37:32.154 fetch POST /api/observability/v1/traces
09:37:32.154 fetched POST /api/observability/v1/traces 204
09:37:32.200 chunk 18 30178 bytes
09:37:32.202 in Ack 18
09:37:32.450 chunk 6 4659 bytes
09:37:32.450 chunk 18 30318 bytes
09:37:32.460 in Ack 6
09:37:32.487 in Ack 18
09:37:32.700 chunk 18 31901 bytes
09:37:32.701 in Ack 18
09:37:32.951 chunk 18 32033 bytes
09:37:32.953 in Ack 18
09:37:33.205 chunk 6 4656 bytes
09:37:33.205 chunk 18 35258 bytes
09:37:33.206 fetch POST /api/observability/v1/traces
09:37:33.206 fetched POST /api/observability/v1/traces 204
09:37:33.207 in Ack 6
09:37:33.210 rpc 27 orchestration.dispatchCommand thread.visit
09:37:33.210 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:33.220 in Ack 18
09:37:33.243 in Interrupt 22
09:37:33.252 rpc 28 subscribeVcsStatus
09:37:33.345 chunk 28 1713 bytes
09:37:33.346 in Ack 28
09:37:33.489 chunk 6 4656 bytes
09:37:33.489 chunk 18 35258 bytes
09:37:33.491 in Ack 6
09:37:33.508 in Ack 18
09:37:33.796 rpc 29 server.reportClientActivity
09:37:33.796 in Ping 
09:37:33.843 exit 29 {"_tag":"Success","value":null}
09:37:34.226 fetch POST /api/observability/v1/traces
09:37:34.226 fetched POST /api/observability/v1/traces 204
09:37:34.960 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/56158a45-6232-4f4d-9189-8508e3cf21c2","expiresAt":1794220654960}}
09:37:35.018 rpc 30 server.refreshProviders
09:37:35.018 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:01.185Z","configurableProviders":false,"con
09:37:35.437 fetch POST /api/observability/v1/traces
09:37:35.437 fetched POST /api/observability/v1/traces 204
09:37:36.021 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:37.042 fetch POST /api/observability/v1/traces
09:37:37.042 fetched POST /api/observability/v1/traces 204
09:37:37.555 chunk 6 4700 bytes
09:37:37.556 in Ack 6
09:37:37.808 chunk 6 4720 bytes
09:37:37.809 in Ack 6
09:37:38.340 fetch POST /api/observability/v1/traces
09:37:38.340 fetched POST /api/observability/v1/traces 204
09:37:38.798 in Ping 
09:37:39.195 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:37:40.013 chunk 18 35894 bytes
09:37:40.015 in Ack 18
09:37:40.199 fetch POST /api/observability/v1/traces
09:37:40.200 fetched POST /api/observability/v1/traces 204
09:37:40.264 chunk 18 36734 bytes
09:37:40.267 in Ack 18
09:37:40.765 chunk 18 37389 bytes
09:37:40.766 in Ack 18
09:37:41.022 chunk 18 38175 bytes
09:37:41.024 in Ack 18
09:37:41.273 fetch POST /api/observability/v1/traces
09:37:41.273 fetched POST /api/observability/v1/traces 204
09:37:41.277 chunk 18 38702 bytes
09:37:41.280 in Ack 18
09:37:41.527 chunk 18 38702 bytes
09:37:41.529 in Ack 18
09:37:41.774 chunk 6 4720 bytes
09:37:41.775 in Ack 6
09:37:42.275 chunk 6 4784 bytes
09:37:42.276 in Ack 6
09:37:42.284 fetch POST /api/observability/v1/traces
09:37:42.284 fetched POST /api/observability/v1/traces 204
09:37:42.285 chunk 18 40723 bytes
09:37:42.286 in Ack 18
09:37:43.210 rpc 32 orchestration.dispatchCommand thread.visit
09:37:43.210 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:37:43.288 fetch POST /api/observability/v1/traces
09:37:43.288 fetched POST /api/observability/v1/traces 204
09:37:43.798 in Ping 
09:37:45.403 fetch POST /api/observability/v1/traces
09:37:45.403 fetched POST /api/observability/v1/traces 204
09:37:45.853 chunk 6 4784 bytes
09:37:45.853 in Ack 6
09:37:46.111 chunk 18 40723 bytes
09:37:46.113 in Ack 18
09:37:46.855 fetch POST /api/observability/v1/traces
09:37:46.855 fetched POST /api/observability/v1/traces 204
09:37:47.695 chunk 6 4784 bytes
09:37:47.696 in Ack 6
09:37:48.346 fetch POST /api/observability/v1/traces
09:37:48.346 fetched POST /api/observability/v1/traces 204
09:37:48.798 in Ping 
09:37:48.906 rpc 33 server.reportClientActivity
09:37:49.602 chunk 18 43726 bytes
09:37:49.604 in Ack 18
09:37:49.873 chunk 18 44136 bytes
09:37:49.876 in Ack 18
09:37:49.907 fetch POST /api/observability/v1/traces
09:37:49.907 fetched POST /api/observability/v1/traces 204
09:37:50.120 chunk 18 44088 bytes
09:37:50.124 in Ack 18
09:37:50.370 chunk 6 4784 bytes
09:37:50.371 in Ack 6
09:37:50.620 chunk 6 4784 bytes
09:37:50.621 in Ack 6
09:37:50.623 chunk 18 45132 bytes
09:37:50.625 in Ack 18
09:37:51.126 fetch POST /api/observability/v1/traces
09:37:51.126 fetched POST /api/observability/v1/traces 204
09:37:51.387 exit 33 {"_tag":"Success","value":null}
09:37:52.388 fetch POST /api/observability/v1/traces
09:37:52.389 fetched POST /api/observability/v1/traces 204
09:37:53.211 rpc 34 orchestration.dispatchCommand thread.visit
09:37:53.211 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:37:53.798 in Ping 
09:37:54.212 fetch POST /api/observability/v1/traces
09:37:54.212 fetched POST /api/observability/v1/traces 204
09:37:55.344 chunk 18 45132 bytes
09:37:55.345 in Ack 18
09:37:55.406 fetch POST /api/observability/v1/traces
09:37:55.406 fetched POST /api/observability/v1/traces 204
09:37:55.842 chunk 6 4784 bytes
09:37:55.843 in Ack 6
09:37:56.844 fetch POST /api/observability/v1/traces
09:37:56.844 fetched POST /api/observability/v1/traces 204
09:37:58.799 in Ping 
09:38:03.799 in Ping 
09:38:05.410 fetch POST /api/observability/v1/traces
09:38:05.410 fetched POST /api/observability/v1/traces 204
09:38:08.800 in Ping 
09:38:09.034 chunk 18 45735 bytes
09:38:09.039 in Ack 18
09:38:09.285 chunk 18 46554 bytes
09:38:09.287 in Ack 18
09:38:09.572 chunk 18 47136 bytes
09:38:09.575 in Ack 18
09:38:09.822 chunk 18 47190 bytes
09:38:09.825 in Ack 18
09:38:10.040 fetch POST /api/observability/v1/traces
09:38:10.040 fetched POST /api/observability/v1/traces 204
09:38:10.074 chunk 18 47664 bytes
09:38:10.078 in Ack 18
09:38:10.997 chunk 18 48666 bytes
09:38:11.000 in Ack 18
09:38:11.097 fetch POST /api/observability/v1/traces
09:38:11.097 fetched POST /api/observability/v1/traces 204
09:38:11.270 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:38:12.269 fetch POST /api/observability/v1/traces
09:38:12.269 fetched POST /api/observability/v1/traces 204
09:38:13.407 exit 35 {"_tag":"Success","value":{"sequence":187}}
09:38:13.800 in Ping 
09:38:13.848 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:13.906 rpc 37 server.reportClientActivity
09:38:14.408 fetch POST /api/observability/v1/traces
09:38:14.408 fetched POST /api/observability/v1/traces 204
09:38:15.967 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:38:16.017 chunk 18 48709 bytes
09:38:16.019 in Ack 18
09:38:16.974 fetch POST /api/observability/v1/traces
09:38:16.974 fetched POST /api/observability/v1/traces 204
09:38:17.179 chunk 18 48759 bytes
09:38:17.181 in Ack 18
09:38:18.182 fetch POST /api/observability/v1/traces
09:38:18.182 fetched POST /api/observability/v1/traces 204
09:38:18.804 in Ping 
09:38:20.239 chunk 6 4786 bytes
09:38:20.240 in Ack 6
09:38:20.242 rpc 38 orchestration.dispatchCommand thread.visit
09:38:20.242 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:38:21.241 chunk 18 51754 bytes
09:38:21.243 fetch POST /api/observability/v1/traces
09:38:21.243 fetched POST /api/observability/v1/traces 204
09:38:21.245 in Ack 18
09:38:21.391 exit 37 {"_tag":"Success","value":null}
09:38:21.490 chunk 18 51994 bytes
09:38:21.492 in Ack 18
09:38:22.246 fetch POST /api/observability/v1/traces
09:38:22.246 fetched POST /api/observability/v1/traces 204
09:38:23.808 in Ping 
09:38:28.426 chunk 18 52256 bytes
09:38:28.430 in Ack 18
09:38:28.926 in Ping 
09:38:29.436 fetch POST /api/observability/v1/traces
09:38:29.436 fetched POST /api/observability/v1/traces 204
09:38:29.485 chunk 18 52672 bytes
09:38:29.490 in Ack 18
