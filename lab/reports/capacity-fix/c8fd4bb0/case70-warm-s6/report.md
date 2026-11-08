# case70, webapp, warm start, seed 6

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 93 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 8.0s |
| Prompt reached the agent | 3.8s |
| Time to first text | 13.5s |
| Whole reply shown | 92.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 37.3s / 13.3s / 33.3s |
| Longest silence | 32.1s |
| Approval shown after it was asked | 29.6s |
| Bytes on the wire down / up | 96476 / 33616 |
| Bytes the app received / sent | 64765 / 76290 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.1 | 0 | 247 |
| 20 | 8.3 | 317 | 643 |
| 30 | 14.3 | 473 | 1049 |
| 40 | 24.3 | 473 | 1077 |
| 50 | 34.3 | 473 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 8.3 | 1101 | 1419 |
| 80 | 18.3 | 1101 | 1596 |
| 90 | 18.8 | 1465 | 1596 |
| 92 | 20.8 | 1465 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","user":"10/10 9/9","assistant":"32/32 21/21","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174218579,"silences":0,"longestSilenceMs":1141,"longestRecoveredGapMs":1141},"stats":{"sendRate":7016.350838075157,"deliveredRate":3514.7178655797093,"loss":0,"srttMs":3.81382164551136,"minRttMs":1,"sentPackets":104,"sentWireBytes":32109,"receivedPackets":128,"receivedWireBytes":67660,"lastHeardMs":8514,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","user":"10/10 9/9","assistant":"32/32 21/21 70/225","attempts":"661/661 661/661","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174229125,"silences":0,"longestSilenceMs":1146,"longestRecoveredGapMs":1146},"stats":{"sendRate":3508.1754190375786,"deliveredRate":2321.388471390734,"loss":0.22499999999999998,"srttMs":1039.4668321973445,"minRttMs":1,"sentPackets":133,"sentWireBytes":41766,"receivedPackets":147,"receivedWireBytes":78232,"lastHeardMs":19060,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 346/1078 597/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971","user":"10/10 9/9","assistant":"32/32 21/21 610/703","attempts":"661/661 661/661 67/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174239233,"silences":0,"longestSilenceMs":1767,"longestRecoveredGapMs":1767},"stats":{"sendRate":3000,"deliveredRate":273.6356648731745,"loss":0.7692307692307692,"srttMs":1305.638668401092,"minRttMs":1,"sentPackets":185,"sentWireBytes":53981,"receivedPackets":170,"receivedWireBytes":86159,"lastHeardMs":29168,"unsyncedBytes":135,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 819/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 52/839","user":"10/10 9/9","assistant":"32/32 21/21 793/883","attempts":"661/661 661/661 484/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174245565,"silences":0,"longestSilenceMs":1767,"longestRecoveredGapMs":1767},"stats":{"sendRate":1500,"deliveredRate":360.28574854173314,"loss":0.8069230769230767,"srttMs":1983.068482932763,"minRttMs":1,"sentPackets":232,"sentWireBytes":63608,"receivedPackets":192,"receivedWireBytes":92798,"lastHeardMs":35500,"unsyncedBytes":0,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 819/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 52/839","user":"10/10 9/9","assistant":"32/32 21/21 793/883","attempts":"661/661 661/661 484/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174245565,"silences":0,"longestSilenceMs":1767,"longestRecoveredGapMs":1767},"stats":{"sendRate":1500,"deliveredRate":360.28574854173314,"loss":0.8069230769230767,"srttMs":1983.068482932763,"minRttMs":1,"sentPackets":237,"sentWireBytes":64323,"receivedPackets":192,"receivedWireBytes":92798,"lastHeardMs":35500,"unsyncedBytes":0,"inflightPackets":65,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 819/849","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 52/839","user":"10/10 9/9","assistant":"32/32 21/21 1237/1340","attempts":"661/661 661/661 484/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"59/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174269317,"silences":2,"longestSilenceMs":17045,"longestRecoveredGapMs":17045},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.8069230769230767,"srttMs":1871.0599225661676,"minRttMs":1,"sentPackets":249,"sentWireBytes":67755,"receivedPackets":203,"receivedWireBytes":96252,"lastHeardMs":59252,"unsyncedBytes":489,"inflightPackets":75,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 819/849 1041/1550","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 52/839","user":"10/10 9/9","assistant":"32/32 21/21 1340/1340","attempts":"661/661 661/661 484/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174279532,"silences":2,"longestSilenceMs":17045,"longestRecoveredGapMs":17045},"stats":{"sendRate":1500,"deliveredRate":327.19886740348016,"loss":0.8019813519813519,"srttMs":2256.1927522706555,"minRttMs":1087,"sentPackets":303,"sentWireBytes":83061,"receivedPackets":224,"receivedWireBytes":104937,"lastHeardMs":69467,"unsyncedBytes":400,"inflightPackets":107,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 849/849 1350/1550 769/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 109/839","user":"10/10 9/9","assistant":"32/32 21/21 1340/1340 208/328","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174290915,"silences":2,"longestSilenceMs":17045,"longestRecoveredGapMs":17045},"stats":{"sendRate":2250,"deliveredRate":288.82012709436015,"loss":0.7709708624708623,"srttMs":2945.800976123497,"minRttMs":1087,"sentPackets":331,"sentWireBytes":93769,"receivedPackets":248,"receivedWireBytes":116406,"lastHeardMs":80850,"unsyncedBytes":889,"inflightPackets":128,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1103/1103 860/860 870/870 1299/1299 1103/1103 1058/1058 740/1078 849/849 1350/1550 769/929","frame":"1031/1031","thread":"951/951","runs":"1014/1014 971/971 215/839","user":"10/10 9/9","assistant":"32/32 21/21 1340/1340 552/723","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","messages":"456/456 548/548 901/901 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":315,"lastDataAt":1794174301815,"silences":2,"longestSilenceMs":17045,"longestRecoveredGapMs":17045},"stats":{"sendRate":1125,"deliveredRate":288.82012709436015,"loss":0.7709708624708623,"srttMs":2945.800976123497,"minRttMs":1087,"sentPackets":343,"sentWireBytes":100143,"receivedPackets":268,"receivedWireBytes":126606,"lastHeardMs":91751,"unsyncedBytes":889,"inflightPackets":120,"duplicateBytes":0}}
- shim trace
21:43:30.354 fetch GET /api/auth/session
21:43:30.355 fetched GET /api/auth/session 200
21:43:30.517 fetch GET /.well-known/t3/environment
21:43:30.518 fetched GET /.well-known/t3/environment 200
21:43:30.527 fetch GET /.well-known/t3/environment
21:43:30.527 fetched GET /.well-known/t3/environment 200
21:43:30.530 rpc 0 subscribeServerConfig
21:43:30.531 chunk 0 38678 bytes
21:43:30.598 in Ack 0
21:43:30.600 rpc 1 subscribeServerLifecycle
21:43:30.601 rpc 2 subscribeServerLifecycle
21:43:30.604 fetch GET /api/auth/session
21:43:30.605 fetched GET /api/auth/session 200
21:43:30.605 rpc 3 subscribeDeviceState
21:43:30.608 rpc 4 preview.reportProfiles
21:43:30.663 rpc 5 subscribeProjectClones
21:43:30.681 fetch GET /api/orchestration/threads/mcp%3A95668be0-afdf-4244-bcaa-b40da7486ac5/bounded
21:43:30.685 fetch GET /api/orchestration/shell
21:43:30.706 rpc 6 server.refreshProviders
21:43:30.744 rpc 7 orchestration.dispatchCommand thread.visit
21:43:30.744 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:43:30.755 fetched GET /api/orchestration/shell 200
21:43:30.756 rpc 8 subscribeTerminalMetadata
21:43:30.757 rpc 9 subscribeVcsStatus
21:43:30.757 rpc 10 subscribePreviewEvents
21:43:30.760 rpc 11 orchestration.subscribeShell
21:43:30.760 chunk 11 2779 bytes
21:43:30.761 rpc 12 assets.createUrl
21:43:30.762 rpc 13 preview.list
21:43:30.762 rpc 14 projects.readFile
21:43:30.767 in Ack 11
21:43:31.010 rpc 15 server.reportClientActivity
21:43:31.010 chunk 11 2755 bytes
21:43:31.010 in Ack 11
21:43:31.016 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"2ec33c87-cb80-4f50-867d-3b05f2d2ea11","sessions":[]}}
21:43:31.016 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:43:31.155 chunk 1 3861 bytes
21:43:31.157 in Ack 1
21:43:31.516 fetch POST /api/observability/v1/traces
21:43:31.516 fetched POST /api/observability/v1/traces 204
21:43:31.523 chunk 2 3861 bytes
21:43:31.523 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:43:18.050Z","configurableProviders":false,"con
21:43:31.523 chunk 9 472 bytes
21:43:31.523 in Ack 2
21:43:31.526 in Ack 9
21:43:31.649 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:43:31.848 chunk 3 876 bytes
21:43:31.848 exit 4 {"_tag":"Success","value":null}
21:43:31.848 chunk 5 4 bytes
21:43:31.848 chunk 8 36 bytes
21:43:31.848 exit 15 {"_tag":"Success","value":null}
21:43:31.849 in Ack 3
21:43:31.852 rpc 16 preview.reportProfiles
21:43:31.852 in Ack 5
21:43:31.853 in Ack 8
21:43:31.899 chunk 11 2798 bytes
21:43:31.900 in Ack 11
21:43:31.937 exit 16 {"_tag":"Success","value":null}
21:43:32.139 fetched GET /api/orchestration/threads/mcp%3A95668be0-afdf-4244-bcaa-b40da7486ac5/bounded 200
21:43:32.142 rpc 17 orchestration.subscribeThread
21:43:32.142 chunk 17 8294 bytes
21:43:32.152 in Interrupt 9
21:43:32.155 rpc 18 subscribeVcsStatus
21:43:32.156 in Ack 17
21:43:32.298 chunk 18 472 bytes
21:43:32.298 in Ack 18
21:43:32.397 chunk 17 11932 bytes
21:43:32.397 in Ack 17
21:43:32.431 rpc 19 server.reportClientActivity
21:43:32.524 fetch POST /api/observability/v1/traces
21:43:32.524 fetched POST /api/observability/v1/traces 204
21:43:32.687 chunk 17 13517 bytes
21:43:32.690 in Ack 17
21:43:32.879 rpc 20 attachments.createUploadUrl
21:43:33.047 exit 19 {"_tag":"Success","value":null}
21:43:33.053 chunk 17 18034 bytes
21:43:33.061 in Ack 17
21:43:33.079 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-fc012d62-9e64-45a5-a186-f4722b6f5a8f","expiresAt":1794174813068,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:43:33.080 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWZjMDEyZDYyLTllNjQtNDVhNS1hMTg2LWY0NzIyYjZmNWE4ZiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NDgxMzA2OH0.sxyRaf2FISYHNPyUTXS_TTCON9Hu7I4ImogJWv3Nkw8
21:43:33.303 chunk 17 19860 bytes
21:43:33.304 in Ack 17
21:43:33.313 in Interrupt 18
21:43:33.315 rpc 21 subscribeVcsStatus
21:43:33.378 chunk 21 472 bytes
21:43:33.378 in Ack 21
21:43:33.575 chunk 17 21197 bytes
21:43:33.576 rpc 22 server.reportClientActivity
21:43:33.578 in Ack 17
21:43:33.691 fetch POST /api/observability/v1/traces
21:43:33.691 fetched POST /api/observability/v1/traces 204
21:43:34.943 exit 22 {"_tag":"Success","value":null}
21:43:35.529 in Ping 
21:43:35.723 rpc 23 orchestration.dispatchCommand message.dispatch
21:43:36.269 fetch POST /api/observability/v1/traces
21:43:36.269 fetched POST /api/observability/v1/traces 204
21:43:36.319 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:43:36.321 rpc 24 attachments.delete
21:43:36.378 exit 24 {"_tag":"Success","value":null}
21:43:36.577 chunk 11 2884 bytes
21:43:36.577 chunk 17 27073 bytes
21:43:36.578 in Ack 11
21:43:36.604 in Ack 17
21:43:36.607 rpc 25 assets.createUrl
21:43:36.828 chunk 17 29320 bytes
21:43:36.829 in Ack 17
21:43:37.078 chunk 17 30305 bytes
21:43:37.080 in Ack 17
21:43:37.321 fetch POST /api/observability/v1/traces
21:43:37.321 fetched POST /api/observability/v1/traces 204
21:43:37.329 chunk 11 2820 bytes
21:43:37.329 chunk 17 30437 bytes
21:43:37.330 in Ack 11
21:43:37.331 rpc 26 orchestration.dispatchCommand thread.visit
21:43:37.331 exit 26 {"_tag":"Success","value":{"sequence":0}}
21:43:37.338 in Ack 17
21:43:37.580 chunk 11 2820 bytes
21:43:37.580 chunk 17 30461 bytes
21:43:37.580 in Ack 11
21:43:37.585 in Ack 17
21:43:37.830 chunk 17 33688 bytes
21:43:37.831 in Ack 17
21:43:37.837 in Interrupt 21
21:43:37.840 rpc 27 subscribeVcsStatus
21:43:37.888 chunk 27 472 bytes
21:43:37.888 in Ack 27
21:43:38.081 chunk 17 34208 bytes
21:43:38.082 in Ack 17
21:43:38.090 rpc 28 server.reportClientActivity
21:43:38.209 exit 28 {"_tag":"Success","value":null}
21:43:38.565 fetch POST /api/observability/v1/traces
21:43:38.565 fetched POST /api/observability/v1/traces 204
21:43:38.565 chunk 17 35231 bytes
21:43:38.568 in Ack 17
21:43:39.267 rpc 29 orchestration.dispatchCommand message.dispatch
21:43:39.569 fetch POST /api/observability/v1/traces
21:43:39.569 fetched POST /api/observability/v1/traces 204
21:43:40.529 in Ping 
21:43:42.268 fetch POST /api/observability/v1/traces
21:43:42.268 fetched POST /api/observability/v1/traces 204
21:43:42.977 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/34ba7a3b-e93a-4e86-a02b-768c30ffa32a","expiresAt":1794177822977}}
21:43:43.704 fetch POST /api/observability/v1/traces
21:43:43.704 fetched POST /api/observability/v1/traces 204
21:43:45.530 in Ping 
21:43:47.082 chunk 17 37698 bytes
21:43:47.082 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:43:47.083 in Ack 17
21:43:47.604 chunk 17 37730 bytes
21:43:47.605 in Ack 17
21:43:48.041 chunk 17 37750 bytes
21:43:48.042 in Ack 17
21:43:48.085 fetch POST /api/observability/v1/traces
21:43:48.085 fetched POST /api/observability/v1/traces 204
21:43:49.126 chunk 17 37812 bytes
21:43:49.127 in Ack 17
21:43:50.128 fetch POST /api/observability/v1/traces
21:43:50.128 fetched POST /api/observability/v1/traces 204
21:43:50.530 in Ping 
21:43:50.893 chunk 17 38112 bytes
21:43:50.894 in Ack 17
21:43:51.895 fetch POST /api/observability/v1/traces
21:43:51.895 fetched POST /api/observability/v1/traces 204
21:43:52.427 chunk 17 38214 bytes
21:43:52.429 in Ack 17
21:43:53.185 chunk 17 38256 bytes
21:43:53.186 in Ack 17
21:43:53.429 fetch POST /api/observability/v1/traces
21:43:53.429 fetched POST /api/observability/v1/traces 204
21:43:53.571 chunk 17 38266 bytes
21:43:53.573 in Ack 17
21:43:53.961 chunk 17 38286 bytes
21:43:53.962 in Ack 17
21:43:54.360 chunk 17 38298 bytes
21:43:54.361 in Ack 17
21:43:54.575 fetch POST /api/observability/v1/traces
21:43:54.575 fetched POST /api/observability/v1/traces 204
21:43:54.763 chunk 17 38320 bytes
21:43:54.764 in Ack 17
21:43:55.194 chunk 17 38340 bytes
21:43:55.195 in Ack 17
21:43:55.449 chunk 17 38598 bytes
21:43:55.450 in Ack 17
21:43:55.530 in Ping 
21:43:55.765 fetch POST /api/observability/v1/traces
21:43:55.765 fetched POST /api/observability/v1/traces 204
21:43:55.770 rpc 30 server.reportClientActivity
21:43:55.790 chunk 17 38628 bytes
21:43:55.793 in Ack 17
21:43:56.135 chunk 17 38640 bytes
21:43:56.136 in Ack 17
21:43:56.385 chunk 17 38652 bytes
21:43:56.386 in Ack 17
21:43:56.771 fetch POST /api/observability/v1/traces
21:43:56.771 fetched POST /api/observability/v1/traces 204
21:43:56.889 chunk 17 38674 bytes
21:43:56.892 in Ack 17
21:43:57.142 chunk 17 38686 bytes
21:43:57.143 in Ack 17
21:43:57.397 chunk 17 38696 bytes
21:43:57.398 in Ack 17
21:43:57.649 chunk 17 38716 bytes
21:43:57.650 in Ack 17
21:43:57.893 fetch POST /api/observability/v1/traces
21:43:57.893 fetched POST /api/observability/v1/traces 204
21:43:57.904 chunk 17 38728 bytes
21:43:57.906 in Ack 17
21:43:58.156 chunk 17 38750 bytes
21:43:58.157 in Ack 17
21:43:58.481 chunk 17 38762 bytes
21:43:58.482 in Ack 17
21:43:58.731 chunk 17 38774 bytes
21:43:58.733 in Ack 17
21:43:58.907 fetch POST /api/observability/v1/traces
21:43:58.907 fetched POST /api/observability/v1/traces 204
21:43:58.982 chunk 17 38784 bytes
21:43:58.983 in Ack 17
21:43:59.233 chunk 17 38910 bytes
21:43:59.235 in Ack 17
21:43:59.486 chunk 17 38926 bytes
21:43:59.487 in Ack 17
21:43:59.744 chunk 17 38992 bytes
21:43:59.746 in Ack 17
21:43:59.984 fetch POST /api/observability/v1/traces
21:43:59.984 fetched POST /api/observability/v1/traces 204
21:44:00.331 chunk 17 39012 bytes
21:44:00.332 in Ack 17
21:44:00.530 in Ping 
21:44:00.586 chunk 17 39052 bytes
21:44:00.587 in Ack 17
21:44:01.027 chunk 17 39074 bytes
21:44:01.028 in Ack 17
21:44:01.277 chunk 17 39084 bytes
21:44:01.278 in Ack 17
21:44:01.332 fetch POST /api/observability/v1/traces
21:44:01.333 fetched POST /api/observability/v1/traces 204
21:44:01.530 chunk 17 39094 bytes
21:44:01.543 in Ack 17
21:44:01.778 chunk 17 39106 bytes
21:44:01.779 in Ack 17
21:44:02.028 chunk 17 39116 bytes
21:44:02.029 in Ack 17
21:44:02.544 fetch POST /api/observability/v1/traces
21:44:02.544 fetched POST /api/observability/v1/traces 204
21:44:02.709 chunk 17 39138 bytes
21:44:02.710 in Ack 17
21:44:02.960 chunk 17 39148 bytes
21:44:02.961 in Ack 17
21:44:03.212 chunk 17 39158 bytes
21:44:03.213 in Ack 17
21:44:03.471 chunk 17 39168 bytes
21:44:03.473 in Ack 17
21:44:03.711 fetch POST /api/observability/v1/traces
21:44:03.711 fetched POST /api/observability/v1/traces 204
21:44:03.722 chunk 17 39178 bytes
21:44:03.723 in Ack 17
21:44:04.162 chunk 17 39198 bytes
21:44:04.163 in Ack 17
21:44:04.411 chunk 17 39208 bytes
21:44:04.412 in Ack 17
21:44:04.666 chunk 17 39220 bytes
21:44:04.667 in Ack 17
21:44:04.724 fetch POST /api/observability/v1/traces
21:44:04.724 fetched POST /api/observability/v1/traces 204
21:44:04.918 chunk 17 39230 bytes
21:44:04.919 in Ack 17
21:44:05.531 in Ping 
21:44:05.566 chunk 17 39274 bytes
21:44:05.567 in Ack 17
21:44:05.920 fetch POST /api/observability/v1/traces
21:44:05.920 fetched POST /api/observability/v1/traces 204
21:44:10.531 in Ping 
21:44:10.774 in Interrupt 30
21:44:11.775 fetch POST /api/observability/v1/traces
21:44:11.775 fetched POST /api/observability/v1/traces 204
21:44:13.723 fetch POST /api/observability/v1/traces
21:44:13.723 fetched POST /api/observability/v1/traces 204
21:44:15.531 in Ping 
21:44:20.532 in Ping 
21:44:20.771 rpc 31 server.reportClientActivity
21:44:21.771 fetch POST /api/observability/v1/traces
21:44:21.771 fetched POST /api/observability/v1/traces 204
21:44:22.612 chunk 17 39498 bytes
21:44:22.614 in Ack 17
21:44:22.867 chunk 17 39506 bytes
21:44:22.868 in Ack 17
21:44:23.117 chunk 17 39572 bytes
21:44:23.118 in Ack 17
21:44:23.450 chunk 17 39824 bytes
21:44:23.451 in Ack 17
21:44:23.616 fetch POST /api/observability/v1/traces
21:44:23.616 fetched POST /api/observability/v1/traces 204
21:44:24.198 chunk 17 39852 bytes
21:44:24.200 in Ack 17
21:44:25.201 fetch POST /api/observability/v1/traces
21:44:25.201 fetched POST /api/observability/v1/traces 204
21:44:25.535 in Ping 
21:44:25.582 chunk 17 40040 bytes
21:44:25.584 in Ack 17
21:44:26.585 fetch POST /api/observability/v1/traces
21:44:26.585 fetched POST /api/observability/v1/traces 204
21:44:29.319 chunk 17 40158 bytes
21:44:29.320 in Ack 17
21:44:30.321 fetch POST /api/observability/v1/traces
21:44:30.321 fetched POST /api/observability/v1/traces 204
21:44:30.539 in Ping 
21:44:31.620 chunk 17 40166 bytes
21:44:31.621 in Ack 17
21:44:32.334 chunk 17 40296 bytes
21:44:32.335 in Ack 17
21:44:32.586 chunk 17 40314 bytes
21:44:32.587 in Ack 17
21:44:32.657 fetch POST /api/observability/v1/traces
21:44:32.657 fetched POST /api/observability/v1/traces 204
21:44:33.735 fetch POST /api/observability/v1/traces
21:44:33.735 fetched POST /api/observability/v1/traces 204
21:44:35.544 in Ping 
21:44:35.771 in Interrupt 31
21:44:35.993 chunk 17 40354 bytes
21:44:35.995 in Ack 17
21:44:36.783 fetch POST /api/observability/v1/traces
21:44:36.783 fetched POST /api/observability/v1/traces 204
21:44:39.038 chunk 17 41362 bytes
21:44:39.040 in Ack 17
21:44:39.655 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:44:40.041 fetch POST /api/observability/v1/traces
21:44:40.041 fetched POST /api/observability/v1/traces 204
21:44:40.547 in Ping 
21:44:43.739 fetch POST /api/observability/v1/traces
21:44:43.739 fetched POST /api/observability/v1/traces 204
21:44:44.120 chunk 17 43873 bytes
21:44:44.121 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:44:44.125 in Ack 17
21:44:44.557 chunk 17 43903 bytes
21:44:44.558 in Ack 17
21:44:44.696 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:44:44.998 chunk 17 43925 bytes
21:44:45.003 in Ack 17
21:44:45.127 fetch POST /api/observability/v1/traces
21:44:45.127 fetched POST /api/observability/v1/traces 204
21:44:45.508 chunk 17 43965 bytes
21:44:45.509 in Ack 17
21:44:45.551 in Ping 
21:44:45.772 rpc 34 server.reportClientActivity
21:44:46.024 chunk 17 43985 bytes
21:44:46.026 in Ack 17
21:44:46.452 chunk 17 43995 bytes
21:44:46.453 in Ack 17
21:44:46.510 fetch POST /api/observability/v1/traces
21:44:46.510 fetched POST /api/observability/v1/traces 204
21:44:47.138 chunk 17 44663 bytes
21:44:47.139 in Ack 17
21:44:47.559 chunk 17 44683 bytes
21:44:47.561 in Ack 17
21:44:48.072 chunk 17 44713 bytes
21:44:48.074 in Ack 17
21:44:48.140 fetch POST /api/observability/v1/traces
21:44:48.140 fetched POST /api/observability/v1/traces 204
21:44:48.503 chunk 17 44735 bytes
21:44:48.505 in Ack 17
21:44:49.281 chunk 17 44747 bytes
21:44:49.282 in Ack 17
21:44:49.506 fetch POST /api/observability/v1/traces
21:44:49.506 fetched POST /api/observability/v1/traces 204
21:44:49.898 chunk 17 44789 bytes
21:44:49.899 in Ack 17
21:44:50.394 chunk 17 44809 bytes
21:44:50.396 in Ack 17
21:44:50.556 in Ping 
21:44:50.904 fetch POST /api/observability/v1/traces
21:44:50.904 fetched POST /api/observability/v1/traces 204
21:44:50.916 chunk 17 44859 bytes
21:44:50.918 in Ack 17
21:44:51.439 chunk 17 44891 bytes
21:44:51.441 in Ack 17
21:44:51.873 chunk 17 44911 bytes
21:44:51.874 in Ack 17
21:44:51.919 fetch POST /api/observability/v1/traces
21:44:51.919 fetched POST /api/observability/v1/traces 204
21:44:52.311 chunk 17 44939 bytes
21:44:52.312 in Ack 17
21:44:52.745 chunk 17 44971 bytes
21:44:52.746 in Ack 17
21:44:53.173 chunk 17 45003 bytes
21:44:53.174 in Ack 17
21:44:53.314 fetch POST /api/observability/v1/traces
21:44:53.315 fetched POST /api/observability/v1/traces 204
21:44:53.684 chunk 17 45049 bytes
21:44:53.685 in Ack 17
21:44:54.118 chunk 17 45111 bytes
21:44:54.120 in Ack 17
21:44:54.552 chunk 17 45137 bytes
21:44:54.554 in Ack 17
21:44:54.686 fetch POST /api/observability/v1/traces
21:44:54.686 fetched POST /api/observability/v1/traces 204
21:44:54.990 chunk 17 45157 bytes
21:44:54.992 in Ack 17
21:44:55.421 chunk 17 45217 bytes
21:44:55.423 in Ack 17
21:44:55.560 in Ping 
21:44:55.993 fetch POST /api/observability/v1/traces
21:44:55.993 fetched POST /api/observability/v1/traces 204
21:44:56.018 chunk 17 45307 bytes
21:44:56.019 in Ack 17
21:44:56.583 chunk 17 45395 bytes
21:44:56.585 in Ack 17
21:44:57.020 fetch POST /api/observability/v1/traces
21:44:57.020 fetched POST /api/observability/v1/traces 204
21:44:57.836 chunk 17 45533 bytes
21:44:57.837 in Ack 17
21:44:58.838 fetch POST /api/observability/v1/traces
21:44:58.838 fetched POST /api/observability/v1/traces 204
21:44:59.391 exit 30 {"_tag":"Success","value":null}
21:44:59.828 exit 31 {"_tag":"Success","value":null}
21:45:00.564 in Ping 
21:45:00.773 in Interrupt 34
21:45:00.955 chunk 17 45607 bytes
21:45:00.956 in Ack 17
21:45:01.775 fetch POST /api/observability/v1/traces
21:45:01.775 fetched POST /api/observability/v1/traces 204
21:45:02.249 chunk 17 45715 bytes
21:45:02.250 in Ack 17
21:45:03.119 chunk 17 45755 bytes
21:45:03.120 in Ack 17
21:45:03.251 fetch POST /api/observability/v1/traces
21:45:03.251 fetched POST /api/observability/v1/traces 204
21:45:05.567 in Ping 
21:45:08.612 chunk 17 45925 bytes
21:45:08.613 in Ack 17
21:45:09.614 fetch POST /api/observability/v1/traces
21:45:09.614 fetched POST /api/observability/v1/traces 204
21:45:10.573 in Ping 
21:45:10.772 rpc 35 server.reportClientActivity
21:45:10.924 chunk 17 45935 bytes
21:45:10.924 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:45:10.925 in Ack 17
