# B, webapp, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 58 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 2.4s |
| Prompt confirmation | 7.5s |
| Prompt reached the agent | 6.9s |
| Time to first text | 8.0s |
| Whole reply shown | 53.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.6s / 0.9s / 1.9s |
| Longest silence | 8.0s |
| Approval shown after it was asked | 1.3s |
| Bytes on the wire down / up | 854725 / 478707 |
| Bytes the app received / sent | 573943 / 90197 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.8 | 89 | 122 |
| 20 | 1.0 | 482 | 523 |
| 30 | 0.9 | 889 | 925 |
| 40 | 1.6 | 1120 | 1181 |
| 50 | 1.8 | 1487 | 1551 |
| 53 | 2.1 | 1579 | 1596 |

## Driver notes

- webapp {"objects":{"assistant":"21/21 32/32","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456","user":"10/10 9/9","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","providerThreads":"810/810","attempts":"662/662 662/662","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2015,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"assistant":"21/21 32/32 130/130","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108 849/849 1083/1083","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456","user":"10/10 9/9","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809","providerTurns":"778/778 778/778","runs":"971/971 1014/1014 714/839","providerThreads":"810/810","attempts":"662/662 662/662 638/638","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8502,"lastDataAt":1794150564302,"silences":0,"longestSilenceMs":336,"longestRecoveredGapMs":336},"stats":{"sendRate":4551.283690980787,"deliveredRate":3034.189127320524,"loss":0,"srttMs":1200.083506989633,"minRttMs":933,"sentPackets":52,"sentWireBytes":17190,"receivedPackets":44,"receivedWireBytes":37883,"lastHeardMs":14412,"unsyncedBytes":167,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 628/628","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108 849/849 1083/1083","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456 451/451 549/549","user":"10/10 9/9 64/64","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809 785/785","providerTurns":"778/778 778/778 660/660","runs":"971/971 1014/1014 839/839","providerThreads":"812/812","attempts":"662/662 662/662 638/638","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8502,"lastDataAt":1794150574553,"silences":0,"longestSilenceMs":375,"longestRecoveredGapMs":375},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1240.0136438257407,"minRttMs":914,"sentPackets":141,"sentWireBytes":35412,"receivedPackets":300,"receivedWireBytes":114878,"lastHeardMs":24663,"unsyncedBytes":167,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1133/1133","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108 849/849 1083/1083","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456 451/451 549/549","user":"10/10 9/9 64/64","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809 785/785","providerTurns":"778/778 778/778 660/660","runs":"971/971 1014/1014 839/839","providerThreads":"812/812","attempts":"662/662 662/662 638/638","frame":"1032/1032","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8502,"lastDataAt":1794150584592,"silences":0,"longestSilenceMs":375,"longestRecoveredGapMs":375},"stats":{"sendRate":3186.3586018747374,"deliveredRate":1525.031904886889,"loss":0.06214219114219113,"srttMs":1331.0788127135384,"minRttMs":914,"sentPackets":203,"sentWireBytes":47548,"receivedPackets":761,"receivedWireBytes":221696,"lastHeardMs":34702,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1340/1340 158/158","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108 849/849 1108/1108 1578/1578 956/956 1083/1083","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456 451/451 550/550","user":"10/10 9/9 64/64","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809 785/785 989/989 1401/1401 1025/1025","providerTurns":"778/778 778/778 660/660","runs":"971/971 1014/1014 839/839","providerThreads":"812/812","attempts":"662/662 662/662 638/638","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8502,"lastDataAt":1794150594927,"silences":0,"longestSilenceMs":375,"longestRecoveredGapMs":375},"stats":{"sendRate":2985.065248912518,"deliveredRate":1912.7464099926929,"loss":0.0823529411764706,"srttMs":1420.136051392284,"minRttMs":914,"sentPackets":274,"sentWireBytes":66057,"receivedPackets":1098,"receivedWireBytes":388229,"lastHeardMs":45037,"unsyncedBytes":439,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"assistant":"21/21 32/32 1340/1340 719/719","providerSessions":"2688/2688","turnItems":"1299/1299 1108/1108 1058/1058 860/860 870/870 1058/1058 1108/1108 849/849 1108/1108 1578/1578 956/956 1083/1083","checkpoints":"779/779 613/613 613/613","messages":"550/550 901/901 550/550 456/456 451/451 550/550 549/549","user":"10/10 9/9 64/64","thread":"951/951","nodes":"1025/1025 1025/1025 809/809 809/809 785/785 989/989 1401/1401 1025/1025","providerTurns":"778/778 778/778 660/660","runs":"971/971 1014/1014 839/839","providerThreads":"812/812","attempts":"662/662 662/662 638/638","frame":"1032/1032","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8502,"lastDataAt":1794150606197,"silences":0,"longestSilenceMs":375,"longestRecoveredGapMs":375},"stats":{"sendRate":2842.520462107791,"deliveredRate":1778.3330783171996,"loss":0.19057663967996705,"srttMs":1408.6265119000266,"minRttMs":914,"sentPackets":357,"sentWireBytes":87027,"receivedPackets":1538,"receivedWireBytes":550162,"lastHeardMs":56307,"unsyncedBytes":0,"inflightPackets":27,"duplicateBytes":0}}
- shim trace
15:09:10.939 fetch GET /api/auth/session
15:09:10.942 fetched GET /api/auth/session 200
15:09:11.139 fetch GET /.well-known/t3/environment
15:09:11.140 fetched GET /.well-known/t3/environment 200
15:09:11.175 fetch GET /.well-known/t3/environment
15:09:11.177 fetched GET /.well-known/t3/environment 200
15:09:11.194 rpc 0 subscribeServerConfig
15:09:11.198 chunk 0 38690 bytes
15:09:11.246 in Ack 0
15:09:11.256 rpc 1 subscribeServerLifecycle
15:09:11.257 rpc 2 subscribeServerLifecycle
15:09:11.262 fetch GET /api/auth/session
15:09:11.263 fetched GET /api/auth/session 200
15:09:11.264 rpc 3 subscribeDeviceState
15:09:11.268 rpc 4 preview.reportProfiles
15:09:11.445 rpc 5 subscribeProjectClones
15:09:11.533 rpc 6 server.reportClientActivity
15:09:11.574 rpc 7 orchestration.subscribeThread
15:09:11.576 chunk 7 35300 bytes
15:09:11.582 in Ack 7
15:09:11.592 fetch GET /api/orchestration/shell
15:09:11.673 rpc 8 server.refreshProviders
15:09:11.784 rpc 9 orchestration.dispatchCommand thread.visit
15:09:11.892 fetched GET /api/orchestration/shell 200
15:09:11.892 rpc 10 subscribeTerminalMetadata
15:09:11.894 rpc 11 subscribePreviewEvents
15:09:11.895 rpc 12 subscribeVcsStatus
15:09:11.898 rpc 13 orchestration.subscribeShell
15:09:11.899 chunk 13 2847 bytes
15:09:11.901 rpc 14 assets.createUrl
15:09:11.902 rpc 15 preview.list
15:09:11.903 rpc 16 projects.readFile
15:09:11.903 rpc 17 assets.createUrl
15:09:11.964 in Ack 13
15:09:12.131 fetch POST /api/observability/v1/traces
15:09:12.131 fetched POST /api/observability/v1/traces 204
15:09:13.405 fetch POST /api/observability/v1/traces
15:09:13.405 fetched POST /api/observability/v1/traces 204
15:09:16.183 in Ping 
15:09:20.161 exit 9 {"_tag":"Success","value":{"sequence":70}}
15:09:20.161 exit 15 {"_tag":"Success","value":{"revision":0,"serverEpoch":"aa5c69fe-e445-45aa-9a96-25fd9bb2a90b","sessions":[]}}
15:09:20.168 rpc 18 orchestration.dispatchCommand message.dispatch
15:09:20.219 chunk 1 3864 bytes
15:09:20.219 in Ack 1
15:09:21.166 fetch POST /api/observability/v1/traces
15:09:21.166 fetched POST /api/observability/v1/traces 204
15:09:21.185 in Ping 
15:09:21.572 exit 18 {"_tag":"Success","value":{"sequence":77}}
15:09:21.572 exit 16 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:09:21.572 chunk 2 3864 bytes
15:09:21.572 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:08:47.662Z","configurableProviders":false,"con
15:09:21.579 in Ack 2
15:09:21.885 chunk 7 35276 bytes
15:09:21.885 chunk 13 2823 bytes
15:09:21.885 chunk 12 1427 bytes
15:09:21.885 exit 14 {"_tag":"Success","value":{"expiresAt":1794153600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:09:21.889 in Ack 7
15:09:21.915 in Ack 13
15:09:21.939 in Ack 12
15:09:22.135 chunk 7 37762 bytes
15:09:22.138 in Ack 7
15:09:22.385 chunk 7 37772 bytes
15:09:22.387 in Ack 7
15:09:22.579 fetch POST /api/observability/v1/traces
15:09:22.579 fetched POST /api/observability/v1/traces 204
15:09:22.636 chunk 7 37802 bytes
15:09:22.638 in Ack 7
15:09:22.886 chunk 7 37812 bytes
15:09:22.888 in Ack 7
15:09:23.137 chunk 7 37842 bytes
15:09:23.138 in Ack 7
15:09:23.386 chunk 7 37872 bytes
15:09:23.387 chunk 13 944 bytes
15:09:23.388 in Ack 7
15:09:23.409 in Ack 13
15:09:23.637 chunk 7 37864 bytes
15:09:23.639 in Ack 7
15:09:23.688 fetch POST /api/observability/v1/traces
15:09:23.689 fetched POST /api/observability/v1/traces 204
15:09:23.887 chunk 7 37874 bytes
15:09:23.889 in Ack 7
15:09:24.138 chunk 7 37894 bytes
15:09:24.139 in Ack 7
15:09:24.180 chunk 13 2887 bytes
15:09:24.183 in Ack 13
15:09:24.189 rpc 19 orchestration.dispatchCommand thread.visit
15:09:24.388 chunk 7 38573 bytes
15:09:24.390 in Ack 7
15:09:24.639 chunk 7 41436 bytes
15:09:24.640 in Ack 7
15:09:24.889 chunk 7 42127 bytes
15:09:24.890 in Ack 7
15:09:24.900 fetch POST /api/observability/v1/traces
15:09:24.900 fetched POST /api/observability/v1/traces 204
15:09:25.139 chunk 7 42157 bytes
15:09:25.140 in Ack 7
15:09:25.313 exit 19 {"_tag":"Success","value":{"sequence":85}}
15:09:25.779 chunk 13 944 bytes
15:09:25.779 in Ack 13
15:09:26.141 fetch POST /api/observability/v1/traces
15:09:26.141 fetched POST /api/observability/v1/traces 204
15:09:26.185 in Ping 
15:09:26.534 in Interrupt 6
15:09:26.659 chunk 7 42287 bytes
15:09:26.660 in Ack 7
15:09:26.789 rpc 20 server.reportClientActivity
15:09:26.869 chunk 13 2887 bytes
15:09:26.869 in Ack 13
15:09:26.909 chunk 7 43631 bytes
15:09:26.910 in Ack 7
15:09:26.952 chunk 3 876 bytes
15:09:26.952 exit 4 {"_tag":"Success","value":null}
15:09:26.952 chunk 5 4 bytes
15:09:26.952 exit 6 {"_tag":"Success","value":null}
15:09:26.952 chunk 10 36 bytes
15:09:26.954 in Ack 3
15:09:26.961 rpc 21 preview.reportProfiles
15:09:26.961 in Ack 5
15:09:26.962 in Ack 10
15:09:27.159 chunk 7 43651 bytes
15:09:27.160 in Ack 7
15:09:27.412 chunk 7 43681 bytes
15:09:27.414 in Ack 7
15:09:27.538 fetch POST /api/observability/v1/traces
15:09:27.538 fetched POST /api/observability/v1/traces 204
15:09:27.663 chunk 7 43701 bytes
15:09:27.664 in Ack 7
15:09:27.872 exit 17 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/ef92197a-cc91-498e-a168-6ce1a8bb790f","expiresAt":1794154167872}}
15:09:27.949 chunk 7 43721 bytes
15:09:27.951 in Ack 7
15:09:27.998 exit 20 {"_tag":"Success","value":null}
15:09:28.199 chunk 7 43751 bytes
15:09:28.201 in Ack 7
15:09:28.227 exit 21 {"_tag":"Success","value":null}
15:09:28.493 chunk 7 43781 bytes
15:09:28.494 in Ack 7
15:09:28.666 fetch POST /api/observability/v1/traces
15:09:28.666 fetched POST /api/observability/v1/traces 204
15:09:28.743 chunk 7 43811 bytes
15:09:28.744 in Ack 7
15:09:29.003 chunk 7 43821 bytes
15:09:29.004 in Ack 7
15:09:29.256 chunk 7 43861 bytes
15:09:29.257 in Ack 7
15:09:29.506 chunk 7 43881 bytes
15:09:29.510 in Ack 7
15:09:29.747 fetch POST /api/observability/v1/traces
15:09:29.748 fetched POST /api/observability/v1/traces 204
15:09:29.759 chunk 7 43911 bytes
15:09:29.760 in Ack 7
15:09:30.034 chunk 7 43941 bytes
15:09:30.036 in Ack 7
15:09:30.284 chunk 7 43971 bytes
15:09:30.291 in Ack 7
15:09:30.534 chunk 7 43991 bytes
15:09:30.535 in Ack 7
15:09:30.762 fetch POST /api/observability/v1/traces
15:09:30.762 fetched POST /api/observability/v1/traces 204
15:09:30.786 chunk 7 44021 bytes
15:09:30.787 in Ack 7
15:09:31.036 chunk 7 44031 bytes
15:09:31.038 in Ack 7
15:09:31.185 in Ping 
15:09:31.288 chunk 7 44061 bytes
15:09:31.290 in Ack 7
15:09:31.538 chunk 7 44081 bytes
15:09:31.541 in Ack 7
15:09:31.800 fetch POST /api/observability/v1/traces
15:09:31.800 fetched POST /api/observability/v1/traces 204
15:09:31.800 chunk 7 44121 bytes
15:09:31.801 in Ack 7
15:09:31.909 chunk 13 944 bytes
15:09:31.909 in Ack 13
15:09:32.053 chunk 7 45201 bytes
15:09:32.054 in Ack 7
15:09:32.159 chunk 13 2887 bytes
15:09:32.160 in Ack 13
15:09:32.330 chunk 7 45231 bytes
15:09:32.331 in Ack 7
15:09:32.581 chunk 7 45261 bytes
15:09:32.582 in Ack 7
15:09:32.814 fetch POST /api/observability/v1/traces
15:09:32.814 fetched POST /api/observability/v1/traces 204
15:09:32.830 chunk 7 45291 bytes
15:09:32.831 in Ack 7
15:09:33.081 chunk 7 45351 bytes
15:09:33.083 in Ack 7
15:09:33.332 chunk 7 45396 bytes
15:09:33.333 in Ack 7
15:09:33.581 chunk 7 45426 bytes
15:09:33.583 in Ack 7
15:09:33.832 chunk 7 45456 bytes
15:09:33.833 fetch POST /api/observability/v1/traces
15:09:33.833 fetched POST /api/observability/v1/traces 204
15:09:33.834 in Ack 7
15:09:34.083 chunk 7 45471 bytes
15:09:34.085 in Ack 7
15:09:34.198 rpc 22 orchestration.dispatchCommand thread.visit
15:09:34.333 chunk 7 45546 bytes
15:09:34.334 in Ack 7
15:09:34.583 chunk 7 45561 bytes
15:09:34.584 in Ack 7
15:09:34.834 chunk 7 45606 bytes
15:09:34.835 in Ack 7
15:09:34.854 fetch POST /api/observability/v1/traces
15:09:34.854 fetched POST /api/observability/v1/traces 204
15:09:35.085 chunk 7 45651 bytes
15:09:35.087 in Ack 7
15:09:35.348 chunk 7 45681 bytes
15:09:35.351 in Ack 7
15:09:35.587 exit 22 {"_tag":"Success","value":{"sequence":88}}
15:09:35.600 chunk 7 45711 bytes
15:09:35.601 in Ack 7
15:09:35.756 chunk 13 2887 bytes
15:09:35.757 in Ack 13
15:09:35.853 chunk 7 45756 bytes
15:09:35.855 in Ack 7
15:09:36.089 fetch POST /api/observability/v1/traces
15:09:36.089 fetched POST /api/observability/v1/traces 204
15:09:36.104 chunk 7 45786 bytes
15:09:36.105 in Ack 7
15:09:36.188 in Ping 
15:09:36.356 chunk 7 45816 bytes
15:09:36.357 in Ack 7
15:09:36.436 rpc 23 server.reportClientActivity
15:09:36.605 chunk 7 45876 bytes
15:09:36.611 in Ack 7
15:09:36.858 chunk 7 45921 bytes
15:09:36.859 in Ack 7
15:09:37.106 fetch POST /api/observability/v1/traces
15:09:37.106 fetched POST /api/observability/v1/traces 204
15:09:37.107 chunk 7 45951 bytes
15:09:37.109 in Ack 7
15:09:37.358 chunk 7 45981 bytes
15:09:37.359 in Ack 7
15:09:37.609 chunk 7 46041 bytes
15:09:37.610 in Ack 7
15:09:37.812 exit 23 {"_tag":"Success","value":null}
15:09:37.859 chunk 7 46071 bytes
15:09:37.861 in Ack 7
15:09:38.109 fetch POST /api/observability/v1/traces
15:09:38.109 fetched POST /api/observability/v1/traces 204
15:09:38.110 chunk 7 46086 bytes
15:09:38.111 in Ack 7
15:09:38.360 chunk 7 46131 bytes
15:09:38.361 in Ack 7
15:09:38.609 chunk 7 46161 bytes
15:09:38.610 in Ack 7
15:09:38.859 chunk 7 46191 bytes
15:09:38.860 in Ack 7
15:09:39.110 chunk 7 46236 bytes
15:09:39.111 in Ack 7
15:09:39.128 fetch POST /api/observability/v1/traces
15:09:39.128 fetched POST /api/observability/v1/traces 204
15:09:39.360 chunk 7 46266 bytes
15:09:39.361 in Ack 7
15:09:39.612 chunk 7 46326 bytes
15:09:39.614 in Ack 7
15:09:39.862 chunk 7 46356 bytes
15:09:39.863 in Ack 7
15:09:40.113 chunk 7 46386 bytes
15:09:40.115 in Ack 7
15:09:40.361 fetch POST /api/observability/v1/traces
15:09:40.362 fetched POST /api/observability/v1/traces 204
15:09:40.363 chunk 7 46431 bytes
15:09:40.365 in Ack 7
15:09:40.614 chunk 7 46461 bytes
15:09:40.615 in Ack 7
15:09:40.865 chunk 7 46506 bytes
15:09:40.865 in Ack 7
15:09:41.114 chunk 7 46536 bytes
15:09:41.115 in Ack 7
15:09:41.188 in Ping 
15:09:41.365 chunk 7 46581 bytes
15:09:41.366 in Ack 7
15:09:41.427 fetch POST /api/observability/v1/traces
15:09:41.427 fetched POST /api/observability/v1/traces 204
15:09:41.616 chunk 7 46611 bytes
15:09:41.617 in Ack 7
15:09:41.866 chunk 7 46626 bytes
15:09:41.867 in Ack 7
15:09:42.118 chunk 7 46686 bytes
15:09:42.120 in Ack 7
15:09:42.430 chunk 7 46761 bytes
15:09:42.618 in Ack 7
15:09:42.629 fetch POST /api/observability/v1/traces
15:09:42.629 fetched POST /api/observability/v1/traces 204
15:09:42.681 chunk 7 46776 bytes
15:09:42.682 in Ack 7
15:09:42.938 chunk 7 46806 bytes
15:09:42.939 in Ack 7
15:09:43.188 chunk 7 46851 bytes
15:09:43.189 in Ack 7
15:09:43.438 chunk 7 46881 bytes
15:09:43.439 in Ack 7
15:09:43.634 fetch POST /api/observability/v1/traces
15:09:43.634 fetched POST /api/observability/v1/traces 204
15:09:43.689 chunk 7 46926 bytes
15:09:43.690 in Ack 7
15:09:43.939 chunk 7 46986 bytes
15:09:43.940 in Ack 7
15:09:44.236 chunk 7 47001 bytes
15:09:44.238 in Ack 7
15:09:44.487 chunk 7 47046 bytes
15:09:44.488 in Ack 7
15:09:44.697 fetch POST /api/observability/v1/traces
15:09:44.697 fetched POST /api/observability/v1/traces 204
15:09:44.737 chunk 7 47106 bytes
15:09:44.739 in Ack 7
15:09:44.995 chunk 7 47121 bytes
15:09:44.997 in Ack 7
15:09:45.246 chunk 7 47181 bytes
15:09:45.247 in Ack 7
15:09:45.495 chunk 7 47196 bytes
15:09:45.496 in Ack 7
15:09:45.742 fetch POST /api/observability/v1/traces
15:09:45.742 fetched POST /api/observability/v1/traces 204
15:09:45.745 chunk 7 47241 bytes
15:09:45.748 in Ack 7
15:09:45.996 chunk 7 47286 bytes
15:09:45.997 in Ack 7
15:09:46.190 in Ping 
15:09:46.247 chunk 7 47316 bytes
15:09:46.248 in Ack 7
15:09:46.497 chunk 7 47346 bytes
15:09:46.501 in Ack 7
15:09:46.748 chunk 7 47391 bytes
15:09:46.750 in Ack 7
15:09:46.761 fetch POST /api/observability/v1/traces
15:09:46.761 fetched POST /api/observability/v1/traces 204
15:09:47.023 chunk 7 47421 bytes
15:09:47.030 in Ack 7
15:09:47.274 chunk 7 47466 bytes
15:09:47.279 in Ack 7
15:09:47.524 chunk 7 47511 bytes
15:09:47.533 in Ack 7
15:09:47.775 chunk 7 47541 bytes
15:09:47.776 in Ack 7
15:09:48.027 chunk 7 47556 bytes
15:09:48.028 in Ack 7
15:09:48.047 fetch POST /api/observability/v1/traces
15:09:48.047 fetched POST /api/observability/v1/traces 204
15:09:48.278 chunk 7 47601 bytes
15:09:48.279 in Ack 7
15:09:48.528 chunk 7 47646 bytes
15:09:48.529 in Ack 7
15:09:48.778 chunk 7 47676 bytes
15:09:48.779 in Ack 7
15:09:48.936 chunk 13 944 bytes
15:09:48.936 in Ack 13
15:09:49.040 chunk 7 53357 bytes
15:09:49.041 in Ack 7
15:09:49.186 chunk 13 3197 bytes
15:09:49.187 in Ack 13
15:09:49.188 rpc 24 orchestration.dispatchCommand thread.visit
15:09:49.280 fetch POST /api/observability/v1/traces
15:09:49.280 fetched POST /api/observability/v1/traces 204
15:09:49.290 chunk 7 54368 bytes
15:09:49.291 in Ack 7
15:09:50.292 fetch POST /api/observability/v1/traces
15:09:50.292 fetched POST /api/observability/v1/traces 204
15:09:50.709 exit 24 {"_tag":"Success","value":{"sequence":97}}
15:09:50.712 rpc 25 orchestration.dispatchCommand runtime-request.respond
15:09:50.940 chunk 7 54368 bytes
15:09:50.940 chunk 13 3197 bytes
15:09:50.941 in Ack 7
15:09:50.950 in Ack 13
15:09:51.191 in Ping 
15:09:51.441 chunk 7 55746 bytes
15:09:51.443 in Ack 7
15:09:51.693 chunk 7 57739 bytes
15:09:51.695 in Ack 7
15:09:51.710 fetch POST /api/observability/v1/traces
15:09:51.710 fetched POST /api/observability/v1/traces 204
15:09:52.313 chunk 7 58094 bytes
15:09:52.313 exit 25 {"_tag":"Success","value":{"sequence":100}}
15:09:52.315 in Ack 7
15:09:52.563 chunk 7 57113 bytes
15:09:52.564 in Ack 7
15:09:52.674 chunk 13 2889 bytes
15:09:52.676 in Ack 13
15:09:52.814 chunk 7 54672 bytes
15:09:52.817 in Ack 7
15:09:53.104 chunk 7 60451 bytes
15:09:53.106 in Ack 7
15:09:53.125 in Interrupt 12
15:09:53.171 rpc 26 subscribeVcsStatus
15:09:53.316 fetch POST /api/observability/v1/traces
15:09:53.316 fetched POST /api/observability/v1/traces 204
15:09:53.355 chunk 7 60481 bytes
15:09:53.357 in Ack 7
15:09:53.435 rpc 27 server.reportClientActivity
15:09:53.608 chunk 7 60501 bytes
15:09:53.609 in Ack 7
15:09:53.858 chunk 7 60531 bytes
15:09:53.859 in Ack 7
15:09:54.109 chunk 7 60561 bytes
15:09:54.110 in Ack 7
15:09:54.359 fetch POST /api/observability/v1/traces
15:09:54.360 fetched POST /api/observability/v1/traces 204
15:09:54.361 chunk 7 60581 bytes
15:09:54.362 in Ack 7
15:09:54.611 chunk 7 60601 bytes
15:09:54.613 in Ack 7
15:09:54.862 chunk 7 60621 bytes
15:09:54.863 in Ack 7
15:09:54.927 chunk 26 1427 bytes
15:09:54.928 in Ack 26
15:09:55.112 chunk 7 60661 bytes
15:09:55.116 in Ack 7
15:09:55.362 chunk 7 60671 bytes
15:09:55.363 fetch POST /api/observability/v1/traces
15:09:55.363 fetched POST /api/observability/v1/traces 204
15:09:55.365 in Ack 7
15:09:55.613 chunk 7 60701 bytes
15:09:55.614 in Ack 7
15:09:55.863 chunk 7 60741 bytes
15:09:55.865 in Ack 7
15:09:56.150 chunk 7 60761 bytes
15:09:56.155 in Ack 7
15:09:56.191 in Ping 
15:09:56.366 fetch POST /api/observability/v1/traces
15:09:56.366 fetched POST /api/observability/v1/traces 204
15:09:56.400 chunk 7 60791 bytes
15:09:56.401 in Ack 7
15:09:56.650 chunk 7 60821 bytes
15:09:56.651 in Ack 7
15:09:56.913 chunk 7 60841 bytes
15:09:56.914 in Ack 7
15:09:57.167 chunk 7 60871 bytes
15:09:57.168 in Ack 7
15:09:57.402 fetch POST /api/observability/v1/traces
15:09:57.402 fetched POST /api/observability/v1/traces 204
15:09:57.418 chunk 7 60891 bytes
15:09:57.419 in Ack 7
15:09:57.671 chunk 7 60901 bytes
15:09:57.672 in Ack 7
15:09:57.937 chunk 7 60931 bytes
15:09:57.938 in Ack 7
15:09:58.189 chunk 7 60951 bytes
15:09:58.190 in Ack 7
15:09:58.255 exit 27 {"_tag":"Success","value":null}
15:09:58.420 fetch POST /api/observability/v1/traces
15:09:58.421 fetched POST /api/observability/v1/traces 204
15:09:58.441 chunk 7 60981 bytes
15:09:58.443 in Ack 7
15:09:58.691 chunk 7 61001 bytes
15:09:58.692 in Ack 7
15:09:58.941 chunk 7 61041 bytes
15:09:58.942 in Ack 7
15:09:59.188 rpc 28 orchestration.dispatchCommand thread.visit
15:09:59.191 chunk 7 61051 bytes
15:09:59.193 in Ack 7
15:09:59.442 chunk 7 61091 bytes
15:09:59.444 in Ack 7
15:09:59.496 fetch POST /api/observability/v1/traces
15:09:59.496 fetched POST /api/observability/v1/traces 204
15:09:59.693 chunk 7 61131 bytes
15:09:59.694 in Ack 7
15:09:59.995 chunk 7 61143 bytes
15:09:59.996 in Ack 7
15:10:00.093 chunk 13 2889 bytes
15:10:00.094 in Ack 13
15:10:00.245 chunk 7 61147 bytes
15:10:00.246 in Ack 7
15:10:00.496 chunk 7 61179 bytes
15:10:00.497 in Ack 7
15:10:00.610 exit 28 {"_tag":"Success","value":{"sequence":105}}
15:10:00.610 chunk 13 2889 bytes
15:10:00.621 in Ack 13
15:10:00.697 fetch POST /api/observability/v1/traces
15:10:00.697 fetched POST /api/observability/v1/traces 204
15:10:01.195 in Ping 
15:10:01.416 rpc 29 server.reportClientActivity
15:10:02.251 chunk 7 62438 bytes
15:10:02.253 in Ack 7
15:10:02.427 fetch POST /api/observability/v1/traces
15:10:02.427 fetched POST /api/observability/v1/traces 204
15:10:02.504 chunk 7 62483 bytes
15:10:02.506 in Ack 7
15:10:02.760 chunk 7 62498 bytes
15:10:02.761 in Ack 7
15:10:03.028 chunk 7 62549 bytes
15:10:03.029 in Ack 7
15:10:03.278 chunk 7 62594 bytes
15:10:03.279 in Ack 7
15:10:03.509 fetch POST /api/observability/v1/traces
15:10:03.509 fetched POST /api/observability/v1/traces 204
15:10:03.530 chunk 7 62639 bytes
15:10:03.531 in Ack 7
15:10:03.780 chunk 7 62708 bytes
15:10:03.781 in Ack 7
15:10:04.047 chunk 7 62723 bytes
15:10:04.087 in Ack 7
15:10:04.290 chunk 7 62777 bytes
15:10:04.291 in Ack 7
15:10:04.532 fetch POST /api/observability/v1/traces
15:10:04.532 fetched POST /api/observability/v1/traces 204
15:10:04.544 chunk 7 62825 bytes
15:10:04.548 in Ack 7
15:10:04.714 exit 29 {"_tag":"Success","value":null}
15:10:04.795 chunk 7 62864 bytes
15:10:04.797 in Ack 7
15:10:05.076 chunk 7 62894 bytes
15:10:05.077 in Ack 7
15:10:05.327 chunk 7 62975 bytes
15:10:05.337 in Ack 7
15:10:05.549 fetch POST /api/observability/v1/traces
15:10:05.549 fetched POST /api/observability/v1/traces 204
15:10:05.578 chunk 7 62996 bytes
15:10:05.580 in Ack 7
15:10:05.838 chunk 7 63044 bytes
15:10:05.839 in Ack 7
15:10:06.089 chunk 7 63062 bytes
15:10:06.090 in Ack 7
15:10:06.198 in Ping 
15:10:06.349 chunk 7 63175 bytes
15:10:06.354 in Ack 7
15:10:06.508 chunk 13 2828 bytes
15:10:06.508 in Ack 13
15:10:06.581 fetch POST /api/observability/v1/traces
15:10:06.582 fetched POST /api/observability/v1/traces 204
15:10:06.598 chunk 7 63331 bytes
15:10:06.600 in Ack 7
15:10:06.850 chunk 7 61103 bytes
15:10:06.851 in Ack 7
15:10:06.859 in Interrupt 26
15:10:06.863 rpc 30 subscribeVcsStatus
15:10:07.008 chunk 13 2825 bytes
15:10:07.009 in Ack 13
15:10:07.010 rpc 31 orchestration.dispatchCommand thread.visit
15:10:07.099 chunk 7 62129 bytes
15:10:07.101 in Ack 7
15:10:07.114 rpc 32 server.reportClientActivity
