# loss70, webapp, warm start, seed 1

Profile loss70 (fault): 70% random loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 49 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 4.3s |
| Prompt reached the agent | 3.9s |
| Time to first text | 5.8s |
| Whole reply shown | 48.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 7.6s / 1.3s / 4.8s |
| Longest silence | 7.8s |
| Approval shown after it was asked | 2.5s |
| Bytes on the wire down / up | 289082 / 62825 |
| Bytes the app received / sent | 253283 / 100502 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.6 | 57 | 243 |
| 20 | 1.5 | 581 | 639 |
| 30 | 0.8 | 1013 | 1045 |
| 40 | 2.1 | 1233 | 1317 |
| 48 | 1.2 | 1558 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","messages":"456/456 546/546 901/901 546/546","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794166059297,"silences":0,"longestSilenceMs":1374,"longestRecoveredGapMs":1374},"stats":{"sendRate":6997.8454197894025,"deliveredRate":3556.948667090719,"loss":0,"srttMs":3.949333527750442,"minRttMs":1,"sentPackets":92,"sentWireBytes":27371,"receivedPackets":120,"receivedWireBytes":61561,"lastHeardMs":8566,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 205/849 575/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 255/285","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"660/660 660/660","messages":"456/456 546/546 901/901 546/546","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794166069152,"silences":0,"longestSilenceMs":1548,"longestRecoveredGapMs":1548},"stats":{"sendRate":2000,"deliveredRate":2024.7056720011049,"loss":0.30000000000000004,"srttMs":509.53832304691093,"minRttMs":1224,"sentPackets":128,"sentWireBytes":38418,"receivedPackets":134,"receivedWireBytes":67905,"lastHeardMs":18421,"unsyncedBytes":0,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 768/793","runs":"1014/1014 971/971 67/839","user":"10/10 9/9","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546","providerTurns":"774/774 774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794166079461,"silences":0,"longestSilenceMs":1548,"longestRecoveredGapMs":1548},"stats":{"sendRate":4142.857142857144,"deliveredRate":276.1904761904762,"loss":0.9333333333333333,"srttMs":564.3699984461994,"minRttMs":570,"sentPackets":190,"sentWireBytes":52132,"receivedPackets":197,"receivedWireBytes":87130,"lastHeardMs":28730,"unsyncedBytes":129,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1298/1298","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 545/545 451/451","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794166089715,"silences":0,"longestSilenceMs":1548,"longestRecoveredGapMs":1548},"stats":{"sendRate":4638.485027343467,"deliveredRate":508.2385075350697,"loss":0.7067783167709482,"srttMs":660.2227710328143,"minRttMs":570,"sentPackets":252,"sentWireBytes":63563,"receivedPackets":335,"receivedWireBytes":141271,"lastHeardMs":38984,"unsyncedBytes":0,"inflightPackets":52,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1098/1098 1058/1058 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 950/950 1570/1570 1073/1073","frame":"1030/1030","thread":"951/951","assistant":"32/32 21/21 1340/1340 378/378","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","messages":"456/456 546/546 901/901 546/546 546/546 451/451","providerTurns":"774/774 774/774 656/656","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 1395/1395 985/985 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794166100732,"silences":0,"longestSilenceMs":1548,"longestRecoveredGapMs":1548},"stats":{"sendRate":5767.3315257441445,"deliveredRate":645.7639317118287,"loss":0.8320460869565217,"srttMs":899.5519582036976,"minRttMs":570,"sentPackets":373,"sentWireBytes":99050,"receivedPackets":486,"receivedWireBytes":245779,"lastHeardMs":50001,"unsyncedBytes":65,"inflightPackets":134,"duplicateBytes":0}}
- shim trace
19:27:31.025 fetch GET /api/auth/session
19:27:31.026 fetched GET /api/auth/session 200
19:27:31.167 fetch GET /.well-known/t3/environment
19:27:31.168 fetched GET /.well-known/t3/environment 200
19:27:31.175 fetch GET /.well-known/t3/environment
19:27:31.176 fetched GET /.well-known/t3/environment 200
19:27:31.179 rpc 0 subscribeServerConfig
19:27:31.180 chunk 0 38710 bytes
19:27:31.200 in Ack 0
19:27:31.202 rpc 1 subscribeServerLifecycle
19:27:31.202 rpc 2 subscribeServerLifecycle
19:27:31.205 fetch GET /api/auth/session
19:27:31.206 fetched GET /api/auth/session 200
19:27:31.206 rpc 3 subscribeDeviceState
19:27:31.208 rpc 4 preview.reportProfiles
19:27:31.278 rpc 5 subscribeProjectClones
19:27:31.298 fetch GET /api/orchestration/threads/mcp%3Ad4b7703a-8e0b-47b9-a2bf-35ca34eb9307/bounded
19:27:31.299 fetched GET /api/orchestration/threads/mcp%3Ad4b7703a-8e0b-47b9-a2bf-35ca34eb9307/bounded 200
19:27:31.302 rpc 6 orchestration.subscribeThread
19:27:31.303 chunk 6 8227 bytes
19:27:31.304 in Ack 6
19:27:31.310 fetch GET /api/orchestration/shell
19:27:31.333 rpc 7 server.refreshProviders
19:27:31.392 rpc 8 orchestration.dispatchCommand thread.visit
19:27:31.392 exit 8 {"_tag":"Success","value":{"sequence":0}}
19:27:31.455 fetched GET /api/orchestration/shell 200
19:27:31.455 rpc 9 subscribeTerminalMetadata
19:27:31.456 rpc 10 subscribePreviewEvents
19:27:31.456 rpc 11 subscribeVcsStatus
19:27:31.458 rpc 12 orchestration.subscribeShell
19:27:31.458 chunk 12 2787 bytes
19:27:31.459 rpc 13 assets.createUrl
19:27:31.460 rpc 14 preview.list
19:27:31.460 rpc 15 projects.readFile
19:27:31.462 in Ack 12
19:27:31.549 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"3e4f3529-7147-4585-bd73-cbab86041a9e","sessions":[]}}
19:27:31.549 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:27:31.688 chunk 6 8267 bytes
19:27:31.688 chunk 1 3869 bytes
19:27:31.689 in Ack 6
19:27:31.711 in Ack 1
19:27:31.712 rpc 16 server.reportClientActivity
19:27:31.713 chunk 12 2763 bytes
19:27:31.713 in Ack 12
19:27:31.949 chunk 2 3869 bytes
19:27:31.949 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:27:19.376Z","configurableProviders":false,"con
19:27:31.949 chunk 11 1493 bytes
19:27:31.949 in Ack 2
19:27:31.952 in Ack 11
19:27:32.065 exit 13 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:27:32.166 fetch POST /api/observability/v1/traces
19:27:32.166 fetched POST /api/observability/v1/traces 204
19:27:32.316 chunk 6 8267 bytes
19:27:32.316 chunk 12 2806 bytes
19:27:32.316 in Ack 6
19:27:32.322 in Ack 12
19:27:32.557 rpc 17 attachments.createUploadUrl
19:27:32.719 chunk 6 12694 bytes
19:27:32.726 in Ack 6
19:27:32.788 exit 17 {"_tag":"Success","value":{"attachmentId":"pending-c4545c3a-0770-41c7-8e3d-e40c75301a84","expiresAt":1794166652728,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:27:32.789 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWM0NTQ1YzNhLTA3NzAtNDFjNy04ZTNkLWU0MGM3NTMwMWE4NCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NjY1MjcyOH0.iPdpPiWmKSrdm-2YjIB9MLdmag8CCD8AP8VU4SuP0Bw
19:27:32.970 chunk 6 16195 bytes
19:27:32.971 in Ack 6
19:27:33.220 chunk 6 19848 bytes
19:27:33.221 in Ack 6
19:27:33.229 in Interrupt 11
19:27:33.231 rpc 18 subscribeVcsStatus
19:27:33.310 chunk 18 1493 bytes
19:27:33.310 in Ack 18
19:27:33.317 fetch POST /api/observability/v1/traces
19:27:33.317 fetched POST /api/observability/v1/traces 204
19:27:33.418 chunk 3 876 bytes
19:27:33.418 exit 4 {"_tag":"Success","value":null}
19:27:33.418 chunk 5 4 bytes
19:27:33.418 chunk 9 36 bytes
19:27:33.418 exit 16 {"_tag":"Success","value":null}
19:27:33.418 in Ack 3
19:27:33.423 rpc 19 preview.reportProfiles
19:27:33.424 in Ack 5
19:27:33.425 in Ack 9
19:27:33.494 chunk 6 21193 bytes
19:27:33.503 in Ack 6
19:27:33.682 rpc 20 server.reportClientActivity
19:27:34.423 fetch POST /api/observability/v1/traces
19:27:34.423 fetched POST /api/observability/v1/traces 204
19:27:34.688 exit 19 {"_tag":"Success","value":null}
19:27:34.807 exit 20 {"_tag":"Success","value":null}
19:27:35.782 rpc 21 orchestration.dispatchCommand message.dispatch
19:27:36.321 fetch POST /api/observability/v1/traces
19:27:36.321 fetched POST /api/observability/v1/traces 204
19:27:36.323 in Ping 
19:27:36.359 exit 21 {"_tag":"Success","value":{"sequence":45}}
19:27:36.360 rpc 22 attachments.delete
19:27:36.537 exit 22 {"_tag":"Success","value":null}
19:27:36.609 chunk 6 23627 bytes
19:27:36.610 in Ack 6
19:27:36.645 chunk 12 2892 bytes
19:27:36.650 in Ack 12
19:27:36.860 chunk 6 27869 bytes
19:27:36.861 in Ack 6
19:27:36.863 rpc 23 assets.createUrl
19:27:37.109 chunk 6 28588 bytes
19:27:37.110 in Ack 6
19:27:37.360 chunk 6 28720 bytes
19:27:37.360 fetch POST /api/observability/v1/traces
19:27:37.360 fetched POST /api/observability/v1/traces 204
19:27:37.362 in Ack 6
19:27:37.444 chunk 12 2828 bytes
19:27:37.445 in Ack 12
19:27:37.446 rpc 24 orchestration.dispatchCommand thread.visit
19:27:37.446 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:27:37.609 chunk 6 31167 bytes
19:27:37.610 in Ack 6
19:27:37.694 chunk 12 2828 bytes
19:27:37.695 in Ack 12
19:27:37.862 chunk 6 33502 bytes
19:27:37.863 in Ack 6
19:27:37.870 in Interrupt 18
19:27:37.871 rpc 25 subscribeVcsStatus
19:27:37.918 chunk 25 1493 bytes
19:27:37.919 in Ack 25
19:27:38.111 chunk 6 34188 bytes
19:27:38.112 in Ack 6
19:27:38.123 rpc 26 server.reportClientActivity
19:27:38.210 exit 26 {"_tag":"Success","value":null}
19:27:38.361 chunk 6 35208 bytes
19:27:38.362 in Ack 6
19:27:38.370 fetch POST /api/observability/v1/traces
19:27:38.370 fetched POST /api/observability/v1/traces 204
19:27:39.265 rpc 27 orchestration.dispatchCommand message.dispatch
19:27:40.265 fetch POST /api/observability/v1/traces
19:27:40.265 fetched POST /api/observability/v1/traces 204
19:27:41.323 in Ping 
19:27:42.828 fetch POST /api/observability/v1/traces
19:27:42.828 fetched POST /api/observability/v1/traces 204
19:27:43.344 exit 27 {"_tag":"Success","value":{"sequence":75}}
19:27:44.345 fetch POST /api/observability/v1/traces
19:27:44.345 fetched POST /api/observability/v1/traces 204
19:27:44.891 chunk 6 37772 bytes
19:27:44.893 in Ack 6
19:27:45.649 chunk 6 37804 bytes
19:27:45.651 in Ack 6
19:27:45.896 fetch POST /api/observability/v1/traces
19:27:45.897 fetched POST /api/observability/v1/traces 204
19:27:46.323 in Ping 
19:27:46.390 chunk 6 37856 bytes
19:27:46.392 in Ack 6
19:27:46.641 chunk 6 37876 bytes
19:27:46.642 in Ack 6
19:27:47.393 fetch POST /api/observability/v1/traces
19:27:47.393 fetched POST /api/observability/v1/traces 204
19:27:47.830 chunk 6 37898 bytes
19:27:47.832 in Ack 6
19:27:48.464 chunk 6 37990 bytes
19:27:48.465 in Ack 6
19:27:48.833 fetch POST /api/observability/v1/traces
19:27:48.833 fetched POST /api/observability/v1/traces 204
19:27:48.977 chunk 6 38050 bytes
19:27:48.978 in Ack 6
19:27:49.227 chunk 6 38146 bytes
19:27:49.228 in Ack 6
19:27:49.780 chunk 6 38176 bytes
19:27:49.782 in Ack 6
19:27:49.979 fetch POST /api/observability/v1/traces
19:27:49.979 fetched POST /api/observability/v1/traces 204
19:27:50.523 chunk 6 38198 bytes
19:27:50.525 in Ack 6
19:27:50.774 chunk 6 38208 bytes
19:27:50.775 in Ack 6
19:27:51.324 in Ping 
19:27:51.525 fetch POST /api/observability/v1/traces
19:27:51.525 fetched POST /api/observability/v1/traces 204
19:27:51.608 chunk 6 38290 bytes
19:27:51.610 in Ack 6
19:27:51.859 chunk 6 38300 bytes
19:27:51.860 in Ack 6
19:27:52.610 fetch POST /api/observability/v1/traces
19:27:52.610 fetched POST /api/observability/v1/traces 204
19:27:52.647 chunk 6 38350 bytes
19:27:52.648 in Ack 6
19:27:52.897 chunk 6 38312 bytes
19:27:52.898 in Ack 6
19:27:53.649 fetch POST /api/observability/v1/traces
19:27:53.649 fetched POST /api/observability/v1/traces 204
19:27:53.838 chunk 6 38382 bytes
19:27:53.840 in Ack 6
19:27:54.089 chunk 6 38394 bytes
19:27:54.090 in Ack 6
19:27:54.339 chunk 6 38444 bytes
19:27:54.340 in Ack 6
19:27:54.589 chunk 6 38640 bytes
19:27:54.590 in Ack 6
19:27:54.839 chunk 6 38660 bytes
19:27:54.840 in Ack 6
19:27:54.850 fetch POST /api/observability/v1/traces
19:27:54.850 fetched POST /api/observability/v1/traces 204
19:27:55.090 chunk 6 38690 bytes
19:27:55.091 in Ack 6
19:27:55.348 chunk 6 38702 bytes
19:27:55.349 in Ack 6
19:27:55.599 chunk 6 38756 bytes
19:27:55.600 in Ack 6
19:27:55.849 chunk 6 38778 bytes
19:27:55.850 in Ack 6
19:27:56.091 fetch POST /api/observability/v1/traces
19:27:56.092 fetched POST /api/observability/v1/traces 204
19:27:56.099 chunk 6 38800 bytes
19:27:56.100 in Ack 6
19:27:56.324 in Ping 
19:27:56.348 chunk 6 38838 bytes
19:27:56.349 in Ack 6
19:27:56.418 rpc 28 server.reportClientActivity
19:27:56.599 chunk 6 38868 bytes
19:27:56.600 in Ack 6
19:27:56.849 chunk 6 38880 bytes
19:27:56.850 in Ack 6
19:27:57.100 chunk 6 38890 bytes
19:27:57.101 in Ack 6
19:27:57.110 fetch POST /api/observability/v1/traces
19:27:57.110 fetched POST /api/observability/v1/traces 204
19:27:57.350 chunk 6 38912 bytes
19:27:57.350 in Ack 6
19:27:57.600 chunk 6 38942 bytes
19:27:57.605 in Ack 6
19:27:57.850 chunk 6 38960 bytes
19:27:57.852 in Ack 6
19:27:58.101 chunk 6 38998 bytes
19:27:58.102 in Ack 6
19:27:58.351 chunk 6 39020 bytes
19:27:58.352 in Ack 6
19:27:58.361 fetch POST /api/observability/v1/traces
19:27:58.361 fetched POST /api/observability/v1/traces 204
19:27:58.600 chunk 6 39048 bytes
19:27:58.601 in Ack 6
19:27:58.851 chunk 6 39070 bytes
19:27:58.852 in Ack 6
19:27:59.101 chunk 6 39088 bytes
19:27:59.102 in Ack 6
19:27:59.351 chunk 6 39757 bytes
19:27:59.352 in Ack 6
19:27:59.602 fetch POST /api/observability/v1/traces
19:27:59.602 fetched POST /api/observability/v1/traces 204
19:27:59.603 chunk 6 39769 bytes
19:27:59.605 in Ack 6
19:27:59.852 chunk 6 39809 bytes
19:27:59.853 in Ack 6
19:28:00.102 chunk 6 39829 bytes
19:28:00.103 in Ack 6
19:28:00.353 chunk 6 39859 bytes
19:28:00.354 in Ack 6
19:28:00.603 chunk 6 41882 bytes
19:28:00.604 in Ack 6
19:28:00.616 fetch POST /api/observability/v1/traces
19:28:00.616 fetched POST /api/observability/v1/traces 204
19:28:00.640 chunk 12 2892 bytes
19:28:00.640 in Ack 12
19:28:00.642 rpc 29 orchestration.dispatchCommand thread.visit
19:28:00.642 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:28:00.853 chunk 6 41902 bytes
19:28:00.854 in Ack 6
19:28:01.103 chunk 6 41942 bytes
19:28:01.104 in Ack 6
19:28:01.325 in Ping 
19:28:01.354 chunk 6 41960 bytes
19:28:01.355 in Ack 6
19:28:01.604 chunk 6 42838 bytes
19:28:01.605 in Ack 6
19:28:01.694 fetch POST /api/observability/v1/traces
19:28:01.694 fetched POST /api/observability/v1/traces 204
19:28:01.854 chunk 6 42858 bytes
19:28:01.855 in Ack 6
19:28:01.991 exit 28 {"_tag":"Success","value":null}
19:28:02.104 chunk 6 42896 bytes
19:28:02.105 in Ack 6
19:28:02.354 chunk 6 42916 bytes
19:28:02.356 in Ack 6
19:28:02.605 chunk 6 42946 bytes
19:28:02.608 in Ack 6
19:28:02.835 fetch POST /api/observability/v1/traces
19:28:02.835 fetched POST /api/observability/v1/traces 204
19:28:02.856 chunk 6 42976 bytes
19:28:02.857 in Ack 6
19:28:03.105 chunk 6 42996 bytes
19:28:03.106 in Ack 6
19:28:03.356 chunk 6 43018 bytes
19:28:03.357 in Ack 6
19:28:03.605 chunk 6 43040 bytes
19:28:03.606 in Ack 6
19:28:03.857 chunk 6 43068 bytes
19:28:03.858 in Ack 6
19:28:03.866 fetch POST /api/observability/v1/traces
19:28:03.866 fetched POST /api/observability/v1/traces 204
19:28:04.107 chunk 6 43088 bytes
19:28:04.108 in Ack 6
19:28:04.357 chunk 6 43108 bytes
19:28:04.358 in Ack 6
19:28:04.607 chunk 6 43174 bytes
19:28:04.608 in Ack 6
19:28:04.881 chunk 6 43184 bytes
19:28:04.881 chunk 12 2892 bytes
19:28:04.887 in Ack 6
19:28:04.906 in Ack 12
19:28:05.112 fetch POST /api/observability/v1/traces
19:28:05.112 fetched POST /api/observability/v1/traces 204
19:28:05.131 chunk 6 43214 bytes
19:28:05.132 in Ack 6
19:28:05.382 chunk 6 43226 bytes
19:28:05.383 in Ack 6
19:28:05.632 chunk 6 43264 bytes
19:28:05.633 in Ack 6
19:28:05.883 chunk 6 43284 bytes
19:28:05.884 in Ack 6
19:28:06.133 fetch POST /api/observability/v1/traces
19:28:06.133 fetched POST /api/observability/v1/traces 204
19:28:06.134 chunk 6 43304 bytes
19:28:06.135 in Ack 6
19:28:06.325 in Ping 
19:28:06.384 chunk 6 45015 bytes
19:28:06.385 in Ack 6
19:28:06.634 chunk 6 45060 bytes
19:28:06.635 in Ack 6
19:28:06.884 chunk 6 45105 bytes
19:28:06.885 in Ack 6
19:28:07.136 fetch POST /api/observability/v1/traces
19:28:07.136 fetched POST /api/observability/v1/traces 204
19:28:07.183 chunk 6 45135 bytes
19:28:07.184 in Ack 6
19:28:07.434 chunk 6 45837 bytes
19:28:07.435 in Ack 6
19:28:07.684 chunk 6 45867 bytes
19:28:07.685 in Ack 6
19:28:07.934 chunk 6 45899 bytes
19:28:07.935 in Ack 6
19:28:08.184 chunk 6 45944 bytes
19:28:08.185 fetch POST /api/observability/v1/traces
19:28:08.185 fetched POST /api/observability/v1/traces 204
19:28:08.186 in Ack 6
19:28:08.435 chunk 6 45959 bytes
19:28:08.436 in Ack 6
19:28:08.686 chunk 6 47321 bytes
19:28:08.687 in Ack 6
19:28:08.937 chunk 6 47351 bytes
19:28:08.938 in Ack 6
19:28:09.186 chunk 6 47396 bytes
19:28:09.187 in Ack 6
19:28:09.214 fetch POST /api/observability/v1/traces
19:28:09.214 fetched POST /api/observability/v1/traces 204
19:28:09.436 chunk 6 47426 bytes
19:28:09.437 in Ack 6
19:28:09.686 chunk 6 47471 bytes
19:28:09.687 in Ack 6
19:28:09.937 chunk 6 47501 bytes
19:28:09.938 in Ack 6
19:28:10.191 chunk 6 47546 bytes
19:28:10.192 in Ack 6
19:28:10.438 fetch POST /api/observability/v1/traces
19:28:10.438 fetched POST /api/observability/v1/traces 204
19:28:10.440 chunk 6 47561 bytes
19:28:10.441 in Ack 6
19:28:10.691 chunk 6 47603 bytes
19:28:10.692 in Ack 6
19:28:11.326 in Ping 
19:28:11.442 fetch POST /api/observability/v1/traces
19:28:11.442 fetched POST /api/observability/v1/traces 204
19:28:12.444 chunk 6 48608 bytes
19:28:12.446 in Ack 6
19:28:12.656 rpc 30 orchestration.dispatchCommand runtime-request.respond
19:28:13.376 chunk 6 50683 bytes
19:28:13.378 in Ack 6
19:28:13.447 fetch POST /api/observability/v1/traces
19:28:13.447 fetched POST /api/observability/v1/traces 204
19:28:13.601 exit 30 {"_tag":"Success","value":{"sequence":96}}
19:28:13.627 chunk 6 53204 bytes
19:28:13.629 in Ack 6
19:28:13.877 chunk 6 53224 bytes
19:28:13.879 in Ack 6
19:28:14.128 chunk 6 53246 bytes
19:28:14.129 in Ack 6
19:28:14.390 chunk 6 53256 bytes
19:28:14.397 in Ack 6
19:28:14.431 rpc 31 orchestration.dispatchCommand runtime-request.respond
19:28:14.602 fetch POST /api/observability/v1/traces
19:28:14.602 fetched POST /api/observability/v1/traces 204
19:28:14.641 chunk 6 53323 bytes
19:28:14.643 in Ack 6
19:28:14.892 chunk 6 53345 bytes
19:28:14.893 in Ack 6
19:28:15.141 chunk 6 53339 bytes
19:28:15.143 in Ack 6
19:28:15.186 chunk 12 2894 bytes
19:28:15.186 in Ack 12
19:28:15.392 chunk 6 53351 bytes
19:28:15.393 in Ack 6
19:28:15.644 fetch POST /api/observability/v1/traces
19:28:15.644 fetched POST /api/observability/v1/traces 204
19:28:15.669 chunk 6 53389 bytes
19:28:15.669 exit 31 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:28:15.670 in Ack 6
19:28:15.680 rpc 32 orchestration.dispatchCommand thread.visit
19:28:15.680 exit 32 {"_tag":"Success","value":{"sequence":0}}
19:28:15.919 chunk 6 53429 bytes
19:28:15.920 in Ack 6
19:28:16.171 chunk 6 53459 bytes
19:28:16.172 in Ack 6
19:28:16.327 in Ping 
19:28:16.421 chunk 6 53479 bytes
19:28:16.422 in Ack 6
19:28:16.671 fetch POST /api/observability/v1/traces
19:28:16.671 fetched POST /api/observability/v1/traces 204
19:28:16.672 chunk 6 53537 bytes
19:28:16.673 in Ack 6
19:28:16.680 in Interrupt 25
19:28:16.708 rpc 33 subscribeVcsStatus
19:28:16.921 chunk 6 53567 bytes
19:28:16.922 in Ack 6
19:28:16.965 rpc 34 server.reportClientActivity
19:28:17.172 chunk 6 53577 bytes
19:28:17.173 in Ack 6
19:28:17.253 chunk 12 2894 bytes
19:28:17.253 in Ack 12
19:28:17.421 chunk 6 53587 bytes
19:28:17.423 in Ack 6
19:28:17.671 chunk 6 53607 bytes
19:28:17.672 in Ack 6
19:28:17.681 fetch POST /api/observability/v1/traces
19:28:17.681 fetched POST /api/observability/v1/traces 204
19:28:17.921 chunk 6 53647 bytes
19:28:17.922 in Ack 6
19:28:18.171 chunk 6 53657 bytes
19:28:18.172 in Ack 6
19:28:18.422 chunk 6 58262 bytes
19:28:18.423 in Ack 6
19:28:18.498 chunk 33 1493 bytes
19:28:18.498 in Ack 33
19:28:18.680 chunk 6 58282 bytes
19:28:18.683 in Ack 6
19:28:18.924 fetch POST /api/observability/v1/traces
19:28:18.924 fetched POST /api/observability/v1/traces 204
19:28:18.931 chunk 6 58292 bytes
19:28:18.932 in Ack 6
19:28:19.182 chunk 6 58312 bytes
19:28:19.183 in Ack 6
19:28:19.433 chunk 6 58352 bytes
19:28:19.434 in Ack 6
19:28:19.709 chunk 6 60784 bytes
19:28:19.711 exit 23 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/3818aa65-7c3d-4e2e-b3a7-0f6f52f460a5","expiresAt":1794169699711}}
19:28:19.712 in Ack 6
19:28:19.933 fetch POST /api/observability/v1/traces
19:28:19.933 fetched POST /api/observability/v1/traces 204
19:28:19.959 chunk 6 60804 bytes
19:28:19.960 in Ack 6
19:28:20.209 chunk 6 60834 bytes
19:28:20.210 in Ack 6
19:28:20.460 chunk 6 60865 bytes
19:28:20.461 in Ack 6
19:28:20.711 chunk 6 60905 bytes
19:28:20.712 in Ack 6
19:28:20.961 chunk 6 60935 bytes
19:28:20.962 fetch POST /api/observability/v1/traces
19:28:20.962 fetched POST /api/observability/v1/traces 204
19:28:20.964 in Ack 6
19:28:21.218 chunk 6 60955 bytes
19:28:21.219 in Ack 6
19:28:21.328 in Ping 
19:28:21.468 chunk 6 60971 bytes
19:28:21.469 in Ack 6
19:28:21.527 chunk 12 2894 bytes
19:28:21.527 in Ack 12
19:28:21.758 chunk 6 61003 bytes
19:28:21.758 exit 34 {"_tag":"Success","value":null}
19:28:21.759 in Ack 6
19:28:21.976 fetch POST /api/observability/v1/traces
19:28:21.976 fetched POST /api/observability/v1/traces 204
19:28:22.009 chunk 6 61023 bytes
19:28:22.010 in Ack 6
19:28:22.115 rpc 35 server.reportClientActivity
19:28:22.259 chunk 6 61043 bytes
19:28:22.260 in Ack 6
19:28:22.509 chunk 6 62066 bytes
19:28:22.512 in Ack 6
19:28:22.760 chunk 6 62096 bytes
19:28:22.761 in Ack 6
19:28:23.018 chunk 6 62111 bytes
19:28:23.019 fetch POST /api/observability/v1/traces
19:28:23.019 fetched POST /api/observability/v1/traces 204
19:28:23.022 in Ack 6
19:28:23.269 chunk 6 62177 bytes
19:28:23.270 in Ack 6
19:28:23.520 chunk 6 62192 bytes
19:28:23.522 in Ack 6
19:28:23.771 chunk 6 62243 bytes
19:28:23.772 in Ack 6
19:28:24.026 chunk 6 62318 bytes
19:28:24.026 fetch POST /api/observability/v1/traces
19:28:24.027 fetched POST /api/observability/v1/traces 204
19:28:24.028 in Ack 6
19:28:24.276 chunk 6 62369 bytes
19:28:24.277 in Ack 6
19:28:24.532 chunk 6 62429 bytes
19:28:24.534 in Ack 6
19:28:24.783 chunk 6 62498 bytes
19:28:24.784 in Ack 6
19:28:25.029 fetch POST /api/observability/v1/traces
19:28:25.029 fetched POST /api/observability/v1/traces 204
19:28:25.033 chunk 6 62513 bytes
19:28:25.034 in Ack 6
19:28:25.189 rpc 36 orchestration.dispatchCommand thread.visit
19:28:25.189 exit 36 {"_tag":"Success","value":{"sequence":0}}
19:28:25.283 chunk 6 62558 bytes
19:28:25.284 in Ack 6
19:28:25.388 exit 35 {"_tag":"Success","value":null}
19:28:25.533 chunk 6 62597 bytes
19:28:25.535 in Ack 6
19:28:25.784 chunk 6 62630 bytes
19:28:25.786 in Ack 6
19:28:26.035 chunk 6 62684 bytes
19:28:26.035 fetch POST /api/observability/v1/traces
19:28:26.035 fetched POST /api/observability/v1/traces 204
19:28:26.037 in Ack 6
19:28:26.292 chunk 6 62729 bytes
19:28:26.293 in Ack 6
19:28:26.365 in Ping 
19:28:26.579 chunk 6 62744 bytes
19:28:26.580 in Ack 6
19:28:26.630 chunk 12 2894 bytes
19:28:26.632 in Ack 12
19:28:26.829 chunk 6 62795 bytes
19:28:26.830 in Ack 6
19:28:27.038 fetch POST /api/observability/v1/traces
19:28:27.038 fetched POST /api/observability/v1/traces 204
19:28:27.079 chunk 6 62834 bytes
19:28:27.080 in Ack 6
19:28:27.330 chunk 6 62882 bytes
19:28:27.331 in Ack 6
19:28:27.581 chunk 6 62912 bytes
19:28:27.582 in Ack 6
19:28:27.830 chunk 6 62995 bytes
19:28:27.831 in Ack 6
19:28:28.133 fetch POST /api/observability/v1/traces
19:28:28.133 fetched POST /api/observability/v1/traces 204
