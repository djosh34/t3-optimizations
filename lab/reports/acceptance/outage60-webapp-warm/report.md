# outage60, webapp, warm start, seed 1

Profile outage60 (fault): one 60 s outage on a weak link. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 94 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.8s |
| Prompt reached the agent | 0.5s |
| Time to first text | 1.2s |
| Whole reply shown | 92.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 67.0s / 21.0s / 62.0s |
| Longest silence | 67.3s |
| Approval shown after it was asked | 50.1s |
| Bytes on the wire down / up | 374953 / 387843 |
| Bytes the app received / sent | 193271 / 50769 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.0 | 336 | 381 |
| 20 | 11.0 | 336 | 783 |
| 30 | 21.0 | 336 | 1077 |
| 40 | 31.0 | 336 | 1077 |
| 50 | 41.0 | 336 | 1077 |
| 60 | 51.0 | 336 | 1077 |
| 70 | 61.0 | 336 | 1077 |
| 80 | 1.2 | 1101 | 1152 |
| 90 | 1.3 | 1473 | 1524 |
| 92 | 0.8 | 1560 | 1590 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","messages":"456/456 546/546 901/901 546/546","providerTurns":"774/774 774/774","providerSessions":"2695/2695","providerThreads":"808/808","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"113/733"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168032879,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":3375,"deliveredRate":2719.726220284558,"loss":0,"srttMs":207.1211049608309,"minRttMs":1,"sentPackets":108,"sentWireBytes":31720,"receivedPackets":115,"receivedWireBytes":53695,"lastHeardMs":9887,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":330,"sentPackets":161,"sentWireBytes":39873,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":171,"sentWireBytes":42243,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":666,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":173,"sentWireBytes":42908,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":666,"inflightPackets":10,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":174,"sentWireBytes":43241,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":666,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":176,"sentWireBytes":43907,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":1165,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":177,"sentWireBytes":44240,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":1165,"inflightPackets":13,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 420/420","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168041935,"silences":0,"longestSilenceMs":1588,"longestRecoveredGapMs":1588},"stats":{"sendRate":50,"deliveredRate":0,"loss":0.006052941176470588,"srttMs":317.93174958224915,"minRttMs":null,"sentPackets":179,"sentWireBytes":44906,"receivedPackets":257,"receivedWireBytes":100334,"lastHeardMs":18944,"unsyncedBytes":1665,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 61/1570 724/926 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 133/133","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168113303,"silences":1,"longestSilenceMs":66315,"longestRecoveredGapMs":66315},"stats":{"sendRate":4500,"deliveredRate":2678.319612708607,"loss":0,"srttMs":366.84809063859296,"minRttMs":290,"sentPackets":224,"sentWireBytes":57813,"receivedPackets":307,"receivedWireBytes":124429,"lastHeardMs":90311,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 679/679","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 451/451 546/546 545/545","providerTurns":"774/774 774/774 656/656","providerSessions":"2695/2695","providerThreads":"810/810","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"733/733","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":241,"lastDataAt":1794168124482,"silences":1,"longestSilenceMs":66315,"longestRecoveredGapMs":66315},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.09515551282051281,"srttMs":372.36209720824195,"minRttMs":290,"sentPackets":312,"sentWireBytes":77302,"receivedPackets":747,"receivedWireBytes":225562,"lastHeardMs":101491,"unsyncedBytes":499,"inflightPackets":16,"duplicateBytes":0}}
- shim trace
20:00:23.258 fetch GET /api/auth/session
20:00:23.259 fetched GET /api/auth/session 200
20:00:23.409 fetch GET /.well-known/t3/environment
20:00:23.409 fetched GET /.well-known/t3/environment 200
20:00:23.417 fetch GET /.well-known/t3/environment
20:00:23.418 fetched GET /.well-known/t3/environment 200
20:00:23.421 rpc 0 subscribeServerConfig
20:00:23.422 chunk 0 38718 bytes
20:00:23.437 in Ack 0
20:00:23.438 rpc 1 subscribeServerLifecycle
20:00:23.439 rpc 2 subscribeServerLifecycle
20:00:23.441 fetch GET /api/auth/session
20:00:23.442 fetched GET /api/auth/session 200
20:00:23.442 rpc 3 subscribeDeviceState
20:00:23.444 rpc 4 preview.reportProfiles
20:00:23.491 rpc 5 subscribeProjectClones
20:00:23.507 fetch GET /api/orchestration/threads/mcp%3A9d944314-69ff-472d-aece-a6000702a089/bounded
20:00:23.512 fetch GET /api/orchestration/shell
20:00:23.529 rpc 6 server.refreshProviders
20:00:23.566 rpc 7 orchestration.dispatchCommand thread.visit
20:00:23.566 exit 7 {"_tag":"Success","value":{"sequence":0}}
20:00:23.576 fetched GET /api/orchestration/shell 200
20:00:23.577 rpc 8 subscribeTerminalMetadata
20:00:23.577 rpc 9 subscribeVcsStatus
20:00:23.578 rpc 10 subscribePreviewEvents
20:00:23.580 rpc 11 orchestration.subscribeShell
20:00:23.580 chunk 11 2789 bytes
20:00:23.581 rpc 12 assets.createUrl
20:00:23.582 rpc 13 preview.list
20:00:23.582 rpc 14 projects.readFile
20:00:23.586 in Ack 11
20:00:23.751 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"6b857f13-4475-4a06-b110-dae518d2765f","sessions":[]}}
20:00:23.751 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
20:00:23.830 rpc 15 server.reportClientActivity
20:00:23.830 chunk 11 2765 bytes
20:00:23.831 in Ack 11
20:00:24.009 chunk 1 3871 bytes
20:00:24.010 in Ack 1
20:00:24.249 chunk 2 3871 bytes
20:00:24.250 in Ack 2
20:00:24.407 fetch POST /api/observability/v1/traces
20:00:24.407 fetched POST /api/observability/v1/traces 204
20:00:24.459 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T20:00:11.497Z","configurableProviders":false,"con
20:00:24.459 chunk 9 2331 bytes
20:00:24.462 in Ack 9
20:00:24.650 exit 12 {"_tag":"Success","value":{"expiresAt":1794171600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
20:00:24.900 chunk 11 2808 bytes
20:00:24.901 in Ack 11
20:00:24.957 chunk 3 876 bytes
20:00:24.957 exit 4 {"_tag":"Success","value":null}
20:00:24.957 chunk 5 4 bytes
20:00:24.957 chunk 8 36 bytes
20:00:24.958 in Ack 3
20:00:24.962 rpc 16 preview.reportProfiles
20:00:24.962 in Ack 5
20:00:24.962 in Ack 8
20:00:25.048 exit 15 {"_tag":"Success","value":null}
20:00:25.048 exit 16 {"_tag":"Success","value":null}
20:00:25.048 fetched GET /api/orchestration/threads/mcp%3A9d944314-69ff-472d-aece-a6000702a089/bounded 200
20:00:25.051 rpc 17 orchestration.subscribeThread
20:00:25.051 chunk 17 8283 bytes
20:00:25.060 in Interrupt 9
20:00:25.062 rpc 18 subscribeVcsStatus
20:00:25.064 in Ack 17
20:00:25.169 chunk 18 2331 bytes
20:00:25.169 in Ack 18
20:00:25.302 chunk 17 8323 bytes
20:00:25.303 in Ack 17
20:00:25.325 rpc 19 server.reportClientActivity
20:00:25.489 fetch POST /api/observability/v1/traces
20:00:25.489 fetched POST /api/observability/v1/traces 204
20:00:25.593 chunk 17 11920 bytes
20:00:25.596 in Ack 17
20:00:25.891 chunk 17 13502 bytes
20:00:25.892 in Ack 17
20:00:25.909 exit 19 {"_tag":"Success","value":null}
20:00:25.951 rpc 20 attachments.createUploadUrl
20:00:26.140 chunk 17 16197 bytes
20:00:26.141 in Ack 17
20:00:26.159 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-0957e274-00f1-456d-a1e7-91dbe02f7839","expiresAt":1794168626138,"relativeUrl":"/api/attachments/upload/eyJ2ZX
20:00:26.160 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA5NTdlMjc0LTAwZjEtNDU2ZC1hMWU3LTkxZGJlMDJmNzgzOSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODYyNjEzOH0.-lSqUwPaJgV-VmZlDYX7brNkbdxME5QLb5JMFe9BYKs
20:00:26.391 chunk 17 17216 bytes
20:00:26.392 in Ack 17
20:00:26.597 fetch POST /api/observability/v1/traces
20:00:26.597 fetched POST /api/observability/v1/traces 204
20:00:26.641 chunk 17 19850 bytes
20:00:26.642 in Ack 17
20:00:26.648 in Interrupt 18
20:00:26.650 rpc 21 subscribeVcsStatus
20:00:26.709 chunk 21 2331 bytes
20:00:26.709 in Ack 21
20:00:26.908 chunk 17 21197 bytes
20:00:26.908 rpc 22 server.reportClientActivity
20:00:26.911 in Ack 17
20:00:27.644 fetch POST /api/observability/v1/traces
20:00:27.644 fetched POST /api/observability/v1/traces 204
20:00:27.927 exit 22 {"_tag":"Success","value":null}
20:00:28.420 in Ping 
20:00:28.929 fetch POST /api/observability/v1/traces
20:00:28.929 fetched POST /api/observability/v1/traces 204
20:00:29.137 rpc 23 orchestration.dispatchCommand message.dispatch
20:00:29.774 exit 23 {"_tag":"Success","value":{"sequence":45}}
20:00:29.777 rpc 24 attachments.delete
20:00:29.868 exit 24 {"_tag":"Success","value":null}
20:00:30.025 chunk 17 23631 bytes
20:00:30.026 in Ack 17
20:00:30.136 fetch POST /api/observability/v1/traces
20:00:30.136 fetched POST /api/observability/v1/traces 204
20:00:30.275 chunk 17 23603 bytes
20:00:30.276 in Ack 17
20:00:30.525 chunk 11 2894 bytes
20:00:30.525 chunk 17 23615 bytes
20:00:30.526 in Ack 11
20:00:30.554 in Ack 17
20:00:30.785 chunk 17 23665 bytes
20:00:30.786 in Ack 17
20:00:31.035 chunk 11 2830 bytes
20:00:31.035 in Ack 11
20:00:31.037 rpc 25 orchestration.dispatchCommand thread.visit
20:00:31.037 exit 25 {"_tag":"Success","value":{"sequence":0}}
20:00:31.277 fetch POST /api/observability/v1/traces
20:00:31.277 fetched POST /api/observability/v1/traces 204
20:00:31.285 chunk 11 2830 bytes
20:00:31.285 chunk 17 24637 bytes
20:00:31.285 in Ack 11
20:00:31.291 in Ack 17
20:00:31.536 chunk 17 30514 bytes
20:00:31.538 in Ack 17
20:00:31.540 rpc 26 assets.createUrl
20:00:31.990 chunk 17 31938 bytes
20:00:31.992 in Ack 17
20:00:31.999 in Interrupt 21
20:00:32.002 rpc 27 subscribeVcsStatus
20:00:32.256 rpc 28 server.reportClientActivity
20:00:32.271 chunk 17 31938 bytes
20:00:32.271 chunk 27 2331 bytes
20:00:32.273 in Ack 17
20:00:32.280 in Ack 27
20:00:32.288 fetch POST /api/observability/v1/traces
20:00:32.288 fetched POST /api/observability/v1/traces 204
20:00:32.535 chunk 17 31938 bytes
20:00:32.541 in Ack 17
20:00:32.841 rpc 29 orchestration.dispatchCommand message.dispatch
20:00:32.879 chunk 17 35212 bytes
20:00:32.882 in Ack 17
20:00:33.034 exit 28 {"_tag":"Success","value":null}
20:00:33.131 chunk 17 35212 bytes
20:00:33.132 in Ack 17
20:00:33.335 exit 29 {"_tag":"Success","value":{"sequence":75}}
20:00:33.420 in Ping 
20:00:33.542 fetch POST /api/observability/v1/traces
20:00:33.542 fetched POST /api/observability/v1/traces 204
20:00:33.631 chunk 17 37646 bytes
20:00:33.632 in Ack 17
20:00:33.881 chunk 17 37666 bytes
20:00:33.882 in Ack 17
20:00:34.131 chunk 17 37686 bytes
20:00:34.132 in Ack 17
20:00:34.381 chunk 17 37668 bytes
20:00:34.382 in Ack 17
20:00:34.635 chunk 17 37698 bytes
20:00:34.635 fetch POST /api/observability/v1/traces
20:00:34.635 fetched POST /api/observability/v1/traces 204
20:00:34.637 in Ack 17
20:00:34.885 chunk 17 37718 bytes
20:00:34.886 in Ack 17
20:00:35.136 chunk 11 2894 bytes
20:00:35.136 chunk 17 37738 bytes
20:00:35.137 in Ack 11
20:00:35.166 in Ack 17
20:00:35.387 chunk 17 38405 bytes
20:00:35.388 in Ack 17
20:00:35.636 chunk 17 38425 bytes
20:00:35.637 in Ack 17
20:00:35.647 fetch POST /api/observability/v1/traces
20:00:35.647 fetched POST /api/observability/v1/traces 204
20:00:35.887 chunk 17 38465 bytes
20:00:35.889 in Ack 17
20:00:36.137 chunk 17 39325 bytes
20:00:36.138 in Ack 17
20:00:36.387 chunk 17 39355 bytes
20:00:36.388 in Ack 17
20:00:36.637 chunk 17 41368 bytes
20:00:36.638 in Ack 17
20:00:36.664 fetch POST /api/observability/v1/traces
20:00:36.664 fetched POST /api/observability/v1/traces 204
20:00:36.888 chunk 17 41408 bytes
20:00:36.889 in Ack 17
20:00:37.139 chunk 17 42075 bytes
20:00:37.140 in Ack 17
20:00:37.390 chunk 17 42105 bytes
20:00:37.391 in Ack 17
20:00:37.640 chunk 17 42921 bytes
20:00:37.641 in Ack 17
20:00:37.890 fetch POST /api/observability/v1/traces
20:00:37.890 fetched POST /api/observability/v1/traces 204
20:00:37.891 chunk 17 42941 bytes
20:00:37.892 in Ack 17
20:00:38.140 chunk 17 42973 bytes
20:00:38.142 in Ack 17
20:00:38.392 chunk 17 43519 bytes
20:00:38.393 in Ack 17
20:00:38.420 in Ping 
20:00:38.642 chunk 17 43539 bytes
20:00:38.644 in Ack 17
20:00:38.894 fetch POST /api/observability/v1/traces
20:00:38.894 fetched POST /api/observability/v1/traces 204
20:00:38.894 chunk 17 43569 bytes
20:00:38.895 in Ack 17
20:00:39.145 chunk 17 43589 bytes
20:00:39.146 in Ack 17
20:00:39.395 chunk 17 43609 bytes
20:00:39.396 in Ack 17
20:00:39.623 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/38b969c3-5a1d-4b95-bd18-08e91fe5d95d","expiresAt":1794171639623}}
20:00:39.645 chunk 17 43639 bytes
20:00:39.646 in Ack 17
20:00:39.895 chunk 17 43659 bytes
20:00:39.896 in Ack 17
20:00:39.911 fetch POST /api/observability/v1/traces
20:00:39.911 fetched POST /api/observability/v1/traces 204
20:00:40.146 chunk 17 43689 bytes
20:00:40.147 in Ack 17
20:00:40.395 chunk 17 43709 bytes
20:00:40.397 in Ack 17
20:00:40.646 chunk 17 43739 bytes
20:00:40.647 in Ack 17
20:00:40.896 chunk 17 43759 bytes
20:00:40.897 in Ack 17
20:00:41.037 rpc 30 orchestration.dispatchCommand thread.visit
20:00:41.037 exit 30 {"_tag":"Success","value":{"sequence":0}}
20:00:41.146 chunk 17 43789 bytes
20:00:41.147 in Ack 17
20:00:41.155 fetch POST /api/observability/v1/traces
20:00:41.155 fetched POST /api/observability/v1/traces 204
20:00:41.396 chunk 17 43809 bytes
20:00:41.397 in Ack 17
20:00:41.647 chunk 11 2894 bytes
20:00:41.647 chunk 17 43829 bytes
20:00:41.648 in Ack 11
20:00:41.655 in Ack 17
20:00:41.899 chunk 17 43859 bytes
20:00:41.900 in Ack 17
20:00:42.149 chunk 17 43869 bytes
20:00:42.150 in Ack 17
20:00:42.398 fetch POST /api/observability/v1/traces
20:00:42.398 fetched POST /api/observability/v1/traces 204
20:00:43.420 in Ping 
20:00:45.087 fetch POST /api/observability/v1/traces
20:00:45.087 fetched POST /api/observability/v1/traces 204
20:00:46.655 fetch POST /api/observability/v1/traces
20:00:46.655 fetched POST /api/observability/v1/traces 204
20:00:48.421 in Ping 
20:00:48.664 rpc 31 server.reportClientActivity
20:00:49.664 fetch POST /api/observability/v1/traces
20:00:49.665 fetched POST /api/observability/v1/traces 204
20:00:51.038 rpc 32 orchestration.dispatchCommand thread.visit
20:00:51.038 exit 32 {"_tag":"Success","value":{"sequence":0}}
20:00:52.038 fetch POST /api/observability/v1/traces
20:00:52.038 fetched POST /api/observability/v1/traces 204
20:00:53.421 in Ping 
20:00:58.422 in Ping 
20:01:03.422 in Ping 
20:01:03.667 in Interrupt 31
20:01:04.668 fetch POST /api/observability/v1/traces
20:01:04.668 fetched POST /api/observability/v1/traces 204
20:01:08.422 in Ping 
20:01:13.423 in Ping 
20:01:13.664 rpc 33 server.reportClientActivity
20:01:14.664 fetch POST /api/observability/v1/traces
20:01:14.664 fetched POST /api/observability/v1/traces 204
20:01:18.426 in Ping 
20:01:23.429 in Ping 
20:01:28.433 in Ping 
20:01:28.665 in Interrupt 33
20:01:29.666 fetch POST /api/observability/v1/traces
20:01:29.666 fetched POST /api/observability/v1/traces 204
20:01:33.437 in Ping 
20:01:38.441 in Ping 
20:01:38.666 rpc 34 server.reportClientActivity
20:01:39.666 fetch POST /api/observability/v1/traces
20:01:39.666 fetched POST /api/observability/v1/traces 204
20:01:43.445 in Ping 
20:01:48.252 chunk 17 44061 bytes
20:01:48.257 in Ack 17
20:01:48.449 in Ping 
20:01:48.502 chunk 17 44267 bytes
20:01:48.504 in Ack 17
20:01:48.753 chunk 17 44545 bytes
20:01:48.754 in Ack 17
20:01:49.003 chunk 17 45097 bytes
20:01:49.004 in Ack 17
20:01:49.253 chunk 17 45605 bytes
20:01:49.254 in Ack 17
20:01:49.267 fetch POST /api/observability/v1/traces
20:01:49.267 fetched POST /api/observability/v1/traces 204
20:01:49.503 chunk 17 45717 bytes
20:01:49.504 in Ack 17
20:01:50.059 chunk 17 46722 bytes
20:01:50.061 in Ack 17
20:01:50.505 fetch POST /api/observability/v1/traces
20:01:50.505 fetched POST /api/observability/v1/traces 204
20:01:50.609 rpc 35 orchestration.dispatchCommand runtime-request.respond
20:01:50.821 chunk 17 50218 bytes
20:01:50.822 in Ack 17
20:01:51.282 exit 35 {"_tag":"Success","value":{"sequence":97}}
20:01:51.321 chunk 17 52689 bytes
20:01:51.322 in Ack 17
20:01:51.571 chunk 17 52729 bytes
20:01:51.572 in Ack 17
20:01:51.608 fetch POST /api/observability/v1/traces
20:01:51.608 fetched POST /api/observability/v1/traces 204
20:01:51.822 chunk 17 52749 bytes
20:01:51.823 in Ack 17
20:01:51.973 rpc 36 orchestration.dispatchCommand runtime-request.respond
20:01:52.072 chunk 17 52822 bytes
20:01:52.073 in Ack 17
20:01:52.322 chunk 17 52852 bytes
20:01:52.323 in Ack 17
20:01:52.582 chunk 17 52922 bytes
20:01:52.583 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
20:01:52.584 in Ack 17
20:01:52.833 chunk 17 52942 bytes
20:01:52.834 fetch POST /api/observability/v1/traces
20:01:52.834 fetched POST /api/observability/v1/traces 204
20:01:52.835 in Ack 17
20:01:53.083 chunk 17 52972 bytes
20:01:53.084 in Ack 17
20:01:53.333 chunk 17 52954 bytes
20:01:53.334 in Ack 17
20:01:53.453 in Ping 
20:01:53.595 chunk 11 2896 bytes
20:01:53.596 chunk 17 52974 bytes
20:01:53.596 in Ack 11
20:01:53.602 rpc 37 orchestration.dispatchCommand thread.visit
20:01:53.602 exit 37 {"_tag":"Success","value":{"sequence":0}}
20:01:53.609 in Ack 17
20:01:53.692 in Interrupt 34
20:01:53.837 fetch POST /api/observability/v1/traces
20:01:53.837 fetched POST /api/observability/v1/traces 204
20:01:53.846 chunk 17 53004 bytes
20:01:53.847 in Ack 17
20:01:53.944 rpc 38 server.reportClientActivity
20:01:54.096 chunk 17 53082 bytes
20:01:54.097 in Ack 17
20:01:54.345 chunk 11 2896 bytes
20:01:54.346 chunk 17 53102 bytes
20:01:54.346 in Ack 11
20:01:54.351 in Ack 17
20:01:54.596 chunk 17 53122 bytes
20:01:54.598 in Ack 17
20:01:54.846 chunk 17 56262 bytes
20:01:54.848 in Ack 17
20:01:54.856 in Interrupt 27
20:01:54.883 rpc 39 subscribeVcsStatus
20:01:54.930 exit 31 {"_tag":"Success","value":null}
20:01:54.930 exit 33 {"_tag":"Success","value":null}
20:01:54.930 exit 34 {"_tag":"Success","value":null}
20:01:54.930 exit 38 {"_tag":"Success","value":null}
20:01:54.931 fetch POST /api/observability/v1/traces
20:01:54.931 fetched POST /api/observability/v1/traces 204
20:01:55.099 chunk 17 59569 bytes
20:01:55.100 in Ack 17
20:01:55.189 rpc 40 server.reportClientActivity
20:01:55.333 chunk 39 2331 bytes
20:01:55.334 in Ack 39
20:01:55.350 chunk 17 60619 bytes
20:01:55.351 in Ack 17
20:01:55.600 chunk 17 60639 bytes
20:01:55.601 in Ack 17
20:01:55.850 chunk 17 60669 bytes
20:01:55.851 in Ack 17
20:01:55.898 exit 40 {"_tag":"Success","value":null}
20:01:55.935 fetch POST /api/observability/v1/traces
20:01:55.935 fetched POST /api/observability/v1/traces 204
20:01:56.100 chunk 17 60689 bytes
20:01:56.101 in Ack 17
20:01:56.350 chunk 17 60719 bytes
20:01:56.351 in Ack 17
20:01:56.624 chunk 17 60749 bytes
20:01:56.625 in Ack 17
20:01:56.875 chunk 17 60769 bytes
20:01:56.876 in Ack 17
20:01:57.106 fetch POST /api/observability/v1/traces
20:01:57.106 fetched POST /api/observability/v1/traces 204
20:01:57.125 chunk 17 60799 bytes
20:01:57.126 in Ack 17
20:01:57.375 chunk 17 60819 bytes
20:01:57.376 in Ack 17
20:01:57.646 chunk 17 60849 bytes
20:01:57.647 in Ack 17
20:01:57.896 chunk 17 60869 bytes
20:01:57.897 in Ack 17
20:01:58.127 fetch POST /api/observability/v1/traces
20:01:58.127 fetched POST /api/observability/v1/traces 204
20:01:58.147 chunk 17 60899 bytes
20:01:58.148 in Ack 17
20:01:58.398 chunk 17 60919 bytes
20:01:58.399 in Ack 17
20:01:58.457 in Ping 
20:01:58.656 chunk 17 60949 bytes
20:01:58.658 in Ack 17
20:01:58.906 chunk 17 60971 bytes
20:01:58.907 in Ack 17
20:01:59.149 fetch POST /api/observability/v1/traces
20:01:59.149 fetched POST /api/observability/v1/traces 204
20:01:59.156 chunk 11 2896 bytes
20:01:59.157 chunk 17 61950 bytes
20:01:59.157 in Ack 11
20:01:59.159 in Ack 17
20:01:59.408 chunk 17 61995 bytes
20:01:59.409 in Ack 17
20:01:59.678 chunk 17 62040 bytes
20:01:59.680 in Ack 17
20:01:59.928 chunk 17 62085 bytes
20:01:59.933 in Ack 17
20:02:00.160 fetch POST /api/observability/v1/traces
20:02:00.160 fetched POST /api/observability/v1/traces 204
20:02:00.178 chunk 17 62115 bytes
20:02:00.179 in Ack 17
20:02:00.428 chunk 17 62166 bytes
20:02:00.429 in Ack 17
20:02:00.687 chunk 17 62196 bytes
20:02:00.688 in Ack 17
20:02:00.939 chunk 17 62247 bytes
20:02:00.941 in Ack 17
20:02:01.180 fetch POST /api/observability/v1/traces
20:02:01.180 fetched POST /api/observability/v1/traces 204
20:02:01.204 chunk 17 62277 bytes
20:02:01.205 in Ack 17
20:02:01.454 chunk 17 62322 bytes
20:02:01.455 in Ack 17
20:02:01.704 chunk 17 62358 bytes
20:02:01.706 in Ack 17
20:02:01.954 chunk 17 62388 bytes
20:02:01.955 in Ack 17
20:02:02.205 chunk 17 62433 bytes
20:02:02.206 in Ack 17
20:02:02.258 fetch POST /api/observability/v1/traces
20:02:02.258 fetched POST /api/observability/v1/traces 204
20:02:02.455 chunk 17 62487 bytes
20:02:02.456 in Ack 17
20:02:02.705 chunk 17 62517 bytes
20:02:02.707 in Ack 17
20:02:02.956 chunk 17 62562 bytes
20:02:02.957 in Ack 17
20:02:03.206 chunk 17 62601 bytes
20:02:03.207 in Ack 17
20:02:03.456 chunk 17 62649 bytes
20:02:03.457 in Ack 17
20:02:03.512 fetch POST /api/observability/v1/traces
20:02:03.512 fetched POST /api/observability/v1/traces 204
20:02:03.513 in Ping 
20:02:03.604 rpc 41 orchestration.dispatchCommand thread.visit
20:02:03.604 exit 41 {"_tag":"Success","value":{"sequence":0}}
20:02:03.664 rpc 42 server.reportClientActivity
20:02:03.706 chunk 17 62688 bytes
20:02:03.707 in Ack 17
20:02:03.956 chunk 17 62733 bytes
20:02:03.958 in Ack 17
20:02:04.205 chunk 11 2896 bytes
20:02:04.206 in Ack 11
20:02:04.215 chunk 17 62769 bytes
20:02:04.217 in Ack 17
20:02:04.465 chunk 17 62784 bytes
20:02:04.466 in Ack 17
20:02:04.604 fetch POST /api/observability/v1/traces
20:02:04.604 fetched POST /api/observability/v1/traces 204
20:02:04.715 chunk 17 62838 bytes
20:02:04.717 in Ack 17
20:02:04.965 chunk 17 62901 bytes
20:02:04.966 in Ack 17
20:02:05.045 exit 42 {"_tag":"Success","value":null}
20:02:05.215 chunk 17 62999 bytes
20:02:05.216 in Ack 17
20:02:05.457 chunk 11 2835 bytes
20:02:05.457 in Ack 11
20:02:05.465 chunk 17 63131 bytes
20:02:05.466 in Ack 17
20:02:05.737 chunk 11 2832 bytes
20:02:05.737 chunk 17 63155 bytes
20:02:05.737 fetch POST /api/observability/v1/traces
20:02:05.737 fetched POST /api/observability/v1/traces 204
20:02:05.738 in Ack 11
20:02:05.740 rpc 43 orchestration.dispatchCommand thread.visit
20:02:05.740 exit 43 {"_tag":"Success","value":{"sequence":0}}
20:02:05.744 in Ack 17
