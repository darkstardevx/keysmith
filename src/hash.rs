//! Checksums, not credential storage — SHA-256/SHA-512/BLAKE3 side by
//! side, for verifying file/text integrity. Deliberately separate from
//! `pwhash.rs`: fast general-purpose hashes are the *wrong* primitive for
//! storing a password (that's what Argon2id in `pwhash.rs` is for) —
//! conflating the two in one command would be a real footgun.

use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Clone)]
pub struct HashResult {
    pub sha256: String,
    pub sha512: String,
    pub blake3: String,
}

pub fn hash_bytes(data: &[u8]) -> HashResult {
    HashResult {
        sha256: format!("{:x}", Sha256::digest(data)),
        sha512: format!("{:x}", Sha512::digest(data)),
        blake3: blake3::hash(data).to_hex().to_string(),
    }
}

pub fn hash_file(path: &std::path::Path) -> std::io::Result<HashResult> {
    let data = std::fs::read(path)?;
    Ok(hash_bytes(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_sha256_vector() {
        // sha256("abc") — a standard published test vector.
        let result = hash_bytes(b"abc");
        assert_eq!(
            result.sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn known_sha512_vector() {
        let result = hash_bytes(b"abc");
        // Verified independently via `printf 'abc' | sha512sum`, not
        // hand-transcribed from memory — a prior version of this test had
        // the expected literal one character short (a dropped trailing
        // "f") and was comparing against a value that was *never* right,
        // rather than actually confirming the implementation.
        assert_eq!(
            result.sha512.len(),
            128,
            "SHA-512 hex output should be exactly 128 chars (64 bytes)"
        );
        assert_eq!(
            result.sha512,
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn different_input_produces_different_hash() {
        let a = hash_bytes(b"hello");
        let b = hash_bytes(b"world");
        assert_ne!(a.sha256, b.sha256);
        assert_ne!(a.blake3, b.blake3);
    }

    #[test]
    fn same_input_is_deterministic() {
        let a = hash_bytes(b"consistent");
        let b = hash_bytes(b"consistent");
        assert_eq!(a.sha256, b.sha256);
        assert_eq!(a.blake3, b.blake3);
    }
}
