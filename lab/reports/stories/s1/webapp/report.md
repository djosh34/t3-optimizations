# story, webapp, warm start, seed 1

Profile story (graded): seed 1's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 61 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.4s |
| Prompt reached the agent | 10.4s |
| Time to first text | 22.5s |
| Whole reply shown | 58.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.5s / 4.5s / 16.5s |
| Longest silence | 22.5s |
| Approval shown after it was asked | 4.0s |
| Bytes on the wire down / up | 435252 / 181539 |
| Bytes the app received / sent | 320232 / 65301 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 9.5 | 0 | 381 |
| 30 | 9.5 | 397 | 783 |
| 40 | 19.5 | 397 | 1077 |
| 50 | 6.5 | 1077 | 1336 |
| 58 | 6.8 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216516188,"silences":0,"longestSilenceMs":1402,"longestRecoveredGapMs":1402},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0,"srttMs":290.7786260948377,"minRttMs":1,"sentPackets":154,"sentWireBytes":40906,"receivedPackets":181,"receivedWireBytes":126057,"lastHeardMs":17880,"unsyncedBytes":974,"inflightPackets":10,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 818/818","user":"10/10 9/9 73/73","attempts":"659/659 659/659 583/583","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 763/763","messages":"544/544 456/456 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216526196,"silences":0,"longestSilenceMs":1402,"longestRecoveredGapMs":1402},"stats":{"sendRate":703.125,"deliveredRate":1298.9749430523918,"loss":0,"srttMs":1625.922027602356,"minRttMs":1,"sentPackets":200,"sentWireBytes":50590,"receivedPackets":285,"receivedWireBytes":175576,"lastHeardMs":27887,"unsyncedBytes":456,"inflightPackets":6,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216536243,"silences":0,"longestSilenceMs":1402,"longestRecoveredGapMs":1402},"stats":{"sendRate":1542.123723219166,"deliveredRate":950.383171476222,"loss":0.07557692307692306,"srttMs":1930.4481575293228,"minRttMs":1504,"sentPackets":229,"sentWireBytes":57760,"receivedPackets":361,"receivedWireBytes":205392,"lastHeardMs":37935,"unsyncedBytes":0,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216546396,"silences":0,"longestSilenceMs":1402,"longestRecoveredGapMs":1402},"stats":{"sendRate":1542.123723219166,"deliveredRate":1422.0354808590102,"loss":0,"srttMs":1731.201845887985,"minRttMs":1504,"sentPackets":285,"sentWireBytes":67318,"receivedPackets":480,"receivedWireBytes":259349,"lastHeardMs":48088,"unsyncedBytes":0,"inflightPackets":10,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"362/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216556507,"silences":1,"longestSilenceMs":2357,"longestRecoveredGapMs":2357},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1724.70748660074,"minRttMs":1504,"sentPackets":335,"sentWireBytes":78440,"receivedPackets":576,"receivedWireBytes":317984,"lastHeardMs":58198,"unsyncedBytes":456,"inflightPackets":37,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 947/947","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 1392/1392 983/983 1016/1016","messages":"544/544 456/456 901/901 544/544 451/451 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1133,"lastDataAt":1794216567072,"silences":1,"longestSilenceMs":2357,"longestRecoveredGapMs":2357},"stats":{"sendRate":1875.744166829019,"deliveredRate":1654.6944309488797,"loss":0.005540769230769226,"srttMs":1833.2533895696486,"minRttMs":1504,"sentPackets":393,"sentWireBytes":95677,"receivedPackets":706,"receivedWireBytes":398850,"lastHeardMs":68763,"unsyncedBytes":0,"inflightPackets":17,"duplicateBytes":0}}
- shim trace
09:28:19.734 fetch GET /api/auth/session
09:28:19.737 fetched GET /api/auth/session 200
09:28:20.175 fetch GET /.well-known/t3/environment
09:28:20.199 fetched GET /.well-known/t3/environment 200
09:28:20.316 fetch GET /.well-known/t3/environment
09:28:20.318 fetched GET /.well-known/t3/environment 200
09:28:20.330 rpc 0 subscribeServerConfig
09:28:20.332 chunk 0 38458 bytes
09:28:20.393 in Ack 0
09:28:20.400 rpc 1 subscribeServerLifecycle
09:28:20.404 rpc 2 subscribeServerLifecycle
09:28:20.412 fetch GET /api/auth/session
09:28:20.415 fetched GET /api/auth/session 200
09:28:20.416 rpc 3 subscribeDeviceState
09:28:20.419 rpc 4 preview.reportProfiles
09:28:20.609 rpc 5 server.reportClientActivity
09:28:20.701 rpc 6 subscribeProjectClones
09:28:20.777 fetch GET /api/orchestration/threads/mcp%3Ac5e9ed2a-4c2e-4516-b931-b0436bf87e49/bounded
09:28:20.788 fetch GET /api/orchestration/shell
09:28:20.790 fetched GET /api/orchestration/shell 200
09:28:20.800 rpc 7 orchestration.subscribeShell
09:28:20.801 chunk 7 4550 bytes
09:28:20.869 rpc 8 server.refreshProviders
09:28:20.959 rpc 9 orchestration.dispatchCommand thread.visit
09:28:20.959 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:28:20.994 rpc 10 subscribeTerminalMetadata
09:28:20.995 rpc 11 subscribeVcsStatus
09:28:20.996 rpc 12 subscribePreviewEvents
09:28:20.998 rpc 13 assets.createUrl
09:28:20.999 rpc 14 preview.list
09:28:21.000 rpc 15 projects.readFile
09:28:21.046 in Ack 7
09:28:21.062 chunk 7 4526 bytes
09:28:21.063 in Ack 7
09:28:21.169 fetch POST /api/observability/v1/traces
09:28:21.169 fetched POST /api/observability/v1/traces 204
09:28:21.482 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"e821db7a-4c21-4fb3-8a4f-970b1f4203c1","sessions":[]}}
09:28:21.482 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:21.510 chunk 1 3814 bytes
09:28:21.511 in Ack 1
09:28:21.970 chunk 2 3814 bytes
09:28:21.970 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:48.594Z","configurableProviders":false,"con
09:28:21.970 chunk 11 1632 bytes
09:28:21.971 in Ack 2
09:28:21.978 in Ack 11
09:28:22.209 exit 13 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:22.459 chunk 7 4569 bytes
09:28:22.459 in Ack 7
09:28:22.492 fetch POST /api/observability/v1/traces
09:28:22.492 fetched POST /api/observability/v1/traces 204
09:28:22.710 chunk 7 4594 bytes
09:28:22.711 in Ack 7
09:28:22.961 chunk 7 4591 bytes
09:28:22.961 in Ack 7
09:28:22.998 chunk 3 876 bytes
09:28:22.998 exit 4 {"_tag":"Success","value":null}
09:28:22.998 exit 5 {"_tag":"Success","value":null}
09:28:22.998 chunk 6 4 bytes
09:28:22.999 in Ack 3
09:28:23.008 rpc 16 preview.reportProfiles
09:28:23.009 in Ack 6
09:28:23.080 chunk 10 36 bytes
09:28:23.080 exit 16 {"_tag":"Success","value":null}
09:28:23.080 in Ack 10
09:28:23.260 rpc 17 server.reportClientActivity
09:28:23.323 exit 17 {"_tag":"Success","value":null}
09:28:23.713 fetch POST /api/observability/v1/traces
09:28:23.713 fetched POST /api/observability/v1/traces 204
09:28:24.959 fetched GET /api/orchestration/threads/mcp%3Ac5e9ed2a-4c2e-4516-b931-b0436bf87e49/bounded 200
09:28:24.965 rpc 18 orchestration.subscribeThread
09:28:24.966 chunk 18 8282 bytes
09:28:24.986 in Interrupt 11
09:28:24.989 rpc 19 subscribeVcsStatus
09:28:24.991 in Ack 18
09:28:25.193 chunk 19 1632 bytes
09:28:25.194 in Ack 19
09:28:25.217 chunk 18 9336 bytes
09:28:25.218 in Ack 18
09:28:25.286 rpc 20 server.reportClientActivity
09:28:25.327 in Ping 
09:28:25.472 chunk 18 11918 bytes
09:28:25.474 in Ack 18
09:28:25.557 exit 20 {"_tag":"Success","value":null}
09:28:25.722 chunk 18 13497 bytes
09:28:25.723 in Ack 18
09:28:25.960 fetch POST /api/observability/v1/traces
09:28:25.960 fetched POST /api/observability/v1/traces 204
09:28:26.185 rpc 21 attachments.createUploadUrl
09:28:26.239 chunk 18 18551 bytes
09:28:26.265 in Ack 18
09:28:26.303 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-c334660a-51c5-455e-bd62-2551002f52bd","expiresAt":1794217106282,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:26.304 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWMzMzQ2NjBhLTUxYzUtNDU1ZS1iZDYyLTI1NTEwMDJmNTJiZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzEwNjI4Mn0.b61pw5AG9szCXFGjukNYVZ7znwJip99FRQob8Ogq0g0
09:28:26.489 chunk 18 19795 bytes
09:28:26.491 in Ack 18
09:28:26.503 in Interrupt 19
09:28:26.506 rpc 22 subscribeVcsStatus
09:28:26.533 chunk 22 1632 bytes
09:28:26.533 in Ack 22
09:28:26.739 chunk 18 21097 bytes
09:28:26.740 in Ack 18
09:28:26.758 rpc 23 server.reportClientActivity
09:28:27.200 fetch POST /api/observability/v1/traces
09:28:27.200 fetched POST /api/observability/v1/traces 204
09:28:28.398 exit 23 {"_tag":"Success","value":null}
09:28:29.400 fetch POST /api/observability/v1/traces
09:28:29.400 fetched POST /api/observability/v1/traces 204
09:28:29.824 rpc 24 orchestration.dispatchCommand message.dispatch
09:28:30.327 in Ping 
09:28:30.412 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:28:30.425 rpc 25 attachments.delete
09:28:30.621 exit 25 {"_tag":"Success","value":null}
09:28:30.821 fetch POST /api/observability/v1/traces
09:28:30.821 fetched POST /api/observability/v1/traces 204
09:28:30.939 chunk 7 4677 bytes
09:28:30.940 in Ack 7
09:28:31.024 chunk 18 26021 bytes
09:28:31.025 rpc 26 orchestration.dispatchCommand thread.visit
09:28:31.025 exit 26 {"_tag":"Success","value":{"sequence":0}}
09:28:31.031 in Ack 18
09:28:31.035 rpc 27 assets.createUrl
09:28:31.306 chunk 18 29264 bytes
09:28:31.311 in Ack 18
09:28:31.604 chunk 7 4616 bytes
09:28:31.605 chunk 18 29264 bytes
09:28:31.606 in Ack 7
09:28:31.691 in Ack 18
09:28:31.941 fetch POST /api/observability/v1/traces
09:28:31.941 fetched POST /api/observability/v1/traces 204
09:28:32.107 chunk 18 29288 bytes
09:28:32.110 in Ack 18
09:28:32.356 chunk 18 30989 bytes
09:28:32.358 in Ack 18
09:28:32.606 chunk 18 31900 bytes
09:28:32.609 in Ack 18
09:28:33.082 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/e880f021-fe69-4510-a2c4-047643670108","expiresAt":1794220113082}}
09:28:33.114 fetch POST /api/observability/v1/traces
09:28:33.114 fetched POST /api/observability/v1/traces 204
09:28:33.356 chunk 7 4613 bytes
09:28:33.357 in Ack 7
09:28:33.359 rpc 28 orchestration.dispatchCommand thread.visit
09:28:33.359 exit 28 {"_tag":"Success","value":{"sequence":0}}
09:28:33.373 chunk 18 34455 bytes
09:28:33.376 in Ack 18
09:28:33.922 chunk 7 4613 bytes
09:28:33.922 chunk 18 35257 bytes
09:28:33.923 in Ack 7
09:28:33.935 in Ack 18
09:28:33.954 in Interrupt 22
09:28:33.958 rpc 29 subscribeVcsStatus
09:28:33.986 chunk 29 1632 bytes
09:28:33.987 in Ack 29
09:28:34.248 rpc 30 server.refreshProviders
09:28:34.248 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:27:48.594Z","configurableProviders":false,"con
09:28:34.300 rpc 31 server.reportClientActivity
09:28:34.367 fetch POST /api/observability/v1/traces
09:28:34.367 fetched POST /api/observability/v1/traces 204
09:28:35.334 in Ping 
09:28:36.105 rpc 32 orchestration.dispatchCommand message.dispatch
09:28:36.726 fetch POST /api/observability/v1/traces
09:28:36.726 fetched POST /api/observability/v1/traces 204
09:28:37.069 exit 31 {"_tag":"Success","value":null}
09:28:38.040 chunk 7 4657 bytes
09:28:38.041 in Ack 7
09:28:38.076 fetch POST /api/observability/v1/traces
09:28:38.076 fetched POST /api/observability/v1/traces 204
09:28:38.151 exit 32 {"_tag":"Success","value":{"sequence":150}}
09:28:38.552 chunk 7 4701 bytes
09:28:38.554 in Ack 7
09:28:38.806 chunk 18 37278 bytes
09:28:38.816 in Ack 18
09:28:39.058 chunk 18 37862 bytes
09:28:39.062 in Ack 18
09:28:39.269 fetch POST /api/observability/v1/traces
09:28:39.269 fetched POST /api/observability/v1/traces 204
09:28:39.583 chunk 18 39206 bytes
09:28:39.588 in Ack 18
09:28:40.095 chunk 18 39206 bytes
09:28:40.102 in Ack 18
09:28:40.336 in Ping 
09:28:40.345 chunk 18 39970 bytes
09:28:40.348 in Ack 18
09:28:40.591 fetch POST /api/observability/v1/traces
09:28:40.591 fetched POST /api/observability/v1/traces 204
09:28:40.598 chunk 18 39970 bytes
09:28:40.602 in Ack 18
09:28:41.599 chunk 7 4721 bytes
09:28:41.600 in Ack 7
09:28:41.607 fetch POST /api/observability/v1/traces
09:28:41.607 fetched POST /api/observability/v1/traces 204
09:28:42.746 fetch POST /api/observability/v1/traces
09:28:42.746 fetched POST /api/observability/v1/traces 204
09:28:43.359 rpc 33 orchestration.dispatchCommand thread.visit
09:28:43.359 exit 33 {"_tag":"Success","value":{"sequence":0}}
09:28:44.360 fetch POST /api/observability/v1/traces
09:28:44.360 fetched POST /api/observability/v1/traces 204
09:28:45.336 in Ping 
09:28:45.429 rpc 34 server.reportClientActivity
09:28:45.612 chunk 7 4721 bytes
09:28:45.614 chunk 18 39970 bytes
09:28:45.615 in Ack 7
09:28:45.630 in Ack 18
09:28:46.431 fetch POST /api/observability/v1/traces
09:28:46.431 fetched POST /api/observability/v1/traces 204
09:28:50.336 in Ping 
09:28:52.753 fetch POST /api/observability/v1/traces
09:28:52.753 fetched POST /api/observability/v1/traces 204
09:28:53.101 chunk 18 39994 bytes
09:28:53.107 in Ack 18
09:28:53.351 chunk 7 4721 bytes
09:28:53.352 in Ack 7
09:28:53.603 chunk 7 4741 bytes
09:28:53.603 chunk 18 40067 bytes
09:28:53.604 in Ack 7
09:28:53.605 rpc 35 orchestration.dispatchCommand thread.visit
09:28:53.605 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:28:53.617 in Ack 18
09:28:53.811 exit 34 {"_tag":"Success","value":null}
09:28:54.108 fetch POST /api/observability/v1/traces
09:28:54.108 fetched POST /api/observability/v1/traces 204
09:28:54.883 chunk 18 40722 bytes
09:28:54.886 in Ack 18
09:28:55.339 in Ping 
09:28:55.888 fetch POST /api/observability/v1/traces
09:28:55.889 fetched POST /api/observability/v1/traces 204
09:28:57.551 chunk 18 43339 bytes
09:28:57.556 in Ack 18
09:28:57.801 chunk 18 43545 bytes
09:28:57.803 in Ack 18
09:28:58.051 chunk 18 43959 bytes
09:28:58.053 in Ack 18
09:28:58.301 chunk 18 44135 bytes
09:28:58.304 in Ack 18
09:28:58.557 fetch POST /api/observability/v1/traces
09:28:58.557 fetched POST /api/observability/v1/traces 204
09:29:00.054 chunk 18 44087 bytes
09:29:00.057 in Ack 18
09:29:00.339 in Ping 
09:29:00.810 chunk 7 4741 bytes
09:29:00.810 chunk 18 44087 bytes
09:29:00.811 in Ack 7
09:29:00.827 in Ack 18
09:29:01.058 fetch POST /api/observability/v1/traces
09:29:01.058 fetched POST /api/observability/v1/traces 204
09:29:01.315 chunk 18 45131 bytes
09:29:01.318 in Ack 18
09:29:02.319 fetch POST /api/observability/v1/traces
09:29:02.319 fetched POST /api/observability/v1/traces 204
09:29:02.964 chunk 7 4741 bytes
09:29:02.965 in Ack 7
09:29:03.606 rpc 36 orchestration.dispatchCommand thread.visit
09:29:03.606 exit 36 {"_tag":"Success","value":{"sequence":0}}
09:29:03.966 fetch POST /api/observability/v1/traces
09:29:03.966 fetched POST /api/observability/v1/traces 204
09:29:05.339 in Ping 
09:29:05.911 chunk 7 5048 bytes
09:29:05.912 chunk 18 45131 bytes
09:29:05.913 in Ack 7
09:29:05.922 in Ack 18
09:29:06.916 fetch POST /api/observability/v1/traces
09:29:06.916 fetched POST /api/observability/v1/traces 204
09:29:07.919 chunk 7 4743 bytes
09:29:07.922 in Ack 7
09:29:08.924 fetch POST /api/observability/v1/traces
09:29:08.924 fetched POST /api/observability/v1/traces 204
09:29:10.339 in Ping 
09:29:10.430 rpc 37 server.reportClientActivity
09:29:10.923 chunk 7 4743 bytes
09:29:10.924 in Ack 7
09:29:11.430 fetch POST /api/observability/v1/traces
09:29:11.430 fetched POST /api/observability/v1/traces 204
09:29:12.760 fetch POST /api/observability/v1/traces
09:29:12.760 fetched POST /api/observability/v1/traces 204
09:29:15.343 in Ping 
09:29:16.443 chunk 18 47663 bytes
09:29:16.445 in Ack 18
09:29:17.066 chunk 18 48665 bytes
09:29:17.070 in Ack 18
09:29:17.319 chunk 18 52152 bytes
09:29:17.325 in Ack 18
09:29:17.482 fetch POST /api/observability/v1/traces
09:29:17.482 fetched POST /api/observability/v1/traces 204
09:29:17.506 exit 37 {"_tag":"Success","value":null}
09:29:17.572 chunk 7 5050 bytes
09:29:17.572 chunk 18 54271 bytes
09:29:17.574 in Ack 7
09:29:17.576 rpc 38 orchestration.dispatchCommand thread.visit
09:29:17.576 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:29:17.595 in Ack 18
09:29:18.073 chunk 18 56600 bytes
09:29:18.077 in Ack 18
09:29:18.278 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:29:18.448 chunk 7 5050 bytes
09:29:18.449 chunk 18 57618 bytes
09:29:18.453 in Ack 7
09:29:18.465 in Ack 18
09:29:18.508 fetch POST /api/observability/v1/traces
09:29:18.508 fetched POST /api/observability/v1/traces 204
09:29:20.347 in Ping 
09:29:20.562 chunk 7 5050 bytes
09:29:20.563 in Ack 7
09:29:20.700 exit 39 {"_tag":"Success","value":{"sequence":198}}
09:29:20.815 chunk 18 57618 bytes
09:29:20.823 in Ack 18
09:29:20.932 rpc 40 orchestration.dispatchCommand runtime-request.respond
09:29:21.066 chunk 18 57661 bytes
09:29:21.070 in Ack 18
09:29:21.566 fetch POST /api/observability/v1/traces
09:29:21.566 fetched POST /api/observability/v1/traces 204
09:29:22.601 chunk 18 57709 bytes
09:29:22.606 in Ack 18
09:29:22.775 fetch POST /api/observability/v1/traces
09:29:22.775 fetched POST /api/observability/v1/traces 204
09:29:23.094 exit 40 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:29:23.107 chunk 18 57757 bytes
09:29:23.203 in Ack 18
09:29:23.219 in Interrupt 29
09:29:23.277 rpc 41 subscribeVcsStatus
09:29:23.535 rpc 42 server.reportClientActivity
09:29:23.611 chunk 18 57781 bytes
09:29:23.614 in Ack 18
09:29:24.101 fetch POST /api/observability/v1/traces
09:29:24.101 fetched POST /api/observability/v1/traces 204
09:29:24.124 chunk 7 4745 bytes
09:29:24.126 in Ack 7
09:29:24.376 chunk 18 57805 bytes
09:29:24.382 in Ack 18
09:29:24.626 chunk 7 4681 bytes
09:29:24.627 in Ack 7
09:29:25.129 fetch POST /api/observability/v1/traces
09:29:25.129 fetched POST /api/observability/v1/traces 204
09:29:25.351 in Ping 
09:29:25.526 chunk 41 1713 bytes
09:29:25.527 in Ack 41
09:29:26.629 exit 42 {"_tag":"Success","value":null}
09:29:27.576 rpc 43 orchestration.dispatchCommand thread.visit
09:29:27.576 exit 43 {"_tag":"Success","value":{"sequence":0}}
09:29:27.631 fetch POST /api/observability/v1/traces
09:29:27.631 fetched POST /api/observability/v1/traces 204
09:29:27.978 chunk 18 61041 bytes
09:29:27.981 in Ack 18
09:29:28.230 chunk 18 60993 bytes
09:29:28.233 in Ack 18
09:29:28.480 chunk 7 4681 bytes
09:29:28.481 chunk 18 61948 bytes
09:29:28.481 in Ack 7
09:29:28.490 in Ack 18
09:29:28.983 fetch POST /api/observability/v1/traces
09:29:28.984 fetched POST /api/observability/v1/traces 204
09:29:29.298 chunk 18 61948 bytes
09:29:29.301 in Ack 18
09:29:29.548 chunk 7 4681 bytes
09:29:29.549 in Ack 7
09:29:30.303 fetch POST /api/observability/v1/traces
09:29:30.303 fetched POST /api/observability/v1/traces 204
09:29:30.355 in Ping 
09:29:32.778 fetch POST /api/observability/v1/traces
09:29:32.778 fetched POST /api/observability/v1/traces 204
09:29:34.159 chunk 18 62581 bytes
09:29:34.161 in Ack 18
09:29:34.420 chunk 18 62965 bytes
09:29:34.423 in Ack 18
09:29:35.196 chunk 18 63015 bytes
09:29:35.205 fetch POST /api/observability/v1/traces
09:29:35.205 fetched POST /api/observability/v1/traces 204
09:29:35.212 in Ack 18
