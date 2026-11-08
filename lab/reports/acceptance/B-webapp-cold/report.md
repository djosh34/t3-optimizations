# B, webapp, cold start, seed 1

Profile B (graded): congested LTE, queue and stall stress. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 55 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.2s |
| Prompt confirmation | 10.0s |
| Prompt reached the agent | 9.4s |
| Time to first text | 10.5s |
| Whole reply shown | 53.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.3s / 1.0s / 1.8s |
| Longest silence | 10.5s |
| Approval shown after it was asked | 0.9s |
| Bytes on the wire down / up | 884049 / 445559 |
| Bytes the app received / sent | 581254 / 74841 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.5 | 0 | 21 |
| 20 | 0.8 | 389 | 419 |
| 30 | 1.0 | 783 | 824 |
| 40 | 1.1 | 1116 | 1158 |
| 50 | 1.5 | 1477 | 1528 |
| 53 | 1.8 | 1567 | 1596 |

## Driver notes

- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108","attempts":"662/662 662/662","assistant":"32/32 21/21","messages":"456/456 901/901 550/550 550/550","frame":"1032/1032","runs":"971/971 1014/1014","nodes":"809/809 809/809 1025/1025 1025/1025","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778","user":"10/10 9/9","checkpointScopes":"726/726","providerThreads":"810/810"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2675,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108 56/1083","attempts":"662/662 662/662","assistant":"32/32 21/21 5/5","messages":"456/456 901/901 550/550 550/550","frame":"1032/1032","runs":"971/971 1014/1014","nodes":"809/809 809/809 1025/1025 1025/1025","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778","user":"10/10 9/9","checkpointScopes":"726/726","providerThreads":"810/810"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8494,"lastDataAt":1794159327085,"silences":0,"longestSilenceMs":270,"longestRecoveredGapMs":270},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":1239.5256042480469,"minRttMs":1123,"sentPackets":20,"sentWireBytes":9950,"receivedPackets":26,"receivedWireBytes":28249,"lastHeardMs":12060,"unsyncedBytes":522,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108 1083/1083 849/849","attempts":"662/662 662/662 638/638","assistant":"32/32 21/21 503/503","messages":"456/456 901/901 550/550 550/550 451/451 470/549","frame":"1032/1032","runs":"971/971 1014/1014 839/839","nodes":"809/809 809/809 1025/1025 1025/1025 785/785","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778 660/660","user":"10/10 9/9 64/64","checkpointScopes":"726/726","providerThreads":"812/812"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8494,"lastDataAt":1794159337250,"silences":0,"longestSilenceMs":337,"longestRecoveredGapMs":337},"stats":{"sendRate":4500,"deliveredRate":2194.304229272515,"loss":0,"srttMs":1215.5766059989794,"minRttMs":889,"sentPackets":126,"sentWireBytes":30841,"receivedPackets":234,"receivedWireBytes":87420,"lastHeardMs":22225,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108 1083/1083 849/849","attempts":"662/662 662/662 638/638","assistant":"32/32 21/21 1003/1003","messages":"456/456 901/901 550/550 550/550 451/451 549/549","frame":"1032/1032","runs":"971/971 1014/1014 839/839","nodes":"809/809 809/809 1025/1025 1025/1025 785/785","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778 660/660","user":"10/10 9/9 64/64","checkpointScopes":"726/726","providerThreads":"812/812"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8494,"lastDataAt":1794159347361,"silences":0,"longestSilenceMs":337,"longestRecoveredGapMs":337},"stats":{"sendRate":3000,"deliveredRate":1721.554373952914,"loss":0.03151372549019607,"srttMs":1273.3193998667628,"minRttMs":889,"sentPackets":196,"sentWireBytes":44548,"receivedPackets":611,"receivedWireBytes":183365,"lastHeardMs":32336,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108 1108/1108 849/849 1578/1578 956/956 1083/1083","attempts":"662/662 662/662 638/638","assistant":"32/32 21/21 1340/1340 143/143","messages":"456/456 901/901 550/550 550/550 451/451 550/550","frame":"1032/1032","runs":"971/971 1014/1014 839/839","nodes":"809/809 809/809 1025/1025 1025/1025 785/785 989/989 1025/1025 1401/1401","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778 660/660","user":"10/10 9/9 64/64","checkpointScopes":"726/726","providerThreads":"812/812","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8494,"lastDataAt":1794159357982,"silences":0,"longestSilenceMs":402,"longestRecoveredGapMs":402},"stats":{"sendRate":3000,"deliveredRate":1318.5430463576158,"loss":0.30000000000000004,"srttMs":1339.2498716471719,"minRttMs":889,"sentPackets":254,"sentWireBytes":60036,"receivedPackets":1035,"receivedWireBytes":336943,"lastHeardMs":42957,"unsyncedBytes":893,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 779/779 613/613","turnItems":"1108/1108 870/870 1058/1058 860/860 1058/1058 1299/1299 1108/1108 1108/1108 849/849 1578/1578 956/956 1083/1083","attempts":"662/662 662/662 638/638","assistant":"32/32 21/21 1340/1340 689/689","messages":"456/456 901/901 550/550 550/550 451/451 550/550 549/549","frame":"1032/1032","runs":"971/971 1014/1014 839/839","nodes":"809/809 809/809 1025/1025 1025/1025 785/785 989/989 1025/1025 1401/1401","providerSessions":"2688/2688","thread":"951/951","providerTurns":"778/778 778/778 660/660","user":"10/10 9/9 64/64","checkpointScopes":"726/726","providerThreads":"812/812","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":8494,"lastDataAt":1794159368970,"silences":0,"longestSilenceMs":402,"longestRecoveredGapMs":402},"stats":{"sendRate":1287.1931452167382,"deliveredRate":0,"loss":0.18818823529411766,"srttMs":1437.0817281815753,"minRttMs":889,"sentPackets":313,"sentWireBytes":72214,"receivedPackets":1503,"receivedWireBytes":557892,"lastHeardMs":53945,"unsyncedBytes":0,"inflightPackets":9,"duplicateBytes":0}}
- shim trace
17:35:15.489 fetch GET /api/auth/session
17:35:15.492 fetched GET /api/auth/session 200
17:35:15.591 fetch GET /.well-known/t3/environment
17:35:15.592 fetched GET /.well-known/t3/environment 200
17:35:15.611 fetch GET /.well-known/t3/environment
17:35:15.612 fetched GET /.well-known/t3/environment 200
17:35:15.619 rpc 0 subscribeServerConfig
17:35:15.620 chunk 0 38690 bytes
17:35:15.642 in Ack 0
17:35:15.645 rpc 1 subscribeServerLifecycle
17:35:15.646 rpc 2 subscribeServerLifecycle
17:35:15.650 fetch GET /api/auth/session
17:35:15.651 fetched GET /api/auth/session 200
17:35:15.651 rpc 3 subscribeDeviceState
17:35:15.653 rpc 4 preview.reportProfiles
17:35:15.753 rpc 5 subscribeProjectClones
17:35:15.793 rpc 6 orchestration.subscribeThread
17:35:15.795 chunk 6 35300 bytes
17:35:15.798 in Ack 6
17:35:15.807 fetch GET /api/orchestration/shell
17:35:15.863 rpc 7 server.refreshProviders
17:35:15.958 rpc 8 orchestration.dispatchCommand thread.visit
17:35:15.958 exit 8 {"_tag":"Success","value":{"sequence":0}}
17:35:16.041 fetched GET /api/orchestration/shell 200
17:35:16.041 rpc 9 subscribeTerminalMetadata
17:35:16.043 rpc 10 subscribePreviewEvents
17:35:16.044 rpc 11 subscribeVcsStatus
17:35:16.046 rpc 12 orchestration.subscribeShell
17:35:16.046 chunk 12 2847 bytes
17:35:16.049 rpc 13 assets.createUrl
17:35:16.049 rpc 14 preview.list
17:35:16.050 rpc 15 projects.readFile
17:35:16.050 rpc 16 assets.createUrl
17:35:16.054 exit 16 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/3d7cf1b9-c43a-4d08-8b3e-c80bfda27c48","expiresAt":1794162916054}}
17:35:16.099 in Ack 12
17:35:16.354 rpc 17 server.reportClientActivity
17:35:16.587 fetch POST /api/observability/v1/traces
17:35:16.587 fetched POST /api/observability/v1/traces 204
17:35:17.004 rpc 18 orchestration.dispatchCommand message.dispatch
17:35:18.004 fetch POST /api/observability/v1/traces
17:35:18.004 fetched POST /api/observability/v1/traces 204
17:35:20.616 in Ping 
17:35:25.072 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"ca8c8110-c579-4aab-837e-e4ff8731adc7","sessions":[]}}
17:35:25.165 chunk 1 3864 bytes
17:35:25.166 in Ack 1
17:35:25.616 in Ping 
17:35:26.074 fetch POST /api/observability/v1/traces
17:35:26.074 fetched POST /api/observability/v1/traces 204
17:35:26.817 exit 18 {"_tag":"Success","value":{"sequence":76}}
17:35:26.817 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:35:26.817 chunk 2 3864 bytes
17:35:26.817 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:34:56.034Z","configurableProviders":false,"con
17:35:26.821 in Ack 2
17:35:26.833 chunk 6 35276 bytes
17:35:26.833 chunk 12 2823 bytes
17:35:26.833 chunk 11 534 bytes
17:35:26.833 exit 13 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:35:26.834 in Ack 6
17:35:26.852 in Ack 12
17:35:26.857 in Ack 11
17:35:27.086 chunk 6 37732 bytes
17:35:27.088 in Ack 6
17:35:27.337 chunk 6 37752 bytes
17:35:27.338 in Ack 6
17:35:27.587 chunk 6 37782 bytes
17:35:27.588 in Ack 6
17:35:27.818 fetch POST /api/observability/v1/traces
17:35:27.818 fetched POST /api/observability/v1/traces 204
17:35:27.837 chunk 6 37802 bytes
17:35:27.838 in Ack 6
17:35:28.086 chunk 6 37774 bytes
17:35:28.088 in Ack 6
17:35:28.337 chunk 6 37804 bytes
17:35:28.338 in Ack 6
17:35:28.389 chunk 12 2887 bytes
17:35:28.390 in Ack 12
17:35:28.393 rpc 19 orchestration.dispatchCommand thread.visit
17:35:28.393 exit 19 {"_tag":"Success","value":{"sequence":0}}
17:35:28.587 chunk 6 37834 bytes
17:35:28.588 in Ack 6
17:35:28.837 chunk 6 38483 bytes
17:35:28.838 in Ack 6
17:35:28.847 fetch POST /api/observability/v1/traces
17:35:28.847 fetched POST /api/observability/v1/traces 204
17:35:29.088 chunk 6 41356 bytes
17:35:29.089 in Ack 6
17:35:29.338 chunk 6 41366 bytes
17:35:29.339 in Ack 6
17:35:29.588 chunk 6 42853 bytes
17:35:29.589 in Ack 6
17:35:29.838 chunk 6 42895 bytes
17:35:29.839 in Ack 6
17:35:30.089 chunk 6 42915 bytes
17:35:30.090 in Ack 6
17:35:30.118 fetch POST /api/observability/v1/traces
17:35:30.118 fetched POST /api/observability/v1/traces 204
17:35:30.160 chunk 12 2887 bytes
17:35:30.160 in Ack 12
17:35:30.339 chunk 6 42935 bytes
17:35:30.340 in Ack 6
17:35:30.588 chunk 6 42955 bytes
17:35:30.589 in Ack 6
17:35:30.617 in Ping 
17:35:30.839 chunk 6 43501 bytes
17:35:30.840 in Ack 6
17:35:31.089 chunk 6 43521 bytes
17:35:31.090 in Ack 6
17:35:31.164 fetch POST /api/observability/v1/traces
17:35:31.164 fetched POST /api/observability/v1/traces 204
17:35:31.339 chunk 6 43561 bytes
17:35:31.340 in Ack 6
17:35:31.359 in Interrupt 17
17:35:31.458 chunk 3 876 bytes
17:35:31.458 in Ack 3
17:35:31.589 chunk 6 43571 bytes
17:35:31.590 in Ack 6
17:35:31.642 exit 4 {"_tag":"Success","value":null}
17:35:31.642 chunk 5 4 bytes
17:35:31.642 chunk 9 36 bytes
17:35:31.642 exit 17 {"_tag":"Success","value":null}
17:35:31.645 rpc 20 preview.reportProfiles
17:35:31.645 in Ack 5
17:35:31.646 in Ack 9
17:35:31.838 chunk 6 43611 bytes
17:35:31.840 in Ack 6
17:35:32.089 chunk 6 43631 bytes
17:35:32.090 in Ack 6
17:35:32.339 chunk 6 43661 bytes
17:35:32.340 in Ack 6
17:35:32.348 fetch POST /api/observability/v1/traces
17:35:32.348 fetched POST /api/observability/v1/traces 204
17:35:32.589 chunk 6 43671 bytes
17:35:32.590 in Ack 6
17:35:32.838 chunk 6 43701 bytes
17:35:32.839 in Ack 6
17:35:32.933 exit 20 {"_tag":"Success","value":null}
17:35:33.089 chunk 6 43721 bytes
17:35:33.090 in Ack 6
17:35:33.348 chunk 6 43741 bytes
17:35:33.350 in Ack 6
17:35:33.591 fetch POST /api/observability/v1/traces
17:35:33.591 fetched POST /api/observability/v1/traces 204
17:35:33.598 chunk 6 43781 bytes
17:35:33.599 in Ack 6
17:35:33.848 chunk 6 43791 bytes
17:35:33.849 in Ack 6
17:35:34.098 chunk 6 43831 bytes
17:35:34.099 in Ack 6
17:35:34.349 chunk 6 43851 bytes
17:35:34.350 in Ack 6
17:35:34.599 chunk 6 43871 bytes
17:35:34.600 in Ack 6
17:35:34.608 fetch POST /api/observability/v1/traces
17:35:34.608 fetched POST /api/observability/v1/traces 204
17:35:34.849 chunk 6 43901 bytes
17:35:34.850 in Ack 6
17:35:35.098 chunk 6 43921 bytes
17:35:35.099 in Ack 6
17:35:35.349 chunk 6 43961 bytes
17:35:35.350 in Ack 6
17:35:35.599 chunk 6 43981 bytes
17:35:35.600 in Ack 6
17:35:35.628 in Ping 
17:35:35.849 chunk 6 43991 bytes
17:35:35.850 in Ack 6
17:35:35.872 fetch POST /api/observability/v1/traces
17:35:35.872 fetched POST /api/observability/v1/traces 204
17:35:36.099 chunk 6 44031 bytes
17:35:36.100 in Ack 6
17:35:36.349 chunk 6 44051 bytes
17:35:36.350 in Ack 6
17:35:36.599 chunk 6 44081 bytes
17:35:36.599 in Ack 6
17:35:36.849 chunk 6 44091 bytes
17:35:36.850 in Ack 6
17:35:37.099 chunk 6 44121 bytes
17:35:37.100 in Ack 6
17:35:37.110 fetch POST /api/observability/v1/traces
17:35:37.110 fetched POST /api/observability/v1/traces 204
17:35:37.279 chunk 12 2887 bytes
17:35:37.279 in Ack 12
17:35:37.350 chunk 6 45201 bytes
17:35:37.351 in Ack 6
17:35:37.600 chunk 6 45231 bytes
17:35:37.601 in Ack 6
17:35:37.850 chunk 6 45306 bytes
17:35:37.851 in Ack 6
17:35:38.100 chunk 6 45321 bytes
17:35:38.101 in Ack 6
17:35:38.280 fetch POST /api/observability/v1/traces
17:35:38.280 fetched POST /api/observability/v1/traces 204
17:35:38.350 chunk 6 45336 bytes
17:35:38.351 in Ack 6
17:35:38.396 rpc 21 orchestration.dispatchCommand thread.visit
17:35:38.396 exit 21 {"_tag":"Success","value":{"sequence":0}}
17:35:38.601 chunk 6 45411 bytes
17:35:38.602 in Ack 6
17:35:38.851 chunk 6 45441 bytes
17:35:38.852 in Ack 6
17:35:39.102 chunk 6 45456 bytes
17:35:39.103 in Ack 6
17:35:39.353 fetch POST /api/observability/v1/traces
17:35:39.353 fetched POST /api/observability/v1/traces 204
17:35:39.353 chunk 6 45501 bytes
17:35:39.354 in Ack 6
17:35:39.603 chunk 6 45531 bytes
17:35:39.604 in Ack 6
17:35:39.848 chunk 12 2887 bytes
17:35:39.848 in Ack 12
17:35:39.856 chunk 6 45576 bytes
17:35:39.857 in Ack 6
17:35:40.106 chunk 6 45636 bytes
17:35:40.107 in Ack 6
17:35:40.355 fetch POST /api/observability/v1/traces
17:35:40.355 fetched POST /api/observability/v1/traces 204
17:35:40.356 chunk 6 45666 bytes
17:35:40.357 in Ack 6
17:35:40.606 chunk 6 45696 bytes
17:35:40.607 in Ack 6
17:35:40.629 in Ping 
17:35:40.844 rpc 22 server.reportClientActivity
17:35:40.857 chunk 6 45741 bytes
17:35:40.858 in Ack 6
17:35:41.107 chunk 6 45771 bytes
17:35:41.108 in Ack 6
17:35:41.357 chunk 6 45801 bytes
17:35:41.358 fetch POST /api/observability/v1/traces
17:35:41.358 fetched POST /api/observability/v1/traces 204
17:35:41.359 in Ack 6
17:35:41.607 chunk 6 45861 bytes
17:35:41.608 in Ack 6
17:35:41.858 chunk 6 45876 bytes
17:35:41.859 in Ack 6
17:35:42.109 chunk 6 45921 bytes
17:35:42.110 in Ack 6
17:35:42.131 exit 22 {"_tag":"Success","value":null}
17:35:42.359 chunk 6 45951 bytes
17:35:42.360 in Ack 6
17:35:42.378 fetch POST /api/observability/v1/traces
17:35:42.378 fetched POST /api/observability/v1/traces 204
17:35:42.610 chunk 6 45996 bytes
17:35:42.611 in Ack 6
17:35:42.861 chunk 6 46041 bytes
17:35:42.861 in Ack 6
17:35:43.114 chunk 6 46056 bytes
17:35:43.115 in Ack 6
17:35:43.365 chunk 6 46086 bytes
17:35:43.366 in Ack 6
17:35:43.611 fetch POST /api/observability/v1/traces
17:35:43.611 fetched POST /api/observability/v1/traces 204
17:35:43.614 chunk 6 46146 bytes
17:35:43.615 in Ack 6
17:35:43.865 chunk 6 46206 bytes
17:35:43.866 in Ack 6
17:35:44.175 chunk 6 46221 bytes
17:35:44.176 in Ack 6
17:35:44.425 chunk 6 46266 bytes
17:35:44.426 in Ack 6
17:35:44.616 fetch POST /api/observability/v1/traces
17:35:44.616 fetched POST /api/observability/v1/traces 204
17:35:44.677 chunk 6 46281 bytes
17:35:44.677 in Ack 6
17:35:44.927 chunk 6 46326 bytes
17:35:44.928 in Ack 6
17:35:45.178 chunk 6 46386 bytes
17:35:45.179 in Ack 6
17:35:45.428 chunk 6 46416 bytes
17:35:45.429 in Ack 6
17:35:45.629 in Ping 
17:35:45.679 chunk 6 46446 bytes
17:35:45.679 fetch POST /api/observability/v1/traces
17:35:45.679 fetched POST /api/observability/v1/traces 204
17:35:45.680 in Ack 6
17:35:45.929 chunk 6 46476 bytes
17:35:45.930 in Ack 6
17:35:46.181 chunk 6 46521 bytes
17:35:46.182 in Ack 6
17:35:46.431 chunk 6 46566 bytes
17:35:46.432 in Ack 6
17:35:46.681 chunk 6 46596 bytes
17:35:46.681 fetch POST /api/observability/v1/traces
17:35:46.681 fetched POST /api/observability/v1/traces 204
17:35:46.682 in Ack 6
17:35:46.931 chunk 6 46656 bytes
17:35:46.932 in Ack 6
17:35:47.346 chunk 6 46686 bytes
17:35:47.347 in Ack 6
17:35:47.597 chunk 6 46746 bytes
17:35:47.598 in Ack 6
17:35:47.685 fetch POST /api/observability/v1/traces
17:35:47.685 fetched POST /api/observability/v1/traces 204
17:35:47.893 chunk 6 46806 bytes
17:35:47.894 in Ack 6
17:35:48.144 chunk 6 46821 bytes
17:35:48.145 in Ack 6
17:35:48.393 chunk 6 46851 bytes
17:35:48.394 in Ack 6
17:35:48.644 chunk 6 46911 bytes
17:35:48.645 in Ack 6
17:35:48.893 chunk 6 46926 bytes
17:35:48.894 in Ack 6
17:35:48.906 fetch POST /api/observability/v1/traces
17:35:48.906 fetched POST /api/observability/v1/traces 204
17:35:49.144 chunk 6 46971 bytes
17:35:49.145 in Ack 6
17:35:49.394 chunk 6 46986 bytes
17:35:49.395 in Ack 6
17:35:49.643 chunk 6 47046 bytes
17:35:49.644 in Ack 6
17:35:49.893 chunk 6 47076 bytes
17:35:49.894 in Ack 6
17:35:50.144 chunk 6 47121 bytes
17:35:50.145 in Ack 6
17:35:50.153 fetch POST /api/observability/v1/traces
17:35:50.153 fetched POST /api/observability/v1/traces 204
17:35:50.394 chunk 6 47151 bytes
17:35:50.395 in Ack 6
17:35:50.629 in Ping 
17:35:50.645 chunk 6 47196 bytes
17:35:50.645 in Ack 6
17:35:50.895 chunk 6 47226 bytes
17:35:50.896 in Ack 6
17:35:51.145 chunk 6 47256 bytes
17:35:51.146 in Ack 6
17:35:51.395 chunk 6 47286 bytes
17:35:51.396 fetch POST /api/observability/v1/traces
17:35:51.396 fetched POST /api/observability/v1/traces 204
17:35:51.397 in Ack 6
17:35:51.646 chunk 6 47346 bytes
17:35:51.647 in Ack 6
17:35:51.896 chunk 6 47376 bytes
17:35:51.897 in Ack 6
17:35:52.145 chunk 6 47421 bytes
17:35:52.146 in Ack 6
17:35:52.396 chunk 6 47451 bytes
17:35:52.397 in Ack 6
17:35:52.430 fetch POST /api/observability/v1/traces
17:35:52.430 fetched POST /api/observability/v1/traces 204
17:35:52.645 chunk 6 47481 bytes
17:35:52.646 in Ack 6
17:35:52.896 chunk 6 47541 bytes
17:35:52.897 in Ack 6
17:35:53.146 chunk 6 47556 bytes
17:35:53.147 in Ack 6
17:35:53.398 chunk 6 47601 bytes
17:35:53.399 in Ack 6
17:35:53.647 fetch POST /api/observability/v1/traces
17:35:53.647 fetched POST /api/observability/v1/traces 204
17:35:53.648 chunk 6 47661 bytes
17:35:53.650 in Ack 6
17:35:53.899 chunk 6 48764 bytes
17:35:53.900 in Ack 6
17:35:54.128 rpc 23 orchestration.dispatchCommand runtime-request.respond
17:35:54.424 chunk 6 54368 bytes
17:35:54.425 in Ack 6
17:35:54.650 fetch POST /api/observability/v1/traces
17:35:54.650 fetched POST /api/observability/v1/traces 204
17:35:54.743 chunk 12 3197 bytes
17:35:54.744 in Ack 12
17:35:54.925 chunk 6 56361 bytes
17:35:54.926 in Ack 6
17:35:55.630 in Ping 
17:35:55.745 fetch POST /api/observability/v1/traces
17:35:55.745 fetched POST /api/observability/v1/traces 204
17:35:55.775 chunk 6 58830 bytes
17:35:55.775 exit 23 {"_tag":"Success","value":{"sequence":98}}
17:35:55.777 in Ack 6
17:35:55.818 rpc 24 orchestration.dispatchCommand thread.visit
17:35:55.818 exit 24 {"_tag":"Success","value":{"sequence":0}}
17:35:56.026 chunk 6 58878 bytes
17:35:56.027 in Ack 6
17:35:56.276 chunk 6 60446 bytes
17:35:56.277 in Ack 6
17:35:56.286 in Interrupt 11
17:35:56.309 rpc 25 subscribeVcsStatus
17:35:56.527 chunk 6 60476 bytes
17:35:56.528 in Ack 6
17:35:56.562 rpc 26 server.reportClientActivity
17:35:56.589 rpc 27 orchestration.dispatchCommand runtime-request.respond
17:35:56.777 chunk 6 60496 bytes
17:35:56.778 in Ack 6
17:35:56.805 fetch POST /api/observability/v1/traces
17:35:56.805 fetched POST /api/observability/v1/traces 204
17:35:57.029 chunk 6 60526 bytes
17:35:57.030 in Ack 6
17:35:57.279 chunk 6 60546 bytes
17:35:57.280 in Ack 6
17:35:57.360 chunk 12 2889 bytes
17:35:57.360 in Ack 12
17:35:57.529 chunk 6 60619 bytes
17:35:57.530 in Ack 6
17:35:57.717 chunk 25 534 bytes
17:35:57.718 in Ack 25
17:35:57.779 chunk 6 60601 bytes
17:35:57.780 in Ack 6
17:35:57.902 fetch POST /api/observability/v1/traces
17:35:57.902 fetched POST /api/observability/v1/traces 204
17:35:58.029 chunk 6 60621 bytes
17:35:58.030 in Ack 6
17:35:58.071 exit 27 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
17:35:58.290 chunk 6 60641 bytes
17:35:58.291 in Ack 6
17:35:58.411 exit 26 {"_tag":"Success","value":null}
17:35:58.541 chunk 6 60671 bytes
17:35:58.542 in Ack 6
17:35:58.791 chunk 6 60691 bytes
17:35:58.792 in Ack 6
17:35:59.031 fetch POST /api/observability/v1/traces
17:35:59.031 fetched POST /api/observability/v1/traces 204
17:35:59.059 chunk 6 60721 bytes
17:35:59.060 in Ack 6
17:35:59.310 chunk 6 60751 bytes
17:35:59.311 in Ack 6
17:35:59.560 chunk 6 60781 bytes
17:35:59.561 in Ack 6
17:35:59.809 chunk 6 60801 bytes
17:35:59.810 in Ack 6
17:36:00.060 chunk 6 60831 bytes
17:36:00.061 in Ack 6
17:36:00.070 fetch POST /api/observability/v1/traces
17:36:00.070 fetched POST /api/observability/v1/traces 204
17:36:00.310 chunk 6 60851 bytes
17:36:00.311 in Ack 6
17:36:00.559 chunk 6 60881 bytes
17:36:00.561 in Ack 6
17:36:00.630 in Ping 
17:36:00.810 chunk 6 60911 bytes
17:36:00.810 in Ack 6
17:36:01.074 chunk 6 60921 bytes
17:36:01.076 in Ack 6
17:36:01.312 fetch POST /api/observability/v1/traces
17:36:01.312 fetched POST /api/observability/v1/traces 204
17:36:01.326 chunk 6 60951 bytes
17:36:01.327 in Ack 6
17:36:01.576 chunk 6 60971 bytes
17:36:01.577 in Ack 6
17:36:01.827 chunk 6 61001 bytes
17:36:01.828 in Ack 6
17:36:02.077 chunk 6 61011 bytes
17:36:02.078 in Ack 6
17:36:02.328 chunk 6 61051 bytes
17:36:02.328 fetch POST /api/observability/v1/traces
17:36:02.328 fetched POST /api/observability/v1/traces 204
17:36:02.329 in Ack 6
17:36:02.578 chunk 6 61071 bytes
17:36:02.579 in Ack 6
17:36:02.827 chunk 6 61101 bytes
17:36:02.828 in Ack 6
17:36:03.077 chunk 6 61131 bytes
17:36:03.078 in Ack 6
17:36:03.330 fetch POST /api/observability/v1/traces
17:36:03.330 fetched POST /api/observability/v1/traces 204
17:36:03.369 chunk 6 61147 bytes
17:36:03.370 in Ack 6
17:36:03.378 chunk 12 2889 bytes
17:36:03.378 in Ack 12
17:36:03.619 chunk 6 61169 bytes
17:36:03.620 in Ack 6
17:36:03.870 chunk 6 61189 bytes
17:36:03.873 in Ack 6
17:36:04.148 chunk 6 61229 bytes
17:36:04.150 in Ack 6
17:36:04.371 fetch POST /api/observability/v1/traces
17:36:04.371 fetched POST /api/observability/v1/traces 204
17:36:04.398 chunk 6 61239 bytes
17:36:04.400 in Ack 6
17:36:04.649 chunk 6 61269 bytes
17:36:04.651 in Ack 6
17:36:04.745 rpc 28 orchestration.dispatchCommand thread.visit
17:36:04.745 exit 28 {"_tag":"Success","value":{"sequence":0}}
17:36:04.899 chunk 6 62357 bytes
17:36:04.900 in Ack 6
17:36:05.156 chunk 6 62387 bytes
17:36:05.159 in Ack 6
17:36:05.401 fetch POST /api/observability/v1/traces
17:36:05.401 fetched POST /api/observability/v1/traces 204
17:36:05.406 chunk 6 62438 bytes
17:36:05.408 in Ack 6
17:36:05.630 in Ping 
17:36:05.662 chunk 6 62468 bytes
17:36:05.663 in Ack 6
17:36:05.844 rpc 29 server.reportClientActivity
17:36:05.913 chunk 6 62519 bytes
17:36:05.914 in Ack 6
17:36:06.167 chunk 6 62549 bytes
17:36:06.168 in Ack 6
17:36:06.248 chunk 12 2889 bytes
17:36:06.249 in Ack 12
17:36:06.408 fetch POST /api/observability/v1/traces
17:36:06.408 fetched POST /api/observability/v1/traces 204
17:36:06.417 chunk 6 62579 bytes
17:36:06.418 in Ack 6
17:36:06.667 chunk 6 62594 bytes
17:36:06.668 in Ack 6
17:36:06.917 chunk 6 62663 bytes
17:36:06.919 in Ack 6
17:36:07.169 chunk 6 62693 bytes
17:36:07.170 in Ack 6
17:36:07.419 fetch POST /api/observability/v1/traces
17:36:07.419 fetched POST /api/observability/v1/traces 204
17:36:07.420 chunk 6 62753 bytes
17:36:07.421 in Ack 6
17:36:07.670 chunk 6 62777 bytes
17:36:07.671 in Ack 6
17:36:07.920 chunk 6 62810 bytes
17:36:07.921 in Ack 6
17:36:08.213 chunk 6 62864 bytes
17:36:08.214 in Ack 6
17:36:08.323 exit 29 {"_tag":"Success","value":null}
17:36:08.422 fetch POST /api/observability/v1/traces
17:36:08.422 fetched POST /api/observability/v1/traces 204
17:36:08.464 chunk 6 62909 bytes
17:36:08.465 in Ack 6
17:36:08.725 chunk 6 62975 bytes
17:36:08.726 in Ack 6
17:36:08.975 chunk 6 63014 bytes
17:36:08.977 in Ack 6
17:36:09.224 chunk 6 63044 bytes
17:36:09.225 in Ack 6
17:36:09.466 fetch POST /api/observability/v1/traces
17:36:09.466 fetched POST /api/observability/v1/traces 204
17:36:09.476 chunk 6 63092 bytes
17:36:09.477 in Ack 6
17:36:09.813 chunk 12 2825 bytes
17:36:09.813 in Ack 12
17:36:09.814 rpc 30 orchestration.dispatchCommand thread.visit
17:36:09.814 exit 30 {"_tag":"Success","value":{"sequence":0}}
17:36:09.976 chunk 6 65573 bytes
17:36:09.977 in Ack 6
17:36:10.292 chunk 6 66377 bytes
17:36:10.309 in Ack 6
17:36:10.316 in Interrupt 25
17:36:10.320 rpc 31 subscribeVcsStatus
