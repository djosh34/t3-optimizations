# case70, webapp, warm start, seed 7

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 85 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 12.4s |
| Prompt reached the agent | 10.8s |
| Time to first text | 32.2s |
| Whole reply shown | 84.4s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 30.4s / 9.0s / 25.4s |
| Longest silence | 38.7s |
| Approval shown after it was asked | 11.1s |
| Bytes on the wire down / up | 81470 / 36650 |
| Bytes the app received / sent | 52955 / 84667 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 9.1 | 0 | 369 |
| 30 | 19.1 | 0 | 770 |
| 40 | 10.0 | 766 | 1077 |
| 50 | 0.4 | 1077 | 1101 |
| 60 | 10.4 | 1077 | 1477 |
| 70 | 20.4 | 1077 | 1596 |
| 80 | 30.4 | 1077 | 1596 |
| 84 | 24.7 | 1450 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","messages":"548/548 456/456","checkpoints":"36/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173006688,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":6750,"deliveredRate":2256.5476190476193,"loss":0,"srttMs":4.913446105726914,"minRttMs":1,"sentPackets":123,"sentWireBytes":36133,"receivedPackets":104,"receivedWireBytes":49551,"lastHeardMs":9825,"unsyncedBytes":1164,"inflightPackets":17,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 65/613 779/779","checkpointScopes":"472/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173017143,"silences":0,"longestSilenceMs":1340,"longestRecoveredGapMs":1340},"stats":{"sendRate":3375,"deliveredRate":1727.1243169398908,"loss":0.1875,"srttMs":856.3087321746972,"minRttMs":1,"sentPackets":146,"sentWireBytes":47961,"receivedPackets":125,"receivedWireBytes":60141,"lastHeardMs":20280,"unsyncedBytes":840,"inflightPackets":34,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 120/430","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 65/613 779/779","checkpointScopes":"472/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173027673,"silences":3,"longestSilenceMs":2671,"longestRecoveredGapMs":2671},"stats":{"sendRate":3375,"deliveredRate":1232.9495695402725,"loss":0.41360294117647056,"srttMs":962.27014065286,"minRttMs":1,"sentPackets":162,"sentWireBytes":54088,"receivedPackets":132,"receivedWireBytes":63728,"lastHeardMs":30810,"unsyncedBytes":595,"inflightPackets":49,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 783/1078","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 783/928","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 159/613 779/779","checkpointScopes":"472/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173037722,"silences":4,"longestSilenceMs":2671,"longestRecoveredGapMs":2671},"stats":{"sendRate":2219.008890293225,"deliveredRate":634.4208242032636,"loss":0.6478658088235294,"srttMs":1702.8585350244894,"minRttMs":1,"sentPackets":208,"sentWireBytes":66754,"receivedPackets":149,"receivedWireBytes":70372,"lastHeardMs":40858,"unsyncedBytes":0,"inflightPackets":78,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 1059/1078 843/849","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 1274/1308","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 159/613 779/779","checkpointScopes":"472/723","runtimeRequests":"289/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173047942,"silences":4,"longestSilenceMs":2671,"longestRecoveredGapMs":2671},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.7499906996991977,"srttMs":1702.8585350244894,"minRttMs":1,"sentPackets":264,"sentWireBytes":75892,"receivedPackets":176,"receivedWireBytes":79112,"lastHeardMs":51079,"unsyncedBytes":489,"inflightPackets":86,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 446/1103 843/849 1547/1550 557/929","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 159/613 779/779","checkpointScopes":"472/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173058909,"silences":4,"longestSilenceMs":2671,"longestRecoveredGapMs":2671},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.63375,"srttMs":4365.0650686768395,"minRttMs":1,"sentPackets":344,"sentWireBytes":98221,"receivedPackets":206,"receivedWireBytes":90925,"lastHeardMs":62046,"unsyncedBytes":400,"inflightPackets":138,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 446/1103 843/849 1550/1550 902/929","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 159/613 779/779","checkpointScopes":"472/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173062379,"silences":4,"longestSilenceMs":2671,"longestRecoveredGapMs":2671},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.63375,"srttMs":4365.0650686768395,"minRttMs":1,"sentPackets":359,"sentWireBytes":104128,"receivedPackets":217,"receivedWireBytes":94909,"lastHeardMs":65515,"unsyncedBytes":400,"inflightPackets":153,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1103/1103 870/870 1058/1058 860/860 1299/1299 1103/1103 1058/1058 490/1103 843/849 1550/1550 902/929","frame":"1031/1031","thread":"951/951","assistant":"32/32 21/21 1340/1340 206/723","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776 466/776","providerThreads":"434/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 347/1022","messages":"548/548 456/456 715/901","checkpoints":"613/613 159/613 779/779","checkpointScopes":"472/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":267,"lastDataAt":1794173080907,"silences":5,"longestSilenceMs":16796,"longestRecoveredGapMs":16796},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.63375,"srttMs":5433.806935092234,"minRttMs":1091,"sentPackets":365,"sentWireBytes":107600,"receivedPackets":221,"receivedWireBytes":96959,"lastHeardMs":84044,"unsyncedBytes":889,"inflightPackets":118,"duplicateBytes":0}}
- shim trace
21:23:17.131 fetch GET /api/auth/session
21:23:17.132 fetched GET /api/auth/session 200
21:23:17.196 fetch GET /.well-known/t3/environment
21:23:17.197 fetched GET /.well-known/t3/environment 200
21:23:17.205 fetch GET /.well-known/t3/environment
21:23:17.206 fetched GET /.well-known/t3/environment 200
21:23:17.210 rpc 0 subscribeServerConfig
21:23:17.211 chunk 0 38678 bytes
21:23:17.240 in Ack 0
21:23:17.242 rpc 1 subscribeServerLifecycle
21:23:17.242 rpc 2 subscribeServerLifecycle
21:23:17.245 fetch GET /api/auth/session
21:23:17.246 fetched GET /api/auth/session 200
21:23:17.246 rpc 3 subscribeDeviceState
21:23:17.248 rpc 4 preview.reportProfiles
21:23:17.316 rpc 5 subscribeProjectClones
21:23:17.334 fetch GET /api/orchestration/threads/mcp%3A3e86b111-bf7f-4244-97c7-0a6e7703cf60/bounded
21:23:17.337 fetch GET /api/orchestration/shell
21:23:17.338 fetched GET /api/orchestration/shell 200
21:23:17.339 rpc 6 orchestration.subscribeShell
21:23:17.340 chunk 6 2779 bytes
21:23:17.359 rpc 7 server.refreshProviders
21:23:17.394 rpc 8 orchestration.dispatchCommand thread.visit
21:23:17.394 exit 8 {"_tag":"Success","value":{"sequence":0}}
21:23:17.407 rpc 9 subscribeTerminalMetadata
21:23:17.408 rpc 10 subscribeVcsStatus
21:23:17.409 rpc 11 subscribePreviewEvents
21:23:17.410 rpc 12 assets.createUrl
21:23:17.412 rpc 13 preview.list
21:23:17.414 rpc 14 projects.readFile
21:23:17.414 in Ack 6
21:23:17.684 rpc 15 server.reportClientActivity
21:23:17.939 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"1259423d-4fd7-407f-bfe2-17ac124adce3","sessions":[]}}
21:23:17.939 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:23:18.196 fetch POST /api/observability/v1/traces
21:23:18.196 fetched POST /api/observability/v1/traces 204
21:23:18.439 chunk 1 3861 bytes
21:23:18.440 in Ack 1
21:23:18.692 chunk 2 3861 bytes
21:23:18.692 in Ack 2
21:23:18.879 chunk 6 2755 bytes
21:23:18.879 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:23:05.341Z","configurableProviders":false,"con
21:23:18.879 chunk 10 472 bytes
21:23:18.879 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:23:18.880 in Ack 6
21:23:18.886 in Ack 10
21:23:19.348 chunk 3 876 bytes
21:23:19.348 exit 4 {"_tag":"Success","value":null}
21:23:19.348 chunk 5 4 bytes
21:23:19.348 chunk 9 36 bytes
21:23:19.348 exit 15 {"_tag":"Success","value":null}
21:23:19.349 in Ack 3
21:23:19.352 rpc 16 preview.reportProfiles
21:23:19.352 in Ack 5
21:23:19.353 in Ack 9
21:23:19.380 chunk 6 2798 bytes
21:23:19.380 in Ack 6
21:23:19.424 exit 16 {"_tag":"Success","value":null}
21:23:19.693 fetch POST /api/observability/v1/traces
21:23:19.693 fetched POST /api/observability/v1/traces 204
21:23:19.759 fetched GET /api/orchestration/threads/mcp%3A3e86b111-bf7f-4244-97c7-0a6e7703cf60/bounded 200
21:23:19.762 rpc 17 orchestration.subscribeThread
21:23:19.762 chunk 17 8294 bytes
21:23:19.771 in Interrupt 10
21:23:19.773 rpc 18 subscribeVcsStatus
21:23:19.774 in Ack 17
21:23:19.918 chunk 18 472 bytes
21:23:19.920 in Ack 18
21:23:20.025 chunk 17 8334 bytes
21:23:20.026 rpc 19 server.reportClientActivity
21:23:20.027 in Ack 17
21:23:20.160 exit 19 {"_tag":"Success","value":null}
21:23:20.275 chunk 17 11932 bytes
21:23:20.276 in Ack 17
21:23:20.525 chunk 17 13517 bytes
21:23:20.526 in Ack 17
21:23:20.624 rpc 20 attachments.createUploadUrl
21:23:20.792 fetch POST /api/observability/v1/traces
21:23:20.792 fetched POST /api/observability/v1/traces 204
21:23:20.869 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-56e6c065-7961-47f0-83f3-c53b216be09d","expiresAt":1794173600828,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:23:20.870 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTU2ZTZjMDY1LTc5NjEtNDdmMC04M2YzLWM1M2IyMTZiZTA5ZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3MzYwMDgyOH0.bIWcQOit3qTSwrVDNTJAwpArQXq7VXj8xwdpuwGzejY
21:23:21.042 chunk 17 17224 bytes
21:23:21.043 in Ack 17
21:23:21.293 chunk 17 19081 bytes
21:23:21.294 in Ack 17
21:23:21.550 chunk 17 21197 bytes
21:23:21.551 in Ack 17
21:23:21.558 in Interrupt 18
21:23:21.560 rpc 21 subscribeVcsStatus
21:23:21.678 chunk 21 472 bytes
21:23:21.680 in Ack 21
21:23:21.810 rpc 22 server.reportClientActivity
21:23:21.871 fetch POST /api/observability/v1/traces
21:23:21.871 fetched POST /api/observability/v1/traces 204
21:23:22.208 in Ping 
21:23:22.222 exit 22 {"_tag":"Success","value":null}
21:23:23.224 fetch POST /api/observability/v1/traces
21:23:23.224 fetched POST /api/observability/v1/traces 204
21:23:23.818 rpc 23 orchestration.dispatchCommand message.dispatch
21:23:24.439 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:23:24.441 rpc 24 attachments.delete
21:23:24.817 fetch POST /api/observability/v1/traces
21:23:24.817 fetched POST /api/observability/v1/traces 204
21:23:24.940 chunk 17 23642 bytes
21:23:24.941 in Ack 17
21:23:25.193 chunk 17 23674 bytes
21:23:25.194 in Ack 17
21:23:25.443 chunk 17 23676 bytes
21:23:25.444 in Ack 17
21:23:25.693 chunk 6 2820 bytes
21:23:25.693 in Ack 6
21:23:25.694 rpc 25 orchestration.dispatchCommand thread.visit
21:23:25.695 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:23:25.942 fetch POST /api/observability/v1/traces
21:23:25.942 fetched POST /api/observability/v1/traces 204
21:23:25.942 chunk 6 2820 bytes
21:23:25.943 in Ack 6
21:23:25.982 chunk 17 24648 bytes
21:23:25.983 in Ack 17
21:23:26.233 chunk 17 28103 bytes
21:23:26.234 in Ack 17
21:23:26.236 rpc 26 assets.createUrl
21:23:26.271 exit 24 {"_tag":"Success","value":null}
21:23:26.679 chunk 17 31336 bytes
21:23:26.680 in Ack 17
21:23:26.929 chunk 17 32116 bytes
21:23:26.930 in Ack 17
21:23:26.937 in Interrupt 21
21:23:26.940 rpc 27 subscribeVcsStatus
21:23:26.947 fetch POST /api/observability/v1/traces
21:23:26.947 fetched POST /api/observability/v1/traces 204
21:23:27.203 rpc 28 server.reportClientActivity
21:23:27.214 in Ping 
21:23:27.514 rpc 29 orchestration.dispatchCommand message.dispatch
21:23:28.203 fetch POST /api/observability/v1/traces
21:23:28.203 fetched POST /api/observability/v1/traces 204
21:23:30.736 chunk 27 472 bytes
21:23:30.737 in Ack 27
21:23:31.157 chunk 17 31950 bytes
21:23:31.162 in Ack 17
21:23:31.269 fetch POST /api/observability/v1/traces
21:23:31.269 fetched POST /api/observability/v1/traces 204
21:23:32.214 in Ping 
21:23:37.215 in Ping 
21:23:39.776 chunk 17 34407 bytes
21:23:39.776 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:23:39.783 in Ack 17
21:23:40.211 chunk 17 34439 bytes
21:23:40.212 in Ack 17
21:23:40.753 chunk 17 34471 bytes
21:23:40.754 in Ack 17
21:23:40.788 fetch POST /api/observability/v1/traces
21:23:40.788 fetched POST /api/observability/v1/traces 204
21:23:41.327 chunk 17 34503 bytes
21:23:41.329 in Ack 17
21:23:42.204 in Interrupt 28
21:23:42.215 in Ping 
21:23:42.330 fetch POST /api/observability/v1/traces
21:23:42.330 fetched POST /api/observability/v1/traces 204
21:23:42.452 rpc 30 server.reportClientActivity
21:23:43.453 fetch POST /api/observability/v1/traces
21:23:43.453 fetched POST /api/observability/v1/traces 204
21:23:43.998 chunk 17 34555 bytes
21:23:44.000 in Ack 17
21:23:45.001 fetch POST /api/observability/v1/traces
21:23:45.001 fetched POST /api/observability/v1/traces 204
21:23:45.397 chunk 17 34587 bytes
21:23:45.398 in Ack 17
21:23:46.399 fetch POST /api/observability/v1/traces
21:23:46.399 fetched POST /api/observability/v1/traces 204
21:23:47.216 in Ping 
21:23:47.674 chunk 17 34639 bytes
21:23:47.676 in Ack 17
21:23:48.676 fetch POST /api/observability/v1/traces
21:23:48.676 fetched POST /api/observability/v1/traces 204
21:23:49.688 chunk 17 34691 bytes
21:23:49.689 in Ack 17
21:23:50.210 chunk 17 34751 bytes
21:23:50.211 in Ack 17
21:23:50.690 fetch POST /api/observability/v1/traces
21:23:50.690 fetched POST /api/observability/v1/traces 204
21:23:50.761 chunk 17 34801 bytes
21:23:50.762 in Ack 17
21:23:51.523 chunk 17 34823 bytes
21:23:51.524 in Ack 17
21:23:51.763 fetch POST /api/observability/v1/traces
21:23:51.763 fetched POST /api/observability/v1/traces 204
21:23:51.873 chunk 17 35079 bytes
21:23:51.876 in Ack 17
21:23:52.128 chunk 17 35363 bytes
21:23:52.129 in Ack 17
21:23:52.216 in Ping 
21:23:52.390 chunk 17 35633 bytes
21:23:52.391 in Ack 17
21:23:52.721 chunk 17 35643 bytes
21:23:52.723 in Ack 17
21:23:52.881 fetch POST /api/observability/v1/traces
21:23:52.881 fetched POST /api/observability/v1/traces 204
21:23:54.713 chunk 17 35735 bytes
21:23:54.715 in Ack 17
21:23:55.143 chunk 17 35785 bytes
21:23:55.144 in Ack 17
21:23:55.576 chunk 17 35815 bytes
21:23:55.577 in Ack 17
21:23:55.729 fetch POST /api/observability/v1/traces
21:23:55.729 fetched POST /api/observability/v1/traces 204
21:23:56.018 chunk 17 35847 bytes
21:23:56.020 in Ack 17
21:23:56.423 chunk 17 35907 bytes
21:23:56.426 in Ack 17
21:23:56.743 chunk 17 35919 bytes
21:23:56.744 in Ack 17
21:23:57.022 fetch POST /api/observability/v1/traces
21:23:57.022 fetched POST /api/observability/v1/traces 204
21:23:57.216 in Ping 
21:23:57.218 chunk 17 35931 bytes
21:23:57.219 in Ack 17
21:23:57.453 in Interrupt 30
21:23:57.471 chunk 17 35951 bytes
21:23:57.472 in Ack 17
21:23:57.723 chunk 17 35971 bytes
21:23:57.724 in Ack 17
21:23:58.159 chunk 17 36021 bytes
21:23:58.160 in Ack 17
21:23:58.220 fetch POST /api/observability/v1/traces
21:23:58.220 fetched POST /api/observability/v1/traces 204
21:23:59.116 chunk 17 36187 bytes
21:23:59.118 in Ack 17
21:23:59.371 chunk 17 36281 bytes
21:23:59.372 in Ack 17
21:23:59.807 chunk 17 36311 bytes
21:23:59.808 in Ack 17
21:24:00.057 chunk 17 36321 bytes
21:24:00.058 in Ack 17
21:24:00.119 fetch POST /api/observability/v1/traces
21:24:00.119 fetched POST /api/observability/v1/traces 204
21:24:00.452 chunk 17 36361 bytes
21:24:00.453 in Ack 17
21:24:01.056 chunk 17 36393 bytes
21:24:01.059 in Ack 17
21:24:01.284 fetch POST /api/observability/v1/traces
21:24:01.284 fetched POST /api/observability/v1/traces 204
21:24:01.558 chunk 17 36415 bytes
21:24:01.559 in Ack 17
21:24:01.808 chunk 17 36437 bytes
21:24:01.809 in Ack 17
21:24:02.063 chunk 17 36447 bytes
21:24:02.065 in Ack 17
21:24:02.216 in Ping 
21:24:02.560 fetch POST /api/observability/v1/traces
21:24:02.560 fetched POST /api/observability/v1/traces 204
21:24:02.579 chunk 17 36459 bytes
21:24:02.580 in Ack 17
21:24:03.171 chunk 17 36599 bytes
21:24:03.172 in Ack 17
21:24:03.423 chunk 17 36667 bytes
21:24:03.424 in Ack 17
21:24:03.582 fetch POST /api/observability/v1/traces
21:24:03.582 fetched POST /api/observability/v1/traces 204
21:24:03.758 chunk 17 36705 bytes
21:24:03.759 in Ack 17
21:24:04.010 chunk 17 36725 bytes
21:24:04.011 in Ack 17
21:24:04.262 chunk 17 36763 bytes
21:24:04.263 in Ack 17
21:24:04.517 chunk 17 36773 bytes
21:24:04.518 in Ack 17
21:24:04.760 fetch POST /api/observability/v1/traces
21:24:04.760 fetched POST /api/observability/v1/traces 204
21:24:04.772 chunk 17 36783 bytes
21:24:04.773 in Ack 17
21:24:05.032 chunk 17 36793 bytes
21:24:05.033 in Ack 17
21:24:05.362 chunk 17 36873 bytes
21:24:05.365 in Ack 17
21:24:05.613 chunk 17 36883 bytes
21:24:05.614 in Ack 17
21:24:05.774 fetch POST /api/observability/v1/traces
21:24:05.774 fetched POST /api/observability/v1/traces 204
21:24:05.868 chunk 17 36931 bytes
21:24:05.869 in Ack 17
21:24:06.119 chunk 17 36941 bytes
21:24:06.120 in Ack 17
21:24:06.870 fetch POST /api/observability/v1/traces
21:24:06.870 fetched POST /api/observability/v1/traces 204
21:24:07.217 in Ping 
21:24:07.453 rpc 31 server.reportClientActivity
21:24:08.453 fetch POST /api/observability/v1/traces
21:24:08.453 fetched POST /api/observability/v1/traces 204
21:24:09.251 chunk 17 37073 bytes
21:24:09.252 in Ack 17
21:24:10.283 fetch POST /api/observability/v1/traces
21:24:10.283 fetched POST /api/observability/v1/traces 204
21:24:11.287 fetch POST /api/observability/v1/traces
21:24:11.287 fetched POST /api/observability/v1/traces 204
21:24:12.220 in Ping 
21:24:16.101 chunk 17 38081 bytes
21:24:16.102 in Ack 17
21:24:16.317 rpc 32 orchestration.dispatchCommand runtime-request.respond
21:24:17.104 fetch POST /api/observability/v1/traces
21:24:17.104 fetched POST /api/observability/v1/traces 204
21:24:17.224 in Ping 
21:24:21.194 chunk 17 41586 bytes
21:24:21.196 in Ack 17
21:24:21.292 fetch POST /api/observability/v1/traces
21:24:21.292 fetched POST /api/observability/v1/traces 204
21:24:22.228 in Ping 
21:24:22.454 in Interrupt 31
21:24:23.455 fetch POST /api/observability/v1/traces
21:24:23.455 fetched POST /api/observability/v1/traces 204
21:24:27.234 in Ping 
21:24:31.293 fetch POST /api/observability/v1/traces
21:24:31.293 fetched POST /api/observability/v1/traces 204
21:24:32.236 in Ping 
21:24:32.453 rpc 33 server.reportClientActivity
21:24:33.453 fetch POST /api/observability/v1/traces
21:24:33.453 fetched POST /api/observability/v1/traces 204
21:24:37.240 in Ping 
21:24:39.176 chunk 17 44323 bytes
21:24:39.176 exit 32 {"_tag":"Success","value":{"sequence":95}}
21:24:39.177 in Ack 17
21:24:39.691 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:24:39.783 chunk 17 44467 bytes
21:24:39.785 in Ack 17
21:24:40.179 fetch POST /api/observability/v1/traces
21:24:40.179 fetched POST /api/observability/v1/traces 204
21:24:41.296 fetch POST /api/observability/v1/traces
21:24:41.297 fetched POST /api/observability/v1/traces 204
21:24:41.342 chunk 17 44753 bytes
21:24:41.344 in Ack 17
21:24:42.244 in Ping 
21:24:42.346 fetch POST /api/observability/v1/traces
21:24:42.346 fetched POST /api/observability/v1/traces 204
21:24:44.114 chunk 17 45147 bytes
21:24:44.115 in Ack 17
21:24:45.116 fetch POST /api/observability/v1/traces
21:24:45.116 fetched POST /api/observability/v1/traces 204
21:24:47.248 in Ping 
21:24:47.455 in Interrupt 33
21:24:47.912 chunk 17 45489 bytes
21:24:47.914 in Ack 17
21:24:48.455 fetch POST /api/observability/v1/traces
21:24:48.455 fetched POST /api/observability/v1/traces 204
21:24:51.211 chunk 17 45521 bytes
21:24:51.212 in Ack 17
21:24:51.299 fetch POST /api/observability/v1/traces
21:24:51.299 fetched POST /api/observability/v1/traces 204
