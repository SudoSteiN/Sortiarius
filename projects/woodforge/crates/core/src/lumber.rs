use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LumberCategory {
    Dimensional,
    Board,
    Sheet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LumberSize {
    pub nominal_label: String,
    pub category: LumberCategory,
    pub actual_width: f64,
    pub actual_height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SheetSize {
    pub label: String,
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LumberCatalog {
    pub sizes: Vec<LumberSize>,
    pub sheets: Vec<SheetSize>,
    pub standard_lengths: Vec<f64>,
}

impl LumberCatalog {
    pub fn new() -> Self {
        Self {
            sizes: Self::default_sizes(),
            sheets: Self::default_sheets(),
            standard_lengths: vec![96.0, 120.0, 144.0, 192.0],
        }
    }

    pub fn find_by_label(&self, label: &str) -> Option<&LumberSize> {
        self.sizes.iter().find(|s| s.nominal_label == label)
    }

    pub fn sizes_by_category(&self, cat: LumberCategory) -> Vec<&LumberSize> {
        self.sizes.iter().filter(|s| s.category == cat).collect()
    }

    fn default_sizes() -> Vec<LumberSize> {
        vec![
            // Board lumber (1x series) — actual thickness 0.75"
            LumberSize { nominal_label: "1x2".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 1.5 },
            LumberSize { nominal_label: "1x3".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 2.5 },
            LumberSize { nominal_label: "1x4".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 3.5 },
            LumberSize { nominal_label: "1x6".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 5.5 },
            LumberSize { nominal_label: "1x8".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 7.25 },
            LumberSize { nominal_label: "1x10".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 9.25 },
            LumberSize { nominal_label: "1x12".into(), category: LumberCategory::Board, actual_width: 0.75, actual_height: 11.25 },
            // Dimensional lumber (2x series) — actual thickness 1.5"
            LumberSize { nominal_label: "2x2".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 1.5 },
            LumberSize { nominal_label: "2x3".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 2.5 },
            LumberSize { nominal_label: "2x4".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 3.5 },
            LumberSize { nominal_label: "2x6".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 5.5 },
            LumberSize { nominal_label: "2x8".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 7.25 },
            LumberSize { nominal_label: "2x10".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 9.25 },
            LumberSize { nominal_label: "2x12".into(), category: LumberCategory::Dimensional, actual_width: 1.5, actual_height: 11.25 },
            // Heavy dimensional (4x, 6x)
            LumberSize { nominal_label: "4x4".into(), category: LumberCategory::Dimensional, actual_width: 3.5, actual_height: 3.5 },
            LumberSize { nominal_label: "4x6".into(), category: LumberCategory::Dimensional, actual_width: 3.5, actual_height: 5.5 },
            LumberSize { nominal_label: "6x6".into(), category: LumberCategory::Dimensional, actual_width: 5.5, actual_height: 5.5 },
        ]
    }

    fn default_sheets() -> Vec<SheetSize> {
        vec![
            SheetSize { label: "1/4\" Plywood".into(), width: 48.0, height: 96.0, thickness: 0.25 },
            SheetSize { label: "1/2\" Plywood".into(), width: 48.0, height: 96.0, thickness: 0.5 },
            SheetSize { label: "3/4\" Plywood".into(), width: 48.0, height: 96.0, thickness: 0.75 },
        ]
    }
}

impl Default for LumberCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_sizes() {
        let catalog = LumberCatalog::new();
        assert_eq!(catalog.sizes.len(), 17);
        assert_eq!(catalog.sheets.len(), 3);
        assert_eq!(catalog.standard_lengths.len(), 4);
    }

    #[test]
    fn test_find_by_label() {
        let catalog = LumberCatalog::new();
        let two_by_four = catalog.find_by_label("2x4").unwrap();
        assert_eq!(two_by_four.actual_width, 1.5);
        assert_eq!(two_by_four.actual_height, 3.5);
        assert_eq!(two_by_four.category, LumberCategory::Dimensional);

        let one_by_six = catalog.find_by_label("1x6").unwrap();
        assert_eq!(one_by_six.actual_width, 0.75);
        assert_eq!(one_by_six.actual_height, 5.5);
        assert_eq!(one_by_six.category, LumberCategory::Board);

        assert!(catalog.find_by_label("3x3").is_none());
    }

    #[test]
    fn test_category_filter() {
        let catalog = LumberCatalog::new();
        let boards = catalog.sizes_by_category(LumberCategory::Board);
        assert_eq!(boards.len(), 7); // 1x2 through 1x12
        let dimensional = catalog.sizes_by_category(LumberCategory::Dimensional);
        assert_eq!(dimensional.len(), 10); // 2x2 through 6x6
    }

    #[test]
    fn test_standard_lengths() {
        let catalog = LumberCatalog::new();
        assert_eq!(catalog.standard_lengths, vec![96.0, 120.0, 144.0, 192.0]);
    }
}
