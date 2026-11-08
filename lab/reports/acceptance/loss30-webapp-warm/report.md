# loss30, webapp, warm start, seed 1

Profile loss30 (fault): 30% random loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.6s |
| Time to first text | 3.0s |
| Whole reply shown | 43.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 2.6s / 0.9s / 1.8s |
| Longest silence | 4.4s |
| Approval shown after it was asked | 1.3s |
| Bytes on the wire down / up | 418786 / 255223 |
| Bytes the app received / sent | 286739 / 63570 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.0 | 333 | 373 |
| 20 | 0.7 | 746 | 774 |
| 30 | 0.6 | 1077 | 1105 |
| 40 | 2.0 | 1414 | 1479 |
| 43 | 1.0 | 1551 | 1583 |

## Driver notes

- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019","messages":"546/546 456/456 901/901 546/546","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":216,"lastDataAt":1794165991822,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":6960.990331276829,"deliveredRate":3589.771368976642,"loss":0,"srttMs":3.4643977555102374,"minRttMs":1,"sentPackets":105,"sentWireBytes":31951,"receivedPackets":133,"receivedWireBytes":66515,"lastHeardMs":9397,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"523/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 450/450","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","messages":"546/546 456/456 901/901 546/546 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"660/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":216,"lastDataAt":1794166001851,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":2000,"deliveredRate":1226.159793814433,"loss":0.4285714285714286,"srttMs":426.66606495490277,"minRttMs":713,"sentPackets":162,"sentWireBytes":42389,"receivedPackets":209,"receivedWireBytes":100378,"lastHeardMs":19426,"unsyncedBytes":0,"inflightPackets":9,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 958/958","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","messages":"546/546 456/456 901/901 546/546 451/451 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":216,"lastDataAt":1794166012098,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":4500,"deliveredRate":1814.0872065165308,"loss":0.38095238095238093,"srttMs":812.9315077273613,"minRttMs":713,"sentPackets":230,"sentWireBytes":56711,"receivedPackets":464,"receivedWireBytes":165685,"lastHeardMs":29673,"unsyncedBytes":0,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1546/1546 926/926","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 38/48","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","messages":"546/546 456/456 901/901 546/546 451/451 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"247/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":216,"lastDataAt":1794166022061,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.326130303030303,"srttMs":855.6689592782909,"minRttMs":713,"sentPackets":284,"sentWireBytes":67233,"receivedPackets":762,"receivedWireBytes":244339,"lastHeardMs":39636,"unsyncedBytes":566,"inflightPackets":27,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1030/1030","turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 589/589","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 985/985 1395/1395 1019/1019","messages":"546/546 456/456 901/901 546/546 451/451 546/546 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":216,"lastDataAt":1794166033220,"silences":0,"longestSilenceMs":1504,"longestRecoveredGapMs":1504},"stats":{"sendRate":2000,"deliveredRate":1721.7821782178219,"loss":0.30000000000000004,"srttMs":905.4354368553844,"minRttMs":647,"sentPackets":374,"sentWireBytes":90875,"receivedPackets":992,"receivedWireBytes":333859,"lastHeardMs":50796,"unsyncedBytes":497,"inflightPackets":49,"duplicateBytes":0}}
- shim trace
19:26:22.706 fetch GET /api/auth/session
19:26:22.707 fetched GET /api/auth/session 200
19:26:22.847 fetch GET /.well-known/t3/environment
19:26:22.847 fetched GET /.well-known/t3/environment 200
19:26:22.856 fetch GET /.well-known/t3/environment
19:26:22.857 fetched GET /.well-known/t3/environment 200
19:26:22.860 rpc 0 subscribeServerConfig
19:26:22.861 chunk 0 38710 bytes
19:26:22.876 in Ack 0
19:26:22.878 rpc 1 subscribeServerLifecycle
19:26:22.879 rpc 2 subscribeServerLifecycle
19:26:22.881 fetch GET /api/auth/session
19:26:22.883 fetched GET /api/auth/session 200
19:26:22.883 rpc 3 subscribeDeviceState
19:26:22.885 rpc 4 preview.reportProfiles
19:26:22.922 rpc 5 subscribeProjectClones
19:26:22.941 fetch GET /api/orchestration/threads/mcp%3A9677f819-f658-4487-b8aa-fb2938bcd146/bounded
19:26:22.945 fetch GET /api/orchestration/shell
19:26:22.963 rpc 6 server.refreshProviders
19:26:23.001 rpc 7 orchestration.dispatchCommand thread.visit
19:26:23.001 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:26:23.011 fetched GET /api/orchestration/shell 200
19:26:23.011 rpc 8 subscribeTerminalMetadata
19:26:23.012 rpc 9 subscribeVcsStatus
19:26:23.012 rpc 10 subscribePreviewEvents
19:26:23.014 rpc 11 orchestration.subscribeShell
19:26:23.014 chunk 11 2787 bytes
19:26:23.015 rpc 12 assets.createUrl
19:26:23.016 rpc 13 preview.list
19:26:23.016 rpc 14 projects.readFile
19:26:23.018 in Ack 11
19:26:23.100 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"82158309-d6af-4f7b-b969-e9a4b690faf9","sessions":[]}}
19:26:23.100 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:26:23.219 chunk 1 3869 bytes
19:26:23.219 in Ack 1
19:26:23.264 rpc 15 server.reportClientActivity
19:26:23.729 chunk 2 3869 bytes
19:26:23.729 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:26:10.835Z","configurableProviders":false,"con
19:26:23.729 chunk 9 1410 bytes
19:26:23.730 in Ack 2
19:26:23.732 in Ack 9
19:26:23.845 fetch POST /api/observability/v1/traces
19:26:23.845 fetched POST /api/observability/v1/traces 204
19:26:23.859 chunk 11 2763 bytes
19:26:23.859 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:26:23.859 in Ack 11
19:26:24.057 chunk 3 876 bytes
19:26:24.058 in Ack 3
19:26:24.109 chunk 11 2806 bytes
19:26:24.110 in Ack 11
19:26:24.138 exit 4 {"_tag":"Success","value":null}
19:26:24.138 chunk 5 4 bytes
19:26:24.138 chunk 8 36 bytes
19:26:24.138 exit 15 {"_tag":"Success","value":null}
19:26:24.139 rpc 16 preview.reportProfiles
19:26:24.139 in Ack 5
19:26:24.139 in Ack 8
19:26:24.213 exit 16 {"_tag":"Success","value":null}
19:26:24.298 fetched GET /api/orchestration/threads/mcp%3A9677f819-f658-4487-b8aa-fb2938bcd146/bounded 200
19:26:24.301 rpc 17 orchestration.subscribeThread
19:26:24.301 chunk 17 8283 bytes
19:26:24.310 in Interrupt 9
19:26:24.312 rpc 18 subscribeVcsStatus
19:26:24.312 in Ack 17
19:26:24.419 chunk 18 1410 bytes
19:26:24.419 in Ack 18
19:26:24.552 chunk 17 11920 bytes
19:26:24.553 in Ack 17
19:26:24.604 rpc 19 server.reportClientActivity
19:26:24.788 exit 19 {"_tag":"Success","value":null}
19:26:24.801 chunk 17 13502 bytes
19:26:24.804 in Ack 17
19:26:24.860 fetch POST /api/observability/v1/traces
19:26:24.860 fetched POST /api/observability/v1/traces 204
19:26:25.052 chunk 17 18024 bytes
19:26:25.053 in Ack 17
19:26:25.183 rpc 20 attachments.createUploadUrl
19:26:25.340 chunk 17 20462 bytes
19:26:25.350 in Ack 17
19:26:25.359 in Interrupt 18
19:26:25.361 rpc 21 subscribeVcsStatus
19:26:25.418 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-0d832725-8e2c-4b4c-8657-694bb6453466","expiresAt":1794166585378,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:26:25.418 chunk 21 1410 bytes
19:26:25.419 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTBkODMyNzI1LThlMmMtNGI0Yy04NjU3LTY5NGJiNjQ1MzQ2NiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjU4NTM3OH0.4dyRkwABeCxfzmi3VckwdTbk4t8mPwUcEQfRbUVmfCo
19:26:25.419 in Ack 21
19:26:25.591 chunk 17 21193 bytes
19:26:25.592 in Ack 17
19:26:25.613 rpc 22 server.reportClientActivity
19:26:26.055 fetch POST /api/observability/v1/traces
19:26:26.055 fetched POST /api/observability/v1/traces 204
19:26:27.148 exit 22 {"_tag":"Success","value":null}
19:26:27.859 in Ping 
19:26:28.149 fetch POST /api/observability/v1/traces
19:26:28.150 fetched POST /api/observability/v1/traces 204
19:26:28.317 rpc 23 orchestration.dispatchCommand message.dispatch
19:26:28.919 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:26:28.921 rpc 24 attachments.delete
19:26:29.028 exit 24 {"_tag":"Success","value":null}
19:26:29.169 chunk 11 2892 bytes
19:26:29.169 chunk 17 27057 bytes
19:26:29.169 in Ack 11
19:26:29.194 in Ack 17
19:26:29.196 rpc 25 assets.createUrl
19:26:29.316 fetch POST /api/observability/v1/traces
19:26:29.316 fetched POST /api/observability/v1/traces 204
19:26:29.420 chunk 17 27869 bytes
19:26:29.421 in Ack 17
19:26:29.671 chunk 17 29374 bytes
19:26:29.672 in Ack 17
19:26:29.921 chunk 11 2828 bytes
19:26:29.921 chunk 17 29530 bytes
19:26:29.921 in Ack 11
19:26:29.922 rpc 26 orchestration.dispatchCommand thread.visit
19:26:29.922 exit 26 {"_tag":"Success","value":{"sequence":0}}
19:26:29.927 in Ack 17
19:26:30.170 chunk 11 2828 bytes
19:26:30.171 chunk 17 29530 bytes
19:26:30.171 in Ack 11
19:26:30.176 in Ack 17
19:26:30.421 chunk 17 32591 bytes
19:26:30.422 fetch POST /api/observability/v1/traces
19:26:30.422 fetched POST /api/observability/v1/traces 204
19:26:30.423 in Ack 17
19:26:30.429 in Interrupt 21
19:26:30.431 rpc 27 subscribeVcsStatus
19:26:30.478 chunk 27 1410 bytes
19:26:30.478 in Ack 27
19:26:30.672 chunk 17 33277 bytes
19:26:30.673 in Ack 17
19:26:30.683 rpc 28 server.reportClientActivity
19:26:30.817 exit 28 {"_tag":"Success","value":null}
19:26:31.159 chunk 17 35208 bytes
19:26:31.160 in Ack 17
19:26:31.426 fetch POST /api/observability/v1/traces
19:26:31.426 fetched POST /api/observability/v1/traces 204
19:26:31.807 rpc 29 orchestration.dispatchCommand message.dispatch
19:26:32.674 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:26:32.808 fetch POST /api/observability/v1/traces
19:26:32.808 fetched POST /api/observability/v1/traces 204
19:26:32.860 in Ping 
19:26:32.917 chunk 17 37642 bytes
19:26:32.918 in Ack 17
19:26:33.168 chunk 17 37654 bytes
19:26:33.169 in Ack 17
19:26:33.418 chunk 17 37676 bytes
19:26:33.419 in Ack 17
19:26:33.718 chunk 17 37650 bytes
19:26:33.720 in Ack 17
19:26:33.919 fetch POST /api/observability/v1/traces
19:26:33.919 fetched POST /api/observability/v1/traces 204
19:26:33.968 chunk 17 37660 bytes
19:26:33.969 in Ack 17
19:26:34.217 chunk 11 2892 bytes
19:26:34.217 in Ack 11
19:26:34.264 chunk 17 37692 bytes
19:26:34.267 in Ack 17
19:26:34.514 chunk 17 38387 bytes
19:26:34.515 in Ack 17
19:26:34.764 chunk 17 38421 bytes
19:26:34.766 in Ack 17
19:26:34.970 fetch POST /api/observability/v1/traces
19:26:34.970 fetched POST /api/observability/v1/traces 204
19:26:35.017 chunk 17 38433 bytes
19:26:35.018 in Ack 17
19:26:35.267 chunk 17 39299 bytes
19:26:35.268 in Ack 17
19:26:35.518 chunk 17 39321 bytes
19:26:35.519 in Ack 17
19:26:35.869 chunk 17 39345 bytes
19:26:35.870 in Ack 17
19:26:36.019 fetch POST /api/observability/v1/traces
19:26:36.020 fetched POST /api/observability/v1/traces 204
19:26:36.120 chunk 17 41368 bytes
19:26:36.121 in Ack 17
19:26:36.370 chunk 17 41398 bytes
19:26:36.372 in Ack 17
19:26:36.621 chunk 17 41444 bytes
19:26:36.622 in Ack 17
19:26:36.975 chunk 17 41484 bytes
19:26:36.976 in Ack 17
19:26:37.123 fetch POST /api/observability/v1/traces
19:26:37.123 fetched POST /api/observability/v1/traces 204
19:26:37.225 chunk 17 42280 bytes
19:26:37.227 in Ack 17
19:26:37.475 chunk 17 42302 bytes
19:26:37.477 in Ack 17
19:26:37.725 chunk 17 42989 bytes
19:26:37.727 in Ack 17
19:26:37.860 in Ping 
19:26:37.976 chunk 17 43019 bytes
19:26:37.977 in Ack 17
19:26:38.227 fetch POST /api/observability/v1/traces
19:26:38.227 fetched POST /api/observability/v1/traces 204
19:26:38.285 chunk 17 43049 bytes
19:26:38.286 in Ack 17
19:26:38.536 chunk 17 43059 bytes
19:26:38.537 in Ack 17
19:26:38.786 chunk 17 43605 bytes
19:26:38.787 in Ack 17
19:26:39.037 chunk 17 43625 bytes
19:26:39.038 in Ack 17
19:26:39.287 fetch POST /api/observability/v1/traces
19:26:39.287 fetched POST /api/observability/v1/traces 204
19:26:39.288 chunk 17 43663 bytes
19:26:39.289 in Ack 17
19:26:39.538 chunk 17 43683 bytes
19:26:39.539 in Ack 17
19:26:39.563 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/8809aa6a-6e1d-4c48-9646-c89d262c3227","expiresAt":1794169599563}}
19:26:39.790 chunk 17 43703 bytes
19:26:39.791 in Ack 17
19:26:39.923 rpc 30 orchestration.dispatchCommand thread.visit
19:26:39.923 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:26:40.038 chunk 17 43743 bytes
19:26:40.039 in Ack 17
19:26:40.288 chunk 17 43763 bytes
19:26:40.289 in Ack 17
19:26:40.298 fetch POST /api/observability/v1/traces
19:26:40.298 fetched POST /api/observability/v1/traces 204
19:26:40.538 chunk 17 43805 bytes
19:26:40.539 in Ack 17
19:26:40.789 chunk 17 43815 bytes
19:26:40.789 in Ack 17
19:26:41.038 chunk 17 43835 bytes
19:26:41.039 in Ack 17
19:26:41.289 chunk 17 43855 bytes
19:26:41.290 in Ack 17
19:26:41.539 chunk 17 43905 bytes
19:26:41.540 in Ack 17
19:26:41.549 fetch POST /api/observability/v1/traces
19:26:41.549 fetched POST /api/observability/v1/traces 204
19:26:41.790 chunk 17 43915 bytes
19:26:41.791 in Ack 17
19:26:42.041 chunk 17 43937 bytes
19:26:42.043 in Ack 17
19:26:42.291 chunk 17 43975 bytes
19:26:42.292 in Ack 17
19:26:42.541 chunk 17 43995 bytes
19:26:42.543 in Ack 17
19:26:42.791 chunk 11 2892 bytes
19:26:42.791 in Ack 11
19:26:42.797 fetch POST /api/observability/v1/traces
19:26:42.797 fetched POST /api/observability/v1/traces 204
19:26:42.798 chunk 17 44027 bytes
19:26:42.799 in Ack 17
19:26:42.860 in Ping 
19:26:43.049 chunk 17 44037 bytes
19:26:43.051 in Ack 17
19:26:43.299 chunk 17 44049 bytes
19:26:43.301 in Ack 17
19:26:43.550 chunk 17 44079 bytes
19:26:43.551 in Ack 17
19:26:43.799 chunk 17 44099 bytes
19:26:43.800 in Ack 17
19:26:43.809 fetch POST /api/observability/v1/traces
19:26:43.809 fetched POST /api/observability/v1/traces 204
19:26:44.050 chunk 17 45251 bytes
19:26:44.051 in Ack 17
19:26:44.292 chunk 11 2892 bytes
19:26:44.293 in Ack 11
19:26:44.299 chunk 17 45281 bytes
19:26:44.300 in Ack 17
19:26:44.550 chunk 17 45326 bytes
19:26:44.552 in Ack 17
19:26:44.800 chunk 17 45356 bytes
19:26:44.801 in Ack 17
19:26:45.052 chunk 17 45401 bytes
19:26:45.053 fetch POST /api/observability/v1/traces
19:26:45.053 fetched POST /api/observability/v1/traces 204
19:26:45.054 in Ack 17
19:26:45.300 chunk 17 45431 bytes
19:26:45.301 in Ack 17
19:26:45.551 chunk 17 45476 bytes
19:26:45.552 in Ack 17
19:26:45.802 chunk 17 45506 bytes
19:26:45.803 in Ack 17
19:26:46.052 chunk 17 45551 bytes
19:26:46.053 in Ack 17
19:26:46.063 fetch POST /api/observability/v1/traces
19:26:46.063 fetched POST /api/observability/v1/traces 204
19:26:46.302 chunk 17 45596 bytes
19:26:46.303 in Ack 17
19:26:46.552 chunk 17 45626 bytes
19:26:46.554 in Ack 17
19:26:46.802 chunk 17 45656 bytes
19:26:46.803 in Ack 17
19:26:47.052 chunk 17 45686 bytes
19:26:47.053 in Ack 17
19:26:47.303 chunk 17 45731 bytes
19:26:47.304 in Ack 17
19:26:47.313 fetch POST /api/observability/v1/traces
19:26:47.313 fetched POST /api/observability/v1/traces 204
19:26:47.554 chunk 17 45761 bytes
19:26:47.555 in Ack 17
19:26:47.803 chunk 17 45821 bytes
19:26:47.804 in Ack 17
19:26:47.861 in Ping 
19:26:48.053 chunk 17 45851 bytes
19:26:48.054 in Ack 17
19:26:48.101 rpc 31 server.reportClientActivity
19:26:48.303 chunk 17 45881 bytes
19:26:48.304 in Ack 17
19:26:48.553 chunk 17 45911 bytes
19:26:48.554 in Ack 17
19:26:48.565 fetch POST /api/observability/v1/traces
19:26:48.565 fetched POST /api/observability/v1/traces 204
19:26:48.804 chunk 17 45956 bytes
19:26:48.805 in Ack 17
19:26:49.053 chunk 17 45986 bytes
19:26:49.054 in Ack 17
19:26:49.303 chunk 17 46031 bytes
19:26:49.304 in Ack 17
19:26:49.554 chunk 17 46061 bytes
19:26:49.555 in Ack 17
19:26:49.806 chunk 17 46091 bytes
19:26:49.806 fetch POST /api/observability/v1/traces
19:26:49.806 fetched POST /api/observability/v1/traces 204
19:26:49.807 in Ack 17
19:26:49.849 exit 31 {"_tag":"Success","value":null}
19:26:49.924 rpc 32 orchestration.dispatchCommand thread.visit
19:26:49.924 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:26:50.062 chunk 17 46121 bytes
19:26:50.064 in Ack 17
19:26:50.313 chunk 17 46181 bytes
19:26:50.314 in Ack 17
19:26:50.563 chunk 17 46211 bytes
19:26:50.564 in Ack 17
19:26:50.808 fetch POST /api/observability/v1/traces
19:26:50.808 fetched POST /api/observability/v1/traces 204
19:26:50.814 chunk 17 46241 bytes
19:26:50.815 in Ack 17
19:26:51.050 chunk 11 2892 bytes
19:26:51.050 in Ack 11
19:26:51.064 chunk 17 46286 bytes
19:26:51.065 in Ack 17
19:26:51.314 chunk 17 46316 bytes
19:26:51.315 in Ack 17
19:26:51.564 chunk 17 46361 bytes
19:26:51.565 in Ack 17
19:26:51.815 chunk 17 46391 bytes
19:26:51.816 in Ack 17
19:26:51.824 fetch POST /api/observability/v1/traces
19:26:51.824 fetched POST /api/observability/v1/traces 204
19:26:52.065 chunk 17 46436 bytes
19:26:52.066 in Ack 17
19:26:52.315 chunk 17 46466 bytes
19:26:52.315 in Ack 17
19:26:52.564 chunk 17 46496 bytes
19:26:52.565 in Ack 17
19:26:52.816 chunk 17 46526 bytes
19:26:52.817 in Ack 17
19:26:52.861 in Ping 
19:26:53.066 chunk 17 46556 bytes
19:26:53.066 fetch POST /api/observability/v1/traces
19:26:53.066 fetched POST /api/observability/v1/traces 204
19:26:53.067 in Ack 17
19:26:53.348 chunk 17 46586 bytes
19:26:53.349 in Ack 17
19:26:53.599 chunk 17 46616 bytes
19:26:53.600 in Ack 17
19:26:53.849 chunk 17 46661 bytes
19:26:53.850 in Ack 17
19:26:54.068 fetch POST /api/observability/v1/traces
19:26:54.068 fetched POST /api/observability/v1/traces 204
19:26:54.100 chunk 17 46721 bytes
19:26:54.101 in Ack 17
19:26:54.350 chunk 17 46766 bytes
19:26:54.351 in Ack 17
19:26:54.601 chunk 17 46826 bytes
19:26:54.602 in Ack 17
19:26:54.852 chunk 17 46871 bytes
19:26:54.853 in Ack 17
19:26:55.102 fetch POST /api/observability/v1/traces
19:26:55.102 fetched POST /api/observability/v1/traces 204
19:26:55.103 chunk 17 46901 bytes
19:26:55.103 in Ack 17
19:26:55.352 chunk 17 46931 bytes
19:26:55.353 in Ack 17
19:26:55.603 chunk 17 46976 bytes
19:26:55.604 in Ack 17
19:26:55.854 chunk 17 47006 bytes
19:26:55.855 in Ack 17
19:26:56.104 chunk 17 47051 bytes
19:26:56.104 fetch POST /api/observability/v1/traces
19:26:56.104 fetched POST /api/observability/v1/traces 204
19:26:56.105 in Ack 17
19:26:56.354 chunk 17 47081 bytes
19:26:56.355 in Ack 17
19:26:56.605 chunk 17 47126 bytes
19:26:56.606 in Ack 17
19:26:56.856 chunk 17 47156 bytes
19:26:56.857 in Ack 17
19:26:57.107 fetch POST /api/observability/v1/traces
19:26:57.107 fetched POST /api/observability/v1/traces 204
19:26:57.107 chunk 17 47186 bytes
19:26:57.108 in Ack 17
19:26:57.358 chunk 17 47216 bytes
19:26:57.359 in Ack 17
19:26:57.608 chunk 17 47261 bytes
19:26:57.609 in Ack 17
19:26:57.858 chunk 17 47306 bytes
19:26:57.859 in Ack 17
19:26:57.867 in Ping 
19:26:58.108 chunk 17 47336 bytes
19:26:58.109 in Ack 17
19:26:58.118 fetch POST /api/observability/v1/traces
19:26:58.118 fetched POST /api/observability/v1/traces 204
19:26:58.358 chunk 17 47366 bytes
19:26:58.359 in Ack 17
19:26:58.609 chunk 17 47411 bytes
19:26:58.610 in Ack 17
19:26:58.859 chunk 17 47441 bytes
19:26:58.862 in Ack 17
19:26:59.109 chunk 17 47471 bytes
19:26:59.110 in Ack 17
19:26:59.359 chunk 17 47501 bytes
19:26:59.360 in Ack 17
19:26:59.367 fetch POST /api/observability/v1/traces
19:26:59.367 fetched POST /api/observability/v1/traces 204
19:26:59.610 chunk 17 47546 bytes
19:26:59.610 in Ack 17
19:26:59.859 chunk 17 47576 bytes
19:26:59.860 in Ack 17
19:27:00.109 chunk 17 47603 bytes
19:27:00.111 in Ack 17
19:27:00.361 chunk 17 48608 bytes
19:27:00.364 in Ack 17
19:27:00.613 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:27:00.660 fetch POST /api/observability/v1/traces
19:27:00.660 fetched POST /api/observability/v1/traces 204
19:27:00.862 chunk 17 52104 bytes
19:27:00.863 in Ack 17
19:27:01.368 chunk 17 54180 bytes
19:27:01.370 in Ack 17
19:27:01.618 chunk 11 3200 bytes
19:27:01.619 chunk 17 54230 bytes
19:27:01.619 in Ack 11
19:27:01.627 in Ack 17
19:27:01.643 exit 33 {"_tag":"Success","value":{"sequence":97}}
19:27:01.644 rpc 34 orchestration.dispatchCommand thread.visit
19:27:01.644 exit 34 {"_tag":"Success","value":{"sequence":0}}
19:27:01.864 fetch POST /api/observability/v1/traces
19:27:01.864 fetched POST /api/observability/v1/traces 204
19:27:01.870 chunk 17 56713 bytes
19:27:01.871 in Ack 17
19:27:01.920 rpc 35 orchestration.dispatchCommand runtime-request.respond
19:27:02.120 chunk 17 56733 bytes
19:27:02.122 in Ack 17
19:27:02.370 chunk 17 56755 bytes
19:27:02.372 in Ack 17
19:27:02.621 chunk 17 56787 bytes
19:27:02.622 in Ack 17
19:27:02.868 in Ping 
19:27:02.871 chunk 17 56749 bytes
19:27:02.872 in Ack 17
19:27:02.881 fetch POST /api/observability/v1/traces
19:27:02.881 fetched POST /api/observability/v1/traces 204
19:27:02.967 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:27:03.179 chunk 11 2894 bytes
19:27:03.179 chunk 17 56813 bytes
19:27:03.181 in Ack 11
19:27:03.188 in Ack 17
19:27:03.297 rpc 36 orchestration.dispatchCommand runtime-request.respond
19:27:03.429 chunk 17 56843 bytes
19:27:03.430 in Ack 17
19:27:03.680 chunk 17 56863 bytes
19:27:03.681 in Ack 17
19:27:03.930 chunk 17 56974 bytes
19:27:03.932 in Ack 17
19:27:03.940 in Interrupt 27
19:27:03.960 rpc 37 subscribeVcsStatus
19:27:04.007 fetch POST /api/observability/v1/traces
19:27:04.007 fetched POST /api/observability/v1/traces 204
19:27:04.210 rpc 38 server.reportClientActivity
19:27:04.419 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:27:04.439 chunk 17 58054 bytes
19:27:04.440 in Ack 17
19:27:04.689 chunk 17 58084 bytes
19:27:04.691 in Ack 17
19:27:05.190 chunk 17 58124 bytes
19:27:05.191 in Ack 17
19:27:05.214 fetch POST /api/observability/v1/traces
19:27:05.214 fetched POST /api/observability/v1/traces 204
19:27:05.410 chunk 37 1410 bytes
19:27:05.411 in Ack 37
19:27:05.441 chunk 17 58144 bytes
19:27:05.442 in Ack 17
19:27:05.691 chunk 17 58174 bytes
19:27:05.692 in Ack 17
19:27:05.941 chunk 17 58194 bytes
19:27:05.942 in Ack 17
19:27:06.192 chunk 17 60654 bytes
19:27:06.193 in Ack 17
19:27:06.442 fetch POST /api/observability/v1/traces
19:27:06.442 fetched POST /api/observability/v1/traces 204
19:27:06.443 chunk 17 60684 bytes
19:27:06.444 in Ack 17
19:27:06.694 chunk 17 60715 bytes
19:27:06.695 in Ack 17
19:27:06.864 exit 38 {"_tag":"Success","value":null}
19:27:06.962 chunk 17 60745 bytes
19:27:06.964 in Ack 17
19:27:07.213 chunk 17 60765 bytes
19:27:07.214 in Ack 17
19:27:07.445 fetch POST /api/observability/v1/traces
19:27:07.445 fetched POST /api/observability/v1/traces 204
19:27:07.464 chunk 17 60785 bytes
19:27:07.465 in Ack 17
19:27:07.714 chunk 17 60815 bytes
19:27:07.716 in Ack 17
19:27:07.868 in Ping 
19:27:07.973 chunk 17 60835 bytes
19:27:07.974 in Ack 17
19:27:08.224 chunk 17 60865 bytes
19:27:08.225 in Ack 17
19:27:08.466 fetch POST /api/observability/v1/traces
19:27:08.466 fetched POST /api/observability/v1/traces 204
19:27:08.474 chunk 17 60885 bytes
19:27:08.476 in Ack 17
19:27:08.724 chunk 17 60905 bytes
19:27:08.725 in Ack 17
19:27:08.974 chunk 17 60925 bytes
19:27:08.976 in Ack 17
19:27:09.224 chunk 17 60945 bytes
19:27:09.226 in Ack 17
19:27:09.475 chunk 17 60967 bytes
19:27:09.476 in Ack 17
19:27:09.502 fetch POST /api/observability/v1/traces
19:27:09.502 fetched POST /api/observability/v1/traces 204
19:27:09.726 chunk 17 61961 bytes
19:27:09.727 in Ack 17
19:27:09.976 chunk 17 62021 bytes
19:27:09.977 in Ack 17
19:27:10.227 chunk 17 62051 bytes
19:27:10.229 in Ack 17
19:27:10.478 chunk 17 62066 bytes
19:27:10.479 in Ack 17
19:27:10.728 fetch POST /api/observability/v1/traces
19:27:10.728 fetched POST /api/observability/v1/traces 204
19:27:10.728 chunk 11 2894 bytes
19:27:10.729 chunk 17 62111 bytes
19:27:10.729 in Ack 11
19:27:10.731 in Ack 17
19:27:10.979 chunk 17 62147 bytes
19:27:10.981 in Ack 17
19:27:11.229 chunk 17 62192 bytes
19:27:11.231 in Ack 17
19:27:11.481 chunk 17 62228 bytes
19:27:11.482 in Ack 17
19:27:11.634 rpc 39 orchestration.dispatchCommand thread.visit
19:27:11.634 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:27:11.730 fetch POST /api/observability/v1/traces
19:27:11.730 fetched POST /api/observability/v1/traces 204
19:27:11.731 chunk 17 62258 bytes
19:27:11.732 in Ack 17
19:27:11.982 chunk 17 62288 bytes
19:27:11.983 in Ack 17
19:27:12.232 chunk 17 62339 bytes
19:27:12.233 in Ack 17
19:27:12.482 chunk 17 62369 bytes
19:27:12.483 in Ack 17
19:27:12.732 chunk 17 62414 bytes
19:27:12.733 in Ack 17
19:27:12.772 fetch POST /api/observability/v1/traces
19:27:12.772 fetched POST /api/observability/v1/traces 204
19:27:12.869 in Ping 
19:27:12.983 chunk 17 62429 bytes
19:27:12.984 in Ack 17
19:27:13.101 rpc 40 server.reportClientActivity
19:27:13.232 chunk 17 62498 bytes
19:27:13.234 in Ack 17
19:27:13.482 chunk 17 62573 bytes
19:27:13.483 in Ack 17
19:27:13.733 chunk 17 62615 bytes
19:27:13.734 in Ack 17
19:27:13.983 chunk 11 2894 bytes
19:27:13.983 chunk 17 62684 bytes
19:27:13.983 in Ack 11
19:27:13.989 in Ack 17
19:27:14.088 fetch POST /api/observability/v1/traces
19:27:14.088 fetched POST /api/observability/v1/traces 204
19:27:14.234 chunk 17 62714 bytes
19:27:14.235 in Ack 17
19:27:14.484 chunk 17 62765 bytes
19:27:14.485 in Ack 17
19:27:14.734 chunk 17 62816 bytes
19:27:14.735 in Ack 17
19:27:14.898 exit 40 {"_tag":"Success","value":null}
19:27:14.984 chunk 17 62849 bytes
19:27:14.985 in Ack 17
19:27:15.235 chunk 17 62897 bytes
19:27:15.237 in Ack 17
19:27:15.302 fetch POST /api/observability/v1/traces
19:27:15.302 fetched POST /api/observability/v1/traces 204
19:27:15.484 chunk 17 62995 bytes
19:27:15.486 in Ack 17
