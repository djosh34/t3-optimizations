# Acceptance on the full chaos lab

The home-screen web app (`--client webapp`, issue #46) in WebKit on every profile of the lab, against the
WebSocket baseline in [../baseline](../baseline/README.md) on the same profiles, starts and seed. See
[lab/README.md](../../README.md) for the profiles and what each measure means.

1 run at a time, 2026-11-08T17:31Z to 2026-11-08T21:00Z, on 8 cores. Load average (1 min, sampled every minute): median 6.54, peak 20.26.

About the load: the idle load of this host is about 3. Before the soak (154 samples) the median was 6.27 and the peak 10.66, at 19:51 to 19:55 while the worker ran sync-core's tests during rate100 to duplicate. During the soak (56 samples) the median was 8.98: the soak's 125 kB reply keeps WebKit's software renderer busier than the other runs. Its peaks of 12 to 20 at 20:06 to 20:13 were a disk cleanup (`du` and `rm` over 15 GB of build output) by the worker; 18.86 at 20:22 has no known cause. The WebSocket baseline in [../baseline](../baseline/README.md) ran 3 at a time without load sampling; the first acceptance attempt, also 3 at a time, ran at a load of about 20, so the baseline most likely ran under a load like that.

The suite's script exited 1 though every run has its report and none failed: its load sampler's exit trap failed once the sampler was already stopped (fixed in the script).

Commit of these runs: fe6e6aee76ac.

## Targets

- Case 70 from a warm start, a typed prompt confirmed within 10 s: **not met**. case70-webapp-warm 12.1s, case70-webapp-warm-s2 6.6s, case70-webapp-warm-s3 9.7s, case70-webapp-warm-s4 5.2s, case70-webapp-warm-s5 2.4s, case70-webapp-warm-s6 10.0s, case70-webapp-warm-s7 9.4s, case70-webapp-warm-s8 6.4s.
- Case 70 from a cold start, a typed prompt confirmed within 10 s: **not met**. case70-webapp-cold never.
- Case 50 from a warm start, the newest text never more than 5 s behind: **met**. Max lag: case50-webapp-warm 4.1s, case50-webapp-warm-s2 3.7s, case50-webapp-warm-s3 4.2s, case50-webapp-warm-s4 2.8s, case50-webapp-warm-s5 3.5s, case50-webapp-warm-s6 3.8s, case50-webapp-warm-s7 3.1s, case50-webapp-warm-s8 1.9s.
- Case 50 from a cold start, the newest text never more than 5 s behind: **met**. Max lag: case50-webapp-cold 3.0s.
- Profiles A to E, text keeps moving and the prompt is confirmed before the deadline (the whole reply shown): **not met**. A-webapp-warm done, prompt 0.3s, A-webapp-cold done, prompt 0.3s, B-webapp-warm done, prompt 1.4s, B-webapp-cold done, prompt 10.0s, C-webapp-warm done, prompt 1.8s, C-webapp-cold done, prompt 8.7s, D-webapp-warm done, prompt 7.9s, D-webapp-cold done, prompt 168.3s, E-webapp-warm not done by deadline, prompt never, E-webapp-cold not done by deadline, prompt never.
- No content delivered twice to the application layer: **met**. Bytes sync-core handed to the web app twice, over all 46 runs: 0.
- After every outage, sync resumes with no user action (runs with outages or an address change, the whole reply shown): **not met**. B-webapp-warm done, B-webapp-cold done, D-webapp-warm done, D-webapp-cold done, E-webapp-warm not done by deadline, E-webapp-cold not done by deadline, E-iid-webapp-warm not done by deadline, outage10-webapp-warm done, outage60-webapp-warm done, flap-webapp-warm done, addrchange-webapp-warm done.
- The newest text and prompt acks overtake backfill in every run: **not met**. When the sidecar queued the prompt's reply and the reply's first newest text, it still had backfill (older messages, the thread list, other threads, images) it had never sent, which the priority order sends after them, and both then reached the phone, in 34 runs. This shows the send order; on a lossy link a later packet can still arrive first. Not established in 10 (the phone got no confirmation or no text, no line for a moment, or no backfill waiting then): D-webapp-warm, E-webapp-warm, E-webapp-cold, E-iid-webapp-warm, case70-webapp-cold, case50-webapp-warm-s8, loss30-webapp-warm, burst90-webapp-warm, rate100-webapp-warm, asym-webapp-warm. Not met means not established in every run, not a failure seen. Per run below.
- Open defect: the web app's composer ignored Enter, and the driver pressed it again, in 0 of 46 runs: No runs. A user who does not press again never gets the prompt sent. The prompt's times count from the first Enter.

Profile F and `derp` are reported with no target. Backfill bytes never sent when the sidecar queued the prompt's reply / the reply's first newest text:

- A-webapp-warm: 3527 / 6419
- A-webapp-cold: 10024 / 12334
- B-webapp-warm: 6192 / 9205
- B-webapp-cold: 886 / 3997
- C-webapp-warm: 4227 / 7238
- C-webapp-cold: 12505 / 14443
- D-webapp-warm: 0 / 3010
- D-webapp-cold: 1894 / 4961
- E-webapp-warm: 8373 / 10716
- E-webapp-cold: n/a / n/a
- E-iid-webapp-warm: 2272 / 5287
- F-webapp-warm: n/a / n/a
- case70-webapp-warm: 5733 / 8739
- case70-webapp-warm-s2: 6078 / 9093
- case70-webapp-warm-s3: 5759 / 8772
- case70-webapp-warm-s4: 6649 / 9662
- case70-webapp-warm-s5: 2352 / 5365
- case70-webapp-warm-s6: 1070 / 4083
- case70-webapp-warm-s7: 53 / 3066
- case70-webapp-warm-s8: 6897 / 9912
- case70-webapp-cold: n/a / n/a
- case50-webapp-warm: 13131 / 15025
- case50-webapp-warm-s2: 6643 / 9656
- case50-webapp-warm-s3: 6075 / 9088
- case50-webapp-warm-s4: 4832 / 7845
- case50-webapp-warm-s5: 7484 / 10083
- case50-webapp-warm-s6: 1601 / 4616
- case50-webapp-warm-s7: 7708 / 10045
- case50-webapp-warm-s8: 0 / 3013
- case50-webapp-cold: 10909 / 13731
- loss30-webapp-warm: 0 / 3010
- loss70-webapp-warm: 974 / 3984
- loss90-webapp-warm: 14037 / 15933
- burst90-webapp-warm: 0 / 3011
- rate100-webapp-warm: n/a / n/a
- rtt3000-webapp-warm: 11104 / 13271
- reorder-webapp-warm: 3641 / 6651
- duplicate-webapp-warm: 10843 / 12737
- corrupt-webapp-warm: 10884 / 13261
- asym-webapp-warm: 0 / 3008
- outage10-webapp-warm: 4554 / 7565
- outage60-webapp-warm: 5581 / 8593
- flap-webapp-warm: 15234 / 17139
- addrchange-webapp-warm: 7405 / 10047
- derp-webapp-warm: 0 / 3010
- soak-webapp-warm: 6013 / 8824

## Every run against the baseline

Each web app run, and below it the WebSocket baseline on the same profile, start and seed (`websocket`, `websocket-nodeflate`). Prompt is from Enter to the confirmation; text is from Enter to the first reply text on screen; lag is the newest-text lag; silence is the longest time with no new reply text.

| Run | Grade | Client | Status | Prompt | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Received/duplicates (B) |
|---|---|---|---|---|---|---|---|---|---|---|
| [A-webapp-warm](A-webapp-warm/report.md) | graded | webapp | done | 0.3s | 1.0s | 1.3s | 1.3s | 1596/1596/1596 | 2121026/1138125 | 62239/0 |
| [baseline](../baseline/A-websocket-warm/report.md) | graded | websocket | done | 0.2s | 10.6s | 16.5s | 16.8s | 1596/1596/1596 | 162829/118056 | 2294/698 |
| [baseline](../baseline/A-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.2s | 10.7s | 16.5s | 16.9s | 1596/1596/1596 | 261891/127164 | 2294/698 |
| [A-webapp-cold](A-webapp-cold/report.md) | graded | webapp | done | 0.3s | 1.0s | 1.4s | 1.5s | 1596/1596/1596 | 2136792/1209130 | 74402/0 |
| [baseline](../baseline/A-websocket-cold/report.md) | graded | websocket | done | 0.2s | 10.7s | 16.5s | 16.8s | 1596/1596/1596 | 3187181/323291 | 2294/698 |
| [baseline](../baseline/A-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 0.2s | 10.6s | 16.6s | 16.8s | 1596/1596/1596 | 3388422/333909 | 2294/698 |
| [B-webapp-warm](B-webapp-warm/report.md) | graded | webapp | done | 1.4s | 1.8s | 1.9s | 2.6s | 1596/1596/1596 | 553757/354595 | 57825/0 |
| [baseline](../baseline/B-websocket-warm/report.md) | graded | websocket | done | 0.7s | 11.3s | 17.3s | 16.8s | 1596/1596/1596 | 30787/122931 | 2294/698 |
| [baseline](../baseline/B-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.8s | 11.6s | 17.2s | 17.0s | 1596/1596/1596 | 159224/133002 | 2294/698 |
| [B-webapp-cold](B-webapp-cold/report.md) | graded | webapp | done | 10.0s | 10.5s | 2.3s | 10.5s | 1596/1596/1596 | 884049/445559 | 58491/0 |
| [baseline](../baseline/B-websocket-cold/report.md) | graded | websocket | done | 1.2s | 11.4s | 17.1s | 17.0s | 1596/1596/1596 | 3074356/345094 | 2294/698 |
| [baseline](../baseline/B-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 2.7s | 13.3s | 17.5s | 16.9s | 1596/1596/1596 | 3270340/381463 | 2294/698 |
| [C-webapp-warm](C-webapp-warm/report.md) | graded | webapp | done | 1.8s | 2.2s | 2.0s | 4.2s | 1596/1596/1596 | 621427/343622 | 56311/0 |
| [baseline](../baseline/C-websocket-warm/report.md) | graded | websocket | done | 1.0s | 11.6s | 17.1s | 17.3s | 1596/1596/1596 | 30710/98628 | 2294/698 |
| [baseline](../baseline/C-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | done | 0.9s | 11.7s | 17.1s | 16.9s | 1596/1596/1596 | 125705/137802 | 2294/698 |
| [C-webapp-cold](C-webapp-cold/report.md) | graded | webapp | done | 8.7s | 9.8s | 1.9s | 9.8s | 1596/1596/1596 | 629215/363440 | 66860/0 |
| [baseline](../baseline/C-websocket-cold/report.md) | graded | websocket | done | 1.1s | 11.5s | 17.1s | 16.7s | 1596/1596/1596 | 3078485/392993 | 2294/698 |
| [baseline](../baseline/C-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | done | 3.3s | 13.1s | 17.0s | 15.8s | 1596/1596/1596 | 3274680/408495 | 2294/698 |
| [D-webapp-warm](D-webapp-warm/report.md) | graded | webapp | done | 7.9s | 8.8s | 19.9s | 34.7s | 1596/1596/1596 | 154358/89691 | 9645/0 |
| [baseline](../baseline/D-websocket-warm/report.md) | graded | websocket | not done by deadline | 6.2s | 20.8s | 884.4s | 878.0s | 397/1077/1596 | 14459/40136 | 397/0 |
| [baseline](../baseline/D-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | 6.5s | never | 895.2s | 899.1s | 0/1077/1596 | 35880/180072 | 0/0 |
| [D-webapp-cold](D-webapp-cold/report.md) | graded | webapp | done | 168.3s | 168.0s | 37.2s | 168.0s | 1596/1596/1596 | 536923/271872 | 21401/0 |
| [baseline](../baseline/D-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 113119/28412 | 0/0 |
| [baseline](../baseline/D-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 98876/21710 | 0/0 |
| [E-webapp-warm](E-webapp-warm/report.md) | graded | webapp | not done by deadline | never | never | 838.6s | 899.1s | 0/1077/1596 | 79238/29469 | 265/0 |
| [baseline](../baseline/E-websocket-warm/report.md) | graded | websocket | not done by deadline | 5.2s | never | 895.3s | 899.3s | 0/1077/1596 | 3282/5479 | 0/0 |
| [baseline](../baseline/E-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | never | never | 891.3s | 899.4s | 0/1077/1596 | 3959/1578 | 0/0 |
| [E-webapp-cold](E-webapp-cold/report.md) | graded | webapp | not done by deadline | never | never | no text produced | 898.0s | 0/0/1596 | 4716/7445 | 0/0 |
| [baseline](../baseline/E-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2578/864 | 0/0 |
| [baseline](../baseline/E-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2574/767 | 0/0 |
| [E-iid-webapp-warm](E-iid-webapp-warm/report.md) | fault | webapp | not done by deadline | never | never | 886.3s | 899.4s | 0/1077/1596 | 103994/28158 | 75/0 |
| [baseline](../baseline/E-iid-websocket-warm/report.md) | fault | websocket | not done by deadline | never | never | 890.5s | 899.5s | 0/1077/1596 | 1630/2429 | 0/0 |
| [baseline](../baseline/E-iid-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | never | never | 886.8s | 899.5s | 0/1077/1596 | 2088/2757 | 0/0 |
| [F-webapp-warm](F-webapp-warm/report.md) | diagnostic | webapp | not done by deadline | never | never | no text produced | 899.6s | 0/0/1596 | 5323/7140 | 0/0 |
| [baseline](../baseline/F-websocket-warm/report.md) | diagnostic | websocket | not done by deadline | never | never | 861.8s | 899.5s | 0/1077/1596 | 3102/508 | 0/0 |
| [baseline](../baseline/F-websocket-nodeflate-warm/report.md) | diagnostic | websocket-nodeflate | not done by deadline | never | never | 800.0s | 899.5s | 0/1077/1596 | 2574/855 | 0/0 |
| [case70-webapp-warm](case70-webapp-warm/report.md) | graded | webapp | done | 12.1s | 26.0s | 22.6s | 26.0s | 1596/1596/1596 | 70997/39474 | 13012/0 |
| [baseline](../baseline/case70-websocket-warm/report.md) | graded | websocket | not done by deadline | 5.5s | never | 894.5s | 899.5s | 0/1077/1596 | 4322/6537 | 0/0 |
| [baseline](../baseline/case70-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | 9.2s | never | 894.0s | 899.4s | 0/1077/1596 | 5276/3360 | 0/0 |
| [case70-webapp-warm-s2](case70-webapp-warm-s2/report.md) | graded | webapp | done | 6.6s | 20.6s | 21.9s | 20.6s | 1596/1596/1596 | 65119/35805 | 10192/0 |
| [case70-webapp-warm-s3](case70-webapp-warm-s3/report.md) | graded | webapp | done | 9.7s | 10.6s | 27.8s | 20.6s | 1596/1596/1596 | 85167/45116 | 15588/0 |
| [case70-webapp-warm-s4](case70-webapp-warm-s4/report.md) | graded | webapp | done | 5.2s | 11.1s | 12.3s | 11.2s | 1596/1596/1596 | 68746/29196 | 13832/0 |
| [case70-webapp-warm-s5](case70-webapp-warm-s5/report.md) | graded | webapp | done | 2.4s | 8.8s | 47.0s | 39.4s | 1596/1596/1596 | 116252/44568 | 17447/0 |
| [case70-webapp-warm-s6](case70-webapp-warm-s6/report.md) | graded | webapp | done | 10.0s | 14.1s | 25.8s | 14.5s | 1596/1596/1596 | 81773/33082 | 15995/0 |
| [case70-webapp-warm-s7](case70-webapp-warm-s7/report.md) | graded | webapp | done | 9.4s | 9.8s | 11.7s | 9.8s | 1596/1596/1596 | 71463/26070 | 14259/0 |
| [case70-webapp-warm-s8](case70-webapp-warm-s8/report.md) | graded | webapp | done | 6.4s | 8.1s | 22.7s | 17.1s | 1596/1596/1596 | 68571/47075 | 13076/0 |
| [case70-webapp-cold](case70-webapp-cold/report.md) | graded | webapp | not done by deadline | never | never | no text produced | 897.9s | 0/0/1596 | 851247/390464 | 0/0 |
| [baseline](../baseline/case70-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2066/946 | 0/0 |
| [baseline](../baseline/case70-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 2000/1012 | 0/0 |
| [case50-webapp-warm](case50-webapp-warm/report.md) | graded | webapp | done | 1.0s | 2.9s | 4.1s | 4.3s | 1596/1596/1596 | 231852/81514 | 55670/0 |
| [baseline](../baseline/case50-websocket-warm/report.md) | graded | websocket | not done by deadline | 16.0s | 36.2s | 836.6s | 847.2s | 1077/1596/1596 | 49362/282281 | 1474/397 |
| [baseline](../baseline/case50-websocket-nodeflate-warm/report.md) | graded | websocket-nodeflate | not done by deadline | 2.7s | never | 896.4s | 899.2s | 0/1077/1596 | 43407/143216 | 0/0 |
| [case50-webapp-warm-s2](case50-webapp-warm-s2/report.md) | graded | webapp | done | 1.4s | 2.9s | 3.7s | 4.3s | 1596/1596/1596 | 294239/109814 | 53648/0 |
| [case50-webapp-warm-s3](case50-webapp-warm-s3/report.md) | graded | webapp | done | 1.0s | 1.6s | 4.2s | 3.1s | 1596/1596/1596 | 357145/120890 | 55278/0 |
| [case50-webapp-warm-s4](case50-webapp-warm-s4/report.md) | graded | webapp | done | 1.0s | 3.3s | 2.8s | 3.3s | 1596/1596/1596 | 324260/115473 | 48190/0 |
| [case50-webapp-warm-s5](case50-webapp-warm-s5/report.md) | graded | webapp | done | 2.7s | 4.2s | 3.5s | 4.9s | 1596/1596/1596 | 400277/136305 | 55960/0 |
| [case50-webapp-warm-s6](case50-webapp-warm-s6/report.md) | graded | webapp | done | 2.0s | 2.2s | 3.8s | 3.4s | 1596/1596/1596 | 335497/117569 | 58078/0 |
| [case50-webapp-warm-s7](case50-webapp-warm-s7/report.md) | graded | webapp | done | 1.0s | 1.6s | 3.1s | 4.1s | 1596/1596/1596 | 273200/102133 | 51447/0 |
| [case50-webapp-warm-s8](case50-webapp-warm-s8/report.md) | graded | webapp | done | 0.9s | 1.4s | 1.9s | 2.9s | 1596/1596/1596 | 394353/167880 | 43151/0 |
| [case50-webapp-cold](case50-webapp-cold/report.md) | graded | webapp | done | 23.2s | 24.3s | 3.0s | 24.3s | 1596/1596/1596 | 291494/118918 | 62731/0 |
| [baseline](../baseline/case50-websocket-cold/report.md) | graded | websocket | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 34720/2566 | 0/0 |
| [baseline](../baseline/case50-websocket-nodeflate-cold/report.md) | graded | websocket-nodeflate | app never loaded by deadline | never | never | no text produced | n/a, no prompt sent | 0/0/1596 | 50635/3554 | 0/0 |
| [loss30-webapp-warm](loss30-webapp-warm/report.md) | fault | webapp | done | 1.0s | 3.0s | 2.6s | 4.4s | 1596/1596/1596 | 418786/255223 | 47542/0 |
| [baseline](../baseline/loss30-websocket-warm/report.md) | fault | websocket | done | 1.4s | 14.1s | 25.8s | 26.6s | 1596/1596/1596 | 23749/88561 | 2294/698 |
| [baseline](../baseline/loss30-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 1.6s | 14.1s | 37.4s | 60.9s | 1596/1596/1596 | 139481/142997 | 2294/698 |
| [loss70-webapp-warm](loss70-webapp-warm/report.md) | fault | webapp | done | 4.3s | 5.8s | 7.6s | 7.8s | 1596/1596/1596 | 289082/62825 | 39891/0 |
| [baseline](../baseline/loss70-websocket-warm/report.md) | fault | websocket | not done by deadline | 4.6s | never | 894.6s | 899.4s | 0/1077/1596 | 2303/4166 | 0/0 |
| [baseline](../baseline/loss70-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | 5.1s | never | 894.6s | 899.4s | 0/1077/1596 | 3922/4850 | 0/0 |
| [loss90-webapp-warm](loss90-webapp-warm/report.md) | fault | webapp | done | 24.9s | 24.9s | 51.5s | 49.6s | 1596/1596/1596 | 44078/26301 | 4307/0 |
| [baseline](../baseline/loss90-websocket-warm/report.md) | fault | websocket | not done by deadline | never | never | no text produced | 899.3s | 0/0/1596 | 362/1000 | 0/0 |
| [baseline](../baseline/loss90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | never | never | no text produced | 899.5s | 0/0/1596 | 214/277 | 0/0 |
| [burst90-webapp-warm](burst90-webapp-warm/report.md) | fault | webapp | done | 39.5s | 215.6s | 196.9s | 215.6s | 1596/1596/1596 | 83994/53053 | 8511/0 |
| [baseline](../baseline/burst90-websocket-warm/report.md) | fault | websocket | not done by deadline | 0.5s | never | 898.4s | 899.5s | 0/1077/1596 | 4190/408 | 0/0 |
| [baseline](../baseline/burst90-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | 0.6s | never | 898.4s | 899.5s | 0/1077/1596 | 8304/933 | 0/0 |
| [rate100-webapp-warm](rate100-webapp-warm/report.md) | fault | webapp | not done by deadline | never | never | no text produced | 899.4s | 0/0/1596 | 90375/72209 | 1065/0 |
| [baseline](../baseline/rate100-websocket-warm/report.md) | fault | websocket | not done by deadline | 4.4s | never | 895.6s | 899.3s | 0/1077/1596 | 31330/71706 | 0/0 |
| [baseline](../baseline/rate100-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | 10.0s | never | 889.6s | 899.0s | 0/1077/1596 | 26731/67643 | 0/0 |
| [rtt3000-webapp-warm](rtt3000-webapp-warm/report.md) | fault | webapp | done | 3.9s | 4.5s | 10.7s | 13.2s | 1596/1596/1596 | 191004/171200 | 16616/0 |
| [baseline](../baseline/rtt3000-websocket-warm/report.md) | fault | websocket | done | 3.1s | 15.0s | 32.0s | 36.1s | 1596/1596/1596 | 67615/79855 | 2294/698 |
| [baseline](../baseline/rtt3000-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 2.4s | 14.1s | 18.3s | 18.1s | 1596/1596/1596 | 136381/79519 | 2294/698 |
| [reorder-webapp-warm](reorder-webapp-warm/report.md) | fault | webapp | done | 1.0s | 1.5s | 1.5s | 1.7s | 1596/1596/1596 | 486405/367016 | 59680/0 |
| [baseline](../baseline/reorder-websocket-warm/report.md) | fault | websocket | done | 0.5s | 10.9s | 16.6s | 16.8s | 1596/1596/1596 | 35699/126589 | 2294/698 |
| [baseline](../baseline/reorder-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 2.7s | 11.2s | 17.5s | 17.6s | 1596/1596/1596 | 126784/136009 | 2294/698 |
| [duplicate-webapp-warm](duplicate-webapp-warm/report.md) | fault | webapp | done | 1.0s | 1.7s | 3.3s | 3.6s | 1596/1596/1596 | 513569/440554 | 50666/0 |
| [baseline](../baseline/duplicate-websocket-warm/report.md) | fault | websocket | done | 0.8s | 11.3s | 17.3s | 16.9s | 1596/1596/1596 | 43938/156675 | 2294/698 |
| [baseline](../baseline/duplicate-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.7s | 11.6s | 17.3s | 16.9s | 1596/1596/1596 | 153224/160887 | 2294/698 |
| [corrupt-webapp-warm](corrupt-webapp-warm/report.md) | fault | webapp | done | 1.4s | 2.4s | 3.2s | 3.2s | 1596/1596/1596 | 449137/388848 | 52427/0 |
| [baseline](../baseline/corrupt-websocket-warm/report.md) | fault | websocket | done | 1.0s | 11.2s | 17.3s | 16.9s | 1596/1596/1596 | 36182/134327 | 2294/698 |
| [baseline](../baseline/corrupt-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.7s | 13.1s | 18.2s | 16.3s | 1596/1596/1596 | 128036/150655 | 2294/698 |
| [asym-webapp-warm](asym-webapp-warm/report.md) | fault | webapp | done | 4.3s | 4.8s | 54.8s | 67.3s | 1596/1596/1596 | 555278/24675 | 22425/0 |
| [baseline](../baseline/asym-websocket-warm/report.md) | fault | websocket | not done by deadline | 6.6s | never | 892.3s | 899.2s | 0/1077/1596 | 33695/108418 | 0/0 |
| [baseline](../baseline/asym-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | 8.6s | never | 890.3s | 899.2s | 0/1077/1596 | 69885/124289 | 0/0 |
| [outage10-webapp-warm](outage10-webapp-warm/report.md) | fault | webapp | done | 0.6s | 1.2s | 12.0s | 12.1s | 1596/1596/1596 | 507604/382653 | 58295/0 |
| [baseline](../baseline/outage10-websocket-warm/report.md) | fault | websocket | done | 0.4s | 25.0s | 24.5s | 25.0s | 1596/1596/1596 | 41656/128802 | 2294/698 |
| [baseline](../baseline/outage10-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.5s | 23.4s | 22.4s | 23.4s | 1596/1596/1596 | 122379/52616 | 2294/698 |
| [outage60-webapp-warm](outage60-webapp-warm/report.md) | fault | webapp | done | 0.8s | 1.2s | 67.0s | 67.3s | 1596/1596/1596 | 374953/387843 | 57215/0 |
| [baseline](../baseline/outage60-websocket-warm/report.md) | fault | websocket | done | 0.4s | 87.6s | 86.5s | 87.6s | 1596/1596/1596 | 74695/291969 | 2294/698 |
| [baseline](../baseline/outage60-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.5s | 132.4s | 131.4s | 132.4s | 1596/1596/1596 | 216251/336528 | 2294/698 |
| [flap-webapp-warm](flap-webapp-warm/report.md) | fault | webapp | done | 0.6s | 1.2s | 8.0s | 7.3s | 1596/1596/1596 | 313669/166619 | 53484/0 |
| [baseline](../baseline/flap-websocket-warm/report.md) | fault | websocket | done | 0.5s | 14.2s | 18.4s | 15.4s | 1596/1596/1596 | 39655/98474 | 2294/698 |
| [baseline](../baseline/flap-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.5s | 17.5s | 21.4s | 17.5s | 1596/1596/1596 | 174199/148963 | 2294/698 |
| [addrchange-webapp-warm](addrchange-webapp-warm/report.md) | fault | webapp | done | 0.6s | 1.4s | 1.9s | 1.9s | 1596/1596/1596 | 890201/777127 | 53602/0 |
| [baseline](../baseline/addrchange-websocket-warm/report.md) | fault | websocket | not done by deadline | 0.4s | 10.8s | 888.5s | 888.5s | 397/1077/1596 | 37148/41143 | 397/0 |
| [baseline](../baseline/addrchange-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 0.5s | 11.2s | 182.4s | 182.3s | 1596/1596/1596 | 275989/368456 | 2294/698 |
| [derp-webapp-warm](derp-webapp-warm/report.md) | fault | webapp | done | 1.0s | 1.2s | 3.4s | 1.6s | 1596/1596/1596 | 419764/194590 | 40663/0 |
| [baseline](../baseline/derp-websocket-warm/report.md) | fault | websocket | done | 0.8s | 10.9s | 16.5s | 16.8s | 1596/1596/1596 | 54587/139637 | 2294/698 |
| [baseline](../baseline/derp-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | done | 3.1s | 13.5s | 18.0s | 16.5s | 1596/1596/1596 | 162581/172103 | 2294/698 |
| [soak-webapp-warm](soak-webapp-warm/report.md) | fault | webapp | done | 1.0s | 2.5s | 15.8s | 7.1s | 125238/125238/125238 | 36860733/15360673 | 833433/0 |
| [baseline](../baseline/soak-websocket-warm/report.md) | fault | websocket | not done by deadline | 15.8s | 29.2s | 16.5s | 3556.3s | 1077/1077/125238 | 69389/313194 | 1474/397 |
| [baseline](../baseline/soak-websocket-nodeflate-warm/report.md) | fault | websocket-nodeflate | not done by deadline | 15.4s | never | 3583.6s | 3599.3s | 0/1077/125238 | 51193/168979 | 0/0 |
