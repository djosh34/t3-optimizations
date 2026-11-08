//! The protocol between a `Server` and a `Phone`, packet by packet.

use sync_core::wire::{Body, Packet};
use sync_core::{Class, Config, EntryState, Incoming, Key, MAX_PACKET, Millis, Phone, Server};

const TOKEN: [u8; 16] = *b"test-token-00001";
const TAIL: u64 = 1;

fn text(len: usize) -> Vec<u8> {
    let words = "the sync loop keeps the newest text first and older messages follow ";
    words.bytes().cycle().take(len).collect()
}

/// A server and a phone on a perfect link; the phone stores every packet at
/// once.
struct Pair {
    now: Millis,
    server: Server,
    phone: Phone,
    incoming: Vec<Incoming>,
}

impl Pair {
    fn new() -> Self {
        Pair {
            now: 0,
            server: Server::new(TOKEN, Config::server(), 0),
            phone: Phone::new(TOKEN, 7, Config::phone(), 0),
            incoming: Vec::new(),
        }
    }

    fn deliver_to_phone(&mut self, bytes: &[u8]) {
        let r = self.phone.receive(self.now, bytes).expect("phone decodes");
        self.phone.committed(r.batch);
    }

    fn deliver_to_server(&mut self, bytes: &[u8]) {
        let got = self
            .server
            .receive(self.now, bytes)
            .expect("server decodes");
        self.incoming.extend(got);
    }

    /// Runs both sides for `ms` milliseconds, 1 ms at a time. Returns the
    /// server's packets.
    fn run(&mut self, ms: Millis) -> Vec<Vec<u8>> {
        let mut sent = Vec::new();
        for _ in 0..ms {
            while let Some(b) = self.phone.poll(self.now) {
                self.deliver_to_server(&b);
            }
            while let Some(b) = self.server.poll(self.now) {
                self.deliver_to_phone(&b);
                sent.push(b);
            }
            self.now += 1;
        }
        sent
    }

    /// The next packet the server sends, delivered nowhere.
    fn next_server_packet(&mut self) -> Packet {
        loop {
            if let Some(b) = self.server.poll(self.now) {
                return Packet::decode(&b).expect("decodes");
            }
            self.now += 1;
        }
    }

    fn reply_to_commands(&mut self) {
        for i in std::mem::take(&mut self.incoming) {
            match i {
                Incoming::Command { seq, .. } | Incoming::Call { seq, .. } => {
                    self.server.reply(seq, b"ok".to_vec()).expect("reply");
                }
                Incoming::Reset => {}
            }
        }
    }
}

/// The byte ranges of the chunks in these packets.
fn chunk_ranges(packets: &[Vec<u8>]) -> Vec<(Key, std::ops::Range<u64>)> {
    packets
        .iter()
        .filter_map(|b| Packet::decode(b).ok())
        .flat_map(|p| p.chunks)
        .map(|c| (c.key, c.offset..c.offset + c.body.raw_len()))
        .collect()
}

#[test]
fn a_new_chunk_overtakes_backfill_in_the_next_packet() {
    let mut p = Pair::new();
    for id in 100..120 {
        p.server
            .set(id, Class::Background, id, Vec::new(), text(20_000))
            .expect("set");
    }
    // Backfill is going out; nothing reaches the phone.
    for _ in 0..5 {
        let packet = p.next_server_packet();
        assert!(packet.chunks.iter().all(|c| c.key != Key::Object(TAIL)));
    }
    p.server
        .set(
            TAIL,
            Class::Tail,
            0,
            b"thread:1 message:9".to_vec(),
            text(40),
        )
        .expect("set");
    let packet = p.next_server_packet();
    assert_eq!(
        packet.chunks.first().map(|c| c.key),
        Some(Key::Object(TAIL))
    );
}

#[test]
fn the_newest_tail_shows_on_a_fresh_phone_without_older_packets() {
    let mut p = Pair::new();
    for id in 100..110 {
        p.server
            .set(id, Class::History, id, Vec::new(), text(2_000))
            .expect("set");
    }
    let reply = "Ünïcödé text, newest bytes first. "
        .repeat(200)
        .into_bytes();
    let meta = b"thread:1 message:9".to_vec();
    p.server
        .set(TAIL, Class::Tail, 0, meta.clone(), reply.clone())
        .expect("set");
    // Only the first packet reaches a phone with an empty cache.
    let bytes = loop {
        if let Some(b) = p.server.poll(p.now) {
            break b;
        }
        p.now += 1;
    };
    p.deliver_to_phone(&bytes);
    let o = p.phone.object(TAIL).expect("tail arrived");
    assert_eq!(o.meta, meta);
    let have = o.have.ranges().to_vec();
    assert_eq!(have.len(), 1);
    assert_eq!(
        have[0].end,
        reply.len() as u64,
        "the newest bytes came first"
    );
    let shown = o.bytes(have[0].clone()).expect("bytes");
    assert!(
        std::str::from_utf8(shown).is_ok(),
        "the chunk starts on a character"
    );
    assert_eq!(shown, &reply[have[0].start as usize..]);
    // It was compressed with the built-in dictionary only.
    assert!((have[0].end - have[0].start) as usize > bytes.len());
}

#[test]
fn a_change_in_the_middle_sends_only_the_bytes_after_it() {
    let mut p = Pair::new();
    let mut content = text(3_000);
    p.server
        .set(TAIL, Class::Tail, 0, Vec::new(), content.clone())
        .expect("set");
    p.run(3_000);
    assert!(p.server.is_synced());
    content.truncate(2_900);
    content.extend_from_slice(b" and then a new ending.");
    p.server
        .set(TAIL, Class::Tail, 0, Vec::new(), content.clone())
        .expect("set");
    let sent = p.run(3_000);
    let o = p.phone.object(TAIL).expect("tail");
    assert_eq!(o.rev, 2);
    assert!(o.data == content);
    // Only bytes after the change went out (some more than once, while
    // their ack was on its way).
    let ranges = chunk_ranges(&sent);
    assert!(!ranges.is_empty());
    assert!(
        ranges.iter().all(|(_, r)| r.start >= 2_900 || r.is_empty()),
        "{ranges:?}"
    );

    // Shorter, with nothing new to send: the revision alone goes out.
    content.truncate(1_000);
    p.server
        .set(TAIL, Class::Tail, 0, Vec::new(), content.clone())
        .expect("set");
    p.run(3_000);
    let o = p.phone.object(TAIL).expect("tail");
    assert_eq!((o.rev, o.len, o.complete()), (3, 1_000, true));
}

#[test]
fn packets_of_an_older_revision_are_ignored() {
    let mut p = Pair::new();
    p.server
        .set(
            TAIL,
            Class::Tail,
            0,
            Vec::new(),
            b"first version of the text".to_vec(),
        )
        .expect("set");
    let old = loop {
        if let Some(b) = p.server.poll(p.now) {
            break b;
        }
        p.now += 1;
    };
    p.server
        .set(TAIL, Class::Tail, 0, Vec::new(), b"second".to_vec())
        .expect("set");
    p.run(3_000);
    // The old packet comes in late.
    p.deliver_to_phone(&old);
    let o = p.phone.object(TAIL).expect("tail");
    assert_eq!(o.data, b"second");
}

#[test]
fn commands_and_calls_get_one_reply_each() {
    let mut p = Pair::new();
    p.run(100);
    let cmd = p
        .phone
        .command(b"{\"type\":\"thread.turn.start\"}".to_vec());
    let call = p.phone.call(b"{\"method\":\"git.status\"}".to_vec(), false);
    let bg = p
        .phone
        .call(b"{\"method\":\"threads.list\"}".to_vec(), true);
    assert_eq!(p.phone.entry_state(cmd), Some(EntryState::Queued));
    // The server has them; nobody replied yet.
    p.run(500);
    assert_eq!(p.phone.entry_state(cmd), Some(EntryState::ServerHas));
    // The first hello came with a new epoch.
    assert_eq!(p.incoming.first(), Some(&Incoming::Reset));
    p.incoming.remove(0);
    assert_eq!(p.incoming.len(), 3);
    assert!(matches!(
        p.incoming[2],
        Incoming::Call {
            background: true,
            ..
        }
    ));
    p.reply_to_commands();
    p.run(500);
    assert_eq!(p.phone.pending().count(), 0);
    for seq in [cmd, call, bg] {
        assert_eq!(p.phone.entry_state(seq), None);
    }
    // A reconnect sends nothing again that was answered.
    p.phone.connected();
    p.server.connected();
    p.run(1_000);
    assert!(p.incoming.is_empty());
}

#[test]
fn commands_are_applied_in_order_when_the_later_one_arrives_first() {
    let mut p = Pair::new();
    p.run(100);
    let first = p.phone.command(text(5_000));
    let second = p.phone.command(b"short".to_vec());
    let mut packets = Vec::new();
    while packets.len() < 10 {
        if let Some(b) = p.phone.poll(p.now) {
            packets.push(b);
        }
        p.now += 1;
    }
    packets.reverse();
    for b in packets {
        p.deliver_to_server(&b);
    }
    let order: Vec<u64> = p
        .incoming
        .iter()
        .filter_map(|i| match i {
            Incoming::Command { seq, .. } => Some(*seq),
            _ => None,
        })
        .collect();
    assert_eq!(order, vec![first, second]);
}

#[test]
fn a_phone_that_lost_its_cache_gets_everything_again() {
    let mut p = Pair::new();
    p.server
        .set(5, Class::History, 0, Vec::new(), text(5_000))
        .expect("set");
    p.run(100);
    p.phone.command(b"prompt".to_vec());
    p.run(1_000);
    p.reply_to_commands();
    p.run(1_000);
    assert!(p.server.is_synced());
    assert!(p.incoming.is_empty());

    // Same token, empty cache: a new epoch.
    p.phone = Phone::new(TOKEN, 8, Config::phone(), p.now);
    p.server.connected();
    p.run(3_000);
    assert_eq!(p.incoming, vec![Incoming::Reset]);
    assert_eq!(p.phone.object(5).map(|o| o.complete()), Some(true));
}

#[test]
fn a_restarted_server_does_not_send_again_what_the_phone_has() {
    let mut p = Pair::new();
    p.server
        .set(5, Class::History, 0, Vec::new(), text(20_000))
        .expect("set");
    p.server
        .set(TAIL, Class::Tail, 1, Vec::new(), text(500))
        .expect("set");
    p.run(3_000);
    assert!(p.server.is_synced());

    let saved = p.server.save();
    p.server = Server::load(&saved, Config::server(), p.now).expect("loads");
    p.server.connected();
    p.phone.connected();
    let mut tail = text(500);
    tail.extend_from_slice(b" more");
    p.server
        .set(TAIL, Class::Tail, 1, Vec::new(), tail.clone())
        .expect("set");
    let sent = p.run(3_000);
    let ranges = chunk_ranges(&sent);
    assert!(!ranges.is_empty());
    assert!(
        ranges
            .iter()
            .all(|(k, r)| *k == Key::Object(TAIL) && r.start >= 500),
        "{ranges:?}"
    );
    assert!(p.phone.object(TAIL).is_some_and(|o| o.data == tail));
}

#[test]
fn the_phone_reloads_from_what_it_saved() {
    let mut p = Pair::new();
    p.server
        .set(5, Class::History, 0, Vec::new(), text(3_000))
        .expect("set");
    p.phone.command(b"queued".to_vec());
    let mut rows = Vec::new();
    while !p.server.is_synced() {
        if let Some(b) = p.server.poll(p.now) {
            let r = p.phone.receive(p.now, &b).expect("decodes");
            for k in &r.changed {
                rows.push(p.phone.save_object(*k).expect("row"));
            }
            p.phone.committed(r.batch);
        }
        while let Some(b) = p.phone.poll(p.now) {
            p.deliver_to_server(&b);
        }
        p.now += 1;
    }
    let state = p.phone.save_state();
    let back = Phone::load(
        &state,
        rows.iter().map(Vec::as_slice),
        Config::phone(),
        p.now,
    )
    .expect("loads");
    assert!(
        back.object(5)
            .is_some_and(|o| o.complete() && o.data == text(3_000)),
        "{} rows",
        rows.len()
    );
    assert_eq!(back.pending().count(), 1);
}

#[test]
fn once_synced_only_the_alive_signal_goes_out() {
    let mut p = Pair::new();
    p.server
        .set(5, Class::History, 0, Vec::new(), text(50_000))
        .expect("set");
    p.run(5_000);
    assert!(p.server.is_synced());
    let sent = p.run(10_000);
    assert!(
        (9..=11).contains(&sent.len()),
        "{} packets in 10 s",
        sent.len()
    );
    for b in &sent {
        assert!(b.len() < 20, "alive packet of {} bytes", b.len());
    }
}

#[test]
fn an_unsynced_sender_uses_its_whole_rate() {
    let mut s = Server::new(TOKEN, Config::server(), 0);
    s.set(5, Class::Background, 0, Vec::new(), text(1_000_000))
        .expect("set");
    // No acks come back; the rate stays where it starts.
    let rate = s.stats().send_rate;
    let mut bytes = 0;
    for now in 0..10_000 {
        while let Some(b) = s.poll(now) {
            assert!(b.len() <= MAX_PACKET);
            bytes += b.len() + sync_core::WIRE_OVERHEAD;
        }
    }
    let used = bytes as f64 / 10.0 / rate;
    assert!(used > 0.95, "used {used:.2} of the rate");
}

#[test]
fn chunks_on_the_wire_are_compressed_or_raw() {
    let mut p = Pair::new();
    p.server
        .set(5, Class::History, 0, Vec::new(), text(10_000))
        .expect("set");
    let packet = p.next_server_packet();
    let c = packet.chunks.first().expect("chunk");
    // Decoding hands over raw bytes.
    assert!(matches!(c.body, Body::Raw(_)));
}

#[test]
fn a_reply_lost_before_storage_comes_again_after_a_reload() {
    let mut p = Pair::new();
    p.phone.command(b"prompt".to_vec());
    while p
        .incoming
        .iter()
        .all(|i| !matches!(i, Incoming::Command { .. }))
    {
        p.run(1);
    }
    // Saved with the command still in the log.
    let saved = p.phone.save_state();
    p.reply_to_commands();
    // The reply arrives, but the page dies before storage commits it. The
    // phone's packets in between must not let the server forget it.
    let mut got_reply = false;
    while !got_reply {
        if let Some(b) = p.server.poll(p.now) {
            let r = p.phone.receive(p.now, &b).expect("decodes");
            got_reply = !r.replies.is_empty();
        }
        p.now += 1;
    }
    // Past the phone's alive interval, so it sends at least one packet.
    let mut sent = 0;
    for _ in 0..3_000 {
        while let Some(b) = p.phone.poll(p.now) {
            p.deliver_to_server(&b);
            sent += 1;
        }
        p.now += 1;
    }
    assert!(sent > 0);
    p.phone = Phone::load(&saved, [], Config::phone(), p.now).expect("loads");
    p.phone.connected();
    p.server.connected();
    p.run(3_000);
    assert_eq!(p.phone.pending().count(), 0, "the reply never came again");
    assert!(
        p.incoming.is_empty(),
        "the command ran twice: {:?}",
        p.incoming
    );
}

#[test]
fn an_object_with_the_largest_meta_still_goes_out() {
    let mut p = Pair::new();
    let meta = vec![b'm'; sync_core::MAX_META];
    p.server
        .set(TAIL, Class::Tail, 0, meta.clone(), b"hello".to_vec())
        .expect("set");
    p.run(3_000);
    assert!(
        p.phone
            .object(TAIL)
            .is_some_and(|o| o.complete() && o.meta == meta)
    );
    let too_big = vec![b'm'; sync_core::MAX_META + 1];
    assert_eq!(
        p.server.set(TAIL, Class::Tail, 0, too_big, Vec::new()),
        Err(sync_core::Error::MetaTooLarge(sync_core::MAX_META + 1))
    );
}

#[test]
fn a_packet_number_beyond_the_limit_is_refused() {
    use sync_core::wire::{Chunk, MAX_SEQ};
    let packet = |seq| {
        Packet {
            seq,
            chunks: vec![Chunk {
                key: Key::Entry(1),
                rev: 1,
                total_len: 0,
                offset: 0,
                keep: None,
                meta: Some(vec![0, 0]),
                body: Body::Raw(Vec::new()),
            }],
            ..Packet::default()
        }
        .encode()
    };
    let mut s = Server::new(TOKEN, Config::server(), 0);
    assert!(s.receive(0, &packet(u64::MAX)).is_err());
    assert!(s.receive(0, &packet(MAX_SEQ + 1)).is_err());
    assert!(s.receive(0, &packet(MAX_SEQ)).is_ok());
    assert!(s.poll(0).is_some());
}

#[test]
fn a_restarted_server_hands_over_again_what_it_never_answered() {
    let mut p = Pair::new();
    let seq = p.phone.command(b"prompt".to_vec());
    while p
        .incoming
        .iter()
        .all(|i| !matches!(i, Incoming::Command { .. }))
    {
        p.run(1);
    }
    p.incoming.clear();
    // The sidecar dies before it answers.
    let saved = p.server.save();
    p.server = Server::load(&saved, Config::server(), p.now).expect("loads");
    p.server.connected();
    p.phone.connected();
    p.run(2_000);
    assert!(matches!(p.incoming.as_slice(), [Incoming::Command { seq: s, .. }] if *s == seq));
    p.reply_to_commands();
    p.run(2_000);
    assert_eq!(p.phone.pending().count(), 0);
}

#[test]
fn a_tail_with_the_largest_meta_overtakes_backfill_that_fits() {
    let mut s = Server::new(TOKEN, Config::server(), 0);
    s.set(100, Class::Background, 0, Vec::new(), text(20_000))
        .expect("set");
    let meta = vec![b'm'; sync_core::MAX_META];
    s.set(TAIL, Class::Tail, 0, meta, b"hello".to_vec())
        .expect("set");
    let mut now = 0;
    let packet = loop {
        if let Some(b) = s.poll(now) {
            assert!(b.len() <= MAX_PACKET);
            break Packet::decode(&b).expect("decodes");
        }
        now += 1;
    };
    assert_eq!(
        packet.chunks.first().map(|c| c.key),
        Some(Key::Object(TAIL))
    );
}

#[test]
fn extreme_numbers_in_a_report_do_not_crash_the_server() {
    use sync_core::wire::Report;
    let mut s = Server::new(TOKEN, Config::server(), 0);
    for (now, time_ms, max_seq) in [(0, u64::MAX, 0), (1, u64::MAX, u64::MAX), (2, 0, 0)] {
        let p = Packet {
            seq: now,
            report: Some(Report {
                time_ms,
                bytes: u64::MAX,
                packets: u64::MAX,
                max_seq,
            }),
            ..Packet::default()
        };
        assert!(s.receive(now, &p.encode()).is_ok());
        let _ = s.poll(now);
    }
    let st = s.stats();
    assert!(st.loss.is_finite() && st.delivered_rate.is_finite() && st.send_rate.is_finite());
}

#[test]
fn while_a_large_meta_tail_waits_for_its_ack_every_packet_carries_it() {
    let mut s = Server::new(TOKEN, Config::server(), 0);
    let meta = vec![b'm'; sync_core::MAX_META];
    s.set(TAIL, Class::Tail, 0, meta, b"hello".to_vec())
        .expect("set");
    // No acks come back.
    let mut packets = 0;
    for now in 0..900 {
        while let Some(b) = s.poll(now) {
            let p = Packet::decode(&b).expect("decodes");
            assert!(!p.chunks.is_empty(), "an empty packet at {now} ms");
            packets += 1;
        }
    }
    assert!(packets > 1);
}

#[test]
fn a_spare_copy_never_grows_a_packet_that_already_carries_data() {
    let mut s = Server::new(TOKEN, Config::server(), 0);
    // Meta that fits the small packet, and bytes that do not compress.
    let mut x: u64 = 7;
    let data: Vec<u8> = (0..300)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (x >> 56) as u8
        })
        .collect();
    s.set(TAIL, Class::Tail, 0, vec![b'm'; 140], data)
        .expect("set");
    // No acks come back; after the first packet, spare copies go out.
    let mut first = true;
    for now in 0..900 {
        while let Some(b) = s.poll(now) {
            let p = Packet::decode(&b).expect("decodes");
            if !first {
                // A packet grows once, by the cost of a meta that does not fit
                // it (140 bytes and a chunk header); a spare copy never grows
                // it further.
                assert!(
                    b.len() <= 200 + 140 + 48 || p.chunks.len() == 1,
                    "{} bytes in {} chunks at {now} ms",
                    b.len(),
                    p.chunks.len()
                );
            }
            first = false;
        }
    }
}

#[test]
fn a_large_meta_does_not_make_every_packet_full_size() {
    let mut s = Server::new(TOKEN, Config::server(), 0);
    // The sidecar's metas are about 200 bytes; long text waits.
    s.set(TAIL, Class::Tail, 0, vec![b'm'; 200], vec![b'x'; 20_000])
        .expect("set");
    let mut sizes = Vec::new();
    for now in 0..3_000 {
        while let Some(b) = s.poll(now) {
            sizes.push(b.len());
        }
    }
    let largest = sizes.iter().copied().max().unwrap_or(0);
    // At the starting rate a packet holds about 200 bytes, and grows once by
    // what the meta costs.
    assert!(largest <= 200 + 200 + 48, "packets up to {largest} bytes");
}

/// A restarted sidecar lists what the phone has and deletes what the
/// forked server no longer has; a saved and loaded server keeps the list.
#[test]
fn deleting_an_object_the_phone_has() {
    let mut p = Pair::new();
    for id in [10, 11] {
        p.server
            .set(id, Class::History, id, b"{}".to_vec(), text(300))
            .expect("set");
    }
    p.run(2_000);
    assert!(p.phone.object(10).is_some_and(|o| o.complete()));
    let saved = p.server.save();
    p.server = Server::load(&saved, Config::server(), p.now).expect("loads");
    let mut ids = p.server.object_ids();
    ids.sort();
    assert_eq!(ids, vec![10, 11]);
    p.server.delete(11);
    assert_eq!(p.server.object_ids(), vec![10]);
    p.run(2_000);
    // Empty content is a deleted object.
    assert!(
        p.phone
            .object(11)
            .is_some_and(|o| o.complete() && o.len == 0),
        "the phone kept a deleted object"
    );
    assert!(p.phone.object(10).is_some_and(|o| o.complete()));
}

#[test]
fn a_reply_to_a_call_the_owner_made_overtakes_history() {
    let mut p = Pair::new();
    p.run(100);
    for id in 100..140 {
        p.server
            .set(id, Class::History, id, Vec::new(), text(20_000))
            .expect("set");
    }
    let shown = p
        .phone
        .call(b"{\"method\":\"subscribeServerLifecycle\"}".to_vec(), false);
    let background = p.phone.call(
        b"{\"method\":\"server.reportClientActivity\"}".to_vec(),
        true,
    );
    // The server gets both calls while history waits for the phone.
    for _ in 0..50 {
        while let Some(b) = p.phone.poll(p.now) {
            p.deliver_to_server(&b);
        }
        p.now += 1;
    }
    p.incoming.retain(|i| *i != Incoming::Reset);
    assert_eq!(p.incoming.len(), 2);
    p.reply_to_commands();
    let packet = p.next_server_packet();
    let keys: Vec<Key> = packet.chunks.iter().map(|c| c.key).collect();
    assert!(keys.contains(&Key::Reply(shown)), "{keys:?}");
    assert!(!keys.contains(&Key::Reply(background)), "{keys:?}");
}
