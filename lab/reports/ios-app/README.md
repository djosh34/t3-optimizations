# The unchanged iOS app: v0.1.7 server against the forked server

The untouched v0.1.7 web client without WebSocket compression (`websocket-nodeflate`, as the
iOS app's SocketRocket talks; issue #7), against the untouched v0.1.7 server and against the
forked server of `t3code/`, on the same profiles and seeds. Warm starts. See
[lab/README.md](../../README.md) for the profiles and what each measure means. What the runs
show, the run conditions and the runs repeated alone are in [findings.md](findings.md).

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's
userspace stack (issue #7), so production numbers differ.

Lab commit of these runs: 16112c251278-dirty.

| Profile | Seed | Server | Status | Prompt confirmed | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Reply received/duplicates (B) |
|---|---|---|---|---|---|---|---|---|---|---|
| A | 1 | [v0.1.7](A-v0.1.7-s1/report.md) | done | 0.2s | 10.5s | 16.6s | 16.8s | 1596/1596/1596 | 262671/124171 | 2294/698 |
| A | 1 | [fork](A-fork-s1/report.md) | done | 0.2s | 10.5s | 16.6s | 17.0s | 1596/1596/1596 | 261277/125387 | 2294/698 |
| A | 2 | [v0.1.7](A-v0.1.7-s2/report.md) | done | 0.2s | 10.5s | 16.6s | 16.8s | 1596/1596/1596 | 265999/128893 | 2294/698 |
| A | 2 | [fork](A-fork-s2/report.md) | done | 0.2s | 10.5s | 16.6s | 17.0s | 1596/1596/1596 | 262050/125559 | 2294/698 |
| B | 1 | [v0.1.7](B-v0.1.7-s1/report.md) | done | 0.7s | 11.5s | 17.3s | 16.8s | 1596/1596/1596 | 119588/125438 | 2294/698 |
| B | 1 | [fork](B-fork-s1/report.md) | done | 0.8s | 11.4s | 17.3s | 17.0s | 1596/1596/1596 | 119903/126744 | 2294/698 |
| B | 2 | [v0.1.7](B-v0.1.7-s2/report.md) | done | 0.8s | 11.6s | 17.3s | 16.8s | 1596/1596/1596 | 124052/129357 | 2294/698 |
| B | 2 | [fork](B-fork-s2/report.md) | done | 0.8s | 11.3s | 17.3s | 17.2s | 1596/1596/1596 | 122218/126532 | 2294/698 |
| C | 1 | [v0.1.7](C-v0.1.7-s1/report.md) | done | 0.9s | 11.4s | 17.3s | 17.0s | 1596/1596/1596 | 122679/134370 | 2294/698 |
| C | 1 | [fork](C-fork-s1/report.md) | done | 0.9s | 11.4s | 17.2s | 17.0s | 1596/1596/1596 | 123962/135825 | 2294/698 |
| C | 2 | [v0.1.7](C-v0.1.7-s2/report.md) | done | 0.9s | 11.4s | 18.3s | 17.8s | 1596/1596/1596 | 115194/125374 | 2294/698 |
| C | 2 | [fork](C-fork-s2/report.md) | done | 0.9s | 11.3s | 17.2s | 17.2s | 1596/1596/1596 | 122390/127481 | 2294/698 |
| D | 1 | [v0.1.7](D-v0.1.7-s1/report.md) | not done by deadline | 6.3s | never | 895.3s | 899.3s | 0/1077/1596 | 24750/95335 | 0/0 |
| D | 1 | [fork](D-fork-s1/report.md) | not done by deadline | 6.2s | 24.1s | 885.3s | 875.3s | 397/1077/1596 | 46753/63118 | 397/0 |
| D | 2 | [v0.1.7](D-v0.1.7-s2/report.md) | not done by deadline | 6.4s | never | 895.1s | 899.1s | 0/1077/1596 | 38740/21482 | 0/0 |
| D | 2 | [fork](D-fork-s2/report.md) | not done by deadline | 6.9s | 18.4s | 885.3s | 880.7s | 397/1077/1596 | 60838/161112 | 397/0 |
| E | 1 | [v0.1.7](E-v0.1.7-s1/report.md) | not done by deadline | never | never | 891.3s | 899.2s | 0/1077/1596 | 3126/1077 | 0/0 |
| E | 1 | [fork](E-fork-s1/report.md) | not done by deadline | never | never | 891.2s | 899.2s | 0/1077/1596 | 2050/855 | 0/0 |
| E | 2 | [v0.1.7](E-v0.1.7-s2/report.md) | not done by deadline | 9.8s | never | 891.4s | 899.4s | 0/1077/1596 | 1684/1198 | 0/0 |
| E | 2 | [fork](E-fork-s2/report.md) | not done by deadline | 9.8s | never | 891.4s | 899.5s | 0/1077/1596 | 1684/1198 | 0/0 |
| case70 | 1 | [v0.1.7](case70-v0.1.7-s1/report.md) | not done by deadline | 9.1s | never | 894.1s | 899.4s | 0/1077/1596 | 6993/3500 | 0/0 |
| case70 | 1 | [fork](case70-fork-s1/report.md) | not done by deadline | 6.8s | never | 894.0s | 899.4s | 0/1077/1596 | 4578/3455 | 0/0 |
| case70 | 2 | [v0.1.7](case70-v0.1.7-s2/report.md) | not done by deadline | 5.4s | never | 897.2s | 899.3s | 0/1077/1596 | 11501/8908 | 0/0 |
| case70 | 2 | [fork](case70-fork-s2/report.md) | not done by deadline | 5.6s | never | 897.2s | 899.3s | 0/1077/1596 | 7284/5381 | 0/0 |
| case50 | 1 | [v0.1.7](case50-v0.1.7-s1/report.md) | not done by deadline | 15.6s | never | 883.5s | 899.5s | 0/1077/1596 | 31039/136348 | 0/0 |
| case50 | 1 | [fork](case50-fork-s1/report.md) | not done by deadline | 15.3s | 33.3s | 873.9s | 866.0s | 397/1077/1596 | 47125/13871 | 397/0 |
| case50 | 2 | [v0.1.7](case50-v0.1.7-s2/report.md) | not done by deadline | 8.8s | never | 897.8s | 899.5s | 0/1077/1596 | 21691/63128 | 0/0 |
| case50 | 2 | [fork](case50-fork-s2/report.md) | not done by deadline | 4.6s | never | 897.8s | 899.4s | 0/1077/1596 | 35969/87900 | 0/0 |
| loss30 | 1 | [v0.1.7](loss30-v0.1.7-s1/report.md) | not done by deadline | 1.6s | never | 897.8s | 899.4s | 0/1077/1596 | 106963/430629 | 0/0 |
| loss30 | 1 | [fork](loss30-fork-s1/report.md) | not done by deadline | 1.8s | 18.5s | 855.7s | 858.7s | 1077/1596/1596 | 211267/694914 | 1474/397 |
| loss30 | 2 | [v0.1.7](loss30-v0.1.7-s2/report.md) | done | 1.9s | 32.4s | 30.6s | 32.4s | 1596/1596/1596 | 102496/70864 | 2294/698 |
| loss30 | 2 | [fork](loss30-fork-s2/report.md) | done | 2.2s | 26.0s | 24.7s | 26.0s | 1596/1596/1596 | 97793/54592 | 2294/698 |
| rate100 | 1 | [v0.1.7](rate100-v0.1.7-s1/report.md) | not done by deadline | 10.0s | never | 890.7s | 899.3s | 0/1077/1596 | 25572/65355 | 0/0 |
| rate100 | 1 | [fork](rate100-fork-s1/report.md) | not done by deadline | 10.0s | never | 890.7s | 899.4s | 0/1077/1596 | 25889/65779 | 0/0 |
| rate100 | 2 | [v0.1.7](rate100-v0.1.7-s2/report.md) | not done by deadline | 10.0s | never | 890.7s | 899.4s | 0/1077/1596 | 26572/67225 | 0/0 |
| rate100 | 2 | [fork](rate100-fork-s2/report.md) | not done by deadline | 10.0s | never | 890.7s | 899.4s | 0/1077/1596 | 25991/65299 | 0/0 |
| rtt3000 | 1 | [v0.1.7](rtt3000-v0.1.7-s1/report.md) | done | 3.1s | 15.3s | 18.3s | 20.9s | 1596/1596/1596 | 115873/54802 | 2294/698 |
| rtt3000 | 1 | [fork](rtt3000-fork-s1/report.md) | done | 3.1s | 13.5s | 18.3s | 17.0s | 1596/1596/1596 | 110002/61747 | 2294/698 |
| rtt3000 | 2 | [v0.1.7](rtt3000-v0.1.7-s2/report.md) | done | 3.3s | 15.6s | 18.8s | 17.3s | 1596/1596/1596 | 120223/59535 | 2294/698 |
| rtt3000 | 2 | [fork](rtt3000-fork-s2/report.md) | done | 3.3s | 14.1s | 18.8s | 21.2s | 1596/1596/1596 | 112074/71049 | 2294/698 |
| outage10 | 1 | [v0.1.7](outage10-v0.1.7-s1/report.md) | done | 0.4s | 23.0s | 21.5s | 23.0s | 1596/1596/1596 | 123764/141937 | 2294/698 |
| outage10 | 1 | [fork](outage10-fork-s1/report.md) | done | 0.4s | 25.1s | 24.5s | 25.1s | 1596/1596/1596 | 133884/147163 | 2294/698 |
| outage10 | 2 | [v0.1.7](outage10-v0.1.7-s2/report.md) | done | 0.5s | 22.5s | 21.3s | 22.5s | 1596/1596/1596 | 117224/54586 | 2294/698 |
| outage10 | 2 | [fork](outage10-fork-s2/report.md) | done | 0.5s | 24.7s | 23.4s | 24.7s | 1596/1596/1596 | 131144/144568 | 2294/698 |
| flap | 1 | [v0.1.7](flap-v0.1.7-s1/report.md) | done | 0.5s | 16.8s | 21.5s | 16.8s | 1596/1596/1596 | 161454/150556 | 2294/698 |
| flap | 1 | [fork](flap-fork-s1/report.md) | done | 0.5s | 17.8s | 20.5s | 17.8s | 1596/1596/1596 | 140859/138929 | 2294/698 |
| flap | 2 | [v0.1.7](flap-v0.1.7-s2/report.md) | done | 0.5s | 16.6s | 21.5s | 16.6s | 1596/1596/1596 | 135431/139436 | 2294/698 |
| flap | 2 | [fork](flap-fork-s2/report.md) | done | 0.4s | 15.9s | 23.5s | 18.6s | 1596/1596/1596 | 150527/177164 | 2294/698 |

| Server | Runs | Done | Prompt confirmed | Reply chars shown | Wire down (B) | Reply received/duplicates (B) |
|---|---|---|---|---|---|---|
| v0.1.7 | 24 | 13 | 23 | 20748 | 2185279 | 29822/9074 |
| fork | 24 | 13 | 23 | 23016 | 2357511 | 32487/9471 |
