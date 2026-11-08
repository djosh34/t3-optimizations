# case70, webapp, warm start, seed 2

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 69 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.4s |
| Prompt reached the agent | 0.9s |
| Time to first text | 22.2s |
| Whole reply shown | 68.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 21.0s / 7.8s / 17.0s |
| Longest silence | 22.2s |
| Approval shown after it was asked | 14.0s |
| Bytes on the wire down / up | 59647 / 37696 |
| Bytes the app received / sent | 42465 / 82314 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 9.0 | 0 | 361 |
| 20 | 19.0 | 0 | 762 |
| 30 | 11.1 | 719 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 5.8 | 1110 | 1336 |
| 60 | 15.8 | 1110 | 1596 |
| 68 | 11.5 | 1564 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173814942,"silences":0,"longestSilenceMs":1010,"longestRecoveredGapMs":1010},"stats":{"sendRate":4500,"deliveredRate":2808.75,"loss":0,"srttMs":4.702319243070594,"minRttMs":1,"sentPackets":93,"sentWireBytes":29618,"receivedPackets":119,"receivedWireBytes":63657,"lastHeardMs":8155,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 731/1078 22/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 40/70","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173823586,"silences":0,"longestSilenceMs":1593,"longestRecoveredGapMs":1593},"stats":{"sendRate":2000,"deliveredRate":2000,"loss":0,"srttMs":600.6145293376868,"minRttMs":4772,"sentPackets":112,"sentWireBytes":35593,"receivedPackets":131,"receivedWireBytes":72864,"lastHeardMs":16799,"unsyncedBytes":0,"inflightPackets":14,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 731/1078 22/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 415/893","user":"10/10 9/9","attempts":"661/661 661/661","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173835140,"silences":1,"longestSilenceMs":11138,"longestRecoveredGapMs":11138},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":705.037713170476,"minRttMs":1436,"sentPackets":146,"sentWireBytes":44890,"receivedPackets":133,"receivedWireBytes":73428,"lastHeardMs":28353,"unsyncedBytes":292,"inflightPackets":43,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 1078/1078 716/849","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1303/1338","user":"10/10 9/9","attempts":"661/661 661/661 124/637","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173845967,"silences":1,"longestSilenceMs":11138,"longestRecoveredGapMs":11138},"stats":{"sendRate":6750,"deliveredRate":0,"loss":0,"srttMs":1021.5048586929098,"minRttMs":1046,"sentPackets":203,"sentWireBytes":57055,"receivedPackets":160,"receivedWireBytes":81361,"lastHeardMs":39180,"unsyncedBytes":0,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 1078/1078 814/849 365/1550","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"661/661 661/661 124/637","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"983/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173855878,"silences":1,"longestSilenceMs":11138,"longestRecoveredGapMs":11138},"stats":{"sendRate":6750,"deliveredRate":162.10045662100458,"loss":0.6428571428571428,"srttMs":1021.5048586929098,"minRttMs":1046,"sentPackets":240,"sentWireBytes":62328,"receivedPackets":182,"receivedWireBytes":92382,"lastHeardMs":49090,"unsyncedBytes":0,"inflightPackets":66,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 1078/1078 849/849 1090/1550","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1340/1340 244/313","user":"10/10 9/9","attempts":"661/661 661/661 307/637","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1008/1008"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173865409,"silences":1,"longestSilenceMs":11138,"longestRecoveredGapMs":11138},"stats":{"sendRate":3408.5153584961563,"deliveredRate":362.9580457161819,"loss":0.8402714932126697,"srttMs":1900.80487548986,"minRttMs":1046,"sentPackets":282,"sentWireBytes":79744,"receivedPackets":203,"receivedWireBytes":99936,"lastHeardMs":58622,"unsyncedBytes":0,"inflightPackets":89,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1031/1031","turnItems":"1103/1103 860/860 1058/1058 870/870 1299/1299 1103/1103 1058/1058 1078/1078 849/849 1090/1550 1103/1103","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 1340/1340 674/723","user":"10/10 9/9","attempts":"661/661 661/661 371/637","messages":"456/456 548/548 901/901 548/548","providerTurns":"776/776 776/776","providerThreads":"809/809","providerSessions":"2685/2685","nodes":"1022/1022 809/809 809/809 1022/1022","checkpoints":"613/613 613/613 779/779","checkpointScopes":"723/723","runtimeRequests":"1038/1051"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":245,"lastDataAt":1794173878065,"silences":4,"longestSilenceMs":11138,"longestRecoveredGapMs":11138},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.7596186166774401,"srttMs":1951.8506077969241,"minRttMs":1046,"sentPackets":327,"sentWireBytes":91920,"receivedPackets":213,"receivedWireBytes":104582,"lastHeardMs":71278,"unsyncedBytes":400,"inflightPackets":92,"duplicateBytes":0}}
- shim trace
21:36:47.107 fetch GET /api/auth/session
21:36:47.108 fetched GET /api/auth/session 200
21:36:47.231 fetch GET /.well-known/t3/environment
21:36:47.232 fetched GET /.well-known/t3/environment 200
21:36:47.240 fetch GET /.well-known/t3/environment
21:36:47.241 fetched GET /.well-known/t3/environment 200
21:36:47.244 rpc 0 subscribeServerConfig
21:36:47.244 chunk 0 38678 bytes
21:36:47.272 in Ack 0
21:36:47.273 rpc 1 subscribeServerLifecycle
21:36:47.274 rpc 2 subscribeServerLifecycle
21:36:47.276 fetch GET /api/auth/session
21:36:47.277 fetched GET /api/auth/session 200
21:36:47.278 rpc 3 subscribeDeviceState
21:36:47.279 rpc 4 preview.reportProfiles
21:36:47.321 rpc 5 subscribeProjectClones
21:36:47.339 fetch GET /api/orchestration/threads/mcp%3A25827777-ea18-4895-91a4-68f18f90a59b/bounded
21:36:47.344 fetch GET /api/orchestration/shell
21:36:47.363 rpc 6 server.refreshProviders
21:36:47.401 rpc 7 orchestration.dispatchCommand thread.visit
21:36:47.401 exit 7 {"_tag":"Success","value":{"sequence":0}}
21:36:47.411 fetched GET /api/orchestration/shell 200
21:36:47.412 rpc 8 subscribeTerminalMetadata
21:36:47.412 rpc 9 subscribeVcsStatus
21:36:47.413 rpc 10 subscribePreviewEvents
21:36:47.415 rpc 11 orchestration.subscribeShell
21:36:47.415 chunk 11 2779 bytes
21:36:47.416 rpc 12 assets.createUrl
21:36:47.417 rpc 13 preview.list
21:36:47.417 rpc 14 projects.readFile
21:36:47.421 in Ack 11
21:36:47.581 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"c28294aa-e7c3-47d0-8ed2-f601e45954b5","sessions":[]}}
21:36:47.581 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
21:36:47.665 rpc 15 server.reportClientActivity
21:36:47.665 chunk 11 2755 bytes
21:36:47.666 in Ack 11
21:36:47.699 chunk 1 3861 bytes
21:36:47.700 in Ack 1
21:36:48.070 chunk 2 3861 bytes
21:36:48.070 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T21:36:35.084Z","configurableProviders":false,"con
21:36:48.070 chunk 9 472 bytes
21:36:48.070 in Ack 2
21:36:48.073 in Ack 9
21:36:48.200 exit 12 {"_tag":"Success","value":{"expiresAt":1794177000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
21:36:48.230 fetch POST /api/observability/v1/traces
21:36:48.230 fetched POST /api/observability/v1/traces 204
21:36:48.399 chunk 3 876 bytes
21:36:48.399 exit 4 {"_tag":"Success","value":null}
21:36:48.399 chunk 5 4 bytes
21:36:48.399 chunk 8 36 bytes
21:36:48.399 exit 15 {"_tag":"Success","value":null}
21:36:48.399 in Ack 3
21:36:48.403 rpc 16 preview.reportProfiles
21:36:48.403 in Ack 5
21:36:48.403 in Ack 8
21:36:48.450 chunk 11 2798 bytes
21:36:48.451 in Ack 11
21:36:48.458 exit 16 {"_tag":"Success","value":null}
21:36:48.458 fetched GET /api/orchestration/threads/mcp%3A25827777-ea18-4895-91a4-68f18f90a59b/bounded 200
21:36:48.461 rpc 17 orchestration.subscribeThread
21:36:48.461 chunk 17 8294 bytes
21:36:48.470 in Interrupt 9
21:36:48.472 rpc 18 subscribeVcsStatus
21:36:48.473 in Ack 17
21:36:48.588 chunk 18 472 bytes
21:36:48.588 in Ack 18
21:36:48.712 chunk 17 11932 bytes
21:36:48.713 in Ack 17
21:36:48.766 rpc 19 server.reportClientActivity
21:36:48.965 chunk 17 13517 bytes
21:36:48.966 in Ack 17
21:36:49.199 rpc 20 attachments.createUploadUrl
21:36:49.391 exit 19 {"_tag":"Success","value":null}
21:36:49.397 chunk 17 19860 bytes
21:36:49.406 in Ack 17
21:36:49.414 in Interrupt 18
21:36:49.416 rpc 21 subscribeVcsStatus
21:36:49.417 fetch POST /api/observability/v1/traces
21:36:49.417 fetched POST /api/observability/v1/traces 204
21:36:49.448 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-9d0cd318-4ec5-4290-a12d-54790a54a644","expiresAt":1794174409419,"relativeUrl":"/api/attachments/upload/eyJ2ZX
21:36:49.449 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTlkMGNkMzE4LTRlYzUtNDI5MC1hMTJkLTU0NzkwYTU0YTY0NCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE3NDQwOTQxOX0.SuR_7P6k-BgHdXigRHyzlZEIymbBbkV-wa3Z_UTnUF0
21:36:49.518 chunk 21 472 bytes
21:36:49.518 in Ack 21
21:36:49.648 chunk 17 21197 bytes
21:36:49.649 in Ack 17
21:36:49.666 rpc 22 server.reportClientActivity
21:36:50.449 fetch POST /api/observability/v1/traces
21:36:50.449 fetched POST /api/observability/v1/traces 204
21:36:50.611 exit 22 {"_tag":"Success","value":null}
21:36:51.612 fetch POST /api/observability/v1/traces
21:36:51.612 fetched POST /api/observability/v1/traces 204
21:36:52.096 rpc 23 orchestration.dispatchCommand message.dispatch
21:36:52.644 in Ping 
21:36:52.709 exit 23 {"_tag":"Success","value":{"sequence":45}}
21:36:52.710 rpc 24 attachments.delete
21:36:52.958 chunk 11 2884 bytes
21:36:52.958 chunk 17 21835 bytes
21:36:52.959 in Ack 11
21:36:52.984 in Ack 17
21:36:53.100 fetch POST /api/observability/v1/traces
21:36:53.100 fetched POST /api/observability/v1/traces 204
21:36:53.209 exit 24 {"_tag":"Success","value":null}
21:36:53.209 chunk 17 29330 bytes
21:36:53.211 in Ack 17
21:36:53.213 rpc 25 assets.createUrl
21:36:53.459 chunk 17 29394 bytes
21:36:53.460 in Ack 17
21:36:53.709 chunk 11 2820 bytes
21:36:53.709 chunk 17 30437 bytes
21:36:53.710 in Ack 11
21:36:53.711 rpc 26 orchestration.dispatchCommand thread.visit
21:36:53.711 exit 26 {"_tag":"Success","value":{"sequence":0}}
21:36:53.715 in Ack 17
21:36:53.959 chunk 11 2820 bytes
21:36:53.960 chunk 17 32884 bytes
21:36:53.960 in Ack 11
21:36:53.964 in Ack 17
21:36:54.210 chunk 17 33522 bytes
21:36:54.210 fetch POST /api/observability/v1/traces
21:36:54.211 fetched POST /api/observability/v1/traces 204
21:36:54.212 in Ack 17
21:36:54.221 in Interrupt 21
21:36:54.223 rpc 27 subscribeVcsStatus
21:36:54.328 chunk 27 472 bytes
21:36:54.329 in Ack 27
21:36:54.461 chunk 17 35231 bytes
21:36:54.462 in Ack 17
21:36:54.475 rpc 28 server.reportClientActivity
21:36:54.549 exit 28 {"_tag":"Success","value":null}
21:36:54.943 chunk 17 35231 bytes
21:36:54.944 in Ack 17
21:36:55.214 fetch POST /api/observability/v1/traces
21:36:55.215 fetched POST /api/observability/v1/traces 204
21:36:55.612 rpc 29 orchestration.dispatchCommand message.dispatch
21:36:56.613 fetch POST /api/observability/v1/traces
21:36:56.613 fetched POST /api/observability/v1/traces 204
21:36:57.644 in Ping 
21:36:58.920 fetch POST /api/observability/v1/traces
21:36:58.920 fetched POST /api/observability/v1/traces 204
21:36:59.978 fetch POST /api/observability/v1/traces
21:36:59.978 fetched POST /api/observability/v1/traces 204
21:37:00.801 exit 29 {"_tag":"Success","value":{"sequence":75}}
21:37:01.436 chunk 17 37688 bytes
21:37:01.437 in Ack 17
21:37:01.803 fetch POST /api/observability/v1/traces
21:37:01.803 fetched POST /api/observability/v1/traces 204
21:37:01.978 chunk 17 37700 bytes
21:37:01.980 in Ack 17
21:37:02.525 chunk 17 37720 bytes
21:37:02.526 in Ack 17
21:37:02.644 in Ping 
21:37:02.981 fetch POST /api/observability/v1/traces
21:37:02.981 fetched POST /api/observability/v1/traces 204
21:37:03.072 chunk 17 37740 bytes
21:37:03.075 in Ack 17
21:37:03.587 chunk 17 37752 bytes
21:37:03.589 in Ack 17
21:37:04.076 fetch POST /api/observability/v1/traces
21:37:04.076 fetched POST /api/observability/v1/traces 204
21:37:07.645 in Ping 
21:37:09.982 fetch POST /api/observability/v1/traces
21:37:09.982 fetched POST /api/observability/v1/traces 204
21:37:12.484 rpc 30 server.reportClientActivity
21:37:12.645 in Ping 
21:37:13.484 fetch POST /api/observability/v1/traces
21:37:13.484 fetched POST /api/observability/v1/traces 204
21:37:14.726 chunk 17 38132 bytes
21:37:14.727 in Ack 17
21:37:15.142 chunk 17 38514 bytes
21:37:15.144 in Ack 17
21:37:15.729 fetch POST /api/observability/v1/traces
21:37:15.729 fetched POST /api/observability/v1/traces 204
21:37:16.234 chunk 17 38580 bytes
21:37:16.235 in Ack 17
21:37:16.494 chunk 17 38628 bytes
21:37:16.497 in Ack 17
21:37:16.830 chunk 17 38930 bytes
21:37:16.831 in Ack 17
21:37:17.200 chunk 17 39106 bytes
21:37:17.202 in Ack 17
21:37:17.240 fetch POST /api/observability/v1/traces
21:37:17.240 fetched POST /api/observability/v1/traces 204
21:37:17.532 chunk 17 39134 bytes
21:37:17.533 in Ack 17
21:37:17.645 in Ping 
21:37:18.534 fetch POST /api/observability/v1/traces
21:37:18.534 fetched POST /api/observability/v1/traces 204
21:37:18.613 chunk 17 39462 bytes
21:37:18.615 in Ack 17
21:37:19.073 chunk 17 39608 bytes
21:37:19.075 in Ack 17
21:37:19.582 chunk 17 39630 bytes
21:37:19.583 in Ack 17
21:37:19.619 fetch POST /api/observability/v1/traces
21:37:19.620 fetched POST /api/observability/v1/traces 204
21:37:20.743 chunk 17 39776 bytes
21:37:20.744 in Ack 17
21:37:20.998 chunk 17 39844 bytes
21:37:20.999 in Ack 17
21:37:21.549 chunk 17 39896 bytes
21:37:21.551 in Ack 17
21:37:21.745 fetch POST /api/observability/v1/traces
21:37:21.745 fetched POST /api/observability/v1/traces 204
21:37:21.863 chunk 17 39916 bytes
21:37:21.865 in Ack 17
21:37:22.291 chunk 17 40022 bytes
21:37:22.293 in Ack 17
21:37:22.547 chunk 17 40098 bytes
21:37:22.548 in Ack 17
21:37:22.645 in Ping 
21:37:22.806 chunk 17 40060 bytes
21:37:22.808 in Ack 17
21:37:22.867 fetch POST /api/observability/v1/traces
21:37:22.867 fetched POST /api/observability/v1/traces 204
21:37:23.060 chunk 17 40082 bytes
21:37:23.061 in Ack 17
21:37:23.562 chunk 17 40092 bytes
21:37:23.563 in Ack 17
21:37:24.063 fetch POST /api/observability/v1/traces
21:37:24.064 fetched POST /api/observability/v1/traces 204
21:37:24.068 chunk 17 40114 bytes
21:37:24.069 in Ack 17
21:37:24.582 chunk 17 40124 bytes
21:37:24.583 in Ack 17
21:37:24.840 chunk 17 40146 bytes
21:37:24.841 in Ack 17
21:37:25.077 fetch POST /api/observability/v1/traces
21:37:25.077 fetched POST /api/observability/v1/traces 204
21:37:25.092 chunk 17 40166 bytes
21:37:25.096 in Ack 17
21:37:25.426 chunk 17 40176 bytes
21:37:25.428 in Ack 17
21:37:25.682 chunk 17 40226 bytes
21:37:25.683 in Ack 17
21:37:25.968 chunk 17 40236 bytes
21:37:25.970 in Ack 17
21:37:26.097 fetch POST /api/observability/v1/traces
21:37:26.097 fetched POST /api/observability/v1/traces 204
21:37:27.487 in Interrupt 30
21:37:27.646 in Ping 
21:37:28.046 chunk 17 40246 bytes
21:37:28.047 in Ack 17
21:37:28.488 fetch POST /api/observability/v1/traces
21:37:28.488 fetched POST /api/observability/v1/traces 204
21:37:29.988 fetch POST /api/observability/v1/traces
21:37:29.988 fetched POST /api/observability/v1/traces 204
21:37:30.310 chunk 17 40274 bytes
21:37:30.311 in Ack 17
21:37:31.312 fetch POST /api/observability/v1/traces
21:37:31.312 fetched POST /api/observability/v1/traces 204
21:37:31.498 chunk 17 40306 bytes
21:37:31.499 in Ack 17
21:37:32.500 fetch POST /api/observability/v1/traces
21:37:32.500 fetched POST /api/observability/v1/traces 204
21:37:32.646 in Ping 
21:37:37.303 chunk 17 41314 bytes
21:37:37.304 in Ack 17
21:37:37.454 rpc 31 orchestration.dispatchCommand runtime-request.respond
21:37:37.505 rpc 32 server.reportClientActivity
21:37:37.646 in Ping 
21:37:38.305 fetch POST /api/observability/v1/traces
21:37:38.305 fetched POST /api/observability/v1/traces 204
21:37:39.992 fetch POST /api/observability/v1/traces
21:37:39.992 fetched POST /api/observability/v1/traces 204
21:37:40.758 chunk 17 43875 bytes
21:37:40.758 exit 31 {"_tag":"Success","value":{"sequence":95}}
21:37:40.760 in Ack 17
21:37:41.184 rpc 33 orchestration.dispatchCommand runtime-request.respond
21:37:41.762 fetch POST /api/observability/v1/traces
21:37:41.762 fetched POST /api/observability/v1/traces 204
21:37:42.618 chunk 17 43917 bytes
21:37:42.619 in Ack 17
21:37:42.648 in Ping 
21:37:43.052 chunk 17 43937 bytes
21:37:43.053 in Ack 17
21:37:43.539 chunk 17 43957 bytes
21:37:43.540 in Ack 17
21:37:43.622 fetch POST /api/observability/v1/traces
21:37:43.622 fetched POST /api/observability/v1/traces 204
21:37:44.109 chunk 17 43989 bytes
21:37:44.110 in Ack 17
21:37:44.547 chunk 17 44049 bytes
21:37:44.548 in Ack 17
21:37:44.984 chunk 17 44079 bytes
21:37:44.985 in Ack 17
21:37:45.111 fetch POST /api/observability/v1/traces
21:37:45.111 fetched POST /api/observability/v1/traces 204
21:37:45.410 chunk 17 44241 bytes
21:37:45.412 in Ack 17
21:37:46.413 fetch POST /api/observability/v1/traces
21:37:46.413 fetched POST /api/observability/v1/traces 204
21:37:47.652 in Ping 
21:37:49.782 chunk 17 44549 bytes
21:37:49.783 in Ack 17
21:37:49.995 fetch POST /api/observability/v1/traces
21:37:49.995 fetched POST /api/observability/v1/traces 204
21:37:50.274 chunk 17 44755 bytes
21:37:50.275 in Ack 17
21:37:51.276 fetch POST /api/observability/v1/traces
21:37:51.276 fetched POST /api/observability/v1/traces 204
21:37:51.300 chunk 17 44871 bytes
21:37:51.301 in Ack 17
21:37:52.302 fetch POST /api/observability/v1/traces
21:37:52.302 fetched POST /api/observability/v1/traces 204
21:37:52.506 in Interrupt 32
21:37:52.656 in Ping 
21:37:53.421 chunk 17 44927 bytes
21:37:53.421 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:37:53.422 in Ack 17
21:37:53.523 fetch POST /api/observability/v1/traces
21:37:53.524 fetched POST /api/observability/v1/traces 204
21:37:54.569 rpc 34 orchestration.dispatchCommand runtime-request.respond
21:37:54.626 chunk 17 45023 bytes
21:37:54.628 in Ack 17
21:37:54.884 chunk 17 45029 bytes
21:37:54.886 in Ack 17
21:37:55.475 chunk 17 45141 bytes
21:37:55.476 in Ack 17
21:37:55.570 fetch POST /api/observability/v1/traces
21:37:55.570 fetched POST /api/observability/v1/traces 204
21:37:57.660 in Ping 
21:37:58.067 chunk 17 45143 bytes
21:37:58.067 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
21:37:58.067 exit 30 {"_tag":"Success","value":null}
21:37:58.068 in Ack 17
21:37:58.237 rpc 35 orchestration.dispatchCommand runtime-request.respond
21:37:59.070 fetch POST /api/observability/v1/traces
21:37:59.070 fetched POST /api/observability/v1/traces 204
21:38:02.484 rpc 36 server.reportClientActivity
21:38:02.664 in Ping 
21:38:03.191 chunk 17 45294 bytes
21:38:03.193 in Ack 17
21:38:03.485 fetch POST /api/observability/v1/traces
21:38:03.485 fetched POST /api/observability/v1/traces 204
