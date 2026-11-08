# case50, webapp, warm start, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.4s |
| Prompt reached the agent | 0.9s |
| Time to first text | 2.9s |
| Whole reply shown | 46.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.7s / 1.1s / 2.7s |
| Longest silence | 4.3s |
| Approval shown after it was asked | 2.3s |
| Bytes on the wire down / up | 294239 / 109814 |
| Bytes the app received / sent | 240011 / 75385 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.6 | 299 | 361 |
| 20 | 1.7 | 693 | 762 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.5 | 1378 | 1431 |
| 46 | 1.6 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 145/774","providerThreads":"744/808","providerSessions":"2696/2696","messages":"456/456 546/546 322/901","nodes":"1019/1019 809/809 809/809 15/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":198,"lastDataAt":1794165501595,"silences":0,"longestSilenceMs":1362,"longestRecoveredGapMs":1362},"stats":{"sendRate":6992.083695880623,"deliveredRate":3564.155187109882,"loss":0,"srttMs":44.00802354499129,"minRttMs":1,"sentPackets":115,"sentWireBytes":34695,"receivedPackets":113,"receivedWireBytes":52456,"lastHeardMs":9890,"unsyncedBytes":1009,"inflightPackets":16,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 425/435","runs":"1014/1014 971/971 453/839","user":"10/10 9/9","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 109/656","providerThreads":"808/808","providerSessions":"2696/2696","messages":"456/456 546/546 599/901 546/546","nodes":"1019/1019 809/809 809/809 917/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"206/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":198,"lastDataAt":1794165511787,"silences":0,"longestSilenceMs":1362,"longestRecoveredGapMs":1362},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.3446785714285714,"srttMs":468.7192271873058,"minRttMs":651,"sentPackets":176,"sentWireBytes":48722,"receivedPackets":161,"receivedWireBytes":74128,"lastHeardMs":20082,"unsyncedBytes":0,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 933/933","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 609/656","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 545/545 451/451","nodes":"1019/1019 809/809 809/809 1019/1019 627/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"545/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":198,"lastDataAt":1794165521845,"silences":0,"longestSilenceMs":1362,"longestRecoveredGapMs":1362},"stats":{"sendRate":4500,"deliveredRate":1883.042528735632,"loss":0.4229166666666666,"srttMs":637.9107145433788,"minRttMs":594,"sentPackets":239,"sentWireBytes":60795,"receivedPackets":259,"receivedWireBytes":106613,"lastHeardMs":30140,"unsyncedBytes":0,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1546/1546 216/926","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 545/545 451/451","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":198,"lastDataAt":1794165532161,"silences":0,"longestSilenceMs":1362,"longestRecoveredGapMs":1362},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.48828984411421905,"srttMs":637.9107145433788,"minRttMs":594,"sentPackets":293,"sentWireBytes":69739,"receivedPackets":383,"receivedWireBytes":159146,"lastHeardMs":40457,"unsyncedBytes":399,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 549/549","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","messages":"456/456 546/546 901/901 546/546 546/546 451/451 545/545","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":198,"lastDataAt":1794165543406,"silences":0,"longestSilenceMs":1362,"longestRecoveredGapMs":1362},"stats":{"sendRate":4372.641509433962,"deliveredRate":1457.5471698113206,"loss":0.5,"srttMs":798.530395310517,"minRttMs":594,"sentPackets":393,"sentWireBytes":98290,"receivedPackets":531,"receivedWireBytes":242797,"lastHeardMs":51701,"unsyncedBytes":0,"inflightPackets":68,"duplicateBytes":0}}
- shim trace
19:18:11.963 fetch GET /api/auth/session
19:18:11.964 fetched GET /api/auth/session 200
19:18:12.090 fetch GET /.well-known/t3/environment
19:18:12.091 fetched GET /.well-known/t3/environment 200
19:18:12.099 fetch GET /.well-known/t3/environment
19:18:12.100 fetched GET /.well-known/t3/environment 200
19:18:12.103 rpc 0 subscribeServerConfig
19:18:12.103 chunk 0 38722 bytes
19:18:12.128 in Ack 0
19:18:12.129 rpc 1 subscribeServerLifecycle
19:18:12.130 rpc 2 subscribeServerLifecycle
19:18:12.132 fetch GET /api/auth/session
19:18:12.134 fetched GET /api/auth/session 200
19:18:12.135 rpc 3 subscribeDeviceState
19:18:12.136 rpc 4 preview.reportProfiles
19:18:12.173 rpc 5 subscribeProjectClones
19:18:12.189 fetch GET /api/orchestration/threads/mcp%3A858b359d-afe2-4960-a049-f6a7e6a48c1a/bounded
19:18:12.194 fetch GET /api/orchestration/shell
19:18:12.213 rpc 6 server.refreshProviders
19:18:12.248 rpc 7 orchestration.dispatchCommand thread.visit
19:18:12.248 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:18:12.257 fetched GET /api/orchestration/shell 200
19:18:12.258 rpc 8 subscribeTerminalMetadata
19:18:12.258 rpc 9 subscribeVcsStatus
19:18:12.259 rpc 10 subscribePreviewEvents
19:18:12.261 rpc 11 orchestration.subscribeShell
19:18:12.261 chunk 11 2790 bytes
19:18:12.262 rpc 12 assets.createUrl
19:18:12.263 rpc 13 preview.list
19:18:12.263 rpc 14 projects.readFile
19:18:12.269 in Ack 11
19:18:12.511 rpc 15 server.reportClientActivity
19:18:12.530 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"8c090af6-e0e6-4401-8713-a7ca2f997c35","sessions":[]}}
19:18:12.530 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:18:12.749 chunk 1 3872 bytes
19:18:12.749 in Ack 1
19:18:13.091 fetch POST /api/observability/v1/traces
19:18:13.091 fetched POST /api/observability/v1/traces 204
19:18:13.229 chunk 2 3872 bytes
19:18:13.229 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:18:00.042Z","configurableProviders":false,"con
19:18:13.229 chunk 9 808 bytes
19:18:13.229 in Ack 2
19:18:13.232 in Ack 9
19:18:13.429 chunk 11 2766 bytes
19:18:13.429 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:18:13.430 in Ack 11
19:18:13.679 chunk 11 2809 bytes
19:18:13.680 in Ack 11
19:18:13.819 chunk 3 876 bytes
19:18:13.819 exit 4 {"_tag":"Success","value":null}
19:18:13.819 chunk 5 4 bytes
19:18:13.819 chunk 8 36 bytes
19:18:13.819 exit 15 {"_tag":"Success","value":null}
19:18:13.820 in Ack 3
19:18:13.822 in Ack 5
19:18:13.823 in Ack 8
19:18:14.058 fetched GET /api/orchestration/threads/mcp%3A858b359d-afe2-4960-a049-f6a7e6a48c1a/bounded 200
19:18:14.061 rpc 16 orchestration.subscribeThread
19:18:14.061 chunk 16 8283 bytes
19:18:14.071 in Interrupt 9
19:18:14.073 rpc 17 subscribeVcsStatus
19:18:14.122 in Ack 16
19:18:14.220 chunk 17 808 bytes
19:18:14.221 in Ack 17
19:18:14.232 fetch POST /api/observability/v1/traces
19:18:14.232 fetched POST /api/observability/v1/traces 204
19:18:14.312 chunk 16 8323 bytes
19:18:14.312 in Ack 16
19:18:14.334 rpc 18 server.reportClientActivity
19:18:14.561 chunk 16 12694 bytes
19:18:14.562 in Ack 16
19:18:14.811 chunk 16 13502 bytes
19:18:14.812 in Ack 16
19:18:14.971 rpc 19 attachments.createUploadUrl
19:18:15.166 exit 18 {"_tag":"Success","value":null}
19:18:15.168 chunk 16 17217 bytes
19:18:15.180 in Ack 16
19:18:15.249 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-7fde94bf-c6df-4fa6-bad0-9b8e3e02d8d9","expiresAt":1794166095198,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:18:15.251 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTdmZGU5NGJmLWM2ZGYtNGZhNi1iYWQwLTliOGUzZTAyZDhkOSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjA5NTE5OH0.zWGkTnbXLJNawg0kFPjk7oVWLWen0k9nEkNKAbRvGAo
19:18:15.313 fetch POST /api/observability/v1/traces
19:18:15.313 fetched POST /api/observability/v1/traces 204
19:18:15.418 chunk 16 19072 bytes
19:18:15.419 in Ack 16
19:18:15.668 chunk 16 21199 bytes
19:18:15.669 in Ack 16
19:18:15.675 in Interrupt 17
19:18:15.677 rpc 20 subscribeVcsStatus
19:18:15.748 chunk 20 808 bytes
19:18:15.748 in Ack 20
19:18:15.929 rpc 21 server.reportClientActivity
19:18:16.420 fetch POST /api/observability/v1/traces
19:18:16.420 fetched POST /api/observability/v1/traces 204
19:18:17.102 in Ping 
19:18:17.167 exit 21 {"_tag":"Success","value":null}
19:18:18.155 rpc 22 orchestration.dispatchCommand message.dispatch
19:18:18.710 fetch POST /api/observability/v1/traces
19:18:18.710 fetched POST /api/observability/v1/traces 204
19:18:18.769 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:18:18.771 rpc 23 attachments.delete
19:18:18.834 exit 23 {"_tag":"Success","value":null}
19:18:19.270 chunk 16 23643 bytes
19:18:19.271 in Ack 16
19:18:19.520 chunk 11 2895 bytes
19:18:19.521 chunk 16 24254 bytes
19:18:19.521 in Ack 11
19:18:19.569 in Ack 16
19:18:19.770 fetch POST /api/observability/v1/traces
19:18:19.770 fetched POST /api/observability/v1/traces 204
19:18:19.771 chunk 16 24304 bytes
19:18:19.772 in Ack 16
19:18:20.021 chunk 11 2831 bytes
19:18:20.022 in Ack 11
19:18:20.023 rpc 24 orchestration.dispatchCommand thread.visit
19:18:20.023 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:18:20.272 chunk 11 2831 bytes
19:18:20.272 chunk 16 25276 bytes
19:18:20.272 in Ack 11
19:18:20.277 in Ack 16
19:18:20.522 chunk 16 31326 bytes
19:18:20.523 in Ack 16
19:18:20.526 rpc 25 assets.createUrl
19:18:21.006 chunk 16 31940 bytes
19:18:21.006 fetch POST /api/observability/v1/traces
19:18:21.006 fetched POST /api/observability/v1/traces 204
19:18:21.008 in Ack 16
19:18:21.015 in Interrupt 20
19:18:21.017 rpc 26 subscribeVcsStatus
19:18:21.267 rpc 27 server.reportClientActivity
19:18:21.784 rpc 28 orchestration.dispatchCommand message.dispatch
19:18:22.009 fetch POST /api/observability/v1/traces
19:18:22.009 fetched POST /api/observability/v1/traces 204
19:18:22.046 chunk 26 808 bytes
19:18:22.047 in Ack 26
19:18:22.103 in Ping 
19:18:22.348 chunk 16 32715 bytes
19:18:22.349 in Ack 16
19:18:22.395 exit 27 {"_tag":"Success","value":null}
19:18:22.598 chunk 16 32715 bytes
19:18:22.599 in Ack 16
19:18:22.849 chunk 16 33283 bytes
19:18:22.850 in Ack 16
19:18:22.953 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:18:23.349 chunk 16 35719 bytes
19:18:23.350 in Ack 16
19:18:23.402 fetch POST /api/observability/v1/traces
19:18:23.402 fetched POST /api/observability/v1/traces 204
19:18:23.599 chunk 16 35731 bytes
19:18:23.600 in Ack 16
19:18:23.860 chunk 16 35751 bytes
19:18:23.861 in Ack 16
19:18:24.109 chunk 16 35771 bytes
19:18:24.111 in Ack 16
19:18:24.364 chunk 16 35837 bytes
19:18:24.365 in Ack 16
19:18:24.601 fetch POST /api/observability/v1/traces
19:18:24.601 fetched POST /api/observability/v1/traces 204
19:18:24.746 chunk 16 35849 bytes
19:18:24.748 in Ack 16
19:18:25.023 chunk 16 35861 bytes
19:18:25.025 in Ack 16
19:18:25.284 chunk 16 35881 bytes
19:18:25.288 in Ack 16
19:18:25.617 fetch POST /api/observability/v1/traces
19:18:25.617 fetched POST /api/observability/v1/traces 204
19:18:25.618 chunk 16 35903 bytes
19:18:25.620 in Ack 16
19:18:26.119 chunk 16 35925 bytes
19:18:26.120 in Ack 16
19:18:26.369 chunk 16 35947 bytes
19:18:26.370 in Ack 16
19:18:26.620 fetch POST /api/observability/v1/traces
19:18:26.620 fetched POST /api/observability/v1/traces 204
19:18:27.030 chunk 16 35987 bytes
19:18:27.031 in Ack 16
19:18:27.104 in Ping 
19:18:27.280 chunk 11 2895 bytes
19:18:27.280 chunk 16 36015 bytes
19:18:27.281 in Ack 11
19:18:27.332 in Ack 16
19:18:27.530 chunk 16 36025 bytes
19:18:27.531 in Ack 16
19:18:27.781 chunk 16 36111 bytes
19:18:27.782 in Ack 16
19:18:28.032 chunk 16 36121 bytes
19:18:28.032 fetch POST /api/observability/v1/traces
19:18:28.032 fetched POST /api/observability/v1/traces 204
19:18:28.033 in Ack 16
19:18:28.283 chunk 16 36141 bytes
19:18:28.284 in Ack 16
19:18:28.533 chunk 16 36171 bytes
19:18:28.534 in Ack 16
19:18:28.783 chunk 16 36219 bytes
19:18:28.784 in Ack 16
19:18:29.033 chunk 16 36239 bytes
19:18:29.034 in Ack 16
19:18:29.042 fetch POST /api/observability/v1/traces
19:18:29.042 fetched POST /api/observability/v1/traces 204
19:18:29.283 chunk 16 36279 bytes
19:18:29.284 in Ack 16
19:18:29.533 chunk 16 36299 bytes
19:18:29.534 in Ack 16
19:18:29.783 chunk 16 36319 bytes
19:18:29.784 in Ack 16
19:18:30.025 rpc 29 orchestration.dispatchCommand thread.visit
19:18:30.025 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:18:30.070 chunk 16 36369 bytes
19:18:30.071 in Ack 16
19:18:30.285 fetch POST /api/observability/v1/traces
19:18:30.285 fetched POST /api/observability/v1/traces 204
19:18:30.433 chunk 16 36399 bytes
19:18:30.434 in Ack 16
19:18:30.849 chunk 16 37068 bytes
19:18:30.850 in Ack 16
19:18:31.435 fetch POST /api/observability/v1/traces
19:18:31.435 fetched POST /api/observability/v1/traces 204
19:18:31.465 chunk 16 37110 bytes
19:18:31.466 in Ack 16
19:18:31.788 chunk 16 37148 bytes
19:18:31.789 in Ack 16
19:18:32.038 chunk 16 37158 bytes
19:18:32.039 in Ack 16
19:18:32.104 in Ping 
19:18:32.289 chunk 16 37196 bytes
19:18:32.290 in Ack 16
19:18:32.467 fetch POST /api/observability/v1/traces
19:18:32.467 fetched POST /api/observability/v1/traces 204
19:18:32.539 chunk 16 37226 bytes
19:18:32.540 in Ack 16
19:18:32.789 chunk 16 37256 bytes
19:18:32.790 in Ack 16
19:18:33.039 chunk 16 37278 bytes
19:18:33.040 in Ack 16
19:18:33.289 chunk 16 37288 bytes
19:18:33.290 in Ack 16
19:18:33.539 chunk 16 37330 bytes
19:18:33.540 in Ack 16
19:18:33.551 fetch POST /api/observability/v1/traces
19:18:33.551 fetched POST /api/observability/v1/traces 204
19:18:33.789 chunk 16 37340 bytes
19:18:33.790 in Ack 16
19:18:34.041 chunk 16 37368 bytes
19:18:34.042 in Ack 16
19:18:34.289 chunk 16 37398 bytes
19:18:34.290 in Ack 16
19:18:34.540 chunk 16 37436 bytes
19:18:34.540 in Ack 16
19:18:34.790 chunk 16 37448 bytes
19:18:34.791 in Ack 16
19:18:34.801 fetch POST /api/observability/v1/traces
19:18:34.801 fetched POST /api/observability/v1/traces 204
19:18:35.041 chunk 16 37468 bytes
19:18:35.042 in Ack 16
19:18:35.598 chunk 16 37530 bytes
19:18:35.599 in Ack 16
19:18:35.847 chunk 16 37558 bytes
19:18:35.848 in Ack 16
19:18:36.042 fetch POST /api/observability/v1/traces
19:18:36.043 fetched POST /api/observability/v1/traces 204
19:18:36.099 chunk 11 2895 bytes
19:18:36.099 chunk 16 37578 bytes
19:18:36.100 in Ack 11
19:18:36.104 in Ack 16
19:18:36.350 chunk 16 37588 bytes
19:18:36.351 in Ack 16
19:18:36.600 chunk 16 38470 bytes
19:18:36.601 in Ack 16
19:18:36.850 chunk 16 40503 bytes
19:18:36.851 in Ack 16
19:18:37.101 chunk 16 40523 bytes
19:18:37.101 fetch POST /api/observability/v1/traces
19:18:37.101 fetched POST /api/observability/v1/traces 204
19:18:37.102 in Ack 16
19:18:37.122 in Ping 
19:18:37.342 rpc 30 server.reportClientActivity
19:18:37.351 chunk 16 40551 bytes
19:18:37.352 in Ack 16
19:18:37.601 chunk 16 40573 bytes
19:18:37.602 in Ack 16
19:18:38.101 chunk 16 41869 bytes
19:18:38.102 in Ack 16
19:18:38.120 fetch POST /api/observability/v1/traces
19:18:38.120 fetched POST /api/observability/v1/traces 204
19:18:38.352 chunk 16 41929 bytes
19:18:38.353 in Ack 16
19:18:38.601 chunk 16 41973 bytes
19:18:38.602 in Ack 16
19:18:38.852 chunk 16 41988 bytes
19:18:38.853 in Ack 16
19:18:39.117 chunk 16 42063 bytes
19:18:39.118 in Ack 16
19:18:39.354 fetch POST /api/observability/v1/traces
19:18:39.354 fetched POST /api/observability/v1/traces 204
19:18:39.367 chunk 16 42093 bytes
19:18:39.371 in Ack 16
19:18:39.617 chunk 16 42123 bytes
19:18:39.618 in Ack 16
19:18:39.867 chunk 16 42180 bytes
19:18:39.868 in Ack 16
19:18:40.026 rpc 31 orchestration.dispatchCommand thread.visit
19:18:40.026 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:18:40.118 chunk 16 43245 bytes
19:18:40.119 in Ack 16
19:18:40.368 chunk 16 44213 bytes
19:18:40.369 in Ack 16
19:18:40.377 fetch POST /api/observability/v1/traces
19:18:40.377 fetched POST /api/observability/v1/traces 204
19:18:40.869 chunk 16 44231 bytes
19:18:40.870 in Ack 16
19:18:41.119 chunk 16 44276 bytes
19:18:41.120 in Ack 16
19:18:41.370 chunk 16 44822 bytes
19:18:41.371 in Ack 16
19:18:41.618 exit 30 {"_tag":"Success","value":null}
19:18:41.620 chunk 16 44909 bytes
19:18:41.622 in Ack 16
19:18:41.871 chunk 16 44939 bytes
19:18:41.871 fetch POST /api/observability/v1/traces
19:18:41.871 fetched POST /api/observability/v1/traces 204
19:18:41.872 in Ack 16
19:18:42.120 chunk 16 44984 bytes
19:18:42.121 in Ack 16
19:18:42.130 in Ping 
19:18:42.370 chunk 16 44999 bytes
19:18:42.371 in Ack 16
19:18:42.620 chunk 16 45017 bytes
19:18:42.621 in Ack 16
19:18:42.870 chunk 16 45035 bytes
19:18:42.871 in Ack 16
19:18:42.892 fetch POST /api/observability/v1/traces
19:18:42.892 fetched POST /api/observability/v1/traces 204
19:18:43.107 chunk 11 2895 bytes
19:18:43.107 in Ack 11
19:18:43.121 chunk 16 45725 bytes
19:18:43.122 in Ack 16
19:18:43.371 chunk 16 45770 bytes
19:18:43.372 in Ack 16
19:18:43.646 chunk 16 45815 bytes
19:18:43.647 in Ack 16
19:18:43.896 chunk 16 45830 bytes
19:18:43.897 in Ack 16
19:18:44.108 fetch POST /api/observability/v1/traces
19:18:44.108 fetched POST /api/observability/v1/traces 204
19:18:44.147 chunk 16 45893 bytes
19:18:44.148 in Ack 16
19:18:44.397 chunk 16 45908 bytes
19:18:44.398 in Ack 16
19:18:44.648 chunk 16 45938 bytes
19:18:44.650 in Ack 16
19:18:44.899 chunk 16 46769 bytes
19:18:44.900 in Ack 16
19:18:45.149 chunk 16 46862 bytes
19:18:45.149 fetch POST /api/observability/v1/traces
19:18:45.149 fetched POST /api/observability/v1/traces 204
19:18:45.150 in Ack 16
19:18:45.502 chunk 16 46922 bytes
19:18:45.503 in Ack 16
19:18:45.752 chunk 16 46967 bytes
19:18:45.753 in Ack 16
19:18:46.002 chunk 16 47012 bytes
19:18:46.003 in Ack 16
19:18:46.151 fetch POST /api/observability/v1/traces
19:18:46.151 fetched POST /api/observability/v1/traces 204
19:18:46.252 chunk 16 47027 bytes
19:18:46.253 in Ack 16
19:18:46.503 chunk 16 47072 bytes
19:18:46.504 in Ack 16
19:18:46.754 chunk 16 47102 bytes
19:18:46.755 in Ack 16
19:18:47.004 chunk 16 47132 bytes
19:18:47.005 in Ack 16
19:18:47.130 in Ping 
19:18:47.254 fetch POST /api/observability/v1/traces
19:18:47.254 fetched POST /api/observability/v1/traces 204
19:18:47.254 chunk 16 47207 bytes
19:18:47.255 in Ack 16
19:18:47.504 chunk 16 47222 bytes
19:18:47.505 in Ack 16
19:18:47.754 chunk 16 47267 bytes
19:18:47.755 in Ack 16
19:18:48.004 chunk 16 47297 bytes
19:18:48.005 in Ack 16
19:18:48.254 chunk 16 47342 bytes
19:18:48.255 in Ack 16
19:18:48.292 fetch POST /api/observability/v1/traces
19:18:48.292 fetched POST /api/observability/v1/traces 204
19:18:48.428 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/7f50d63c-fa63-4ecd-8ca2-83d68a180818","expiresAt":1794169128428}}
19:18:48.504 chunk 16 47372 bytes
19:18:48.505 in Ack 16
19:18:48.754 chunk 16 47402 bytes
19:18:48.755 in Ack 16
19:18:49.004 chunk 16 47447 bytes
19:18:49.005 in Ack 16
19:18:49.254 chunk 16 47492 bytes
19:18:49.255 in Ack 16
19:18:49.429 fetch POST /api/observability/v1/traces
19:18:49.429 fetched POST /api/observability/v1/traces 204
19:18:49.504 chunk 16 47522 bytes
19:18:49.505 in Ack 16
19:18:49.754 chunk 16 47552 bytes
19:18:49.755 in Ack 16
19:18:50.005 chunk 16 47609 bytes
19:18:50.006 in Ack 16
19:18:50.506 fetch POST /api/observability/v1/traces
19:18:50.506 fetched POST /api/observability/v1/traces 204
19:18:51.757 chunk 16 52110 bytes
19:18:51.759 in Ack 16
19:18:51.934 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:18:52.130 in Ping 
19:18:52.760 fetch POST /api/observability/v1/traces
19:18:52.760 fetched POST /api/observability/v1/traces 204
19:18:52.805 exit 32 {"_tag":"Success","value":{"sequence":97}}
19:18:53.055 chunk 16 54591 bytes
19:18:53.056 in Ack 16
19:18:53.216 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:18:53.308 chunk 16 54613 bytes
19:18:53.310 in Ack 16
19:18:53.559 chunk 16 54623 bytes
19:18:53.560 in Ack 16
19:18:53.775 fetch POST /api/observability/v1/traces
19:18:53.775 fetched POST /api/observability/v1/traces 204
19:18:53.810 chunk 16 54663 bytes
19:18:53.811 in Ack 16
19:18:54.189 chunk 16 54663 bytes
19:18:54.189 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:18:54.190 in Ack 16
19:18:54.439 chunk 16 54683 bytes
19:18:54.440 in Ack 16
19:18:54.593 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:18:54.690 chunk 16 54693 bytes
19:18:54.691 in Ack 16
19:18:54.812 fetch POST /api/observability/v1/traces
19:18:54.812 fetched POST /api/observability/v1/traces 204
19:18:54.940 chunk 16 54705 bytes
19:18:54.942 in Ack 16
19:18:55.190 chunk 16 54760 bytes
19:18:55.191 in Ack 16
19:18:55.418 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:18:55.441 chunk 16 54800 bytes
19:18:55.442 in Ack 16
19:18:55.847 chunk 16 54900 bytes
19:18:55.848 in Ack 16
19:18:55.943 fetch POST /api/observability/v1/traces
19:18:55.943 fetched POST /api/observability/v1/traces 204
19:18:56.098 chunk 16 54936 bytes
19:18:56.099 in Ack 16
19:18:56.351 chunk 16 54966 bytes
19:18:56.352 in Ack 16
19:18:56.602 chunk 16 55006 bytes
19:18:56.603 in Ack 16
19:18:56.852 chunk 16 55016 bytes
19:18:56.853 in Ack 16
19:18:57.100 fetch POST /api/observability/v1/traces
19:18:57.100 fetched POST /api/observability/v1/traces 204
19:18:57.103 chunk 11 2897 bytes
19:18:57.103 chunk 16 55046 bytes
19:18:57.103 in Ack 11
19:18:57.105 rpc 35 orchestration.dispatchCommand thread.visit
19:18:57.105 exit 35 {"_tag":"Success","value":{"sequence":0}}
19:18:57.107 in Ack 16
19:18:57.139 in Ping 
19:18:57.353 chunk 16 55077 bytes
19:18:57.354 in Ack 16
19:18:57.603 chunk 16 55097 bytes
19:18:57.604 in Ack 16
19:18:57.853 chunk 16 55117 bytes
19:18:57.854 in Ack 16
19:18:58.106 chunk 16 55147 bytes
19:18:58.106 fetch POST /api/observability/v1/traces
19:18:58.106 fetched POST /api/observability/v1/traces 204
19:18:58.107 in Ack 16
19:18:58.357 chunk 16 55159 bytes
19:18:58.358 in Ack 16
19:18:58.608 chunk 16 56223 bytes
19:18:58.609 in Ack 16
19:18:58.858 chunk 16 57657 bytes
19:18:58.859 in Ack 16
19:18:59.107 chunk 11 2897 bytes
19:18:59.108 chunk 16 57687 bytes
19:18:59.108 in Ack 11
19:18:59.113 fetch POST /api/observability/v1/traces
19:18:59.113 fetched POST /api/observability/v1/traces 204
19:18:59.114 in Ack 16
19:18:59.358 chunk 16 57697 bytes
19:18:59.359 in Ack 16
19:18:59.607 chunk 16 57717 bytes
19:18:59.608 in Ack 16
19:18:59.857 chunk 16 59871 bytes
19:18:59.859 in Ack 16
19:18:59.867 in Interrupt 26
19:18:59.891 rpc 36 subscribeVcsStatus
19:19:00.108 chunk 16 59891 bytes
19:19:00.109 in Ack 16
19:19:00.116 fetch POST /api/observability/v1/traces
19:19:00.116 fetched POST /api/observability/v1/traces 204
19:19:00.144 rpc 37 server.reportClientActivity
19:19:00.358 chunk 16 60951 bytes
19:19:00.359 in Ack 16
19:19:00.608 chunk 16 60973 bytes
19:19:00.609 in Ack 16
19:19:00.880 chunk 36 808 bytes
19:19:00.880 chunk 16 61967 bytes
19:19:00.880 in Ack 36
19:19:00.883 in Ack 16
19:19:01.140 chunk 16 61997 bytes
19:19:01.141 in Ack 16
19:19:01.200 fetch POST /api/observability/v1/traces
19:19:01.200 fetched POST /api/observability/v1/traces 204
19:19:01.401 chunk 16 62042 bytes
19:19:01.403 in Ack 16
19:19:01.652 chunk 16 62057 bytes
19:19:01.653 in Ack 16
19:19:01.903 chunk 16 62087 bytes
19:19:01.904 in Ack 16
19:19:02.140 in Ping 
19:19:02.152 chunk 11 2897 bytes
19:19:02.153 chunk 16 62153 bytes
19:19:02.153 in Ack 11
19:19:02.155 in Ack 16
19:19:02.270 exit 37 {"_tag":"Success","value":null}
19:19:02.343 rpc 38 server.reportClientActivity
19:19:02.419 chunk 16 62183 bytes
19:19:02.419 fetch POST /api/observability/v1/traces
19:19:02.419 fetched POST /api/observability/v1/traces 204
19:19:02.421 in Ack 16
19:19:02.669 chunk 16 62234 bytes
19:19:02.670 in Ack 16
19:19:02.922 chunk 16 62264 bytes
19:19:02.923 in Ack 16
19:19:03.172 chunk 16 62309 bytes
19:19:03.173 in Ack 16
19:19:03.214 exit 38 {"_tag":"Success","value":null}
19:19:03.421 chunk 16 62375 bytes
19:19:03.422 fetch POST /api/observability/v1/traces
19:19:03.422 fetched POST /api/observability/v1/traces 204
19:19:03.423 in Ack 16
19:19:03.672 chunk 16 62390 bytes
19:19:03.673 in Ack 16
19:19:03.922 chunk 16 62420 bytes
19:19:03.924 in Ack 16
19:19:04.173 chunk 16 62465 bytes
19:19:04.174 in Ack 16
19:19:04.423 chunk 16 62504 bytes
19:19:04.424 in Ack 16
19:19:04.495 fetch POST /api/observability/v1/traces
19:19:04.495 fetched POST /api/observability/v1/traces 204
19:19:04.673 chunk 16 62534 bytes
19:19:04.674 in Ack 16
19:19:04.923 chunk 16 62564 bytes
19:19:04.924 in Ack 16
19:19:05.174 chunk 16 62603 bytes
19:19:05.175 in Ack 16
19:19:05.424 chunk 16 62636 bytes
19:19:05.425 in Ack 16
19:19:05.673 chunk 16 62675 bytes
19:19:05.674 in Ack 16
19:19:05.752 fetch POST /api/observability/v1/traces
19:19:05.752 fetched POST /api/observability/v1/traces 204
19:19:05.924 chunk 16 62720 bytes
19:19:05.925 in Ack 16
19:19:06.174 chunk 16 62750 bytes
19:19:06.176 in Ack 16
19:19:06.424 chunk 16 62801 bytes
19:19:06.425 in Ack 16
19:19:06.675 chunk 16 62840 bytes
19:19:06.676 in Ack 16
19:19:06.925 chunk 16 62888 bytes
19:19:06.926 in Ack 16
19:19:06.938 fetch POST /api/observability/v1/traces
19:19:06.938 fetched POST /api/observability/v1/traces 204
19:19:07.107 rpc 39 orchestration.dispatchCommand thread.visit
19:19:07.107 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:19:07.140 in Ping 
19:19:07.175 chunk 16 62918 bytes
19:19:07.176 in Ack 16
19:19:07.425 chunk 16 63001 bytes
19:19:07.427 in Ack 16
19:19:07.674 chunk 11 2833 bytes
19:19:07.675 in Ack 11
19:19:07.676 rpc 40 orchestration.dispatchCommand thread.visit
19:19:07.676 exit 40 {"_tag":"Success","value":{"sequence":0}}
19:19:07.681 chunk 16 63133 bytes
19:19:07.696 in Ack 16
19:19:07.944 chunk 16 63157 bytes
19:19:07.946 in Ack 16
19:19:08.107 fetch POST /api/observability/v1/traces
19:19:08.107 fetched POST /api/observability/v1/traces 204
19:19:08.260 chunk 16 65699 bytes
19:19:08.368 in Ack 16
