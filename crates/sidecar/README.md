# sidecar

Connects a stock t3 server (v0.1.7) to phones over the sync protocol (`sync-core`) on the WebRTC data channel (`pipe`), with a WebSocket fallback ([issue #44](https://github.com/djosh34/t3-optimizations/issues/44)). It is a normal client of that server ([issue #58](https://github.com/djosh34/t3-optimizations/issues/58)), runs beside it on the same host and never touches the official t3code.

```sh
sidecar --t3-url http://127.0.0.1:13773 --t3-base-dir ~/.t3opt \
  --t3-command "node /opt/t3opt/t3code/apps/server/src/bin.ts" \
  --data-dir ~/.t3opt-sidecar --http 127.0.0.1:12480 --udp 0.0.0.0:12444 --udp-advertise <tailnet IP> \
  --web-dir /opt/t3opt/web
```

`--config <file.toml>` takes the same settings, `t3-url = "..."` and so on; flags win. Nothing has a built-in port or path. Two settings have defaults: `--cache-mb` (256) and `--preload` (50), see "Memory" below. On start it prints `sidecar ready http=<addr> udp=<addr>` on stdout and logs to stderr.

## The t3 side

- **Pairing with t3.** The sidecar runs `<t3-command> pair --base-dir <t3-base-dir>`, exchanges the printed code at `/oauth/token` and keeps the bearer token in `<data-dir>/t3-token`. When t3 rejects the token (it lives 30 days), the sidecar pairs again by itself.
- **Effect RPC over `/ws`**, with a ticket from `POST /api/auth/websocket-ticket/rpc`. It acks every stream chunk, pings every 5 s, drops a socket that is silent for 15 s, and reconnects with backoff. Requests without an exit go out again on the new socket: subscriptions start over with a snapshot, and t3 applies a command once per `commandId`.
- **A subscription that ends** (t3 ended it, it failed, or t3 has no such call) is logged and starts again after 1 s, then 2, 4 and so on up to 60 s while it keeps ending without a value; a value resets the wait.
- **What it reads.** `orchestration.subscribeShell` (the thread list), and nothing of a thread until a phone needs it ([issue #84](https://github.com/djosh34/t3-optimizations/issues/84)). A thread a connected phone has open gets `orchestration.subscribeThread` with `acceptBoundedSnapshot`: t3 sends its newest part, about 1 MB at most, then live events. Its older history follows from `GET /api/orchestration/threads/:id/history`, newest page first, unless the sidecar has it all already. A thread sent ahead of time (below) comes from `GET /api/orchestration/threads/:id/bounded` and the same pages. Up to three fetches run at once, a phone's open thread first, then images, then preloads. Images come from `assets.createUrl`, then a GET of the signed URL. The server's config comes from `server.getConfig`, again after each event of `subscribeServerConfig`; the environment descriptor and the sidecar's own session from `/.well-known/t3/environment` and `/api/auth/session` once connected.
- **Text as t3 releases it.** t3 holds assistant text back until a line or paragraph is finished, and a code block until it closes (`apps/server/src/orchestration-v2/assistantStreaming.ts`). The sidecar sends what t3 released, as every client of the server sees it.

## What phones get

The sidecar cuts t3's state into sync-core objects. Each has a JSON `meta`:

| Object | meta | content |
|---|---|---|
| Thread list header | `{"c":"shell"}` | `schemaVersion`, `snapshotSequence`, `olderSettled` of the shell snapshot |
| Project | `{"c":"project","i":id}` | the project shell |
| Listed thread | `{"c":"thread","i":id,"l":"active"\|"archive"}` | the thread shell |
| Entity of a thread | `{"t":thread,"c":collection,"i":id}` | one entity of the thread projection (`thread`, `runs`, `turnItems`, `messages`, ...), as t3's JSON, `"text":""` for messages and turn items |
| Text | `{"t":thread,"m":key,"k":kind}` | the text of a message (key: message id, the same for the message and its turn item) or of a reasoning item (key: item id); `k` is `assistant`, `user`, `reasoning`, ... |
| Frame | `{"t":thread,"c":"frame"}` | the snapshot fields (`snapshotSequence`, `historyCursor`, `hasMoreHistory`, `latestLocalTurnOrdinal`, `updatedAt`) and `visibleTurnItems`, local rows without their `item`; rows of history pages that come from other threads (a fork's) go first |
| Image | `{"c":"image","i":attachment,"y":mime,"o":thread}` | the bytes; `o` is one of the threads that show it, and phones delete the image when that thread leaves the list |

Every id in meta (`t`, `i`, `m`, `o`) is `sync_core::hash_id` of the id: 16 hex digits, however long t3's id is, so every meta fits sync-core's 512 bytes. The content has the real ids; the web app hashes the id it looks up ([issue #72](https://github.com/djosh34/t3-optimizations/issues/72)).
| Config | `{"c":"config"}`, `{"c":"descriptor"}`, `{"c":"session"}` | the server's config, its environment descriptor, the sidecar's session: what the web app needs before it shows anything |

An object with empty content is deleted. A body is not sent again when only its `updatedAt` changed, because streaming bumps it on every update.

Classes, per phone, follow [issue #16](https://github.com/djosh34/t3-optimizations/issues/16). The open thread is the one the phone named with `sidecar.focus`.

- **Tail:** the text of the open thread's newest assistant message.
- **Status:** the open thread's frame, its row in the thread list, its `thread`, `runs`, `attempts`, `runtimeRequests`, `plans`, `subagents`, and turn items that are pending, running or waiting.
- **History:** the config objects, the thread list's header and the projects first (the web app shows nothing before it has them), then the rest of the open thread.
- **ThreadList:** the thread list.
- **Background:** other threads.
- **Image:** images.

## Commands and calls

- **Command:** a phone command's payload is the JSON of an `orchestration.dispatchCommand`. The reply is `{"exit": <t3's exit>}`.
- **Call:** a phone call's payload is `{"tag": <rpc method>, "payload": ...}`. The reply is `{"exit": ...}`, or `{"chunk": [values]}` for a stream (the sidecar takes its first chunk and ends it).
- **`sidecar.focus`** `{"threadId": id | null}` is the sidecar's own call: it opens a thread for this phone, and the sidecar keeps it across restarts. The web app sends it as soon as the client asks for the thread's snapshot.
- **`sidecar.close`** `{"threadId": id}`: the phone closed this thread; it is no longer open unless the phone opened another since.
- **`sidecar.http`** `{"method", "path", "contentType", "body"}` (the body in base64) is the sidecar's other call: an HTTP request to the t3 server with the sidecar's token, for what the web app does over HTTP (image uploads, history pages, project icons). The reply is `{"http": {"status", "contentType", "body"}}`. Each request runs in a thread of its own.
- A payload the sidecar cannot read gets `{"error": "..."}`.

## Connections

Each phone pairs once, then reaches the sidecar like this:

1. **Pairing.** `POST /api/pair {"code"}` with a code from `t3 pair --base-dir <t3 base dir>`. The sidecar exchanges the code with t3, then answers `{"device", "token"}`. The token is 128 bits as hex, and every later request carries it as `Authorization: Bearer <token>`.
2. **First connection.** The phone sets its offer's ICE credentials to the derived ones (`pipe::creds`, generation 1, restart 0) and posts the offer to `POST /api/rtc/offer`. It keeps the answer and its `RTCCertificate` (IndexedDB).
3. **Later connections.** The phone raises the generation and connects without signaling: same offer, and the kept answer with the server's derived credentials for the new generation.
4. **ICE restart.** On the same connection, the restart number goes up and the phone munges the offer and the kept answer the same way. The sidecar sees the new username fragment and restarts ICE on that connection. DTLS and SCTP stay.
5. **Hello and handover.** The phone's first packets carry the sync-core hello. Packets go out on a connection only after its hello. A newer connection takes over from the old one and closes it; a connection of an older generation is ignored.
6. **Never on a timeout.** The sidecar never closes a connection on a timeout. SCTP retransmits without a limit, every 2 s at most.
7. **After a sidecar restart.** The phone's ICE restart for its current generation gets a fresh connection. ICE connects, the old DTLS session fails, and the phone makes a new connection without signaling.
8. **WebSocket fallback.** `GET /sync` upgrades to a WebSocket. Each binary message is one sync packet, starting with a hello.

What the native phone and the web app (`t3code/apps/web/src/t3opt`, issue #45) do:
- Restart ICE after 8 s with nothing from the sidecar, and again every 8 s while it stays silent. Not on ICE's own disconnected state: at heavy loss ICE reports that often while data still arrives.
- Make a new connection when ICE connects again but no data comes for 30 s, or when the channel closes. A new connection costs a DTLS handshake, which takes minutes at 70% loss.

Restarting sooner hurts. In the lab at 70% loss, a 4 s rule (and restarting on ICE's disconnected state) restarted every few seconds, and each restart dropped a working path for seconds.

Tested in WebKit (`tests/webkit`, Playwright 1.64.1, Linux):
- derived credentials in a munged offer
- an ICE restart without signaling
- a new connection from the kept answer with the certificate from IndexedDB, while the old one is open
- the WebSocket fallback

The real iPhone is not tested here.

`POST /api/link-report` (bearer token) stores the web app's link statistics in `<data-dir>/link-reports.jsonl`, 4 MB, then one older file.

## Memory

The sidecar holds only what phones opened or were sent ahead of time, and never the whole history ([issue #81](https://github.com/djosh34/t3-optimizations/issues/81)):

- **Threads load on open.** A phone's open thread loads as above; leaving it fetches nothing.
- **Live subscriptions follow connected phones** and the threads they have open. They end when a phone goes away or closes the thread, and start again when it comes back.
- **One cache, one limit.** Threads and images count against `--cache-mb` (default 256) as their encoded bytes. Past it, the thread used least recently goes, with the images no other thread in the cache shows; an open or loading thread stays. It loads again when it is needed. The sidecar keeps no JSON trees of a thread, only its objects' bytes, shared with each phone's sync state.
- **Phones keep what they have.** Eviction deletes nothing on a phone and never resets a revision: what a phone has not stored of an evicted object waits until the thread comes back. A phone's sync state holds an object's bytes only until the phone stored them, then their length and hash; its saved state holds no bytes at all.
- **Preload.** When no open thread is loading, the sidecar sends ahead the newest `--preload` (default 50) threads that are not archived, newest `updatedAt` first, one at a time, to connected phones whose link is idle (under 1 MB waiting) and that lack them. A thread whose `updatedAt` changed since the sidecar had it is fetched again; an unchanged one is not. Archived threads load only on open. `--preload 0` turns it off.
- **Cleanup by the thread list.** Phones delete the objects of threads that left the thread list, and, once the config is here, other objects the sidecar no longer has (a project removed while it was down). Evicted threads are still listed and stay.

On the main t3code (1,226 threads, 600 MB of turn items), two phones opening 30 large threads each kept the sidecar under 340 MB, with `MemoryMax=768M` ([issue #84](https://github.com/djosh34/t3-optimizations/issues/84)).

## Data dir

| File | What |
|---|---|
| `t3-token` | bearer token for the t3 server |
| `cert.der`, `key.der` | the DTLS certificate of every connection, so a kept answer stays valid |
| `devices/<id>.json` | a paired phone: token, certificate fingerprint, newest generation, open thread |
| `devices/<id>.sync` | its sync state (`Server::save`: revisions, lengths and hashes, no bytes), saved every 2 s while it changes, so a restart does not sync again what the phone has |
| `link-reports.jsonl` | link reports |

Every file is readable by the owner only.

## Tests

- `cargo test -p sidecar` runs the unit tests.
- `tests/integration.rs` runs a throwaway stock t3 server (temp `--base-dir`, the lab's scripted agent), the sidecar, a UDP relay that can cut the link, and native phones (`crates/native-phone`) over the real data channel. They need node with t3code's dependencies and Go, so plain `cargo test` skips them:
  - `sidecar_end_to_end` (CI's lab-smoke job): a prompt whose reply arrives a paragraph at a time as t3 releases it, an approval, a new thread, a sidecar restart, a page reload with the old connection alive, and a 10 s outage recovered by ICE restart.
  - `webkit_transport` (lab-smoke): the WebKit test above; it also needs docker.
  - `outage_5_minutes`, `outage_60_minutes`: run on the VM.

```sh
cargo test -p sidecar --test integration -- --ignored --exact sidecar_end_to_end
```

## After a restart

A restarted sidecar loads every phone's sync state, so it knows what each phone has, and loads threads again as above. Each thread list it gets deletes on phones the threads that left it, such as a thread deleted while the sidecar was down (`Server::delete`). The preload fetches its threads once more, and phones get only what changed.
