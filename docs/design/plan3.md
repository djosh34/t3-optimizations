# Plan 3: a layered design for t3opt

This is the answer to [A layered design for t3opt](https://github.com/djosh34/t3-optimizations/issues/119). It replaces the v0.2.0 sync design with three layers: a link that works like a WebSocket with priorities, a cache that keeps repeated bytes off the link, and a translation that carries t3's own protocol through both. It is written against `main` at `504223a2` (release v0.2.0) and the research on #90, #91, #92, #110 and #116.

The gates it is built for ([#103](https://github.com/djosh34/t3-optimizations/issues/103), [#104](https://github.com/djosh34/t3-optimizations/issues/104), [#107](https://github.com/djosh34/t3-optimizations/issues/107), [#112](https://github.com/djosh34/t3-optimizations/issues/112)): every action takes at most native's time ÷ 0.8 on every seed, on any link, with any number of phones, and never stalls. The fixed constraints still hold: stock t3 v0.1.7, nothing upstream, t3opt is an addition to the normal path, the phone is the t3 web app in WebKit with a shim, WebRTC data channel with WebSocket fallback.

## The idea in one paragraph

v0.2.0 cuts t3's state into sync objects and rebuilds t3's responses from them on the phone. Most faults the research found come from that rebuilding, from the one loop that does it, or from a link that cannot tell when one direction has stopped. Plan 3 stops rebuilding. The stock client's own requests travel to the sidecar, which runs them against stock t3 and sends back t3's own answers. The stock client then does what it does against a stock server: a bounded snapshot, its history cursor, `afterSequence` resume, incremental config, the shell's events. Between the two ends sits a link that is resilient on bad networks and sends the most important stream first, and a cache that replaces bytes the phone already holds with 17-byte references. The sidecar runs on tokio with one task per phone.

## The layers

```text
 phone (WebKit)                                  sidecar (tokio)                       stock t3 v0.1.7
 ┌──────────────────────────┐                    ┌──────────────────────────────┐
 │ stock t3 web client      │                    │                              │
 │  Effect RPC frames, fetch│                    │                              │
 ├──────────────────────────┤  RPC and HTTP as   ├──────────────────────────────┤  one WebSocket and
 │ translate: the shim      │◄── streams ───────►│ translate: the relay         │◄─ HTTP per phone ─► t3
 ├──────────────────────────┤                    ├──────────────────────────────┤
 │ cache: chunk store       │◄── refs, chunks ──►│ cache: encoder, have-set     │
 ├──────────────────────────┤                    ├──────────────────────────────┤
 │ link: session            │◄── packets ───────►│ link: session                │
 │  carriers: data channel, │                    │  carriers: str0m connection, │
 │  WebSocket               │                    │  WebSocket                   │
 └──────────────────────────┘                    └──────────────────────────────┘
```

| Layer | Its one job | It owns | It does not know |
|---|---|---|---|
| Link | Carry messages on prioritized streams between one phone and the sidecar, reliably, across any carrier, and say whether both directions work | packet numbers, acks, retransmits, the send window, stream buffers, carriers, liveness | JSON, t3 |
| Cache | Never send bytes the other side already holds | chunks cut at JSON value boundaries, the phone's chunk store, the sidecar's record of what each phone holds, warm entries | t3's protocol, packets |
| Translate | Make the stock client and stock t3 talk as if they were connected directly | the mapping of RPC requests and HTTP calls to streams, request priorities, one t3 connection per phone | packets, chunks |

Three layers is the smallest number that keeps each job separate. A link that knew about JSON would have to change with t3's protocol. A translation that knew about chunks would mix caching into every request type. The cache cannot sit inside the link because the link carries bytes it must not interpret, and it cannot sit inside the translation because then each request type would need its own dedup.

Crates and files after plan 3:

| Where | What |
|---|---|
| `crates/link` | the link session, sans-IO, built for the sidecar and for wasm32 (grows out of `sync-core`) |
| `crates/cache` | chunking, the encoder and the have-set, sans-IO, native and wasm32 |
| `crates/pipe` | the WebRTC carrier: str0m connections, derived credentials, the UDP receive task |
| `crates/sidecar` | tokio tasks: carriers, one session task per phone, the relay, warm entries, the store, HTTP |
| `crates/phone-wasm` | link and cache for the web app, one plain C ABI |
| `crates/native-phone` | a Rust phone over the same three layers, for tests and the lab |
| `t3code/apps/web/src/t3opt` | carriers, the chunk store and the shim in TypeScript |

## The link layer

### What it is

A link session is one phone's connection to the sidecar. It lives as long as the page on the phone and survives every change of carrier under it: an ICE restart, a new WebRTC connection, a move to or from the WebSocket. Either side opens streams. A stream carries messages in order and can be cancelled. Every stream has a priority, and a packet always takes from the highest priority that has bytes to send. Nothing in the link knows what a message means.

The link keeps what plan 1 proved on bad links ([#6](https://github.com/djosh34/t3-optimizations/issues/6), [#16](https://github.com/djosh34/t3-optimizations/issues/16), [#25](https://github.com/djosh34/t3-optimizations/issues/25)): an unordered, never-retransmitting data channel underneath, its own acks and retransmits, loss that never lowers the rate, repeated copies of small urgent data at the measured loss, Brotli-compressed packets at most 1,100 bytes, and no handshake beyond what the browser forces.

### Interface

Rust, in `crates/link`. The core does no I/O and reads no clock, as `sync-core` does today.

```rust
pub type Millis = u64;

/// Phone-opened streams have odd ids, sidecar-opened ones even.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StreamId(pub u64);

/// Highest first. Acks, pings, credit and resets are not a priority:
/// they go in every packet that has room, and in a packet of their own when due.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Priority { Interactive, Visible, Background, Prefetch }

pub enum Event {
    Opened { stream: StreamId, priority: Priority },
    Message { stream: StreamId, body: Vec<u8> },
    Finished { stream: StreamId },          // the peer sends no more on it
    Reset { stream: StreamId },             // the peer cancelled it
    Reprioritized { stream: StreamId, priority: Priority },
    Health(Health),
}

pub enum Health {
    Live,
    /// Which direction has made no progress, and since when.
    Stalled { inbound: bool, outbound: bool, since: Millis },
    /// The sidecar does not know this session (it expired or the sidecar restarted).
    Unknown,
}

impl Session {
    pub fn phone(token: [u8; 16], now: Millis) -> Session;
    pub fn sidecar(device: DeviceId, session: u64, now: Millis) -> Session;

    // The layer above.
    pub fn open(&mut self, priority: Priority) -> StreamId;
    pub fn send(&mut self, s: StreamId, body: &[u8]) -> Result<(), WouldBlock>;
    pub fn space(&self, s: StreamId) -> usize;           // bytes `send` takes now
    pub fn consumed(&mut self, s: StreamId, bytes: usize); // gives the peer credit back
    pub fn set_priority(&mut self, s: StreamId, p: Priority);
    pub fn finish(&mut self, s: StreamId);
    pub fn reset(&mut self, s: StreamId);
    pub fn poll_event(&mut self) -> Option<Event>;

    // The carrier below.
    pub fn receive(&mut self, now: Millis, packet: &[u8]) -> Result<(), DecodeError>;
    /// The next packet, or None. `queued` is what the carrier holds unsent:
    /// SCTP's buffered amount, or the WebSocket's unsent bytes.
    pub fn poll_transmit(&mut self, now: Millis, queued: usize) -> Option<Vec<u8>>;
    pub fn next_wakeup(&self, now: Millis) -> Millis;
    /// A new carrier took over. What was in flight on the old one counts as lost.
    pub fn new_carrier(&mut self, now: Millis, reliable: bool);
    pub fn stats(&self) -> Stats;
}
```

TypeScript, in `t3opt/link.ts`, around the same session compiled to wasm:

```ts
export type Priority = "interactive" | "visible" | "background" | "prefetch";

export interface LinkStream {
  readonly id: number;
  send(body: Uint8Array): boolean; // false: no space, wait for onSpace
  consumed(bytes: number): void;
  setPriority(p: Priority): void;
  finish(): void;
  reset(): void;
  onMessage: (body: Uint8Array) => void;
  onFinish: () => void;
  onReset: () => void;
  onSpace: () => void;
}

export interface Link {
  open(priority: Priority): LinkStream;
  onOpen: (s: LinkStream) => void; // streams the sidecar opens: warm entries
  readonly health: Health;
  onHealth: (h: Health) => void;
}
```

### Packets and framing

The format keeps `sync-core`'s varints, packet numbers and ack ranges (`crates/sync-core/src/wire.rs`), and swaps objects for stream frames, as QUIC does:

```text
packet = version:u8 pn:v [ack] [report] frame*
frame  = STREAM   id:v [priority:u8] offset:v fin:u8 len:v data
       | RESET    id:v
       | CREDIT   id:v max_offset:v          id 0 is the whole session
       | PRIORITY id:v priority:u8
       | PING                                 asks for an ack
       | HELLO    token:16 session:v          the phone's first packets on a carrier
       | WELCOME  session:v                   the sidecar knows the session
       | UNKNOWN                              the sidecar does not
message = len:v body                          inside a stream's bytes
```

Packet numbers belong to the session, not to a carrier, so a new carrier needs no renumbering. Acks name packets; the sender maps them to stream byte ranges (`RangeSet` from `sync-core`), so a byte acked on any carrier is never sent again. The packet is Brotli-compressed with the built-in dictionary only, as today, so every packet decodes alone.

### Priorities and preemption

Every packet is filled in this order:

1. Acks, credit, resets and a ping when due.
2. Retransmissions and repeated copies of the highest priority that has any, then its new bytes.
3. The next priority down, and so on.

Within one priority, streams take turns one packet at a time, so a 1 MB snapshot and a 300-byte command reply on the same priority interleave and the reply goes in the next packet. New bytes of a higher priority overtake everything of a lower one at the next packet. Because the carrier queue is held to about one packet (next section), a new Interactive message waits at most for one packet already in the carrier, plus what is on the wire.

Repeated copies stay as in plan 1: bytes of Interactive streams in messages of at most one packet go out in as many packets in a row as make 90% arrive at the measured loss, at most six. Lower priorities rely on retransmission.

### Sending: admission, window and pacing

This replaces today's rate control (`crates/sync-core/src/rate.rs`).

A packet is built only when three things allow it:

- **The carrier is writable.** `poll_transmit` gets the carrier's unsent bytes and returns nothing while they exceed `LOW_WATER`, one full packet (1,100 bytes). On the sidecar that is SCTP's `buffered_amount` for the channel; on the phone the data channel's or WebSocket's `bufferedAmount`; on the sidecar's WebSocket the socket is opened with `TCP_NOTSENT_LOWAT` at 16 kB, so it reports writable only when the kernel holds less than that unsent. Behind `tailscale serve` that socket only sees the local hop to tailscaled, so on the WebSocket it is the window (next point) that keeps the queue in the link. A packet the carrier refused was never built: it has no number and counts for nothing. This removes #91's fake loss.
- **The window has room.** Bytes in flight (sent, not acked, not yet declared lost) stay under `cwnd`. When acks stop, the window fills and sending stops. There is no 50 B/s trickle into a link that has gone quiet; a ping every retransmit timeout (capped at 2 s, as SCTP's is) asks whether the path is back.
- **The pacer allows it.** Packets leave at the pacing rate, so a full window does not go out as one burst into a shallow queue.

The numbers come from what `rate.rs` already measures well: the delivery rate from the peer's cumulative reports, and the RTT with its minimum over 60 s. The policy is new:

- `cwnd = max(10 packets, 2 × delivery_rate × min_rtt)`. Pacing rate = 1.25 × the delivery rate while the sender is window-limited, else the delivery rate.
- Start: if the device's last session measured this path (same carrier kind and remote address) less than 10 minutes ago, start at half its delivery rate. Otherwise start at 10 packets and double per round trip until the RTT grows by more than a quarter above its minimum. A 1 MB response at 60 ms RTT takes about seven round trips from cold, the same as TCP's slow start on the normal path; v0.2.0 started at 2,000 B/s and needed 6.4 s for the config alone (#92).
- Loss never lowers the window or the rate (plan 1, #25). RTT growth does: when the smoothed RTT stays more than a quarter above the minimum plus three times the jitter for a round trip, the delivery-rate estimate drops to 75% and the window follows. The path-change rule of `rate.rs` stays: a drain that does not bring the RTT down makes the current RTT the new minimum.
- On a reliable carrier (WebSocket) the link sends no repeated copies and does not retransmit: TCP does both. It still acks, so a later move to WebRTC knows what arrived.

On a clean link the window, not the pacer, ends up the limit, and the carrier check keeps SCTP's queue near empty. That leaves the queue in the link, where priorities still apply.

### Flow control

Each stream has a credit: the highest offset the receiver accepts. The sender never sends past it. The receiver gives credit back as the layer above calls `consumed`, which the phone's shim does as the stock client acks each chunk, and the relay does as t3 accepts its writes. Default credit is 2 MB per stream and 16 MB for the session; a single message larger than the stream credit raises that stream's credit to the message size, because a message is useful only whole.

Credit protects memory. It is not what keeps urgent data first: priorities do that, so a Prefetch stream the phone is slow to read never stands in front of an Interactive one.

### Liveness in both directions

Each side keeps three clocks:

- `last_in`: the last valid packet received.
- `last_out_progress`: the last time an ack covered something new that this side sent. It moves only while something was outstanding.
- `last_drain`: the last time the carrier's unsent bytes went down, or were below `LOW_WATER`.

With nothing to send for a second, a side sends a PING, which the peer must ack. So both sides always have outstanding data to measure progress on. Health is:

- `Live` when `last_in` and `last_out_progress` are both within `max(4 × srtt, 3 s)` and the carrier drains.
- `Stalled { inbound, outbound }` otherwise. A packet coming in never resets the outbound clock. This is the watchdog #90 proved broken: in v0.2.0 a downstream packet every 5 s kept the phone's only clock fresh while its send buffer never drained.

What each side does with it:

- **Phone, in the foreground.** No progress in either direction, or a carrier whose unsent bytes have not gone down, for 8 s restarts ICE on the same connection, and again with the wait doubling up to 30 s. These are today's timings in `link.ts`, which the lab tuned (#45, #46: restarting after 4 s hurt at 70% loss). What changes is what counts as silence. The later steps stay too: a new connection when ICE is up but nothing moves for 60 s, the WebSocket when ICE never connects. A restart costs one STUN round trip and keeps DTLS and SCTP.
- **Phone, coming back.** On `visibilitychange` to visible, `pageshow`, `online`, and a change of `navigator.connection` where WebKit has it, the phone sends a PING at once. Without an ack within `max(2 × srtt, 1 s)` it restarts ICE at once instead of waiting out a silence clock, and opens the WebSocket in parallel. Whichever carrier answers first carries the session (next section). While iOS has suspended the page no timer runs, so the resume event is what counts.
- **Sidecar.** It never tears down a session because of silence. With the window full and nothing acked, it only pings. After 30 s with nothing in, the session is suspended: the relay stops acking t3's stream chunks, so t3 keeps and merges what would have gone out. After 15 minutes with nothing in, the session expires: its t3 connection closes and its memory is freed. The phone's next HELLO then gets UNKNOWN.

Every report the phone posts to `/api/link-report` gains what #111 asked for: visibility state, per-direction packet counters, the carrier's unsent bytes, ICE state, health, and the time of each change, so the next iPhone silence names its layer.

### Carriers, resume after background and network change

A carrier is anything that moves whole packets: the data channel (unordered, `maxRetransmits: 0`) or the sidecar's WebSocket (`/sync`, one binary message per packet).

- The phone's first packets on any carrier carry HELLO with the device token and its session number. The sidecar answers WELCOME and moves the session to that carrier, or UNKNOWN.
- An ICE restart keeps the carrier. Nothing happens in the link.
- A new carrier for a known session calls `new_carrier` on both sides. Packets in flight on the old carrier count as lost and their bytes go again; bytes already acked never do. Streams, their order and the layers above see nothing.
- On a cold start both carriers start at once: the WebSocket to the sidecar's HTTPS route (TCP, TLS, upgrade, as the normal GUI pays; through `tailscale serve` on the live hosts) and the data channel without signaling (the kept answer and certificate, #27). The session runs on whichever opens first and moves to the data channel when it opens. The link no longer waits 20 s for ICE before it can carry anything.
- `UNKNOWN` (the session expired, or the sidecar restarted) ends the session on the phone. The link starts a new one, the shim closes its sockets, and the stock client reconnects through its own code and resumes each subscription with `afterSequence`. This is the same path the normal client takes after a dropped WebSocket.
- A new session from the same device (a reload) ends the older one at once, as a newer connection generation does today.

## The cache layer

### Encoding

Every message body the translation hands down goes through the cache. Bodies under 1 kB pass as they are. Larger ones are cut into flat chunks at JSON value boundaries:

- A value of at least 512 bytes with no child that large is a chunk of its own: a turn item, a message, one provider's settings, a run of the shell's thread rows.
- A larger value is split into its parts, down to such values; the brackets, keys and commas between them go with the next chunk.
- Values under 512 bytes are grouped. A group ends after a value whose hash matches a mask, or at 4 kB, so the boundaries depend on the content and not on position.
- A string over 8 kB, and any body that is not JSON (an image, an upload), is cut by a rolling hash over its bytes (FastCDC: 512 bytes to 8 kB, about 2 kB on average).

Each chunk is named by the first 16 bytes of its BLAKE3 hash. The encoded body is a list:

```text
encoded = segment*
segment = LIT  len:v bytes             small or not worth keeping
        | NEW  hash:16 len:v bytes     store this chunk under this hash
        | REF  hash:16                 a chunk the phone holds
```

The receiver concatenates the segments and has the original bytes. Chunks are flat: no chunk refers to another, so the phone can evict any chunk alone. Because a turn item or a provider is one chunk wherever it appears, the same item in a snapshot, an event and a later snapshot is sent once, and a timestamp changed inside one provider of the config changes that provider's chunk and nothing else. The cache reads JSON only to find boundaries; it does not know t3's types, so config, thread snapshots, history pages, shell events and image bytes all go the same way.

Interfaces:

```rust
// crates/cache
pub type Hash = [u8; 16];

/// A body cut into chunks once, shared by every phone it goes to.
pub struct Prepared { pub bytes: bytes::Bytes, pub chunks: Vec<(Hash, Range<usize>)> }
pub fn prepare(bytes: bytes::Bytes) -> Prepared;

/// One per phone: what that phone holds.
pub struct Encoder { /* the have-set */ }
impl Encoder {
    pub fn encode(&mut self, body: &Prepared) -> Vec<u8>;
    pub fn stored(&mut self, hashes: &[Hash]);                       // the phone reported them durable
    pub fn drop_request(&mut self, hashes: &[Hash], sent: u64) -> u64; // returns the message count to wait for
    pub fn digest(&self) -> Hash;
}
```

```ts
// t3opt/cache.ts
export interface ChunkStore {
  /** The original bytes. Chunks in NEW segments enter memory before any lookup. */
  decode(encoded: Uint8Array): Promise<Uint8Array>;
  /** The warm entry for a request key, if one is held. */
  warm(key: string): Promise<Uint8Array | null>;
}
```

### What each side stores

- **Phone**: chunks in IndexedDB (`chunks` store, hash to bytes, with last use), a memory map of recent chunks (32 MB), and warm entries (`warm` store, request key to encoded body). Its cache epoch is random and changes when it starts with an empty store.
- **Sidecar**: for each device, the have-set: the hashes the phone reported durable, under the phone's epoch. About 100,000 hashes for a 200 MB phone store, 1.6 MB on disk. It holds no chunk bytes for live traffic; the body it is sending has them. It also holds the warm entries it prepared, in one LRU shared by all phones, bounded by `--cache-mb`.

### How it decides nothing unnecessary is sent

The rule has one line: the sidecar sends `REF h` only if `h` is in that phone's have-set. Everything else follows from keeping that set true.

- **Growing the set.** After a NEW chunk is committed to IndexedDB, the phone sends `Stored [h…]` on the cache's control stream (phone-opened, Background priority). Only then does `h` enter the have-set. A chunk received but not yet committed is sent again if needed; that costs a few duplicate kilobytes within one commit time, and keeps the rule simple. Commits never gate acks or sends: the link acks on receipt.
- **Shrinking the set without a race.** When the phone wants to evict, it sends `Drop [h…]`. The sidecar removes them from the have-set and answers `Dropped { wait_for: n }`, where `n` is the number of messages it had sent on the session at that moment. The phone deletes the chunks once it has received every message numbered below `n`. Any message that could still refer to them arrives first.
- **A missing chunk.** If the phone cannot read a chunk it was promised (IndexedDB lost it), it fails that one response, which the stock client retries through its own code, and sends `Drop` for it. Tests make this path rare; it is never silent.
- **Resume from a sequence.** The cache does not do this; t3 does. A subscription resumes with `afterSequence` and gets only events after it (translate, below). The cache then makes what does get sent small: an event that repeats a turn item the phone has, or a snapshot of a thread the phone held, is mostly references.

### Warm entries

Plan 2's preload ([#81](https://github.com/djosh34/t3-optimizations/issues/81)) becomes warm entries: the sidecar fetches the bounded snapshot of each of the newest `--preload` (50) live threads once, prepares it once, and offers it to every connected phone that lacks it on a sidecar-opened Prefetch stream: `{key: "GET /api/orchestration/threads/<id>/bounded", sequence}` plus the encoded body. Prefetch goes only when no other priority has bytes, so it never delays anything.

When the stock client asks for that path, the shim answers from the warm entry at once. That answer is a real t3 response at a real sequence, so the client's next step, subscribing with `afterSequence` at that sequence, brings it current through t3's own replay or a fresh snapshot. Nothing on screen is put together by t3opt. History pages are not warmed: native fetches them when the reader scrolls, and so does t3opt (#116).

### Memory bounds, eviction, restart

| Where | Bound | When over |
|---|---|---|
| Phone chunk store | 200 MB in IndexedDB | least recently used chunks go, through the Drop exchange |
| Phone memory map | 32 MB | least recently used chunks leave memory, stay in IndexedDB |
| Phone warm entries | 50 entries | oldest first; their chunks stay until evicted on their own |
| Sidecar have-set | 16 bytes per chunk the phone holds, about 1.6 MB on disk per phone | follows the phone |
| Sidecar warm LRU | `--cache-mb` (256 MB) | least recently offered goes |

On a sidecar restart the have-sets load from disk. At the start of every session the phone sends its epoch and the XOR of all its chunk hashes. If either differs from the sidecar's copy, the sidecar empties that phone's have-set and asks the phone to report its hashes again at Background priority. Until it has them it sends no references. A phone that lost its store starts a new epoch and the sidecar empties the set.

### Measured on stock t3

I ran a throwaway stock t3 v0.1.7 (`--base-dir` under `/tmp`, port 13805) with one project and two seeded 600-message threads (`scripts/research116/seed.py`), took its real config, bounded snapshots and history pages, and counted what three ways of chunking would send. The scripts are in `scripts/research119/`. "Plain" is FastCDC over the bytes; "subtrees" sends a reference for any JSON subtree of 512 bytes or more the phone holds (nested, so it needs a tree-shaped store); "aligned" is the scheme above. Bytes before compression; each row assumes the phone holds what the rows above it sent.

| Response | Bytes | Plain | Subtrees | Aligned |
|---|---:|---:|---:|---:|
| Config, first time (`subscribeServerConfig` snapshot) | 101,437 | 101,998 | 102,364 | 104,174 |
| Config again, same content (`server.getConfig`) | 101,420 | 7,821 | 2,740 | 4,421 |
| Config again, all 17 `checkedAt` values changed, as a provider health refresh does (#90) | 101,420 | 26,060 | 9,667 | 10,968 |
| Config again, one `checkedAt` changed | 101,420 | 8,630 | 3,766 | 3,376 |
| Bounded snapshot of a 600-message thread, first time | 85,602 | 85,806 | 39,594 | 44,963 |
| Its first history page | 59,342 | 59,495 | 59,424 | 60,719 |
| The bounded snapshot again | 85,602 | 204 | 20 | 2,000 |
| The bounded snapshot after the newest reply grew by 620 bytes | 86,222 | 4,495 | 4,697 | 5,131 |

What this shows:

- A first fetch costs about 2% more than the plain bytes, for the chunk headers.
- Repeats cost a few kilobytes instead of the whole. A changed provider costs that provider. On the main t3code, where #92 measured 194 kB of providers with workspace snapshots left out, every provider health refresh would cost its changed providers, not 194 kB (and not v0.2.0's 563 kB).
- A snapshot repeats its own turn items (`turnItems` and the rows of `visibleTurnItems` carry the same item), so even a first snapshot halves with JSON-aware cuts. Plain byte chunks miss that.
- History pages share nothing with the snapshot, as expected: they are older items.
- Aligned chunks send 40 to 60% of what plain chunks send on config and snapshots, and within 15% of nested subtrees where something changed. On an unchanged repeat they cost 2 to 4 kB of references where subtrees cost tens of bytes. They keep the store flat, which decides the scheme (open choice 4).

The empty server's config did not change on its own in 7 minutes of sampling, so the `checkedAt` rows change those values in the real config by hand, as #90 observed them change on the main t3code. The two seeded threads have the same synthetic text, so I report only the first; its text compresses about 30 times, far more than real text, so compressed sizes are left out.

## The translation layer

### The phone: the shim

The shim stays where v0.2.0 put it, in front of `WebSocket('/ws')`, `fetch` and `XMLHttpRequest` (`shim.ts`, `installShim`). It no longer answers from synced state. It forwards:

- **A `Request` frame** opens a link stream at the request's priority, with one message `{tag, payload}`. The payload goes unchanged, so every option the client sets (`workspaceSnapshotsOnDemand`, `afterSequence`, `acceptBoundedSnapshot`, `requestCompletionMarker`, `acceptDedupedBodies`, …) reaches t3 as the client sent it.
- **Messages back** are `{chunk: values}` or `{exit: exit}`, delivered to the client as `Chunk` and `Exit` frames under the client's request id. The shim hands a chunk only after the client acked the one before, as a stock server does, and calls `consumed` as it hands them, which gives the sidecar credit.
- **`Interrupt`** resets the stream; the sidecar interrupts t3.
- **`Ping`** gets a local `Pong` while the link session exists. The link watches both directions and repairs the carrier; if the client's 15 s socket watchdog fired on a bad link it would tear down every subscription and start again, which is what makes the normal path fail on bad links (plan 1, #42). When the session ends (`Health::Unknown`), the shim closes the socket and the stock client's own reconnect runs.
- **A fetch to `/api/…`, `/.well-known/…`, `/attachments/…`, `/oauth/…`** opens a stream with `{method, path, headers, body}`; the answer is `{status, headers}` then the body. Three paths are still answered on the phone: the websocket ticket (the sidecar holds the real t3 credentials), pairing (as today), and a GET of a thread snapshot that has a warm entry.

The focused thread is the one in the page's route. The shim reads it from `location` on each navigation (it wraps `history.pushState` and listens to `popstate`) and sets priorities on open streams when it changes.

`snapshots.ts`, `objects.ts`, the stand-in rows and `whenReady` are deleted.

### The sidecar: the relay

For each phone session the relay opens one WebSocket to t3 (`/ws` with a ticket from the sidecar's token, as `t3.rs` does now) and makes HTTP calls with the same token.

- A phone stream with `{tag, payload}` becomes one RPC request on that phone's t3 socket. The relay maps the stream id to its own request id and back.
- Each t3 `Chunk` goes to the cache encoder and onto the stream. The relay acks the chunk to t3 when the stream's unsent bytes in the link are under 64 kB. Until then t3 holds the next chunk, and for a thread subscription merges streaming updates while it waits (`ThreadLiveEventCoalescer.ts`, 50 ms window). A slow link thus gets fewer, merged events from t3 itself, with no merging logic in t3opt.
- An `Exit` goes as the stream's last message.
- An HTTP stream becomes one HTTP request; the body is prepared and encoded like any message.
- Values go through as raw JSON text (`serde_json::value::RawValue`): the relay never builds a tree of a snapshot to send it on. The cache scans the text once for value boundaries.

The relay reads three things in what it forwards, and only these:

- the `sequence` of thread and shell events and the `snapshotSequence` of snapshots, per stream, to resume (below);
- the `threadId` of `orchestration.subscribeThread` and of thread HTTP paths, to know which streams belong to the focused thread;
- the `commandId` of commands, for the log.

### How the stock protocol maps through

| Stock mechanism | What happens on the t3opt path |
|---|---|
| Bounded snapshot (`GET …/threads/:id/bounded`, `acceptBoundedSnapshot`) | Forwarded, or answered from a warm entry. The client keeps its real `historyCursor`, `hasMoreHistory` and `latestLocalTurnOrdinal`. |
| History paging (`GET …/threads/:id/history?cursor=`) | Forwarded when the client's `loadEarlier` asks, at Interactive priority for the focused thread. Merged by the client's own `mergeOlderHistoryIntoProjection`. Nothing is fetched before the reader scrolls. |
| `afterSequence` replay | Forwarded. t3 replays up to 128 events (`THREAD_RESUME_MAX_REPLAY_EVENTS`) or sends a snapshot. The client drops events at or below its sequence (`threads.ts:486`). |
| A live stream t3 ends because the client is too slow (`LiveStreamBufferError`, over 1,000 items or 8 MB) | For thread and shell subscriptions the relay subscribes again with `afterSequence` set to the last sequence it forwarded, on the same phone stream. The client sees one stream with no gap. Other streams pass the failure to the client, which retries as it would on the normal path. |
| The relay's t3 socket drops (t3 restarted) | The relay reconnects, sends requests without an exit again (t3 applies a command once per `commandId`), and resumes thread and shell subscriptions as above. |
| Incremental config (`subscribeServerConfig` with its options) | Forwarded unchanged: one snapshot, then `providerStatuses`, `settingsUpdated`, `keybindingsUpdated` events. Workspace snapshots stay out when the client asks for that (#92 measured 194 kB against v0.2.0's 563 kB). |
| Shell stream (`subscribeShell`, `afterSequence`, `omitOlderSettled`) | Forwarded. The client applies `thread.updated` and the other shell events itself; no whole shell snapshots per change. |
| Commands (`orchestration.dispatchCommand`) and their acks | Forwarded at Interactive. The `Exit` is t3's ack. |
| Side subscriptions (`subscribeVcsStatus`, `subscribeAuthAccess`, …) | Forwarded as real streams with live updates, no longer frozen at their first value. |
| Session and descriptor (`/api/auth/session`, `/.well-known/t3/environment`) | Forwarded. No bootstrap barrier: each request travels as soon as the client makes it. |

Because every value the client sees is a value t3 sent, the client's state is always one the normal client could also be in. That is what removes #116's flashing: no half-built thread, no invented ordinal, no history the reader did not ask for.

### Priorities by request

The shim sets them; the relay sends replies at the priority of their stream.

| Priority | Requests |
|---|---|
| Interactive | `orchestration.dispatchCommand` (prompts, approvals, stop, plan actions); `subscribeThread`, the bounded snapshot and history pages of the focused thread; uploads; `assets.createUrl` and attachment GETs of the focused thread |
| Visible | `subscribeShell`, `subscribeServerConfig`, `server.getConfig`, session and descriptor, other threads' subscriptions, every other call by default |
| Background | the client's own reports and side subscriptions (`BACKGROUND_CALLS` in `shim.ts` today), the cache's control stream |
| Prefetch | warm entries |

When the reader switches threads, the old thread's streams drop to Visible and the new one's rise to Interactive in the same packet.

### Commands

v0.2.0 writes every command to IndexedDB before it may leave, and that write blocks the phone's whole transmitter when it hangs (#110, fault 2). Plan 3 sends a command the moment the client makes it, like the normal path. `commandId` makes it safe to send twice. To keep plan 1's promise that a typed prompt survives a page killed on a bad link, the shim also writes each `message.dispatch` command to a small `outbox` store, without anything waiting on that write. An entry leaves the outbox when its `Exit` arrives. On the next page start, the shim sends every entry still there again with its `commandId`; t3 applies a command once per `commandId`, so one that already arrived changes nothing. See the open choices.

## The sidecar on tokio

One multi-threaded tokio runtime. No task does another phone's work, and no task waits on disk or on another phone.

| Task | How many | Owns | Talks to |
|---|---|---|---|
| `udp_rx` | 1 | the receive half of the UDP socket | session tasks, by `try_send` of datagrams |
| `session` | 1 per phone with a live session | the link session, its str0m connections and WebSocket carrier, the cache encoder, the relay's stream table | `udp_rx`, its `t3_conn`, `store`, `warm` |
| `t3_conn` | 1 per phone session | that phone's WebSocket to t3 | its `session` |
| `t3_http` | per request, bounded | one HTTP call to t3 | the `session` that asked |
| `warm` | 1 | the shared shell subscription, the warm LRU, a semaphore of 2 fetches | t3, sessions by `try_send` |
| `store` | 1 | device records, have-sets, link reports on disk | sessions, by `try_send` |
| `http` | hyper | pairing, offers, static files, link reports, the `/sync` upgrade | sessions through their mailbox |

How the pieces fit:

- `udp_rx` reads datagrams in a loop and does nothing else. It routes a STUN request by its username fragment and anything else by source address to the session that owns that connection, with `try_send` into a channel of 256. A full channel drops the datagram, which is loss the link handles. Nothing on this task waits for a send timer, so #91's starvation cannot happen. An unknown, authentic STUN request (checked with the derived credentials, `pipe::creds`) goes to the device's session, which creates the connection.
- Each `session` task runs a `select!` over: datagrams from `udp_rx`, WebSocket frames, t3 frames from its `t3_conn`, finished HTTP calls, warm offers, and the link's next wakeup. It owns its `str0m::Rtc` directly, so SCTP's buffered amount is a plain call, and sends with `UdpSocket::try_send_to` on the shared socket (a full kernel buffer is loss).
- Heavy work stays bounded: preparing a body (one scan for value boundaries and BLAKE3 of each chunk, a few milliseconds per megabyte) runs inline up to 256 kB and on `spawn_blocking` above that; Brotli of a large body likewise.
- `store` gets the newest state per device and writes it on `spawn_blocking` with `fsync`. A session never awaits it. If the channel is full the session keeps its newest state and offers it again on its next save tick.
- `warm` prepares each warm entry once and hands the same `Arc<Prepared>` to every session; each session encodes it against its own have-set. Two phones never hash the same bytes twice (#110 measured 93 ms for one 24 MB object set into 8 phones).

Bounds per phone session:

| What | Bound |
|---|---|
| Datagrams from `udp_rx` | 256 |
| t3 frames from `t3_conn` | 64; t3's own per-chunk acks hold the rest in t3 |
| Unsent bytes per stream in the link | 64 kB before the relay acks t3 again |
| Unacked bytes per session | the window, at most the session credit, 16 MB |
| Open streams | 128; a further `open` from the phone waits for one to close |
| Requests in flight to t3 | 64 |

Memory is one shared budget, not a worst case per phone. The sidecar holds a byte budget (`MemoryMax` minus the warm LRU minus 128 MB of headroom, 384 MB with today's unit) as a tokio `Semaphore`. Every byte a session queues or keeps unacked holds permits from it. A session takes a reserve of 2 MB for Interactive traffic when it starts; Interactive bytes draw on that reserve first, so a prompt, an approval or the open thread always moves. Lower priorities wait for permits: while they wait, the relay acks none of their t3 chunks and forwards none of their new requests, and t3 holds and merges what would have come. A phone whose session cannot get its reserve (about 190 sessions at once) gets "busy" in answer to HELLO and tries again; a test asserts it.

The shared parts are the UDP socket, which `udp_rx` drains without waiting, the byte budget, t3 itself, as with any number of normal clients, and CPU. Session tasks that have nothing to do cost nothing.

## The phone's main thread

The link and cache run in wasm on the page's main thread; the shim is JavaScript there too. Per received packet the work is: decode the packet, copy its stream bytes into the stream buffer once, and when a message completes, resolve its references and hand JSON text to the client. No step touches the whole of an object per packet, so a 563 kB config costs about its size in copies, not #110's 476 MB. IndexedDB writes happen after the fact and never gate an ack or a send. Cold start does not read the store: chunks load when a message refers to them, in one transaction per message.

## Faults from the research and the layer that removes each

| Fault | Found in | Removed by | How |
|---|---|---|---|
| Phone hidden, nothing reached it for 439.6 s; the sidecar trickled 50 B/s into the gap | #90 | Link | The window stops data when acks stop; resume events probe at once and restart ICE after one unanswered probe; the trace from #111 names the layer next time. |
| Watchdog never repairs a link whose send buffer is stuck while packets arrive | #90 | Link | Outbound progress and carrier drain have their own clocks; an inbound packet resets neither. |
| 563 kB config resent whole on timestamp changes | #90, #110 (7) | Translate, cache | The stock incremental config with the client's options; a changed provider costs its changed chunks. |
| The 99% bar | #90 | Translate | Nothing is "synced to 100%" any more. The status line shows link health and requests in flight. |
| The pipe skips reading UDP while a send deadline has passed | #91 | Sidecar tasks | `udp_rx` only reads and routes; timers live in session tasks. |
| Packets counted as sent before SCTP takes them; refused writes become fake loss | #91, #110 (4) | Link | A packet is built only when the carrier is writable; a refusal never consumes a number. |
| The focused thread's reasoning and tool items wait behind bulk config | #91 | Translate, link | The focused thread's subscription is Interactive and config Visible; within the thread stream t3's own order holds. |
| Two phones at once, worst open 18.7 s | #91, #97 | Sidecar tasks | One task per phone, bounded queues, shared preparation of warm entries. |
| Cold start waits for a full bootstrap before answering | #92 | Translate | Every request travels as the client makes it. |
| Stock options ignored, 563 kB against 194 kB | #92 | Translate | Payloads forwarded unchanged. |
| 2,000 B/s start | #92 | Link | Ten packets doubling per round trip, or the device's last measured rate. |
| Whole shell snapshots up to 867 kB per change | #92 | Translate | The stock shell stream's events. |
| Command acks about 2.5 times slower | #92 | Translate | No IndexedDB write before sending; Interactive priority; t3's own `Exit`. |
| No check that both directions work; the shim answers Ping locally | #110 (1) | Link | Two-way liveness in the link; Ping stays local because the link owns repair. |
| One stuck IndexedDB write stops every packet the phone sends | #110 (2) | Link, cache | Nothing on the send or ack path waits for storage. |
| One loop for all phones, conversion and fsync | #110 (3) | Sidecar tasks | Per-phone tasks; `store` and large preparation off the session tasks. |
| An open has no completion or error; paging disabled | #110 (5), #116 | Translate | t3's own 200, 404 and errors reach the client; its real cursor pages on demand. |
| Whole-object copies per chunk, whole-thread rebuilds | #110 (6), #116 | Link, translate | Messages reassembled once; the client applies events itself. |
| Open threads ignore the cache limit | #110 (7) | Sidecar tasks | The sidecar holds no thread state for open threads; t3 does. Its only cache is the bounded warm LRU. |
| Half-built thread states, stand-ins with invented ordinals | #116 | Translate | Only t3's responses reach the client. |
| 37.5 MB of whole-thread snapshots per 10 s of streaming | #116 | Translate | t3's events, merged by t3 when the link is slow. |
| Newest text waits behind its own backfill on a sudden rate drop (plan 1 finding on #58) | #1 | Link | Backfill is Prefetch; the carrier queue holds one packet; Interactive goes first in the next packet. |

## What is kept, rewritten and deleted

Kept:

- The vendored sctp-proto, `is` and dimpl patches, str0m, and `pipe::creds`: derived ICE credentials, signaling-free new connections and ICE restarts, the DTLS server role without cookies.
- From `sync-core`: varints, packet numbers and ack ranges (`wire.rs`, `inbox.rs`), `RangeSet`, the delivery-rate and RTT measurements in `rate.rs` (not its policy), repeated copies sized to loss, per-packet Brotli, and the simulator's link model, WebKit SCTP model and story generator.
- From the sidecar: config and flags, pairing with t3 (`t3 pair`, token exchange, re-pairing), the RPC framing in `t3.rs`, the data dir (`cert.der`, `devices/`, `link-reports.jsonl`), the HTTP routes.
- From the web app: the carrier code in `link.ts` (one `RTCPeerConnection` per page, kept answer and certificate, generations, ICE restart, WebSocket fallback and its lab-tuned timings), `store.ts` for pairing, the fetch, XHR and WebSocket interception in `shim.ts`, `StatusLine.tsx` with new inputs, the service worker.
- `native-phone`, rewired to the new layers.
- The lab, its profiles, network stories and WebKit driver.

Rewritten:

- `sync-core` objects, classes, revisions, outbox and inbox become `crates/link`'s streams.
- `crates/sidecar`'s `app.rs`, `device.rs` and `store.rs` become the tokio tasks above.
- `phone-wasm`'s ABI: packets and events of a link session, plus the chunk decoder.
- `engine.ts` becomes the session pump; `link.ts`'s watchdog becomes the two-way health rules.
- `rate.rs`'s policy becomes the window and pacer.

Deleted: `world.rs` (cutting threads into objects), `objects.ts` and `snapshots.ts` (rebuilding them), stand-in rows, object classes and orders, the 50 B/s floor and the idle restart, preload by objects, the phone's command log written before sending.

## Order of work

`main` stays releasable throughout: v0.2.0's engine stays the default until the new one passes every gate, and the switch is one release.

1. **Failing tests and the measuring ground.** The v0.2.0 failing tests from the research branches go into the repo as gates for the new engine (#96). The seeded fake-history generator (#105) and the lab's side-by-side WebKit harness with alternating runs and medians (#106, #112) land, with the #111 trace fields. The harness runs native against v0.2.0 first: it must show v0.2.0 failing the gates.
2. **`crates/link`** with its simulator, on its own. Nothing uses it yet.
3. **`crates/cache`** with its property tests, on its own.
4. **The sidecar's tokio skeleton** behind `--engine v3`: `udp_rx`, session tasks, both carriers, `store`, HTTP. `native-phone` speaks the link to it over the real data channel.
5. **The relay** in the session task: `native-phone` makes stock RPC calls and HTTP requests through it, compared byte for byte with direct calls to t3.
6. **The web app's side** behind a build flag: link wasm, carriers, chunk store, the new shim. The WebKit side-by-side runs start gating here.
7. **Warm entries**, the persisted have-set, and the remembered path rate.
8. **The switch:** `--engine v3` and the new web build become the default; the slow release runs (100,000 synthetic seeds, 200 WebKit seeds per profile); v0.3.0; both sidecars upgraded live and checked again (#99, #109).
9. **Removal** of the v0.2.0 engine and its tests.

Each step is a PR that keeps `make check` green and changes nothing for a running v0.2.0 phone until step 8.

## Tests

### Each layer alone

- **Link.** The sans-IO session runs in the deterministic simulator that grows out of `sync-core/src/sim.rs`: netem-style rate, loss (including gemodel bursts), queue, delay, duplication, outages; the WebKit SCTP sender model; carrier writability; and an iOS suspension model (the page stops for a seeded time, then resumes, often on a new path; #111). Per seed it checks: every message arrives once and in order on its stream; no acked byte goes again; no packet over 1,100 bytes; no data packet goes out with the window full; an Interactive message's first copy leaves within one packet plus the carrier's queue of being sent; `Stalled` is reported within `max(4 × srtt, 3 s)` of a half-open path in either direction; after a resume the session is `Live` within one round trip plus one ICE restart once the path works.
- **Cache.** Round trips over random bodies and random edits; the Drop exchange under random interleavings of sends, acks, Drops and evictions (no REF ever reaches a phone without the chunk); epoch and digest mismatch after a simulated sidecar restart and after a cleared phone store; the memory bounds.
- **Translate.** A transparency test against a throwaway stock t3 with the fake history: the same requests made directly and through the relay (with the link and cache in memory) give byte-identical values, including `afterSequence` replays, a forced `LiveStreamBufferError` and a t3 restart. The shim's tests drive it with recorded stock frames and check that the client receives exactly those frames, in order, one chunk per ack.
- **Sidecar concurrency.** Phone A gets a slow t3, a slow disk, a 120 MB thread and a silent link while phone B opens, sends and reads; B's times must not change. Repeated with 1, 2, 4, 8 and 16 phones (#110's proof list).

### Together: the fast seeded test

The fast test (#98, #108) runs the real link, cache and relay code in the simulator, against a recorded model of t3's responses on the fake history, with a model of the stock client's requests for each action recorded from the WebKit runs. A second model runs the normal path (the stock client over TCP on the same simulated link). Each seed gives t3opt's and native's time for every action on the #104 list, and the gate is t3opt ≤ native ÷ 0.8 on every seed, 10,000 seeds per PR and 100,000 before the release (#107). It may gate only when it is never more forgiving than WebKit (#114, still open): it must also run v0.2.0's engine and fail it on the 99% stall, the two-phone tail and the slow cold start, and it may never report t3opt faster than the WebKit runs measured for the same seed.

### WebKit side by side

In the lab without Tailscale (#106): a stock t3 with the fake history (about 6,000 threads, 4 GB, largest 120 MB, #105), native straight to it and t3opt through the sidecar, both behind the same link profile and seed, both iPhone 17 Pro sizes. Every action from #104 (cold start, cold start after reload, first pairing until usable, open a cached, uncached, largest, archived and delegated-task thread, scroll up until older messages show, fast switching, prompt until shown and until first reply text, streamed text lag, approvals, plan actions, stop, thread list updates, back from background, network change), plus #116's scroll checks: the reading anchor may not move more than 4 px during a hold, as native does not. Five alternating runs per seed and action, medians compared (#112); 20 seeds per profile per PR, 200 per release; a run on an overloaded host counts as failed. Phones per run: one, two and four, each held to the same gate (#97).

### The v0.2.0 failing tests from the research branches

| Test | Branch | Becomes |
|---|---|---|
| `scripts/research90/watchdog.ts` | `research/90-phone-silence` | A test of the new phone link's health: send buffer held at 1,200 bytes with packets still arriving every 5 s must give `Stalled { outbound }` and an ICE restart within 8 s. |
| `crates/pipe/tests/expired.rs` | `research/91-open-timings` | A `udp_rx` test: a datagram waiting while a session has a full send backlog is routed within 1 ms. |
| `device.rs` test `newest_reasoning_of_the_open_thread_precedes_bulk_config` | `research/91-open-timings` | A link scheduler test: an Interactive message queued behind a 600 kB Visible message leaves in the next packet. |
| `scripts/research116/regression.ts` and `assert-scroll.mjs` | `research/116-scroll-flash` | The shim's transparency test (no row the client did not get from t3) and the WebKit scroll gate at both sizes. |
| `scripts/research110/probe.rs` (half-open path, copy volume, cache limit, multi-phone hashing) | `research/110-design-probes` | Simulator liveness test; a phone copy counter (bytes copied per received message at most three times its size); the sidecar memory bound with 16 phones; shared preparation of warm entries. |
| #92's cold-start and command-ack timings | `research/92-open-timing` | WebKit gate actions. |

Each first runs against v0.2.0 and fails there, then gates the new engine.

## Open choices

1. **Answer bootstrap reads from the phone's cache before the link is up?** Options: (a) forward `/api/auth/session`, the descriptor and the config like everything else; (b) answer from the last copy at once and let the stream catch up. Recommendation: (a). The cold-start race of both carriers costs about what the normal path pays for TCP and TLS, and (b) can show a stale auth state the client cannot take back. Switch to (b) for session and descriptor only if the cold-start gate fails on link setup.
2. **When the relay acks t3's chunks.** Options: (a) when the link's unsent bytes for the stream are under 64 kB; (b) when the phone's client acked the chunk, end to end. Recommendation: (a). (b) costs a round trip per chunk on a slow link, as the normal path does, and gains nothing (a) does not give.
3. **Resume thread and shell streams in the relay, or let the client do it.** Recommendation: in the relay, as above. It is about 40 lines and the client never shows a stream error for a phone that was in the background. Letting the client do it is the fallback if it turns out to hide a real failure.
4. **How the cache cuts bodies.** Options: (a) plain content-defined chunks over the bytes, which know nothing of JSON; (b) nested JSON subtrees, the smallest on the wire but a tree-shaped store whose eviction must follow parents; (c) flat chunks aligned to JSON values. Recommendation: (c). The measurements above put it at 40 to 60% of (a) on config and snapshots and within 15% of (b) where something changed, with a flat store. If unchanged repeats turn out to matter, a reference to a whole known body (one hash for the lot) closes that gap without nesting.
5. **References to chunks received but not yet committed.** Options: (a) only durable chunks, as designed; (b) also chunks received in this session, kept in memory until committed. Recommendation: (a) until the WebKit runs show duplicates that cost a gate; (b) needs per-message delivery events in the link and a pinning rule on the phone.
6. **Link and cache on the main thread or in a worker.** Recommendation: main thread, since per-packet work is now proportional to the packet. Move to a worker only if the side-by-side runs show input delay, and only after checking that WebKit on iOS can transfer an `RTCDataChannel` to one; without that, the worker would get packets through `postMessage`.
7. **One t3 socket per phone or one shared.** Recommendation: one per phone. It isolates phones and matches what t3 does for many normal clients; a shared socket would need its own fair scheduler.
8. **The sidecar's congestion control.** Options: (a) the window and pacer above; (b) put back SCTP's congestion control (undo the vendored cwnd patches) and rely on its writability alone. Recommendation: (a). SCTP halves its window on loss, which plan 1 removed because it starves a 70% loss link.
9. **A prompt typed just before the page is killed.** Options: (a) the side outbox above, resent on the next start if t3 has not seen its `commandId`; (b) no outbox, as the normal path. Recommendation: (a). It keeps plan 1's promise without putting storage on the send path. It is named here because it is the one place t3opt sends a command the client did not just make.
10. **Warm history pages.** Recommendation: no. Native fetches pages on scroll, the gate compares with that, and #116 showed what history arriving unasked does.

## Risks

- **Streams are ordered.** v0.2.0 sent each message's newest bytes first; plan 3 sends a thread's events in t3's order. On a lossy link a lost packet holds the focused stream until it is repeated or retransmitted. Repeated copies of Interactive bytes (90% first-try arrival at the measured loss) and t3's merging under backpressure keep this short, and the chaos lab cases 50 and 70 and the network stories from plan 1 run against plan 3 to show it is no worse than v0.2.0 there.
- **One t3 socket per phone adds load on t3.** It is the load of one normal client per phone; the concurrency tests measure t3's own time with 16 phones.
- **WebKit behaviour the simulator does not model** (ICE agent pauses, timers while hidden) can still differ. The #111 trace in every link report is how the live phone tells us.
