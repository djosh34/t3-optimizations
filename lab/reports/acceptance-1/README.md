# Acceptance, first attempt (stopped)

The first acceptance suite on the web app, stopped on 2026-11-08 after 16 of its 46 runs (3 at once, started 15:07 UTC): the runs showed two defects, fixed in 9baf4844, that the remaining runs would only have measured again (the composer ignoring the first Enter while the open thread's row was being updated, and a prompt held behind the client's own `thread.visit`). The load average was about 20 on 8 cores during these runs, which can stretch the browser's timers. The full suite on the fixed build is in [../acceptance](../acceptance/README.md). Web app build of these runs: sidecar from c8e9f49e, web app from 0f01ba75.

WebSocket baseline in [../baseline](../baseline/README.md) on the same profiles, starts and seed.

Commit of these runs: 0f01ba755fce-dirty.

## Targets

- Case 70 from a warm start, a typed prompt confirmed within 10 s: **not met**. case70-webapp-warm 3.9s, case70-webapp-warm-s2 6.5s, case70-webapp-warm-s3 48.4s.
- Case 70 from a cold start, a typed prompt confirmed within 10 s: **not met**. case70-webapp-cold never.
- Case 50 from a warm start, the newest text never more than 5 s behind: **not met**. Max lag: No runs.
- Case 50 from a cold start, the newest text never more than 5 s behind: **not met**. Max lag: No runs.
- Profiles A to E, text keeps moving and the prompt is confirmed before the deadline (the whole reply shown): **not met**. A-webapp-warm done, prompt 0.4s, A-webapp-cold done, prompt 0.4s, B-webapp-warm done, prompt 1.9s, B-webapp-cold done, prompt 7.5s, C-webapp-warm done, prompt 1.5s, C-webapp-cold done, prompt 7.5s, D-webapp-warm done, prompt 8.6s, D-webapp-cold done, prompt 290.7s, E-webapp-warm not done by deadline, prompt never, E-webapp-cold not done by deadline, prompt never.
- No content delivered twice to the application layer: **met**. Bytes sync-core handed to the web app twice, over all 16 runs: 0.
- After every outage, sync resumes with no user action (runs with outages or an address change, the whole reply shown): **not met**. B-webapp-warm done, B-webapp-cold done, D-webapp-warm done, D-webapp-cold done, E-webapp-warm not done by deadline, E-webapp-cold not done by deadline, E-iid-webapp-warm not done by deadline.
- The newest text and prompt acks overtake backfill in every run: **not met**. When the sidecar queued the prompt's reply and the reply's first newest text, it still had backfill (older messages, the thread list, other threads, images) it had never sent, which the priority order sends after them, and both then reached the phone, in 11 runs. This shows the send order; on a lossy link a later packet can still arrive first. Not established in 4 (the phone got no confirmation or no text, no line for a moment, or no backfill waiting then): E-webapp-warm, E-webapp-cold, E-iid-webapp-warm, case70-webapp-cold. Not met means not established in every run, not a failure seen. Per run below.
- Open defect: the web app's composer ignored Enter, and the driver pressed it again, in 0 of 16 runs: No runs. A user who does not press again never gets the prompt sent. The prompt's times count from the first Enter.

Profile F and `derp` are reported with no target. Backfill bytes never sent when the sidecar queued the prompt's reply / the reply's first newest text:

- A-webapp-warm: 13964 / 15697
- A-webapp-cold: 12699 / 14559
- B-webapp-warm: 6202 / 8996
- B-webapp-cold: 4184 / 7195
- C-webapp-warm: 2680 / 5691
- C-webapp-cold: 12383 / 14281
- D-webapp-warm: 1633 / 4644
- D-webapp-cold: 10325 / 12670
- E-webapp-warm: n/a / n/a
- E-webapp-cold: n/a / n/a
- E-iid-webapp-warm: n/a / n/a
- F-webapp-warm: n/a / n/a
- case70-webapp-warm: 3476 / 6494
- case70-webapp-warm-s2: 7002 / 10021
- case70-webapp-warm-s3: 7326 / 10051
- case70-webapp-cold: n/a / n/a

## Every run against the baseline

Each web app run, and below it the WebSocket baseline on the same profile, start and seed (`websocket`, `websocket-nodeflate`). Prompt is from Enter to the confirmation; text is from Enter to the first reply text on screen; lag is the newest-text lag; silence is the longest time with no new reply text.

| Run | Grade | Client | Status | Prompt | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Received/duplicates (B) |
|---|---|---|---|---|---|---|---|---|---|---|
| [A-webapp-warm](A-webapp-warm/report.md) | graded | webapp | done | 0.4s | 1.4s | 1.4s | 1.7s | 1596/1596/1596 | 3666811/1909589 | 71633/0 |
| [baseline](../baseline/A-websocket-warm/report.md) | graded | websocket | done | 0.2s | 10.6s | 16.5s | 16.8s | 1596/1596/1596 | 162829/118056 | 2294/698 |
| [baseline](../baseline/A-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.2s | 10.7s | 16.5s | 16.9s | 1596/1596/1596 | 261891/127164 | 2294/698 |
| [A-webapp-cold](A-webapp-cold/report.md) | graded | webapp | done | 0.4s | 1.1s | 1.9s | 2.0s | 1596/1596/1596 | 4309419/2366256 | 81869/0 |
| [baseline](../baseline/A-websocket-cold/report.md) | graded | websocket | done | 0.2s | 10.7s | 16.5s | 16.8s | 1596/1596/1596 | 3187181/323291 | 2294/698 |
| [baseline](../baseline/A-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 0.2s | 10.6s | 16.6s | 16.8s | 1596/1596/1596 | 3388422/333909 | 2294/698 |
| [B-webapp-warm](B-webapp-warm/report.md) | graded | webapp | done | 1.9s | 3.3s | 4.1s | 6.7s | 1596/1596/1596 | 658786/413864 | 63634/0 |
| [baseline](../baseline/B-websocket-warm/report.md) | graded | websocket | done | 0.7s | 11.3s | 17.3s | 16.8s | 1596/1596/1596 | 30787/122931 | 2294/698 |
| [baseline](../baseline/B-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.8s | 11.6s | 17.2s | 17.0s | 1596/1596/1596 | 159224/133002 | 2294/698 |
| [B-webapp-cold](B-webapp-cold/report.md) | graded | webapp | done | 7.5s | 8.0s | 3.6s | 8.0s | 1596/1596/1596 | 854725/478707 | 63793/0 |
| [baseline](../baseline/B-websocket-cold/report.md) | graded | websocket | done | 1.2s | 11.4s | 17.1s | 17.0s | 1596/1596/1596 | 3074356/345094 | 2294/698 |
| [baseline](../baseline/B-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 2.7s | 13.3s | 17.5s | 16.9s | 1596/1596/1596 | 3270340/381463 | 2294/698 |
| [C-webapp-warm](C-webapp-warm/report.md) | graded | webapp | done | 1.5s | 2.8s | 2.3s | 4.7s | 1596/1596/1596 | 680374/369687 | 58003/0 |
| [baseline](../baseline/C-websocket-warm/report.md) | graded | websocket | done | 1.0s | 11.6s | 17.1s | 17.3s | 1596/1596/1596 | 30710/98628 | 2294/698 |
| [baseline](../baseline/C-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.9s | 11.7s | 17.1s | 16.9s | 1596/1596/1596 | 125705/137802 | 2294/698 |
| [C-webapp-cold](C-webapp-cold/report.md) | graded | webapp | done | 7.5s | 9.7s | 3.1s | 9.7s | 1596/1596/1596 | 580117/318215 | 71082/0 |
| [baseline](../baseline/C-websocket-cold/report.md) | graded | websocket | done | 1.1s | 11.5s | 17.1s | 16.7s | 1596/1596/1596 | 3078485/392993 | 2294/698 |
| [baseline](../baseline/C-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 3.3s | 13.1s | 17.0s | 15.8s | 1596/1596/1596 | 3274680/408495 | 2294/698 |
| [D-webapp-warm](D-webapp-warm/report.md) | graded | webapp | done | 8.6s | 26.3s | 22.0s | 118.0s | 1596/1596/1596 | 303345/271104 | 17047/0 |
| [baseline](../baseline/D-websocket-warm/report.md) | graded | websocket | not done by deadline | 6.2s | 20.8s | 884.4s | 878.0s | 397/1077/1596 | 14459/40136 | 397/0 |
| [baseline](../baseline/D-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | 6.5s | never | 895.2s | 899.1s | 0/1077/1596 | 35880/180072 | 0/0 |
| [D-webapp-cold](D-webapp-cold/report.md) | graded | webapp | done | 290.7s | 322.6s | 36.8s | 322.6s | 1596/1596/1596 | 756136/417273 | 34860/0 |
| [baseline](../baseline/D-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 113119/28412 | 0/0 |
| [baseline](../baseline/D-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 98876/21710 | 0/0 |
| [E-webapp-warm](E-webapp-warm/report.md) | graded | webapp | not done by deadline | never | never | no text produced | 899.0s | 0/0/1596 | 74730/31127 | 320/0 |
| [baseline](../baseline/E-websocket-warm/report.md) | graded | websocket | not done by deadline | 5.2s | never | 895.3s | 899.3s | 0/1077/1596 | 3282/5479 | 0/0 |
| [baseline](../baseline/E-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | never | never | 891.3s | 899.4s | 0/1077/1596 | 3959/1578 | 0/0 |
| [E-webapp-cold](E-webapp-cold/report.md) | graded | webapp | not done by deadline | never | never | no text produced | 897.2s | 0/0/1596 | 4150/8109 | 0/0 |
| [baseline](../baseline/E-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2578/864 | 0/0 |
| [baseline](../baseline/E-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2574/767 | 0/0 |
| [E-iid-webapp-warm](E-iid-webapp-warm/report.md) | fault | webapp | not done by deadline | never | never | no text produced | 899.0s | 0/0/1596 | 103558/39342 | 937/0 |
| [baseline](../baseline/E-iid-websocket-warm/report.md) | fault | websocket | not done by deadline | never | never | 890.5s | 899.5s | 0/1077/1596 | 1630/2429 | 0/0 |
| [baseline](../baseline/E-iid-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | never | never | 886.8s | 899.5s | 0/1077/1596 | 2088/2757 | 0/0 |
| [F-webapp-warm](F-webapp-warm/report.md) | diagnostic | webapp | not done by deadline | never | never | no text produced | 898.5s | 0/0/1596 | 6904/8452 | 0/0 |
| [baseline](../baseline/F-websocket-warm/report.md) | diagnostic | websocket | not done by deadline | never | never | 861.8s | 899.5s | 0/1077/1596 | 3102/508 | 0/0 |
| [baseline](../baseline/F-websocket-nodeflate-warm/report.md) | diagnostic | websocket-nodeflate | not done by deadline | never | never | 800.0s | 899.5s | 0/1077/1596 | 2574/855 | 0/0 |
| [case70-webapp-warm](case70-webapp-warm/report.md) | graded | webapp | done | 3.9s | 12.7s | 39.0s | 101.5s | 1596/1596/1596 | 153461/73668 | 23872/0 |
| [baseline](../baseline/case70-websocket-warm/report.md) | graded | websocket | not done by deadline | 5.5s | never | 894.5s | 899.5s | 0/1077/1596 | 4322/6537 | 0/0 |
| [baseline](../baseline/case70-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | 9.2s | never | 894.0s | 899.4s | 0/1077/1596 | 5276/3360 | 0/0 |
| [case70-webapp-warm-s2](case70-webapp-warm-s2/report.md) | graded | webapp | done | 6.5s | 15.8s | 12.8s | 52.0s | 1596/1596/1596 | 116652/52573 | 19130/0 |
| [case70-webapp-warm-s3](case70-webapp-warm-s3/report.md) | graded | webapp | done | 48.4s | 52.7s | 25.2s | 52.7s | 1596/1596/1596 | 144440/87221 | 24566/0 |
| [case70-webapp-cold](case70-webapp-cold/report.md) | graded | webapp | not done by deadline | never | never | no text produced | 896.1s | 0/0/1596 | 121852/189396 | 0/0 |
| [baseline](../baseline/case70-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2066/946 | 0/0 |
| [baseline](../baseline/case70-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2000/1012 | 0/0 |
