use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Price per board foot for a species
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeciesPrice {
    pub species_id: String,
    pub species_name: String,
    pub price_per_bf: f64,
    pub is_user_edited: bool,
}

/// Cost estimation for a single material
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialCost {
    pub species: String,
    pub board_feet: f64,
    pub price_per_bf: f64,
    pub subtotal: f64,
}

/// Hardware cost entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareCost {
    pub item: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub subtotal: f64,
}

/// Complete cost estimate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostEstimate {
    pub materials: Vec<MaterialCost>,
    pub hardware: Vec<HardwareCost>,
    pub material_subtotal: f64,
    pub hardware_subtotal: f64,
    pub waste_factor: f64,
    pub waste_cost: f64,
    pub total: f64,
    pub notes: Vec<String>,
}

/// Price database with defaults and user overrides
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceDatabase {
    prices: HashMap<String, SpeciesPrice>,
}

/// Default prices (approximate US retail $/bf, 2024-2025)
fn default_prices() -> Vec<(& 'static str, &'static str, f64)> {
    vec![
        ("pine", "Pine", 3.50),
        ("douglas_fir", "Douglas Fir", 4.00),
        ("red_oak", "Red Oak", 6.50),
        ("white_oak", "White Oak", 7.00),
        ("hard_maple", "Hard Maple", 7.50),
        ("black_walnut", "Black Walnut", 12.00),
        ("cherry", "Cherry", 8.50),
        ("western_red_cedar", "Western Red Cedar", 5.50),
        ("poplar", "Poplar", 4.50),
        ("birch", "Birch", 5.50),
        ("ash", "Ash", 6.00),
        ("mahogany", "Mahogany", 14.00),
    ]
}

impl PriceDatabase {
    /// Create with default prices (approximate US retail 2024-2025)
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        for (id, name, price) in default_prices() {
            prices.insert(
                id.to_string(),
                SpeciesPrice {
                    species_id: id.to_string(),
                    species_name: name.to_string(),
                    price_per_bf: price,
                    is_user_edited: false,
                },
            );
        }
        Self { prices }
    }

    /// Get price for a species
    pub fn get_price(&self, species_id: &str) -> Option<&SpeciesPrice> {
        self.prices.get(species_id)
    }

    /// Set user override price
    pub fn set_price(&mut self, species_id: &str, price_per_bf: f64) {
        if let Some(entry) = self.prices.get_mut(species_id) {
            entry.price_per_bf = price_per_bf;
            entry.is_user_edited = true;
        } else {
            self.prices.insert(
                species_id.to_string(),
                SpeciesPrice {
                    species_id: species_id.to_string(),
                    species_name: species_id.to_string(),
                    price_per_bf,
                    is_user_edited: true,
                },
            );
        }
    }

    /// Reset to default price
    pub fn reset_price(&mut self, species_id: &str) {
        for (id, name, price) in default_prices() {
            if id == species_id {
                self.prices.insert(
                    id.to_string(),
                    SpeciesPrice {
                        species_id: id.to_string(),
                        species_name: name.to_string(),
                        price_per_bf: price,
                        is_user_edited: false,
                    },
                );
                return;
            }
        }
    }

    /// Get all prices
    pub fn all_prices(&self) -> Vec<&SpeciesPrice> {
        self.prices.values().collect()
    }
}

impl Default for PriceDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimate hardware costs from joints.
///
/// Each joint type may require specific hardware:
/// - PocketHole: 2 pocket screws (~$0.15 each)
/// - DowelJoint: dowels (~$0.25 each)
/// - BiscuitJoint: biscuits (~$0.10 each)
/// Others are glue-only (no hardware cost).
pub fn estimate_hardware_costs(
    joints: &[(String, String, u32)], // (joint_type, display_name, count)
) -> Vec<HardwareCost> {
    let mut costs = Vec::new();

    for (joint_type, display_name, count) in joints {
        match joint_type.as_str() {
            "PocketHole" => {
                let screws_per_joint = 2u32;
                let total_screws = screws_per_joint * count;
                let unit_price = 0.15;
                costs.push(HardwareCost {
                    item: format!("Pocket screws (for {} joints)", display_name),
                    quantity: total_screws,
                    unit_price,
                    subtotal: total_screws as f64 * unit_price,
                });
            }
            "DowelJoint" => {
                let dowels_per_joint = 2u32;
                let total_dowels = dowels_per_joint * count;
                let unit_price = 0.25;
                costs.push(HardwareCost {
                    item: format!("Dowel pins (for {} joints)", display_name),
                    quantity: total_dowels,
                    unit_price,
                    subtotal: total_dowels as f64 * unit_price,
                });
            }
            "BiscuitJoint" => {
                let biscuits_per_joint = 1u32;
                let total_biscuits = biscuits_per_joint * count;
                let unit_price = 0.10;
                costs.push(HardwareCost {
                    item: format!("Biscuits (for {} joints)", display_name),
                    quantity: total_biscuits,
                    unit_price,
                    subtotal: total_biscuits as f64 * unit_price,
                });
            }
            _ => {
                // Glue-only joints have no hardware cost
            }
        }
    }

    costs
}

/// Generate complete cost estimate
pub fn estimate_costs(
    material_board_feet: &[(String, String, f64)], // (species_id, species_name, board_feet)
    joints: &[(String, String, u32)],               // (joint_type, display_name, count)
    price_db: &PriceDatabase,
    waste_factor: f64,
) -> CostEstimate {
    let mut materials = Vec::new();
    let mut notes = Vec::new();

    for (species_id, species_name, board_feet) in material_board_feet {
        let price_per_bf = match price_db.get_price(species_id) {
            Some(sp) => sp.price_per_bf,
            None => {
                notes.push(format!(
                    "No price found for '{}', using $0.00",
                    species_name
                ));
                0.0
            }
        };

        let subtotal = board_feet * price_per_bf;
        materials.push(MaterialCost {
            species: species_name.clone(),
            board_feet: *board_feet,
            price_per_bf,
            subtotal,
        });
    }

    let hardware = estimate_hardware_costs(joints);

    let material_subtotal: f64 = materials.iter().map(|m| m.subtotal).sum();
    let hardware_subtotal: f64 = hardware.iter().map(|h| h.subtotal).sum();
    let waste_cost = material_subtotal * waste_factor;
    let total = material_subtotal + hardware_subtotal + waste_cost;

    CostEstimate {
        materials,
        hardware,
        material_subtotal,
        hardware_subtotal,
        waste_factor,
        waste_cost,
        total,
        notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_prices() {
        let db = PriceDatabase::new();
        let all = db.all_prices();
        assert_eq!(all.len(), 12);

        assert!(db.get_price("pine").is_some());
        assert!(db.get_price("black_walnut").is_some());
        assert!(db.get_price("mahogany").is_some());
        assert!((db.get_price("pine").unwrap().price_per_bf - 3.50).abs() < 0.001);
    }

    #[test]
    fn test_user_override() {
        let mut db = PriceDatabase::new();
        db.set_price("pine", 5.00);

        let pine = db.get_price("pine").unwrap();
        assert!((pine.price_per_bf - 5.00).abs() < 0.001);
        assert!(pine.is_user_edited);
    }

    #[test]
    fn test_reset_price() {
        let mut db = PriceDatabase::new();
        db.set_price("pine", 99.99);
        assert!((db.get_price("pine").unwrap().price_per_bf - 99.99).abs() < 0.001);

        db.reset_price("pine");
        let pine = db.get_price("pine").unwrap();
        assert!((pine.price_per_bf - 3.50).abs() < 0.001);
        assert!(!pine.is_user_edited);
    }

    #[test]
    fn test_cost_calculation() {
        let db = PriceDatabase::new();
        let materials = vec![("pine".to_string(), "Pine".to_string(), 10.0)];
        let joints: Vec<(String, String, u32)> = vec![];

        let estimate = estimate_costs(&materials, &joints, &db, 0.0);
        assert!((estimate.material_subtotal - 35.0).abs() < 0.001); // 10 bf * $3.50
        assert!((estimate.total - 35.0).abs() < 0.001);
    }

    #[test]
    fn test_waste_factor() {
        let db = PriceDatabase::new();
        let materials = vec![("pine".to_string(), "Pine".to_string(), 10.0)];
        let joints: Vec<(String, String, u32)> = vec![];

        let estimate = estimate_costs(&materials, &joints, &db, 0.15);
        let expected_material = 35.0; // 10 * 3.50
        let expected_waste = expected_material * 0.15; // 5.25
        let expected_total = expected_material + expected_waste; // 40.25

        assert!((estimate.material_subtotal - expected_material).abs() < 0.001);
        assert!((estimate.waste_cost - expected_waste).abs() < 0.001);
        assert!((estimate.total - expected_total).abs() < 0.001);
    }

    #[test]
    fn test_hardware_pocket_hole() {
        let joints = vec![("PocketHole".to_string(), "Pocket Hole".to_string(), 4u32)];
        let hardware = estimate_hardware_costs(&joints);

        assert_eq!(hardware.len(), 1);
        assert_eq!(hardware[0].quantity, 8); // 4 joints * 2 screws each
        assert!((hardware[0].unit_price - 0.15).abs() < 0.001);
        assert!((hardware[0].subtotal - 1.20).abs() < 0.001); // 8 * 0.15
    }

    #[test]
    fn test_total() {
        let db = PriceDatabase::new();
        let materials = vec![
            ("pine".to_string(), "Pine".to_string(), 10.0),
            ("red_oak".to_string(), "Red Oak".to_string(), 5.0),
        ];
        let joints = vec![
            ("PocketHole".to_string(), "Pocket Hole".to_string(), 4u32),
            ("DowelJoint".to_string(), "Dowel Joint".to_string(), 2u32),
        ];

        let estimate = estimate_costs(&materials, &joints, &db, 0.10);

        // Materials: 10*3.50 + 5*6.50 = 35.00 + 32.50 = 67.50
        let expected_material = 67.50;
        // Hardware: pocket screws 8*0.15=1.20, dowels 4*0.25=1.00, total=2.20
        let expected_hardware = 2.20;
        // Waste: 67.50 * 0.10 = 6.75
        let expected_waste = 6.75;
        // Total: 67.50 + 2.20 + 6.75 = 76.45
        let expected_total = expected_material + expected_hardware + expected_waste;

        assert!((estimate.material_subtotal - expected_material).abs() < 0.01);
        assert!((estimate.hardware_subtotal - expected_hardware).abs() < 0.01);
        assert!((estimate.waste_cost - expected_waste).abs() < 0.01);
        assert!((estimate.total - expected_total).abs() < 0.01);
    }
}
