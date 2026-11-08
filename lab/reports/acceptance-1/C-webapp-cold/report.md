# C, webapp, cold start, seed 1

Profile C (graded): historical EDGE-like. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 57 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 2.1s |
| Prompt confirmation | 7.5s |
| Prompt reached the agent | 6.9s |
| Time to first text | 9.7s |
| Whole reply shown | 53.6s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.1s / 0.8s / 1.8s |
| Longest silence | 9.7s |
| Approval shown after it was asked | 3.2s |
| Bytes on the wire down / up | 580117 / 318215 |
| Bytes the app received / sent | 384075 / 68823 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.9 | 85 | 126 |
| 20 | 0.8 | 489 | 526 |
| 30 | 0.8 | 893 | 929 |
| 40 | 1.2 | 1109 | 1158 |
| 50 | 2.1 | 1452 | 1528 |
| 53 | 1.3 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"assistant":"21/21 32/32","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860","providerSessions":"2688/2688","checkpoints":"613/613 779/779 779/779","runs":"1014/1014 971/971","messages":"456/456 550/550 60/901","frame":"1032/1032","nodes":"809/809 1025/1025 809/809","checkpointScopes":"726/726","providerThreads":"810/810","thread":"951/951","user":"9/9 10/10","attempts":"662/662 662/662","providerTurns":"778/778"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2353,"inflightPackets":0,"duplicateBytes":0}}
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"assistant":"21/21 32/32 140/140","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860 1083/1083 153/849","providerSessions":"2688/2688","checkpoints":"439/613 613/613 779/779","runs":"1014/1014 971/971","messages":"456/456 550/550 76/901","frame":"1032/1032","nodes":"809/809 1025/1025 809/809","checkpointScopes":"726/726","providerThreads":"810/810","thread":"951/951","user":"9/9 10/10","attempts":"662/662 662/662","providerTurns":"778/778"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":7775,"lastDataAt":1794150564374,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":4002.9371112646854,"deliveredRate":2668.6247408431236,"loss":0,"srttMs":1021.1942090690136,"minRttMs":925,"sentPackets":30,"sentWireBytes":12006,"receivedPackets":19,"receivedWireBytes":19098,"lastHeardMs":13794,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 653/653","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860 1083/1083 849/849","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","runs":"1014/1014 971/971 839/839","messages":"456/456 550/550 96/901 549/549 451/451","frame":"1032/1032","nodes":"809/809 1025/1025 809/809 785/785","checkpointScopes":"726/726","providerThreads":"812/812","thread":"951/951","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 660/660 778/778"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":7775,"lastDataAt":1794150574668,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":3000,"deliveredRate":1494.8176537310198,"loss":0,"srttMs":976.81077004532,"minRttMs":855,"sentPackets":85,"sentWireBytes":20731,"receivedPackets":111,"receivedWireBytes":50859,"lastHeardMs":24088,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1153/1153","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860 1083/1083 849/849","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","runs":"1014/1014 971/971 839/839","messages":"456/456 550/550 901/901 549/549 451/451 550/550","frame":"1032/1032","nodes":"809/809 1025/1025 809/809 785/785 1025/1025","checkpointScopes":"726/726","providerThreads":"812/812","thread":"951/951","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 660/660 778/778"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":7775,"lastDataAt":1794150584799,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":3379.3238859943076,"deliveredRate":1603.7974961627747,"loss":0,"srttMs":990.4228489219233,"minRttMs":855,"sentPackets":149,"sentWireBytes":32830,"receivedPackets":382,"receivedWireBytes":120712,"lastHeardMs":34218,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1340/1340 153/153","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860 1108/1108 849/849 1578/1578 956/956 1083/1083","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","runs":"1014/1014 971/971 839/839","messages":"456/456 550/550 901/901 550/550 451/451 550/550","frame":"1032/1032","nodes":"809/809 1025/1025 809/809 785/785 1025/1025 1145/1401 989/989 997/1025","checkpointScopes":"726/726","providerThreads":"812/812","thread":"951/951","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 660/660 778/778","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":7775,"lastDataAt":1794150595296,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":3492.3910777569313,"deliveredRate":1904.9405878674172,"loss":0.18181818181818177,"srttMs":1085.0613073546326,"minRttMs":855,"sentPackets":216,"sentWireBytes":48785,"receivedPackets":606,"receivedWireBytes":228660,"lastHeardMs":44715,"unsyncedBytes":492,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1340/1340 654/654","turnItems":"1299/1299 1058/1058 1108/1108 1108/1108 1058/1058 870/870 860/860 1108/1108 849/849 1578/1578 956/956 1083/1083","providerSessions":"2688/2688","checkpoints":"613/613 613/613 779/779","runs":"1014/1014 971/971 839/839","messages":"456/456 550/550 901/901 550/550 451/451 550/550 549/549","frame":"1032/1032","nodes":"809/809 1025/1025 809/809 785/785 1025/1025 1401/1401 989/989 1025/1025","checkpointScopes":"726/726","providerThreads":"812/812","thread":"951/951","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 660/660 778/778","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":7775,"lastDataAt":1794150605414,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":4500,"deliveredRate":2118.512350818762,"loss":0.06896551724137934,"srttMs":1089.6365520767954,"minRttMs":855,"sentPackets":287,"sentWireBytes":65015,"receivedPackets":891,"receivedWireBytes":360127,"lastHeardMs":54833,"unsyncedBytes":0,"inflightPackets":6,"duplicateBytes":0}}
- shim trace
15:09:11.544 fetch GET /api/auth/session
15:09:11.549 fetched GET /api/auth/session 200
15:09:11.727 fetch GET /.well-known/t3/environment
15:09:11.728 fetched GET /.well-known/t3/environment 200
15:09:11.754 fetch GET /.well-known/t3/environment
15:09:11.755 fetched GET /.well-known/t3/environment 200
15:09:11.764 rpc 0 subscribeServerConfig
15:09:11.764 chunk 0 38690 bytes
15:09:11.789 in Ack 0
15:09:11.793 rpc 1 subscribeServerLifecycle
15:09:11.794 rpc 2 subscribeServerLifecycle
15:09:11.799 fetch GET /api/auth/session
15:09:11.800 fetched GET /api/auth/session 200
15:09:11.801 rpc 3 subscribeDeviceState
15:09:11.804 rpc 4 preview.reportProfiles
15:09:11.933 rpc 5 subscribeProjectClones
15:09:12.002 rpc 6 server.reportClientActivity
15:09:12.038 rpc 7 orchestration.subscribeThread
15:09:12.040 chunk 7 32178 bytes
15:09:12.054 fetch GET /api/orchestration/shell
15:09:12.150 rpc 8 server.refreshProviders
15:09:12.269 rpc 9 orchestration.dispatchCommand thread.visit
15:09:12.364 fetched GET /api/orchestration/shell 200
15:09:12.364 rpc 10 subscribeTerminalMetadata
15:09:12.365 rpc 11 subscribePreviewEvents
15:09:12.366 rpc 12 subscribeVcsStatus
15:09:12.371 rpc 13 orchestration.subscribeShell
15:09:12.372 chunk 13 2847 bytes
15:09:12.375 rpc 14 assets.createUrl
15:09:12.375 rpc 15 preview.list
15:09:12.376 rpc 16 projects.readFile
15:09:12.435 in Ack 7
15:09:12.451 in Interrupt 12
15:09:12.454 rpc 17 assets.createUrl
15:09:12.501 rpc 18 subscribeVcsStatus
15:09:12.504 in Ack 13
15:09:12.721 fetch POST /api/observability/v1/traces
15:09:12.722 fetched POST /api/observability/v1/traces 204
15:09:13.982 fetch POST /api/observability/v1/traces
15:09:13.982 fetched POST /api/observability/v1/traces 204
15:09:16.761 in Ping 
15:09:20.062 chunk 7 31988 bytes
15:09:20.062 chunk 13 2823 bytes
15:09:20.066 in Ack 7
15:09:20.083 in Ack 13
15:09:20.140 exit 9 {"_tag":"Success","value":{"sequence":70}}
15:09:20.142 rpc 19 orchestration.dispatchCommand message.dispatch
15:09:20.312 chunk 7 31374 bytes
15:09:20.313 in Ack 7
15:09:20.333 chunk 1 3864 bytes
15:09:20.334 in Ack 1
15:09:21.098 fetch POST /api/observability/v1/traces
15:09:21.098 fetched POST /api/observability/v1/traces 204
15:09:21.251 exit 15 {"_tag":"Success","value":{"revision":0,"serverEpoch":"6c3bb658-e7c8-4935-943b-37fcd17f697f","sessions":[]}}
15:09:21.251 exit 16 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:09:21.407 exit 19 {"_tag":"Success","value":{"sequence":77}}
15:09:21.764 in Ping 
15:09:22.253 fetch POST /api/observability/v1/traces
15:09:22.253 fetched POST /api/observability/v1/traces 204
15:09:22.788 chunk 7 33852 bytes
15:09:22.795 in Ack 7
15:09:22.988 chunk 7 33872 bytes
15:09:22.989 in Ack 7
15:09:23.106 chunk 2 3864 bytes
15:09:23.106 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:08:47.849Z","configurableProviders":false,"con
15:09:23.107 in Ack 2
15:09:23.238 chunk 7 33882 bytes
15:09:23.239 in Ack 7
15:09:23.487 chunk 7 33922 bytes
15:09:23.491 in Ack 7
15:09:23.738 chunk 7 34020 bytes
15:09:23.739 in Ack 7
15:09:23.751 chunk 12 1427 bytes
15:09:23.752 exit 14 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:09:23.804 fetch POST /api/observability/v1/traces
15:09:23.804 fetched POST /api/observability/v1/traces 204
15:09:23.992 chunk 7 34030 bytes
15:09:23.994 in Ack 7
15:09:24.071 chunk 18 1427 bytes
15:09:24.072 in Ack 18
15:09:24.244 chunk 7 34022 bytes
15:09:24.245 in Ack 7
15:09:24.494 chunk 7 34052 bytes
15:09:24.496 in Ack 7
15:09:24.744 chunk 7 34082 bytes
15:09:24.745 in Ack 7
15:09:24.994 fetch POST /api/observability/v1/traces
15:09:24.994 fetched POST /api/observability/v1/traces 204
15:09:24.996 chunk 7 34102 bytes
15:09:24.997 in Ack 7
15:09:25.246 chunk 7 34142 bytes
15:09:25.247 in Ack 7
15:09:25.486 chunk 13 944 bytes
15:09:25.487 in Ack 13
15:09:25.558 chunk 7 34172 bytes
15:09:25.563 in Ack 7
15:09:25.812 chunk 7 34192 bytes
15:09:25.814 in Ack 7
15:09:25.998 fetch POST /api/observability/v1/traces
15:09:25.998 fetched POST /api/observability/v1/traces 204
15:09:26.064 chunk 7 34222 bytes
15:09:26.065 in Ack 7
15:09:26.315 chunk 7 34242 bytes
15:09:26.316 in Ack 7
15:09:26.565 chunk 7 34262 bytes
15:09:26.566 in Ack 7
15:09:26.765 in Ping 
15:09:26.885 chunk 7 34292 bytes
15:09:26.887 in Ack 7
15:09:27.003 in Interrupt 6
15:09:27.069 fetch POST /api/observability/v1/traces
15:09:27.069 fetched POST /api/observability/v1/traces 204
15:09:27.137 chunk 7 34332 bytes
15:09:27.138 in Ack 7
15:09:27.256 rpc 20 server.reportClientActivity
15:09:27.415 chunk 7 34362 bytes
15:09:27.416 in Ack 7
15:09:27.508 chunk 13 2887 bytes
15:09:27.509 in Ack 13
15:09:27.511 rpc 21 orchestration.dispatchCommand thread.visit
15:09:27.670 chunk 7 34372 bytes
15:09:27.671 in Ack 7
15:09:27.920 chunk 7 34392 bytes
15:09:27.921 in Ack 7
15:09:28.139 fetch POST /api/observability/v1/traces
15:09:28.139 fetched POST /api/observability/v1/traces 204
15:09:28.171 chunk 7 35071 bytes
15:09:28.172 in Ack 7
15:09:28.421 chunk 7 35091 bytes
15:09:28.422 in Ack 7
15:09:28.672 chunk 7 35101 bytes
15:09:28.673 in Ack 7
15:09:28.805 exit 21 {"_tag":"Success","value":{"sequence":85}}
15:09:28.922 chunk 7 35981 bytes
15:09:28.923 in Ack 7
15:09:29.037 chunk 13 944 bytes
15:09:29.037 in Ack 13
15:09:29.180 fetch POST /api/observability/v1/traces
15:09:29.180 fetched POST /api/observability/v1/traces 204
15:09:29.180 chunk 7 36001 bytes
15:09:29.183 in Ack 7
15:09:29.565 chunk 7 36023 bytes
15:09:29.566 in Ack 7
15:09:29.816 chunk 7 36043 bytes
15:09:29.818 in Ack 7
15:09:30.069 chunk 7 36063 bytes
15:09:30.070 in Ack 7
15:09:30.185 fetch POST /api/observability/v1/traces
15:09:30.185 fetched POST /api/observability/v1/traces 204
15:09:30.325 chunk 13 2887 bytes
15:09:30.326 in Ack 13
15:09:30.826 chunk 7 38184 bytes
15:09:30.829 in Ack 7
15:09:31.076 chunk 7 38855 bytes
15:09:31.077 in Ack 7
15:09:31.326 chunk 7 38885 bytes
15:09:31.327 in Ack 7
15:09:31.339 fetch POST /api/observability/v1/traces
15:09:31.339 fetched POST /api/observability/v1/traces 204
15:09:31.577 chunk 7 38891 bytes
15:09:31.577 chunk 13 944 bytes
15:09:31.578 in Ack 7
15:09:31.590 in Ack 13
15:09:31.765 in Ping 
15:09:31.828 chunk 7 38911 bytes
15:09:31.830 in Ack 7
15:09:32.099 chunk 7 38941 bytes
15:09:32.100 in Ack 7
15:09:32.350 chunk 7 38971 bytes
15:09:32.351 in Ack 7
15:09:32.608 fetch POST /api/observability/v1/traces
15:09:32.608 fetched POST /api/observability/v1/traces 204
15:09:32.610 chunk 7 40091 bytes
15:09:32.611 in Ack 7
15:09:32.621 chunk 13 2887 bytes
15:09:32.622 in Ack 13
15:09:32.860 chunk 7 40207 bytes
15:09:32.861 in Ack 7
15:09:33.111 chunk 7 40783 bytes
15:09:33.112 in Ack 7
15:09:33.367 chunk 7 40798 bytes
15:09:33.372 in Ack 7
15:09:33.613 fetch POST /api/observability/v1/traces
15:09:33.613 fetched POST /api/observability/v1/traces 204
15:09:33.617 chunk 7 40843 bytes
15:09:33.618 in Ack 7
15:09:33.867 chunk 7 42213 bytes
15:09:33.868 in Ack 7
15:09:34.117 chunk 7 42258 bytes
15:09:34.118 in Ack 7
15:09:34.376 chunk 7 42303 bytes
15:09:34.377 in Ack 7
15:09:34.619 fetch POST /api/observability/v1/traces
15:09:34.619 fetched POST /api/observability/v1/traces 204
15:09:34.627 chunk 7 42333 bytes
15:09:34.628 in Ack 7
15:09:34.876 chunk 7 43157 bytes
15:09:34.877 in Ack 7
15:09:35.126 chunk 7 43774 bytes
15:09:35.129 in Ack 7
15:09:35.380 chunk 7 43804 bytes
15:09:35.383 in Ack 7
15:09:35.628 chunk 7 43834 bytes
15:09:35.630 in Ack 7
15:09:35.680 fetch POST /api/observability/v1/traces
15:09:35.680 fetched POST /api/observability/v1/traces 204
15:09:35.877 chunk 7 44905 bytes
15:09:35.881 in Ack 7
15:09:36.081 chunk 3 876 bytes
15:09:36.081 exit 4 {"_tag":"Success","value":null}
15:09:36.081 chunk 5 4 bytes
15:09:36.081 exit 6 {"_tag":"Success","value":null}
15:09:36.081 in Ack 3
15:09:36.089 rpc 22 preview.reportProfiles
15:09:36.089 in Ack 5
15:09:36.127 chunk 7 45846 bytes
15:09:36.128 in Ack 7
15:09:36.151 chunk 10 36 bytes
15:09:36.151 exit 20 {"_tag":"Success","value":null}
15:09:36.151 in Ack 10
15:09:36.376 chunk 7 45891 bytes
15:09:36.379 in Ack 7
15:09:36.629 chunk 7 45921 bytes
15:09:36.630 in Ack 7
15:09:36.765 in Ping 
15:09:36.882 chunk 7 45951 bytes
15:09:36.883 fetch POST /api/observability/v1/traces
15:09:36.883 fetched POST /api/observability/v1/traces 204
15:09:36.885 in Ack 7
15:09:36.980 rpc 23 server.reportClientActivity
15:09:37.134 chunk 7 45996 bytes
15:09:37.135 in Ack 7
15:09:37.194 exit 22 {"_tag":"Success","value":null}
15:09:37.387 chunk 7 46041 bytes
15:09:37.388 in Ack 7
15:09:37.511 rpc 24 orchestration.dispatchCommand thread.visit
15:09:37.637 chunk 7 46086 bytes
15:09:37.638 in Ack 7
15:09:37.886 fetch POST /api/observability/v1/traces
15:09:37.886 fetched POST /api/observability/v1/traces 204
15:09:37.887 chunk 7 46116 bytes
15:09:37.888 in Ack 7
15:09:38.002 exit 17 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/ab8b3d1e-400b-4b23-91d2-25c0e4f02b35","expiresAt":1794154178002}}
15:09:38.137 chunk 7 46146 bytes
15:09:38.144 in Ack 7
15:09:38.388 chunk 7 46176 bytes
15:09:38.389 in Ack 7
15:09:38.407 exit 23 {"_tag":"Success","value":null}
15:09:38.742 exit 24 {"_tag":"Success","value":{"sequence":88}}
15:09:38.793 chunk 13 944 bytes
15:09:38.793 in Ack 13
15:09:38.889 fetch POST /api/observability/v1/traces
15:09:38.889 fetched POST /api/observability/v1/traces 204
15:09:38.993 chunk 7 46281 bytes
15:09:38.993 in Ack 7
15:09:39.047 chunk 13 2887 bytes
15:09:39.049 in Ack 13
15:09:39.243 chunk 7 46311 bytes
15:09:39.244 in Ack 7
15:09:39.492 chunk 7 46356 bytes
15:09:39.493 in Ack 7
15:09:39.748 chunk 7 46386 bytes
15:09:39.749 in Ack 7
15:09:39.996 fetch POST /api/observability/v1/traces
15:09:39.996 fetched POST /api/observability/v1/traces 204
15:09:40.001 chunk 7 46431 bytes
15:09:40.004 in Ack 7
15:09:40.249 chunk 7 46461 bytes
15:09:40.250 in Ack 7
15:09:40.499 chunk 7 46506 bytes
15:09:40.501 in Ack 7
15:09:40.750 chunk 7 46536 bytes
15:09:40.751 in Ack 7
15:09:41.000 chunk 7 46581 bytes
15:09:41.001 in Ack 7
15:09:41.015 fetch POST /api/observability/v1/traces
15:09:41.015 fetched POST /api/observability/v1/traces 204
15:09:41.252 chunk 7 46611 bytes
15:09:41.253 in Ack 7
15:09:41.502 chunk 7 46656 bytes
15:09:41.503 in Ack 7
15:09:41.793 chunk 7 46686 bytes
15:09:41.793 in Ping 
15:09:41.801 in Ack 7
15:09:42.044 chunk 7 46731 bytes
15:09:42.045 in Ack 7
15:09:42.254 fetch POST /api/observability/v1/traces
15:09:42.254 fetched POST /api/observability/v1/traces 204
15:09:42.295 chunk 7 46791 bytes
15:09:42.296 in Ack 7
15:09:42.545 chunk 7 46806 bytes
15:09:42.547 in Ack 7
15:09:42.796 chunk 7 46851 bytes
15:09:42.797 in Ack 7
15:09:43.057 chunk 7 46881 bytes
15:09:43.059 in Ack 7
15:09:43.303 fetch POST /api/observability/v1/traces
15:09:43.303 fetched POST /api/observability/v1/traces 204
15:09:43.308 chunk 7 46926 bytes
15:09:43.309 in Ack 7
15:09:43.558 chunk 7 46971 bytes
15:09:43.559 in Ack 7
15:09:43.808 chunk 7 47001 bytes
15:09:43.809 in Ack 7
15:09:44.058 chunk 7 47046 bytes
15:09:44.060 in Ack 7
15:09:44.308 chunk 7 47061 bytes
15:09:44.309 in Ack 7
15:09:44.321 fetch POST /api/observability/v1/traces
15:09:44.321 fetched POST /api/observability/v1/traces 204
15:09:44.574 chunk 7 47121 bytes
15:09:44.575 in Ack 7
15:09:44.825 chunk 7 47151 bytes
15:09:44.826 in Ack 7
15:09:45.075 chunk 7 47181 bytes
15:09:45.076 in Ack 7
15:09:45.327 chunk 7 47226 bytes
15:09:45.328 in Ack 7
15:09:45.576 fetch POST /api/observability/v1/traces
15:09:45.576 fetched POST /api/observability/v1/traces 204
15:09:45.577 chunk 7 47256 bytes
15:09:45.578 in Ack 7
15:09:45.828 chunk 7 47286 bytes
15:09:45.829 in Ack 7
15:09:46.078 chunk 7 47331 bytes
15:09:46.079 in Ack 7
15:09:46.328 chunk 7 47346 bytes
15:09:46.329 in Ack 7
15:09:46.578 chunk 7 47406 bytes
15:09:46.579 in Ack 7
15:09:46.648 fetch POST /api/observability/v1/traces
15:09:46.648 fetched POST /api/observability/v1/traces 204
15:09:46.797 in Ping 
15:09:46.828 chunk 7 47466 bytes
15:09:46.830 in Ack 7
15:09:47.080 chunk 7 47496 bytes
15:09:47.081 in Ack 7
15:09:47.340 chunk 7 47526 bytes
15:09:47.342 in Ack 7
15:09:47.591 chunk 7 47571 bytes
15:09:47.591 in Ack 7
15:09:47.831 fetch POST /api/observability/v1/traces
15:09:47.831 fetched POST /api/observability/v1/traces 204
15:09:47.843 chunk 7 47586 bytes
15:09:47.845 in Ack 7
15:09:48.095 chunk 7 47646 bytes
15:09:48.096 in Ack 7
15:09:48.346 chunk 7 47661 bytes
15:09:48.347 in Ack 7
15:09:48.595 chunk 7 47703 bytes
15:09:48.596 in Ack 7
15:09:48.851 fetch POST /api/observability/v1/traces
15:09:48.852 fetched POST /api/observability/v1/traces 204
15:09:49.685 chunk 13 944 bytes
15:09:49.686 in Ack 13
15:09:50.104 chunk 7 47751 bytes
15:09:50.105 in Ack 7
15:09:50.359 chunk 7 47753 bytes
15:09:50.360 in Ack 7
15:09:50.687 fetch POST /api/observability/v1/traces
15:09:50.687 fetched POST /api/observability/v1/traces 204
15:09:50.863 chunk 7 48764 bytes
15:09:50.864 in Ack 7
15:09:51.113 chunk 7 49730 bytes
15:09:51.114 in Ack 7
15:09:51.354 rpc 25 orchestration.dispatchCommand runtime-request.respond
15:09:51.399 chunk 7 55334 bytes
15:09:51.401 in Ack 7
15:09:51.500 chunk 13 3197 bytes
15:09:51.500 in Ack 13
15:09:51.799 in Ping 
15:09:51.877 fetch POST /api/observability/v1/traces
15:09:51.877 fetched POST /api/observability/v1/traces 204
15:09:52.619 exit 25 {"_tag":"Success","value":{"sequence":99}}
15:09:52.620 rpc 26 orchestration.dispatchCommand thread.visit
15:09:52.651 chunk 7 59206 bytes
15:09:52.652 in Ack 7
15:09:52.901 chunk 7 59178 bytes
15:09:52.901 chunk 13 944 bytes
15:09:52.902 in Ack 7
15:09:52.923 in Ack 13
15:09:53.151 chunk 7 58197 bytes
15:09:53.151 chunk 13 2889 bytes
15:09:53.152 in Ack 7
15:09:53.198 in Ack 13
15:09:53.401 chunk 7 55756 bytes
15:09:53.402 in Ack 7
15:09:53.620 fetch POST /api/observability/v1/traces
15:09:53.620 fetched POST /api/observability/v1/traces 204
15:09:53.652 chunk 7 52298 bytes
15:09:53.653 in Ack 7
15:09:53.893 exit 26 {"_tag":"Success","value":{"sequence":102}}
15:09:53.950 chunk 13 2889 bytes
15:09:53.951 in Ack 13
15:09:54.154 chunk 7 54496 bytes
15:09:54.155 in Ack 7
15:09:54.163 in Interrupt 18
15:09:54.189 rpc 27 subscribeVcsStatus
15:09:54.406 chunk 7 54526 bytes
15:09:54.407 in Ack 7
15:09:54.505 rpc 28 server.reportClientActivity
15:09:54.654 fetch POST /api/observability/v1/traces
15:09:54.654 fetched POST /api/observability/v1/traces 204
15:09:54.656 chunk 7 54556 bytes
15:09:54.657 in Ack 7
15:09:54.906 chunk 7 54576 bytes
15:09:54.909 in Ack 7
15:09:55.156 chunk 7 54586 bytes
15:09:55.157 in Ack 7
15:09:55.354 chunk 27 1427 bytes
15:09:55.354 in Ack 27
15:09:55.406 chunk 7 58213 bytes
15:09:55.407 in Ack 7
15:09:55.656 chunk 7 58233 bytes
15:09:55.658 in Ack 7
15:09:55.670 fetch POST /api/observability/v1/traces
15:09:55.670 fetched POST /api/observability/v1/traces 204
15:09:55.925 chunk 7 58263 bytes
15:09:55.926 in Ack 7
15:09:56.179 chunk 7 58283 bytes
15:09:56.180 in Ack 7
15:09:56.430 chunk 7 58313 bytes
15:09:56.431 in Ack 7
15:09:56.524 exit 28 {"_tag":"Success","value":null}
15:09:56.680 chunk 7 60771 bytes
15:09:56.682 in Ack 7
15:09:56.799 in Ping 
15:09:56.928 fetch POST /api/observability/v1/traces
15:09:56.928 fetched POST /api/observability/v1/traces 204
15:09:56.930 chunk 7 60801 bytes
15:09:56.936 in Ack 7
15:09:57.182 chunk 7 60821 bytes
15:09:57.183 in Ack 7
15:09:57.436 chunk 7 60851 bytes
15:09:57.437 in Ack 7
15:09:57.686 chunk 7 60871 bytes
15:09:57.687 in Ack 7
15:09:57.937 chunk 7 60901 bytes
15:09:57.937 fetch POST /api/observability/v1/traces
15:09:57.937 fetched POST /api/observability/v1/traces 204
15:09:57.938 in Ack 7
15:09:58.187 chunk 7 60921 bytes
15:09:58.188 in Ack 7
15:09:58.437 chunk 7 60951 bytes
15:09:58.438 in Ack 7
15:09:58.687 chunk 7 60971 bytes
15:09:58.688 in Ack 7
15:09:58.938 chunk 7 61001 bytes
15:09:58.939 in Ack 7
15:09:58.995 fetch POST /api/observability/v1/traces
15:09:58.995 fetched POST /api/observability/v1/traces 204
15:09:59.188 chunk 7 61031 bytes
15:09:59.189 in Ack 7
15:09:59.439 chunk 7 61051 bytes
15:09:59.440 in Ack 7
15:09:59.688 chunk 7 61081 bytes
15:09:59.689 in Ack 7
15:09:59.945 chunk 7 61111 bytes
15:09:59.948 in Ack 7
15:10:00.190 fetch POST /api/observability/v1/traces
15:10:00.190 fetched POST /api/observability/v1/traces 204
15:10:00.194 chunk 7 61121 bytes
15:10:00.196 in Ack 7
15:10:00.444 chunk 7 62102 bytes
15:10:00.445 in Ack 7
15:10:00.556 chunk 13 2889 bytes
15:10:00.556 in Ack 13
15:10:00.694 chunk 7 62126 bytes
15:10:00.696 in Ack 7
15:10:00.972 chunk 7 62186 bytes
15:10:00.973 in Ack 7
15:10:01.197 fetch POST /api/observability/v1/traces
15:10:01.197 fetched POST /api/observability/v1/traces 204
15:10:01.222 chunk 7 62231 bytes
15:10:01.223 in Ack 7
15:10:01.476 chunk 7 62261 bytes
15:10:01.477 in Ack 7
15:10:01.503 rpc 29 orchestration.dispatchCommand thread.visit
15:10:01.723 chunk 7 62291 bytes
15:10:01.724 in Ack 7
15:10:01.810 in Ping 
15:10:01.972 chunk 7 62357 bytes
15:10:01.973 in Ack 7
15:10:02.105 rpc 30 server.reportClientActivity
15:10:02.222 chunk 7 62387 bytes
15:10:02.223 in Ack 7
15:10:02.246 fetch POST /api/observability/v1/traces
15:10:02.246 fetched POST /api/observability/v1/traces 204
15:10:02.477 chunk 7 62438 bytes
15:10:02.478 in Ack 7
15:10:02.632 exit 29 {"_tag":"Success","value":{"sequence":105}}
15:10:02.632 chunk 13 2889 bytes
15:10:02.633 in Ack 13
15:10:03.479 fetch POST /api/observability/v1/traces
15:10:03.479 fetched POST /api/observability/v1/traces 204
15:10:03.965 exit 30 {"_tag":"Success","value":null}
15:10:03.984 chunk 7 62678 bytes
15:10:03.986 in Ack 7
15:10:04.268 chunk 7 62723 bytes
15:10:04.269 in Ack 7
15:10:04.519 chunk 7 62753 bytes
15:10:04.521 in Ack 7
15:10:04.769 chunk 7 62795 bytes
15:10:04.770 in Ack 7
15:10:04.970 fetch POST /api/observability/v1/traces
15:10:04.970 fetched POST /api/observability/v1/traces 204
15:10:05.065 chunk 7 62864 bytes
15:10:05.068 in Ack 7
15:10:05.316 chunk 7 62879 bytes
15:10:05.318 in Ack 7
15:10:05.568 chunk 7 62924 bytes
15:10:05.571 in Ack 7
15:10:05.818 chunk 7 62975 bytes
15:10:05.819 in Ack 7
15:10:06.071 chunk 7 63014 bytes
15:10:06.071 fetch POST /api/observability/v1/traces
15:10:06.071 fetched POST /api/observability/v1/traces 204
15:10:06.073 in Ack 7
15:10:06.321 chunk 7 63062 bytes
15:10:06.324 in Ack 7
15:10:06.572 chunk 7 63092 bytes
15:10:06.574 in Ack 7
15:10:06.776 chunk 13 944 bytes
15:10:06.776 in Ack 13
15:10:06.814 in Ping 
15:10:06.821 chunk 7 62536 bytes
15:10:06.823 in Ack 7
15:10:07.038 chunk 13 2825 bytes
15:10:07.038 in Ack 13
15:10:07.040 rpc 31 orchestration.dispatchCommand thread.visit
15:10:07.071 chunk 7 61696 bytes
15:10:07.072 in Ack 7
15:10:07.109 fetch POST /api/observability/v1/traces
15:10:07.109 fetched POST /api/observability/v1/traces 204
15:10:07.321 chunk 7 62359 bytes
15:10:07.322 in Ack 7
