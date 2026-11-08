# A, webapp, warm start, seed 1

Profile A (graded): weak 4G, still useful. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 43 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 0.3s |
| Prompt reached the agent | 0.3s |
| Time to first text | 1.0s |
| Whole reply shown | 42.2s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.3s / 0.5s / 1.0s |
| Longest silence | 1.3s |
| Approval shown after it was asked | 0.2s |
| Bytes on the wire down / up | 2121026 / 1138125 |
| Bytes the app received / sent | 1709418 / 40071 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.5 | 365 | 385 |
| 20 | 0.2 | 779 | 787 |
| 30 | 0.3 | 1163 | 1173 |
| 40 | 0.5 | 1524 | 1542 |
| 42 | 1.2 | 1567 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":256,"lastDataAt":1794159110079,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":10125,"deliveredRate":3999.4048567403092,"loss":0,"srttMs":18.069756733338064,"minRttMs":1,"sentPackets":100,"sentWireBytes":32279,"receivedPackets":96,"receivedWireBytes":52947,"lastHeardMs":8108,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 490/490","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025 785/785","messages":"550/550 456/456 901/901 550/550 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":256,"lastDataAt":1794159120232,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":55.67301640067178,"minRttMs":1,"sentPackets":159,"sentWireBytes":41159,"receivedPackets":826,"receivedWireBytes":175861,"lastHeardMs":18261,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 988/988","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025 785/785","messages":"550/550 456/456 901/901 550/550 451/451 549/549","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":256,"lastDataAt":1794159130327,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":4500,"deliveredRate":1416.5571616294349,"loss":0,"srttMs":79.95424268038674,"minRttMs":1,"sentPackets":217,"sentWireBytes":49911,"receivedPackets":2286,"receivedWireBytes":405923,"lastHeardMs":28356,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1108/1108 1058/1058 860/860 1299/1299 1108/1108 1058/1058 849/849 1108/1108 956/956 1578/1578 1083/1083","frame":"1032/1032","thread":"951/951","assistant":"32/32 21/21 1340/1340 208/208","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"662/662 662/662 638/638","providerTurns":"778/778 778/778 660/660","providerThreads":"812/812","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025 785/785 989/989 1401/1401 1025/1025","messages":"550/550 456/456 901/901 550/550 451/451 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1054/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":256,"lastDataAt":1794159141278,"silences":0,"longestSilenceMs":1041,"longestRecoveredGapMs":1041},"stats":{"sendRate":6750,"deliveredRate":1796.4426877470355,"loss":0,"srttMs":87.27119821535766,"minRttMs":1,"sentPackets":278,"sentWireBytes":60372,"receivedPackets":5116,"receivedWireBytes":882881,"lastHeardMs":39307,"unsyncedBytes":0,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
17:31:42.256 fetch GET /api/auth/session
17:31:42.258 fetched GET /api/auth/session 200
17:31:42.404 fetch GET /.well-known/t3/environment
17:31:42.404 fetched GET /.well-known/t3/environment 200
17:31:42.420 fetch GET /.well-known/t3/environment
17:31:42.420 fetched GET /.well-known/t3/environment 200
17:31:42.423 rpc 0 subscribeServerConfig
17:31:42.424 chunk 0 38690 bytes
17:31:42.440 in Ack 0
17:31:42.441 rpc 1 subscribeServerLifecycle
17:31:42.442 rpc 2 subscribeServerLifecycle
17:31:42.444 fetch GET /api/auth/session
17:31:42.445 fetched GET /api/auth/session 200
17:31:42.445 rpc 3 subscribeDeviceState
17:31:42.447 rpc 4 preview.reportProfiles
17:31:42.484 rpc 5 subscribeProjectClones
17:31:42.502 fetch GET /api/orchestration/threads/mcp%3Abdbbbb1c-5518-4614-bd16-9b02051eb4df/bounded
17:31:42.505 fetch GET /api/orchestration/shell
17:31:42.506 fetched GET /api/orchestration/shell 200
17:31:42.508 rpc 6 orchestration.subscribeShell
17:31:42.508 chunk 6 2782 bytes
17:31:42.527 rpc 7 server.refreshProviders
17:31:42.564 rpc 8 orchestration.dispatchCommand thread.visit
17:31:42.564 exit 8 {"_tag":"Success","value":{"sequence":0}}
17:31:42.578 rpc 9 subscribeTerminalMetadata
17:31:42.579 rpc 10 subscribeVcsStatus
17:31:42.579 rpc 11 subscribePreviewEvents
17:31:42.580 rpc 12 assets.createUrl
17:31:42.581 rpc 13 preview.list
17:31:42.581 rpc 14 projects.readFile
17:31:42.585 fetched GET /api/orchestration/threads/mcp%3Abdbbbb1c-5518-4614-bd16-9b02051eb4df/bounded 200
17:31:42.588 rpc 15 orchestration.subscribeThread
17:31:42.589 chunk 15 8249 bytes
17:31:42.616 in Interrupt 10
17:31:42.655 rpc 16 subscribeVcsStatus
17:31:42.655 in Ack 6
17:31:42.663 in Ack 15
17:31:42.758 chunk 6 2758 bytes
17:31:42.759 in Ack 6
17:31:42.821 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3939c129-3942-4125-a773-10070167a28c","sessions":[]}}
17:31:42.821 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:31:42.919 rpc 17 server.reportClientActivity
17:31:43.049 chunk 15 8289 bytes
17:31:43.049 chunk 1 3864 bytes
17:31:43.050 in Ack 15
17:31:43.071 in Ack 1
17:31:43.403 fetch POST /api/observability/v1/traces
17:31:43.403 fetched POST /api/observability/v1/traces 204
17:31:43.474 chunk 2 3864 bytes
17:31:43.474 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:31:29.684Z","configurableProviders":false,"con
17:31:43.474 chunk 10 472 bytes
17:31:43.476 in Ack 2
17:31:43.686 rpc 18 attachments.createUploadUrl
17:31:43.707 exit 12 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:31:43.707 chunk 16 472 bytes
17:31:43.861 in Ack 16
17:31:43.908 exit 18 {"_tag":"Success","value":{"attachmentId":"pending-0b3cd2b8-cbae-4281-907b-c74a07acc772","expiresAt":1794159703898,"relativeUrl":"/api/attachments/upload/eyJ2ZX
17:31:43.909 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTBiM2NkMmI4LWNiYWUtNDI4MS05MDdiLWM3NGEwN2FjYzc3MiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1OTcwMzg5OH0.f7strLuAzbHUdsa7NbG7B5NEqF8LaCYukc2bDHLeCqI
17:31:43.956 chunk 15 8289 bytes
17:31:43.957 in Ack 15
17:31:44.239 chunk 6 2801 bytes
17:31:44.239 chunk 15 9359 bytes
17:31:44.247 in Ack 6
17:31:44.252 in Ack 15
17:31:44.477 fetch POST /api/observability/v1/traces
17:31:44.477 fetched POST /api/observability/v1/traces 204
17:31:44.489 chunk 15 12722 bytes
17:31:44.490 in Ack 15
17:31:44.739 chunk 15 13532 bytes
17:31:44.740 in Ack 15
17:31:44.989 chunk 15 18055 bytes
17:31:44.990 in Ack 15
17:31:45.240 chunk 15 19883 bytes
17:31:45.241 in Ack 15
17:31:45.247 in Interrupt 16
17:31:45.248 rpc 19 subscribeVcsStatus
17:31:45.312 chunk 19 472 bytes
17:31:45.313 in Ack 19
17:31:45.490 chunk 15 21223 bytes
17:31:45.490 in Ack 15
17:31:45.498 fetch POST /api/observability/v1/traces
17:31:45.498 fetched POST /api/observability/v1/traces 204
17:31:45.547 chunk 3 876 bytes
17:31:45.548 exit 4 {"_tag":"Success","value":null}
17:31:45.548 chunk 5 4 bytes
17:31:45.548 chunk 9 36 bytes
17:31:45.548 exit 17 {"_tag":"Success","value":null}
17:31:45.564 in Ack 3
17:31:45.568 rpc 20 preview.reportProfiles
17:31:45.568 in Ack 5
17:31:45.568 in Ack 9
17:31:45.637 exit 20 {"_tag":"Success","value":null}
17:31:45.822 rpc 21 server.reportClientActivity
17:31:45.857 exit 21 {"_tag":"Success","value":null}
17:31:46.541 rpc 22 orchestration.dispatchCommand message.dispatch
17:31:47.079 fetch POST /api/observability/v1/traces
17:31:47.079 fetched POST /api/observability/v1/traces 204
17:31:47.149 exit 22 {"_tag":"Success","value":{"sequence":45}}
17:31:47.151 rpc 23 attachments.delete
17:31:47.269 exit 23 {"_tag":"Success","value":null}
17:31:47.399 chunk 6 2887 bytes
17:31:47.399 chunk 15 24655 bytes
17:31:47.400 in Ack 6
17:31:47.427 in Ack 15
17:31:47.430 rpc 24 assets.createUrl
17:31:47.450 in Ping 
17:31:47.650 chunk 15 27913 bytes
17:31:47.651 in Ack 15
17:31:47.900 chunk 15 29432 bytes
17:31:47.901 in Ack 15
17:31:48.150 chunk 6 2823 bytes
17:31:48.150 in Ack 6
17:31:48.151 rpc 25 orchestration.dispatchCommand thread.visit
17:31:48.151 exit 25 {"_tag":"Success","value":{"sequence":0}}
17:31:48.156 fetch POST /api/observability/v1/traces
17:31:48.156 fetched POST /api/observability/v1/traces 204
17:31:48.400 chunk 6 2823 bytes
17:31:48.401 in Ack 6
17:31:48.409 chunk 15 29564 bytes
17:31:48.410 in Ack 15
17:31:48.660 chunk 15 32035 bytes
17:31:48.660 in Ack 15
17:31:48.910 chunk 15 32649 bytes
17:31:48.911 in Ack 15
17:31:48.917 in Interrupt 19
17:31:48.919 rpc 26 subscribeVcsStatus
17:31:48.958 chunk 26 472 bytes
17:31:48.958 in Ack 26
17:31:49.362 fetch POST /api/observability/v1/traces
17:31:49.362 fetched POST /api/observability/v1/traces 204
17:31:49.363 chunk 15 32767 bytes
17:31:49.363 rpc 27 server.reportClientActivity
17:31:49.365 in Ack 15
17:31:49.616 chunk 15 35276 bytes
17:31:49.618 in Ack 15
17:31:49.661 exit 27 {"_tag":"Success","value":null}
17:31:49.869 chunk 15 35276 bytes
17:31:49.871 in Ack 15
17:31:50.047 rpc 28 orchestration.dispatchCommand message.dispatch
17:31:50.221 exit 28 {"_tag":"Success","value":{"sequence":75}}
17:31:50.365 fetch POST /api/observability/v1/traces
17:31:50.365 fetched POST /api/observability/v1/traces 204
17:31:50.622 chunk 6 2887 bytes
17:31:50.622 chunk 15 37684 bytes
17:31:50.622 in Ack 6
17:31:50.662 in Ack 15
17:31:50.873 chunk 15 41196 bytes
17:31:50.874 in Ack 15
17:31:51.123 chunk 15 42675 bytes
17:31:51.125 in Ack 15
17:31:51.373 chunk 15 43201 bytes
17:31:51.374 in Ack 15
17:31:51.623 fetch POST /api/observability/v1/traces
17:31:51.623 fetched POST /api/observability/v1/traces 204
17:31:51.624 chunk 15 43241 bytes
17:31:51.625 in Ack 15
17:31:51.710 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/c29420e1-10b9-4ec0-85a4-6bc742132ae7","expiresAt":1794162711710}}
17:31:51.874 chunk 15 43261 bytes
17:31:51.875 in Ack 15
17:31:52.124 chunk 15 43291 bytes
17:31:52.125 in Ack 15
17:31:52.375 chunk 15 43311 bytes
17:31:52.376 in Ack 15
17:31:52.450 in Ping 
17:31:52.624 chunk 15 43341 bytes
17:31:52.626 in Ack 15
17:31:52.636 fetch POST /api/observability/v1/traces
17:31:52.636 fetched POST /api/observability/v1/traces 204
17:31:52.874 chunk 15 43361 bytes
17:31:52.875 in Ack 15
17:31:53.125 chunk 15 43391 bytes
17:31:53.126 in Ack 15
17:31:53.375 chunk 15 43411 bytes
17:31:53.376 in Ack 15
17:31:53.625 chunk 15 43441 bytes
17:31:53.626 in Ack 15
17:31:53.874 chunk 15 43461 bytes
17:31:53.876 in Ack 15
17:31:53.893 fetch POST /api/observability/v1/traces
17:31:53.893 fetched POST /api/observability/v1/traces 204
17:31:54.125 chunk 15 43481 bytes
17:31:54.127 in Ack 15
17:31:54.375 chunk 15 43511 bytes
17:31:54.377 in Ack 15
17:31:54.626 chunk 15 43541 bytes
17:31:54.627 in Ack 15
17:31:54.875 chunk 15 43561 bytes
17:31:54.876 in Ack 15
17:31:55.125 chunk 15 43581 bytes
17:31:55.126 in Ack 15
17:31:55.135 fetch POST /api/observability/v1/traces
17:31:55.135 fetched POST /api/observability/v1/traces 204
17:31:55.376 chunk 15 43611 bytes
17:31:55.377 in Ack 15
17:31:55.626 chunk 15 43641 bytes
17:31:55.627 in Ack 15
17:31:55.876 chunk 15 43661 bytes
17:31:55.877 in Ack 15
17:31:56.126 chunk 15 43691 bytes
17:31:56.127 in Ack 15
17:31:56.376 chunk 15 43711 bytes
17:31:56.377 in Ack 15
17:31:56.385 fetch POST /api/observability/v1/traces
17:31:56.385 fetched POST /api/observability/v1/traces 204
17:31:56.627 chunk 15 43731 bytes
17:31:56.628 in Ack 15
17:31:56.877 chunk 15 43761 bytes
17:31:56.878 in Ack 15
17:31:57.127 chunk 15 43791 bytes
17:31:57.129 in Ack 15
17:31:57.378 chunk 15 43811 bytes
17:31:57.378 in Ack 15
17:31:57.450 in Ping 
17:31:57.628 chunk 15 43841 bytes
17:31:57.629 in Ack 15
17:31:57.639 fetch POST /api/observability/v1/traces
17:31:57.639 fetched POST /api/observability/v1/traces 204
17:31:57.879 chunk 15 43861 bytes
17:31:57.879 in Ack 15
17:31:58.129 chunk 15 43881 bytes
17:31:58.130 in Ack 15
17:31:58.157 rpc 29 orchestration.dispatchCommand thread.visit
17:31:58.157 exit 29 {"_tag":"Success","value":{"sequence":0}}
17:31:58.379 chunk 6 2887 bytes
17:31:58.379 chunk 15 43911 bytes
17:31:58.380 in Ack 6
17:31:58.387 in Ack 15
17:31:58.630 chunk 15 43931 bytes
17:31:58.631 in Ack 15
17:31:58.880 chunk 15 43961 bytes
17:31:58.880 fetch POST /api/observability/v1/traces
17:31:58.880 fetched POST /api/observability/v1/traces 204
17:31:58.881 in Ack 15
17:31:59.130 chunk 15 43981 bytes
17:31:59.131 in Ack 15
17:31:59.379 chunk 15 44011 bytes
17:31:59.381 in Ack 15
17:31:59.630 chunk 15 44031 bytes
17:31:59.631 in Ack 15
17:31:59.880 chunk 15 44061 bytes
17:31:59.881 in Ack 15
17:31:59.891 fetch POST /api/observability/v1/traces
17:31:59.891 fetched POST /api/observability/v1/traces 204
17:32:00.131 chunk 15 44091 bytes
17:32:00.132 in Ack 15
17:32:00.381 chunk 15 44111 bytes
17:32:00.382 in Ack 15
17:32:00.630 chunk 6 2887 bytes
17:32:00.631 chunk 15 45186 bytes
17:32:00.631 in Ack 6
17:32:00.633 in Ack 15
17:32:00.881 chunk 15 45231 bytes
17:32:00.883 in Ack 15
17:32:01.131 chunk 15 45261 bytes
17:32:01.132 in Ack 15
17:32:01.151 fetch POST /api/observability/v1/traces
17:32:01.151 fetched POST /api/observability/v1/traces 204
17:32:01.381 chunk 15 45306 bytes
17:32:01.382 in Ack 15
17:32:01.632 chunk 15 45336 bytes
17:32:01.633 in Ack 15
17:32:01.881 chunk 15 45381 bytes
17:32:01.882 in Ack 15
17:32:02.131 chunk 15 45411 bytes
17:32:02.132 in Ack 15
17:32:02.382 chunk 15 45456 bytes
17:32:02.383 in Ack 15
17:32:02.404 fetch POST /api/observability/v1/traces
17:32:02.404 fetched POST /api/observability/v1/traces 204
17:32:02.451 in Ping 
17:32:02.645 chunk 15 45486 bytes
17:32:02.647 in Ack 15
17:32:02.882 chunk 15 45531 bytes
17:32:02.883 in Ack 15
17:32:03.132 chunk 15 45561 bytes
17:32:03.133 in Ack 15
17:32:03.383 chunk 15 45606 bytes
17:32:03.384 in Ack 15
17:32:03.632 chunk 15 45636 bytes
17:32:03.634 in Ack 15
17:32:03.652 fetch POST /api/observability/v1/traces
17:32:03.652 fetched POST /api/observability/v1/traces 204
17:32:03.883 chunk 15 45681 bytes
17:32:03.884 in Ack 15
17:32:04.132 chunk 15 45711 bytes
17:32:04.133 in Ack 15
17:32:04.384 chunk 15 45756 bytes
17:32:04.385 in Ack 15
17:32:04.634 chunk 15 45786 bytes
17:32:04.636 in Ack 15
17:32:04.884 fetch POST /api/observability/v1/traces
17:32:04.885 fetched POST /api/observability/v1/traces 204
17:32:04.885 chunk 15 45831 bytes
17:32:04.887 in Ack 15
17:32:05.135 chunk 15 45861 bytes
17:32:05.136 in Ack 15
17:32:05.385 chunk 15 45906 bytes
17:32:05.386 in Ack 15
17:32:05.636 chunk 15 45936 bytes
17:32:05.637 in Ack 15
17:32:05.886 chunk 15 45981 bytes
17:32:05.887 in Ack 15
17:32:05.911 fetch POST /api/observability/v1/traces
17:32:05.911 fetched POST /api/observability/v1/traces 204
17:32:06.137 chunk 15 46011 bytes
17:32:06.138 in Ack 15
17:32:06.387 chunk 15 46056 bytes
17:32:06.388 in Ack 15
17:32:06.637 chunk 15 46086 bytes
17:32:06.638 in Ack 15
17:32:06.888 chunk 15 46131 bytes
17:32:06.889 in Ack 15
17:32:07.138 chunk 15 46161 bytes
17:32:07.139 fetch POST /api/observability/v1/traces
17:32:07.139 fetched POST /api/observability/v1/traces 204
17:32:07.140 in Ack 15
17:32:07.389 chunk 15 46206 bytes
17:32:07.390 in Ack 15
17:32:07.451 in Ping 
17:32:07.639 chunk 15 46236 bytes
17:32:07.641 in Ack 15
17:32:07.687 rpc 30 server.reportClientActivity
17:32:07.819 exit 30 {"_tag":"Success","value":null}
17:32:07.890 chunk 15 46281 bytes
17:32:07.891 in Ack 15
17:32:08.140 chunk 15 46311 bytes
17:32:08.141 in Ack 15
17:32:08.150 fetch POST /api/observability/v1/traces
17:32:08.150 fetched POST /api/observability/v1/traces 204
17:32:08.171 rpc 31 orchestration.dispatchCommand thread.visit
17:32:08.171 exit 31 {"_tag":"Success","value":{"sequence":0}}
17:32:08.390 chunk 6 2887 bytes
17:32:08.390 chunk 15 46356 bytes
17:32:08.391 in Ack 6
17:32:08.396 in Ack 15
17:32:08.640 chunk 15 46386 bytes
17:32:08.641 in Ack 15
17:32:08.890 chunk 15 46431 bytes
17:32:08.892 in Ack 15
17:32:09.140 chunk 15 46461 bytes
17:32:09.141 in Ack 15
17:32:09.178 fetch POST /api/observability/v1/traces
17:32:09.178 fetched POST /api/observability/v1/traces 204
17:32:09.391 chunk 15 46506 bytes
17:32:09.391 in Ack 15
17:32:09.640 chunk 15 46536 bytes
17:32:09.642 in Ack 15
17:32:09.891 chunk 15 46581 bytes
17:32:09.892 in Ack 15
17:32:10.140 chunk 15 46611 bytes
17:32:10.141 in Ack 15
17:32:10.391 chunk 15 46656 bytes
17:32:10.392 in Ack 15
17:32:10.401 fetch POST /api/observability/v1/traces
17:32:10.401 fetched POST /api/observability/v1/traces 204
17:32:10.764 chunk 15 46701 bytes
17:32:10.765 in Ack 15
17:32:11.015 chunk 15 46746 bytes
17:32:11.016 in Ack 15
17:32:11.265 chunk 15 46791 bytes
17:32:11.266 in Ack 15
17:32:11.516 chunk 15 46821 bytes
17:32:11.517 in Ack 15
17:32:11.766 chunk 15 46866 bytes
17:32:11.767 fetch POST /api/observability/v1/traces
17:32:11.767 fetched POST /api/observability/v1/traces 204
17:32:11.768 in Ack 15
17:32:12.017 chunk 15 46896 bytes
17:32:12.018 in Ack 15
17:32:12.269 chunk 15 46941 bytes
17:32:12.271 in Ack 15
17:32:12.451 in Ping 
17:32:12.519 chunk 15 46971 bytes
17:32:12.520 in Ack 15
17:32:12.769 chunk 15 47016 bytes
17:32:12.770 fetch POST /api/observability/v1/traces
17:32:12.770 fetched POST /api/observability/v1/traces 204
17:32:12.771 in Ack 15
17:32:13.021 chunk 15 47046 bytes
17:32:13.023 in Ack 15
17:32:13.272 chunk 15 47091 bytes
17:32:13.274 in Ack 15
17:32:13.522 chunk 15 47121 bytes
17:32:13.523 in Ack 15
17:32:13.772 chunk 15 47151 bytes
17:32:13.772 fetch POST /api/observability/v1/traces
17:32:13.772 fetched POST /api/observability/v1/traces 204
17:32:13.773 in Ack 15
17:32:14.024 chunk 15 47196 bytes
17:32:14.025 in Ack 15
17:32:14.275 chunk 15 47226 bytes
17:32:14.276 in Ack 15
17:32:14.525 chunk 15 47271 bytes
17:32:14.526 in Ack 15
17:32:14.776 fetch POST /api/observability/v1/traces
17:32:14.777 fetched POST /api/observability/v1/traces 204
17:32:14.781 chunk 15 47301 bytes
17:32:14.785 in Ack 15
17:32:15.032 chunk 15 47346 bytes
17:32:15.033 in Ack 15
17:32:15.281 chunk 15 47376 bytes
17:32:15.282 in Ack 15
17:32:15.532 chunk 15 47421 bytes
17:32:15.533 in Ack 15
17:32:15.782 chunk 15 47466 bytes
17:32:15.783 in Ack 15
17:32:15.793 fetch POST /api/observability/v1/traces
17:32:15.793 fetched POST /api/observability/v1/traces 204
17:32:16.032 chunk 15 47496 bytes
17:32:16.033 in Ack 15
17:32:16.283 chunk 15 47526 bytes
17:32:16.284 in Ack 15
17:32:16.533 chunk 15 47571 bytes
17:32:16.533 in Ack 15
17:32:16.783 chunk 15 47601 bytes
17:32:16.784 in Ack 15
17:32:17.034 chunk 15 47646 bytes
17:32:17.034 fetch POST /api/observability/v1/traces
17:32:17.034 fetched POST /api/observability/v1/traces 204
17:32:17.035 in Ack 15
17:32:17.284 chunk 15 52228 bytes
17:32:17.285 in Ack 15
17:32:17.442 rpc 32 orchestration.dispatchCommand runtime-request.respond
17:32:17.491 in Ping 
17:32:17.534 chunk 6 3197 bytes
17:32:17.534 in Ack 6
17:32:17.541 chunk 15 57739 bytes
17:32:17.542 in Ack 15
17:32:17.602 exit 32 {"_tag":"Success","value":{"sequence":97}}
17:32:17.784 chunk 6 2889 bytes
17:32:17.784 in Ack 6
17:32:17.791 chunk 15 60391 bytes
17:32:17.792 in Ack 15
17:32:17.798 in Interrupt 26
17:32:17.821 rpc 33 subscribeVcsStatus
17:32:17.916 chunk 33 472 bytes
17:32:17.916 in Ack 33
17:32:18.037 fetch POST /api/observability/v1/traces
17:32:18.037 fetched POST /api/observability/v1/traces 204
17:32:18.041 chunk 15 60411 bytes
17:32:18.041 in Ack 15
17:32:18.073 rpc 34 server.reportClientActivity
17:32:18.171 rpc 35 orchestration.dispatchCommand thread.visit
17:32:18.171 exit 35 {"_tag":"Success","value":{"sequence":0}}
17:32:18.253 exit 34 {"_tag":"Success","value":null}
17:32:18.291 chunk 15 60441 bytes
17:32:18.292 in Ack 15
17:32:18.535 chunk 6 2889 bytes
17:32:18.535 in Ack 6
17:32:18.541 chunk 15 60471 bytes
17:32:18.542 in Ack 15
17:32:18.791 chunk 15 60491 bytes
17:32:18.792 in Ack 15
17:32:19.040 chunk 15 60521 bytes
17:32:19.041 in Ack 15
17:32:19.049 fetch POST /api/observability/v1/traces
17:32:19.049 fetched POST /api/observability/v1/traces 204
17:32:19.291 chunk 15 60541 bytes
17:32:19.292 in Ack 15
17:32:19.541 chunk 15 60571 bytes
17:32:19.542 in Ack 15
17:32:19.792 chunk 15 60591 bytes
17:32:19.793 in Ack 15
17:32:20.042 chunk 15 60621 bytes
17:32:20.043 in Ack 15
17:32:20.291 chunk 15 60641 bytes
17:32:20.292 in Ack 15
17:32:20.300 fetch POST /api/observability/v1/traces
17:32:20.300 fetched POST /api/observability/v1/traces 204
17:32:20.543 chunk 15 60671 bytes
17:32:20.544 in Ack 15
17:32:20.796 chunk 15 60691 bytes
17:32:20.797 in Ack 15
17:32:21.046 chunk 15 60721 bytes
17:32:21.048 in Ack 15
17:32:21.297 chunk 15 60741 bytes
17:32:21.298 in Ack 15
17:32:21.545 fetch POST /api/observability/v1/traces
17:32:21.545 fetched POST /api/observability/v1/traces 204
17:32:21.547 chunk 15 60771 bytes
17:32:21.548 in Ack 15
17:32:21.796 chunk 15 60791 bytes
17:32:21.797 in Ack 15
17:32:22.047 chunk 15 60821 bytes
17:32:22.048 in Ack 15
17:32:22.297 chunk 15 60841 bytes
17:32:22.298 in Ack 15
17:32:22.492 in Ping 
17:32:22.546 chunk 15 60871 bytes
17:32:22.547 in Ack 15
17:32:22.556 fetch POST /api/observability/v1/traces
17:32:22.556 fetched POST /api/observability/v1/traces 204
17:32:22.796 chunk 15 60891 bytes
17:32:22.797 in Ack 15
17:32:23.047 chunk 15 60921 bytes
17:32:23.048 in Ack 15
17:32:23.297 chunk 15 60941 bytes
17:32:23.298 in Ack 15
17:32:23.546 chunk 15 60971 bytes
17:32:23.548 in Ack 15
17:32:23.797 chunk 15 60991 bytes
17:32:23.798 in Ack 15
17:32:23.806 fetch POST /api/observability/v1/traces
17:32:23.806 fetched POST /api/observability/v1/traces 204
17:32:24.047 chunk 15 61021 bytes
17:32:24.048 in Ack 15
17:32:24.297 chunk 15 61041 bytes
17:32:24.298 in Ack 15
17:32:24.547 chunk 15 61071 bytes
17:32:24.548 in Ack 15
17:32:24.798 chunk 15 61091 bytes
17:32:24.799 in Ack 15
17:32:25.048 chunk 15 61121 bytes
17:32:25.049 fetch POST /api/observability/v1/traces
17:32:25.049 fetched POST /api/observability/v1/traces 204
17:32:25.050 in Ack 15
17:32:25.299 chunk 15 61143 bytes
17:32:25.300 in Ack 15
17:32:25.545 chunk 6 2889 bytes
17:32:25.546 in Ack 6
17:32:25.549 chunk 15 62126 bytes
17:32:25.550 in Ack 15
17:32:25.799 chunk 15 62171 bytes
17:32:25.800 in Ack 15
17:32:26.049 chunk 15 62216 bytes
17:32:26.050 in Ack 15
17:32:26.091 fetch POST /api/observability/v1/traces
17:32:26.091 fetched POST /api/observability/v1/traces 204
17:32:26.299 chunk 15 62246 bytes
17:32:26.300 in Ack 15
17:32:26.549 chunk 15 62291 bytes
17:32:26.550 in Ack 15
17:32:26.799 chunk 15 62327 bytes
17:32:26.800 in Ack 15
17:32:27.049 chunk 15 62372 bytes
17:32:27.050 in Ack 15
17:32:27.299 chunk 15 62408 bytes
17:32:27.300 in Ack 15
17:32:27.369 fetch POST /api/observability/v1/traces
17:32:27.369 fetched POST /api/observability/v1/traces 204
17:32:27.492 in Ping 
17:32:27.549 chunk 15 62438 bytes
17:32:27.550 in Ack 15
17:32:27.798 chunk 15 62483 bytes
17:32:27.799 in Ack 15
17:32:28.048 chunk 15 62534 bytes
17:32:28.050 in Ack 15
17:32:28.171 rpc 36 orchestration.dispatchCommand thread.visit
17:32:28.171 exit 36 {"_tag":"Success","value":{"sequence":0}}
17:32:28.299 chunk 15 62564 bytes
17:32:28.300 in Ack 15
17:32:28.548 chunk 6 2889 bytes
17:32:28.549 chunk 15 62594 bytes
17:32:28.549 in Ack 6
17:32:28.555 in Ack 15
17:32:28.571 fetch POST /api/observability/v1/traces
17:32:28.571 fetched POST /api/observability/v1/traces 204
17:32:28.799 chunk 15 62639 bytes
17:32:28.800 in Ack 15
17:32:29.050 chunk 15 62693 bytes
17:32:29.051 in Ack 15
17:32:29.300 chunk 15 62723 bytes
17:32:29.301 in Ack 15
17:32:29.550 chunk 15 62753 bytes
17:32:29.551 in Ack 15
17:32:29.800 chunk 15 62810 bytes
17:32:29.801 in Ack 15
17:32:29.887 fetch POST /api/observability/v1/traces
17:32:29.887 fetched POST /api/observability/v1/traces 204
17:32:30.059 chunk 15 62864 bytes
17:32:30.061 in Ack 15
17:32:30.309 chunk 15 62894 bytes
17:32:30.310 in Ack 15
17:32:30.559 chunk 15 62945 bytes
17:32:30.562 in Ack 15
17:32:30.810 chunk 15 62975 bytes
17:32:30.811 in Ack 15
17:32:31.061 chunk 15 63029 bytes
17:32:31.062 in Ack 15
17:32:31.137 fetch POST /api/observability/v1/traces
17:32:31.137 fetched POST /api/observability/v1/traces 204
17:32:31.312 chunk 15 63062 bytes
17:32:31.313 in Ack 15
17:32:31.562 chunk 15 63175 bytes
17:32:31.563 in Ack 15
17:32:31.812 chunk 6 2825 bytes
17:32:31.812 chunk 15 67536 bytes
17:32:31.812 in Ack 6
17:32:31.814 rpc 37 orchestration.dispatchCommand thread.visit
17:32:31.814 exit 37 {"_tag":"Success","value":{"sequence":0}}
17:32:31.819 in Ack 15
17:32:31.831 in Interrupt 33
17:32:31.852 rpc 38 subscribeVcsStatus
17:32:32.046 chunk 38 472 bytes
17:32:32.046 in Ack 38
17:32:32.062 chunk 15 67536 bytes
17:32:32.063 in Ack 15
17:32:32.103 rpc 39 server.reportClientActivity
17:32:32.312 chunk 6 2825 bytes
17:32:32.313 in Ack 6
17:32:32.319 fetch POST /api/observability/v1/traces
17:32:32.319 fetched POST /api/observability/v1/traces 204
