/// Parse an optional fixed-point string into `f64` if a numeric value exists.
///
/// This is to prevent `None` values from being interpreted as `0.0`. This still returns
/// `0.0` for malformed values.
pub fn parse_f64_opt(value: Option<&str>) -> Option<f64> {
    value.and_then(|raw| raw.parse::<f64>().ok())
}

/// Parse an optional fixed-point string into `f64`.
///
/// This is intended for approximate calculations and display logic where the
/// ergonomics of `f64` matter more than exact decimal preservation. Missing or
/// invalid values fall back to `0.0`.
pub fn parse_f64(value: Option<&str>) -> f64 {
    parse_f64_opt(value).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::parse_f64;

    #[test]
    fn parse_f64_accepts_decimal_strings() {
        assert_eq!(parse_f64(Some("123.45")), 123.45);
        assert_eq!(parse_f64(Some("0.00")), 0.0);
    }

    #[test]
    fn parse_f64_defaults_to_zero_for_missing_or_invalid_values() {
        assert_eq!(parse_f64(None), 0.0);
        assert_eq!(parse_f64(Some("not-a-number")), 0.0);
    }
}
