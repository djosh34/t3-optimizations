# case70, webapp, warm start, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 99 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 6.5s |
| Prompt reached the agent | 4.4s |
| Time to first text | 15.8s |
| Whole reply shown | 97.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 12.8s / 2.5s / 11.5s |
| Longest silence | 52.0s |
| Approval shown after it was asked | 42.7s |
| Bytes on the wire down / up | 116652 / 52573 |
| Bytes the app received / sent | 93797 / 99563 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 5.5 | 0 | 222 |
| 20 | 6.4 | 367 | 621 |
| 30 | 9.6 | 635 | 1023 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 0.0 | 1077 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 4.8 | 1077 | 1270 |
| 90 | 9.5 | 1288 | 1596 |
| 97 | 8.3 | 1579 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 23/901 146/550","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 36/778","providerThreads":"810/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152010248,"silences":0,"longestSilenceMs":1356,"longestRecoveredGapMs":1356},"stats":{"sendRate":3375,"deliveredRate":2103.921568627451,"loss":0,"srttMs":116.58213515207537,"minRttMs":866,"sentPackets":118,"sentWireBytes":37281,"receivedPackets":121,"receivedWireBytes":58531,"lastHeardMs":11311,"unsyncedBytes":1009,"inflightPackets":26,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 70/225","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 104/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152020268,"silences":1,"longestSilenceMs":2031,"longestRecoveredGapMs":2031},"stats":{"sendRate":1687.5,"deliveredRate":1328.5175047419543,"loss":0.29901923076923076,"srttMs":1680.2381880340859,"minRttMs":866,"sentPackets":146,"sentWireBytes":47994,"receivedPackets":132,"receivedWireBytes":65904,"lastHeardMs":21331,"unsyncedBytes":0,"inflightPackets":23,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1070/1083 658/849","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 545/598","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152030794,"silences":1,"longestSilenceMs":2031,"longestRecoveredGapMs":2031},"stats":{"sendRate":1687.5,"deliveredRate":0,"loss":0.29901923076923076,"srttMs":1680.2381880340859,"minRttMs":866,"sentPackets":198,"sentWireBytes":60393,"receivedPackets":160,"receivedWireBytes":76788,"lastHeardMs":31857,"unsyncedBytes":500,"inflightPackets":58,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1075/1083 695/849","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 1098/1173","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152040793,"silences":1,"longestSilenceMs":2031,"longestRecoveredGapMs":2031},"stats":{"sendRate":1687.5,"deliveredRate":0,"loss":0.29901923076923076,"srttMs":1680.2381880340859,"minRttMs":866,"sentPackets":220,"sentWireBytes":66715,"receivedPackets":193,"receivedWireBytes":87416,"lastHeardMs":41856,"unsyncedBytes":500,"inflightPackets":80,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1083/1083 752/849 1101/1554 144/932","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 46/638","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152050958,"silences":1,"longestSilenceMs":2031,"longestRecoveredGapMs":2031},"stats":{"sendRate":632.8125,"deliveredRate":124.07796718313703,"loss":0.8741902834008097,"srttMs":2511.693728488996,"minRttMs":866,"sentPackets":251,"sentWireBytes":75428,"receivedPackets":223,"receivedWireBytes":98241,"lastHeardMs":52021,"unsyncedBytes":500,"inflightPackets":99,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 933/1108 849/849 1554/1554 919/932","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 46/638","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152061033,"silences":2,"longestSilenceMs":2035,"longestRecoveredGapMs":2035},"stats":{"sendRate":632.8125,"deliveredRate":167.26031328567257,"loss":0.7746032388663968,"srttMs":2346.2320124278713,"minRttMs":866,"sentPackets":291,"sentWireBytes":81709,"receivedPackets":251,"receivedWireBytes":107397,"lastHeardMs":62096,"unsyncedBytes":0,"inflightPackets":100,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1108/1108 849/849 1554/1554 932/932","thread":"951/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 79/638","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"830/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152071491,"silences":2,"longestSilenceMs":2035,"longestRecoveredGapMs":2035},"stats":{"sendRate":949.21875,"deliveredRate":0,"loss":0.7172222672064776,"srttMs":2084.9276345150893,"minRttMs":1114,"sentPackets":334,"sentWireBytes":88608,"receivedPackets":281,"receivedWireBytes":116266,"lastHeardMs":72554,"unsyncedBytes":0,"inflightPackets":88,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1108/1108 849/849 1554/1554 932/932","thread":"591/951","runs":"1014/1014 971/971","messages":"456/456 550/550 140/901 550/550","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 160/638","providerTurns":"778/778 135/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"857/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152081467,"silences":2,"longestSilenceMs":2035,"longestRecoveredGapMs":2035},"stats":{"sendRate":2135.7421875,"deliveredRate":735.5003186743148,"loss":0.7142857142857143,"srttMs":1751.1256333403048,"minRttMs":1074,"sentPackets":385,"sentWireBytes":98342,"receivedPackets":314,"receivedWireBytes":125703,"lastHeardMs":82530,"unsyncedBytes":0,"inflightPackets":91,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1108/1108 849/849 1554/1554 932/932","thread":"951/951","runs":"1014/1014 971/971 811/839","messages":"456/456 550/550 182/901 550/550","assistant":"32/32 21/21 1340/1340 103/198","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 416/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"243/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152092088,"silences":2,"longestSilenceMs":2035,"longestRecoveredGapMs":2035},"stats":{"sendRate":1500,"deliveredRate":219.78327313142537,"loss":0.8666666666666667,"srttMs":2291.721188026171,"minRttMs":1074,"sentPackets":439,"sentWireBytes":111867,"receivedPackets":345,"receivedWireBytes":137536,"lastHeardMs":93150,"unsyncedBytes":0,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 1108/1108 849/849 1554/1554 932/932 186/1108","thread":"951/951","runs":"1014/1014 971/971 811/839","messages":"456/456 550/550 182/901 550/550","assistant":"32/32 21/21 1340/1340 698/723","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 510/778","providerThreads":"740/810","providerSessions":"2696/2696","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"734/734","runtimeRequests":"255/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":291,"lastDataAt":1794152104105,"silences":2,"longestSilenceMs":2035,"longestRecoveredGapMs":2035},"stats":{"sendRate":1500,"deliveredRate":207.82608695652172,"loss":0.6,"srttMs":2382.2396595825376,"minRttMs":1074,"sentPackets":494,"sentWireBytes":123152,"receivedPackets":376,"receivedWireBytes":150033,"lastHeardMs":105168,"unsyncedBytes":317,"inflightPackets":75,"duplicateBytes":0}}
- shim trace
15:33:19.325 fetch GET /api/auth/session
15:33:19.326 fetched GET /api/auth/session 200
15:33:19.423 fetch GET /.well-known/t3/environment
15:33:19.423 fetched GET /.well-known/t3/environment 200
15:33:19.432 fetch GET /.well-known/t3/environment
15:33:19.432 fetched GET /.well-known/t3/environment 200
15:33:19.436 rpc 0 subscribeServerConfig
15:33:19.437 chunk 0 38722 bytes
15:33:19.479 in Ack 0
15:33:19.480 rpc 1 subscribeServerLifecycle
15:33:19.484 rpc 2 subscribeServerLifecycle
15:33:19.487 fetch GET /api/auth/session
15:33:19.488 fetched GET /api/auth/session 200
15:33:19.489 rpc 3 subscribeDeviceState
15:33:19.490 rpc 4 preview.reportProfiles
15:33:19.556 rpc 5 subscribeProjectClones
15:33:19.574 fetch GET /api/orchestration/threads/mcp%3A241c54b8-15e7-4e75-99e4-74d2d6d8431b/bounded
15:33:19.580 fetch GET /api/orchestration/shell
15:33:19.581 fetched GET /api/orchestration/shell 200
15:33:19.583 rpc 6 orchestration.subscribeShell
15:33:19.583 chunk 6 2790 bytes
15:33:19.604 rpc 7 server.refreshProviders
15:33:19.645 rpc 8 orchestration.dispatchCommand thread.visit
15:33:19.662 rpc 9 subscribeTerminalMetadata
15:33:19.663 rpc 10 subscribeVcsStatus
15:33:19.664 rpc 11 subscribePreviewEvents
15:33:19.667 rpc 12 assets.createUrl
15:33:19.682 rpc 13 preview.list
15:33:19.683 rpc 14 projects.readFile
15:33:19.690 in Ack 6
15:33:19.791 exit 8 {"_tag":"Success","value":{"sequence":38}}
15:33:19.791 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"7777ecd9-87ba-4139-9136-210f019bbc51","sessions":[]}}
15:33:19.791 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:33:19.918 rpc 15 server.reportClientActivity
15:33:19.919 chunk 1 3872 bytes
15:33:19.919 in Ack 1
15:33:20.423 fetch POST /api/observability/v1/traces
15:33:20.423 fetched POST /api/observability/v1/traces 204
15:33:20.439 chunk 2 3872 bytes
15:33:20.439 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:33:04.023Z","configurableProviders":false,"con
15:33:20.439 chunk 10 777 bytes
15:33:20.439 in Ack 2
15:33:20.443 in Ack 10
15:33:20.571 chunk 6 952 bytes
15:33:20.571 exit 12 {"_tag":"Success","value":{"expiresAt":1794155400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:33:20.571 in Ack 6
15:33:20.820 chunk 3 876 bytes
15:33:20.820 exit 4 {"_tag":"Success","value":null}
15:33:20.820 chunk 5 4 bytes
15:33:20.820 chunk 9 36 bytes
15:33:20.820 exit 15 {"_tag":"Success","value":null}
15:33:20.820 chunk 6 2809 bytes
15:33:20.821 in Ack 3
15:33:20.822 rpc 16 preview.reportProfiles
15:33:20.822 in Ack 5
15:33:20.822 in Ack 9
15:33:20.823 in Ack 6
15:33:20.832 rpc 17 server.refreshProviders
15:33:20.832 exit 17 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:33:04.023Z","configurableProviders":false,"con
15:33:20.976 exit 16 {"_tag":"Success","value":null}
15:33:21.118 fetched GET /api/orchestration/threads/mcp%3A241c54b8-15e7-4e75-99e4-74d2d6d8431b/bounded 200
15:33:21.123 rpc 18 orchestration.subscribeThread
15:33:21.123 chunk 18 8305 bytes
15:33:21.146 in Interrupt 10
15:33:21.190 rpc 19 subscribeVcsStatus
15:33:21.231 in Ack 18
15:33:21.309 chunk 19 777 bytes
15:33:21.309 in Ack 19
15:33:21.385 chunk 18 11282 bytes
15:33:21.386 in Ack 18
15:33:21.440 fetch POST /api/observability/v1/traces
15:33:21.441 fetched POST /api/observability/v1/traces 204
15:33:21.442 rpc 20 server.reportClientActivity
15:33:21.637 chunk 18 13532 bytes
15:33:21.638 in Ack 18
15:33:21.888 chunk 18 18063 bytes
15:33:21.891 in Ack 18
15:33:21.967 exit 20 {"_tag":"Success","value":null}
15:33:22.173 rpc 21 attachments.createUploadUrl
15:33:22.180 chunk 18 21239 bytes
15:33:22.377 in Ack 18
15:33:22.385 in Interrupt 19
15:33:22.388 rpc 22 subscribeVcsStatus
15:33:22.442 fetch POST /api/observability/v1/traces
15:33:22.442 fetched POST /api/observability/v1/traces 204
15:33:22.448 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-faf9534f-a9ab-4e8c-85a0-a091e9170cb3","expiresAt":1794152602409,"relativeUrl":"/api/attachments/upload/eyJ2ZX
15:33:22.448 chunk 22 777 bytes
15:33:22.450 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWZhZjk1MzRmLWE5YWItNGU4Yy04NWEwLWEwOTFlOTE3MGNiMyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1MjYwMjQwOX0.tjcVjTowyt8MXbQO60fseTVIXAzrQmRP6pVNunWSJyk
15:33:22.450 in Ack 22
15:33:22.639 rpc 23 server.reportClientActivity
15:33:23.452 fetch POST /api/observability/v1/traces
15:33:23.452 fetched POST /api/observability/v1/traces 204
15:33:24.297 exit 23 {"_tag":"Success","value":null}
15:33:24.434 in Ping 
15:33:25.298 fetch POST /api/observability/v1/traces
15:33:25.299 fetched POST /api/observability/v1/traces 204
15:33:25.826 rpc 24 orchestration.dispatchCommand message.dispatch
15:33:26.582 exit 24 {"_tag":"Success","value":{"sequence":45}}
15:33:26.587 rpc 25 attachments.delete
15:33:26.825 fetch POST /api/observability/v1/traces
15:33:26.825 fetched POST /api/observability/v1/traces 204
15:33:27.082 chunk 18 23705 bytes
15:33:27.083 in Ack 18
15:33:27.333 chunk 18 23727 bytes
15:33:27.335 in Ack 18
15:33:27.451 exit 25 {"_tag":"Success","value":null}
15:33:27.597 chunk 6 952 bytes
15:33:27.598 chunk 18 23729 bytes
15:33:27.598 in Ack 6
15:33:27.611 in Ack 18
15:33:27.849 chunk 6 2831 bytes
15:33:27.855 chunk 18 24701 bytes
15:33:27.860 in Ack 6
15:33:27.862 rpc 26 orchestration.dispatchCommand thread.visit
15:33:27.905 in Ack 18
15:33:27.998 exit 26 {"_tag":"Success","value":{"sequence":68}}
15:33:28.085 fetch POST /api/observability/v1/traces
15:33:28.085 fetched POST /api/observability/v1/traces 204
15:33:28.101 chunk 6 952 bytes
15:33:28.101 in Ack 6
15:33:28.351 chunk 6 2831 bytes
15:33:28.352 chunk 18 28157 bytes
15:33:28.352 in Ack 6
15:33:28.357 in Ack 18
15:33:28.361 rpc 27 assets.createUrl
15:33:28.852 chunk 18 32170 bytes
15:33:28.861 in Ack 18
15:33:28.877 in Interrupt 22
15:33:28.882 rpc 28 subscribeVcsStatus
15:33:29.114 fetch POST /api/observability/v1/traces
15:33:29.114 fetched POST /api/observability/v1/traces 204
15:33:29.114 chunk 18 32004 bytes
15:33:29.116 in Ack 18
15:33:29.134 rpc 29 server.reportClientActivity
15:33:29.457 in Ping 
15:33:30.147 fetch POST /api/observability/v1/traces
15:33:30.147 fetched POST /api/observability/v1/traces 204
15:33:30.452 rpc 30 orchestration.dispatchCommand message.dispatch
15:33:31.455 fetch POST /api/observability/v1/traces
15:33:31.455 fetched POST /api/observability/v1/traces 204
15:33:31.610 chunk 18 32004 bytes
15:33:31.610 chunk 28 853 bytes
15:33:31.611 in Ack 18
15:33:31.622 in Ack 28
15:33:32.130 chunk 18 32576 bytes
15:33:32.132 in Ack 18
15:33:32.613 fetch POST /api/observability/v1/traces
15:33:32.613 fetched POST /api/observability/v1/traces 204
15:33:34.458 in Ping 
15:33:35.588 chunk 18 31766 bytes
15:33:35.599 in Ack 18
15:33:36.539 chunk 6 952 bytes
15:33:36.547 chunk 18 34234 bytes
15:33:36.548 exit 30 {"_tag":"Success","value":{"sequence":75}}
15:33:36.549 in Ack 6
15:33:36.838 in Ack 18
15:33:36.937 fetch POST /api/observability/v1/traces
15:33:36.937 fetched POST /api/observability/v1/traces 204
15:33:37.557 chunk 18 34276 bytes
15:33:37.594 in Ack 18
15:33:37.995 chunk 18 34296 bytes
15:33:37.996 in Ack 18
15:33:38.596 fetch POST /api/observability/v1/traces
15:33:38.596 fetched POST /api/observability/v1/traces 204
15:33:39.458 in Ping 
15:33:39.717 chunk 18 34318 bytes
15:33:39.718 in Ack 18
15:33:40.269 chunk 18 34358 bytes
15:33:40.271 in Ack 18
15:33:40.721 fetch POST /api/observability/v1/traces
15:33:40.721 fetched POST /api/observability/v1/traces 204
15:33:41.093 chunk 18 34430 bytes
15:33:41.094 in Ack 18
15:33:41.527 chunk 18 34450 bytes
15:33:41.529 in Ack 18
15:33:41.961 chunk 18 34470 bytes
15:33:41.967 in Ack 18
15:33:42.100 fetch POST /api/observability/v1/traces
15:33:42.100 fetched POST /api/observability/v1/traces 204
15:33:42.395 chunk 18 34480 bytes
15:33:42.397 in Ack 18
15:33:42.823 chunk 18 34502 bytes
15:33:42.833 in Ack 18
15:33:43.251 chunk 18 34522 bytes
15:33:43.252 in Ack 18
15:33:43.451 fetch POST /api/observability/v1/traces
15:33:43.451 fetched POST /api/observability/v1/traces 204
15:33:43.503 chunk 18 34544 bytes
15:33:43.504 in Ack 18
15:33:43.754 chunk 18 34554 bytes
15:33:43.756 in Ack 18
15:33:44.005 chunk 18 34564 bytes
15:33:44.007 in Ack 18
15:33:44.135 in Interrupt 29
15:33:44.260 chunk 18 34574 bytes
15:33:44.261 in Ack 18
15:33:44.459 in Ping 
15:33:44.506 fetch POST /api/observability/v1/traces
15:33:44.506 fetched POST /api/observability/v1/traces 204
15:33:44.677 rpc 31 server.reportClientActivity
15:33:45.045 chunk 18 34646 bytes
15:33:45.046 in Ack 18
15:33:45.356 chunk 18 34796 bytes
15:33:45.357 in Ack 18
15:33:45.607 chunk 18 34892 bytes
15:33:45.608 in Ack 18
15:33:45.683 fetch POST /api/observability/v1/traces
15:33:45.684 fetched POST /api/observability/v1/traces 204
15:33:46.070 chunk 18 35058 bytes
15:33:46.071 in Ack 18
15:33:46.325 chunk 18 35098 bytes
15:33:46.326 in Ack 18
15:33:46.584 chunk 18 35108 bytes
15:33:46.585 in Ack 18
15:33:46.973 chunk 18 35130 bytes
15:33:46.975 in Ack 18
15:33:47.072 fetch POST /api/observability/v1/traces
15:33:47.072 fetched POST /api/observability/v1/traces 204
15:33:47.851 chunk 18 35170 bytes
15:33:47.852 in Ack 18
15:33:48.106 chunk 18 35180 bytes
15:33:48.112 in Ack 18
15:33:48.356 chunk 18 35190 bytes
15:33:48.357 in Ack 18
15:33:48.767 chunk 18 35210 bytes
15:33:48.768 in Ack 18
15:33:48.854 fetch POST /api/observability/v1/traces
15:33:48.855 fetched POST /api/observability/v1/traces 204
15:33:49.324 chunk 18 35222 bytes
15:33:49.325 in Ack 18
15:33:49.460 in Ping 
15:33:49.804 chunk 18 35262 bytes
15:33:49.805 in Ack 18
15:33:50.314 chunk 18 35272 bytes
15:33:50.316 in Ack 18
15:33:50.329 fetch POST /api/observability/v1/traces
15:33:50.329 fetched POST /api/observability/v1/traces 204
15:33:50.795 chunk 18 35310 bytes
15:33:50.796 in Ack 18
15:33:51.047 chunk 18 35320 bytes
15:33:51.048 in Ack 18
15:33:51.392 chunk 18 35358 bytes
15:33:51.393 in Ack 18
15:33:51.798 fetch POST /api/observability/v1/traces
15:33:51.798 fetched POST /api/observability/v1/traces 204
15:33:52.032 chunk 18 35396 bytes
15:33:52.034 in Ack 18
15:33:52.422 chunk 18 35408 bytes
15:33:52.426 in Ack 18
15:33:52.674 chunk 18 35438 bytes
15:33:52.680 in Ack 18
15:33:53.035 fetch POST /api/observability/v1/traces
15:33:53.035 fetched POST /api/observability/v1/traces 204
15:33:53.155 chunk 18 35450 bytes
15:33:53.156 in Ack 18
15:33:53.713 chunk 18 35520 bytes
15:33:53.714 in Ack 18
15:33:53.964 chunk 18 35560 bytes
15:33:53.965 in Ack 18
15:33:54.157 fetch POST /api/observability/v1/traces
15:33:54.157 fetched POST /api/observability/v1/traces 204
15:33:54.460 in Ping 
15:33:54.467 chunk 18 35592 bytes
15:33:54.469 in Ack 18
15:33:54.722 chunk 18 35602 bytes
15:33:54.723 in Ack 18
15:33:55.059 chunk 18 35642 bytes
15:33:55.060 in Ack 18
15:33:55.318 chunk 18 35652 bytes
15:33:55.319 in Ack 18
15:33:55.470 fetch POST /api/observability/v1/traces
15:33:55.470 fetched POST /api/observability/v1/traces 204
15:33:55.713 chunk 18 35682 bytes
15:33:55.715 in Ack 18
15:33:55.970 chunk 18 35694 bytes
15:33:55.972 in Ack 18
15:33:56.227 chunk 18 35740 bytes
15:33:56.228 in Ack 18
15:33:56.615 chunk 18 35742 bytes
15:33:56.618 in Ack 18
15:33:56.719 fetch POST /api/observability/v1/traces
15:33:56.719 fetched POST /api/observability/v1/traces 204
15:33:56.874 chunk 18 35800 bytes
15:33:56.875 in Ack 18
15:33:57.148 chunk 18 35840 bytes
15:33:57.151 in Ack 18
15:33:57.460 chunk 18 35862 bytes
15:33:57.462 in Ack 18
15:33:57.713 chunk 18 35872 bytes
15:33:57.715 in Ack 18
15:33:57.884 fetch POST /api/observability/v1/traces
15:33:57.884 fetched POST /api/observability/v1/traces 204
15:33:57.965 chunk 18 35894 bytes
15:33:57.967 in Ack 18
15:33:58.611 chunk 18 35904 bytes
15:33:58.612 in Ack 18
15:33:58.941 chunk 18 36006 bytes
15:33:58.942 in Ack 18
15:33:58.971 fetch POST /api/observability/v1/traces
15:33:58.971 fetched POST /api/observability/v1/traces 204
15:33:59.199 chunk 18 36016 bytes
15:33:59.200 in Ack 18
15:33:59.466 chunk 18 36126 bytes
15:33:59.467 in Ping 
15:33:59.468 in Ack 18
15:33:59.677 in Interrupt 31
15:33:59.718 chunk 18 36164 bytes
15:33:59.719 in Ack 18
15:34:00.029 chunk 18 36224 bytes
15:34:00.030 in Ack 18
15:34:00.207 fetch POST /api/observability/v1/traces
15:34:00.207 fetched POST /api/observability/v1/traces 204
15:34:00.286 chunk 18 36272 bytes
15:34:00.287 in Ack 18
15:34:00.540 chunk 18 36294 bytes
15:34:00.541 in Ack 18
15:34:00.794 chunk 18 36420 bytes
15:34:00.796 in Ack 18
15:34:01.133 chunk 18 36460 bytes
15:34:01.134 in Ack 18
15:34:01.288 fetch POST /api/observability/v1/traces
15:34:01.288 fetched POST /api/observability/v1/traces 204
15:34:01.525 chunk 18 36470 bytes
15:34:01.527 in Ack 18
15:34:01.775 chunk 18 36490 bytes
15:34:01.776 in Ack 18
15:34:02.026 chunk 18 36520 bytes
15:34:02.027 in Ack 18
15:34:02.277 chunk 18 36530 bytes
15:34:02.278 in Ack 18
15:34:02.528 fetch POST /api/observability/v1/traces
15:34:02.528 fetched POST /api/observability/v1/traces 204
15:34:02.533 chunk 18 36550 bytes
15:34:02.539 in Ack 18
15:34:02.961 chunk 18 36560 bytes
15:34:02.962 in Ack 18
15:34:03.389 chunk 18 36590 bytes
15:34:03.390 in Ack 18
15:34:03.541 fetch POST /api/observability/v1/traces
15:34:03.541 fetched POST /api/observability/v1/traces 204
15:34:03.647 chunk 18 36628 bytes
15:34:03.650 in Ack 18
15:34:04.125 chunk 18 36638 bytes
15:34:04.129 in Ack 18
15:34:04.374 chunk 18 36648 bytes
15:34:04.375 in Ack 18
15:34:04.470 in Ping 
15:34:04.625 chunk 18 36668 bytes
15:34:04.626 in Ack 18
15:34:04.655 fetch POST /api/observability/v1/traces
15:34:04.655 fetched POST /api/observability/v1/traces 204
15:34:04.875 chunk 18 36630 bytes
15:34:04.876 in Ack 18
15:34:05.125 chunk 18 36650 bytes
15:34:05.126 in Ack 18
15:34:05.375 chunk 18 36660 bytes
15:34:05.378 in Ack 18
15:34:05.625 chunk 18 36680 bytes
15:34:05.626 in Ack 18
15:34:05.878 fetch POST /api/observability/v1/traces
15:34:05.878 fetched POST /api/observability/v1/traces 204
15:34:06.040 chunk 18 36698 bytes
15:34:06.041 in Ack 18
15:34:06.938 chunk 18 36852 bytes
15:34:06.939 in Ack 18
15:34:07.049 fetch POST /api/observability/v1/traces
15:34:07.049 fetched POST /api/observability/v1/traces 204
15:34:09.470 in Ping 
15:34:09.679 rpc 32 server.reportClientActivity
15:34:10.680 fetch POST /api/observability/v1/traces
15:34:10.680 fetched POST /api/observability/v1/traces 204
15:34:12.659 fetch POST /api/observability/v1/traces
15:34:12.659 fetched POST /api/observability/v1/traces 204
15:34:13.926 chunk 18 36900 bytes
15:34:13.928 in Ack 18
15:34:14.361 chunk 18 40414 bytes
15:34:14.362 in Ack 18
15:34:14.472 in Ping 
15:34:14.930 fetch POST /api/observability/v1/traces
15:34:14.930 fetched POST /api/observability/v1/traces 204
15:34:19.477 in Ping 
15:34:21.457 chunk 18 40416 bytes
15:34:21.459 in Ack 18
15:34:22.461 fetch POST /api/observability/v1/traces
15:34:22.461 fetched POST /api/observability/v1/traces 204
15:34:23.485 chunk 18 42505 bytes
15:34:23.487 in Ack 18
15:34:24.481 in Ping 
15:34:24.488 fetch POST /api/observability/v1/traces
15:34:24.488 fetched POST /api/observability/v1/traces 204
15:34:24.680 in Interrupt 32
15:34:25.681 fetch POST /api/observability/v1/traces
15:34:25.681 fetched POST /api/observability/v1/traces 204
15:34:29.218 chunk 6 3205 bytes
15:34:29.219 in Ack 6
15:34:29.225 rpc 33 orchestration.dispatchCommand thread.visit
15:34:29.496 in Ping 
15:34:30.229 fetch POST /api/observability/v1/traces
15:34:30.229 fetched POST /api/observability/v1/traces 204
15:34:30.901 exit 33 {"_tag":"Success","value":{"sequence":93}}
15:34:31.905 fetch POST /api/observability/v1/traces
15:34:31.905 fetched POST /api/observability/v1/traces 204
15:34:32.253 chunk 6 952 bytes
15:34:32.254 in Ack 6
15:34:33.255 fetch POST /api/observability/v1/traces
15:34:33.255 fetched POST /api/observability/v1/traces 204
15:34:34.502 in Ping 
15:34:34.716 rpc 34 server.reportClientActivity
15:34:35.716 fetch POST /api/observability/v1/traces
15:34:35.716 fetched POST /api/observability/v1/traces 204
15:34:39.504 in Ping 
15:34:40.243 fetch POST /api/observability/v1/traces
15:34:40.243 fetched POST /api/observability/v1/traces 204
15:34:40.711 chunk 6 3205 bytes
15:34:40.713 in Ack 6
15:34:41.715 fetch POST /api/observability/v1/traces
15:34:41.715 fetched POST /api/observability/v1/traces 204
15:34:43.912 chunk 18 43516 bytes
15:34:43.915 in Ack 18
15:34:44.516 in Ping 
15:34:44.541 chunk 18 44155 bytes
15:34:44.549 in Ack 18
15:34:44.854 rpc 35 orchestration.dispatchCommand runtime-request.respond
15:34:44.933 fetch POST /api/observability/v1/traces
15:34:44.933 fetched POST /api/observability/v1/traces 204
15:34:47.067 chunk 18 46159 bytes
15:34:47.068 in Ack 18
15:34:48.069 fetch POST /api/observability/v1/traces
15:34:48.070 fetched POST /api/observability/v1/traces 204
15:34:49.091 chunk 18 45148 bytes
15:34:49.092 exit 35 {"_tag":"Success","value":{"sequence":96}}
15:34:49.093 in Ack 18
15:34:49.516 in Ping 
15:34:49.523 chunk 18 47641 bytes
15:34:49.524 in Ack 18
15:34:49.718 in Interrupt 34
15:34:50.097 fetch POST /api/observability/v1/traces
15:34:50.097 fetched POST /api/observability/v1/traces 204
15:34:50.102 chunk 6 952 bytes
15:34:50.103 chunk 18 47683 bytes
15:34:50.103 in Ack 6
15:34:50.112 in Ack 18
15:34:50.536 chunk 18 47695 bytes
15:34:50.537 in Ack 18
15:34:51.049 chunk 18 47737 bytes
15:34:51.051 in Ack 18
15:34:51.106 fetch POST /api/observability/v1/traces
15:34:51.106 fetched POST /api/observability/v1/traces 204
15:34:51.566 chunk 18 47747 bytes
15:34:51.567 in Ack 18
15:34:52.088 chunk 18 47809 bytes
15:34:52.090 in Ack 18
15:34:52.569 fetch POST /api/observability/v1/traces
15:34:52.569 fetched POST /api/observability/v1/traces 204
15:34:52.946 chunk 18 47821 bytes
15:34:52.947 in Ack 18
15:34:53.379 chunk 18 47831 bytes
15:34:53.381 in Ack 18
15:34:53.815 chunk 18 47841 bytes
15:34:53.817 in Ack 18
15:34:53.951 fetch POST /api/observability/v1/traces
15:34:53.951 fetched POST /api/observability/v1/traces 204
15:34:54.224 chunk 18 47977 bytes
15:34:54.226 in Ack 18
15:34:54.520 in Ping 
15:34:54.662 chunk 18 48007 bytes
15:34:54.663 in Ack 18
15:34:55.109 chunk 18 48047 bytes
15:34:55.111 in Ack 18
15:34:55.229 fetch POST /api/observability/v1/traces
15:34:55.229 fetched POST /api/observability/v1/traces 204
15:34:55.538 chunk 18 48077 bytes
15:34:55.539 in Ack 18
15:34:56.033 chunk 18 48209 bytes
15:34:56.036 in Ack 18
15:34:56.540 fetch POST /api/observability/v1/traces
15:34:56.540 fetched POST /api/observability/v1/traces 204
15:34:56.565 chunk 18 48237 bytes
15:34:56.567 in Ack 18
15:34:57.032 chunk 18 48355 bytes
15:34:57.045 in Ack 18
15:34:57.455 chunk 18 48365 bytes
15:34:57.475 in Ack 18
15:34:57.733 fetch POST /api/observability/v1/traces
15:34:57.733 fetched POST /api/observability/v1/traces 204
15:34:57.962 chunk 18 48455 bytes
15:34:57.969 in Ack 18
15:34:58.402 chunk 18 48475 bytes
15:34:58.411 in Ack 18
15:34:58.826 chunk 18 48485 bytes
15:34:58.830 in Ack 18
15:34:59.122 fetch POST /api/observability/v1/traces
15:34:59.129 fetched POST /api/observability/v1/traces 204
15:34:59.244 chunk 18 48535 bytes
15:34:59.256 in Ack 18
15:34:59.502 chunk 18 48577 bytes
15:34:59.504 in Ack 18
15:34:59.748 in Ping 
15:34:59.769 rpc 36 server.reportClientActivity
15:34:59.777 chunk 18 48587 bytes
15:34:59.794 in Ack 18
15:35:00.193 chunk 18 48617 bytes
15:35:00.219 in Ack 18
15:35:00.551 fetch POST /api/observability/v1/traces
15:35:00.551 fetched POST /api/observability/v1/traces 204
15:35:00.553 chunk 18 48663 bytes
15:35:00.597 in Ack 18
15:35:00.905 chunk 18 48709 bytes
15:35:00.915 in Ack 18
15:35:01.381 chunk 18 48867 bytes
15:35:01.395 in Ack 18
15:35:01.799 fetch POST /api/observability/v1/traces
15:35:01.799 fetched POST /api/observability/v1/traces 204
15:35:01.842 chunk 18 48889 bytes
15:35:01.847 in Ack 18
15:35:02.141 chunk 18 48957 bytes
15:35:02.143 in Ack 18
15:35:02.511 chunk 18 48971 bytes
15:35:02.518 in Ack 18
15:35:02.852 fetch POST /api/observability/v1/traces
15:35:02.852 fetched POST /api/observability/v1/traces 204
15:35:03.148 chunk 18 49033 bytes
15:35:03.152 in Ack 18
15:35:04.106 chunk 18 49045 bytes
15:35:04.109 in Ack 18
15:35:04.723 fetch POST /api/observability/v1/traces
15:35:04.723 fetched POST /api/observability/v1/traces 204
15:35:04.784 chunk 18 49055 bytes
15:35:04.788 in Ping 
15:35:04.790 in Ack 18
15:35:05.798 fetch POST /api/observability/v1/traces
15:35:05.798 fetched POST /api/observability/v1/traces 204
15:35:07.998 chunk 18 49095 bytes
15:35:08.001 in Ack 18
15:35:09.044 fetch POST /api/observability/v1/traces
15:35:09.044 fetched POST /api/observability/v1/traces 204
