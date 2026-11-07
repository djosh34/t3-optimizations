//! ICE credentials derived from a paired phone's token, so neither side
//! needs signaling for a new connection or an ICE restart (issues #24, #27).
//!
//! Each connection of a phone has a generation, which the phone raises for
//! every new `RTCPeerConnection`, and each ICE restart on it a restart
//! number. Both sides compute both sides' credentials from the token, the
//! phone's device id, the generation and the restart number.
//!
//! The username fragment is public, so it carries the device id, side,
//! generation and restart number in clear: the server finds the phone from
//! the first STUN packet of a connection. The password is an HMAC of these
//! under the token, so only the paired phone can pass ICE's integrity check.
//!
//! The web app computes the same values (WebCrypto HMAC-SHA256, standard
//! base64 without padding).

use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use hmac::digest::KeyInit;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use str0m::IceCreds;

/// Which end of the connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The sidecar.
    Server,
    /// The phone.
    Phone,
}

impl Side {
    fn letter(self) -> char {
        match self {
            Side::Server => 's',
            Side::Phone => 'p',
        }
    }
}

/// What a username fragment names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ufrag {
    pub device: String,
    pub side: Side,
    pub generation: u64,
    pub restart: u64,
}

impl Ufrag {
    /// Reads a fragment made by [`derive`]; None for any other.
    pub fn parse(s: &str) -> Option<Ufrag> {
        let mut parts = s.split('+');
        let device = parts.next()?;
        let side_gen = parts.next()?;
        let restart = parts.next()?.parse().ok()?;
        if parts.next().is_some() || !valid_device(device) {
            return None;
        }
        let side = match side_gen.chars().next()? {
            's' => Side::Server,
            'p' => Side::Phone,
            _ => return None,
        };
        let generation = side_gen[1..].parse().ok()?;
        Some(Ufrag {
            device: device.to_owned(),
            side,
            generation,
            restart,
        })
    }
}

/// A device id: 4 to 16 lowercase letters and digits, chosen by the sidecar
/// at pairing.
pub fn valid_device(device: &str) -> bool {
    (4..=16).contains(&device.len())
        && device
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
}

/// The credentials of `side` for connection `generation`, restart `restart`.
pub fn derive(
    token: &[u8; 16],
    device: &str,
    side: Side,
    generation: u64,
    restart: u64,
) -> IceCreds {
    let ufrag = format!("{device}+{}{generation}+{restart}", side.letter());
    // HMAC pads a short key with zeros to the block size; padding it here
    // gives the same MAC without a fallible constructor.
    let mut key = [0u8; 64];
    key[..16].copy_from_slice(token);
    let mut mac = <Hmac<Sha256> as KeyInit>::new(&key.into());
    mac.update(b"t3opt ice ");
    mac.update(ufrag.as_bytes());
    let pass = STANDARD_NO_PAD.encode(mac.finalize().into_bytes());
    // 24 of base64's characters, all valid ICE characters (RFC 8839 asks
    // for at least 22).
    IceCreds {
        ufrag,
        pass: pass[..24].to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: [u8; 16] = [7; 16];

    #[test]
    fn round_trip() {
        let c = derive(&TOKEN, "abcd1234", Side::Phone, 12, 3);
        assert_eq!(c.ufrag, "abcd1234+p12+3");
        assert_eq!(c.pass.len(), 24);
        assert_eq!(
            Ufrag::parse(&c.ufrag),
            Some(Ufrag {
                device: "abcd1234".into(),
                side: Side::Phone,
                generation: 12,
                restart: 3
            })
        );
    }

    #[test]
    fn passwords_differ_by_every_input() {
        let base = derive(&TOKEN, "abcd1234", Side::Server, 1, 0).pass;
        assert_ne!(base, derive(&[8; 16], "abcd1234", Side::Server, 1, 0).pass);
        assert_ne!(base, derive(&TOKEN, "abcd1235", Side::Server, 1, 0).pass);
        assert_ne!(base, derive(&TOKEN, "abcd1234", Side::Phone, 1, 0).pass);
        assert_ne!(base, derive(&TOKEN, "abcd1234", Side::Server, 2, 0).pass);
        assert_ne!(base, derive(&TOKEN, "abcd1234", Side::Server, 1, 1).pass);
    }

    /// The web app computes the same value; this pins it.
    #[test]
    fn known_value() {
        let c = derive(&[0; 16], "dev0", Side::Server, 1, 0);
        assert_eq!(c.ufrag, "dev0+s1+0");
        assert_eq!(c.pass, KNOWN_PASS);
    }

    /// From Node: HMAC-SHA256 with 16 zero bytes as key over
    /// "t3opt ice dev0+s1+0", base64 without padding, first 24 characters.
    const KNOWN_PASS: &str = "ZHKLNYpMayxN6WM1nVghvbU+";

    #[test]
    fn rejects_other_fragments() {
        for s in [
            "",
            "abcd",
            "abcd+s1",
            "abcd+x1+0",
            "ab+s1+0",
            "ABCD+s1+0",
            "abcd+s1+0+1",
            "abcd+s+0",
            "abcd+s1+-1",
        ] {
            assert_eq!(Ufrag::parse(s), None, "{s}");
        }
    }
}
