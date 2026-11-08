# case50, webapp, warm start, seed 4

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 44 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.2s |
| Prompt reached the agent | 0.7s |
| Time to first text | 1.8s |
| Whole reply shown | 43.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 4.0s / 1.3s / 2.2s |
| Longest silence | 2.4s |
| Approval shown after it was asked | 0.6s |
| Bytes on the wire down / up | 359318 / 138880 |
| Bytes the app received / sent | 295875 / 79409 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.0 | 291 | 369 |
| 20 | 0.8 | 738 | 770 |
| 30 | 1.2 | 1086 | 1132 |
| 40 | 1.0 | 1472 | 1505 |
| 43 | 1.4 | 1560 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058","thread":"951/951","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"661/661 661/661","providerTurns":"776/776","providerThreads":"809/809","messages":"456/456 548/548 127/901","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"359/613 401/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794174734144,"silences":0,"longestSilenceMs":1466,"longestRecoveredGapMs":1466},"stats":{"sendRate":7231.212017131467,"deliveredRate":3111.3692664837145,"loss":0,"srttMs":46.588511103934756,"minRttMs":1,"sentPackets":118,"sentWireBytes":35907,"receivedPackets":110,"receivedWireBytes":50760,"lastHeardMs":10115,"unsyncedBytes":998,"inflightPackets":18,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","assistant":"32/32 21/21 425/450","runs":"1014/1014 971/971 254/839","user":"10/10 9/9","attempts":"661/661 661/661 570/637","providerTurns":"776/776 56/776","providerThreads":"692/809","messages":"456/456 548/548 341/901 467/548","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809","checkpoints":"613/613 401/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794174744182,"silences":0,"longestSilenceMs":1466,"longestRecoveredGapMs":1466},"stats":{"sendRate":3615.6060085657336,"deliveredRate":1768.4747387671805,"loss":0.2986209150326797,"srttMs":548.366580007995,"minRttMs":1,"sentPackets":169,"sentWireBytes":46881,"receivedPackets":183,"receivedWireBytes":75674,"lastHeardMs":20154,"unsyncedBytes":0,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1078/1078","thread":"951/951","assistant":"32/32 21/21 953/953","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","messages":"456/456 548/548 901/901 548/548 547/547 451/451","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 785/785 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794174754481,"silences":0,"longestSilenceMs":1466,"longestRecoveredGapMs":1466},"stats":{"sendRate":3375,"deliveredRate":960.5587510271159,"loss":0.43999999999999995,"srttMs":647.72126448349,"minRttMs":564,"sentPackets":233,"sentWireBytes":60315,"receivedPackets":324,"receivedWireBytes":137486,"lastHeardMs":30452,"unsyncedBytes":0,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 48/1574 929/929 1078/1078","thread":"951/951","assistant":"32/32 21/21 1340/1340 133/133","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","messages":"456/456 548/548 901/901 548/548 547/547 451/451","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 785/785 1022/1022 18/987 1398/1398 711/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794174765264,"silences":0,"longestSilenceMs":1466,"longestRecoveredGapMs":1466},"stats":{"sendRate":4500,"deliveredRate":0,"loss":0.75,"srttMs":750.743153046415,"minRttMs":564,"sentPackets":300,"sentWireBytes":75573,"receivedPackets":596,"receivedWireBytes":237305,"lastHeardMs":41235,"unsyncedBytes":0,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1058/1058 1103/1103 870/870 860/860 1299/1299 1103/1103 1058/1058 849/849 1103/1103 1574/1574 953/953 1078/1078","thread":"951/951","assistant":"32/32 21/21 1340/1340 679/679","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 64/64","attempts":"661/661 661/661 637/637","providerTurns":"776/776 776/776 658/658","providerThreads":"811/811","messages":"456/456 548/548 901/901 548/548 548/548 451/451 547/547","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 785/785 1022/1022 987/987 1398/1398 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1051/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":237,"lastDataAt":1794174776229,"silences":0,"longestSilenceMs":1466,"longestRecoveredGapMs":1466},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.49968167121212115,"srttMs":1076.1514911655493,"minRttMs":564,"sentPackets":415,"sentWireBytes":105429,"receivedPackets":867,"receivedWireBytes":334192,"lastHeardMs":52200,"unsyncedBytes":473,"inflightPackets":101,"duplicateBytes":0}}
- shim trace
21:52:04.303 fetch GET /api/auth/session
21:52:04.304 fetched GET /api/auth/session 200
21:52:04.449 fetch GET /.well-known/t3/environment
21:52:04.450 fetched GET /.well-known/t3/environment 200
21:52:04.458 fetch GET /.well-known/t3/environment
21:52:04.459 fetched GET /.well-known/t3/environment 200
21:52:04.462 rpc 0 subscribeServerConfig
21:52:04.462 chunk 0 38678 bytes
21:52:04.478 in Ack 0
21:52:04.480 rpc 1 subscribeServerLifecycle
21:52:04.481 rpc 2 subscribeServerLifecycle
21:52:04.483 fetch GET /api/auth/session
21:52:04.484 fetched GET /api/auth/session 200
21:52:04.484 rpc 3 subscribeDeviceState
21:52:04.486 rpc 4 preview.reportProfiles
21:52:04.533 rpc 5 subscribeProjectClones
21:52:04.549 fetch GET /api/orchestration/threads/mcp%3A21bd9fd3-bed3-403e-bd3a-fc5bc9476259/bounded
21:52:04.554 fetch GET /api/orchestration/shell
21:52:04.573 rpc 6 server.refreshProviders
21:52:04.610 rpc 7 orchestration.dispatchCommand thread.visit
21:52:04.610 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:52:04.622 fetched GET /api/orchestration/shell 200
21:52:04.622 rpc 8 subscribeTerminalMetadata
21:52:04.623 rpc 9 subscribeVcsStatus
21:52:04.623 rpc 10 subscribePreviewEvents
21:52:04.625 rpc 11 orchestration.subscribeShell
21:52:04.625 chunk 11 2779 bytes
21:52:04.627 rpc 12 assets.createUrl
21:52:04.628 rpc 13 preview.list
21:52:04.628 rpc 14 projects.readFile
21:52:04.636 in Ack 11
21:52:04.877 rpc 15 server.reportClientActivity
21:52:04.911 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"24465333-92f3-477f-8ff5-6f22c5358d6c","sessions":[]}}
21:52:04.911 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:52:05.179 chunk 1 3861 bytes
21:52:05.179 in Ack 1
21:52:05.399 chunk 2 3861 bytes
21:52:05.400 in Ack 2
21:52:05.452 fetch POST /api/observability/v1/traces
21:52:05.452 fetched POST /api/observability/v1/traces 204
21:52:05.601 chunk 11 2755 bytes
21:52:05.601 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:51:52.576Z","configurableProviders":false,"con
21:52:05.601 chunk 9 472 bytes
21:52:05.601 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:52:05.602 in Ack 11
21:52:05.608 in Ack 9
21:52:06.078 chunk 3 876 bytes
21:52:06.078 exit 4 {"_tag":"Success","value":null}
21:52:06.078 chunk 5 4 bytes
21:52:06.078 chunk 8 36 bytes
21:52:06.078 exit 15 {"_tag":"Success","value":null}
21:52:06.078 in Ack 3
21:52:06.082 rpc 16 preview.reportProfiles
21:52:06.082 in Ack 5
21:52:06.082 in Ack 8
21:52:06.102 chunk 11 2798 bytes
21:52:06.103 in Ack 11
21:52:06.178 exit 16 {"_tag":"Success","value":null}
21:52:06.498 fetched GET /api/orchestration/threads/mcp%3A21bd9fd3-bed3-403e-bd3a-fc5bc9476259/bounded 200
21:52:06.502 rpc 17 orchestration.subscribeThread
21:52:06.502 chunk 17 8294 bytes
21:52:06.511 in Interrupt 9
21:52:06.513 rpc 18 subscribeVcsStatus
21:52:06.514 in Ack 17
21:52:06.598 chunk 18 472 bytes
21:52:06.599 in Ack 18
21:52:06.603 fetch POST /api/observability/v1/traces
21:52:06.603 fetched POST /api/observability/v1/traces 204
21:52:06.752 chunk 17 8334 bytes
21:52:06.753 in Ack 17
21:52:06.774 rpc 19 server.reportClientActivity
21:52:06.868 exit 19 {"_tag":"Success","value":null}
21:52:07.003 chunk 17 11932 bytes
21:52:07.067 in Ack 17
21:52:07.259 chunk 17 13517 bytes
21:52:07.260 in Ack 17
21:52:07.312 rpc 20 attachments.createUploadUrl
21:52:07.518 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-0c5e2800-7614-4af2-bfaa-fb7ed54de0e7","expiresAt":1794175327498,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:52:07.519 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTBjNWUyODAwLTc2MTQtNGFmMi1iZmFhLWZiN2VkNTRkZTBlNyIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NTMyNzQ5OH0.2fa2U5bLJ7QMmbzYSnu9u67obCZAWZomifOOmQmt8lM
21:52:07.754 fetch POST /api/observability/v1/traces
21:52:07.754 fetched POST /api/observability/v1/traces 204
21:52:07.760 chunk 17 18034 bytes
21:52:07.761 in Ack 17
21:52:08.011 chunk 17 20474 bytes
21:52:08.013 in Ack 17
21:52:08.021 in Interrupt 18
21:52:08.023 rpc 21 subscribeVcsStatus
21:52:08.081 chunk 21 472 bytes
21:52:08.082 in Ack 21
21:52:08.260 chunk 17 21197 bytes
21:52:08.261 in Ack 17
21:52:08.276 rpc 22 server.reportClientActivity
21:52:08.762 fetch POST /api/observability/v1/traces
21:52:08.762 fetched POST /api/observability/v1/traces 204
21:52:09.328 exit 22 {"_tag":"Success","value":null}
21:52:09.461 in Ping 
21:52:10.397 rpc 23 orchestration.dispatchCommand message.dispatch
21:52:10.957 fetch POST /api/observability/v1/traces
21:52:10.957 fetched POST /api/observability/v1/traces 204
21:52:11.020 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:52:11.021 rpc 24 attachments.delete
21:52:11.108 exit 24 {"_tag":"Success","value":null}
21:52:11.286 chunk 17 23642 bytes
21:52:11.287 in Ack 17
21:52:11.536 chunk 17 23652 bytes
21:52:11.537 in Ack 17
21:52:11.789 chunk 11 2884 bytes
21:52:11.789 chunk 17 23626 bytes
21:52:11.790 in Ack 11
21:52:11.836 in Ack 17
21:52:12.021 fetch POST /api/observability/v1/traces
21:52:12.021 fetched POST /api/observability/v1/traces 204
21:52:12.039 chunk 17 23676 bytes
21:52:12.040 in Ack 17
21:52:12.289 chunk 11 2820 bytes
21:52:12.290 in Ack 11
21:52:12.291 rpc 25 orchestration.dispatchCommand thread.visit
21:52:12.291 exit 25 {"_tag":"Success","value":{"sequence":0}}
21:52:12.538 chunk 17 24648 bytes
21:52:12.539 in Ack 17
21:52:12.788 chunk 11 2820 bytes
21:52:12.789 chunk 17 28103 bytes
21:52:12.789 in Ack 11
21:52:12.795 in Ack 17
21:52:12.797 rpc 26 assets.createUrl
21:52:13.273 chunk 17 32116 bytes
21:52:13.274 fetch POST /api/observability/v1/traces
21:52:13.274 fetched POST /api/observability/v1/traces 204
21:52:13.275 in Ack 17
21:52:13.282 in Interrupt 21
21:52:13.284 rpc 27 subscribeVcsStatus
21:52:13.540 rpc 28 server.reportClientActivity
21:52:14.112 rpc 29 orchestration.dispatchCommand message.dispatch
21:52:14.276 fetch POST /api/observability/v1/traces
21:52:14.276 fetched POST /api/observability/v1/traces 204
21:52:14.288 chunk 27 472 bytes
21:52:14.288 in Ack 27
21:52:14.461 in Ping 
21:52:14.663 chunk 17 31950 bytes
21:52:14.664 in Ack 17
21:52:15.009 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:52:15.415 chunk 17 34395 bytes
21:52:15.418 in Ack 17
21:52:15.665 chunk 17 34435 bytes
21:52:15.666 fetch POST /api/observability/v1/traces
21:52:15.666 fetched POST /api/observability/v1/traces 204
21:52:15.667 in Ack 17
21:52:16.099 chunk 17 34447 bytes
21:52:16.100 in Ack 17
21:52:16.350 chunk 17 34457 bytes
21:52:16.351 in Ack 17
21:52:16.601 chunk 17 34467 bytes
21:52:16.602 in Ack 17
21:52:16.667 fetch POST /api/observability/v1/traces
21:52:16.667 fetched POST /api/observability/v1/traces 204
21:52:16.851 chunk 17 34489 bytes
21:52:16.852 in Ack 17
21:52:17.106 chunk 17 34557 bytes
21:52:17.108 in Ack 17
21:52:17.853 fetch POST /api/observability/v1/traces
21:52:17.853 fetched POST /api/observability/v1/traces 204
21:52:18.179 chunk 17 34589 bytes
21:52:18.180 in Ack 17
21:52:18.430 chunk 17 34649 bytes
21:52:18.431 in Ack 17
21:52:18.681 chunk 17 34677 bytes
21:52:18.682 in Ack 17
21:52:18.931 chunk 17 34755 bytes
21:52:18.932 in Ack 17
21:52:19.182 fetch POST /api/observability/v1/traces
21:52:19.182 fetched POST /api/observability/v1/traces 204
21:52:19.182 chunk 17 34785 bytes
21:52:19.183 in Ack 17
21:52:19.433 chunk 17 34805 bytes
21:52:19.434 in Ack 17
21:52:19.473 in Ping 
21:52:19.696 chunk 17 34817 bytes
21:52:19.697 in Ack 17
21:52:19.946 chunk 17 34829 bytes
21:52:19.947 in Ack 17
21:52:20.184 fetch POST /api/observability/v1/traces
21:52:20.184 fetched POST /api/observability/v1/traces 204
21:52:20.196 chunk 17 34859 bytes
21:52:20.197 in Ack 17
21:52:20.446 chunk 17 34849 bytes
21:52:20.447 in Ack 17
21:52:20.696 chunk 17 34887 bytes
21:52:20.697 in Ack 17
21:52:20.946 chunk 17 34909 bytes
21:52:20.948 in Ack 17
21:52:21.198 fetch POST /api/observability/v1/traces
21:52:21.198 fetched POST /api/observability/v1/traces 204
21:52:21.476 chunk 17 34921 bytes
21:52:21.477 in Ack 17
21:52:21.726 chunk 17 34953 bytes
21:52:21.727 in Ack 17
21:52:21.977 chunk 17 34971 bytes
21:52:21.978 in Ack 17
21:52:22.228 chunk 17 34983 bytes
21:52:22.229 in Ack 17
21:52:22.478 fetch POST /api/observability/v1/traces
21:52:22.478 fetched POST /api/observability/v1/traces 204
21:52:22.478 chunk 17 35005 bytes
21:52:22.480 in Ack 17
21:52:22.729 chunk 17 35017 bytes
21:52:22.730 in Ack 17
21:52:22.979 chunk 17 35073 bytes
21:52:22.980 in Ack 17
21:52:23.229 chunk 17 35093 bytes
21:52:23.230 in Ack 17
21:52:23.479 chunk 17 35131 bytes
21:52:23.480 in Ack 17
21:52:23.506 fetch POST /api/observability/v1/traces
21:52:23.506 fetched POST /api/observability/v1/traces 204
21:52:23.730 chunk 17 35141 bytes
21:52:23.731 in Ack 17
21:52:23.980 chunk 17 35171 bytes
21:52:23.981 in Ack 17
21:52:24.230 chunk 17 35193 bytes
21:52:24.231 in Ack 17
21:52:24.474 in Ping 
21:52:24.480 chunk 17 35213 bytes
21:52:24.481 in Ack 17
21:52:24.731 chunk 17 37244 bytes
21:52:24.732 in Ack 17
21:52:24.746 fetch POST /api/observability/v1/traces
21:52:24.746 fetched POST /api/observability/v1/traces 204
21:52:24.979 chunk 11 2884 bytes
21:52:24.979 in Ack 11
21:52:24.980 rpc 30 orchestration.dispatchCommand thread.visit
21:52:24.980 exit 30 {"_tag":"Success","value":{"sequence":0}}
21:52:25.019 chunk 17 37266 bytes
21:52:25.024 in Ack 17
21:52:25.270 chunk 17 37934 bytes
21:52:25.271 in Ack 17
21:52:25.520 chunk 17 37956 bytes
21:52:25.521 in Ack 17
21:52:25.770 chunk 17 37976 bytes
21:52:25.772 in Ack 17
21:52:25.980 fetch POST /api/observability/v1/traces
21:52:25.980 fetched POST /api/observability/v1/traces 204
21:52:26.021 chunk 17 38004 bytes
21:52:26.022 in Ack 17
21:52:26.271 chunk 17 38042 bytes
21:52:26.272 in Ack 17
21:52:26.522 chunk 17 38080 bytes
21:52:26.523 in Ack 17
21:52:26.772 chunk 17 38100 bytes
21:52:26.773 in Ack 17
21:52:27.022 chunk 17 38132 bytes
21:52:27.023 in Ack 17
21:52:27.031 fetch POST /api/observability/v1/traces
21:52:27.031 fetched POST /api/observability/v1/traces 204
21:52:27.272 chunk 17 38160 bytes
21:52:27.273 in Ack 17
21:52:27.524 chunk 17 39020 bytes
21:52:27.525 in Ack 17
21:52:27.775 chunk 17 39032 bytes
21:52:27.776 in Ack 17
21:52:28.026 chunk 17 39074 bytes
21:52:28.027 in Ack 17
21:52:28.147 fetch POST /api/observability/v1/traces
21:52:28.147 fetched POST /api/observability/v1/traces 204
21:52:28.235 chunk 11 2884 bytes
21:52:28.236 in Ack 11
21:52:28.276 chunk 17 39084 bytes
21:52:28.277 in Ack 17
21:52:28.527 chunk 17 39116 bytes
21:52:28.528 in Ack 17
21:52:28.546 in Interrupt 28
21:52:28.777 chunk 17 39164 bytes
21:52:28.778 in Ack 17
21:52:29.237 fetch POST /api/observability/v1/traces
21:52:29.237 fetched POST /api/observability/v1/traces 204
21:52:29.278 chunk 17 39186 bytes
21:52:29.279 in Ack 17
21:52:29.474 in Ping 
21:52:29.529 chunk 17 39226 bytes
21:52:29.530 in Ack 17
21:52:29.701 rpc 31 server.reportClientActivity
21:52:29.781 chunk 17 39246 bytes
21:52:29.782 in Ack 17
21:52:30.031 chunk 17 39290 bytes
21:52:30.032 in Ack 17
21:52:30.280 fetch POST /api/observability/v1/traces
21:52:30.280 fetched POST /api/observability/v1/traces 204
21:52:30.281 chunk 17 39312 bytes
21:52:30.282 in Ack 17
21:52:30.531 chunk 17 39342 bytes
21:52:30.532 in Ack 17
21:52:30.781 chunk 17 39352 bytes
21:52:30.782 in Ack 17
21:52:31.031 chunk 17 41515 bytes
21:52:31.033 in Ack 17
21:52:31.281 chunk 17 42204 bytes
21:52:31.282 in Ack 17
21:52:31.290 fetch POST /api/observability/v1/traces
21:52:31.291 fetched POST /api/observability/v1/traces 204
21:52:31.515 exit 28 {"_tag":"Success","value":null}
21:52:31.531 chunk 17 42765 bytes
21:52:31.532 in Ack 17
21:52:31.782 chunk 17 42795 bytes
21:52:31.784 in Ack 17
21:52:32.034 chunk 17 43395 bytes
21:52:32.035 in Ack 17
21:52:32.284 chunk 17 44211 bytes
21:52:32.285 in Ack 17
21:52:32.533 fetch POST /api/observability/v1/traces
21:52:32.533 fetched POST /api/observability/v1/traces 204
21:52:32.534 chunk 17 44241 bytes
21:52:32.535 in Ack 17
21:52:32.785 chunk 17 44316 bytes
21:52:32.786 in Ack 17
21:52:33.035 chunk 17 46280 bytes
21:52:33.036 in Ack 17
21:52:33.285 chunk 17 46295 bytes
21:52:33.286 in Ack 17
21:52:33.310 exit 31 {"_tag":"Success","value":null}
21:52:33.535 chunk 17 46340 bytes
21:52:33.537 in Ack 17
21:52:33.561 fetch POST /api/observability/v1/traces
21:52:33.561 fetched POST /api/observability/v1/traces 204
21:52:33.784 chunk 17 46370 bytes
21:52:33.786 in Ack 17
21:52:34.035 chunk 17 46400 bytes
21:52:34.036 in Ack 17
21:52:34.285 chunk 17 46415 bytes
21:52:34.286 in Ack 17
21:52:34.474 in Ping 
21:52:34.535 chunk 17 46475 bytes
21:52:34.536 in Ack 17
21:52:34.785 chunk 17 46505 bytes
21:52:34.786 in Ack 17
21:52:34.795 fetch POST /api/observability/v1/traces
21:52:34.795 fetched POST /api/observability/v1/traces 204
21:52:34.927 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/b14b628e-985f-405b-836a-1c3822e921b1","expiresAt":1794178354927}}
21:52:34.981 rpc 32 orchestration.dispatchCommand thread.visit
21:52:34.981 exit 32 {"_tag":"Success","value":{"sequence":0}}
21:52:35.035 chunk 17 46565 bytes
21:52:35.036 in Ack 17
21:52:35.285 chunk 17 46610 bytes
21:52:35.286 in Ack 17
21:52:35.621 chunk 17 46640 bytes
21:52:35.629 in Ack 17
21:52:35.871 chunk 17 46670 bytes
21:52:35.872 in Ack 17
21:52:35.928 fetch POST /api/observability/v1/traces
21:52:35.928 fetched POST /api/observability/v1/traces 204
21:52:36.122 chunk 17 46730 bytes
21:52:36.123 in Ack 17
21:52:36.372 chunk 17 46760 bytes
21:52:36.373 in Ack 17
21:52:36.625 chunk 17 46805 bytes
21:52:36.627 in Ack 17
21:52:36.785 chunk 11 2884 bytes
21:52:36.786 in Ack 11
21:52:36.875 chunk 17 46850 bytes
21:52:36.876 in Ack 17
21:52:37.125 fetch POST /api/observability/v1/traces
21:52:37.125 fetched POST /api/observability/v1/traces 204
21:52:37.125 chunk 17 46880 bytes
21:52:37.126 in Ack 17
21:52:37.376 chunk 17 46910 bytes
21:52:37.377 in Ack 17
21:52:37.626 chunk 17 46940 bytes
21:52:37.627 in Ack 17
21:52:37.876 chunk 17 46985 bytes
21:52:37.877 in Ack 17
21:52:38.126 chunk 17 47000 bytes
21:52:38.127 in Ack 17
21:52:38.136 fetch POST /api/observability/v1/traces
21:52:38.136 fetched POST /api/observability/v1/traces 204
21:52:38.376 chunk 17 47030 bytes
21:52:38.377 in Ack 17
21:52:38.627 chunk 17 47060 bytes
21:52:38.629 in Ack 17
21:52:38.877 chunk 17 47105 bytes
21:52:38.878 in Ack 17
21:52:39.127 chunk 17 47135 bytes
21:52:39.128 in Ack 17
21:52:39.377 chunk 17 47165 bytes
21:52:39.378 in Ack 17
21:52:39.385 fetch POST /api/observability/v1/traces
21:52:39.385 fetched POST /api/observability/v1/traces 204
21:52:39.478 in Ping 
21:52:39.627 chunk 17 47195 bytes
21:52:39.628 in Ack 17
21:52:39.877 chunk 17 47225 bytes
21:52:39.878 in Ack 17
21:52:40.128 chunk 17 47255 bytes
21:52:40.129 in Ack 17
21:52:40.378 chunk 17 47315 bytes
21:52:40.379 in Ack 17
21:52:40.628 chunk 17 47375 bytes
21:52:40.629 in Ack 17
21:52:40.637 fetch POST /api/observability/v1/traces
21:52:40.637 fetched POST /api/observability/v1/traces 204
21:52:40.877 chunk 17 47435 bytes
21:52:40.878 in Ack 17
21:52:41.128 chunk 17 47495 bytes
21:52:41.129 in Ack 17
21:52:41.378 chunk 17 47510 bytes
21:52:41.379 in Ack 17
21:52:41.628 chunk 17 47555 bytes
21:52:41.629 in Ack 17
21:52:41.878 chunk 17 47585 bytes
21:52:41.879 in Ack 17
21:52:41.900 fetch POST /api/observability/v1/traces
21:52:41.900 fetched POST /api/observability/v1/traces 204
21:52:42.128 chunk 17 48650 bytes
21:52:42.129 in Ack 17
21:52:42.372 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:52:42.656 chunk 17 52155 bytes
21:52:42.657 in Ack 17
21:52:42.906 chunk 17 54238 bytes
21:52:42.907 in Ack 17
21:52:43.130 fetch POST /api/observability/v1/traces
21:52:43.130 fetched POST /api/observability/v1/traces 204
21:52:43.157 chunk 17 54288 bytes
21:52:43.158 in Ack 17
21:52:43.275 exit 33 {"_tag":"Success","value":{"sequence":97}}
21:52:43.407 chunk 17 56790 bytes
21:52:43.408 in Ack 17
21:52:43.657 chunk 17 56812 bytes
21:52:43.658 in Ack 17
21:52:43.942 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:52:43.949 chunk 17 56824 bytes
21:52:43.952 in Ack 17
21:52:44.159 fetch POST /api/observability/v1/traces
21:52:44.159 fetched POST /api/observability/v1/traces 204
21:52:44.204 chunk 17 56798 bytes
21:52:44.205 in Ack 17
21:52:44.454 chunk 17 56818 bytes
21:52:44.456 in Ack 17
21:52:44.485 in Ping 
21:52:44.703 chunk 17 58324 bytes
21:52:44.705 in Ack 17
21:52:44.771 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:52:45.059 chunk 17 58344 bytes
21:52:45.060 in Ack 17
21:52:45.207 fetch POST /api/observability/v1/traces
21:52:45.207 fetched POST /api/observability/v1/traces 204
21:52:45.311 chunk 17 58384 bytes
21:52:45.312 in Ack 17
21:52:45.563 chunk 11 2886 bytes
21:52:45.563 chunk 17 58404 bytes
21:52:45.563 in Ack 11
21:52:45.565 rpc 35 orchestration.dispatchCommand thread.visit
21:52:45.565 exit 35 {"_tag":"Success","value":{"sequence":0}}
21:52:45.566 in Ack 17
21:52:45.813 chunk 17 58424 bytes
21:52:45.814 in Ack 17
21:52:46.063 chunk 17 58502 bytes
21:52:46.065 in Ack 17
21:52:46.313 fetch POST /api/observability/v1/traces
21:52:46.313 fetched POST /api/observability/v1/traces 204
21:52:46.314 chunk 17 59546 bytes
21:52:46.316 in Ack 17
21:52:46.568 chunk 17 59566 bytes
21:52:46.569 in Ack 17
21:52:46.829 chunk 11 2886 bytes
21:52:46.829 chunk 17 59606 bytes
21:52:46.829 in Ack 11
21:52:46.835 in Ack 17
21:52:47.080 chunk 17 59626 bytes
21:52:47.081 in Ack 17
21:52:47.317 fetch POST /api/observability/v1/traces
21:52:47.317 fetched POST /api/observability/v1/traces 204
21:52:47.337 chunk 17 60682 bytes
21:52:47.348 in Ack 17
21:52:47.358 in Interrupt 27
21:52:47.382 rpc 36 subscribeVcsStatus
21:52:47.587 chunk 17 60712 bytes
21:52:47.588 in Ack 17
21:52:47.633 rpc 37 server.reportClientActivity
21:52:47.837 chunk 17 60732 bytes
21:52:47.838 in Ack 17
21:52:48.087 chunk 17 60762 bytes
21:52:48.088 in Ack 17
21:52:48.276 chunk 36 472 bytes
21:52:48.276 in Ack 36
21:52:48.338 chunk 17 60792 bytes
21:52:48.339 in Ack 17
21:52:48.349 fetch POST /api/observability/v1/traces
21:52:48.349 fetched POST /api/observability/v1/traces 204
21:52:48.589 chunk 17 60822 bytes
21:52:48.590 in Ack 17
21:52:48.839 chunk 17 60832 bytes
21:52:48.840 in Ack 17
21:52:49.088 chunk 17 60862 bytes
21:52:49.090 in Ack 17
21:52:49.339 chunk 17 60882 bytes
21:52:49.340 in Ack 17
21:52:49.485 in Ping 
21:52:49.589 chunk 17 60912 bytes
21:52:49.590 in Ack 17
21:52:49.622 fetch POST /api/observability/v1/traces
21:52:49.622 fetched POST /api/observability/v1/traces 204
21:52:49.839 chunk 17 60932 bytes
21:52:49.841 in Ack 17
21:52:50.090 chunk 17 60952 bytes
21:52:50.091 in Ack 17
21:52:50.340 chunk 17 60972 bytes
21:52:50.341 in Ack 17
21:52:50.572 exit 37 {"_tag":"Success","value":null}
21:52:50.591 chunk 17 60992 bytes
21:52:50.592 in Ack 17
21:52:50.841 chunk 17 61012 bytes
21:52:50.842 in Ack 17
21:52:50.850 fetch POST /api/observability/v1/traces
21:52:50.851 fetched POST /api/observability/v1/traces 204
21:52:51.091 chunk 17 61032 bytes
21:52:51.092 in Ack 17
21:52:51.341 chunk 11 2886 bytes
21:52:51.342 chunk 17 61048 bytes
21:52:51.342 in Ack 11
21:52:51.344 in Ack 17
21:52:51.592 chunk 17 62040 bytes
21:52:51.593 in Ack 17
21:52:51.842 chunk 17 62055 bytes
21:52:51.843 in Ack 17
21:52:52.092 chunk 17 62100 bytes
21:52:52.093 fetch POST /api/observability/v1/traces
21:52:52.093 fetched POST /api/observability/v1/traces 204
21:52:52.095 in Ack 17
21:52:52.345 chunk 17 62130 bytes
21:52:52.347 in Ack 17
21:52:52.598 chunk 17 62190 bytes
21:52:52.599 in Ack 17
21:52:52.849 chunk 17 62256 bytes
21:52:52.850 in Ack 17
21:52:53.096 fetch POST /api/observability/v1/traces
21:52:53.096 fetched POST /api/observability/v1/traces 204
21:52:53.099 chunk 17 62307 bytes
21:52:53.100 in Ack 17
21:52:53.349 chunk 17 62367 bytes
21:52:53.350 in Ack 17
21:52:53.600 chunk 17 62418 bytes
21:52:53.601 in Ack 17
21:52:53.851 chunk 17 62478 bytes
21:52:53.852 in Ack 17
21:52:54.101 chunk 17 62508 bytes
21:52:54.101 fetch POST /api/observability/v1/traces
21:52:54.101 fetched POST /api/observability/v1/traces 204
21:52:54.103 in Ack 17
21:52:54.351 chunk 17 62538 bytes
21:52:54.353 in Ack 17
21:52:54.486 in Ping 
21:52:54.602 chunk 17 62592 bytes
21:52:54.603 in Ack 17
21:52:54.702 rpc 38 server.reportClientActivity
21:52:54.853 chunk 17 62622 bytes
21:52:54.854 in Ack 17
21:52:55.107 chunk 17 62676 bytes
21:52:55.108 fetch POST /api/observability/v1/traces
21:52:55.108 fetched POST /api/observability/v1/traces 204
21:52:55.109 in Ack 17
21:52:55.357 chunk 17 62724 bytes
21:52:55.359 in Ack 17
21:52:55.565 rpc 39 orchestration.dispatchCommand thread.visit
21:52:55.565 exit 39 {"_tag":"Success","value":{"sequence":0}}
21:52:55.607 chunk 17 62748 bytes
21:52:55.608 in Ack 17
21:52:55.857 chunk 17 62793 bytes
21:52:55.858 in Ack 17
21:52:56.109 chunk 17 62844 bytes
21:52:56.110 in Ack 17
21:52:56.200 fetch POST /api/observability/v1/traces
21:52:56.200 fetched POST /api/observability/v1/traces 204
21:52:56.358 chunk 17 62874 bytes
21:52:56.360 in Ack 17
21:52:56.607 chunk 11 2886 bytes
21:52:56.608 in Ack 11
21:52:56.613 chunk 17 62895 bytes
21:52:56.614 in Ack 17
21:52:56.863 chunk 17 62961 bytes
21:52:56.864 in Ack 17
21:52:57.113 chunk 17 62991 bytes
21:52:57.114 in Ack 17
21:52:57.361 fetch POST /api/observability/v1/traces
21:52:57.361 fetched POST /api/observability/v1/traces 204
21:52:57.363 chunk 17 63074 bytes
21:52:57.364 in Ack 17
21:52:57.708 chunk 11 2822 bytes
21:52:57.710 in Ack 11
21:52:57.711 rpc 40 orchestration.dispatchCommand thread.visit
21:52:57.711 exit 40 {"_tag":"Success","value":{"sequence":0}}
