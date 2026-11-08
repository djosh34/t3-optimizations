# reorder, webapp, warm start, seed 1

Profile reorder (fault): 25% of packets reordered, 10% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 46 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.7s |
| Time to first text | 1.5s |
| Whole reply shown | 45.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 1.5s / 0.7s / 1.4s |
| Longest silence | 1.7s |
| Approval shown after it was asked | 1.0s |
| Bytes on the wire down / up | 486405 / 367016 |
| Bytes the app received / sent | 319262 / 56180 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.7 | 341 | 373 |
| 20 | 0.6 | 746 | 774 |
| 30 | 0.7 | 1077 | 1109 |
| 40 | 1.5 | 1429 | 1483 |
| 44 | 1.0 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774 774/774","messages":"456/456 546/546 597/901 546/546","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":265,"lastDataAt":1794167625487,"silences":0,"longestSilenceMs":1104,"longestRecoveredGapMs":1104},"stats":{"sendRate":3375,"deliveredRate":3173.857605926693,"loss":0.019999999999999993,"srttMs":210.4737411794941,"minRttMs":2,"sentPackets":117,"sentWireBytes":34633,"receivedPackets":124,"receivedWireBytes":55796,"lastHeardMs":10180,"unsyncedBytes":509,"inflightPackets":9,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 470/470","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","messages":"456/456 546/546 901/901 546/546 451/451","providerThreads":"810/810","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":265,"lastDataAt":1794167635547,"silences":0,"longestSilenceMs":1104,"longestRecoveredGapMs":1104},"stats":{"sendRate":2000,"deliveredRate":1591.3461538461538,"loss":0.36363636363636365,"srttMs":478.5530319586568,"minRttMs":420,"sentPackets":174,"sentWireBytes":44831,"receivedPackets":231,"receivedWireBytes":99795,"lastHeardMs":20240,"unsyncedBytes":0,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 953/953","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","messages":"456/456 546/546 901/901 546/546 451/451 545/545","providerThreads":"810/810","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019 785/785","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":265,"lastDataAt":1794167645764,"silences":0,"longestSilenceMs":1104,"longestRecoveredGapMs":1104},"stats":{"sendRate":1500,"deliveredRate":1410.3081742986112,"loss":0.36,"srttMs":560.7465387098968,"minRttMs":3,"sentPackets":235,"sentWireBytes":55942,"receivedPackets":499,"receivedWireBytes":160650,"lastHeardMs":30457,"unsyncedBytes":0,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 926/926 1546/1546","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 53/53","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","messages":"456/456 546/546 901/901 546/546 451/451 545/545","providerThreads":"810/810","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 389/961 1104/1371","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"286/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":265,"lastDataAt":1794167655795,"silences":0,"longestSilenceMs":1104,"longestRecoveredGapMs":1104},"stats":{"sendRate":1687.5,"deliveredRate":0,"loss":0.20166963636363633,"srttMs":679.3668065045073,"minRttMs":3,"sentPackets":290,"sentWireBytes":66755,"receivedPackets":843,"receivedWireBytes":242457,"lastHeardMs":40488,"unsyncedBytes":566,"inflightPackets":26,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1098/1098 870/870 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 950/950 1570/1570 1073/1073","frame":"1030/1030","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 609/609","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 774/774 656/656","messages":"456/456 546/546 901/901 546/546 451/451 546/546 545/545","providerThreads":"810/810","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809 1019/1019 785/785 985/985 1395/1395 1019/1019","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":265,"lastDataAt":1794167666821,"silences":0,"longestSilenceMs":1104,"longestRecoveredGapMs":1104},"stats":{"sendRate":3559.5703125,"deliveredRate":0,"loss":0.16853977272727272,"srttMs":534.5553942453348,"minRttMs":2,"sentPackets":354,"sentWireBytes":80669,"receivedPackets":1115,"receivedWireBytes":334099,"lastHeardMs":51514,"unsyncedBytes":317,"inflightPackets":36,"duplicateBytes":0}}
- shim trace
19:53:35.665 fetch GET /api/auth/session
19:53:35.666 fetched GET /api/auth/session 200
19:53:35.823 fetch GET /.well-known/t3/environment
19:53:35.824 fetched GET /.well-known/t3/environment 200
19:53:35.839 fetch GET /.well-known/t3/environment
19:53:35.840 fetched GET /.well-known/t3/environment 200
19:53:35.843 rpc 0 subscribeServerConfig
19:53:35.843 chunk 0 38714 bytes
19:53:35.869 in Ack 0
19:53:35.871 rpc 1 subscribeServerLifecycle
19:53:35.872 rpc 2 subscribeServerLifecycle
19:53:35.875 fetch GET /api/auth/session
19:53:35.876 fetched GET /api/auth/session 200
19:53:35.877 rpc 3 subscribeDeviceState
19:53:35.879 rpc 4 preview.reportProfiles
19:53:35.926 rpc 5 subscribeProjectClones
19:53:35.952 fetch GET /api/orchestration/threads/mcp%3A8c8382f7-2b52-4079-a0c2-5cd2fc575eaa/bounded
19:53:35.961 fetch GET /api/orchestration/shell
19:53:35.982 rpc 6 server.refreshProviders
19:53:36.029 rpc 7 orchestration.dispatchCommand thread.visit
19:53:36.029 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:53:36.044 fetched GET /api/orchestration/shell 200
19:53:36.045 rpc 8 subscribeTerminalMetadata
19:53:36.046 rpc 9 subscribeVcsStatus
19:53:36.047 rpc 10 subscribePreviewEvents
19:53:36.049 rpc 11 orchestration.subscribeShell
19:53:36.049 chunk 11 2788 bytes
19:53:36.051 rpc 12 assets.createUrl
19:53:36.052 rpc 13 preview.list
19:53:36.053 rpc 14 projects.readFile
19:53:36.058 in Ack 11
19:53:36.271 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"1502eabb-1c72-4bba-bb04-01bc101e5007","sessions":[]}}
19:53:36.271 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:53:36.299 rpc 15 server.reportClientActivity
19:53:36.533 chunk 1 3870 bytes
19:53:36.535 in Ack 1
19:53:36.750 chunk 2 3870 bytes
19:53:36.754 in Ack 2
19:53:36.822 fetch POST /api/observability/v1/traces
19:53:36.822 fetched POST /api/observability/v1/traces 204
19:53:36.944 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:53:22.451Z","configurableProviders":false,"con
19:53:36.944 chunk 9 1911 bytes
19:53:36.944 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:53:36.947 in Ack 9
19:53:37.149 chunk 11 2764 bytes
19:53:37.150 in Ack 11
19:53:37.400 chunk 11 2807 bytes
19:53:37.400 in Ack 11
19:53:37.428 chunk 3 876 bytes
19:53:37.428 exit 4 {"_tag":"Success","value":null}
19:53:37.428 chunk 5 4 bytes
19:53:37.428 chunk 8 36 bytes
19:53:37.428 exit 15 {"_tag":"Success","value":null}
19:53:37.429 in Ack 3
19:53:37.432 in Ack 5
19:53:37.432 in Ack 8
19:53:37.848 fetched GET /api/orchestration/threads/mcp%3A8c8382f7-2b52-4079-a0c2-5cd2fc575eaa/bounded 200
19:53:37.852 rpc 16 orchestration.subscribeThread
19:53:37.852 chunk 16 8283 bytes
19:53:37.861 in Interrupt 9
19:53:37.864 rpc 17 subscribeVcsStatus
19:53:37.913 in Ack 16
19:53:37.945 fetch POST /api/observability/v1/traces
19:53:37.946 fetched POST /api/observability/v1/traces 204
19:53:38.000 chunk 17 1911 bytes
19:53:38.001 in Ack 17
19:53:38.103 chunk 16 11260 bytes
19:53:38.104 in Ack 16
19:53:38.138 rpc 18 server.reportClientActivity
19:53:38.355 chunk 16 12694 bytes
19:53:38.355 in Ack 16
19:53:38.397 exit 18 {"_tag":"Success","value":null}
19:53:38.605 chunk 16 13502 bytes
19:53:38.606 in Ack 16
19:53:38.732 rpc 19 attachments.createUploadUrl
19:53:38.899 chunk 16 18025 bytes
19:53:38.907 in Ack 16
19:53:38.968 exit 19 {"_tag":"Success","value":{"attachmentId":"pending-2adaf489-751a-4fc6-864d-8dc22428541d","expiresAt":1794168218938,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:53:38.969 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTJhZGFmNDg5LTc1MWEtNGZjNi04NjRkLThkYzIyNDI4NTQxZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODIxODkzOH0.0MtTtHMCBGBJjB6jVvCHymE41vZkyqYdKJEVqfuI-k0
19:53:39.105 fetch POST /api/observability/v1/traces
19:53:39.105 fetched POST /api/observability/v1/traces 204
19:53:39.150 chunk 16 19849 bytes
19:53:39.151 in Ack 16
19:53:39.159 in Interrupt 17
19:53:39.160 rpc 20 subscribeVcsStatus
19:53:39.221 chunk 20 1911 bytes
19:53:39.222 in Ack 20
19:53:39.401 chunk 16 21195 bytes
19:53:39.403 in Ack 16
19:53:39.467 rpc 21 server.reportClientActivity
19:53:40.154 fetch POST /api/observability/v1/traces
19:53:40.154 fetched POST /api/observability/v1/traces 204
19:53:40.844 in Ping 
19:53:40.897 exit 21 {"_tag":"Success","value":null}
19:53:41.619 rpc 22 orchestration.dispatchCommand message.dispatch
19:53:42.174 fetch POST /api/observability/v1/traces
19:53:42.174 fetched POST /api/observability/v1/traces 204
19:53:42.218 exit 22 {"_tag":"Success","value":{"sequence":45}}
19:53:42.219 rpc 23 attachments.delete
19:53:42.488 exit 23 {"_tag":"Success","value":null}
19:53:42.738 chunk 16 23601 bytes
19:53:42.739 in Ack 16
19:53:42.988 chunk 16 23613 bytes
19:53:42.989 in Ack 16
19:53:43.219 fetch POST /api/observability/v1/traces
19:53:43.219 fetched POST /api/observability/v1/traces 204
19:53:43.238 chunk 16 23663 bytes
19:53:43.239 in Ack 16
19:53:43.488 chunk 11 2829 bytes
19:53:43.489 in Ack 11
19:53:43.490 rpc 24 orchestration.dispatchCommand thread.visit
19:53:43.490 exit 24 {"_tag":"Success","value":{"sequence":0}}
19:53:43.739 chunk 16 24635 bytes
19:53:43.740 in Ack 16
19:53:43.988 chunk 11 2829 bytes
19:53:43.989 in Ack 11
19:53:43.994 chunk 16 28089 bytes
19:53:43.995 in Ack 16
19:53:43.998 rpc 25 assets.createUrl
19:53:44.491 fetch POST /api/observability/v1/traces
19:53:44.491 fetched POST /api/observability/v1/traces 204
19:53:44.492 chunk 16 31936 bytes
19:53:44.494 in Ack 16
19:53:44.501 in Interrupt 20
19:53:44.503 rpc 26 subscribeVcsStatus
19:53:44.746 chunk 16 31936 bytes
19:53:44.747 in Ack 16
19:53:44.760 rpc 27 server.reportClientActivity
19:53:44.958 chunk 26 1911 bytes
19:53:44.967 in Ack 26
19:53:44.999 chunk 16 31936 bytes
19:53:45.000 in Ack 16
19:53:45.253 chunk 16 32711 bytes
19:53:45.262 in Ack 16
19:53:45.452 rpc 28 orchestration.dispatchCommand message.dispatch
19:53:45.496 fetch POST /api/observability/v1/traces
19:53:45.496 fetched POST /api/observability/v1/traces 204
19:53:45.505 chunk 16 35210 bytes
19:53:45.508 in Ack 16
19:53:45.571 exit 27 {"_tag":"Success","value":null}
19:53:45.756 chunk 16 35210 bytes
19:53:45.757 in Ack 16
19:53:45.844 in Ping 
19:53:46.308 exit 28 {"_tag":"Success","value":{"sequence":75}}
19:53:46.509 fetch POST /api/observability/v1/traces
19:53:46.509 fetched POST /api/observability/v1/traces 204
19:53:46.758 chunk 16 37654 bytes
19:53:46.759 in Ack 16
19:53:47.008 chunk 16 37674 bytes
19:53:47.009 in Ack 16
19:53:47.259 chunk 16 37704 bytes
19:53:47.261 in Ack 16
19:53:47.553 chunk 16 37716 bytes
19:53:47.555 in Ack 16
19:53:47.582 fetch POST /api/observability/v1/traces
19:53:47.582 fetched POST /api/observability/v1/traces 204
19:53:47.804 chunk 11 2893 bytes
19:53:47.804 chunk 16 37756 bytes
19:53:47.805 in Ack 11
19:53:47.841 in Ack 16
19:53:48.054 chunk 16 38413 bytes
19:53:48.056 in Ack 16
19:53:48.305 chunk 16 38393 bytes
19:53:48.306 in Ack 16
19:53:48.557 chunk 16 38423 bytes
19:53:48.558 in Ack 16
19:53:48.806 fetch POST /api/observability/v1/traces
19:53:48.806 fetched POST /api/observability/v1/traces 204
19:53:48.808 chunk 16 38457 bytes
19:53:48.810 in Ack 16
19:53:49.059 chunk 16 40468 bytes
19:53:49.060 in Ack 16
19:53:49.310 chunk 16 40488 bytes
19:53:49.311 in Ack 16
19:53:49.560 chunk 16 40500 bytes
19:53:49.561 in Ack 16
19:53:49.810 chunk 16 41368 bytes
19:53:49.811 in Ack 16
19:53:49.886 fetch POST /api/observability/v1/traces
19:53:49.886 fetched POST /api/observability/v1/traces 204
19:53:50.060 chunk 16 41400 bytes
19:53:50.061 in Ack 16
19:53:50.309 chunk 16 42216 bytes
19:53:50.310 in Ack 16
19:53:50.560 chunk 16 42750 bytes
19:53:50.561 in Ack 16
19:53:50.810 chunk 16 42798 bytes
19:53:50.811 in Ack 16
19:53:50.845 in Ping 
19:53:51.061 chunk 16 42818 bytes
19:53:51.062 in Ack 16
19:53:51.089 fetch POST /api/observability/v1/traces
19:53:51.089 fetched POST /api/observability/v1/traces 204
19:53:51.311 chunk 16 43497 bytes
19:53:51.312 in Ack 16
19:53:51.562 chunk 16 43507 bytes
19:53:51.563 in Ack 16
19:53:51.813 chunk 16 43539 bytes
19:53:51.814 in Ack 16
19:53:52.064 chunk 16 43559 bytes
19:53:52.065 in Ack 16
19:53:52.323 fetch POST /api/observability/v1/traces
19:53:52.323 fetched POST /api/observability/v1/traces 204
19:53:52.324 chunk 16 43601 bytes
19:53:52.325 in Ack 16
19:53:52.574 chunk 16 43627 bytes
19:53:52.575 in Ack 16
19:53:52.825 chunk 16 43657 bytes
19:53:52.826 in Ack 16
19:53:53.076 chunk 16 43677 bytes
19:53:53.077 in Ack 16
19:53:53.203 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/1472dca3-a274-4b38-837c-b92187da6929","expiresAt":1794171233203}}
19:53:53.325 chunk 16 43697 bytes
19:53:53.327 in Ack 16
19:53:53.337 fetch POST /api/observability/v1/traces
19:53:53.337 fetched POST /api/observability/v1/traces 204
19:53:53.490 rpc 29 orchestration.dispatchCommand thread.visit
19:53:53.490 exit 29 {"_tag":"Success","value":{"sequence":0}}
19:53:53.577 chunk 16 43727 bytes
19:53:53.578 in Ack 16
19:53:53.827 chunk 16 43757 bytes
19:53:53.829 in Ack 16
19:53:54.078 chunk 16 43789 bytes
19:53:54.079 in Ack 16
19:53:54.329 chunk 16 43807 bytes
19:53:54.330 in Ack 16
19:53:54.491 fetch POST /api/observability/v1/traces
19:53:54.491 fetched POST /api/observability/v1/traces 204
19:53:54.580 chunk 16 43837 bytes
19:53:54.581 in Ack 16
19:53:54.830 chunk 16 43857 bytes
19:53:54.831 in Ack 16
19:53:55.079 chunk 11 2893 bytes
19:53:55.079 in Ack 11
19:53:55.084 chunk 16 43877 bytes
19:53:55.085 in Ack 16
19:53:55.333 chunk 16 43897 bytes
19:53:55.335 in Ack 16
19:53:55.582 fetch POST /api/observability/v1/traces
19:53:55.582 fetched POST /api/observability/v1/traces 204
19:53:55.584 chunk 16 43967 bytes
19:53:55.585 in Ack 16
19:53:55.845 in Ping 
19:53:55.962 chunk 16 43977 bytes
19:53:55.963 in Ack 16
19:53:56.213 chunk 16 44017 bytes
19:53:56.214 in Ack 16
19:53:56.559 chunk 16 44027 bytes
19:53:56.560 in Ack 16
19:53:56.587 fetch POST /api/observability/v1/traces
19:53:56.587 fetched POST /api/observability/v1/traces 204
19:53:56.808 chunk 11 2893 bytes
19:53:56.809 chunk 16 44047 bytes
19:53:56.809 in Ack 11
19:53:56.811 in Ack 16
19:53:57.059 chunk 16 44077 bytes
19:53:57.060 in Ack 16
19:53:57.309 chunk 16 45238 bytes
19:53:57.311 in Ack 16
19:53:57.692 chunk 16 45253 bytes
19:53:57.693 in Ack 16
19:53:57.811 fetch POST /api/observability/v1/traces
19:53:57.811 fetched POST /api/observability/v1/traces 204
19:53:57.943 chunk 16 45268 bytes
19:53:57.944 in Ack 16
19:53:58.194 chunk 16 45358 bytes
19:53:58.195 in Ack 16
19:53:58.531 chunk 16 45373 bytes
19:53:58.532 in Ack 16
19:53:58.781 chunk 16 45433 bytes
19:53:58.782 in Ack 16
19:53:58.945 fetch POST /api/observability/v1/traces
19:53:58.945 fetched POST /api/observability/v1/traces 204
19:53:59.031 chunk 16 45448 bytes
19:53:59.032 in Ack 16
19:53:59.282 chunk 16 45478 bytes
19:53:59.283 in Ack 16
19:53:59.533 chunk 16 45523 bytes
19:53:59.534 in Ack 16
19:53:59.782 chunk 16 45583 bytes
19:53:59.783 in Ack 16
19:54:00.033 fetch POST /api/observability/v1/traces
19:54:00.033 fetched POST /api/observability/v1/traces 204
19:54:00.059 chunk 16 45598 bytes
19:54:00.060 in Ack 16
19:54:00.309 chunk 16 45688 bytes
19:54:00.310 in Ack 16
19:54:00.731 chunk 16 45703 bytes
19:54:00.732 in Ack 16
19:54:00.846 in Ping 
19:54:00.981 chunk 16 45733 bytes
19:54:00.982 in Ack 16
19:54:01.061 fetch POST /api/observability/v1/traces
19:54:01.061 fetched POST /api/observability/v1/traces 204
19:54:01.078 rpc 30 server.reportClientActivity
19:54:01.231 chunk 16 45793 bytes
19:54:01.232 in Ack 16
19:54:01.491 chunk 16 45808 bytes
19:54:01.492 in Ack 16
19:54:01.741 chunk 16 45838 bytes
19:54:01.742 in Ack 16
19:54:01.967 exit 30 {"_tag":"Success","value":null}
19:54:01.992 chunk 16 45883 bytes
19:54:01.993 in Ack 16
19:54:02.078 fetch POST /api/observability/v1/traces
19:54:02.079 fetched POST /api/observability/v1/traces 204
19:54:02.242 chunk 16 45913 bytes
19:54:02.243 in Ack 16
19:54:02.493 chunk 16 46018 bytes
19:54:02.494 in Ack 16
19:54:02.797 chunk 16 46063 bytes
19:54:02.799 in Ack 16
19:54:03.244 fetch POST /api/observability/v1/traces
19:54:03.244 fetched POST /api/observability/v1/traces 204
19:54:03.341 chunk 16 46078 bytes
19:54:03.342 in Ack 16
19:54:03.491 rpc 31 orchestration.dispatchCommand thread.visit
19:54:03.491 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:54:03.592 chunk 16 46183 bytes
19:54:03.593 in Ack 16
19:54:03.842 chunk 16 46198 bytes
19:54:03.843 in Ack 16
19:54:04.343 fetch POST /api/observability/v1/traces
19:54:04.343 fetched POST /api/observability/v1/traces 204
19:54:04.404 chunk 16 46243 bytes
19:54:04.405 in Ack 16
19:54:04.654 chunk 11 2893 bytes
19:54:04.654 chunk 16 46273 bytes
19:54:04.654 in Ack 11
19:54:04.660 in Ack 16
19:54:04.907 chunk 16 46348 bytes
19:54:04.908 in Ack 16
19:54:05.174 chunk 16 46363 bytes
19:54:05.175 in Ack 16
19:54:05.407 fetch POST /api/observability/v1/traces
19:54:05.407 fetched POST /api/observability/v1/traces 204
19:54:05.425 chunk 16 46393 bytes
19:54:05.426 in Ack 16
19:54:05.676 chunk 16 46423 bytes
19:54:05.677 in Ack 16
19:54:05.847 in Ping 
19:54:05.927 chunk 16 46513 bytes
19:54:05.928 in Ack 16
19:54:06.228 chunk 16 46573 bytes
19:54:06.230 in Ack 16
19:54:06.611 fetch POST /api/observability/v1/traces
19:54:06.611 fetched POST /api/observability/v1/traces 204
19:54:06.612 chunk 16 46588 bytes
19:54:06.614 in Ack 16
19:54:06.863 chunk 16 46603 bytes
19:54:06.864 in Ack 16
19:54:07.114 chunk 16 46633 bytes
19:54:07.115 in Ack 16
19:54:07.364 chunk 16 46723 bytes
19:54:07.365 in Ack 16
19:54:07.615 fetch POST /api/observability/v1/traces
19:54:07.615 fetched POST /api/observability/v1/traces 204
19:54:07.708 chunk 16 46798 bytes
19:54:07.709 in Ack 16
19:54:07.958 chunk 16 46828 bytes
19:54:07.959 in Ack 16
19:54:08.243 chunk 16 46873 bytes
19:54:08.244 in Ack 16
19:54:08.627 chunk 16 46933 bytes
19:54:08.629 in Ack 16
19:54:08.709 fetch POST /api/observability/v1/traces
19:54:08.709 fetched POST /api/observability/v1/traces 204
19:54:09.281 chunk 16 46948 bytes
19:54:09.283 in Ack 16
19:54:09.532 chunk 16 46978 bytes
19:54:09.533 in Ack 16
19:54:09.782 chunk 16 47008 bytes
19:54:09.783 in Ack 16
19:54:10.033 chunk 16 47113 bytes
19:54:10.034 in Ack 16
19:54:10.284 fetch POST /api/observability/v1/traces
19:54:10.284 fetched POST /api/observability/v1/traces 204
19:54:10.284 chunk 16 47173 bytes
19:54:10.286 in Ack 16
19:54:10.534 chunk 16 47203 bytes
19:54:10.535 in Ack 16
19:54:10.784 chunk 16 47233 bytes
19:54:10.785 in Ack 16
19:54:10.847 in Ping 
19:54:11.272 chunk 16 47248 bytes
19:54:11.273 in Ack 16
19:54:11.294 fetch POST /api/observability/v1/traces
19:54:11.294 fetched POST /api/observability/v1/traces 204
19:54:11.522 chunk 16 47263 bytes
19:54:11.523 in Ack 16
19:54:11.773 chunk 16 47368 bytes
19:54:11.774 in Ack 16
19:54:12.172 chunk 16 47383 bytes
19:54:12.173 in Ack 16
19:54:12.422 chunk 16 47413 bytes
19:54:12.423 in Ack 16
19:54:12.524 fetch POST /api/observability/v1/traces
19:54:12.524 fetched POST /api/observability/v1/traces 204
19:54:12.673 chunk 16 47443 bytes
19:54:12.674 in Ack 16
19:54:12.924 chunk 16 47473 bytes
19:54:12.925 in Ack 16
19:54:13.173 chunk 16 47518 bytes
19:54:13.174 in Ack 16
19:54:13.423 chunk 16 47563 bytes
19:54:13.424 in Ack 16
19:54:13.673 chunk 16 47605 bytes
19:54:13.674 in Ack 16
19:54:13.683 fetch POST /api/observability/v1/traces
19:54:13.683 fetched POST /api/observability/v1/traces 204
19:54:13.924 chunk 16 50685 bytes
19:54:13.925 in Ack 16
19:54:14.299 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:54:14.434 chunk 16 54182 bytes
19:54:14.435 in Ack 16
19:54:14.684 chunk 16 54232 bytes
19:54:14.685 in Ack 16
19:54:14.927 fetch POST /api/observability/v1/traces
19:54:14.927 fetched POST /api/observability/v1/traces 204
19:54:14.933 chunk 11 3201 bytes
19:54:14.933 in Ack 11
19:54:15.151 exit 32 {"_tag":"Success","value":{"sequence":97}}
19:54:15.152 rpc 33 orchestration.dispatchCommand thread.visit
19:54:15.152 exit 33 {"_tag":"Success","value":{"sequence":0}}
19:54:15.188 chunk 16 56703 bytes
19:54:15.189 in Ack 16
19:54:15.439 chunk 16 56723 bytes
19:54:15.440 in Ack 16
19:54:15.633 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:54:15.689 chunk 16 56743 bytes
19:54:15.690 in Ack 16
19:54:15.852 in Ping 
19:54:15.935 fetch POST /api/observability/v1/traces
19:54:15.935 fetched POST /api/observability/v1/traces 204
19:54:15.940 chunk 16 56773 bytes
19:54:15.941 in Ack 16
19:54:16.191 chunk 16 56836 bytes
19:54:16.192 in Ack 16
19:54:16.442 chunk 16 56808 bytes
19:54:16.443 in Ack 16
19:54:16.691 chunk 16 56828 bytes
19:54:16.693 in Ack 16
19:54:16.714 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:54:16.942 chunk 16 56868 bytes
19:54:16.943 fetch POST /api/observability/v1/traces
19:54:16.943 fetched POST /api/observability/v1/traces 204
19:54:16.944 in Ack 16
19:54:17.192 chunk 16 56936 bytes
19:54:17.193 in Ack 16
19:54:17.199 in Interrupt 26
19:54:17.203 rpc 35 subscribeVcsStatus
19:54:17.442 chunk 16 57952 bytes
19:54:17.444 in Ack 16
19:54:17.509 rpc 36 server.reportClientActivity
19:54:17.693 chunk 16 60399 bytes
19:54:17.694 in Ack 16
19:54:17.825 chunk 35 1911 bytes
19:54:17.825 in Ack 35
19:54:17.942 chunk 11 2895 bytes
19:54:17.942 chunk 16 60517 bytes
19:54:17.943 in Ack 11
19:54:17.950 in Ack 16
19:54:18.013 fetch POST /api/observability/v1/traces
19:54:18.013 fetched POST /api/observability/v1/traces 204
19:54:18.315 chunk 16 60527 bytes
19:54:18.317 in Ack 16
19:54:18.565 chunk 16 60547 bytes
19:54:18.567 in Ack 16
19:54:18.815 chunk 16 60567 bytes
19:54:18.816 in Ack 16
19:54:19.020 exit 36 {"_tag":"Success","value":null}
19:54:19.066 chunk 16 60637 bytes
19:54:19.067 in Ack 16
19:54:19.320 chunk 16 60657 bytes
19:54:19.321 fetch POST /api/observability/v1/traces
19:54:19.321 fetched POST /api/observability/v1/traces 204
19:54:19.322 in Ack 16
19:54:19.762 chunk 16 60667 bytes
19:54:19.763 in Ack 16
19:54:20.014 chunk 16 60687 bytes
19:54:20.016 in Ack 16
19:54:20.264 chunk 16 60707 bytes
19:54:20.265 in Ack 16
19:54:20.323 fetch POST /api/observability/v1/traces
19:54:20.323 fetched POST /api/observability/v1/traces 204
19:54:20.515 chunk 16 60757 bytes
19:54:20.516 in Ack 16
19:54:20.766 chunk 16 60767 bytes
19:54:20.767 in Ack 16
19:54:20.852 in Ping 
19:54:21.024 chunk 16 60827 bytes
19:54:21.025 in Ack 16
19:54:21.336 chunk 16 60867 bytes
19:54:21.338 in Ack 16
19:54:21.517 fetch POST /api/observability/v1/traces
19:54:21.517 fetched POST /api/observability/v1/traces 204
19:54:21.877 chunk 16 60917 bytes
19:54:21.879 in Ack 16
19:54:22.310 chunk 16 60957 bytes
19:54:22.311 in Ack 16
19:54:22.561 chunk 16 60969 bytes
19:54:22.562 in Ack 16
19:54:22.880 fetch POST /api/observability/v1/traces
19:54:22.880 fetched POST /api/observability/v1/traces 204
19:54:23.080 chunk 16 60973 bytes
19:54:23.081 in Ack 16
19:54:23.331 chunk 11 2895 bytes
19:54:23.331 chunk 16 60985 bytes
19:54:23.332 in Ack 11
19:54:23.334 in Ack 16
19:54:23.584 chunk 16 61978 bytes
19:54:23.585 in Ack 16
19:54:23.835 chunk 16 62008 bytes
19:54:23.836 in Ack 16
19:54:24.082 fetch POST /api/observability/v1/traces
19:54:24.082 fetched POST /api/observability/v1/traces 204
19:54:24.085 chunk 16 62149 bytes
19:54:24.086 in Ack 16
19:54:24.539 chunk 16 62245 bytes
19:54:24.540 in Ack 16
19:54:24.936 rpc 37 orchestration.dispatchCommand thread.visit
19:54:24.936 exit 37 {"_tag":"Success","value":{"sequence":0}}
19:54:24.958 chunk 16 62320 bytes
19:54:24.959 in Ack 16
19:54:25.087 fetch POST /api/observability/v1/traces
19:54:25.087 fetched POST /api/observability/v1/traces 204
19:54:25.557 chunk 16 62416 bytes
19:54:25.558 in Ack 16
19:54:25.807 chunk 16 62416 bytes
19:54:25.809 in Ack 16
19:54:25.852 in Ping 
19:54:26.057 chunk 11 2895 bytes
19:54:26.058 in Ack 11
19:54:26.091 rpc 38 server.reportClientActivity
19:54:26.467 chunk 16 62560 bytes
19:54:26.468 in Ack 16
19:54:26.567 fetch POST /api/observability/v1/traces
19:54:26.567 fetched POST /api/observability/v1/traces 204
19:54:27.108 chunk 16 62671 bytes
19:54:27.108 exit 38 {"_tag":"Success","value":null}
19:54:27.110 in Ack 16
19:54:27.568 chunk 16 62746 bytes
19:54:27.570 in Ack 16
19:54:27.631 fetch POST /api/observability/v1/traces
19:54:27.631 fetched POST /api/observability/v1/traces 204
19:54:27.819 chunk 16 62782 bytes
19:54:27.820 in Ack 16
19:54:28.070 chunk 16 62818 bytes
19:54:28.071 in Ack 16
19:54:28.418 chunk 16 62884 bytes
19:54:28.420 in Ack 16
19:54:28.821 fetch POST /api/observability/v1/traces
19:54:28.821 fetched POST /api/observability/v1/traces 204
19:54:29.280 chunk 16 62899 bytes
19:54:29.281 in Ack 16
19:54:29.531 chunk 16 62997 bytes
19:54:29.532 in Ack 16
19:54:29.782 chunk 16 63129 bytes
19:54:29.784 in Ack 16
19:54:30.283 fetch POST /api/observability/v1/traces
19:54:30.283 fetched POST /api/observability/v1/traces 204
19:54:30.283 chunk 11 2831 bytes
19:54:30.283 chunk 16 63153 bytes
19:54:30.283 in Ack 11
19:54:30.285 rpc 39 orchestration.dispatchCommand thread.visit
19:54:30.285 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:54:30.290 in Ack 16
