# story, webapp, warm start, seed 17

Profile story (graded): seed 17's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 106 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 26.9s |
| Prompt reached the agent | 26.1s |
| Time to first text | 46.0s |
| Whole reply shown | 104.7s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 49.8s / 8.3s / 43.8s |
| Longest silence | 46.0s |
| Approval shown after it was asked | 35.7s |
| Bytes on the wire down / up | 258607 / 110528 |
| Bytes the app received / sent | 119432 / 105319 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 3.8 | 0 | 153 |
| 40 | 13.8 | 0 | 555 |
| 50 | 13.8 | 397 | 957 |
| 60 | 23.8 | 397 | 1077 |
| 70 | 33.8 | 397 | 1077 |
| 80 | 43.8 | 397 | 1077 |
| 90 | 0.3 | 1077 | 1094 |
| 100 | 2.6 | 1378 | 1472 |
| 104 | 6.6 | 1378 | 1596 |

## Driver notes

- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217061347,"silences":0,"longestSilenceMs":1277,"longestRecoveredGapMs":1277},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0,"srttMs":10.681747531143243,"minRttMs":1,"sentPackets":114,"sentWireBytes":33349,"receivedPackets":156,"receivedWireBytes":110773,"lastHeardMs":12397,"unsyncedBytes":975,"inflightPackets":14,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217064112,"silences":0,"longestSilenceMs":1277,"longestRecoveredGapMs":1277},"stats":{"sendRate":2250,"deliveredRate":0,"loss":0,"srttMs":1431.7048036675476,"minRttMs":1,"sentPackets":124,"sentWireBytes":38386,"receivedPackets":168,"receivedWireBytes":122414,"lastHeardMs":15162,"unsyncedBytes":518,"inflightPackets":17,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217080287,"silences":2,"longestSilenceMs":9532,"longestRecoveredGapMs":9532},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":1431.7048036675476,"minRttMs":null,"sentPackets":129,"sentWireBytes":40642,"receivedPackets":170,"receivedWireBytes":122977,"lastHeardMs":31337,"unsyncedBytes":975,"inflightPackets":22,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 154/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217090950,"silences":4,"longestSilenceMs":9532,"longestRecoveredGapMs":9532},"stats":{"sendRate":2000,"deliveredRate":109.50104025988247,"loss":0.6,"srttMs":4252.741703209104,"minRttMs":24000,"sentPackets":140,"sentWireBytes":44613,"receivedPackets":187,"receivedWireBytes":129965,"lastHeardMs":42001,"unsyncedBytes":457,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 154/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217090950,"silences":4,"longestSilenceMs":9532,"longestRecoveredGapMs":9532},"stats":{"sendRate":2000,"deliveredRate":109.50104025988247,"loss":0.6,"srttMs":4252.741703209104,"minRttMs":24000,"sentPackets":140,"sentWireBytes":44613,"receivedPackets":187,"receivedWireBytes":129965,"lastHeardMs":42001,"unsyncedBytes":914,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 154/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217111680,"silences":5,"longestSilenceMs":13889,"longestRecoveredGapMs":13889},"stats":{"sendRate":2000,"deliveredRate":873.9027097557419,"loss":0.3075692307692307,"srttMs":3471.759067409197,"minRttMs":1314,"sentPackets":175,"sentWireBytes":51827,"receivedPackets":226,"receivedWireBytes":143980,"lastHeardMs":62730,"unsyncedBytes":0,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 609/1340","runs":"1014/1014 971/971 473/839","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217114914,"silences":5,"longestSilenceMs":13889,"longestRecoveredGapMs":13889},"stats":{"sendRate":3000,"deliveredRate":1417.370325693607,"loss":0.08333333333333337,"srttMs":2658.613896691768,"minRttMs":1314,"sentPackets":202,"sentWireBytes":57641,"receivedPackets":248,"receivedWireBytes":151526,"lastHeardMs":65964,"unsyncedBytes":0,"inflightPackets":33,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 609/1340","runs":"1014/1014 971/971 473/839","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217114914,"silences":5,"longestSilenceMs":13889,"longestRecoveredGapMs":13889},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.08333333333333337,"srttMs":2658.613896691768,"minRttMs":1314,"sentPackets":227,"sentWireBytes":64349,"receivedPackets":248,"receivedWireBytes":151526,"lastHeardMs":65964,"unsyncedBytes":457,"inflightPackets":56,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","thread":"961/961","assistant":"32/32 21/21 609/1340","runs":"1014/1014 971/971 473/839","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217114914,"silences":5,"longestSilenceMs":13889,"longestRecoveredGapMs":13889},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.08333333333333337,"srttMs":2658.613896691768,"minRttMs":1314,"sentPackets":245,"sentWireBytes":69394,"receivedPackets":248,"receivedWireBytes":151526,"lastHeardMs":65964,"unsyncedBytes":457,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1194/1542","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 473/839","user":"10/10 9/9","messages":"456/456 544/544 901/901 544/544","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"599/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217152445,"silences":6,"longestSilenceMs":29906,"longestRecoveredGapMs":29906},"stats":{"sendRate":5375.883550637272,"deliveredRate":1980.7950365326433,"loss":0.47046967213114743,"srttMs":1634.641607796731,"minRttMs":1278,"sentPackets":353,"sentWireBytes":100921,"receivedPackets":278,"receivedWireBytes":167421,"lastHeardMs":103495,"unsyncedBytes":855,"inflightPackets":121,"duplicateBytes":0}}
- webapp {"objects":{"frame":"1029/1029","turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1566/1566 1068/1068 610/947","thread":"961/961","assistant":"32/32 21/21 1340/1340 409/409","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","messages":"456/456 544/544 901/901 544/544 543/543","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 796/983","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":593,"lastDataAt":1794217163555,"silences":6,"longestSilenceMs":29906,"longestRecoveredGapMs":29906},"stats":{"sendRate":7002.709614085885,"deliveredRate":1243.0202437243654,"loss":0.5265238315137422,"srttMs":1778.9173608846493,"minRttMs":1278,"sentPackets":449,"sentWireBytes":127975,"receivedPackets":378,"receivedWireBytes":210013,"lastHeardMs":114606,"unsyncedBytes":0,"inflightPackets":149,"duplicateBytes":0}}
- shim trace
09:37:29.703 fetch GET /api/auth/session
09:37:29.705 fetched GET /api/auth/session 200
09:37:30.410 fetch GET /.well-known/t3/environment
09:37:30.412 fetched GET /.well-known/t3/environment 200
09:37:30.429 fetch GET /.well-known/t3/environment
09:37:30.430 fetched GET /.well-known/t3/environment 200
09:37:30.435 rpc 0 subscribeServerConfig
09:37:30.436 chunk 0 38462 bytes
09:37:30.485 in Ack 0
09:37:30.487 rpc 1 subscribeServerLifecycle
09:37:30.488 rpc 2 subscribeServerLifecycle
09:37:30.492 fetch GET /api/auth/session
09:37:30.496 fetched GET /api/auth/session 200
09:37:30.497 rpc 3 subscribeDeviceState
09:37:30.499 rpc 4 preview.reportProfiles
09:37:30.550 rpc 5 subscribeProjectClones
09:37:30.557 fetch GET /api/orchestration/threads/mcp%3A2149add5-0750-4c72-b28a-c23500cefe62/bounded
09:37:30.601 fetch GET /api/orchestration/shell
09:37:30.602 fetched GET /api/orchestration/shell 200
09:37:30.606 rpc 6 orchestration.subscribeShell
09:37:30.606 chunk 6 4576 bytes
09:37:30.641 rpc 7 server.refreshProviders
09:37:30.713 rpc 8 orchestration.dispatchCommand thread.visit
09:37:30.713 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:30.742 rpc 9 subscribeTerminalMetadata
09:37:30.742 rpc 10 subscribeVcsStatus
09:37:30.743 rpc 11 subscribePreviewEvents
09:37:30.745 rpc 12 assets.createUrl
09:37:30.746 rpc 13 preview.list
09:37:30.748 rpc 14 projects.readFile
09:37:30.750 in Ack 6
09:37:30.889 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"bbc441ea-781c-4985-8276-d3ad5d558772","sessions":[]}}
09:37:30.889 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:30.996 rpc 15 server.reportClientActivity
09:37:31.133 chunk 1 3815 bytes
09:37:31.133 chunk 2 3815 bytes
09:37:31.142 in Ack 1
09:37:31.143 in Ack 2
09:37:31.228 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:04.843Z","configurableProviders":false,"con
09:37:31.228 chunk 10 1632 bytes
09:37:31.228 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:31.235 in Ack 10
09:37:31.329 chunk 6 4595 bytes
09:37:31.330 in Ack 6
09:37:31.407 fetch POST /api/observability/v1/traces
09:37:31.407 fetched POST /api/observability/v1/traces 204
09:37:31.581 chunk 6 4592 bytes
09:37:31.582 in Ack 6
09:37:31.621 chunk 3 876 bytes
09:37:31.621 exit 4 {"_tag":"Success","value":null}
09:37:31.621 chunk 5 4 bytes
09:37:31.621 chunk 9 36 bytes
09:37:31.621 exit 15 {"_tag":"Success","value":null}
09:37:31.623 in Ack 3
09:37:31.629 in Ack 5
09:37:31.629 in Ack 9
09:37:31.832 chunk 6 4592 bytes
09:37:31.832 in Ack 6
09:37:32.583 fetch POST /api/observability/v1/traces
09:37:32.583 fetched POST /api/observability/v1/traces 204
09:37:32.666 fetched GET /api/orchestration/threads/mcp%3A2149add5-0750-4c72-b28a-c23500cefe62/bounded 200
09:37:32.671 rpc 16 orchestration.subscribeThread
09:37:32.671 chunk 16 8282 bytes
09:37:32.689 in Interrupt 10
09:37:32.692 rpc 17 subscribeVcsStatus
09:37:32.756 in Ack 16
09:37:32.863 chunk 17 1632 bytes
09:37:32.864 in Ack 17
09:37:32.921 chunk 16 11259 bytes
09:37:32.922 in Ack 16
09:37:33.010 rpc 18 server.reportClientActivity
09:37:33.171 chunk 16 13497 bytes
09:37:33.172 in Ack 16
09:37:33.434 exit 18 {"_tag":"Success","value":null}
09:37:33.435 chunk 16 19796 bytes
09:37:33.437 in Ack 16
09:37:33.444 in Interrupt 17
09:37:33.446 rpc 19 subscribeVcsStatus
09:37:33.493 chunk 19 1632 bytes
09:37:33.493 in Ack 19
09:37:33.692 rpc 20 attachments.createUploadUrl
09:37:33.726 fetch POST /api/observability/v1/traces
09:37:33.726 fetched POST /api/observability/v1/traces 204
09:37:33.766 chunk 16 21099 bytes
09:37:33.767 rpc 21 server.reportClientActivity
09:37:33.783 in Ack 16
09:37:33.840 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e43ac2d1-5ac8-4758-8b14-a752c83a6916","expiresAt":1794217653813,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:33.841 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWU0M2FjMmQxLTVhYzgtNDc1OC04YjE0LWE3NTJjODNhNjkxNiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1MzgxM30.SUjlCMgqSAS45n64VtPfvbvOHXQtTpOpcEDmc38PGvU
09:37:33.904 exit 21 {"_tag":"Success","value":null}
09:37:34.767 fetch POST /api/observability/v1/traces
09:37:34.767 fetched POST /api/observability/v1/traces 204
09:37:35.432 in Ping 
09:37:36.813 rpc 22 orchestration.dispatchCommand message.dispatch
09:37:37.196 exit 22 {"_tag":"Success","value":{"sequence":109}}
09:37:37.199 rpc 23 attachments.delete
09:37:37.373 exit 23 {"_tag":"Success","value":null}
09:37:37.461 chunk 6 4678 bytes
09:37:37.461 chunk 16 26809 bytes
09:37:37.462 in Ack 6
09:37:37.521 in Ack 16
09:37:37.524 rpc 24 assets.createUrl
09:37:37.713 chunk 16 27722 bytes
09:37:37.715 in Ack 16
09:37:37.811 fetch POST /api/observability/v1/traces
09:37:37.811 fetched POST /api/observability/v1/traces 204
09:37:37.964 chunk 16 30179 bytes
09:37:37.966 in Ack 16
09:37:38.214 chunk 6 4617 bytes
09:37:38.214 chunk 16 30885 bytes
09:37:38.215 in Ack 6
09:37:38.224 in Ack 16
09:37:38.466 chunk 16 31902 bytes
09:37:38.468 in Ack 16
09:37:38.689 exit 24 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/8e44bde3-32e8-4088-ba8b-a8200d953554","expiresAt":1794220658689}}
09:37:38.967 fetch POST /api/observability/v1/traces
09:37:38.967 fetched POST /api/observability/v1/traces 204
09:37:39.235 chunk 16 32034 bytes
09:37:39.238 in Ack 16
09:37:39.613 chunk 6 4614 bytes
09:37:39.614 chunk 16 35259 bytes
09:37:39.615 in Ack 6
09:37:39.616 rpc 25 orchestration.dispatchCommand thread.visit
09:37:39.616 exit 25 {"_tag":"Success","value":{"sequence":0}}
09:37:39.626 in Ack 16
09:37:39.634 in Interrupt 19
09:37:39.637 rpc 26 subscribeVcsStatus
09:37:39.668 chunk 26 1632 bytes
09:37:39.668 in Ack 26
09:37:39.884 chunk 16 35259 bytes
09:37:39.940 in Ack 16
09:37:39.951 rpc 27 server.reportClientActivity
09:37:40.248 fetch POST /api/observability/v1/traces
09:37:40.248 fetched POST /api/observability/v1/traces 204
09:37:40.474 in Ping 
09:37:40.590 chunk 6 4614 bytes
09:37:40.592 in Ack 6
09:37:41.291 rpc 28 orchestration.dispatchCommand message.dispatch
09:37:41.594 fetch POST /api/observability/v1/traces
09:37:41.594 fetched POST /api/observability/v1/traces 204
09:37:43.728 exit 27 {"_tag":"Success","value":null}
09:37:44.194 fetch POST /api/observability/v1/traces
09:37:44.194 fetched POST /api/observability/v1/traces 204
09:37:45.474 in Ping 
09:37:50.475 in Ping 
09:37:55.475 in Ping 
09:37:55.673 rpc 29 server.reportClientActivity
09:37:56.673 fetch POST /api/observability/v1/traces
09:37:56.673 fetched POST /api/observability/v1/traces 204
09:38:00.476 in Ping 
09:38:05.477 in Ping 
09:38:07.880 exit 28 {"_tag":"Success","value":{"sequence":153}}
09:38:08.882 fetch POST /api/observability/v1/traces
09:38:08.882 fetched POST /api/observability/v1/traces 204
09:38:10.477 in Ping 
09:38:10.674 in Interrupt 29
09:38:11.675 fetch POST /api/observability/v1/traces
09:38:11.675 fetched POST /api/observability/v1/traces 204
09:38:15.477 in Ping 
09:38:20.478 in Ping 
09:38:20.671 rpc 30 server.reportClientActivity
09:38:21.670 fetch POST /api/observability/v1/traces
09:38:21.670 fetched POST /api/observability/v1/traces 204
09:38:25.482 in Ping 
09:38:25.607 chunk 16 38074 bytes
09:38:25.610 in Ack 16
09:38:26.398 chunk 16 38508 bytes
09:38:26.402 in Ack 16
09:38:26.611 fetch POST /api/observability/v1/traces
09:38:26.611 fetched POST /api/observability/v1/traces 204
09:38:26.888 chunk 16 38672 bytes
09:38:26.903 in Ack 16
09:38:27.904 fetch POST /api/observability/v1/traces
09:38:27.904 fetched POST /api/observability/v1/traces 204
09:38:29.682 chunk 16 38624 bytes
09:38:29.684 in Ack 16
09:38:30.485 in Ping 
09:38:30.686 fetch POST /api/observability/v1/traces
09:38:30.686 fetched POST /api/observability/v1/traces 204
09:38:33.147 chunk 6 4678 bytes
09:38:33.148 in Ack 6
09:38:33.153 rpc 31 orchestration.dispatchCommand thread.visit
09:38:33.153 exit 31 {"_tag":"Success","value":{"sequence":0}}
09:38:33.648 chunk 16 39260 bytes
09:38:33.649 in Ack 16
09:38:34.149 fetch POST /api/observability/v1/traces
09:38:34.149 fetched POST /api/observability/v1/traces 204
09:38:34.945 chunk 16 39488 bytes
09:38:34.948 in Ack 16
09:38:35.489 in Ping 
09:38:35.671 in Interrupt 30
09:38:35.949 fetch POST /api/observability/v1/traces
09:38:35.949 fetched POST /api/observability/v1/traces 204
09:38:40.493 in Ping 
09:38:45.497 in Ping 
09:38:45.677 rpc 32 server.reportClientActivity
09:38:46.677 fetch POST /api/observability/v1/traces
09:38:46.677 fetched POST /api/observability/v1/traces 204
09:38:50.502 in Ping 
09:38:55.505 in Ping 
09:39:00.509 in Ping 
09:39:00.678 in Interrupt 32
09:39:01.679 fetch POST /api/observability/v1/traces
09:39:01.679 fetched POST /api/observability/v1/traces 204
09:39:05.513 in Ping 
09:39:06.427 chunk 16 39904 bytes
09:39:06.430 in Ack 16
09:39:06.677 chunk 16 40334 bytes
09:39:06.682 in Ack 16
09:39:06.976 chunk 16 40366 bytes
09:39:06.978 in Ack 16
09:39:07.229 chunk 16 40408 bytes
09:39:07.233 in Ack 16
09:39:07.431 fetch POST /api/observability/v1/traces
09:39:07.431 fetched POST /api/observability/v1/traces 204
09:39:07.480 chunk 16 40858 bytes
09:39:07.481 in Ack 16
09:39:07.783 chunk 16 40948 bytes
09:39:07.784 in Ack 16
09:39:08.483 fetch POST /api/observability/v1/traces
09:39:08.483 fetched POST /api/observability/v1/traces 204
09:39:09.746 chunk 16 41950 bytes
09:39:09.748 in Ack 16
09:39:09.928 rpc 33 orchestration.dispatchCommand runtime-request.respond
09:39:10.517 in Ping 
09:39:10.671 rpc 34 server.reportClientActivity
09:39:10.749 fetch POST /api/observability/v1/traces
09:39:10.749 fetched POST /api/observability/v1/traces 204
09:39:11.370 exit 29 {"_tag":"Success","value":null}
09:39:11.573 exit 33 {"_tag":"Success","value":{"sequence":203}}
09:39:12.421 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:39:12.574 fetch POST /api/observability/v1/traces
09:39:12.574 fetched POST /api/observability/v1/traces 204
09:39:13.521 chunk 16 41993 bytes
09:39:13.523 in Ack 16
09:39:14.371 chunk 16 42043 bytes
09:39:14.373 in Ack 16
09:39:14.524 fetch POST /api/observability/v1/traces
09:39:14.524 fetched POST /api/observability/v1/traces 204
09:39:14.653 chunk 6 4680 bytes
09:39:14.654 exit 35 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:39:14.654 in Ack 6
09:39:14.667 rpc 36 orchestration.dispatchCommand thread.visit
09:39:14.667 exit 36 {"_tag":"Success","value":{"sequence":0}}
09:39:15.521 in Ping 
09:39:15.656 fetch POST /api/observability/v1/traces
09:39:15.656 fetched POST /api/observability/v1/traces 204
09:39:16.737 exit 30 {"_tag":"Success","value":null}
09:39:16.895 chunk 16 44904 bytes
09:39:16.897 in Ack 16
09:39:17.482 fetch POST /api/observability/v1/traces
09:39:17.482 fetched POST /api/observability/v1/traces 204
09:39:19.259 chunk 16 47893 bytes
09:39:19.261 in Ack 16
09:39:19.509 chunk 16 51675 bytes
09:39:19.512 in Ack 16
09:39:20.010 chunk 16 51627 bytes
09:39:20.011 in Ack 16
09:39:20.263 fetch POST /api/observability/v1/traces
09:39:20.263 fetched POST /api/observability/v1/traces 204
09:39:20.526 in Ping 
09:39:21.011 chunk 6 4680 bytes
09:39:21.012 in Ack 6
09:39:21.512 chunk 16 51627 bytes
09:39:21.514 in Ack 16
09:39:22.020 fetch POST /api/observability/v1/traces
09:39:22.020 fetched POST /api/observability/v1/traces 204
09:39:22.513 chunk 16 52582 bytes
09:39:22.515 in Ack 16
09:39:23.516 fetch POST /api/observability/v1/traces
09:39:23.516 fetched POST /api/observability/v1/traces 204
09:39:23.767 chunk 16 54699 bytes
09:39:23.770 in Ack 16
09:39:23.785 in Interrupt 26
09:39:23.828 rpc 37 subscribeVcsStatus
09:39:24.029 chunk 16 55683 bytes
09:39:24.033 in Ack 16
09:39:24.529 chunk 16 57076 bytes
09:39:24.532 in Ack 16
09:39:24.660 rpc 38 orchestration.dispatchCommand thread.visit
09:39:24.660 exit 38 {"_tag":"Success","value":{"sequence":0}}
09:39:24.781 fetch POST /api/observability/v1/traces
09:39:24.781 fetched POST /api/observability/v1/traces 204
09:39:25.367 chunk 37 1713 bytes
09:39:25.368 in Ack 37
09:39:25.529 in Ping 
09:39:25.533 chunk 16 59110 bytes
09:39:25.535 in Ack 16
09:39:25.696 in Interrupt 34
09:39:26.336 rpc 39 server.reportClientActivity
