//! Random character-based password generation. Uses `rand::thread_rng()`,
//! which is a CSPRNG (ChaCha-based) — appropriate for generating actual
//! credentials, not just a fast general-purpose PRNG.

use rand::Rng;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>?";
/// Characters that are easy to misread in many fonts — 0/O, 1/l/I, etc.
const AMBIGUOUS: &str = "0O1lI|";

#[derive(Debug, Clone, Copy)]
pub struct Charset {
    pub lower: bool,
    pub upper: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

impl Default for Charset {
    fn default() -> Self {
        Self { lower: true, upper: true, digits: true, symbols: true, exclude_ambiguous: false }
    }
}

impl Charset {
    fn pool(&self) -> Vec<char> {
        let mut pool = String::new();
        if self.lower {
            pool.push_str(LOWER);
        }
        if self.upper {
            pool.push_str(UPPER);
        }
        if self.digits {
            pool.push_str(DIGITS);
        }
        if self.symbols {
            pool.push_str(SYMBOLS);
        }
        let mut chars: Vec<char> = pool.chars().collect();
        if self.exclude_ambiguous {
            chars.retain(|c| !AMBIGUOUS.contains(*c));
        }
        chars
    }
}

/// `None` if the charset is empty (nothing selected) or the pool has
/// fewer than 2 distinct characters (entropy would be meaningless).
pub fn generate(length: usize, charset: &Charset) -> Option<String> {
    let pool = charset.pool();
    if pool.len() < 2 || length == 0 {
        return None;
    }
    let mut rng = rand::thread_rng();
    Some((0..length).map(|_| pool[rng.gen_range(0..pool.len())]).collect())
}

/// Bits of entropy for a password drawn uniformly from a pool of this
/// size, this many characters long: `length * log2(pool_size)`.
pub fn entropy_bits(length: usize, charset: &Charset) -> f64 {
    let pool_size = charset.pool().len();
    if pool_size < 2 {
        return 0.0;
    }
    length as f64 * (pool_size as f64).log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_requested_length() {
        let pw = generate(16, &Charset::default()).unwrap();
        assert_eq!(pw.chars().count(), 16);
    }

    #[test]
    fn empty_charset_returns_none() {
        let cs = Charset { lower: false, upper: false, digits: false, symbols: false, exclude_ambiguous: false };
        assert!(generate(16, &cs).is_none());
    }

    #[test]
    fn zero_length_returns_none() {
        assert!(generate(0, &Charset::default()).is_none());
    }

    #[test]
    fn respects_restricted_charset() {
        let cs = Charset { lower: true, upper: false, digits: false, symbols: false, exclude_ambiguous: false };
        let pw = generate(50, &cs).unwrap();
        assert!(pw.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn exclude_ambiguous_removes_flagged_chars() {
        let cs = Charset { lower: false, upper: false, digits: true, symbols: false, exclude_ambiguous: true };
        // digits pool minus ambiguous "0" and "1" leaves 8 chars — generate
        // a long password and confirm none of the excluded chars appear.
        let pw = generate(200, &cs).unwrap();
        assert!(!pw.contains('0'));
        assert!(!pw.contains('1'));
    }

    #[test]
    fn entropy_matches_hand_calculation() {
        // lowercase-only, 26 chars, length 10: 10 * log2(26)
        let cs = Charset { lower: true, upper: false, digits: false, symbols: false, exclude_ambiguous: false };
        let bits = entropy_bits(10, &cs);
        let expected = 10.0 * (26f64).log2();
        assert!((bits - expected).abs() < 0.001);
    }

    #[test]
    fn entropy_scales_with_length() {
        let cs = Charset::default();
        assert!(entropy_bits(20, &cs) > entropy_bits(10, &cs));
    }
}
