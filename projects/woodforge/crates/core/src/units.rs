use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnitSystem {
    Imperial,
    Metric,
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::Imperial
    }
}

pub fn inches_to_mm(inches: f64) -> f64 {
    inches * 25.4
}

pub fn mm_to_inches(mm: f64) -> f64 {
    mm / 25.4
}

pub fn inches_to_feet(inches: f64) -> f64 {
    inches / 12.0
}

pub fn feet_to_inches(feet: f64) -> f64 {
    feet * 12.0
}

/// Format a dimension (in inches) for display in the given unit system
pub fn format_dimension(inches: f64, unit_system: UnitSystem) -> String {
    match unit_system {
        UnitSystem::Imperial => format_imperial(inches),
        UnitSystem::Metric => format_metric(inches),
    }
}

fn format_imperial(inches: f64) -> String {
    if inches >= 12.0 {
        let feet = (inches / 12.0).floor() as i64;
        let remaining = inches - (feet as f64 * 12.0);
        if remaining.abs() < 1.0 / 128.0 {
            return format!("{}'", feet);
        }
        return format!("{}' {}", feet, format_fractional_inches(remaining));
    }
    format_fractional_inches(inches)
}

fn format_fractional_inches(inches: f64) -> String {
    let whole = inches.floor() as i64;
    let frac = inches - whole as f64;

    // Check common fractions (1/16 precision)
    let sixteenths = (frac * 16.0).round() as i64;

    if sixteenths == 0 {
        return format!("{}\"", whole);
    }
    if sixteenths == 16 {
        return format!("{}\"", whole + 1);
    }

    let (num, den) = simplify_fraction(sixteenths, 16);

    if whole == 0 {
        format!("{}/{}\"", num, den)
    } else {
        format!("{}-{}/{}\"", whole, num, den)
    }
}

fn simplify_fraction(mut num: i64, mut den: i64) -> (i64, i64) {
    while num % 2 == 0 && den % 2 == 0 {
        num /= 2;
        den /= 2;
    }
    (num, den)
}

fn format_metric(inches: f64) -> String {
    let mm = inches_to_mm(inches);
    if mm >= 1000.0 {
        format!("{:.1} m", mm / 1000.0)
    } else {
        format!("{:.1} mm", mm)
    }
}

/// Parse a dimension string to inches. Accepts various formats:
/// "3.5", "3-1/2\"", "3-1/2", "88.9mm", "8'", "8' 6\"", "2ft 6in"
pub fn parse_dimension(input: &str) -> Option<f64> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    // Try mm
    if let Some(mm_str) = input.strip_suffix("mm") {
        return mm_str.trim().parse::<f64>().ok().map(mm_to_inches);
    }

    // Try meters
    if let Some(m_str) = input.strip_suffix('m') {
        if !m_str.ends_with('m') {
            return m_str.trim().parse::<f64>().ok().map(|m| mm_to_inches(m * 1000.0));
        }
    }

    // Try feet + inches: 8' 6" or 8'6"
    if input.contains('\'') {
        let parts: Vec<&str> = input.split('\'').collect();
        let feet: f64 = parts[0].trim().parse().ok()?;
        let mut total = feet_to_inches(feet);
        if parts.len() > 1 {
            let inch_str = parts[1].trim().trim_end_matches('"').trim();
            if !inch_str.is_empty() {
                total += parse_inches_value(inch_str)?;
            }
        }
        return Some(total);
    }

    // Try "Xft Yin"
    if input.contains("ft") {
        let parts: Vec<&str> = input.split("ft").collect();
        let feet: f64 = parts[0].trim().parse().ok()?;
        let mut total = feet_to_inches(feet);
        if parts.len() > 1 {
            let inch_str = parts[1].trim().trim_end_matches("in").trim();
            if !inch_str.is_empty() {
                total += parse_inches_value(inch_str)?;
            }
        }
        return Some(total);
    }

    // Plain inches (with optional " suffix)
    let inch_str = input.trim_end_matches('"').trim();
    parse_inches_value(inch_str)
}

/// Parse an inches value that may contain fractions: "3-1/2", "3.5", "1/2"
fn parse_inches_value(s: &str) -> Option<f64> {
    let s = s.trim();

    // Check for fractional notation: "3-1/2" or "1/2"
    if s.contains('/') {
        if let Some((whole_str, frac_str)) = s.split_once('-') {
            let whole: f64 = whole_str.trim().parse().ok()?;
            let frac = parse_fraction(frac_str.trim())?;
            return Some(whole + frac);
        }
        // Just a fraction like "1/2"
        return parse_fraction(s);
    }

    // Plain decimal
    s.parse::<f64>().ok()
}

fn parse_fraction(s: &str) -> Option<f64> {
    let (num_str, den_str) = s.split_once('/')?;
    let num: f64 = num_str.trim().parse().ok()?;
    let den: f64 = den_str.trim().parse().ok()?;
    if den == 0.0 {
        return None;
    }
    Some(num / den)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inch_mm_roundtrip() {
        let inches = 3.5;
        let mm = inches_to_mm(inches);
        assert!((mm - 88.9).abs() < 0.001);
        assert!((mm_to_inches(mm) - inches).abs() < 0.0001);
    }

    #[test]
    fn test_format_imperial_fractions() {
        assert_eq!(format_dimension(3.5, UnitSystem::Imperial), "3-1/2\"");
        assert_eq!(format_dimension(1.5, UnitSystem::Imperial), "1-1/2\"");
        assert_eq!(format_dimension(0.75, UnitSystem::Imperial), "3/4\"");
        assert_eq!(format_dimension(4.0, UnitSystem::Imperial), "4\"");
    }

    #[test]
    fn test_format_imperial_feet() {
        assert_eq!(format_dimension(96.0, UnitSystem::Imperial), "8'");
        assert_eq!(format_dimension(120.0, UnitSystem::Imperial), "10'");
        assert_eq!(format_dimension(102.0, UnitSystem::Imperial), "8' 6\"");
    }

    #[test]
    fn test_format_metric() {
        assert_eq!(format_dimension(1.0, UnitSystem::Metric), "25.4 mm");
        assert_eq!(format_dimension(3.5, UnitSystem::Metric), "88.9 mm");
    }

    #[test]
    fn test_standard_lumber_dimensions() {
        // 2x4 actual: 1.5" × 3.5"
        assert_eq!(format_dimension(1.5, UnitSystem::Imperial), "1-1/2\"");
        assert_eq!(format_dimension(3.5, UnitSystem::Imperial), "3-1/2\"");
        // 2x8 actual height: 7.25"
        assert_eq!(format_dimension(7.25, UnitSystem::Imperial), "7-1/4\"");
        // 2x10 actual height: 9.25"
        assert_eq!(format_dimension(9.25, UnitSystem::Imperial), "9-1/4\"");
        // 2x12 actual height: 11.25"
        assert_eq!(format_dimension(11.25, UnitSystem::Imperial), "11-1/4\"");
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_dimension("3.5"), Some(3.5));
        assert_eq!(parse_dimension("3.5\""), Some(3.5));
    }

    #[test]
    fn test_parse_fractional() {
        assert_eq!(parse_dimension("3-1/2\""), Some(3.5));
        assert_eq!(parse_dimension("3-1/2"), Some(3.5));
        assert_eq!(parse_dimension("1/2\""), Some(0.5));
    }

    #[test]
    fn test_parse_feet() {
        assert_eq!(parse_dimension("8'"), Some(96.0));
        assert_eq!(parse_dimension("8' 6\""), Some(102.0));
    }

    #[test]
    fn test_parse_metric() {
        let result = parse_dimension("88.9mm").unwrap();
        assert!((result - 3.5).abs() < 0.001);
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse_dimension(""), None);
        assert_eq!(parse_dimension("  "), None);
    }
}
