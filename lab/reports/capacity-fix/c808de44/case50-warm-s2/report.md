# case50, webapp, warm start, seed 2

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 45 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.4s |
| Prompt reached the agent | 0.7s |
| Time to first text | 1.6s |
| Whole reply shown | 44.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 3.9s / 1.0s / 2.0s |
| Longest silence | 4.7s |
| Approval shown after it was asked | 1.0s |
| Bytes on the wire down / up | 356915 / 128301 |
| Bytes the app received / sent | 296433 / 69356 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 1.3 | 318 | 369 |
| 20 | 0.9 | 734 | 770 |
| 30 | 0.9 | 1077 | 1113 |
| 40 | 1.3 | 1443 | 1487 |
| 44 | 2.0 | 1558 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","runs":"1014/1014 971/971","providerTurns":"776/776 776/776","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 549/1022","messages":"548/548 456/456 548/548 901/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":297,"lastDataAt":1794173271850,"silences":0,"longestSilenceMs":1013,"longestRecoveredGapMs":1013},"stats":{"sendRate":6750,"deliveredRate":2865.777481594313,"loss":0,"srttMs":127.56802543363597,"minRttMs":1,"sentPackets":119,"sentWireBytes":35837,"receivedPackets":114,"receivedWireBytes":56350,"lastHeardMs":9788,"unsyncedBytes":509,"inflightPackets":11,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","runs":"1014/1014 971/971 746/839","providerTurns":"776/776 776/776","assistant":"32/32 21/21 430/445","user":"10/10 9/9 48/64","attempts":"661/661 661/661 362/637","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 860/1022","messages":"548/548 456/456 548/548 901/901","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":297,"lastDataAt":1794173281867,"silences":0,"longestSilenceMs":1013,"longestRecoveredGapMs":1013},"stats":{"sendRate":1500,"deliveredRate":0,"loss":0.3751444976076555,"srttMs":521.0373940029365,"minRttMs":1,"sentPackets":179,"sentWireBytes":48745,"receivedPackets":187,"receivedWireBytes":82050,"lastHeardMs":19805,"unsyncedBytes":0,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"779/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 943/943","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785","messages":"548/548 456/456 548/548 901/901 547/547 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":297,"lastDataAt":1794173291925,"silences":0,"longestSilenceMs":1013,"longestRecoveredGapMs":1013},"stats":{"sendRate":1500,"deliveredRate":1217.5771971496438,"loss":0.2142857142857143,"srttMs":661.0275819632341,"minRttMs":635,"sentPackets":238,"sentWireBytes":59640,"receivedPackets":322,"receivedWireBytes":138921,"lastHeardMs":29863,"unsyncedBytes":0,"inflightPackets":21,"duplicateBytes":0}}
- sample failed: locator.click: Timeout 2000ms exceeded.
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 867/1574 80/953 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 148/148","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 673/1398 963/963 1022/1022","messages":"548/548 456/456 548/548 901/901 482/548 451/451","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":297,"lastDataAt":1794173303822,"silences":0,"longestSilenceMs":1013,"longestRecoveredGapMs":1013},"stats":{"sendRate":3737.3988799004355,"deliveredRate":1585.5631611698818,"loss":0.36363636363636365,"srttMs":697.339752160616,"minRttMs":635,"sentPackets":303,"sentWireBytes":71942,"receivedPackets":651,"receivedWireBytes":240220,"lastHeardMs":41760,"unsyncedBytes":0,"inflightPackets":28,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"870/870 1103/1103 1058/1058 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","thread":"951/951","runs":"1014/1014 971/971 839/839","providerTurns":"776/776 776/776 658/658","assistant":"32/32 21/21 1340/1340 679/679","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerThreads":"811/811","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022 785/785 1398/1398 987/987 1022/1022","messages":"548/548 456/456 548/548 901/901 548/548 451/451 547/547","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":297,"lastDataAt":1794173314874,"silences":0,"longestSilenceMs":1013,"longestRecoveredGapMs":1013},"stats":{"sendRate":2250,"deliveredRate":1579.378268405151,"loss":0.4276315789473684,"srttMs":1018.5753269225181,"minRttMs":635,"sentPackets":411,"sentWireBytes":99393,"receivedPackets":859,"receivedWireBytes":336669,"lastHeardMs":52812,"unsyncedBytes":136,"inflightPackets":79,"duplicateBytes":0}}
- shim trace
21:27:42.345 fetch GET /api/auth/session
21:27:42.347 fetched GET /api/auth/session 200
21:27:42.454 fetch GET /.well-known/t3/environment
21:27:42.454 fetched GET /.well-known/t3/environment 200
21:27:42.462 fetch GET /.well-known/t3/environment
21:27:42.463 fetched GET /.well-known/t3/environment 200
21:27:42.466 rpc 0 subscribeServerConfig
21:27:42.466 chunk 0 38678 bytes
21:27:42.529 in Ack 0
21:27:42.531 rpc 1 subscribeServerLifecycle
21:27:42.534 rpc 2 subscribeServerLifecycle
21:27:42.537 fetch GET /api/auth/session
21:27:42.538 fetched GET /api/auth/session 200
21:27:42.538 rpc 3 subscribeDeviceState
21:27:42.541 rpc 4 preview.reportProfiles
21:27:42.589 rpc 5 subscribeProjectClones
21:27:42.607 fetch GET /api/orchestration/threads/mcp%3Af479e90b-21be-40f6-9818-f72927e474f3/bounded
21:27:42.611 fetch GET /api/orchestration/shell
21:27:42.631 rpc 6 server.refreshProviders
21:27:42.668 rpc 7 orchestration.dispatchCommand thread.visit
21:27:42.668 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:27:42.679 fetched GET /api/orchestration/shell 200
21:27:42.680 rpc 8 subscribeTerminalMetadata
21:27:42.680 rpc 9 subscribeVcsStatus
21:27:42.681 rpc 10 subscribePreviewEvents
21:27:42.683 rpc 11 orchestration.subscribeShell
21:27:42.683 chunk 11 2779 bytes
21:27:42.684 rpc 12 assets.createUrl
21:27:42.685 rpc 13 preview.list
21:27:42.686 rpc 14 projects.readFile
21:27:42.688 in Ack 11
21:27:42.892 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"761a7cfc-0b6e-4d13-af9f-df3cdb766083","sessions":[]}}
21:27:42.892 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:27:42.932 rpc 15 server.reportClientActivity
21:27:43.009 chunk 1 3861 bytes
21:27:43.010 in Ack 1
21:27:43.413 chunk 2 3861 bytes
21:27:43.413 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:27:30.096Z","configurableProviders":false,"con
21:27:43.413 chunk 9 539 bytes
21:27:43.416 in Ack 2
21:27:43.419 in Ack 9
21:27:43.452 fetch POST /api/observability/v1/traces
21:27:43.452 fetched POST /api/observability/v1/traces 204
21:27:43.516 chunk 11 2755 bytes
21:27:43.516 exit 12 {"_tag":"Success","value":{"expiresAt":1794175200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:27:43.517 in Ack 11
21:27:43.717 chunk 3 876 bytes
21:27:43.717 exit 4 {"_tag":"Success","value":null}
21:27:43.717 chunk 5 4 bytes
21:27:43.717 chunk 8 36 bytes
21:27:43.717 exit 15 {"_tag":"Success","value":null}
21:27:43.718 in Ack 3
21:27:43.741 rpc 16 preview.reportProfiles
21:27:43.743 in Ack 5
21:27:43.743 in Ack 8
21:27:43.827 chunk 11 2798 bytes
21:27:43.828 in Ack 11
21:27:43.846 exit 16 {"_tag":"Success","value":null}
21:27:43.928 fetched GET /api/orchestration/threads/mcp%3Af479e90b-21be-40f6-9818-f72927e474f3/bounded 200
21:27:43.942 rpc 17 orchestration.subscribeThread
21:27:43.943 chunk 17 8294 bytes
21:27:43.959 in Interrupt 9
21:27:43.964 rpc 18 subscribeVcsStatus
21:27:44.063 in Ack 17
21:27:44.177 chunk 18 539 bytes
21:27:44.178 in Ack 18
21:27:44.192 chunk 17 12047 bytes
21:27:44.194 in Ack 17
21:27:44.234 rpc 19 server.reportClientActivity
21:27:44.442 chunk 17 13517 bytes
21:27:44.443 in Ack 17
21:27:44.519 fetch POST /api/observability/v1/traces
21:27:44.519 fetched POST /api/observability/v1/traces 204
21:27:44.568 exit 19 {"_tag":"Success","value":null}
21:27:44.715 chunk 17 17224 bytes
21:27:44.716 in Ack 17
21:27:44.922 rpc 20 attachments.createUploadUrl
21:27:45.089 chunk 17 20474 bytes
21:27:45.095 in Ack 17
21:27:45.103 in Interrupt 18
21:27:45.105 rpc 21 subscribeVcsStatus
21:27:45.139 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-7716fc67-7859-43ac-9785-de6b4ecf8888","expiresAt":1794173865109,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:27:45.139 chunk 21 539 bytes
21:27:45.140 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTc3MTZmYzY3LTc4NTktNDNhYy05Nzg1LWRlNmI0ZWNmODg4OCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3Mzg2NTEwOX0.u22PDk5CZtSUf30eFboumXxASl46wXi5vWFNK_Kngu0
21:27:45.141 in Ack 21
21:27:45.339 chunk 17 21197 bytes
21:27:45.340 in Ack 17
21:27:45.357 rpc 22 server.reportClientActivity
21:27:45.569 fetch POST /api/observability/v1/traces
21:27:45.569 fetched POST /api/observability/v1/traces 204
21:27:46.446 exit 22 {"_tag":"Success","value":null}
21:27:47.448 fetch POST /api/observability/v1/traces
21:27:47.448 fetched POST /api/observability/v1/traces 204
21:27:47.464 in Ping 
21:27:48.101 rpc 23 orchestration.dispatchCommand message.dispatch
21:27:48.709 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:27:48.711 rpc 24 attachments.delete
21:27:49.100 fetch POST /api/observability/v1/traces
21:27:49.100 fetched POST /api/observability/v1/traces 204
21:27:49.210 chunk 17 23652 bytes
21:27:49.211 in Ack 17
21:27:49.470 chunk 17 23674 bytes
21:27:49.472 in Ack 17
21:27:49.720 chunk 17 23676 bytes
21:27:49.721 in Ack 17
21:27:49.970 chunk 11 2820 bytes
21:27:49.970 chunk 17 30526 bytes
21:27:49.971 in Ack 11
21:27:49.972 rpc 25 orchestration.dispatchCommand thread.visit
21:27:49.972 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:27:49.999 in Ack 17
21:27:50.002 rpc 26 assets.createUrl
21:27:50.213 fetch POST /api/observability/v1/traces
21:27:50.213 fetched POST /api/observability/v1/traces 204
21:27:50.221 chunk 17 31336 bytes
21:27:50.222 in Ack 17
21:27:50.359 exit 24 {"_tag":"Success","value":null}
21:27:50.470 chunk 11 2820 bytes
21:27:50.470 chunk 17 31950 bytes
21:27:50.471 in Ack 11
21:27:50.476 in Ack 17
21:27:50.482 in Interrupt 21
21:27:50.484 rpc 27 subscribeVcsStatus
21:27:50.538 chunk 27 543 bytes
21:27:50.538 in Ack 27
21:27:50.934 chunk 17 32727 bytes
21:27:50.936 rpc 28 server.reportClientActivity
21:27:50.937 in Ack 17
21:27:51.186 chunk 17 33297 bytes
21:27:51.187 in Ack 17
21:27:51.228 fetch POST /api/observability/v1/traces
21:27:51.228 fetched POST /api/observability/v1/traces 204
21:27:51.482 exit 28 {"_tag":"Success","value":null}
21:27:51.699 chunk 17 34208 bytes
21:27:51.702 in Ack 17
21:27:51.809 rpc 29 orchestration.dispatchCommand message.dispatch
21:27:52.472 in Ping 
21:27:52.490 fetch POST /api/observability/v1/traces
21:27:52.490 fetched POST /api/observability/v1/traces 204
21:27:52.941 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:27:52.981 chunk 17 36653 bytes
21:27:52.985 in Ack 17
21:27:53.231 chunk 17 36673 bytes
21:27:53.232 in Ack 17
21:27:53.481 chunk 17 36693 bytes
21:27:53.482 in Ack 17
21:27:53.731 chunk 17 36723 bytes
21:27:53.732 in Ack 17
21:27:53.942 fetch POST /api/observability/v1/traces
21:27:53.943 fetched POST /api/observability/v1/traces 204
21:27:53.982 chunk 17 36753 bytes
21:27:53.983 in Ack 17
21:27:54.232 chunk 17 36773 bytes
21:27:54.233 in Ack 17
21:27:54.482 chunk 17 36793 bytes
21:27:54.483 in Ack 17
21:27:54.732 chunk 17 36813 bytes
21:27:54.733 in Ack 17
21:27:54.982 chunk 17 36833 bytes
21:27:54.982 in Ack 17
21:27:54.991 fetch POST /api/observability/v1/traces
21:27:54.991 fetched POST /api/observability/v1/traces 204
21:27:55.232 chunk 17 36873 bytes
21:27:55.233 in Ack 17
21:27:55.483 chunk 17 36893 bytes
21:27:55.483 in Ack 17
21:27:55.733 chunk 17 36905 bytes
21:27:55.734 in Ack 17
21:27:55.983 chunk 17 36879 bytes
21:27:55.984 in Ack 17
21:27:56.234 fetch POST /api/observability/v1/traces
21:27:56.234 fetched POST /api/observability/v1/traces 204
21:27:56.235 chunk 17 36909 bytes
21:27:56.236 in Ack 17
21:27:56.484 chunk 17 36939 bytes
21:27:56.485 in Ack 17
21:27:56.734 chunk 17 36967 bytes
21:27:56.735 in Ack 17
21:27:57.146 chunk 17 36989 bytes
21:27:57.147 in Ack 17
21:27:57.237 fetch POST /api/observability/v1/traces
21:27:57.237 fetched POST /api/observability/v1/traces 204
21:27:57.397 chunk 17 37027 bytes
21:27:57.399 in Ack 17
21:27:57.478 in Ping 
21:27:57.648 chunk 17 37075 bytes
21:27:57.649 in Ack 17
21:27:57.899 chunk 17 37095 bytes
21:27:57.900 in Ack 17
21:27:58.149 chunk 17 37115 bytes
21:27:58.150 in Ack 17
21:27:58.399 chunk 17 37125 bytes
21:27:58.400 fetch POST /api/observability/v1/traces
21:27:58.400 fetched POST /api/observability/v1/traces 204
21:27:58.401 in Ack 17
21:27:58.657 chunk 17 37175 bytes
21:27:58.659 in Ack 17
21:27:58.908 chunk 17 37195 bytes
21:27:58.909 in Ack 17
21:27:59.158 chunk 17 37217 bytes
21:27:59.159 in Ack 17
21:27:59.402 fetch POST /api/observability/v1/traces
21:27:59.402 fetched POST /api/observability/v1/traces 204
21:27:59.409 chunk 17 37229 bytes
21:27:59.410 in Ack 17
21:27:59.659 chunk 17 37241 bytes
21:27:59.660 in Ack 17
21:28:00.048 chunk 17 37263 bytes
21:28:00.049 in Ack 17
21:28:00.298 chunk 17 37291 bytes
21:28:00.299 in Ack 17
21:28:00.411 fetch POST /api/observability/v1/traces
21:28:00.411 fetched POST /api/observability/v1/traces 204
21:28:00.547 chunk 11 2884 bytes
21:28:00.548 chunk 17 37347 bytes
21:28:00.548 in Ack 11
21:28:00.550 rpc 30 orchestration.dispatchCommand thread.visit
21:28:00.550 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:28:00.596 in Ack 17
21:28:00.799 chunk 17 37357 bytes
21:28:00.800 in Ack 17
21:28:01.048 chunk 17 37385 bytes
21:28:01.049 in Ack 17
21:28:01.299 chunk 17 37417 bytes
21:28:01.300 in Ack 17
21:28:01.549 chunk 17 37437 bytes
21:28:01.550 fetch POST /api/observability/v1/traces
21:28:01.550 fetched POST /api/observability/v1/traces 204
21:28:01.551 in Ack 17
21:28:01.800 chunk 17 37449 bytes
21:28:01.801 in Ack 17
21:28:02.051 chunk 17 37489 bytes
21:28:02.052 in Ack 17
21:28:02.301 chunk 17 37511 bytes
21:28:02.302 in Ack 17
21:28:02.478 in Ping 
21:28:02.550 chunk 17 37531 bytes
21:28:02.551 in Ack 17
21:28:02.569 fetch POST /api/observability/v1/traces
21:28:02.569 fetched POST /api/observability/v1/traces 204
21:28:02.801 chunk 17 37559 bytes
21:28:02.802 in Ack 17
21:28:03.052 chunk 17 37579 bytes
21:28:03.053 in Ack 17
21:28:03.302 chunk 17 37617 bytes
21:28:03.303 in Ack 17
21:28:03.552 chunk 17 37647 bytes
21:28:03.553 in Ack 17
21:28:03.801 chunk 17 37667 bytes
21:28:03.802 in Ack 17
21:28:03.821 fetch POST /api/observability/v1/traces
21:28:03.821 fetched POST /api/observability/v1/traces 204
21:28:04.052 chunk 17 37687 bytes
21:28:04.053 in Ack 17
21:28:04.302 chunk 17 37725 bytes
21:28:04.303 in Ack 17
21:28:04.552 chunk 17 37755 bytes
21:28:04.553 in Ack 17
21:28:04.803 chunk 17 37775 bytes
21:28:04.804 in Ack 17
21:28:05.072 chunk 17 41276 bytes
21:28:05.073 fetch POST /api/observability/v1/traces
21:28:05.073 fetched POST /api/observability/v1/traces 204
21:28:05.075 in Ack 17
21:28:05.322 chunk 11 2884 bytes
21:28:05.323 chunk 17 41306 bytes
21:28:05.323 in Ack 11
21:28:05.328 in Ack 17
21:28:05.572 chunk 17 41316 bytes
21:28:05.574 in Ack 17
21:28:05.823 chunk 17 41340 bytes
21:28:05.824 in Ack 17
21:28:06.074 chunk 17 41352 bytes
21:28:06.075 in Ack 17
21:28:06.084 fetch POST /api/observability/v1/traces
21:28:06.084 fetched POST /api/observability/v1/traces 204
21:28:06.326 chunk 17 41374 bytes
21:28:06.327 in Ack 17
21:28:06.576 chunk 17 41394 bytes
21:28:06.577 in Ack 17
21:28:06.827 chunk 17 41422 bytes
21:28:06.828 in Ack 17
21:28:07.078 chunk 17 41452 bytes
21:28:07.079 in Ack 17
21:28:07.328 chunk 17 42507 bytes
21:28:07.328 fetch POST /api/observability/v1/traces
21:28:07.328 fetched POST /api/observability/v1/traces 204
21:28:07.329 in Ack 17
21:28:07.478 in Ping 
21:28:07.578 chunk 17 42515 bytes
21:28:07.579 in Ack 17
21:28:07.706 rpc 31 server.reportClientActivity
21:28:07.828 chunk 17 42537 bytes
21:28:07.829 in Ack 17
21:28:08.079 chunk 17 42555 bytes
21:28:08.080 in Ack 17
21:28:08.329 chunk 17 42641 bytes
21:28:08.330 in Ack 17
21:28:08.339 fetch POST /api/observability/v1/traces
21:28:08.339 fetched POST /api/observability/v1/traces 204
21:28:08.566 exit 31 {"_tag":"Success","value":null}
21:28:08.578 chunk 17 42661 bytes
21:28:08.580 in Ack 17
21:28:08.835 chunk 17 44019 bytes
21:28:08.837 in Ack 17
21:28:09.086 chunk 17 44049 bytes
21:28:09.087 in Ack 17
21:28:09.335 chunk 17 44079 bytes
21:28:09.336 in Ack 17
21:28:09.567 fetch POST /api/observability/v1/traces
21:28:09.567 fetched POST /api/observability/v1/traces 204
21:28:09.585 chunk 17 46100 bytes
21:28:09.586 in Ack 17
21:28:09.836 chunk 17 46130 bytes
21:28:09.837 in Ack 17
21:28:10.085 chunk 17 46160 bytes
21:28:10.086 in Ack 17
21:28:10.336 chunk 17 46205 bytes
21:28:10.337 in Ack 17
21:28:10.550 rpc 32 orchestration.dispatchCommand thread.visit
21:28:10.550 exit 32 {"_tag":"Success","value":{"sequence":0}}
21:28:10.586 chunk 17 46250 bytes
21:28:10.587 in Ack 17
21:28:10.597 fetch POST /api/observability/v1/traces
21:28:10.597 fetched POST /api/observability/v1/traces 204
21:28:10.876 chunk 17 46280 bytes
21:28:10.877 in Ack 17
21:28:11.127 chunk 17 46325 bytes
21:28:11.128 in Ack 17
21:28:11.376 chunk 17 46355 bytes
21:28:11.377 in Ack 17
21:28:11.627 chunk 17 46400 bytes
21:28:11.628 in Ack 17
21:28:11.877 chunk 17 46430 bytes
21:28:11.878 in Ack 17
21:28:11.886 fetch POST /api/observability/v1/traces
21:28:11.886 fetched POST /api/observability/v1/traces 204
21:28:12.127 chunk 11 2884 bytes
21:28:12.127 chunk 17 46475 bytes
21:28:12.128 in Ack 11
21:28:12.134 in Ack 17
21:28:12.378 chunk 17 46505 bytes
21:28:12.379 in Ack 17
21:28:12.478 in Ping 
21:28:12.628 chunk 17 46535 bytes
21:28:12.629 in Ack 17
21:28:12.879 chunk 17 46580 bytes
21:28:12.880 in Ack 17
21:28:12.909 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/c56d3ccd-fd02-4ea4-b976-858144f8c22d","expiresAt":1794176892909}}
21:28:13.129 fetch POST /api/observability/v1/traces
21:28:13.129 fetched POST /api/observability/v1/traces 204
21:28:13.130 chunk 17 46625 bytes
21:28:13.131 in Ack 17
21:28:13.395 chunk 17 46655 bytes
21:28:13.399 in Ack 17
21:28:13.646 chunk 17 46700 bytes
21:28:13.647 in Ack 17
21:28:13.897 chunk 17 46730 bytes
21:28:13.898 in Ack 17
21:28:14.132 fetch POST /api/observability/v1/traces
21:28:14.132 fetched POST /api/observability/v1/traces 204
21:28:14.148 chunk 17 46775 bytes
21:28:14.149 in Ack 17
21:28:14.398 chunk 17 46805 bytes
21:28:14.399 in Ack 17
21:28:14.649 chunk 17 46850 bytes
21:28:14.652 in Ack 17
21:28:14.899 chunk 17 46895 bytes
21:28:14.901 in Ack 17
21:28:15.150 chunk 17 46925 bytes
21:28:15.150 fetch POST /api/observability/v1/traces
21:28:15.150 fetched POST /api/observability/v1/traces 204
21:28:15.151 in Ack 17
21:28:15.399 chunk 17 46955 bytes
21:28:15.400 in Ack 17
21:28:15.650 chunk 17 47000 bytes
21:28:15.651 in Ack 17
21:28:15.901 chunk 17 47030 bytes
21:28:15.902 in Ack 17
21:28:16.152 chunk 17 47060 bytes
21:28:16.152 fetch POST /api/observability/v1/traces
21:28:16.152 fetched POST /api/observability/v1/traces 204
21:28:16.154 in Ack 17
21:28:16.401 chunk 17 47090 bytes
21:28:16.402 in Ack 17
21:28:16.652 chunk 17 47120 bytes
21:28:16.653 in Ack 17
21:28:16.902 chunk 17 47150 bytes
21:28:16.903 in Ack 17
21:28:17.152 chunk 17 47180 bytes
21:28:17.153 in Ack 17
21:28:17.162 fetch POST /api/observability/v1/traces
21:28:17.162 fetched POST /api/observability/v1/traces 204
21:28:17.403 chunk 17 47210 bytes
21:28:17.404 in Ack 17
21:28:17.478 in Ping 
21:28:17.653 chunk 17 47240 bytes
21:28:17.654 in Ack 17
21:28:17.904 chunk 17 47285 bytes
21:28:17.905 in Ack 17
21:28:18.155 chunk 17 47330 bytes
21:28:18.156 in Ack 17
21:28:18.405 fetch POST /api/observability/v1/traces
21:28:18.405 fetched POST /api/observability/v1/traces 204
21:28:18.405 chunk 17 47360 bytes
21:28:18.406 in Ack 17
21:28:18.655 chunk 17 47405 bytes
21:28:18.657 in Ack 17
21:28:18.906 chunk 17 47450 bytes
21:28:18.907 in Ack 17
21:28:19.156 chunk 17 47480 bytes
21:28:19.158 in Ack 17
21:28:19.406 chunk 17 47510 bytes
21:28:19.407 in Ack 17
21:28:19.462 fetch POST /api/observability/v1/traces
21:28:19.462 fetched POST /api/observability/v1/traces 204
21:28:19.657 chunk 17 47585 bytes
21:28:19.658 in Ack 17
21:28:19.907 chunk 17 47630 bytes
21:28:19.908 in Ack 17
21:28:20.157 chunk 17 48650 bytes
21:28:20.158 in Ack 17
21:28:20.414 chunk 17 52155 bytes
21:28:20.416 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:28:20.463 in Ack 17
21:28:20.659 fetch POST /api/observability/v1/traces
21:28:20.659 fetched POST /api/observability/v1/traces 204
21:28:20.914 chunk 11 3193 bytes
21:28:20.915 chunk 17 54238 bytes
21:28:20.915 in Ack 11
21:28:20.924 in Ack 17
21:28:21.307 exit 33 {"_tag":"Success","value":{"sequence":97}}
21:28:21.308 rpc 34 orchestration.dispatchCommand thread.visit
21:28:21.308 exit 34 {"_tag":"Success","value":{"sequence":0}}
21:28:21.417 chunk 17 57684 bytes
21:28:21.419 in Ack 17
21:28:21.674 chunk 17 57757 bytes
21:28:21.686 in Ack 17
21:28:21.917 fetch POST /api/observability/v1/traces
21:28:21.917 fetched POST /api/observability/v1/traces 204
21:28:21.925 chunk 17 57777 bytes
21:28:21.927 in Ack 17
21:28:22.175 chunk 17 57797 bytes
21:28:22.176 in Ack 17
21:28:22.425 chunk 17 57877 bytes
21:28:22.426 in Ack 17
21:28:22.479 in Ping 
21:28:22.675 chunk 17 57897 bytes
21:28:22.676 in Ack 17
21:28:22.925 chunk 17 57919 bytes
21:28:22.926 in Ack 17
21:28:22.935 fetch POST /api/observability/v1/traces
21:28:22.935 fetched POST /api/observability/v1/traces 204
21:28:23.175 chunk 17 57929 bytes
21:28:23.176 in Ack 17
21:28:23.426 chunk 17 57923 bytes
21:28:23.427 in Ack 17
21:28:23.675 chunk 17 57949 bytes
21:28:23.676 in Ack 17
21:28:23.925 chunk 17 59002 bytes
21:28:23.926 in Ack 17
21:28:24.176 chunk 17 59032 bytes
21:28:24.177 in Ack 17
21:28:24.195 fetch POST /api/observability/v1/traces
21:28:24.195 fetched POST /api/observability/v1/traces 204
21:28:24.427 chunk 17 59052 bytes
21:28:24.428 in Ack 17
21:28:24.676 chunk 17 59082 bytes
21:28:24.677 in Ack 17
21:28:24.927 chunk 17 59128 bytes
21:28:24.929 in Ack 17
21:28:25.180 chunk 11 2886 bytes
21:28:25.180 chunk 17 60555 bytes
21:28:25.181 in Ack 11
21:28:25.187 in Ack 17
21:28:25.429 fetch POST /api/observability/v1/traces
21:28:25.429 fetched POST /api/observability/v1/traces 204
21:28:25.431 chunk 17 60613 bytes
21:28:25.432 in Ack 17
21:28:25.681 chunk 17 60653 bytes
21:28:25.683 in Ack 17
21:28:25.931 chunk 17 60683 bytes
21:28:25.932 in Ack 17
21:28:26.182 chunk 17 60713 bytes
21:28:26.183 in Ack 17
21:28:26.432 chunk 17 60781 bytes
21:28:26.433 in Ack 17
21:28:26.440 in Interrupt 27
21:28:26.466 rpc 35 subscribeVcsStatus
21:28:26.500 fetch POST /api/observability/v1/traces
21:28:26.501 fetched POST /api/observability/v1/traces 204
21:28:26.682 chunk 17 60801 bytes
21:28:26.683 in Ack 17
21:28:26.717 rpc 36 server.reportClientActivity
21:28:26.933 chunk 17 60842 bytes
21:28:26.934 in Ack 17
21:28:27.174 chunk 35 543 bytes
21:28:27.174 in Ack 35
21:28:27.190 chunk 17 60872 bytes
21:28:27.191 in Ack 17
21:28:27.440 chunk 17 60892 bytes
21:28:27.441 in Ack 17
21:28:27.479 in Ping 
21:28:27.684 fetch POST /api/observability/v1/traces
21:28:27.684 fetched POST /api/observability/v1/traces 204
21:28:27.690 chunk 17 60912 bytes
21:28:27.691 in Ack 17
21:28:27.942 chunk 17 60932 bytes
21:28:27.943 in Ack 17
21:28:28.192 chunk 17 60972 bytes
21:28:28.193 in Ack 17
21:28:28.442 chunk 17 60982 bytes
21:28:28.443 in Ack 17
21:28:28.692 fetch POST /api/observability/v1/traces
21:28:28.692 fetched POST /api/observability/v1/traces 204
21:28:28.693 chunk 17 61022 bytes
21:28:28.694 in Ack 17
21:28:28.949 chunk 17 61044 bytes
21:28:28.950 in Ack 17
21:28:29.200 chunk 17 61060 bytes
21:28:29.201 in Ack 17
21:28:29.617 chunk 17 61100 bytes
21:28:29.619 in Ack 17
21:28:29.695 fetch POST /api/observability/v1/traces
21:28:29.695 fetched POST /api/observability/v1/traces 204
21:28:29.868 chunk 17 61120 bytes
21:28:29.869 in Ack 17
21:28:30.118 chunk 11 2886 bytes
21:28:30.118 in Ack 11
21:28:30.120 chunk 17 61150 bytes
21:28:30.121 in Ack 17
21:28:30.369 chunk 17 61170 bytes
21:28:30.370 in Ack 17
21:28:30.619 chunk 17 61204 bytes
21:28:30.620 in Ack 17
21:28:30.870 chunk 17 61234 bytes
21:28:30.870 fetch POST /api/observability/v1/traces
21:28:30.870 fetched POST /api/observability/v1/traces 204
21:28:30.871 in Ack 17
21:28:30.928 rpc 37 orchestration.dispatchCommand thread.visit
21:28:30.928 exit 37 {"_tag":"Success","value":{"sequence":0}}
21:28:30.945 exit 36 {"_tag":"Success","value":null}
21:28:31.119 chunk 17 62322 bytes
21:28:31.121 in Ack 17
21:28:31.370 chunk 17 62352 bytes
21:28:31.371 in Ack 17
21:28:31.620 chunk 17 62397 bytes
21:28:31.621 in Ack 17
21:28:31.870 chunk 17 62433 bytes
21:28:31.871 in Ack 17
21:28:31.922 fetch POST /api/observability/v1/traces
21:28:31.922 fetched POST /api/observability/v1/traces 204
21:28:32.118 chunk 11 2886 bytes
21:28:32.119 in Ack 11
21:28:32.124 chunk 17 62463 bytes
21:28:32.125 in Ack 17
21:28:32.374 chunk 17 62478 bytes
21:28:32.375 in Ack 17
21:28:32.479 in Ping 
21:28:32.624 chunk 17 62538 bytes
21:28:32.625 in Ack 17
21:28:32.706 rpc 38 server.reportClientActivity
21:28:32.875 chunk 17 62577 bytes
21:28:32.876 in Ack 17
21:28:33.120 fetch POST /api/observability/v1/traces
21:28:33.120 fetched POST /api/observability/v1/traces 204
21:28:33.124 chunk 17 62592 bytes
21:28:33.125 in Ack 17
21:28:33.375 chunk 17 62637 bytes
21:28:33.376 in Ack 17
21:28:33.624 chunk 17 62652 bytes
21:28:33.626 in Ack 17
21:28:33.874 chunk 17 62709 bytes
21:28:33.876 in Ack 17
21:28:34.126 chunk 17 62748 bytes
21:28:34.126 fetch POST /api/observability/v1/traces
21:28:34.126 fetched POST /api/observability/v1/traces 204
21:28:34.128 in Ack 17
21:28:34.375 chunk 17 62793 bytes
21:28:34.376 in Ack 17
21:28:34.624 chunk 17 62823 bytes
21:28:34.626 in Ack 17
21:28:34.875 chunk 17 62874 bytes
21:28:34.876 in Ack 17
21:28:35.125 chunk 17 62913 bytes
21:28:35.126 in Ack 17
21:28:35.252 fetch POST /api/observability/v1/traces
21:28:35.252 fetched POST /api/observability/v1/traces 204
21:28:35.374 chunk 17 62943 bytes
21:28:35.376 in Ack 17
21:28:35.483 exit 38 {"_tag":"Success","value":null}
21:28:35.625 chunk 17 63006 bytes
21:28:35.626 in Ack 17
21:28:35.875 chunk 17 63206 bytes
21:28:35.876 in Ack 17
21:28:36.162 chunk 11 2822 bytes
21:28:36.163 in Ack 11
21:28:36.165 rpc 39 orchestration.dispatchCommand thread.visit
21:28:36.165 exit 39 {"_tag":"Success","value":{"sequence":0}}
