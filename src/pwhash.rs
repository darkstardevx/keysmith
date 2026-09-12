//! Argon2id password hashing — the correct primitive for storing a
//! password (deliberately slow and memory-hard, unlike `hash.rs`'s fast
//! checksums, which are the wrong tool for this). Each call generates a
//! fresh random salt via the OS CSPRNG; the returned PHC string encodes
//! the algorithm, parameters, salt, and hash together, so nothing else
//! needs to be stored alongside it to verify later.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map(|h| h.to_string()).map_err(|e| e.to_string())
}

pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, String> {
    let parsed = PasswordHash::new(phc_hash).map_err(|e| format!("not a valid PHC hash string: {e}"))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_password_verifies() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(verify_password("correct horse battery staple", &hash).unwrap());
    }

    #[test]
    fn wrong_password_does_not_verify() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(!verify_password("wrong password", &hash).unwrap());
    }

    #[test]
    fn same_password_produces_different_hashes_each_time() {
        // Different random salt per call — this is a feature, not a bug;
        // it's what stops identical passwords from producing identical
        // stored hashes.
        let a = hash_password("same input").unwrap();
        let b = hash_password("same input").unwrap();
        assert_ne!(a, b);
        assert!(verify_password("same input", &a).unwrap());
        assert!(verify_password("same input", &b).unwrap());
    }

    #[test]
    fn malformed_hash_string_errors_cleanly_instead_of_panicking() {
        assert!(verify_password("anything", "not a real phc string").is_err());
    }
}
