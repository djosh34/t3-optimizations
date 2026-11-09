# Chaos lab

The lab runs a t3 client over a shaped, faulty link and measures what the phone sees. The first client is the untouched t3code v0.1.7 web client over WebSocket, the baseline every later outcome is compared with. Its reports are in [reports/baseline](reports/baseline/README.md).

## Run it

Once, as yourself:

```sh
make setup                              # from the repo root: t3code dependencies
lab/scripts/build-baseline-client.sh    # the v0.1.7 web client, into ~/.cache/t3lab
cd lab && go build -o bin/lab ./cmd/lab
```

Then, from `lab/`:

```sh
sudo ./bin/lab run --profile E --seed 7 --client websocket
sudo ./bin/lab run --profile case70 --seed 7 --client websocket-nodeflate --kind cold
./bin/lab profiles                      # every profile
sudo ./bin/lab cleanup                  # remove leftovers of labs that died
scripts/run-baseline.sh 3               # every baseline run, 3 at a time, into reports/baseline
```

`run` flags:

- `--client`: `websocket` (the web client as Safari runs it, with WebSocket compression) or `websocket-nodeflate` (no compression, like the unchanged iOS app, which never negotiates it; issue #7). `sidecar-native` is the sync protocol through the sidecar on the data channel, with a native phone in the web app's place; `webapp` is the home-screen web app in WebKit through the sidecar; see [Sidecar runs](#sidecar-runs). The pipe clients `pipe`, `pipe-unpatched` and `udp` run ticks instead of the t3 web client; see [Pipe runs](#pipe-runs).
- `--kind`: `warm` (default) starts connected on a clean link and then switches to the profile. `cold` sets everything up on the clean link, closes the page, switches to the profile, and then opens the app again.
- `--seed`: seeds netem. The same seed gives the same fault sequence. Timings still differ from run to run, because the browser and the server do not run in lockstep.
- `--deadline`: overrides the run's deadline (15 minutes, 60 for `soak`). A run that is not done by then reports "not done by deadline" instead of waiting.
- `--out`, `--name`: where the run directory goes (default `reports/runs/<time>-<profile>-<client>-<kind>-s<seed>`, ignored by git).
- `--node`: the node binary for the t3 server. The default is `node` on root's `PATH`.

The lab needs root for `ip`, `tc` and docker, and does nothing else as root: the t3 server, the HTTPS front, the tunnel, the browser and every file it writes run as, or belong to, the user who ran `sudo`.

## What a run does

```
 phone container (docker, --network none)          server namespace t3lab<id>srv
 ┌──────────────────────────────────┐             ┌───────────────────────────────────┐
 │ WebKit, driven by driver.mjs     │   veth      │ lab front: HTTPS :8443            │
 │ via `docker exec`                ├─netem──────►│   └─► stock t3 server :3780       │
 │ 10.231.0.2                       │◄──────netem─┤        └─► lab fakeagent (ACP)    │
 └──────────────────────────────────┘ 10.231.0.1  └───────────────────────────────────┘
```

1. Cleanup removes what earlier labs left behind (anything named `t3lab*` whose lab process is gone). Running labs are left alone, so several runs can share the host.
2. The lab creates the server namespace and a veth pair with GSO off, so netem sees wire-sized packets. Both ends have fixed MAC addresses and permanent neighbor entries for each other: with ARP over the shaped link, a neighbor entry fails now and then at heavy loss and the kernel drops what it queued for it, which a real phone link does not do. (The WebSocket baseline of issue #42 was measured before this fix.) It starts the phone container with no network, moves one veth end into the container's namespace and names it `t3lab<id>phone`. Playwright talks to WebKit over `docker exec`, so its control traffic never crosses the shaped link.
3. In the server namespace it starts the stock v0.1.7 t3 server from `t3code/` with `--base-dir` set to a fresh temp dir (deleted after the run), a home dir of its own and `PATH` without the host's provider CLIs. Its settings turn Codex and Claude off and add the fake agent as provider `lab`, so no prompt can reach a real model. The server serves no web client: the lab front serves the baseline client build itself (the build's files, and `index.html` for page loads, which the client routes itself) and forwards every other request, API calls and the WebSocket, to the server. It compresses and caches the build as t3 does, so the bytes on the link stay what they were when t3 served it. The front terminates TLS with a fresh self-signed certificate, like `tailscale serve` in production; the page is a secure context, so service workers work.
4. The t3 CLI adds the project, starts the "Lab thread" with a short warm-up turn and mints a pairing URL. The phone pairs, opens the thread, and sends one turn with an image attached. It is ready once that turn has ended on the phone and the composer would start a new turn (its send button reads "Submit message"): a prompt typed earlier is steered into the running turn or queued.
5. The lab applies the profile with `tc netem` on both veth ends (server end: down, phone end: up), then tells the phone to type the measured prompt.
6. The fake agent streams the reply. Outages, flapping and the address change happen on the profile's schedule. The phone approves the approval request when it shows.
7. The run ends when the phone shows the whole reply, or at the deadline. The lab deletes the container and the namespaces, also on Ctrl-C and on errors.

### The fake agent

`lab fakeagent` is an ACP agent over stdio that the t3 server starts like any local agent. For the measured prompt (64 bytes) it streams a fixed reply of 2,040 characters at 50 characters per second, 5 characters every 100 ms:

- two long paragraphs, so the server's paragraph buffering shows,
- an approval request after them; the reply stops until the phone answers,
- an image (a 48x48 PNG); t3 v0.1.7 shows images from agents as the text `[ACP image (image/png)]`, so the image the thread really holds is the one the phone attached in setup,
- a paragraph, then a code block that is never closed.

It logs when it produced each character (`agent.jsonl`). All lags are measured from that moment, not from when the server published the text.

## The two graded cases

The plan's pass numbers apply to these two cases ([issue #18](https://github.com/djosh34/t3-optimizations/issues/18)):

| | case 70 | case 50 |
|---|---|---|
| Rate, each direction | 10 kbit/s | 100 kbit/s |
| Loss, each direction | 70% independent random | 50% independent random |
| One-way delay | 300 ms, normal jitter ±50 ms (RTT about 600 ms) | same |
| Queue | 20 packets | 50 packets |
| Start | warm | warm |
| Prompt | 64 bytes, typed once the profile is applied | same |
| Reply | the fake agent's reply above, 50 characters/s | same |
| Deadline | 15 minutes | 15 minutes |
| Pass | prompt confirmed within 10 s | newest text never more than 5 s behind |

## Profiles

Rates are per direction (down / up when they differ). Delay is one-way with normal jitter. A to F come from [issue #15](https://github.com/djosh34/t3-optimizations/issues/15); the rest cover every fault type of issue #18. Outages silence both directions by dropping packets at each end's ingress, so packets in flight are lost too.

| Profile | Grade | Rate | Delay | Loss | Queue | Other faults |
|---|---|---|---|---|---|---|
| A | graded | 10 Mbit/s / 1 Mbit/s | 35 ± 10 ms | 0.15% | 128 | |
| B | graded | 512 / 128 kbit/s | 500 ± 100 ms | bursty, 4.8% | 64 | down 2 s every 120 s |
| C | graded | 200 / 114 kbit/s | 450 ± 50 ms | bursty, 2% | 32 | |
| D | graded | 64 / 32 kbit/s | 4000 ± 500 ms | bursty, 16.7% | 64 | down 6 s every 60 s |
| E | graded | 1 kbit/s | 2000 ± 500 ms | bursty, 75% | 2 | down 10 s every 60 s |
| E-iid | fault | 1 kbit/s | 2000 ± 500 ms | 75% | 2 | down 10 s every 60 s |
| F | diagnostic | 64 bit/s | 2000 ± 500 ms | bursty, 75% | 2 | down 20 s every 30 s |
| case70 | graded | 10 kbit/s | 300 ± 50 ms | 70% | 20 | |
| case50 | graded | 100 kbit/s | 300 ± 50 ms | 50% | 50 | |
| loss30 | fault | 100 kbit/s | 300 ± 50 ms | 30% | 50 | |
| loss70 | fault | 100 kbit/s | 300 ± 50 ms | 70% | 50 | |
| loss90 | fault | 100 kbit/s | 300 ± 50 ms | 90% | 50 | |
| burst90 | fault | 100 kbit/s | 300 ± 50 ms | bursty, 90%, runs of 20 | 50 | |
| rate100 | fault | 800 bit/s (100 B/s) | 300 ± 50 ms | none | 4 | |
| rtt3000 | fault | 200 kbit/s | 1500 ± 300 ms | 1% | 64 | |
| reorder | fault | 100 kbit/s | 300 ± 50 ms | 10% | 50 | 25% reordered |
| duplicate | fault | 100 kbit/s | 300 ± 50 ms | 5% | 50 | 20% duplicated |
| corrupt | fault | 100 kbit/s | 300 ± 50 ms | none | 50 | 10% corrupted |
| asym | fault | 200 kbit/s / 2 kbit/s | 150 ± 20 / 400 ± 100 ms | 1% / 50% | 32 / 8 | |
| outage10 | fault | 200 kbit/s | 150 ± 20 ms | 1% | 32 | one 10 s outage at 10 s |
| outage60 | fault | 200 kbit/s | 150 ± 20 ms | 1% | 32 | one 60 s outage at 10 s |
| flap | fault | 200 kbit/s | 150 ± 20 ms | 1% | 32 | down 5 s every 15 s |
| addrchange | fault | 200 kbit/s | 150 ± 20 ms | 1% | 32 | the phone's address changes at 15 s |
| derp | fault | 200 kbit/s | 150 ± 20 ms | 10% | 32 | all traffic in one TCP connection |
| soak | fault | 100 kbit/s | 300 ± 50 ms | 50% | 50 | a 55-minute reply, 60-minute deadline |

"Bursty" is netem's Gilbert-Elliott model; the percentage is the mean loss. Profile F cannot carry even one connection check in its 10 s up windows, so it is diagnostic: always run and reported, never graded. `derp` carries the phone's packets through a TUN device and one TCP connection across the shaped link (MTU 1280), like a Tailscale DERP relay; loss on the link then stalls everything in the tunnel.

## The report

Each run writes `report.json` and `report.md`, plus the raw `phone.jsonl` (the driver's events) and `agent.jsonl` (the agent's production log), the server, front and driver logs, and the phone's screenshots in `phone/`.

- **Status**: `done` (the phone shows the whole reply), `not done by deadline`, or for cold runs `app never loaded by deadline` / `prompt never sent by deadline`.
- **Prompt confirmation**: from pressing Enter to the server's success reply for the command that carries the prompt reaching the page.
- **Time to first text**: from pressing Enter to the first reply text on screen.
- **Prompt reached the agent**: from pressing Enter to the fake agent receiving the prompt.
- **Newest-text lag**: every second, how old the oldest text is that the agent produced and the phone does not show yet; 0 when the phone shows everything produced. Reported as a series and as max, median and p95. When the agent produced no text, because the prompt never reached it, there is no lag to report and the report says "no text produced".
- **Longest silence**: the longest time after pressing Enter in which the phone showed no new reply text, up to the end of the reply or of the run. A phone that showed nothing has a silence as long as the run. A cold run that never loaded the app sent no prompt, so it reports "n/a, no prompt sent".
- **Bytes on the wire**: what crossed the server's end of the link after the switch, down and up, with TCP, TLS and every other request the page makes (the web client also posts its traces to the server).
- **Bytes the app received / sent**: WebSocket payloads after decompression, from the switch to the end.
- **Reply received / duplicates**: how much reply text the application received over WebSocket, and how much of it it had already received. t3 sends each assistant message with its whole text so far, so the lab counts the text of assistant messages created after the prompt was sent. Because every update repeats the text before it, duplicates are above 0 even on a clean link: on profile A the app received 2,294 characters of reply text for the 1,596-character reply, 698 of them duplicates.

The phone reads the reply from the page's text. It matches pieces of at least 8 letters and digits against the reply, ignoring whitespace and punctuation, and counts what it has seen once even after the row scrolls away.

The lab's TCP is the kernel's, over a veth pair. In production the phone's TCP ends in tailscaled's userspace stack (issue #7), so production numbers differ. Every report says so.

## Pipe runs

A pipe run tests the transport alone, before any protocol is built on it (issue #49): each end sends a 25-byte tick every 100 ms, and the report shows how the ticks crossed the link. Its reports are in [reports/pipe](reports/pipe/README.md).

```sh
lab/scripts/build-pipe-lab.sh           # once, as yourself: lab/bin/pipe-lab and pipe-lab-unpatched
sudo ./bin/lab run --profile loss70 --seed 1 --client pipe
scripts/run-pipe.sh 3                   # every pipe run, 3 at a time, into reports/pipe
./bin/lab pipe-summary reports/pipe     # the table of reports/pipe/README.md
```

- `pipe`: a WebRTC data channel from WebKit (unordered, `maxRetransmits: 0`) to `pipe-lab webrtc` (`crates/pipe`), the server side on str0m with the patched SCTP layer (`crates/vendor/README.md`). The page loads from the lab front, which also passes the SDP offer to `pipe-lab` over HTTP; the data channel runs on UDP port 12444.
- `pipe-unpatched`: the same with sctp-proto as published on crates.io.
- `udp`: plain UDP between node in the phone container and `pipe-lab udp`: the best any transport can do on the link.

The run starts the pipe on the clean link, lets ticks flow for 3 s, switches to the profile and measures for 2 minutes (`--deadline` changes that). Pipe runs are warm only. The report gives for each direction the ticks sent and received after the switch, the longest gap between two arrivals (counted from the switch to the end of the run), the gaps over 2 s, and the age of the ticks when they arrived (arrival minus send time; both ends share the host's clock). Raw logs: `phone.jsonl` (driver) and `server.jsonl` (`pipe-lab`).

## Sidecar runs

`--client sidecar-native` runs the same scenario as the WebSocket client through the sidecar (issue #44): the stock t3 server and `crates/sidecar` in the server namespace, and `crates/native-phone` in the phone container. The native phone does what the web app will do (sync-core's phone over a WebRTC data channel, the sidecar's derived ICE credentials, storing before it acks), on str0m instead of WebKit. It pairs with a code from the t3 server, opens the lab thread, sends the measured prompt, approves the approval, and prints the same events as `driver.mjs`, so the report is the same. It skips the setup turn with an attached image: uploads do not go through the sidecar yet.

```sh
lab/scripts/build-sidecar.sh            # once, as yourself: lab/bin/sidecar and lab/bin/native-phone
sudo ./bin/lab run --profile case70 --seed 1 --client sidecar-native
```

The sidecar listens on 10.231.0.1:8480 (HTTP) and UDP 12444 in the server namespace; its log is `sidecar.log` in the run directory. The results of the graded cases are in [reports/sidecar](reports/sidecar/README.md). Profiles that carry the link in a TCP tunnel (`derp`) are not supported for the native phone yet; the web app runs them.

## Web app runs

`--client webapp` runs the same scenario with the home-screen web app ([issue #45](https://github.com/djosh34/t3-optimizations/issues/45), `t3code/apps/web/src/t3opt`) in WebKit: the sidecar serves the app's build, the lab front gives it HTTPS (the app needs a secure context), and the app talks to the sidecar on the data channel. The driver is `driver.mjs` as for the WebSocket client; it reads the prompt's state from the app's sync shim, and the byte counts from its sync engine.

```sh
lab/scripts/build-sidecar.sh                        # once, as yourself
lab/scripts/build-webapp.sh                         # lab/bin/webapp
sudo ./bin/lab run --profile case70 --seed 1 --client webapp
```

What differs from the WebSocket client's runs:

- On `derp` the sidecar's UDP port and the HTTPS front are reached through the TCP tunnel, so everything the app sends crosses the shaped link in one TCP connection.
- **Reply received / duplicates** counts all content sync-core handed the app after the switch, and of it the bytes it had handed over before (`crates/phone-wasm` checks each object's revision and ranges). It should stay 0.
- The driver presses Enter again, 1.5 s apart, while the composer still holds the prompt (it ignored the first Enter in some runs, for reasons not found); the measure starts at the first Enter, and each retry is a note in the report. The WebSocket client never gets a second Enter: its composer may hold the text while its socket is stalled, and a second Enter could send the prompt twice.
- The report notes each time the timeline shows "Scroll to end", that is, stopped following new text; the driver does not tap it.

`scripts/run-acceptance.sh` runs the web app on every profile, from a warm start and on the graded profiles from a cold start too, with eight seeds of case 70 and case 50, into [reports/acceptance](reports/acceptance/README.md); `./bin/lab acceptance <dir> reports/baseline` writes its report with each target of issue #46 and the baseline beside every run.

### End to end

`lab webapp-e2e` walks the app through everything a phone does, in WebKit at the iPhone 17 Pro's size (`driver/webapp-e2e.mjs`), with a screenshot of each state as the Safari tab and as the home-screen app. The link is profile A between the steps that shape it.

```sh
lab/scripts/build-webapp.sh lab/bin/webapp-next next   # once: the build the app updates to
sudo ./bin/lab webapp-e2e                              # --outage 60m for the long outage
```

The steps: pair with a code from `t3 pair`; the service worker takes over; add a project and start a thread from the app; stream a reply and answer its approval; interrupt a reply; attach an image to a prompt (the upload goes through the sidecar); send a prompt on case 70 (a screenshot of each prompt state, the status line and the details panel); a thread fills in newest first after the phone lost its cache; reload with the old connection alive; recover from an outage on the same connection (ICE restart, no new connection); reopen history and images with HTTPS to the server blocked; open the app with the link down; fall back to the WebSocket when UDP goes away under an open page, and with UDP blocked from the start; update to a new build; a thread deleted while the sidecar was down leaves the phone; the cache lost with the token kept; the token lost, which pairs again.

The driver asks the lab for what it cannot do itself (`shape case70`, `outage on`, `block udp on`, `block https on`, `swap build`, `delete-offline <thread>`, `mint-code`); the lab answers on its stdin. `lab/scripts/t3-command.mjs` deletes the thread through the server's own RPC while the sidecar is down. The run directory has `e2e.md` with each step's result, `phone.jsonl` and the screenshots in `phone/`.

## Story runs

A story is a few minutes of a 2G-like link drawn from a seed ([issue #58](https://github.com/djosh34/t3-optimizations/issues/58)): it changes every 2 to 8 s between an ordinary EDGE link (40 to 200 kbit/s, round trip 0.9 to 2.4 s, queues of 2 to 10 s), slow stretches down to bytes per second, round trips spiking to 8 s, loss in bursts, and gaps where nothing gets through for a second up to a minute and a half. One story in five is a survival story: 1 to 8 kbit/s at 70% loss. After 150 s the link comes back as the story's ordinary link. The phone keeps its address. The numbers follow [issue #15](https://github.com/djosh34/t3-optimizations/issues/15).

The one generator is in `crates/sync-core/src/sim/story.rs`; `lab/bin/story <seed>` prints a seed's story as JSON, and the simulator runs the same stories (`crates/sync-core/README.md`), so a seed that fails in one can be replayed in the other.

```sh
lab/scripts/build-sidecar.sh            # once, as yourself: also lab/bin/story
lab/scripts/build-webapp.sh
sudo ./bin/lab story --seed 7
./bin/lab story-summary reports/runs    # over every story run in a directory
```

`lab story` runs both web clients at the same time on the seed's story, against one stock t3 server:

- Two labs, one per client, each with its own phone container and shaped link. Both links follow the story in lockstep (`tc qdisc change`, which keeps what is queued), each with the seed for netem's dice.
- The server, the sidecar and the web app's front run in the second lab (`t3 pair`, which the sidecar runs, finds the server on its own host); the v0.1.7 client's front runs in the first. An unshaped veth pair joins the two server namespaces, and the first front forwards to the server across it.
- Each client has its own thread, with the setup turns of the other runs, then sends the measured prompt (tagged with its client, so the agent's log tells the turns apart) as the story starts. The reply is the standard one, about 2 kB. The deadline is the story's end, 5 minutes after it starts.
- `lab/scripts/t3-watch.mjs` is a client of the server on its own host: it logs when the server releases each piece of assistant text (`releases.jsonl`). The server releases only finished lines, and code blocks once they close.

Each run writes each client's usual report in `websocket/` and `webapp/`, and `story.md` and `story.json` with both side by side:

- **Prompt confirmed**, and whether within 10 s (case 70's target).
- **First reply text** and **whole reply shown**; not done by the deadline is a failure.
- **Newest-text lag from the server's release**, the largest, and whether within 5 s for the whole reply (case 50's target, graded from the release as [issue #59](https://github.com/djosh34/t3-optimizations/issues/59) decided). The lag from the agent is there too, for information.
- **Freezes**: stretches in which the phone showed nothing new while the server had released text it did not show. The longest, and every one of 30 s or more as a hang.
- **Reloads**: page loads after the switch. **WebSockets opened** after the switch: the v0.1.7 client's reconnects, the web app's fallback. **Transports**: the web app's transport at each connection; `webrtc` throughout means it stayed on the data channel (UDP), so nothing it showed waited on a TCP connection.
- The host's load: CPUs, mean and highest load, least memory available.

On GitHub Actions, a dispatch of CI with `stories` runs the seeds as a matrix, one job per seed, and the simulator on 100,000 seeds in 20 jobs; see below.

## CI

The `lab-smoke` job runs profiles A, outage10 and case70 for 3 minutes each with the same setup: namespaces, the stock server, WebKit in the same image. A must finish; the other two must produce a report. It then runs `--client pipe` on case50 for 30 s, which must carry ticks both ways, `--client sidecar-native` on case70 for 3 minutes, which must get the prompt confirmed, and `--client webapp` on case70 for 3 minutes, which must too. The reports are uploaded as an artifact.

A dispatch with `stories` set runs the story jobs instead: one build, a lab job per seed (`lab-seeds`, default 50) running `lab story`, 20 simulator jobs sharing `sim-seeds` (default 100,000; a broken seed fails its job and is named in its log), and a summary of both in the run's summary and the `stories-report` artifact.

```sh
gh workflow run ci.yml --ref <branch> -f stories=true
```

## Code

- `cmd/lab`: the command.
- `internal/run`: one run, from setup to report.
- `internal/netlab`: namespaces, veths, netem, outages, cleanup.
- `internal/profile`: the profiles and their netem arguments.
- `internal/t3server`: the stock t3 server and its setup through the t3 CLI.
- `internal/agent`: the fake agent and its reply.
- `internal/front`: the HTTPS front.
- `internal/tunnel`: the TCP outer path.
- `internal/phone`, `driver/`: the container and the Playwright drivers, `driver.mjs` for the t3 web client and `pipe.mjs` for pipe runs (Playwright 1.64.1, image `mcr.microsoft.com/playwright:v1.64.1-noble`).
- `internal/run/sidecar.go`: sidecar runs; the native phone is `crates/native-phone`.
- `internal/report`: the measures, JSON and Markdown.
- `internal/pipereport`: the measures of pipe runs.
