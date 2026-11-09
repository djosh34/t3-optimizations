# Network stories: 50 seeds on the real stack, 100,000 in the simulator

Issue [#58](https://github.com/djosh34/t3-optimizations/issues/58). Each seed draws a few minutes of a 2G-like link (`crates/sync-core/src/sim/story.rs`; `lab/README.md`, "Story runs"). For each seed the untouched t3code v0.1.7 web client (`websocket`) and the t3opt web app (`webapp`) ran at the same time on the same story, each on its own shaped link, against one stock t3 v0.1.7 server. The deadline is 5 minutes after the story starts; not done by then is a failure.

CI run [42592837717](https://github.com/djosh34/t3-optimizations/actions/runs/42592837717) on 9c461bdc, a dispatch of CI with `stories` (`.github/workflows/ci.yml`), one GitHub Actions job per lab seed and 20 jobs for the simulator. The host load each lab run saw is in its `story.md`. Each seed's directory has `story.md` and `story.json` with both clients side by side, `story-input.json` (the story), and each client's full report. The raw logs and screenshots are in the run's `stories-report` artifact.

The commits after 9c461bdc change what the web app does only after a second connection in a row whose handshake stalled (from 9 minutes on), and on the WebSocket path, which no seed reached: a five-minute story runs the same on them.

## Lab: both web clients on each seed's story

50 seeds (8 survival stories), lab commit 9c461bdca554.

| Over all seeds | websocket | webapp |
|---|---|---|
| Reply completed by the deadline | 31/50 | 48/50 |
| Prompt confirmed within 10 s (case 70) | 36/50 | 38/50 |
| Newest text within 5 s of the server's release (case 50) | 10/50 | 12/50 |
| Max lag from the server's release, median / p95 / worst | 8.9 / 272.5 / 287.2 s (without any released text: 8) | 13.7 / 103.9 / 256.8 s |
| Longest freeze, median / p95 / worst | 49.8 / 298.8 / 299.0 s | 13.7 / 103.9 / 256.7 s |
| Hangs (freezes of 30 s or more) | 27 | 13 |
| Reloads | 0 | 0 |
| WebSockets opened after the switch (v0.1.7: reconnects; web app: probes) | 22 | 0 |
| Stayed on the data channel (UDP), never on the WebSocket | - | 50/50 |

Every hang (a freeze of 30 s or more, seconds after the prompt) and reload:

| Seed | Client | Hangs | Reloads |
|---|---|---|---|
| 2 | websocket | 71 s at 44 s, to the end | 0 |
| 3 | websocket | 299 s at 0 s, to the end | 0 |
| 3 | webapp | 62 s at 101 s | 0 |
| 6 | websocket | 268 s at 30 s, to the end | 0 |
| 9 | websocket | 171 s at 56 s | 0 |
| 9 | webapp | 104 s at 52 s, to the end | 0 |
| 10 | websocket | 299 s at 0 s, to the end | 0 |
| 10 | webapp | 122 s at 50 s | 0 |
| 12 | websocket | 118 s at 64 s, to the end | 0 |
| 14 | websocket | 268 s at 31 s, to the end | 0 |
| 16 | websocket | 299 s at 0 s, to the end | 0 |
| 17 | websocket | 36 s at 12 s; 163 s at 57 s, to the end | 0 |
| 17 | webapp | 34 s at 53 s | 0 |
| 18 | websocket | 299 s at 0 s, to the end | 0 |
| 19 | websocket | 172 s at 28 s | 0 |
| 21 | websocket | 298 s at 0 s, to the end | 0 |
| 21 | webapp | 257 s at 38 s | 0 |
| 23 | websocket | 268 s at 31 s, to the end | 0 |
| 26 | websocket | 265 s at 33 s, to the end | 0 |
| 26 | webapp | 82 s at 32 s | 0 |
| 28 | websocket | 50 s at 52 s, to the end | 0 |
| 29 | websocket | 299 s at 0 s, to the end | 0 |
| 31 | websocket | 165 s at 46 s, to the end | 0 |
| 32 | websocket | 281 s at 18 s, to the end | 0 |
| 33 | websocket | 299 s at 0 s, to the end | 0 |
| 33 | webapp | 146 s at 16 s | 0 |
| 35 | websocket | 298 s at 0 s, to the end | 0 |
| 35 | webapp | 90 s at 77 s | 0 |
| 36 | webapp | 33 s at 52 s, to the end | 0 |
| 38 | websocket | 264 s at 35 s, to the end | 0 |
| 38 | webapp | 48 s at 77 s | 0 |
| 41 | websocket | 299 s at 0 s, to the end | 0 |
| 41 | webapp | 34 s at 20 s | 0 |
| 42 | websocket | 299 s at 0 s, to the end | 0 |
| 42 | webapp | 40 s at 16 s | 0 |
| 44 | websocket | 281 s at 18 s, to the end | 0 |
| 48 | websocket | 299 s at 0 s, to the end | 0 |
| 49 | websocket | 299 s at 0 s, to the end | 0 |
| 49 | webapp | 79 s at 46 s | 0 |

Worst seeds, by the t3opt web app's longest freeze:

| Seed | Story | websocket status / freeze | webapp status / freeze |
|---|---|---|---|
| [21](s21/story.md) | survival | not done by deadline / 298 s | not done by deadline / 257 s |
| [33](s33/story.md) | survival | not done by deadline / 299 s | done / 146 s |
| [10](s10/story.md) | survival | not done by deadline / 299 s | done / 122 s |
| [9](s9/story.md) | 2G | done / 171 s | done / 104 s |
| [35](s35/story.md) | 2G | not done by deadline / 298 s | done / 90 s |
| [26](s26/story.md) | 2G | not done by deadline / 265 s | done / 82 s |
| [49](s49/story.md) | survival | not done by deadline / 299 s | done / 79 s |
| [3](s3/story.md) | survival | not done by deadline / 299 s | done / 62 s |
| [38](s38/story.md) | 2G | not done by deadline / 264 s | done / 48 s |
| [42](s42/story.md) | 2G | not done by deadline / 299 s | done / 40 s |

## Simulator: the sync protocol alone

The same stories on seeds 1 to 100,000, `crates/sync-core/examples/stories.rs`, through the model of WebKit's SCTP. Every seed must hold: nothing stored twice, no acked byte sent again, no command applied twice, no packet over 1,100 bytes, the prompt confirmed and the reply whole before the run ends.

100000 seeds, 20113 survival stories, **0 broken**.

| Stories | Prompt confirmed (s) | Reply whole (s) | Max text lag (s) |
|---|---|---|---|
| | median / p95 / p99 / max | median / p95 / p99 / max | median / p95 / p99 / max |
| 2G | 2.3 / 18.3 / 69.5 / 179.5 | 44.5 / 104.0 / 133.2 / 230.7 | 10.1 / 75.5 / 95.3 / 226.1 |
| survival | 7.6 / 31.5 / 85.1 / 211.8 | 74.2 / 147.7 / 174.6 / 239.6 | 52.4 / 125.8 / 156.8 / 230.0 |

## How the numbers moved

The same 50 seeds ran on each fix the runs found (the `websocket` column is the same client every time; it varies from run to run, as timing and the runners' load do):

| Run, commit | v0.1.7 done | web app done | web app freeze, median / p95 | web app on UDP |
|---|---|---|---|---|
| [42582280792](https://github.com/djosh34/t3-optimizations/actions/runs/42582280792), f5845268 | 37/50 | 49/50 | 14.2 / 128.0 s | 47/50 |
| [42585163766](https://github.com/djosh34/t3-optimizations/actions/runs/42585163766), 89ceda52 | 31/50 | 48/50 | 11.6 / 100.6 s | 49/50 |
| [42588613293](https://github.com/djosh34/t3-optimizations/actions/runs/42588613293), 824dc42f | 36/50 | 47/50 | 10.8 / 101.1 s | 50/50 |
| [42592837717](https://github.com/djosh34/t3-optimizations/actions/runs/42592837717), 9c461bdc | 31/50 | 48/50 | 13.7 / 103.9 s | 50/50 |

- f5845268 to 89ceda52: in survival stories a new WebRTC connection whose ICE took longer than 20 s was taken for blocked UDP, and the app moved to the WebSocket. A later connection now gets three minutes once WebRTC worked on the page.
- 89ceda52 to 824dc42f: a connection made during a 90 s gap never got ICE through after the link came back (WebKit stops sending unanswered checks); it is now replaced every minute.
- 824dc42f to 9c461bdc: in seed 49 a connection got ICE through but its handshake never finished, also after the link came back, and the app showed nothing until the deadline; a connection whose channel has not opened in three minutes is now replaced. Seed 49 finished on 9c461bdc.

## Open

- The newest text within 5 s of its release (case 50's target) is met on 12 of 50 seeds, the prompt within 10 s (case 70's) on 38: gaps of pure loss up to 90 s make both impossible on many seeds, for any client. They are reported, not loosened.
- On a sudden rate drop the web app's newest text waits behind its own backfill in the link's queue ([#58](https://github.com/djosh34/t3-optimizations/issues/58#issuecomment-6734879588)); not changed here, by the decision on #58.
- Seeds 18 and 21 (survival) are not done by the deadline for either client: their links come back at 223 s and 185 s.
