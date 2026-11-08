# case50, webapp, warm start, seed 1

Profile case50 (graded): graded: 50% loss, 100 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: done** after 47 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | 1.0s |
| Prompt reached the agent | 0.7s |
| Time to first text | 2.9s |
| Whole reply shown | 45.9s |
| Reply shown / produced / total (chars) | 1596 / 1596 / 1596 |
| Newest-text lag: max / median / p95 | 4.1s / 1.2s / 2.6s |
| Longest silence | 4.3s |
| Approval shown after it was asked | 1.1s |
| Bytes on the wire down / up | 231852 / 81514 |
| Bytes the app received / sent | 186594 / 61875 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 3.6 | 226 | 373 |
| 20 | 1.5 | 710 | 774 |
| 30 | 0.0 | 1077 | 1077 |
| 40 | 0.8 | 1418 | 1446 |
| 45 | 0.9 | 1583 | 1596 |

## Driver notes

- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 272/1058","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"660/660 660/660","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 576/809","messages":"456/456 546/546","checkpoints":"779/779 613/613","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794165336745,"silences":0,"longestSilenceMs":1176,"longestRecoveredGapMs":1176},"stats":{"sendRate":4500,"deliveredRate":1839.4495412844035,"loss":0,"srttMs":3.9994127551918863,"minRttMs":1,"sentPackets":75,"sentWireBytes":25129,"receivedPackets":91,"receivedWireBytes":43287,"lastHeardMs":8862,"unsyncedBytes":509,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- prompt sent
- prompt confirmed
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 708/849 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971","assistant":"32/32 21/21 355/415","user":"10/10 9/9","attempts":"660/660 660/660 45/636","providerTurns":"774/774","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 597/809","messages":"456/456 546/546","checkpoints":"779/779 613/613 190/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794165346254,"silences":1,"longestSilenceMs":2588,"longestRecoveredGapMs":2588},"stats":{"sendRate":2250,"deliveredRate":851.7191841911007,"loss":0.3022636363636364,"srttMs":265.55038784185706,"minRttMs":1,"sentPackets":115,"sentWireBytes":33696,"receivedPackets":118,"receivedWireBytes":54219,"lastHeardMs":18370,"unsyncedBytes":0,"inflightPackets":7,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073","frame":"1030/1030","thread":"441/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 933/948","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 120/656","providerThreads":"808/808","providerSessions":"2693/2693","nodes":"1019/1019 809/809 597/809","messages":"456/456 546/546 300/545","checkpoints":"779/779 613/613 190/779","checkpointScopes":"731/731"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794165356896,"silences":1,"longestSilenceMs":2588,"longestRecoveredGapMs":2588},"stats":{"sendRate":1125,"deliveredRate":1018.1124880838895,"loss":0.3076923076923077,"srttMs":592.5276449371277,"minRttMs":1,"sentPackets":171,"sentWireBytes":44368,"receivedPackets":191,"receivedWireBytes":76046,"lastHeardMs":29013,"unsyncedBytes":0,"inflightPackets":15,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1073/1073 1546/1546 490/926","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 43/43","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 728/774","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 785/785 1019/1019","messages":"456/456 546/546 545/545 451/451 546/546 644/901","checkpoints":"613/613 613/613 596/779","checkpointScopes":"731/731","runtimeRequests":"439/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794165367940,"silences":1,"longestSilenceMs":2588,"longestRecoveredGapMs":2588},"stats":{"sendRate":1687.5,"deliveredRate":512.3222748815166,"loss":0.6363636363636364,"srttMs":672.128143431163,"minRttMs":743,"sentPackets":225,"sentWireBytes":56114,"receivedPackets":314,"receivedWireBytes":134816,"lastHeardMs":40056,"unsyncedBytes":630,"inflightPackets":32,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1098/1098 1058/1058 860/860 1299/1299 1098/1098 1058/1058 849/849 1098/1098 1570/1570 950/950 1073/1073","frame":"1030/1030","thread":"951/951","runs":"1014/1014 971/971 839/839","assistant":"32/32 21/21 1340/1340 599/599","user":"10/10 9/9 64/64","attempts":"660/660 660/660 636/636","providerTurns":"774/774 656/656 774/774","providerThreads":"810/810","providerSessions":"2693/2693","nodes":"1019/1019 809/809 809/809 785/785 1019/1019 985/985 1395/1395 1019/1019","messages":"456/456 546/546 546/546 451/451 546/546 901/901 545/545","checkpoints":"613/613 613/613 779/779","checkpointScopes":"731/731","runtimeRequests":"1048/1048"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":289,"lastDataAt":1794165379110,"silences":1,"longestSilenceMs":2588,"longestRecoveredGapMs":2588},"stats":{"sendRate":2592.19495919955,"deliveredRate":726.637806853826,"loss":0.5795236363636362,"srttMs":706.9073905047204,"minRttMs":594,"sentPackets":308,"sentWireBytes":78049,"receivedPackets":423,"receivedWireBytes":197583,"lastHeardMs":51226,"unsyncedBytes":497,"inflightPackets":72,"duplicateBytes":0}}
- shim trace
19:15:28.121 fetch GET /api/auth/session
19:15:28.122 fetched GET /api/auth/session 200
19:15:28.235 fetch GET /.well-known/t3/environment
19:15:28.235 fetched GET /.well-known/t3/environment 200
19:15:28.243 fetch GET /.well-known/t3/environment
19:15:28.244 fetched GET /.well-known/t3/environment 200
19:15:28.247 rpc 0 subscribeServerConfig
19:15:28.247 chunk 0 38710 bytes
19:15:28.275 in Ack 0
19:15:28.277 rpc 1 subscribeServerLifecycle
19:15:28.278 rpc 2 subscribeServerLifecycle
19:15:28.281 fetch GET /api/auth/session
19:15:28.282 fetched GET /api/auth/session 200
19:15:28.282 rpc 3 subscribeDeviceState
19:15:28.293 rpc 4 preview.reportProfiles
19:15:28.346 rpc 5 subscribeProjectClones
19:15:28.363 fetch GET /api/orchestration/threads/mcp%3Ae097341d-586a-487e-82d4-86e5b0ab7865/bounded
19:15:28.368 fetch GET /api/orchestration/shell
19:15:28.388 rpc 6 server.refreshProviders
19:15:28.425 rpc 7 orchestration.dispatchCommand thread.visit
19:15:28.425 exit 7 {"_tag":"Success","value":{"sequence":0}}
19:15:28.435 fetched GET /api/orchestration/shell 200
19:15:28.435 rpc 8 subscribeTerminalMetadata
19:15:28.436 rpc 9 subscribeVcsStatus
19:15:28.436 rpc 10 subscribePreviewEvents
19:15:28.439 rpc 11 orchestration.subscribeShell
19:15:28.439 chunk 11 2787 bytes
19:15:28.440 rpc 12 assets.createUrl
19:15:28.440 rpc 13 preview.list
19:15:28.441 rpc 14 projects.readFile
19:15:28.448 in Ack 11
19:15:28.558 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"780c2f1f-e7d6-443a-b7c4-bb3b735c642d","sessions":[]}}
19:15:28.662 chunk 1 3869 bytes
19:15:28.663 in Ack 1
19:15:28.689 rpc 15 server.reportClientActivity
19:15:28.689 chunk 11 2763 bytes
19:15:28.690 in Ack 11
19:15:29.010 chunk 2 3869 bytes
19:15:29.011 in Ack 2
19:15:29.129 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
19:15:29.129 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T19:15:15.937Z","configurableProviders":false,"con
19:15:29.129 chunk 9 713 bytes
19:15:29.129 exit 12 {"_tag":"Success","value":{"expiresAt":1794168000000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
19:15:29.132 in Ack 9
19:15:29.234 fetch POST /api/observability/v1/traces
19:15:29.234 fetched POST /api/observability/v1/traces 204
19:15:29.378 chunk 3 876 bytes
19:15:29.378 exit 4 {"_tag":"Success","value":null}
19:15:29.378 chunk 5 4 bytes
19:15:29.378 chunk 8 36 bytes
19:15:29.378 in Ack 3
19:15:29.382 rpc 16 preview.reportProfiles
19:15:29.382 in Ack 5
19:15:29.382 in Ack 8
19:15:29.438 exit 15 {"_tag":"Success","value":null}
19:15:29.438 exit 16 {"_tag":"Success","value":null}
19:15:29.500 chunk 11 2806 bytes
19:15:29.500 in Ack 11
19:15:29.519 fetched GET /api/orchestration/threads/mcp%3Ae097341d-586a-487e-82d4-86e5b0ab7865/bounded 200
19:15:29.522 rpc 17 orchestration.subscribeThread
19:15:29.522 chunk 17 8283 bytes
19:15:29.531 in Interrupt 9
19:15:29.533 rpc 18 subscribeVcsStatus
19:15:29.533 in Ack 17
19:15:29.638 chunk 18 713 bytes
19:15:29.638 in Ack 18
19:15:29.773 chunk 17 11920 bytes
19:15:29.774 in Ack 17
19:15:29.825 rpc 19 server.reportClientActivity
19:15:29.910 exit 19 {"_tag":"Success","value":null}
19:15:30.023 chunk 17 13502 bytes
19:15:30.026 in Ack 17
19:15:30.124 rpc 20 attachments.createUploadUrl
19:15:30.285 chunk 17 18602 bytes
19:15:30.292 in Ack 17
19:15:30.329 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-06041c9c-060b-46ed-b6bf-4edd6ade9dab","expiresAt":1794165930298,"relativeUrl":"/api/attachments/upload/eyJ2ZX
19:15:30.330 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLTA2MDQxYzljLTA2MGItNDZlZC1iNmJmLTRlZGQ2YWRlOWRhYiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2NTkzMDI5OH0.G6YTY2LjIWWODTtbOxpYBJ5hP_GIQw5ihpAuksO18ts
19:15:30.382 fetch POST /api/observability/v1/traces
19:15:30.382 fetched POST /api/observability/v1/traces 204
19:15:30.536 chunk 17 21193 bytes
19:15:30.537 in Ack 17
19:15:30.543 in Interrupt 18
19:15:30.545 rpc 21 subscribeVcsStatus
19:15:30.598 chunk 21 713 bytes
19:15:30.598 in Ack 21
19:15:30.795 rpc 22 server.reportClientActivity
19:15:31.517 exit 22 {"_tag":"Success","value":null}
19:15:31.538 fetch POST /api/observability/v1/traces
19:15:31.538 fetched POST /api/observability/v1/traces 204
19:15:33.287 rpc 23 orchestration.dispatchCommand message.dispatch
19:15:33.853 in Ping 
19:15:33.952 exit 23 {"_tag":"Success","value":{"sequence":45}}
19:15:33.954 rpc 24 attachments.delete
19:15:34.286 fetch POST /api/observability/v1/traces
19:15:34.286 fetched POST /api/observability/v1/traces 204
19:15:34.484 chunk 17 23637 bytes
19:15:34.485 in Ack 17
19:15:34.735 chunk 17 23609 bytes
19:15:34.736 in Ack 17
19:15:34.985 chunk 17 23661 bytes
19:15:34.986 in Ack 17
19:15:35.241 chunk 11 2828 bytes
19:15:35.242 in Ack 11
19:15:35.243 rpc 25 orchestration.dispatchCommand thread.visit
19:15:35.243 exit 25 {"_tag":"Success","value":{"sequence":0}}
19:15:35.487 fetch POST /api/observability/v1/traces
19:15:35.487 fetched POST /api/observability/v1/traces 204
19:15:35.742 chunk 11 2828 bytes
19:15:35.742 chunk 17 24633 bytes
19:15:35.742 in Ack 11
19:15:35.748 in Ack 17
19:15:36.746 fetch POST /api/observability/v1/traces
19:15:36.746 fetched POST /api/observability/v1/traces 204
19:15:36.841 rpc 26 orchestration.dispatchCommand message.dispatch
19:15:36.878 chunk 17 28087 bytes
19:15:36.879 in Ack 17
19:15:36.881 rpc 27 assets.createUrl
19:15:37.172 chunk 17 30510 bytes
19:15:37.173 in Ack 17
19:15:37.311 exit 24 {"_tag":"Success","value":null}
19:15:37.704 exit 26 {"_tag":"Success","value":{"sequence":75}}
19:15:37.842 fetch POST /api/observability/v1/traces
19:15:37.842 fetched POST /api/observability/v1/traces 204
19:15:38.112 chunk 17 32946 bytes
19:15:38.116 in Ack 17
19:15:38.361 chunk 17 32968 bytes
19:15:38.362 in Ack 17
19:15:38.708 chunk 17 32980 bytes
19:15:38.709 in Ack 17
19:15:38.853 in Ping 
19:15:39.025 chunk 17 32992 bytes
19:15:39.027 in Ack 17
19:15:39.117 fetch POST /api/observability/v1/traces
19:15:39.117 fetched POST /api/observability/v1/traces 204
19:15:39.275 chunk 17 33012 bytes
19:15:39.277 in Ack 17
19:15:39.525 chunk 17 33094 bytes
19:15:39.526 in Ack 17
19:15:39.826 chunk 17 33124 bytes
19:15:39.827 in Ack 17
19:15:40.205 chunk 17 33146 bytes
19:15:40.210 in Ack 17
19:15:40.278 fetch POST /api/observability/v1/traces
19:15:40.278 fetched POST /api/observability/v1/traces 204
19:15:40.456 chunk 17 33156 bytes
19:15:40.457 in Ack 17
19:15:40.713 chunk 17 33176 bytes
19:15:40.715 in Ack 17
19:15:41.125 chunk 17 33188 bytes
19:15:41.127 in Ack 17
19:15:41.376 chunk 17 33198 bytes
19:15:41.378 in Ack 17
19:15:41.458 fetch POST /api/observability/v1/traces
19:15:41.458 fetched POST /api/observability/v1/traces 204
19:15:41.627 chunk 17 33162 bytes
19:15:41.628 in Ack 17
19:15:42.089 chunk 17 33260 bytes
19:15:42.090 in Ack 17
19:15:42.628 fetch POST /api/observability/v1/traces
19:15:42.629 fetched POST /api/observability/v1/traces 204
19:15:42.645 chunk 17 33340 bytes
19:15:42.646 in Ack 17
19:15:42.895 chunk 17 33350 bytes
19:15:42.896 in Ack 17
19:15:43.145 chunk 17 33388 bytes
19:15:43.146 in Ack 17
19:15:43.396 chunk 17 33446 bytes
19:15:43.396 in Ack 17
19:15:43.647 fetch POST /api/observability/v1/traces
19:15:43.647 fetched POST /api/observability/v1/traces 204
19:15:43.854 in Ping 
19:15:45.971 chunk 17 33590 bytes
19:15:45.972 in Ack 17
19:15:46.254 chunk 17 33600 bytes
19:15:46.256 in Ack 17
19:15:46.973 fetch POST /api/observability/v1/traces
19:15:46.973 fetched POST /api/observability/v1/traces 204
19:15:46.977 chunk 17 33680 bytes
19:15:46.979 in Ack 17
19:15:47.228 chunk 17 33816 bytes
19:15:47.228 in Ack 17
19:15:47.490 chunk 17 33846 bytes
19:15:47.491 in Ack 17
19:15:47.866 chunk 17 33886 bytes
19:15:47.867 in Ack 17
19:15:47.980 fetch POST /api/observability/v1/traces
19:15:47.980 fetched POST /api/observability/v1/traces 204
19:15:48.116 chunk 17 33906 bytes
19:15:48.117 in Ack 17
19:15:48.366 chunk 17 33926 bytes
19:15:48.367 in Ack 17
19:15:48.616 chunk 17 33956 bytes
19:15:48.617 in Ack 17
19:15:48.854 in Ping 
19:15:48.867 chunk 17 33976 bytes
19:15:48.868 in Ack 17
19:15:49.118 fetch POST /api/observability/v1/traces
19:15:49.118 fetched POST /api/observability/v1/traces 204
19:15:49.118 chunk 17 34006 bytes
19:15:49.119 in Ack 17
19:15:49.368 chunk 17 34026 bytes
19:15:49.369 in Ack 17
19:15:49.618 chunk 17 34048 bytes
19:15:49.619 in Ack 17
19:15:49.870 chunk 17 34068 bytes
19:15:49.871 in Ack 17
19:15:50.120 fetch POST /api/observability/v1/traces
19:15:50.120 fetched POST /api/observability/v1/traces 204
19:15:50.387 chunk 17 34098 bytes
19:15:50.388 in Ack 17
19:15:50.910 chunk 17 34120 bytes
19:15:50.912 in Ack 17
19:15:51.160 chunk 17 34160 bytes
19:15:51.161 in Ack 17
19:15:51.389 fetch POST /api/observability/v1/traces
19:15:51.389 fetched POST /api/observability/v1/traces 204
19:15:51.411 chunk 17 34200 bytes
19:15:51.411 in Ack 17
19:15:51.663 chunk 17 34210 bytes
19:15:51.664 in Ack 17
19:15:51.968 chunk 17 34250 bytes
19:15:51.970 in Ack 17
19:15:52.217 chunk 17 34270 bytes
19:15:52.218 in Ack 17
19:15:52.412 fetch POST /api/observability/v1/traces
19:15:52.412 fetched POST /api/observability/v1/traces 204
19:15:52.467 chunk 17 34290 bytes
19:15:52.468 in Ack 17
19:15:52.718 chunk 17 34330 bytes
19:15:52.720 in Ack 17
19:15:53.220 chunk 17 34408 bytes
19:15:53.220 in Ack 17
19:15:53.469 fetch POST /api/observability/v1/traces
19:15:53.469 fetched POST /api/observability/v1/traces 204
19:15:53.470 chunk 11 2892 bytes
19:15:53.470 chunk 17 34428 bytes
19:15:53.470 in Ack 11
19:15:53.472 rpc 28 orchestration.dispatchCommand thread.visit
19:15:53.472 exit 28 {"_tag":"Success","value":{"sequence":0}}
19:15:53.511 in Ack 17
19:15:53.523 rpc 29 server.reportClientActivity
19:15:53.720 chunk 17 34448 bytes
19:15:53.721 in Ack 17
19:15:53.854 in Ping 
19:15:53.972 chunk 17 34478 bytes
19:15:53.973 in Ack 17
19:15:54.223 chunk 17 35137 bytes
19:15:54.224 in Ack 17
19:15:54.471 fetch POST /api/observability/v1/traces
19:15:54.471 fetched POST /api/observability/v1/traces 204
19:15:54.474 chunk 17 35987 bytes
19:15:54.475 in Ack 17
19:15:54.724 chunk 17 36017 bytes
19:15:54.725 in Ack 17
19:15:54.974 chunk 17 38058 bytes
19:15:54.975 in Ack 17
19:15:55.224 chunk 17 38088 bytes
19:15:55.225 in Ack 17
19:15:55.474 chunk 17 38118 bytes
19:15:55.476 in Ack 17
19:15:55.484 fetch POST /api/observability/v1/traces
19:15:55.484 fetched POST /api/observability/v1/traces 204
19:15:55.725 chunk 17 38128 bytes
19:15:55.726 in Ack 17
19:15:55.975 chunk 17 38160 bytes
19:15:55.976 in Ack 17
19:15:56.236 chunk 17 38180 bytes
19:15:56.237 in Ack 17
19:15:56.487 chunk 17 38190 bytes
19:15:56.488 in Ack 17
19:15:56.727 fetch POST /api/observability/v1/traces
19:15:56.727 fetched POST /api/observability/v1/traces 204
19:15:56.737 chunk 17 38222 bytes
19:15:56.738 in Ack 17
19:15:56.987 chunk 17 38252 bytes
19:15:56.988 in Ack 17
19:15:57.237 chunk 17 38280 bytes
19:15:57.238 in Ack 17
19:15:57.488 chunk 17 38308 bytes
19:15:57.488 in Ack 17
19:15:57.737 chunk 11 2892 bytes
19:15:57.738 in Ack 11
19:15:57.742 chunk 17 38338 bytes
19:15:57.742 fetch POST /api/observability/v1/traces
19:15:57.742 fetched POST /api/observability/v1/traces 204
19:15:57.743 in Ack 17
19:15:57.992 chunk 17 38366 bytes
19:15:57.993 in Ack 17
19:15:58.276 chunk 17 38386 bytes
19:15:58.427 in Ack 17
19:15:58.527 chunk 17 38416 bytes
19:15:58.527 in Ack 17
19:15:58.744 fetch POST /api/observability/v1/traces
19:15:58.744 fetched POST /api/observability/v1/traces 204
19:15:58.777 chunk 17 38446 bytes
19:15:58.778 in Ack 17
19:15:58.854 in Ping 
19:15:59.027 chunk 17 40047 bytes
19:15:59.028 in Ack 17
19:15:59.278 chunk 17 40122 bytes
19:15:59.280 in Ack 17
19:15:59.528 chunk 17 40152 bytes
19:15:59.529 in Ack 17
19:15:59.778 chunk 17 40170 bytes
19:15:59.779 fetch POST /api/observability/v1/traces
19:15:59.779 fetched POST /api/observability/v1/traces 204
19:15:59.780 in Ack 17
19:16:00.028 chunk 17 40986 bytes
19:16:00.029 in Ack 17
19:16:00.278 chunk 17 41004 bytes
19:16:00.280 in Ack 17
19:16:00.336 exit 29 {"_tag":"Success","value":null}
19:16:00.528 chunk 17 41019 bytes
19:16:00.529 in Ack 17
19:16:00.778 chunk 17 41118 bytes
19:16:00.779 in Ack 17
19:16:00.808 fetch POST /api/observability/v1/traces
19:16:00.808 fetched POST /api/observability/v1/traces 204
19:16:01.029 chunk 17 41148 bytes
19:16:01.029 in Ack 17
19:16:01.280 chunk 17 41993 bytes
19:16:01.280 in Ack 17
19:16:01.531 chunk 17 42050 bytes
19:16:01.532 in Ack 17
19:16:01.781 chunk 17 42722 bytes
19:16:01.782 in Ack 17
19:16:02.030 fetch POST /api/observability/v1/traces
19:16:02.030 fetched POST /api/observability/v1/traces 204
19:16:02.031 chunk 17 43350 bytes
19:16:02.031 in Ack 17
19:16:02.281 chunk 17 43380 bytes
19:16:02.282 in Ack 17
19:16:02.532 chunk 17 43425 bytes
19:16:02.533 in Ack 17
19:16:02.782 chunk 17 43986 bytes
19:16:02.783 in Ack 17
19:16:03.038 chunk 17 44016 bytes
19:16:03.038 fetch POST /api/observability/v1/traces
19:16:03.039 fetched POST /api/observability/v1/traces 204
19:16:03.040 in Ack 17
19:16:03.288 chunk 17 44046 bytes
19:16:03.289 in Ack 17
19:16:03.538 chunk 17 43925 bytes
19:16:03.539 in Ack 17
19:16:03.545 in Interrupt 21
19:16:03.548 rpc 30 subscribeVcsStatus
19:16:03.788 chunk 17 43928 bytes
19:16:03.789 in Ack 17
19:16:03.799 rpc 31 server.reportClientActivity
19:16:03.855 in Ping 
19:16:04.039 chunk 17 43961 bytes
19:16:04.040 in Ack 17
19:16:04.048 fetch POST /api/observability/v1/traces
19:16:04.048 fetched POST /api/observability/v1/traces 204
19:16:04.288 chunk 17 45026 bytes
19:16:04.289 in Ack 17
19:16:04.326 chunk 30 713 bytes
19:16:04.327 in Ack 30
19:16:04.538 chunk 17 45095 bytes
19:16:04.539 in Ack 17
19:16:04.790 chunk 17 45137 bytes
19:16:04.790 in Ack 17
19:16:05.289 fetch POST /api/observability/v1/traces
19:16:05.289 fetched POST /api/observability/v1/traces 204
19:16:05.290 chunk 17 46142 bytes
19:16:05.291 in Ack 17
19:16:05.540 chunk 17 49638 bytes
19:16:05.541 in Ack 17
19:16:06.292 fetch POST /api/observability/v1/traces
19:16:06.292 fetched POST /api/observability/v1/traces 204
19:16:06.677 rpc 32 orchestration.dispatchCommand runtime-request.respond
19:16:07.658 chunk 17 52109 bytes
19:16:07.658 exit 32 {"_tag":"Success","value":{"sequence":96}}
19:16:07.659 in Ack 17
19:16:07.700 fetch POST /api/observability/v1/traces
19:16:07.700 fetched POST /api/observability/v1/traces 204
19:16:07.908 chunk 17 52129 bytes
19:16:07.909 in Ack 17
19:16:07.959 rpc 33 orchestration.dispatchCommand runtime-request.respond
19:16:08.158 chunk 17 52149 bytes
19:16:08.159 in Ack 17
19:16:08.408 chunk 17 52179 bytes
19:16:08.409 in Ack 17
19:16:08.658 chunk 17 52189 bytes
19:16:08.659 in Ack 17
19:16:08.855 in Ping 
19:16:08.910 chunk 17 52231 bytes
19:16:08.910 fetch POST /api/observability/v1/traces
19:16:08.910 fetched POST /api/observability/v1/traces 204
19:16:08.911 in Ack 17
19:16:09.159 chunk 17 52294 bytes
19:16:09.160 in Ack 17
19:16:09.445 exit 33 {"_tag":"Failure","cause":[{"_tag":"Fail","error":{"_tag":"OrchestrationV2DispatchCommandError","cause":{"cause":"Runtime request runtime-request:provider:acpRe
19:16:09.672 chunk 11 2894 bytes
19:16:09.673 chunk 17 53111 bytes
19:16:09.673 in Ack 11
19:16:09.674 rpc 34 orchestration.dispatchCommand thread.visit
19:16:09.674 exit 34 {"_tag":"Success","value":{"sequence":0}}
19:16:09.676 in Ack 17
19:16:09.921 fetch POST /api/observability/v1/traces
19:16:09.921 fetched POST /api/observability/v1/traces 204
19:16:09.923 chunk 17 53149 bytes
19:16:09.924 in Ack 17
19:16:10.173 chunk 17 53219 bytes
19:16:10.174 in Ack 17
19:16:10.550 chunk 17 53301 bytes
19:16:10.551 in Ack 17
19:16:10.799 chunk 17 54101 bytes
19:16:10.800 in Ack 17
19:16:10.807 in Interrupt 30
19:16:10.810 rpc 35 subscribeVcsStatus
19:16:10.925 fetch POST /api/observability/v1/traces
19:16:10.925 fetched POST /api/observability/v1/traces 204
19:16:11.060 chunk 17 54113 bytes
19:16:11.062 in Ack 17
19:16:11.310 chunk 17 54151 bytes
19:16:11.311 in Ack 17
19:16:11.560 chunk 11 2894 bytes
19:16:11.561 chunk 17 54179 bytes
19:16:11.561 in Ack 11
19:16:11.566 in Ack 17
19:16:11.611 chunk 35 713 bytes
19:16:11.611 in Ack 35
19:16:11.811 chunk 17 54169 bytes
19:16:11.813 in Ack 17
19:16:12.078 chunk 17 54197 bytes
19:16:12.078 fetch POST /api/observability/v1/traces
19:16:12.078 fetched POST /api/observability/v1/traces 204
19:16:12.081 in Ack 17
19:16:12.337 chunk 17 54207 bytes
19:16:12.339 in Ack 17
19:16:12.588 chunk 17 54229 bytes
19:16:12.589 in Ack 17
19:16:12.838 chunk 17 54241 bytes
19:16:12.839 in Ack 17
19:16:13.084 fetch POST /api/observability/v1/traces
19:16:13.084 fetched POST /api/observability/v1/traces 204
19:16:13.089 chunk 17 54253 bytes
19:16:13.090 in Ack 17
19:16:13.339 chunk 17 58358 bytes
19:16:13.340 in Ack 17
19:16:13.348 in Interrupt 35
19:16:13.375 rpc 36 subscribeVcsStatus
19:16:13.601 chunk 17 58388 bytes
19:16:13.602 in Ack 17
19:16:13.852 chunk 17 58398 bytes
19:16:13.853 in Ack 17
19:16:13.863 in Ping 
19:16:14.091 fetch POST /api/observability/v1/traces
19:16:14.091 fetched POST /api/observability/v1/traces 204
19:16:14.102 chunk 17 59438 bytes
19:16:14.103 in Ack 17
19:16:14.274 chunk 36 713 bytes
19:16:14.274 in Ack 36
19:16:14.352 chunk 17 59460 bytes
19:16:14.353 in Ack 17
19:16:14.602 chunk 17 59498 bytes
19:16:14.603 in Ack 17
19:16:14.853 chunk 17 59518 bytes
19:16:14.854 in Ack 17
19:16:15.103 chunk 17 59548 bytes
19:16:15.103 fetch POST /api/observability/v1/traces
19:16:15.103 fetched POST /api/observability/v1/traces 204
19:16:15.105 in Ack 17
19:16:15.353 chunk 17 59574 bytes
19:16:15.354 in Ack 17
19:16:15.559 exit 31 {"_tag":"Success","value":null}
19:16:15.603 chunk 17 59596 bytes
19:16:15.604 in Ack 17
19:16:15.811 rpc 37 server.reportClientActivity
19:16:15.853 chunk 17 59606 bytes
19:16:15.854 in Ack 17
19:16:16.103 chunk 17 59647 bytes
19:16:16.104 in Ack 17
19:16:16.193 fetch POST /api/observability/v1/traces
19:16:16.193 fetched POST /api/observability/v1/traces 204
19:16:16.353 chunk 17 59677 bytes
19:16:16.354 in Ack 17
19:16:16.603 chunk 11 2894 bytes
19:16:16.603 chunk 17 59711 bytes
19:16:16.603 in Ack 11
19:16:16.606 in Ack 17
19:16:16.853 chunk 17 59721 bytes
19:16:16.854 in Ack 17
19:16:17.103 chunk 17 59751 bytes
19:16:17.105 in Ack 17
19:16:17.353 chunk 17 59785 bytes
19:16:17.354 in Ack 17
19:16:17.447 fetch POST /api/observability/v1/traces
19:16:17.447 fetched POST /api/observability/v1/traces 204
19:16:17.604 chunk 17 59805 bytes
19:16:17.605 in Ack 17
19:16:17.854 chunk 17 59825 bytes
19:16:17.855 in Ack 17
19:16:18.104 chunk 17 62354 bytes
19:16:18.105 in Ack 17
19:16:18.355 chunk 17 62414 bytes
19:16:18.356 in Ack 17
19:16:18.483 exit 37 {"_tag":"Success","value":null}
19:16:18.494 rpc 38 server.reportClientActivity
19:16:18.606 fetch POST /api/observability/v1/traces
19:16:18.606 fetched POST /api/observability/v1/traces 204
19:16:18.611 chunk 17 62429 bytes
19:16:18.612 in Ack 17
19:16:18.861 chunk 17 62459 bytes
19:16:18.862 in Ack 17
19:16:18.875 in Ping 
19:16:19.111 chunk 17 62528 bytes
19:16:19.112 in Ack 17
19:16:19.192 exit 27 {"_tag":"Success","value":{"relativeUrl":"blob:https://10.231.0.1:8443/932a7baa-6943-4511-a2b3-7237b0c4fbd5","expiresAt":1794168979192}}
19:16:19.361 chunk 17 62543 bytes
19:16:19.363 in Ack 17
19:16:19.611 chunk 17 62597 bytes
19:16:19.613 in Ack 17
19:16:19.688 fetch POST /api/observability/v1/traces
19:16:19.688 fetched POST /api/observability/v1/traces 204
19:16:19.691 rpc 39 orchestration.dispatchCommand thread.visit
19:16:19.691 exit 39 {"_tag":"Success","value":{"sequence":0}}
19:16:19.862 chunk 17 62645 bytes
19:16:19.863 in Ack 17
19:16:20.112 chunk 17 62684 bytes
19:16:20.113 in Ack 17
19:16:20.362 chunk 17 62729 bytes
19:16:20.363 in Ack 17
19:16:20.504 exit 38 {"_tag":"Success","value":null}
19:16:20.612 chunk 17 62744 bytes
19:16:20.613 in Ack 17
19:16:20.691 fetch POST /api/observability/v1/traces
19:16:20.691 fetched POST /api/observability/v1/traces 204
19:16:20.862 chunk 17 62816 bytes
19:16:20.863 in Ack 17
19:16:21.111 chunk 11 2894 bytes
19:16:21.112 in Ack 11
19:16:21.116 chunk 17 62834 bytes
19:16:21.117 in Ack 17
19:16:21.367 chunk 17 62882 bytes
19:16:21.368 in Ack 17
19:16:21.617 chunk 17 62933 bytes
19:16:21.618 in Ack 17
19:16:21.864 fetch POST /api/observability/v1/traces
19:16:21.864 fetched POST /api/observability/v1/traces 204
19:16:21.867 chunk 17 62995 bytes
19:16:21.868 in Ack 17
19:16:22.111 chunk 11 2830 bytes
19:16:22.112 in Ack 11
19:16:22.113 rpc 40 orchestration.dispatchCommand thread.visit
19:16:22.113 exit 40 {"_tag":"Success","value":{"sequence":0}}
19:16:22.119 chunk 17 63127 bytes
19:16:22.120 in Ack 17
19:16:22.369 chunk 17 63151 bytes
19:16:22.370 in Ack 17
19:16:22.619 chunk 17 65599 bytes
19:16:22.620 in Ack 17
