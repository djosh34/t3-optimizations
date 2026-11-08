# case70, webapp, cold start, seed 1

Profile case70 (graded): graded: 70% loss, 10 kbit/s. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| App loaded after the switch | 1.3s |
| Prompt confirmation | never |
| Prompt reached the agent | never |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 0 / 1596 |
| Newest-text lag: max / median / p95 | no text produced / no text produced / no text produced |
| Longest silence | 897.9s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 851247 / 390464 |
| Bytes the app received / sent | 0 / 3558 |

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

- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2874,"inflightPackets":0,"duplicateBytes":0}}
- prompt queued
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2874,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":2874,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3371,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3371,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":3869,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4367,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4367,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4865,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":4865,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5031,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5529,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":5529,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6027,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6027,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6027,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6525,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":6525,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7023,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7023,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7023,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":0,"newConnections":0,"handshakeMs":null,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":0,"sentWireBytes":0,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7521,"inflightPackets":0,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":7521,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8019,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8019,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8019,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8517,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":8517,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9015,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9015,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9015,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9513,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":9513,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10011,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10011,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10011,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10509,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":10509,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11007,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11007,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11007,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11505,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":11505,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12003,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12003,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12003,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12501,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12501,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12999,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12999,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":12999,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13497,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13497,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13995,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":13995,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14493,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14493,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14493,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14991,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":14991,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15489,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15489,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15489,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15987,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":15987,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16485,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16485,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16485,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16983,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":16983,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17481,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17481,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17481,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17979,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":17979,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18477,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18477,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18477,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18975,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":18975,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19473,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19473,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19473,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19971,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":19971,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20469,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20469,"inflightPackets":3,"duplicateBytes":0}}
- webapp {"objects":{"turnItems":"870/870 1093/1093 1299/1299 860/860 1058/1058 1058/1058 1093/1093","messages":"544/544 20/901 456/456","providerTurns":"772/772","checkpointScopes":"731/731","providerSessions":"2693/2693","nodes":"1016/1016 809/809 809/809","checkpoints":"779/779 779/779 613/613","thread":"951/951","providerThreads":"419/809","runs":"1014/1014 971/971","frame":"1029/1029","attempts":"659/659 659/659","assistant":"32/32 21/21","user":"9/9 10/10"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":4,"iceRestarts":0,"newConnections":1,"handshakeMs":233912,"lastDataAt":null,"silences":0,"longestSilenceMs":0,"longestRecoveredGapMs":0},"stats":{"sendRate":2000,"deliveredRate":0,"loss":0,"srttMs":null,"minRttMs":null,"sentPackets":3,"sentWireBytes":3558,"receivedPackets":0,"receivedWireBytes":0,"lastHeardMs":null,"unsyncedBytes":20469,"inflightPackets":3,"duplicateBytes":0}}
- shim trace
18:48:36.406 fetch GET /api/auth/session
18:48:36.410 fetched GET /api/auth/session 200
18:48:36.520 fetch GET /.well-known/t3/environment
18:48:36.521 fetched GET /.well-known/t3/environment 200
18:48:36.541 fetch GET /.well-known/t3/environment
18:48:36.543 fetched GET /.well-known/t3/environment 200
18:48:36.550 rpc 0 subscribeServerConfig
18:48:36.551 chunk 0 38710 bytes
18:48:36.574 in Ack 0
18:48:36.578 rpc 1 subscribeServerLifecycle
18:48:36.578 rpc 2 subscribeServerLifecycle
18:48:36.582 fetch GET /api/auth/session
18:48:36.583 fetched GET /api/auth/session 200
18:48:36.584 rpc 3 subscribeDeviceState
18:48:36.587 rpc 4 preview.reportProfiles
18:48:36.684 rpc 5 subscribeProjectClones
18:48:36.727 rpc 6 orchestration.subscribeThread
18:48:36.728 chunk 6 31285 bytes
18:48:36.731 in Ack 6
18:48:36.738 fetch GET /api/orchestration/shell
18:48:36.794 rpc 7 server.refreshProviders
18:48:36.889 rpc 8 orchestration.dispatchCommand thread.visit
18:48:36.889 exit 8 {"_tag":"Success","value":{"sequence":0}}
18:48:36.974 fetched GET /api/orchestration/shell 200
18:48:36.975 rpc 9 subscribeTerminalMetadata
18:48:36.976 rpc 10 subscribePreviewEvents
18:48:36.976 rpc 11 subscribeVcsStatus
18:48:36.979 rpc 12 orchestration.subscribeShell
18:48:36.979 chunk 12 2852 bytes
18:48:36.982 rpc 13 assets.createUrl
18:48:36.983 rpc 14 preview.list
18:48:36.983 rpc 15 projects.readFile
18:48:36.984 rpc 16 assets.createUrl
18:48:37.051 in Ack 12
18:48:37.264 rpc 17 server.reportClientActivity
18:48:37.531 fetch POST /api/observability/v1/traces
18:48:37.531 fetched POST /api/observability/v1/traces 204
18:48:37.951 rpc 18 orchestration.dispatchCommand message.dispatch
18:48:38.950 fetch POST /api/observability/v1/traces
18:48:38.950 fetched POST /api/observability/v1/traces 204
18:48:41.547 in Ping 
18:48:46.547 in Ping 
18:48:51.548 in Ping 
18:48:52.264 in Interrupt 17
18:48:53.265 fetch POST /api/observability/v1/traces
18:48:53.265 fetched POST /api/observability/v1/traces 204
18:48:56.548 in Ping 
18:49:01.549 in Ping 
18:49:01.773 rpc 19 server.reportClientActivity
18:49:02.773 fetch POST /api/observability/v1/traces
18:49:02.773 fetched POST /api/observability/v1/traces 204
18:49:06.549 in Ping 
18:49:11.549 in Ping 
18:49:16.549 in Ping 
18:49:16.773 in Interrupt 19
18:49:17.774 fetch POST /api/observability/v1/traces
18:49:17.774 fetched POST /api/observability/v1/traces 204
18:49:21.550 in Ping 
18:49:26.550 in Ping 
18:49:26.773 rpc 20 server.reportClientActivity
18:49:27.773 fetch POST /api/observability/v1/traces
18:49:27.773 fetched POST /api/observability/v1/traces 204
18:49:31.552 in Ping 
18:49:36.556 in Ping 
18:49:41.560 in Ping 
18:49:41.774 in Interrupt 20
18:49:42.775 fetch POST /api/observability/v1/traces
18:49:42.775 fetched POST /api/observability/v1/traces 204
18:49:46.564 in Ping 
18:49:51.568 in Ping 
18:49:51.774 rpc 21 server.reportClientActivity
18:49:52.774 fetch POST /api/observability/v1/traces
18:49:52.774 fetched POST /api/observability/v1/traces 204
18:49:56.572 in Ping 
18:50:01.576 in Ping 
18:50:06.580 in Ping 
18:50:06.774 in Interrupt 21
18:50:07.776 fetch POST /api/observability/v1/traces
18:50:07.776 fetched POST /api/observability/v1/traces 204
18:50:11.584 in Ping 
18:50:16.588 in Ping 
18:50:16.774 rpc 22 server.reportClientActivity
18:50:17.774 fetch POST /api/observability/v1/traces
18:50:17.774 fetched POST /api/observability/v1/traces 204
18:50:21.592 in Ping 
18:50:26.596 in Ping 
18:50:31.600 in Ping 
18:50:31.775 in Interrupt 22
18:50:32.777 fetch POST /api/observability/v1/traces
18:50:32.777 fetched POST /api/observability/v1/traces 204
18:50:36.604 in Ping 
18:50:41.608 in Ping 
18:50:41.773 rpc 23 server.reportClientActivity
18:50:42.774 fetch POST /api/observability/v1/traces
18:50:42.775 fetched POST /api/observability/v1/traces 204
18:50:46.612 in Ping 
18:50:51.616 in Ping 
18:50:56.620 in Ping 
18:50:56.774 in Interrupt 23
18:50:57.775 fetch POST /api/observability/v1/traces
18:50:57.775 fetched POST /api/observability/v1/traces 204
18:51:01.625 in Ping 
18:51:06.628 in Ping 
18:51:06.774 rpc 24 server.reportClientActivity
18:51:07.775 fetch POST /api/observability/v1/traces
18:51:07.775 fetched POST /api/observability/v1/traces 204
18:51:11.632 in Ping 
18:51:16.636 in Ping 
18:51:21.640 in Ping 
18:51:21.774 in Interrupt 24
18:51:22.775 fetch POST /api/observability/v1/traces
18:51:22.775 fetched POST /api/observability/v1/traces 204
18:51:26.644 in Ping 
18:51:31.648 in Ping 
18:51:31.775 rpc 25 server.reportClientActivity
18:51:32.775 fetch POST /api/observability/v1/traces
18:51:32.775 fetched POST /api/observability/v1/traces 204
18:51:36.652 in Ping 
18:51:41.656 in Ping 
18:51:46.660 in Ping 
18:51:46.775 in Interrupt 25
18:51:47.776 fetch POST /api/observability/v1/traces
18:51:47.777 fetched POST /api/observability/v1/traces 204
18:51:51.664 in Ping 
18:51:56.668 in Ping 
18:51:56.775 rpc 26 server.reportClientActivity
18:51:57.775 fetch POST /api/observability/v1/traces
18:51:57.776 fetched POST /api/observability/v1/traces 204
18:52:01.672 in Ping 
18:52:06.676 in Ping 
18:52:11.680 in Ping 
18:52:11.776 in Interrupt 26
18:52:12.780 fetch POST /api/observability/v1/traces
18:52:12.780 fetched POST /api/observability/v1/traces 204
18:52:16.684 in Ping 
18:52:21.688 in Ping 
18:52:21.777 rpc 27 server.reportClientActivity
18:52:22.780 fetch POST /api/observability/v1/traces
18:52:22.780 fetched POST /api/observability/v1/traces 204
18:52:26.692 in Ping 
18:52:31.696 in Ping 
18:52:36.700 in Ping 
18:52:36.781 in Interrupt 27
18:52:37.785 fetch POST /api/observability/v1/traces
18:52:37.785 fetched POST /api/observability/v1/traces 204
18:52:41.704 in Ping 
18:52:46.708 in Ping 
18:52:46.777 rpc 28 server.reportClientActivity
18:52:47.780 fetch POST /api/observability/v1/traces
18:52:47.780 fetched POST /api/observability/v1/traces 204
18:52:51.712 in Ping 
18:52:56.717 in Ping 
18:53:01.720 in Ping 
18:53:01.780 in Interrupt 28
18:53:02.784 fetch POST /api/observability/v1/traces
18:53:02.784 fetched POST /api/observability/v1/traces 204
18:53:06.724 in Ping 
18:53:11.728 in Ping 
18:53:11.781 rpc 29 server.reportClientActivity
18:53:12.785 fetch POST /api/observability/v1/traces
18:53:12.785 fetched POST /api/observability/v1/traces 204
18:53:16.732 in Ping 
18:53:21.736 in Ping 
18:53:26.740 in Ping 
18:53:26.784 in Interrupt 29
18:53:27.788 fetch POST /api/observability/v1/traces
18:53:27.788 fetched POST /api/observability/v1/traces 204
18:53:31.746 in Ping 
18:53:36.748 in Ping 
18:53:36.785 rpc 30 server.reportClientActivity
18:53:37.789 fetch POST /api/observability/v1/traces
18:53:37.789 fetched POST /api/observability/v1/traces 204
18:53:41.752 in Ping 
18:53:46.756 in Ping 
18:53:51.760 in Ping 
18:53:51.788 in Interrupt 30
18:53:52.792 fetch POST /api/observability/v1/traces
18:53:52.792 fetched POST /api/observability/v1/traces 204
18:53:56.764 in Ping 
18:54:01.768 in Ping 
18:54:01.789 rpc 31 server.reportClientActivity
18:54:02.792 fetch POST /api/observability/v1/traces
18:54:02.792 fetched POST /api/observability/v1/traces 204
18:54:06.772 in Ping 
18:54:11.776 in Ping 
18:54:16.780 in Ping 
18:54:16.792 in Interrupt 31
18:54:17.797 fetch POST /api/observability/v1/traces
18:54:17.797 fetched POST /api/observability/v1/traces 204
18:54:21.784 in Ping 
18:54:26.788 in Ping 
18:54:26.793 rpc 32 server.reportClientActivity
18:54:27.796 fetch POST /api/observability/v1/traces
18:54:27.796 fetched POST /api/observability/v1/traces 204
18:54:31.792 in Ping 
18:54:36.796 in Ping 
18:54:41.796 in Interrupt 32
18:54:41.800 in Ping 
18:54:42.801 fetch POST /api/observability/v1/traces
18:54:42.801 fetched POST /api/observability/v1/traces 204
18:54:46.804 in Ping 
18:54:51.798 rpc 33 server.reportClientActivity
18:54:51.808 in Ping 
18:54:52.800 fetch POST /api/observability/v1/traces
18:54:52.801 fetched POST /api/observability/v1/traces 204
18:54:56.812 in Ping 
18:55:01.816 in Ping 
18:55:06.800 in Interrupt 33
18:55:06.820 in Ping 
18:55:07.805 fetch POST /api/observability/v1/traces
18:55:07.805 fetched POST /api/observability/v1/traces 204
18:55:11.824 in Ping 
18:55:16.802 rpc 34 server.reportClientActivity
18:55:16.828 in Ping 
18:55:17.804 fetch POST /api/observability/v1/traces
18:55:17.805 fetched POST /api/observability/v1/traces 204
18:55:21.832 in Ping 
18:55:26.836 in Ping 
18:55:31.823 in Interrupt 34
18:55:31.840 in Ping 
18:55:32.824 fetch POST /api/observability/v1/traces
18:55:32.824 fetched POST /api/observability/v1/traces 204
18:55:36.844 in Ping 
18:55:41.805 rpc 35 server.reportClientActivity
18:55:41.848 in Ping 
18:55:42.809 fetch POST /api/observability/v1/traces
18:55:42.809 fetched POST /api/observability/v1/traces 204
18:55:46.852 in Ping 
18:55:51.856 in Ping 
18:55:56.808 in Interrupt 35
18:55:56.864 in Ping 
18:55:57.812 fetch POST /api/observability/v1/traces
18:55:57.812 fetched POST /api/observability/v1/traces 204
18:56:01.868 in Ping 
18:56:06.809 rpc 36 server.reportClientActivity
18:56:06.872 in Ping 
18:56:07.812 fetch POST /api/observability/v1/traces
18:56:07.812 fetched POST /api/observability/v1/traces 204
18:56:11.876 in Ping 
18:56:16.880 in Ping 
18:56:21.812 in Interrupt 36
18:56:21.884 in Ping 
18:56:22.816 fetch POST /api/observability/v1/traces
18:56:22.817 fetched POST /api/observability/v1/traces 204
18:56:26.888 in Ping 
18:56:31.813 rpc 37 server.reportClientActivity
18:56:31.892 in Ping 
18:56:32.816 fetch POST /api/observability/v1/traces
18:56:32.816 fetched POST /api/observability/v1/traces 204
18:56:36.896 in Ping 
18:56:41.900 in Ping 
18:56:46.816 in Interrupt 37
18:56:46.904 in Ping 
18:56:47.821 fetch POST /api/observability/v1/traces
18:56:47.821 fetched POST /api/observability/v1/traces 204
18:56:51.908 in Ping 
18:56:56.817 rpc 38 server.reportClientActivity
18:56:56.912 in Ping 
18:56:57.820 fetch POST /api/observability/v1/traces
18:56:57.820 fetched POST /api/observability/v1/traces 204
18:57:01.916 in Ping 
18:57:06.920 in Ping 
18:57:11.820 in Interrupt 38
18:57:11.924 in Ping 
18:57:12.825 fetch POST /api/observability/v1/traces
18:57:12.825 fetched POST /api/observability/v1/traces 204
18:57:16.928 in Ping 
18:57:21.821 rpc 39 server.reportClientActivity
18:57:21.932 in Ping 
18:57:22.824 fetch POST /api/observability/v1/traces
18:57:22.824 fetched POST /api/observability/v1/traces 204
18:57:26.936 in Ping 
18:57:31.940 in Ping 
18:57:36.824 in Interrupt 39
18:57:36.944 in Ping 
18:57:37.832 fetch POST /api/observability/v1/traces
18:57:37.832 fetched POST /api/observability/v1/traces 204
18:57:41.948 in Ping 
18:57:46.826 rpc 40 server.reportClientActivity
18:57:46.952 in Ping 
18:57:47.830 fetch POST /api/observability/v1/traces
18:57:47.830 fetched POST /api/observability/v1/traces 204
18:57:51.956 in Ping 
18:57:56.960 in Ping 
18:58:01.828 in Interrupt 40
18:58:01.964 in Ping 
18:58:02.855 fetch POST /api/observability/v1/traces
18:58:02.855 fetched POST /api/observability/v1/traces 204
18:58:06.968 in Ping 
18:58:11.829 rpc 41 server.reportClientActivity
18:58:11.972 in Ping 
18:58:12.833 fetch POST /api/observability/v1/traces
18:58:12.833 fetched POST /api/observability/v1/traces 204
18:58:16.976 in Ping 
18:58:21.980 in Ping 
18:58:26.832 in Interrupt 41
18:58:26.984 in Ping 
18:58:27.837 fetch POST /api/observability/v1/traces
18:58:27.837 fetched POST /api/observability/v1/traces 204
18:58:31.988 in Ping 
18:58:36.833 rpc 42 server.reportClientActivity
18:58:36.992 in Ping 
18:58:37.837 fetch POST /api/observability/v1/traces
18:58:37.837 fetched POST /api/observability/v1/traces 204
18:58:41.996 in Ping 
18:58:47.000 in Ping 
18:58:51.836 in Interrupt 42
18:58:52.004 in Ping 
18:58:52.840 fetch POST /api/observability/v1/traces
18:58:52.841 fetched POST /api/observability/v1/traces 204
18:58:57.008 in Ping 
18:59:01.837 rpc 43 server.reportClientActivity
18:59:02.023 in Ping 
18:59:02.840 fetch POST /api/observability/v1/traces
18:59:02.840 fetched POST /api/observability/v1/traces 204
18:59:07.024 in Ping 
18:59:12.028 in Ping 
18:59:16.840 in Interrupt 43
18:59:17.032 in Ping 
18:59:17.845 fetch POST /api/observability/v1/traces
18:59:17.845 fetched POST /api/observability/v1/traces 204
18:59:22.038 in Ping 
18:59:26.841 rpc 44 server.reportClientActivity
18:59:27.040 in Ping 
18:59:27.845 fetch POST /api/observability/v1/traces
18:59:27.845 fetched POST /api/observability/v1/traces 204
18:59:32.044 in Ping 
18:59:37.048 in Ping 
18:59:41.844 in Interrupt 44
18:59:42.057 in Ping 
18:59:42.849 fetch POST /api/observability/v1/traces
18:59:42.849 fetched POST /api/observability/v1/traces 204
18:59:47.060 in Ping 
18:59:51.845 rpc 45 server.reportClientActivity
18:59:52.064 in Ping 
18:59:52.849 fetch POST /api/observability/v1/traces
18:59:52.849 fetched POST /api/observability/v1/traces 204
18:59:57.068 in Ping 
19:00:02.072 in Ping 
19:00:06.848 in Interrupt 45
19:00:07.076 in Ping 
19:00:07.852 fetch POST /api/observability/v1/traces
19:00:07.852 fetched POST /api/observability/v1/traces 204
19:00:12.080 in Ping 
19:00:16.849 rpc 46 server.reportClientActivity
19:00:17.084 in Ping 
19:00:17.853 fetch POST /api/observability/v1/traces
19:00:17.853 fetched POST /api/observability/v1/traces 204
19:00:22.088 in Ping 
19:00:27.092 in Ping 
19:00:31.852 in Interrupt 46
19:00:32.096 in Ping 
19:00:32.856 fetch POST /api/observability/v1/traces
19:00:32.856 fetched POST /api/observability/v1/traces 204
19:00:37.100 in Ping 
19:00:41.853 rpc 47 server.reportClientActivity
19:00:42.104 in Ping 
19:00:42.856 fetch POST /api/observability/v1/traces
19:00:42.856 fetched POST /api/observability/v1/traces 204
19:00:47.108 in Ping 
19:00:52.112 in Ping 
19:00:56.857 in Interrupt 47
19:00:57.116 in Ping 
19:00:57.860 fetch POST /api/observability/v1/traces
19:00:57.860 fetched POST /api/observability/v1/traces 204
19:01:02.120 in Ping 
19:01:06.857 rpc 48 server.reportClientActivity
19:01:07.124 in Ping 
19:01:07.861 fetch POST /api/observability/v1/traces
19:01:07.861 fetched POST /api/observability/v1/traces 204
19:01:12.128 in Ping 
19:01:17.132 in Ping 
19:01:21.860 in Interrupt 48
19:01:22.136 in Ping 
19:01:22.864 fetch POST /api/observability/v1/traces
19:01:22.864 fetched POST /api/observability/v1/traces 204
19:01:27.140 in Ping 
19:01:31.862 rpc 49 server.reportClientActivity
19:01:32.144 in Ping 
19:01:32.865 fetch POST /api/observability/v1/traces
19:01:32.865 fetched POST /api/observability/v1/traces 204
19:01:37.148 in Ping 
19:01:42.152 in Ping 
19:01:46.864 in Interrupt 49
19:01:47.156 in Ping 
19:01:47.868 fetch POST /api/observability/v1/traces
19:01:47.868 fetched POST /api/observability/v1/traces 204
19:01:52.160 in Ping 
19:01:56.865 rpc 50 server.reportClientActivity
19:01:57.164 in Ping 
19:01:57.868 fetch POST /api/observability/v1/traces
19:01:57.868 fetched POST /api/observability/v1/traces 204
19:02:02.168 in Ping 
19:02:07.172 in Ping 
19:02:11.868 in Interrupt 50
19:02:12.176 in Ping 
19:02:12.873 fetch POST /api/observability/v1/traces
19:02:12.873 fetched POST /api/observability/v1/traces 204
19:02:17.180 in Ping 
19:02:21.869 rpc 51 server.reportClientActivity
19:02:22.184 in Ping 
19:02:22.872 fetch POST /api/observability/v1/traces
19:02:22.872 fetched POST /api/observability/v1/traces 204
19:02:27.188 in Ping 
19:02:32.192 in Ping 
19:02:36.872 in Interrupt 51
19:02:37.196 in Ping 
19:02:37.877 fetch POST /api/observability/v1/traces
19:02:37.877 fetched POST /api/observability/v1/traces 204
19:02:42.200 in Ping 
19:02:46.873 rpc 52 server.reportClientActivity
19:02:47.204 in Ping 
19:02:47.876 fetch POST /api/observability/v1/traces
19:02:47.876 fetched POST /api/observability/v1/traces 204
19:02:52.208 in Ping 
19:02:57.212 in Ping 
19:03:01.876 in Interrupt 52
19:03:02.216 in Ping 
19:03:02.881 fetch POST /api/observability/v1/traces
19:03:02.881 fetched POST /api/observability/v1/traces 204
19:03:07.220 in Ping 
19:03:11.878 rpc 53 server.reportClientActivity
19:03:12.224 in Ping 
19:03:12.880 fetch POST /api/observability/v1/traces
19:03:12.880 fetched POST /api/observability/v1/traces 204
19:03:17.228 in Ping 
19:03:22.232 in Ping 
19:03:26.880 in Interrupt 53
19:03:27.236 in Ping 
19:03:27.884 fetch POST /api/observability/v1/traces
19:03:27.884 fetched POST /api/observability/v1/traces 204
19:03:32.240 in Ping 
