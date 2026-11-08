# outage10, webapp, warm start, seed 1

Profile outage10 (fault): one 10 s outage on a weak link. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.6s |
| Prompt reached the agent | 0.5s |
| Time to first text | 1.2s |
| Whole reply shown | 43.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 12.0s / 0.7s / 9.0s |
| Longest silence | 12.1s |
| Approval shown after it was asked | 0.7s |
| Bytes on the wire down / up | 507604 / 382653 |
| Bytes the app received / sent | 297625 / 46002 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.0 | 336 | 381 |
| 20 | 11.0 | 336 | 783 |
| 30 | 0.9 | 1101 | 1140 |
| 40 | 1.2 | 1469 | 1512 |
| 43 | 1.5 | 1560 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","messages":"456/456 546/546 901/901 546/546","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","nodes":"1019/1019 809/809 809/809 1019/1019","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2695/2695","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":250,"lastDataAt":1794167964689,"silences":0,"longestSilenceMs":1098,"longestRecoveredGapMs":1098},"stats":{"sendRate":3429.985900238229,"deliveredRate":3206.357747638108,"loss":0,"srttMs":218.73651748182004,"minRttMs":1,"sentPackets":113,"sentWireBytes":33013,"receivedPackets":125,"receivedWireBytes":60327,"lastHeardMs":10131,"unsyncedBytes":509,"inflightPackets":6,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","messages":"456/456 546/546 901/901 546/546 451/451","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2695/2695","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":250,"lastDataAt":1794167973777,"silences":0,"longestSilenceMs":1098,"longestRecoveredGapMs":1098},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.007349999999999995,"srttMs":336.37930803377094,"minRttMs":336,"sentPackets":165,"sentWireBytes":41151,"receivedPackets":285,"receivedWireBytes":108172,"lastHeardMs":19218,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","messages":"456/456 546/546 901/901 546/546 451/451","assistant":"32/32 21/21 625/953","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2695/2695","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":250,"lastDataAt":1794167984437,"silences":1,"longestSilenceMs":10660,"longestRecoveredGapMs":10660},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.007349999999999995,"srttMs":336.37930803377094,"minRttMs":null,"sentPackets":179,"sentWireBytes":44828,"receivedPackets":286,"receivedWireBytes":108456,"lastHeardMs":29878,"unsyncedBytes":666,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","thread":"951/951","messages":"456/456 546/546 901/901 546/546 451/451 546/546","assistant":"32/32 21/21 1340/1340 128/128","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 985/985 1395/1395 1019/1019","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2695/2695","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":250,"lastDataAt":1794167995457,"silences":1,"longestSilenceMs":10660,"longestRecoveredGapMs":10660},"stats":{"sendRate":4800.738157699614,"deliveredRate":3200.4921051330757,"loss":0,"srttMs":362.1875472996854,"minRttMs":286,"sentPackets":252,"sentWireBytes":60138,"receivedPackets":649,"receivedWireBytes":204327,"lastHeardMs":40898,"unsyncedBytes":0,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","thread":"951/951","messages":"456/456 546/546 901/901 546/546 451/451 546/546 545/545","assistant":"32/32 21/21 1340/1340 684/684","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 985/985 1395/1395 1019/1019","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2695/2695","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":250,"lastDataAt":1794168006493,"silences":1,"longestSilenceMs":10660,"longestRecoveredGapMs":10660},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.375,"srttMs":420.0155672444401,"minRttMs":286,"sentPackets":321,"sentWireBytes":73237,"receivedPackets":1241,"receivedWireBytes":328628,"lastHeardMs":51934,"unsyncedBytes":145,"inflightPackets":26,"duplicateBytes":0}}
- shim trace
19:59:14.907 fetch GET /api/auth/session
19:59:14.908 fetched GET /api/auth/session 200
19:59:15.047 fetch GET /.well-known/t3/environment
19:59:15.047 fetched GET /.well-known/t3/environment 200
19:59:15.063 fetch GET /.well-known/t3/environment
19:59:15.064 fetched GET /.well-known/t3/environment 200
19:59:15.068 rpc 0 subscribeServerConfig
19:59:15.068 chunk 0 38718 bytes
19:59:15.088 in Ack 0
19:59:15.090 rpc 1 subscribeServerLifecycle
19:59:15.090 rpc 2 subscribeServerLifecycle
19:59:15.093 fetch GET /api/auth/session
19:59:15.095 fetched GET /api/auth/session 200
19:59:15.095 rpc 3 subscribeDeviceState
19:59:15.097 rpc 4 preview.reportProfiles
19:59:15.129 rpc 5 subscribeProjectClones
19:59:15.146 fetch GET /api/orchestration/threads/mcp%3Ad9021353-fc2a-45a2-b2c2-56911296693a/bounded
19:59:15.151 fetch GET /api/orchestration/shell
19:59:15.152 fetched GET /api/orchestration/shell 200
19:59:15.154 rpc 6 orchestration.subscribeShell
19:59:15.154 chunk 6 2789 bytes
19:59:15.176 rpc 7 server.refreshProviders
19:59:15.215 rpc 8 orchestration.dispatchCommand thread.visit
19:59:15.215 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:59:15.228 rpc 9 subscribeTerminalMetadata
19:59:15.229 rpc 10 subscribeVcsStatus
19:59:15.229 rpc 11 subscribePreviewEvents
19:59:15.230 rpc 12 assets.createUrl
19:59:15.231 rpc 13 preview.list
19:59:15.232 rpc 14 projects.readFile
19:59:15.236 in Ack 6
19:59:15.483 rpc 15 server.reportClientActivity
19:59:15.490 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"958de12b-96d7-4b8c-8321-15db6a900ecb","sessions":[]}}
19:59:15.490 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:59:15.735 chunk 1 3871 bytes
19:59:15.752 in Ack 1
19:59:15.984 chunk 2 3871 bytes
19:59:15.985 in Ack 2
19:59:16.045 fetch POST /api/observability/v1/traces
19:59:16.045 fetched POST /api/observability/v1/traces 204
19:59:16.141 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:59:03.175Z","configurableProviders":false,"con
19:59:16.141 chunk 10 2246 bytes
19:59:16.144 in Ack 10
19:59:16.339 chunk 6 2765 bytes
19:59:16.339 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:59:16.340 in Ack 6
19:59:16.588 chunk 6 2808 bytes
19:59:16.589 in Ack 6
19:59:16.618 chunk 3 876 bytes
19:59:16.618 exit 4 {"_tag":"Success","value":null}
19:59:16.618 chunk 5 4 bytes
19:59:16.618 chunk 9 36 bytes
19:59:16.618 exit 15 {"_tag":"Success","value":null}
19:59:16.618 in Ack 3
19:59:16.622 in Ack 5
19:59:16.622 in Ack 9
19:59:17.122 fetched GET /api/orchestration/threads/mcp%3Ad9021353-fc2a-45a2-b2c2-56911296693a/bounded 200
19:59:17.125 rpc 16 orchestration.subscribeThread
19:59:17.125 chunk 16 8283 bytes
19:59:17.134 in Interrupt 10
19:59:17.135 rpc 17 subscribeVcsStatus
19:59:17.136 in Ack 16
19:59:17.191 fetch POST /api/observability/v1/traces
19:59:17.191 fetched POST /api/observability/v1/traces 204
19:59:17.289 chunk 17 2246 bytes
19:59:17.290 in Ack 17
19:59:17.375 chunk 16 9337 bytes
19:59:17.376 in Ack 16
19:59:17.400 rpc 18 server.reportClientActivity
19:59:17.628 chunk 16 12694 bytes
19:59:17.631 in Ack 16
19:59:17.878 chunk 16 13502 bytes
19:59:17.879 in Ack 16
19:59:17.973 rpc 19 attachments.createUploadUrl
19:59:18.140 chunk 16 16197 bytes
19:59:18.149 in Ack 16
19:59:18.228 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-373cced4-1a90-4cba-93ba-ab5e25fa3098","expiresAt":1794168558173,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:59:18.229 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTM3M2NjZWQ0LTFhOTAtNGNiYS05M2JhLWFiNWUyNWZhMzA5OCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODU1ODE3M30.p_FWvrtZDQ0w0U2TWOElJzRwqjYnLbfut5aAWTcgImY
19:59:18.377 fetch POST /api/observability/v1/traces
19:59:18.377 fetched POST /api/observability/v1/traces 204
19:59:18.378 exit 18 {"_tag":"Success","value":null}
19:59:18.391 chunk 16 19071 bytes
19:59:18.392 in Ack 16
19:59:18.688 chunk 16 21197 bytes
19:59:18.690 in Ack 16
19:59:18.697 in Interrupt 17
19:59:18.699 rpc 20 subscribeVcsStatus
19:59:18.748 chunk 20 2246 bytes
19:59:18.748 in Ack 20
19:59:18.951 rpc 21 server.reportClientActivity
19:59:19.379 fetch POST /api/observability/v1/traces
19:59:19.379 fetched POST /api/observability/v1/traces 204
19:59:20.076 in Ping 
19:59:20.187 exit 21 {"_tag":"Success","value":null}
19:59:20.874 rpc 22 orchestration.dispatchCommand message.dispatch
19:59:21.417 fetch POST /api/observability/v1/traces
19:59:21.417 fetched POST /api/observability/v1/traces 204
19:59:21.480 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:59:21.481 rpc 23 attachments.delete
19:59:21.729 chunk 16 23631 bytes
19:59:21.730 in Ack 16
19:59:21.967 exit 23 {"_tag":"Success","value":null}
19:59:21.979 chunk 6 2894 bytes
19:59:21.980 chunk 16 23603 bytes
19:59:21.980 in Ack 6
19:59:22.024 in Ack 16
19:59:22.231 chunk 16 24252 bytes
19:59:22.232 in Ack 16
19:59:22.482 fetch POST /api/observability/v1/traces
19:59:22.482 fetched POST /api/observability/v1/traces 204
19:59:22.482 chunk 16 24302 bytes
19:59:22.483 in Ack 16
19:59:22.733 chunk 6 2830 bytes
19:59:22.733 chunk 16 28067 bytes
19:59:22.733 in Ack 6
19:59:22.735 rpc 24 orchestration.dispatchCommand thread.visit
19:59:22.735 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:59:22.740 in Ack 16
19:59:22.742 rpc 25 assets.createUrl
19:59:22.986 chunk 6 2830 bytes
19:59:22.986 chunk 16 28091 bytes
19:59:22.986 in Ack 6
19:59:22.991 in Ack 16
19:59:23.237 chunk 16 30514 bytes
19:59:23.238 in Ack 16
19:59:23.725 fetch POST /api/observability/v1/traces
19:59:23.725 fetched POST /api/observability/v1/traces 204
19:59:23.726 chunk 16 31938 bytes
19:59:23.728 in Ack 16
19:59:23.741 in Interrupt 20
19:59:23.744 rpc 26 subscribeVcsStatus
19:59:23.996 rpc 27 server.reportClientActivity
19:59:24.025 chunk 26 2246 bytes
19:59:24.026 in Ack 26
19:59:24.229 chunk 16 33281 bytes
19:59:24.231 in Ack 16
19:59:24.481 chunk 16 35212 bytes
19:59:24.483 in Ack 16
19:59:24.657 rpc 28 orchestration.dispatchCommand message.dispatch
19:59:24.690 exit 27 {"_tag":"Success","value":null}
19:59:24.729 fetch POST /api/observability/v1/traces
19:59:24.729 fetched POST /api/observability/v1/traces 204
19:59:24.733 chunk 16 35212 bytes
19:59:24.734 in Ack 16
19:59:25.029 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:59:25.077 in Ping 
19:59:25.484 chunk 16 37646 bytes
19:59:25.485 in Ack 16
19:59:25.736 fetch POST /api/observability/v1/traces
19:59:25.736 fetched POST /api/observability/v1/traces 204
19:59:25.737 chunk 16 37676 bytes
19:59:25.738 in Ack 16
19:59:25.987 chunk 16 37648 bytes
19:59:25.988 in Ack 16
19:59:26.259 chunk 16 37678 bytes
19:59:26.262 in Ack 16
19:59:26.509 chunk 16 37708 bytes
19:59:26.510 in Ack 16
19:59:26.739 fetch POST /api/observability/v1/traces
19:59:26.739 fetched POST /api/observability/v1/traces 204
19:59:26.758 chunk 6 2894 bytes
19:59:26.759 chunk 16 37728 bytes
19:59:26.759 in Ack 6
19:59:26.787 in Ack 16
19:59:27.009 chunk 16 37758 bytes
19:59:27.010 in Ack 16
19:59:27.260 chunk 16 37778 bytes
19:59:27.261 in Ack 16
19:59:27.510 chunk 16 38435 bytes
19:59:27.511 in Ack 16
19:59:27.760 fetch POST /api/observability/v1/traces
19:59:27.760 fetched POST /api/observability/v1/traces 204
19:59:27.761 chunk 16 38465 bytes
19:59:27.761 in Ack 16
19:59:28.012 chunk 16 41328 bytes
19:59:28.013 in Ack 16
19:59:28.263 chunk 16 41348 bytes
19:59:28.264 in Ack 16
19:59:28.513 chunk 16 42045 bytes
19:59:28.514 in Ack 16
19:59:28.762 fetch POST /api/observability/v1/traces
19:59:28.762 fetched POST /api/observability/v1/traces 204
19:59:28.763 chunk 16 42075 bytes
19:59:28.764 in Ack 16
19:59:29.013 chunk 16 42095 bytes
19:59:29.014 in Ack 16
19:59:29.264 chunk 16 42115 bytes
19:59:29.265 in Ack 16
19:59:29.515 chunk 16 42921 bytes
19:59:29.516 in Ack 16
19:59:29.764 fetch POST /api/observability/v1/traces
19:59:29.764 fetched POST /api/observability/v1/traces 204
19:59:29.765 chunk 16 43469 bytes
19:59:29.767 in Ack 16
19:59:30.016 chunk 16 43499 bytes
19:59:30.017 in Ack 16
19:59:30.077 in Ping 
19:59:30.266 chunk 16 43519 bytes
19:59:30.267 in Ack 16
19:59:30.516 chunk 16 43539 bytes
19:59:30.517 in Ack 16
19:59:30.766 chunk 16 43569 bytes
19:59:30.767 in Ack 16
19:59:30.775 fetch POST /api/observability/v1/traces
19:59:30.775 fetched POST /api/observability/v1/traces 204
19:59:30.907 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/ec656d95-d6c3-420d-9259-381cb85180ef","expiresAt":1794171570907}}
19:59:31.024 chunk 16 43599 bytes
19:59:31.026 in Ack 16
19:59:31.275 chunk 16 43629 bytes
19:59:31.276 in Ack 16
19:59:31.525 chunk 16 43649 bytes
19:59:31.527 in Ack 16
19:59:31.776 chunk 16 43669 bytes
19:59:31.777 in Ack 16
19:59:31.908 fetch POST /api/observability/v1/traces
19:59:31.909 fetched POST /api/observability/v1/traces 204
19:59:32.025 chunk 16 43699 bytes
19:59:32.026 in Ack 16
19:59:32.276 chunk 16 43719 bytes
19:59:32.277 in Ack 16
19:59:32.527 chunk 16 43749 bytes
19:59:32.528 in Ack 16
19:59:32.735 rpc 29 orchestration.dispatchCommand thread.visit
19:59:32.735 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:59:32.777 chunk 16 43769 bytes
19:59:32.778 in Ack 16
19:59:33.029 chunk 16 43799 bytes
19:59:33.029 fetch POST /api/observability/v1/traces
19:59:33.029 fetched POST /api/observability/v1/traces 204
19:59:33.031 in Ack 16
19:59:33.279 chunk 6 2894 bytes
19:59:33.279 chunk 16 43829 bytes
19:59:33.279 in Ack 6
19:59:33.285 in Ack 16
19:59:33.530 chunk 16 43849 bytes
19:59:33.531 in Ack 16
19:59:33.780 chunk 16 43869 bytes
19:59:33.781 in Ack 16
19:59:34.032 fetch POST /api/observability/v1/traces
19:59:34.032 fetched POST /api/observability/v1/traces 204
19:59:35.077 in Ping 
19:59:36.663 fetch POST /api/observability/v1/traces
19:59:36.663 fetched POST /api/observability/v1/traces 204
19:59:38.688 fetch POST /api/observability/v1/traces
19:59:38.688 fetched POST /api/observability/v1/traces 204
19:59:40.077 in Ping 
19:59:40.300 rpc 30 server.reportClientActivity
19:59:41.301 fetch POST /api/observability/v1/traces
19:59:41.301 fetched POST /api/observability/v1/traces 204
19:59:42.735 rpc 31 orchestration.dispatchCommand thread.visit
19:59:42.735 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:59:43.736 fetch POST /api/observability/v1/traces
19:59:43.736 fetched POST /api/observability/v1/traces 204
19:59:44.438 chunk 16 44281 bytes
19:59:44.441 in Ack 16
19:59:44.892 chunk 16 44429 bytes
19:59:44.893 in Ack 16
19:59:45.078 in Ping 
19:59:45.142 chunk 16 44449 bytes
19:59:45.143 in Ack 16
19:59:45.393 chunk 16 44735 bytes
19:59:45.394 in Ack 16
19:59:45.442 fetch POST /api/observability/v1/traces
19:59:45.443 fetched POST /api/observability/v1/traces 204
19:59:45.643 chunk 16 44989 bytes
19:59:45.644 in Ack 16
19:59:45.893 chunk 16 45069 bytes
19:59:45.894 in Ack 16
19:59:46.214 chunk 16 45109 bytes
19:59:46.215 in Ack 16
19:59:46.465 chunk 16 45129 bytes
19:59:46.466 in Ack 16
19:59:46.646 fetch POST /api/observability/v1/traces
19:59:46.646 fetched POST /api/observability/v1/traces 204
19:59:46.715 chunk 6 2894 bytes
19:59:46.716 chunk 16 45159 bytes
19:59:46.716 in Ack 6
19:59:46.721 in Ack 16
19:59:46.905 exit 30 {"_tag":"Success","value":null}
19:59:46.966 chunk 16 45189 bytes
19:59:46.967 in Ack 16
19:59:47.216 chunk 16 46845 bytes
19:59:47.217 in Ack 16
19:59:47.466 chunk 16 46875 bytes
19:59:47.468 in Ack 16
19:59:47.717 fetch POST /api/observability/v1/traces
19:59:47.717 fetched POST /api/observability/v1/traces 204
19:59:47.717 chunk 16 46935 bytes
19:59:47.718 in Ack 16
19:59:47.968 chunk 16 46965 bytes
19:59:47.969 in Ack 16
19:59:48.218 chunk 16 47010 bytes
19:59:48.219 in Ack 16
19:59:48.468 chunk 16 47040 bytes
19:59:48.469 in Ack 16
19:59:48.718 chunk 16 47070 bytes
19:59:48.719 in Ack 16
19:59:48.730 fetch POST /api/observability/v1/traces
19:59:48.730 fetched POST /api/observability/v1/traces 204
19:59:48.968 chunk 16 47100 bytes
19:59:48.972 in Ack 16
19:59:49.219 chunk 16 47145 bytes
19:59:49.220 in Ack 16
19:59:49.469 chunk 16 47205 bytes
19:59:49.471 in Ack 16
19:59:49.720 chunk 16 47235 bytes
19:59:49.721 in Ack 16
19:59:49.970 chunk 16 47250 bytes
19:59:49.971 in Ack 16
19:59:49.983 fetch POST /api/observability/v1/traces
19:59:49.983 fetched POST /api/observability/v1/traces 204
19:59:50.078 in Ping 
19:59:50.220 chunk 16 47310 bytes
19:59:50.221 in Ack 16
19:59:50.470 chunk 16 47355 bytes
19:59:50.471 in Ack 16
19:59:50.720 chunk 16 47385 bytes
19:59:50.721 in Ack 16
19:59:50.970 chunk 16 47415 bytes
19:59:50.971 in Ack 16
19:59:51.220 chunk 16 47460 bytes
19:59:51.221 in Ack 16
19:59:51.232 fetch POST /api/observability/v1/traces
19:59:51.232 fetched POST /api/observability/v1/traces 204
19:59:51.470 chunk 16 47490 bytes
19:59:51.471 in Ack 16
19:59:51.720 chunk 16 47535 bytes
19:59:51.721 in Ack 16
19:59:51.970 chunk 16 47565 bytes
19:59:51.971 in Ack 16
19:59:52.219 chunk 16 47607 bytes
19:59:52.220 in Ack 16
19:59:52.470 chunk 16 50687 bytes
19:59:52.471 in Ack 16
19:59:52.540 fetch POST /api/observability/v1/traces
19:59:52.540 fetched POST /api/observability/v1/traces 204
19:59:52.720 chunk 6 3202 bytes
19:59:52.720 chunk 16 50737 bytes
19:59:52.720 in Ack 6
19:59:52.728 in Ack 16
19:59:52.738 rpc 32 orchestration.dispatchCommand thread.visit
19:59:52.738 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:59:52.833 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:59:52.970 chunk 16 57589 bytes
19:59:52.971 in Ack 16
19:59:53.296 exit 33 {"_tag":"Success","value":{"sequence":98}}
19:59:53.471 chunk 16 60113 bytes
19:59:53.473 in Ack 16
19:59:53.722 fetch POST /api/observability/v1/traces
19:59:53.722 fetched POST /api/observability/v1/traces 204
19:59:53.723 chunk 16 60095 bytes
19:59:53.724 in Ack 16
19:59:53.972 chunk 6 2896 bytes
19:59:53.972 in Ack 6
19:59:53.977 chunk 16 60211 bytes
19:59:53.978 in Ack 16
19:59:53.986 in Interrupt 26
19:59:54.009 rpc 34 subscribeVcsStatus
19:59:54.228 chunk 16 60289 bytes
19:59:54.230 in Ack 16
19:59:54.263 rpc 35 server.reportClientActivity
19:59:54.479 chunk 16 60309 bytes
19:59:54.480 in Ack 16
19:59:54.546 chunk 34 2246 bytes
19:59:54.546 in Ack 34
19:59:54.725 fetch POST /api/observability/v1/traces
19:59:54.725 fetched POST /api/observability/v1/traces 204
19:59:54.729 chunk 16 60349 bytes
19:59:54.730 in Ack 16
19:59:54.979 chunk 16 60359 bytes
19:59:54.980 in Ack 16
19:59:55.004 exit 35 {"_tag":"Success","value":null}
19:59:55.078 in Ping 
19:59:55.229 chunk 16 60389 bytes
19:59:55.230 in Ack 16
19:59:55.478 chunk 16 60409 bytes
19:59:55.480 in Ack 16
19:59:55.729 chunk 16 60439 bytes
19:59:55.730 in Ack 16
19:59:55.748 fetch POST /api/observability/v1/traces
19:59:55.748 fetched POST /api/observability/v1/traces 204
19:59:55.979 chunk 16 60469 bytes
19:59:55.980 in Ack 16
19:59:56.229 chunk 16 60489 bytes
19:59:56.230 in Ack 16
19:59:56.479 chunk 16 60509 bytes
19:59:56.480 in Ack 16
19:59:56.729 chunk 16 60539 bytes
19:59:56.730 in Ack 16
19:59:56.978 chunk 16 60559 bytes
19:59:56.979 in Ack 16
19:59:56.989 fetch POST /api/observability/v1/traces
19:59:56.989 fetched POST /api/observability/v1/traces 204
19:59:57.228 chunk 16 60589 bytes
19:59:57.229 in Ack 16
19:59:57.479 chunk 16 60609 bytes
19:59:57.480 in Ack 16
19:59:57.737 chunk 16 60639 bytes
19:59:57.738 in Ack 16
19:59:57.987 chunk 16 60669 bytes
19:59:57.988 in Ack 16
19:59:58.230 fetch POST /api/observability/v1/traces
19:59:58.230 fetched POST /api/observability/v1/traces 204
19:59:58.238 chunk 16 60699 bytes
19:59:58.239 in Ack 16
19:59:58.488 chunk 16 60719 bytes
19:59:58.489 in Ack 16
19:59:58.738 chunk 16 60749 bytes
19:59:58.740 in Ack 16
19:59:58.989 chunk 16 60769 bytes
19:59:58.990 in Ack 16
19:59:59.239 chunk 16 60799 bytes
19:59:59.240 fetch POST /api/observability/v1/traces
19:59:59.240 fetched POST /api/observability/v1/traces 204
19:59:59.241 in Ack 16
19:59:59.490 chunk 16 60819 bytes
19:59:59.491 in Ack 16
19:59:59.740 chunk 16 60849 bytes
19:59:59.741 in Ack 16
19:59:59.990 chunk 16 60869 bytes
19:59:59.991 in Ack 16
20:00:00.079 in Ping 
20:00:00.240 chunk 16 60899 bytes
20:00:00.241 in Ack 16
20:00:00.250 fetch POST /api/observability/v1/traces
20:00:00.250 fetched POST /api/observability/v1/traces 204
20:00:00.490 chunk 16 60929 bytes
20:00:00.491 in Ack 16
20:00:00.740 chunk 16 60949 bytes
20:00:00.741 in Ack 16
20:00:00.990 chunk 16 60971 bytes
20:00:00.991 in Ack 16
20:00:01.228 chunk 6 2896 bytes
20:00:01.228 in Ack 6
20:00:01.240 chunk 16 60987 bytes
20:00:01.241 in Ack 16
20:00:01.491 chunk 16 61007 bytes
20:00:01.491 fetch POST /api/observability/v1/traces
20:00:01.491 fetched POST /api/observability/v1/traces 204
20:00:01.493 in Ack 16
20:00:01.741 chunk 16 62025 bytes
20:00:01.742 in Ack 16
20:00:01.991 chunk 16 62070 bytes
20:00:01.992 in Ack 16
20:00:02.242 chunk 16 62100 bytes
20:00:02.244 in Ack 16
20:00:02.493 chunk 16 62166 bytes
20:00:02.493 fetch POST /api/observability/v1/traces
20:00:02.493 fetched POST /api/observability/v1/traces 204
20:00:02.495 in Ack 16
20:00:02.739 rpc 36 orchestration.dispatchCommand thread.visit
20:00:02.739 exit 36 {"_tag":"Success","value":{"sequence":0}}
20:00:02.743 chunk 16 62196 bytes
20:00:02.745 in Ack 16
20:00:02.993 chunk 16 62247 bytes
20:00:02.994 in Ack 16
20:00:03.242 chunk 16 62277 bytes
20:00:03.246 in Ack 16
20:00:03.492 chunk 6 2896 bytes
20:00:03.493 chunk 16 62307 bytes
20:00:03.493 in Ack 6
20:00:03.499 in Ack 16
20:00:03.512 fetch POST /api/observability/v1/traces
20:00:03.512 fetched POST /api/observability/v1/traces 204
20:00:03.743 chunk 16 62322 bytes
20:00:03.745 in Ack 16
20:00:03.994 chunk 16 62358 bytes
20:00:03.995 in Ack 16
20:00:04.244 chunk 16 62418 bytes
20:00:04.245 in Ack 16
20:00:04.494 chunk 16 62463 bytes
20:00:04.495 in Ack 16
20:00:04.744 chunk 16 62517 bytes
20:00:04.745 in Ack 16
20:00:04.835 fetch POST /api/observability/v1/traces
20:00:04.835 fetched POST /api/observability/v1/traces 204
20:00:04.994 chunk 16 62547 bytes
20:00:04.996 in Ack 16
20:00:05.082 in Ping 
20:00:05.266 chunk 16 62601 bytes
20:00:05.272 in Ack 16
20:00:05.373 rpc 37 server.reportClientActivity
20:00:05.500 chunk 16 62634 bytes
20:00:05.501 in Ack 16
20:00:05.750 chunk 16 62688 bytes
20:00:05.752 in Ack 16
20:00:05.997 fetch POST /api/observability/v1/traces
20:00:05.997 fetched POST /api/observability/v1/traces 204
20:00:06.001 chunk 16 62718 bytes
20:00:06.002 in Ack 16
20:00:06.252 chunk 16 62769 bytes
20:00:06.254 in Ack 16
20:00:06.503 chunk 16 62820 bytes
20:00:06.504 in Ack 16
20:00:06.753 chunk 16 62853 bytes
20:00:06.754 in Ack 16
20:00:06.892 exit 37 {"_tag":"Success","value":null}
20:00:07.003 chunk 16 62886 bytes
20:00:07.003 fetch POST /api/observability/v1/traces
20:00:07.004 fetched POST /api/observability/v1/traces 204
20:00:07.005 in Ack 16
20:00:07.253 chunk 16 62916 bytes
20:00:07.254 in Ack 16
20:00:07.503 chunk 16 63000 bytes
20:00:07.505 in Ack 16
20:00:07.789 chunk 16 63132 bytes
20:00:07.792 in Ack 16
