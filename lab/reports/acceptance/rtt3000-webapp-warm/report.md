# rtt3000, webapp, warm start, seed 1

Profile rtt3000 (fault): 3 s round trip with 300 ms jitter, 200 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 58 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.9s |
| Prompt reached the agent | 1.9s |
| Time to first text | 4.5s |
| Whole reply shown | 57.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 10.7s / 2.4s / 7.7s |
| Longest silence | 13.2s |
| Approval shown after it was asked | 8.6s |
| Bytes on the wire down / up | 191004 / 171200 |
| Bytes the app received / sent | 66541 / 67360 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 2.6 | 218 | 325 |
| 20 | 2.9 | 604 | 727 |
| 30 | 3.2 | 996 | 1077 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 3.8 | 1309 | 1450 |
| 57 | 3.3 | 1571 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","messages":"456/456 546/546","attempts":"660/660 660/660","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"779/779 613/613 505/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167536035,"silences":0,"longestSilenceMs":1567,"longestRecoveredGapMs":1567},"stats":{"sendRate":6889.3621355489995,"deliveredRate":3154.636142498637,"loss":0,"srttMs":7.110922511444409,"minRttMs":2,"sentPackets":104,"sentWireBytes":30213,"receivedPackets":112,"receivedWireBytes":48315,"lastHeardMs":9500,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 295/295","user":"10/10 9/9","messages":"456/456 546/546 12/901","attempts":"660/660 660/660","providerTurns":"774/774","providerThreads":"638/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167546538,"silences":0,"longestSilenceMs":1660,"longestRecoveredGapMs":1660},"stats":{"sendRate":3895.5885533384635,"deliveredRate":2417.262640789508,"loss":0.06923076923076922,"srttMs":2921.0387284281787,"minRttMs":3610,"sentPackets":142,"sentWireBytes":43201,"receivedPackets":155,"receivedWireBytes":72115,"lastHeardMs":20003,"unsyncedBytes":0,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 808/823","user":"10/10 9/9","messages":"456/456 546/546 12/901","attempts":"660/660 660/660 636/636","providerTurns":"774/774","providerThreads":"638/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167557074,"silences":0,"longestSilenceMs":1660,"longestRecoveredGapMs":1660},"stats":{"sendRate":2000,"deliveredRate":802.7053777631145,"loss":0.0625,"srttMs":3729.4675875455446,"minRttMs":3581,"sentPackets":190,"sentWireBytes":53862,"receivedPackets":202,"receivedWireBytes":88376,"lastHeardMs":30538,"unsyncedBytes":193,"inflightPackets":29,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1318/1318","user":"10/10 9/9 64/64","messages":"456/456 546/546 106/901 348/545","attempts":"660/660 660/660 636/636","providerTurns":"774/774","providerThreads":"638/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167567183,"silences":0,"longestSilenceMs":1660,"longestRecoveredGapMs":1660},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.05092603609625669,"srttMs":3523.608766638254,"minRttMs":3052,"sentPackets":251,"sentWireBytes":68506,"receivedPackets":236,"receivedWireBytes":100127,"lastHeardMs":40647,"unsyncedBytes":167,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849 641/1546","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9 64/64","messages":"456/456 546/546 106/901 495/545","attempts":"660/660 660/660 636/636","providerTurns":"774/774","providerThreads":"638/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1005/1005"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167577425,"silences":0,"longestSilenceMs":1660,"longestRecoveredGapMs":1660},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.013999999999999995,"srttMs":3599.2061273172476,"minRttMs":3052,"sentPackets":298,"sentWireBytes":81801,"receivedPackets":253,"receivedWireBytes":107585,"lastHeardMs":50889,"unsyncedBytes":897,"inflightPackets":26,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 870/870 1098/1098 860/860 1299/1299 1098/1098 1058/1058 1073/1073 849/849 641/1546 144/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 449/449","user":"10/10 9/9 64/64","messages":"456/456 546/546 216/901 495/545","attempts":"660/660 660/660 636/636","providerTurns":"774/774","providerThreads":"638/808","providerSessions":"2694/2694","nodes":"1019/1019 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"732/732","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":103,"lastDataAt":1794167587440,"silences":1,"longestSilenceMs":2835,"longestRecoveredGapMs":2835},"stats":{"sendRate":2414.664436504821,"deliveredRate":1648.468559997636,"loss":0.022866666666666657,"srttMs":3543.702462849891,"minRttMs":3052,"sentPackets":333,"sentWireBytes":95433,"receivedPackets":262,"receivedWireBytes":112920,"lastHeardMs":60904,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- shim trace
19:52:06.862 fetch GET /api/auth/session
19:52:06.864 fetched GET /api/auth/session 200
19:52:06.983 fetch GET /.well-known/t3/environment
19:52:06.984 fetched GET /.well-known/t3/environment 200
19:52:06.994 fetch GET /.well-known/t3/environment
19:52:06.995 fetched GET /.well-known/t3/environment 200
19:52:06.998 rpc 0 subscribeServerConfig
19:52:06.999 chunk 0 38714 bytes
19:52:07.013 in Ack 0
19:52:07.015 rpc 1 subscribeServerLifecycle
19:52:07.016 rpc 2 subscribeServerLifecycle
19:52:07.018 fetch GET /api/auth/session
19:52:07.019 fetched GET /api/auth/session 200
19:52:07.019 rpc 3 subscribeDeviceState
19:52:07.021 rpc 4 preview.reportProfiles
19:52:07.062 rpc 5 subscribeProjectClones
19:52:07.080 fetch GET /api/orchestration/threads/mcp%3A3b57bde0-a567-42cf-a7fb-79a0d6dec68c/bounded
19:52:07.085 fetch GET /api/orchestration/shell
19:52:07.103 rpc 6 server.refreshProviders
19:52:07.139 rpc 7 orchestration.dispatchCommand thread.visit
19:52:07.139 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:52:07.151 fetched GET /api/orchestration/shell 200
19:52:07.151 rpc 8 subscribeTerminalMetadata
19:52:07.152 rpc 9 subscribeVcsStatus
19:52:07.153 rpc 10 subscribePreviewEvents
19:52:07.155 rpc 11 orchestration.subscribeShell
19:52:07.155 chunk 11 2788 bytes
19:52:07.156 rpc 12 assets.createUrl
19:52:07.157 rpc 13 preview.list
19:52:07.157 rpc 14 projects.readFile
19:52:07.161 in Ack 11
19:52:07.401 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f0e598d4-ad55-4c12-9b46-93a3deabf407","sessions":[]}}
19:52:07.401 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:52:07.407 rpc 15 server.reportClientActivity
19:52:07.661 chunk 1 3870 bytes
19:52:07.662 in Ack 1
19:52:07.864 chunk 2 3870 bytes
19:52:07.865 in Ack 2
19:52:07.981 fetch POST /api/observability/v1/traces
19:52:07.981 fetched POST /api/observability/v1/traces 204
19:52:08.063 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:51:53.705Z","configurableProviders":false,"con
19:52:08.063 chunk 9 1827 bytes
19:52:08.063 exit 12 {"_tag":"Success","value":{"expiresAt":1794169800000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:52:08.066 in Ack 9
19:52:08.279 chunk 11 2764 bytes
19:52:08.279 in Ack 11
19:52:08.529 chunk 11 2807 bytes
19:52:08.530 in Ack 11
19:52:08.578 chunk 3 876 bytes
19:52:08.578 exit 4 {"_tag":"Success","value":null}
19:52:08.578 chunk 5 4 bytes
19:52:08.578 chunk 8 36 bytes
19:52:08.578 exit 15 {"_tag":"Success","value":null}
19:52:08.578 in Ack 3
19:52:08.582 rpc 16 preview.reportProfiles
19:52:08.582 in Ack 5
19:52:08.582 in Ack 8
19:52:08.656 exit 16 {"_tag":"Success","value":null}
19:52:08.979 fetched GET /api/orchestration/threads/mcp%3A3b57bde0-a567-42cf-a7fb-79a0d6dec68c/bounded 200
19:52:08.982 rpc 17 orchestration.subscribeThread
19:52:08.982 chunk 17 8283 bytes
19:52:08.993 in Interrupt 9
19:52:08.996 rpc 18 subscribeVcsStatus
19:52:08.997 in Ack 17
19:52:09.064 fetch POST /api/observability/v1/traces
19:52:09.064 fetched POST /api/observability/v1/traces 204
19:52:09.148 chunk 18 1827 bytes
19:52:09.149 in Ack 18
19:52:09.236 chunk 17 11260 bytes
19:52:09.237 in Ack 17
19:52:09.267 rpc 19 server.reportClientActivity
19:52:09.487 chunk 17 12694 bytes
19:52:09.487 in Ack 17
19:52:09.738 chunk 17 13502 bytes
19:52:09.738 in Ack 17
19:52:09.866 rpc 20 attachments.createUploadUrl
19:52:10.118 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-31a7c6a3-0bc1-48d2-b7d6-6ce1e828d475","expiresAt":1794168130059,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:52:10.119 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTMxYTdjNmEzLTBiYzEtNDhkMi1iN2Q2LTZjZTFlODI4ZDQ3NSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2ODEzMDA1OX0.JCpz_hIwKj_C-TwWEHH_PoGrRmfrccaOs3ELi8aUBsA
19:52:10.242 fetch POST /api/observability/v1/traces
19:52:10.242 fetched POST /api/observability/v1/traces 204
19:52:10.291 chunk 17 18491 bytes
19:52:10.292 in Ack 17
19:52:10.356 exit 19 {"_tag":"Success","value":null}
19:52:10.544 chunk 17 19849 bytes
19:52:10.545 in Ack 17
19:52:10.552 in Interrupt 18
19:52:10.554 rpc 21 subscribeVcsStatus
19:52:10.688 chunk 21 1827 bytes
19:52:10.689 in Ack 21
19:52:10.798 chunk 17 21195 bytes
19:52:10.807 in Ack 17
19:52:10.816 rpc 22 server.reportClientActivity
19:52:11.293 fetch POST /api/observability/v1/traces
19:52:11.293 fetched POST /api/observability/v1/traces 204
19:52:11.947 exit 22 {"_tag":"Success","value":null}
19:52:11.997 in Ping 
19:52:12.948 fetch POST /api/observability/v1/traces
19:52:12.948 fetched POST /api/observability/v1/traces 204
19:52:13.097 rpc 23 orchestration.dispatchCommand message.dispatch
19:52:13.723 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:52:13.725 rpc 24 attachments.delete
19:52:13.983 chunk 17 23629 bytes
19:52:13.984 in Ack 17
19:52:14.097 fetch POST /api/observability/v1/traces
19:52:14.097 fetched POST /api/observability/v1/traces 204
19:52:14.233 chunk 17 23639 bytes
19:52:14.234 in Ack 17
19:52:14.288 exit 24 {"_tag":"Success","value":null}
19:52:14.488 chunk 17 23613 bytes
19:52:14.489 in Ack 17
19:52:14.738 chunk 17 23663 bytes
19:52:14.741 in Ack 17
19:52:14.988 chunk 11 2829 bytes
19:52:14.988 in Ack 11
19:52:14.989 rpc 25 orchestration.dispatchCommand thread.visit
19:52:14.989 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:52:15.235 fetch POST /api/observability/v1/traces
19:52:15.235 fetched POST /api/observability/v1/traces 204
19:52:15.238 chunk 17 24635 bytes
19:52:15.239 in Ack 17
19:52:15.488 chunk 11 2829 bytes
19:52:15.488 chunk 17 25296 bytes
19:52:15.488 in Ack 11
19:52:15.495 in Ack 17
19:52:15.738 chunk 17 30512 bytes
19:52:15.740 in Ack 17
19:52:15.742 rpc 26 assets.createUrl
19:52:16.035 chunk 17 31322 bytes
19:52:16.038 in Ack 17
19:52:16.242 fetch POST /api/observability/v1/traces
19:52:16.242 fetched POST /api/observability/v1/traces 204
19:52:16.756 rpc 27 orchestration.dispatchCommand message.dispatch
19:52:16.997 in Ping 
19:52:17.756 fetch POST /api/observability/v1/traces
19:52:17.757 fetched POST /api/observability/v1/traces 204
19:52:17.947 chunk 17 32102 bytes
19:52:17.948 in Ack 17
19:52:17.963 in Interrupt 21
19:52:17.969 rpc 28 subscribeVcsStatus
19:52:18.220 rpc 29 server.reportClientActivity
19:52:18.950 fetch POST /api/observability/v1/traces
19:52:18.950 fetched POST /api/observability/v1/traces 204
19:52:19.600 chunk 17 31936 bytes
19:52:19.601 in Ack 17
19:52:20.213 chunk 17 31936 bytes
19:52:20.214 in Ack 17
19:52:20.438 exit 27 {"_tag":"Success","value":{"sequence":75}}
19:52:20.490 fetch POST /api/observability/v1/traces
19:52:20.490 fetched POST /api/observability/v1/traces 204
19:52:20.905 chunk 17 34370 bytes
19:52:20.906 in Ack 17
19:52:21.156 chunk 17 34440 bytes
19:52:21.157 in Ack 17
19:52:21.664 chunk 17 34470 bytes
19:52:21.665 in Ack 17
19:52:21.907 fetch POST /api/observability/v1/traces
19:52:21.907 fetched POST /api/observability/v1/traces 204
19:52:21.914 chunk 17 34520 bytes
19:52:21.915 in Ack 17
19:52:21.998 in Ping 
19:52:22.364 chunk 17 34540 bytes
19:52:22.365 in Ack 17
19:52:22.614 chunk 17 34570 bytes
19:52:22.615 in Ack 17
19:52:22.864 chunk 17 34590 bytes
19:52:22.865 in Ack 17
19:52:22.916 fetch POST /api/observability/v1/traces
19:52:22.916 fetched POST /api/observability/v1/traces 204
19:52:23.197 chunk 17 34620 bytes
19:52:23.198 in Ack 17
19:52:23.448 chunk 17 34650 bytes
19:52:23.449 in Ack 17
19:52:23.697 chunk 17 34670 bytes
19:52:23.698 in Ack 17
19:52:23.947 chunk 17 34750 bytes
19:52:23.949 in Ack 17
19:52:24.199 fetch POST /api/observability/v1/traces
19:52:24.199 fetched POST /api/observability/v1/traces 204
19:52:24.357 chunk 17 34770 bytes
19:52:24.358 in Ack 17
19:52:24.608 chunk 17 34800 bytes
19:52:24.609 in Ack 17
19:52:24.997 chunk 17 34820 bytes
19:52:24.997 chunk 28 1827 bytes
19:52:24.998 in Ack 17
19:52:25.007 in Ack 28
19:52:25.359 fetch POST /api/observability/v1/traces
19:52:25.359 fetched POST /api/observability/v1/traces 204
19:52:25.825 chunk 17 34852 bytes
19:52:25.826 in Ack 17
19:52:26.539 chunk 17 34902 bytes
19:52:26.542 in Ack 17
19:52:26.828 fetch POST /api/observability/v1/traces
19:52:26.828 fetched POST /api/observability/v1/traces 204
19:52:26.997 chunk 17 34962 bytes
19:52:26.998 in Ack 17
19:52:27.043 in Ping 
19:52:27.247 chunk 17 34984 bytes
19:52:27.249 in Ack 17
19:52:27.559 chunk 17 35004 bytes
19:52:27.561 in Ack 17
19:52:27.809 chunk 17 35064 bytes
19:52:27.811 in Ack 17
19:52:27.999 fetch POST /api/observability/v1/traces
19:52:27.999 fetched POST /api/observability/v1/traces 204
19:52:28.612 chunk 17 35134 bytes
19:52:28.613 in Ack 17
19:52:28.863 chunk 17 35154 bytes
19:52:28.864 in Ack 17
19:52:29.544 chunk 17 35194 bytes
19:52:29.547 in Ack 17
19:52:29.614 fetch POST /api/observability/v1/traces
19:52:29.614 fetched POST /api/observability/v1/traces 204
19:52:30.264 chunk 17 35264 bytes
19:52:30.265 in Ack 17
19:52:30.514 chunk 17 35322 bytes
19:52:30.515 in Ack 17
19:52:30.926 chunk 17 35342 bytes
19:52:30.927 in Ack 17
19:52:31.177 chunk 17 35362 bytes
19:52:31.178 in Ack 17
19:52:31.287 fetch POST /api/observability/v1/traces
19:52:31.287 fetched POST /api/observability/v1/traces 204
19:52:31.427 chunk 17 35382 bytes
19:52:31.428 in Ack 17
19:52:31.677 chunk 17 35422 bytes
19:52:31.678 in Ack 17
19:52:31.927 chunk 17 35442 bytes
19:52:31.928 in Ack 17
19:52:32.043 in Ping 
19:52:32.369 chunk 17 35452 bytes
19:52:32.370 in Ack 17
19:52:32.428 fetch POST /api/observability/v1/traces
19:52:32.429 fetched POST /api/observability/v1/traces 204
19:52:32.619 chunk 17 35542 bytes
19:52:32.620 in Ack 17
19:52:32.870 chunk 17 35562 bytes
19:52:32.871 in Ack 17
19:52:33.221 in Interrupt 29
19:52:33.436 chunk 17 35572 bytes
19:52:33.437 in Ack 17
19:52:33.472 rpc 30 server.reportClientActivity
19:52:33.621 fetch POST /api/observability/v1/traces
19:52:33.621 fetched POST /api/observability/v1/traces 204
19:52:33.687 chunk 17 36279 bytes
19:52:33.688 in Ack 17
19:52:34.225 chunk 17 36309 bytes
19:52:34.227 in Ack 17
19:52:34.501 chunk 17 36319 bytes
19:52:34.502 in Ack 17
19:52:34.688 fetch POST /api/observability/v1/traces
19:52:34.688 fetched POST /api/observability/v1/traces 204
19:52:34.753 chunk 17 36339 bytes
19:52:34.754 in Ack 17
19:52:35.095 chunk 17 36349 bytes
19:52:35.096 in Ack 17
19:52:35.346 chunk 17 36409 bytes
19:52:35.347 in Ack 17
19:52:35.596 chunk 17 36459 bytes
19:52:35.597 in Ack 17
19:52:35.756 fetch POST /api/observability/v1/traces
19:52:35.756 fetched POST /api/observability/v1/traces 204
19:52:35.882 chunk 17 36481 bytes
19:52:35.884 in Ack 17
19:52:36.295 chunk 17 36491 bytes
19:52:36.298 in Ack 17
19:52:36.546 chunk 17 36541 bytes
19:52:36.547 in Ack 17
19:52:36.884 fetch POST /api/observability/v1/traces
19:52:36.885 fetched POST /api/observability/v1/traces 204
19:52:37.043 in Ping 
19:52:37.055 chunk 17 36551 bytes
19:52:37.056 in Ack 17
19:52:37.306 chunk 17 36571 bytes
19:52:37.307 in Ack 17
19:52:37.555 chunk 17 36581 bytes
19:52:37.556 in Ack 17
19:52:37.806 chunk 17 36681 bytes
19:52:37.808 in Ack 17
19:52:38.057 fetch POST /api/observability/v1/traces
19:52:38.057 fetched POST /api/observability/v1/traces 204
19:52:38.252 chunk 17 36701 bytes
19:52:38.254 in Ack 17
19:52:38.598 chunk 17 36721 bytes
19:52:38.600 in Ack 17
19:52:38.849 chunk 17 36741 bytes
19:52:38.850 in Ack 17
19:52:39.099 chunk 17 36781 bytes
19:52:39.100 in Ack 17
19:52:39.255 fetch POST /api/observability/v1/traces
19:52:39.255 fetched POST /api/observability/v1/traces 204
19:52:39.757 chunk 17 36813 bytes
19:52:39.759 in Ack 17
19:52:40.007 chunk 17 36873 bytes
19:52:40.008 in Ack 17
19:52:40.258 chunk 17 36903 bytes
19:52:40.274 in Ack 17
19:52:40.496 fetch POST /api/observability/v1/traces
19:52:40.496 fetched POST /api/observability/v1/traces 204
19:52:40.983 chunk 17 36933 bytes
19:52:40.984 in Ack 17
19:52:41.241 chunk 17 36963 bytes
19:52:41.242 in Ack 17
19:52:41.510 chunk 17 37003 bytes
19:52:41.511 in Ack 17
19:52:41.765 chunk 17 37023 bytes
19:52:41.766 in Ack 17
19:52:41.985 fetch POST /api/observability/v1/traces
19:52:41.985 fetched POST /api/observability/v1/traces 204
19:52:42.043 in Ping 
19:52:42.098 chunk 17 39046 bytes
19:52:42.100 in Ack 17
19:52:42.349 chunk 17 39958 bytes
19:52:42.350 in Ack 17
19:52:42.855 chunk 17 39988 bytes
19:52:42.856 in Ack 17
19:52:43.101 fetch POST /api/observability/v1/traces
19:52:43.101 fetched POST /api/observability/v1/traces 204
19:52:43.412 chunk 17 40028 bytes
19:52:43.413 in Ack 17
19:52:43.705 chunk 17 40078 bytes
19:52:43.706 in Ack 17
19:52:44.111 chunk 17 40108 bytes
19:52:44.112 in Ack 17
19:52:44.360 chunk 17 40138 bytes
19:52:44.362 in Ack 17
19:52:44.414 fetch POST /api/observability/v1/traces
19:52:44.414 fetched POST /api/observability/v1/traces 204
19:52:44.718 chunk 17 40158 bytes
19:52:44.719 in Ack 17
19:52:45.025 chunk 17 40188 bytes
19:52:45.026 in Ack 17
19:52:45.274 chunk 11 2893 bytes
19:52:45.275 chunk 17 40272 bytes
19:52:45.275 in Ack 11
19:52:45.277 rpc 31 orchestration.dispatchCommand thread.visit
19:52:45.277 exit 31 {"_tag":"Success","value":{"sequence":0}}
19:52:45.287 in Ack 17
19:52:46.379 fetch POST /api/observability/v1/traces
19:52:46.379 fetched POST /api/observability/v1/traces 204
19:52:46.416 chunk 17 40292 bytes
19:52:46.419 in Ack 17
19:52:46.796 chunk 17 40402 bytes
19:52:46.797 in Ack 17
19:52:47.139 in Ping 
19:52:47.215 chunk 17 40432 bytes
19:52:47.265 in Ack 17
19:52:47.427 fetch POST /api/observability/v1/traces
19:52:47.427 fetched POST /api/observability/v1/traces 204
19:52:47.466 chunk 17 40462 bytes
19:52:47.467 in Ack 17
19:52:47.737 chunk 17 40480 bytes
19:52:47.738 in Ack 17
19:52:48.484 fetch POST /api/observability/v1/traces
19:52:48.484 fetched POST /api/observability/v1/traces 204
19:52:48.496 in Interrupt 30
19:52:49.558 fetch POST /api/observability/v1/traces
19:52:49.558 fetched POST /api/observability/v1/traces 204
19:52:50.591 fetch POST /api/observability/v1/traces
19:52:50.591 fetched POST /api/observability/v1/traces 204
19:52:52.139 in Ping 
19:52:53.860 chunk 17 41485 bytes
19:52:53.861 in Ack 17
19:52:54.295 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:52:54.862 fetch POST /api/observability/v1/traces
19:52:54.862 fetched POST /api/observability/v1/traces 204
19:52:57.140 in Ping 
19:52:57.235 rpc 33 server.reportClientActivity
19:52:58.239 fetch POST /api/observability/v1/traces
19:52:58.239 fetched POST /api/observability/v1/traces 204
19:52:58.804 exit 32 {"_tag":"Success","value":{"sequence":96}}
19:52:59.769 rpc 34 orchestration.dispatchCommand runtime-request.respond
19:52:59.807 fetch POST /api/observability/v1/traces
19:52:59.807 fetched POST /api/observability/v1/traces 204
19:52:59.969 chunk 17 44055 bytes
19:52:59.970 in Ack 17
19:53:00.227 chunk 17 44145 bytes
19:53:00.229 in Ack 17
19:53:00.971 fetch POST /api/observability/v1/traces
19:53:00.971 fetched POST /api/observability/v1/traces 204
19:53:01.996 chunk 17 44265 bytes
19:53:01.997 in Ack 17
19:53:02.150 in Ping 
19:53:02.890 chunk 17 44375 bytes
19:53:02.892 in Ack 17
19:53:03.133 fetch POST /api/observability/v1/traces
19:53:03.133 fetched POST /api/observability/v1/traces 204
19:53:03.908 chunk 17 44485 bytes
19:53:03.909 in Ack 17
19:53:04.606 chunk 17 44588 bytes
19:53:04.606 exit 34 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:53:04.608 in Ack 17
19:53:05.143 fetch POST /api/observability/v1/traces
19:53:05.143 fetched POST /api/observability/v1/traces 204
19:53:07.156 in Ping 
19:53:07.441 chunk 17 44856 bytes
19:53:07.443 in Ack 17
19:53:08.444 fetch POST /api/observability/v1/traces
19:53:08.444 fetched POST /api/observability/v1/traces 204
19:53:08.478 chunk 17 45014 bytes
19:53:08.479 in Ack 17
19:53:09.481 fetch POST /api/observability/v1/traces
19:53:09.481 fetched POST /api/observability/v1/traces 204
19:53:09.721 chunk 17 45098 bytes
19:53:09.723 in Ack 17
19:53:10.502 chunk 17 45194 bytes
19:53:10.504 in Ack 17
19:53:10.724 fetch POST /api/observability/v1/traces
19:53:10.725 fetched POST /api/observability/v1/traces 204
19:53:11.756 chunk 17 45288 bytes
19:53:11.757 in Ack 17
19:53:12.085 chunk 17 45378 bytes
19:53:12.086 in Ack 17
19:53:12.170 in Ping 
19:53:12.296 in Interrupt 33
19:53:12.759 fetch POST /api/observability/v1/traces
19:53:12.759 fetched POST /api/observability/v1/traces 204
19:53:13.279 chunk 17 45452 bytes
19:53:13.281 in Ack 17
