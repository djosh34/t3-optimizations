# C, webapp, warm start, seed 1

Profile C (graded): historical EDGE-like. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.5s |
| Prompt reached the agent | 1.1s |
| Time to first text | 2.8s |
| Whole reply shown | 47.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.3s / 0.9s / 1.9s |
| Longest silence | 4.7s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 680374 / 369687 |
| Bytes the app received / sent | 461072 / 87292 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.8 | 321 | 357 |
| 20 | 1.2 | 705 | 758 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.9 | 1375 | 1409 |
| 47 | 1.9 | 1579 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1058/1058 1108/1108 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","user":"10/10 9/9","nodes":"809/809 1025/1025 809/809 1025/1025","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":290,"lastDataAt":1794150551738,"silences":0,"longestSilenceMs":1358,"longestRecoveredGapMs":1358},"stats":{"sendRate":3375,"deliveredRate":1833.7793578970047,"loss":0,"srttMs":689.9270538545736,"minRttMs":1,"sentPackets":118,"sentWireBytes":37971,"receivedPackets":132,"receivedWireBytes":65924,"lastHeardMs":11262,"unsyncedBytes":686,"inflightPackets":15,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1058/1058 1108/1108 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"778/778 778/778 660/660","messages":"456/456 550/550 901/901 550/550 451/451","assistant":"32/32 21/21 430/430","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","attempts":"662/662 662/662 638/638","providerThreads":"812/812","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":290,"lastDataAt":1794150561818,"silences":0,"longestSilenceMs":1358,"longestRecoveredGapMs":1358},"stats":{"sendRate":3000,"deliveredRate":1834.6118881118878,"loss":0,"srttMs":981.9667065651898,"minRttMs":903,"sentPackets":175,"sentWireBytes":49488,"receivedPackets":276,"receivedWireBytes":124732,"lastHeardMs":21342,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1108/1108 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"778/778 778/778 660/660","messages":"456/456 550/550 901/901 550/550 451/451 549/549","assistant":"32/32 21/21 918/918","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785","attempts":"662/662 662/662 638/638","providerThreads":"812/812","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":290,"lastDataAt":1794150571725,"silences":0,"longestSilenceMs":1358,"longestRecoveredGapMs":1358},"stats":{"sendRate":2904.607843137255,"deliveredRate":1692.696148145288,"loss":0.20439560439560436,"srttMs":1141.5564488828209,"minRttMs":903,"sentPackets":242,"sentWireBytes":63827,"receivedPackets":591,"receivedWireBytes":203610,"lastHeardMs":31249,"unsyncedBytes":0,"inflightPackets":7,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1058/1058 1108/1108 860/860 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1525/1554 932/932","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"778/778 778/778 660/660","messages":"456/456 550/550 901/901 550/550 451/451 549/549","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785 220/965 1132/1377","attempts":"662/662 662/662 638/638","providerThreads":"812/812","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":290,"lastDataAt":1794150581918,"silences":0,"longestSilenceMs":1358,"longestRecoveredGapMs":1358},"stats":{"sendRate":2904.607843137255,"deliveredRate":1150.2501559606774,"loss":0.14004020979020973,"srttMs":1141.5564488828209,"minRttMs":903,"sentPackets":290,"sentWireBytes":70650,"receivedPackets":912,"receivedWireBytes":292398,"lastHeardMs":41442,"unsyncedBytes":0,"inflightPackets":7,"duplicateBytes":0}}
- Scroll to end shows
- webapp {"objects":{"turnItems":"870/870 1058/1058 1108/1108 860/860 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1578/1578 956/956 1083/1083","frame":"1032/1032","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"778/778 778/778 660/660","messages":"456/456 550/550 901/901 550/550 451/451 550/550 393/549","assistant":"32/32 21/21 1340/1340 459/459","user":"10/10 9/9 64/64","nodes":"809/809 1025/1025 809/809 1025/1025 785/785 989/989 1401/1401 1025/1025","attempts":"662/662 662/662 638/638","providerThreads":"812/812","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":290,"lastDataAt":1794150592592,"silences":0,"longestSilenceMs":1358,"longestRecoveredGapMs":1358},"stats":{"sendRate":4778.199700953558,"deliveredRate":2704.7048739391057,"loss":0.11464736842105265,"srttMs":1074.132859093478,"minRttMs":903,"sentPackets":400,"sentWireBytes":104230,"receivedPackets":1128,"receivedWireBytes":447166,"lastHeardMs":52116,"unsyncedBytes":267,"inflightPackets":35,"duplicateBytes":0}}
- shim trace
15:09:00.820 fetch GET /api/auth/session
15:09:00.821 fetched GET /api/auth/session 200
15:09:00.917 fetch GET /.well-known/t3/environment
15:09:00.918 fetched GET /.well-known/t3/environment 200
15:09:00.946 fetch GET /.well-known/t3/environment
15:09:00.948 fetched GET /.well-known/t3/environment 200
15:09:00.953 rpc 0 subscribeServerConfig
15:09:00.954 chunk 0 38690 bytes
15:09:01.036 in Ack 0
15:09:01.039 rpc 1 subscribeServerLifecycle
15:09:01.041 rpc 2 subscribeServerLifecycle
15:09:01.044 fetch GET /api/auth/session
15:09:01.046 fetched GET /api/auth/session 200
15:09:01.047 rpc 3 subscribeDeviceState
15:09:01.049 rpc 4 preview.reportProfiles
15:09:01.145 rpc 5 subscribeProjectClones
15:09:01.170 fetch GET /api/orchestration/threads/mcp%3Ab19188b1-ffa0-48c5-8436-f361bf9c972b/bounded
15:09:01.191 fetch GET /api/orchestration/shell
15:09:01.230 rpc 6 server.refreshProviders
15:09:01.274 rpc 7 orchestration.dispatchCommand thread.visit
15:09:01.288 fetched GET /api/orchestration/shell 200
15:09:01.288 rpc 8 subscribeTerminalMetadata
15:09:01.289 rpc 9 subscribeVcsStatus
15:09:01.295 rpc 10 subscribePreviewEvents
15:09:01.298 rpc 11 orchestration.subscribeShell
15:09:01.299 chunk 11 2782 bytes
15:09:01.300 rpc 12 assets.createUrl
15:09:01.305 rpc 13 preview.list
15:09:01.305 rpc 14 projects.readFile
15:09:01.307 in Ack 11
15:09:01.373 exit 7 {"_tag":"Success","value":{"sequence":38}}
15:09:01.373 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"1d91ea88-0999-4b8f-8833-2b65f6777983","sessions":[]}}
15:09:01.373 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:09:01.548 rpc 15 server.reportClientActivity
15:09:01.660 chunk 1 3864 bytes
15:09:01.681 in Ack 1
15:09:01.739 chunk 2 3864 bytes
15:09:01.739 in Ack 2
15:09:01.859 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:08:47.834Z","configurableProviders":false,"con
15:09:01.859 chunk 9 1427 bytes
15:09:01.862 in Ack 9
15:09:01.913 fetch POST /api/observability/v1/traces
15:09:01.913 fetched POST /api/observability/v1/traces 204
15:09:01.989 chunk 11 944 bytes
15:09:01.990 exit 12 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:09:01.990 in Ack 11
15:09:02.189 chunk 3 876 bytes
15:09:02.189 exit 4 {"_tag":"Success","value":null}
15:09:02.189 chunk 5 4 bytes
15:09:02.189 chunk 8 36 bytes
15:09:02.189 exit 15 {"_tag":"Success","value":null}
15:09:02.190 in Ack 3
15:09:02.191 rpc 16 preview.reportProfiles
15:09:02.191 in Ack 5
15:09:02.191 in Ack 8
15:09:02.253 chunk 11 2801 bytes
15:09:02.254 in Ack 11
15:09:02.264 rpc 17 server.refreshProviders
15:09:02.264 exit 17 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:08:47.834Z","configurableProviders":false,"con
15:09:02.322 exit 16 {"_tag":"Success","value":null}
15:09:02.449 fetched GET /api/orchestration/threads/mcp%3Ab19188b1-ffa0-48c5-8436-f361bf9c972b/bounded 200
15:09:02.452 rpc 18 orchestration.subscribeThread
15:09:02.453 chunk 18 8305 bytes
15:09:02.475 in Interrupt 9
15:09:02.515 rpc 19 subscribeVcsStatus
15:09:02.574 in Ack 18
15:09:02.650 chunk 19 1427 bytes
15:09:02.651 in Ack 19
15:09:02.703 chunk 18 12060 bytes
15:09:02.704 in Ack 18
15:09:02.765 rpc 20 server.reportClientActivity
15:09:03.005 chunk 18 13532 bytes
15:09:03.005 fetch POST /api/observability/v1/traces
15:09:03.005 fetched POST /api/observability/v1/traces 204
15:09:03.009 in Ack 18
15:09:03.216 exit 20 {"_tag":"Success","value":null}
15:09:03.257 chunk 18 19104 bytes
15:09:03.258 in Ack 18
15:09:03.556 rpc 21 attachments.createUploadUrl
15:09:03.725 chunk 18 21223 bytes
15:09:03.750 in Ack 18
15:09:03.766 in Interrupt 19
15:09:03.775 rpc 22 subscribeVcsStatus
15:09:03.821 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-59f768ee-9164-46ae-8047-e25e43b6d8dc","expiresAt":1794151143809,"relativeUrl":"/api/attachments/upload/eyJ2ZX
15:09:03.821 chunk 22 1427 bytes
15:09:03.822 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTU5Zjc2OGVlLTkxNjQtNDZhZS04MDQ3LWUyNWU0M2I2ZDhkYyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1MTE0MzgwOX0.DheTUp3YSWGN7R4O5Vw95TYaR5S7x3ptlSRvXDPSV4k
15:09:03.823 in Ack 22
15:09:04.010 fetch POST /api/observability/v1/traces
15:09:04.010 fetched POST /api/observability/v1/traces 204
15:09:04.027 rpc 23 server.reportClientActivity
15:09:04.958 exit 23 {"_tag":"Success","value":null}
15:09:05.028 fetch POST /api/observability/v1/traces
15:09:05.028 fetched POST /api/observability/v1/traces 204
15:09:05.951 in Ping 
15:09:06.920 rpc 24 orchestration.dispatchCommand message.dispatch
15:09:07.579 exit 24 {"_tag":"Success","value":{"sequence":45}}
15:09:07.581 rpc 25 attachments.delete
15:09:07.925 fetch POST /api/observability/v1/traces
15:09:07.925 fetched POST /api/observability/v1/traces 204
15:09:08.079 chunk 18 23689 bytes
15:09:08.080 in Ack 18
15:09:08.329 chunk 18 23711 bytes
15:09:08.332 in Ack 18
15:09:08.580 chunk 11 944 bytes
15:09:08.583 chunk 18 23713 bytes
15:09:08.586 in Ack 11
15:09:08.598 in Ack 18
15:09:08.735 exit 25 {"_tag":"Success","value":null}
15:09:08.831 chunk 11 2823 bytes
15:09:08.831 chunk 18 25348 bytes
15:09:08.832 in Ack 11
15:09:08.833 rpc 26 orchestration.dispatchCommand thread.visit
15:09:08.888 in Ack 18
15:09:08.978 exit 26 {"_tag":"Success","value":{"sequence":68}}
15:09:09.082 fetch POST /api/observability/v1/traces
15:09:09.082 fetched POST /api/observability/v1/traces 204
15:09:09.083 chunk 11 944 bytes
15:09:09.083 in Ack 11
15:09:09.333 chunk 11 2823 bytes
15:09:09.334 in Ack 11
15:09:09.339 chunk 18 31374 bytes
15:09:09.340 in Ack 18
15:09:09.342 rpc 27 assets.createUrl
15:09:09.661 chunk 18 31178 bytes
15:09:09.903 in Ack 18
15:09:09.916 in Interrupt 22
15:09:09.920 rpc 28 subscribeVcsStatus
15:09:09.923 chunk 18 31988 bytes
15:09:09.925 in Ack 18
15:09:10.093 fetch POST /api/observability/v1/traces
15:09:10.093 fetched POST /api/observability/v1/traces 204
15:09:10.171 rpc 29 server.reportClientActivity
15:09:10.174 chunk 18 32767 bytes
15:09:10.209 in Ack 18
15:09:10.887 chunk 18 33339 bytes
15:09:10.887 chunk 28 1427 bytes
15:09:10.889 in Ack 18
15:09:10.899 in Ack 28
15:09:10.970 in Ping 
15:09:11.146 chunk 18 34550 bytes
15:09:11.157 in Ack 18
15:09:11.176 fetch POST /api/observability/v1/traces
15:09:11.176 fetched POST /api/observability/v1/traces 204
15:09:11.469 chunk 18 35276 bytes
15:09:11.481 in Ack 18
15:09:11.696 rpc 30 orchestration.dispatchCommand message.dispatch
15:09:12.250 exit 29 {"_tag":"Success","value":null}
15:09:12.483 fetch POST /api/observability/v1/traces
15:09:12.483 fetched POST /api/observability/v1/traces 204
15:09:12.943 exit 30 {"_tag":"Success","value":{"sequence":75}}
15:09:13.945 fetch POST /api/observability/v1/traces
15:09:13.945 fetched POST /api/observability/v1/traces 204
15:09:14.247 chunk 18 37812 bytes
15:09:14.249 in Ack 18
15:09:14.498 chunk 11 944 bytes
15:09:14.499 chunk 18 37832 bytes
15:09:14.499 in Ack 11
15:09:14.544 in Ack 18
15:09:14.749 chunk 18 37852 bytes
15:09:14.751 in Ack 18
15:09:15.223 chunk 18 37874 bytes
15:09:15.228 in Ack 18
15:09:15.250 fetch POST /api/observability/v1/traces
15:09:15.250 fetched POST /api/observability/v1/traces 204
15:09:15.503 chunk 18 37894 bytes
15:09:15.505 in Ack 18
15:09:15.754 chunk 18 37924 bytes
15:09:15.755 in Ack 18
15:09:15.970 in Ping 
15:09:16.003 chunk 18 37896 bytes
15:09:16.005 in Ack 18
15:09:16.255 chunk 18 37936 bytes
15:09:16.256 in Ack 18
15:09:16.505 chunk 18 37986 bytes
15:09:16.506 fetch POST /api/observability/v1/traces
15:09:16.506 fetched POST /api/observability/v1/traces 204
15:09:16.507 in Ack 18
15:09:16.755 chunk 11 2887 bytes
15:09:16.756 in Ack 11
15:09:16.762 chunk 18 38663 bytes
15:09:16.763 in Ack 18
15:09:17.012 chunk 18 39523 bytes
15:09:17.013 in Ack 18
15:09:17.263 chunk 18 41556 bytes
15:09:17.264 in Ack 18
15:09:17.508 fetch POST /api/observability/v1/traces
15:09:17.508 fetched POST /api/observability/v1/traces 204
15:09:17.513 chunk 18 41566 bytes
15:09:17.515 in Ack 18
15:09:17.763 chunk 18 41606 bytes
15:09:17.764 in Ack 18
15:09:18.016 chunk 18 41626 bytes
15:09:18.018 in Ack 18
15:09:18.266 chunk 18 42307 bytes
15:09:18.268 in Ack 18
15:09:18.516 chunk 18 42323 bytes
15:09:18.517 in Ack 18
15:09:18.532 fetch POST /api/observability/v1/traces
15:09:18.532 fetched POST /api/observability/v1/traces 204
15:09:18.766 chunk 18 43155 bytes
15:09:18.767 in Ack 18
15:09:18.846 rpc 31 orchestration.dispatchCommand thread.visit
15:09:19.050 chunk 18 43701 bytes
15:09:19.051 in Ack 18
15:09:19.302 chunk 18 43721 bytes
15:09:19.303 in Ack 18
15:09:19.513 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/5d9a232c-9029-411d-abd4-6a3320df8b44","expiresAt":1794154159513}}
15:09:19.552 chunk 18 43741 bytes
15:09:19.553 in Ack 18
15:09:19.768 fetch POST /api/observability/v1/traces
15:09:19.768 fetched POST /api/observability/v1/traces 204
15:09:19.802 chunk 18 43771 bytes
15:09:19.803 in Ack 18
15:09:19.812 exit 31 {"_tag":"Success","value":{"sequence":83}}
15:09:20.052 chunk 11 2887 bytes
15:09:20.052 in Ack 11
15:09:20.303 chunk 18 43821 bytes
15:09:20.304 in Ack 18
15:09:20.553 chunk 18 43851 bytes
15:09:20.554 in Ack 18
15:09:20.805 chunk 18 43871 bytes
15:09:20.806 fetch POST /api/observability/v1/traces
15:09:20.806 fetched POST /api/observability/v1/traces 204
15:09:20.807 in Ack 18
15:09:20.975 in Ping 
15:09:21.083 chunk 18 43901 bytes
15:09:21.084 in Ack 18
15:09:21.334 chunk 18 43921 bytes
15:09:21.336 in Ack 18
15:09:21.585 chunk 18 43951 bytes
15:09:21.586 in Ack 18
15:09:21.808 fetch POST /api/observability/v1/traces
15:09:21.808 fetched POST /api/observability/v1/traces 204
15:09:21.835 chunk 18 43981 bytes
15:09:21.836 in Ack 18
15:09:22.107 chunk 18 44001 bytes
15:09:22.109 in Ack 18
15:09:22.358 chunk 18 44031 bytes
15:09:22.360 in Ack 18
15:09:22.608 chunk 18 44051 bytes
15:09:22.609 in Ack 18
15:09:22.820 fetch POST /api/observability/v1/traces
15:09:22.820 fetched POST /api/observability/v1/traces 204
15:09:22.858 chunk 18 44081 bytes
15:09:22.859 in Ack 18
15:09:23.124 chunk 18 44101 bytes
15:09:23.125 in Ack 18
15:09:23.374 chunk 11 2887 bytes
15:09:23.375 chunk 18 45201 bytes
15:09:23.375 in Ack 11
15:09:23.380 in Ack 18
15:09:23.625 chunk 18 45216 bytes
15:09:23.630 in Ack 18
15:09:23.859 fetch POST /api/observability/v1/traces
15:09:23.859 fetched POST /api/observability/v1/traces 204
15:09:23.876 chunk 18 45246 bytes
15:09:23.877 in Ack 18
15:09:24.155 chunk 18 45306 bytes
15:09:24.156 in Ack 18
15:09:24.406 chunk 18 45336 bytes
15:09:24.407 in Ack 18
15:09:24.681 chunk 18 45381 bytes
15:09:24.683 in Ack 18
15:09:24.878 fetch POST /api/observability/v1/traces
15:09:24.878 fetched POST /api/observability/v1/traces 204
15:09:24.938 chunk 18 45411 bytes
15:09:24.939 in Ack 18
15:09:25.188 chunk 18 45456 bytes
15:09:25.189 in Ack 18
15:09:25.439 chunk 18 45486 bytes
15:09:25.440 in Ack 18
15:09:25.689 chunk 18 45516 bytes
15:09:25.691 in Ack 18
15:09:25.939 chunk 18 45591 bytes
15:09:25.941 in Ack 18
15:09:25.954 fetch POST /api/observability/v1/traces
15:09:25.954 fetched POST /api/observability/v1/traces 204
15:09:25.989 in Ping 
15:09:26.170 rpc 32 server.reportClientActivity
15:09:26.192 chunk 18 45606 bytes
15:09:26.193 in Ack 18
15:09:26.441 chunk 18 45636 bytes
15:09:26.444 in Ack 18
15:09:26.694 chunk 18 45681 bytes
15:09:26.696 in Ack 18
15:09:26.945 chunk 18 45726 bytes
15:09:26.946 in Ack 18
15:09:27.170 fetch POST /api/observability/v1/traces
15:09:27.170 fetched POST /api/observability/v1/traces 204
15:09:27.195 chunk 18 45741 bytes
15:09:27.196 in Ack 18
15:09:27.445 chunk 18 45786 bytes
15:09:27.447 in Ack 18
15:09:27.694 chunk 18 45831 bytes
15:09:27.696 in Ack 18
15:09:27.945 chunk 18 45876 bytes
15:09:27.946 in Ack 18
15:09:28.197 fetch POST /api/observability/v1/traces
15:09:28.197 fetched POST /api/observability/v1/traces 204
15:09:28.204 chunk 18 45906 bytes
15:09:28.205 in Ack 18
15:09:28.444 exit 32 {"_tag":"Success","value":null}
15:09:28.454 chunk 18 45936 bytes
15:09:28.455 in Ack 18
15:09:28.705 chunk 18 45981 bytes
15:09:28.706 in Ack 18
15:09:28.849 rpc 33 orchestration.dispatchCommand thread.visit
15:09:28.955 chunk 18 46011 bytes
15:09:28.956 in Ack 18
15:09:29.205 chunk 18 46041 bytes
15:09:29.206 fetch POST /api/observability/v1/traces
15:09:29.206 fetched POST /api/observability/v1/traces 204
15:09:29.208 in Ack 18
15:09:29.456 chunk 18 46086 bytes
15:09:29.457 in Ack 18
15:09:29.706 chunk 18 46116 bytes
15:09:29.707 in Ack 18
15:09:29.957 chunk 18 46161 bytes
15:09:29.958 in Ack 18
15:09:30.018 exit 33 {"_tag":"Success","value":{"sequence":86}}
15:09:30.205 chunk 11 2887 bytes
15:09:30.206 in Ack 11
15:09:30.224 chunk 18 46176 bytes
15:09:30.225 fetch POST /api/observability/v1/traces
15:09:30.225 fetched POST /api/observability/v1/traces 204
15:09:30.227 in Ack 18
15:09:30.474 chunk 18 46221 bytes
15:09:30.475 in Ack 18
15:09:30.724 chunk 18 46266 bytes
15:09:30.725 in Ack 18
15:09:30.977 chunk 18 46296 bytes
15:09:30.978 in Ack 18
15:09:30.990 in Ping 
15:09:31.228 chunk 18 46311 bytes
15:09:31.228 fetch POST /api/observability/v1/traces
15:09:31.229 fetched POST /api/observability/v1/traces 204
15:09:31.230 in Ack 18
15:09:31.478 chunk 18 46386 bytes
15:09:31.479 in Ack 18
15:09:31.728 chunk 18 46431 bytes
15:09:31.729 in Ack 18
15:09:31.978 chunk 18 46461 bytes
15:09:31.979 in Ack 18
15:09:32.228 chunk 18 46506 bytes
15:09:32.229 in Ack 18
15:09:32.272 fetch POST /api/observability/v1/traces
15:09:32.272 fetched POST /api/observability/v1/traces 204
15:09:32.479 chunk 18 46536 bytes
15:09:32.480 in Ack 18
15:09:32.729 chunk 18 46581 bytes
15:09:32.730 in Ack 18
15:09:32.980 chunk 18 46626 bytes
15:09:32.981 in Ack 18
15:09:33.229 chunk 18 46641 bytes
15:09:33.230 in Ack 18
15:09:33.480 chunk 18 46701 bytes
15:09:33.481 fetch POST /api/observability/v1/traces
15:09:33.481 fetched POST /api/observability/v1/traces 204
15:09:33.482 in Ack 18
15:09:33.825 chunk 18 46716 bytes
15:09:33.829 in Ack 18
15:09:34.075 chunk 18 46776 bytes
15:09:34.079 in Ack 18
15:09:34.326 chunk 18 46806 bytes
15:09:34.327 in Ack 18
15:09:34.482 fetch POST /api/observability/v1/traces
15:09:34.483 fetched POST /api/observability/v1/traces 204
15:09:34.579 chunk 18 46851 bytes
15:09:34.580 in Ack 18
15:09:34.829 chunk 18 46881 bytes
15:09:34.830 in Ack 18
15:09:35.079 chunk 18 46926 bytes
15:09:35.082 in Ack 18
15:09:35.330 chunk 18 46971 bytes
15:09:35.331 in Ack 18
15:09:35.580 fetch POST /api/observability/v1/traces
15:09:35.580 fetched POST /api/observability/v1/traces 204
15:09:35.581 chunk 18 47016 bytes
15:09:35.583 in Ack 18
15:09:35.831 chunk 18 47031 bytes
15:09:35.832 in Ack 18
15:09:35.990 in Ping 
15:09:36.082 chunk 18 47091 bytes
15:09:36.083 in Ack 18
15:09:36.349 chunk 18 47106 bytes
15:09:36.350 in Ack 18
15:09:36.584 fetch POST /api/observability/v1/traces
15:09:36.584 fetched POST /api/observability/v1/traces 204
15:09:36.600 chunk 18 47166 bytes
15:09:36.601 in Ack 18
15:09:36.909 chunk 18 47196 bytes
15:09:36.910 in Ack 18
15:09:37.161 chunk 18 47226 bytes
15:09:37.162 in Ack 18
15:09:37.411 chunk 18 47271 bytes
15:09:37.412 in Ack 18
15:09:37.603 fetch POST /api/observability/v1/traces
15:09:37.603 fetched POST /api/observability/v1/traces 204
15:09:37.661 chunk 18 47316 bytes
15:09:37.664 in Ack 18
15:09:37.911 chunk 18 47346 bytes
15:09:37.912 in Ack 18
15:09:38.162 chunk 18 47391 bytes
15:09:38.163 in Ack 18
15:09:38.427 chunk 18 47406 bytes
15:09:38.428 in Ack 18
15:09:38.669 fetch POST /api/observability/v1/traces
15:09:38.669 fetched POST /api/observability/v1/traces 204
15:09:38.677 chunk 18 47466 bytes
15:09:38.678 in Ack 18
15:09:38.927 chunk 18 47481 bytes
15:09:38.930 in Ack 18
15:09:39.179 chunk 18 47541 bytes
15:09:39.186 in Ack 18
15:09:39.430 chunk 18 47571 bytes
15:09:39.431 in Ack 18
15:09:39.680 fetch POST /api/observability/v1/traces
15:09:39.680 fetched POST /api/observability/v1/traces 204
15:09:39.682 chunk 18 47616 bytes
15:09:39.683 in Ack 18
15:09:39.933 chunk 18 47646 bytes
15:09:39.934 in Ack 18
15:09:40.186 chunk 18 47703 bytes
15:09:40.187 in Ack 18
15:09:40.684 fetch POST /api/observability/v1/traces
15:09:40.684 fetched POST /api/observability/v1/traces 204
15:09:40.938 chunk 11 944 bytes
15:09:40.938 in Ack 11
15:09:40.990 in Ping 
15:09:41.189 chunk 18 49840 bytes
15:09:41.190 in Ack 18
15:09:41.440 chunk 18 49842 bytes
15:09:41.441 in Ack 18
15:09:41.943 fetch POST /api/observability/v1/traces
15:09:41.944 fetched POST /api/observability/v1/traces 204
15:09:41.944 chunk 18 50853 bytes
15:09:41.945 in Ack 18
15:09:42.196 chunk 18 54369 bytes
15:09:42.198 in Ack 18
15:09:42.553 rpc 34 orchestration.dispatchCommand runtime-request.respond
15:09:42.847 chunk 11 3197 bytes
15:09:42.848 in Ack 11
15:09:42.946 fetch POST /api/observability/v1/traces
15:09:42.946 fetched POST /api/observability/v1/traces 204
15:09:43.117 chunk 18 55335 bytes
15:09:43.119 in Ack 18
15:09:43.368 chunk 18 56713 bytes
15:09:43.369 in Ack 18
15:09:43.956 chunk 18 59158 bytes
15:09:43.956 exit 34 {"_tag":"Success","value":{"sequence":97}}
15:09:43.957 in Ack 18
15:09:43.994 rpc 35 orchestration.dispatchCommand thread.visit
15:09:44.120 fetch POST /api/observability/v1/traces
15:09:44.120 fetched POST /api/observability/v1/traces 204
15:09:44.206 chunk 18 53553 bytes
15:09:44.207 in Ack 18
15:09:44.459 chunk 18 53633 bytes
15:09:44.460 in Ack 18
15:09:44.724 chunk 11 944 bytes
15:09:44.725 chunk 18 53643 bytes
15:09:44.725 in Ack 11
15:09:44.737 in Ack 18
15:09:44.974 chunk 18 53673 bytes
15:09:44.975 in Ack 18
15:09:45.209 fetch POST /api/observability/v1/traces
15:09:45.209 fetched POST /api/observability/v1/traces 204
15:09:45.231 chunk 18 53683 bytes
15:09:45.234 in Ack 18
15:09:45.420 exit 35 {"_tag":"Success","value":{"sequence":100}}
15:09:45.420 rpc 36 orchestration.dispatchCommand runtime-request.respond
15:09:45.480 chunk 11 2889 bytes
15:09:45.481 in Ack 11
15:09:45.991 in Ping 
15:09:46.232 chunk 18 53904 bytes
15:09:46.234 in Ack 18
15:09:46.330 fetch POST /api/observability/v1/traces
15:09:46.330 fetched POST /api/observability/v1/traces 204
15:09:46.493 chunk 18 53914 bytes
15:09:46.495 in Ack 18
15:09:46.743 chunk 18 53944 bytes
15:09:46.744 in Ack 18
15:09:46.822 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
15:09:47.166 chunk 18 59685 bytes
15:09:47.169 in Ack 18
15:09:47.183 in Interrupt 28
15:09:47.212 rpc 37 subscribeVcsStatus
15:09:47.416 chunk 18 59705 bytes
15:09:47.417 in Ack 18
15:09:47.463 rpc 38 server.reportClientActivity
15:09:47.496 fetch POST /api/observability/v1/traces
15:09:47.496 fetched POST /api/observability/v1/traces 204
15:09:47.667 chunk 18 60761 bytes
15:09:47.669 in Ack 18
15:09:47.919 chunk 18 60781 bytes
15:09:47.920 in Ack 18
15:09:48.169 chunk 18 60811 bytes
15:09:48.170 in Ack 18
15:09:48.397 chunk 37 1427 bytes
15:09:48.397 in Ack 37
15:09:48.418 chunk 18 60841 bytes
15:09:48.420 in Ack 18
15:09:48.669 chunk 18 60861 bytes
15:09:48.669 fetch POST /api/observability/v1/traces
15:09:48.669 fetched POST /api/observability/v1/traces 204
15:09:48.671 in Ack 18
15:09:48.919 chunk 18 60891 bytes
15:09:48.921 in Ack 18
15:09:49.174 chunk 18 60911 bytes
15:09:49.175 in Ack 18
15:09:49.424 chunk 18 60931 bytes
15:09:49.425 in Ack 18
15:09:49.508 exit 38 {"_tag":"Success","value":null}
15:09:49.671 fetch POST /api/observability/v1/traces
15:09:49.672 fetched POST /api/observability/v1/traces 204
15:09:49.674 chunk 18 60981 bytes
15:09:49.675 in Ack 18
15:09:49.925 chunk 18 60991 bytes
15:09:49.926 in Ack 18
15:09:50.175 chunk 18 61031 bytes
15:09:50.176 in Ack 18
15:09:50.450 chunk 18 61051 bytes
15:09:50.451 in Ack 18
15:09:50.676 fetch POST /api/observability/v1/traces
15:09:50.676 fetched POST /api/observability/v1/traces 204
15:09:50.700 chunk 18 61071 bytes
15:09:50.702 in Ack 18
15:09:50.951 chunk 18 61091 bytes
15:09:50.952 in Ack 18
15:09:51.034 in Ping 
15:09:51.170 rpc 39 server.reportClientActivity
15:09:51.201 chunk 18 61131 bytes
15:09:51.202 in Ack 18
15:09:51.455 chunk 18 61143 bytes
15:09:51.457 in Ack 18
15:09:51.544 chunk 11 2889 bytes
15:09:51.545 in Ack 11
15:09:51.704 fetch POST /api/observability/v1/traces
15:09:51.704 fetched POST /api/observability/v1/traces 204
15:09:51.705 chunk 18 61159 bytes
15:09:51.707 in Ack 18
15:09:51.959 chunk 18 61179 bytes
15:09:51.961 in Ack 18
15:09:52.228 chunk 18 61199 bytes
15:09:52.231 in Ack 18
15:09:52.479 chunk 18 61239 bytes
15:09:52.481 in Ack 18
15:09:52.717 fetch POST /api/observability/v1/traces
15:09:52.717 fetched POST /api/observability/v1/traces 204
15:09:52.729 chunk 18 61259 bytes
15:09:52.730 in Ack 18
15:09:52.850 rpc 40 orchestration.dispatchCommand thread.visit
15:09:52.980 chunk 18 62342 bytes
15:09:52.981 in Ack 18
15:09:53.265 chunk 18 62387 bytes
15:09:53.267 in Ack 18
15:09:53.510 chunk 18 62423 bytes
15:09:53.512 in Ack 18
15:09:53.731 fetch POST /api/observability/v1/traces
15:09:53.731 fetched POST /api/observability/v1/traces 204
15:09:53.760 chunk 18 62453 bytes
15:09:53.761 in Ack 18
15:09:53.839 exit 39 {"_tag":"Success","value":null}
15:09:53.987 exit 40 {"_tag":"Success","value":{"sequence":103}}
15:09:54.013 chunk 18 62498 bytes
15:09:54.015 in Ack 18
15:09:54.159 chunk 11 2889 bytes
15:09:54.160 in Ack 11
15:09:54.263 chunk 18 62534 bytes
15:09:54.264 in Ack 18
15:09:54.514 chunk 18 62549 bytes
15:09:54.515 in Ack 18
15:09:54.762 fetch POST /api/observability/v1/traces
15:09:54.762 fetched POST /api/observability/v1/traces 204
15:09:54.764 chunk 18 62564 bytes
15:09:54.765 in Ack 18
15:09:55.022 chunk 18 62579 bytes
15:09:55.023 in Ack 18
15:09:55.272 chunk 18 62678 bytes
15:09:55.273 in Ack 18
15:09:55.523 chunk 18 62738 bytes
15:09:55.524 in Ack 18
15:09:55.766 fetch POST /api/observability/v1/traces
15:09:55.766 fetched POST /api/observability/v1/traces 204
15:09:55.773 chunk 18 62795 bytes
15:09:55.774 in Ack 18
15:09:56.051 chunk 18 62825 bytes
15:09:56.051 in Ping 
15:09:56.052 in Ack 18
15:09:56.301 chunk 18 62894 bytes
15:09:56.303 in Ack 18
15:09:56.551 chunk 18 62924 bytes
15:09:56.552 in Ack 18
15:09:56.775 fetch POST /api/observability/v1/traces
15:09:56.775 fetched POST /api/observability/v1/traces 204
15:09:56.801 chunk 18 62960 bytes
15:09:56.803 in Ack 18
15:09:57.051 chunk 18 62996 bytes
15:09:57.055 in Ack 18
15:09:57.301 chunk 18 63044 bytes
15:09:57.302 in Ack 18
15:09:57.551 chunk 18 63092 bytes
15:09:57.552 in Ack 18
15:09:57.801 chunk 18 61728 bytes
15:09:57.802 in Ack 18
15:09:57.907 fetch POST /api/observability/v1/traces
15:09:57.907 fetched POST /api/observability/v1/traces 204
15:09:57.931 chunk 11 944 bytes
15:09:57.931 in Ack 11
15:09:58.051 chunk 18 61860 bytes
15:09:58.052 in Ack 18
15:09:58.182 chunk 11 2825 bytes
15:09:58.182 in Ack 11
15:09:58.184 rpc 41 orchestration.dispatchCommand thread.visit
15:09:58.329 chunk 18 61858 bytes
15:09:58.330 in Ack 18
