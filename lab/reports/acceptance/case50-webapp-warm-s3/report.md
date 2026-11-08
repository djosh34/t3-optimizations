# case50, webapp, warm start, seed 3

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 1.6s |
| Whole reply shown | 45.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 4.2s / 0.9s / 2.2s |
| Longest silence | 3.1s |
| Approval shown after it was asked | 2.1s |
| Bytes on the wire down / up | 357145 / 120890 |
| Bytes the app received / sent | 296674 / 61172 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.5 | 315 | 373 |
| 20 | 0.9 | 738 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.6 | 1398 | 1452 |
| 45 | 1.1 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","frame":"1030/1030","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","messages":"456/456 546/546 534/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":282,"lastDataAt":1794165572549,"silences":0,"longestSilenceMs":1173,"longestRecoveredGapMs":1173},"stats":{"sendRate":2250,"deliveredRate":2008.993859691534,"loss":0,"srttMs":47.68171604977375,"minRttMs":1,"sentPackets":93,"sentWireBytes":29772,"receivedPackets":110,"receivedWireBytes":53570,"lastHeardMs":9241,"unsyncedBytes":509,"inflightPackets":6,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"439/951","frame":"1030/1030","runs":"1014/1014 971/971 759/839","assistant":"32/32 21/21 445/455","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 495/656","providerThreads":"590/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 435/1019 714/785","messages":"456/456 546/546 901/901 546/546 134/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"465/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":282,"lastDataAt":1794165582671,"silences":0,"longestSilenceMs":1173,"longestRecoveredGapMs":1173},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.32555454545454543,"srttMs":400.3032272779077,"minRttMs":1,"sentPackets":151,"sentWireBytes":41130,"receivedPackets":185,"receivedWireBytes":83363,"lastHeardMs":19363,"unsyncedBytes":0,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 958/958","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","messages":"456/456 546/546 901/901 546/546 451/451 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":282,"lastDataAt":1794165592850,"silences":0,"longestSilenceMs":1173,"longestRecoveredGapMs":1173},"stats":{"sendRate":843.75,"deliveredRate":447.79874213836473,"loss":0.7,"srttMs":599.7573135862971,"minRttMs":1,"sentPackets":201,"sentWireBytes":50837,"receivedPackets":322,"receivedWireBytes":139715,"lastHeardMs":29542,"unsyncedBytes":0,"inflightPackets":25,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1546/1546 926/926","thread":"951/951","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 43/48","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1368/1371 48/961 836/1019","messages":"456/456 546/546 901/901 546/546 451/451 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"701/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":282,"lastDataAt":1794165603562,"silences":0,"longestSilenceMs":1173,"longestRecoveredGapMs":1173},"stats":{"sendRate":2238.9937106918233,"deliveredRate":0,"loss":0.5441,"srttMs":639.5329432145087,"minRttMs":757,"sentPackets":256,"sentWireBytes":61106,"receivedPackets":633,"receivedWireBytes":231263,"lastHeardMs":40254,"unsyncedBytes":566,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","thread":"951/951","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 599/599","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","messages":"456/456 546/546 901/901 546/546 451/451 546/546 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":282,"lastDataAt":1794165614568,"silences":0,"longestSilenceMs":1173,"longestRecoveredGapMs":1173},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.4893825108225107,"srttMs":769.9953865887941,"minRttMs":620,"sentPackets":337,"sentWireBytes":81156,"receivedPackets":828,"receivedWireBytes":328252,"lastHeardMs":51260,"unsyncedBytes":500,"inflightPackets":56,"duplicateBytes":0}}
- shim trace
19:19:23.596 fetch GET /api/auth/session
19:19:23.598 fetched GET /api/auth/session 200
19:19:23.745 fetch GET /.well-known/t3/environment
19:19:23.745 fetched GET /.well-known/t3/environment 200
19:19:23.758 fetch GET /.well-known/t3/environment
19:19:23.759 fetched GET /.well-known/t3/environment 200
19:19:23.773 rpc 0 subscribeServerConfig
19:19:23.774 chunk 0 38722 bytes
19:19:23.794 in Ack 0
19:19:23.796 rpc 1 subscribeServerLifecycle
19:19:23.796 rpc 2 subscribeServerLifecycle
19:19:23.798 fetch GET /api/auth/session
19:19:23.801 fetched GET /api/auth/session 200
19:19:23.801 rpc 3 subscribeDeviceState
19:19:23.803 rpc 4 preview.reportProfiles
19:19:23.834 rpc 5 subscribeProjectClones
19:19:23.851 fetch GET /api/orchestration/threads/mcp%3A1b8d5ca2-eea0-4226-a2e6-adc97d4490eb/bounded
19:19:23.855 fetch GET /api/orchestration/shell
19:19:23.873 rpc 6 server.refreshProviders
19:19:23.910 rpc 7 orchestration.dispatchCommand thread.visit
19:19:23.910 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:19:23.920 fetched GET /api/orchestration/shell 200
19:19:23.920 rpc 8 subscribeTerminalMetadata
19:19:23.921 rpc 9 subscribeVcsStatus
19:19:23.921 rpc 10 subscribePreviewEvents
19:19:23.923 rpc 11 orchestration.subscribeShell
19:19:23.923 chunk 11 2790 bytes
19:19:23.924 rpc 12 assets.createUrl
19:19:23.925 rpc 13 preview.list
19:19:23.925 rpc 14 projects.readFile
19:19:23.929 in Ack 11
19:19:24.040 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3266f4ae-7c8e-4d62-bfd4-ca356d9070d5","sessions":[]}}
19:19:24.040 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:19:24.149 chunk 1 3872 bytes
19:19:24.149 in Ack 1
19:19:24.173 rpc 15 server.reportClientActivity
19:19:24.173 chunk 11 2766 bytes
19:19:24.174 in Ack 11
19:19:24.510 chunk 2 3872 bytes
19:19:24.510 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:19:11.988Z","configurableProviders":false,"con
19:19:24.510 chunk 9 894 bytes
19:19:24.510 in Ack 2
19:19:24.513 in Ack 9
19:19:24.648 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:19:24.755 fetch POST /api/observability/v1/traces
19:19:24.755 fetched POST /api/observability/v1/traces 204
19:19:24.868 chunk 3 876 bytes
19:19:24.868 exit 4 {"_tag":"Success","value":null}
19:19:24.868 chunk 5 4 bytes
19:19:24.868 chunk 8 36 bytes
19:19:24.868 exit 15 {"_tag":"Success","value":null}
19:19:24.869 in Ack 3
19:19:24.872 in Ack 5
19:19:24.872 in Ack 8
19:19:24.898 chunk 11 2809 bytes
19:19:24.899 in Ack 11
19:19:24.931 fetched GET /api/orchestration/threads/mcp%3A1b8d5ca2-eea0-4226-a2e6-adc97d4490eb/bounded 200
19:19:24.934 rpc 16 orchestration.subscribeThread
19:19:24.934 chunk 16 8283 bytes
19:19:24.943 in Interrupt 9
19:19:24.945 rpc 17 subscribeVcsStatus
19:19:24.946 in Ack 16
19:19:25.077 chunk 17 894 bytes
19:19:25.077 in Ack 17
19:19:25.185 chunk 16 12694 bytes
19:19:25.186 in Ack 16
19:19:25.236 rpc 18 server.reportClientActivity
19:19:25.469 exit 18 {"_tag":"Success","value":null}
19:19:25.471 chunk 16 13502 bytes
19:19:25.474 in Ack 16
19:19:25.749 rpc 19 attachments.createUploadUrl
19:19:25.905 chunk 16 18027 bytes
19:19:25.911 fetch POST /api/observability/v1/traces
19:19:25.911 fetched POST /api/observability/v1/traces 204
19:19:25.914 in Ack 16
19:19:26.018 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-b3348e31-b950-4903-8ee9-41c0626dbbdb","expiresAt":1794166165969,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:19:26.019 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWIzMzQ4ZTMxLWI5NTAtNDkwMy04ZWU5LTQxYzA2MjZkYmJkYiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjE2NTk2OX0.RISVjO9kxunMaqloDa-Ej2ubKDPbTLiqB1CqU2iQe8g
19:19:26.156 chunk 16 21199 bytes
19:19:26.157 in Ack 16
19:19:26.163 in Interrupt 17
19:19:26.165 rpc 20 subscribeVcsStatus
19:19:26.233 chunk 20 894 bytes
19:19:26.233 in Ack 20
19:19:26.416 rpc 21 server.reportClientActivity
19:19:26.917 fetch POST /api/observability/v1/traces
19:19:26.917 fetched POST /api/observability/v1/traces 204
19:19:27.117 exit 21 {"_tag":"Success","value":null}
19:19:28.121 fetch POST /api/observability/v1/traces
19:19:28.121 fetched POST /api/observability/v1/traces 204
19:19:28.771 in Ping 
19:19:28.920 rpc 22 orchestration.dispatchCommand message.dispatch
19:19:29.517 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:19:29.519 rpc 23 attachments.delete
19:19:29.859 chunk 16 23633 bytes
19:19:29.859 in Ack 16
19:19:29.922 fetch POST /api/observability/v1/traces
19:19:29.922 fetched POST /api/observability/v1/traces 204
19:19:30.109 chunk 16 23653 bytes
19:19:30.110 in Ack 16
19:19:30.359 chunk 16 23665 bytes
19:19:30.360 in Ack 16
19:19:30.609 chunk 16 23667 bytes
19:19:30.610 in Ack 16
19:19:30.768 exit 23 {"_tag":"Success","value":null}
19:19:30.859 chunk 11 2831 bytes
19:19:30.859 chunk 16 28093 bytes
19:19:30.860 in Ack 11
19:19:30.861 rpc 24 orchestration.dispatchCommand thread.visit
19:19:30.861 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:19:30.902 in Ack 16
19:19:30.905 rpc 25 assets.createUrl
19:19:31.109 chunk 11 2831 bytes
19:19:31.110 in Ack 11
19:19:31.114 fetch POST /api/observability/v1/traces
19:19:31.114 fetched POST /api/observability/v1/traces 204
19:19:31.359 chunk 16 32106 bytes
19:19:31.360 in Ack 16
19:19:31.367 in Interrupt 20
19:19:31.369 rpc 26 subscribeVcsStatus
19:19:31.449 chunk 26 894 bytes
19:19:31.449 in Ack 26
19:19:31.758 chunk 16 31940 bytes
19:19:31.759 rpc 27 server.reportClientActivity
19:19:31.761 in Ack 16
19:19:32.011 chunk 16 31940 bytes
19:19:32.012 in Ack 16
19:19:32.361 fetch POST /api/observability/v1/traces
19:19:32.361 fetched POST /api/observability/v1/traces 204
19:19:32.519 rpc 28 orchestration.dispatchCommand message.dispatch
19:19:32.550 chunk 16 32715 bytes
19:19:32.558 in Ack 16
19:19:32.800 chunk 16 33283 bytes
19:19:32.801 in Ack 16
19:19:33.300 chunk 16 34194 bytes
19:19:33.302 in Ack 16
19:19:33.347 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:19:33.519 fetch POST /api/observability/v1/traces
19:19:33.519 fetched POST /api/observability/v1/traces 204
19:19:33.772 in Ping 
19:19:33.802 chunk 16 36628 bytes
19:19:33.803 in Ack 16
19:19:34.052 chunk 16 36668 bytes
19:19:34.054 in Ack 16
19:19:34.302 chunk 16 36688 bytes
19:19:34.303 in Ack 16
19:19:34.552 chunk 16 36698 bytes
19:19:34.553 in Ack 16
19:19:34.802 chunk 16 36740 bytes
19:19:34.804 in Ack 16
19:19:34.841 fetch POST /api/observability/v1/traces
19:19:34.841 fetched POST /api/observability/v1/traces 204
19:19:35.146 chunk 16 36752 bytes
19:19:35.148 in Ack 16
19:19:35.397 chunk 16 36770 bytes
19:19:35.398 in Ack 16
19:19:35.647 chunk 16 36810 bytes
19:19:35.648 in Ack 16
19:19:36.144 chunk 16 36822 bytes
19:19:36.146 in Ack 16
19:19:36.164 fetch POST /api/observability/v1/traces
19:19:36.164 fetched POST /api/observability/v1/traces 204
19:19:36.396 chunk 16 36834 bytes
19:19:36.397 in Ack 16
19:19:36.646 chunk 16 36846 bytes
19:19:36.647 in Ack 16
19:19:36.897 chunk 16 36906 bytes
19:19:36.898 in Ack 16
19:19:37.147 chunk 16 36896 bytes
19:19:37.148 in Ack 16
19:19:37.397 chunk 16 36942 bytes
19:19:37.398 in Ack 16
19:19:37.411 fetch POST /api/observability/v1/traces
19:19:37.411 fetched POST /api/observability/v1/traces 204
19:19:37.646 chunk 16 36952 bytes
19:19:37.647 in Ack 16
19:19:37.897 chunk 16 36984 bytes
19:19:37.898 in Ack 16
19:19:38.146 chunk 16 37004 bytes
19:19:38.147 in Ack 16
19:19:38.482 chunk 16 37036 bytes
19:19:38.483 in Ack 16
19:19:38.648 fetch POST /api/observability/v1/traces
19:19:38.648 fetched POST /api/observability/v1/traces 204
19:19:38.764 chunk 16 37066 bytes
19:19:38.766 in Ack 16
19:19:38.774 in Ping 
19:19:39.017 chunk 16 37096 bytes
19:19:39.018 in Ack 16
19:19:39.268 chunk 16 37733 bytes
19:19:39.268 in Ack 16
19:19:39.517 chunk 11 2895 bytes
19:19:39.517 chunk 16 37775 bytes
19:19:39.518 in Ack 11
19:19:39.556 in Ack 16
19:19:39.766 fetch POST /api/observability/v1/traces
19:19:39.766 fetched POST /api/observability/v1/traces 204
19:19:39.768 chunk 16 37795 bytes
19:19:39.769 in Ack 16
19:19:40.020 chunk 16 37817 bytes
19:19:40.021 in Ack 16
19:19:40.271 chunk 16 37835 bytes
19:19:40.272 in Ack 16
19:19:40.522 chunk 16 39892 bytes
19:19:40.523 in Ack 16
19:19:40.769 fetch POST /api/observability/v1/traces
19:19:40.769 fetched POST /api/observability/v1/traces 204
19:19:40.773 chunk 16 39912 bytes
19:19:40.774 in Ack 16
19:19:40.862 rpc 29 orchestration.dispatchCommand thread.visit
19:19:40.862 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:19:41.023 chunk 16 39950 bytes
19:19:41.024 in Ack 16
19:19:41.273 chunk 16 39982 bytes
19:19:41.274 in Ack 16
19:19:41.523 chunk 16 39994 bytes
19:19:41.524 in Ack 16
19:19:41.775 chunk 16 40014 bytes
19:19:41.776 fetch POST /api/observability/v1/traces
19:19:41.776 fetched POST /api/observability/v1/traces 204
19:19:41.777 in Ack 16
19:19:42.026 chunk 16 40024 bytes
19:19:42.027 in Ack 16
19:19:42.278 chunk 16 40052 bytes
19:19:42.279 in Ack 16
19:19:42.528 chunk 16 40092 bytes
19:19:42.529 in Ack 16
19:19:42.778 fetch POST /api/observability/v1/traces
19:19:42.778 fetched POST /api/observability/v1/traces 204
19:19:42.779 chunk 16 40104 bytes
19:19:42.781 in Ack 16
19:19:43.029 chunk 16 40116 bytes
19:19:43.030 in Ack 16
19:19:43.280 chunk 16 40146 bytes
19:19:43.281 in Ack 16
19:19:43.530 chunk 16 40166 bytes
19:19:43.531 in Ack 16
19:19:43.774 in Ping 
19:19:43.780 chunk 16 40224 bytes
19:19:43.781 in Ack 16
19:19:43.792 fetch POST /api/observability/v1/traces
19:19:43.792 fetched POST /api/observability/v1/traces 204
19:19:44.280 chunk 16 40262 bytes
19:19:44.281 in Ack 16
19:19:44.531 chunk 16 40292 bytes
19:19:44.532 in Ack 16
19:19:44.781 chunk 16 40320 bytes
19:19:44.782 in Ack 16
19:19:45.032 chunk 16 40350 bytes
19:19:45.033 in Ack 16
19:19:45.282 chunk 11 2895 bytes
19:19:45.282 chunk 16 41823 bytes
19:19:45.282 fetch POST /api/observability/v1/traces
19:19:45.283 fetched POST /api/observability/v1/traces 204
19:19:45.283 in Ack 11
19:19:45.288 in Ack 16
19:19:45.425 exit 27 {"_tag":"Success","value":null}
19:19:45.532 chunk 16 41833 bytes
19:19:45.533 in Ack 16
19:19:45.783 chunk 16 41863 bytes
19:19:45.784 in Ack 16
19:19:46.033 chunk 16 43884 bytes
19:19:46.034 in Ack 16
19:19:46.285 chunk 16 43944 bytes
19:19:46.285 fetch POST /api/observability/v1/traces
19:19:46.285 fetched POST /api/observability/v1/traces 204
19:19:46.286 in Ack 16
19:19:46.534 chunk 16 44994 bytes
19:19:46.536 in Ack 16
19:19:46.784 chunk 16 45039 bytes
19:19:46.785 in Ack 16
19:19:47.035 chunk 16 45069 bytes
19:19:47.036 in Ack 16
19:19:47.290 chunk 16 45084 bytes
19:19:47.290 fetch POST /api/observability/v1/traces
19:19:47.290 fetched POST /api/observability/v1/traces 204
19:19:47.292 in Ack 16
19:19:47.540 chunk 16 45677 bytes
19:19:47.541 in Ack 16
19:19:48.040 chunk 16 45722 bytes
19:19:48.041 in Ack 16
19:19:48.290 chunk 16 45752 bytes
19:19:48.291 in Ack 16
19:19:48.301 fetch POST /api/observability/v1/traces
19:19:48.301 fetched POST /api/observability/v1/traces 204
19:19:48.540 chunk 16 45812 bytes
19:19:48.541 in Ack 16
19:19:48.774 in Ping 
19:19:48.790 chunk 16 45857 bytes
19:19:48.791 in Ack 16
19:19:48.997 rpc 30 server.reportClientActivity
19:19:49.040 chunk 16 45887 bytes
19:19:49.041 in Ack 16
19:19:49.291 chunk 16 45917 bytes
19:19:49.291 in Ack 16
19:19:49.540 chunk 16 45932 bytes
19:19:49.541 in Ack 16
19:19:49.550 fetch POST /api/observability/v1/traces
19:19:49.550 fetched POST /api/observability/v1/traces 204
19:19:49.791 chunk 16 45992 bytes
19:19:49.792 in Ack 16
19:19:50.041 chunk 16 46037 bytes
19:19:50.042 in Ack 16
19:19:50.292 chunk 16 46082 bytes
19:19:50.293 in Ack 16
19:19:50.543 chunk 16 46127 bytes
19:19:50.544 in Ack 16
19:19:50.793 fetch POST /api/observability/v1/traces
19:19:50.793 fetched POST /api/observability/v1/traces 204
19:19:50.794 chunk 16 46142 bytes
19:19:50.795 in Ack 16
19:19:50.864 rpc 31 orchestration.dispatchCommand thread.visit
19:19:50.864 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:19:51.044 chunk 16 46187 bytes
19:19:51.045 in Ack 16
19:19:51.229 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/39ff6f1d-0f62-4c8c-9442-28e72159c11e","expiresAt":1794169191229}}
19:19:51.294 chunk 16 46232 bytes
19:19:51.295 in Ack 16
19:19:51.544 chunk 16 46247 bytes
19:19:51.545 in Ack 16
19:19:51.795 chunk 16 46307 bytes
19:19:51.795 fetch POST /api/observability/v1/traces
19:19:51.795 fetched POST /api/observability/v1/traces 204
19:19:51.797 in Ack 16
19:19:52.045 chunk 16 46337 bytes
19:19:52.046 in Ack 16
19:19:52.299 chunk 16 46382 bytes
19:19:52.300 in Ack 16
19:19:52.550 chunk 11 2895 bytes
19:19:52.550 chunk 16 46412 bytes
19:19:52.550 in Ack 11
19:19:52.555 in Ack 16
19:19:52.798 fetch POST /api/observability/v1/traces
19:19:52.798 fetched POST /api/observability/v1/traces 204
19:19:52.801 chunk 16 46442 bytes
19:19:52.802 in Ack 16
19:19:53.051 chunk 16 46487 bytes
19:19:53.052 in Ack 16
19:19:53.301 chunk 16 46517 bytes
19:19:53.302 in Ack 16
19:19:53.551 chunk 16 46547 bytes
19:19:53.552 in Ack 16
19:19:53.651 exit 30 {"_tag":"Success","value":null}
19:19:53.774 in Ping 
19:19:53.802 chunk 16 46592 bytes
19:19:53.803 in Ack 16
19:19:53.811 fetch POST /api/observability/v1/traces
19:19:53.811 fetched POST /api/observability/v1/traces 204
19:19:54.122 chunk 16 46637 bytes
19:19:54.124 in Ack 16
19:19:54.373 chunk 16 46682 bytes
19:19:54.374 in Ack 16
19:19:54.626 chunk 16 46697 bytes
19:19:54.627 in Ack 16
19:19:54.876 chunk 16 46742 bytes
19:19:54.877 in Ack 16
19:19:55.125 fetch POST /api/observability/v1/traces
19:19:55.125 fetched POST /api/observability/v1/traces 204
19:19:55.126 chunk 16 46787 bytes
19:19:55.126 in Ack 16
19:19:55.376 chunk 16 46832 bytes
19:19:55.377 in Ack 16
19:19:55.627 chunk 16 46862 bytes
19:19:55.628 in Ack 16
19:19:55.877 chunk 16 46877 bytes
19:19:55.878 in Ack 16
19:19:56.128 fetch POST /api/observability/v1/traces
19:19:56.128 fetched POST /api/observability/v1/traces 204
19:19:56.129 chunk 16 46922 bytes
19:19:56.130 in Ack 16
19:19:56.379 chunk 16 46967 bytes
19:19:56.380 in Ack 16
19:19:56.629 chunk 16 46997 bytes
19:19:56.630 in Ack 16
19:19:56.879 chunk 16 47027 bytes
19:19:56.880 in Ack 16
19:19:57.129 chunk 16 47057 bytes
19:19:57.130 in Ack 16
19:19:57.150 fetch POST /api/observability/v1/traces
19:19:57.150 fetched POST /api/observability/v1/traces 204
19:19:57.379 chunk 16 47117 bytes
19:19:57.380 in Ack 16
19:19:57.630 chunk 16 47147 bytes
19:19:57.631 in Ack 16
19:19:57.879 chunk 16 47177 bytes
19:19:57.880 in Ack 16
19:19:58.129 chunk 16 47207 bytes
19:19:58.130 in Ack 16
19:19:58.380 chunk 16 47252 bytes
19:19:58.381 in Ack 16
19:19:58.389 fetch POST /api/observability/v1/traces
19:19:58.389 fetched POST /api/observability/v1/traces 204
19:19:58.630 chunk 16 47297 bytes
19:19:58.631 in Ack 16
19:19:58.775 in Ping 
19:19:58.881 chunk 16 47357 bytes
19:19:58.882 in Ack 16
19:19:59.130 chunk 16 47387 bytes
19:19:59.131 in Ack 16
19:19:59.381 chunk 16 47417 bytes
19:19:59.381 in Ack 16
19:19:59.631 chunk 16 47477 bytes
19:19:59.632 in Ack 16
19:19:59.640 fetch POST /api/observability/v1/traces
19:19:59.640 fetched POST /api/observability/v1/traces 204
19:19:59.882 chunk 16 47507 bytes
19:19:59.883 in Ack 16
19:20:00.131 chunk 16 47537 bytes
19:20:00.133 in Ack 16
19:20:00.381 chunk 16 47582 bytes
19:20:00.382 in Ack 16
19:20:00.632 chunk 16 47609 bytes
19:20:00.633 in Ack 16
19:20:00.885 fetch POST /api/observability/v1/traces
19:20:00.885 fetched POST /api/observability/v1/traces 204
19:20:01.429 chunk 16 49684 bytes
19:20:01.431 in Ack 16
19:20:01.679 chunk 16 49734 bytes
19:20:01.680 in Ack 16
19:20:01.929 chunk 16 54236 bytes
19:20:01.931 in Ack 16
19:20:02.191 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:20:02.432 fetch POST /api/observability/v1/traces
19:20:02.432 fetched POST /api/observability/v1/traces 204
19:20:02.866 chunk 11 3203 bytes
19:20:02.866 in Ack 11
19:20:03.054 exit 32 {"_tag":"Success","value":{"sequence":97}}
19:20:03.055 rpc 33 orchestration.dispatchCommand thread.visit
19:20:03.055 exit 33 {"_tag":"Success","value":{"sequence":0}}
19:20:03.117 chunk 16 56707 bytes
19:20:03.118 in Ack 16
19:20:03.367 chunk 16 56727 bytes
19:20:03.368 in Ack 16
19:20:03.452 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:20:03.618 chunk 16 56749 bytes
19:20:03.619 in Ack 16
19:20:03.775 in Ping 
19:20:03.868 fetch POST /api/observability/v1/traces
19:20:03.868 fetched POST /api/observability/v1/traces 204
19:20:03.869 chunk 16 56779 bytes
19:20:03.870 in Ack 16
19:20:04.120 chunk 16 56789 bytes
19:20:04.121 in Ack 16
19:20:04.371 chunk 16 56821 bytes
19:20:04.372 in Ack 16
19:20:04.789 chunk 16 56843 bytes
19:20:04.790 in Ack 16
19:20:04.871 fetch POST /api/observability/v1/traces
19:20:04.871 fetched POST /api/observability/v1/traces 204
19:20:05.039 chunk 16 56896 bytes
19:20:05.040 in Ack 16
19:20:05.082 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:20:05.288 chunk 11 2897 bytes
19:20:05.288 chunk 16 56918 bytes
19:20:05.289 in Ack 11
19:20:05.294 in Ack 16
19:20:05.539 chunk 16 57974 bytes
19:20:05.541 in Ack 16
19:20:05.549 in Interrupt 26
19:20:05.573 rpc 35 subscribeVcsStatus
19:20:05.790 chunk 16 58054 bytes
19:20:05.791 in Ack 16
19:20:05.824 rpc 36 server.reportClientActivity
19:20:06.039 chunk 16 59046 bytes
19:20:06.040 in Ack 16
19:20:06.049 fetch POST /api/observability/v1/traces
19:20:06.049 fetched POST /api/observability/v1/traces 204
19:20:06.289 chunk 16 59066 bytes
19:20:06.290 in Ack 16
19:20:06.517 chunk 35 894 bytes
19:20:06.517 in Ack 35
19:20:06.539 chunk 16 59086 bytes
19:20:06.540 in Ack 16
19:20:06.790 chunk 16 59116 bytes
19:20:06.791 in Ack 16
19:20:07.040 chunk 16 59156 bytes
19:20:07.041 in Ack 16
19:20:07.291 chunk 16 59224 bytes
19:20:07.292 fetch POST /api/observability/v1/traces
19:20:07.292 fetched POST /api/observability/v1/traces 204
19:20:07.293 in Ack 16
19:20:07.542 chunk 16 59244 bytes
19:20:07.543 in Ack 16
19:20:07.792 chunk 16 59274 bytes
19:20:07.793 in Ack 16
19:20:08.043 chunk 16 59294 bytes
19:20:08.044 in Ack 16
19:20:08.293 chunk 16 60711 bytes
19:20:08.294 in Ack 16
19:20:08.356 fetch POST /api/observability/v1/traces
19:20:08.357 fetched POST /api/observability/v1/traces 204
19:20:08.486 exit 36 {"_tag":"Success","value":null}
19:20:08.543 chunk 16 60751 bytes
19:20:08.544 in Ack 16
19:20:08.775 in Ping 
19:20:08.794 chunk 16 60761 bytes
19:20:08.795 in Ack 16
19:20:09.044 chunk 16 60801 bytes
19:20:09.045 in Ack 16
19:20:09.295 chunk 16 60811 bytes
19:20:09.296 in Ack 16
19:20:09.487 fetch POST /api/observability/v1/traces
19:20:09.487 fetched POST /api/observability/v1/traces 204
19:20:09.544 chunk 16 60841 bytes
19:20:09.545 in Ack 16
19:20:09.795 chunk 16 60861 bytes
19:20:09.796 in Ack 16
19:20:10.044 chunk 16 60881 bytes
19:20:10.045 in Ack 16
19:20:10.294 chunk 16 60911 bytes
19:20:10.295 in Ack 16
19:20:10.545 chunk 16 60931 bytes
19:20:10.546 in Ack 16
19:20:10.555 fetch POST /api/observability/v1/traces
19:20:10.555 fetched POST /api/observability/v1/traces 204
19:20:10.795 chunk 16 60961 bytes
19:20:10.796 in Ack 16
19:20:11.046 chunk 16 60973 bytes
19:20:11.047 in Ack 16
19:20:11.295 chunk 11 2897 bytes
19:20:11.296 in Ack 11
19:20:11.297 chunk 16 60989 bytes
19:20:11.301 in Ack 16
19:20:11.550 chunk 16 61009 bytes
19:20:11.551 in Ack 16
19:20:11.797 fetch POST /api/observability/v1/traces
19:20:11.797 fetched POST /api/observability/v1/traces 204
19:20:11.806 chunk 16 61029 bytes
19:20:11.807 in Ack 16
19:20:12.057 chunk 16 61049 bytes
19:20:12.058 in Ack 16
19:20:12.307 chunk 16 61089 bytes
19:20:12.308 in Ack 16
19:20:12.557 chunk 16 62138 bytes
19:20:12.559 in Ack 16
19:20:12.811 chunk 16 62198 bytes
19:20:12.811 fetch POST /api/observability/v1/traces
19:20:12.811 fetched POST /api/observability/v1/traces 204
19:20:12.812 in Ack 16
19:20:12.873 rpc 37 orchestration.dispatchCommand thread.visit
19:20:12.873 exit 37 {"_tag":"Success","value":{"sequence":0}}
19:20:13.061 chunk 16 62249 bytes
19:20:13.062 in Ack 16
19:20:13.312 chunk 16 62294 bytes
19:20:13.313 in Ack 16
19:20:13.563 chunk 16 62375 bytes
19:20:13.564 in Ack 16
19:20:13.781 in Ping 
19:20:13.814 fetch POST /api/observability/v1/traces
19:20:13.814 fetched POST /api/observability/v1/traces 204
19:20:13.814 chunk 16 62375 bytes
19:20:13.816 in Ack 16
19:20:13.997 rpc 38 server.reportClientActivity
19:20:14.069 chunk 11 2897 bytes
19:20:14.069 chunk 16 62450 bytes
19:20:14.070 in Ack 11
19:20:14.084 in Ack 16
19:20:14.319 chunk 16 62489 bytes
19:20:14.321 in Ack 16
19:20:14.570 chunk 16 62534 bytes
19:20:14.571 in Ack 16
19:20:14.817 fetch POST /api/observability/v1/traces
19:20:14.817 fetched POST /api/observability/v1/traces 204
19:20:14.821 chunk 16 62564 bytes
19:20:14.822 in Ack 16
19:20:15.070 chunk 16 62621 bytes
19:20:15.071 in Ack 16
19:20:15.320 chunk 16 62675 bytes
19:20:15.322 in Ack 16
19:20:15.570 chunk 16 62705 bytes
19:20:15.572 in Ack 16
19:20:15.820 chunk 16 62735 bytes
19:20:15.821 in Ack 16
19:20:15.877 fetch POST /api/observability/v1/traces
19:20:15.878 fetched POST /api/observability/v1/traces 204
19:20:16.070 chunk 16 62786 bytes
19:20:16.072 in Ack 16
19:20:16.320 chunk 16 62801 bytes
19:20:16.321 in Ack 16
19:20:16.375 exit 38 {"_tag":"Success","value":null}
19:20:16.571 chunk 16 62870 bytes
19:20:16.572 in Ack 16
19:20:16.827 chunk 16 62888 bytes
19:20:16.829 in Ack 16
19:20:17.073 fetch POST /api/observability/v1/traces
19:20:17.073 fetched POST /api/observability/v1/traces 204
19:20:17.077 chunk 16 63001 bytes
19:20:17.078 in Ack 16
19:20:18.077 chunk 16 63025 bytes
19:20:18.079 in Ack 16
19:20:18.086 fetch POST /api/observability/v1/traces
19:20:18.086 fetched POST /api/observability/v1/traces 204
19:20:18.350 chunk 16 65449 bytes
19:20:18.352 in Ack 16
