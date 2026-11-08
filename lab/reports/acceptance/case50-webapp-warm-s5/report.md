# case50, webapp, warm start, seed 5

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 48 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 2.7s |
| Prompt reached the agent | 2.2s |
| Time to first text | 4.2s |
| Whole reply shown | 47.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.5s / 1.0s / 2.2s |
| Longest silence | 4.9s |
| Approval shown after it was asked | 2.0s |
| Bytes on the wire down / up | 400277 / 136305 |
| Bytes the app received / sent | 331629 / 73127 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.6 | 247 | 311 |
| 20 | 0.7 | 681 | 710 |
| 30 | 1.0 | 1073 | 1077 |
| 40 | 0.9 | 1363 | 1393 |
| 47 | 1.5 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 546/546 163/901 68/546","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 419/774","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":271,"lastDataAt":1794165710846,"silences":0,"longestSilenceMs":1172,"longestRecoveredGapMs":1172},"stats":{"sendRate":3000,"deliveredRate":1868.2242990654204,"loss":0,"srttMs":55.51487300325952,"minRttMs":1,"sentPackets":91,"sentWireBytes":30367,"receivedPackets":106,"receivedWireBytes":53749,"lastHeardMs":9235,"unsyncedBytes":1009,"inflightPackets":6,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 233/839","messages":"456/456 546/546 261/901 107/546","assistant":"32/32 21/21 360/370","user":"10/10 9/9 59/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 689/774 52/656","providerThreads":"808/808","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 192/1019 64/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":271,"lastDataAt":1794165720859,"silences":0,"longestSilenceMs":1371,"longestRecoveredGapMs":1371},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.40188076923076915,"srttMs":261.0539090627305,"minRttMs":662,"sentPackets":146,"sentWireBytes":44895,"receivedPackets":167,"receivedWireBytes":76495,"lastHeardMs":19248,"unsyncedBytes":167,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 545/545 451/451","assistant":"32/32 21/21 878/878","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":271,"lastDataAt":1794165731071,"silences":0,"longestSilenceMs":1371,"longestRecoveredGapMs":1371},"stats":{"sendRate":4500,"deliveredRate":0,"loss":0.5346491228070175,"srttMs":668.8809060313505,"minRttMs":662,"sentPackets":220,"sentWireBytes":60695,"receivedPackets":309,"receivedWireBytes":144037,"lastHeardMs":29460,"unsyncedBytes":167,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849 822/1546 926/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 545/545 451/451","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"962/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":271,"lastDataAt":1794165741108,"silences":0,"longestSilenceMs":1371,"longestRecoveredGapMs":1371},"stats":{"sendRate":6750,"deliveredRate":741.5302823954794,"loss":0.515,"srttMs":734.8603008792204,"minRttMs":662,"sentPackets":271,"sentWireBytes":68232,"receivedPackets":635,"receivedWireBytes":231288,"lastHeardMs":39496,"unsyncedBytes":0,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 1098/1098 849/849 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","messages":"456/456 546/546 901/901 546/546 546/546 451/451 446/545","assistant":"32/32 21/21 1340/1340 469/469","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":271,"lastDataAt":1794165751976,"silences":0,"longestSilenceMs":1371,"longestRecoveredGapMs":1371},"stats":{"sendRate":2582.0933892728117,"deliveredRate":1242.003148540334,"loss":0.41068151449275353,"srttMs":822.745737274704,"minRttMs":605,"sentPackets":353,"sentWireBytes":90213,"receivedPackets":784,"receivedWireBytes":329506,"lastHeardMs":50365,"unsyncedBytes":0,"inflightPackets":69,"duplicateBytes":0}}
- shim trace
19:21:41.925 fetch GET /api/auth/session
19:21:41.927 fetched GET /api/auth/session 200
19:21:42.059 fetch GET /.well-known/t3/environment
19:21:42.059 fetched GET /.well-known/t3/environment 200
19:21:42.069 fetch GET /.well-known/t3/environment
19:21:42.079 fetched GET /.well-known/t3/environment 200
19:21:42.083 rpc 0 subscribeServerConfig
19:21:42.083 chunk 0 38722 bytes
19:21:42.112 in Ack 0
19:21:42.114 rpc 1 subscribeServerLifecycle
19:21:42.114 rpc 2 subscribeServerLifecycle
19:21:42.117 fetch GET /api/auth/session
19:21:42.119 fetched GET /api/auth/session 200
19:21:42.119 rpc 3 subscribeDeviceState
19:21:42.121 rpc 4 preview.reportProfiles
19:21:42.145 rpc 5 subscribeProjectClones
19:21:42.163 fetch GET /api/orchestration/threads/mcp%3A6bbe6da0-6644-4cd2-98be-d5c922c842fe/bounded
19:21:42.167 fetch GET /api/orchestration/shell
19:21:42.168 fetched GET /api/orchestration/shell 200
19:21:42.169 rpc 6 orchestration.subscribeShell
19:21:42.169 chunk 6 2790 bytes
19:21:42.188 rpc 7 server.refreshProviders
19:21:42.230 rpc 8 orchestration.dispatchCommand thread.visit
19:21:42.230 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:21:42.243 rpc 9 subscribeTerminalMetadata
19:21:42.244 rpc 10 subscribeVcsStatus
19:21:42.245 rpc 11 subscribePreviewEvents
19:21:42.245 rpc 12 assets.createUrl
19:21:42.246 rpc 13 preview.list
19:21:42.247 rpc 14 projects.readFile
19:21:42.251 in Ack 6
19:21:42.370 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"55f6cd69-7e4f-482b-bc42-ad23ed363d40","sessions":[]}}
19:21:42.370 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:21:42.420 chunk 6 2766 bytes
19:21:42.420 in Ack 6
19:21:42.488 chunk 1 3872 bytes
19:21:42.490 in Ack 1
19:21:42.497 rpc 15 server.reportClientActivity
19:21:42.859 chunk 2 3872 bytes
19:21:42.859 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:21:30.567Z","configurableProviders":false,"con
19:21:42.859 chunk 10 1066 bytes
19:21:42.860 in Ack 2
19:21:42.862 in Ack 10
19:21:42.978 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:21:43.057 fetch POST /api/observability/v1/traces
19:21:43.057 fetched POST /api/observability/v1/traces 204
19:21:43.168 chunk 3 876 bytes
19:21:43.168 exit 4 {"_tag":"Success","value":null}
19:21:43.168 chunk 5 4 bytes
19:21:43.168 chunk 9 36 bytes
19:21:43.168 exit 15 {"_tag":"Success","value":null}
19:21:43.169 in Ack 3
19:21:43.172 rpc 16 preview.reportProfiles
19:21:43.172 in Ack 5
19:21:43.173 in Ack 9
19:21:43.228 chunk 6 2809 bytes
19:21:43.229 in Ack 6
19:21:43.253 exit 16 {"_tag":"Success","value":null}
19:21:43.319 fetched GET /api/orchestration/threads/mcp%3A6bbe6da0-6644-4cd2-98be-d5c922c842fe/bounded 200
19:21:43.322 rpc 17 orchestration.subscribeThread
19:21:43.322 chunk 17 8283 bytes
19:21:43.331 in Interrupt 10
19:21:43.333 rpc 18 subscribeVcsStatus
19:21:43.334 in Ack 17
19:21:43.449 chunk 18 1066 bytes
19:21:43.449 in Ack 18
19:21:43.573 chunk 17 11920 bytes
19:21:43.575 in Ack 17
19:21:43.641 rpc 19 server.reportClientActivity
19:21:43.840 chunk 17 13502 bytes
19:21:43.855 in Ack 17
19:21:44.008 exit 19 {"_tag":"Success","value":null}
19:21:44.091 chunk 17 19072 bytes
19:21:44.093 in Ack 17
19:21:44.148 rpc 20 attachments.createUploadUrl
19:21:44.308 fetch POST /api/observability/v1/traces
19:21:44.308 fetched POST /api/observability/v1/traces 204
19:21:44.341 chunk 17 19851 bytes
19:21:44.342 in Ack 17
19:21:44.350 in Interrupt 18
19:21:44.352 rpc 21 subscribeVcsStatus
19:21:44.420 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-2aeaf02e-cb8e-4794-843a-67b489601d94","expiresAt":1794166304328,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:21:44.420 chunk 21 1066 bytes
19:21:44.421 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTJhZWFmMDJlLWNiOGUtNDc5NC04NDNhLTY3YjQ4OTYwMWQ5NCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjMwNDMyOH0.Onq1folNWMUpjP-wo1uBHt2jiCtmXHTo9mCdY-IiqAQ
19:21:44.421 in Ack 21
19:21:44.591 chunk 17 21199 bytes
19:21:44.592 in Ack 17
19:21:44.604 rpc 22 server.reportClientActivity
19:21:45.343 fetch POST /api/observability/v1/traces
19:21:45.343 fetched POST /api/observability/v1/traces 204
19:21:45.467 exit 22 {"_tag":"Success","value":null}
19:21:46.468 fetch POST /api/observability/v1/traces
19:21:46.468 fetched POST /api/observability/v1/traces 204
19:21:47.081 in Ping 
19:21:47.314 rpc 23 orchestration.dispatchCommand message.dispatch
19:21:47.889 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:21:47.890 rpc 24 attachments.delete
19:21:48.313 fetch POST /api/observability/v1/traces
19:21:48.313 fetched POST /api/observability/v1/traces 204
19:21:48.391 chunk 17 23653 bytes
19:21:48.392 in Ack 17
19:21:48.642 chunk 17 23665 bytes
19:21:48.643 in Ack 17
19:21:48.880 exit 24 {"_tag":"Success","value":null}
19:21:48.893 chunk 17 23667 bytes
19:21:48.894 in Ack 17
19:21:49.143 chunk 6 2831 bytes
19:21:49.143 chunk 17 24639 bytes
19:21:49.143 in Ack 6
19:21:49.145 rpc 25 orchestration.dispatchCommand thread.visit
19:21:49.145 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:21:49.179 in Ack 17
19:21:49.392 chunk 17 28093 bytes
19:21:49.394 in Ack 17
19:21:49.396 rpc 26 assets.createUrl
19:21:49.429 fetch POST /api/observability/v1/traces
19:21:49.429 fetched POST /api/observability/v1/traces 204
19:21:49.642 chunk 6 2831 bytes
19:21:49.642 chunk 17 31326 bytes
19:21:49.643 in Ack 6
19:21:49.647 in Ack 17
19:21:49.894 chunk 17 31940 bytes
19:21:50.135 in Ack 17
19:21:50.142 in Interrupt 21
19:21:50.144 rpc 27 subscribeVcsStatus
19:21:50.398 rpc 28 server.reportClientActivity
19:21:50.399 chunk 17 31940 bytes
19:21:50.400 in Ack 17
19:21:50.657 fetch POST /api/observability/v1/traces
19:21:50.657 fetched POST /api/observability/v1/traces 204
19:21:50.813 rpc 29 orchestration.dispatchCommand message.dispatch
19:21:50.847 chunk 27 1066 bytes
19:21:50.848 in Ack 27
19:21:51.814 fetch POST /api/observability/v1/traces
19:21:51.815 fetched POST /api/observability/v1/traces 204
19:21:52.082 in Ping 
19:21:53.364 chunk 17 34374 bytes
19:21:53.365 exit 29 {"_tag":"Success","value":{"sequence":75}}
19:21:53.366 in Ack 17
19:21:53.676 fetch POST /api/observability/v1/traces
19:21:53.676 fetched POST /api/observability/v1/traces 204
19:21:54.519 chunk 17 34416 bytes
19:21:54.520 in Ack 17
19:21:54.785 chunk 17 34504 bytes
19:21:54.787 in Ack 17
19:21:54.872 fetch POST /api/observability/v1/traces
19:21:54.872 fetched POST /api/observability/v1/traces 204
19:21:55.165 chunk 17 34516 bytes
19:21:55.166 in Ack 17
19:21:55.416 chunk 17 34528 bytes
19:21:55.417 in Ack 17
19:21:55.667 chunk 17 34558 bytes
19:21:55.668 in Ack 17
19:21:55.917 chunk 17 34598 bytes
19:21:55.918 in Ack 17
19:21:56.167 chunk 17 34634 bytes
19:21:56.168 fetch POST /api/observability/v1/traces
19:21:56.168 fetched POST /api/observability/v1/traces 204
19:21:56.169 in Ack 17
19:21:56.417 chunk 17 34654 bytes
19:21:56.418 in Ack 17
19:21:56.667 chunk 17 34636 bytes
19:21:56.668 in Ack 17
19:21:56.917 chunk 17 34648 bytes
19:21:56.918 in Ack 17
19:21:57.082 in Ping 
19:21:57.167 chunk 17 34660 bytes
19:21:57.168 in Ack 17
19:21:57.177 fetch POST /api/observability/v1/traces
19:21:57.177 fetched POST /api/observability/v1/traces 204
19:21:57.417 chunk 17 34680 bytes
19:21:57.418 in Ack 17
19:21:57.667 chunk 17 34690 bytes
19:21:57.668 in Ack 17
19:21:57.917 chunk 17 34750 bytes
19:21:57.918 in Ack 17
19:21:58.167 chunk 17 34778 bytes
19:21:58.169 in Ack 17
19:21:58.418 chunk 17 34790 bytes
19:21:58.419 in Ack 17
19:21:58.427 fetch POST /api/observability/v1/traces
19:21:58.427 fetched POST /api/observability/v1/traces 204
19:21:58.668 chunk 17 34828 bytes
19:21:58.669 in Ack 17
19:21:58.919 chunk 17 34848 bytes
19:21:58.920 in Ack 17
19:21:59.177 chunk 17 34868 bytes
19:21:59.180 in Ack 17
19:21:59.428 chunk 17 34900 bytes
19:21:59.429 in Ack 17
19:21:59.670 fetch POST /api/observability/v1/traces
19:21:59.670 fetched POST /api/observability/v1/traces 204
19:21:59.678 chunk 17 34910 bytes
19:21:59.679 in Ack 17
19:21:59.929 chunk 17 34950 bytes
19:21:59.930 in Ack 17
19:22:00.179 chunk 17 34962 bytes
19:22:00.179 in Ack 17
19:22:00.428 chunk 6 2895 bytes
19:22:00.428 chunk 17 34982 bytes
19:22:00.429 in Ack 6
19:22:00.430 rpc 30 orchestration.dispatchCommand thread.visit
19:22:00.430 exit 30 {"_tag":"Success","value":{"sequence":0}}
19:22:00.464 in Ack 17
19:22:00.679 chunk 17 35657 bytes
19:22:00.680 in Ack 17
19:22:00.688 fetch POST /api/observability/v1/traces
19:22:00.688 fetched POST /api/observability/v1/traces 204
19:22:00.930 chunk 17 35677 bytes
19:22:00.931 in Ack 17
19:22:01.180 chunk 17 37718 bytes
19:22:01.181 in Ack 17
19:22:01.430 chunk 17 37730 bytes
19:22:01.431 in Ack 17
19:22:01.680 chunk 17 37742 bytes
19:22:01.681 in Ack 17
19:22:01.931 chunk 17 37772 bytes
19:22:01.931 fetch POST /api/observability/v1/traces
19:22:01.931 fetched POST /api/observability/v1/traces 204
19:22:01.932 in Ack 17
19:22:02.082 in Ping 
19:22:02.182 chunk 17 37794 bytes
19:22:02.183 in Ack 17
19:22:02.432 chunk 17 37856 bytes
19:22:02.433 in Ack 17
19:22:02.683 chunk 17 37876 bytes
19:22:02.684 in Ack 17
19:22:02.934 fetch POST /api/observability/v1/traces
19:22:02.934 fetched POST /api/observability/v1/traces 204
19:22:02.935 chunk 17 37898 bytes
19:22:02.936 in Ack 17
19:22:03.185 chunk 17 37920 bytes
19:22:03.187 in Ack 17
19:22:03.436 chunk 17 37932 bytes
19:22:03.437 in Ack 17
19:22:03.686 chunk 17 37944 bytes
19:22:03.687 in Ack 17
19:22:03.936 chunk 17 37966 bytes
19:22:03.937 in Ack 17
19:22:03.944 fetch POST /api/observability/v1/traces
19:22:03.944 fetched POST /api/observability/v1/traces 204
19:22:04.186 chunk 17 37986 bytes
19:22:04.187 in Ack 17
19:22:04.464 chunk 17 38846 bytes
19:22:04.466 in Ack 17
19:22:04.740 chunk 17 38866 bytes
19:22:04.743 in Ack 17
19:22:04.990 chunk 17 38890 bytes
19:22:04.991 in Ack 17
19:22:05.188 fetch POST /api/observability/v1/traces
19:22:05.188 fetched POST /api/observability/v1/traces 204
19:22:05.240 chunk 17 38926 bytes
19:22:05.242 in Ack 17
19:22:05.399 in Interrupt 28
19:22:05.491 chunk 17 39724 bytes
19:22:05.492 in Ack 17
19:22:05.742 chunk 6 2895 bytes
19:22:05.742 chunk 17 39794 bytes
19:22:05.742 in Ack 6
19:22:05.747 in Ack 17
19:22:05.992 chunk 17 41157 bytes
19:22:05.993 in Ack 17
19:22:06.241 chunk 17 41189 bytes
19:22:06.243 in Ack 17
19:22:06.250 fetch POST /api/observability/v1/traces
19:22:06.251 fetched POST /api/observability/v1/traces 204
19:22:06.492 chunk 17 41209 bytes
19:22:06.493 in Ack 17
19:22:06.742 chunk 17 41221 bytes
19:22:06.743 in Ack 17
19:22:06.992 chunk 17 43155 bytes
19:22:06.993 in Ack 17
19:22:07.082 in Ping 
19:22:07.242 chunk 17 43185 bytes
19:22:07.243 in Ack 17
19:22:07.310 rpc 31 server.reportClientActivity
19:22:07.493 chunk 17 43230 bytes
19:22:07.494 in Ack 17
19:22:07.501 fetch POST /api/observability/v1/traces
19:22:07.501 fetched POST /api/observability/v1/traces 204
19:22:07.742 chunk 17 43260 bytes
19:22:07.743 in Ack 17
19:22:07.992 chunk 17 43821 bytes
19:22:07.993 in Ack 17
19:22:08.027 exit 28 {"_tag":"Success","value":null}
19:22:08.242 chunk 17 45797 bytes
19:22:08.243 in Ack 17
19:22:08.493 chunk 17 45827 bytes
19:22:08.494 in Ack 17
19:22:08.743 chunk 17 45857 bytes
19:22:08.744 in Ack 17
19:22:08.769 fetch POST /api/observability/v1/traces
19:22:08.769 fetched POST /api/observability/v1/traces 204
19:22:08.995 chunk 17 45887 bytes
19:22:08.996 in Ack 17
19:22:09.087 exit 31 {"_tag":"Success","value":null}
19:22:09.245 chunk 17 45932 bytes
19:22:09.246 in Ack 17
19:22:09.494 chunk 17 45962 bytes
19:22:09.495 in Ack 17
19:22:09.745 chunk 17 46007 bytes
19:22:09.746 in Ack 17
19:22:09.996 chunk 17 46052 bytes
19:22:09.997 fetch POST /api/observability/v1/traces
19:22:09.997 fetched POST /api/observability/v1/traces 204
19:22:09.998 in Ack 17
19:22:10.246 chunk 17 46082 bytes
19:22:10.247 in Ack 17
19:22:10.431 rpc 32 orchestration.dispatchCommand thread.visit
19:22:10.431 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:22:10.497 chunk 17 46127 bytes
19:22:10.498 in Ack 17
19:22:10.747 chunk 17 46142 bytes
19:22:10.748 in Ack 17
19:22:10.996 chunk 17 46202 bytes
19:22:10.997 in Ack 17
19:22:11.005 fetch POST /api/observability/v1/traces
19:22:11.006 fetched POST /api/observability/v1/traces 204
19:22:11.039 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/fbe8d8fe-f981-4a6d-9e5d-2a5ae13efab7","expiresAt":1794169331039}}
19:22:11.247 chunk 17 46247 bytes
19:22:11.248 in Ack 17
19:22:11.497 chunk 17 46277 bytes
19:22:11.498 in Ack 17
19:22:11.747 chunk 17 46307 bytes
19:22:11.748 in Ack 17
19:22:11.998 chunk 17 46337 bytes
19:22:11.999 in Ack 17
19:22:12.040 fetch POST /api/observability/v1/traces
19:22:12.040 fetched POST /api/observability/v1/traces 204
19:22:12.083 in Ping 
19:22:12.248 chunk 17 46367 bytes
19:22:12.249 in Ack 17
19:22:12.498 chunk 17 46397 bytes
19:22:12.500 in Ack 17
19:22:12.747 chunk 6 2895 bytes
19:22:12.748 in Ack 6
19:22:12.752 chunk 17 46442 bytes
19:22:12.753 in Ack 17
19:22:13.003 chunk 17 46487 bytes
19:22:13.005 in Ack 17
19:22:13.250 fetch POST /api/observability/v1/traces
19:22:13.250 fetched POST /api/observability/v1/traces 204
19:22:13.254 chunk 17 46502 bytes
19:22:13.255 in Ack 17
19:22:13.504 chunk 17 46547 bytes
19:22:13.505 in Ack 17
19:22:13.755 chunk 17 46562 bytes
19:22:13.756 in Ack 17
19:22:14.041 chunk 17 46607 bytes
19:22:14.044 in Ack 17
19:22:14.257 fetch POST /api/observability/v1/traces
19:22:14.257 fetched POST /api/observability/v1/traces 204
19:22:14.291 chunk 17 46637 bytes
19:22:14.292 in Ack 17
19:22:14.541 chunk 17 46697 bytes
19:22:14.542 in Ack 17
19:22:14.791 chunk 17 46772 bytes
19:22:14.793 in Ack 17
19:22:15.041 chunk 17 46817 bytes
19:22:15.042 in Ack 17
19:22:15.292 chunk 17 46862 bytes
19:22:15.293 in Ack 17
19:22:15.315 fetch POST /api/observability/v1/traces
19:22:15.315 fetched POST /api/observability/v1/traces 204
19:22:15.541 chunk 17 46877 bytes
19:22:15.542 in Ack 17
19:22:15.791 chunk 17 46922 bytes
19:22:15.792 in Ack 17
19:22:16.042 chunk 17 46967 bytes
19:22:16.043 in Ack 17
19:22:16.292 chunk 17 46997 bytes
19:22:16.293 in Ack 17
19:22:16.542 chunk 17 47027 bytes
19:22:16.543 in Ack 17
19:22:16.551 fetch POST /api/observability/v1/traces
19:22:16.551 fetched POST /api/observability/v1/traces 204
19:22:16.793 chunk 17 47072 bytes
19:22:16.794 in Ack 17
19:22:17.043 chunk 17 47117 bytes
19:22:17.044 in Ack 17
19:22:17.083 in Ping 
19:22:17.293 chunk 17 47147 bytes
19:22:17.294 in Ack 17
19:22:17.543 chunk 17 47177 bytes
19:22:17.544 in Ack 17
19:22:17.793 chunk 17 47207 bytes
19:22:17.794 in Ack 17
19:22:17.803 fetch POST /api/observability/v1/traces
19:22:17.803 fetched POST /api/observability/v1/traces 204
19:22:18.043 chunk 17 47252 bytes
19:22:18.045 in Ack 17
19:22:18.294 chunk 17 47297 bytes
19:22:18.295 in Ack 17
19:22:18.544 chunk 17 47327 bytes
19:22:18.545 in Ack 17
19:22:18.795 chunk 17 47372 bytes
19:22:18.796 in Ack 17
19:22:19.044 chunk 17 47402 bytes
19:22:19.045 in Ack 17
19:22:19.054 fetch POST /api/observability/v1/traces
19:22:19.054 fetched POST /api/observability/v1/traces 204
19:22:19.294 chunk 17 47432 bytes
19:22:19.296 in Ack 17
19:22:19.544 chunk 17 47462 bytes
19:22:19.545 in Ack 17
19:22:19.795 chunk 17 47507 bytes
19:22:19.796 in Ack 17
19:22:20.045 chunk 17 47537 bytes
19:22:20.046 in Ack 17
19:22:20.295 chunk 17 47582 bytes
19:22:20.296 in Ack 17
19:22:20.303 fetch POST /api/observability/v1/traces
19:22:20.303 fetched POST /api/observability/v1/traces 204
19:22:20.545 chunk 17 47609 bytes
19:22:20.547 in Ack 17
19:22:21.046 chunk 17 49684 bytes
19:22:21.048 in Ack 17
19:22:21.297 chunk 17 49734 bytes
19:22:21.298 in Ack 17
19:22:21.548 fetch POST /api/observability/v1/traces
19:22:21.548 fetched POST /api/observability/v1/traces 204
19:22:21.799 chunk 17 51701 bytes
19:22:21.800 in Ack 17
19:22:22.058 chunk 17 55198 bytes
19:22:22.060 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:22:22.069 in Ack 17
19:22:22.154 in Ping 
19:22:22.645 chunk 6 3203 bytes
19:22:22.646 in Ack 6
19:22:22.815 fetch POST /api/observability/v1/traces
19:22:22.815 fetched POST /api/observability/v1/traces 204
19:22:22.949 chunk 17 56571 bytes
19:22:22.950 in Ack 17
19:22:23.306 chunk 17 59042 bytes
19:22:23.306 exit 33 {"_tag":"Success","value":{"sequence":97}}
19:22:23.307 in Ack 17
19:22:23.349 rpc 34 orchestration.dispatchCommand thread.visit
19:22:23.349 exit 34 {"_tag":"Success","value":{"sequence":0}}
19:22:23.557 chunk 17 59085 bytes
19:22:23.559 in Ack 17
19:22:23.807 chunk 17 59107 bytes
19:22:23.808 in Ack 17
19:22:23.953 fetch POST /api/observability/v1/traces
19:22:23.953 fetched POST /api/observability/v1/traces 204
19:22:24.056 chunk 6 2897 bytes
19:22:24.057 chunk 17 59079 bytes
19:22:24.057 in Ack 6
19:22:24.062 in Ack 17
19:22:24.307 chunk 17 59171 bytes
19:22:24.308 in Ack 17
19:22:24.558 chunk 17 59199 bytes
19:22:24.561 in Ack 17
19:22:24.808 chunk 17 59219 bytes
19:22:24.809 in Ack 17
19:22:25.058 fetch POST /api/observability/v1/traces
19:22:25.058 fetched POST /api/observability/v1/traces 204
19:22:25.059 chunk 17 59229 bytes
19:22:25.060 in Ack 17
19:22:25.309 chunk 6 2897 bytes
19:22:25.309 chunk 17 59249 bytes
19:22:25.310 in Ack 6
19:22:25.314 in Ack 17
19:22:25.560 chunk 17 59289 bytes
19:22:25.561 in Ack 17
19:22:25.811 chunk 17 59309 bytes
19:22:25.812 in Ack 17
19:22:26.061 fetch POST /api/observability/v1/traces
19:22:26.061 fetched POST /api/observability/v1/traces 204
19:22:26.061 chunk 17 59319 bytes
19:22:26.062 in Ack 17
19:22:26.312 chunk 17 59339 bytes
19:22:26.314 in Ack 17
19:22:26.563 chunk 17 59369 bytes
19:22:26.564 in Ack 17
19:22:26.814 chunk 17 59491 bytes
19:22:26.815 in Ack 17
19:22:26.822 in Interrupt 27
19:22:26.845 rpc 35 subscribeVcsStatus
19:22:27.071 fetch POST /api/observability/v1/traces
19:22:27.071 fetched POST /api/observability/v1/traces 204
19:22:27.071 chunk 17 60551 bytes
19:22:27.072 in Ack 17
19:22:27.096 rpc 36 server.reportClientActivity
19:22:27.155 in Ping 
19:22:27.322 chunk 17 60581 bytes
19:22:27.323 in Ack 17
19:22:27.572 chunk 17 60641 bytes
19:22:27.573 in Ack 17
19:22:27.785 chunk 35 1066 bytes
19:22:27.785 in Ack 35
19:22:27.822 chunk 17 60671 bytes
19:22:27.823 in Ack 17
19:22:28.072 chunk 17 60701 bytes
19:22:28.073 fetch POST /api/observability/v1/traces
19:22:28.073 fetched POST /api/observability/v1/traces 204
19:22:28.074 in Ack 17
19:22:28.323 chunk 17 60731 bytes
19:22:28.324 in Ack 17
19:22:28.668 chunk 17 60761 bytes
19:22:28.670 in Ack 17
19:22:28.919 chunk 17 60781 bytes
19:22:28.920 in Ack 17
19:22:29.075 fetch POST /api/observability/v1/traces
19:22:29.075 fetched POST /api/observability/v1/traces 204
19:22:29.169 chunk 17 60811 bytes
19:22:29.170 in Ack 17
19:22:29.420 chunk 17 60841 bytes
19:22:29.421 in Ack 17
19:22:29.670 chunk 17 60861 bytes
19:22:29.672 in Ack 17
19:22:29.921 chunk 17 60881 bytes
19:22:29.922 in Ack 17
19:22:30.170 chunk 17 60921 bytes
19:22:30.172 in Ack 17
19:22:30.180 fetch POST /api/observability/v1/traces
19:22:30.180 fetched POST /api/observability/v1/traces 204
19:22:30.421 chunk 17 60931 bytes
19:22:30.422 in Ack 17
19:22:30.670 chunk 17 60973 bytes
19:22:30.671 in Ack 17
19:22:30.921 chunk 17 60977 bytes
19:22:30.924 in Ack 17
19:22:31.186 chunk 17 61019 bytes
19:22:31.186 exit 36 {"_tag":"Success","value":null}
19:22:31.187 in Ack 17
19:22:31.422 fetch POST /api/observability/v1/traces
19:22:31.423 fetched POST /api/observability/v1/traces 204
19:22:31.436 chunk 17 61029 bytes
19:22:31.437 in Ack 17
19:22:31.694 chunk 17 61059 bytes
19:22:31.696 in Ack 17
19:22:31.944 chunk 6 2897 bytes
19:22:31.944 chunk 17 61099 bytes
19:22:31.945 in Ack 6
19:22:31.947 in Ack 17
19:22:32.155 in Ping 
19:22:32.195 chunk 17 61123 bytes
19:22:32.196 in Ack 17
19:22:32.315 rpc 37 server.reportClientActivity
19:22:32.438 fetch POST /api/observability/v1/traces
19:22:32.438 fetched POST /api/observability/v1/traces 204
19:22:32.446 chunk 17 61153 bytes
19:22:32.447 in Ack 17
19:22:32.649 rpc 38 orchestration.dispatchCommand thread.visit
19:22:32.649 exit 38 {"_tag":"Success","value":{"sequence":0}}
19:22:32.695 chunk 17 61177 bytes
19:22:32.697 in Ack 17
19:22:32.975 chunk 17 61197 bytes
19:22:32.976 in Ack 17
19:22:33.228 chunk 17 62324 bytes
19:22:33.234 in Ack 17
19:22:33.449 fetch POST /api/observability/v1/traces
19:22:33.449 fetched POST /api/observability/v1/traces 204
19:22:33.477 chunk 17 62360 bytes
19:22:33.478 in Ack 17
19:22:33.728 chunk 17 62375 bytes
19:22:33.733 in Ack 17
19:22:33.977 chunk 17 62420 bytes
19:22:33.979 in Ack 17
19:22:34.228 chunk 17 62465 bytes
19:22:34.230 in Ack 17
19:22:34.479 chunk 17 62519 bytes
19:22:34.479 fetch POST /api/observability/v1/traces
19:22:34.480 fetched POST /api/observability/v1/traces 204
19:22:34.481 in Ack 17
19:22:34.553 exit 37 {"_tag":"Success","value":null}
19:22:34.727 chunk 6 2897 bytes
19:22:34.728 in Ack 6
19:22:34.733 chunk 17 62564 bytes
19:22:34.735 in Ack 17
19:22:34.985 chunk 17 62603 bytes
19:22:34.986 in Ack 17
19:22:35.235 chunk 17 62636 bytes
19:22:35.236 in Ack 17
19:22:35.482 fetch POST /api/observability/v1/traces
19:22:35.482 fetched POST /api/observability/v1/traces 204
19:22:35.484 chunk 17 62705 bytes
19:22:35.486 in Ack 17
19:22:35.734 chunk 17 62720 bytes
19:22:35.736 in Ack 17
19:22:35.984 chunk 17 62771 bytes
19:22:35.986 in Ack 17
19:22:36.235 chunk 17 62801 bytes
19:22:36.237 in Ack 17
19:22:36.485 chunk 17 62855 bytes
19:22:36.486 in Ack 17
19:22:36.557 fetch POST /api/observability/v1/traces
19:22:36.557 fetched POST /api/observability/v1/traces 204
19:22:36.734 chunk 17 62888 bytes
19:22:36.736 in Ack 17
19:22:36.984 chunk 17 62918 bytes
19:22:36.986 in Ack 17
19:22:37.157 in Ping 
19:22:37.235 chunk 17 63001 bytes
19:22:37.236 in Ack 17
19:22:37.730 chunk 6 2833 bytes
19:22:37.731 in Ack 6
19:22:37.732 rpc 39 orchestration.dispatchCommand thread.visit
19:22:37.732 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:22:37.737 fetch POST /api/observability/v1/traces
19:22:37.737 fetched POST /api/observability/v1/traces 204
19:22:37.987 chunk 17 63157 bytes
19:22:37.988 in Ack 17
19:22:38.266 chunk 17 62991 bytes
19:22:38.267 in Ack 17
19:22:38.272 in Interrupt 35
19:22:38.274 rpc 40 subscribeVcsStatus
