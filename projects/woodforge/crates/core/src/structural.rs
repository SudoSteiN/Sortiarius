use serde::{Deserialize, Serialize};

/// Load status indicator
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LoadStatus {
    /// Under 60% capacity -- safe
    Green,
    /// 60-85% capacity -- caution
    Yellow,
    /// Over 85% capacity -- danger
    Red,
}

/// Result of beam deflection analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeflectionResult {
    pub deflection_inches: f64,
    /// L/360 for shelves, L/240 for floors
    pub max_allowable: f64,
    /// deflection / max_allowable (0-1+)
    pub ratio: f64,
    pub status: LoadStatus,
    /// Actual span-to-deflection ratio, e.g. "L/480"
    pub span_ratio: String,
}

/// Result of compressive capacity analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompressionResult {
    pub applied_load_lbs: f64,
    pub capacity_lbs: f64,
    /// applied / capacity (0-1+)
    pub utilization: f64,
    pub status: LoadStatus,
    /// True if slenderness ratio > 50
    pub buckling_risk: bool,
}

/// Result of joint strength analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JointStrengthResult {
    /// 0-100 from joint type
    pub base_strength: u32,
    /// Multiplier based on wood hardness/density
    pub species_factor: f64,
    /// base * species_factor, capped at 100
    pub adjusted_strength: u32,
    pub status: LoadStatus,
    pub recommendation: String,
}

/// Beam deflection calculator for shelves/spans.
///
/// Uses the standard engineering formula for uniformly distributed load:
///   delta = 5 * w * L^4 / (384 * E * I)
///
/// where:
///   w = distributed load (lb/in)
///   L = span length (inches)
///   E = modulus of elasticity (psi)
///   I = moment of inertia (in^4) = b * h^3 / 12
///
/// # Parameters
/// - `span_length`: length of unsupported span in inches
/// - `width`: board width perpendicular to load (inches) -- this is 'b'
/// - `height`: board height in direction of load (inches) -- this is 'h'
/// - `moe`: modulus of elasticity (psi)
/// - `load_per_linear_inch`: distributed load in lb/in
/// - `deflection_limit`: denominator for allowable deflection (e.g. 360.0 for L/360)
pub fn calculate_beam_deflection(
    span_length: f64,
    width: f64,
    height: f64,
    moe: f64,
    load_per_linear_inch: f64,
    deflection_limit: f64,
) -> DeflectionResult {
    // Moment of inertia: I = b * h^3 / 12
    let moment_of_inertia = width * height.powi(3) / 12.0;

    // Deflection: delta = 5 * w * L^4 / (384 * E * I)
    let deflection =
        5.0 * load_per_linear_inch * span_length.powi(4) / (384.0 * moe * moment_of_inertia);

    // Maximum allowable deflection: L / limit
    let max_allowable = span_length / deflection_limit;

    let ratio = if max_allowable > 0.0 {
        deflection / max_allowable
    } else {
        0.0
    };

    // Actual span-to-deflection ratio
    let span_ratio = if deflection > 0.0 {
        let actual_ratio = (span_length / deflection).round() as u64;
        format!("L/{}", actual_ratio)
    } else {
        "L/inf".to_string()
    };

    DeflectionResult {
        deflection_inches: deflection,
        max_allowable,
        ratio,
        status: load_status_from_ratio(ratio),
        span_ratio,
    }
}

/// Compressive capacity for legs/posts.
///
/// Considers both crushing strength and Euler buckling (pin-pin ends):
///   Crushing capacity = compressive_strength * cross_section_area
///   Euler buckling: P_cr = pi^2 * E * I_min / L^2
///
/// Actual capacity is min(crushing, buckling) with a 2.5 safety factor on buckling.
///
/// # Parameters
/// - `length`: post/leg height in inches
/// - `width`: cross-section width (inches)
/// - `height`: cross-section depth (inches)
/// - `moe`: modulus of elasticity (psi)
/// - `compressive_strength`: parallel to grain (psi)
/// - `applied_load`: load in pounds
pub fn calculate_compression(
    length: f64,
    width: f64,
    height: f64,
    moe: f64,
    compressive_strength: f64,
    applied_load: f64,
) -> CompressionResult {
    let area = width * height;

    // Crushing capacity
    let crushing_capacity = compressive_strength * area;

    // Euler buckling uses the minimum moment of inertia (weakest axis)
    let min_dim = width.min(height);
    let max_dim = width.max(height);
    let i_min = max_dim * min_dim.powi(3) / 12.0;

    // Slenderness ratio: effective length / radius of gyration
    // radius of gyration = sqrt(I / A), for rectangular = min_dim / sqrt(12)
    let radius_of_gyration = min_dim / (12.0_f64).sqrt();
    let slenderness_ratio = length / radius_of_gyration;
    let buckling_risk = slenderness_ratio > 50.0;

    // Euler critical buckling load
    let euler_load = std::f64::consts::PI.powi(2) * moe * i_min / length.powi(2);

    // Apply safety factor of 2.5 on buckling
    let safe_buckling = euler_load / 2.5;

    // Capacity is the lesser of crushing and safe buckling
    let capacity = crushing_capacity.min(safe_buckling);

    let utilization = if capacity > 0.0 {
        applied_load / capacity
    } else {
        1.0
    };

    CompressionResult {
        applied_load_lbs: applied_load,
        capacity_lbs: capacity,
        utilization,
        status: load_status_from_ratio(utilization),
        buckling_risk,
    }
}

/// Joint strength analysis.
///
/// Adjusts base joint strength by wood species properties.
/// Species factor formula:
///   (janka_hardness / 1000) * 0.4 + (density / 40) * 0.3 + (moe / 1_800_000) * 0.3
/// Normalized so "average" wood (janka ~1000, density ~40, moe ~1.8M) gives factor ~1.0.
///
/// # Parameters
/// - `base_strength`: 0-100 score from joint type
/// - `janka_hardness`: Janka hardness rating
/// - `density_lb_ft3`: density in pounds per cubic foot
/// - `moe`: modulus of elasticity (psi)
pub fn calculate_joint_strength(
    base_strength: u32,
    janka_hardness: u32,
    density_lb_ft3: f64,
    moe: f64,
) -> JointStrengthResult {
    let species_factor = (janka_hardness as f64 / 1000.0) * 0.4
        + (density_lb_ft3 / 40.0) * 0.3
        + (moe / 1_800_000.0) * 0.3;

    let raw = (base_strength as f64 * species_factor).round() as u32;
    let adjusted_strength = raw.min(100);

    let status = if adjusted_strength >= 70 {
        LoadStatus::Green
    } else if adjusted_strength >= 45 {
        LoadStatus::Yellow
    } else {
        LoadStatus::Red
    };

    let recommendation = match status {
        LoadStatus::Green => "Joint is adequate for this species.".to_string(),
        LoadStatus::Yellow => {
            "Joint may be marginal. Consider reinforcement (glue blocks, screws, or a stronger joint type).".to_string()
        }
        LoadStatus::Red => {
            "Joint is weak for this application. Use a stronger joint type or add mechanical fasteners.".to_string()
        }
    };

    JointStrengthResult {
        base_strength,
        species_factor,
        adjusted_strength,
        status,
        recommendation,
    }
}

/// Shelf sag calculator -- convenience wrapper.
///
/// Takes shelf dimensions, wood properties, and total load weight.
/// Uses L/360 as the deflection limit (standard for shelves).
pub fn analyze_shelf(
    span_inches: f64,
    board_width: f64,
    board_height: f64,
    moe: f64,
    total_load_lbs: f64,
) -> DeflectionResult {
    let load_per_inch = total_load_lbs / span_inches;
    calculate_beam_deflection(
        span_inches,
        board_width,
        board_height,
        moe,
        load_per_inch,
        360.0,
    )
}

/// Table leg analysis -- convenience wrapper.
///
/// Takes leg dimensions, wood properties, and load per leg.
pub fn analyze_leg(
    leg_length: f64,
    leg_width: f64,
    leg_height: f64,
    moe: f64,
    compressive_strength: f64,
    load_per_leg: f64,
) -> CompressionResult {
    calculate_compression(
        leg_length,
        leg_width,
        leg_height,
        moe,
        compressive_strength,
        load_per_leg,
    )
}

fn load_status_from_ratio(ratio: f64) -> LoadStatus {
    if ratio < 0.6 {
        LoadStatus::Green
    } else if ratio < 0.85 {
        LoadStatus::Yellow
    } else {
        LoadStatus::Red
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shelf_deflection_2x10_pine() {
        // 36" shelf, 2x10 pine (1.5" x 9.25"), 50 lbs total
        // MoE = 1,700,000 psi
        let result = calculate_beam_deflection(36.0, 1.5, 9.25, 1_700_000.0, 50.0 / 36.0, 360.0);
        assert!(
            result.deflection_inches < 0.1,
            "Expected minimal deflection, got {}",
            result.deflection_inches
        );
        assert_eq!(result.status, LoadStatus::Green);
    }

    #[test]
    fn test_long_shelf_sags() {
        // 72" shelf, 1x12 pine (11.25" wide x 0.75" thick), 200 lbs
        let result = analyze_shelf(72.0, 11.25, 0.75, 1_700_000.0, 200.0);
        // This should show significant deflection
        assert!(
            result.deflection_inches > 0.05,
            "Expected noticeable deflection, got {}",
            result.deflection_inches
        );
    }

    #[test]
    fn test_table_leg_compression() {
        // 30" leg, 4x4 (3.5" x 3.5"), pine, 200 lbs per leg
        let result = calculate_compression(30.0, 3.5, 3.5, 1_700_000.0, 5_020.0, 200.0);
        assert_eq!(result.status, LoadStatus::Green);
        assert!(!result.buckling_risk);
    }

    #[test]
    fn test_thin_post_buckling_risk() {
        // 48" tall, 1x2 (0.75" x 1.5"), loaded heavily
        let result = calculate_compression(48.0, 0.75, 1.5, 1_700_000.0, 5_020.0, 500.0);
        assert!(
            result.buckling_risk,
            "Slender post should flag buckling risk"
        );
    }

    #[test]
    fn test_joint_strength_hard_vs_soft() {
        // Hard maple (janka 1450, density 44, moe 1.83M) should be stronger
        let hard = calculate_joint_strength(85, 1450, 44.0, 1_830_000.0);
        // Pine (janka 870, density 35, moe 1.7M) should be weaker
        let soft = calculate_joint_strength(85, 870, 35.0, 1_700_000.0);
        assert!(
            hard.adjusted_strength >= soft.adjusted_strength,
            "Hard maple ({}) should be >= pine ({})",
            hard.adjusted_strength,
            soft.adjusted_strength
        );
    }

    #[test]
    fn test_load_status_boundaries() {
        assert_eq!(load_status_from_ratio(0.3), LoadStatus::Green);
        assert_eq!(load_status_from_ratio(0.59), LoadStatus::Green);
        assert_eq!(load_status_from_ratio(0.6), LoadStatus::Yellow);
        assert_eq!(load_status_from_ratio(0.7), LoadStatus::Yellow);
        assert_eq!(load_status_from_ratio(0.84), LoadStatus::Yellow);
        assert_eq!(load_status_from_ratio(0.85), LoadStatus::Red);
        assert_eq!(load_status_from_ratio(0.95), LoadStatus::Red);
        assert_eq!(load_status_from_ratio(1.5), LoadStatus::Red);
    }

    #[test]
    fn test_analyze_shelf_wrapper() {
        // Same as manual calculation: 36" span, 1.5" x 9.25", 50 lbs
        let wrapper = analyze_shelf(36.0, 1.5, 9.25, 1_700_000.0, 50.0);
        let manual = calculate_beam_deflection(36.0, 1.5, 9.25, 1_700_000.0, 50.0 / 36.0, 360.0);
        assert!(
            (wrapper.deflection_inches - manual.deflection_inches).abs() < 1e-10,
            "Wrapper should match manual calculation"
        );
    }

    #[test]
    fn test_analyze_leg_wrapper() {
        // Same as manual calculation
        let wrapper = analyze_leg(30.0, 3.5, 3.5, 1_700_000.0, 5_020.0, 200.0);
        let manual = calculate_compression(30.0, 3.5, 3.5, 1_700_000.0, 5_020.0, 200.0);
        assert!(
            (wrapper.utilization - manual.utilization).abs() < 1e-10,
            "Wrapper should match manual calculation"
        );
    }

    #[test]
    fn test_zero_deflection_span_ratio() {
        // With zero load, deflection should be zero and span_ratio should be "L/inf"
        let result = calculate_beam_deflection(36.0, 1.5, 9.25, 1_700_000.0, 0.0, 360.0);
        assert_eq!(result.deflection_inches, 0.0);
        assert_eq!(result.span_ratio, "L/inf");
    }

    #[test]
    fn test_joint_strength_capped_at_100() {
        // Very hard wood with high base strength should still cap at 100
        let result = calculate_joint_strength(95, 3000, 80.0, 3_000_000.0);
        assert!(
            result.adjusted_strength <= 100,
            "Adjusted strength should cap at 100, got {}",
            result.adjusted_strength
        );
    }

    #[test]
    fn test_compression_heavy_load_status() {
        // Overloaded small post: 1x1 pine, 24" tall, 10000 lbs
        let result = calculate_compression(24.0, 0.75, 0.75, 1_700_000.0, 5_020.0, 10_000.0);
        assert_eq!(
            result.status,
            LoadStatus::Red,
            "Overloaded post should be Red"
        );
        assert!(result.utilization > 1.0, "Should exceed capacity");
    }
}
