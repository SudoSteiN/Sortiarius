use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A piece that needs to be cut
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutPiece {
    pub id: String,
    pub label: String,
    pub length: f64,
    pub width: f64,
    pub material: String,
}

/// A stock board that pieces are cut from
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockBoard {
    pub label: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

/// Result of placing a piece on a stock board
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Placement {
    pub piece_id: String,
    pub piece_label: String,
    pub start_offset: f64,
    pub length: f64,
}

/// A single stock board with its cut layout
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutLayout {
    pub stock: StockBoard,
    pub placements: Vec<Placement>,
    pub used_length: f64,
    pub waste_length: f64,
    pub utilization: f64,
}

/// Complete optimization result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub layouts: Vec<CutLayout>,
    pub total_stock_boards: u32,
    pub total_waste_inches: f64,
    pub total_waste_percent: f64,
    pub kerf_width: f64,
    pub unplaceable: Vec<CutPiece>,
}

/// Find the best stock board size for a set of pieces.
/// Tries all available stock sizes and returns the one with least waste
/// whose width is compatible (stock width >= piece width).
fn best_stock_for_pieces(
    pieces: &[&CutPiece],
    available_stock: &[StockBoard],
    kerf_width: f64,
) -> Option<StockBoard> {
    if pieces.is_empty() {
        return None;
    }

    let max_piece_width = pieces
        .iter()
        .map(|p| p.width)
        .fold(0.0_f64, f64::max);

    let total_length: f64 = pieces.iter().map(|p| p.length).sum::<f64>()
        + kerf_width * (pieces.len().saturating_sub(1)) as f64;

    let mut best: Option<StockBoard> = None;
    let mut best_waste = f64::MAX;

    for stock in available_stock {
        // Stock must be wide enough for the pieces
        if stock.width < max_piece_width {
            continue;
        }

        // Calculate how many boards we'd need
        let boards_needed = (total_length / stock.length).ceil() as u64;
        let total_stock_length = boards_needed as f64 * stock.length;
        let waste = total_stock_length - total_length;

        if waste < best_waste {
            best_waste = waste;
            best = Some(stock.clone());
        }
    }

    best
}

/// Optimize cut list using First Fit Decreasing bin-packing.
///
/// Algorithm:
/// 1. Sort pieces by length (longest first)
/// 2. Group pieces by material (same species/type cut from same stock)
/// 3. For each group, try to fit pieces into stock boards:
///    a. Try each existing partially-used board
///    b. If no fit, open a new stock board
/// 4. Account for kerf (saw blade width) between cuts
pub fn optimize_cuts(
    pieces: &[CutPiece],
    available_stock: &[StockBoard],
    kerf_width: f64,
) -> OptimizationResult {
    let mut layouts: Vec<CutLayout> = Vec::new();
    let mut unplaceable: Vec<CutPiece> = Vec::new();

    // Group pieces by material
    let mut groups: HashMap<String, Vec<&CutPiece>> = HashMap::new();
    for piece in pieces {
        groups
            .entry(piece.material.clone())
            .or_default()
            .push(piece);
    }

    for (_material, mut group_pieces) in groups {
        // Sort longest first (FFD)
        group_pieces.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap_or(std::cmp::Ordering::Equal));

        // Find the best stock board for this group
        let stock = match best_stock_for_pieces(&group_pieces, available_stock, kerf_width) {
            Some(s) => s,
            None => {
                // No compatible stock found; all pieces are unplaceable
                for piece in group_pieces {
                    unplaceable.push(piece.clone());
                }
                continue;
            }
        };

        // Track layouts for this material group (indices into layouts vec)
        let mut group_layout_indices: Vec<usize> = Vec::new();

        for piece in &group_pieces {
            // Check if piece fits on the stock at all
            if piece.length > stock.length || piece.width > stock.width {
                unplaceable.push((*piece).clone());
                continue;
            }

            // Try to fit into an existing layout
            let mut placed = false;
            for &idx in &group_layout_indices {
                let layout = &mut layouts[idx];
                let needed = if layout.placements.is_empty() {
                    piece.length
                } else {
                    piece.length + kerf_width
                };

                let remaining = layout.stock.length - layout.used_length;
                if needed <= remaining {
                    let start_offset = if layout.placements.is_empty() {
                        0.0
                    } else {
                        layout.used_length + kerf_width
                    };

                    layout.placements.push(Placement {
                        piece_id: piece.id.clone(),
                        piece_label: piece.label.clone(),
                        start_offset,
                        length: piece.length,
                    });
                    layout.used_length = start_offset + piece.length;
                    placed = true;
                    break;
                }
            }

            if !placed {
                // Open a new stock board
                let new_layout = CutLayout {
                    stock: stock.clone(),
                    placements: vec![Placement {
                        piece_id: piece.id.clone(),
                        piece_label: piece.label.clone(),
                        start_offset: 0.0,
                        length: piece.length,
                    }],
                    used_length: piece.length,
                    waste_length: 0.0, // calculated later
                    utilization: 0.0,  // calculated later
                };
                layouts.push(new_layout);
                group_layout_indices.push(layouts.len() - 1);
            }
        }
    }

    // Calculate waste and utilization for each layout
    let mut total_waste = 0.0;
    let mut total_stock_length = 0.0;

    for layout in &mut layouts {
        layout.waste_length = layout.stock.length - layout.used_length;
        layout.utilization = if layout.stock.length > 0.0 {
            layout.used_length / layout.stock.length
        } else {
            0.0
        };
        total_waste += layout.waste_length;
        total_stock_length += layout.stock.length;
    }

    let total_waste_percent = if total_stock_length > 0.0 {
        (total_waste / total_stock_length) * 100.0
    } else {
        0.0
    };

    OptimizationResult {
        total_stock_boards: layouts.len() as u32,
        layouts,
        total_waste_inches: total_waste,
        total_waste_percent,
        kerf_width,
        unplaceable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stock_8ft_2x4() -> StockBoard {
        StockBoard {
            label: "2x4 x 8'".to_string(),
            length: 96.0,
            width: 1.5,
            height: 3.5,
        }
    }

    fn stock_10ft_2x4() -> StockBoard {
        StockBoard {
            label: "2x4 x 10'".to_string(),
            length: 120.0,
            width: 1.5,
            height: 3.5,
        }
    }

    fn piece(id: &str, label: &str, length: f64, material: &str) -> CutPiece {
        CutPiece {
            id: id.to_string(),
            label: label.to_string(),
            length,
            width: 1.5,
            material: material.to_string(),
        }
    }

    #[test]
    fn test_single_piece() {
        let pieces = vec![piece("1", "Leg", 30.0, "pine")];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        assert_eq!(result.total_stock_boards, 1);
        assert_eq!(result.layouts[0].placements.len(), 1);
        assert_eq!(result.unplaceable.len(), 0);
        assert!((result.layouts[0].used_length - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_multiple_pieces_one_board() {
        // Three short pieces that fit on one 8' board: 24 + 0.125 + 24 + 0.125 + 24 = 72.25"
        let pieces = vec![
            piece("1", "Rail A", 24.0, "pine"),
            piece("2", "Rail B", 24.0, "pine"),
            piece("3", "Rail C", 24.0, "pine"),
        ];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        assert_eq!(result.total_stock_boards, 1);
        assert_eq!(result.layouts[0].placements.len(), 3);
    }

    #[test]
    fn test_kerf_accounted() {
        // Without kerf: 3 x 32" = 96" (fits on one 8' board)
        // With 0.125" kerf: 32 + 0.125 + 32 + 0.125 + 32 = 96.25" (needs two boards)
        let pieces = vec![
            piece("1", "Part A", 32.0, "pine"),
            piece("2", "Part B", 32.0, "pine"),
            piece("3", "Part C", 32.0, "pine"),
        ];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        assert_eq!(result.total_stock_boards, 2, "Kerf should force a second board");
    }

    #[test]
    fn test_unplaceable_piece() {
        // A 120" piece won't fit on a 96" stock board
        let pieces = vec![piece("1", "Long Rail", 120.0, "pine")];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        assert_eq!(result.unplaceable.len(), 1);
        assert_eq!(result.unplaceable[0].id, "1");
        assert_eq!(result.total_stock_boards, 0);
    }

    #[test]
    fn test_material_grouping() {
        let pieces = vec![
            piece("1", "Oak Leg", 30.0, "oak"),
            piece("2", "Pine Rail", 30.0, "pine"),
            piece("3", "Oak Leg", 30.0, "oak"),
            piece("4", "Pine Rail", 30.0, "pine"),
        ];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        // Should be 2 boards: one for oak, one for pine
        assert_eq!(result.total_stock_boards, 2);

        for layout in &result.layouts {
            let materials: Vec<&str> = layout
                .placements
                .iter()
                .map(|p| {
                    // Find matching piece to check material
                    pieces
                        .iter()
                        .find(|piece| piece.id == p.piece_id)
                        .unwrap()
                        .material
                        .as_str()
                })
                .collect();
            // All pieces on a board should be the same material
            assert!(
                materials.windows(2).all(|w| w[0] == w[1]),
                "Mixed materials on a single board: {:?}",
                materials
            );
        }
    }

    #[test]
    fn test_optimal_vs_naive() {
        // FFD should pack better: one 60" piece and two 30" pieces
        // FFD: sorts to [60, 30, 30] -> 60" on board 1, then 30+30=60.125" on board 1 won't fit
        //   60 placed first, remaining = 96 - 60 = 36, next 30 fits (60 + 0.125 + 30 = 90.125)
        //   remaining = 96 - 90.125 = 5.875, last 30 won't fit -> board 2
        // Naive (sequential without sorting): might do worse with unlucky ordering
        //
        // Test with pieces that FFD handles in 2 boards:
        // Pieces: 50", 46", 46" -> sorted: [50, 46, 46]
        //   Board 1: 50, remaining=46. 46 fits (50+0.125+46=96.125 > 96)? No. -> Board 2: 46
        //   Board 2: remaining=96-46=50. 46 fits (46+0.125+46=92.125)? Yes. -> Board 2.
        //   Result: 2 boards
        //
        // Without FFD (worst case order: 46, 46, 50):
        //   Board 1: 46, then 46 fits (46+0.125+46=92.125). remaining=3.875
        //   Board 2: 50
        //   Result: 2 boards (same here, but let's test a clear case)
        //
        // Better test: pieces 48, 48, 48, 24, 24 on 96" boards
        // FFD: [48,48,48,24,24] -> Board1: 48+48=96.125 > 96, so 48 alone.
        //   Actually: 48, remaining=48. Next 48: 48+0.125+48=96.125>96, no fit. Board2: 48.
        //   Board2: 48, remaining=48. Next 48: same, no fit. Board3: 48.
        //   Board3 remaining=48, next 24: 48+0.125+24=72.125, fits.
        //   Board3 remaining=96-72.125=23.875, next 24: 72.125+0.125+24=96.25>96, no fit. Board4: 24.
        //   Total: 4 boards
        //
        // Actually the best test: compare board count to a trivially bad packing.
        // Let's just verify FFD produces a reasonable result.
        let pieces = vec![
            piece("1", "Long A", 60.0, "pine"),
            piece("2", "Short B", 24.0, "pine"),
            piece("3", "Short C", 24.0, "pine"),
            piece("4", "Short D", 24.0, "pine"),
        ];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        // FFD: sorts to [60, 24, 24, 24]
        // Board 1: 60, remaining=36. 24 fits (60+0.125+24=84.125). remaining=11.875.
        //   next 24: 84.125+0.125+24=108.25 > 96, no. Board 2: 24.
        //   Actually remaining = 96 - 84.125 = 11.875, 24 won't fit.
        // Board 2: 24, remaining=72. 24 fits (24+0.125+24=48.125). remaining=47.875.
        // Total: 2 boards.
        //
        // Naive (no sort, sequential 1-per-board): would be 4 boards in worst case.
        // FFD achieves 2 boards.
        assert_eq!(result.total_stock_boards, 2, "FFD should pack into 2 boards");
        assert_eq!(result.unplaceable.len(), 0);
    }

    #[test]
    fn test_utilization_calculation() {
        let pieces = vec![piece("1", "Part", 48.0, "pine")];
        let stock = vec![stock_8ft_2x4()];
        let result = optimize_cuts(&pieces, &stock, 0.125);

        assert_eq!(result.layouts.len(), 1);
        let layout = &result.layouts[0];
        // 48 used out of 96 = 50% utilization
        assert!((layout.utilization - 0.5).abs() < 0.001);
        assert!((layout.waste_length - 48.0).abs() < 0.001);
        assert!((layout.used_length - 48.0).abs() < 0.001);
    }
}
