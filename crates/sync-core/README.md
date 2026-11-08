# sync-core

The sync protocol between the sidecar and the phone ([issue #43](https://github.com/djosh34/t3-optimizations/issues/43)). One crate, built natively for the sidecar and for `wasm32-unknown-unknown` for the web app. It does no I/O and reads no clock: the caller passes the time in milliseconds and carries packets between the two sides.

## The two sides

`Server` is the sidecar's side, one per paired phone. The sidecar puts what the phone should have into it as objects, and runs what the phone asks for:

```rust
let mut s = Server::new(token, Config::server(), now);
s.set(id, Class::Tail, order, meta, text)?;      // again whenever it changes
for incoming in s.receive(now, &packet)? {       // a packet from the phone
    match incoming {
        Incoming::Command { seq, payload } => s.reply(seq, run_command(payload))?,
        Incoming::Call { seq, payload, .. } => s.reply(seq, run_call(payload))?,
        Incoming::Reset => {}                     // the phone syncs from zero
    }
}
while let Some(p) = s.poll(now) { send(p); }      // then wake at s.next_wakeup(now)
```

`Phone` is the web app's side. It stores what arrives and acks a packet only after its content is stored:

```rust
let r = phone.receive(now, &packet)?;
// In one storage transaction: phone.save_object(k) for k in r.changed,
// delete r.removed, phone.save_state(). Hand r.replies to the UI.
phone.committed(r.batch);                        // only now is it acked
let seq = phone.command(dispatch_command_json);  // stays in the log until its reply is stored
```

`Phone::load` rebuilds the phone from what it saved. `Server::save` and `Server::load` let a restarted sidecar go on without a full resync.

## Objects and revisions

An object is bytes with an id, a class, an order within the class, a revision, and `meta` of at most 512 bytes: what the phone needs to show it alone, such as thread and message ids. `Server::set` compares new content with the old:

- Content that only grows keeps its revision. The phone gets the new bytes.
- Any other change (an edit in the middle, a shorter text, new meta) is a new revision. Its chunks say how many leading bytes of an older revision stay valid (`keep`). The phone cuts its copy to those bytes and fills in the rest.
- The phone ignores chunks of a revision older than the one it holds.

The server tracks per object which bytes of the current revision the phone has stored. Those are never sent again. Bytes in flight go out again after the retransmit timeout; with spare room in a packet, unacked bytes go out again early, so the sender never idles until the phone has everything.

## Order

One queue. Each packet takes from the highest class that has data:

| Class | What | Within the class |
|---|---|---|
| `Tail` | newest text of the last message | newest bytes first, then its start |
| `Urgent` | replies to commands and to calls the owner made | commands' first (a prompt's result does not wait behind call replies, which are repeated at loss too), then calls'; lowest number first in each |
| `Status` | approvals and thread status | highest order first |
| `History` | older messages of the current thread | highest order (newest) first |
| `ThreadList` | the thread list | |
| `Background` | other threads, replies to background calls | |
| `Image` | images | |

The packet header is the alive signal: with nothing to send, a packet of a few bytes goes out every second. `Urgent` objects up to 256 bytes ride in every packet until stored, and so does the ack, so text never starves command replies. New data in a higher class goes out in the next packet.

The tail chunk carries the meta and is compressed with Brotli's built-in dictionary only, so it decodes and shows without any older packet. Chunks are cut on UTF-8 character boundaries.

## Redundancy

`Tail`, `Urgent` and `Status` bytes go out in several packets in a row, as many as make at least 90% of them arrive at the measured loss (6 at 70% loss). This is repetition coding, the only erasure code there is for data that fits in one packet. If none arrives, they go out again after a short timeout (a quarter above the smoothed RTT). Other classes are resent after the usual retransmit timeout, which adds four times the RTT variation. Copies never resend what the phone has stored.

## Rate

The sender paces packets at a send rate in wire bytes per second, counting 93 bytes of IP, UDP, DTLS and SCTP headers per packet ([issue #25](https://github.com/djosh34/t3-optimizations/issues/25)).

- What arrived (from the phone's cumulative reports) is kept apart from what the sender may send. Loss never lowers the send rate.
- About once per round trip, on RTT samples of packets sent after the last change: if RTT grew (25% above the base, plus 150 ms or three times the measured jitter), the rate drops to 75% of the lower of itself and the measured send capacity, but never below half. Otherwise, with data waiting, it grows by half (at the start) or a quarter, at most that much above the measured capacity.
- One fresh sample above twice the base plus a second counts as RTT growth on its own: at heavy loss samples are rare, and waiting for three more lets the queue grow.
- The base is the lowest RTT of the last 60 s. If draining to 30% of the rate does not bring the RTT down, the path itself got slower: its RTT becomes the base, and the rate goes back to where the drains started, but not above the measured capacity.
- Sending with nothing at all heard from the other side for 15 s (or four smoothed RTTs, if longer) lowers the rate to 75% at each decision, and the next RTT samples start a new base in slow start. Either the link went down, or the rate is so far above a link that just got slower that its queue drops the other side's acks too: in the lab, after a switch from the clean link to 10 kbit/s, the sidecar kept its clean-link rate, WebKit's acks never got through, and no RTT sample ever came to lower it (issue #45). An idle side still sends a packet a second, so at 70% loss 15 s without one happens by chance in about 0.5% of such spells.
- After 2 s with nothing to send, the rate starts again from 2 kB/s. It never goes below 150 B/s.
- Packets hold about 100 ms of what the link delivers, between 200 and 1100 bytes, so a queue that counts packets stays short on a slow link.

## Commands and calls

The phone keeps a log. A command (`orchestration.dispatchCommand`) or call gets a number and is sent until the server has it; the server hands it to the sidecar once, and commands in the order they were made. The one exception is a sidecar restart: what it was handed but had not answered is handed over again, so the sidecar dispatches every command with its t3 `commandId`, which t3 applies once. The entry leaves the log when its reply is stored on the phone; until storage commits, the phone still reports it as the lowest entry it keeps, so the server keeps the reply for a page that dies before the commit. `Phone::entry_state` gives queued, sent, or "server has it" for the status line.

## Connections

The first packets of a connection carry the hello: the 16-byte token and the cache epoch. There is no other handshake. Packet numbers start at 0 on every connection.

- The sidecar finds the phone's `Server` with `Server::hello(packet)`, sends nothing on a connection before that, and calls `Server::connected()` when a new connection replaces the old one. Packets of the old connection must not reach `receive` after that.
- The web app calls `Phone::connected()` on every new connection. Log entries are then sent again in full; the server still applies each once.
- An ICE restart keeps the connection: no call to either.
- A phone that kept its token but lost its cache starts with a new random epoch. The server sees the new epoch and syncs everything again (`Incoming::Reset`). This takes the place of the one-byte "empty cache" flag of [issue #19](https://github.com/djosh34/t3-optimizations/issues/19): a flag repeated in a late duplicate packet would wipe the server's state a second time, an epoch does not. A phone that lost its token too pairs again.

## Packets

At most 1100 bytes of application data each, so DTLS, SCTP and IP headers fit the 1280-byte MTU of the tunnel. The format is in `src/wire.rs`.

## Simulator

`sim::run` runs a `Server` and a `Phone` over a simulated link with a scripted agent and the phone's storage. The link follows the lab's netem setup: rate, loss before the rate limit, a queue in packets, delay with normal jitter, duplication, outages. The same scenario and seed give the same run. `Scenario::case70` and `Scenario::case50` are the lab's graded cases from `lab/README.md`, with 200 kB of other threads' content changing as the profile applies.

```sh
cargo run --release -p sync-core --example sim -- case70 1 10       # 15 minutes, seeds 1 to 10
cargo run --release -p sync-core --example sim -- case50 1 3 180    # 3 minutes each
```

The tests in `tests/sim.rs` run both cases, a reply faster than the link with a second prompt, page kills between receipt and storage, duplication with reordering, and the send rate at 70% loss. Results of the full runs are on issue #43.

What the simulator does not model: the browser's own SCTP on the phone-to-server direction (issue #49 measured stalls of up to 7.6 s there at 70% loss), the cost of a new connection after a page kill, and timers that do not run in lockstep.
