# case70, webapp, warm start, seed 3

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 151 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 48.4s |
| Prompt reached the agent | 47.1s |
| Time to first text | 52.7s |
| Whole reply shown | 149.8s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 25.2s / 0.0s / 18.2s |
| Longest silence | 52.7s |
| Approval shown after it was asked | 45.5s |
| Bytes on the wire down / up | 144440 / 87221 |
| Bytes the app received / sent | 109511 / 224181 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.0 | 0 | 0 |
| 40 | 0.0 | 0 | 0 |
| 50 | 2.8 | 0 | 117 |
| 60 | 5.9 | 279 | 519 |
| 70 | 13.7 | 367 | 922 |
| 80 | 8.1 | 992 | 1077 |
| 90 | 0.0 | 1077 | 1077 |
| 100 | 0.0 | 1077 | 1077 |
| 110 | 0.0 | 1077 | 1077 |
| 120 | 0.0 | 1077 | 1077 |
| 130 | 6.9 | 1158 | 1425 |
| 140 | 16.2 | 1185 | 1596 |
| 149 | 25.2 | 1185 | 1596 |

## Driver notes

- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778","providerThreads":"810/810","assistant":"32/32 21/21","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 502/901","nodes":"1025/1025 809/809 809/809","checkpoints":"779/779 613/613 779/779","checkpointScopes":"734/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152151489,"silences":0,"longestSilenceMs":1306,"longestRecoveredGapMs":1306},"stats":{"sendRate":10125,"deliveredRate":3575.1745947435193,"loss":0,"srttMs":34.50744761899873,"minRttMs":2,"sentPackets":132,"sentWireBytes":42814,"receivedPackets":118,"receivedWireBytes":57679,"lastHeardMs":14063,"unsyncedBytes":1186,"inflightPackets":25,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152161549,"silences":0,"longestSilenceMs":1306,"longestRecoveredGapMs":1306},"stats":{"sendRate":5062.5,"deliveredRate":2769.339368914178,"loss":0.15714285714285714,"srttMs":536.0603895832959,"minRttMs":2,"sentPackets":140,"sentWireBytes":48068,"receivedPackets":130,"receivedWireBytes":68977,"lastHeardMs":24123,"unsyncedBytes":1009,"inflightPackets":31,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152168510,"silences":0,"longestSilenceMs":1426,"longestRecoveredGapMs":1426},"stats":{"sendRate":5062.5,"deliveredRate":2769.339368914178,"loss":0.15714285714285714,"srttMs":536.0603895832959,"minRttMs":2,"sentPackets":148,"sentWireBytes":51896,"receivedPackets":137,"receivedWireBytes":76747,"lastHeardMs":31084,"unsyncedBytes":1509,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152178820,"silences":2,"longestSilenceMs":3556,"longestRecoveredGapMs":3556},"stats":{"sendRate":5062.5,"deliveredRate":2769.339368914178,"loss":0.15714285714285714,"srttMs":536.0603895832959,"minRttMs":2,"sentPackets":154,"sentWireBytes":55211,"receivedPackets":142,"receivedWireBytes":82302,"lastHeardMs":41394,"unsyncedBytes":1509,"inflightPackets":45,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152190504,"silences":3,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":5062.5,"deliveredRate":2769.339368914178,"loss":0.15714285714285714,"srttMs":536.0603895832959,"minRttMs":2,"sentPackets":166,"sentWireBytes":61841,"receivedPackets":144,"receivedWireBytes":84524,"lastHeardMs":53078,"unsyncedBytes":2009,"inflightPackets":57,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 161/849","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 25/75","attempts":"662/662 662/662","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152200748,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":2531.25,"deliveredRate":1943.038215270148,"loss":0.40444444444444444,"srttMs":773.4278408853838,"minRttMs":2,"sentPackets":202,"sentWireBytes":76511,"receivedPackets":152,"receivedWireBytes":91578,"lastHeardMs":63322,"unsyncedBytes":1271,"inflightPackets":92,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 948/1083","user":"10/10 9/9","runs":"1014/1014 971/971","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 425/455","attempts":"662/662 662/662 45/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152212065,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":4277.8956417291065,"deliveredRate":1179.8258504200967,"loss":0.5833463656267104,"srttMs":2782.9906745552908,"minRttMs":1525,"sentPackets":289,"sentWireBytes":100706,"receivedPackets":183,"receivedWireBytes":101324,"lastHeardMs":74639,"unsyncedBytes":449,"inflightPackets":133,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083","user":"10/10 9/9","runs":"1014/1014 971/971 301/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 904/998","attempts":"662/662 662/662 338/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152222083,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":2138.9478208645533,"deliveredRate":719.9590873713886,"loss":0.744495181341962,"srttMs":4148.321693946862,"minRttMs":1525,"sentPackets":355,"sentWireBytes":119246,"receivedPackets":216,"receivedWireBytes":111771,"lastHeardMs":84657,"unsyncedBytes":645,"inflightPackets":184,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1083/1083 648/1554","user":"10/10 9/9","runs":"1014/1014 971/971 301/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1295/1340","attempts":"662/662 662/662 338/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152230961,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":2937.0869092096036,"deliveredRate":620.7406591001368,"loss":0.7358178876347995,"srttMs":2943.2963906170257,"minRttMs":1525,"sentPackets":425,"sentWireBytes":137211,"receivedPackets":241,"receivedWireBytes":120064,"lastHeardMs":93535,"unsyncedBytes":0,"inflightPackets":209,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 530/1108 1554/1554 838/932","user":"10/10 9/9","runs":"1014/1014 971/971 301/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340","attempts":"662/662 662/662 338/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152241995,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0.6864079077981946,"srttMs":2738.2593417898975,"minRttMs":1303,"sentPackets":483,"sentWireBytes":148053,"receivedPackets":267,"receivedWireBytes":128846,"lastHeardMs":104569,"unsyncedBytes":500,"inflightPackets":217,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1554/1554 932/932","user":"10/10 9/9","runs":"1014/1014 971/971 301/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340","attempts":"662/662 662/662 338/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734","runtimeRequests":"72/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152252390,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":2000,"deliveredRate":362.9449275362319,"loss":0.6984848484848485,"srttMs":1867.1200146501378,"minRttMs":1081,"sentPackets":547,"sentWireBytes":162908,"receivedPackets":295,"receivedWireBytes":137147,"lastHeardMs":114964,"unsyncedBytes":0,"inflightPackets":238,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1554/1554 932/932","user":"10/10 9/9","runs":"1014/1014 971/971 301/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340","attempts":"662/662 662/662 338/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734","runtimeRequests":"619/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152262362,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":3000,"deliveredRate":189.1385767790262,"loss":0.8064516129032258,"srttMs":1726.8575112165117,"minRttMs":1081,"sentPackets":605,"sentWireBytes":173495,"receivedPackets":318,"receivedWireBytes":143987,"lastHeardMs":124936,"unsyncedBytes":0,"inflightPackets":216,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1554/1554 932/932","user":"10/10 9/9","runs":"1014/1014 971/971 616/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340","attempts":"662/662 662/662 574/638","providerSessions":"2696/2696","messages":"456/456 550/550 620/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734","runtimeRequests":"1011/1011"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152273525,"silences":4,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":3000,"deliveredRate":618.3608863876187,"loss":0.6666666666666667,"srttMs":1738.7470628546619,"minRttMs":1081,"sentPackets":683,"sentWireBytes":198525,"receivedPackets":348,"receivedWireBytes":153233,"lastHeardMs":136099,"unsyncedBytes":482,"inflightPackets":197,"duplicateBytes":0}}
- Scroll to end shows
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1554/1554 932/932 1083/1083","user":"10/10 9/9","runs":"1014/1014 971/971 772/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340 338/509","attempts":"662/662 662/662 638/638","providerSessions":"2696/2696","messages":"456/456 550/550 662/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734","runtimeRequests":"242/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152282745,"silences":6,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":3601.6453164882623,"deliveredRate":771.6858576253555,"loss":0.678611111111111,"srttMs":2074.0508214372494,"minRttMs":1081,"sentPackets":758,"sentWireBytes":223160,"receivedPackets":357,"receivedWireBytes":157942,"lastHeardMs":145319,"unsyncedBytes":0,"inflightPackets":180,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","frame":"1032/1032","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058 849/849 1108/1108 1554/1554 932/932 1108/1108","user":"10/10 9/9","runs":"1014/1014 971/971 772/839","providerTurns":"778/778 778/778","providerThreads":"731/810","assistant":"32/32 21/21 1340/1340 713/723","attempts":"662/662 662/662 638/638","providerSessions":"2696/2696","messages":"456/456 550/550 662/901 550/550","nodes":"1025/1025 809/809 809/809","checkpoints":"613/613 613/613 779/779","checkpointScopes":"95/734","runtimeRequests":"362/1054"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":3131,"lastDataAt":1794152295261,"silences":7,"longestSilenceMs":10016,"longestRecoveredGapMs":10016},"stats":{"sendRate":3000,"deliveredRate":0,"loss":0.6850277777777777,"srttMs":1919.0837456715908,"minRttMs":1081,"sentPackets":833,"sentWireBytes":240875,"receivedPackets":377,"receivedWireBytes":164126,"lastHeardMs":157835,"unsyncedBytes":155,"inflightPackets":186,"duplicateBytes":0}}
- shim trace
15:35:39.144 fetch GET /api/auth/session
15:35:39.158 fetched GET /api/auth/session 200
15:35:39.387 fetch GET /.well-known/t3/environment
15:35:39.389 fetched GET /.well-known/t3/environment 200
15:35:39.451 fetch GET /.well-known/t3/environment
15:35:39.457 fetched GET /.well-known/t3/environment 200
15:35:39.480 rpc 0 subscribeServerConfig
15:35:39.481 chunk 0 38722 bytes
15:35:39.547 in Ack 0
15:35:39.550 rpc 1 subscribeServerLifecycle
15:35:39.553 rpc 2 subscribeServerLifecycle
15:35:39.567 fetch GET /api/auth/session
15:35:39.569 fetched GET /api/auth/session 200
15:35:39.570 rpc 3 subscribeDeviceState
15:35:39.573 rpc 4 preview.reportProfiles
15:35:39.687 rpc 5 server.reportClientActivity
15:35:39.844 rpc 6 subscribeProjectClones
15:35:39.898 fetch GET /api/orchestration/threads/mcp%3A92f22172-a74e-4161-9bd7-09c4c686e895/bounded
15:35:39.905 fetch GET /api/orchestration/shell
15:35:39.964 rpc 7 server.refreshProviders
15:35:40.124 rpc 8 orchestration.dispatchCommand thread.visit
15:35:40.222 fetched GET /api/orchestration/shell 200
15:35:40.223 rpc 9 subscribeTerminalMetadata
15:35:40.224 rpc 10 subscribeVcsStatus
15:35:40.225 rpc 11 subscribePreviewEvents
15:35:40.234 rpc 12 orchestration.subscribeShell
15:35:40.234 chunk 12 2790 bytes
15:35:40.245 rpc 13 assets.createUrl
15:35:40.249 rpc 14 preview.list
15:35:40.251 rpc 15 projects.readFile
15:35:40.267 in Ack 12
15:35:40.507 fetch POST /api/observability/v1/traces
15:35:40.508 fetched POST /api/observability/v1/traces 204
15:35:40.925 chunk 12 2766 bytes
15:35:40.927 in Ack 12
15:35:41.163 exit 8 {"_tag":"Success","value":{"sequence":38}}
15:35:41.413 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"1fb39ade-5dc7-4108-a5a7-58ba4486071d","sessions":[]}}
15:35:41.413 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
15:35:41.416 chunk 1 3872 bytes
15:35:41.438 in Ack 1
15:35:41.644 chunk 2 3872 bytes
15:35:41.644 in Ack 2
15:35:41.742 fetch POST /api/observability/v1/traces
15:35:41.742 fetched POST /api/observability/v1/traces 204
15:35:41.870 chunk 12 952 bytes
15:35:41.870 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:35:19.637Z","configurableProviders":false,"con
15:35:41.870 chunk 10 933 bytes
15:35:41.870 exit 13 {"_tag":"Success","value":{"expiresAt":1794155400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
15:35:41.871 in Ack 12
15:35:41.879 in Ack 10
15:35:42.358 chunk 3 876 bytes
15:35:42.358 exit 4 {"_tag":"Success","value":null}
15:35:42.358 exit 5 {"_tag":"Success","value":null}
15:35:42.358 chunk 6 4 bytes
15:35:42.358 chunk 9 36 bytes
15:35:42.359 in Ack 3
15:35:42.360 rpc 16 preview.reportProfiles
15:35:42.361 in Ack 6
15:35:42.361 in Ack 9
15:35:42.372 chunk 12 2809 bytes
15:35:42.373 in Ack 12
15:35:42.382 rpc 17 server.refreshProviders
15:35:42.383 exit 17 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T15:35:19.637Z","configurableProviders":false,"con
15:35:42.467 exit 16 {"_tag":"Success","value":null}
15:35:42.615 rpc 18 server.reportClientActivity
15:35:42.693 exit 18 {"_tag":"Success","value":null}
15:35:42.694 fetched GET /api/orchestration/threads/mcp%3A92f22172-a74e-4161-9bd7-09c4c686e895/bounded 200
15:35:42.729 rpc 19 orchestration.subscribeThread
15:35:42.730 chunk 19 10228 bytes
15:35:42.789 in Interrupt 10
15:35:42.860 rpc 20 subscribeVcsStatus
15:35:42.922 in Ack 19
15:35:42.997 fetch POST /api/observability/v1/traces
15:35:42.997 fetched POST /api/observability/v1/traces 204
15:35:42.999 chunk 19 11996 bytes
15:35:43.000 in Ack 19
15:35:43.115 chunk 20 933 bytes
15:35:43.115 in Ack 20
15:35:43.167 rpc 21 server.reportClientActivity
15:35:43.254 chunk 19 12722 bytes
15:35:43.255 in Ack 19
15:35:43.335 exit 21 {"_tag":"Success","value":null}
15:35:43.504 chunk 19 13532 bytes
15:35:43.506 in Ack 19
15:35:44.090 chunk 19 18645 bytes
15:35:44.092 fetch POST /api/observability/v1/traces
15:35:44.092 fetched POST /api/observability/v1/traces 204
15:35:44.101 in Ack 19
15:35:44.341 chunk 19 19112 bytes
15:35:44.342 in Ack 19
15:35:44.506 in Ping 
15:35:44.652 rpc 22 attachments.createUploadUrl
15:35:45.002 chunk 19 21239 bytes
15:35:45.039 in Ack 19
15:35:45.053 in Interrupt 20
15:35:45.058 rpc 23 subscribeVcsStatus
15:35:45.103 fetch POST /api/observability/v1/traces
15:35:45.103 fetched POST /api/observability/v1/traces 204
15:35:45.116 exit 22 {"_tag":"Success","value":{"attachmentId":"pending-0916ec86-99fd-40fc-b857-95e01440c534","expiresAt":1794152745100,"relativeUrl":"/api/attachments/upload/eyJ2ZX
15:35:45.116 chunk 23 933 bytes
15:35:45.120 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA5MTZlYzg2LTk5ZmQtNDBmYy1iODU3LTk1ZTAxNDQwYzUzNCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE1Mjc0NTEwMH0.sjKaYWIsFC6A24lb9AQmwqwniw3PpwmnX8_y132wj7Y
15:35:45.120 in Ack 23
15:35:45.309 rpc 24 server.reportClientActivity
15:35:46.118 fetch POST /api/observability/v1/traces
15:35:46.118 fetched POST /api/observability/v1/traces 204
15:35:46.424 exit 24 {"_tag":"Success","value":null}
15:35:46.930 rpc 25 orchestration.dispatchCommand message.dispatch
15:35:47.997 fetch POST /api/observability/v1/traces
15:35:47.997 fetched POST /api/observability/v1/traces 204
15:35:48.150 exit 25 {"_tag":"Success","value":{"sequence":45}}
15:35:48.153 rpc 26 attachments.delete
15:35:48.399 chunk 12 952 bytes
15:35:48.399 in Ack 12
15:35:48.481 exit 26 {"_tag":"Success","value":null}
15:35:48.699 chunk 12 2895 bytes
15:35:48.700 chunk 19 24671 bytes
15:35:48.701 in Ack 12
15:35:48.705 rpc 27 orchestration.dispatchCommand thread.visit
15:35:48.757 in Ack 19
15:35:48.763 rpc 28 assets.createUrl
15:35:48.868 exit 27 {"_tag":"Success","value":{"sequence":53}}
15:35:48.984 chunk 12 952 bytes
15:35:48.986 chunk 19 27099 bytes
15:35:48.988 in Ack 12
15:35:49.068 in Ack 19
15:35:49.153 fetch POST /api/observability/v1/traces
15:35:49.153 fetched POST /api/observability/v1/traces 204
15:35:49.269 chunk 19 27159 bytes
15:35:49.272 in Ack 19
15:35:49.508 in Ping 
15:35:49.520 chunk 19 27161 bytes
15:35:49.521 in Ack 19
15:35:49.777 chunk 12 2831 bytes
15:35:49.777 chunk 19 27494 bytes
15:35:49.787 in Ack 12
15:35:49.789 rpc 29 orchestration.dispatchCommand thread.visit
15:35:49.902 in Ack 19
15:35:50.024 exit 29 {"_tag":"Success","value":{"sequence":69}}
15:35:50.501 fetch POST /api/observability/v1/traces
15:35:50.501 fetched POST /api/observability/v1/traces 204
15:35:50.501 chunk 12 2831 bytes
15:35:50.502 chunk 19 32170 bytes
15:35:50.504 in Ack 12
15:35:50.512 in Ack 19
15:35:50.521 in Interrupt 23
15:35:50.525 rpc 30 subscribeVcsStatus
15:35:50.785 rpc 31 server.reportClientActivity
15:35:51.466 rpc 32 orchestration.dispatchCommand message.dispatch
15:35:51.533 fetch POST /api/observability/v1/traces
15:35:51.533 fetched POST /api/observability/v1/traces 204
15:35:52.469 chunk 19 30776 bytes
15:35:52.469 chunk 30 933 bytes
15:35:52.470 in Ack 19
15:35:52.492 in Ack 30
15:35:53.462 chunk 19 31555 bytes
15:35:53.464 in Ack 19
15:35:53.488 fetch POST /api/observability/v1/traces
15:35:53.488 fetched POST /api/observability/v1/traces 204
15:35:53.972 chunk 19 32127 bytes
15:35:53.976 in Ack 19
15:35:54.493 chunk 19 31393 bytes
15:35:54.508 in Ack 19
15:35:54.550 in Ping 
15:35:54.979 fetch POST /api/observability/v1/traces
15:35:54.979 fetched POST /api/observability/v1/traces 204
15:35:55.004 chunk 19 30583 bytes
15:35:55.005 in Ack 19
15:35:56.006 fetch POST /api/observability/v1/traces
15:35:56.006 fetched POST /api/observability/v1/traces 204
15:35:57.485 chunk 19 31197 bytes
15:35:57.486 in Ack 19
15:35:58.515 fetch POST /api/observability/v1/traces
15:35:58.515 fetched POST /api/observability/v1/traces 204
15:35:58.594 chunk 19 31811 bytes
15:35:58.597 in Ack 19
15:35:59.552 in Ping 
15:35:59.599 fetch POST /api/observability/v1/traces
15:35:59.599 fetched POST /api/observability/v1/traces 204
15:36:04.966 fetch POST /api/observability/v1/traces
15:36:04.967 fetched POST /api/observability/v1/traces 204
15:36:04.974 in Ping 
15:36:05.965 in Interrupt 31
15:36:06.304 rpc 33 server.reportClientActivity
15:36:06.972 fetch POST /api/observability/v1/traces
15:36:06.972 fetched POST /api/observability/v1/traces 204
15:36:09.975 in Ping 
15:36:14.976 in Ping 
15:36:19.981 in Ping 
15:36:21.326 in Interrupt 33
15:36:22.328 fetch POST /api/observability/v1/traces
15:36:22.328 fetched POST /api/observability/v1/traces 204
15:36:24.982 in Ping 
15:36:29.983 in Ping 
15:36:30.224 rpc 34 server.reportClientActivity
15:36:31.228 fetch POST /api/observability/v1/traces
15:36:31.228 fetched POST /api/observability/v1/traces 204
15:36:34.986 in Ping 
15:36:39.704 exit 32 {"_tag":"Success","value":{"sequence":76}}
15:36:39.990 in Ping 
15:36:40.134 chunk 12 952 bytes
15:36:40.136 chunk 19 34289 bytes
15:36:40.139 in Ack 12
15:36:40.349 in Ack 19
15:36:40.709 fetch POST /api/observability/v1/traces
15:36:40.709 fetched POST /api/observability/v1/traces 204
15:36:40.749 chunk 19 34311 bytes
15:36:40.751 in Ack 19
15:36:41.753 fetch POST /api/observability/v1/traces
15:36:41.753 fetched POST /api/observability/v1/traces 204
15:36:42.440 chunk 19 34333 bytes
15:36:42.442 in Ack 19
15:36:43.455 chunk 19 34513 bytes
15:36:43.459 fetch POST /api/observability/v1/traces
15:36:43.459 fetched POST /api/observability/v1/traces 204
15:36:43.461 in Ack 19
15:36:43.968 chunk 19 34629 bytes
15:36:43.969 in Ack 19
15:36:44.463 fetch POST /api/observability/v1/traces
15:36:44.463 fetched POST /api/observability/v1/traces 204
15:36:44.639 chunk 19 34651 bytes
15:36:44.640 in Ack 19
15:36:44.891 chunk 19 34663 bytes
15:36:44.892 in Ack 19
15:36:44.994 in Ping 
15:36:45.225 in Interrupt 34
15:36:45.480 rpc 35 server.reportClientActivity
15:36:45.491 chunk 19 34695 bytes
15:36:45.496 in Ack 19
15:36:45.641 fetch POST /api/observability/v1/traces
15:36:45.641 fetched POST /api/observability/v1/traces 204
15:36:46.070 chunk 19 34813 bytes
15:36:46.073 in Ack 19
15:36:46.571 chunk 19 34823 bytes
15:36:46.572 in Ack 19
15:36:46.911 chunk 19 34927 bytes
15:36:46.912 in Ack 19
15:36:47.074 fetch POST /api/observability/v1/traces
15:36:47.074 fetched POST /api/observability/v1/traces 204
15:36:47.167 chunk 19 34947 bytes
15:36:47.169 in Ack 19
15:36:47.418 chunk 19 34959 bytes
15:36:47.420 in Ack 19
15:36:47.669 chunk 19 34969 bytes
15:36:47.671 in Ack 19
15:36:48.170 chunk 19 34981 bytes
15:36:48.170 fetch POST /api/observability/v1/traces
15:36:48.170 fetched POST /api/observability/v1/traces 204
15:36:48.171 in Ack 19
15:36:48.838 chunk 19 34991 bytes
15:36:48.840 in Ack 19
15:36:49.098 chunk 19 35001 bytes
15:36:49.099 in Ack 19
15:36:49.179 fetch POST /api/observability/v1/traces
15:36:49.179 fetched POST /api/observability/v1/traces 204
15:36:49.355 chunk 19 35011 bytes
15:36:49.356 in Ack 19
15:36:49.857 chunk 19 35021 bytes
15:36:49.858 in Ack 19
15:36:49.998 in Ping 
15:36:50.117 chunk 19 35033 bytes
15:36:50.120 in Ack 19
15:36:50.357 fetch POST /api/observability/v1/traces
15:36:50.357 fetched POST /api/observability/v1/traces 204
15:36:50.375 chunk 19 35043 bytes
15:36:50.376 in Ack 19
15:36:50.663 chunk 19 35063 bytes
15:36:50.666 in Ack 19
15:36:50.914 chunk 19 35073 bytes
15:36:50.915 in Ack 19
15:36:51.165 chunk 19 35083 bytes
15:36:51.167 in Ack 19
15:36:51.377 fetch POST /api/observability/v1/traces
15:36:51.377 fetched POST /api/observability/v1/traces 204
15:36:51.558 chunk 19 35103 bytes
15:36:51.576 in Ack 19
15:36:51.813 chunk 19 35113 bytes
15:36:51.815 in Ack 19
15:36:52.574 chunk 19 35143 bytes
15:36:52.575 in Ack 19
15:36:52.696 fetch POST /api/observability/v1/traces
15:36:52.696 fetched POST /api/observability/v1/traces 204
15:36:52.834 chunk 19 35153 bytes
15:36:52.836 in Ack 19
15:36:53.088 chunk 19 35173 bytes
15:36:53.089 in Ack 19
15:36:53.402 chunk 19 35183 bytes
15:36:53.404 in Ack 19
15:36:53.817 chunk 19 35193 bytes
15:36:53.819 in Ack 19
15:36:53.863 fetch POST /api/observability/v1/traces
15:36:53.863 fetched POST /api/observability/v1/traces 204
15:36:54.079 chunk 19 35225 bytes
15:36:54.080 in Ack 19
15:36:54.330 chunk 19 35177 bytes
15:36:54.332 in Ack 19
15:36:54.810 chunk 19 35199 bytes
15:36:54.903 in Ack 19
15:36:55.048 in Ping 
15:36:55.088 fetch POST /api/observability/v1/traces
15:36:55.088 fetched POST /api/observability/v1/traces 204
15:36:55.196 chunk 19 35209 bytes
15:36:55.199 in Ack 19
15:36:55.577 chunk 19 35239 bytes
15:36:55.579 in Ack 19
15:36:55.963 chunk 19 35281 bytes
15:36:55.996 in Ack 19
15:36:56.266 fetch POST /api/observability/v1/traces
15:36:56.270 fetched POST /api/observability/v1/traces 204
15:36:56.366 chunk 19 35291 bytes
15:36:56.368 in Ack 19
15:36:56.651 chunk 19 35303 bytes
15:36:56.653 in Ack 19
15:36:56.947 chunk 19 35323 bytes
15:36:56.948 in Ack 19
15:36:57.263 chunk 19 35333 bytes
15:36:57.290 in Ack 19
15:36:57.369 fetch POST /api/observability/v1/traces
15:36:57.369 fetched POST /api/observability/v1/traces 204
15:36:57.514 chunk 19 35345 bytes
15:36:57.515 in Ack 19
15:36:57.764 chunk 19 35437 bytes
15:36:57.768 in Ack 19
15:36:58.181 chunk 19 35475 bytes
15:36:58.192 in Ack 19
15:36:58.539 chunk 19 35531 bytes
15:36:58.540 fetch POST /api/observability/v1/traces
15:36:58.540 fetched POST /api/observability/v1/traces 204
15:36:58.542 in Ack 19
15:36:58.970 chunk 19 35733 bytes
15:36:58.976 in Ack 19
15:36:59.222 chunk 19 35753 bytes
15:36:59.223 in Ack 19
15:36:59.485 chunk 19 35765 bytes
15:36:59.487 in Ack 19
15:36:59.575 fetch POST /api/observability/v1/traces
15:36:59.576 fetched POST /api/observability/v1/traces 204
15:36:59.743 chunk 19 35775 bytes
15:36:59.748 in Ack 19
15:36:59.996 chunk 19 35805 bytes
15:36:59.998 in Ack 19
15:37:00.095 in Ping 
15:37:00.248 chunk 19 35835 bytes
15:37:00.250 in Ack 19
15:37:00.490 in Interrupt 35
15:37:00.581 chunk 19 35857 bytes
15:37:00.584 in Ack 19
15:37:00.750 rpc 36 server.reportClientActivity
15:37:00.751 fetch POST /api/observability/v1/traces
15:37:00.751 fetched POST /api/observability/v1/traces 204
15:37:00.833 chunk 19 35867 bytes
15:37:00.835 in Ack 19
15:37:01.178 chunk 19 35909 bytes
15:37:01.179 in Ack 19
15:37:01.438 chunk 19 35921 bytes
15:37:01.444 in Ack 19
15:37:01.837 fetch POST /api/observability/v1/traces
15:37:01.837 fetched POST /api/observability/v1/traces 204
15:37:01.838 chunk 19 35961 bytes
15:37:01.840 in Ack 19
15:37:02.102 chunk 19 36031 bytes
15:37:02.103 in Ack 19
15:37:02.425 chunk 19 36081 bytes
15:37:02.427 in Ack 19
15:37:02.683 chunk 19 36101 bytes
15:37:02.684 in Ack 19
15:37:02.841 fetch POST /api/observability/v1/traces
15:37:02.841 fetched POST /api/observability/v1/traces 204
15:37:03.113 chunk 19 36121 bytes
15:37:03.120 in Ack 19
15:37:03.377 chunk 19 36181 bytes
15:37:03.378 in Ack 19
15:37:03.628 chunk 19 36209 bytes
15:37:03.637 in Ack 19
15:37:03.879 chunk 19 36219 bytes
15:37:03.880 in Ack 19
15:37:04.121 fetch POST /api/observability/v1/traces
15:37:04.121 fetched POST /api/observability/v1/traces 204
15:37:04.438 chunk 19 36281 bytes
15:37:04.439 in Ack 19
15:37:04.690 chunk 19 36313 bytes
15:37:04.691 in Ack 19
15:37:04.941 chunk 19 36323 bytes
15:37:04.943 in Ack 19
15:37:05.098 in Ping 
15:37:05.195 chunk 19 36333 bytes
15:37:05.196 in Ack 19
15:37:05.440 fetch POST /api/observability/v1/traces
15:37:05.440 fetched POST /api/observability/v1/traces 204
15:37:05.446 chunk 19 36355 bytes
15:37:05.449 in Ack 19
15:37:05.704 chunk 19 36375 bytes
15:37:05.705 in Ack 19
15:37:05.955 chunk 19 36433 bytes
15:37:05.956 in Ack 19
15:37:06.450 fetch POST /api/observability/v1/traces
15:37:06.450 fetched POST /api/observability/v1/traces 204
15:37:06.490 chunk 19 36475 bytes
15:37:06.492 in Ack 19
15:37:06.801 chunk 19 36497 bytes
15:37:06.803 in Ack 19
15:37:07.133 chunk 19 36593 bytes
15:37:07.137 in Ack 19
15:37:07.383 chunk 19 36669 bytes
15:37:07.384 in Ack 19
15:37:07.495 fetch POST /api/observability/v1/traces
15:37:07.495 fetched POST /api/observability/v1/traces 204
15:37:07.953 chunk 19 36689 bytes
15:37:07.963 in Ack 19
15:37:08.294 chunk 19 36725 bytes
15:37:08.296 in Ack 19
15:37:08.970 fetch POST /api/observability/v1/traces
15:37:08.970 fetched POST /api/observability/v1/traces 204
15:37:09.404 chunk 19 36793 bytes
15:37:09.406 in Ack 19
15:37:10.102 in Ping 
15:37:10.410 fetch POST /api/observability/v1/traces
15:37:10.410 fetched POST /api/observability/v1/traces 204
15:37:10.963 chunk 19 36811 bytes
15:37:10.965 in Ack 19
15:37:11.569 fetch POST /api/observability/v1/traces
15:37:11.569 fetched POST /api/observability/v1/traces 204
15:37:12.265 chunk 19 36897 bytes
15:37:12.267 in Ack 19
15:37:13.268 fetch POST /api/observability/v1/traces
15:37:13.269 fetched POST /api/observability/v1/traces 204
15:37:15.106 in Ping 
15:37:15.751 in Interrupt 36
15:37:16.752 fetch POST /api/observability/v1/traces
15:37:16.752 fetched POST /api/observability/v1/traces 204
15:37:18.627 chunk 19 36945 bytes
15:37:18.629 in Ack 19
15:37:19.630 fetch POST /api/observability/v1/traces
15:37:19.630 fetched POST /api/observability/v1/traces 204
15:37:19.903 chunk 19 40459 bytes
15:37:19.905 in Ack 19
15:37:20.113 in Ping 
15:37:20.303 rpc 37 server.reportClientActivity
15:37:20.912 fetch POST /api/observability/v1/traces
15:37:20.912 fetched POST /api/observability/v1/traces 204
15:37:23.270 chunk 19 42548 bytes
15:37:23.272 in Ack 19
15:37:24.274 fetch POST /api/observability/v1/traces
15:37:24.274 fetched POST /api/observability/v1/traces 204
15:37:25.114 in Ping 
15:37:26.383 chunk 19 42550 bytes
15:37:26.386 in Ack 19
15:37:27.389 fetch POST /api/observability/v1/traces
15:37:27.389 fetched POST /api/observability/v1/traces 204
15:37:30.118 in Ping 
15:37:31.586 fetch POST /api/observability/v1/traces
15:37:31.586 fetched POST /api/observability/v1/traces 204
15:37:33.488 chunk 12 3205 bytes
15:37:33.489 in Ack 12
15:37:33.496 rpc 38 orchestration.dispatchCommand thread.visit
15:37:34.490 fetch POST /api/observability/v1/traces
15:37:34.490 fetched POST /api/observability/v1/traces 204
15:37:35.050 exit 38 {"_tag":"Success","value":{"sequence":94}}
15:37:35.122 in Ping 
15:37:35.304 in Interrupt 37
15:37:36.052 fetch POST /api/observability/v1/traces
15:37:36.052 fetched POST /api/observability/v1/traces 204
15:37:37.025 chunk 12 952 bytes
15:37:37.026 in Ack 12
15:37:38.028 fetch POST /api/observability/v1/traces
15:37:38.028 fetched POST /api/observability/v1/traces 204
15:37:40.126 in Ping 
15:37:44.532 fetch POST /api/observability/v1/traces
15:37:44.532 fetched POST /api/observability/v1/traces 204
15:37:45.131 in Ping 
15:37:45.303 rpc 39 server.reportClientActivity
15:37:46.303 fetch POST /api/observability/v1/traces
15:37:46.303 fetched POST /api/observability/v1/traces 204
15:37:48.289 chunk 12 3205 bytes
15:37:48.291 in Ack 12
15:37:49.293 fetch POST /api/observability/v1/traces
15:37:49.294 fetched POST /api/observability/v1/traces 204
15:37:50.134 in Ping 
15:37:50.153 chunk 19 42550 bytes
15:37:50.155 in Ack 19
15:37:50.664 chunk 19 43561 bytes
15:37:50.666 in Ack 19
15:37:51.003 rpc 40 orchestration.dispatchCommand runtime-request.respond
15:37:51.156 fetch POST /api/observability/v1/traces
15:37:51.156 fetched POST /api/observability/v1/traces 204
15:37:51.634 exit 28 {"_tag":"Success","value":{"expiresAt":1794155870079,"imageDimensions":{"height":48,"width":48},"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNob
15:37:52.636 fetch POST /api/observability/v1/traces
15:37:52.636 fetched POST /api/observability/v1/traces 204
15:37:54.334 chunk 19 46060 bytes
15:37:54.334 exit 40 {"_tag":"Success","value":{"sequence":97}}
15:37:54.337 in Ack 19
15:37:54.637 fetch POST /api/observability/v1/traces
15:37:54.637 fetched POST /api/observability/v1/traces 204
15:37:54.914 rpc 41 orchestration.dispatchCommand runtime-request.respond
15:37:55.138 in Ping 
15:37:55.189 chunk 19 46761 bytes
15:37:55.191 in Ack 19
15:37:55.915 fetch POST /api/observability/v1/traces
15:37:55.915 fetched POST /api/observability/v1/traces 204
15:37:55.927 chunk 19 46813 bytes
15:37:55.929 in Ack 19
15:37:56.365 chunk 19 46845 bytes
15:37:56.366 in Ack 19
15:37:56.930 fetch POST /api/observability/v1/traces
15:37:56.930 fetched POST /api/observability/v1/traces 204
15:37:58.819 chunk 19 46951 bytes
15:37:58.819 exit 41 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
15:37:58.820 in Ack 19
15:37:59.826 fetch POST /api/observability/v1/traces
15:37:59.826 fetched POST /api/observability/v1/traces 204
15:38:00.142 in Ping 
15:38:00.304 in Interrupt 39
15:38:00.428 rpc 42 orchestration.dispatchCommand runtime-request.respond
15:38:01.243 fetch POST /api/observability/v1/traces
15:38:01.243 fetched POST /api/observability/v1/traces 204
15:38:01.590 chunk 19 46004 bytes
15:38:01.592 in Ack 19
15:38:02.200 chunk 19 46178 bytes
15:38:02.202 in Ack 19
15:38:02.593 fetch POST /api/observability/v1/traces
15:38:02.593 fetched POST /api/observability/v1/traces 204
15:38:02.747 chunk 19 46280 bytes
15:38:02.748 in Ack 19
15:38:03.749 fetch POST /api/observability/v1/traces
15:38:03.749 fetched POST /api/observability/v1/traces 204
15:38:04.621 chunk 19 46542 bytes
15:38:04.622 in Ack 19
15:38:05.171 in Ping 
15:38:05.623 fetch POST /api/observability/v1/traces
15:38:05.623 fetched POST /api/observability/v1/traces 204
15:38:06.408 chunk 12 952 bytes
15:38:06.408 chunk 19 46570 bytes
15:38:06.409 in Ack 12
15:38:06.418 in Ack 19
15:38:06.750 chunk 19 46652 bytes
15:38:06.752 in Ack 19
15:38:07.008 chunk 19 46662 bytes
15:38:07.009 in Ack 19
15:38:07.266 chunk 19 46674 bytes
15:38:07.267 in Ack 19
15:38:07.411 fetch POST /api/observability/v1/traces
15:38:07.411 fetched POST /api/observability/v1/traces 204
15:38:07.521 chunk 19 46684 bytes
15:38:07.523 in Ack 19
15:38:07.776 chunk 19 46694 bytes
15:38:07.778 in Ack 19
15:38:08.217 chunk 19 46764 bytes
15:38:08.218 in Ack 19
15:38:08.524 fetch POST /api/observability/v1/traces
15:38:08.524 fetched POST /api/observability/v1/traces 204
15:38:08.903 chunk 19 46988 bytes
15:38:08.905 in Ack 19
15:38:09.906 fetch POST /api/observability/v1/traces
15:38:09.906 fetched POST /api/observability/v1/traces 204
15:38:10.117 chunk 19 47040 bytes
15:38:10.119 in Ack 19
15:38:10.193 in Ping 
15:38:10.302 rpc 43 server.reportClientActivity
15:38:11.120 fetch POST /api/observability/v1/traces
15:38:11.120 fetched POST /api/observability/v1/traces 204
15:38:11.754 chunk 19 47118 bytes
15:38:11.756 in Ack 19
15:38:12.761 fetch POST /api/observability/v1/traces
15:38:12.762 fetched POST /api/observability/v1/traces 204
15:38:15.194 in Ping 
15:38:15.262 chunk 19 47120 bytes
15:38:15.263 in Ack 19
15:38:16.264 fetch POST /api/observability/v1/traces
15:38:16.264 fetched POST /api/observability/v1/traces 204
15:38:20.198 in Ping 
15:38:20.372 chunk 19 47138 bytes
15:38:20.373 in Ack 19
15:38:21.282 fetch POST /api/observability/v1/traces
15:38:21.282 fetched POST /api/observability/v1/traces 204
