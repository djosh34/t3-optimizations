# case70, webapp, warm start, seed 4

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 80 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.6s |
| Prompt reached the agent | 4.6s |
| Time to first text | 6.0s |
| Whole reply shown | 79.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 32.6s / 7.6s / 28.6s |
| Longest silence | 29.1s |
| Approval shown after it was asked | 13.0s |
| Bytes on the wire down / up | 91804 / 30610 |
| Bytes the app received / sent | 58270 / 76401 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.0 | 12 | 214 |
| 20 | 5.3 | 397 | 613 |
| 30 | 9.4 | 635 | 1017 |
| 40 | 9.7 | 1028 | 1077 |
| 50 | 4.0 | 1077 | 1233 |
| 60 | 13.6 | 1094 | 1595 |
| 70 | 23.6 | 1094 | 1596 |
| 79 | 32.6 | 1094 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172702540,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":4500,"deliveredRate":2146.5901848311028,"loss":0,"srttMs":5.025087652435106,"minRttMs":1,"sentPackets":99,"sentWireBytes":30609,"receivedPackets":111,"receivedWireBytes":66542,"lastHeardMs":8446,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 254/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 115/225","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172713217,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":2155.6291390728484,"deliveredRate":149.38952997034465,"loss":0.9240274599542333,"srttMs":335.95670773389566,"minRttMs":1393,"sentPackets":144,"sentWireBytes":43063,"receivedPackets":124,"receivedWireBytes":73877,"lastHeardMs":19123,"unsyncedBytes":0,"inflightPackets":23,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 570/633","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172723655,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.8896763648251061,"srttMs":707.5871192671586,"minRttMs":1393,"sentPackets":207,"sentWireBytes":56484,"receivedPackets":154,"receivedWireBytes":83335,"lastHeardMs":29561,"unsyncedBytes":324,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 973/1108","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661 150/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172733601,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":1500,"deliveredRate":449.8569656347207,"loss":0.6799999999999999,"srttMs":3502.9973237954637,"minRttMs":1393,"sentPackets":253,"sentWireBytes":68886,"receivedPackets":184,"receivedWireBytes":93014,"lastHeardMs":39507,"unsyncedBytes":0,"inflightPackets":88,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 385/1550","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1335/1340","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661 150/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"933/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172743617,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":1500,"deliveredRate":268.6507099991626,"loss":0.7301516883116882,"srttMs":3503.747658321031,"minRttMs":1393,"sentPackets":304,"sentWireBytes":76168,"receivedPackets":208,"receivedWireBytes":103441,"lastHeardMs":49523,"unsyncedBytes":0,"inflightPackets":87,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 1550/1550 640/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1340/1340 63/88","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661 231/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172753791,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":1125,"deliveredRate":465.45523843687636,"loss":0.7699999999999999,"srttMs":3420.517121003124,"minRttMs":1393,"sentPackets":342,"sentWireBytes":88988,"receivedPackets":236,"receivedWireBytes":114685,"lastHeardMs":59697,"unsyncedBytes":470,"inflightPackets":116,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 1550/1550 640/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1340/1340 103/153","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661 231/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172755541,"silences":0,"longestSilenceMs":1918,"longestRecoveredGapMs":1918},"stats":{"sendRate":1125,"deliveredRate":465.45523843687636,"loss":0.7699999999999999,"srttMs":3420.517121003124,"minRttMs":1393,"sentPackets":363,"sentWireBytes":98631,"receivedPackets":240,"receivedWireBytes":116745,"lastHeardMs":61447,"unsyncedBytes":470,"inflightPackets":126,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 1078/1078 849/849 1550/1550 640/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 53/839","assistant":"32/32 21/21 1340/1340 423/723","user":"10/10 9/9","messages":"456/456 548/548 901/901 548/548","attempts":"661/661 661/661 231/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":251,"lastDataAt":1794172775093,"silences":1,"longestSilenceMs":17998,"longestRecoveredGapMs":17998},"stats":{"sendRate":1125,"deliveredRate":465.45523843687636,"loss":0.7699999999999999,"srttMs":3787.0774808777333,"minRttMs":2798,"sentPackets":372,"sentWireBytes":103625,"receivedPackets":244,"receivedWireBytes":117885,"lastHeardMs":80999,"unsyncedBytes":489,"inflightPackets":117,"duplicateBytes":0}}
- shim trace
21:18:14.402 fetch GET /api/auth/session
21:18:14.403 fetched GET /api/auth/session 200
21:18:14.537 fetch GET /.well-known/t3/environment
21:18:14.538 fetched GET /.well-known/t3/environment 200
21:18:14.551 fetch GET /.well-known/t3/environment
21:18:14.552 fetched GET /.well-known/t3/environment 200
21:18:14.567 rpc 0 subscribeServerConfig
21:18:14.567 chunk 0 38678 bytes
21:18:14.596 in Ack 0
21:18:14.598 rpc 1 subscribeServerLifecycle
21:18:14.599 rpc 2 subscribeServerLifecycle
21:18:14.602 fetch GET /api/auth/session
21:18:14.604 fetched GET /api/auth/session 200
21:18:14.604 rpc 3 subscribeDeviceState
21:18:14.606 rpc 4 preview.reportProfiles
21:18:14.661 rpc 5 subscribeProjectClones
21:18:14.681 fetch GET /api/orchestration/threads/mcp%3A1bc5b331-8777-4267-be39-3df9e4736df3/bounded
21:18:14.685 fetch GET /api/orchestration/shell
21:18:14.686 fetched GET /api/orchestration/shell 200
21:18:14.688 rpc 6 orchestration.subscribeShell
21:18:14.688 chunk 6 2779 bytes
21:18:14.709 rpc 7 server.refreshProviders
21:18:14.748 rpc 8 orchestration.dispatchCommand thread.visit
21:18:14.748 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:18:14.763 rpc 9 subscribeTerminalMetadata
21:18:14.763 rpc 10 subscribeVcsStatus
21:18:14.764 rpc 11 subscribePreviewEvents
21:18:14.765 rpc 12 assets.createUrl
21:18:14.766 rpc 13 preview.list
21:18:14.767 rpc 14 projects.readFile
21:18:14.769 in Ack 6
21:18:14.900 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f48b5b9d-ff5b-4107-a5bf-f8417de05bcb","sessions":[]}}
21:18:14.900 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:18:14.938 chunk 6 2755 bytes
21:18:14.939 in Ack 6
21:18:15.016 rpc 15 server.reportClientActivity
21:18:15.269 chunk 1 3861 bytes
21:18:15.270 in Ack 1
21:18:15.399 chunk 2 3861 bytes
21:18:15.400 in Ack 2
21:18:15.498 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:18:02.936Z","configurableProviders":false,"con
21:18:15.498 chunk 10 472 bytes
21:18:15.498 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:18:15.501 in Ack 10
21:18:15.537 fetch POST /api/observability/v1/traces
21:18:15.537 fetched POST /api/observability/v1/traces 204
21:18:15.748 chunk 6 2798 bytes
21:18:15.749 in Ack 6
21:18:15.758 chunk 3 876 bytes
21:18:15.758 exit 4 {"_tag":"Success","value":null}
21:18:15.758 chunk 5 4 bytes
21:18:15.758 chunk 9 36 bytes
21:18:15.758 exit 15 {"_tag":"Success","value":null}
21:18:15.759 in Ack 3
21:18:15.762 rpc 16 preview.reportProfiles
21:18:15.762 in Ack 5
21:18:15.763 in Ack 9
21:18:15.804 exit 16 {"_tag":"Success","value":null}
21:18:16.004 fetched GET /api/orchestration/threads/mcp%3A1bc5b331-8777-4267-be39-3df9e4736df3/bounded 200
21:18:16.007 rpc 17 orchestration.subscribeThread
21:18:16.007 chunk 17 8294 bytes
21:18:16.017 in Interrupt 10
21:18:16.019 rpc 18 subscribeVcsStatus
21:18:16.020 in Ack 17
21:18:16.138 chunk 18 472 bytes
21:18:16.138 in Ack 18
21:18:16.257 chunk 17 11932 bytes
21:18:16.258 in Ack 17
21:18:16.289 rpc 19 server.reportClientActivity
21:18:16.507 chunk 17 13517 bytes
21:18:16.508 in Ack 17
21:18:16.750 fetch POST /api/observability/v1/traces
21:18:16.750 fetched POST /api/observability/v1/traces 204
21:18:16.759 chunk 17 19081 bytes
21:18:16.759 in Ack 17
21:18:16.887 rpc 20 attachments.createUploadUrl
21:18:17.040 exit 19 {"_tag":"Success","value":null}
21:18:17.042 chunk 17 21197 bytes
21:18:17.050 in Ack 17
21:18:17.058 in Interrupt 18
21:18:17.060 rpc 21 subscribeVcsStatus
21:18:17.088 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-8abdc56a-39e5-46d3-87e6-88d29d8670b1","expiresAt":1794173297058,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:18:17.089 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLThhYmRjNTZhLTM5ZTUtNDZkMy04N2U2LTg4ZDI5ZDg2NzBiMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzI5NzA1OH0.0_aAZjwnC55B8-eWxGPRt_8qG5MbZuoIsMJ11IGDNjQ
21:18:17.148 chunk 21 472 bytes
21:18:17.148 in Ack 21
21:18:17.311 rpc 22 server.reportClientActivity
21:18:17.761 fetch POST /api/observability/v1/traces
21:18:17.761 fetched POST /api/observability/v1/traces 204
21:18:18.219 exit 22 {"_tag":"Success","value":null}
21:18:19.221 fetch POST /api/observability/v1/traces
21:18:19.221 fetched POST /api/observability/v1/traces 204
21:18:19.565 in Ping 
21:18:19.705 rpc 23 orchestration.dispatchCommand message.dispatch
21:18:20.309 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:18:20.311 rpc 24 attachments.delete
21:18:20.507 exit 24 {"_tag":"Success","value":null}
21:18:20.559 chunk 6 2884 bytes
21:18:20.559 chunk 17 27073 bytes
21:18:20.559 in Ack 6
21:18:20.584 in Ack 17
21:18:20.586 rpc 25 assets.createUrl
21:18:20.704 fetch POST /api/observability/v1/traces
21:18:20.704 fetched POST /api/observability/v1/traces 204
21:18:20.810 chunk 17 28544 bytes
21:18:20.811 in Ack 17
21:18:21.062 chunk 17 29394 bytes
21:18:21.063 in Ack 17
21:18:21.311 chunk 6 2820 bytes
21:18:21.312 chunk 17 29550 bytes
21:18:21.312 in Ack 6
21:18:21.313 rpc 26 orchestration.dispatchCommand thread.visit
21:18:21.314 exit 26 {"_tag":"Success","value":{"sequence":0}}
21:18:21.318 in Ack 17
21:18:21.562 chunk 6 2820 bytes
21:18:21.562 chunk 17 31973 bytes
21:18:21.562 in Ack 6
21:18:21.567 in Ack 17
21:18:21.812 fetch POST /api/observability/v1/traces
21:18:21.812 fetched POST /api/observability/v1/traces 204
21:18:21.813 chunk 17 32727 bytes
21:18:21.814 in Ack 17
21:18:21.821 in Interrupt 21
21:18:21.823 rpc 27 subscribeVcsStatus
21:18:21.860 chunk 27 472 bytes
21:18:21.860 in Ack 27
21:18:22.063 chunk 17 35231 bytes
21:18:22.064 in Ack 17
21:18:22.073 rpc 28 server.reportClientActivity
21:18:22.118 exit 28 {"_tag":"Success","value":null}
21:18:22.832 fetch POST /api/observability/v1/traces
21:18:22.832 fetched POST /api/observability/v1/traces 204
21:18:23.277 rpc 29 orchestration.dispatchCommand message.dispatch
21:18:24.277 fetch POST /api/observability/v1/traces
21:18:24.277 fetched POST /api/observability/v1/traces 204
21:18:24.567 in Ping 
21:18:26.083 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/2150dc17-0540-4443-b35b-e29208bba768","expiresAt":1794176306083}}
21:18:26.201 fetch POST /api/observability/v1/traces
21:18:26.201 fetched POST /api/observability/v1/traces 204
21:18:27.520 fetch POST /api/observability/v1/traces
21:18:27.520 fetched POST /api/observability/v1/traces 204
21:18:28.643 chunk 17 37676 bytes
21:18:28.643 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:18:28.645 in Ack 17
21:18:29.076 chunk 17 37696 bytes
21:18:29.077 in Ack 17
21:18:29.568 in Ping 
21:18:29.653 fetch POST /api/observability/v1/traces
21:18:29.653 fetched POST /api/observability/v1/traces 204
21:18:29.925 chunk 17 37748 bytes
21:18:29.927 in Ack 17
21:18:30.412 chunk 17 37798 bytes
21:18:30.413 in Ack 17
21:18:30.929 fetch POST /api/observability/v1/traces
21:18:30.929 fetched POST /api/observability/v1/traces 204
21:18:31.273 chunk 17 37830 bytes
21:18:31.275 in Ack 17
21:18:31.708 chunk 17 37850 bytes
21:18:31.709 in Ack 17
21:18:32.275 fetch POST /api/observability/v1/traces
21:18:32.275 fetched POST /api/observability/v1/traces 204
21:18:33.218 chunk 17 37902 bytes
21:18:33.220 in Ack 17
21:18:33.659 chunk 17 38108 bytes
21:18:33.664 in Ack 17
21:18:34.221 fetch POST /api/observability/v1/traces
21:18:34.221 fetched POST /api/observability/v1/traces 204
21:18:34.570 in Ping 
21:18:34.830 chunk 17 38140 bytes
21:18:34.831 in Ack 17
21:18:35.176 chunk 17 38316 bytes
21:18:35.177 in Ack 17
21:18:35.685 chunk 17 38348 bytes
21:18:35.687 in Ack 17
21:18:35.832 fetch POST /api/observability/v1/traces
21:18:35.832 fetched POST /api/observability/v1/traces 204
21:18:36.266 chunk 17 38380 bytes
21:18:36.268 in Ack 17
21:18:36.649 chunk 17 38412 bytes
21:18:36.650 in Ack 17
21:18:37.045 chunk 17 38442 bytes
21:18:37.047 in Ack 17
21:18:37.269 fetch POST /api/observability/v1/traces
21:18:37.269 fetched POST /api/observability/v1/traces 204
21:18:37.493 chunk 17 38546 bytes
21:18:37.494 in Ack 17
21:18:37.745 chunk 17 38556 bytes
21:18:37.746 in Ack 17
21:18:37.997 chunk 17 38576 bytes
21:18:37.999 in Ack 17
21:18:38.443 chunk 17 38588 bytes
21:18:38.444 in Ack 17
21:18:38.495 fetch POST /api/observability/v1/traces
21:18:38.495 fetched POST /api/observability/v1/traces 204
21:18:38.951 chunk 17 38600 bytes
21:18:38.952 in Ack 17
21:18:39.201 chunk 17 38642 bytes
21:18:39.202 in Ack 17
21:18:39.570 in Ping 
21:18:39.690 chunk 17 38654 bytes
21:18:39.691 in Ack 17
21:18:39.790 rpc 30 server.reportClientActivity
21:18:39.954 fetch POST /api/observability/v1/traces
21:18:39.954 fetched POST /api/observability/v1/traces 204
21:18:40.522 chunk 17 38714 bytes
21:18:40.523 in Ack 17
21:18:41.449 chunk 17 38726 bytes
21:18:41.450 in Ack 17
21:18:41.524 fetch POST /api/observability/v1/traces
21:18:41.524 fetched POST /api/observability/v1/traces 204
21:18:41.965 chunk 17 38698 bytes
21:18:41.967 in Ack 17
21:18:42.218 chunk 17 38708 bytes
21:18:42.220 in Ack 17
21:18:42.469 chunk 17 38728 bytes
21:18:42.470 in Ack 17
21:18:42.799 chunk 17 38740 bytes
21:18:42.800 in Ack 17
21:18:42.967 fetch POST /api/observability/v1/traces
21:18:42.968 fetched POST /api/observability/v1/traces 204
21:18:43.139 chunk 17 38760 bytes
21:18:43.140 in Ack 17
21:18:43.398 chunk 17 38770 bytes
21:18:43.399 in Ack 17
21:18:44.000 chunk 17 38800 bytes
21:18:44.001 in Ack 17
21:18:44.141 fetch POST /api/observability/v1/traces
21:18:44.141 fetched POST /api/observability/v1/traces 204
21:18:44.258 chunk 17 38810 bytes
21:18:44.259 in Ack 17
21:18:44.570 in Ping 
21:18:44.775 chunk 17 38820 bytes
21:18:44.776 in Ack 17
21:18:45.260 fetch POST /api/observability/v1/traces
21:18:45.260 fetched POST /api/observability/v1/traces 204
21:18:45.538 chunk 17 38876 bytes
21:18:45.539 in Ack 17
21:18:45.794 chunk 17 38886 bytes
21:18:45.817 in Ack 17
21:18:46.048 chunk 17 38926 bytes
21:18:46.050 in Ack 17
21:18:46.303 chunk 17 38966 bytes
21:18:46.304 in Ack 17
21:18:46.540 fetch POST /api/observability/v1/traces
21:18:46.540 fetched POST /api/observability/v1/traces 204
21:18:47.073 chunk 17 38986 bytes
21:18:47.074 in Ack 17
21:18:47.325 chunk 17 38996 bytes
21:18:47.327 in Ack 17
21:18:47.661 chunk 17 39026 bytes
21:18:47.662 in Ack 17
21:18:47.917 chunk 17 39068 bytes
21:18:47.918 in Ack 17
21:18:48.075 fetch POST /api/observability/v1/traces
21:18:48.075 fetched POST /api/observability/v1/traces 204
21:18:48.168 chunk 17 39078 bytes
21:18:48.171 in Ack 17
21:18:48.434 chunk 17 39118 bytes
21:18:48.435 in Ack 17
21:18:48.684 chunk 17 39168 bytes
21:18:48.685 in Ack 17
21:18:48.934 chunk 17 39180 bytes
21:18:48.935 in Ack 17
21:18:49.172 fetch POST /api/observability/v1/traces
21:18:49.172 fetched POST /api/observability/v1/traces 204
21:18:49.444 chunk 17 39266 bytes
21:18:49.445 in Ack 17
21:18:49.570 in Ping 
21:18:49.702 chunk 17 39294 bytes
21:18:49.703 in Ack 17
21:18:49.961 chunk 17 39304 bytes
21:18:49.962 in Ack 17
21:18:50.211 chunk 17 39336 bytes
21:18:50.212 in Ack 17
21:18:50.446 fetch POST /api/observability/v1/traces
21:18:50.446 fetched POST /api/observability/v1/traces 204
21:18:50.631 chunk 17 39356 bytes
21:18:50.632 in Ack 17
21:18:51.059 chunk 17 39368 bytes
21:18:51.060 in Ack 17
21:18:51.491 chunk 17 39388 bytes
21:18:51.492 in Ack 17
21:18:51.638 fetch POST /api/observability/v1/traces
21:18:51.638 fetched POST /api/observability/v1/traces 204
21:18:52.424 chunk 17 39418 bytes
21:18:52.425 in Ack 17
21:18:52.852 chunk 17 39430 bytes
21:18:52.853 in Ack 17
21:18:53.179 chunk 17 39480 bytes
21:18:53.180 in Ack 17
21:18:53.427 fetch POST /api/observability/v1/traces
21:18:53.427 fetched POST /api/observability/v1/traces 204
21:18:53.602 chunk 17 39580 bytes
21:18:53.603 in Ack 17
21:18:53.854 chunk 17 39640 bytes
21:18:53.855 in Ack 17
21:18:54.129 chunk 17 39670 bytes
21:18:54.130 in Ack 17
21:18:54.379 chunk 17 39730 bytes
21:18:54.380 in Ack 17
21:18:54.571 in Ping 
21:18:54.605 fetch POST /api/observability/v1/traces
21:18:54.605 fetched POST /api/observability/v1/traces 204
21:18:54.629 chunk 17 39768 bytes
21:18:54.630 in Ack 17
21:18:54.793 in Interrupt 30
21:18:55.033 chunk 17 39848 bytes
21:18:55.036 in Ack 17
21:18:55.424 chunk 17 39858 bytes
21:18:55.426 in Ack 17
21:18:55.631 fetch POST /api/observability/v1/traces
21:18:55.631 fetched POST /api/observability/v1/traces 204
21:18:55.683 chunk 17 39950 bytes
21:18:55.684 in Ack 17
21:18:55.938 chunk 17 39980 bytes
21:18:55.940 in Ack 17
21:18:56.335 chunk 17 40010 bytes
21:18:56.336 in Ack 17
21:18:56.662 chunk 17 40100 bytes
21:18:56.663 in Ack 17
21:18:56.687 fetch POST /api/observability/v1/traces
21:18:56.687 fetched POST /api/observability/v1/traces 204
21:18:56.921 chunk 17 40110 bytes
21:18:56.922 in Ack 17
21:18:57.396 chunk 17 40120 bytes
21:18:57.397 in Ack 17
21:18:57.878 chunk 17 40170 bytes
21:18:57.879 in Ack 17
21:18:57.923 fetch POST /api/observability/v1/traces
21:18:57.923 fetched POST /api/observability/v1/traces 204
21:18:58.681 chunk 17 40238 bytes
21:18:58.682 in Ack 17
21:18:58.931 chunk 17 40246 bytes
21:18:58.932 in Ack 17
21:18:59.571 in Ping 
21:18:59.683 fetch POST /api/observability/v1/traces
21:18:59.683 fetched POST /api/observability/v1/traces 204
21:18:59.993 chunk 17 40254 bytes
21:18:59.994 in Ack 17
21:19:00.527 chunk 17 40298 bytes
21:19:00.528 in Ack 17
21:19:00.996 fetch POST /api/observability/v1/traces
21:19:00.996 fetched POST /api/observability/v1/traces 204
21:19:04.571 in Ping 
21:19:04.793 rpc 31 server.reportClientActivity
21:19:05.334 chunk 17 40306 bytes
21:19:05.335 in Ack 17
21:19:05.793 fetch POST /api/observability/v1/traces
21:19:05.793 fetched POST /api/observability/v1/traces 204
21:19:07.533 fetch POST /api/observability/v1/traces
21:19:07.533 fetched POST /api/observability/v1/traces 204
21:19:07.673 chunk 17 41314 bytes
21:19:07.674 in Ack 17
21:19:07.824 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:19:08.675 fetch POST /api/observability/v1/traces
21:19:08.675 fetched POST /api/observability/v1/traces 204
21:19:09.574 in Ping 
21:19:11.282 chunk 17 44819 bytes
21:19:11.283 in Ack 17
21:19:12.284 fetch POST /api/observability/v1/traces
21:19:12.284 fetched POST /api/observability/v1/traces 204
21:19:12.917 chunk 17 47340 bytes
21:19:12.917 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:19:12.918 in Ack 17
21:19:13.355 chunk 17 47362 bytes
21:19:13.357 in Ack 17
21:19:13.792 chunk 17 47384 bytes
21:19:13.793 in Ack 17
21:19:13.919 fetch POST /api/observability/v1/traces
21:19:13.919 fetched POST /api/observability/v1/traces 204
21:19:13.929 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:19:14.230 chunk 17 47404 bytes
21:19:14.232 in Ack 17
21:19:14.578 in Ping 
21:19:14.670 chunk 17 47426 bytes
21:19:14.671 in Ack 17
21:19:14.929 fetch POST /api/observability/v1/traces
21:19:14.930 fetched POST /api/observability/v1/traces 204
21:19:15.105 chunk 17 47436 bytes
21:19:15.106 in Ack 17
21:19:15.542 chunk 17 47468 bytes
21:19:15.544 in Ack 17
21:19:16.107 fetch POST /api/observability/v1/traces
21:19:16.107 fetched POST /api/observability/v1/traces 204
21:19:17.539 fetch POST /api/observability/v1/traces
21:19:17.539 fetched POST /api/observability/v1/traces 204
21:19:19.581 in Ping 
21:19:19.795 in Interrupt 31
21:19:20.796 fetch POST /api/observability/v1/traces
21:19:20.796 fetched POST /api/observability/v1/traces 204
21:19:24.585 in Ping 
21:19:29.589 in Ping 
21:19:29.791 rpc 34 server.reportClientActivity
21:19:30.791 fetch POST /api/observability/v1/traces
21:19:30.791 fetched POST /api/observability/v1/traces 204
21:19:33.540 chunk 17 47718 bytes
21:19:33.541 in Ack 17
21:19:34.200 chunk 17 47880 bytes
21:19:34.201 in Ack 17
21:19:34.543 fetch POST /api/observability/v1/traces
21:19:34.543 fetched POST /api/observability/v1/traces 204
21:19:34.593 in Ping 
21:19:35.094 chunk 17 48144 bytes
21:19:35.095 in Ack 17
21:19:36.096 fetch POST /api/observability/v1/traces
21:19:36.096 fetched POST /api/observability/v1/traces 204
21:19:36.488 chunk 17 48424 bytes
21:19:36.489 in Ack 17
21:19:36.998 chunk 17 48604 bytes
21:19:37.000 in Ack 17
21:19:37.490 fetch POST /api/observability/v1/traces
21:19:37.490 fetched POST /api/observability/v1/traces 204
21:19:39.204 chunk 17 48662 bytes
21:19:39.206 in Ack 17
21:19:39.597 in Ping 
21:19:40.208 fetch POST /api/observability/v1/traces
21:19:40.208 fetched POST /api/observability/v1/traces 204
21:19:41.097 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:19:41.097 exit 30 {"_tag":"Success","value":null}
21:19:41.819 rpc 35 orchestration.dispatchCommand runtime-request.respond
21:19:42.104 fetch POST /api/observability/v1/traces
21:19:42.104 fetched POST /api/observability/v1/traces 204
21:19:42.262 chunk 17 48754 bytes
21:19:42.264 in Ack 17
