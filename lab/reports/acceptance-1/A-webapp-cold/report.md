# A, webapp, cold start, seed 1

Profile A (graded): weak 4G, still useful. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 2.2s |
| Prompt confirmation | 0.4s |
| Prompt reached the agent | 0.4s |
| Time to first text | 1.1s |
| Whole reply shown | 43.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.9s / 0.3s / 1.4s |
| Longest silence | 2.0s |
| Approval shown after it was asked | 0.3s |
| Bytes on the wire down / up | 4309419 / 2366256 |
| Bytes the app received / sent | 3144725 / 49727 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.1 | 377 | 385 |
| 20 | 0.3 | 770 | 787 |
| 30 | 0.9 | 1116 | 1155 |
| 40 | 0.9 | 1492 | 1524 |
| 43 | 1.4 | 1575 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1108/1108 1108/1108 860/860 1058/1058 1299/1299 870/870","thread":"951/951","frame":"1032/1032","runs":"1014/1014 971/971","providerThreads":"810/810","assistant":"32/32 21/21","checkpointScopes":"726/726","nodes":"809/809 1025/1025 809/809","providerSessions":"2688/2688","messages":"456/456 550/550","providerTurns":"778/778 14/778","user":"9/9 10/10","attempts":"662/662 662/662","checkpoints":"613/613 144/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":1495,"lastDataAt":1794150480315,"silences":0,"longestSilenceMs":255,"longestRecoveredGapMs":255},"stats":{"sendRate":3375,"deliveredRate":0,"loss":0,"srttMs":255.9667365923524,"minRttMs":41,"sentPackets":20,"sentWireBytes":6914,"receivedPackets":21,"receivedWireBytes":12216,"lastHeardMs":4035,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1108/1108 1108/1108 860/860 1058/1058 1299/1299 870/870 849/849 1083/1083","thread":"951/951","frame":"1032/1032","runs":"1014/1014 971/971 839/839","providerThreads":"812/812","assistant":"32/32 21/21 485/485","checkpointScopes":"726/726","nodes":"809/809 1025/1025 809/809 785/785 1025/1025","providerSessions":"2688/2688","messages":"456/456 550/550 451/451 901/901 550/550","providerTurns":"778/778 778/778 660/660","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":1495,"lastDataAt":1794150490355,"silences":0,"longestSilenceMs":255,"longestRecoveredGapMs":255},"stats":{"sendRate":3000,"deliveredRate":2019.8609731876863,"loss":0,"srttMs":133.43839574564424,"minRttMs":41,"sentPackets":84,"sentWireBytes":16838,"receivedPackets":740,"receivedWireBytes":135738,"lastHeardMs":14076,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1108/1108 1108/1108 860/860 1058/1058 1299/1299 870/870 849/849 1083/1083","thread":"951/951","frame":"1032/1032","runs":"1014/1014 971/971 839/839","providerThreads":"812/812","assistant":"32/32 21/21 988/988","checkpointScopes":"726/726","nodes":"809/809 1025/1025 809/809 785/785 1025/1025","providerSessions":"2688/2688","messages":"456/456 550/550 451/451 901/901 550/550 549/549","providerTurns":"778/778 778/778 660/660","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","checkpoints":"613/613 613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":1495,"lastDataAt":1794150500568,"silences":0,"longestSilenceMs":255,"longestRecoveredGapMs":255},"stats":{"sendRate":3000,"deliveredRate":654.950495049505,"loss":0,"srttMs":128.42836625617474,"minRttMs":41,"sentPackets":138,"sentWireBytes":24581,"receivedPackets":3726,"receivedWireBytes":566350,"lastHeardMs":24288,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1108/1108 1108/1108 860/860 1058/1058 1299/1299 870/870 849/849 1108/1108 1578/1578 956/956 1083/1083","thread":"951/951","frame":"1032/1032","runs":"1014/1014 971/971 839/839","providerThreads":"812/812","assistant":"32/32 21/21 1340/1340 153/153","checkpointScopes":"726/726","nodes":"809/809 1025/1025 809/809 785/785 1025/1025 1401/1401 989/989 1025/1025","providerSessions":"2688/2688","messages":"456/456 550/550 451/451 901/901 550/550 550/550","providerTurns":"778/778 778/778 660/660","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":1495,"lastDataAt":1794150510873,"silences":0,"longestSilenceMs":303,"longestRecoveredGapMs":303},"stats":{"sendRate":10125,"deliveredRate":4224.875621890547,"loss":0,"srttMs":127.11095396018055,"minRttMs":41,"sentPackets":206,"sentWireBytes":37797,"receivedPackets":11123,"receivedWireBytes":1754892,"lastHeardMs":34593,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1108/1108 1108/1108 860/860 1058/1058 1299/1299 870/870 849/849 1108/1108 1578/1578 956/956 1083/1083","thread":"951/951","frame":"1032/1032","runs":"1014/1014 971/971 839/839","providerThreads":"812/812","assistant":"32/32 21/21 1340/1340 644/644","checkpointScopes":"726/726","nodes":"809/809 1025/1025 809/809 785/785 1025/1025 1401/1401 989/989 1025/1025","providerSessions":"2688/2688","messages":"456/456 550/550 451/451 901/901 550/550 550/550 549/549","providerTurns":"778/778 778/778 660/660","user":"9/9 10/10 64/64","attempts":"662/662 662/662 638/638","checkpoints":"613/613 613/613 779/779","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":1495,"lastDataAt":1794150520908,"silences":0,"longestSilenceMs":303,"longestRecoveredGapMs":303},"stats":{"sendRate":3000,"deliveredRate":651.5453639082752,"loss":0,"srttMs":131.49859093651307,"minRttMs":41,"sentPackets":259,"sentWireBytes":46123,"receivedPackets":18045,"receivedWireBytes":2841730,"lastHeardMs":44629,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
15:07:57.170 fetch GET /api/auth/session
15:07:57.174 fetched GET /api/auth/session 200
15:07:57.361 fetch GET /.well-known/t3/environment
15:07:57.363 fetched GET /.well-known/t3/environment 200
15:07:57.384 fetch GET /.well-known/t3/environment
15:07:57.385 fetched GET /.well-known/t3/environment 200
15:07:57.393 rpc 0 subscribeServerConfig
15:07:57.393 chunk 0 38690 bytes
15:07:57.419 in Ack 0
15:07:57.423 rpc 1 subscribeServerLifecycle
15:07:57.424 rpc 2 subscribeServerLifecycle
15:07:57.429 fetch GET /api/auth/session
15:07:57.431 fetched GET /api/auth/session 200
15:07:57.432 rpc 3 subscribeDeviceState
15:07:57.436 rpc 4 preview.reportProfiles
15:07:57.588 rpc 5 subscribeProjectClones
15:07:57.660 rpc 6 server.reportClientActivity
15:07:57.670 rpc 7 orchestration.subscribeThread
15:07:57.672 chunk 7 28165 bytes
15:07:57.682 in Ack 7
15:07:57.696 fetch GET /api/orchestration/shell
15:07:57.768 rpc 8 server.refreshProviders
15:07:57.894 rpc 9 orchestration.dispatchCommand thread.visit
15:07:58.049 fetched GET /api/orchestration/shell 200
15:07:58.049 rpc 10 subscribeTerminalMetadata
15:07:58.050 rpc 11 subscribePreviewEvents
15:07:58.051 rpc 12 subscribeVcsStatus
15:07:58.055 rpc 13 orchestration.subscribeShell
15:07:58.055 chunk 13 2847 bytes
15:07:58.058 rpc 14 assets.createUrl
15:07:58.058 rpc 15 preview.list
15:07:58.059 rpc 16 projects.readFile
15:07:58.060 rpc 17 assets.createUrl
15:07:58.151 in Ack 13
15:07:58.575 fetch POST /api/observability/v1/traces
15:07:58.575 fetched POST /api/observability/v1/traces 204
15:07:58.775 exit 9 {"_tag":"Success","value":{"sequence":69}}
15:07:58.776 chunk 1 3864 bytes
15:07:58.780 in Ack 1
15:07:58.935 exit 15 {"_tag":"Success","value":{"revision":0,"serverEpoch":"cfcbe99e-dfa2-4b13-b2b6-00f4c9cf76e2","sessions":[]}}
15:07:58.935 chunk 2 3864 bytes
15:07:58.943 in Ack 2
15:07:59.059 exit 16 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:07:59.059 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:07:30.161Z","configurableProviders":false,"con
15:07:59.059 chunk 12 1363 bytes
15:07:59.064 in Ack 12
15:07:59.205 exit 14 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:07:59.378 chunk 7 28141 bytes
15:07:59.378 chunk 13 2823 bytes
15:07:59.380 in Ack 7
15:07:59.389 in Ack 13
15:07:59.587 fetch POST /api/observability/v1/traces
15:07:59.587 fetched POST /api/observability/v1/traces 204
15:07:59.661 chunk 7 30564 bytes
15:07:59.663 in Ack 7
15:07:59.931 chunk 7 31374 bytes
15:07:59.932 in Ack 7
15:07:59.940 in Interrupt 12
15:07:59.942 rpc 18 subscribeVcsStatus
15:08:00.286 rpc 19 orchestration.dispatchCommand message.dispatch
15:08:00.316 chunk 18 1363 bytes
15:08:00.318 chunk 7 31374 bytes
15:08:00.322 in Ack 18
15:08:00.329 in Ack 7
15:08:00.458 exit 19 {"_tag":"Success","value":{"sequence":76}}
15:08:00.664 fetch POST /api/observability/v1/traces
15:08:00.664 fetched POST /api/observability/v1/traces 204
15:08:00.819 chunk 7 33830 bytes
15:08:00.819 chunk 13 944 bytes
15:08:00.820 in Ack 7
15:08:00.943 in Ack 13
15:08:01.070 chunk 7 33812 bytes
15:08:01.072 in Ack 7
15:08:01.321 chunk 7 33832 bytes
15:08:01.322 chunk 13 2887 bytes
15:08:01.323 in Ack 7
15:08:01.354 in Ack 13
15:08:01.573 chunk 7 37344 bytes
15:08:01.574 in Ack 7
15:08:01.821 fetch POST /api/observability/v1/traces
15:08:01.821 fetched POST /api/observability/v1/traces 204
15:08:01.823 chunk 7 38025 bytes
15:08:01.824 in Ack 7
15:08:02.073 chunk 7 38843 bytes
15:08:02.074 in Ack 7
15:08:02.323 chunk 7 40772 bytes
15:08:02.326 in Ack 7
15:08:02.389 in Ping 
15:08:02.574 chunk 7 41374 bytes
15:08:02.575 in Ack 7
15:08:02.750 chunk 3 876 bytes
15:08:02.750 exit 4 {"_tag":"Success","value":null}
15:08:02.751 in Ack 3
15:08:02.755 rpc 20 preview.reportProfiles
15:08:02.816 chunk 5 4 bytes
15:08:02.816 exit 6 {"_tag":"Success","value":null}
15:08:02.816 chunk 10 36 bytes
15:08:02.817 in Ack 5
15:08:02.819 in Ack 10
15:08:02.833 chunk 7 43341 bytes
15:08:02.835 fetch POST /api/observability/v1/traces
15:08:02.835 fetched POST /api/observability/v1/traces 204
15:08:02.839 in Ack 7
15:08:02.909 exit 20 {"_tag":"Success","value":null}
15:08:03.072 rpc 21 server.reportClientActivity
15:08:03.094 chunk 7 43351 bytes
15:08:03.108 in Ack 7
15:08:03.303 exit 21 {"_tag":"Success","value":null}
15:08:03.345 chunk 7 43381 bytes
15:08:03.346 in Ack 7
15:08:03.595 chunk 7 43411 bytes
15:08:03.596 in Ack 7
15:08:03.766 exit 17 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/3bd91501-c5da-4dbf-9af0-db2e282c66ad","expiresAt":1794154083765}}
15:08:03.841 fetch POST /api/observability/v1/traces
15:08:03.841 fetched POST /api/observability/v1/traces 204
15:08:03.847 chunk 7 43431 bytes
15:08:03.849 in Ack 7
15:08:04.108 chunk 7 43461 bytes
15:08:04.109 in Ack 7
15:08:04.359 chunk 7 43491 bytes
15:08:04.360 in Ack 7
15:08:04.609 chunk 7 43511 bytes
15:08:04.610 in Ack 7
15:08:04.850 fetch POST /api/observability/v1/traces
15:08:04.850 fetched POST /api/observability/v1/traces 204
15:08:04.859 chunk 7 43541 bytes
15:08:04.860 in Ack 7
15:08:05.109 chunk 7 43561 bytes
15:08:05.110 in Ack 7
15:08:05.359 chunk 7 43581 bytes
15:08:05.360 in Ack 7
15:08:05.609 chunk 7 43611 bytes
15:08:05.610 in Ack 7
15:08:05.859 chunk 7 43641 bytes
15:08:05.860 in Ack 7
15:08:05.871 fetch POST /api/observability/v1/traces
15:08:05.871 fetched POST /api/observability/v1/traces 204
15:08:06.109 chunk 7 43661 bytes
15:08:06.110 in Ack 7
15:08:06.359 chunk 7 43691 bytes
15:08:06.360 in Ack 7
15:08:06.610 chunk 7 43711 bytes
15:08:06.612 in Ack 7
15:08:06.861 chunk 7 43741 bytes
15:08:06.862 in Ack 7
15:08:07.111 fetch POST /api/observability/v1/traces
15:08:07.111 fetched POST /api/observability/v1/traces 204
15:08:07.112 chunk 7 43761 bytes
15:08:07.113 in Ack 7
15:08:07.363 chunk 7 43781 bytes
15:08:07.364 in Ack 7
15:08:07.391 in Ping 
15:08:07.613 chunk 7 43811 bytes
15:08:07.614 in Ack 7
15:08:07.863 chunk 7 43841 bytes
15:08:07.865 in Ack 7
15:08:07.895 rpc 22 orchestration.dispatchCommand thread.visit
15:08:08.059 exit 22 {"_tag":"Success","value":{"sequence":84}}
15:08:08.114 fetch POST /api/observability/v1/traces
15:08:08.114 fetched POST /api/observability/v1/traces 204
15:08:08.114 chunk 7 43861 bytes
15:08:08.115 in Ack 7
15:08:08.254 chunk 13 2887 bytes
15:08:08.254 in Ack 13
15:08:08.366 chunk 7 43881 bytes
15:08:08.367 in Ack 7
15:08:08.628 chunk 7 43911 bytes
15:08:08.634 in Ack 7
15:08:08.883 chunk 7 43941 bytes
15:08:08.884 in Ack 7
15:08:09.116 fetch POST /api/observability/v1/traces
15:08:09.116 fetched POST /api/observability/v1/traces 204
15:08:09.133 chunk 7 43961 bytes
15:08:09.134 in Ack 7
15:08:09.383 chunk 7 43991 bytes
15:08:09.384 in Ack 7
15:08:09.634 chunk 7 44011 bytes
15:08:09.635 in Ack 7
15:08:09.884 chunk 7 44041 bytes
15:08:09.885 in Ack 7
15:08:10.150 chunk 7 44061 bytes
15:08:10.150 fetch POST /api/observability/v1/traces
15:08:10.150 fetched POST /api/observability/v1/traces 204
15:08:10.152 in Ack 7
15:08:10.405 chunk 7 44091 bytes
15:08:10.406 in Ack 7
15:08:10.656 chunk 7 44111 bytes
15:08:10.657 in Ack 7
15:08:10.787 chunk 13 2887 bytes
15:08:10.787 in Ack 13
15:08:10.906 chunk 7 45201 bytes
15:08:10.907 in Ack 7
15:08:11.167 fetch POST /api/observability/v1/traces
15:08:11.167 fetched POST /api/observability/v1/traces 204
15:08:11.168 chunk 7 45231 bytes
15:08:11.171 in Ack 7
15:08:11.418 chunk 7 45276 bytes
15:08:11.419 in Ack 7
15:08:11.669 chunk 7 45321 bytes
15:08:11.670 in Ack 7
15:08:11.920 chunk 7 45351 bytes
15:08:11.921 in Ack 7
15:08:12.190 chunk 7 45396 bytes
15:08:12.191 fetch POST /api/observability/v1/traces
15:08:12.191 fetched POST /api/observability/v1/traces 204
15:08:12.194 in Ack 7
15:08:12.392 in Ping 
15:08:12.457 chunk 7 45426 bytes
15:08:12.458 in Ack 7
15:08:12.707 chunk 7 45471 bytes
15:08:12.708 in Ack 7
15:08:12.959 chunk 7 45501 bytes
15:08:12.960 in Ack 7
15:08:13.195 fetch POST /api/observability/v1/traces
15:08:13.196 fetched POST /api/observability/v1/traces 204
15:08:13.209 chunk 7 45546 bytes
15:08:13.210 in Ack 7
15:08:13.460 chunk 7 45591 bytes
15:08:13.461 in Ack 7
15:08:13.710 chunk 7 45621 bytes
15:08:13.712 in Ack 7
15:08:13.960 chunk 7 45666 bytes
15:08:13.961 in Ack 7
15:08:14.210 chunk 7 45696 bytes
15:08:14.211 in Ack 7
15:08:14.222 fetch POST /api/observability/v1/traces
15:08:14.222 fetched POST /api/observability/v1/traces 204
15:08:14.460 chunk 7 45726 bytes
15:08:14.461 in Ack 7
15:08:14.710 chunk 7 45771 bytes
15:08:14.711 in Ack 7
15:08:14.961 chunk 7 45801 bytes
15:08:14.962 in Ack 7
15:08:15.211 chunk 7 45846 bytes
15:08:15.212 in Ack 7
15:08:15.461 chunk 7 45876 bytes
15:08:15.462 fetch POST /api/observability/v1/traces
15:08:15.462 fetched POST /api/observability/v1/traces 204
15:08:15.463 in Ack 7
15:08:15.711 chunk 7 45921 bytes
15:08:15.712 in Ack 7
15:08:15.961 chunk 7 45951 bytes
15:08:15.963 in Ack 7
15:08:16.212 chunk 7 45996 bytes
15:08:16.213 in Ack 7
15:08:16.462 chunk 7 46026 bytes
15:08:16.465 in Ack 7
15:08:16.479 fetch POST /api/observability/v1/traces
15:08:16.479 fetched POST /api/observability/v1/traces 204
15:08:16.717 chunk 7 46071 bytes
15:08:16.718 in Ack 7
15:08:16.967 chunk 7 46116 bytes
15:08:16.968 in Ack 7
15:08:17.217 chunk 7 46146 bytes
15:08:17.218 in Ack 7
15:08:17.392 in Ping 
15:08:17.469 chunk 7 46176 bytes
15:08:17.470 in Ack 7
15:08:17.719 fetch POST /api/observability/v1/traces
15:08:17.719 fetched POST /api/observability/v1/traces 204
15:08:17.720 chunk 7 46221 bytes
15:08:17.720 in Ack 7
15:08:17.896 rpc 23 orchestration.dispatchCommand thread.visit
15:08:17.974 chunk 7 46251 bytes
15:08:17.975 in Ack 7
15:08:18.021 exit 23 {"_tag":"Success","value":{"sequence":87}}
15:08:18.168 chunk 13 2887 bytes
15:08:18.169 in Ack 13
15:08:18.224 chunk 7 46296 bytes
15:08:18.225 in Ack 7
15:08:18.475 chunk 7 46326 bytes
15:08:18.476 in Ack 7
15:08:18.722 fetch POST /api/observability/v1/traces
15:08:18.722 fetched POST /api/observability/v1/traces 204
15:08:18.725 chunk 7 46371 bytes
15:08:18.727 in Ack 7
15:08:18.975 chunk 7 46401 bytes
15:08:18.976 in Ack 7
15:08:19.225 chunk 7 46446 bytes
15:08:19.227 in Ack 7
15:08:19.476 chunk 7 46476 bytes
15:08:19.477 in Ack 7
15:08:19.726 chunk 7 46521 bytes
15:08:19.727 in Ack 7
15:08:19.742 fetch POST /api/observability/v1/traces
15:08:19.742 fetched POST /api/observability/v1/traces 204
15:08:19.977 chunk 7 46551 bytes
15:08:19.977 in Ack 7
15:08:20.226 chunk 7 46596 bytes
15:08:20.227 in Ack 7
15:08:20.478 chunk 7 46626 bytes
15:08:20.479 in Ack 7
15:08:20.727 chunk 7 46671 bytes
15:08:20.728 in Ack 7
15:08:21.060 chunk 7 46686 bytes
15:08:21.062 fetch POST /api/observability/v1/traces
15:08:21.062 fetched POST /api/observability/v1/traces 204
15:08:21.067 in Ack 7
15:08:21.310 chunk 7 46761 bytes
15:08:21.311 in Ack 7
15:08:21.560 chunk 7 46791 bytes
15:08:21.561 in Ack 7
15:08:21.811 chunk 7 46836 bytes
15:08:21.812 in Ack 7
15:08:22.066 chunk 7 46866 bytes
15:08:22.068 in Ack 7
15:08:22.078 fetch POST /api/observability/v1/traces
15:08:22.078 fetched POST /api/observability/v1/traces 204
15:08:22.317 chunk 7 46911 bytes
15:08:22.318 in Ack 7
15:08:22.392 in Ping 
15:08:22.568 chunk 7 46941 bytes
15:08:22.569 in Ack 7
15:08:22.650 rpc 24 server.reportClientActivity
15:08:22.794 exit 24 {"_tag":"Success","value":null}
15:08:22.818 chunk 7 46986 bytes
15:08:22.819 in Ack 7
15:08:23.070 chunk 7 47016 bytes
15:08:23.071 in Ack 7
15:08:23.319 fetch POST /api/observability/v1/traces
15:08:23.319 fetched POST /api/observability/v1/traces 204
15:08:23.322 chunk 7 47061 bytes
15:08:23.323 in Ack 7
15:08:23.572 chunk 7 47091 bytes
15:08:23.573 in Ack 7
15:08:23.828 chunk 7 47136 bytes
15:08:23.829 in Ack 7
15:08:24.087 chunk 7 47166 bytes
15:08:24.088 in Ack 7
15:08:24.328 fetch POST /api/observability/v1/traces
15:08:24.328 fetched POST /api/observability/v1/traces 204
15:08:24.339 chunk 7 47211 bytes
15:08:24.340 in Ack 7
15:08:24.587 chunk 7 47241 bytes
15:08:24.590 in Ack 7
15:08:24.862 chunk 7 47286 bytes
15:08:24.863 in Ack 7
15:08:25.115 chunk 7 47331 bytes
15:08:25.121 in Ack 7
15:08:25.341 fetch POST /api/observability/v1/traces
15:08:25.341 fetched POST /api/observability/v1/traces 204
15:08:25.365 chunk 7 47361 bytes
15:08:25.366 in Ack 7
15:08:25.617 chunk 7 47391 bytes
15:08:25.618 in Ack 7
15:08:25.867 chunk 7 47436 bytes
15:08:25.868 in Ack 7
15:08:26.119 chunk 7 47481 bytes
15:08:26.122 in Ack 7
15:08:26.367 fetch POST /api/observability/v1/traces
15:08:26.367 fetched POST /api/observability/v1/traces 204
15:08:26.371 chunk 7 47511 bytes
15:08:26.374 in Ack 7
15:08:26.621 chunk 7 47556 bytes
15:08:26.628 in Ack 7
15:08:26.871 chunk 7 47586 bytes
15:08:26.873 in Ack 7
15:08:27.122 chunk 7 47616 bytes
15:08:27.123 in Ack 7
15:08:27.374 chunk 7 47661 bytes
15:08:27.375 in Ack 7
15:08:27.406 fetch POST /api/observability/v1/traces
15:08:27.406 fetched POST /api/observability/v1/traces 204
15:08:27.407 in Ping 
15:08:27.626 chunk 7 57739 bytes
15:08:27.630 in Ack 7
15:08:27.779 chunk 13 3197 bytes
15:08:27.780 in Ack 13
15:08:27.896 rpc 25 orchestration.dispatchCommand thread.visit
15:08:27.986 chunk 7 57739 bytes
15:08:27.987 exit 25 {"_tag":"Success","value":{"sequence":96}}
15:08:27.988 in Ack 7
15:08:28.221 rpc 26 orchestration.dispatchCommand runtime-request.respond
15:08:28.316 chunk 13 3197 bytes
15:08:28.324 in Ack 13
15:08:28.386 exit 26 {"_tag":"Success","value":{"sequence":99}}
15:08:28.505 chunk 7 60391 bytes
15:08:28.506 in Ack 7
15:08:28.516 in Interrupt 18
15:08:28.566 rpc 27 subscribeVcsStatus
15:08:28.603 chunk 13 2889 bytes
15:08:28.604 in Ack 13
15:08:28.632 fetch POST /api/observability/v1/traces
15:08:28.632 fetched POST /api/observability/v1/traces 204
15:08:28.776 chunk 27 1363 bytes
15:08:28.777 chunk 7 60411 bytes
15:08:28.777 in Ack 27
15:08:28.781 in Ack 7
15:08:28.817 rpc 28 server.reportClientActivity
15:08:29.030 chunk 7 60441 bytes
15:08:29.031 in Ack 7
15:08:29.124 exit 28 {"_tag":"Success","value":null}
15:08:29.283 chunk 7 60471 bytes
15:08:29.286 in Ack 7
15:08:29.530 chunk 7 60491 bytes
15:08:29.532 in Ack 7
15:08:29.780 chunk 7 60521 bytes
15:08:29.781 in Ack 7
15:08:29.795 fetch POST /api/observability/v1/traces
15:08:29.795 fetched POST /api/observability/v1/traces 204
15:08:30.044 chunk 7 60541 bytes
15:08:30.046 in Ack 7
15:08:30.294 chunk 7 60571 bytes
15:08:30.295 in Ack 7
15:08:30.554 chunk 7 60591 bytes
15:08:30.555 in Ack 7
15:08:30.804 chunk 7 60621 bytes
15:08:30.805 in Ack 7
15:08:31.048 fetch POST /api/observability/v1/traces
15:08:31.048 fetched POST /api/observability/v1/traces 204
15:08:31.054 chunk 7 60641 bytes
15:08:31.056 in Ack 7
15:08:31.305 chunk 7 60671 bytes
15:08:31.310 in Ack 7
15:08:31.556 chunk 7 60691 bytes
15:08:31.558 in Ack 7
15:08:31.806 chunk 7 60721 bytes
15:08:31.807 in Ack 7
15:08:32.056 chunk 7 60741 bytes
15:08:32.057 fetch POST /api/observability/v1/traces
15:08:32.057 fetched POST /api/observability/v1/traces 204
15:08:32.058 in Ack 7
15:08:32.306 chunk 7 60771 bytes
15:08:32.307 in Ack 7
15:08:32.407 in Ping 
15:08:32.556 chunk 7 60791 bytes
15:08:32.557 in Ack 7
15:08:32.806 chunk 7 60821 bytes
15:08:32.807 in Ack 7
15:08:33.061 chunk 7 60841 bytes
15:08:33.062 fetch POST /api/observability/v1/traces
15:08:33.062 fetched POST /api/observability/v1/traces 204
15:08:33.069 in Ack 7
15:08:33.312 chunk 7 60871 bytes
15:08:33.320 in Ack 7
15:08:33.562 chunk 7 60891 bytes
15:08:33.563 in Ack 7
15:08:33.812 chunk 7 60921 bytes
15:08:33.814 in Ack 7
15:08:34.062 chunk 7 60941 bytes
15:08:34.063 in Ack 7
15:08:34.076 fetch POST /api/observability/v1/traces
15:08:34.076 fetched POST /api/observability/v1/traces 204
15:08:34.312 chunk 7 60971 bytes
15:08:34.329 in Ack 7
15:08:34.563 chunk 7 60991 bytes
15:08:34.564 in Ack 7
15:08:34.813 chunk 7 61021 bytes
15:08:34.814 in Ack 7
15:08:35.062 chunk 7 61041 bytes
15:08:35.064 in Ack 7
15:08:35.313 chunk 7 61071 bytes
15:08:35.314 in Ack 7
15:08:35.354 fetch POST /api/observability/v1/traces
15:08:35.354 fetched POST /api/observability/v1/traces 204
15:08:35.563 chunk 7 61091 bytes
15:08:35.564 in Ack 7
15:08:35.814 chunk 7 61121 bytes
15:08:35.817 in Ack 7
15:08:36.064 chunk 7 61143 bytes
15:08:36.068 in Ack 7
15:08:36.211 chunk 13 2889 bytes
15:08:36.211 in Ack 13
15:08:36.315 chunk 7 62141 bytes
15:08:36.316 in Ack 7
15:08:36.570 fetch POST /api/observability/v1/traces
15:08:36.570 fetched POST /api/observability/v1/traces 204
15:08:36.571 chunk 7 62171 bytes
15:08:36.573 in Ack 7
15:08:36.821 chunk 7 62216 bytes
15:08:36.824 in Ack 7
15:08:37.073 chunk 7 62246 bytes
15:08:37.074 in Ack 7
15:08:37.323 chunk 7 62291 bytes
15:08:37.324 in Ack 7
15:08:37.414 in Ping 
15:08:37.574 fetch POST /api/observability/v1/traces
15:08:37.574 fetched POST /api/observability/v1/traces 204
15:08:37.575 chunk 7 62327 bytes
15:08:37.577 in Ack 7
15:08:37.826 chunk 7 62372 bytes
15:08:37.828 in Ack 7
15:08:37.925 rpc 29 orchestration.dispatchCommand thread.visit
15:08:38.077 chunk 7 62408 bytes
15:08:38.079 in Ack 7
15:08:38.123 exit 29 {"_tag":"Success","value":{"sequence":104}}
15:08:38.308 chunk 13 2889 bytes
15:08:38.309 in Ack 13
15:08:38.329 chunk 7 62453 bytes
15:08:38.330 in Ack 7
15:08:38.578 fetch POST /api/observability/v1/traces
15:08:38.578 fetched POST /api/observability/v1/traces 204
15:08:38.579 chunk 7 62483 bytes
15:08:38.580 in Ack 7
15:08:38.829 chunk 7 62534 bytes
15:08:38.831 in Ack 7
15:08:39.080 chunk 7 62564 bytes
15:08:39.082 in Ack 7
15:08:39.330 chunk 7 62609 bytes
15:08:39.332 in Ack 7
15:08:39.581 chunk 7 62639 bytes
15:08:39.582 in Ack 7
15:08:39.606 fetch POST /api/observability/v1/traces
15:08:39.606 fetched POST /api/observability/v1/traces 204
15:08:39.833 chunk 7 62693 bytes
15:08:39.835 in Ack 7
15:08:40.085 chunk 7 62723 bytes
15:08:40.088 in Ack 7
15:08:40.336 chunk 7 62777 bytes
15:08:40.337 in Ack 7
15:08:40.589 chunk 7 62810 bytes
15:08:40.592 in Ack 7
15:08:40.837 fetch POST /api/observability/v1/traces
15:08:40.837 fetched POST /api/observability/v1/traces 204
15:08:40.838 chunk 7 62864 bytes
15:08:40.839 in Ack 7
15:08:41.092 chunk 7 62894 bytes
15:08:41.093 in Ack 7
15:08:41.343 chunk 7 62945 bytes
15:08:41.344 in Ack 7
15:08:41.594 chunk 7 62996 bytes
15:08:41.595 in Ack 7
15:08:41.840 fetch POST /api/observability/v1/traces
15:08:41.840 fetched POST /api/observability/v1/traces 204
15:08:41.844 chunk 7 63029 bytes
15:08:41.845 in Ack 7
15:08:42.094 chunk 7 63062 bytes
15:08:42.096 in Ack 7
15:08:42.344 chunk 7 64342 bytes
15:08:42.345 in Ack 7
15:08:42.494 in Ping 
15:08:42.495 chunk 13 2825 bytes
15:08:42.496 in Ack 13
15:08:42.498 rpc 30 orchestration.dispatchCommand thread.visit
15:08:42.595 chunk 7 67536 bytes
15:08:42.597 in Ack 7
15:08:42.627 in Interrupt 27
15:08:42.677 rpc 31 subscribeVcsStatus
15:08:42.749 exit 30 {"_tag":"Success","value":{"sequence":120}}
15:08:42.845 chunk 7 67536 bytes
15:08:42.847 in Ack 7
15:08:42.856 fetch POST /api/observability/v1/traces
15:08:42.856 fetched POST /api/observability/v1/traces 204
15:08:42.879 chunk 31 1427 bytes
15:08:42.879 in Ack 31
15:08:42.929 rpc 32 server.reportClientActivity
15:08:43.000 chunk 13 2825 bytes
15:08:43.000 in Ack 13
