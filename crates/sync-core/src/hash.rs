//! The one hash of sync-core: SHA-256, cut to 64 bits. The sidecar hashes
//! every id it puts in an object's meta with [`hash_id`], so a meta stays
//! short however long t3's ids are; the web app gets the same function
//! through `phone-wasm` and hashes the id it looks up. The server's outbox
//! keeps [`hash`] of an object's bytes once the phone stored them.

use sha2::{Digest, Sha256};

/// The first 64 bits of the SHA-256 of `bytes`.
pub fn hash(bytes: &[u8]) -> u64 {
    let digest = Sha256::digest(bytes);
    let mut first = [0u8; 8];
    first.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(first)
}

/// An id as it goes in meta: [`hash`] of its UTF-8 bytes as 16 lowercase
/// hex digits.
pub fn hash_id(id: &str) -> String {
    format!("{:016x}", hash(id.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_hash_to_16_hex_digits_whatever_their_length() {
        // The web app's tests check the same value through the WASM build.
        assert_eq!(hash_id("t1"), "628b49d96dcde97a");
        let long = "thread:delegated-task:".to_owned() + &"%25".repeat(500);
        assert_eq!(hash_id(&long).len(), 16);
        assert_ne!(hash_id(&long), hash_id(&long[..long.len() - 1]));
    }
}
