# Baseline reports

The untouched t3code v0.1.7 web client over WebSocket, with compression (`websocket`) and
without it like the unchanged iOS app (`websocket-nodeflate`). Seed 1. See [lab/README.md](../../README.md)
for the profiles and what each measure means.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's
userspace stack (issue #7), so production numbers differ.

Lab commit of these runs: 3be906ff21e3, cc6b9676f812.

| Profile | Grade | Client | Start | Status | Prompt confirmed | Prompt reached agent | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Reply received/duplicates (B) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| [A](A-websocket-warm/report.md) | graded | websocket | warm | done | 0.3s | 0.5s | 10.9s | 16.4s | 16.7s | 1596/1596/1596 | 170960/153021 | 2294/698 |
| [A](A-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 0.3s | 0.6s | 10.7s | 16.3s | 17.0s | 1596/1596/1596 | 277461/147173 | 2294/698 |
| [A](A-websocket-cold/report.md) | graded | websocket | cold | done | 0.2s | 0.3s | 10.5s | 16.6s | 17.0s | 1596/1596/1596 | 3189240/331692 | 2294/698 |
| [A](A-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 0.2s | 0.3s | 10.6s | 16.6s | 16.8s | 1596/1596/1596 | 3356853/347695 | 2294/698 |
| [B](B-websocket-warm/report.md) | graded | websocket | warm | done | 2.0s | 1.6s | 12.3s | 17.3s | 16.9s | 1596/1596/1596 | 33329/132931 | 2294/698 |
| [B](B-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 1.7s | 1.2s | 12.1s | 16.7s | 16.8s | 1596/1596/1596 | 121455/144865 | 2294/698 |
| [B](B-websocket-cold/report.md) | graded | websocket | cold | done | 1.5s | 1.1s | 11.9s | 16.8s | 16.6s | 1596/1596/1596 | 3067524/375039 | 2294/698 |
| [B](B-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 5.0s | 0.9s | 13.9s | 321.9s | 316.7s | 1596/1596/1596 | 3448408/724539 | 2294/698 |
| [C](C-websocket-warm/report.md) | graded | websocket | warm | done | 2.9s | 2.5s | 13.1s | 17.4s | 16.9s | 1596/1596/1596 | 36465/132335 | 2294/698 |
| [C](C-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 1.7s | 1.5s | 12.3s | 17.4s | 17.0s | 1596/1596/1596 | 124912/161784 | 2294/698 |
| [C](C-websocket-cold/report.md) | graded | websocket | cold | done | 1.1s | 0.8s | 11.6s | 17.1s | 16.8s | 1596/1596/1596 | 3073441/412090 | 2294/698 |
| [C](C-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 8.6s | 0.8s | 14.2s | 17.0s | 14.4s | 1596/1596/1596 | 3247450/408767 | 2294/698 |
| [D](D-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 27.4s | 15.0s | 36.9s | 873.9s | 862.3s | 397/1077/1596 | 35092/220811 | 397/0 |
| [D](D-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | 13.1s | 6.1s | never | 892.8s | 899.2s | 0/1077/1596 | 60239/363523 | 0/0 |
| [D](D-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 113156/39300 | 0/0 |
| [D](D-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 54282/10677 | 0/0 |
| [E](E-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 2397/1938 | 0/0 |
| [E](E-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 5902/25880 | 0/0 |
| [E](E-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 3002/911 | 0/0 |
| [E](E-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 7167/27189 | 0/0 |
| [E-iid](E-iid-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | never | never | no text produced | 899.5s | 0/0/1596 | 2382/2204 | 0/0 |
| [E-iid](E-iid-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 6384/5739 | 0/0 |
| [F](F-websocket-warm/report.md) | diagnostic | websocket | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 1612/824 | 0/0 |
| [F](F-websocket-nodeflate-warm/report.md) | diagnostic | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.3s | 0/0/1596 | 1613/1981 | 0/0 |
| [case70](case70-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | never | never | never | no text produced | 899.3s | 0/0/1596 | 2238/6271 | 0/0 |
| [case70](case70-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | never | 14.4s | never | 884.5s | 899.4s | 0/1077/1596 | 11167/21773 | 0/0 |
| [case70](case70-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 4147/2379 | 0/0 |
| [case70](case70-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 3001/2754 | 0/0 |
| [case50](case50-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 3.5s | 0.7s | 15.9s | 888.2s | 883.5s | 397/1077/1596 | 11180/33014 | 397/0 |
| [case50](case50-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | 32.4s | 31.3s | never | 867.6s | 899.2s | 0/1077/1596 | 32205/121616 | 0/0 |
| [case50](case50-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 32790/41196 | 0/0 |
| [case50](case50-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 17357/4421 | 0/0 |
| [loss30](loss30-websocket-warm/report.md) | fault | websocket | warm | done | 1.0s | 0.8s | 11.3s | 18.1s | 18.3s | 1596/1596/1596 | 20882/56565 | 2294/698 |
| [loss30](loss30-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 4.7s | 2.0s | never | 896.9s | 899.3s | 0/1077/1596 | 186614/454974 | 0/0 |
| [loss90](loss90-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | never | never | no text produced | 899.5s | 0/0/1596 | 608/959 | 0/0 |
| [loss90](loss90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 1589/21935 | 0/0 |
| [burst90](burst90-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | never | never | no text produced | 899.3s | 0/0/1596 | 3682/12121 | 0/0 |
| [burst90](burst90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.4s | 0/0/1596 | 4470/7360 | 0/0 |
| [rate100](rate100-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 24.7s | 21.0s | never | 877.9s | 899.3s | 0/1077/1596 | 24360/68911 | 0/0 |
| [rate100](rate100-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | 17.1s | never | 881.8s | 899.3s | 0/1077/1596 | 26595/57763 | 0/0 |
| [rtt3000](rtt3000-websocket-warm/report.md) | fault | websocket | warm | done | 4.6s | 3.0s | 16.0s | 18.8s | 22.2s | 1596/1596/1596 | 25101/93435 | 2294/698 |
| [rtt3000](rtt3000-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 6.2s | 4.4s | 16.9s | 18.5s | 18.7s | 1596/1596/1596 | 126777/100400 | 2294/698 |
| [reorder](reorder-websocket-warm/report.md) | fault | websocket | warm | done | 1.6s | 1.5s | 11.8s | 16.4s | 17.1s | 1596/1596/1596 | 33707/121370 | 2294/698 |
| [reorder](reorder-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 3.0s | 2.9s | 14.6s | 17.0s | 16.4s | 1596/1596/1596 | 129532/147475 | 2294/698 |
| [duplicate](duplicate-websocket-warm/report.md) | fault | websocket | warm | done | 0.9s | 0.7s | 12.8s | 16.2s | 15.2s | 1596/1596/1596 | 45180/135831 | 2294/698 |
| [duplicate](duplicate-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 1.0s | 0.7s | 11.5s | 21.2s | 20.8s | 1596/1596/1596 | 167042/200015 | 2294/698 |
| [corrupt](corrupt-websocket-warm/report.md) | fault | websocket | warm | done | 0.9s | 0.7s | 13.1s | 17.2s | 15.1s | 1596/1596/1596 | 38784/135201 | 2294/698 |
| [corrupt](corrupt-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 2.8s | 0.7s | 11.6s | 17.2s | 17.1s | 1596/1596/1596 | 135249/167251 | 2294/698 |
| [asym](asym-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | never | never | no text produced | 899.1s | 0/0/1596 | 36125/67803 | 0/0 |
| [asym](asym-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 11.3s | 11.2s | 29.3s | 844.2s | 860.7s | 1077/1596/1596 | 136891/87029 | 1474/397 |
| [outage10](outage10-websocket-warm/report.md) | fault | websocket | warm | done | 0.5s | 0.4s | 30.9s | 29.5s | 30.9s | 1596/1596/1596 | 63681/187635 | 2294/698 |
| [outage10](outage10-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.4s | 0.4s | 24.7s | 23.5s | 24.7s | 1596/1596/1596 | 125159/133329 | 2294/698 |
| [outage60](outage60-websocket-warm/report.md) | fault | websocket | warm | done | 0.5s | 0.4s | 84.8s | 83.5s | 84.8s | 1596/1596/1596 | 74898/259550 | 2294/698 |
| [outage60](outage60-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.5s | 0.4s | 81.8s | 80.5s | 81.8s | 1596/1596/1596 | 208712/314638 | 2294/698 |
| [flap](flap-websocket-warm/report.md) | fault | websocket | warm | done | 0.4s | 0.4s | 18.2s | 19.5s | 18.2s | 1596/1596/1596 | 42740/127153 | 2294/698 |
| [flap](flap-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.4s | 0.4s | 16.8s | 19.5s | 16.8s | 1596/1596/1596 | 150943/174936 | 2294/698 |
| [addrchange](addrchange-websocket-warm/report.md) | fault | websocket | warm | done | 0.5s | 0.5s | 10.9s | 26.4s | 26.3s | 1596/1596/1596 | 77575/233337 | 2294/698 |
| [addrchange](addrchange-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 0.7s | 0.7s | 11.2s | 888.2s | 888.0s | 397/1077/1596 | 78063/52952 | 397/0 |
| [derp](derp-websocket-warm/report.md) | fault | websocket | warm | done | 1.0s | 0.9s | 11.3s | 17.0s | 17.0s | 1596/1596/1596 | 62891/164790 | 2294/698 |
| [derp](derp-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 2.0s | 1.2s | 12.0s | 17.7s | 17.7s | 1596/1596/1596 | 161780/199897 | 2294/698 |
| [soak](soak-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 15.2s | 2.7s | never | 3596.2s | 3599.0s | 0/1077/125238 | 41021/175939 | 0/0 |
| [soak](soak-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 11.2s | 1.9s | never | 3596.0s | 3598.9s | 0/1077/125238 | 59299/241268 | 0/0 |
