//! Network stories: a seed turns into a few minutes of a 2G-like link that
//! changes every few seconds (issue #58). The lab replays a story with netem
//! (`lab story`, from `examples/story.rs`), the simulator with
//! [`Scenario::story`](super::Scenario::story), so a seed that fails in one
//! can be replayed in the other.
//!
//! The numbers follow issue #15: EDGE carries tens to about 200 kbit/s with
//! a round trip of 0.9 to 2.4 s that can spike to 8 s, deep queues (2 to
//! 10 s at the link's rate, as the lab's profiles B to D), little random
//! loss, losses in short bursts (a Gilbert-Elliott model as netem's
//! `gemodel`), and gaps where nothing gets through, from a second to more
//! than a minute. One story in five is a survival story: bytes per second
//! and 70% loss. The phone keeps its address.

use super::{LinkSpec, Rng};
use crate::Millis;

/// Faults happen in the first this-many milliseconds; then the link comes
/// back as the story's ordinary link until the run ends.
pub const STORY_MS: Millis = 150_000;

/// How long a run on a story lasts after the story starts: the lab's
/// deadline and the simulator's end.
pub const RUN_MS: Millis = 300_000;

/// One stretch of a story.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    /// When it starts, after the story starts.
    pub at_ms: Millis,
    /// What it is, for reports: `2G`, `slow`, `rtt spike`, `bursts`, `gap`,
    /// `survival` or `back`.
    pub name: &'static str,
    /// Server to phone.
    pub down: LinkSpec,
    /// Phone to server.
    pub up: LinkSpec,
}

/// A seed's story.
#[derive(Debug, Clone, PartialEq)]
pub struct Story {
    pub seed: u64,
    pub survival: bool,
    /// In order of `at_ms`, the first at 0. The last lasts until the run
    /// ends.
    pub segments: Vec<Segment>,
}

/// Uniform in [lo, hi).
fn uniform(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    lo + rng.f64() * (hi - lo)
}

/// Log-uniform in [lo, hi): as many small values as large ones per factor.
fn log_uniform(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    (lo.ln() + rng.f64() * (hi.ln() - lo.ln())).exp()
}

/// A link whose queue holds `queue_s` seconds of full packets at its rate.
fn spec(rate_bps: f64, delay_ms: f64, jitter_ms: f64, queue_s: f64, loss: f64) -> LinkSpec {
    let queue = (rate_bps / 8.0 * queue_s / 1_200.0)
        .round()
        .clamp(2.0, 200.0) as usize;
    LinkSpec {
        rate_bps: rate_bps.round().max(1.0) as u64,
        loss,
        delay_ms: delay_ms.round() as u64,
        jitter_ms: jitter_ms.round() as u64,
        queue,
        duplicate: 0.0,
        outages: Vec::new(),
        burst: None,
    }
}

impl Story {
    /// The story of a seed. The same seed gives the same story on every
    /// platform.
    pub fn generate(seed: u64) -> Story {
        let mut rng = Rng::new(seed);
        let survival = rng.f64() < 0.2;
        // The story's ordinary link: an EDGE cell, uplink slower.
        let rate = log_uniform(&mut rng, 40_000.0, 200_000.0);
        let up_share = uniform(&mut rng, 0.4, 0.8);
        let delay = uniform(&mut rng, 450.0, 1_200.0);
        let jitter = uniform(&mut rng, 20.0, 90.0);
        let queue_s = uniform(&mut rng, 2.0, 10.0);
        let ordinary = |rate: f64, delay: f64, loss: f64| {
            (
                spec(rate, delay, jitter, queue_s, loss),
                spec(rate * up_share, delay, jitter, queue_s, loss),
            )
        };

        let mut segments = Vec::new();
        let mut at: Millis = 0;
        while at < STORY_MS {
            let mut len = uniform(&mut rng, 2_000.0, 8_000.0) as Millis;
            let roll = rng.f64();
            let (name, (mut down, mut up)) = if roll < 0.1 {
                // Nothing gets through, for a second up to a minute and a half.
                len = log_uniform(&mut rng, 1_000.0, 90_000.0) as Millis;
                ("gap", ordinary(rate, delay, 1.0))
            } else if survival {
                let slow = log_uniform(&mut rng, 800.0, 8_000.0);
                ("survival", ordinary(slow, delay.max(1_000.0), 0.7))
            } else if roll < 0.2 {
                // Down to tens of kbit/s, sometimes bytes per second.
                let slow = log_uniform(&mut rng, 800.0, 20_000.0);
                ("slow", ordinary(slow, delay, 0.005))
            } else if roll < 0.3 {
                // The round trip spikes, up to 8 s.
                let spike = uniform(&mut rng, 2_000.0, 4_000.0);
                ("rtt spike", ordinary(rate, spike, 0.005))
            } else if roll < 0.5 {
                ("bursts", ordinary(rate, delay, 0.0))
            } else {
                let r = rate * uniform(&mut rng, 0.5, 1.2);
                ("2G", ordinary(r, delay, uniform(&mut rng, 0.0, 0.01)))
            };
            if name == "bursts" {
                // Mean loss p/(p+r): about 3% to 33%, in runs of 3 to 10.
                let p = uniform(&mut rng, 0.01, 0.05);
                let r = uniform(&mut rng, 0.1, 0.3);
                down.burst = Some((p, r));
                up.burst = Some((p, r));
            }
            segments.push(Segment {
                at_ms: at,
                name,
                down,
                up,
            });
            at += len;
        }
        let (down, up) = ordinary(rate, delay, 0.0);
        segments.push(Segment {
            at_ms: at,
            name: "back",
            down,
            up,
        });
        Story {
            seed,
            survival,
            segments,
        }
    }

    /// The story as JSON, for the lab.
    pub fn to_json(&self) -> String {
        let link = |l: &LinkSpec| {
            let burst = l
                .burst
                .map_or("null".to_string(), |(p, r)| format!("[{p:.4},{r:.4}]"));
            format!(
                r#"{{"rateBps":{},"delayMs":{},"jitterMs":{},"queue":{},"loss":{:.4},"burst":{burst}}}"#,
                l.rate_bps, l.delay_ms, l.jitter_ms, l.queue, l.loss
            )
        };
        let segments: Vec<String> = self
            .segments
            .iter()
            .map(|s| {
                format!(
                    r#"{{"atMs":{},"name":"{}","down":{},"up":{}}}"#,
                    s.at_ms,
                    s.name,
                    link(&s.down),
                    link(&s.up)
                )
            })
            .collect();
        format!(
            r#"{{"seed":{},"survival":{},"runMs":{RUN_MS},"segments":[{}]}}"#,
            self.seed,
            self.survival,
            segments.join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_gives_one_story() {
        assert_eq!(Story::generate(7), Story::generate(7));
        assert_ne!(Story::generate(7), Story::generate(8));
    }

    #[test]
    fn stories_look_like_2g() {
        let stories: Vec<Story> = (1..=1_000).map(Story::generate).collect();
        let survival = stories.iter().filter(|s| s.survival).count();
        assert!(
            (150..250).contains(&survival),
            "{survival} survival stories"
        );
        for s in &stories {
            assert_eq!(s.segments[0].at_ms, 0);
            assert!(s.segments.windows(2).all(|w| w[0].at_ms < w[1].at_ms));
            let last = s.segments.last().map(|l| l.at_ms).unwrap_or(0);
            assert!((STORY_MS..RUN_MS).contains(&last), "seed {}", s.seed);
            for seg in &s.segments {
                assert!(
                    (800..=240_000).contains(&seg.down.rate_bps),
                    "seed {}",
                    s.seed
                );
                assert!(seg.down.delay_ms * 2 <= 8_000, "seed {}", s.seed);
            }
        }
        let gaps = stories
            .iter()
            .filter(|s| s.segments.iter().any(|g| g.name == "gap"))
            .count();
        assert!(gaps > 500, "{gaps} stories with a gap");
    }

    #[test]
    fn json_names_every_segment() {
        let s = Story::generate(3);
        let json = s.to_json();
        assert_eq!(json.matches(r#""atMs""#).count(), s.segments.len());
        assert!(json.starts_with(r#"{"seed":3,"#));
    }
}
