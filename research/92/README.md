# Thread-opening measurements for issue 92

Research on release v0.2.0, commit `504223a2`. No application or service changes. The issue comment is the report. These files preserve its measurements and experimental browser drivers.

- `samples.json` contains the accepted individual timings and each timing's host load averages. The load cutoff is 8 for the eight-core host. There are three cold starts and five opens per route and viewport, plus five prompt measurements per plan-1 route and viewport.
- `summary.json` contains median and maximum values in milliseconds, except load averages and request counts. Intervals overlap. Do not add their medians together. A fetch interval ends when `fetch` returns its response. The following interval also includes response consumption, decoding and state application.
- `login-labels.json` lists only labels supplied by this experiment and their counts. It contains no login identifiers or credentials.
- `open.mjs` measures the main routes. It blocks every orchestration command except locally acknowledging and discarding visits, and blocks snapshots and subscriptions for threads outside the supplied allowlist. The first thread in the input manifest must be one of the existing test threads. It creates no main threads, projects or prompts.
- `prompt.mjs` measures the isolated plan-1 pair only. It refuses ports other than 13773 and 12443. Its two input manifests identify dedicated test threads. The driver reads each current title because the server may rename a thread after a prompt. It uses different arithmetic answers to distinguish new responses from previously rendered messages. The final cohort used GPT-6 Luna with low thinking on both routes.
- `diagnostic.mjs` adds config-field sizes, receive progress, IndexedDB transaction timestamps and browser-container CPU accounting. It is an experimental driver, not an application patch.

## Running a driver

Use the repository's `t3lab-phone:1.64.1` image from `lab/driver/Dockerfile`. Its base is `mcr.microsoft.com/playwright:v1.64.1-noble` and its driver package is Playwright 1.64.1. Use host networking. The scripts import Playwright from `/opt/driver` inside that image.

Mount a private directory at `/private`, and one driver at `/run.mjs`. Supply arguments `normal|opt`, `safari|home-screen`, a fresh pairing code, and a run number. Pair against the correct base directory before the run. Never use the prompt driver against the main service.

The opening manifest is `/private/config.json` with `origins.normal`, `origins.opt`, and `tests`. Each test has its existing `id`, `title`, and expected last assistant `reply`. The prompt manifests are `/private/plan1-fresh-safari.json` and `/private/plan1-fresh-home-screen.json`, with the same origin and thread fields. Origins, pairing material, thread identities and response text are private inputs and are deliberately absent here.

Do not upload the private mount or unfiltered driver output. A failure writes local diagnostic text there. No driver takes screenshots. The public samples have only numeric measurements, route aliases and protocol names.

## Clock definitions

Cold starts have fresh browser contexts and HTTP caches. The normal route receives its login cookie before timing. The sidecar route receives only a pairing record in IndexedDB before timing, not a saved sync state or objects. The login exchange itself is outside the cold-start clock. The sidebar opens as soon as the app's control exists. Its thread list must be inside the viewport and fully opaque.

An open starts at the page's actual `pointerdown`, not before Playwright waits for its click target. It ends when the expected assistant reply is inside the viewport and fully opaque. Before a missing open the sidecar phone has zero complete frame objects for that thread. Before a cached open it has one. The cached measurement follows an app reload in the same context, so its HTTP and persistent data caches remain.

A prompt starts at Enter. Its text must appear in the viewport. A separate animation-frame observer records the first visible text of the new number-only assistant answer. The exact complete answer is checked before the run proceeds. RPC receive and sidecar object-arrival timestamps locate the final rendering cost. The prompt driver waits outside the measurement when the one-minute host load is at least 7.

Pilot measurements with wrong selectors, stale titles or ambiguous old-response detection are not samples. Sixty-four high-load main timings were excluded and rerun. One pilot main click timed out before it delivered a pointer event. The final trace cohort completed every prompt and every expected-answer check. These measurements do not establish a long-history scroll or stall pass.
