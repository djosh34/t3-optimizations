# story, webapp, warm start, seed 32

Profile story (graded): seed 32's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 107 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 3.7s |
| Prompt reached the agent | 3.4s |
| Time to first text | 15.6s |
| Whole reply shown | 105.0s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 45.9s / 6.5s / 40.5s |
| Longest silence | 38.8s |
| Approval shown after it was asked | 25.0s |
| Bytes on the wire down / up | 494616 / 213219 |
| Bytes the app received / sent | 254920 / 81900 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 6.5 | 0 | 262 |
| 20 | 6.5 | 397 | 659 |
| 30 | 16.5 | 397 | 1065 |
| 40 | 26.5 | 397 | 1077 |
| 50 | 36.5 | 397 | 1077 |
| 60 | 45.9 | 424 | 1077 |
| 70 | 0.0 | 1077 | 1077 |
| 80 | 0.0 | 1077 | 1077 |
| 90 | 2.1 | 1077 | 1163 |
| 100 | 4.4 | 1378 | 1532 |
| 104 | 8.3 | 1379 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","nodes":"809/809 1016/1016 809/809 1016/1016","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217101995,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":6823.208666918735,"deliveredRate":3316.155767533732,"loss":0,"srttMs":5.612925836894105,"minRttMs":1,"sentPackets":137,"sentWireBytes":37423,"receivedPackets":187,"receivedWireBytes":104365,"lastHeardMs":16487,"unsyncedBytes":736,"inflightPackets":13,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt server-has
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217112139,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":1705.8021667296837,"deliveredRate":775.3424657534247,"loss":0.08333333333333337,"srttMs":3246.042287780227,"minRttMs":1,"sentPackets":192,"sentWireBytes":53107,"receivedPackets":295,"receivedWireBytes":180281,"lastHeardMs":26631,"unsyncedBytes":624,"inflightPackets":24,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"505/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217122280,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":2000,"deliveredRate":761.4718614718614,"loss":0.3529411764705882,"srttMs":2440.8404457893866,"minRttMs":2214,"sentPackets":255,"sentWireBytes":67754,"receivedPackets":406,"receivedWireBytes":231017,"lastHeardMs":36772,"unsyncedBytes":0,"inflightPackets":16,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217128326,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":2000,"deliveredRate":638.5965106793491,"loss":0.35547700534759347,"srttMs":2381.7985620948043,"minRttMs":2214,"sentPackets":289,"sentWireBytes":72299,"receivedPackets":468,"receivedWireBytes":250167,"lastHeardMs":42817,"unsyncedBytes":0,"inflightPackets":12,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217128326,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.35547700534759347,"srttMs":2381.7985620948043,"minRttMs":2214,"sentPackets":310,"sentWireBytes":77710,"receivedPackets":468,"receivedWireBytes":250167,"lastHeardMs":42817,"unsyncedBytes":457,"inflightPackets":30,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217128326,"silences":0,"longestSilenceMs":1028,"longestRecoveredGapMs":1028},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.35547700534759347,"srttMs":2381.7985620948043,"minRttMs":2214,"sentPackets":334,"sentWireBytes":84524,"receivedPackets":468,"receivedWireBytes":250167,"lastHeardMs":42817,"unsyncedBytes":457,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068 303/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217162304,"silences":2,"longestSilenceMs":27779,"longestRecoveredGapMs":27779},"stats":{"sendRate":2000,"deliveredRate":221.5020968356843,"loss":0.9365079365079365,"srttMs":2765.5737418329536,"minRttMs":2214,"sentPackets":356,"sentWireBytes":93501,"receivedPackets":479,"receivedWireBytes":255017,"lastHeardMs":76795,"unsyncedBytes":1113,"inflightPackets":72,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1542/1542 631/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217173757,"silences":4,"longestSilenceMs":27779,"longestRecoveredGapMs":27779},"stats":{"sendRate":2000,"deliveredRate":221.5020968356843,"loss":0.9365079365079365,"srttMs":2765.5737418329536,"minRttMs":2214,"sentPackets":358,"sentWireBytes":94663,"receivedPackets":497,"receivedWireBytes":263905,"lastHeardMs":88249,"unsyncedBytes":1113,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1542/1542 631/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217184583,"silences":4,"longestSilenceMs":27779,"longestRecoveredGapMs":27779},"stats":{"sendRate":2000,"deliveredRate":221.5020968356843,"loss":0.9365079365079365,"srttMs":2765.5737418329536,"minRttMs":2214,"sentPackets":358,"sentWireBytes":94663,"receivedPackets":554,"receivedWireBytes":289590,"lastHeardMs":99074,"unsyncedBytes":1113,"inflightPackets":74,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1068/1068 1542/1542 631/923","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217194113,"silences":4,"longestSilenceMs":27779,"longestRecoveredGapMs":27779},"stats":{"sendRate":1500,"deliveredRate":163.93401846237535,"loss":0.901010101010101,"srttMs":5829.720521090855,"minRttMs":3467,"sentPackets":375,"sentWireBytes":102503,"receivedPackets":585,"receivedWireBytes":305904,"lastHeardMs":108605,"unsyncedBytes":855,"inflightPackets":87,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 860/860 1093/1093 870/870 1299/1299 1093/1093 1058/1058 849/849 1093/1093 1542/1542 200/947 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340 723/723","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","nodes":"809/809 1016/1016 809/809 1016/1016 785/785","providerSessions":"2653/2653","messages":"544/544 456/456 544/544 901/901 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":3588,"lastDataAt":1794217205757,"silences":4,"longestSilenceMs":27779,"longestRecoveredGapMs":27779},"stats":{"sendRate":1696.260142395416,"deliveredRate":570.6796101179872,"loss":0.4738026418026417,"srttMs":5418.321944973092,"minRttMs":3467,"sentPackets":412,"sentWireBytes":114817,"receivedPackets":660,"receivedWireBytes":341648,"lastHeardMs":120248,"unsyncedBytes":0,"inflightPackets":99,"duplicateBytes":0}}
- shim trace
09:38:06.379 fetch GET /api/auth/session
09:38:06.382 fetched GET /api/auth/session 200
09:38:06.738 fetch GET /.well-known/t3/environment
09:38:06.739 fetched GET /.well-known/t3/environment 200
09:38:06.754 fetch GET /.well-known/t3/environment
09:38:06.755 fetched GET /.well-known/t3/environment 200
09:38:06.763 rpc 0 subscribeServerConfig
09:38:06.764 chunk 0 38462 bytes
09:38:06.806 in Ack 0
09:38:06.808 rpc 1 subscribeServerLifecycle
09:38:06.809 rpc 2 subscribeServerLifecycle
09:38:06.813 fetch GET /api/auth/session
09:38:06.814 fetched GET /api/auth/session 200
09:38:06.815 rpc 3 subscribeDeviceState
09:38:06.817 rpc 4 preview.reportProfiles
09:38:06.884 rpc 5 subscribeProjectClones
09:38:06.892 fetch GET /api/orchestration/threads/mcp%3A8e7e565c-323b-49d4-b158-dd0c994047a0/bounded
09:38:06.922 fetch GET /api/orchestration/shell
09:38:06.923 fetched GET /api/orchestration/shell 200
09:38:06.927 rpc 6 orchestration.subscribeShell
09:38:06.927 chunk 6 2736 bytes
09:38:06.983 rpc 7 server.refreshProviders
09:38:07.048 rpc 8 orchestration.dispatchCommand thread.visit
09:38:07.048 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:38:07.080 rpc 9 subscribeTerminalMetadata
09:38:07.081 rpc 10 subscribeVcsStatus
09:38:07.083 rpc 11 subscribePreviewEvents
09:38:07.084 rpc 12 assets.createUrl
09:38:07.085 rpc 13 preview.list
09:38:07.086 rpc 14 projects.readFile
09:38:07.093 in Ack 6
09:38:07.426 rpc 15 server.reportClientActivity
09:38:07.734 fetch POST /api/observability/v1/traces
09:38:07.734 fetched POST /api/observability/v1/traces 204
09:38:09.555 chunk 6 2712 bytes
09:38:09.556 in Ack 6
09:38:10.037 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"d4b00d02-eba4-4b4c-8d92-d8755c126b04","sessions":[]}}
09:38:10.038 chunk 1 3815 bytes
09:38:10.044 in Ack 1
09:38:10.279 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:38:10.279 chunk 2 3815 bytes
09:38:10.280 in Ack 2
09:38:10.508 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:40.499Z","configurableProviders":false,"con
09:38:10.508 chunk 10 1632 bytes
09:38:10.508 exit 12 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:38:10.512 in Ack 10
09:38:10.558 fetch POST /api/observability/v1/traces
09:38:10.558 fetched POST /api/observability/v1/traces 204
09:38:11.008 chunk 6 4635 bytes
09:38:11.009 in Ack 6
09:38:11.385 chunk 3 876 bytes
09:38:11.385 exit 4 {"_tag":"Success","value":null}
09:38:11.385 chunk 5 4 bytes
09:38:11.385 chunk 9 36 bytes
09:38:11.386 in Ack 3
09:38:11.391 rpc 16 preview.reportProfiles
09:38:11.392 in Ack 5
09:38:11.392 in Ack 9
09:38:11.467 exit 15 {"_tag":"Success","value":null}
09:38:11.467 exit 16 {"_tag":"Success","value":null}
09:38:11.757 in Ping 
09:38:12.010 fetch POST /api/observability/v1/traces
09:38:12.010 fetched POST /api/observability/v1/traces 204
09:38:12.821 fetched GET /api/orchestration/threads/mcp%3A8e7e565c-323b-49d4-b158-dd0c994047a0/bounded 200
09:38:12.827 rpc 17 orchestration.subscribeThread
09:38:12.827 chunk 17 8283 bytes
09:38:12.844 in Interrupt 10
09:38:12.847 rpc 18 subscribeVcsStatus
09:38:12.848 in Ack 17
09:38:12.978 chunk 18 1632 bytes
09:38:12.979 in Ack 18
09:38:13.078 chunk 17 8323 bytes
09:38:13.080 in Ack 17
09:38:13.133 rpc 19 server.reportClientActivity
09:38:13.339 chunk 17 11919 bytes
09:38:13.340 in Ack 17
09:38:13.394 exit 19 {"_tag":"Success","value":null}
09:38:13.589 chunk 17 13498 bytes
09:38:13.590 in Ack 17
09:38:13.799 rpc 20 attachments.createUploadUrl
09:38:13.848 fetch POST /api/observability/v1/traces
09:38:13.848 fetched POST /api/observability/v1/traces 204
09:38:13.871 chunk 17 17167 bytes
09:38:13.876 in Ack 17
09:38:13.930 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-de3da0fa-c0bf-4596-ab09-08abad74177e","expiresAt":1794217693890,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:38:13.933 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWRlM2RhMGZhLWMwYmYtNDU5Ni1hYjA5LTA4YWJhZDc0MTc3ZSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY5Mzg5MH0.NqgzsDKdtgKsZ6AhI3e_hyFw8eg8h83fdsq0IrOlOek
09:38:14.121 chunk 17 19020 bytes
09:38:14.123 in Ack 17
09:38:14.374 chunk 17 21100 bytes
09:38:14.385 in Ack 17
09:38:14.395 in Interrupt 18
09:38:14.398 rpc 21 subscribeVcsStatus
09:38:14.554 chunk 21 1632 bytes
09:38:14.556 in Ack 21
09:38:14.649 rpc 22 server.reportClientActivity
09:38:14.877 fetch POST /api/observability/v1/traces
09:38:14.877 fetched POST /api/observability/v1/traces 204
09:38:15.923 exit 22 {"_tag":"Success","value":null}
09:38:16.730 rpc 23 orchestration.dispatchCommand message.dispatch
09:38:16.980 in Ping 
09:38:16.982 fetch POST /api/observability/v1/traces
09:38:16.982 fetched POST /api/observability/v1/traces 204
09:38:17.032 exit 23 {"_tag":"Success","value":{"sequence":110}}
09:38:17.034 rpc 24 attachments.delete
09:38:17.230 chunk 6 4721 bytes
09:38:17.230 chunk 17 21736 bytes
09:38:17.231 in Ack 6
09:38:17.233 rpc 25 orchestration.dispatchCommand thread.visit
09:38:17.233 exit 25 {"_tag":"Success","value":{"sequence":0}}
09:38:17.264 in Ack 17
09:38:17.375 exit 24 {"_tag":"Success","value":null}
09:38:17.480 chunk 6 4721 bytes
09:38:17.480 chunk 17 26024 bytes
09:38:17.481 in Ack 6
09:38:17.489 in Ack 17
09:38:17.491 rpc 26 assets.createUrl
09:38:17.733 chunk 17 29267 bytes
09:38:17.735 in Ack 17
09:38:17.981 chunk 6 4660 bytes
09:38:17.982 in Ack 6
09:38:17.998 chunk 17 29291 bytes
09:38:18.002 in Ack 17
09:38:18.034 fetch POST /api/observability/v1/traces
09:38:18.034 fetched POST /api/observability/v1/traces 204
09:38:18.247 chunk 17 31903 bytes
09:38:18.249 in Ack 17
09:38:18.497 chunk 17 31903 bytes
09:38:18.498 in Ack 17
09:38:18.746 chunk 17 32035 bytes
09:38:18.748 in Ack 17
09:38:18.985 chunk 6 4657 bytes
09:38:18.986 in Ack 6
09:38:18.991 rpc 27 orchestration.dispatchCommand thread.visit
09:38:18.991 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:38:19.012 chunk 17 35260 bytes
09:38:19.019 in Ack 17
09:38:19.036 in Interrupt 21
09:38:19.040 rpc 28 subscribeVcsStatus
09:38:19.078 chunk 28 1713 bytes
09:38:19.078 in Ack 28
09:38:19.280 chunk 6 4657 bytes
09:38:19.281 fetch POST /api/observability/v1/traces
09:38:19.281 fetched POST /api/observability/v1/traces 204
09:38:19.282 chunk 17 35260 bytes
09:38:19.610 in Ack 6
09:38:19.618 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/7fe40513-c5e0-4551-ab7c-59ae16e80c1d","expiresAt":1794220699618}}
09:38:19.620 in Ack 17
09:38:19.630 rpc 29 server.reportClientActivity
09:38:19.670 exit 29 {"_tag":"Success","value":null}
09:38:20.538 rpc 30 server.refreshProviders
09:38:20.538 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:40.499Z","configurableProviders":false,"con
09:38:20.613 fetch POST /api/observability/v1/traces
09:38:20.613 fetched POST /api/observability/v1/traces 204
09:38:21.946 rpc 31 orchestration.dispatchCommand message.dispatch
09:38:22.000 in Ping 
09:38:22.948 fetch POST /api/observability/v1/traces
09:38:22.948 fetched POST /api/observability/v1/traces 204
09:38:23.865 chunk 6 4701 bytes
09:38:23.868 in Ack 6
09:38:24.118 chunk 6 4721 bytes
09:38:24.120 in Ack 6
09:38:24.408 fetch POST /api/observability/v1/traces
09:38:24.408 fetched POST /api/observability/v1/traces 204
09:38:25.239 exit 31 {"_tag":"Success","value":{"sequence":153}}
09:38:25.488 chunk 6 4765 bytes
09:38:25.488 in Ack 6
09:38:25.747 chunk 17 38684 bytes
09:38:25.748 in Ack 17
09:38:26.002 chunk 17 39973 bytes
09:38:26.004 in Ack 17
09:38:26.249 fetch POST /api/observability/v1/traces
09:38:26.249 fetched POST /api/observability/v1/traces 204
09:38:26.250 chunk 17 39973 bytes
09:38:26.252 in Ack 17
09:38:26.501 chunk 17 40025 bytes
09:38:26.503 in Ack 17
09:38:26.739 chunk 6 4785 bytes
09:38:26.740 in Ack 6
09:38:26.751 chunk 17 40046 bytes
09:38:26.753 in Ack 17
09:38:27.001 in Ping 
09:38:27.002 chunk 17 40723 bytes
09:38:27.004 in Ack 17
09:38:27.254 chunk 17 40725 bytes
09:38:27.254 fetch POST /api/observability/v1/traces
09:38:27.254 fetched POST /api/observability/v1/traces 204
09:38:27.259 in Ack 17
09:38:28.260 fetch POST /api/observability/v1/traces
09:38:28.260 fetched POST /api/observability/v1/traces 204
09:38:28.994 rpc 32 orchestration.dispatchCommand thread.visit
09:38:28.994 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:38:29.233 chunk 6 4785 bytes
09:38:29.234 in Ack 6
09:38:29.995 fetch POST /api/observability/v1/traces
09:38:29.995 fetched POST /api/observability/v1/traces 204
09:38:31.995 rpc 33 server.reportClientActivity
09:38:32.001 in Ping 
09:38:32.546 chunk 17 40725 bytes
09:38:32.548 in Ack 17
09:38:32.797 chunk 6 4785 bytes
09:38:32.798 in Ack 6
09:38:32.995 fetch POST /api/observability/v1/traces
09:38:32.995 fetched POST /api/observability/v1/traces 204
09:38:34.779 chunk 6 4785 bytes
09:38:34.779 in Ack 6
09:38:35.697 exit 33 {"_tag":"Success","value":null}
09:38:35.781 fetch POST /api/observability/v1/traces
09:38:35.781 fetched POST /api/observability/v1/traces 204
09:38:36.965 chunk 17 43672 bytes
09:38:36.967 in Ack 17
09:38:37.053 in Ping 
09:38:37.215 chunk 17 44138 bytes
09:38:37.216 in Ack 17
09:38:37.717 chunk 17 44090 bytes
09:38:37.720 in Ack 17
09:38:37.967 chunk 6 4785 bytes
09:38:37.968 in Ack 6
09:38:37.971 fetch POST /api/observability/v1/traces
09:38:37.971 fetched POST /api/observability/v1/traces 204
09:38:38.467 chunk 17 45134 bytes
09:38:38.469 in Ack 17
09:38:38.994 rpc 34 orchestration.dispatchCommand thread.visit
09:38:38.994 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:38:39.471 fetch POST /api/observability/v1/traces
09:38:39.472 fetched POST /api/observability/v1/traces 204
09:38:41.337 chunk 6 4785 bytes
09:38:41.338 in Ack 6
09:38:42.054 in Ping 
09:38:42.339 fetch POST /api/observability/v1/traces
09:38:42.339 fetched POST /api/observability/v1/traces 204
09:38:44.597 chunk 6 4785 bytes
09:38:44.597 in Ack 6
09:38:44.847 chunk 17 45134 bytes
09:38:44.849 in Ack 17
09:38:45.599 fetch POST /api/observability/v1/traces
09:38:45.599 fetched POST /api/observability/v1/traces 204
09:38:47.054 in Ping 
09:38:48.604 fetch POST /api/observability/v1/traces
09:38:48.604 fetched POST /api/observability/v1/traces 204
09:38:52.055 in Ping 
09:38:56.996 rpc 35 server.reportClientActivity
09:38:57.057 in Ping 
09:38:57.997 fetch POST /api/observability/v1/traces
09:38:57.997 fetched POST /api/observability/v1/traces 204
09:39:02.058 in Ping 
09:39:07.062 in Ping 
09:39:11.997 in Interrupt 35
09:39:12.066 in Ping 
09:39:12.999 fetch POST /api/observability/v1/traces
09:39:12.999 fetched POST /api/observability/v1/traces 204
09:39:16.107 chunk 17 45227 bytes
09:39:16.110 in Ack 17
09:39:16.999 chunk 17 46229 bytes
09:39:17.004 in Ack 17
09:39:17.109 in Ping 
09:39:17.111 fetch POST /api/observability/v1/traces
09:39:17.111 fetched POST /api/observability/v1/traces 204
09:39:17.426 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:39:18.426 fetch POST /api/observability/v1/traces
09:39:18.426 fetched POST /api/observability/v1/traces 204
09:39:21.774 chunk 17 46808 bytes
09:39:21.776 in Ack 17
09:39:21.997 rpc 37 server.reportClientActivity
09:39:22.026 chunk 17 47522 bytes
09:39:22.028 in Ack 17
09:39:22.110 in Ping 
09:39:22.275 chunk 17 48668 bytes
09:39:22.277 in Ack 17
09:39:22.777 fetch POST /api/observability/v1/traces
09:39:22.777 fetched POST /api/observability/v1/traces 204
09:39:27.114 in Ping 
09:39:27.304 chunk 17 52155 bytes
09:39:27.306 in Ack 17
09:39:28.307 fetch POST /api/observability/v1/traces
09:39:28.307 fetched POST /api/observability/v1/traces 204
09:39:32.118 in Ping 
09:39:36.997 in Interrupt 37
09:39:37.124 in Ping 
09:39:37.998 fetch POST /api/observability/v1/traces
09:39:37.998 fetched POST /api/observability/v1/traces 204
09:39:42.126 in Ping 
09:39:46.996 rpc 38 server.reportClientActivity
09:39:47.130 in Ping 
09:39:47.996 fetch POST /api/observability/v1/traces
09:39:47.996 fetched POST /api/observability/v1/traces 204
09:39:52.134 in Ping 
09:39:52.904 exit 36 {"_tag":"Success","value":{"sequence":188}}
09:39:53.278 rpc 39 orchestration.dispatchCommand runtime-request.respond
09:39:53.905 fetch POST /api/observability/v1/traces
09:39:53.905 fetched POST /api/observability/v1/traces 204
09:39:54.181 chunk 17 52198 bytes
09:39:54.183 in Ack 17
09:39:55.184 fetch POST /api/observability/v1/traces
09:39:55.184 fetched POST /api/observability/v1/traces 204
09:39:57.139 in Ping 
09:39:58.993 chunk 17 52248 bytes
09:39:58.995 in Ack 17
09:39:59.035 exit 39 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:39:59.494 chunk 6 4787 bytes
09:39:59.494 in Ack 6
09:39:59.497 rpc 40 orchestration.dispatchCommand thread.visit
09:39:59.497 exit 40 {"_tag":"Success","value":{"sequence":0}}
09:39:59.997 fetch POST /api/observability/v1/traces
09:39:59.997 fetched POST /api/observability/v1/traces 204
09:40:00.084 chunk 17 55242 bytes
09:40:00.086 in Ack 17
09:40:00.450 chunk 17 55484 bytes
09:40:00.452 exit 35 {"_tag":"Success","value":null}
09:40:00.456 in Ack 17
09:40:01.088 fetch POST /api/observability/v1/traces
09:40:01.088 fetched POST /api/observability/v1/traces 204
09:40:01.893 chunk 17 55436 bytes
09:40:01.895 in Ack 17
09:40:01.998 in Interrupt 38
09:40:02.142 in Ping 
09:40:02.898 fetch POST /api/observability/v1/traces
09:40:02.898 fetched POST /api/observability/v1/traces 204
09:40:05.111 chunk 6 4787 bytes
09:40:05.113 in Ack 6
09:40:05.462 chunk 17 55860 bytes
09:40:05.474 in Ack 17
09:40:05.720 chunk 17 55992 bytes
09:40:05.724 in Ack 17
09:40:06.038 chunk 17 56114 bytes
09:40:06.044 in Ack 17
09:40:06.278 fetch POST /api/observability/v1/traces
09:40:06.278 fetched POST /api/observability/v1/traces 204
09:40:07.210 in Ping 
