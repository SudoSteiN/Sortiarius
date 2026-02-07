use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FinishType {
    Natural,
    Stain(StainFinish),
    Paint(PaintFinish),
    Oil(OilFinish),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StainFinish {
    pub name: String,
    pub color: [f32; 3],
    pub opacity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaintFinish {
    pub name: String,
    pub color: [f32; 3],
    pub sheen: Sheen,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OilFinish {
    pub name: String,
    pub color_shift: [f32; 3],
    pub enhances_grain: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Sheen {
    Matte,
    Satin,
    SemiGloss,
    Gloss,
}

fn lerp_color(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
    ]
}

fn clamp_color(c: [f32; 3]) -> [f32; 3] {
    [
        c[0].clamp(0.0, 1.0),
        c[1].clamp(0.0, 1.0),
        c[2].clamp(0.0, 1.0),
    ]
}

/// Apply a finish to a base wood color, returning the resulting display color.
/// All output channels are clamped to 0-1.
pub fn apply_finish(base_color: [f32; 3], finish: &FinishType) -> [f32; 3] {
    match finish {
        FinishType::Natural => base_color,
        FinishType::Stain(stain) => clamp_color(lerp_color(base_color, stain.color, stain.opacity)),
        FinishType::Paint(paint) => {
            // Paint at high sheen is more opaque visually; we treat paint as
            // nearly full coverage with a slight show-through of the base.
            let opacity = match paint.sheen {
                Sheen::Matte => 0.90,
                Sheen::Satin => 0.92,
                Sheen::SemiGloss => 0.95,
                Sheen::Gloss => 0.97,
            };
            clamp_color(lerp_color(base_color, paint.color, opacity))
        }
        FinishType::Oil(oil) => clamp_color([
            base_color[0] * (1.0 + oil.color_shift[0]),
            base_color[1] * (1.0 + oil.color_shift[1]),
            base_color[2] * (1.0 + oil.color_shift[2]),
        ]),
    }
}

/// Built-in stain presets.
pub fn stain_presets() -> Vec<StainFinish> {
    vec![
        StainFinish {
            name: "Early American".into(),
            color: [0.55, 0.38, 0.22],
            opacity: 0.5,
        },
        StainFinish {
            name: "Dark Walnut".into(),
            color: [0.30, 0.20, 0.12],
            opacity: 0.6,
        },
        StainFinish {
            name: "Golden Oak".into(),
            color: [0.72, 0.55, 0.28],
            opacity: 0.45,
        },
        StainFinish {
            name: "Ebony".into(),
            color: [0.10, 0.08, 0.06],
            opacity: 0.7,
        },
        StainFinish {
            name: "Provincial".into(),
            color: [0.48, 0.35, 0.22],
            opacity: 0.5,
        },
        StainFinish {
            name: "Red Mahogany".into(),
            color: [0.50, 0.18, 0.10],
            opacity: 0.55,
        },
        StainFinish {
            name: "Weathered Gray".into(),
            color: [0.55, 0.55, 0.52],
            opacity: 0.45,
        },
        StainFinish {
            name: "Natural".into(),
            color: [0.80, 0.70, 0.55],
            opacity: 0.15,
        },
    ]
}

/// Built-in paint presets.
pub fn paint_presets() -> Vec<PaintFinish> {
    vec![
        PaintFinish {
            name: "Classic White".into(),
            color: [0.97, 0.97, 0.95],
            sheen: Sheen::SemiGloss,
        },
        PaintFinish {
            name: "Antique White".into(),
            color: [0.93, 0.89, 0.82],
            sheen: Sheen::Satin,
        },
        PaintFinish {
            name: "Navy Blue".into(),
            color: [0.12, 0.18, 0.35],
            sheen: Sheen::Satin,
        },
        PaintFinish {
            name: "Forest Green".into(),
            color: [0.15, 0.32, 0.18],
            sheen: Sheen::Satin,
        },
        PaintFinish {
            name: "Barn Red".into(),
            color: [0.55, 0.12, 0.10],
            sheen: Sheen::Matte,
        },
        PaintFinish {
            name: "Charcoal".into(),
            color: [0.22, 0.22, 0.22],
            sheen: Sheen::Matte,
        },
        PaintFinish {
            name: "Sage Green".into(),
            color: [0.60, 0.68, 0.55],
            sheen: Sheen::Satin,
        },
        PaintFinish {
            name: "Dusty Rose".into(),
            color: [0.75, 0.52, 0.52],
            sheen: Sheen::Satin,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_no_change() {
        let base = [0.8, 0.6, 0.4];
        let result = apply_finish(base, &FinishType::Natural);
        assert_eq!(result, base);
    }

    #[test]
    fn test_stain_blending() {
        let base = [0.8, 0.6, 0.4];
        let stain = FinishType::Stain(StainFinish {
            name: "Test".into(),
            color: [0.3, 0.2, 0.1],
            opacity: 0.5,
        });
        let result = apply_finish(base, &stain);
        // At 50% opacity, result should be midpoint between base and stain color
        assert!((result[0] - 0.55).abs() < 0.01);
        assert!((result[1] - 0.40).abs() < 0.01);
        assert!((result[2] - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_paint_covers() {
        let base = [0.8, 0.6, 0.4];
        let paint = FinishType::Paint(PaintFinish {
            name: "White".into(),
            color: [1.0, 1.0, 1.0],
            sheen: Sheen::Gloss,
        });
        let result = apply_finish(base, &paint);
        // Gloss paint at 97% opacity should be very close to paint color
        for c in 0..3 {
            assert!(
                result[c] > 0.95,
                "Paint should nearly cover base. Channel {} = {}",
                c,
                result[c]
            );
        }
    }

    #[test]
    fn test_oil_darkens() {
        let base = [0.8, 0.6, 0.4];
        let oil = FinishType::Oil(OilFinish {
            name: "Tung Oil".into(),
            color_shift: [0.05, 0.03, 0.01],
            enhances_grain: true,
        });
        let result = apply_finish(base, &oil);
        // Oil should warm/darken: multiply by (1 + shift)
        assert!(result[0] > base[0]);
        assert!(result[1] > base[1]);
        assert!(result[2] > base[2]);
    }

    #[test]
    fn test_presets_count() {
        let stains = stain_presets();
        assert!(
            stains.len() >= 8,
            "Expected at least 8 stain presets, got {}",
            stains.len()
        );

        let paints = paint_presets();
        assert!(
            paints.len() >= 8,
            "Expected at least 8 paint presets, got {}",
            paints.len()
        );
    }

    #[test]
    fn test_color_clamped() {
        // Use extreme values that could push out of range
        let base = [0.95, 0.95, 0.95];
        let oil = FinishType::Oil(OilFinish {
            name: "Heavy".into(),
            color_shift: [0.2, 0.2, 0.2],
            enhances_grain: false,
        });
        let result = apply_finish(base, &oil);
        for c in 0..3 {
            assert!(
                (0.0..=1.0).contains(&result[c]),
                "Output channel {} = {} is out of 0-1 range",
                c,
                result[c]
            );
        }

        // Stain should also clamp
        let stain = FinishType::Stain(StainFinish {
            name: "Test".into(),
            color: [1.0, 1.0, 1.0],
            opacity: 1.0,
        });
        let result = apply_finish(base, &stain);
        for c in 0..3 {
            assert!(
                (0.0..=1.0).contains(&result[c]),
                "Stain output channel {} = {} is out of 0-1 range",
                c,
                result[c]
            );
        }
    }
}
