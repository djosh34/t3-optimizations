# E, webapp, cold start, seed 1

Profile E (graded): owner extreme: 1 kbit/s, 75% bursty loss. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.7s |
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 897.2s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 4150 / 8109 |
| Bytes the app received / sent | 0 / 0 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 0.0 | 0 | 0 |
| 30 | 0.0 | 0 | 0 |
| 40 | 0.0 | 0 | 0 |
| 50 | 0.0 | 0 | 0 |
| 60 | 0.0 | 0 | 0 |
| 70 | 0.0 | 0 | 0 |
| 80 | 0.0 | 0 | 0 |
| 90 | 0.0 | 0 | 0 |
| 100 | 0.0 | 0 | 0 |
| 110 | 0.0 | 0 | 0 |
| 120 | 0.0 | 0 | 0 |
| 130 | 0.0 | 0 | 0 |
| 140 | 0.0 | 0 | 0 |
| 150 | 0.0 | 0 | 0 |
| 160 | 0.0 | 0 | 0 |
| 170 | 0.0 | 0 | 0 |
| 180 | 0.0 | 0 | 0 |
| 190 | 0.0 | 0 | 0 |
| 200 | 0.0 | 0 | 0 |
| 210 | 0.0 | 0 | 0 |
| 220 | 0.0 | 0 | 0 |
| 230 | 0.0 | 0 | 0 |
| 240 | 0.0 | 0 | 0 |
| 250 | 0.0 | 0 | 0 |
| 260 | 0.0 | 0 | 0 |
| 270 | 0.0 | 0 | 0 |
| 280 | 0.0 | 0 | 0 |
| 290 | 0.0 | 0 | 0 |
| 300 | 0.0 | 0 | 0 |
| 310 | 0.0 | 0 | 0 |
| 320 | 0.0 | 0 | 0 |
| 330 | 0.0 | 0 | 0 |
| 340 | 0.0 | 0 | 0 |
| 350 | 0.0 | 0 | 0 |
| 360 | 0.0 | 0 | 0 |
| 370 | 0.0 | 0 | 0 |
| 380 | 0.0 | 0 | 0 |
| 390 | 0.0 | 0 | 0 |
| 400 | 0.0 | 0 | 0 |
| 410 | 0.0 | 0 | 0 |
| 420 | 0.0 | 0 | 0 |
| 430 | 0.0 | 0 | 0 |
| 440 | 0.0 | 0 | 0 |
| 450 | 0.0 | 0 | 0 |
| 460 | 0.0 | 0 | 0 |
| 470 | 0.0 | 0 | 0 |
| 480 | 0.0 | 0 | 0 |
| 490 | 0.0 | 0 | 0 |
| 500 | 0.0 | 0 | 0 |
| 510 | 0.0 | 0 | 0 |
| 520 | 0.0 | 0 | 0 |
| 530 | 0.0 | 0 | 0 |
| 540 | 0.0 | 0 | 0 |
| 550 | 0.0 | 0 | 0 |
| 560 | 0.0 | 0 | 0 |
| 570 | 0.0 | 0 | 0 |
| 580 | 0.0 | 0 | 0 |
| 590 | 0.0 | 0 | 0 |
| 600 | 0.0 | 0 | 0 |
| 610 | 0.0 | 0 | 0 |
| 620 | 0.0 | 0 | 0 |
| 630 | 0.0 | 0 | 0 |
| 640 | 0.0 | 0 | 0 |
| 650 | 0.0 | 0 | 0 |
| 660 | 0.0 | 0 | 0 |
| 670 | 0.0 | 0 | 0 |
| 680 | 0.0 | 0 | 0 |
| 690 | 0.0 | 0 | 0 |
| 700 | 0.0 | 0 | 0 |
| 710 | 0.0 | 0 | 0 |
| 720 | 0.0 | 0 | 0 |
| 730 | 0.0 | 0 | 0 |
| 740 | 0.0 | 0 | 0 |
| 750 | 0.0 | 0 | 0 |
| 760 | 0.0 | 0 | 0 |
| 770 | 0.0 | 0 | 0 |
| 780 | 0.0 | 0 | 0 |
| 790 | 0.0 | 0 | 0 |
| 800 | 0.0 | 0 | 0 |
| 810 | 0.0 | 0 | 0 |
| 820 | 0.0 | 0 | 0 |
| 830 | 0.0 | 0 | 0 |
| 840 | 0.0 | 0 | 0 |
| 850 | 0.0 | 0 | 0 |
| 860 | 0.0 | 0 | 0 |
| 870 | 0.0 | 0 | 0 |
| 880 | 0.0 | 0 | 0 |
| 890 | 0.0 | 0 | 0 |
| 897 | 0.0 | 0 | 0 |

## Driver notes

- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2015,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2015,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2507,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2999,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2999,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3492,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3492,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3492,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3985,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3985,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4478,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4478,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4644,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5137,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5137,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5630,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5630,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5630,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6123,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6123,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6616,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6616,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6616,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7109,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7109,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7602,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7602,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7602,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8095,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8095,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8588,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8588,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9081,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9081,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9081,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9574,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9574,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10067,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10067,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10067,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10560,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10560,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11053,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11053,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11053,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11546,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11546,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12039,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12039,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12039,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12532,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12532,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13025,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13025,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13025,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13518,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13518,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14011,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14011,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14011,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14504,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14504,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14997,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14997,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14997,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15490,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15490,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15983,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15983,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15983,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16476,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16476,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16969,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16969,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16969,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17462,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17462,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17955,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17955,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18448,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18448,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18448,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18941,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18941,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19434,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19434,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19434,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19927,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"checkpoints":"613/613 613/613 779/779","user":"10/10 9/9","turnItems":"870/870 1108/1108 1108/1108 1299/1299 1058/1058 860/860 1058/1058","providerTurns":"778/778 778/778","runs":"971/971 1014/1014","frame":"1032/1032","messages":"456/456 550/550 901/901 550/550","assistant":"32/32 21/21","nodes":"809/809 809/809 1025/1025 1025/1025","attempts":"662/662 662/662","checkpointScopes":"726/726","providerThreads":"810/810","providerSessions":"2688/2688","thread":"951/951"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19927,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
15:13:46.365 fetch GET /api/auth/session
15:13:46.369 fetched GET /api/auth/session 200
15:13:46.519 fetch GET /.well-known/t3/environment
15:13:46.521 fetched GET /.well-known/t3/environment 200
15:13:46.549 fetch GET /.well-known/t3/environment
15:13:46.550 fetched GET /.well-known/t3/environment 200
15:13:46.563 rpc 0 subscribeServerConfig
15:13:46.564 chunk 0 38690 bytes
15:13:46.598 in Ack 0
15:13:46.601 rpc 1 subscribeServerLifecycle
15:13:46.602 rpc 2 subscribeServerLifecycle
15:13:46.609 fetch GET /api/auth/session
15:13:46.613 fetched GET /api/auth/session 200
15:13:46.613 rpc 3 subscribeDeviceState
15:13:46.618 rpc 4 preview.reportProfiles
15:13:46.749 rpc 5 subscribeProjectClones
15:13:46.803 rpc 6 server.reportClientActivity
15:13:46.811 rpc 7 orchestration.subscribeThread
15:13:46.814 chunk 7 35300 bytes
15:13:46.825 in Ack 7
15:13:46.843 fetch GET /api/orchestration/shell
15:13:46.902 rpc 8 server.refreshProviders
15:13:47.010 rpc 9 orchestration.dispatchCommand thread.visit
15:13:47.148 fetched GET /api/orchestration/shell 200
15:13:47.149 rpc 10 subscribeTerminalMetadata
15:13:47.150 rpc 11 subscribePreviewEvents
15:13:47.150 rpc 12 subscribeVcsStatus
15:13:47.153 rpc 13 orchestration.subscribeShell
15:13:47.153 chunk 13 2847 bytes
15:13:47.156 rpc 14 assets.createUrl
15:13:47.156 rpc 15 preview.list
15:13:47.157 rpc 16 projects.readFile
15:13:47.157 rpc 17 assets.createUrl
15:13:47.228 in Ack 13
15:13:47.572 fetch POST /api/observability/v1/traces
15:13:47.572 fetched POST /api/observability/v1/traces 204
15:13:48.659 fetch POST /api/observability/v1/traces
15:13:48.659 fetched POST /api/observability/v1/traces 204
15:13:51.558 in Ping 
15:13:56.558 in Ping 
15:14:01.558 in Ping 
15:14:01.804 in Interrupt 6
15:14:02.057 rpc 18 server.reportClientActivity
15:14:02.805 fetch POST /api/observability/v1/traces
15:14:02.805 fetched POST /api/observability/v1/traces 204
15:14:06.559 in Ping 
15:14:11.560 in Ping 
15:14:16.560 in Ping 
15:14:17.058 in Interrupt 18
15:14:17.310 rpc 19 server.reportClientActivity
15:14:18.059 fetch POST /api/observability/v1/traces
15:14:18.059 fetched POST /api/observability/v1/traces 204
15:14:21.561 in Ping 
15:14:26.561 in Ping 
15:14:31.562 in Ping 
15:14:32.310 in Interrupt 19
15:14:33.312 fetch POST /api/observability/v1/traces
15:14:33.312 fetched POST /api/observability/v1/traces 204
15:14:36.562 in Ping 
15:14:36.775 rpc 20 server.reportClientActivity
15:14:37.775 fetch POST /api/observability/v1/traces
15:14:37.775 fetched POST /api/observability/v1/traces 204
15:14:41.565 in Ping 
15:14:46.568 in Ping 
15:14:51.573 in Ping 
15:14:51.775 in Interrupt 20
15:14:52.777 fetch POST /api/observability/v1/traces
15:14:52.777 fetched POST /api/observability/v1/traces 204
15:14:56.577 in Ping 
15:15:01.580 in Ping 
15:15:01.772 rpc 21 server.reportClientActivity
15:15:02.773 fetch POST /api/observability/v1/traces
15:15:02.773 fetched POST /api/observability/v1/traces 204
15:15:06.585 in Ping 
15:15:11.589 in Ping 
15:15:16.592 in Ping 
15:15:16.775 in Interrupt 21
15:15:17.777 fetch POST /api/observability/v1/traces
15:15:17.777 fetched POST /api/observability/v1/traces 204
15:15:21.597 in Ping 
15:15:26.600 in Ping 
15:15:26.775 rpc 22 server.reportClientActivity
15:15:27.776 fetch POST /api/observability/v1/traces
15:15:27.776 fetched POST /api/observability/v1/traces 204
15:15:31.604 in Ping 
15:15:36.608 in Ping 
15:15:41.613 in Ping 
15:15:41.775 in Interrupt 22
15:15:42.777 fetch POST /api/observability/v1/traces
15:15:42.778 fetched POST /api/observability/v1/traces 204
15:15:46.618 in Ping 
15:15:51.620 in Ping 
15:15:51.774 rpc 23 server.reportClientActivity
15:15:52.775 fetch POST /api/observability/v1/traces
15:15:52.775 fetched POST /api/observability/v1/traces 204
15:15:56.624 in Ping 
15:16:01.629 in Ping 
15:16:06.633 in Ping 
15:16:06.775 in Interrupt 23
15:16:07.776 fetch POST /api/observability/v1/traces
15:16:07.776 fetched POST /api/observability/v1/traces 204
15:16:11.636 in Ping 
15:16:16.640 in Ping 
15:16:16.777 rpc 24 server.reportClientActivity
15:16:17.777 fetch POST /api/observability/v1/traces
15:16:17.777 fetched POST /api/observability/v1/traces 204
15:16:21.644 in Ping 
15:16:26.651 in Ping 
15:16:31.654 in Ping 
15:16:31.781 in Interrupt 24
15:16:32.782 fetch POST /api/observability/v1/traces
15:16:32.782 fetched POST /api/observability/v1/traces 204
15:16:36.656 in Ping 
15:16:41.660 in Ping 
15:16:41.777 rpc 25 server.reportClientActivity
15:16:42.777 fetch POST /api/observability/v1/traces
15:16:42.777 fetched POST /api/observability/v1/traces 204
15:16:46.665 in Ping 
15:16:51.668 in Ping 
15:16:56.673 in Ping 
15:16:56.777 in Interrupt 25
15:16:57.778 fetch POST /api/observability/v1/traces
15:16:57.778 fetched POST /api/observability/v1/traces 204
15:17:01.677 in Ping 
15:17:06.680 in Ping 
15:17:06.780 rpc 26 server.reportClientActivity
15:17:07.782 fetch POST /api/observability/v1/traces
15:17:07.782 fetched POST /api/observability/v1/traces 204
15:17:11.686 in Ping 
15:17:16.689 in Ping 
15:17:21.692 in Ping 
15:17:21.781 in Interrupt 26
15:17:22.785 fetch POST /api/observability/v1/traces
15:17:22.785 fetched POST /api/observability/v1/traces 204
15:17:26.697 in Ping 
15:17:31.700 in Ping 
15:17:31.780 rpc 27 server.reportClientActivity
15:17:32.782 fetch POST /api/observability/v1/traces
15:17:32.782 fetched POST /api/observability/v1/traces 204
15:17:36.704 in Ping 
15:17:41.708 in Ping 
15:17:46.712 in Ping 
15:17:46.781 in Interrupt 27
15:17:47.787 fetch POST /api/observability/v1/traces
15:17:47.787 fetched POST /api/observability/v1/traces 204
15:17:51.717 in Ping 
15:17:56.725 in Ping 
15:17:56.796 rpc 28 server.reportClientActivity
15:17:57.793 fetch POST /api/observability/v1/traces
15:17:57.793 fetched POST /api/observability/v1/traces 204
15:18:01.729 in Ping 
15:18:06.738 in Ping 
15:18:11.741 in Ping 
15:18:11.798 in Interrupt 28
15:18:12.801 fetch POST /api/observability/v1/traces
15:18:12.801 fetched POST /api/observability/v1/traces 204
15:18:16.746 in Ping 
15:18:21.749 in Ping 
15:18:21.788 rpc 29 server.reportClientActivity
15:18:22.789 fetch POST /api/observability/v1/traces
15:18:22.789 fetched POST /api/observability/v1/traces 204
15:18:26.752 in Ping 
15:18:31.757 in Ping 
15:18:36.760 in Ping 
15:18:36.793 in Interrupt 29
15:18:37.799 fetch POST /api/observability/v1/traces
15:18:37.799 fetched POST /api/observability/v1/traces 204
15:18:41.765 in Ping 
15:18:46.768 in Ping 
15:18:46.794 rpc 30 server.reportClientActivity
15:18:47.797 fetch POST /api/observability/v1/traces
15:18:47.797 fetched POST /api/observability/v1/traces 204
15:18:51.772 in Ping 
15:18:56.776 in Ping 
15:19:01.780 in Ping 
15:19:01.797 in Interrupt 30
15:19:02.801 fetch POST /api/observability/v1/traces
15:19:02.801 fetched POST /api/observability/v1/traces 204
15:19:06.784 in Ping 
15:19:11.789 in Ping 
15:19:11.793 rpc 31 server.reportClientActivity
15:19:12.797 fetch POST /api/observability/v1/traces
15:19:12.797 fetched POST /api/observability/v1/traces 204
15:19:16.793 in Ping 
15:19:21.796 in Ping 
15:19:26.797 in Interrupt 31
15:19:26.800 in Ping 
15:19:27.804 fetch POST /api/observability/v1/traces
15:19:27.804 fetched POST /api/observability/v1/traces 204
15:19:31.806 in Ping 
15:19:36.798 rpc 32 server.reportClientActivity
15:19:36.808 in Ping 
15:19:37.801 fetch POST /api/observability/v1/traces
15:19:37.801 fetched POST /api/observability/v1/traces 204
15:19:41.812 in Ping 
15:19:46.817 in Ping 
15:19:51.802 in Interrupt 32
15:19:51.820 in Ping 
15:19:52.806 fetch POST /api/observability/v1/traces
15:19:52.806 fetched POST /api/observability/v1/traces 204
15:19:56.824 in Ping 
15:20:01.802 rpc 33 server.reportClientActivity
15:20:01.828 in Ping 
15:20:02.805 fetch POST /api/observability/v1/traces
15:20:02.805 fetched POST /api/observability/v1/traces 204
15:20:06.832 in Ping 
15:20:11.836 in Ping 
15:20:16.805 in Interrupt 33
15:20:16.841 in Ping 
15:20:17.809 fetch POST /api/observability/v1/traces
15:20:17.809 fetched POST /api/observability/v1/traces 204
15:20:21.844 in Ping 
15:20:26.806 rpc 34 server.reportClientActivity
15:20:26.848 in Ping 
15:20:27.809 fetch POST /api/observability/v1/traces
15:20:27.809 fetched POST /api/observability/v1/traces 204
15:20:31.853 in Ping 
15:20:36.857 in Ping 
15:20:41.809 in Interrupt 34
15:20:41.861 in Ping 
15:20:42.813 fetch POST /api/observability/v1/traces
15:20:42.813 fetched POST /api/observability/v1/traces 204
15:20:46.864 in Ping 
15:20:51.810 rpc 35 server.reportClientActivity
15:20:51.872 in Ping 
15:20:52.814 fetch POST /api/observability/v1/traces
15:20:52.814 fetched POST /api/observability/v1/traces 204
15:20:56.872 in Ping 
15:21:01.876 in Ping 
15:21:06.813 in Interrupt 35
15:21:06.881 in Ping 
15:21:07.817 fetch POST /api/observability/v1/traces
15:21:07.818 fetched POST /api/observability/v1/traces 204
15:21:11.885 in Ping 
15:21:16.815 rpc 36 server.reportClientActivity
15:21:16.888 in Ping 
15:21:17.817 fetch POST /api/observability/v1/traces
15:21:17.817 fetched POST /api/observability/v1/traces 204
15:21:21.893 in Ping 
15:21:26.897 in Ping 
15:21:31.817 in Interrupt 36
15:21:31.900 in Ping 
15:21:32.821 fetch POST /api/observability/v1/traces
15:21:32.821 fetched POST /api/observability/v1/traces 204
15:21:36.904 in Ping 
15:21:41.823 rpc 37 server.reportClientActivity
15:21:41.908 in Ping 
15:21:42.825 fetch POST /api/observability/v1/traces
15:21:42.825 fetched POST /api/observability/v1/traces 204
15:21:46.913 in Ping 
15:21:51.916 in Ping 
15:21:56.825 in Interrupt 37
15:21:56.921 in Ping 
15:21:57.829 fetch POST /api/observability/v1/traces
15:21:57.829 fetched POST /api/observability/v1/traces 204
15:22:01.924 in Ping 
15:22:06.822 rpc 38 server.reportClientActivity
15:22:06.928 in Ping 
15:22:07.826 fetch POST /api/observability/v1/traces
15:22:07.826 fetched POST /api/observability/v1/traces 204
15:22:11.932 in Ping 
15:22:16.937 in Ping 
15:22:21.825 in Interrupt 38
15:22:21.940 in Ping 
15:22:22.829 fetch POST /api/observability/v1/traces
15:22:22.829 fetched POST /api/observability/v1/traces 204
15:22:26.944 in Ping 
15:22:31.828 rpc 39 server.reportClientActivity
15:22:31.949 in Ping 
15:22:32.829 fetch POST /api/observability/v1/traces
15:22:32.829 fetched POST /api/observability/v1/traces 204
15:22:36.954 in Ping 
15:22:41.957 in Ping 
15:22:46.829 in Interrupt 39
15:22:46.963 in Ping 
15:22:47.833 fetch POST /api/observability/v1/traces
15:22:47.833 fetched POST /api/observability/v1/traces 204
15:22:51.965 in Ping 
15:22:56.831 rpc 40 server.reportClientActivity
15:22:56.968 in Ping 
15:22:57.833 fetch POST /api/observability/v1/traces
15:22:57.833 fetched POST /api/observability/v1/traces 204
15:23:01.973 in Ping 
15:23:06.976 in Ping 
15:23:11.833 in Interrupt 40
15:23:11.987 in Ping 
15:23:12.839 fetch POST /api/observability/v1/traces
15:23:12.839 fetched POST /api/observability/v1/traces 204
15:23:16.988 in Ping 
15:23:21.834 rpc 41 server.reportClientActivity
15:23:21.992 in Ping 
15:23:22.837 fetch POST /api/observability/v1/traces
15:23:22.837 fetched POST /api/observability/v1/traces 204
15:23:26.996 in Ping 
15:23:32.001 in Ping 
15:23:36.839 in Interrupt 41
15:23:37.005 in Ping 
15:23:37.842 fetch POST /api/observability/v1/traces
15:23:37.842 fetched POST /api/observability/v1/traces 204
15:23:42.008 in Ping 
15:23:46.844 rpc 42 server.reportClientActivity
15:23:47.054 in Ping 
15:23:47.845 fetch POST /api/observability/v1/traces
15:23:47.845 fetched POST /api/observability/v1/traces 204
15:23:52.057 in Ping 
15:23:57.060 in Ping 
15:24:01.849 in Interrupt 42
15:24:02.065 in Ping 
15:24:02.853 fetch POST /api/observability/v1/traces
15:24:02.853 fetched POST /api/observability/v1/traces 204
15:24:07.068 in Ping 
15:24:11.845 rpc 43 server.reportClientActivity
15:24:12.089 in Ping 
15:24:12.847 fetch POST /api/observability/v1/traces
15:24:12.847 fetched POST /api/observability/v1/traces 204
15:24:17.092 in Ping 
15:24:22.096 in Ping 
15:24:26.849 in Interrupt 43
15:24:27.103 in Ping 
15:24:27.853 fetch POST /api/observability/v1/traces
15:24:27.853 fetched POST /api/observability/v1/traces 204
15:24:32.104 in Ping 
15:24:36.976 rpc 44 server.reportClientActivity
15:24:37.192 in Ping 
15:24:37.999 fetch POST /api/observability/v1/traces
15:24:38.000 fetched POST /api/observability/v1/traces 204
15:24:42.198 in Ping 
15:24:47.201 in Ping 
15:24:51.978 in Interrupt 44
15:24:52.204 in Ping 
15:24:52.984 fetch POST /api/observability/v1/traces
15:24:52.984 fetched POST /api/observability/v1/traces 204
15:24:57.210 in Ping 
15:25:01.957 rpc 45 server.reportClientActivity
15:25:02.214 in Ping 
15:25:02.959 fetch POST /api/observability/v1/traces
15:25:02.959 fetched POST /api/observability/v1/traces 204
15:25:07.217 in Ping 
15:25:12.221 in Ping 
15:25:16.965 in Interrupt 45
15:25:17.225 in Ping 
15:25:17.970 fetch POST /api/observability/v1/traces
15:25:17.970 fetched POST /api/observability/v1/traces 204
15:25:22.228 in Ping 
15:25:26.959 rpc 46 server.reportClientActivity
15:25:27.233 in Ping 
15:25:27.965 fetch POST /api/observability/v1/traces
15:25:27.965 fetched POST /api/observability/v1/traces 204
15:25:32.236 in Ping 
15:25:37.241 in Ping 
15:25:41.971 in Interrupt 46
15:25:42.245 in Ping 
15:25:42.973 fetch POST /api/observability/v1/traces
15:25:42.973 fetched POST /api/observability/v1/traces 204
15:25:47.248 in Ping 
15:25:51.963 rpc 47 server.reportClientActivity
15:25:52.254 in Ping 
15:25:52.966 fetch POST /api/observability/v1/traces
15:25:52.966 fetched POST /api/observability/v1/traces 204
15:25:57.256 in Ping 
15:26:02.260 in Ping 
15:26:06.965 in Interrupt 47
15:26:07.264 in Ping 
15:26:07.969 fetch POST /api/observability/v1/traces
15:26:07.969 fetched POST /api/observability/v1/traces 204
15:26:12.268 in Ping 
15:26:16.971 rpc 48 server.reportClientActivity
15:26:17.272 in Ping 
15:26:17.973 fetch POST /api/observability/v1/traces
15:26:17.973 fetched POST /api/observability/v1/traces 204
15:26:22.276 in Ping 
15:26:27.280 in Ping 
15:26:31.973 in Interrupt 48
15:26:32.285 in Ping 
15:26:32.977 fetch POST /api/observability/v1/traces
15:26:32.977 fetched POST /api/observability/v1/traces 204
15:26:37.288 in Ping 
15:26:41.974 rpc 49 server.reportClientActivity
15:26:42.292 in Ping 
15:26:42.977 fetch POST /api/observability/v1/traces
15:26:42.977 fetched POST /api/observability/v1/traces 204
15:26:47.297 in Ping 
15:26:52.300 in Ping 
15:26:56.977 in Interrupt 49
15:26:57.305 in Ping 
15:26:57.983 fetch POST /api/observability/v1/traces
15:26:57.983 fetched POST /api/observability/v1/traces 204
15:27:02.309 in Ping 
15:27:06.978 rpc 50 server.reportClientActivity
15:27:07.312 in Ping 
15:27:07.981 fetch POST /api/observability/v1/traces
15:27:07.981 fetched POST /api/observability/v1/traces 204
15:27:12.317 in Ping 
15:27:17.321 in Ping 
15:27:21.984 in Interrupt 50
15:27:22.325 in Ping 
15:27:22.991 fetch POST /api/observability/v1/traces
15:27:22.991 fetched POST /api/observability/v1/traces 204
15:27:27.330 in Ping 
15:27:31.986 rpc 51 server.reportClientActivity
15:27:32.335 in Ping 
15:27:32.989 fetch POST /api/observability/v1/traces
15:27:32.989 fetched POST /api/observability/v1/traces 204
15:27:37.340 in Ping 
15:27:42.344 in Ping 
15:27:46.989 in Interrupt 51
15:27:47.348 in Ping 
15:27:47.993 fetch POST /api/observability/v1/traces
15:27:47.993 fetched POST /api/observability/v1/traces 204
15:27:52.352 in Ping 
15:27:56.990 rpc 52 server.reportClientActivity
15:27:57.357 in Ping 
15:27:57.993 fetch POST /api/observability/v1/traces
15:27:57.994 fetched POST /api/observability/v1/traces 204
15:28:02.360 in Ping 
15:28:07.364 in Ping 
15:28:11.995 in Interrupt 52
15:28:12.369 in Ping 
15:28:12.997 fetch POST /api/observability/v1/traces
15:28:12.997 fetched POST /api/observability/v1/traces 204
15:28:17.373 in Ping 
15:28:21.990 rpc 53 server.reportClientActivity
15:28:22.379 in Ping 
15:28:22.993 fetch POST /api/observability/v1/traces
15:28:22.993 fetched POST /api/observability/v1/traces 204
15:28:27.380 in Ping 
15:28:32.384 in Ping 
15:28:36.993 in Interrupt 53
15:28:37.388 in Ping 
15:28:37.998 fetch POST /api/observability/v1/traces
15:28:37.998 fetched POST /api/observability/v1/traces 204
15:28:42.392 in Ping 
