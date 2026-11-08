# case70, webapp, cold start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit 0f01ba755fce-dirty.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 2.0s |
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 896.1s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 121852 / 189396 |
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
| 896 | 0.0 | 0 | 0 |

## Driver notes

- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2035,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2035,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2532,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3029,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3029,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3527,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3527,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3527,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4025,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4025,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4523,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4523,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4689,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5187,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5187,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5685,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5685,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6183,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6183,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6183,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6681,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6681,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7179,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7179,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7179,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7677,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7677,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8175,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8175,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8175,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8673,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8673,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9171,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9171,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9171,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9669,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9669,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10167,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10167,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10167,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10665,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10665,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11163,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11163,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11163,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11661,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11661,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12159,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12159,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12159,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12657,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12657,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13155,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13155,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13653,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13653,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13653,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14151,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14151,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14649,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14649,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14649,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15147,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15147,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15645,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15645,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15645,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16143,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16143,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16641,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16641,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16641,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17139,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17139,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17637,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17637,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17637,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18135,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18135,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18633,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18633,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18633,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19131,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19131,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19629,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19629,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19629,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20127,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"user":"9/9 10/10","turnItems":"1299/1299 1108/1108 870/870 860/860 1058/1058 1058/1058 1108/1108","thread":"951/951","assistant":"32/32 21/21","attempts":"662/662 662/662","nodes":"1025/1025 29/809 809/809","runs":"1014/1014 971/971","frame":"1032/1032","providerTurns":"778/778","providerSessions":"2693/2693","checkpointScopes":"731/731","messages":"456/456 550/550","providerThreads":"810/810","checkpoints":"613/613 779/779"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20127,"inflightPackets":0,"duplicateBytes":0}}
- shim trace
15:32:05.387 fetch GET /api/auth/session
15:32:05.391 fetched GET /api/auth/session 200
15:32:05.747 fetch GET /.well-known/t3/environment
15:32:05.748 fetched GET /.well-known/t3/environment 200
15:32:05.774 fetch GET /.well-known/t3/environment
15:32:05.775 fetched GET /.well-known/t3/environment 200
15:32:05.783 rpc 0 subscribeServerConfig
15:32:05.784 chunk 0 38710 bytes
15:32:05.818 in Ack 0
15:32:05.822 rpc 1 subscribeServerLifecycle
15:32:05.823 rpc 2 subscribeServerLifecycle
15:32:05.828 fetch GET /api/auth/session
15:32:05.830 fetched GET /api/auth/session 200
15:32:05.830 rpc 3 subscribeDeviceState
15:32:05.834 rpc 4 preview.reportProfiles
15:32:06.011 rpc 5 subscribeProjectClones
15:32:06.102 rpc 6 server.reportClientActivity
15:32:06.115 rpc 7 orchestration.subscribeThread
15:32:06.116 chunk 7 30598 bytes
15:32:06.120 in Ack 7
15:32:06.132 fetch GET /api/orchestration/shell
15:32:06.197 rpc 8 server.refreshProviders
15:32:06.356 rpc 9 orchestration.dispatchCommand thread.visit
15:32:06.460 fetched GET /api/orchestration/shell 200
15:32:06.461 rpc 10 subscribeTerminalMetadata
15:32:06.462 rpc 11 subscribePreviewEvents
15:32:06.463 rpc 12 subscribeVcsStatus
15:32:06.466 rpc 13 orchestration.subscribeShell
15:32:06.466 chunk 13 2852 bytes
15:32:06.468 rpc 14 assets.createUrl
15:32:06.469 rpc 15 preview.list
15:32:06.470 rpc 16 projects.readFile
15:32:06.470 rpc 17 assets.createUrl
15:32:06.545 in Ack 13
15:32:06.741 fetch POST /api/observability/v1/traces
15:32:06.741 fetched POST /api/observability/v1/traces 204
15:32:07.991 fetch POST /api/observability/v1/traces
15:32:07.991 fetched POST /api/observability/v1/traces 204
15:32:10.781 in Ping 
15:32:15.784 in Ping 
15:32:20.785 in Ping 
15:32:21.507 in Interrupt 6
15:32:21.810 rpc 18 server.reportClientActivity
15:32:22.545 fetch POST /api/observability/v1/traces
15:32:22.545 fetched POST /api/observability/v1/traces 204
15:32:25.785 in Ping 
15:32:30.797 in Ping 
15:32:35.797 in Ping 
15:32:36.812 in Interrupt 18
15:32:37.066 rpc 19 server.reportClientActivity
15:32:37.813 fetch POST /api/observability/v1/traces
15:32:37.814 fetched POST /api/observability/v1/traces 204
15:32:40.797 in Ping 
15:32:45.798 in Ping 
15:32:50.798 in Ping 
15:32:52.069 in Interrupt 19
15:32:53.070 fetch POST /api/observability/v1/traces
15:32:53.070 fetched POST /api/observability/v1/traces 204
15:32:55.807 in Ping 
15:32:56.049 rpc 20 server.reportClientActivity
15:32:57.052 fetch POST /api/observability/v1/traces
15:32:57.052 fetched POST /api/observability/v1/traces 204
15:33:00.809 in Ping 
15:33:05.813 in Ping 
15:33:10.826 in Ping 
15:33:11.071 in Interrupt 20
15:33:12.074 fetch POST /api/observability/v1/traces
15:33:12.074 fetched POST /api/observability/v1/traces 204
15:33:15.837 in Ping 
15:33:20.842 in Ping 
15:33:21.051 rpc 21 server.reportClientActivity
15:33:22.053 fetch POST /api/observability/v1/traces
15:33:22.054 fetched POST /api/observability/v1/traces 204
15:33:25.846 in Ping 
15:33:30.849 in Ping 
15:33:35.853 in Ping 
15:33:36.054 in Interrupt 21
15:33:37.065 fetch POST /api/observability/v1/traces
15:33:37.065 fetched POST /api/observability/v1/traces 204
15:33:40.857 in Ping 
15:33:45.861 in Ping 
15:33:46.060 rpc 22 server.reportClientActivity
15:33:47.060 fetch POST /api/observability/v1/traces
15:33:47.060 fetched POST /api/observability/v1/traces 204
15:33:50.867 in Ping 
15:33:55.870 in Ping 
15:34:00.873 in Ping 
15:34:01.061 in Interrupt 22
15:34:02.070 fetch POST /api/observability/v1/traces
15:34:02.070 fetched POST /api/observability/v1/traces 204
15:34:05.877 in Ping 
15:34:10.881 in Ping 
15:34:11.051 rpc 23 server.reportClientActivity
15:34:12.052 fetch POST /api/observability/v1/traces
15:34:12.052 fetched POST /api/observability/v1/traces 204
15:34:15.885 in Ping 
15:34:20.889 in Ping 
15:34:25.894 in Ping 
15:34:26.058 in Interrupt 23
15:34:27.059 fetch POST /api/observability/v1/traces
15:34:27.059 fetched POST /api/observability/v1/traces 204
15:34:30.897 in Ping 
15:34:35.901 in Ping 
15:34:36.142 rpc 24 server.reportClientActivity
15:34:37.128 fetch POST /api/observability/v1/traces
15:34:37.129 fetched POST /api/observability/v1/traces 204
15:34:40.906 in Ping 
15:34:45.909 in Ping 
15:34:50.913 in Ping 
15:34:51.147 in Interrupt 24
15:34:52.150 fetch POST /api/observability/v1/traces
15:34:52.150 fetched POST /api/observability/v1/traces 204
15:34:55.918 in Ping 
15:35:02.427 in Ping 
15:35:03.698 rpc 25 server.reportClientActivity
15:35:04.047 fetch POST /api/observability/v1/traces
15:35:04.053 fetched POST /api/observability/v1/traces 204
15:35:07.509 in Ping 
15:35:12.513 in Ping 
15:35:17.519 in Ping 
15:35:18.746 in Interrupt 25
15:35:19.751 fetch POST /api/observability/v1/traces
15:35:19.752 fetched POST /api/observability/v1/traces 204
15:35:22.522 in Ping 
15:35:27.504 rpc 26 server.reportClientActivity
15:35:27.525 in Ping 
15:35:28.504 fetch POST /api/observability/v1/traces
15:35:28.504 fetched POST /api/observability/v1/traces 204
15:35:32.529 in Ping 
15:35:37.538 in Ping 
15:35:42.507 in Interrupt 26
15:35:42.541 in Ping 
15:35:43.518 fetch POST /api/observability/v1/traces
15:35:43.518 fetched POST /api/observability/v1/traces 204
15:35:47.545 in Ping 
15:35:52.505 rpc 27 server.reportClientActivity
15:35:52.564 in Ping 
15:35:53.506 fetch POST /api/observability/v1/traces
15:35:53.507 fetched POST /api/observability/v1/traces 204
15:35:57.567 in Ping 
15:36:02.571 in Ping 
15:36:07.511 in Interrupt 27
15:36:07.592 in Ping 
15:36:08.528 fetch POST /api/observability/v1/traces
15:36:08.528 fetched POST /api/observability/v1/traces 204
15:36:12.593 in Ping 
15:36:17.521 rpc 28 server.reportClientActivity
15:36:17.597 in Ping 
15:36:18.528 fetch POST /api/observability/v1/traces
15:36:18.529 fetched POST /api/observability/v1/traces 204
15:36:22.601 in Ping 
15:36:27.605 in Ping 
15:36:32.527 in Interrupt 28
15:36:32.609 in Ping 
15:36:33.532 fetch POST /api/observability/v1/traces
15:36:33.532 fetched POST /api/observability/v1/traces 204
15:36:37.613 in Ping 
15:36:42.508 rpc 29 server.reportClientActivity
15:36:42.617 in Ping 
15:36:43.510 fetch POST /api/observability/v1/traces
15:36:43.510 fetched POST /api/observability/v1/traces 204
15:36:47.629 in Ping 
15:36:52.636 in Ping 
15:36:57.513 in Interrupt 29
15:36:57.641 in Ping 
15:36:58.523 fetch POST /api/observability/v1/traces
15:36:58.536 fetched POST /api/observability/v1/traces 204
15:37:02.646 in Ping 
15:37:07.520 rpc 30 server.reportClientActivity
15:37:07.649 in Ping 
15:37:08.520 fetch POST /api/observability/v1/traces
15:37:08.520 fetched POST /api/observability/v1/traces 204
15:37:12.653 in Ping 
15:37:17.657 in Ping 
15:37:22.522 in Interrupt 30
15:37:22.661 in Ping 
15:37:23.527 fetch POST /api/observability/v1/traces
15:37:23.527 fetched POST /api/observability/v1/traces 204
15:37:27.665 in Ping 
15:37:32.515 rpc 31 server.reportClientActivity
15:37:32.669 in Ping 
15:37:33.518 fetch POST /api/observability/v1/traces
15:37:33.518 fetched POST /api/observability/v1/traces 204
15:37:37.674 in Ping 
15:37:42.677 in Ping 
15:37:47.518 in Interrupt 31
15:37:47.681 in Ping 
15:37:48.523 fetch POST /api/observability/v1/traces
15:37:48.523 fetched POST /api/observability/v1/traces 204
15:37:52.685 in Ping 
15:37:57.521 rpc 32 server.reportClientActivity
15:37:57.689 in Ping 
15:37:58.523 fetch POST /api/observability/v1/traces
15:37:58.523 fetched POST /api/observability/v1/traces 204
15:38:02.693 in Ping 
15:38:07.697 in Ping 
15:38:12.525 in Interrupt 32
15:38:12.701 in Ping 
15:38:13.532 fetch POST /api/observability/v1/traces
15:38:13.533 fetched POST /api/observability/v1/traces 204
15:38:17.706 in Ping 
15:38:22.522 rpc 33 server.reportClientActivity
15:38:22.709 in Ping 
15:38:23.526 fetch POST /api/observability/v1/traces
15:38:23.526 fetched POST /api/observability/v1/traces 204
15:38:27.715 in Ping 
15:38:32.718 in Ping 
15:38:37.526 in Interrupt 33
15:38:37.721 in Ping 
15:38:38.531 fetch POST /api/observability/v1/traces
15:38:38.531 fetched POST /api/observability/v1/traces 204
15:38:42.725 in Ping 
15:38:47.531 rpc 34 server.reportClientActivity
15:38:47.730 in Ping 
15:38:48.531 fetch POST /api/observability/v1/traces
15:38:48.531 fetched POST /api/observability/v1/traces 204
15:38:52.734 in Ping 
15:38:57.737 in Ping 
15:39:02.533 in Interrupt 34
15:39:02.741 in Ping 
15:39:03.538 fetch POST /api/observability/v1/traces
15:39:03.538 fetched POST /api/observability/v1/traces 204
15:39:07.745 in Ping 
15:39:12.530 rpc 35 server.reportClientActivity
15:39:12.749 in Ping 
15:39:13.534 fetch POST /api/observability/v1/traces
15:39:13.534 fetched POST /api/observability/v1/traces 204
15:39:17.755 in Ping 
15:39:22.757 in Ping 
15:39:27.533 in Interrupt 35
15:39:27.761 in Ping 
15:39:28.538 fetch POST /api/observability/v1/traces
15:39:28.538 fetched POST /api/observability/v1/traces 204
15:39:32.765 in Ping 
15:39:37.536 rpc 36 server.reportClientActivity
15:39:37.769 in Ping 
15:39:38.538 fetch POST /api/observability/v1/traces
15:39:38.538 fetched POST /api/observability/v1/traces 204
15:39:42.773 in Ping 
15:39:47.777 in Ping 
15:39:52.538 in Interrupt 36
15:39:52.781 in Ping 
15:39:53.542 fetch POST /api/observability/v1/traces
15:39:53.542 fetched POST /api/observability/v1/traces 204
15:39:57.785 in Ping 
15:40:02.539 rpc 37 server.reportClientActivity
15:40:02.790 in Ping 
15:40:03.544 fetch POST /api/observability/v1/traces
15:40:03.544 fetched POST /api/observability/v1/traces 204
15:40:07.793 in Ping 
15:40:12.797 in Ping 
15:40:17.542 in Interrupt 37
15:40:17.802 in Ping 
15:40:18.549 fetch POST /api/observability/v1/traces
15:40:18.549 fetched POST /api/observability/v1/traces 204
15:40:22.806 in Ping 
15:40:27.546 rpc 38 server.reportClientActivity
15:40:27.809 in Ping 
15:40:28.551 fetch POST /api/observability/v1/traces
15:40:28.551 fetched POST /api/observability/v1/traces 204
15:40:32.813 in Ping 
15:40:37.817 in Ping 
15:40:42.549 in Interrupt 38
15:40:42.821 in Ping 
15:40:43.554 fetch POST /api/observability/v1/traces
15:40:43.554 fetched POST /api/observability/v1/traces 204
15:40:47.825 in Ping 
15:40:52.546 rpc 39 server.reportClientActivity
15:40:52.829 in Ping 
15:40:53.550 fetch POST /api/observability/v1/traces
15:40:53.550 fetched POST /api/observability/v1/traces 204
15:40:57.834 in Ping 
15:41:02.838 in Ping 
15:41:07.549 in Interrupt 39
15:41:07.841 in Ping 
15:41:08.560 fetch POST /api/observability/v1/traces
15:41:08.561 fetched POST /api/observability/v1/traces 204
15:41:12.845 in Ping 
15:41:17.552 rpc 40 server.reportClientActivity
15:41:17.849 in Ping 
15:41:18.554 fetch POST /api/observability/v1/traces
15:41:18.554 fetched POST /api/observability/v1/traces 204
15:41:22.853 in Ping 
15:41:27.857 in Ping 
15:41:32.559 in Interrupt 40
15:41:32.862 in Ping 
15:41:33.562 fetch POST /api/observability/v1/traces
15:41:33.562 fetched POST /api/observability/v1/traces 204
15:41:37.865 in Ping 
15:41:42.554 rpc 41 server.reportClientActivity
15:41:42.869 in Ping 
15:41:43.557 fetch POST /api/observability/v1/traces
15:41:43.557 fetched POST /api/observability/v1/traces 204
15:41:47.873 in Ping 
15:41:52.877 in Ping 
15:41:57.558 in Interrupt 41
15:41:57.881 in Ping 
15:41:58.562 fetch POST /api/observability/v1/traces
15:41:58.562 fetched POST /api/observability/v1/traces 204
15:42:02.885 in Ping 
15:42:07.569 rpc 42 server.reportClientActivity
15:42:07.889 in Ping 
15:42:08.571 fetch POST /api/observability/v1/traces
15:42:08.571 fetched POST /api/observability/v1/traces 204
15:42:12.893 in Ping 
15:42:17.897 in Ping 
15:42:22.573 in Interrupt 42
15:42:22.901 in Ping 
15:42:23.578 fetch POST /api/observability/v1/traces
15:42:23.578 fetched POST /api/observability/v1/traces 204
15:42:27.905 in Ping 
15:42:32.563 rpc 43 server.reportClientActivity
15:42:32.909 in Ping 
15:42:33.566 fetch POST /api/observability/v1/traces
15:42:33.566 fetched POST /api/observability/v1/traces 204
15:42:37.913 in Ping 
15:42:42.917 in Ping 
15:42:47.565 in Interrupt 43
15:42:47.921 in Ping 
15:42:48.570 fetch POST /api/observability/v1/traces
15:42:48.570 fetched POST /api/observability/v1/traces 204
15:42:52.925 in Ping 
15:42:57.567 rpc 44 server.reportClientActivity
15:42:57.929 in Ping 
15:42:58.570 fetch POST /api/observability/v1/traces
15:42:58.570 fetched POST /api/observability/v1/traces 204
15:43:02.933 in Ping 
15:43:07.938 in Ping 
15:43:12.569 in Interrupt 44
15:43:12.941 in Ping 
15:43:13.574 fetch POST /api/observability/v1/traces
15:43:13.574 fetched POST /api/observability/v1/traces 204
15:43:17.945 in Ping 
15:43:22.571 rpc 45 server.reportClientActivity
15:43:22.949 in Ping 
15:43:23.574 fetch POST /api/observability/v1/traces
15:43:23.574 fetched POST /api/observability/v1/traces 204
15:43:27.953 in Ping 
15:43:32.960 in Ping 
15:43:37.573 in Interrupt 45
15:43:37.961 in Ping 
15:43:38.578 fetch POST /api/observability/v1/traces
15:43:38.578 fetched POST /api/observability/v1/traces 204
15:43:42.965 in Ping 
15:43:47.574 rpc 46 server.reportClientActivity
15:43:47.969 in Ping 
15:43:48.578 fetch POST /api/observability/v1/traces
15:43:48.578 fetched POST /api/observability/v1/traces 204
15:43:52.973 in Ping 
15:43:57.977 in Ping 
15:44:02.578 in Interrupt 46
15:44:02.981 in Ping 
15:44:03.582 fetch POST /api/observability/v1/traces
15:44:03.582 fetched POST /api/observability/v1/traces 204
15:44:07.985 in Ping 
15:44:12.579 rpc 47 server.reportClientActivity
15:44:12.989 in Ping 
15:44:13.582 fetch POST /api/observability/v1/traces
15:44:13.582 fetched POST /api/observability/v1/traces 204
15:44:17.993 in Ping 
15:44:22.997 in Ping 
15:44:27.582 in Interrupt 47
15:44:28.001 in Ping 
15:44:28.586 fetch POST /api/observability/v1/traces
15:44:28.586 fetched POST /api/observability/v1/traces 204
15:44:33.005 in Ping 
15:44:37.583 rpc 48 server.reportClientActivity
15:44:38.009 in Ping 
15:44:38.586 fetch POST /api/observability/v1/traces
15:44:38.586 fetched POST /api/observability/v1/traces 204
15:44:43.061 in Ping 
15:44:48.065 in Ping 
15:44:52.585 in Interrupt 48
15:44:53.069 in Ping 
15:44:53.589 fetch POST /api/observability/v1/traces
15:44:53.590 fetched POST /api/observability/v1/traces 204
15:44:58.073 in Ping 
15:45:02.587 rpc 49 server.reportClientActivity
15:45:03.085 in Ping 
15:45:03.590 fetch POST /api/observability/v1/traces
15:45:03.590 fetched POST /api/observability/v1/traces 204
15:45:08.085 in Ping 
15:45:13.089 in Ping 
15:45:17.591 in Interrupt 49
15:45:18.093 in Ping 
15:45:18.597 fetch POST /api/observability/v1/traces
15:45:18.597 fetched POST /api/observability/v1/traces 204
15:45:23.097 in Ping 
15:45:27.591 rpc 50 server.reportClientActivity
15:45:28.101 in Ping 
15:45:28.593 fetch POST /api/observability/v1/traces
15:45:28.594 fetched POST /api/observability/v1/traces 204
15:45:33.105 in Ping 
15:45:38.109 in Ping 
15:45:42.594 in Interrupt 50
15:45:43.114 in Ping 
15:45:43.598 fetch POST /api/observability/v1/traces
15:45:43.598 fetched POST /api/observability/v1/traces 204
15:45:48.118 in Ping 
15:45:52.595 rpc 51 server.reportClientActivity
15:45:53.121 in Ping 
15:45:53.598 fetch POST /api/observability/v1/traces
15:45:53.598 fetched POST /api/observability/v1/traces 204
15:45:58.125 in Ping 
15:46:03.129 in Ping 
15:46:07.598 in Interrupt 51
15:46:08.133 in Ping 
15:46:08.602 fetch POST /api/observability/v1/traces
15:46:08.602 fetched POST /api/observability/v1/traces 204
15:46:13.137 in Ping 
15:46:17.601 rpc 52 server.reportClientActivity
15:46:18.141 in Ping 
15:46:18.602 fetch POST /api/observability/v1/traces
15:46:18.602 fetched POST /api/observability/v1/traces 204
15:46:23.145 in Ping 
15:46:28.149 in Ping 
15:46:32.605 in Interrupt 52
15:46:33.153 in Ping 
15:46:33.610 fetch POST /api/observability/v1/traces
15:46:33.610 fetched POST /api/observability/v1/traces 204
15:46:38.157 in Ping 
15:46:42.603 rpc 53 server.reportClientActivity
15:46:43.161 in Ping 
15:46:43.606 fetch POST /api/observability/v1/traces
15:46:43.606 fetched POST /api/observability/v1/traces 204
15:46:48.165 in Ping 
15:46:53.169 in Ping 
15:46:57.605 in Interrupt 53
15:46:58.173 in Ping 
15:46:58.610 fetch POST /api/observability/v1/traces
15:46:58.610 fetched POST /api/observability/v1/traces 204
15:47:03.177 in Ping 
