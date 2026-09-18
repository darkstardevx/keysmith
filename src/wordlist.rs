//! The EFF long wordlist (7776 words = 6^5, the standard diceware word
//! count) embedded at compile time — same `include_str!` pattern
//! `cybercore` uses for its theme data. Source:
//! <https://www.eff.org/dice> ("EFF Large Wordlist"), public domain.

const RAW: &str = include_str!("wordlist.txt");

pub fn words() -> Vec<&'static str> {
    RAW.lines().filter(|l| !l.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_the_full_diceware_word_count() {
        assert_eq!(
            words().len(),
            7776,
            "EFF large wordlist should be exactly 6^5 words"
        );
    }

    #[test]
    fn no_duplicate_words() {
        let w = words();
        let unique: std::collections::HashSet<_> = w.iter().collect();
        assert_eq!(w.len(), unique.len(), "wordlist should have no duplicates");
    }

    #[test]
    fn all_words_are_lowercase_ascii_letters_or_hyphens() {
        // The real EFF list includes a handful of legitimate hyphenated
        // compounds ("t-shirt", "yo-yo", "drop-down", "felt-tip") — found
        // by this exact assertion failing against the real embedded data
        // rather than a synthetic guess at the format, so the fix is to
        // allow hyphens, not to assume the fetched list was corrupted.
        assert!(words()
            .iter()
            .all(|w| w.chars().all(|c| c.is_ascii_lowercase() || c == '-')));
    }
}
