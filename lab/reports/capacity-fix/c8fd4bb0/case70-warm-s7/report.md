# case70, webapp, warm start, seed 7

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 73 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.8s |
| Prompt reached the agent | 3.6s |
| Time to first text | 5.9s |
| Whole reply shown | 72.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 15.0s / 5.3s / 12.2s |
| Longest silence | 6.3s |
| Approval shown after it was asked | 17.5s |
| Bytes on the wire down / up | 79992 / 38877 |
| Bytes the app received / sent | 61017 / 93479 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.9 | 57 | 258 |
| 20 | 5.0 | 452 | 655 |
| 30 | 9.5 | 672 | 1061 |
| 40 | 11.0 | 1017 | 1077 |
| 50 | 0.9 | 1077 | 1116 |
| 60 | 7.1 | 1225 | 1492 |
| 70 | 10.2 | 1479 | 1596 |
| 72 | 12.2 | 1479 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 69/548","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174338255,"silences":0,"longestSilenceMs":1239,"longestRecoveredGapMs":1239},"stats":{"sendRate":3551.6887185341375,"deliveredRate":3150.4562655551335,"loss":0,"srttMs":28.592236062349095,"minRttMs":2,"sentPackets":115,"sentWireBytes":34242,"receivedPackets":102,"receivedWireBytes":51841,"lastHeardMs":9058,"unsyncedBytes":998,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 410/849 104/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 295/295","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174349293,"silences":0,"longestSilenceMs":1630,"longestRecoveredGapMs":1630},"stats":{"sendRate":2146.1702696050484,"deliveredRate":1354.8351473319844,"loss":0.4375765550239234,"srttMs":850.1928012542694,"minRttMs":2,"sentPackets":141,"sentWireBytes":42436,"receivedPackets":118,"receivedWireBytes":59500,"lastHeardMs":20096,"unsyncedBytes":0,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 818/849 1031/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 668/693","user":"10/10 9/9","attempts":"661/661 661/661 168/637","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174359109,"silences":0,"longestSilenceMs":1686,"longestRecoveredGapMs":1686},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.5180682943990993,"srttMs":1126.6687010974856,"minRttMs":3062,"sentPackets":200,"sentWireBytes":55875,"receivedPackets":145,"receivedWireBytes":67670,"lastHeardMs":29912,"unsyncedBytes":327,"inflightPackets":55,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 47/839","assistant":"32/32 21/21 998/1103","user":"10/10 9/9","attempts":"661/661 661/661 534/637","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174369245,"silences":0,"longestSilenceMs":1686,"longestRecoveredGapMs":1686},"stats":{"sendRate":2000,"deliveredRate":453.5930622511004,"loss":0.6861413043478259,"srttMs":3110.302377275432,"minRttMs":3062,"sentPackets":243,"sentWireBytes":67607,"receivedPackets":182,"receivedWireBytes":78452,"lastHeardMs":40048,"unsyncedBytes":134,"inflightPackets":87,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 99/839","assistant":"32/32 21/21 1330/1340","user":"10/10 9/9","attempts":"661/661 661/661 534/637","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"451/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174379294,"silences":0,"longestSilenceMs":1686,"longestRecoveredGapMs":1686},"stats":{"sendRate":2167.8213884208426,"deliveredRate":453.5930622511004,"loss":0.6861413043478259,"srttMs":3110.302377275432,"minRttMs":3062,"sentPackets":255,"sentWireBytes":70857,"receivedPackets":212,"receivedWireBytes":89265,"lastHeardMs":50097,"unsyncedBytes":134,"inflightPackets":99,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078 917/1550 50/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 99/839","assistant":"32/32 21/21 1340/1340 53/53","user":"10/10 9/9","attempts":"661/661 661/661 534/637","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174389428,"silences":0,"longestSilenceMs":1686,"longestRecoveredGapMs":1686},"stats":{"sendRate":2533.7791860938705,"deliveredRate":395.48186531300985,"loss":0.7658743109007695,"srttMs":2299.904033512373,"minRttMs":1297,"sentPackets":321,"sentWireBytes":92042,"receivedPackets":238,"receivedWireBytes":98252,"lastHeardMs":60231,"unsyncedBytes":400,"inflightPackets":148,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 722/1103 917/1550 50/929 981/1078","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 99/839","assistant":"32/32 21/21 1340/1340 349/469","user":"10/10 9/9","attempts":"661/661 661/661 603/637","providerTurns":"776/776 776/776","providerSessions":"2685/2685","providerThreads":"809/809","messages":"456/456 548/548 548/548 901/901","nodes":"1022/1022 809/809 809/809 999/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"867/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":236,"lastDataAt":1794174400508,"silences":0,"longestSilenceMs":1998,"longestRecoveredGapMs":1998},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.7661120176305386,"srttMs":2879.510235088835,"minRttMs":1297,"sentPackets":381,"sentWireBytes":111917,"receivedPackets":256,"receivedWireBytes":107789,"lastHeardMs":71311,"unsyncedBytes":400,"inflightPackets":169,"duplicateBytes":0}}
- shim trace
21:45:29.498 fetch GET /api/auth/session
21:45:29.499 fetched GET /api/auth/session 200
21:45:29.639 fetch GET /.well-known/t3/environment
21:45:29.639 fetched GET /.well-known/t3/environment 200
21:45:29.655 fetch GET /.well-known/t3/environment
21:45:29.656 fetched GET /.well-known/t3/environment 200
21:45:29.659 rpc 0 subscribeServerConfig
21:45:29.660 chunk 0 38678 bytes
21:45:29.675 in Ack 0
21:45:29.676 rpc 1 subscribeServerLifecycle
21:45:29.677 rpc 2 subscribeServerLifecycle
21:45:29.679 fetch GET /api/auth/session
21:45:29.680 fetched GET /api/auth/session 200
21:45:29.680 rpc 3 subscribeDeviceState
21:45:29.682 rpc 4 preview.reportProfiles
21:45:29.716 rpc 5 subscribeProjectClones
21:45:29.732 fetch GET /api/orchestration/threads/mcp%3A8c61a6e9-fe62-4772-b711-e2edc790770d/bounded
21:45:29.737 fetch GET /api/orchestration/shell
21:45:29.758 rpc 6 server.refreshProviders
21:45:29.796 rpc 7 orchestration.dispatchCommand thread.visit
21:45:29.796 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:45:29.805 fetched GET /api/orchestration/shell 200
21:45:29.806 rpc 8 subscribeTerminalMetadata
21:45:29.807 rpc 9 subscribeVcsStatus
21:45:29.807 rpc 10 subscribePreviewEvents
21:45:29.809 rpc 11 orchestration.subscribeShell
21:45:29.809 chunk 11 2779 bytes
21:45:29.810 rpc 12 assets.createUrl
21:45:29.811 rpc 13 preview.list
21:45:29.811 rpc 14 projects.readFile
21:45:29.817 in Ack 11
21:45:30.060 rpc 15 server.reportClientActivity
21:45:30.091 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"a53dca1b-b3cc-45b5-8d10-f03fb98bce87","sessions":[]}}
21:45:30.091 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:45:30.600 chunk 1 3861 bytes
21:45:30.601 in Ack 1
21:45:30.651 fetch POST /api/observability/v1/traces
21:45:30.651 fetched POST /api/observability/v1/traces 204
21:45:30.859 chunk 2 3861 bytes
21:45:30.860 in Ack 2
21:45:31.049 chunk 11 2755 bytes
21:45:31.049 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:45:16.802Z","configurableProviders":false,"con
21:45:31.049 chunk 9 472 bytes
21:45:31.049 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:45:31.049 in Ack 11
21:45:31.056 in Ack 9
21:45:31.529 chunk 3 876 bytes
21:45:31.529 exit 4 {"_tag":"Success","value":null}
21:45:31.529 chunk 5 4 bytes
21:45:31.529 chunk 8 36 bytes
21:45:31.529 exit 15 {"_tag":"Success","value":null}
21:45:31.530 in Ack 3
21:45:31.533 rpc 16 preview.reportProfiles
21:45:31.534 in Ack 5
21:45:31.534 in Ack 8
21:45:31.550 chunk 11 2798 bytes
21:45:31.550 in Ack 11
21:45:31.606 exit 16 {"_tag":"Success","value":null}
21:45:31.859 fetched GET /api/orchestration/threads/mcp%3A8c61a6e9-fe62-4772-b711-e2edc790770d/bounded 200
21:45:31.861 rpc 17 orchestration.subscribeThread
21:45:31.862 chunk 17 8294 bytes
21:45:31.871 in Interrupt 9
21:45:31.873 rpc 18 subscribeVcsStatus
21:45:31.873 fetch POST /api/observability/v1/traces
21:45:31.873 fetched POST /api/observability/v1/traces 204
21:45:31.874 in Ack 17
21:45:32.028 chunk 18 472 bytes
21:45:32.028 in Ack 18
21:45:32.112 chunk 17 9348 bytes
21:45:32.113 in Ack 17
21:45:32.133 rpc 19 server.reportClientActivity
21:45:32.365 chunk 17 11932 bytes
21:45:32.366 in Ack 17
21:45:32.518 exit 19 {"_tag":"Success","value":null}
21:45:32.542 rpc 20 attachments.createUploadUrl
21:45:32.729 chunk 17 13517 bytes
21:45:32.747 in Ack 17
21:45:32.838 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-6eb9cd03-0d46-4c02-9999-8f09a4661be4","expiresAt":1794174932748,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:45:32.839 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTZlYjljZDAzLTBkNDYtNGMwMi05OTk5LThmMDlhNDY2MWJlNCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NDkzMjc0OH0.-Rw7-J1E6sybs9heFyRjE377GBDNanVvLbo9sBEO6b4
21:45:32.876 fetch POST /api/observability/v1/traces
21:45:32.876 fetched POST /api/observability/v1/traces 204
21:45:32.979 chunk 17 16202 bytes
21:45:32.980 in Ack 17
21:45:33.273 chunk 17 18614 bytes
21:45:33.276 in Ack 17
21:45:33.523 chunk 17 20474 bytes
21:45:33.524 in Ack 17
21:45:33.530 in Interrupt 18
21:45:33.531 rpc 21 subscribeVcsStatus
21:45:33.619 chunk 21 472 bytes
21:45:33.619 in Ack 21
21:45:33.774 chunk 17 21197 bytes
21:45:33.774 in Ack 17
21:45:33.783 rpc 22 server.reportClientActivity
21:45:33.981 fetch POST /api/observability/v1/traces
21:45:33.981 fetched POST /api/observability/v1/traces 204
21:45:34.597 exit 22 {"_tag":"Success","value":null}
21:45:34.658 in Ping 
21:45:35.412 rpc 23 orchestration.dispatchCommand message.dispatch
21:45:35.958 fetch POST /api/observability/v1/traces
21:45:35.958 fetched POST /api/observability/v1/traces 204
21:45:36.000 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:45:36.002 rpc 24 attachments.delete
21:45:36.250 chunk 17 23642 bytes
21:45:36.251 in Ack 17
21:45:36.360 exit 24 {"_tag":"Success","value":null}
21:45:36.500 chunk 11 2884 bytes
21:45:36.500 chunk 17 23614 bytes
21:45:36.501 in Ack 11
21:45:36.536 in Ack 17
21:45:36.751 chunk 17 24264 bytes
21:45:36.752 in Ack 17
21:45:37.000 chunk 11 2820 bytes
21:45:37.000 chunk 17 24314 bytes
21:45:37.000 in Ack 11
21:45:37.002 rpc 25 orchestration.dispatchCommand thread.visit
21:45:37.002 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:45:37.007 in Ack 17
21:45:37.040 fetch POST /api/observability/v1/traces
21:45:37.040 fetched POST /api/observability/v1/traces 204
21:45:37.250 chunk 11 2820 bytes
21:45:37.251 chunk 17 24314 bytes
21:45:37.251 in Ack 11
21:45:37.255 in Ack 17
21:45:37.501 chunk 17 28103 bytes
21:45:37.502 in Ack 17
21:45:37.505 rpc 26 assets.createUrl
21:45:37.751 chunk 17 31336 bytes
21:45:37.752 in Ack 17
21:45:38.002 chunk 17 31950 bytes
21:45:38.004 in Ack 17
21:45:38.010 in Interrupt 21
21:45:38.012 rpc 27 subscribeVcsStatus
21:45:38.255 chunk 27 472 bytes
21:45:38.256 fetch POST /api/observability/v1/traces
21:45:38.256 fetched POST /api/observability/v1/traces 204
21:45:38.256 in Ack 27
21:45:38.263 rpc 28 server.reportClientActivity
21:45:38.510 chunk 17 32727 bytes
21:45:38.511 in Ack 17
21:45:39.096 rpc 29 orchestration.dispatchCommand message.dispatch
21:45:39.263 fetch POST /api/observability/v1/traces
21:45:39.263 fetched POST /api/observability/v1/traces 204
21:45:39.347 chunk 17 33638 bytes
21:45:39.348 in Ack 17
21:45:39.658 in Ping 
21:45:40.349 fetch POST /api/observability/v1/traces
21:45:40.349 fetched POST /api/observability/v1/traces 204
21:45:40.507 exit 28 {"_tag":"Success","value":null}
21:45:41.507 fetch POST /api/observability/v1/traces
21:45:41.508 fetched POST /api/observability/v1/traces 204
21:45:41.763 chunk 17 34208 bytes
21:45:41.764 in Ack 17
21:45:42.765 fetch POST /api/observability/v1/traces
21:45:42.765 fetched POST /api/observability/v1/traces 204
21:45:43.664 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:45:44.660 in Ping 
21:45:44.665 fetch POST /api/observability/v1/traces
21:45:44.665 fetched POST /api/observability/v1/traces 204
21:45:44.803 chunk 17 36783 bytes
21:45:44.804 in Ack 17
21:45:45.805 fetch POST /api/observability/v1/traces
21:45:45.805 fetched POST /api/observability/v1/traces 204
21:45:45.822 chunk 17 36835 bytes
21:45:45.823 in Ack 17
21:45:46.824 fetch POST /api/observability/v1/traces
21:45:46.824 fetched POST /api/observability/v1/traces 204
21:45:47.452 chunk 17 36867 bytes
21:45:47.454 in Ack 17
21:45:48.455 fetch POST /api/observability/v1/traces
21:45:48.455 fetched POST /api/observability/v1/traces 204
21:45:48.713 chunk 17 37125 bytes
21:45:48.715 in Ack 17
21:45:49.294 chunk 17 37233 bytes
21:45:49.295 in Ack 17
21:45:49.661 in Ping 
21:45:49.716 fetch POST /api/observability/v1/traces
21:45:49.716 fetched POST /api/observability/v1/traces 204
21:45:50.980 chunk 17 37265 bytes
21:45:50.981 in Ack 17
21:45:51.232 chunk 17 37277 bytes
21:45:51.233 in Ack 17
21:45:51.880 chunk 17 37299 bytes
21:45:51.881 in Ack 17
21:45:51.983 fetch POST /api/observability/v1/traces
21:45:51.983 fetched POST /api/observability/v1/traces 204
21:45:52.366 chunk 17 37469 bytes
21:45:52.367 in Ack 17
21:45:52.617 chunk 17 37481 bytes
21:45:52.618 in Ack 17
21:45:52.949 chunk 17 37511 bytes
21:45:52.950 in Ack 17
21:45:53.242 chunk 17 37533 bytes
21:45:53.243 in Ack 17
21:45:53.368 fetch POST /api/observability/v1/traces
21:45:53.368 fetched POST /api/observability/v1/traces 204
21:45:53.627 chunk 17 37575 bytes
21:45:53.628 in Ack 17
21:45:54.138 chunk 17 37639 bytes
21:45:54.139 in Ack 17
21:45:54.392 chunk 17 37669 bytes
21:45:54.393 in Ack 17
21:45:54.629 fetch POST /api/observability/v1/traces
21:45:54.629 fetched POST /api/observability/v1/traces 204
21:45:54.648 chunk 17 37699 bytes
21:45:54.649 in Ack 17
21:45:54.661 in Ping 
21:45:54.890 rpc 30 server.reportClientActivity
21:45:54.979 chunk 17 37731 bytes
21:45:54.980 in Ack 17
21:45:55.407 chunk 17 37741 bytes
21:45:55.408 in Ack 17
21:45:55.650 fetch POST /api/observability/v1/traces
21:45:55.650 fetched POST /api/observability/v1/traces 204
21:45:55.832 chunk 17 37803 bytes
21:45:55.833 in Ack 17
21:45:56.082 chunk 17 37847 bytes
21:45:56.083 in Ack 17
21:45:56.389 chunk 17 37867 bytes
21:45:56.390 in Ack 17
21:45:56.833 fetch POST /api/observability/v1/traces
21:45:56.833 fetched POST /api/observability/v1/traces 204
21:45:56.906 chunk 17 37877 bytes
21:45:56.908 in Ack 17
21:45:57.160 chunk 17 37897 bytes
21:45:57.162 in Ack 17
21:45:57.750 chunk 17 37907 bytes
21:45:57.751 in Ack 17
21:45:57.908 fetch POST /api/observability/v1/traces
21:45:57.908 fetched POST /api/observability/v1/traces 204
21:45:58.017 chunk 17 37919 bytes
21:45:58.018 in Ack 17
21:45:58.267 chunk 17 37931 bytes
21:45:58.268 in Ack 17
21:45:58.517 chunk 17 37959 bytes
21:45:58.518 in Ack 17
21:45:59.024 fetch POST /api/observability/v1/traces
21:45:59.024 fetched POST /api/observability/v1/traces 204
21:45:59.110 chunk 17 37989 bytes
21:45:59.112 in Ack 17
21:45:59.619 chunk 17 37999 bytes
21:45:59.620 in Ack 17
21:45:59.662 in Ping 
21:46:00.113 fetch POST /api/observability/v1/traces
21:46:00.113 fetched POST /api/observability/v1/traces 204
21:46:00.128 chunk 17 37963 bytes
21:46:00.130 in Ack 17
21:46:00.385 chunk 17 37983 bytes
21:46:00.386 in Ack 17
21:46:00.637 chunk 17 38003 bytes
21:46:00.638 in Ack 17
21:46:00.896 chunk 17 38061 bytes
21:46:00.899 in Ack 17
21:46:01.131 fetch POST /api/observability/v1/traces
21:46:01.131 fetched POST /api/observability/v1/traces 204
21:46:01.402 chunk 17 38091 bytes
21:46:01.403 in Ack 17
21:46:01.737 chunk 17 38101 bytes
21:46:01.738 in Ack 17
21:46:01.992 chunk 17 38131 bytes
21:46:01.993 in Ack 17
21:46:02.404 fetch POST /api/observability/v1/traces
21:46:02.404 fetched POST /api/observability/v1/traces 204
21:46:02.502 chunk 17 38133 bytes
21:46:02.504 in Ack 17
21:46:02.760 chunk 17 38171 bytes
21:46:02.763 in Ack 17
21:46:03.264 chunk 17 38183 bytes
21:46:03.265 in Ack 17
21:46:03.429 fetch POST /api/observability/v1/traces
21:46:03.429 fetched POST /api/observability/v1/traces 204
21:46:03.513 chunk 17 38213 bytes
21:46:03.516 in Ack 17
21:46:03.764 chunk 17 38233 bytes
21:46:03.766 in Ack 17
21:46:04.101 chunk 17 38255 bytes
21:46:04.102 in Ack 17
21:46:04.351 chunk 17 38267 bytes
21:46:04.352 in Ack 17
21:46:04.517 fetch POST /api/observability/v1/traces
21:46:04.517 fetched POST /api/observability/v1/traces 204
21:46:04.662 in Ping 
21:46:04.850 chunk 17 38277 bytes
21:46:04.851 in Ack 17
21:46:05.441 chunk 17 38309 bytes
21:46:05.443 in Ack 17
21:46:05.781 chunk 17 38319 bytes
21:46:05.782 in Ack 17
21:46:05.852 fetch POST /api/observability/v1/traces
21:46:05.852 fetched POST /api/observability/v1/traces 204
21:46:06.119 chunk 17 38371 bytes
21:46:06.120 in Ack 17
21:46:06.373 chunk 17 38419 bytes
21:46:06.375 in Ack 17
21:46:06.632 chunk 17 38449 bytes
21:46:06.633 in Ack 17
21:46:06.884 chunk 17 38461 bytes
21:46:06.886 in Ack 17
21:46:07.120 fetch POST /api/observability/v1/traces
21:46:07.121 fetched POST /api/observability/v1/traces 204
21:46:07.139 chunk 17 38463 bytes
21:46:07.140 in Ack 17
21:46:07.480 chunk 17 38501 bytes
21:46:07.481 in Ack 17
21:46:07.737 chunk 17 38511 bytes
21:46:07.738 in Ack 17
21:46:07.996 chunk 17 38539 bytes
21:46:07.997 in Ack 17
21:46:08.141 fetch POST /api/observability/v1/traces
21:46:08.141 fetched POST /api/observability/v1/traces 204
21:46:08.248 chunk 17 38549 bytes
21:46:08.249 in Ack 17
21:46:08.506 chunk 17 38559 bytes
21:46:08.507 in Ack 17
21:46:08.987 chunk 17 38589 bytes
21:46:08.988 in Ack 17
21:46:09.246 chunk 17 38609 bytes
21:46:09.247 in Ack 17
21:46:09.255 fetch POST /api/observability/v1/traces
21:46:09.255 fetched POST /api/observability/v1/traces 204
21:46:09.501 chunk 17 38639 bytes
21:46:09.502 in Ack 17
21:46:09.662 in Ping 
21:46:09.759 chunk 17 38657 bytes
21:46:09.760 in Ack 17
21:46:09.892 in Interrupt 30
21:46:10.093 chunk 17 38697 bytes
21:46:10.094 in Ack 17
21:46:10.344 chunk 17 38739 bytes
21:46:10.345 in Ack 17
21:46:10.503 fetch POST /api/observability/v1/traces
21:46:10.503 fetched POST /api/observability/v1/traces 204
21:46:10.856 chunk 17 38817 bytes
21:46:10.857 in Ack 17
21:46:11.222 chunk 17 38839 bytes
21:46:11.224 in Ack 17
21:46:11.473 chunk 17 38879 bytes
21:46:11.474 in Ack 17
21:46:11.723 chunk 17 38909 bytes
21:46:11.724 in Ack 17
21:46:11.858 fetch POST /api/observability/v1/traces
21:46:11.858 fetched POST /api/observability/v1/traces 204
21:46:11.972 chunk 17 38949 bytes
21:46:11.973 in Ack 17
21:46:12.364 chunk 17 38981 bytes
21:46:12.365 in Ack 17
21:46:12.976 fetch POST /api/observability/v1/traces
21:46:12.976 fetched POST /api/observability/v1/traces 204
21:46:13.022 chunk 17 39001 bytes
21:46:13.023 in Ack 17
21:46:13.273 chunk 17 39023 bytes
21:46:13.274 in Ack 17
21:46:13.523 chunk 17 39053 bytes
21:46:13.524 in Ack 17
21:46:13.773 chunk 17 39083 bytes
21:46:13.775 in Ack 17
21:46:14.023 chunk 17 39103 bytes
21:46:14.025 in Ack 17
21:46:14.034 fetch POST /api/observability/v1/traces
21:46:14.034 fetched POST /api/observability/v1/traces 204
21:46:14.274 chunk 17 39123 bytes
21:46:14.275 in Ack 17
21:46:14.525 chunk 17 39187 bytes
21:46:14.526 in Ack 17
21:46:14.663 in Ping 
21:46:15.276 fetch POST /api/observability/v1/traces
21:46:15.276 fetched POST /api/observability/v1/traces 204
21:46:15.380 chunk 17 39215 bytes
21:46:15.381 in Ack 17
21:46:15.919 chunk 17 39233 bytes
21:46:15.920 in Ack 17
21:46:16.382 fetch POST /api/observability/v1/traces
21:46:16.382 fetched POST /api/observability/v1/traces 204
21:46:16.981 chunk 17 39251 bytes
21:46:16.982 in Ack 17
21:46:17.982 fetch POST /api/observability/v1/traces
21:46:17.982 fetched POST /api/observability/v1/traces 204
21:46:18.127 chunk 17 39259 bytes
21:46:18.128 in Ack 17
21:46:18.957 chunk 17 39267 bytes
21:46:18.958 in Ack 17
21:46:19.129 fetch POST /api/observability/v1/traces
21:46:19.129 fetched POST /api/observability/v1/traces 204
21:46:19.663 in Ping 
21:46:19.893 rpc 31 server.reportClientActivity
21:46:20.893 fetch POST /api/observability/v1/traces
21:46:20.893 fetched POST /api/observability/v1/traces 204
21:46:23.323 chunk 17 39283 bytes
21:46:23.324 in Ack 17
21:46:23.434 fetch POST /api/observability/v1/traces
21:46:23.434 fetched POST /api/observability/v1/traces 204
21:46:24.665 in Ping 
21:46:26.853 chunk 17 40291 bytes
21:46:26.854 in Ack 17
21:46:27.007 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:46:27.855 fetch POST /api/observability/v1/traces
21:46:27.855 fetched POST /api/observability/v1/traces 204
21:46:29.429 chunk 17 42832 bytes
21:46:29.429 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:46:29.430 in Ack 17
21:46:29.589 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:46:29.669 in Ping 
21:46:30.302 chunk 17 42892 bytes
21:46:30.303 in Ack 17
21:46:30.431 fetch POST /api/observability/v1/traces
21:46:30.431 fetched POST /api/observability/v1/traces 204
21:46:30.906 chunk 17 42952 bytes
21:46:30.907 in Ack 17
21:46:31.335 chunk 17 42962 bytes
21:46:31.336 in Ack 17
21:46:31.859 chunk 17 42972 bytes
21:46:31.860 in Ack 17
21:46:31.908 fetch POST /api/observability/v1/traces
21:46:31.908 fetched POST /api/observability/v1/traces 204
21:46:33.437 fetch POST /api/observability/v1/traces
21:46:33.437 fetched POST /api/observability/v1/traces 204
21:46:33.857 chunk 17 43082 bytes
21:46:33.857 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:46:33.858 in Ack 17
21:46:34.384 chunk 17 43142 bytes
21:46:34.385 in Ack 17
21:46:34.486 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:46:34.673 in Ping 
21:46:34.821 chunk 17 43152 bytes
21:46:34.822 in Ack 17
21:46:34.859 fetch POST /api/observability/v1/traces
21:46:34.859 fetched POST /api/observability/v1/traces 204
21:46:34.893 in Interrupt 31
21:46:35.249 chunk 17 43174 bytes
21:46:35.250 in Ack 17
21:46:35.683 chunk 17 43194 bytes
21:46:35.686 in Ack 17
21:46:35.894 fetch POST /api/observability/v1/traces
21:46:35.894 fetched POST /api/observability/v1/traces 204
21:46:36.125 chunk 17 43246 bytes
21:46:36.126 in Ack 17
21:46:36.905 chunk 17 43298 bytes
21:46:36.906 in Ack 17
21:46:37.127 fetch POST /api/observability/v1/traces
21:46:37.127 fetched POST /api/observability/v1/traces 204
21:46:37.403 chunk 17 43320 bytes
21:46:37.404 in Ack 17
21:46:37.913 chunk 17 43350 bytes
21:46:37.914 in Ack 17
21:46:38.404 fetch POST /api/observability/v1/traces
21:46:38.405 fetched POST /api/observability/v1/traces 204
21:46:39.374 chunk 17 43392 bytes
21:46:39.374 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:46:39.375 in Ack 17
21:46:39.677 in Ping 
21:46:39.750 chunk 17 43404 bytes
21:46:39.751 in Ack 17
21:46:40.005 chunk 17 43414 bytes
21:46:40.006 in Ack 17
21:46:40.257 chunk 17 43424 bytes
21:46:40.258 in Ack 17
21:46:40.376 fetch POST /api/observability/v1/traces
21:46:40.376 fetched POST /api/observability/v1/traces 204
21:46:40.509 chunk 17 43434 bytes
21:46:40.510 in Ack 17
21:46:40.607 rpc 35 orchestration.dispatchCommand runtime-request.respond
21:46:40.845 chunk 17 43476 bytes
21:46:40.846 in Ack 17
21:46:41.188 chunk 17 43506 bytes
21:46:41.190 in Ack 17
21:46:41.511 fetch POST /api/observability/v1/traces
21:46:41.511 fetched POST /api/observability/v1/traces 204
21:46:41.531 chunk 17 43542 bytes
21:46:41.533 in Ack 17
21:46:41.790 chunk 17 43642 bytes
21:46:41.791 in Ack 17
21:46:42.041 chunk 17 43698 bytes
21:46:42.042 in Ack 17
21:46:42.429 chunk 17 43828 bytes
21:46:42.431 in Ack 17
21:46:42.533 fetch POST /api/observability/v1/traces
21:46:42.533 fetched POST /api/observability/v1/traces 204
21:46:42.680 chunk 17 43854 bytes
21:46:42.681 in Ack 17
21:46:43.184 chunk 17 43922 bytes
21:46:43.185 in Ack 17
21:46:43.682 fetch POST /api/observability/v1/traces
21:46:43.682 fetched POST /api/observability/v1/traces 204
21:46:44.681 in Ping 
21:46:44.893 rpc 36 server.reportClientActivity
21:46:45.893 fetch POST /api/observability/v1/traces
21:46:45.894 fetched POST /api/observability/v1/traces 204
21:46:46.373 chunk 17 44046 bytes
21:46:46.374 in Ack 17
21:46:47.375 fetch POST /api/observability/v1/traces
21:46:47.375 fetched POST /api/observability/v1/traces 204
21:46:47.787 chunk 17 44089 bytes
21:46:47.788 in Ack 17
21:46:48.046 chunk 17 44163 bytes
21:46:48.047 in Ack 17
21:46:48.789 fetch POST /api/observability/v1/traces
21:46:48.789 fetched POST /api/observability/v1/traces 204
21:46:49.685 in Ping 
21:46:50.620 chunk 17 44269 bytes
21:46:50.620 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:46:50.621 in Ack 17
21:46:51.729 fetch POST /api/observability/v1/traces
21:46:51.729 fetched POST /api/observability/v1/traces 204
