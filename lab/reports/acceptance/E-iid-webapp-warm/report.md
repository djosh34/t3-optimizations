# E-iid, webapp, warm start, seed 1

Profile E-iid (fault): E with independent 75% loss instead of bursts. Commit fe6e6aee76ac.

WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale.

**Status: not done by deadline** after 900 s (deadline 15m0s).

| Measure | Value |
|---|---|
| Prompt confirmation | never |
| Prompt reached the agent | 12.6s |
| Time to first text | never |
| Whole reply shown | never |
| Reply shown / produced / total (chars) | 0 / 1077 / 1596 |
| Newest-text lag: max / median / p95 | 886.3s / 436.3s / 841.3s |
| Longest silence | 899.4s |
| Approval shown after it was asked | never |
| Bytes on the wire down / up | 103994 / 28158 |
| Bytes the app received / sent | 820 / 26491 |

## Newest-text lag over time

Every 10 s after the prompt was sent.

| After (s) | Lag (s) | Shown | Produced |
|---|---|---|---|
| 0 | 0.0 | 0 | 0 |
| 10 | 0.0 | 0 | 0 |
| 20 | 7.3 | 0 | 299 |
| 30 | 17.3 | 0 | 697 |
| 40 | 27.3 | 0 | 1077 |
| 50 | 37.3 | 0 | 1077 |
| 60 | 47.3 | 0 | 1077 |
| 70 | 57.3 | 0 | 1077 |
| 80 | 67.3 | 0 | 1077 |
| 90 | 77.3 | 0 | 1077 |
| 100 | 87.3 | 0 | 1077 |
| 110 | 97.3 | 0 | 1077 |
| 120 | 107.3 | 0 | 1077 |
| 130 | 117.3 | 0 | 1077 |
| 140 | 127.3 | 0 | 1077 |
| 150 | 137.3 | 0 | 1077 |
| 160 | 147.3 | 0 | 1077 |
| 170 | 157.3 | 0 | 1077 |
| 180 | 167.3 | 0 | 1077 |
| 190 | 177.3 | 0 | 1077 |
| 200 | 187.3 | 0 | 1077 |
| 210 | 197.3 | 0 | 1077 |
| 220 | 207.3 | 0 | 1077 |
| 230 | 217.3 | 0 | 1077 |
| 240 | 227.3 | 0 | 1077 |
| 250 | 237.3 | 0 | 1077 |
| 260 | 247.3 | 0 | 1077 |
| 270 | 257.3 | 0 | 1077 |
| 280 | 267.3 | 0 | 1077 |
| 290 | 277.3 | 0 | 1077 |
| 300 | 287.3 | 0 | 1077 |
| 310 | 297.3 | 0 | 1077 |
| 320 | 307.3 | 0 | 1077 |
| 330 | 317.3 | 0 | 1077 |
| 340 | 327.3 | 0 | 1077 |
| 350 | 337.3 | 0 | 1077 |
| 360 | 347.3 | 0 | 1077 |
| 370 | 357.3 | 0 | 1077 |
| 380 | 367.3 | 0 | 1077 |
| 390 | 377.3 | 0 | 1077 |
| 400 | 387.3 | 0 | 1077 |
| 410 | 397.3 | 0 | 1077 |
| 420 | 407.3 | 0 | 1077 |
| 430 | 417.3 | 0 | 1077 |
| 440 | 427.3 | 0 | 1077 |
| 450 | 437.3 | 0 | 1077 |
| 460 | 447.3 | 0 | 1077 |
| 470 | 457.3 | 0 | 1077 |
| 480 | 467.3 | 0 | 1077 |
| 490 | 477.3 | 0 | 1077 |
| 500 | 487.3 | 0 | 1077 |
| 510 | 497.3 | 0 | 1077 |
| 520 | 507.3 | 0 | 1077 |
| 530 | 517.3 | 0 | 1077 |
| 540 | 527.3 | 0 | 1077 |
| 550 | 537.3 | 0 | 1077 |
| 560 | 547.3 | 0 | 1077 |
| 570 | 557.3 | 0 | 1077 |
| 580 | 567.3 | 0 | 1077 |
| 590 | 577.3 | 0 | 1077 |
| 600 | 587.3 | 0 | 1077 |
| 610 | 597.3 | 0 | 1077 |
| 620 | 607.3 | 0 | 1077 |
| 630 | 617.3 | 0 | 1077 |
| 640 | 627.3 | 0 | 1077 |
| 650 | 637.3 | 0 | 1077 |
| 660 | 647.3 | 0 | 1077 |
| 670 | 657.3 | 0 | 1077 |
| 680 | 667.3 | 0 | 1077 |
| 690 | 677.3 | 0 | 1077 |
| 700 | 687.3 | 0 | 1077 |
| 710 | 697.3 | 0 | 1077 |
| 720 | 707.3 | 0 | 1077 |
| 730 | 717.3 | 0 | 1077 |
| 740 | 727.3 | 0 | 1077 |
| 750 | 737.3 | 0 | 1077 |
| 760 | 747.3 | 0 | 1077 |
| 770 | 757.3 | 0 | 1077 |
| 780 | 767.3 | 0 | 1077 |
| 790 | 777.3 | 0 | 1077 |
| 800 | 787.3 | 0 | 1077 |
| 810 | 797.3 | 0 | 1077 |
| 820 | 807.3 | 0 | 1077 |
| 830 | 817.3 | 0 | 1077 |
| 840 | 827.3 | 0 | 1077 |
| 850 | 837.3 | 0 | 1077 |
| 860 | 847.3 | 0 | 1077 |
| 870 | 857.3 | 0 | 1077 |
| 880 | 867.3 | 0 | 1077 |
| 890 | 877.3 | 0 | 1077 |
| 899 | 886.3 | 0 | 1077 |

## Driver notes

- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161764655,"silences":0,"longestSilenceMs":1605,"longestRecoveredGapMs":1605},"stats":{"sendRate":6932.013900294654,"deliveredRate":3359.558750499545,"loss":0,"srttMs":4.511285931929409,"minRttMs":1,"sentPackets":103,"sentWireBytes":31346,"receivedPackets":134,"receivedWireBytes":66036,"lastHeardMs":8853,"unsyncedBytes":509,"inflightPackets":3,"duplicateBytes":0}}
- prompt queued
- prompt sent
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc live","open":true,"generation":2,"iceRestarts":0,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":2000,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":123,"sentWireBytes":39918,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":509,"inflightPackets":23,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 1","open":true,"generation":2,"iceRestarts":1,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":2000,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":134,"sentWireBytes":44954,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1005,"inflightPackets":34,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":2000,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":146,"sentWireBytes":51300,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1005,"inflightPackets":46,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":153,"sentWireBytes":55130,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1005,"inflightPackets":53,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 2","open":true,"generation":2,"iceRestarts":2,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":154,"sentWireBytes":55784,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1501,"inflightPackets":54,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"ICE restart 3","open":true,"generation":2,"iceRestarts":3,"newConnections":0,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1501,"inflightPackets":48,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1998,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1998,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":1998,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":2495,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":2495,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":3158,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":3158,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":3158,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":3655,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":3655,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":4152,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":4152,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":4152,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":4649,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":4649,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":5146,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":5146,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":5643,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":5643,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":5643,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":6140,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":6140,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":6637,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":6637,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":6637,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":7134,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":7134,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":7631,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":7631,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":7631,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":8128,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":8128,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":8625,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":8625,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":8625,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":9122,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":9122,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":9619,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":9619,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":9619,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":10116,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":10116,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":10613,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":10613,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":10613,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":11110,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":11110,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":11607,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":11607,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":11607,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":12104,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":12104,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":12601,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":12601,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":12601,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":13098,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":13098,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":13595,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":13595,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":13595,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":14092,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":14092,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":14589,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":14589,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":14589,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":15086,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":15086,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":15583,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":15583,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":16080,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":16080,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":16080,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":16577,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":16577,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":17074,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":17074,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":17074,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":17571,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":17571,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":18068,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":18068,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":18068,"inflightPackets":36,"duplicateBytes":0}}
- webapp {"objects":{"thread":"951/951","turnItems":"1058/1058 1108/1108 860/860 870/870 1299/1299 1108/1108 1058/1058","frame":"1032/1032","runs":"1014/1014 971/971","assistant":"32/32 21/21","user":"10/10 9/9","attempts":"662/662 662/662","providerTurns":"778/778 778/778","providerThreads":"810/810","providerSessions":"2692/2692","nodes":"1025/1025 809/809 809/809 1025/1025","messages":"550/550 456/456 901/901 550/550","checkpoints":"613/613 613/613 779/779","checkpointScopes":"730/730"},"link":{"transport":"webrtc","trying":"webrtc connecting","open":false,"generation":3,"iceRestarts":3,"newConnections":1,"handshakeMs":258,"lastDataAt":1794161772866,"silences":1,"longestSilenceMs":8211,"longestRecoveredGapMs":8211},"stats":{"sendRate":50,"deliveredRate":3363.1196967782525,"loss":0,"srttMs":4.511285931929409,"minRttMs":null,"sentPackets":155,"sentWireBytes":56438,"receivedPackets":135,"receivedWireBytes":66856,"lastHeardMs":17064,"unsyncedBytes":18565,"inflightPackets":36,"duplicateBytes":0}}
- shim trace
18:15:56.108 fetch GET /api/auth/session
18:15:56.110 fetched GET /api/auth/session 200
18:15:56.230 fetch GET /.well-known/t3/environment
18:15:56.230 fetched GET /.well-known/t3/environment 200
18:15:56.246 fetch GET /.well-known/t3/environment
18:15:56.247 fetched GET /.well-known/t3/environment 200
18:15:56.250 rpc 0 subscribeServerConfig
18:15:56.251 chunk 0 38706 bytes
18:15:56.273 in Ack 0
18:15:56.275 rpc 1 subscribeServerLifecycle
18:15:56.276 rpc 2 subscribeServerLifecycle
18:15:56.279 fetch GET /api/auth/session
18:15:56.281 fetched GET /api/auth/session 200
18:15:56.281 rpc 3 subscribeDeviceState
18:15:56.283 rpc 4 preview.reportProfiles
18:15:56.314 rpc 5 subscribeProjectClones
18:15:56.332 fetch GET /api/orchestration/threads/mcp%3Af048a1e3-fafd-4633-bf59-2ce8e7130d82/bounded
18:15:56.337 fetch GET /api/orchestration/shell
18:15:56.355 rpc 6 server.refreshProviders
18:15:56.391 rpc 7 orchestration.dispatchCommand thread.visit
18:15:56.391 exit 7 {"_tag":"Success","value":{"sequence":0}}
18:15:56.401 fetched GET /api/orchestration/shell 200
18:15:56.402 rpc 8 subscribeTerminalMetadata
18:15:56.402 rpc 9 subscribeVcsStatus
18:15:56.403 rpc 10 subscribePreviewEvents
18:15:56.405 rpc 11 orchestration.subscribeShell
18:15:56.405 chunk 11 2786 bytes
18:15:56.406 rpc 12 assets.createUrl
18:15:56.407 rpc 13 preview.list
18:15:56.407 rpc 14 projects.readFile
18:15:56.411 in Ack 11
18:15:56.521 exit 13 {"_tag":"Success","value":{"revision":0,"serverEpoch":"23c40ce7-3416-4ed2-8ae4-102babb73e0f","sessions":[]}}
18:15:56.521 exit 14 {"_tag":"Success","value":{"byteLength":0,"contents":"","exists":false,"relativePath":"t3.json","truncated":false}}
18:15:56.654 rpc 15 server.reportClientActivity
18:15:56.655 chunk 11 2762 bytes
18:15:56.655 in Ack 11
18:15:56.901 chunk 1 3868 bytes
18:15:56.902 in Ack 1
18:15:57.049 chunk 2 3868 bytes
18:15:57.050 in Ack 2
18:15:57.149 exit 6 {"_tag":"Success","value":{"providers":[{"auth":{"canLogout":false,"status":"unknown"},"checkedAt":"2026-11-08T18:15:44.190Z","configurableProviders":false,"con
18:15:57.149 chunk 9 534 bytes
18:15:57.149 exit 12 {"_tag":"Success","value":{"expiresAt":1794164400000,"relativeUrl":"/api/assets/eyJ2ZXJzaW9uIjoxLCJraW5kIjoicHJvamVjdC1mYXZpY29uIiwid29ya3NwYWNlUm9vdCI6Ii9ob21l
18:15:57.152 in Ack 9
18:15:57.249 fetch POST /api/observability/v1/traces
18:15:57.249 fetched POST /api/observability/v1/traces 204
18:15:57.401 chunk 11 2805 bytes
18:15:57.401 in Ack 11
18:15:57.429 chunk 3 876 bytes
18:15:57.429 exit 4 {"_tag":"Success","value":null}
18:15:57.429 chunk 5 4 bytes
18:15:57.429 chunk 8 36 bytes
18:15:57.429 exit 15 {"_tag":"Success","value":null}
18:15:57.429 in Ack 3
18:15:57.433 in Ack 5
18:15:57.433 in Ack 8
18:15:57.558 fetched GET /api/orchestration/threads/mcp%3Af048a1e3-fafd-4633-bf59-2ce8e7130d82/bounded 200
18:15:57.561 rpc 16 orchestration.subscribeThread
18:15:57.562 chunk 16 8305 bytes
18:15:57.571 in Interrupt 9
18:15:57.573 rpc 17 subscribeVcsStatus
18:15:57.574 in Ack 16
18:15:57.689 chunk 17 534 bytes
18:15:57.690 in Ack 17
18:15:57.812 chunk 16 11944 bytes
18:15:57.813 in Ack 16
18:15:57.865 rpc 18 server.reportClientActivity
18:15:58.034 exit 18 {"_tag":"Success","value":null}
18:15:58.064 chunk 16 13532 bytes
18:15:58.065 in Ack 16
18:15:58.313 chunk 16 18059 bytes
18:15:58.314 in Ack 16
18:15:58.402 fetch POST /api/observability/v1/traces
18:15:58.402 fetched POST /api/observability/v1/traces 204
18:15:58.564 chunk 16 19887 bytes
18:15:58.570 in Ack 16
18:15:58.581 in Interrupt 17
18:15:58.583 rpc 19 subscribeVcsStatus
18:15:58.612 rpc 20 attachments.createUploadUrl
18:15:58.797 chunk 19 534 bytes
18:15:58.801 in Ack 19
18:15:58.814 chunk 16 21231 bytes
18:15:58.815 in Ack 16
18:15:58.834 rpc 21 server.reportClientActivity
18:15:58.867 exit 20 {"_tag":"Success","value":{"attachmentId":"pending-e06db397-64ce-48d2-b3e9-0fff5f3d8b5b","expiresAt":1794162358838,"relativeUrl":"/api/attachments/upload/eyJ2ZX
18:15:58.868 fetch POST /api/attachments/upload/eyJ2ZXJzaW9uIjoxLCJraW5kIjoiYXR0YWNobWVudC11cGxvYWQiLCJ0eXBlIjoiaW1hZ2UiLCJhdHRhY2htZW50SWQiOiJwZW5kaW5nLWUwNmRiMzk3LTY0Y2UtNDhkMi1iM2U5LTBmZmY1ZjNkOGI1YiIsIm5hbWUiOiJsYWIucG5nIiwibWltZVR5cGUiOiJpbWFnZS9wbmciLCJzaXplQnl0ZXMiOjcwMzAsImV4cGlyZXNBdCI6MTc5NDE2MjM1ODgzOH0.25SLmuN-ZkQaBzdoDsoAKqSesmE1Vx1M-jxE_mIbSqk
18:15:58.911 exit 21 {"_tag":"Success","value":null}
18:15:59.571 fetch POST /api/observability/v1/traces
18:15:59.571 fetched POST /api/observability/v1/traces 204
18:16:01.249 in Ping 
18:16:01.808 rpc 22 orchestration.dispatchCommand message.dispatch
18:16:02.387 exit 22 {"_tag":"Success","value":{"sequence":45}}
18:16:02.389 rpc 23 attachments.delete
18:16:02.677 chunk 16 23687 bytes
18:16:02.678 in Ack 16
18:16:02.856 fetch POST /api/observability/v1/traces
18:16:02.856 fetched POST /api/observability/v1/traces 204
18:16:02.899 exit 23 {"_tag":"Success","value":null}
18:16:02.927 chunk 11 2891 bytes
18:16:02.927 chunk 16 24298 bytes
18:16:02.927 in Ack 11
18:16:02.949 in Ack 16
18:16:03.178 chunk 16 27993 bytes
18:16:03.180 in Ack 16
18:16:03.183 rpc 24 assets.createUrl
18:16:03.429 chunk 11 2827 bytes
18:16:03.429 chunk 16 28125 bytes
18:16:03.430 in Ack 11
18:16:03.431 rpc 25 orchestration.dispatchCommand thread.visit
18:16:03.431 exit 25 {"_tag":"Success","value":{"sequence":0}}
18:16:03.436 in Ack 16
18:16:03.680 chunk 11 2827 bytes
18:16:03.680 chunk 16 28149 bytes
18:16:03.681 in Ack 11
18:16:03.686 in Ack 16
18:16:03.900 fetch POST /api/observability/v1/traces
18:16:03.900 fetched POST /api/observability/v1/traces 204
18:16:03.930 chunk 16 31996 bytes
18:16:03.931 in Ack 16
18:16:03.937 in Interrupt 19
18:16:03.940 rpc 26 subscribeVcsStatus
18:16:03.998 chunk 26 534 bytes
18:16:03.999 in Ack 26
18:16:04.180 chunk 16 34258 bytes
18:16:04.181 in Ack 16
18:16:04.191 rpc 27 server.reportClientActivity
18:16:04.238 exit 27 {"_tag":"Success","value":null}
18:16:04.656 chunk 16 35284 bytes
18:16:04.657 in Ack 16
18:16:04.936 fetch POST /api/observability/v1/traces
18:16:04.936 fetched POST /api/observability/v1/traces 204
18:16:05.347 rpc 28 orchestration.dispatchCommand message.dispatch
18:16:06.249 in Ping 
18:16:06.347 fetch POST /api/observability/v1/traces
18:16:06.347 fetched POST /api/observability/v1/traces 204
18:16:07.929 fetch POST /api/observability/v1/traces
18:16:07.929 fetched POST /api/observability/v1/traces 204
18:16:09.076 fetch POST /api/observability/v1/traces
18:16:09.076 fetched POST /api/observability/v1/traces 204
18:16:11.249 in Ping 
18:16:16.250 in Ping 
18:16:21.250 in Ping 
18:16:21.481 rpc 29 server.reportClientActivity
18:16:22.482 fetch POST /api/observability/v1/traces
18:16:22.482 fetched POST /api/observability/v1/traces 204
18:16:26.250 in Ping 
18:16:31.250 in Ping 
18:16:36.251 in Ping 
18:16:36.484 in Interrupt 29
18:16:37.485 fetch POST /api/observability/v1/traces
18:16:37.485 fetched POST /api/observability/v1/traces 204
18:16:41.251 in Ping 
18:16:46.251 in Ping 
18:16:46.481 rpc 30 server.reportClientActivity
18:16:47.482 fetch POST /api/observability/v1/traces
18:16:47.483 fetched POST /api/observability/v1/traces 204
18:16:51.254 in Ping 
18:16:56.258 in Ping 
18:17:01.262 in Ping 
18:17:01.482 in Interrupt 30
18:17:02.483 fetch POST /api/observability/v1/traces
18:17:02.483 fetched POST /api/observability/v1/traces 204
18:17:06.266 in Ping 
18:17:11.270 in Ping 
18:17:11.483 rpc 31 server.reportClientActivity
18:17:12.486 fetch POST /api/observability/v1/traces
18:17:12.486 fetched POST /api/observability/v1/traces 204
18:17:16.274 in Ping 
18:17:21.278 in Ping 
18:17:26.282 in Ping 
18:17:26.484 in Interrupt 31
18:17:27.485 fetch POST /api/observability/v1/traces
18:17:27.485 fetched POST /api/observability/v1/traces 204
18:17:31.286 in Ping 
18:17:36.290 in Ping 
18:17:36.482 rpc 32 server.reportClientActivity
18:17:37.483 fetch POST /api/observability/v1/traces
18:17:37.483 fetched POST /api/observability/v1/traces 204
18:17:41.294 in Ping 
18:17:46.298 in Ping 
18:17:51.302 in Ping 
18:17:51.483 in Interrupt 32
18:17:52.484 fetch POST /api/observability/v1/traces
18:17:52.484 fetched POST /api/observability/v1/traces 204
18:17:56.306 in Ping 
18:18:01.310 in Ping 
18:18:01.483 rpc 33 server.reportClientActivity
18:18:02.483 fetch POST /api/observability/v1/traces
18:18:02.483 fetched POST /api/observability/v1/traces 204
18:18:06.316 in Ping 
18:18:11.318 in Ping 
18:18:16.322 in Ping 
18:18:16.484 in Interrupt 33
18:18:17.485 fetch POST /api/observability/v1/traces
18:18:17.485 fetched POST /api/observability/v1/traces 204
18:18:21.326 in Ping 
18:18:26.330 in Ping 
18:18:26.483 rpc 34 server.reportClientActivity
18:18:27.483 fetch POST /api/observability/v1/traces
18:18:27.483 fetched POST /api/observability/v1/traces 204
18:18:31.334 in Ping 
18:18:36.338 in Ping 
18:18:41.342 in Ping 
18:18:41.484 in Interrupt 34
18:18:42.485 fetch POST /api/observability/v1/traces
18:18:42.485 fetched POST /api/observability/v1/traces 204
18:18:46.346 in Ping 
18:18:51.350 in Ping 
18:18:51.483 rpc 35 server.reportClientActivity
18:18:52.484 fetch POST /api/observability/v1/traces
18:18:52.484 fetched POST /api/observability/v1/traces 204
18:18:56.354 in Ping 
18:19:01.358 in Ping 
18:19:06.362 in Ping 
18:19:06.484 in Interrupt 35
18:19:07.485 fetch POST /api/observability/v1/traces
18:19:07.485 fetched POST /api/observability/v1/traces 204
18:19:11.366 in Ping 
18:19:16.370 in Ping 
18:19:16.484 rpc 36 server.reportClientActivity
18:19:17.484 fetch POST /api/observability/v1/traces
18:19:17.484 fetched POST /api/observability/v1/traces 204
18:19:21.374 in Ping 
18:19:26.378 in Ping 
18:19:31.382 in Ping 
18:19:31.484 in Interrupt 36
18:19:32.486 fetch POST /api/observability/v1/traces
18:19:32.487 fetched POST /api/observability/v1/traces 204
18:19:36.386 in Ping 
18:19:41.390 in Ping 
18:19:41.484 rpc 37 server.reportClientActivity
18:19:42.486 fetch POST /api/observability/v1/traces
18:19:42.486 fetched POST /api/observability/v1/traces 204
18:19:46.394 in Ping 
18:19:51.399 in Ping 
18:19:56.402 in Ping 
18:19:56.486 in Interrupt 37
18:19:57.491 fetch POST /api/observability/v1/traces
18:19:57.491 fetched POST /api/observability/v1/traces 204
18:20:01.406 in Ping 
18:20:06.410 in Ping 
18:20:06.488 rpc 38 server.reportClientActivity
18:20:07.490 fetch POST /api/observability/v1/traces
18:20:07.490 fetched POST /api/observability/v1/traces 204
18:20:11.414 in Ping 
18:20:16.418 in Ping 
18:20:21.425 in Ping 
18:20:21.490 in Interrupt 38
18:20:22.495 fetch POST /api/observability/v1/traces
18:20:22.495 fetched POST /api/observability/v1/traces 204
18:20:26.426 in Ping 
18:20:31.430 in Ping 
18:20:31.487 rpc 39 server.reportClientActivity
18:20:32.490 fetch POST /api/observability/v1/traces
18:20:32.490 fetched POST /api/observability/v1/traces 204
18:20:36.434 in Ping 
18:20:41.438 in Ping 
18:20:46.442 in Ping 
18:20:46.490 in Interrupt 39
18:20:47.494 fetch POST /api/observability/v1/traces
18:20:47.494 fetched POST /api/observability/v1/traces 204
18:20:51.446 in Ping 
18:20:56.450 in Ping 
18:20:56.491 rpc 40 server.reportClientActivity
18:20:57.495 fetch POST /api/observability/v1/traces
18:20:57.495 fetched POST /api/observability/v1/traces 204
18:21:01.454 in Ping 
18:21:06.458 in Ping 
18:21:11.462 in Ping 
18:21:11.494 in Interrupt 40
18:21:12.499 fetch POST /api/observability/v1/traces
18:21:12.499 fetched POST /api/observability/v1/traces 204
18:21:16.466 in Ping 
18:21:21.470 in Ping 
18:21:21.496 rpc 41 server.reportClientActivity
18:21:22.499 fetch POST /api/observability/v1/traces
18:21:22.499 fetched POST /api/observability/v1/traces 204
18:21:26.474 in Ping 
18:21:31.478 in Ping 
18:21:36.482 in Ping 
18:21:36.498 in Interrupt 41
18:21:37.503 fetch POST /api/observability/v1/traces
18:21:37.503 fetched POST /api/observability/v1/traces 204
18:21:41.486 in Ping 
18:21:46.490 in Ping 
18:21:46.499 rpc 42 server.reportClientActivity
18:21:47.502 fetch POST /api/observability/v1/traces
18:21:47.502 fetched POST /api/observability/v1/traces 204
18:21:51.494 in Ping 
18:21:56.498 in Ping 
18:22:01.502 in Interrupt 42
18:22:01.503 in Ping 
18:22:02.507 fetch POST /api/observability/v1/traces
18:22:02.507 fetched POST /api/observability/v1/traces 204
18:22:06.506 in Ping 
18:22:11.506 rpc 43 server.reportClientActivity
18:22:11.510 in Ping 
18:22:12.506 fetch POST /api/observability/v1/traces
18:22:12.507 fetched POST /api/observability/v1/traces 204
18:22:16.514 in Ping 
18:22:21.518 in Ping 
18:22:26.510 in Interrupt 43
18:22:26.522 in Ping 
18:22:27.515 fetch POST /api/observability/v1/traces
18:22:27.515 fetched POST /api/observability/v1/traces 204
18:22:31.526 in Ping 
18:22:36.507 rpc 44 server.reportClientActivity
18:22:36.530 in Ping 
18:22:37.510 fetch POST /api/observability/v1/traces
18:22:37.510 fetched POST /api/observability/v1/traces 204
18:22:41.534 in Ping 
18:22:46.538 in Ping 
18:22:51.510 in Interrupt 44
18:22:51.542 in Ping 
18:22:52.514 fetch POST /api/observability/v1/traces
18:22:52.514 fetched POST /api/observability/v1/traces 204
18:22:56.546 in Ping 
18:23:01.511 rpc 45 server.reportClientActivity
18:23:01.550 in Ping 
18:23:02.515 fetch POST /api/observability/v1/traces
18:23:02.515 fetched POST /api/observability/v1/traces 204
18:23:06.554 in Ping 
18:23:11.558 in Ping 
18:23:16.514 in Interrupt 45
18:23:16.562 in Ping 
18:23:17.518 fetch POST /api/observability/v1/traces
18:23:17.518 fetched POST /api/observability/v1/traces 204
18:23:21.566 in Ping 
18:23:26.515 rpc 46 server.reportClientActivity
18:23:26.570 in Ping 
18:23:27.519 fetch POST /api/observability/v1/traces
18:23:27.519 fetched POST /api/observability/v1/traces 204
18:23:31.574 in Ping 
18:23:36.578 in Ping 
18:23:41.518 in Interrupt 46
18:23:41.582 in Ping 
18:23:42.522 fetch POST /api/observability/v1/traces
18:23:42.522 fetched POST /api/observability/v1/traces 204
18:23:46.586 in Ping 
18:23:51.519 rpc 47 server.reportClientActivity
18:23:51.590 in Ping 
18:23:52.523 fetch POST /api/observability/v1/traces
18:23:52.523 fetched POST /api/observability/v1/traces 204
18:23:56.594 in Ping 
18:24:01.598 in Ping 
18:24:06.522 in Interrupt 47
18:24:06.602 in Ping 
18:24:07.526 fetch POST /api/observability/v1/traces
18:24:07.526 fetched POST /api/observability/v1/traces 204
18:24:11.606 in Ping 
18:24:16.523 rpc 48 server.reportClientActivity
18:24:16.610 in Ping 
18:24:17.527 fetch POST /api/observability/v1/traces
18:24:17.527 fetched POST /api/observability/v1/traces 204
18:24:21.614 in Ping 
18:24:26.618 in Ping 
18:24:31.526 in Interrupt 48
18:24:31.622 in Ping 
18:24:32.530 fetch POST /api/observability/v1/traces
18:24:32.530 fetched POST /api/observability/v1/traces 204
18:24:36.626 in Ping 
18:24:41.527 rpc 49 server.reportClientActivity
18:24:41.630 in Ping 
18:24:42.530 fetch POST /api/observability/v1/traces
18:24:42.530 fetched POST /api/observability/v1/traces 204
18:24:46.634 in Ping 
18:24:51.638 in Ping 
18:24:56.530 in Interrupt 49
18:24:56.642 in Ping 
18:24:57.535 fetch POST /api/observability/v1/traces
18:24:57.535 fetched POST /api/observability/v1/traces 204
18:25:01.646 in Ping 
18:25:06.532 rpc 50 server.reportClientActivity
18:25:06.650 in Ping 
18:25:07.534 fetch POST /api/observability/v1/traces
18:25:07.535 fetched POST /api/observability/v1/traces 204
18:25:11.654 in Ping 
18:25:16.658 in Ping 
18:25:21.534 in Interrupt 50
18:25:21.662 in Ping 
18:25:22.538 fetch POST /api/observability/v1/traces
18:25:22.539 fetched POST /api/observability/v1/traces 204
18:25:26.666 in Ping 
18:25:31.535 rpc 51 server.reportClientActivity
18:25:31.670 in Ping 
18:25:32.538 fetch POST /api/observability/v1/traces
18:25:32.538 fetched POST /api/observability/v1/traces 204
18:25:36.674 in Ping 
18:25:41.678 in Ping 
18:25:46.538 in Interrupt 51
18:25:46.682 in Ping 
18:25:47.543 fetch POST /api/observability/v1/traces
18:25:47.543 fetched POST /api/observability/v1/traces 204
18:25:51.686 in Ping 
18:25:56.539 rpc 52 server.reportClientActivity
18:25:56.690 in Ping 
18:25:57.542 fetch POST /api/observability/v1/traces
18:25:57.543 fetched POST /api/observability/v1/traces 204
18:26:01.694 in Ping 
18:26:06.698 in Ping 
18:26:11.542 in Interrupt 52
18:26:11.702 in Ping 
18:26:12.546 fetch POST /api/observability/v1/traces
18:26:12.546 fetched POST /api/observability/v1/traces 204
18:26:16.706 in Ping 
18:26:21.544 rpc 53 server.reportClientActivity
18:26:21.710 in Ping 
18:26:22.547 fetch POST /api/observability/v1/traces
18:26:22.547 fetched POST /api/observability/v1/traces 204
18:26:26.714 in Ping 
18:26:31.718 in Ping 
18:26:36.546 in Interrupt 53
18:26:36.722 in Ping 
18:26:37.550 fetch POST /api/observability/v1/traces
18:26:37.550 fetched POST /api/observability/v1/traces 204
18:26:41.726 in Ping 
18:26:46.547 rpc 54 server.reportClientActivity
18:26:46.730 in Ping 
18:26:47.550 fetch POST /api/observability/v1/traces
18:26:47.550 fetched POST /api/observability/v1/traces 204
18:26:51.734 in Ping 
18:26:56.740 in Ping 
18:27:01.550 in Interrupt 54
18:27:01.742 in Ping 
18:27:02.554 fetch POST /api/observability/v1/traces
18:27:02.554 fetched POST /api/observability/v1/traces 204
18:27:06.746 in Ping 
18:27:11.551 rpc 55 server.reportClientActivity
18:27:11.750 in Ping 
18:27:12.554 fetch POST /api/observability/v1/traces
18:27:12.554 fetched POST /api/observability/v1/traces 204
18:27:16.755 in Ping 
18:27:21.766 in Ping 
18:27:26.554 in Interrupt 55
18:27:26.766 in Ping 
18:27:27.559 fetch POST /api/observability/v1/traces
18:27:27.559 fetched POST /api/observability/v1/traces 204
18:27:31.770 in Ping 
18:27:36.557 rpc 56 server.reportClientActivity
18:27:36.774 in Ping 
18:27:37.558 fetch POST /api/observability/v1/traces
18:27:37.559 fetched POST /api/observability/v1/traces 204
18:27:41.778 in Ping 
18:27:46.782 in Ping 
18:27:51.558 in Interrupt 56
18:27:51.786 in Ping 
18:27:52.562 fetch POST /api/observability/v1/traces
18:27:52.562 fetched POST /api/observability/v1/traces 204
18:27:56.790 in Ping 
18:28:01.560 rpc 57 server.reportClientActivity
18:28:01.794 in Ping 
18:28:02.563 fetch POST /api/observability/v1/traces
18:28:02.563 fetched POST /api/observability/v1/traces 204
18:28:06.798 in Ping 
18:28:11.802 in Ping 
18:28:16.562 in Interrupt 57
18:28:16.806 in Ping 
18:28:17.567 fetch POST /api/observability/v1/traces
18:28:17.567 fetched POST /api/observability/v1/traces 204
18:28:21.810 in Ping 
18:28:26.563 rpc 58 server.reportClientActivity
18:28:26.814 in Ping 
18:28:27.566 fetch POST /api/observability/v1/traces
18:28:27.566 fetched POST /api/observability/v1/traces 204
18:28:31.818 in Ping 
18:28:36.822 in Ping 
18:28:41.566 in Interrupt 58
18:28:41.826 in Ping 
18:28:42.570 fetch POST /api/observability/v1/traces
18:28:42.570 fetched POST /api/observability/v1/traces 204
18:28:46.830 in Ping 
18:28:51.568 rpc 59 server.reportClientActivity
18:28:51.834 in Ping 
18:28:52.570 fetch POST /api/observability/v1/traces
18:28:52.570 fetched POST /api/observability/v1/traces 204
18:28:56.838 in Ping 
18:29:01.842 in Ping 
18:29:06.570 in Interrupt 59
18:29:06.846 in Ping 
18:29:07.574 fetch POST /api/observability/v1/traces
18:29:07.574 fetched POST /api/observability/v1/traces 204
18:29:11.850 in Ping 
18:29:16.571 rpc 60 server.reportClientActivity
18:29:16.854 in Ping 
18:29:17.574 fetch POST /api/observability/v1/traces
18:29:17.574 fetched POST /api/observability/v1/traces 204
18:29:21.858 in Ping 
18:29:26.862 in Ping 
18:29:31.574 in Interrupt 60
18:29:31.866 in Ping 
18:29:32.578 fetch POST /api/observability/v1/traces
18:29:32.578 fetched POST /api/observability/v1/traces 204
18:29:36.870 in Ping 
18:29:41.575 rpc 61 server.reportClientActivity
18:29:41.874 in Ping 
18:29:42.578 fetch POST /api/observability/v1/traces
18:29:42.578 fetched POST /api/observability/v1/traces 204
18:29:46.878 in Ping 
18:29:51.882 in Ping 
18:29:56.580 in Interrupt 61
18:29:56.886 in Ping 
18:29:57.583 fetch POST /api/observability/v1/traces
18:29:57.583 fetched POST /api/observability/v1/traces 204
18:30:01.890 in Ping 
18:30:06.579 rpc 62 server.reportClientActivity
18:30:06.894 in Ping 
18:30:07.582 fetch POST /api/observability/v1/traces
18:30:07.582 fetched POST /api/observability/v1/traces 204
18:30:11.899 in Ping 
18:30:16.902 in Ping 
18:30:21.582 in Interrupt 62
18:30:21.906 in Ping 
18:30:22.586 fetch POST /api/observability/v1/traces
18:30:22.586 fetched POST /api/observability/v1/traces 204
18:30:26.910 in Ping 
18:30:31.585 rpc 63 server.reportClientActivity
18:30:31.914 in Ping 
18:30:32.587 fetch POST /api/observability/v1/traces
18:30:32.587 fetched POST /api/observability/v1/traces 204
18:30:36.918 in Ping 
18:30:41.922 in Ping 
18:30:46.586 in Interrupt 63
18:30:46.926 in Ping 
18:30:47.591 fetch POST /api/observability/v1/traces
18:30:47.591 fetched POST /api/observability/v1/traces 204
18:30:51.930 in Ping 
18:30:56.587 rpc 64 server.reportClientActivity
18:30:56.934 in Ping 
18:30:57.591 fetch POST /api/observability/v1/traces
18:30:57.591 fetched POST /api/observability/v1/traces 204
18:31:01.938 in Ping 
