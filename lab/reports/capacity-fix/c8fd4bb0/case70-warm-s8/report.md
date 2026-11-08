# case70, webapp, warm start, seed 8

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 61 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.2s |
| Prompt reached the agent | 1.0s |
| Time to first text | 8.7s |
| Whole reply shown | 60.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 11.4s / 2.5s / 8.4s |
| Longest silence | 14.4s |
| Approval shown after it was asked | 13.8s |
| Bytes on the wire down / up | 44015 / 30019 |
| Bytes the app received / sent | 33197 / 53925 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 299 | 357 |
| 20 | 2.4 | 659 | 758 |
| 30 | 2.9 | 1045 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 5.4 | 1090 | 1301 |
| 60 | 6.6 | 1425 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174435623,"silences":0,"longestSilenceMs":1225,"longestRecoveredGapMs":1225},"stats":{"sendRate":4500,"deliveredRate":1433.1210191082803,"loss":0,"srttMs":7.147069303129321,"minRttMs":2,"sentPackets":84,"sentWireBytes":28839,"receivedPackets":102,"receivedWireBytes":49500,"lastHeardMs":8851,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058 393/1078","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21 380/425","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174446571,"silences":1,"longestSilenceMs":2314,"longestRecoveredGapMs":2314},"stats":{"sendRate":2250,"deliveredRate":859.9363366323181,"loss":0.25390909090909086,"srttMs":543.1750999352084,"minRttMs":1747,"sentPackets":102,"sentWireBytes":33442,"receivedPackets":113,"receivedWireBytes":54190,"lastHeardMs":19800,"unsyncedBytes":0,"inflightPackets":6,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058 1028/1078 146/849","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21 878/923","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174456742,"silences":1,"longestSilenceMs":2314,"longestRecoveredGapMs":2314},"stats":{"sendRate":3000,"deliveredRate":531.1111111111111,"loss":0.6923076923076923,"srttMs":845.2550935245071,"minRttMs":1019,"sentPackets":154,"sentWireBytes":46493,"receivedPackets":129,"receivedWireBytes":58953,"lastHeardMs":29971,"unsyncedBytes":132,"inflightPackets":40,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058 1078/1078 843/849","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21 1323/1340","user":"10/10 9/9","attempts":"661/661 661/661 45/637","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723","runtimeRequests":"218/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174466303,"silences":1,"longestSilenceMs":2314,"longestRecoveredGapMs":2314},"stats":{"sendRate":3000,"deliveredRate":489.0857542773905,"loss":0.6219615384615383,"srttMs":992.0703059797007,"minRttMs":1019,"sentPackets":207,"sentWireBytes":55198,"receivedPackets":150,"receivedWireBytes":65877,"lastHeardMs":39531,"unsyncedBytes":0,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058 1078/1078 843/849 62/1550","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"661/661 661/661 45/637","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723","runtimeRequests":"970/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174476534,"silences":3,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":3000,"deliveredRate":323.7404060806146,"loss":0.6186897252747251,"srttMs":992.0703059797007,"minRttMs":1019,"sentPackets":244,"sentWireBytes":60480,"receivedPackets":161,"receivedWireBytes":69760,"lastHeardMs":49762,"unsyncedBytes":0,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1103/1103 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 660/1550","thread":"951/951","frame":"1031/1031","runs":"1014/1014 971/971","providerTurns":"776/776","assistant":"32/32 21/21 1340/1340 203/248","user":"10/10 9/9","attempts":"661/661 661/661 274/637","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"456/456 548/548 25/901","checkpoints":"779/779 613/613 401/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":303,"lastDataAt":1794174486939,"silences":3,"longestSilenceMs":3318,"longestRecoveredGapMs":3318},"stats":{"sendRate":3000,"deliveredRate":1034.642857142857,"loss":0.18181818181818177,"srttMs":1484.3433420593515,"minRttMs":1019,"sentPackets":278,"sentWireBytes":74171,"receivedPackets":179,"receivedWireBytes":77322,"lastHeardMs":60167,"unsyncedBytes":535,"inflightPackets":64,"duplicateBytes":0}}
- shim trace
21:47:07.052 fetch GET /api/auth/session
21:47:07.054 fetched GET /api/auth/session 200
21:47:07.155 fetch GET /.well-known/t3/environment
21:47:07.156 fetched GET /.well-known/t3/environment 200
21:47:07.165 fetch GET /.well-known/t3/environment
21:47:07.166 fetched GET /.well-known/t3/environment 200
21:47:07.169 rpc 0 subscribeServerConfig
21:47:07.169 chunk 0 38678 bytes
21:47:07.187 in Ack 0
21:47:07.189 rpc 1 subscribeServerLifecycle
21:47:07.190 rpc 2 subscribeServerLifecycle
21:47:07.192 fetch GET /api/auth/session
21:47:07.193 fetched GET /api/auth/session 200
21:47:07.193 rpc 3 subscribeDeviceState
21:47:07.195 rpc 4 preview.reportProfiles
21:47:07.253 rpc 5 subscribeProjectClones
21:47:07.271 fetch GET /api/orchestration/threads/mcp%3Ac505c99a-8668-45d3-b29d-974601dd38d4/bounded
21:47:07.276 fetch GET /api/orchestration/shell
21:47:07.295 rpc 6 server.refreshProviders
21:47:07.333 rpc 7 orchestration.dispatchCommand thread.visit
21:47:07.333 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:47:07.345 fetched GET /api/orchestration/shell 200
21:47:07.345 rpc 8 subscribeTerminalMetadata
21:47:07.346 rpc 9 subscribeVcsStatus
21:47:07.346 rpc 10 subscribePreviewEvents
21:47:07.348 rpc 11 orchestration.subscribeShell
21:47:07.349 chunk 11 2779 bytes
21:47:07.350 rpc 12 assets.createUrl
21:47:07.351 rpc 13 preview.list
21:47:07.359 rpc 14 projects.readFile
21:47:07.368 in Ack 11
21:47:07.601 rpc 15 server.reportClientActivity
21:47:08.031 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"bec1e718-d14d-42af-9c75-0fa159e3b992","sessions":[]}}
21:47:08.031 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:47:08.031 chunk 1 3861 bytes
21:47:08.038 in Ack 1
21:47:08.154 fetch POST /api/observability/v1/traces
21:47:08.154 fetched POST /api/observability/v1/traces 204
21:47:08.264 chunk 2 3861 bytes
21:47:08.264 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:46:55.410Z","configurableProviders":false,"con
21:47:08.264 chunk 9 472 bytes
21:47:08.264 in Ack 2
21:47:08.268 in Ack 9
21:47:08.379 chunk 11 2755 bytes
21:47:08.379 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:47:08.379 in Ack 11
21:47:08.589 chunk 3 876 bytes
21:47:08.589 exit 4 {"_tag":"Success","value":null}
21:47:08.589 chunk 5 4 bytes
21:47:08.589 chunk 8 36 bytes
21:47:08.589 exit 15 {"_tag":"Success","value":null}
21:47:08.589 in Ack 3
21:47:08.593 rpc 16 preview.reportProfiles
21:47:08.593 in Ack 5
21:47:08.593 in Ack 8
21:47:08.629 chunk 11 2798 bytes
21:47:08.630 in Ack 11
21:47:08.669 exit 16 {"_tag":"Success","value":null}
21:47:08.790 fetched GET /api/orchestration/threads/mcp%3Ac505c99a-8668-45d3-b29d-974601dd38d4/bounded 200
21:47:08.793 rpc 17 orchestration.subscribeThread
21:47:08.793 chunk 17 8294 bytes
21:47:08.802 in Interrupt 9
21:47:08.804 rpc 18 subscribeVcsStatus
21:47:08.805 in Ack 17
21:47:08.919 chunk 18 472 bytes
21:47:08.919 in Ack 18
21:47:09.044 chunk 17 11932 bytes
21:47:09.045 in Ack 17
21:47:09.101 rpc 19 server.reportClientActivity
21:47:09.259 exit 19 {"_tag":"Success","value":null}
21:47:09.271 fetch POST /api/observability/v1/traces
21:47:09.271 fetched POST /api/observability/v1/traces 204
21:47:09.296 chunk 17 13517 bytes
21:47:09.297 in Ack 17
21:47:09.574 rpc 20 attachments.createUploadUrl
21:47:09.580 chunk 17 18614 bytes
21:47:09.734 in Ack 17
21:47:09.783 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-4c117dc9-d938-4012-966b-3605dfe658b1","expiresAt":1794175029768,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:47:09.784 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTRjMTE3ZGM5LWQ5MzgtNDAxMi05NjZiLTM2MDVkZmU2NThiMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTAyOTc2OH0.Em3ru0JuIcVAAkXsGvhNz4Xs945vYdnPTq6dNyfws_w
21:47:09.831 chunk 17 21197 bytes
21:47:09.832 in Ack 17
21:47:09.839 in Interrupt 18
21:47:09.841 rpc 21 subscribeVcsStatus
21:47:09.887 chunk 21 472 bytes
21:47:09.887 in Ack 21
21:47:10.093 rpc 22 server.reportClientActivity
21:47:10.298 fetch POST /api/observability/v1/traces
21:47:10.298 fetched POST /api/observability/v1/traces 204
21:47:10.897 exit 22 {"_tag":"Success","value":null}
21:47:11.900 fetch POST /api/observability/v1/traces
21:47:11.900 fetched POST /api/observability/v1/traces 204
21:47:12.168 in Ping 
21:47:12.749 rpc 23 orchestration.dispatchCommand message.dispatch
21:47:13.379 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:47:13.381 rpc 24 attachments.delete
21:47:13.749 fetch POST /api/observability/v1/traces
21:47:13.750 fetched POST /api/observability/v1/traces 204
21:47:13.912 chunk 17 23652 bytes
21:47:13.915 in Ack 17
21:47:14.163 chunk 17 23626 bytes
21:47:14.164 in Ack 17
21:47:14.230 exit 24 {"_tag":"Success","value":null}
21:47:14.416 chunk 17 23676 bytes
21:47:14.417 in Ack 17
21:47:14.666 chunk 11 2820 bytes
21:47:14.667 in Ack 11
21:47:14.668 rpc 25 orchestration.dispatchCommand thread.visit
21:47:14.668 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:47:14.917 fetch POST /api/observability/v1/traces
21:47:14.917 fetched POST /api/observability/v1/traces 204
21:47:14.918 chunk 17 27441 bytes
21:47:14.919 in Ack 17
21:47:14.921 rpc 26 assets.createUrl
21:47:15.167 chunk 11 2820 bytes
21:47:15.167 in Ack 11
21:47:15.172 chunk 17 28103 bytes
21:47:15.173 in Ack 17
21:47:15.624 chunk 17 31336 bytes
21:47:15.625 in Ack 17
21:47:15.925 fetch POST /api/observability/v1/traces
21:47:15.925 fetched POST /api/observability/v1/traces 204
21:47:16.436 rpc 27 orchestration.dispatchCommand message.dispatch
21:47:17.168 in Ping 
21:47:17.436 fetch POST /api/observability/v1/traces
21:47:17.436 fetched POST /api/observability/v1/traces 204
21:47:18.855 fetch POST /api/observability/v1/traces
21:47:18.855 fetched POST /api/observability/v1/traces 204
21:47:19.292 chunk 17 33803 bytes
21:47:19.292 exit 27 {"_tag":"Success","value":{"sequence":75}}
21:47:19.293 in Ack 17
21:47:19.814 chunk 17 33823 bytes
21:47:19.816 in Ack 17
21:47:20.253 chunk 17 33845 bytes
21:47:20.254 in Ack 17
21:47:20.294 fetch POST /api/observability/v1/traces
21:47:20.294 fetched POST /api/observability/v1/traces 204
21:47:21.714 chunk 17 34081 bytes
21:47:21.715 in Ack 17
21:47:22.168 in Ping 
21:47:22.716 fetch POST /api/observability/v1/traces
21:47:22.716 fetched POST /api/observability/v1/traces 204
21:47:24.028 chunk 17 34349 bytes
21:47:24.033 in Ack 17
21:47:24.464 chunk 17 34379 bytes
21:47:24.465 in Ack 17
21:47:24.824 chunk 17 34461 bytes
21:47:24.825 in Ack 17
21:47:25.034 fetch POST /api/observability/v1/traces
21:47:25.034 fetched POST /api/observability/v1/traces 204
21:47:25.591 chunk 17 34511 bytes
21:47:25.592 in Ack 17
21:47:26.573 chunk 17 34533 bytes
21:47:26.574 in Ack 17
21:47:26.602 fetch POST /api/observability/v1/traces
21:47:26.602 fetched POST /api/observability/v1/traces 204
21:47:27.168 in Ping 
21:47:27.910 chunk 17 34683 bytes
21:47:27.911 in Ack 17
21:47:28.635 chunk 17 34813 bytes
21:47:28.637 in Ack 17
21:47:28.913 fetch POST /api/observability/v1/traces
21:47:28.913 fetched POST /api/observability/v1/traces 204
21:47:29.543 chunk 17 34873 bytes
21:47:29.544 in Ack 17
21:47:30.528 fetch POST /api/observability/v1/traces
21:47:30.528 fetched POST /api/observability/v1/traces 204
21:47:31.143 chunk 17 35013 bytes
21:47:31.145 in Ack 17
21:47:31.962 chunk 17 35075 bytes
21:47:31.963 in Ack 17
21:47:32.146 fetch POST /api/observability/v1/traces
21:47:32.146 fetched POST /api/observability/v1/traces 204
21:47:32.174 in Ping 
21:47:32.354 chunk 17 35097 bytes
21:47:32.355 in Ack 17
21:47:32.407 rpc 28 server.reportClientActivity
21:47:32.604 chunk 17 35107 bytes
21:47:32.605 in Ack 17
21:47:32.856 chunk 17 35137 bytes
21:47:32.857 in Ack 17
21:47:33.111 chunk 17 35157 bytes
21:47:33.112 in Ack 17
21:47:33.365 fetch POST /api/observability/v1/traces
21:47:33.365 fetched POST /api/observability/v1/traces 204
21:47:33.638 chunk 17 35331 bytes
21:47:33.639 in Ack 17
21:47:34.198 chunk 17 35363 bytes
21:47:34.199 in Ack 17
21:47:34.537 chunk 17 35403 bytes
21:47:34.538 in Ack 17
21:47:34.641 fetch POST /api/observability/v1/traces
21:47:34.641 fetched POST /api/observability/v1/traces 204
21:47:34.884 chunk 17 35415 bytes
21:47:34.886 in Ack 17
21:47:35.664 chunk 17 35465 bytes
21:47:35.665 in Ack 17
21:47:35.887 fetch POST /api/observability/v1/traces
21:47:35.887 fetched POST /api/observability/v1/traces 204
21:47:36.403 chunk 17 35515 bytes
21:47:36.406 in Ack 17
21:47:36.743 chunk 17 35535 bytes
21:47:36.744 in Ack 17
21:47:37.177 in Ping 
21:47:37.407 fetch POST /api/observability/v1/traces
21:47:37.407 fetched POST /api/observability/v1/traces 204
21:47:37.559 chunk 17 35655 bytes
21:47:37.561 in Ack 17
21:47:38.243 chunk 17 35687 bytes
21:47:38.244 in Ack 17
21:47:38.573 fetch POST /api/observability/v1/traces
21:47:38.573 fetched POST /api/observability/v1/traces 204
21:47:38.829 chunk 17 35707 bytes
21:47:38.831 in Ack 17
21:47:39.085 chunk 17 35699 bytes
21:47:39.086 in Ack 17
21:47:39.831 fetch POST /api/observability/v1/traces
21:47:39.831 fetched POST /api/observability/v1/traces 204
21:47:40.806 chunk 17 35789 bytes
21:47:40.807 in Ack 17
21:47:41.143 chunk 17 35859 bytes
21:47:41.144 in Ack 17
21:47:41.393 chunk 17 35879 bytes
21:47:41.394 in Ack 17
21:47:41.650 chunk 17 35889 bytes
21:47:41.651 in Ack 17
21:47:41.808 fetch POST /api/observability/v1/traces
21:47:41.808 fetched POST /api/observability/v1/traces 204
21:47:41.988 chunk 17 35929 bytes
21:47:41.989 in Ack 17
21:47:42.178 in Ping 
21:47:42.414 chunk 17 36107 bytes
21:47:42.415 in Ack 17
21:47:42.753 chunk 17 36137 bytes
21:47:42.754 in Ack 17
21:47:42.990 fetch POST /api/observability/v1/traces
21:47:42.990 fetched POST /api/observability/v1/traces 204
21:47:43.381 chunk 17 36227 bytes
21:47:43.382 in Ack 17
21:47:43.712 chunk 17 36237 bytes
21:47:43.713 in Ack 17
21:47:43.967 chunk 17 36247 bytes
21:47:43.968 in Ack 17
21:47:44.223 chunk 17 36275 bytes
21:47:44.224 in Ack 17
21:47:44.384 fetch POST /api/observability/v1/traces
21:47:44.384 fetched POST /api/observability/v1/traces 204
21:47:44.563 chunk 17 36305 bytes
21:47:44.565 in Ack 17
21:47:44.821 chunk 17 36327 bytes
21:47:44.822 in Ack 17
21:47:45.161 chunk 17 36375 bytes
21:47:45.162 in Ack 17
21:47:45.566 fetch POST /api/observability/v1/traces
21:47:45.566 fetched POST /api/observability/v1/traces 204
21:47:45.695 chunk 17 36377 bytes
21:47:45.696 in Ack 17
21:47:46.697 fetch POST /api/observability/v1/traces
21:47:46.697 fetched POST /api/observability/v1/traces 204
21:47:47.178 in Ping 
21:47:47.391 chunk 17 36411 bytes
21:47:47.392 in Ack 17
21:47:47.413 in Interrupt 28
21:47:48.393 fetch POST /api/observability/v1/traces
21:47:48.393 fetched POST /api/observability/v1/traces 204
21:47:50.389 fetch POST /api/observability/v1/traces
21:47:50.389 fetched POST /api/observability/v1/traces 204
21:47:52.179 in Ping 
21:47:57.179 in Ping 
21:47:57.408 rpc 29 server.reportClientActivity
21:47:57.864 chunk 17 37419 bytes
21:47:57.866 in Ack 17
21:47:58.158 rpc 30 orchestration.dispatchCommand runtime-request.respond
21:47:58.408 fetch POST /api/observability/v1/traces
21:47:58.408 fetched POST /api/observability/v1/traces 204
21:48:00.384 fetch POST /api/observability/v1/traces
21:48:00.384 fetched POST /api/observability/v1/traces 204
21:48:01.654 chunk 17 39930 bytes
21:48:01.654 exit 30 {"_tag":"Success","value":{"sequence":95}}
21:48:01.655 in Ack 17
21:48:01.916 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:48:02.181 in Ping 
21:48:02.656 fetch POST /api/observability/v1/traces
21:48:02.656 fetched POST /api/observability/v1/traces 204
21:48:03.051 chunk 17 40012 bytes
21:48:03.052 in Ack 17
21:48:03.489 chunk 17 40034 bytes
21:48:03.490 in Ack 17
21:48:04.053 fetch POST /api/observability/v1/traces
21:48:04.053 fetched POST /api/observability/v1/traces 204
21:48:04.452 chunk 17 40114 bytes
21:48:04.453 in Ack 17
21:48:05.111 chunk 17 40134 bytes
21:48:05.112 in Ack 17
21:48:05.454 fetch POST /api/observability/v1/traces
21:48:05.454 fetched POST /api/observability/v1/traces 204
21:48:05.550 chunk 17 40154 bytes
21:48:05.551 in Ack 17
21:48:06.068 chunk 17 40204 bytes
21:48:06.069 in Ack 17
21:48:06.507 chunk 17 40236 bytes
21:48:06.509 in Ack 17
21:48:06.552 fetch POST /api/observability/v1/traces
21:48:06.552 fetched POST /api/observability/v1/traces 204
21:48:06.940 chunk 17 40266 bytes
21:48:06.941 in Ack 17
21:48:07.185 in Ping 
21:48:07.375 chunk 17 40278 bytes
21:48:07.376 in Ack 17
21:48:07.812 chunk 17 40338 bytes
21:48:07.813 in Ack 17
21:48:07.942 fetch POST /api/observability/v1/traces
21:48:07.942 fetched POST /api/observability/v1/traces 204
21:48:08.692 chunk 17 40480 bytes
21:48:08.693 in Ack 17
21:48:09.694 fetch POST /api/observability/v1/traces
21:48:09.694 fetched POST /api/observability/v1/traces 204
21:48:09.701 chunk 17 40578 bytes
21:48:09.702 in Ack 17
21:48:10.158 chunk 17 40628 bytes
21:48:10.159 in Ack 17
21:48:10.596 chunk 17 40662 bytes
21:48:10.598 in Ack 17
21:48:10.712 fetch POST /api/observability/v1/traces
21:48:10.712 fetched POST /api/observability/v1/traces 204
21:48:11.853 chunk 17 40764 bytes
21:48:11.855 in Ack 17
21:48:12.112 chunk 17 40784 bytes
21:48:12.113 in Ack 17
21:48:12.189 in Ping 
21:48:12.362 chunk 17 40878 bytes
21:48:12.363 in Ack 17
21:48:12.413 in Interrupt 29
21:48:12.615 chunk 17 40888 bytes
21:48:12.616 in Ack 17
21:48:12.856 fetch POST /api/observability/v1/traces
21:48:12.856 fetched POST /api/observability/v1/traces 204
21:48:13.138 chunk 17 40966 bytes
21:48:13.139 in Ack 17
21:48:14.140 fetch POST /api/observability/v1/traces
21:48:14.140 fetched POST /api/observability/v1/traces 204
21:48:14.633 chunk 17 41170 bytes
21:48:14.635 in Ack 17
21:48:15.501 chunk 17 41324 bytes
21:48:15.503 in Ack 17
21:48:15.635 fetch POST /api/observability/v1/traces
21:48:15.636 fetched POST /api/observability/v1/traces 204
21:48:15.841 chunk 17 41354 bytes
21:48:15.842 in Ack 17
