//! The lab's tick: a 25-byte message sent every 100 ms each way, to measure
//! how the pipe delivers tiny messages. The lab's driver encodes the same
//! layout in `lab/driver/pipe.mjs`.
//!
//! Layout: sequence number (u32, big-endian), the sender's Unix time in
//! milliseconds (u64, big-endian), then zeros.

/// A tick's size on the data channel.
pub const SIZE: usize = 25;

/// One tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tick {
    pub seq: u32,
    /// When the sender sent it, in Unix milliseconds.
    pub sent_ms: u64,
}

impl Tick {
    pub fn encode(self) -> [u8; SIZE] {
        let mut out = [0; SIZE];
        out[..4].copy_from_slice(&self.seq.to_be_bytes());
        out[4..12].copy_from_slice(&self.sent_ms.to_be_bytes());
        out
    }

    /// Reads a tick; None when the message is not one.
    pub fn decode(data: &[u8]) -> Option<Tick> {
        if data.len() != SIZE {
            return None;
        }
        Some(Tick {
            seq: u32::from_be_bytes(data[..4].try_into().ok()?),
            sent_ms: u64::from_be_bytes(data[4..12].try_into().ok()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let tick = Tick {
            seq: 0x0102_0304,
            sent_ms: 1_762_500_000_123,
        };
        let data = tick.encode();
        assert_eq!(&data[..4], &[1, 2, 3, 4]);
        assert_eq!(Tick::decode(&data), Some(tick));
    }

    #[test]
    fn rejects_other_sizes() {
        assert_eq!(Tick::decode(&[0; 24]), None);
        assert_eq!(Tick::decode(&[0; 26]), None);
    }
}
