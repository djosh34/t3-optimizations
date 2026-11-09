# story, webapp, warm start, seed 30

Profile story (graded): seed 30's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 53 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.2s |
| Prompt reached the agent | 3.9s |
| Time to first text | 17.4s |
| Whole reply shown | 51.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.0s / 6.0s / 15.0s |
| Longest silence | 17.4s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 547391 / 245057 |
| Bytes the app received / sent | 307267 / 85143 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.0 | 0 | 247 |
| 20 | 6.0 | 397 | 643 |
| 30 | 16.0 | 397 | 1049 |
| 40 | 5.8 | 1077 | 1309 |
| 50 | 8.0 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1066,"lastDataAt":1794216521031,"silences":0,"longestSilenceMs":1066,"longestRecoveredGapMs":1066},"stats":{"sendRate":5856.380648644489,"deliveredRate":3470.8867006410374,"loss":0,"srttMs":8.440166176839888,"minRttMs":1,"sentPackets":142,"sentWireBytes":40292,"receivedPackets":161,"receivedWireBytes":106773,"lastHeardMs":13525,"unsyncedBytes":736,"inflightPackets":20,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1066,"lastDataAt":1794216532916,"silences":1,"longestSilenceMs":2424,"longestRecoveredGapMs":2424},"stats":{"sendRate":1333.801335172421,"deliveredRate":2124.4397674140396,"loss":0.13526923076923075,"srttMs":2370.6274119194354,"minRttMs":1,"sentPackets":182,"sentWireBytes":53200,"receivedPackets":273,"receivedWireBytes":168435,"lastHeardMs":25411,"unsyncedBytes":0,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 65/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 111/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1066,"lastDataAt":1794216543100,"silences":1,"longestSilenceMs":2424,"longestRecoveredGapMs":2424},"stats":{"sendRate":1333.801335172421,"deliveredRate":834.7676419965577,"loss":0.29032258064516125,"srttMs":3704.5086874359977,"minRttMs":3407,"sentPackets":236,"sentWireBytes":66775,"receivedPackets":335,"receivedWireBytes":203304,"lastHeardMs":35595,"unsyncedBytes":167,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1066,"lastDataAt":1794216553207,"silences":1,"longestSilenceMs":2424,"longestRecoveredGapMs":2424},"stats":{"sendRate":2028.9706517047905,"deliveredRate":1248.597324126025,"loss":0.07692307692307687,"srttMs":1941.7726187140047,"minRttMs":1403,"sentPackets":289,"sentWireBytes":76827,"receivedPackets":454,"receivedWireBytes":257509,"lastHeardMs":45701,"unsyncedBytes":0,"inflightPackets":18,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1392/1392 983/983 750/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1066,"lastDataAt":1794216563535,"silences":1,"longestSilenceMs":2424,"longestRecoveredGapMs":2424},"stats":{"sendRate":3000,"deliveredRate":1796.512337953593,"loss":0.15765873015873016,"srttMs":1652.81781728956,"minRttMs":1403,"sentPackets":358,"sentWireBytes":95830,"receivedPackets":555,"receivedWireBytes":319416,"lastHeardMs":56029,"unsyncedBytes":0,"inflightPackets":34,"duplicateBytes":0}}
- shim trace
09:28:28.942 fetch GET /api/auth/session
09:28:28.945 fetched GET /api/auth/session 200
09:28:29.405 fetch GET /.well-known/t3/environment
09:28:29.411 fetched GET /.well-known/t3/environment 200
09:28:29.452 fetch GET /.well-known/t3/environment
09:28:29.453 fetched GET /.well-known/t3/environment 200
09:28:29.465 rpc 0 subscribeServerConfig
09:28:29.468 chunk 0 38462 bytes
09:28:29.606 in Ack 0
09:28:29.614 rpc 1 subscribeServerLifecycle
09:28:29.617 rpc 2 subscribeServerLifecycle
09:28:29.626 fetch GET /api/auth/session
09:28:29.640 fetched GET /api/auth/session 200
09:28:29.641 rpc 3 subscribeDeviceState
09:28:29.653 rpc 4 preview.reportProfiles
09:28:29.808 rpc 5 server.reportClientActivity
09:28:30.236 rpc 6 subscribeProjectClones
09:28:30.303 fetch GET /api/orchestration/threads/mcp%3A962ab4b0-35c8-4f3c-9afa-bdabbef21602/bounded
09:28:30.314 fetch GET /api/orchestration/shell
09:28:30.370 rpc 7 server.refreshProviders
09:28:30.493 rpc 8 orchestration.dispatchCommand thread.visit
09:28:30.493 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:28:30.544 fetched GET /api/orchestration/shell 200
09:28:30.544 rpc 9 subscribeTerminalMetadata
09:28:30.546 rpc 10 subscribeVcsStatus
09:28:30.547 rpc 11 subscribePreviewEvents
09:28:30.551 rpc 12 orchestration.subscribeShell
09:28:30.551 chunk 12 4594 bytes
09:28:30.553 rpc 13 assets.createUrl
09:28:30.554 rpc 14 preview.list
09:28:30.559 rpc 15 projects.readFile
09:28:30.562 fetch POST /api/observability/v1/traces
09:28:30.562 fetched POST /api/observability/v1/traces 204
09:28:30.564 in Ack 12
09:28:30.857 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"45bc7b8f-c0b5-492d-a63d-65b20df851e4","sessions":[]}}
09:28:30.930 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:30.930 chunk 1 3815 bytes
09:28:30.932 in Ack 1
09:28:31.357 chunk 2 3815 bytes
09:28:31.357 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:28:00.176Z","configurableProviders":false,"con
09:28:31.357 chunk 10 1632 bytes
09:28:31.361 in Ack 2
09:28:31.370 in Ack 10
09:28:31.564 fetch POST /api/observability/v1/traces
09:28:31.564 fetched POST /api/observability/v1/traces 204
09:28:31.568 chunk 12 4570 bytes
09:28:31.568 exit 13 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:31.569 in Ack 12
09:28:31.819 chunk 12 4635 bytes
09:28:31.820 in Ack 12
09:28:32.047 chunk 3 876 bytes
09:28:32.047 exit 4 {"_tag":"Success","value":null}
09:28:32.047 exit 5 {"_tag":"Success","value":null}
09:28:32.047 chunk 6 4 bytes
09:28:32.047 chunk 9 36 bytes
09:28:32.048 in Ack 3
09:28:32.054 rpc 16 preview.reportProfiles
09:28:32.054 in Ack 6
09:28:32.055 in Ack 9
09:28:32.146 exit 16 {"_tag":"Success","value":null}
09:28:32.306 rpc 17 server.reportClientActivity
09:28:32.391 exit 17 {"_tag":"Success","value":null}
09:28:32.570 fetch POST /api/observability/v1/traces
09:28:32.570 fetched POST /api/observability/v1/traces 204
09:28:33.018 fetched GET /api/orchestration/threads/mcp%3A962ab4b0-35c8-4f3c-9afa-bdabbef21602/bounded 200
09:28:33.023 rpc 18 orchestration.subscribeThread
09:28:33.023 chunk 18 8283 bytes
09:28:33.041 in Interrupt 10
09:28:33.044 rpc 19 subscribeVcsStatus
09:28:33.046 in Ack 18
09:28:33.177 chunk 19 1632 bytes
09:28:33.178 in Ack 19
09:28:33.275 chunk 18 8323 bytes
09:28:33.276 in Ack 18
09:28:33.328 rpc 20 server.reportClientActivity
09:28:33.475 exit 20 {"_tag":"Success","value":null}
09:28:33.533 chunk 18 12691 bytes
09:28:33.534 in Ack 18
09:28:33.784 chunk 18 13498 bytes
09:28:33.785 in Ack 18
09:28:34.027 fetch POST /api/observability/v1/traces
09:28:34.027 fetched POST /api/observability/v1/traces 204
09:28:34.034 chunk 18 17167 bytes
09:28:34.036 in Ack 18
09:28:34.157 rpc 21 attachments.createUploadUrl
09:28:34.284 chunk 18 19020 bytes
09:28:34.286 in Ack 18
09:28:34.338 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-430b20d8-374a-4a9c-be7c-96a75259f363","expiresAt":1794217114317,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:34.342 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTQzMGIyMGQ4LTM3NGEtNGE5Yy1iZTdjLTk2YTc1MjU5ZjM2MyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzExNDMxN30.WZOfUyMfW6s_KPxOIK6B-LU2tbZJkfb2WgGksOymxzo
09:28:34.458 in Ping 
09:28:34.535 chunk 18 21100 bytes
09:28:34.536 in Ack 18
09:28:34.553 in Interrupt 19
09:28:34.555 rpc 22 subscribeVcsStatus
09:28:34.644 chunk 22 1632 bytes
09:28:34.645 in Ack 22
09:28:34.805 rpc 23 server.reportClientActivity
09:28:35.038 fetch POST /api/observability/v1/traces
09:28:35.038 fetched POST /api/observability/v1/traces 204
09:28:37.174 exit 23 {"_tag":"Success","value":null}
09:28:37.993 rpc 24 orchestration.dispatchCommand message.dispatch
09:28:38.386 fetch POST /api/observability/v1/traces
09:28:38.387 fetched POST /api/observability/v1/traces 204
09:28:38.439 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:28:38.444 rpc 25 attachments.delete
09:28:38.704 chunk 12 4721 bytes
09:28:38.704 chunk 18 21736 bytes
09:28:38.705 in Ack 12
09:28:38.751 in Ack 18
09:28:38.763 exit 25 {"_tag":"Success","value":null}
09:28:38.955 chunk 18 26810 bytes
09:28:38.958 in Ack 18
09:28:38.963 rpc 26 assets.createUrl
09:28:39.207 chunk 18 29269 bytes
09:28:39.209 in Ack 18
09:28:39.440 fetch POST /api/observability/v1/traces
09:28:39.440 fetched POST /api/observability/v1/traces 204
09:28:39.457 chunk 12 4660 bytes
09:28:39.458 chunk 18 29293 bytes
09:28:39.459 in Ack 12
09:28:39.467 in Ping 
09:28:39.469 in Ack 18
09:28:39.711 chunk 18 30107 bytes
09:28:39.732 in Ack 18
09:28:39.961 chunk 12 4657 bytes
09:28:39.961 chunk 18 32554 bytes
09:28:39.962 in Ack 12
09:28:39.966 rpc 27 orchestration.dispatchCommand thread.visit
09:28:39.966 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:28:39.984 in Ack 18
09:28:40.211 chunk 12 4657 bytes
09:28:40.212 chunk 18 35260 bytes
09:28:40.214 in Ack 12
09:28:40.234 in Ack 18
09:28:40.252 in Interrupt 22
09:28:40.256 rpc 28 subscribeVcsStatus
09:28:40.321 chunk 28 1713 bytes
09:28:40.321 in Ack 28
09:28:40.460 fetch POST /api/observability/v1/traces
09:28:40.461 fetched POST /api/observability/v1/traces 204
09:28:40.464 chunk 18 35260 bytes
09:28:40.466 in Ack 18
09:28:40.509 rpc 29 server.reportClientActivity
09:28:41.032 exit 29 {"_tag":"Success","value":null}
09:28:41.383 rpc 30 server.refreshProviders
09:28:41.383 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:28:00.176Z","configurableProviders":false,"con
09:28:41.496 fetch POST /api/observability/v1/traces
09:28:41.496 fetched POST /api/observability/v1/traces 204
09:28:42.843 rpc 31 orchestration.dispatchCommand message.dispatch
09:28:43.574 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/ba7f5a2c-4918-4a64-8140-842626260f56","expiresAt":1794220123574}}
09:28:43.843 fetch POST /api/observability/v1/traces
09:28:43.843 fetched POST /api/observability/v1/traces 204
09:28:44.468 in Ping 
09:28:47.442 chunk 12 4721 bytes
09:28:47.443 in Ack 12
09:28:48.444 fetch POST /api/observability/v1/traces
09:28:48.444 fetched POST /api/observability/v1/traces 204
09:28:48.620 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:28:49.468 in Ping 
09:28:49.625 fetch POST /api/observability/v1/traces
09:28:49.625 fetched POST /api/observability/v1/traces 204
09:28:49.986 chunk 12 4785 bytes
09:28:49.987 in Ack 12
09:28:49.991 rpc 32 orchestration.dispatchCommand thread.visit
09:28:49.991 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:28:50.988 chunk 18 38776 bytes
09:28:50.991 in Ack 18
09:28:51.098 fetch POST /api/observability/v1/traces
09:28:51.098 fetched POST /api/observability/v1/traces 204
09:28:51.497 chunk 18 40089 bytes
09:28:51.502 in Ack 18
09:28:51.748 chunk 18 40089 bytes
09:28:51.750 in Ack 18
09:28:51.997 chunk 12 4785 bytes
09:28:51.998 in Ack 12
09:28:52.108 fetch POST /api/observability/v1/traces
09:28:52.109 fetched POST /api/observability/v1/traces 204
09:28:52.249 chunk 12 4785 bytes
09:28:52.249 chunk 18 40089 bytes
09:28:52.250 in Ack 12
09:28:52.260 in Ack 18
09:28:53.002 chunk 18 40725 bytes
09:28:53.004 in Ack 18
09:28:53.251 fetch POST /api/observability/v1/traces
09:28:53.252 fetched POST /api/observability/v1/traces 204
09:28:54.468 in Ping 
09:28:54.667 rpc 33 server.reportClientActivity
09:28:55.667 fetch POST /api/observability/v1/traces
09:28:55.667 fetched POST /api/observability/v1/traces 204
09:28:59.468 in Ping 
09:28:59.830 chunk 18 43360 bytes
09:28:59.835 in Ack 18
09:28:59.999 rpc 34 orchestration.dispatchCommand thread.visit
09:28:59.999 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:29:00.387 chunk 18 44138 bytes
09:29:00.389 in Ack 18
09:29:00.849 fetch POST /api/observability/v1/traces
09:29:00.850 fetched POST /api/observability/v1/traces 204
09:29:01.958 exit 33 {"_tag":"Success","value":null}
09:29:02.114 fetch POST /api/observability/v1/traces
09:29:02.114 fetched POST /api/observability/v1/traces 204
09:29:04.357 chunk 18 44090 bytes
09:29:04.359 in Ack 18
09:29:04.470 in Ping 
09:29:04.608 chunk 18 45134 bytes
09:29:04.610 in Ack 18
09:29:05.360 fetch POST /api/observability/v1/traces
09:29:05.360 fetched POST /api/observability/v1/traces 204
09:29:05.737 chunk 12 4785 bytes
09:29:05.738 in Ack 12
09:29:05.991 chunk 18 45134 bytes
09:29:05.995 in Ack 18
09:29:06.241 chunk 12 4785 bytes
09:29:06.242 in Ack 12
09:29:06.740 fetch POST /api/observability/v1/traces
09:29:06.740 fetched POST /api/observability/v1/traces 204
09:29:09.471 in Ping 
09:29:09.544 chunk 12 4785 bytes
09:29:09.545 in Ack 12
09:29:10.000 rpc 35 orchestration.dispatchCommand thread.visit
09:29:10.000 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:29:10.555 fetch POST /api/observability/v1/traces
09:29:10.556 fetched POST /api/observability/v1/traces 204
09:29:12.118 fetch POST /api/observability/v1/traces
09:29:12.118 fetched POST /api/observability/v1/traces 204
09:29:12.225 chunk 18 45134 bytes
09:29:12.228 in Ack 18
09:29:13.229 fetch POST /api/observability/v1/traces
09:29:13.229 fetched POST /api/observability/v1/traces 204
09:29:14.059 chunk 12 4785 bytes
09:29:14.064 in Ack 12
09:29:14.311 chunk 18 46979 bytes
09:29:14.315 in Ack 18
09:29:14.472 in Ping 
09:29:14.568 chunk 18 48668 bytes
09:29:14.577 in Ack 18
09:29:14.851 chunk 18 52155 bytes
09:29:14.854 in Ack 18
09:29:15.273 fetch POST /api/observability/v1/traces
09:29:15.273 fetched POST /api/observability/v1/traces 204
09:29:15.892 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:29:16.278 chunk 18 54224 bytes
09:29:16.281 in Ack 18
09:29:16.892 fetch POST /api/observability/v1/traces
09:29:16.892 fetched POST /api/observability/v1/traces 204
09:29:17.705 exit 36 {"_tag":"Success","value":{"sequence":188}}
09:29:18.034 chunk 18 54267 bytes
09:29:18.037 in Ack 18
09:29:18.572 chunk 12 4787 bytes
09:29:18.573 in Ack 12
09:29:18.707 fetch POST /api/observability/v1/traces
09:29:18.707 fetched POST /api/observability/v1/traces 204
09:29:18.987 chunk 18 54317 bytes
09:29:18.990 in Ack 18
09:29:19.238 chunk 18 54318 bytes
09:29:19.249 in Ack 18
09:29:19.472 in Ping 
09:29:19.670 rpc 37 server.reportClientActivity
09:29:19.992 fetch POST /api/observability/v1/traces
09:29:19.992 fetched POST /api/observability/v1/traces 204
09:29:19.997 rpc 38 orchestration.dispatchCommand thread.visit
09:29:19.997 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:29:20.239 chunk 18 54366 bytes
09:29:20.246 in Ack 18
09:29:20.742 chunk 18 55759 bytes
09:29:20.744 in Ack 18
09:29:20.992 chunk 18 56791 bytes
09:29:20.997 in Ack 18
09:29:21.012 in Interrupt 28
09:29:21.065 rpc 39 subscribeVcsStatus
09:29:21.066 fetch POST /api/observability/v1/traces
09:29:21.066 fetched POST /api/observability/v1/traces 204
09:29:21.245 chunk 12 5094 bytes
09:29:21.250 in Ack 12
09:29:22.123 fetch POST /api/observability/v1/traces
09:29:22.123 fetched POST /api/observability/v1/traces 204
09:29:22.748 chunk 18 56791 bytes
09:29:22.751 in Ack 18
09:29:22.946 chunk 39 1713 bytes
09:29:22.946 in Ack 39
09:29:23.576 exit 37 {"_tag":"Success","value":null}
09:29:23.749 chunk 12 5094 bytes
09:29:23.750 chunk 18 57808 bytes
09:29:23.750 in Ack 12
09:29:23.761 in Ack 18
09:29:23.775 fetch POST /api/observability/v1/traces
09:29:23.775 fetched POST /api/observability/v1/traces 204
09:29:23.829 rpc 40 server.reportClientActivity
09:29:24.474 in Ping 
09:29:24.829 fetch POST /api/observability/v1/traces
09:29:24.829 fetched POST /api/observability/v1/traces 204
09:29:25.476 chunk 18 61044 bytes
09:29:25.478 in Ack 18
09:29:25.726 chunk 18 60996 bytes
09:29:25.729 in Ack 18
09:29:25.976 chunk 12 5094 bytes
09:29:25.977 in Ack 12
09:29:26.227 chunk 18 61951 bytes
09:29:26.232 in Ack 18
09:29:26.479 fetch POST /api/observability/v1/traces
09:29:26.479 fetched POST /api/observability/v1/traces 204
09:29:27.301 exit 40 {"_tag":"Success","value":null}
09:29:28.303 fetch POST /api/observability/v1/traces
09:29:28.303 fetched POST /api/observability/v1/traces 204
09:29:28.626 chunk 12 5094 bytes
09:29:28.628 in Ack 12
09:29:29.477 in Ping 
09:29:29.629 fetch POST /api/observability/v1/traces
09:29:29.629 fetched POST /api/observability/v1/traces 204
09:29:29.996 rpc 41 orchestration.dispatchCommand thread.visit
09:29:29.996 exit 41 {"_tag":"Success","value":{"sequence":0}}
09:29:30.651 chunk 12 4789 bytes
09:29:30.652 in Ack 12
09:29:30.996 fetch POST /api/observability/v1/traces
09:29:30.996 fetched POST /api/observability/v1/traces 204
09:29:32.101 chunk 18 63018 bytes
09:29:32.115 in Ack 18
09:29:32.552 chunk 18 63018 bytes
09:29:32.559 fetch POST /api/observability/v1/traces
09:29:32.559 fetched POST /api/observability/v1/traces 204
09:29:32.592 in Ack 18
09:29:32.831 chunk 12 4728 bytes
09:29:32.832 chunk 18 65596 bytes
09:29:32.832 in Ack 12
09:29:32.854 in Ack 18
09:29:33.106 chunk 12 4725 bytes
09:29:33.106 chunk 18 66398 bytes
09:29:33.108 in Ack 12
09:29:33.109 rpc 42 orchestration.dispatchCommand thread.visit
09:29:33.109 exit 42 {"_tag":"Success","value":{"sequence":0}}
09:29:33.118 in Ack 18
09:29:33.140 in Interrupt 39
09:29:33.147 rpc 43 subscribeVcsStatus
09:29:33.357 chunk 18 66517 bytes
09:29:33.359 in Ack 18
09:29:33.398 rpc 44 server.reportClientActivity
09:29:33.689 fetch POST /api/observability/v1/traces
09:29:33.689 fetched POST /api/observability/v1/traces 204
09:29:33.691 chunk 18 67534 bytes
09:29:33.990 in Ack 18
