//! The deterministic simulator on the lab's two graded cases and on the
//! faults the protocol must survive. Runs are shortened to three minutes
//! here; `examples/sim.rs` runs the full 15 minutes.

use sync_core::MAX_PACKET;
use sync_core::sim::{LinkSpec, Outcome, Scenario, run};

const SECS: u64 = 1000;

fn check(name: &str, o: &Outcome) {
    assert_eq!(o.duplicate_bytes, 0, "{name}: bytes stored twice");
    assert_eq!(o.acked_resent_bytes, 0, "{name}: acked bytes sent again");
    assert_eq!(o.commands_twice, 0, "{name}: command applied twice");
    assert!(
        o.largest_packet <= MAX_PACKET,
        "{name}: packet of {} bytes",
        o.largest_packet
    );
}

fn simulate(name: &str, s: &Scenario) -> Outcome {
    match run(s) {
        Ok(o) => {
            check(name, &o);
            o
        }
        Err(e) => panic!("{name}: {e}"),
    }
}

#[test]
fn case70_prompt_confirmed_within_10_s() {
    for seed in 1..=3 {
        let s = Scenario::case70(seed).shortened(180 * SECS);
        // Also through the model of WebKit's SCTP sender.
        for (name, s) in [("case70", s.clone()), ("case70-webkit", s.in_webkit())] {
            let o = simulate(name, &s);
            let confirmed = o.prompt_confirmed_ms[0];
            assert!(
                confirmed.is_some_and(|t| t <= 10 * SECS),
                "{name} seed {seed}: {confirmed:?}"
            );
        }
    }
}

#[test]
fn case50_newest_text_never_more_than_5_s_behind() {
    for seed in 1..=3 {
        let o = simulate("case50", &Scenario::case50(seed).shortened(180 * SECS));
        assert!(
            o.lag_max_ms <= 5 * SECS,
            "seed {seed}: lag {} ms",
            o.lag_max_ms
        );
        assert!(o.phone_matches, "seed {seed}: phone differs from server");
    }
}

#[test]
fn a_large_clean_resync_probes_past_its_own_delivery_average() {
    let link = LinkSpec {
        delay_ms: 20,
        ..LinkSpec::clean()
    };
    let s = Scenario {
        warmup_ms: 0,
        down: link.clone(),
        up: link,
        backfill_bytes: 2_000_000,
        backfill_lead_ms: 0,
        prompts_at: vec![0],
        reply_len: 1,
        duration_ms: 30_000,
        ..Scenario::case70(1).in_webkit()
    };
    let o = simulate("clean-bulk", &s);
    assert!(o.synced_ms.is_some_and(|t| t < 10_000), "{:?}", o.synced_ms);
    assert!(o.phone_matches);
}

#[test]
fn a_prompt_is_acked_while_a_reply_floods_the_link() {
    // A reply at 2000 characters/s on 10 kbit/s with 70% loss: the text
    // alone fills the link. A second prompt is typed in the middle of it.
    let s = Scenario {
        prompts_at: vec![SECS, 30 * SECS],
        reply_chars_per_sec: 2_000,
        reply_len: 200_000,
        ..Scenario::case70(1).shortened(120 * SECS)
    };
    let o = simulate("flood", &s);
    let second = o.prompt_confirmed_ms[1];
    assert!(second.is_some_and(|t| t <= 10 * SECS), "{second:?}");
}

#[test]
fn killing_the_page_between_receipt_and_storage_loses_nothing() {
    // Storage takes 300 ms, so every kill drops packets received but not
    // stored yet.
    let s = Scenario {
        kills_at: (1..=12).map(|i| i * 5 * SECS).collect(),
        commit_delay_ms: 300,
        ..Scenario::case50(1).shortened(120 * SECS)
    };
    let o = simulate("kills", &s);
    assert!(o.phone_matches, "phone differs from server");
    assert!(o.synced_ms.is_some(), "never synced");
    assert!(o.prompt_confirmed_ms[0].is_some(), "prompt never confirmed");
}

#[test]
fn duplicates_and_reordering_apply_each_command_once() {
    let link = LinkSpec {
        duplicate: 0.2,
        jitter_ms: 200,
        ..LinkSpec::lab(100_000, 0.3, 50)
    };
    let s = Scenario {
        down: link.clone(),
        up: link,
        prompts_at: vec![SECS, 2 * SECS, 3 * SECS, 20 * SECS],
        ..Scenario::case50(1).shortened(90 * SECS)
    };
    let o = simulate("duplicates", &s);
    for (i, p) in o.prompt_confirmed_ms.iter().enumerate() {
        assert!(p.is_some(), "prompt {i} never confirmed");
    }
    assert!(o.phone_matches);
}

#[test]
fn the_send_rate_does_not_decay_at_70_percent_loss() {
    // 100 kbit/s with 70% loss each way, and more backfill than the run
    // can send: the sender always has data waiting.
    let link = LinkSpec::lab(100_000, 0.7, 50);
    let s = Scenario {
        down: link.clone(),
        up: link,
        backfill_bytes: 20_000_000,
        ..Scenario::case70(1).shortened(240 * SECS)
    };
    let o = simulate("loss70", &s);
    let mean = |from: u64, to: u64| {
        let r: Vec<f64> = o
            .trace
            .iter()
            .filter(|(t, _, _)| (from * SECS..to * SECS).contains(t))
            .map(|(_, s, _)| s.send_rate)
            .collect();
        r.iter().sum::<f64>() / r.len() as f64
    };
    let early = mean(30, 90);
    let late = mean(180, 240);
    assert!(
        late >= 0.8 * early,
        "rate fell from {early:.0} to {late:.0} B/s"
    );
    // The link takes 12.5 kB/s after loss; it sends at least that.
    assert!(late >= 12_500.0, "rate {late:.0} B/s");
}

/// Network stories that broke once: each seed is a bug that was fixed.
#[test]
fn stories_that_broke_hold() {
    // 18 and 69: after a gap of more than a minute the server had forgotten
    // what its unacked packets carried, so late acks could not mark their
    // bytes stored, and they went out again.
    for seed in [18, 69] {
        let o = simulate("story", &Scenario::story(seed));
        assert!(
            o.prompt_confirmed_ms[0].is_some(),
            "seed {seed}: prompt never confirmed"
        );
        assert!(o.reply_done_ms.is_some(), "seed {seed}: reply never whole");
    }
}
