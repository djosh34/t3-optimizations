# Capacity fix attempts (dropped)

The manager's decision on #46 after the full suite: case 70 warm missed the 10 s prompt target on seed 1 (12.1 s), with the sidecar's rate at 5.3 kB/s on the 1.25 kB/s link after the switch, because the capacity estimate was still the clean link's. Make one fix for the stale estimate, check it with the 30 simulator seeds of every #43 set, case 70 warm and case 50 warm on 8 seeds each in WebKit and D warm, keep it only if it helps with no regression, within about 3 hours.

Both versions below were dropped: `crates/sync-core/src/rate.rs` is back to fe6e6aee, the commit of the full suite in [../acceptance](../acceptance/README.md). The runs here are kept as measured; one run at a time, as the suite.

## c808de44: drop the capacity at a silence and hold slow start until the new link is measured

- Simulator, 30 seeds: case 70 prompt at most 3.2 s (was 4.2 s), case 50 worst lag **7.4 s, 2 seeds over 5 s** (was 4.6 s, none), WebKit models as before (case 70 prompt 7.1 s, case 50 lag 4.5 s).
- Case 70 warm, prompt confirmed, seeds 1 to 8: 14.0, 6.5, 2.5, 5.6, 7.1, 4.4, 12.4, 3.7 s, two over 10 s (the suite: one).
- Case 50 warm, seeds 1 and 2: max lag 3.0 and 3.9 s. The other seeds and D were not run: the simulator regression already ruled this version out.
- Load average: median 6.01, peak 8.11 (16 samples).

## c8fd4bb0: cap the capacity at the drained rate

- Simulator, 30 seeds: as before the change (case 70 prompt at most 4.2 s, WebKit model 7.1 s; case 50 lag at most 4.6 s, WebKit model 4.5 s).
- Case 70 warm, prompt confirmed, seeds 1 to 8: 4.8, 5.4, 6.0, 12.4, 1.6, 8.0, 4.8, 3.2 s, one over 10 s (the suite: 12.1, 6.6, 9.7, 5.2, 2.4, 10.0, 9.4, 6.4 s).
- Case 50 warm, max lag, seeds 1 to 8: 3.1, 2.6, 2.4, 4.0, 2.7, 2.1, **5.9**, 3.2 s; seed 7 is over the 5 s target the suite met on all eight (4.1, 3.7, 4.2, 2.8, 3.5, 3.8, 3.1, 1.9 s).
- D warm: done, prompt 8.5 s, max lag 16.1 s, approval shown after 15.2 s (the suite: 7.9 s, 19.9 s, 15.3 s).
- Load average: median 5.86, peak 8.77 (25 samples).

Case 70 warm stays not met with the build of the suite: seven of eight seeds within 10 s (seed 6 at 10.0 s), seed 1 at 12.1 s.
