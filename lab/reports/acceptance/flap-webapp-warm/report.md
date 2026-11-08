# flap, webapp, warm start, seed 1

Profile flap (fault): down 5 s of every 15 s on a weak link. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.6s |
| Prompt reached the agent | 0.5s |
| Time to first text | 1.2s |
| Whole reply shown | 47.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 8.0s / 0.7s / 6.0s |
| Longest silence | 7.3s |
| Approval shown after it was asked | 4.0s |
| Bytes on the wire down / up | 313669 / 166619 |
| Bytes the app received / sent | 161375 / 46943 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.9 | 341 | 381 |
| 20 | 0.5 | 758 | 783 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 1.7 | 1336 | 1402 |
| 47 | 8.0 | 1363 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103","thread":"951/951","runs":"1014/1014 871/971","assistant":"32/32 21/21","user":"10/10","attempts":"661/661 49/661","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2691/2691","nodes":"809/809 1022/1022","messages":"548/548 456/456","checkpoints":"779/779 613/613","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":207,"lastDataAt":1794168155287,"silences":0,"longestSilenceMs":1141,"longestRecoveredGapMs":1141},"stats":{"sendRate":5605.518162734286,"deliveredRate":2857.115229963498,"loss":0,"srttMs":4.637970010899881,"minRttMs":1,"sentPackets":103,"sentWireBytes":29168,"receivedPackets":111,"receivedWireBytes":43964,"lastHeardMs":10828,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 849/849 1078/1078 1058/1058","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 425/425","user":"10/10 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 658/658","providerThreads":"811/811","providerSessions":"2691/2691","nodes":"809/809 1022/1022 785/785 809/809","messages":"548/548 456/456 451/451 23/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":207,"lastDataAt":1794168164473,"silences":0,"longestSilenceMs":1141,"longestRecoveredGapMs":1141},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0,"srttMs":186.00376470865461,"minRttMs":1,"sentPackets":161,"sentWireBytes":40951,"receivedPackets":191,"receivedWireBytes":77030,"lastHeardMs":20014,"unsyncedBytes":495,"inflightPackets":7,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 849/849 1078/1078 1058/1058","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 973/973","user":"10/10 64/64 9/9","attempts":"661/661 661/661 637/637","providerTurns":"776/776 658/658 776/776","providerThreads":"811/811","providerSessions":"2691/2691","nodes":"809/809 1022/1022 785/785 809/809 1022/1022","messages":"548/548 456/456 451/451 901/901 547/547 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":207,"lastDataAt":1794168175461,"silences":1,"longestSilenceMs":6053,"longestRecoveredGapMs":6053},"stats":{"sendRate":2456.6394658753698,"deliveredRate":2047.6190476190475,"loss":0,"srttMs":281.07552738163923,"minRttMs":302,"sentPackets":204,"sentWireBytes":49741,"receivedPackets":248,"receivedWireBytes":96890,"lastHeardMs":31002,"unsyncedBytes":0,"inflightPackets":18,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 849/849 1078/1078 1058/1058","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 64/64 9/9","attempts":"661/661 661/661 637/637","providerTurns":"776/776 658/658 776/776","providerThreads":"811/811","providerSessions":"2691/2691","nodes":"809/809 1022/1022 785/785 809/809 1022/1022","messages":"548/548 456/456 451/451 901/901 547/547 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"566/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":207,"lastDataAt":1794168185618,"silences":2,"longestSilenceMs":6053,"longestRecoveredGapMs":6053},"stats":{"sendRate":3216.7429094236045,"deliveredRate":1656.8026219363048,"loss":0.025000000000000012,"srttMs":346.09942557012164,"minRttMs":302,"sentPackets":236,"sentWireBytes":54920,"receivedPackets":385,"receivedWireBytes":130218,"lastHeardMs":41160,"unsyncedBytes":0,"inflightPackets":19,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 849/849 1103/1103 1058/1058 1574/1574 953/953 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 388/388","user":"10/10 64/64 9/9","attempts":"661/661 661/661 637/637","providerTurns":"776/776 658/658 776/776","providerThreads":"811/811","providerSessions":"2691/2691","nodes":"809/809 1022/1022 785/785 809/809 1022/1022 1398/1398 987/987 1022/1022","messages":"548/548 456/456 451/451 901/901 548/548 548/548","checkpoints":"613/613 613/613 779/779","checkpointScopes":"729/729","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":207,"lastDataAt":1794168194428,"silences":2,"longestSilenceMs":6053,"longestRecoveredGapMs":6053},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.04285714285714287,"srttMs":344.84931983197527,"minRttMs":300,"sentPackets":312,"sentWireBytes":71642,"receivedPackets":617,"receivedWireBytes":198321,"lastHeardMs":49970,"unsyncedBytes":495,"inflightPackets":34,"duplicateBytes":0}}
- shim trace
20:02:24.800 fetch GET /api/auth/session
20:02:24.801 fetched GET /api/auth/session 200
20:02:24.916 fetch GET /.well-known/t3/environment
20:02:24.917 fetched GET /.well-known/t3/environment 200
20:02:24.928 fetch GET /.well-known/t3/environment
20:02:24.928 fetched GET /.well-known/t3/environment 200
20:02:24.932 rpc 0 subscribeServerConfig
20:02:24.932 chunk 0 38702 bytes
20:02:24.964 in Ack 0
20:02:24.966 rpc 1 subscribeServerLifecycle
20:02:24.967 rpc 2 subscribeServerLifecycle
20:02:24.969 fetch GET /api/auth/session
20:02:24.970 fetched GET /api/auth/session 200
20:02:24.971 rpc 3 subscribeDeviceState
20:02:24.973 rpc 4 preview.reportProfiles
20:02:25.042 rpc 5 subscribeProjectClones
20:02:25.060 fetch GET /api/orchestration/threads/mcp%3Ab923363b-e293-42e5-b20e-cadee4d49b39/bounded
20:02:25.065 fetch GET /api/orchestration/shell
20:02:25.085 rpc 6 server.refreshProviders
20:02:25.131 rpc 7 orchestration.dispatchCommand thread.visit
20:02:25.131 exit 7 {"_tag":"Success","value":{"sequence":0}}
20:02:25.145 fetched GET /api/orchestration/shell 200
20:02:25.145 rpc 8 subscribeTerminalMetadata
20:02:25.146 rpc 9 subscribeVcsStatus
20:02:25.147 rpc 10 subscribePreviewEvents
20:02:25.150 rpc 11 orchestration.subscribeShell
20:02:25.150 chunk 11 2785 bytes
20:02:25.151 rpc 12 assets.createUrl
20:02:25.152 rpc 13 preview.list
20:02:25.154 rpc 14 projects.readFile
20:02:25.156 in Ack 11
20:02:25.401 rpc 15 server.reportClientActivity
20:02:25.401 chunk 11 2761 bytes
20:02:25.401 in Ack 11
20:02:25.421 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"7ac6858a-d433-4337-8f14-351e5b2b5301","sessions":[]}}
20:02:25.421 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
20:02:25.682 chunk 1 3867 bytes
20:02:25.683 in Ack 1
20:02:25.890 chunk 2 3867 bytes
20:02:25.890 in Ack 2
20:02:25.915 fetch POST /api/observability/v1/traces
20:02:25.915 fetched POST /api/observability/v1/traces 204
20:02:26.060 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T20:02:10.176Z","configurableProviders":false,"con
20:02:26.060 chunk 9 2416 bytes
20:02:26.060 exit 12 {"_tag":"Success","value":{"expiresAt":1794171600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
20:02:26.064 in Ack 9
20:02:26.511 chunk 11 2804 bytes
20:02:26.511 in Ack 11
20:02:26.558 chunk 3 876 bytes
20:02:26.558 exit 4 {"_tag":"Success","value":null}
20:02:26.558 chunk 5 4 bytes
20:02:26.558 chunk 8 36 bytes
20:02:26.558 exit 15 {"_tag":"Success","value":null}
20:02:26.559 in Ack 3
20:02:26.562 rpc 16 preview.reportProfiles
20:02:26.562 in Ack 5
20:02:26.563 in Ack 8
20:02:26.659 exit 16 {"_tag":"Success","value":null}
20:02:26.819 fetched GET /api/orchestration/threads/mcp%3Ab923363b-e293-42e5-b20e-cadee4d49b39/bounded 200
20:02:26.823 rpc 17 orchestration.subscribeThread
20:02:26.823 chunk 17 8294 bytes
20:02:26.834 in Interrupt 9
20:02:26.837 rpc 18 subscribeVcsStatus
20:02:26.838 in Ack 17
20:02:26.995 chunk 18 2416 bytes
20:02:26.996 in Ack 18
20:02:27.062 fetch POST /api/observability/v1/traces
20:02:27.062 fetched POST /api/observability/v1/traces 204
20:02:27.074 chunk 17 11271 bytes
20:02:27.075 in Ack 17
20:02:27.138 rpc 19 server.reportClientActivity
20:02:27.326 chunk 17 12708 bytes
20:02:27.327 in Ack 17
20:02:27.388 exit 19 {"_tag":"Success","value":null}
20:02:27.609 chunk 17 13517 bytes
20:02:27.611 in Ack 17
20:02:27.882 rpc 20 attachments.createUploadUrl
20:02:27.890 chunk 17 17230 bytes
20:02:28.068 in Ack 17
20:02:28.086 fetch POST /api/observability/v1/traces
20:02:28.086 fetched POST /api/observability/v1/traces 204
20:02:28.140 chunk 17 19866 bytes
20:02:28.141 in Ack 17
20:02:28.149 in Interrupt 18
20:02:28.151 rpc 21 subscribeVcsStatus
20:02:28.158 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-3e4d2e0a-b3fa-4191-980f-fa1bed339219","expiresAt":1794168748119,"relativeUrl":"/api/attachments/upload/eyJ2ZX
20:02:28.159 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTNlNGQyZTBhLWIzZmEtNDE5MS05ODBmLWZhMWJlZDMzOTIxOSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODc0ODExOX0.ikCSrHdQNk4v-hlwDJdFBzB1AiFf7Q8Q46-J44sjqMQ
20:02:28.232 chunk 21 2416 bytes
20:02:28.232 in Ack 21
20:02:28.390 chunk 17 21209 bytes
20:02:28.392 in Ack 17
20:02:28.402 rpc 22 server.reportClientActivity
20:02:29.142 fetch POST /api/observability/v1/traces
20:02:29.142 fetched POST /api/observability/v1/traces 204
20:02:29.931 in Ping 
20:02:30.878 exit 22 {"_tag":"Success","value":null}
20:02:31.487 rpc 23 orchestration.dispatchCommand message.dispatch
20:02:32.109 fetch POST /api/observability/v1/traces
20:02:32.110 fetched POST /api/observability/v1/traces 204
20:02:32.189 exit 23 {"_tag":"Success","value":{"sequence":45}}
20:02:32.191 rpc 24 attachments.delete
20:02:32.689 chunk 17 23654 bytes
20:02:32.691 in Ack 17
20:02:32.939 chunk 17 23638 bytes
20:02:32.941 in Ack 17
20:02:33.189 chunk 17 23688 bytes
20:02:33.190 in Ack 17
20:02:33.225 fetch POST /api/observability/v1/traces
20:02:33.225 fetched POST /api/observability/v1/traces 204
20:02:33.939 chunk 11 2826 bytes
20:02:33.940 in Ack 11
20:02:33.941 rpc 25 orchestration.dispatchCommand thread.visit
20:02:33.941 exit 25 {"_tag":"Success","value":{"sequence":0}}
20:02:34.936 in Ping 
20:02:34.943 fetch POST /api/observability/v1/traces
20:02:34.943 fetched POST /api/observability/v1/traces 204
20:02:35.252 rpc 26 orchestration.dispatchCommand message.dispatch
20:02:35.540 chunk 11 2826 bytes
20:02:35.541 in Ack 11
20:02:35.669 exit 26 {"_tag":"Success","value":{"sequence":75}}
20:02:36.253 fetch POST /api/observability/v1/traces
20:02:36.253 fetched POST /api/observability/v1/traces 204
20:02:36.292 chunk 17 26143 bytes
20:02:36.293 in Ack 17
20:02:36.541 chunk 17 26173 bytes
20:02:36.542 in Ack 17
20:02:36.792 chunk 17 26203 bytes
20:02:36.793 in Ack 17
20:02:37.052 chunk 17 26233 bytes
20:02:37.053 in Ack 17
20:02:37.294 fetch POST /api/observability/v1/traces
20:02:37.294 fetched POST /api/observability/v1/traces 204
20:02:37.419 chunk 17 26273 bytes
20:02:37.420 in Ack 17
20:02:37.670 chunk 17 26293 bytes
20:02:37.671 in Ack 17
20:02:37.920 chunk 17 26265 bytes
20:02:37.921 in Ack 17
20:02:38.170 chunk 17 26285 bytes
20:02:38.171 in Ack 17
20:02:38.343 fetch POST /api/observability/v1/traces
20:02:38.343 fetched POST /api/observability/v1/traces 204
20:02:38.421 chunk 17 26315 bytes
20:02:38.422 in Ack 17
20:02:38.671 chunk 17 26335 bytes
20:02:38.672 in Ack 17
20:02:38.922 chunk 17 26365 bytes
20:02:38.922 in Ack 17
20:02:39.173 chunk 17 26395 bytes
20:02:39.174 in Ack 17
20:02:39.422 chunk 11 2890 bytes
20:02:39.423 in Ack 11
20:02:39.428 fetch POST /api/observability/v1/traces
20:02:39.429 fetched POST /api/observability/v1/traces 204
20:02:39.429 chunk 17 26415 bytes
20:02:39.430 in Ack 17
20:02:39.680 chunk 17 27083 bytes
20:02:39.682 in Ack 17
20:02:39.930 chunk 17 27103 bytes
20:02:39.931 in Ack 17
20:02:39.943 in Ping 
20:02:40.179 chunk 17 29976 bytes
20:02:40.180 in Ack 17
20:02:40.429 chunk 17 30968 bytes
20:02:40.430 in Ack 17
20:02:40.444 fetch POST /api/observability/v1/traces
20:02:40.444 fetched POST /api/observability/v1/traces 204
20:02:40.680 chunk 17 31660 bytes
20:02:40.681 in Ack 17
20:02:40.930 chunk 17 31690 bytes
20:02:40.931 in Ack 17
20:02:41.180 chunk 17 31700 bytes
20:02:41.182 in Ack 17
20:02:41.430 chunk 17 34822 bytes
20:02:41.431 in Ack 17
20:02:41.681 chunk 17 34842 bytes
20:02:41.682 in Ack 17
20:02:41.690 fetch POST /api/observability/v1/traces
20:02:41.690 fetched POST /api/observability/v1/traces 204
20:02:41.725 exit 24 {"_tag":"Success","value":null}
20:02:41.931 chunk 17 35648 bytes
20:02:41.932 in Ack 17
20:02:42.181 chunk 17 35680 bytes
20:02:42.182 in Ack 17
20:02:42.431 chunk 17 36226 bytes
20:02:42.432 in Ack 17
20:02:42.681 chunk 17 36246 bytes
20:02:42.682 in Ack 17
20:02:42.726 fetch POST /api/observability/v1/traces
20:02:42.726 fetched POST /api/observability/v1/traces 204
20:02:42.931 chunk 17 36266 bytes
20:02:42.932 in Ack 17
20:02:43.181 chunk 17 36296 bytes
20:02:43.182 in Ack 17
20:02:43.431 chunk 17 37136 bytes
20:02:43.432 in Ack 17
20:02:43.682 chunk 17 37156 bytes
20:02:43.682 in Ack 17
20:02:43.933 chunk 17 37966 bytes
20:02:43.933 fetch POST /api/observability/v1/traces
20:02:43.933 fetched POST /api/observability/v1/traces 204
20:02:43.934 in Ack 17
20:02:43.944 in Interrupt 21
20:02:43.949 rpc 27 subscribeVcsStatus
20:02:43.951 rpc 28 orchestration.dispatchCommand thread.visit
20:02:43.951 exit 28 {"_tag":"Success","value":{"sequence":0}}
20:02:44.184 chunk 17 37986 bytes
20:02:44.185 in Ack 17
20:02:44.200 rpc 29 server.reportClientActivity
20:02:44.436 chunk 17 37840 bytes
20:02:44.438 in Ack 17
20:02:44.476 chunk 27 2416 bytes
20:02:44.476 in Ack 27
20:02:44.684 chunk 17 37850 bytes
20:02:44.685 in Ack 17
20:02:44.935 fetch POST /api/observability/v1/traces
20:02:44.935 fetched POST /api/observability/v1/traces 204
20:02:44.944 in Ping 
20:02:46.658 fetch POST /api/observability/v1/traces
20:02:46.658 fetched POST /api/observability/v1/traces 204
20:02:48.345 fetch POST /api/observability/v1/traces
20:02:48.345 fetched POST /api/observability/v1/traces 204
20:02:49.945 in Ping 
20:02:50.528 chunk 17 38050 bytes
20:02:50.532 in Ack 17
20:02:50.988 chunk 17 38442 bytes
20:02:50.989 in Ack 17
20:02:51.238 chunk 17 38462 bytes
20:02:51.239 in Ack 17
20:02:51.488 chunk 17 38530 bytes
20:02:51.489 in Ack 17
20:02:51.534 fetch POST /api/observability/v1/traces
20:02:51.534 fetched POST /api/observability/v1/traces 204
20:02:51.738 chunk 17 38560 bytes
20:02:51.739 in Ack 17
20:02:51.988 chunk 17 38600 bytes
20:02:51.989 in Ack 17
20:02:52.238 chunk 17 38610 bytes
20:02:52.239 in Ack 17
20:02:52.488 chunk 17 38650 bytes
20:02:52.489 in Ack 17
20:02:52.738 chunk 11 2890 bytes
20:02:52.738 chunk 17 38670 bytes
20:02:52.739 in Ack 11
20:02:52.745 in Ack 17
20:02:52.773 fetch POST /api/observability/v1/traces
20:02:52.773 fetched POST /api/observability/v1/traces 204
20:02:52.988 chunk 17 38700 bytes
20:02:52.989 in Ack 17
20:02:53.238 chunk 17 38730 bytes
20:02:53.239 in Ack 17
20:02:53.489 chunk 17 38740 bytes
20:02:53.490 in Ack 17
20:02:53.739 chunk 17 40203 bytes
20:02:53.740 in Ack 17
20:02:53.953 rpc 30 orchestration.dispatchCommand thread.visit
20:02:53.953 exit 30 {"_tag":"Success","value":{"sequence":0}}
20:02:53.989 chunk 17 41025 bytes
20:02:53.990 in Ack 17
20:02:54.000 fetch POST /api/observability/v1/traces
20:02:54.000 fetched POST /api/observability/v1/traces 204
20:02:54.240 chunk 17 41625 bytes
20:02:54.241 in Ack 17
20:02:54.489 chunk 17 41670 bytes
20:02:54.490 in Ack 17
20:02:54.739 chunk 17 41700 bytes
20:02:54.740 in Ack 17
20:02:54.945 in Ping 
20:02:54.989 chunk 17 41730 bytes
20:02:54.990 in Ack 17
20:02:55.239 chunk 11 2890 bytes
20:02:55.239 chunk 17 41775 bytes
20:02:55.240 in Ack 11
20:02:55.246 in Ack 17
20:02:55.258 fetch POST /api/observability/v1/traces
20:02:55.258 fetched POST /api/observability/v1/traces 204
20:02:55.394 exit 29 {"_tag":"Success","value":null}
20:02:55.489 chunk 17 46547 bytes
20:02:55.491 in Ack 17
20:02:55.496 rpc 31 assets.createUrl
20:02:55.647 rpc 32 server.reportClientActivity
20:02:55.739 chunk 17 46577 bytes
20:02:55.741 in Ack 17
20:02:55.989 chunk 17 46622 bytes
20:02:55.990 in Ack 17
20:02:56.268 exit 32 {"_tag":"Success","value":null}
20:02:56.397 fetch POST /api/observability/v1/traces
20:02:56.397 fetched POST /api/observability/v1/traces 204
20:02:56.518 chunk 17 46697 bytes
20:02:56.519 in Ack 17
20:02:56.768 chunk 17 46727 bytes
20:02:56.770 in Ack 17
20:02:57.029 chunk 17 46772 bytes
20:02:57.032 in Ack 17
20:02:57.169 exit 31 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/a950236d-0d8f-4d3e-91fc-573f790c0b42","expiresAt":1794171777169}}
20:02:57.279 chunk 17 46802 bytes
20:02:57.280 in Ack 17
20:02:57.520 fetch POST /api/observability/v1/traces
20:02:57.520 fetched POST /api/observability/v1/traces 204
20:02:57.529 chunk 17 46847 bytes
20:02:57.530 in Ack 17
20:02:57.779 chunk 17 46892 bytes
20:02:57.781 in Ack 17
20:02:58.030 chunk 17 46922 bytes
20:02:58.032 in Ack 17
20:02:58.280 chunk 17 46967 bytes
20:02:58.282 in Ack 17
20:02:58.530 chunk 17 46997 bytes
20:02:58.531 in Ack 17
20:02:58.540 fetch POST /api/observability/v1/traces
20:02:58.540 fetched POST /api/observability/v1/traces 204
20:02:58.781 chunk 17 47042 bytes
20:02:58.782 in Ack 17
20:02:59.033 chunk 17 47087 bytes
20:02:59.056 in Ack 17
20:02:59.283 chunk 17 47117 bytes
20:02:59.285 in Ack 17
20:02:59.533 chunk 17 47132 bytes
20:02:59.535 in Ack 17
20:02:59.783 fetch POST /api/observability/v1/traces
20:02:59.783 fetched POST /api/observability/v1/traces 204
20:02:59.946 in Ping 
20:03:04.615 chunk 17 47654 bytes
20:03:04.617 in Ack 17
20:03:04.946 in Ping 
20:03:05.618 fetch POST /api/observability/v1/traces
20:03:05.618 fetched POST /api/observability/v1/traces 204
20:03:06.370 chunk 17 48662 bytes
20:03:06.372 in Ack 17
20:03:06.641 rpc 33 orchestration.dispatchCommand runtime-request.respond
20:03:06.929 chunk 17 52167 bytes
20:03:06.931 in Ack 17
20:03:07.119 exit 33 {"_tag":"Success","value":{"sequence":97}}
20:03:07.180 chunk 17 54669 bytes
20:03:07.181 in Ack 17
20:03:07.373 fetch POST /api/observability/v1/traces
20:03:07.373 fetched POST /api/observability/v1/traces 204
20:03:07.430 chunk 17 54679 bytes
20:03:07.432 in Ack 17
20:03:07.681 chunk 17 54752 bytes
20:03:07.682 in Ack 17
20:03:07.939 chunk 17 54782 bytes
20:03:07.940 in Ack 17
20:03:08.190 chunk 17 54802 bytes
20:03:08.191 in Ack 17
20:03:08.434 fetch POST /api/observability/v1/traces
20:03:08.434 fetched POST /api/observability/v1/traces 204
20:03:08.441 chunk 17 54784 bytes
20:03:08.442 in Ack 17
20:03:08.691 chunk 17 54804 bytes
20:03:08.693 in Ack 17
20:03:08.941 chunk 17 54874 bytes
20:03:08.943 in Ack 17
20:03:09.191 chunk 11 2892 bytes
20:03:09.191 chunk 17 54894 bytes
20:03:09.192 in Ack 11
20:03:09.194 rpc 34 orchestration.dispatchCommand thread.visit
20:03:09.194 exit 34 {"_tag":"Success","value":{"sequence":0}}
20:03:09.196 in Ack 17
20:03:09.447 chunk 17 54972 bytes
20:03:09.448 fetch POST /api/observability/v1/traces
20:03:09.448 fetched POST /api/observability/v1/traces 204
20:03:09.449 in Ack 17
20:03:09.698 chunk 17 57133 bytes
20:03:09.699 in Ack 17
20:03:09.711 in Interrupt 27
20:03:09.735 rpc 35 subscribeVcsStatus
20:03:09.946 in Ping 
20:03:09.948 chunk 11 2892 bytes
20:03:09.948 chunk 17 58141 bytes
20:03:09.948 in Ack 11
20:03:09.954 in Ack 17
20:03:10.069 rpc 36 server.reportClientActivity
20:03:10.135 chunk 35 2416 bytes
20:03:10.136 in Ack 35
20:03:10.199 chunk 17 58162 bytes
20:03:10.200 in Ack 17
20:03:10.450 chunk 17 58192 bytes
20:03:10.451 in Ack 17
20:03:10.462 fetch POST /api/observability/v1/traces
20:03:10.462 fetched POST /api/observability/v1/traces 204
20:03:10.704 chunk 17 60644 bytes
20:03:10.705 in Ack 17
20:03:10.736 exit 36 {"_tag":"Success","value":null}
20:03:10.954 chunk 17 60674 bytes
20:03:10.955 in Ack 17
20:03:11.204 chunk 17 60694 bytes
20:03:11.206 in Ack 17
20:03:11.457 chunk 17 60714 bytes
20:03:11.458 in Ack 17
20:03:11.714 chunk 17 60744 bytes
20:03:11.714 fetch POST /api/observability/v1/traces
20:03:11.714 fetched POST /api/observability/v1/traces 204
20:03:11.716 in Ack 17
20:03:11.963 chunk 17 60774 bytes
20:03:11.964 in Ack 17
20:03:12.220 chunk 17 60794 bytes
20:03:12.222 in Ack 17
20:03:12.470 chunk 17 60814 bytes
20:03:12.472 in Ack 17
20:03:12.719 fetch POST /api/observability/v1/traces
20:03:12.719 fetched POST /api/observability/v1/traces 204
20:03:12.720 chunk 17 60844 bytes
20:03:12.722 in Ack 17
20:03:12.970 chunk 17 60864 bytes
20:03:12.971 in Ack 17
20:03:13.220 chunk 17 60894 bytes
20:03:13.221 in Ack 17
20:03:13.470 chunk 17 60914 bytes
20:03:13.471 in Ack 17
20:03:13.721 chunk 17 60944 bytes
20:03:13.722 in Ack 17
20:03:13.730 fetch POST /api/observability/v1/traces
20:03:13.730 fetched POST /api/observability/v1/traces 204
20:03:13.970 chunk 17 60974 bytes
20:03:13.971 in Ack 17
20:03:14.221 chunk 17 60994 bytes
20:03:14.222 in Ack 17
20:03:14.471 chunk 17 61014 bytes
20:03:14.472 in Ack 17
20:03:14.946 in Ping 
20:03:14.973 fetch POST /api/observability/v1/traces
20:03:14.973 fetched POST /api/observability/v1/traces 204
20:03:15.170 rpc 37 server.reportClientActivity
20:03:16.170 fetch POST /api/observability/v1/traces
20:03:16.170 fetched POST /api/observability/v1/traces 204
20:03:19.721 chunk 17 61392 bytes
20:03:19.723 in Ack 17
20:03:19.950 in Ping 
20:03:20.236 fetch POST /api/observability/v1/traces
20:03:20.236 fetched POST /api/observability/v1/traces 204
20:03:21.098 chunk 17 61642 bytes
20:03:21.100 in Ack 17
20:03:21.348 chunk 17 61662 bytes
20:03:21.350 in Ack 17
20:03:21.598 chunk 17 61738 bytes
20:03:21.600 in Ack 17
20:03:22.102 fetch POST /api/observability/v1/traces
20:03:22.102 fetched POST /api/observability/v1/traces 204
20:03:22.531 chunk 17 61788 bytes
20:03:22.533 in Ack 17
