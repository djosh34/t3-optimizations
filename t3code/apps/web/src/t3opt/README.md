# t3opt: the home-screen web app

The t3code web client, served by the sidecar (`crates/sidecar`), talking to it through sync-core instead of the forked server's WebSocket ([issue #45](https://github.com/djosh34/t3-optimizations/issues/45)). `lab/scripts/build-webapp.sh` builds it with `T3OPT_BUILD=1`; a normal build never loads this folder.

`src/bootstrap.ts` runs `startT3opt()` (`index.ts`) before the app: it opens IndexedDB, starts the sync engine when the app is paired, puts the shim in front of `fetch` and `WebSocket`, and mounts the status line. The rest of the web client is unchanged.

## The parts

- `phoneWasm.ts`: sync-core's phone from `crates/phone-wasm` (`/phone.wasm`), through its plain C ABI.
- `store.ts`: IndexedDB. The pairing (device id, 128-bit token), the WebRTC certificate and the kept answer, the connection generation, and sync-core's saved state and objects.
- `engine.ts`: moves packets between the phone and the link. What a packet brought is committed to IndexedDB first; only then does the phone ack it, so a page killed in between gets it again. Commands and calls are stored in the phone's log before they go out.
- `link.ts`: the transport (below).
- `objects.ts`, `snapshots.ts`: the synced objects (`crates/sidecar/README.md`, "What phones get") and the forked server's state rebuilt from them.
- `shim.ts`: the in-page server (below).
- `StatusLine.tsx`: the status line and its details panel.
- `serviceWorker.ts` and `../../t3opt/sw.js`: the offline shell and updates.

## The shim

The client keeps its own RPC and HTTP code; the shim takes the place of the forked server behind it.

A WebSocket to `/ws` is a `ShimSocket` that speaks the same Effect RPC JSON:

- `subscribeServerConfig`, `server.getConfig`, `orchestration.subscribeShell` and `orchestration.subscribeThread` are answered from synced state. Each change of the state sends a new snapshot; while the client has not acked the last one, only the newest waits.
- `orchestration.subscribeThread` also tells the sidecar which thread is open (`sidecar.focus`), so its newest text goes first.
- `assets.createUrl` for a synced image answers with a `blob:` URL of its bytes.
- `orchestration.dispatchCommand` goes out as a sync-core command; the status line follows a prompt from queued to sent to "server has it" to confirmed.
- Every other call goes to the sidecar as one call and one reply. Subscriptions (`subscribeServerLifecycle` and the like) get the first value t3 sends and stay open. Subscriptions, the client's own activity reports and commands it sends by itself (`thread.visit`) go at the background class; other calls go first, because an action may wait for them (a prompt waits for `server.refreshProviders`).

Fetches to the page's own `/api`, `/oauth` and `/.well-known` paths:

- `/api/auth/session` and `/.well-known/t3/environment` come from synced objects; before pairing the session says "not authenticated", so the client shows its pairing screen.
- `/api/auth/browser-session` (the pairing screen's code) goes to the sidecar's `/api/pair`, which exchanges the code with t3 and answers with a device id and token. The page then reloads and syncs.
- An assistant text can arrive before its turn item: the newest text goes first, and its turn item, a kilobyte of JSON, waits behind the thread's status. The shim shows the text in a stand-in row, a copy of the thread's newest complete assistant item with this text, in the newest run, after every other item. The real item has the same id and takes its place.
- The snapshot loaders (`/api/orchestration/shell`, `/api/orchestration/threads/:id` and `/bounded`) come from synced state. A thread shows as soon as its thread body is here; turn items and texts fill in as they arrive, a missing part of a text shown as `…`. Every turn item the phone has gets a row, by t3's own visibility rule, so `hasMoreHistory` is always false and history never needs the network.
- The client's own traces are dropped.
- Everything else goes to the forked server through the sidecar (`sidecar.http`); a request with a body, such as an image upload, at the background class.

## The link

WebRTC data channel first (unordered, no retransmits), the sidecar's WebSocket when WebRTC does not open:

- One `RTCPeerConnection` lives as long as the page and is never closed on purpose. ICE credentials are derived from the token (`pipe::creds`), so nothing but the first connection needs signaling: later connections and ICE restarts use the kept answer and the certificate in IndexedDB.
- With nothing from the sidecar for 8 s the app restarts ICE on the same connection, and again every 8 s. Only when the channel closes, or after 60 s of silence during which ICE was connected for 20 s (the path works, the connection does not: the sidecar restarted), does it make a new connection (the next generation). An outage never connects ICE, so it is recovered on the same connection; a link starved at 70% loss can be silent for half a minute with ICE up, and a new connection there needs a handshake that takes minutes, so both bounds are long. A reload is a new generation too; the old page's connection may still be alive.
- At heavy loss WebKit calls the connection failed while data still arrives; only a closed channel counts as gone.
- If ICE did not connect within 20 s of the page opening, UDP is blocked: the WebSocket carries the packets and WebRTC is tried again every minute; when it opens, it takes over. A connection whose ICE connected but whose handshake is slow is waited for: it takes minutes at 70% loss, and TCP does worse there. `?transport=websocket` uses only the WebSocket.
- The app makes its next packet only when WebKit's send buffer is nearly empty, so packets do not get old in WebKit's SCTP queue while its window is small.

Every minute, and when the page is hidden, the app posts its link statistics to the sidecar's `/api/link-report`.

## Pairing and the cache

- First open: t3code's pairing screen; a code from `t3 pair --base-dir <the forked server's dir>`.
- IndexedDB lost its content but kept the token: the phone starts with a new random epoch, which tells the sidecar to sync everything again (sync-core's README: the epoch takes the place of the one-byte flag of issue #19).
- IndexedDB lost the token too: the pairing screen again.

## Offline and updates

`scripts/t3opt-sw.mjs` writes `sw.js` with the files of the entry and every route, and a version that changes with any of them. The service worker caches them on install, serves the cached `index.html` to every navigation, and keeps other hashed assets the first time they load. A new build installs in the background over HTTPS; the status line then shows "Update ready, reload". The sync link never carries the app itself. `/__sw` answers with the version in control.
