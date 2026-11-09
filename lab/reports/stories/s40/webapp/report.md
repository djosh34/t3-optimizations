# story, webapp, warm start, seed 40

Profile story (graded): seed 40's 2G story. Commit 9c461bdca554.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 69 s (deadline 5m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.8s |
| Prompt reached the agent | 2.4s |
| Time to first text | 15.6s |
| Whole reply shown | 67.5s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 29.2s / 9.5s / 25.2s |
| Longest silence | 33.9s |
| Approval shown after it was asked | 4.7s |
| Bytes on the wire down / up | 434361 / 122209 |
| Bytes the app received / sent | 239085 / 41332 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 7.5 | 0 | 303 |
| 20 | 7.5 | 397 | 701 |
| 30 | 17.5 | 397 | 1077 |
| 40 | 2.2 | 1077 | 1166 |
| 50 | 12.2 | 1077 | 1537 |
| 60 | 22.2 | 1077 | 1596 |
| 67 | 29.2 | 1077 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971","user":"10/10 9/9","attempts":"659/659 659/659","providerTurns":"772/772 772/772","providerThreads":"807/807","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016","messages":"456/456 544/544 901/901 544/544","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217078817,"silences":0,"longestSilenceMs":772,"longestRecoveredGapMs":772},"stats":{"sendRate":1500,"deliveredRate":1996.5136548518303,"loss":0,"srttMs":45.227352244647705,"minRttMs":1,"sentPackets":132,"sentWireBytes":37176,"receivedPackets":122,"receivedWireBytes":98153,"lastHeardMs":14020,"unsyncedBytes":736,"inflightPackets":8,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217088642,"silences":0,"longestSilenceMs":772,"longestRecoveredGapMs":772},"stats":{"sendRate":843.75,"deliveredRate":1336.3286264441592,"loss":0,"srttMs":992.6913432910691,"minRttMs":1049,"sentPackets":180,"sentWireBytes":45492,"receivedPackets":248,"receivedWireBytes":164299,"lastHeardMs":23844,"unsyncedBytes":0,"inflightPackets":1,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217099426,"silences":1,"longestSilenceMs":2443,"longestRecoveredGapMs":2443},"stats":{"sendRate":632.8125,"deliveredRate":337.20382865212616,"loss":0.08333333333333337,"srttMs":3122.2795648231304,"minRttMs":1049,"sentPackets":203,"sentWireBytes":51725,"receivedPackets":389,"receivedWireBytes":250043,"lastHeardMs":34628,"unsyncedBytes":0,"inflightPackets":8,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1068/1068","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 498/498","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217109422,"silences":2,"longestSilenceMs":5511,"longestRecoveredGapMs":5511},"stats":{"sendRate":632.8125,"deliveredRate":460.75469322160075,"loss":0.11287878787878788,"srttMs":3009.8943445166838,"minRttMs":1049,"sentPackets":229,"sentWireBytes":55423,"receivedPackets":443,"receivedWireBytes":280584,"lastHeardMs":44625,"unsyncedBytes":0,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 923/923 1542/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 122/1368","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"1002/1002"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217119711,"silences":2,"longestSilenceMs":5511,"longestRecoveredGapMs":5511},"stats":{"sendRate":632.8125,"deliveredRate":636.6138866675789,"loss":0.09000000000000001,"srttMs":3009.8943445166838,"minRttMs":1049,"sentPackets":259,"sentWireBytes":63011,"receivedPackets":520,"receivedWireBytes":322180,"lastHeardMs":54913,"unsyncedBytes":855,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 923/923 1542/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 122/1368","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"827/1045"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217120772,"silences":2,"longestSilenceMs":5511,"longestRecoveredGapMs":5511},"stats":{"sendRate":474.609375,"deliveredRate":685.5503205790828,"loss":0.063,"srttMs":3890.028482520586,"minRttMs":1049,"sentPackets":267,"sentWireBytes":67095,"receivedPackets":526,"receivedWireBytes":325949,"lastHeardMs":55974,"unsyncedBytes":855,"inflightPackets":20,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"1058/1058 1093/1093 870/870 860/860 1299/1299 1093/1093 1058/1058 849/849 1093/1093 923/923 1542/1542","frame":"1030/1030","thread":"961/961","assistant":"32/32 21/21 1340/1340","runs":"1014/1014 971/971 839/839","user":"10/10 9/9 73/73","attempts":"659/659 659/659 635/635","providerTurns":"772/772 772/772 654/654","providerThreads":"809/809","providerSessions":"2653/2653","nodes":"1016/1016 809/809 809/809 1016/1016 785/785 122/1368","messages":"456/456 544/544 901/901 544/544 451/451 543/543","checkpoints":"777/777 611/611 777/777","checkpointScopes":"691/691","runtimeRequests":"827/1045"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":1021,"lastDataAt":1794217120772,"silences":2,"longestSilenceMs":5511,"longestRecoveredGapMs":5511},"stats":{"sendRate":474.609375,"deliveredRate":685.5503205790828,"loss":0.063,"srttMs":3890.028482520586,"minRttMs":1049,"sentPackets":281,"sentWireBytes":73426,"receivedPackets":526,"receivedWireBytes":325949,"lastHeardMs":55974,"unsyncedBytes":1312,"inflightPackets":34,"duplicateBytes":0}}
- shim trace
09:37:45.956 fetch GET /api/auth/session
09:37:45.958 fetched GET /api/auth/session 200
09:37:46.013 fetch GET /.well-known/t3/environment
09:37:46.017 fetched GET /.well-known/t3/environment 200
09:37:46.035 fetch GET /.well-known/t3/environment
09:37:46.037 fetched GET /.well-known/t3/environment 200
09:37:46.044 rpc 0 subscribeServerConfig
09:37:46.045 chunk 0 38462 bytes
09:37:46.124 in Ack 0
09:37:46.127 rpc 1 subscribeServerLifecycle
09:37:46.128 rpc 2 subscribeServerLifecycle
09:37:46.186 fetch GET /api/auth/session
09:37:46.188 fetched GET /api/auth/session 200
09:37:46.193 rpc 3 subscribeDeviceState
09:37:46.200 rpc 4 preview.reportProfiles
09:37:46.342 rpc 5 server.reportClientActivity
09:37:46.849 rpc 6 subscribeProjectClones
09:37:46.856 fetch GET /api/orchestration/shell
09:37:46.887 rpc 7 server.refreshProviders
09:37:46.978 rpc 8 orchestration.dispatchCommand thread.visit
09:37:46.978 exit 8 {"_tag":"Success","value":{"sequence":0}}
09:37:46.997 fetched GET /api/orchestration/shell 200
09:37:46.998 rpc 9 subscribeTerminalMetadata
09:37:46.999 rpc 10 subscribeVcsStatus
09:37:47.002 rpc 11 subscribePreviewEvents
09:37:47.006 rpc 12 orchestration.subscribeShell
09:37:47.006 chunk 12 4680 bytes
09:37:47.008 rpc 13 assets.createUrl
09:37:47.009 rpc 14 preview.list
09:37:47.010 rpc 15 projects.readFile
09:37:47.056 fetch GET /api/orchestration/threads/mcp%3Afc9ae0e4-5ba8-4cfc-aa55-5760a07c3af2/bounded
09:37:47.064 fetch POST /api/observability/v1/traces
09:37:47.064 fetched POST /api/observability/v1/traces 204
09:37:47.065 in Ack 12
09:37:47.256 chunk 12 4656 bytes
09:37:47.257 in Ack 12
09:37:47.827 exit 14 {"_tag":"Success","value":{"revision":0,"serverEpoch":"f58cb51d-0306-49cf-960a-eb8736ae3f6d","sessions":[]}}
09:37:47.827 exit 15 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
09:37:47.827 chunk 1 3815 bytes
09:37:47.835 in Ack 1
09:37:48.059 chunk 2 3815 bytes
09:37:48.059 in Ack 2
09:37:48.066 fetch POST /api/observability/v1/traces
09:37:48.066 fetched POST /api/observability/v1/traces 204
09:37:48.295 exit 7 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:22.617Z","configurableProviders":false,"con
09:37:48.295 chunk 10 1632 bytes
09:37:48.295 exit 13 {"_tag":"Success","value":{"expiresAt":1794220200000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
09:37:48.300 in Ack 10
09:37:48.796 chunk 12 4635 bytes
09:37:48.797 in Ack 12
09:37:48.964 chunk 3 876 bytes
09:37:48.964 exit 4 {"_tag":"Success","value":null}
09:37:48.964 exit 5 {"_tag":"Success","value":null}
09:37:48.964 chunk 6 4 bytes
09:37:48.965 in Ack 3
09:37:48.970 rpc 16 preview.reportProfiles
09:37:48.970 in Ack 6
09:37:49.048 chunk 9 36 bytes
09:37:49.048 exit 16 {"_tag":"Success","value":null}
09:37:49.048 in Ack 9
09:37:49.221 rpc 17 server.reportClientActivity
09:37:49.294 exit 17 {"_tag":"Success","value":null}
09:37:49.296 fetch POST /api/observability/v1/traces
09:37:49.296 fetched POST /api/observability/v1/traces 204
09:37:50.316 fetched GET /api/orchestration/threads/mcp%3Afc9ae0e4-5ba8-4cfc-aa55-5760a07c3af2/bounded 200
09:37:50.321 rpc 18 orchestration.subscribeThread
09:37:50.321 chunk 18 8283 bytes
09:37:50.334 in Interrupt 10
09:37:50.336 rpc 19 subscribeVcsStatus
09:37:50.338 in Ack 18
09:37:50.471 chunk 19 1632 bytes
09:37:50.472 in Ack 19
09:37:50.572 chunk 18 8323 bytes
09:37:50.573 in Ack 18
09:37:50.651 rpc 20 server.reportClientActivity
09:37:50.825 chunk 18 11919 bytes
09:37:50.834 in Ack 18
09:37:50.912 exit 20 {"_tag":"Success","value":null}
09:37:51.041 in Ping 
09:37:51.075 chunk 18 13498 bytes
09:37:51.079 in Ack 18
09:37:51.338 rpc 21 attachments.createUploadUrl
09:37:51.427 fetch POST /api/observability/v1/traces
09:37:51.427 fetched POST /api/observability/v1/traces 204
09:37:51.428 chunk 18 16151 bytes
09:37:51.441 in Ack 18
09:37:51.575 exit 21 {"_tag":"Success","value":{"attachmentId":"pending-1cdd0627-7083-41d7-8eec-5c63dd7c7d6d","expiresAt":1794217671500,"relativeUrl":"/api/attachments/upload/eyJ2ZX
09:37:51.577 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTFjZGQwNjI3LTcwODMtNDFkNy04ZWVjLTVjNjNkZDdjN2Q2ZCIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDIxNzY3MTUwMH0.gB6OE2o_iTs3y4wdsu-kNSmotMjJNClkt_LbKYXYBNE
09:37:51.677 chunk 18 19797 bytes
09:37:51.678 in Ack 18
09:37:51.692 in Interrupt 19
09:37:51.699 rpc 22 subscribeVcsStatus
09:37:51.722 chunk 22 1632 bytes
09:37:51.722 in Ack 22
09:37:51.927 chunk 18 21100 bytes
09:37:51.929 in Ack 18
09:37:51.951 rpc 23 server.reportClientActivity
09:37:52.529 fetch POST /api/observability/v1/traces
09:37:52.529 fetched POST /api/observability/v1/traces 204
09:37:52.703 exit 23 {"_tag":"Success","value":null}
09:37:53.710 fetch POST /api/observability/v1/traces
09:37:53.711 fetched POST /api/observability/v1/traces 204
09:37:54.730 rpc 24 orchestration.dispatchCommand message.dispatch
09:37:55.045 exit 24 {"_tag":"Success","value":{"sequence":110}}
09:37:55.048 rpc 25 attachments.delete
09:37:55.296 chunk 12 4721 bytes
09:37:55.296 chunk 18 25369 bytes
09:37:55.297 in Ack 12
09:37:55.344 in Ack 18
09:37:55.347 rpc 26 assets.createUrl
09:37:55.546 chunk 18 26812 bytes
09:37:55.548 in Ack 18
09:37:55.608 exit 25 {"_tag":"Success","value":null}
09:37:55.729 fetch POST /api/observability/v1/traces
09:37:55.729 fetched POST /api/observability/v1/traces 204
09:37:55.797 chunk 18 30180 bytes
09:37:55.799 in Ack 18
09:37:56.041 in Ping 
09:37:56.046 chunk 12 4660 bytes
09:37:56.047 chunk 18 30202 bytes
09:37:56.047 in Ack 12
09:37:56.056 in Ack 18
09:37:56.304 chunk 18 32035 bytes
09:37:56.306 in Ack 18
09:37:56.554 chunk 12 4657 bytes
09:37:56.555 chunk 18 34458 bytes
09:37:56.556 in Ack 12
09:37:56.558 rpc 27 orchestration.dispatchCommand thread.visit
09:37:56.558 exit 27 {"_tag":"Success","value":{"sequence":0}}
09:37:56.573 in Ack 18
09:37:56.800 fetch POST /api/observability/v1/traces
09:37:56.800 fetched POST /api/observability/v1/traces 204
09:37:56.804 chunk 12 4657 bytes
09:37:56.805 in Ack 12
09:37:56.812 chunk 18 35260 bytes
09:37:56.814 in Ack 18
09:37:56.824 in Interrupt 22
09:37:56.828 rpc 28 subscribeVcsStatus
09:37:56.881 chunk 28 1713 bytes
09:37:56.882 in Ack 28
09:37:57.080 rpc 29 server.reportClientActivity
09:37:57.427 exit 29 {"_tag":"Success","value":null}
09:37:57.427 exit 26 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/1df13302-b282-4cd5-8ea9-786eee1c6ffa","expiresAt":1794220677427}}
09:37:57.819 fetch POST /api/observability/v1/traces
09:37:57.819 fetched POST /api/observability/v1/traces 204
09:37:58.348 rpc 30 server.refreshProviders
09:37:58.348 exit 30 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-09T09:37:22.617Z","configurableProviders":false,"con
09:37:59.147 rpc 31 orchestration.dispatchCommand message.dispatch
09:37:59.348 fetch POST /api/observability/v1/traces
09:37:59.348 fetched POST /api/observability/v1/traces 204
09:38:00.318 chunk 12 4701 bytes
09:38:00.320 in Ack 12
09:38:00.570 chunk 12 4721 bytes
09:38:00.573 in Ack 12
09:38:00.624 exit 31 {"_tag":"Success","value":{"sequence":152}}
09:38:00.824 chunk 18 37281 bytes
09:38:00.834 in Ack 18
09:38:01.050 in Ping 
09:38:01.075 chunk 12 4765 bytes
09:38:01.075 chunk 18 38684 bytes
09:38:01.076 in Ack 12
09:38:01.088 in Ack 18
09:38:01.325 fetch POST /api/observability/v1/traces
09:38:01.325 fetched POST /api/observability/v1/traces 204
09:38:01.326 chunk 18 39209 bytes
09:38:01.328 in Ack 18
09:38:01.579 chunk 18 39973 bytes
09:38:01.586 in Ack 18
09:38:02.329 fetch POST /api/observability/v1/traces
09:38:02.329 fetched POST /api/observability/v1/traces 204
09:38:02.330 chunk 12 4785 bytes
09:38:02.331 chunk 18 40025 bytes
09:38:02.331 in Ack 12
09:38:02.341 in Ack 18
09:38:02.581 chunk 18 40046 bytes
09:38:02.583 in Ack 18
09:38:03.085 chunk 18 40068 bytes
09:38:03.087 in Ack 18
09:38:03.333 fetch POST /api/observability/v1/traces
09:38:03.333 fetched POST /api/observability/v1/traces 204
09:38:03.588 chunk 18 40070 bytes
09:38:03.589 in Ack 18
09:38:03.839 chunk 12 4785 bytes
09:38:03.840 in Ack 12
09:38:04.090 chunk 18 40725 bytes
09:38:04.091 in Ack 18
09:38:04.591 fetch POST /api/observability/v1/traces
09:38:04.591 fetched POST /api/observability/v1/traces 204
09:38:06.052 in Ping 
09:38:06.563 rpc 32 orchestration.dispatchCommand thread.visit
09:38:06.563 exit 32 {"_tag":"Success","value":{"sequence":0}}
09:38:07.563 fetch POST /api/observability/v1/traces
09:38:07.563 fetched POST /api/observability/v1/traces 204
09:38:07.899 chunk 12 4785 bytes
09:38:07.899 in Ack 12
09:38:07.908 chunk 18 40725 bytes
09:38:07.910 in Ack 18
09:38:08.901 fetch POST /api/observability/v1/traces
09:38:08.901 fetched POST /api/observability/v1/traces 204
09:38:11.053 in Ping 
09:38:11.281 rpc 33 server.reportClientActivity
09:38:12.281 fetch POST /api/observability/v1/traces
09:38:12.281 fetched POST /api/observability/v1/traces 204
09:38:13.228 chunk 12 4785 bytes
09:38:13.229 in Ack 12
09:38:14.230 fetch POST /api/observability/v1/traces
09:38:14.230 fetched POST /api/observability/v1/traces 204
09:38:14.551 chunk 18 44138 bytes
09:38:14.553 in Ack 18
09:38:14.802 chunk 12 4785 bytes
09:38:14.803 chunk 18 44090 bytes
09:38:14.803 in Ack 12
09:38:14.808 in Ack 18
09:38:15.053 chunk 18 45134 bytes
09:38:15.056 in Ack 18
09:38:15.556 fetch POST /api/observability/v1/traces
09:38:15.556 fetched POST /api/observability/v1/traces 204
09:38:16.053 in Ping 
09:38:16.562 rpc 34 orchestration.dispatchCommand thread.visit
09:38:16.562 exit 34 {"_tag":"Success","value":{"sequence":0}}
09:38:16.903 chunk 12 4785 bytes
09:38:16.904 in Ack 12
09:38:17.529 exit 33 {"_tag":"Success","value":null}
09:38:17.563 fetch POST /api/observability/v1/traces
09:38:17.563 fetched POST /api/observability/v1/traces 204
09:38:19.335 chunk 18 45134 bytes
09:38:19.337 in Ack 18
09:38:19.585 chunk 12 4785 bytes
09:38:19.585 in Ack 12
09:38:20.337 fetch POST /api/observability/v1/traces
09:38:20.337 fetched POST /api/observability/v1/traces 204
09:38:21.053 in Ping 
09:38:26.054 in Ping 
09:38:28.531 fetch POST /api/observability/v1/traces
09:38:28.531 fetched POST /api/observability/v1/traces 204
09:38:30.920 chunk 12 5092 bytes
09:38:30.921 in Ack 12
09:38:31.054 in Ping 
09:38:31.922 fetch POST /api/observability/v1/traces
09:38:31.922 fetched POST /api/observability/v1/traces 204
09:38:32.143 chunk 18 46838 bytes
09:38:32.145 in Ack 18
09:38:32.396 chunk 18 47666 bytes
09:38:32.398 in Ack 18
09:38:32.919 chunk 18 48668 bytes
09:38:32.922 in Ack 18
09:38:33.146 fetch POST /api/observability/v1/traces
09:38:33.146 fetched POST /api/observability/v1/traces 204
09:38:33.391 rpc 35 orchestration.dispatchCommand runtime-request.respond
09:38:33.806 chunk 18 52155 bytes
09:38:33.809 in Ack 18
09:38:34.391 fetch POST /api/observability/v1/traces
09:38:34.391 fetched POST /api/observability/v1/traces 204
09:38:35.057 chunk 18 54224 bytes
09:38:35.059 in Ack 18
09:38:36.054 in Ping 
09:38:36.060 fetch POST /api/observability/v1/traces
09:38:36.060 fetched POST /api/observability/v1/traces 204
09:38:36.281 rpc 36 server.reportClientActivity
09:38:37.281 fetch POST /api/observability/v1/traces
09:38:37.281 fetched POST /api/observability/v1/traces 204
09:38:38.458 chunk 18 54274 bytes
09:38:38.461 in Ack 18
09:38:38.539 fetch POST /api/observability/v1/traces
09:38:38.539 fetched POST /api/observability/v1/traces 204
09:38:40.343 exit 35 {"_tag":"Success","value":{"sequence":187}}
09:38:40.782 rpc 37 orchestration.dispatchCommand runtime-request.respond
09:38:41.058 in Ping 
09:38:41.344 fetch POST /api/observability/v1/traces
09:38:41.344 fetched POST /api/observability/v1/traces 204
09:38:46.067 in Ping 
09:38:51.070 in Ping 
09:38:51.282 in Interrupt 36
09:38:52.287 fetch POST /api/observability/v1/traces
09:38:52.287 fetched POST /api/observability/v1/traces 204
09:38:56.074 in Ping 
09:39:01.078 in Ping 
09:39:01.292 rpc 38 server.reportClientActivity
09:39:02.282 fetch POST /api/observability/v1/traces
09:39:02.282 fetched POST /api/observability/v1/traces 204
09:39:04.407 chunk 18 57230 bytes
09:39:04.410 in Ack 18
09:39:04.831 chunk 18 57636 bytes
09:39:04.833 in Ack 18
09:39:05.361 chunk 18 58188 bytes
09:39:05.364 in Ack 18
09:39:05.411 fetch POST /api/observability/v1/traces
09:39:05.412 fetched POST /api/observability/v1/traces 204
09:39:06.082 in Ping 
