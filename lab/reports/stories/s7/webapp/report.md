# story, webapp, warm start, seed 7

Profile story (graded): seed 7's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 56 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 5.4s |
| Prompt reached the agent | 5.0s |
| Time to first text | 17.9s |
| Whole reply shown | 54.1s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 18.9s / 6.3s / 15.9s |
| Longest silence | 17.9s |
| Approval shown after it was asked | 3.3s |
| Bytes on the wire down / up | 298127 / 124250 |
| Bytes the app received / sent | 181977 / 60999 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.9 | 0 | 197 |
| 20 | 4.9 | 397 | 597 |
| 30 | 14.9 | 397 | 1000 |
| 40 | 2.0 | 1077 | 1158 |
| 50 | 5.9 | 1324 | 1528 |
| 54 | 8.3 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216520096,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":6750,"deliveredRate":2994.5636340561005,"loss":0,"srttMs":5.434568429917893,"minRttMs":1,"sentPackets":110,"sentWireBytes":31727,"receivedPackets":195,"receivedWireBytes":113043,"lastHeardMs":12153,"unsyncedBytes":518,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216530189,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":3375,"deliveredRate":1625.8082474211906,"loss":0.0692,"srttMs":2032.3677463876868,"minRttMs":1,"sentPackets":148,"sentWireBytes":41175,"receivedPackets":278,"receivedWireBytes":139473,"lastHeardMs":22246,"unsyncedBytes":0,"inflightPackets":4,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"865/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 545/654","providerThreads":"643/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 319/785","messages":"544/544 456/456 901/901 544/544 347/451 372/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"379/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216540226,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":1757.767428877004,"deliveredRate":891.1034880082352,"loss":0.493048128342246,"srttMs":2287.8614057584127,"minRttMs":2209,"sentPackets":207,"sentWireBytes":55977,"receivedPackets":369,"receivedWireBytes":179228,"lastHeardMs":32283,"unsyncedBytes":104,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216546632,"silences":0,"longestSilenceMs":1094,"longestRecoveredGapMs":1094},"stats":{"sendRate":2636.651143315506,"deliveredRate":0,"loss":0.335514037433155,"srttMs":2204.7648003788745,"minRttMs":1925,"sentPackets":261,"sentWireBytes":68473,"receivedPackets":429,"receivedWireBytes":203723,"lastHeardMs":38689,"unsyncedBytes":0,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 923/923","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 160/1368","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216560355,"silences":1,"longestSilenceMs":5215,"longestRecoveredGapMs":5215},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.5060606060606061,"srttMs":2187.794200331515,"minRttMs":1925,"sentPackets":288,"sentWireBytes":74827,"receivedPackets":502,"receivedWireBytes":240749,"lastHeardMs":52413,"unsyncedBytes":854,"inflightPackets":47,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 382/1566 923/923 1068/1068","frame":"1029/1029","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2652/2652","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 160/1368","messages":"544/544 456/456 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"690/690","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":574,"lastDataAt":1794216571551,"silences":1,"longestSilenceMs":5215,"longestRecoveredGapMs":5215},"stats":{"sendRate":1000,"deliveredRate":613.9599472073355,"loss":0.13375,"srttMs":4127.382720352473,"minRttMs":1925,"sentPackets":325,"sentWireBytes":87182,"receivedPackets":582,"receivedWireBytes":281353,"lastHeardMs":63608,"unsyncedBytes":0,"inflightPackets":54,"duplicateBytes":0}}
- shim trace
09:28:28.806 fetch GET /api/auth/session
09:28:28.808 fetched GET /api/auth/session 200
09:28:29.287 fetch GET /.well-known/t3/environment
09:28:29.289 fetched GET /.well-known/t3/environment 200
09:28:29.306 fetch GET /.well-known/t3/environment
09:28:29.307 fetched GET /.well-known/t3/environment 200
09:28:29.312 rpc 0 subscribeServerConfig
09:28:29.313 chunk 0 38458 bytes
09:28:29.365 in Ack 0
09:28:29.368 rpc 1 subscribeServerLifecycle
09:28:29.369 rpc 2 subscribeServerLifecycle
09:28:29.373 fetch GET /api/auth/session
09:28:29.375 fetched GET /api/auth/session 200
09:28:29.376 rpc 3 subscribeDeviceState
09:28:29.378 rpc 4 preview.reportProfiles
09:28:29.431 rpc 5 subscribeProjectClones
09:28:29.437 fetch GET /api/orchestration/threads/mcp%3Ac1be928a-e08c-4f61-a4a4-e54b08f47274/bounded
09:28:29.462 fetch GET /api/orchestration/shell
09:28:29.463 fetched GET /api/orchestration/shell 200
09:28:29.465 rpc 6 orchestration.subscribeShell
09:28:29.466 chunk 6 4593 bytes
09:28:29.494 rpc 7 server.refreshProviders
09:28:29.556 rpc 8 orchestration.dispatchCommand thread.visit
09:28:29.556 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:28:29.580 rpc 9 subscribeTerminalMetadata
09:28:29.581 rpc 10 subscribeVcsStatus
09:28:29.591 rpc 11 subscribePreviewEvents
09:28:29.592 rpc 12 assets.createUrl
09:28:29.593 rpc 13 preview.list
09:28:29.594 rpc 14 projects.readFile
09:28:29.599 in Ack 6
09:28:29.680 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"8d773389-7ebb-42fe-a2ac-db79a46ca77d","sessions":[]}}
09:28:29.718 chunk 6 4569 bytes
09:28:29.720 in Ack 6
09:28:29.797 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:28:29.799 chunk 1 3814 bytes
09:28:29.819 in Ack 1
09:28:29.843 rpc 15 server.reportClientActivity
09:28:30.018 chunk 2 3814 bytes
09:28:30.044 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:28:06.201Z","configurableProviders":false,"con
09:28:30.044 chunk 10 1632 bytes
09:28:30.044 in Ack 2
09:28:30.051 in Ack 10
09:28:30.151 exit 12 {"_tag":"Success","value":{"expiresAt":1794218400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:28:30.282 fetch POST /api/observability/v1/traces
09:28:30.283 fetched POST /api/observability/v1/traces 204
09:28:30.401 chunk 6 4637 bytes
09:28:30.402 in Ack 6
09:28:30.651 chunk 6 4634 bytes
09:28:30.652 in Ack 6
09:28:30.690 chunk 3 876 bytes
09:28:30.690 exit 4 {"_tag":"Success","value":null}
09:28:30.690 chunk 5 4 bytes
09:28:30.690 chunk 9 36 bytes
09:28:30.690 exit 15 {"_tag":"Success","value":null}
09:28:30.701 in Ack 3
09:28:30.713 in Ack 5
09:28:30.714 in Ack 9
09:28:30.902 chunk 6 4634 bytes
09:28:30.903 in Ack 6
09:28:31.403 fetch POST /api/observability/v1/traces
09:28:31.404 fetched POST /api/observability/v1/traces 204
09:28:31.740 fetched GET /api/orchestration/threads/mcp%3Ac1be928a-e08c-4f61-a4a4-e54b08f47274/bounded 200
09:28:31.745 rpc 16 orchestration.subscribeThread
09:28:31.745 chunk 16 8282 bytes
09:28:31.758 in Interrupt 10
09:28:31.760 rpc 17 subscribeVcsStatus
09:28:31.761 in Ack 16
09:28:31.873 chunk 17 1632 bytes
09:28:31.873 in Ack 17
09:28:31.998 chunk 16 11259 bytes
09:28:31.999 in Ack 16
09:28:32.080 rpc 18 server.reportClientActivity
09:28:32.258 exit 18 {"_tag":"Success","value":null}
09:28:32.262 chunk 16 13497 bytes
09:28:32.267 in Ack 16
09:28:32.512 chunk 16 18551 bytes
09:28:32.514 in Ack 16
09:28:32.600 rpc 19 attachments.createUploadUrl
09:28:32.712 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-3b5a3bae-16b0-48ef-98a6-2d73f11c22f1","expiresAt":1794217112678,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:28:32.713 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTNiNWEzYmFlLTE2YjAtNDhlZi05OGE2LTJkNzNmMTFjMjJmMSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzExMjY3OH0.o7P6q0Jx1WMAOmIqM1Q6HmicA1LWqF76YsXNZiSy1zI
09:28:32.741 fetch POST /api/observability/v1/traces
09:28:32.741 fetched POST /api/observability/v1/traces 204
09:28:32.762 chunk 16 21097 bytes
09:28:32.764 in Ack 16
09:28:32.775 in Interrupt 17
09:28:32.777 rpc 20 subscribeVcsStatus
09:28:32.842 chunk 20 1632 bytes
09:28:32.842 in Ack 20
09:28:33.029 rpc 21 server.reportClientActivity
09:28:33.765 fetch POST /api/observability/v1/traces
09:28:33.765 fetched POST /api/observability/v1/traces 204
09:28:34.309 in Ping 
09:28:34.654 exit 21 {"_tag":"Success","value":null}
09:28:35.667 rpc 22 orchestration.dispatchCommand message.dispatch
09:28:35.920 fetch POST /api/observability/v1/traces
09:28:35.920 fetched POST /api/observability/v1/traces 204
09:28:35.954 exit 22 {"_tag":"Success","value":{"sequence":110}}
09:28:35.956 rpc 23 attachments.delete
09:28:36.217 chunk 6 4720 bytes
09:28:36.217 chunk 16 26021 bytes
09:28:36.218 in Ack 6
09:28:36.264 in Ack 16
09:28:36.267 rpc 24 assets.createUrl
09:28:36.330 exit 23 {"_tag":"Success","value":null}
09:28:36.468 chunk 16 27720 bytes
09:28:36.470 in Ack 16
09:28:36.718 chunk 6 4659 bytes
09:28:36.719 chunk 16 30201 bytes
09:28:36.720 in Ack 6
09:28:36.729 in Ack 16
09:28:36.955 fetch POST /api/observability/v1/traces
09:28:36.955 fetched POST /api/observability/v1/traces 204
09:28:36.969 chunk 16 31900 bytes
09:28:36.971 in Ack 16
09:28:37.108 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/5b368621-6f26-4e54-8e42-1db233b74058","expiresAt":1794220117108}}
09:28:37.473 chunk 16 34479 bytes
09:28:37.474 in Ack 16
09:28:37.723 chunk 6 4656 bytes
09:28:37.723 chunk 16 35257 bytes
09:28:37.724 in Ack 6
09:28:37.725 rpc 25 orchestration.dispatchCommand thread.visit
09:28:37.725 exit 25 {"_tag":"Success","value":{"sequence":0}}
09:28:37.734 in Ack 16
09:28:37.743 in Interrupt 20
09:28:37.748 rpc 26 subscribeVcsStatus
09:28:37.783 chunk 26 1632 bytes
09:28:37.783 in Ack 26
09:28:38.283 fetch POST /api/observability/v1/traces
09:28:38.283 fetched POST /api/observability/v1/traces 204
09:28:38.283 chunk 6 4656 bytes
09:28:38.284 chunk 16 35257 bytes
09:28:38.285 rpc 27 server.reportClientActivity
09:28:38.286 in Ack 6
09:28:38.293 in Ack 16
09:28:38.320 exit 27 {"_tag":"Success","value":null}
09:28:39.289 fetch POST /api/observability/v1/traces
09:28:39.289 fetched POST /api/observability/v1/traces 204
09:28:39.319 in Ping 
09:28:40.037 rpc 28 orchestration.dispatchCommand message.dispatch
09:28:41.037 fetch POST /api/observability/v1/traces
09:28:41.037 fetched POST /api/observability/v1/traces 204
09:28:43.300 fetch POST /api/observability/v1/traces
09:28:43.300 fetched POST /api/observability/v1/traces 204
09:28:44.320 in Ping 
09:28:44.576 chunk 6 4720 bytes
09:28:44.577 in Ack 6
09:28:45.168 exit 28 {"_tag":"Success","value":{"sequence":152}}
09:28:45.578 fetch POST /api/observability/v1/traces
09:28:45.579 fetched POST /api/observability/v1/traces 204
09:28:49.320 in Ping 
09:28:50.957 chunk 16 35893 bytes
09:28:50.958 in Ack 16
09:28:51.109 fetch POST /api/observability/v1/traces
09:28:51.109 fetched POST /api/observability/v1/traces 204
09:28:53.307 fetch POST /api/observability/v1/traces
09:28:53.307 fetched POST /api/observability/v1/traces 204
09:28:53.435 chunk 16 37914 bytes
09:28:53.437 in Ack 16
09:28:53.934 chunk 6 4784 bytes
09:28:53.935 in Ack 6
09:28:53.937 rpc 29 orchestration.dispatchCommand thread.visit
09:28:53.937 exit 29 {"_tag":"Success","value":{"sequence":0}}
09:28:54.320 in Ping 
09:28:54.438 fetch POST /api/observability/v1/traces
09:28:54.438 fetched POST /api/observability/v1/traces 204
09:28:54.545 rpc 30 server.reportClientActivity
09:28:54.782 chunk 16 38754 bytes
09:28:54.784 in Ack 16
09:28:55.547 fetch POST /api/observability/v1/traces
09:28:55.547 fetched POST /api/observability/v1/traces 204
09:28:56.283 chunk 16 41215 bytes
09:28:56.285 in Ack 16
09:28:56.533 chunk 16 41285 bytes
09:28:56.537 in Ack 16
09:28:56.784 chunk 16 41359 bytes
09:28:56.786 in Ack 16
09:28:57.033 chunk 16 41395 bytes
09:28:57.035 in Ack 16
09:28:57.287 fetch POST /api/observability/v1/traces
09:28:57.287 fetched POST /api/observability/v1/traces 204
09:28:57.536 chunk 16 42097 bytes
09:28:57.538 in Ack 16
09:28:58.287 chunk 16 42049 bytes
09:28:58.289 in Ack 16
09:28:58.539 fetch POST /api/observability/v1/traces
09:28:58.539 fetched POST /api/observability/v1/traces 204
09:28:59.321 in Ping 
09:28:59.792 chunk 16 42119 bytes
09:28:59.795 in Ack 16
09:29:00.796 fetch POST /api/observability/v1/traces
09:29:00.797 fetched POST /api/observability/v1/traces 204
09:29:02.217 chunk 6 4784 bytes
09:29:02.217 in Ack 6
09:29:02.553 chunk 16 42119 bytes
09:29:02.554 in Ack 16
09:29:03.055 chunk 16 42119 bytes
09:29:03.057 in Ack 16
09:29:03.220 fetch POST /api/observability/v1/traces
09:29:03.220 fetched POST /api/observability/v1/traces 204
09:29:03.940 rpc 31 orchestration.dispatchCommand thread.visit
09:29:03.940 exit 31 {"_tag":"Success","value":{"sequence":0}}
09:29:04.321 in Ping 
09:29:04.942 fetch POST /api/observability/v1/traces
09:29:04.942 fetched POST /api/observability/v1/traces 204
09:29:05.184 chunk 16 44604 bytes
09:29:05.186 in Ack 16
09:29:06.187 fetch POST /api/observability/v1/traces
09:29:06.187 fetched POST /api/observability/v1/traces 204
09:29:06.205 chunk 16 45131 bytes
09:29:06.206 in Ack 16
09:29:07.208 fetch POST /api/observability/v1/traces
09:29:07.208 fetched POST /api/observability/v1/traces 204
09:29:09.321 in Ping 
09:29:09.551 in Interrupt 30
09:29:10.553 fetch POST /api/observability/v1/traces
09:29:10.553 fetched POST /api/observability/v1/traces 204
09:29:13.428 chunk 16 45692 bytes
09:29:13.430 in Ack 16
09:29:13.680 chunk 16 47537 bytes
09:29:13.688 in Ack 16
09:29:13.930 chunk 16 47663 bytes
09:29:13.932 in Ack 16
09:29:14.365 in Ping 
09:29:14.431 fetch POST /api/observability/v1/traces
09:29:14.431 fetched POST /api/observability/v1/traces 204
09:29:14.704 exit 30 {"_tag":"Success","value":null}
09:29:14.866 chunk 16 48665 bytes
09:29:14.868 in Ack 16
09:29:15.272 rpc 32 orchestration.dispatchCommand runtime-request.respond
09:29:15.869 fetch POST /api/observability/v1/traces
09:29:15.869 fetched POST /api/observability/v1/traces 204
09:29:16.729 chunk 16 52152 bytes
09:29:16.731 in Ack 16
09:29:17.732 fetch POST /api/observability/v1/traces
09:29:17.732 fetched POST /api/observability/v1/traces 204
09:29:18.273 chunk 16 54221 bytes
09:29:18.274 in Ack 16
09:29:19.120 chunk 16 54271 bytes
09:29:19.124 in Ack 16
09:29:19.276 fetch POST /api/observability/v1/traces
09:29:19.276 fetched POST /api/observability/v1/traces 204
09:29:19.366 in Ping 
09:29:19.545 rpc 33 server.reportClientActivity
09:29:20.546 fetch POST /api/observability/v1/traces
09:29:20.546 fetched POST /api/observability/v1/traces 204
09:29:20.968 exit 32 {"_tag":"Success","value":{"sequence":186}}
09:29:21.617 rpc 34 orchestration.dispatchCommand runtime-request.respond
09:29:21.833 chunk 16 54314 bytes
09:29:21.836 in Ack 16
09:29:21.977 fetch POST /api/observability/v1/traces
09:29:21.977 fetched POST /api/observability/v1/traces 204
09:29:24.370 in Ping 
09:29:27.217 chunk 16 56836 bytes
09:29:27.221 in Ack 16
09:29:27.468 chunk 16 56984 bytes
09:29:27.470 in Ack 16
09:29:27.718 chunk 16 57136 bytes
09:29:27.721 in Ack 16
09:29:27.977 chunk 16 57270 bytes
09:29:27.980 in Ack 16
09:29:28.223 fetch POST /api/observability/v1/traces
09:29:28.223 fetched POST /api/observability/v1/traces 204
09:29:28.352 chunk 16 57416 bytes
09:29:28.354 in Ack 16
09:29:28.677 chunk 16 57510 bytes
09:29:28.679 in Ack 16
09:29:29.356 fetch POST /api/observability/v1/traces
09:29:29.356 fetched POST /api/observability/v1/traces 204
09:29:29.374 in Ping 
09:29:29.439 chunk 16 57462 bytes
09:29:29.439 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:29:29.441 in Ack 16
09:29:30.445 fetch POST /api/observability/v1/traces
09:29:30.445 fetched POST /api/observability/v1/traces 204
09:29:30.785 chunk 16 57502 bytes
09:29:30.787 in Ack 16
09:29:31.788 fetch POST /api/observability/v1/traces
09:29:31.788 fetched POST /api/observability/v1/traces 204
09:29:32.762 chunk 6 4786 bytes
09:29:32.763 in Ack 6
09:29:32.767 rpc 35 orchestration.dispatchCommand thread.visit
09:29:32.767 exit 35 {"_tag":"Success","value":{"sequence":0}}
09:29:33.264 chunk 16 58098 bytes
09:29:33.267 in Ack 16
09:29:33.517 chunk 16 58180 bytes
09:29:33.531 in Ack 16
09:29:33.789 fetch POST /api/observability/v1/traces
09:29:33.789 fetched POST /api/observability/v1/traces 204
