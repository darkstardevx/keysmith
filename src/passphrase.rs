//! Diceware-style passphrase generation — several random words from a
//! fixed wordlist joined together, rather than a string of random
//! characters. Genuinely more memorable per bit of entropy for a human,
//! which is the entire point of diceware as a method.

use rand::Rng;

pub fn generate(
    word_count: usize,
    separator: &str,
    capitalize: bool,
    wordlist: &[&str],
) -> Option<String> {
    if word_count == 0 || wordlist.is_empty() {
        return None;
    }
    let mut rng = rand::thread_rng();
    let words: Vec<String> = (0..word_count)
        .map(|_| {
            let w = wordlist[rng.gen_range(0..wordlist.len())];
            if capitalize {
                capitalize_first(w)
            } else {
                w.to_string()
            }
        })
        .collect();
    Some(words.join(separator))
}

fn capitalize_first(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Bits of entropy for `word_count` words drawn uniformly from a wordlist
/// of `wordlist_size` — `word_count * log2(wordlist_size)`. For the
/// standard EFF large wordlist (7776 words), that's ~12.925 bits/word.
pub fn entropy_bits(word_count: usize, wordlist_size: usize) -> f64 {
    if wordlist_size < 2 {
        return 0.0;
    }
    word_count as f64 * (wordlist_size as f64).log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wordlist() -> Vec<&'static str> {
        vec!["apple", "banana", "cherry", "date"]
    }

    #[test]
    fn generates_requested_word_count() {
        let p = generate(5, "-", false, &sample_wordlist()).unwrap();
        assert_eq!(p.split('-').count(), 5);
    }

    #[test]
    fn zero_words_returns_none() {
        assert!(generate(0, "-", false, &sample_wordlist()).is_none());
    }

    #[test]
    fn empty_wordlist_returns_none() {
        assert!(generate(3, "-", false, &[]).is_none());
    }

    #[test]
    fn capitalize_uppercases_first_letter_of_each_word() {
        let p = generate(3, "-", true, &sample_wordlist()).unwrap();
        for word in p.split('-') {
            let first = word.chars().next().unwrap();
            assert!(first.is_uppercase(), "{word} should start uppercase");
        }
    }

    #[test]
    fn uses_the_given_separator() {
        let p = generate(3, "_", false, &sample_wordlist()).unwrap();
        assert_eq!(p.split('_').count(), 3);
    }

    #[test]
    fn entropy_matches_hand_calculation_for_eff_wordlist_size() {
        let bits = entropy_bits(6, 7776);
        let expected = 6.0 * (7776f64).log2();
        assert!((bits - expected).abs() < 0.001);
    }
}
