# D, webapp, warm start, seed 1

Profile D (graded): worst observed EDGE, inspired envelope. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 72 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 7.9s |
| Prompt reached the agent | 3.5s |
| Time to first text | 8.8s |
| Whole reply shown | 71.3s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 19.9s / 5.0s / 15.9s |
| Longest silence | 34.7s |
| Approval shown after it was asked | 15.3s |
| Bytes on the wire down / up | 154358 / 89691 |
| Bytes the app received / sent | 83608 / 56219 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 4.9 | 61 | 262 |
| 20 | 9.8 | 266 | 659 |
| 30 | 8.5 | 718 | 1065 |
| 40 | 0.0 | 1077 | 1077 |
| 50 | 0.0 | 1077 | 1077 |
| 60 | 8.9 | 1077 | 1425 |
| 70 | 18.9 | 1077 | 1596 |
| 71 | 19.9 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159543219,"silences":0,"longestSilenceMs":651,"longestRecoveredGapMs":651},"stats":{"sendRate":15187.5,"deliveredRate":4667.820364337571,"loss":0,"srttMs":5.305154931506391,"minRttMs":1,"sentPackets":89,"sentWireBytes":31293,"receivedPackets":139,"receivedWireBytes":69621,"lastHeardMs":5821,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 823/849","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 100/100","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159554092,"silences":2,"longestSilenceMs":3896,"longestRecoveredGapMs":3896},"stats":{"sendRate":2000,"deliveredRate":3025.3497110835488,"loss":0.02142857142857142,"srttMs":5473.224598983649,"minRttMs":7659,"sentPackets":128,"sentWireBytes":45707,"receivedPackets":156,"receivedWireBytes":76158,"lastHeardMs":16695,"unsyncedBytes":0,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 823/849","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 330/330","user":"10/10 9/9","attempts":"662/662 662/662","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159558954,"silences":2,"longestSilenceMs":3896,"longestRecoveredGapMs":3896},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.06115384615384615,"srttMs":7235.006655800718,"minRttMs":7659,"sentPackets":151,"sentWireBytes":50282,"receivedPackets":166,"receivedWireBytes":81158,"lastHeardMs":21556,"unsyncedBytes":492,"inflightPackets":22,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 1008/1113","user":"10/10 9/9","attempts":"662/662 662/662 159/638","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159574302,"silences":4,"longestSilenceMs":6023,"longestRecoveredGapMs":6023},"stats":{"sendRate":2000,"deliveredRate":258.52713178294573,"loss":0,"srttMs":7909.005095847425,"minRttMs":7659,"sentPackets":172,"sentWireBytes":56096,"receivedPackets":201,"receivedWireBytes":93669,"lastHeardMs":36905,"unsyncedBytes":0,"inflightPackets":38,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 159/638","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"566/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159584408,"silences":4,"longestSilenceMs":6023,"longestRecoveredGapMs":6023},"stats":{"sendRate":2000,"deliveredRate":1277.3330564784053,"loss":0,"srttMs":11127.253680115351,"minRttMs":7659,"sentPackets":196,"sentWireBytes":59279,"receivedPackets":238,"receivedWireBytes":107492,"lastHeardMs":47011,"unsyncedBytes":0,"inflightPackets":11,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083 1281/1554","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 1340/1340","user":"10/10 9/9","attempts":"662/662 662/662 259/638","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159594888,"silences":4,"longestSilenceMs":6023,"longestRecoveredGapMs":6023},"stats":{"sendRate":2000,"deliveredRate":283.57894736842104,"loss":0,"srttMs":11127.253680115351,"minRttMs":7659,"sentPackets":230,"sentWireBytes":69600,"receivedPackets":268,"receivedWireBytes":124774,"lastHeardMs":57490,"unsyncedBytes":893,"inflightPackets":37,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1108/1108 1058/1058 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083 1281/1554","frame":"1032/1032","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","assistant":"32/32 21/21 1340/1340 110/318","user":"10/10 9/9","attempts":"662/662 662/662 367/638","providerThreads":"810/810","providerSessions":"2688/2688","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"456/456 550/550 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"726/726","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":107,"lastDataAt":1794159605910,"silences":5,"longestSilenceMs":6694,"longestRecoveredGapMs":6694},"stats":{"sendRate":2000,"deliveredRate":571.6895207831835,"loss":0,"srttMs":10575.437523983526,"minRttMs":7659,"sentPackets":247,"sentWireBytes":76301,"receivedPackets":286,"receivedWireBytes":134469,"lastHeardMs":68513,"unsyncedBytes":401,"inflightPackets":28,"duplicateBytes":0}}
- shim trace
17:38:57.680 fetch GET /api/auth/session
17:38:57.681 fetched GET /api/auth/session 200
17:38:57.795 fetch GET /.well-known/t3/environment
17:38:57.795 fetched GET /.well-known/t3/environment 200
17:38:57.803 fetch GET /.well-known/t3/environment
17:38:57.804 fetched GET /.well-known/t3/environment 200
17:38:57.807 rpc 0 subscribeServerConfig
17:38:57.807 chunk 0 38690 bytes
17:38:57.833 in Ack 0
17:38:57.835 rpc 1 subscribeServerLifecycle
17:38:57.836 rpc 2 subscribeServerLifecycle
17:38:57.839 fetch GET /api/auth/session
17:38:57.840 fetched GET /api/auth/session 200
17:38:57.840 rpc 3 subscribeDeviceState
17:38:57.842 rpc 4 preview.reportProfiles
17:38:57.865 rpc 5 subscribeProjectClones
17:38:57.883 fetch GET /api/orchestration/threads/mcp%3Ae732d31b-402a-45c1-961d-655429f965a3/bounded
17:38:57.887 fetch GET /api/orchestration/shell
17:38:57.904 rpc 6 server.refreshProviders
17:38:57.940 rpc 7 orchestration.dispatchCommand thread.visit
17:38:57.940 exit 7 {"_tag":"Success","value":{"sequence":0}}
17:38:57.951 fetched GET /api/orchestration/shell 200
17:38:57.951 rpc 8 subscribeTerminalMetadata
17:38:57.951 rpc 9 subscribeVcsStatus
17:38:57.952 rpc 10 subscribePreviewEvents
17:38:57.954 rpc 11 orchestration.subscribeShell
17:38:57.954 chunk 11 2782 bytes
17:38:57.955 rpc 12 assets.createUrl
17:38:57.956 rpc 13 preview.list
17:38:57.956 rpc 14 projects.readFile
17:38:57.958 in Ack 11
17:38:58.040 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"73b308f5-9d9c-41d0-bc90-d93a40c3f081","sessions":[]}}
17:38:58.041 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
17:38:58.182 chunk 1 3864 bytes
17:38:58.183 in Ack 1
17:38:58.204 rpc 15 server.reportClientActivity
17:38:58.205 chunk 11 2758 bytes
17:38:58.205 in Ack 11
17:38:58.411 chunk 2 3864 bytes
17:38:58.411 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T17:38:45.863Z","configurableProviders":false,"con
17:38:58.411 chunk 9 534 bytes
17:38:58.412 in Ack 2
17:38:58.414 in Ack 9
17:38:58.519 exit 12 {"_tag":"Success","value":{"expiresAt":1794162600000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
17:38:58.738 chunk 3 876 bytes
17:38:58.738 exit 4 {"_tag":"Success","value":null}
17:38:58.738 chunk 5 4 bytes
17:38:58.738 chunk 8 36 bytes
17:38:58.738 exit 15 {"_tag":"Success","value":null}
17:38:58.739 in Ack 3
17:38:58.742 rpc 16 preview.reportProfiles
17:38:58.742 in Ack 5
17:38:58.743 in Ack 8
17:38:58.771 chunk 11 2801 bytes
17:38:58.771 in Ack 11
17:38:58.794 fetch POST /api/observability/v1/traces
17:38:58.794 fetched POST /api/observability/v1/traces 204
17:38:58.808 exit 16 {"_tag":"Success","value":null}
17:38:58.879 fetched GET /api/orchestration/threads/mcp%3Ae732d31b-402a-45c1-961d-655429f965a3/bounded 200
17:38:58.881 rpc 17 orchestration.subscribeThread
17:38:58.881 chunk 17 8305 bytes
17:38:58.890 in Interrupt 9
17:38:58.891 rpc 18 subscribeVcsStatus
17:38:58.892 in Ack 17
17:38:59.018 chunk 18 534 bytes
17:38:59.018 in Ack 18
17:38:59.132 chunk 17 11944 bytes
17:38:59.133 in Ack 17
17:38:59.162 rpc 19 server.reportClientActivity
17:38:59.350 exit 19 {"_tag":"Success","value":null}
17:38:59.404 chunk 17 13532 bytes
17:38:59.407 in Ack 17
17:38:59.671 rpc 20 attachments.createUploadUrl
17:38:59.821 chunk 17 18637 bytes
17:38:59.827 fetch POST /api/observability/v1/traces
17:38:59.827 fetched POST /api/observability/v1/traces 204
17:38:59.830 in Ack 17
17:38:59.876 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-35372d73-70cb-42de-9d82-7041d0afd210","expiresAt":1794160139848,"relativeUrl":"/api/attachments/upload/eyJ2ZX
17:38:59.877 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTM1MzcyZDczLTcwY2ItNDJkZS05ZDgyLTcwNDFkMGFmZDIxMCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2MDEzOTg0OH0.67zhZzf0vIVjzEnsnrzixadHt8VTdB8LMb3xUmwiE6U
17:39:00.072 chunk 17 21223 bytes
17:39:00.072 in Ack 17
17:39:00.079 in Interrupt 18
17:39:00.080 rpc 21 subscribeVcsStatus
17:39:00.098 chunk 21 534 bytes
17:39:00.098 in Ack 21
17:39:00.331 rpc 22 server.reportClientActivity
17:39:00.583 exit 22 {"_tag":"Success","value":null}
17:39:00.727 rpc 23 orchestration.dispatchCommand message.dispatch
17:39:01.264 fetch POST /api/observability/v1/traces
17:39:01.264 fetched POST /api/observability/v1/traces 204
17:39:01.309 exit 23 {"_tag":"Success","value":{"sequence":45}}
17:39:01.310 rpc 24 attachments.delete
17:39:01.566 chunk 11 2887 bytes
17:39:01.566 chunk 17 26156 bytes
17:39:01.567 in Ack 11
17:39:01.593 in Ack 17
17:39:01.595 rpc 25 assets.createUrl
17:39:01.676 exit 24 {"_tag":"Success","value":null}
17:39:01.817 chunk 17 30273 bytes
17:39:01.818 in Ack 17
17:39:02.066 chunk 11 2826 bytes
17:39:02.067 chunk 17 30345 bytes
17:39:02.067 in Ack 11
17:39:02.073 in Ack 17
17:39:02.311 fetch POST /api/observability/v1/traces
17:39:02.311 fetched POST /api/observability/v1/traces 204
17:39:02.317 chunk 11 2823 bytes
17:39:02.317 chunk 17 32924 bytes
17:39:02.317 in Ack 11
17:39:02.318 rpc 26 orchestration.dispatchCommand thread.visit
17:39:02.318 exit 26 {"_tag":"Success","value":{"sequence":0}}
17:39:02.323 in Ack 17
17:39:02.566 chunk 11 2823 bytes
17:39:02.567 chunk 17 33562 bytes
17:39:02.567 in Ack 11
17:39:02.572 in Ack 17
17:39:02.578 in Interrupt 21
17:39:02.580 rpc 27 subscribeVcsStatus
17:39:02.669 chunk 27 534 bytes
17:39:02.669 in Ack 27
17:39:02.810 in Ping 
17:39:02.817 chunk 17 35276 bytes
17:39:02.818 in Ack 17
17:39:02.832 rpc 28 server.reportClientActivity
17:39:02.929 exit 28 {"_tag":"Success","value":null}
17:39:03.048 exit 25 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/f0c0b65b-e7dd-43f0-ba7a-027ac2f9f280","expiresAt":1794163143048}}
17:39:03.318 fetch POST /api/observability/v1/traces
17:39:03.318 fetched POST /api/observability/v1/traces 204
17:39:04.231 rpc 29 orchestration.dispatchCommand message.dispatch
17:39:05.231 fetch POST /api/observability/v1/traces
17:39:05.231 fetched POST /api/observability/v1/traces 204
17:39:07.810 in Ping 
17:39:09.461 fetch POST /api/observability/v1/traces
17:39:09.461 fetched POST /api/observability/v1/traces 204
17:39:11.979 exit 29 {"_tag":"Success","value":{"sequence":75}}
17:39:12.480 chunk 17 37732 bytes
17:39:12.481 in Ack 17
17:39:12.731 chunk 17 37752 bytes
17:39:12.732 in Ack 17
17:39:12.811 in Ping 
17:39:12.981 fetch POST /api/observability/v1/traces
17:39:12.981 fetched POST /api/observability/v1/traces 204
17:39:13.155 chunk 17 37782 bytes
17:39:13.156 in Ack 17
17:39:13.406 chunk 17 37822 bytes
17:39:13.407 in Ack 17
17:39:13.656 chunk 17 37852 bytes
17:39:13.657 in Ack 17
17:39:13.907 chunk 17 37872 bytes
17:39:13.908 in Ack 17
17:39:14.157 chunk 17 37922 bytes
17:39:14.157 fetch POST /api/observability/v1/traces
17:39:14.157 fetched POST /api/observability/v1/traces 204
17:39:14.158 in Ack 17
17:39:14.810 chunk 17 37942 bytes
17:39:14.811 in Ack 17
17:39:15.061 chunk 17 37962 bytes
17:39:15.062 in Ack 17
17:39:15.159 fetch POST /api/observability/v1/traces
17:39:15.159 fetched POST /api/observability/v1/traces 204
17:39:15.312 chunk 17 38032 bytes
17:39:15.313 in Ack 17
17:39:15.915 chunk 17 38152 bytes
17:39:15.917 in Ack 17
17:39:16.314 fetch POST /api/observability/v1/traces
17:39:16.314 fetched POST /api/observability/v1/traces 204
17:39:16.405 chunk 17 38182 bytes
17:39:16.406 in Ack 17
17:39:17.083 chunk 17 38212 bytes
17:39:17.084 in Ack 17
17:39:17.333 chunk 17 38232 bytes
17:39:17.334 in Ack 17
17:39:17.407 fetch POST /api/observability/v1/traces
17:39:17.407 fetched POST /api/observability/v1/traces 204
17:39:17.583 chunk 17 38262 bytes
17:39:17.584 in Ack 17
17:39:17.811 in Ping 
17:39:18.158 chunk 17 38362 bytes
17:39:18.159 in Ack 17
17:39:18.585 fetch POST /api/observability/v1/traces
17:39:18.585 fetched POST /api/observability/v1/traces 204
17:39:18.956 chunk 17 38382 bytes
17:39:18.957 in Ack 17
17:39:19.958 fetch POST /api/observability/v1/traces
17:39:19.958 fetched POST /api/observability/v1/traces 204
17:39:22.811 in Ping 
17:39:23.050 rpc 30 server.reportClientActivity
17:39:24.050 fetch POST /api/observability/v1/traces
17:39:24.050 fetched POST /api/observability/v1/traces 204
17:39:24.978 chunk 17 38414 bytes
17:39:24.979 in Ack 17
17:39:25.228 chunk 17 38434 bytes
17:39:25.229 in Ack 17
17:39:25.652 chunk 17 38464 bytes
17:39:25.654 in Ack 17
17:39:25.902 chunk 17 38474 bytes
17:39:25.904 in Ack 17
17:39:25.980 fetch POST /api/observability/v1/traces
17:39:25.981 fetched POST /api/observability/v1/traces 204
17:39:26.153 chunk 17 38494 bytes
17:39:26.155 in Ack 17
17:39:26.403 chunk 17 38906 bytes
17:39:26.404 in Ack 17
17:39:26.707 chunk 17 39152 bytes
17:39:26.709 in Ack 17
17:39:26.957 chunk 17 39162 bytes
17:39:26.958 in Ack 17
17:39:27.156 fetch POST /api/observability/v1/traces
17:39:27.156 fetched POST /api/observability/v1/traces 204
17:39:27.209 chunk 17 39212 bytes
17:39:27.211 in Ack 17
17:39:27.733 chunk 17 39194 bytes
17:39:27.734 in Ack 17
17:39:27.811 in Ping 
17:39:27.983 chunk 17 39274 bytes
17:39:27.984 in Ack 17
17:39:28.211 fetch POST /api/observability/v1/traces
17:39:28.212 fetched POST /api/observability/v1/traces 204
17:39:28.233 chunk 17 39284 bytes
17:39:28.234 in Ack 17
17:39:28.965 chunk 17 39294 bytes
17:39:28.966 in Ack 17
17:39:29.216 chunk 17 39324 bytes
17:39:29.217 in Ack 17
17:39:29.247 fetch POST /api/observability/v1/traces
17:39:29.248 fetched POST /api/observability/v1/traces 204
17:39:29.508 chunk 17 39344 bytes
17:39:29.510 in Ack 17
17:39:29.758 chunk 17 39394 bytes
17:39:29.760 in Ack 17
17:39:30.019 chunk 17 39444 bytes
17:39:30.020 in Ack 17
17:39:30.425 fetch POST /api/observability/v1/traces
17:39:30.425 fetched POST /api/observability/v1/traces 204
17:39:32.812 in Ping 
17:39:33.001 chunk 17 39486 bytes
17:39:33.002 in Ack 17
17:39:33.508 chunk 17 39496 bytes
17:39:33.511 in Ack 17
17:39:33.758 chunk 17 39546 bytes
17:39:33.759 in Ack 17
17:39:34.003 fetch POST /api/observability/v1/traces
17:39:34.004 fetched POST /api/observability/v1/traces 204
17:39:34.088 chunk 17 39578 bytes
17:39:34.089 in Ack 17
17:39:34.339 chunk 17 39700 bytes
17:39:34.340 in Ack 17
17:39:34.799 chunk 17 39770 bytes
17:39:34.800 in Ack 17
17:39:35.090 fetch POST /api/observability/v1/traces
17:39:35.090 fetched POST /api/observability/v1/traces 204
17:39:35.583 chunk 17 39800 bytes
17:39:35.584 in Ack 17
17:39:35.833 chunk 17 39958 bytes
17:39:35.834 in Ack 17
17:39:36.083 chunk 17 40024 bytes
17:39:36.084 in Ack 17
17:39:36.333 chunk 17 40054 bytes
17:39:36.334 in Ack 17
17:39:36.583 chunk 17 40064 bytes
17:39:36.584 in Ack 17
17:39:36.592 fetch POST /api/observability/v1/traces
17:39:36.592 fetched POST /api/observability/v1/traces 204
17:39:36.834 chunk 17 40084 bytes
17:39:36.835 in Ack 17
17:39:37.505 chunk 17 40126 bytes
17:39:37.506 in Ack 17
17:39:37.755 chunk 17 40156 bytes
17:39:37.756 in Ack 17
17:39:37.812 in Ping 
17:39:37.835 fetch POST /api/observability/v1/traces
17:39:37.835 fetched POST /api/observability/v1/traces 204
17:39:38.015 chunk 17 40186 bytes
17:39:38.016 in Ack 17
17:39:38.052 in Interrupt 30
17:39:38.491 chunk 17 40196 bytes
17:39:38.492 in Ack 17
17:39:38.741 chunk 17 40226 bytes
17:39:38.742 in Ack 17
17:39:39.018 fetch POST /api/observability/v1/traces
17:39:39.018 fetched POST /api/observability/v1/traces 204
17:39:39.653 chunk 17 40254 bytes
17:39:39.654 in Ack 17
17:39:39.980 chunk 17 40284 bytes
17:39:39.981 in Ack 17
17:39:40.231 chunk 17 40324 bytes
17:39:40.232 in Ack 17
17:39:40.427 fetch POST /api/observability/v1/traces
17:39:40.427 fetched POST /api/observability/v1/traces 204
17:39:40.574 chunk 17 40362 bytes
17:39:40.575 in Ack 17
17:39:41.575 fetch POST /api/observability/v1/traces
17:39:41.575 fetched POST /api/observability/v1/traces 204
17:39:42.812 in Ping 
17:39:47.813 in Ping 
17:39:48.050 rpc 31 server.reportClientActivity
17:39:49.050 fetch POST /api/observability/v1/traces
17:39:49.050 fetched POST /api/observability/v1/traces 204
17:39:49.726 chunk 17 41373 bytes
17:39:49.727 in Ack 17
17:39:49.876 rpc 32 orchestration.dispatchCommand runtime-request.respond
17:39:50.430 fetch POST /api/observability/v1/traces
17:39:50.430 fetched POST /api/observability/v1/traces 204
17:39:52.816 in Ping 
17:39:57.820 in Ping 
17:40:00.433 fetch POST /api/observability/v1/traces
17:40:00.433 fetched POST /api/observability/v1/traces 204
17:40:02.825 in Ping 
17:40:03.050 in Interrupt 31
17:40:04.052 fetch POST /api/observability/v1/traces
17:40:04.052 fetched POST /api/observability/v1/traces 204
17:40:04.168 chunk 17 43851 bytes
17:40:04.169 exit 32 {"_tag":"Success","value":{"sequence":95}}
17:40:04.170 in Ack 17
17:40:04.419 chunk 17 43911 bytes
17:40:04.420 in Ack 17
17:40:04.510 rpc 33 orchestration.dispatchCommand runtime-request.respond
17:40:04.669 chunk 17 43981 bytes
17:40:04.670 in Ack 17
17:40:05.171 fetch POST /api/observability/v1/traces
17:40:05.171 fetched POST /api/observability/v1/traces 204
17:40:05.775 chunk 17 44001 bytes
17:40:05.776 in Ack 17
17:40:06.025 chunk 17 44061 bytes
17:40:06.026 in Ack 17
17:40:06.276 chunk 17 44081 bytes
17:40:06.277 in Ack 17
17:40:06.526 chunk 17 44101 bytes
17:40:06.527 in Ack 17
17:40:06.777 chunk 17 44171 bytes
17:40:06.777 fetch POST /api/observability/v1/traces
17:40:06.777 fetched POST /api/observability/v1/traces 204
17:40:06.778 in Ack 17
17:40:07.027 chunk 17 44191 bytes
17:40:07.028 in Ack 17
17:40:07.277 chunk 17 44227 bytes
17:40:07.278 in Ack 17
17:40:07.528 chunk 17 44239 bytes
17:40:07.530 in Ack 17
17:40:07.779 fetch POST /api/observability/v1/traces
17:40:07.779 fetched POST /api/observability/v1/traces 204
17:40:07.828 in Ping 
17:40:08.400 chunk 17 44259 bytes
17:40:08.402 in Ack 17
17:40:08.650 chunk 17 44319 bytes
17:40:08.652 in Ack 17
17:40:08.901 chunk 17 44355 bytes
17:40:08.903 in Ack 17
17:40:09.152 chunk 17 44375 bytes
17:40:09.153 in Ack 17
17:40:09.403 fetch POST /api/observability/v1/traces
17:40:09.403 fetched POST /api/observability/v1/traces 204
17:40:09.501 chunk 17 44395 bytes
17:40:09.503 in Ack 17
17:40:09.752 chunk 17 44419 bytes
17:40:09.753 in Ack 17
17:40:10.437 fetch POST /api/observability/v1/traces
17:40:10.437 fetched POST /api/observability/v1/traces 204
17:40:10.477 chunk 17 44439 bytes
17:40:10.478 in Ack 17
17:40:10.727 chunk 17 44459 bytes
17:40:10.728 in Ack 17
17:40:10.978 chunk 17 44515 bytes
17:40:10.979 in Ack 17
17:40:11.411 chunk 17 44563 bytes
17:40:11.413 in Ack 17
17:40:11.479 fetch POST /api/observability/v1/traces
17:40:11.479 fetched POST /api/observability/v1/traces 204
17:40:12.365 chunk 17 44591 bytes
17:40:12.367 in Ack 17
17:40:12.832 in Ping 
17:40:12.947 chunk 17 44627 bytes
17:40:12.948 in Ack 17
17:40:13.051 rpc 34 server.reportClientActivity
17:40:13.368 fetch POST /api/observability/v1/traces
17:40:13.368 fetched POST /api/observability/v1/traces 204
17:40:13.819 chunk 17 44649 bytes
17:40:13.821 in Ack 17
17:40:14.182 chunk 17 44671 bytes
17:40:14.183 in Ack 17
17:40:14.433 chunk 17 45319 bytes
17:40:14.434 in Ack 17
17:40:14.689 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
17:40:14.689 exit 30 {"_tag":"Success","value":null}
17:40:14.922 fetch POST /api/observability/v1/traces
17:40:14.922 fetched POST /api/observability/v1/traces 204
17:40:15.040 exit 31 {"_tag":"Success","value":null}
