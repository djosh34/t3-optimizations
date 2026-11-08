# C, webapp, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.8s |
| Prompt reached the agent | 1.3s |
| Time to first text | 2.2s |
| Whole reply shown | 46.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.0s / 0.8s / 1.7s |
| Longest silence | 4.2s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 621427 / 343622 |
| Bytes the app received / sent | 416774 / 68144 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.7 | 318 | 348 |
| 20 | 0.8 | 714 | 750 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.4 | 1355 | 1409 |
| 46 | 1.0 | 1575 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 870/870 860/860 1299/1299 1108/1108 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 29/778","nodes":"809/809 1025/1025 809/809","providerThreads":"810/810","providerSessions":"2688/2688","messages":"550/550 456/456 321/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":244,"lastDataAt":1794159395743,"silences":0,"longestSilenceMs":1561,"longestRecoveredGapMs":1561},"stats":{"sendRate":7069.535724112106,"deliveredRate":3065.4772264564326,"loss":0,"srttMs":49.78234647436712,"minRttMs":1,"sentPackets":117,"sentWireBytes":34364,"receivedPackets":113,"receivedWireBytes":53977,"lastHeardMs":9939,"unsyncedBytes":1001,"inflightPackets":14,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 870/870 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 410/410","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":244,"lastDataAt":1794159405874,"silences":0,"longestSilenceMs":1561,"longestRecoveredGapMs":1561},"stats":{"sendRate":1500,"deliveredRate":2519.8113207547167,"loss":0,"srttMs":978.5645046289692,"minRttMs":1,"sentPackets":179,"sentWireBytes":48093,"receivedPackets":202,"receivedWireBytes":97382,"lastHeardMs":20070,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 870/870 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 908/908","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 549/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":244,"lastDataAt":1794159415961,"silences":0,"longestSilenceMs":1561,"longestRecoveredGapMs":1561},"stats":{"sendRate":2250,"deliveredRate":1487.558371040724,"loss":0.027272727272727282,"srttMs":1048.9237669565534,"minRttMs":983,"sentPackets":239,"sentWireBytes":58744,"receivedPackets":493,"receivedWireBytes":172138,"lastHeardMs":30157,"unsyncedBytes":0,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 870/870 860/860 1299/1299 1108/1108 1058/1058 849/849 1108/1108 932/932 1225/1554","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 549/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"800/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":244,"lastDataAt":1794159426011,"silences":0,"longestSilenceMs":1561,"longestRecoveredGapMs":1561},"stats":{"sendRate":2250,"deliveredRate":978.743676241367,"loss":0.08060454545454546,"srttMs":1044.0666340761113,"minRttMs":983,"sentPackets":288,"sentWireBytes":65705,"receivedPackets":786,"receivedWireBytes":259532,"lastHeardMs":40208,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 870/870 860/860 1299/1299 1108/1108 1058/1058 849/849 1108/1108 956/956 1578/1578 1083/1083","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 449/449","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","nodes":"809/809 1025/1025 809/809 1025/1025 785/785 989/989 1401/1401 1025/1025","providerThreads":"812/812","providerSessions":"2688/2688","messages":"550/550 456/456 901/901 550/550 451/451 550/550 180/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":244,"lastDataAt":1794159436583,"silences":0,"longestSilenceMs":1561,"longestRecoveredGapMs":1561},"stats":{"sendRate":3987.6644736842104,"deliveredRate":2453.9473684210525,"loss":0.07692307692307687,"srttMs":1088.617316161327,"minRttMs":983,"sentPackets":365,"sentWireBytes":85042,"receivedPackets":950,"receivedWireBytes":390893,"lastHeardMs":50779,"unsyncedBytes":659,"inflightPackets":15,"duplicateBytes":0}}
- shim trace
17:36:26.093 fetch GET /api/auth/session
17:36:26.095 fetched GET /api/auth/session 200
17:36:26.218 fetch GET /.well-known/t3/environment
17:36:26.219 fetched GET /.well-known/t3/environment 200
17:36:26.237 fetch GET /.well-known/t3/environment
17:36:26.238 fetched GET /.well-known/t3/environment 200
17:36:26.241 rpc 0 subscribeServerConfig
17:36:26.242 chunk 0 38690 bytes
17:36:26.270 in Ack 0
17:36:26.271 rpc 1 subscribeServerLifecycle
17:36:26.272 rpc 2 subscribeServerLifecycle
17:36:26.275 fetch GET /api/auth/session
17:36:26.276 fetched GET /api/auth/session 200
17:36:26.276 rpc 3 subscribeDeviceState
17:36:26.278 rpc 4 preview.reportProfiles
17:36:26.299 rpc 5 subscribeProjectClones
17:36:26.315 fetch GET /api/orchestration/threads/mcp%3A6d90397d-68a7-45dc-be5e-4de920e3292d/bounded
17:36:26.319 fetch GET /api/orchestration/shell
17:36:26.336 rpc 6 server.refreshProviders
17:36:26.372 rpc 7 orchestration.dispatchCommand thread.visit
17:36:26.372 exit 7 {"_tag":"Success","value":{"sequence":0}}
17:36:26.402 fetched GET /api/orchestration/shell 200
17:36:26.402 rpc 8 subscribeTerminalMetadata
17:36:26.403 rpc 9 subscribeVcsStatus
17:36:26.404 rpc 10 subscribePreviewEvents
17:36:26.405 rpc 11 orchestration.subscribeShell
17:36:26.405 chunk 11 2782 bytes
17:36:26.407 rpc 12 assets.createUrl
17:36:26.408 rpc 13 preview.list
17:36:26.408 rpc 14 projects.readFile
17:36:26.409 in Ack 11
17:36:26.656 rpc 15 server.reportClientActivity
17:36:26.691 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"fa21e16c-f833-4b3e-bd9c-83e4c068a123","sessions":[]}}
17:36:26.691 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:36:26.969 chunk 1 3864 bytes
17:36:26.970 in Ack 1
17:36:27.179 chunk 2 3864 bytes
17:36:27.180 in Ack 2
17:36:27.217 fetch POST /api/observability/v1/traces
17:36:27.217 fetched POST /api/observability/v1/traces 204
17:36:27.400 chunk 11 2758 bytes
17:36:27.400 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:36:14.012Z","configurableProviders":false,"con
17:36:27.400 chunk 9 534 bytes
17:36:27.400 exit 12 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:36:27.400 in Ack 11
17:36:27.407 in Ack 9
17:36:27.898 chunk 3 876 bytes
17:36:27.898 exit 4 {"_tag":"Success","value":null}
17:36:27.898 chunk 5 4 bytes
17:36:27.898 chunk 8 36 bytes
17:36:27.898 exit 15 {"_tag":"Success","value":null}
17:36:27.898 in Ack 3
17:36:27.902 rpc 16 preview.reportProfiles
17:36:27.902 in Ack 5
17:36:27.903 in Ack 8
17:36:27.906 chunk 11 2801 bytes
17:36:27.906 in Ack 11
17:36:27.998 exit 16 {"_tag":"Success","value":null}
17:36:28.398 fetched GET /api/orchestration/threads/mcp%3A6d90397d-68a7-45dc-be5e-4de920e3292d/bounded 200
17:36:28.402 rpc 17 orchestration.subscribeThread
17:36:28.402 chunk 17 8305 bytes
17:36:28.411 in Interrupt 9
17:36:28.414 rpc 18 subscribeVcsStatus
17:36:28.414 fetch POST /api/observability/v1/traces
17:36:28.414 fetched POST /api/observability/v1/traces 204
17:36:28.415 in Ack 17
17:36:28.588 chunk 18 534 bytes
17:36:28.588 in Ack 18
17:36:28.653 chunk 17 9359 bytes
17:36:28.654 in Ack 17
17:36:28.675 rpc 19 server.reportClientActivity
17:36:28.914 chunk 17 12722 bytes
17:36:28.916 in Ack 17
17:36:28.978 exit 19 {"_tag":"Success","value":null}
17:36:29.161 rpc 20 attachments.createUploadUrl
17:36:29.325 chunk 17 13532 bytes
17:36:29.328 in Ack 17
17:36:29.398 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-83e59263-2271-41ac-934b-b5e8c6e4b2c2","expiresAt":1794159989358,"relativeUrl":"/api/attachments/upload/eyJ2ZX
17:36:29.399 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTgzZTU5MjYzLTIyNzEtNDFhYy05MzRiLWI1ZThjNmU0YjJjMiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1OTk4OTM1OH0.IuinMp7yBWUBOp5tG7RFrNBaAucV3Xx_3hFxgTR7M3w
17:36:29.417 fetch POST /api/observability/v1/traces
17:36:29.417 fetched POST /api/observability/v1/traces 204
17:36:29.575 chunk 17 16220 bytes
17:36:29.576 in Ack 17
17:36:29.825 chunk 17 18637 bytes
17:36:29.826 in Ack 17
17:36:30.079 chunk 17 20497 bytes
17:36:30.080 in Ack 17
17:36:30.086 in Interrupt 18
17:36:30.088 rpc 21 subscribeVcsStatus
17:36:30.193 chunk 21 534 bytes
17:36:30.194 in Ack 21
17:36:30.329 chunk 17 21223 bytes
17:36:30.330 in Ack 17
17:36:30.339 rpc 22 server.reportClientActivity
17:36:30.577 fetch POST /api/observability/v1/traces
17:36:30.577 fetched POST /api/observability/v1/traces 204
17:36:31.157 exit 22 {"_tag":"Success","value":null}
17:36:31.240 in Ping 
17:36:32.159 fetch POST /api/observability/v1/traces
17:36:32.159 fetched POST /api/observability/v1/traces 204
17:36:32.340 rpc 23 orchestration.dispatchCommand message.dispatch
17:36:32.939 exit 23 {"_tag":"Success","value":{"sequence":45}}
17:36:32.941 rpc 24 attachments.delete
17:36:33.339 fetch POST /api/observability/v1/traces
17:36:33.339 fetched POST /api/observability/v1/traces 204
17:36:33.418 exit 24 {"_tag":"Success","value":null}
17:36:33.438 chunk 11 2887 bytes
17:36:33.439 chunk 17 23641 bytes
17:36:33.439 in Ack 11
17:36:33.493 in Ack 17
17:36:33.689 chunk 17 23663 bytes
17:36:33.690 in Ack 17
17:36:33.939 chunk 17 23713 bytes
17:36:33.940 in Ack 17
17:36:34.189 chunk 11 2823 bytes
17:36:34.189 chunk 17 28141 bytes
17:36:34.190 in Ack 11
17:36:34.190 rpc 25 orchestration.dispatchCommand thread.visit
17:36:34.190 exit 25 {"_tag":"Success","value":{"sequence":0}}
17:36:34.195 in Ack 17
17:36:34.197 rpc 26 assets.createUrl
17:36:34.420 fetch POST /api/observability/v1/traces
17:36:34.420 fetched POST /api/observability/v1/traces 204
17:36:34.440 chunk 11 2823 bytes
17:36:34.440 in Ack 11
17:36:34.690 chunk 17 31374 bytes
17:36:34.691 in Ack 17
17:36:35.171 chunk 17 32154 bytes
17:36:35.173 in Ack 17
17:36:35.180 in Interrupt 21
17:36:35.182 rpc 27 subscribeVcsStatus
17:36:35.423 chunk 17 31988 bytes
17:36:35.424 in Ack 17
17:36:35.438 rpc 28 server.reportClientActivity
17:36:35.449 fetch POST /api/observability/v1/traces
17:36:35.449 fetched POST /api/observability/v1/traces 204
17:36:35.636 chunk 27 534 bytes
17:36:35.637 in Ack 27
17:36:35.853 rpc 29 orchestration.dispatchCommand message.dispatch
17:36:35.951 chunk 17 31988 bytes
17:36:35.954 in Ack 17
17:36:36.240 in Ping 
17:36:36.702 chunk 17 33586 bytes
17:36:36.703 in Ack 17
17:36:36.848 exit 28 {"_tag":"Success","value":null}
17:36:36.854 fetch POST /api/observability/v1/traces
17:36:36.854 fetched POST /api/observability/v1/traces 204
17:36:36.953 chunk 17 33586 bytes
17:36:36.954 in Ack 17
17:36:37.385 exit 29 {"_tag":"Success","value":{"sequence":75}}
17:36:37.911 fetch POST /api/observability/v1/traces
17:36:37.911 fetched POST /api/observability/v1/traces 204
17:36:37.953 chunk 17 36062 bytes
17:36:37.954 in Ack 17
17:36:38.203 chunk 17 36082 bytes
17:36:38.204 in Ack 17
17:36:38.455 chunk 17 36112 bytes
17:36:38.456 in Ack 17
17:36:38.705 chunk 17 36132 bytes
17:36:38.706 in Ack 17
17:36:38.955 fetch POST /api/observability/v1/traces
17:36:38.955 fetched POST /api/observability/v1/traces 204
17:36:38.957 chunk 17 36114 bytes
17:36:38.958 in Ack 17
17:36:39.207 chunk 11 2887 bytes
17:36:39.208 chunk 17 36144 bytes
17:36:39.208 in Ack 11
17:36:39.251 in Ack 17
17:36:39.459 chunk 17 36154 bytes
17:36:39.460 in Ack 17
17:36:39.710 chunk 17 36194 bytes
17:36:39.711 in Ack 17
17:36:39.959 fetch POST /api/observability/v1/traces
17:36:39.959 fetched POST /api/observability/v1/traces 204
17:36:39.960 chunk 17 36204 bytes
17:36:39.961 in Ack 17
17:36:40.211 chunk 17 36883 bytes
17:36:40.212 in Ack 17
17:36:40.461 chunk 17 36903 bytes
17:36:40.462 in Ack 17
17:36:40.711 chunk 17 36923 bytes
17:36:40.712 in Ack 17
17:36:40.962 chunk 17 36943 bytes
17:36:40.963 in Ack 17
17:36:40.971 fetch POST /api/observability/v1/traces
17:36:40.971 fetched POST /api/observability/v1/traces 204
17:36:41.212 chunk 17 37823 bytes
17:36:41.213 in Ack 17
17:36:41.284 in Ping 
17:36:41.462 chunk 17 39836 bytes
17:36:41.463 in Ack 17
17:36:41.713 chunk 17 39866 bytes
17:36:41.714 in Ack 17
17:36:41.963 chunk 17 40547 bytes
17:36:41.964 in Ack 17
17:36:42.213 chunk 17 40587 bytes
17:36:42.214 in Ack 17
17:36:42.223 fetch POST /api/observability/v1/traces
17:36:42.223 fetched POST /api/observability/v1/traces 204
17:36:42.463 chunk 17 41393 bytes
17:36:42.464 in Ack 17
17:36:42.713 chunk 17 41413 bytes
17:36:42.714 in Ack 17
17:36:42.964 chunk 17 41445 bytes
17:36:42.965 in Ack 17
17:36:43.224 chunk 17 41475 bytes
17:36:43.225 in Ack 17
17:36:43.465 fetch POST /api/observability/v1/traces
17:36:43.465 fetched POST /api/observability/v1/traces 204
17:36:43.475 chunk 17 41495 bytes
17:36:43.476 in Ack 17
17:36:43.725 chunk 17 41515 bytes
17:36:43.726 in Ack 17
17:36:43.976 chunk 17 41535 bytes
17:36:43.977 in Ack 17
17:36:44.191 rpc 30 orchestration.dispatchCommand thread.visit
17:36:44.191 exit 30 {"_tag":"Success","value":{"sequence":0}}
17:36:44.226 chunk 17 42091 bytes
17:36:44.228 in Ack 17
17:36:44.477 fetch POST /api/observability/v1/traces
17:36:44.477 fetched POST /api/observability/v1/traces 204
17:36:44.477 chunk 17 42111 bytes
17:36:44.478 in Ack 17
17:36:44.730 chunk 17 42131 bytes
17:36:44.735 in Ack 17
17:36:44.978 chunk 17 43841 bytes
17:36:44.979 in Ack 17
17:36:45.229 chunk 17 43881 bytes
17:36:45.230 in Ack 17
17:36:45.479 fetch POST /api/observability/v1/traces
17:36:45.479 fetched POST /api/observability/v1/traces 204
17:36:45.480 chunk 17 43911 bytes
17:36:45.480 in Ack 17
17:36:45.730 chunk 11 2887 bytes
17:36:45.731 chunk 17 43931 bytes
17:36:45.731 in Ack 11
17:36:45.736 in Ack 17
17:36:45.982 chunk 17 43941 bytes
17:36:45.983 in Ack 17
17:36:46.232 chunk 17 43981 bytes
17:36:46.234 in Ack 17
17:36:46.285 in Ping 
17:36:46.482 fetch POST /api/observability/v1/traces
17:36:46.482 fetched POST /api/observability/v1/traces 204
17:36:46.483 chunk 17 44001 bytes
17:36:46.485 in Ack 17
17:36:46.718 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/b8329f3c-c9e8-484d-8fac-352cdf028b78","expiresAt":1794163006718}}
17:36:46.734 chunk 17 44031 bytes
17:36:46.735 in Ack 17
17:36:46.984 chunk 17 44051 bytes
17:36:46.985 in Ack 17
17:36:47.234 chunk 17 44071 bytes
17:36:47.235 in Ack 17
17:36:47.484 chunk 17 44111 bytes
17:36:47.485 in Ack 17
17:36:47.493 fetch POST /api/observability/v1/traces
17:36:47.493 fetched POST /api/observability/v1/traces 204
17:36:47.735 chunk 17 44131 bytes
17:36:47.736 in Ack 17
17:36:47.985 chunk 11 2887 bytes
17:36:47.986 chunk 17 44151 bytes
17:36:47.986 in Ack 11
17:36:47.988 in Ack 17
17:36:48.237 chunk 17 45261 bytes
17:36:48.238 in Ack 17
17:36:48.486 chunk 17 45291 bytes
17:36:48.487 in Ack 17
17:36:48.737 fetch POST /api/observability/v1/traces
17:36:48.737 fetched POST /api/observability/v1/traces 204
17:36:48.737 chunk 17 45336 bytes
17:36:48.740 in Ack 17
17:36:48.987 chunk 17 45366 bytes
17:36:48.989 in Ack 17
17:36:49.237 chunk 17 45396 bytes
17:36:49.238 in Ack 17
17:36:49.487 chunk 17 45441 bytes
17:36:49.488 in Ack 17
17:36:49.737 chunk 17 45486 bytes
17:36:49.738 in Ack 17
17:36:49.765 fetch POST /api/observability/v1/traces
17:36:49.765 fetched POST /api/observability/v1/traces 204
17:36:49.987 chunk 17 45531 bytes
17:36:49.988 in Ack 17
17:36:50.237 chunk 17 45561 bytes
17:36:50.238 in Ack 17
17:36:50.487 chunk 17 45591 bytes
17:36:50.488 in Ack 17
17:36:50.737 chunk 17 45636 bytes
17:36:50.739 in Ack 17
17:36:50.988 chunk 17 45666 bytes
17:36:50.989 in Ack 17
17:36:50.997 fetch POST /api/observability/v1/traces
17:36:50.997 fetched POST /api/observability/v1/traces 204
17:36:51.238 chunk 17 45711 bytes
17:36:51.239 in Ack 17
17:36:51.285 in Ping 
17:36:51.471 rpc 31 server.reportClientActivity
17:36:51.488 chunk 17 45741 bytes
17:36:51.490 in Ack 17
17:36:51.738 chunk 17 45786 bytes
17:36:51.739 in Ack 17
17:36:51.988 chunk 17 45816 bytes
17:36:51.989 in Ack 17
17:36:52.239 chunk 17 45846 bytes
17:36:52.240 in Ack 17
17:36:52.259 fetch POST /api/observability/v1/traces
17:36:52.259 fetched POST /api/observability/v1/traces 204
17:36:52.489 chunk 17 45891 bytes
17:36:52.490 in Ack 17
17:36:52.603 exit 31 {"_tag":"Success","value":null}
17:36:52.739 chunk 17 45936 bytes
17:36:52.740 in Ack 17
17:36:52.989 chunk 17 45966 bytes
17:36:52.990 in Ack 17
17:36:53.239 chunk 17 45996 bytes
17:36:53.240 in Ack 17
17:36:53.489 chunk 17 46041 bytes
17:36:53.490 in Ack 17
17:36:53.503 fetch POST /api/observability/v1/traces
17:36:53.503 fetched POST /api/observability/v1/traces 204
17:36:53.740 chunk 17 46071 bytes
17:36:53.741 in Ack 17
17:36:53.990 chunk 17 46131 bytes
17:36:53.991 in Ack 17
17:36:54.191 rpc 32 orchestration.dispatchCommand thread.visit
17:36:54.191 exit 32 {"_tag":"Success","value":{"sequence":0}}
17:36:54.241 chunk 17 46146 bytes
17:36:54.242 in Ack 17
17:36:54.491 chunk 17 46176 bytes
17:36:54.493 in Ack 17
17:36:54.742 chunk 17 46236 bytes
17:36:54.742 fetch POST /api/observability/v1/traces
17:36:54.742 fetched POST /api/observability/v1/traces 204
17:36:54.744 in Ack 17
17:36:54.998 chunk 17 46266 bytes
17:36:55.000 in Ack 17
17:36:55.248 chunk 17 46296 bytes
17:36:55.249 in Ack 17
17:36:55.498 chunk 17 46341 bytes
17:36:55.499 in Ack 17
17:36:55.745 fetch POST /api/observability/v1/traces
17:36:55.745 fetched POST /api/observability/v1/traces 204
17:36:55.748 chunk 17 46356 bytes
17:36:55.749 in Ack 17
17:36:56.000 chunk 17 46416 bytes
17:36:56.001 in Ack 17
17:36:56.250 chunk 17 46446 bytes
17:36:56.251 in Ack 17
17:36:56.287 in Ping 
17:36:56.490 chunk 11 2887 bytes
17:36:56.491 in Ack 11
17:36:56.500 chunk 17 46491 bytes
17:36:56.501 in Ack 17
17:36:56.750 fetch POST /api/observability/v1/traces
17:36:56.750 fetched POST /api/observability/v1/traces 204
17:36:56.750 chunk 17 46536 bytes
17:36:56.751 in Ack 17
17:36:57.000 chunk 17 46566 bytes
17:36:57.001 in Ack 17
17:36:57.250 chunk 17 46611 bytes
17:36:57.251 in Ack 17
17:36:57.501 chunk 17 46641 bytes
17:36:57.502 in Ack 17
17:36:57.751 chunk 17 46671 bytes
17:36:57.752 in Ack 17
17:36:57.759 fetch POST /api/observability/v1/traces
17:36:57.759 fetched POST /api/observability/v1/traces 204
17:36:58.085 chunk 17 46701 bytes
17:36:58.086 in Ack 17
17:36:58.335 chunk 17 46761 bytes
17:36:58.336 in Ack 17
17:36:58.585 chunk 17 46776 bytes
17:36:58.586 in Ack 17
17:36:58.835 chunk 17 46836 bytes
17:36:58.836 in Ack 17
17:36:59.086 chunk 17 46881 bytes
17:36:59.086 in Ack 17
17:36:59.095 fetch POST /api/observability/v1/traces
17:36:59.095 fetched POST /api/observability/v1/traces 204
17:36:59.336 chunk 17 46911 bytes
17:36:59.337 in Ack 17
17:36:59.586 chunk 17 46956 bytes
17:36:59.587 in Ack 17
17:36:59.837 chunk 17 46986 bytes
17:36:59.838 in Ack 17
17:37:00.087 chunk 17 47001 bytes
17:37:00.088 in Ack 17
17:37:00.338 chunk 17 47061 bytes
17:37:00.338 fetch POST /api/observability/v1/traces
17:37:00.338 fetched POST /api/observability/v1/traces 204
17:37:00.339 in Ack 17
17:37:00.588 chunk 17 47091 bytes
17:37:00.589 in Ack 17
17:37:00.838 chunk 17 47136 bytes
17:37:00.839 in Ack 17
17:37:01.088 chunk 17 47181 bytes
17:37:01.089 in Ack 17
17:37:01.289 in Ping 
17:37:01.338 chunk 17 47226 bytes
17:37:01.339 in Ack 17
17:37:01.347 fetch POST /api/observability/v1/traces
17:37:01.347 fetched POST /api/observability/v1/traces 204
17:37:01.588 chunk 17 47256 bytes
17:37:01.589 in Ack 17
17:37:01.839 chunk 17 47301 bytes
17:37:01.840 in Ack 17
17:37:02.089 chunk 17 47316 bytes
17:37:02.090 in Ack 17
17:37:02.339 chunk 17 47361 bytes
17:37:02.340 in Ack 17
17:37:02.590 fetch POST /api/observability/v1/traces
17:37:02.590 fetched POST /api/observability/v1/traces 204
17:37:02.591 chunk 17 47406 bytes
17:37:02.592 in Ack 17
17:37:02.842 chunk 17 47421 bytes
17:37:02.843 in Ack 17
17:37:03.092 chunk 17 47481 bytes
17:37:03.093 in Ack 17
17:37:03.343 chunk 17 47511 bytes
17:37:03.344 in Ack 17
17:37:03.593 chunk 17 47556 bytes
17:37:03.593 fetch POST /api/observability/v1/traces
17:37:03.593 fetched POST /api/observability/v1/traces 204
17:37:03.594 in Ack 17
17:37:03.843 chunk 17 47586 bytes
17:37:03.844 in Ack 17
17:37:04.093 chunk 17 47616 bytes
17:37:04.094 in Ack 17
17:37:04.344 chunk 17 47646 bytes
17:37:04.345 in Ack 17
17:37:04.594 chunk 17 47703 bytes
17:37:04.595 in Ack 17
17:37:04.603 fetch POST /api/observability/v1/traces
17:37:04.603 fetched POST /api/observability/v1/traces 204
17:37:05.845 chunk 17 49842 bytes
17:37:05.846 in Ack 17
17:37:06.092 chunk 11 3197 bytes
17:37:06.092 in Ack 11
17:37:06.093 rpc 33 orchestration.dispatchCommand thread.visit
17:37:06.093 exit 33 {"_tag":"Success","value":{"sequence":0}}
17:37:06.290 in Ping 
17:37:06.348 chunk 17 50853 bytes
17:37:06.349 in Ack 17
17:37:06.535 rpc 34 orchestration.dispatchCommand runtime-request.respond
17:37:06.606 chunk 17 55334 bytes
17:37:06.608 in Ack 17
17:37:06.848 fetch POST /api/observability/v1/traces
17:37:06.848 fetched POST /api/observability/v1/traces 204
17:37:07.303 chunk 17 55334 bytes
17:37:07.304 in Ack 17
17:37:07.553 chunk 11 3197 bytes
17:37:07.553 in Ack 11
17:37:07.912 chunk 17 57827 bytes
17:37:07.912 exit 34 {"_tag":"Success","value":{"sequence":98}}
17:37:07.914 in Ack 17
17:37:07.969 fetch POST /api/observability/v1/traces
17:37:07.969 fetched POST /api/observability/v1/traces 204
17:37:08.162 chunk 17 57842 bytes
17:37:08.164 in Ack 17
17:37:08.413 chunk 17 57864 bytes
17:37:08.414 in Ack 17
17:37:08.663 chunk 17 58928 bytes
17:37:08.665 in Ack 17
17:37:08.913 chunk 17 58948 bytes
17:37:08.914 in Ack 17
17:37:09.163 chunk 17 58968 bytes
17:37:09.165 in Ack 17
17:37:09.172 fetch POST /api/observability/v1/traces
17:37:09.172 fetched POST /api/observability/v1/traces 204
17:37:09.413 chunk 17 58998 bytes
17:37:09.414 in Ack 17
17:37:09.664 chunk 17 59008 bytes
17:37:09.665 in Ack 17
17:37:09.913 chunk 11 2889 bytes
17:37:09.913 chunk 17 59038 bytes
17:37:09.914 in Ack 11
17:37:09.922 in Ack 17
17:37:10.179 chunk 17 59078 bytes
17:37:10.181 in Ack 17
17:37:10.416 fetch POST /api/observability/v1/traces
17:37:10.416 fetched POST /api/observability/v1/traces 204
17:37:10.429 chunk 17 59098 bytes
17:37:10.430 in Ack 17
17:37:10.680 chunk 17 59108 bytes
17:37:10.681 in Ack 17
17:37:10.931 chunk 17 59138 bytes
17:37:10.932 in Ack 17
17:37:11.180 chunk 17 59206 bytes
17:37:11.181 in Ack 17
17:37:11.290 in Ping 
17:37:11.430 chunk 17 59236 bytes
17:37:11.431 fetch POST /api/observability/v1/traces
17:37:11.431 fetched POST /api/observability/v1/traces 204
17:37:11.433 in Ack 17
17:37:11.680 chunk 17 59339 bytes
17:37:11.682 in Ack 17
17:37:11.689 in Interrupt 27
17:37:11.714 rpc 35 subscribeVcsStatus
17:37:11.930 chunk 17 59359 bytes
17:37:11.931 in Ack 17
17:37:11.974 rpc 36 server.reportClientActivity
17:37:12.180 chunk 17 59389 bytes
17:37:12.182 in Ack 17
17:37:12.431 chunk 17 59419 bytes
17:37:12.432 in Ack 17
17:37:12.445 fetch POST /api/observability/v1/traces
17:37:12.446 fetched POST /api/observability/v1/traces 204
17:37:12.683 chunk 17 59439 bytes
17:37:12.684 in Ack 17
17:37:12.934 chunk 17 60871 bytes
17:37:12.936 in Ack 17
17:37:13.019 chunk 35 534 bytes
17:37:13.019 in Ack 35
17:37:13.184 chunk 17 60891 bytes
17:37:13.185 in Ack 17
17:37:13.435 chunk 17 60911 bytes
17:37:13.436 in Ack 17
17:37:13.687 chunk 17 60931 bytes
17:37:13.688 fetch POST /api/observability/v1/traces
17:37:13.688 fetched POST /api/observability/v1/traces 204
17:37:13.689 in Ack 17
17:37:13.938 chunk 17 60971 bytes
17:37:13.939 in Ack 17
17:37:14.155 exit 36 {"_tag":"Success","value":null}
17:37:14.189 chunk 17 61011 bytes
17:37:14.190 in Ack 17
17:37:14.439 chunk 17 61031 bytes
17:37:14.441 in Ack 17
17:37:14.717 chunk 17 61061 bytes
17:37:14.717 fetch POST /api/observability/v1/traces
17:37:14.717 fetched POST /api/observability/v1/traces 204
17:37:14.719 in Ack 17
17:37:14.967 chunk 17 61081 bytes
17:37:14.968 in Ack 17
17:37:15.217 chunk 17 61111 bytes
17:37:15.218 in Ack 17
17:37:15.467 chunk 17 61131 bytes
17:37:15.469 in Ack 17
17:37:15.717 chunk 11 2889 bytes
17:37:15.717 chunk 17 61147 bytes
17:37:15.718 in Ack 11
17:37:15.719 in Ack 17
17:37:15.728 fetch POST /api/observability/v1/traces
17:37:15.728 fetched POST /api/observability/v1/traces 204
17:37:15.995 chunk 17 61169 bytes
17:37:15.996 in Ack 17
17:37:16.102 rpc 37 orchestration.dispatchCommand thread.visit
17:37:16.102 exit 37 {"_tag":"Success","value":{"sequence":0}}
17:37:16.245 chunk 17 61189 bytes
17:37:16.246 in Ack 17
17:37:16.379 in Ping 
17:37:16.471 rpc 38 server.reportClientActivity
17:37:16.496 chunk 17 61219 bytes
17:37:16.498 in Ack 17
17:37:16.746 chunk 17 61239 bytes
17:37:16.748 in Ack 17
17:37:16.997 chunk 17 62312 bytes
17:37:16.997 fetch POST /api/observability/v1/traces
17:37:16.997 fetched POST /api/observability/v1/traces 204
17:37:16.998 in Ack 17
17:37:17.246 chunk 17 62342 bytes
17:37:17.248 in Ack 17
17:37:17.496 chunk 11 2889 bytes
17:37:17.496 chunk 17 62387 bytes
17:37:17.497 in Ack 11
17:37:17.503 in Ack 17
17:37:17.747 chunk 17 62423 bytes
17:37:17.748 in Ack 17
17:37:17.997 chunk 17 62453 bytes
17:37:17.998 in Ack 17
17:37:18.009 fetch POST /api/observability/v1/traces
17:37:18.009 fetched POST /api/observability/v1/traces 204
17:37:18.247 chunk 17 62498 bytes
17:37:18.248 in Ack 17
17:37:18.498 chunk 17 62519 bytes
17:37:18.499 in Ack 17
17:37:18.696 exit 38 {"_tag":"Success","value":null}
17:37:18.748 chunk 17 62579 bytes
17:37:18.750 in Ack 17
17:37:18.998 chunk 17 62639 bytes
17:37:18.999 in Ack 17
17:37:19.248 chunk 17 62693 bytes
17:37:19.249 fetch POST /api/observability/v1/traces
17:37:19.249 fetched POST /api/observability/v1/traces 204
17:37:19.250 in Ack 17
17:37:19.498 chunk 17 62723 bytes
17:37:19.499 in Ack 17
17:37:19.749 chunk 17 62777 bytes
17:37:19.750 in Ack 17
17:37:19.999 chunk 17 62810 bytes
17:37:20.000 in Ack 17
17:37:20.249 chunk 17 62849 bytes
17:37:20.251 in Ack 17
17:37:20.324 fetch POST /api/observability/v1/traces
17:37:20.324 fetched POST /api/observability/v1/traces 204
17:37:20.500 chunk 17 62894 bytes
17:37:20.501 in Ack 17
17:37:20.749 chunk 17 62924 bytes
17:37:20.751 in Ack 17
17:37:20.999 chunk 17 62975 bytes
17:37:21.000 in Ack 17
17:37:21.249 chunk 17 63029 bytes
17:37:21.251 in Ack 17
17:37:21.380 in Ping 
17:37:21.500 chunk 17 63062 bytes
17:37:21.501 in Ack 17
17:37:21.515 fetch POST /api/observability/v1/traces
17:37:21.515 fetched POST /api/observability/v1/traces 204
17:37:21.751 chunk 17 63107 bytes
17:37:21.752 in Ack 17
17:37:22.001 chunk 17 63257 bytes
17:37:22.002 in Ack 17
17:37:22.251 chunk 11 2825 bytes
17:37:22.252 in Ack 11
17:37:22.254 rpc 39 orchestration.dispatchCommand thread.visit
17:37:22.254 exit 39 {"_tag":"Success","value":{"sequence":0}}
17:37:22.503 chunk 17 63307 bytes
17:37:22.504 in Ack 17
17:37:22.768 fetch POST /api/observability/v1/traces
17:37:22.768 fetched POST /api/observability/v1/traces 204
