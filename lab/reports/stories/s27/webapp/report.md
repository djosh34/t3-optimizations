# story, webapp, warm start, seed 27

Profile story (graded): seed 27's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 98 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 41.8s |
| Prompt reached the agent | 15.4s |
| Time to first text | 51.0s |
| Whole reply shown | 96.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 37.5s / 14.8s / 34.5s |
| Longest silence | 51.0s |
| Approval shown after it was asked | 22.7s |
| Bytes on the wire down / up | 182896 / 76968 |
| Bytes the app received / sent | 55764 / 57449 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 4.5 | 0 | 186 |
| 30 | 14.5 | 0 | 585 |
| 40 | 24.5 | 0 | 988 |
| 50 | 34.5 | 0 | 1077 |
| 60 | 34.5 | 397 | 1077 |
| 70 | 3.8 | 1077 | 1233 |
| 80 | 13.8 | 1077 | 1595 |
| 90 | 23.8 | 1077 | 1596 |
| 96 | 29.8 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217061359,"silences":0,"longestSilenceMs":497,"longestRecoveredGapMs":497},"stats":{"sendRate":3413.884328728165,"deliveredRate":3152.74750677668,"loss":0,"srttMs":11.123939490670839,"minRttMs":null,"sentPackets":135,"sentWireBytes":37139,"receivedPackets":119,"receivedWireBytes":89145,"lastHeardMs":11362,"unsyncedBytes":736,"inflightPackets":7,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217069680,"silences":1,"longestSilenceMs":8321,"longestRecoveredGapMs":8321},"stats":{"sendRate":2000,"deliveredRate":3152.74750677668,"loss":0,"srttMs":11.123939490670839,"minRttMs":null,"sentPackets":148,"sentWireBytes":44958,"receivedPackets":120,"receivedWireBytes":90056,"lastHeardMs":19682,"unsyncedBytes":736,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217076976,"silences":2,"longestSilenceMs":8321,"longestRecoveredGapMs":8321},"stats":{"sendRate":2000,"deliveredRate":3152.74750677668,"loss":0,"srttMs":11.123939490670839,"minRttMs":null,"sentPackets":163,"sentWireBytes":52503,"receivedPackets":121,"receivedWireBytes":90973,"lastHeardMs":26979,"unsyncedBytes":1193,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217084595,"silences":3,"longestSilenceMs":8321,"longestRecoveredGapMs":8321},"stats":{"sendRate":2000,"deliveredRate":3037.149499087567,"loss":0,"srttMs":11.123939490670839,"minRttMs":null,"sentPackets":163,"sentWireBytes":52503,"receivedPackets":138,"receivedWireBytes":96498,"lastHeardMs":34598,"unsyncedBytes":1193,"inflightPackets":35,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217084595,"silences":3,"longestSilenceMs":8321,"longestRecoveredGapMs":8321},"stats":{"sendRate":2000,"deliveredRate":3037.149499087567,"loss":0,"srttMs":11.123939490670839,"minRttMs":null,"sentPackets":163,"sentWireBytes":52503,"receivedPackets":138,"receivedWireBytes":96498,"lastHeardMs":34598,"unsyncedBytes":1650,"inflightPackets":35,"duplicateBytes":0}}
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 349/498","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217108782,"silences":5,"longestSilenceMs":19645,"longestRecoveredGapMs":19645},"stats":{"sendRate":2000,"deliveredRate":2138.860866977877,"loss":0.22,"srttMs":2358.983447054337,"minRttMs":18794,"sentPackets":163,"sentWireBytes":52503,"receivedPackets":140,"receivedWireBytes":97500,"lastHeardMs":58785,"unsyncedBytes":914,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1091/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217123229,"silences":7,"longestSilenceMs":19645,"longestRecoveredGapMs":19645},"stats":{"sendRate":2000,"deliveredRate":2138.860866977877,"loss":0.22,"srttMs":2358.983447054337,"minRttMs":18794,"sentPackets":170,"sentWireBytes":54494,"receivedPackets":164,"receivedWireBytes":109688,"lastHeardMs":73231,"unsyncedBytes":914,"inflightPackets":39,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 1542/1542 508/923 533/1093","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217131968,"silences":7,"longestSilenceMs":19645,"longestRecoveredGapMs":19645},"stats":{"sendRate":2000,"deliveredRate":1607.6222051276109,"loss":0.2764379411764705,"srttMs":2123.3500285264713,"minRttMs":1901,"sentPackets":228,"sentWireBytes":73673,"receivedPackets":216,"receivedWireBytes":137405,"lastHeardMs":81970,"unsyncedBytes":398,"inflightPackets":60,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 1542/1542 508/923 533/1093","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 4","open":true,"generation":2,"iceRestarts":4,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217131968,"silences":7,"longestSilenceMs":19645,"longestRecoveredGapMs":19645},"stats":{"sendRate":3000,"deliveredRate":1607.6222051276109,"loss":0.2764379411764705,"srttMs":2123.3500285264713,"minRttMs":1901,"sentPackets":245,"sentWireBytes":79963,"receivedPackets":216,"receivedWireBytes":137405,"lastHeardMs":81970,"unsyncedBytes":398,"inflightPackets":77,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 860/860 870/870 1299/1299 1093/1093 1058/1058 1542/1542 508/923 533/1093","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"544/544 456/456 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1045/1045"},"link":{"transport":"webrtc","trying":"ICE restart 5","open":true,"generation":2,"iceRestarts":5,"newConnections":0,"handshakeMs":740,"lastDataAt":1794217131968,"silences":7,"longestSilenceMs":19645,"longestRecoveredGapMs":19645},"stats":{"sendRate":2000,"deliveredRate":1607.6222051276109,"loss":0.2764379411764705,"srttMs":2123.3500285264713,"minRttMs":null,"sentPackets":257,"sentWireBytes":85198,"receivedPackets":216,"receivedWireBytes":137405,"lastHeardMs":81970,"unsyncedBytes":855,"inflightPackets":89,"duplicateBytes":0}}
- shim trace
09:37:30.791 fetch GET /api/auth/session
09:37:30.794 fetched GET /api/auth/session 200
09:37:31.052 fetch GET /.well-known/t3/environment
09:37:31.057 fetched GET /.well-known/t3/environment 200
09:37:31.089 fetch GET /.well-known/t3/environment
09:37:31.090 fetched GET /.well-known/t3/environment 200
09:37:31.097 rpc 0 subscribeServerConfig
09:37:31.098 chunk 0 38462 bytes
09:37:31.326 in Ack 0
09:37:31.331 rpc 1 subscribeServerLifecycle
09:37:31.333 rpc 2 subscribeServerLifecycle
09:37:31.347 fetch GET /api/auth/session
09:37:31.348 fetched GET /api/auth/session 200
09:37:31.349 rpc 3 subscribeDeviceState
09:37:31.351 rpc 4 preview.reportProfiles
09:37:31.421 rpc 5 server.reportClientActivity
09:37:31.479 rpc 6 subscribeProjectClones
09:37:31.521 fetch GET /api/orchestration/threads/mcp%3Af5d527a7-8914-4e21-b08f-3eadf6719883/bounded
09:37:31.528 fetch GET /api/orchestration/shell
09:37:31.529 fetched GET /api/orchestration/shell 200
09:37:31.532 rpc 7 orchestration.subscribeShell
09:37:31.532 chunk 7 4576 bytes
09:37:31.567 rpc 8 server.refreshProviders
09:37:31.639 rpc 9 orchestration.dispatchCommand thread.visit
09:37:31.639 exit 9 {"_tag":"Success","value":{"sequence":0}}
09:37:31.661 rpc 10 subscribeTerminalMetadata
09:37:31.662 rpc 11 subscribeVcsStatus
09:37:31.664 rpc 12 subscribePreviewEvents
09:37:31.665 rpc 13 assets.createUrl
09:37:31.666 rpc 14 preview.list
09:37:31.669 rpc 15 projects.readFile
09:37:31.673 in Ack 7
09:37:31.782 chunk 7 4552 bytes
09:37:31.783 in Ack 7
09:37:31.869 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"58589356-6852-4bad-9cbd-02ffdc8234f8","sessions":[]}}
09:37:31.869 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:32.046 fetch POST /api/observability/v1/traces
09:37:32.046 fetched POST /api/observability/v1/traces 204
09:37:32.108 chunk 1 3815 bytes
09:37:32.109 in Ack 1
09:37:32.353 chunk 2 3815 bytes
09:37:32.359 in Ack 2
09:37:32.562 exit 8 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:09.166Z","configurableProviders":false,"con
09:37:32.562 chunk 11 1632 bytes
09:37:32.562 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:32.566 in Ack 11
09:37:32.999 chunk 7 4592 bytes
09:37:33.000 in Ack 7
09:37:33.048 fetch POST /api/observability/v1/traces
09:37:33.048 fetched POST /api/observability/v1/traces 204
09:37:33.298 chunk 3 876 bytes
09:37:33.298 exit 4 {"_tag":"Success","value":null}
09:37:33.298 exit 5 {"_tag":"Success","value":null}
09:37:33.298 chunk 6 4 bytes
09:37:33.299 in Ack 3
09:37:33.305 rpc 16 preview.reportProfiles
09:37:33.306 in Ack 6
09:37:33.378 chunk 10 36 bytes
09:37:33.378 exit 16 {"_tag":"Success","value":null}
09:37:33.378 in Ack 10
09:37:33.557 rpc 17 server.reportClientActivity
09:37:33.620 exit 17 {"_tag":"Success","value":null}
09:37:34.305 fetch POST /api/observability/v1/traces
09:37:34.305 fetched POST /api/observability/v1/traces 204
09:37:34.649 fetched GET /api/orchestration/threads/mcp%3Af5d527a7-8914-4e21-b08f-3eadf6719883/bounded 200
09:37:34.654 rpc 18 orchestration.subscribeThread
09:37:34.654 chunk 18 8283 bytes
09:37:34.673 in Interrupt 11
09:37:34.675 rpc 19 subscribeVcsStatus
09:37:34.677 in Ack 18
09:37:34.797 chunk 19 1632 bytes
09:37:34.797 in Ack 19
09:37:34.908 chunk 18 8323 bytes
09:37:34.909 in Ack 18
09:37:34.987 rpc 20 server.reportClientActivity
09:37:35.166 chunk 18 11919 bytes
09:37:35.175 in Ack 18
09:37:35.247 exit 20 {"_tag":"Success","value":null}
09:37:35.414 chunk 18 13498 bytes
09:37:35.416 in Ack 18
09:37:35.703 rpc 21 attachments.createUploadUrl
09:37:35.753 fetch POST /api/observability/v1/traces
09:37:35.753 fetched POST /api/observability/v1/traces 204
09:37:35.754 chunk 18 16151 bytes
09:37:35.766 in Ack 18
09:37:35.808 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-fc71a71c-6d78-4484-80e6-c51adb0ec041","expiresAt":1794217655786,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:35.809 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWZjNzFhNzFjLTZkNzgtNDQ4NC04MGU2LWM1MWFkYjBlYzA0MSIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY1NTc4Nn0.Nd92mkwelW8KXpGHdto8NHmPR0ly54GIxdxf1BtG_hI
09:37:36.004 chunk 18 19797 bytes
09:37:36.006 in Ack 18
09:37:36.022 in Interrupt 19
09:37:36.025 rpc 22 subscribeVcsStatus
09:37:36.138 in Ping 
09:37:36.139 chunk 22 1632 bytes
09:37:36.141 in Ack 22
09:37:36.254 chunk 18 21100 bytes
09:37:36.257 in Ack 18
09:37:36.282 rpc 23 server.reportClientActivity
09:37:36.767 fetch POST /api/observability/v1/traces
09:37:36.768 fetched POST /api/observability/v1/traces 204
09:37:37.759 exit 23 {"_tag":"Success","value":null}
09:37:38.753 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:38.975 fetch POST /api/observability/v1/traces
09:37:38.975 fetched POST /api/observability/v1/traces 204
09:37:39.006 exit 24 {"_tag":"Success","value":{"sequence":109}}
09:37:39.010 rpc 25 attachments.delete
09:37:39.275 chunk 7 4678 bytes
09:37:39.275 chunk 18 25369 bytes
09:37:39.277 in Ack 7
09:37:39.314 in Ack 18
09:37:39.318 rpc 26 assets.createUrl
09:37:39.526 chunk 18 26810 bytes
09:37:39.527 in Ack 18
09:37:39.776 chunk 18 29269 bytes
09:37:39.777 in Ack 18
09:37:40.008 fetch POST /api/observability/v1/traces
09:37:40.008 fetched POST /api/observability/v1/traces 204
09:37:40.024 chunk 7 4617 bytes
09:37:40.025 in Ack 7
09:37:40.037 chunk 18 29291 bytes
09:37:40.038 in Ack 18
09:37:40.287 chunk 18 29975 bytes
09:37:40.289 in Ack 18
09:37:40.525 chunk 7 4614 bytes
09:37:40.525 in Ack 7
09:37:40.527 rpc 27 orchestration.dispatchCommand thread.visit
09:37:40.527 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:40.538 chunk 18 32554 bytes
09:37:40.539 in Ack 18
09:37:40.603 exit 25 {"_tag":"Success","value":null}
09:37:40.775 chunk 7 4614 bytes
09:37:40.776 in Ack 7
09:37:40.800 chunk 18 35260 bytes
09:37:40.803 in Ack 18
09:37:40.836 in Interrupt 22
09:37:40.840 rpc 28 subscribeVcsStatus
09:37:40.859 chunk 28 1713 bytes
09:37:40.860 in Ack 28
09:37:41.027 fetch POST /api/observability/v1/traces
09:37:41.027 fetched POST /api/observability/v1/traces 204
09:37:41.310 rpc 29 server.reportClientActivity
09:37:41.311 in Ping 
09:37:41.360 exit 29 {"_tag":"Success","value":null}
09:37:42.331 fetch POST /api/observability/v1/traces
09:37:42.331 fetched POST /api/observability/v1/traces 204
09:37:42.567 rpc 30 server.refreshProviders
09:37:42.567 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:09.166Z","configurableProviders":false,"con
09:37:42.806 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:43.579 fetch POST /api/observability/v1/traces
09:37:43.579 fetched POST /api/observability/v1/traces 204
09:37:46.256 fetch POST /api/observability/v1/traces
09:37:46.256 fetched POST /api/observability/v1/traces 204
09:37:46.311 in Ping 
09:37:49.683 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/b59fab20-2f29-4ea8-897e-3fa49daa1e85","expiresAt":1794220669683}}
09:37:50.687 fetch POST /api/observability/v1/traces
09:37:50.687 fetched POST /api/observability/v1/traces 204
09:37:51.311 in Ping 
09:37:56.312 rpc 32 server.reportClientActivity
09:37:56.313 in Ping 
09:37:57.316 fetch POST /api/observability/v1/traces
09:37:57.316 fetched POST /api/observability/v1/traces 204
09:38:01.313 in Ping 
09:38:06.314 in Ping 
09:38:11.317 in Interrupt 32
09:38:11.319 in Ping 
09:38:12.319 fetch POST /api/observability/v1/traces
09:38:12.319 fetched POST /api/observability/v1/traces 204
09:38:16.319 in Ping 
09:38:21.313 rpc 33 server.reportClientActivity
09:38:21.319 in Ping 
09:38:22.313 fetch POST /api/observability/v1/traces
09:38:22.313 fetched POST /api/observability/v1/traces 204
09:38:24.242 chunk 18 38029 bytes
09:38:24.242 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:38:24.244 in Ack 18
09:38:25.245 fetch POST /api/observability/v1/traces
09:38:25.246 fetched POST /api/observability/v1/traces 204
09:38:26.322 in Ping 
09:38:28.784 chunk 18 38377 bytes
09:38:28.787 in Ack 18
09:38:29.788 fetch POST /api/observability/v1/traces
09:38:29.788 fetched POST /api/observability/v1/traces 204
09:38:31.326 in Ping 
09:38:33.465 chunk 18 38673 bytes
09:38:33.468 in Ack 18
09:38:34.355 chunk 18 39133 bytes
09:38:34.360 in Ack 18
09:38:34.470 fetch POST /api/observability/v1/traces
09:38:34.470 fetched POST /api/observability/v1/traces 204
09:38:34.606 chunk 18 39829 bytes
09:38:34.607 in Ack 18
09:38:34.856 chunk 18 39865 bytes
09:38:34.858 in Ack 18
09:38:35.609 fetch POST /api/observability/v1/traces
09:38:35.610 fetched POST /api/observability/v1/traces 204
09:38:36.314 in Interrupt 33
09:38:36.330 in Ping 
09:38:37.316 fetch POST /api/observability/v1/traces
09:38:37.316 fetched POST /api/observability/v1/traces 204
09:38:41.334 in Ping 
09:38:45.303 chunk 18 40271 bytes
09:38:45.306 in Ack 18
09:38:45.435 fetch POST /api/observability/v1/traces
09:38:45.435 fetched POST /api/observability/v1/traces 204
09:38:45.559 chunk 18 40361 bytes
09:38:45.561 in Ack 18
09:38:46.338 in Ping 
09:38:46.357 rpc 34 server.reportClientActivity
09:38:46.563 fetch POST /api/observability/v1/traces
09:38:46.563 fetched POST /api/observability/v1/traces 204
09:38:47.418 chunk 18 41363 bytes
09:38:47.422 in Ack 18
09:38:47.730 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:38:48.423 fetch POST /api/observability/v1/traces
09:38:48.423 fetched POST /api/observability/v1/traces 204
09:38:49.024 chunk 18 44850 bytes
09:38:49.026 in Ack 18
09:38:49.818 exit 35 {"_tag":"Success","value":{"sequence":184}}
09:38:50.027 fetch POST /api/observability/v1/traces
09:38:50.027 fetched POST /api/observability/v1/traces 204
09:38:50.283 rpc 36 orchestration.dispatchCommand runtime-request.respond
09:38:50.305 chunk 18 44893 bytes
09:38:50.307 in Ack 18
09:38:51.283 fetch POST /api/observability/v1/traces
09:38:51.283 fetched POST /api/observability/v1/traces 204
09:38:51.342 in Ping 
09:38:55.421 fetch POST /api/observability/v1/traces
09:38:55.421 fetched POST /api/observability/v1/traces 204
09:38:56.346 in Ping 
09:39:01.350 in Ping 
09:39:01.360 in Interrupt 34
09:39:02.361 fetch POST /api/observability/v1/traces
09:39:02.361 fetched POST /api/observability/v1/traces 204
09:39:06.356 in Ping 
09:39:11.357 rpc 37 server.reportClientActivity
09:39:11.359 in Ping 
09:39:12.357 fetch POST /api/observability/v1/traces
09:39:12.357 fetched POST /api/observability/v1/traces 204
09:39:16.362 in Ping 
09:39:16.494 chunk 18 47636 bytes
09:39:16.497 in Ack 18
09:39:16.698 exit 36 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
09:39:16.819 chunk 18 47638 bytes
09:39:16.827 in Ack 18
09:39:17.434 chunk 18 48120 bytes
09:39:17.439 in Ack 18
09:39:17.534 fetch POST /api/observability/v1/traces
09:39:17.534 fetched POST /api/observability/v1/traces 204
09:39:17.687 chunk 18 48226 bytes
09:39:17.690 in Ack 18
09:39:17.936 chunk 18 48586 bytes
09:39:17.939 in Ack 18
09:39:18.190 chunk 18 48588 bytes
09:39:18.192 in Ack 18
09:39:18.439 chunk 18 48810 bytes
09:39:18.441 in Ack 18
09:39:18.692 fetch POST /api/observability/v1/traces
09:39:18.692 fetched POST /api/observability/v1/traces 204
