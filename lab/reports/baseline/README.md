# Baseline reports

The untouched t3code v0.1.7 web client over WebSocket, with compression (`websocket`) and
without it like the unchanged iOS app (`websocket-nodeflate`). Seed 1. See [lab/README.md](../../README.md)
for the profiles and what each measure means.

Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in tailscaled's
userspace stack (issue #7), so production numbers differ.

Lab commit of these runs: 0f01ba755fce-dirty, 41bbe7f3bf7e-dirty, 594d30a3ef10-dirty, 5e3d34bc8cb8-dirty, afcea2ca249e-dirty, b7cefb8394e5-dirty, e14300ea99fc.

All 64 runs are from one rerun on 2026-11-08, 11:13 to 15:07 UTC, on the lab fixed in #46. The lab binary was rebuilt during the rerun; between e14300ea and 0f01ba75 only the acceptance grading (`internal/report/acceptance.go`), a doc comment, the README and `build-pipe-lab.sh` changed, nothing that drives or measures a run. "-dirty" is the reports of the rerun itself, not yet committed.

| Profile | Grade | Client | Start | Status | Prompt confirmed | Prompt reached agent | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Reply received/duplicates (B) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| [A](A-websocket-warm/report.md) | graded | websocket | warm | done | 0.2s | 0.4s | 10.6s | 16.5s | 16.8s | 1596/1596/1596 | 162829/118056 | 2294/698 |
| [A](A-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 0.2s | 0.4s | 10.7s | 16.5s | 16.9s | 1596/1596/1596 | 261891/127164 | 2294/698 |
| [A](A-websocket-cold/report.md) | graded | websocket | cold | done | 0.2s | 0.4s | 10.7s | 16.5s | 16.8s | 1596/1596/1596 | 3187181/323291 | 2294/698 |
| [A](A-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 0.2s | 0.3s | 10.6s | 16.6s | 16.8s | 1596/1596/1596 | 3388422/333909 | 2294/698 |
| [B](B-websocket-warm/report.md) | graded | websocket | warm | done | 0.7s | 0.6s | 11.3s | 17.3s | 16.8s | 1596/1596/1596 | 30787/122931 | 2294/698 |
| [B](B-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 0.8s | 0.7s | 11.6s | 17.2s | 17.0s | 1596/1596/1596 | 159224/133002 | 2294/698 |
| [B](B-websocket-cold/report.md) | graded | websocket | cold | done | 1.2s | 0.8s | 11.4s | 17.1s | 17.0s | 1596/1596/1596 | 3074356/345094 | 2294/698 |
| [B](B-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 2.7s | 2.4s | 13.3s | 17.5s | 16.9s | 1596/1596/1596 | 3270340/381463 | 2294/698 |
| [C](C-websocket-warm/report.md) | graded | websocket | warm | done | 1.0s | 0.8s | 11.6s | 17.1s | 17.3s | 1596/1596/1596 | 30710/98628 | 2294/698 |
| [C](C-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | done | 0.9s | 0.8s | 11.7s | 17.1s | 16.9s | 1596/1596/1596 | 125705/137802 | 2294/698 |
| [C](C-websocket-cold/report.md) | graded | websocket | cold | done | 1.1s | 0.8s | 11.5s | 17.1s | 16.7s | 1596/1596/1596 | 3078485/392993 | 2294/698 |
| [C](C-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | done | 3.3s | 0.9s | 13.1s | 17.0s | 15.8s | 1596/1596/1596 | 3274680/408495 | 2294/698 |
| [D](D-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 6.2s | 3.5s | 20.8s | 884.4s | 878.0s | 397/1077/1596 | 14459/40136 | 397/0 |
| [D](D-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | 6.5s | 3.7s | never | 895.2s | 899.1s | 0/1077/1596 | 35880/180072 | 0/0 |
| [D](D-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 113119/28412 | 0/0 |
| [D](D-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 98876/21710 | 0/0 |
| [E](E-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 5.2s | 3.6s | never | 895.3s | 899.3s | 0/1077/1596 | 3282/5479 | 0/0 |
| [E](E-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | never | 7.6s | never | 891.3s | 899.4s | 0/1077/1596 | 3959/1578 | 0/0 |
| [E](E-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2578/864 | 0/0 |
| [E](E-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2574/767 | 0/0 |
| [E-iid](E-iid-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | 8.4s | never | 890.5s | 899.5s | 0/1077/1596 | 1630/2429 | 0/0 |
| [E-iid](E-iid-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | 12.1s | never | 886.8s | 899.5s | 0/1077/1596 | 2088/2757 | 0/0 |
| [F](F-websocket-warm/report.md) | diagnostic | websocket | warm | not done by deadline | never | 37.1s | never | 861.8s | 899.5s | 0/1077/1596 | 3102/508 | 0/0 |
| [F](F-websocket-nodeflate-warm/report.md) | diagnostic | websocket-nodeflate | warm | not done by deadline | never | 98.9s | never | 800.0s | 899.5s | 0/1077/1596 | 2574/855 | 0/0 |
| [case70](case70-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 5.5s | 4.4s | never | 894.5s | 899.5s | 0/1077/1596 | 4322/6537 | 0/0 |
| [case70](case70-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | 9.2s | 4.9s | never | 894.0s | 899.4s | 0/1077/1596 | 5276/3360 | 0/0 |
| [case70](case70-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2066/946 | 0/0 |
| [case70](case70-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2000/1012 | 0/0 |
| [case50](case50-websocket-warm/report.md) | graded | websocket | warm | not done by deadline | 16.0s | 15.5s | 36.2s | 836.6s | 847.2s | 1077/1596/1596 | 49362/282281 | 1474/397 |
| [case50](case50-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | warm | not done by deadline | 2.7s | 2.5s | never | 896.4s | 899.2s | 0/1077/1596 | 43407/143216 | 0/0 |
| [case50](case50-websocket-cold/report.md) | graded | websocket | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 34720/2566 | 0/0 |
| [case50](case50-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | cold | app never loaded by deadline | never | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 50635/3554 | 0/0 |
| [loss30](loss30-websocket-warm/report.md) | fault | websocket | warm | done | 1.4s | 1.0s | 14.1s | 25.8s | 26.6s | 1596/1596/1596 | 23749/88561 | 2294/698 |
| [loss30](loss30-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 1.6s | 1.1s | 14.1s | 37.4s | 60.9s | 1596/1596/1596 | 139481/142997 | 2294/698 |
| [loss70](loss70-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 4.6s | 4.3s | never | 894.6s | 899.4s | 0/1077/1596 | 2303/4166 | 0/0 |
| [loss70](loss70-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 5.1s | 4.3s | never | 894.6s | 899.4s | 0/1077/1596 | 3922/4850 | 0/0 |
| [loss90](loss90-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | never | never | never | no text produced | 899.3s | 0/0/1596 | 362/1000 | 0/0 |
| [loss90](loss90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | never | never | never | no text produced | 899.5s | 0/0/1596 | 214/277 | 0/0 |
| [burst90](burst90-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 0.5s | 0.5s | never | 898.4s | 899.5s | 0/1077/1596 | 4190/408 | 0/0 |
| [burst90](burst90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 0.6s | 0.5s | never | 898.4s | 899.5s | 0/1077/1596 | 8304/933 | 0/0 |
| [rate100](rate100-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 4.4s | 3.3s | never | 895.6s | 899.3s | 0/1077/1596 | 31330/71706 | 0/0 |
| [rate100](rate100-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 10.0s | 8.3s | never | 889.6s | 899.0s | 0/1077/1596 | 26731/67643 | 0/0 |
| [rtt3000](rtt3000-websocket-warm/report.md) | fault | websocket | warm | done | 3.1s | 1.6s | 15.0s | 32.0s | 36.1s | 1596/1596/1596 | 67615/79855 | 2294/698 |
| [rtt3000](rtt3000-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 2.4s | 1.6s | 14.1s | 18.3s | 18.1s | 1596/1596/1596 | 136381/79519 | 2294/698 |
| [reorder](reorder-websocket-warm/report.md) | fault | websocket | warm | done | 0.5s | 0.3s | 10.9s | 16.6s | 16.8s | 1596/1596/1596 | 35699/126589 | 2294/698 |
| [reorder](reorder-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 2.7s | 0.4s | 11.2s | 17.5s | 17.6s | 1596/1596/1596 | 126784/136009 | 2294/698 |
| [duplicate](duplicate-websocket-warm/report.md) | fault | websocket | warm | done | 0.8s | 0.6s | 11.3s | 17.3s | 16.9s | 1596/1596/1596 | 43938/156675 | 2294/698 |
| [duplicate](duplicate-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.7s | 0.6s | 11.6s | 17.3s | 16.9s | 1596/1596/1596 | 153224/160887 | 2294/698 |
| [corrupt](corrupt-websocket-warm/report.md) | fault | websocket | warm | done | 1.0s | 0.6s | 11.2s | 17.3s | 16.9s | 1596/1596/1596 | 36182/134327 | 2294/698 |
| [corrupt](corrupt-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.7s | 0.7s | 13.1s | 18.2s | 16.3s | 1596/1596/1596 | 128036/150655 | 2294/698 |
| [asym](asym-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 6.6s | 6.6s | never | 892.3s | 899.2s | 0/1077/1596 | 33695/108418 | 0/0 |
| [asym](asym-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 8.6s | 8.6s | never | 890.3s | 899.2s | 0/1077/1596 | 69885/124289 | 0/0 |
| [outage10](outage10-websocket-warm/report.md) | fault | websocket | warm | done | 0.4s | 0.4s | 25.0s | 24.5s | 25.0s | 1596/1596/1596 | 41656/128802 | 2294/698 |
| [outage10](outage10-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.5s | 0.5s | 23.4s | 22.4s | 23.4s | 1596/1596/1596 | 122379/52616 | 2294/698 |
| [outage60](outage60-websocket-warm/report.md) | fault | websocket | warm | done | 0.4s | 0.4s | 87.6s | 86.5s | 87.6s | 1596/1596/1596 | 74695/291969 | 2294/698 |
| [outage60](outage60-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.5s | 0.5s | 132.4s | 131.4s | 132.4s | 1596/1596/1596 | 216251/336528 | 2294/698 |
| [flap](flap-websocket-warm/report.md) | fault | websocket | warm | done | 0.5s | 0.5s | 14.2s | 18.4s | 15.4s | 1596/1596/1596 | 39655/98474 | 2294/698 |
| [flap](flap-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.5s | 0.5s | 17.5s | 21.4s | 17.5s | 1596/1596/1596 | 174199/148963 | 2294/698 |
| [addrchange](addrchange-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 0.4s | 0.4s | 10.8s | 888.5s | 888.5s | 397/1077/1596 | 37148/41143 | 397/0 |
| [addrchange](addrchange-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 0.5s | 0.5s | 11.2s | 182.4s | 182.3s | 1596/1596/1596 | 275989/368456 | 2294/698 |
| [derp](derp-websocket-warm/report.md) | fault | websocket | warm | done | 0.8s | 0.4s | 10.9s | 16.5s | 16.8s | 1596/1596/1596 | 54587/139637 | 2294/698 |
| [derp](derp-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | done | 3.1s | 1.9s | 13.5s | 18.0s | 16.5s | 1596/1596/1596 | 162581/172103 | 2294/698 |
| [soak](soak-websocket-warm/report.md) | fault | websocket | warm | not done by deadline | 15.8s | 15.4s | 29.2s | 16.5s | 3556.3s | 1077/1077/125238 | 69389/313194 | 1474/397 |
| [soak](soak-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | warm | not done by deadline | 15.4s | 15.3s | never | 3583.6s | 3599.3s | 0/1077/125238 | 51193/168979 | 0/0 |
