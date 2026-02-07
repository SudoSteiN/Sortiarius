use serde::{Deserialize, Serialize};

/// How easy a wood species is to work with hand and power tools.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Workability {
    Easy,
    Moderate,
    Difficult,
}

/// Mechanical and visual properties for a wood species.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WoodSpecies {
    pub id: &'static str,
    pub name: &'static str,
    #[serde(skip)]
    pub common_names: &'static [&'static str],
    pub density_lb_ft3: f64,
    pub modulus_of_elasticity: f64,
    pub modulus_of_rupture: f64,
    pub janka_hardness: u32,
    pub compressive_strength: f64,
    pub color: [f32; 3],
    pub grain_scale: f32,
    pub workability: Workability,
}

pub struct SpeciesDatabase {
    species: Vec<WoodSpecies>,
}

impl SpeciesDatabase {
    pub fn new() -> Self {
        Self {
            species: vec![
                WoodSpecies {
                    id: "pine_sy",
                    name: "Pine (Southern Yellow)",
                    common_names: &["Southern Yellow Pine", "SYP", "Yellow Pine", "Pine"],
                    density_lb_ft3: 35.0,
                    modulus_of_elasticity: 1_700_000.0,
                    modulus_of_rupture: 12_800.0,
                    janka_hardness: 870,
                    compressive_strength: 6_900.0,
                    color: [0.92, 0.85, 0.65],
                    grain_scale: 1.0,
                    workability: Workability::Easy,
                },
                WoodSpecies {
                    id: "douglas_fir",
                    name: "Douglas Fir",
                    common_names: &["Douglas Fir", "Doug Fir", "Fir"],
                    density_lb_ft3: 32.0,
                    modulus_of_elasticity: 1_900_000.0,
                    modulus_of_rupture: 12_400.0,
                    janka_hardness: 710,
                    compressive_strength: 7_230.0,
                    color: [0.76, 0.55, 0.38],
                    grain_scale: 1.1,
                    workability: Workability::Easy,
                },
                WoodSpecies {
                    id: "red_oak",
                    name: "Red Oak",
                    common_names: &["Red Oak", "Northern Red Oak", "Oak"],
                    density_lb_ft3: 44.0,
                    modulus_of_elasticity: 1_820_000.0,
                    modulus_of_rupture: 14_300.0,
                    janka_hardness: 1290,
                    compressive_strength: 6_760.0,
                    color: [0.72, 0.53, 0.39],
                    grain_scale: 0.8,
                    workability: Workability::Moderate,
                },
                WoodSpecies {
                    id: "white_oak",
                    name: "White Oak",
                    common_names: &["White Oak", "Oak"],
                    density_lb_ft3: 47.0,
                    modulus_of_elasticity: 1_780_000.0,
                    modulus_of_rupture: 15_200.0,
                    janka_hardness: 1360,
                    compressive_strength: 7_440.0,
                    color: [0.78, 0.65, 0.45],
                    grain_scale: 0.75,
                    workability: Workability::Moderate,
                },
                WoodSpecies {
                    id: "hard_maple",
                    name: "Hard Maple",
                    common_names: &["Hard Maple", "Sugar Maple", "Rock Maple", "Maple"],
                    density_lb_ft3: 44.0,
                    modulus_of_elasticity: 1_830_000.0,
                    modulus_of_rupture: 15_800.0,
                    janka_hardness: 1450,
                    compressive_strength: 7_830.0,
                    color: [0.90, 0.83, 0.72],
                    grain_scale: 0.6,
                    workability: Workability::Difficult,
                },
                WoodSpecies {
                    id: "black_walnut",
                    name: "Black Walnut",
                    common_names: &["Black Walnut", "Walnut", "American Walnut"],
                    density_lb_ft3: 38.0,
                    modulus_of_elasticity: 1_680_000.0,
                    modulus_of_rupture: 14_600.0,
                    janka_hardness: 1010,
                    compressive_strength: 7_580.0,
                    color: [0.40, 0.28, 0.18],
                    grain_scale: 0.7,
                    workability: Workability::Moderate,
                },
                WoodSpecies {
                    id: "cherry",
                    name: "Cherry",
                    common_names: &["Cherry", "American Cherry", "Black Cherry"],
                    density_lb_ft3: 35.0,
                    modulus_of_elasticity: 1_490_000.0,
                    modulus_of_rupture: 12_300.0,
                    janka_hardness: 995,
                    compressive_strength: 7_110.0,
                    color: [0.68, 0.42, 0.28],
                    grain_scale: 0.65,
                    workability: Workability::Easy,
                },
                WoodSpecies {
                    id: "western_red_cedar",
                    name: "Western Red Cedar",
                    common_names: &["Western Red Cedar", "Red Cedar", "Cedar"],
                    density_lb_ft3: 23.0,
                    modulus_of_elasticity: 1_110_000.0,
                    modulus_of_rupture: 7_500.0,
                    janka_hardness: 350,
                    compressive_strength: 4_560.0,
                    color: [0.72, 0.45, 0.30],
                    grain_scale: 1.3,
                    workability: Workability::Easy,
                },
                WoodSpecies {
                    id: "poplar",
                    name: "Poplar",
                    common_names: &["Poplar", "Yellow Poplar", "Tulip Poplar", "Tulipwood"],
                    density_lb_ft3: 29.0,
                    modulus_of_elasticity: 1_580_000.0,
                    modulus_of_rupture: 10_100.0,
                    janka_hardness: 540,
                    compressive_strength: 5_540.0,
                    color: [0.82, 0.80, 0.65],
                    grain_scale: 0.9,
                    workability: Workability::Easy,
                },
                WoodSpecies {
                    id: "birch",
                    name: "Birch",
                    common_names: &["Birch", "Yellow Birch", "Sweet Birch"],
                    density_lb_ft3: 43.0,
                    modulus_of_elasticity: 2_010_000.0,
                    modulus_of_rupture: 16_600.0,
                    janka_hardness: 1260,
                    compressive_strength: 8_170.0,
                    color: [0.88, 0.82, 0.70],
                    grain_scale: 0.55,
                    workability: Workability::Moderate,
                },
                WoodSpecies {
                    id: "ash",
                    name: "Ash",
                    common_names: &["Ash", "White Ash", "American Ash"],
                    density_lb_ft3: 41.0,
                    modulus_of_elasticity: 1_740_000.0,
                    modulus_of_rupture: 15_000.0,
                    janka_hardness: 1320,
                    compressive_strength: 7_410.0,
                    color: [0.82, 0.73, 0.55],
                    grain_scale: 0.7,
                    workability: Workability::Moderate,
                },
                WoodSpecies {
                    id: "mahogany",
                    name: "Mahogany",
                    common_names: &["Mahogany", "Genuine Mahogany", "Honduras Mahogany"],
                    density_lb_ft3: 31.0,
                    modulus_of_elasticity: 1_400_000.0,
                    modulus_of_rupture: 11_500.0,
                    janka_hardness: 800,
                    compressive_strength: 6_460.0,
                    color: [0.55, 0.30, 0.18],
                    grain_scale: 0.8,
                    workability: Workability::Easy,
                },
            ],
        }
    }

    pub fn find_by_id(&self, id: &str) -> Option<&WoodSpecies> {
        self.species.iter().find(|s| s.id == id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&WoodSpecies> {
        let lower = name.to_lowercase();
        self.species.iter().find(|s| s.name.to_lowercase() == lower)
    }

    pub fn all_species(&self) -> &[WoodSpecies] {
        &self.species
    }
}

impl Default for SpeciesDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_count() {
        let db = SpeciesDatabase::new();
        assert_eq!(db.all_species().len(), 12);
    }

    #[test]
    fn test_find_by_id() {
        let db = SpeciesDatabase::new();
        let pine = db.find_by_id("pine_sy").unwrap();
        assert_eq!(pine.name, "Pine (Southern Yellow)");

        let walnut = db.find_by_id("black_walnut").unwrap();
        assert_eq!(walnut.name, "Black Walnut");

        assert!(db.find_by_id("nonexistent").is_none());
    }

    #[test]
    fn test_find_by_name() {
        let db = SpeciesDatabase::new();
        let oak = db.find_by_name("red oak").unwrap();
        assert_eq!(oak.id, "red_oak");

        let oak_upper = db.find_by_name("RED OAK").unwrap();
        assert_eq!(oak_upper.id, "red_oak");

        let cherry = db.find_by_name("Cherry").unwrap();
        assert_eq!(cherry.id, "cherry");

        assert!(db.find_by_name("Spruce").is_none());
    }

    #[test]
    fn test_all_have_valid_color() {
        let db = SpeciesDatabase::new();
        for species in db.all_species() {
            for (i, channel) in species.color.iter().enumerate() {
                assert!(
                    (0.0..=1.0).contains(channel),
                    "Species {} has invalid color channel {} = {}",
                    species.name,
                    i,
                    channel
                );
            }
        }
    }
}
