//! Human-readable number and duration formatting for Cookie-CLI.

const SHORT_SUFFIXES: &[&str] = &[
    "", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "Dc",
];

const LONG_NAMES: &[&str] = &[
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];

/// Formats a large number using short-scale suffixes (`K`, `M`, `B`, ..., `Dc`)
/// and falls back to scientific notation at or above `1e36`.
pub fn format_number(val: f64) -> String {
    if val.is_nan() {
        return "0".to_string();
    }
    if val.is_infinite() {
        return if val.is_sign_negative() {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        };
    }
    if val < 0.0 {
        return format!("-{}", format_number(-val));
    }
    if val < 1_000.0 {
        return format!("{}", val.floor() as u64);
    }

    let exp = (val.log10() / 3.0).floor() as usize;
    if exp >= SHORT_SUFFIXES.len() {
        return format!("{val:.2e}");
    }

    let scaled = val / 10_f64.powi((exp as i32) * 3);
    let suffix = SHORT_SUFFIXES[exp];
    format!("{scaled:.2} {suffix}")
}

/// Formats a rate (such as CPS or CPC), showing one decimal place for small non-integers.
pub fn format_rate(val: f64) -> String {
    if val.is_nan() {
        return "0".to_string();
    }
    if val.is_infinite() {
        return "Infinity".to_string();
    }
    if val < 0.0 {
        return format!("-{}", format_rate(-val));
    }
    if val < 1_000.0 {
        let rounded = (val * 10.0).round() / 10.0;
        if (rounded - rounded.round()).abs() < 1e-6 {
            return format!("{}", rounded.round() as u64);
        }
        return format!("{rounded:.1}");
    }
    format_number(val)
}

/// Formats a number using full English short-scale names (`million`, `billion`, ..., `decillion`).
pub fn format_long(val: f64) -> String {
    if val.is_nan() {
        return "0".to_string();
    }
    if val.is_infinite() {
        return "Infinity".to_string();
    }
    if val < 0.0 {
        return format!("-{}", format_long(-val));
    }
    if val < 1_000.0 {
        return format!("{}", val.floor() as u64);
    }

    let exp = (val.log10() / 3.0).floor() as usize;
    if exp >= LONG_NAMES.len() {
        return format!("{val:.3e}");
    }

    let scaled = val / 10_f64.powi((exp as i32) * 3);
    let name = LONG_NAMES[exp];
    format!("{scaled:.2} {name}")
}

/// Formats a duration in seconds into a compact human-readable string (`1h 02m 03s`).
pub fn format_duration(seconds: f64) -> String {
    if !seconds.is_finite() || seconds <= 0.0 {
        return "0s".to_string();
    }
    let total_secs = seconds.floor() as u64;
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let mins = (total_secs % 3_600) / 60;
    let secs = total_secs % 60;

    if days > 0 {
        format!("{days}d {hours:02}h {mins:02}m")
    } else if hours > 0 {
        format!("{hours}h {mins:02}m {secs:02}s")
    } else if mins > 0 {
        format!("{mins}m {secs:02}s")
    } else {
        format!("{secs}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(0.9), "0");
        assert_eq!(format_number(42.7), "42");
        assert_eq!(format_number(999.9), "999");
    }

    #[test]
    fn test_format_number_suffixes() {
        assert_eq!(format_number(1_000.0), "1.00 K");
        assert_eq!(format_number(1_500.0), "1.50 K");
        assert_eq!(format_number(1_234_567.0), "1.23 M");
        assert_eq!(format_number(1e9), "1.00 B");
        assert_eq!(format_number(1e12), "1.00 T");
        assert_eq!(format_number(1e15), "1.00 Qa");
        assert_eq!(format_number(1e18), "1.00 Qi");
        assert_eq!(format_number(1e21), "1.00 Sx");
        assert_eq!(format_number(1e24), "1.00 Sp");
        assert_eq!(format_number(1e27), "1.00 Oc");
        assert_eq!(format_number(1e30), "1.00 No");
        assert_eq!(format_number(1e33), "1.00 Dc");
    }

    #[test]
    fn test_format_number_scientific_and_edge_cases() {
        assert_eq!(format_number(1e36), "1.00e36");
        assert_eq!(format_number(f64::NAN), "0");
        assert_eq!(format_number(f64::INFINITY), "Infinity");
        assert_eq!(format_number(-1_500.0), "-1.50 K");
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(0.0), "0");
        assert_eq!(format_rate(0.1), "0.1");
        assert_eq!(format_rate(1.5), "1.5");
        assert_eq!(format_rate(10.0), "10");
        assert_eq!(format_rate(2_500.0), "2.50 K");
    }

    #[test]
    fn test_format_long() {
        assert_eq!(format_long(500.0), "500");
        assert_eq!(format_long(1_000_000.0), "1.00 million");
        assert_eq!(format_long(2.5e12), "2.50 trillion");
        assert_eq!(format_long(1e33), "1.00 decillion");
        assert_eq!(format_long(1e36), "1.000e36");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "0s");
        assert_eq!(format_duration(45.9), "45s");
        assert_eq!(format_duration(125.0), "2m 05s");
        assert_eq!(format_duration(3_661.0), "1h 01m 01s");
        assert_eq!(format_duration(90_061.0), "1d 01h 01m");
    }
}
