//! Turns a bits-of-entropy number into something a human can act on: a
//! qualitative label and a colored visual meter bar, rather than just a
//! raw number nobody has an intuition for.

pub fn label(bits: f64) -> &'static str {
    match bits {
        b if b < 28.0 => "Very Weak",
        b if b < 36.0 => "Weak",
        b if b < 60.0 => "Fair",
        b if b < 128.0 => "Strong",
        _ => "Very Strong",
    }
}

fn color_for(bits: f64) -> String {
    match bits {
        b if b < 28.0 => cybercore::palette::red(),
        b if b < 36.0 => cybercore::palette::orange(),
        b if b < 60.0 => cybercore::palette::hot_pink(),
        b if b < 128.0 => cybercore::palette::cyan(),
        _ => cybercore::palette::acid_green(),
    }
}

/// A `[████░░░░]`-style bar, filled proportionally to `bits` against a
/// 128-bit "effectively maxed out" reference point (128 bits is well
/// beyond any realistic brute-force budget — the bar isn't meant to
/// distinguish "strong" from "overkill," just to visualize the weak end).
pub fn meter(bits: f64, width: usize, color_on: bool) -> String {
    const REFERENCE_MAX: f64 = 128.0;
    let filled = ((bits / REFERENCE_MAX).clamp(0.0, 1.0) * width as f64).round() as usize;
    let empty = width - filled;
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    if color_on {
        format!("{}{}{}", color_for(bits), bar, cybercore::palette::RESET)
    } else {
        bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_entropy_is_very_weak() {
        assert_eq!(label(10.0), "Very Weak");
    }

    #[test]
    fn high_entropy_is_very_strong() {
        assert_eq!(label(150.0), "Very Strong");
    }

    #[test]
    fn boundaries_are_inclusive_on_the_lower_bound() {
        assert_eq!(label(28.0), "Weak"); // exactly at the Weak threshold, not still Very Weak
        assert_eq!(label(128.0), "Very Strong");
    }

    #[test]
    fn meter_is_fully_empty_at_zero_bits() {
        let m = meter(0.0, 10, false);
        assert_eq!(m, format!("[{}]", "░".repeat(10)));
    }

    #[test]
    fn meter_is_fully_filled_at_or_above_reference_max() {
        let m = meter(200.0, 10, false);
        assert_eq!(m, format!("[{}]", "█".repeat(10)));
    }

    #[test]
    fn meter_width_is_always_respected() {
        let m = meter(50.0, 20, false);
        // strip brackets, count the bar characters
        let inner = &m[1..m.len() - 1];
        assert_eq!(inner.chars().count(), 20);
    }
}
