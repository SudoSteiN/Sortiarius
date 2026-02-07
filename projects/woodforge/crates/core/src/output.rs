use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Note: Dimensions3 and NodeId are used by callers when building CutListInput.
// They are not directly used in this module's implementation.

// ========== Cut List ==========

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutListEntry {
    pub piece_id: String,
    pub label: String,
    pub material: String,
    pub length: f64,
    pub width: f64,
    pub thickness: f64,
    pub quantity: u32,
    pub grain_direction: String,
    pub joint_notes: Vec<String>,
    pub node_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CutList {
    pub entries: Vec<CutListEntry>,
    pub total_pieces: u32,
    pub total_cuts: u32,
}

/// Input data for cut list generation.
pub struct CutListInput {
    pub boards: Vec<BoardInfo>,
    pub joints: Vec<JointInfo>,
}

pub struct BoardInfo {
    pub node_id: String,
    pub label: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub material: String,
    pub grain_direction: String,
}

pub struct JointInfo {
    pub joint_type: String,
    pub board_a_id: String,
    pub board_b_id: String,
    pub description: String,
}

/// Key for grouping identical boards in the cut list.
#[derive(Hash, PartialEq, Eq)]
struct BoardGroupKey {
    material: String,
    /// Dimensions encoded as integer thousandths to allow hashing.
    width_thou: i64,
    height_thou: i64,
    depth_thou: i64,
    grain_direction: String,
}

fn to_thousandths(v: f64) -> i64 {
    (v * 1000.0).round() as i64
}

/// Generate cut list from project data.
///
/// Groups identical boards (same material, dimensions, and grain direction),
/// assigns piece IDs (A1, A2, B1...), and collects joint notes per board.
pub fn generate_cut_list(input: &CutListInput) -> CutList {
    if input.boards.is_empty() {
        return CutList {
            entries: Vec::new(),
            total_pieces: 0,
            total_cuts: 0,
        };
    }

    // Build joint notes lookup: node_id -> list of descriptions
    let mut joint_notes_map: HashMap<&str, Vec<String>> = HashMap::new();
    for joint in &input.joints {
        let note = format!("{}: {}", joint.joint_type, joint.description);
        joint_notes_map
            .entry(&joint.board_a_id)
            .or_default()
            .push(note.clone());
        joint_notes_map
            .entry(&joint.board_b_id)
            .or_default()
            .push(note);
    }

    // Group boards by material + dimensions + grain
    let mut groups: Vec<(BoardGroupKey, Vec<&BoardInfo>)> = Vec::new();
    let mut group_index: HashMap<BoardGroupKey, usize> = HashMap::new();

    for board in &input.boards {
        let key = BoardGroupKey {
            material: board.material.clone(),
            width_thou: to_thousandths(board.width),
            height_thou: to_thousandths(board.height),
            depth_thou: to_thousandths(board.depth),
            grain_direction: board.grain_direction.clone(),
        };

        // We need to check membership without consuming the key
        let key_for_lookup = BoardGroupKey {
            material: board.material.clone(),
            width_thou: to_thousandths(board.width),
            height_thou: to_thousandths(board.height),
            depth_thou: to_thousandths(board.depth),
            grain_direction: board.grain_direction.clone(),
        };

        if let Some(&idx) = group_index.get(&key_for_lookup) {
            groups[idx].1.push(board);
        } else {
            let idx = groups.len();
            group_index.insert(key, idx);
            groups.push((key_for_lookup, vec![board]));
        }
    }

    // Build entries with piece IDs: A1, A2, B1, B2, etc.
    let mut entries = Vec::new();
    let mut total_pieces: u32 = 0;

    for (group_idx, (_key, boards)) in groups.iter().enumerate() {
        let letter = group_letter(group_idx);
        let representative = boards[0];
        let quantity = boards.len() as u32;

        // Collect all joint notes for boards in this group
        let mut all_joint_notes: Vec<String> = Vec::new();
        let mut all_node_ids: Vec<String> = Vec::new();

        for (i, board) in boards.iter().enumerate() {
            all_node_ids.push(board.node_id.clone());
            if let Some(notes) = joint_notes_map.get(board.node_id.as_str()) {
                for note in notes {
                    let tagged = format!("{}{}: {}", letter, i + 1, note);
                    all_joint_notes.push(tagged);
                }
            }
        }

        // Deduplicate identical joint notes (can happen with grouped boards)
        all_joint_notes.sort();
        all_joint_notes.dedup();

        entries.push(CutListEntry {
            piece_id: if quantity == 1 {
                format!("{}{}", letter, 1)
            } else {
                format!("{}{}-{}{}", letter, 1, letter, quantity)
            },
            label: representative.label.clone(),
            material: representative.material.clone(),
            length: representative.depth,
            width: representative.width,
            thickness: representative.height,
            quantity,
            grain_direction: representative.grain_direction.clone(),
            joint_notes: all_joint_notes,
            node_ids: all_node_ids,
        });

        total_pieces += quantity;
    }

    // Each piece needs at least one cut to length; additional cuts for
    // pieces that differ from stock widths are captured in joint notes.
    let total_cuts = total_pieces;

    CutList {
        entries,
        total_pieces,
        total_cuts,
    }
}

/// Convert a group index (0-based) into letter(s): 0->A, 1->B, ..., 25->Z, 26->AA, etc.
fn group_letter(index: usize) -> String {
    let mut result = String::new();
    let mut n = index;
    loop {
        result.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    result
}

// ========== Material List ==========

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialListEntry {
    pub material: String,
    pub board_feet: f64,
    pub board_feet_with_waste: f64,
    pub pieces: Vec<PieceSummary>,
    pub suggested_stock: Vec<StockSuggestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PieceSummary {
    pub label: String,
    pub quantity: u32,
    pub dimensions: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockSuggestion {
    pub lumber_label: String,
    pub quantity: u32,
    pub reasoning: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialList {
    pub entries: Vec<MaterialListEntry>,
    pub total_board_feet: f64,
    pub waste_factor: f64,
    pub total_with_waste: f64,
}

/// Calculate board feet from dimensions (in inches).
///
/// Board feet = (thickness x width x length) / 144
pub fn calculate_board_feet(thickness: f64, width: f64, length: f64) -> f64 {
    (thickness * width * length) / 144.0
}

/// Format a dimension in inches to a fractional string.
/// e.g., 1.5 -> "1-1/2\"", 3.5 -> "3-1/2\"", 96.0 -> "96\""
fn format_dimension(inches: f64) -> String {
    let whole = inches.floor() as i64;
    let frac = inches - whole as f64;

    if frac.abs() < 0.001 {
        format!("{}\"", whole)
    } else if (frac - 0.25).abs() < 0.001 {
        format!("{}-1/4\"", whole)
    } else if (frac - 0.5).abs() < 0.001 {
        format!("{}-1/2\"", whole)
    } else if (frac - 0.75).abs() < 0.001 {
        format!("{}-3/4\"", whole)
    } else if (frac - 0.125).abs() < 0.001 {
        format!("{}-1/8\"", whole)
    } else if (frac - 0.375).abs() < 0.001 {
        format!("{}-3/8\"", whole)
    } else if (frac - 0.625).abs() < 0.001 {
        format!("{}-5/8\"", whole)
    } else if (frac - 0.875).abs() < 0.001 {
        format!("{}-7/8\"", whole)
    } else {
        format!("{:.2}\"", inches)
    }
}

/// Generate material list.
///
/// Aggregates board feet by material, applies waste factor, and suggests
/// stock sizes from standard lumber lengths.
pub fn generate_material_list(
    input: &CutListInput,
    waste_factor: f64,
    standard_lengths: &[f64],
) -> MaterialList {
    if input.boards.is_empty() {
        return MaterialList {
            entries: Vec::new(),
            total_board_feet: 0.0,
            waste_factor,
            total_with_waste: 0.0,
        };
    }

    // Group boards by material
    let mut material_groups: HashMap<String, Vec<&BoardInfo>> = HashMap::new();
    for board in &input.boards {
        material_groups
            .entry(board.material.clone())
            .or_default()
            .push(board);
    }

    let mut entries = Vec::new();
    let mut total_board_feet = 0.0;

    let mut materials: Vec<String> = material_groups.keys().cloned().collect();
    materials.sort();

    for material in &materials {
        let boards = &material_groups[material];

        // Build piece summaries by grouping identical boards
        let mut piece_groups: HashMap<(i64, i64, i64), (String, u32)> = HashMap::new();
        let mut material_bf = 0.0;

        for board in boards {
            let bf = calculate_board_feet(board.height, board.width, board.depth);
            material_bf += bf;

            let dim_key = (
                to_thousandths(board.height),
                to_thousandths(board.width),
                to_thousandths(board.depth),
            );
            let dims_str = format!(
                "{} x {} x {}",
                format_dimension(board.height),
                format_dimension(board.width),
                format_dimension(board.depth),
            );
            piece_groups
                .entry(dim_key)
                .and_modify(|e| e.1 += 1)
                .or_insert((dims_str, 1));
        }

        let pieces: Vec<PieceSummary> = {
            let mut v: Vec<_> = piece_groups
                .into_iter()
                .map(|(_, (dims, qty))| PieceSummary {
                    label: boards[0].label.clone(),
                    quantity: qty,
                    dimensions: dims,
                })
                .collect();
            v.sort_by(|a, b| b.quantity.cmp(&a.quantity));
            v
        };

        let bf_with_waste = material_bf * (1.0 + waste_factor);

        // Suggest stock sizes
        let suggested_stock =
            suggest_stock(boards, standard_lengths);

        entries.push(MaterialListEntry {
            material: material.clone(),
            board_feet: material_bf,
            board_feet_with_waste: bf_with_waste,
            pieces,
            suggested_stock,
        });

        total_board_feet += material_bf;
    }

    let total_with_waste = total_board_feet * (1.0 + waste_factor);

    MaterialList {
        entries,
        total_board_feet,
        waste_factor,
        total_with_waste,
    }
}

/// Suggest stock boards needed for a set of pieces.
///
/// Algorithm:
/// 1. For each standard length, calculate how many pieces fit end-to-end.
/// 2. Pick the standard length that yields the fewest stock boards.
/// 3. Prefer 8' (96") unless pieces are longer than that.
fn suggest_stock(boards: &[&BoardInfo], standard_lengths: &[f64]) -> Vec<StockSuggestion> {
    if boards.is_empty() || standard_lengths.is_empty() {
        return Vec::new();
    }

    // Group by cross-section (width x height) to pack along length
    let mut cross_groups: HashMap<(i64, i64), Vec<f64>> = HashMap::new();
    for board in boards {
        let key = (to_thousandths(board.width), to_thousandths(board.height));
        cross_groups.entry(key).or_default().push(board.depth);
    }

    let mut suggestions = Vec::new();

    for ((w_thou, h_thou), lengths) in &cross_groups {
        let width = *w_thou as f64 / 1000.0;
        let height = *h_thou as f64 / 1000.0;
        let max_piece_len = lengths.iter().cloned().fold(0.0_f64, f64::max);

        // Filter standard lengths that can fit the longest piece
        let viable: Vec<f64> = standard_lengths
            .iter()
            .copied()
            .filter(|&sl| sl >= max_piece_len)
            .collect();

        if viable.is_empty() {
            // No standard length fits; suggest custom
            suggestions.push(StockSuggestion {
                lumber_label: format!(
                    "Custom {} x {} x {}'+ stock",
                    format_dimension(height),
                    format_dimension(width),
                    (max_piece_len / 12.0).ceil() as u32,
                ),
                quantity: lengths.len() as u32,
                reasoning: format!(
                    "Pieces exceed all standard lengths (max {:.1}\")",
                    max_piece_len
                ),
            });
            continue;
        }

        // For each viable length, calculate total stock boards needed
        let mut best_length = viable[0];
        let mut best_qty = u32::MAX;
        let mut best_per_board = 1u32;

        for &stock_len in &viable {
            // How many pieces fit end-to-end in one stock board?
            // Use a simple greedy: floor(stock_len / piece_len) per unique length
            let total_needed = greedy_pack(lengths, stock_len);
            if total_needed < best_qty
                || (total_needed == best_qty && stock_len < best_length)
            {
                best_length = stock_len;
                best_qty = total_needed;
                best_per_board = (stock_len / max_piece_len).floor().max(1.0) as u32;
            }
        }

        let feet = (best_length / 12.0).round() as u32;
        suggestions.push(StockSuggestion {
            lumber_label: format!(
                "{} x {} x {}'",
                format_dimension(height),
                format_dimension(width),
                feet,
            ),
            quantity: best_qty,
            reasoning: if best_per_board > 1 {
                format!(
                    "Can cut up to {} pieces from each {}' board",
                    best_per_board, feet
                )
            } else {
                format!("1 piece per {}' board", feet)
            },
        });
    }

    suggestions.sort_by(|a, b| a.lumber_label.cmp(&b.lumber_label));
    suggestions
}

/// Greedy bin-packing: given a list of piece lengths and a stock length,
/// return the minimum number of stock boards using first-fit-decreasing.
fn greedy_pack(piece_lengths: &[f64], stock_length: f64) -> u32 {
    let mut sorted: Vec<f64> = piece_lengths.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut bins: Vec<f64> = Vec::new(); // remaining capacity per bin

    for piece in &sorted {
        // Find first bin that fits
        let mut placed = false;
        for remaining in bins.iter_mut() {
            if *remaining >= *piece {
                *remaining -= *piece;
                placed = true;
                break;
            }
        }
        if !placed {
            bins.push(stock_length - *piece);
        }
    }

    bins.len() as u32
}

// ========== Build Instructions ==========

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildStep {
    pub step_number: u32,
    pub phase: BuildPhase,
    pub description: String,
    pub pieces_involved: Vec<String>,
    pub joint_type: Option<String>,
    pub tools_needed: Vec<String>,
    pub tips: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildPhase {
    Preparation,
    Cutting,
    JoineryPrep,
    SubAssembly,
    FinalAssembly,
    Finishing,
}

impl std::fmt::Display for BuildPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildPhase::Preparation => write!(f, "Preparation"),
            BuildPhase::Cutting => write!(f, "Cutting"),
            BuildPhase::JoineryPrep => write!(f, "Joinery Prep"),
            BuildPhase::SubAssembly => write!(f, "Sub-Assembly"),
            BuildPhase::FinalAssembly => write!(f, "Final Assembly"),
            BuildPhase::Finishing => write!(f, "Finishing"),
        }
    }
}

/// Joint types that require preparatory cuts before assembly.
fn is_joinery_prep_joint(joint_type: &str) -> bool {
    let prep_joints = [
        "Dado",
        "Rabbet",
        "Groove",
        "Mortise",
        "Tenon",
        "MortiseAndTenon",
        "HalfLap",
        "CrossLap",
        "BridalJoint",
        "FingerJoint",
        "BoxJoint",
        "Dovetail",
        "SlidingDovetail",
        "ThroughDovetail",
        "HalfBlindDovetail",
        "LockMiter",
        "TonguAndGroove",
        "TongueAndGroove",
    ];
    prep_joints.iter().any(|p| joint_type.contains(p))
}

/// Tools typically needed for a joint type.
fn tools_for_joint(joint_type: &str) -> Vec<String> {
    if joint_type.contains("Dovetail") {
        vec![
            "Dovetail saw".into(),
            "Chisels".into(),
            "Marking gauge".into(),
        ]
    } else if joint_type.contains("Mortise") || joint_type.contains("Tenon") {
        vec![
            "Mortising chisel".into(),
            "Drill press or hand drill".into(),
            "Tenon saw".into(),
        ]
    } else if joint_type.contains("Dado") || joint_type.contains("Rabbet") || joint_type.contains("Groove") {
        vec!["Router or table saw with dado blade".into()]
    } else if joint_type.contains("Miter") {
        vec!["Miter saw or miter box".into()]
    } else if joint_type.contains("Lap") {
        vec!["Table saw or router".into(), "Chisels".into()]
    } else if joint_type.contains("Finger") || joint_type.contains("Box") {
        vec!["Table saw with box joint jig".into()]
    } else if joint_type.contains("Biscuit") {
        vec!["Biscuit joiner".into()]
    } else if joint_type.contains("Dowel") {
        vec!["Doweling jig".into(), "Drill".into()]
    } else if joint_type.contains("Pocket") {
        vec!["Pocket hole jig".into(), "Drill".into()]
    } else {
        vec!["Saw".into(), "Clamps".into()]
    }
}

/// Generate build instructions.
///
/// Algorithm:
/// 1. Preparation phase: gather materials, verify dimensions
/// 2. Cutting phase: cut all pieces (group by material to minimize tool changes)
/// 3. Joinery prep: cut dados, mortises, rabbets, etc. before assembly
/// 4. Sub-assembly: group connected boards, assemble small units first
/// 5. Final assembly: join sub-assemblies together
/// 6. Finishing: apply finishes
pub fn generate_build_instructions(
    input: &CutListInput,
    has_finish: bool,
) -> BuildInstructions {
    if input.boards.is_empty() {
        return BuildInstructions {
            steps: Vec::new(),
            total_steps: 0,
            estimated_phases: Vec::new(),
        };
    }

    let cut_list = generate_cut_list(input);
    let mut steps: Vec<BuildStep> = Vec::new();
    let mut step_num: u32 = 1;

    // --- Phase 1: Preparation ---
    steps.push(BuildStep {
        step_number: step_num,
        phase: BuildPhase::Preparation,
        description: "Gather all materials and verify dimensions against the cut list.".into(),
        pieces_involved: cut_list
            .entries
            .iter()
            .map(|e| e.piece_id.clone())
            .collect(),
        joint_type: None,
        tools_needed: vec!["Tape measure".into(), "Square".into(), "Pencil".into()],
        tips: vec![
            "Check all boards for warping, splits, or defects before cutting.".into(),
            "Mark each board with its piece ID for easy identification.".into(),
        ],
    });
    step_num += 1;

    // --- Phase 2: Cutting ---
    // Group cut list entries by material to minimize tool changes
    let mut material_order: Vec<String> = Vec::new();
    let mut seen_materials: HashMap<String, Vec<&CutListEntry>> = HashMap::new();
    for entry in &cut_list.entries {
        seen_materials
            .entry(entry.material.clone())
            .or_default()
            .push(entry);
        if !material_order.contains(&entry.material) {
            material_order.push(entry.material.clone());
        }
    }

    for material in &material_order {
        let entries = &seen_materials[material];
        let piece_ids: Vec<String> = entries.iter().map(|e| e.piece_id.clone()).collect();
        let descriptions: Vec<String> = entries
            .iter()
            .map(|e| {
                format!(
                    "{}: {} x {} x {} (x{})",
                    e.piece_id,
                    format_dimension(e.thickness),
                    format_dimension(e.width),
                    format_dimension(e.length),
                    e.quantity,
                )
            })
            .collect();

        steps.push(BuildStep {
            step_number: step_num,
            phase: BuildPhase::Cutting,
            description: format!(
                "Cut {} pieces:\n{}",
                material,
                descriptions.join("\n")
            ),
            pieces_involved: piece_ids,
            joint_type: None,
            tools_needed: vec!["Miter saw or table saw".into(), "Tape measure".into()],
            tips: vec![
                "Cut slightly long and trim to final dimension for accuracy.".into(),
                format!("Group all {} cuts together to minimize setup changes.", material),
            ],
        });
        step_num += 1;
    }

    // --- Phase 3: Joinery Prep ---
    let joinery_joints: Vec<&JointInfo> = input
        .joints
        .iter()
        .filter(|j| is_joinery_prep_joint(&j.joint_type))
        .collect();

    for joint in &joinery_joints {
        let board_a_label = input
            .boards
            .iter()
            .find(|b| b.node_id == joint.board_a_id)
            .map(|b| b.label.as_str())
            .unwrap_or("unknown");
        let board_b_label = input
            .boards
            .iter()
            .find(|b| b.node_id == joint.board_b_id)
            .map(|b| b.label.as_str())
            .unwrap_or("unknown");

        steps.push(BuildStep {
            step_number: step_num,
            phase: BuildPhase::JoineryPrep,
            description: format!(
                "Prepare {} joint between \"{}\" and \"{}\": {}",
                joint.joint_type, board_a_label, board_b_label, joint.description,
            ),
            pieces_involved: vec![
                joint.board_a_id.clone(),
                joint.board_b_id.clone(),
            ],
            joint_type: Some(joint.joint_type.clone()),
            tools_needed: tools_for_joint(&joint.joint_type),
            tips: vec![
                "Test fit joints before applying glue.".into(),
                "Use scrap wood for practice cuts when trying a new joint.".into(),
            ],
        });
        step_num += 1;
    }

    // --- Phase 4 & 5: Assembly ---
    // Build an adjacency list from joints to find connected components (sub-assemblies).
    let assembly_joints: Vec<&JointInfo> = input.joints.iter().collect();

    if !assembly_joints.is_empty() {
        // Find connected components using union-find
        let board_ids: Vec<&str> = input.boards.iter().map(|b| b.node_id.as_str()).collect();
        let components = find_connected_components(&board_ids, &assembly_joints);

        if components.len() > 1 {
            // Multiple sub-assemblies: build each, then join
            for (comp_idx, component) in components.iter().enumerate() {
                let comp_joints: Vec<&&JointInfo> = assembly_joints
                    .iter()
                    .filter(|j| {
                        component.contains(&j.board_a_id.as_str())
                            || component.contains(&j.board_b_id.as_str())
                    })
                    .collect();

                let joint_descs: Vec<String> = comp_joints
                    .iter()
                    .map(|j| format!("{} ({} + {})", j.joint_type, j.board_a_id, j.board_b_id))
                    .collect();

                steps.push(BuildStep {
                    step_number: step_num,
                    phase: BuildPhase::SubAssembly,
                    description: format!(
                        "Assemble sub-unit {} ({} boards):\n{}",
                        comp_idx + 1,
                        component.len(),
                        joint_descs.join("\n"),
                    ),
                    pieces_involved: component.iter().map(|s| s.to_string()).collect(),
                    joint_type: None,
                    tools_needed: vec!["Clamps".into(), "Wood glue".into(), "Square".into()],
                    tips: vec![
                        "Dry-fit all pieces before applying glue.".into(),
                        "Check for square after clamping.".into(),
                    ],
                });
                step_num += 1;
            }

            steps.push(BuildStep {
                step_number: step_num,
                phase: BuildPhase::FinalAssembly,
                description: format!(
                    "Join all {} sub-assemblies together.",
                    components.len()
                ),
                pieces_involved: input.boards.iter().map(|b| b.node_id.clone()).collect(),
                joint_type: None,
                tools_needed: vec![
                    "Clamps".into(),
                    "Wood glue".into(),
                    "Square".into(),
                    "Mallet".into(),
                ],
                tips: vec![
                    "Work on a flat surface to ensure the assembly stays true.".into(),
                    "Allow adequate drying time between sub-assembly joins.".into(),
                ],
            });
            step_num += 1;
        } else {
            // Single connected component: one assembly step
            let joint_descs: Vec<String> = assembly_joints
                .iter()
                .map(|j| format!("{} ({} + {})", j.joint_type, j.board_a_id, j.board_b_id))
                .collect();

            steps.push(BuildStep {
                step_number: step_num,
                phase: BuildPhase::FinalAssembly,
                description: format!(
                    "Assemble all pieces:\n{}",
                    joint_descs.join("\n"),
                ),
                pieces_involved: input.boards.iter().map(|b| b.node_id.clone()).collect(),
                joint_type: None,
                tools_needed: vec!["Clamps".into(), "Wood glue".into(), "Square".into()],
                tips: vec![
                    "Dry-fit all pieces before applying glue.".into(),
                    "Check for square after clamping.".into(),
                ],
            });
            step_num += 1;
        }
    }

    // --- Phase 6: Finishing ---
    if has_finish {
        steps.push(BuildStep {
            step_number: step_num,
            phase: BuildPhase::Finishing,
            description: "Sand all surfaces (120, 180, then 220 grit). Apply finish as desired."
                .into(),
            pieces_involved: Vec::new(),
            joint_type: None,
            tools_needed: vec![
                "Sandpaper (120, 180, 220 grit)".into(),
                "Tack cloth".into(),
                "Brush or applicator".into(),
            ],
            tips: vec![
                "Sand with the grain to avoid scratches.".into(),
                "Apply finish in thin, even coats and allow proper drying time between coats."
                    .into(),
            ],
        });
        step_num += 1;
    }

    // Count steps per phase
    let mut phase_counts: HashMap<BuildPhase, u32> = HashMap::new();
    for step in &steps {
        *phase_counts.entry(step.phase).or_insert(0) += 1;
    }

    let phase_order = [
        BuildPhase::Preparation,
        BuildPhase::Cutting,
        BuildPhase::JoineryPrep,
        BuildPhase::SubAssembly,
        BuildPhase::FinalAssembly,
        BuildPhase::Finishing,
    ];

    let estimated_phases: Vec<(BuildPhase, u32)> = phase_order
        .iter()
        .filter_map(|p| phase_counts.get(p).map(|c| (*p, *c)))
        .collect();

    let total_steps = step_num - 1;

    BuildInstructions {
        steps,
        total_steps,
        estimated_phases,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildInstructions {
    pub steps: Vec<BuildStep>,
    pub total_steps: u32,
    pub estimated_phases: Vec<(BuildPhase, u32)>,
}

/// Find connected components among boards via their joints (union-find).
fn find_connected_components<'a>(
    board_ids: &[&'a str],
    joints: &[&JointInfo],
) -> Vec<Vec<&'a str>> {
    // Map board id -> index
    let id_to_idx: HashMap<&str, usize> = board_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();

    let n = board_ids.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = vec![0; n];

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra == rb {
            return;
        }
        if rank[ra] < rank[rb] {
            parent[ra] = rb;
        } else if rank[ra] > rank[rb] {
            parent[rb] = ra;
        } else {
            parent[rb] = ra;
            rank[ra] += 1;
        }
    }

    for joint in joints {
        if let (Some(&a), Some(&b)) = (
            id_to_idx.get(joint.board_a_id.as_str()),
            id_to_idx.get(joint.board_b_id.as_str()),
        ) {
            union(&mut parent, &mut rank, a, b);
        }
    }

    let mut components: HashMap<usize, Vec<&'a str>> = HashMap::new();
    for (i, id) in board_ids.iter().enumerate() {
        let root = find(&mut parent, i);
        components.entry(root).or_default().push(id);
    }

    let mut result: Vec<Vec<&'a str>> = components.into_values().collect();
    // Sort by size descending for consistent ordering (largest sub-assembly last)
    result.sort_by(|a, b| a.len().cmp(&b.len()));
    result
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board(id: &str, label: &str, material: &str, w: f64, h: f64, d: f64) -> BoardInfo {
        BoardInfo {
            node_id: id.into(),
            label: label.into(),
            width: w,
            height: h,
            depth: d,
            material: material.into(),
            grain_direction: "AlongLength".into(),
        }
    }

    fn make_joint(jtype: &str, a: &str, b: &str, desc: &str) -> JointInfo {
        JointInfo {
            joint_type: jtype.into(),
            board_a_id: a.into(),
            board_b_id: b.into(),
            description: desc.into(),
        }
    }

    #[test]
    fn test_cut_list_grouping() {
        let input = CutListInput {
            boards: vec![
                make_board("n1", "Side", "Oak", 3.5, 0.75, 24.0),
                make_board("n2", "Side", "Oak", 3.5, 0.75, 24.0),
                make_board("n3", "Side", "Oak", 3.5, 0.75, 24.0),
            ],
            joints: vec![],
        };

        let cut_list = generate_cut_list(&input);
        assert_eq!(cut_list.entries.len(), 1, "3 identical boards should group into 1 entry");
        assert_eq!(cut_list.entries[0].quantity, 3);
        assert_eq!(cut_list.total_pieces, 3);
        assert_eq!(cut_list.entries[0].piece_id, "A1-A3");
    }

    #[test]
    fn test_board_feet_calculation() {
        // 1x12x12" = 1 board foot
        let bf = calculate_board_feet(1.0, 12.0, 12.0);
        assert!((bf - 1.0).abs() < 0.001, "1x12x12 should be 1 board foot, got {}", bf);

        // 2x4x96" (actual 1.5 x 3.5 x 96)
        let bf2 = calculate_board_feet(1.5, 3.5, 96.0);
        let expected = (1.5 * 3.5 * 96.0) / 144.0;
        assert!((bf2 - expected).abs() < 0.001);
    }

    #[test]
    fn test_material_list_waste() {
        let input = CutListInput {
            boards: vec![
                make_board("n1", "Shelf", "Pine", 5.5, 0.75, 36.0),
                make_board("n2", "Shelf", "Pine", 5.5, 0.75, 36.0),
            ],
            joints: vec![],
        };

        let waste_factor = 0.15;
        let ml = generate_material_list(&input, waste_factor, &[96.0, 120.0]);

        assert_eq!(ml.entries.len(), 1);
        assert_eq!(ml.waste_factor, 0.15);

        let entry = &ml.entries[0];
        let expected_bf = 2.0 * calculate_board_feet(0.75, 5.5, 36.0);
        assert!(
            (entry.board_feet - expected_bf).abs() < 0.001,
            "Board feet mismatch: got {}, expected {}",
            entry.board_feet,
            expected_bf,
        );
        let expected_with_waste = expected_bf * 1.15;
        assert!(
            (entry.board_feet_with_waste - expected_with_waste).abs() < 0.001,
            "Waste factor not applied correctly: got {}, expected {}",
            entry.board_feet_with_waste,
            expected_with_waste,
        );
        assert!(
            (ml.total_with_waste - expected_with_waste).abs() < 0.001,
            "Total with waste mismatch",
        );
    }

    #[test]
    fn test_instructions_order() {
        let input = CutListInput {
            boards: vec![
                make_board("n1", "Top", "Maple", 11.25, 0.75, 48.0),
                make_board("n2", "Leg", "Maple", 1.5, 1.5, 30.0),
            ],
            joints: vec![make_joint(
                "MortiseAndTenon",
                "n1",
                "n2",
                "1\" x 1\" x 2\" deep mortise",
            )],
        };

        let instructions = generate_build_instructions(&input, true);

        assert!(instructions.total_steps >= 4, "Expected at least 4 steps");

        // Verify phase ordering: Preparation < Cutting < JoineryPrep < Assembly < Finishing
        let phases: Vec<BuildPhase> = instructions.steps.iter().map(|s| s.phase).collect();

        let phase_order_value = |p: &BuildPhase| -> u8 {
            match p {
                BuildPhase::Preparation => 0,
                BuildPhase::Cutting => 1,
                BuildPhase::JoineryPrep => 2,
                BuildPhase::SubAssembly => 3,
                BuildPhase::FinalAssembly => 4,
                BuildPhase::Finishing => 5,
            }
        };

        for window in phases.windows(2) {
            assert!(
                phase_order_value(&window[0]) <= phase_order_value(&window[1]),
                "Phases out of order: {:?} should come before {:?}",
                window[0],
                window[1],
            );
        }

        // Check that finishing is the last step
        assert_eq!(phases.last(), Some(&BuildPhase::Finishing));
    }

    #[test]
    fn test_empty_project() {
        let input = CutListInput {
            boards: vec![],
            joints: vec![],
        };

        let cut_list = generate_cut_list(&input);
        assert_eq!(cut_list.entries.len(), 0);
        assert_eq!(cut_list.total_pieces, 0);

        let ml = generate_material_list(&input, 0.15, &[96.0]);
        assert_eq!(ml.entries.len(), 0);
        assert_eq!(ml.total_board_feet, 0.0);

        let instructions = generate_build_instructions(&input, false);
        assert_eq!(instructions.steps.len(), 0);
        assert_eq!(instructions.total_steps, 0);
    }

    #[test]
    fn test_group_letter() {
        assert_eq!(group_letter(0), "A");
        assert_eq!(group_letter(1), "B");
        assert_eq!(group_letter(25), "Z");
        assert_eq!(group_letter(26), "AA");
        assert_eq!(group_letter(27), "AB");
    }

    #[test]
    fn test_format_dimension() {
        assert_eq!(format_dimension(1.5), "1-1/2\"");
        assert_eq!(format_dimension(3.5), "3-1/2\"");
        assert_eq!(format_dimension(96.0), "96\"");
        assert_eq!(format_dimension(0.75), "0-3/4\"");
    }

    #[test]
    fn test_greedy_pack() {
        // Two 36" pieces should fit in one 96" board
        assert_eq!(greedy_pack(&[36.0, 36.0], 96.0), 1);

        // Three 36" pieces: two fit in one, third needs another
        assert_eq!(greedy_pack(&[36.0, 36.0, 36.0], 96.0), 2);

        // Four 50" pieces: each only fits one per 96" board
        assert_eq!(greedy_pack(&[50.0, 50.0, 50.0, 50.0], 96.0), 4);

        // Mix: 60" + 30" fit together, 60" + 30" fit together
        assert_eq!(greedy_pack(&[60.0, 60.0, 30.0, 30.0], 96.0), 2);
    }

    #[test]
    fn test_connected_components() {
        let joints = vec![
            JointInfo {
                joint_type: "ButtJoint".into(),
                board_a_id: "a".into(),
                board_b_id: "b".into(),
                description: "".into(),
            },
            JointInfo {
                joint_type: "ButtJoint".into(),
                board_a_id: "c".into(),
                board_b_id: "d".into(),
                description: "".into(),
            },
        ];
        let joint_refs: Vec<&JointInfo> = joints.iter().collect();
        let boards = vec!["a", "b", "c", "d", "e"];
        let components = find_connected_components(&boards, &joint_refs);

        // Should be 3 components: {a,b}, {c,d}, {e}
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_multiple_materials_cut_list() {
        let input = CutListInput {
            boards: vec![
                make_board("n1", "Side", "Oak", 3.5, 0.75, 24.0),
                make_board("n2", "Top", "Maple", 11.25, 0.75, 48.0),
                make_board("n3", "Side", "Oak", 3.5, 0.75, 24.0),
            ],
            joints: vec![],
        };

        let cut_list = generate_cut_list(&input);
        assert_eq!(cut_list.entries.len(), 2, "Should have 2 groups (Oak + Maple)");
        assert_eq!(cut_list.total_pieces, 3);
    }

    #[test]
    fn test_joint_notes_in_cut_list() {
        let input = CutListInput {
            boards: vec![
                make_board("n1", "Side", "Oak", 3.5, 0.75, 24.0),
                make_board("n2", "Shelf", "Oak", 3.5, 0.75, 20.0),
            ],
            joints: vec![make_joint(
                "Dado",
                "n1",
                "n2",
                "3/4\" wide x 3/8\" deep",
            )],
        };

        let cut_list = generate_cut_list(&input);
        // Both entries should have joint notes
        let all_notes: Vec<&String> = cut_list
            .entries
            .iter()
            .flat_map(|e| e.joint_notes.iter())
            .collect();
        assert!(!all_notes.is_empty(), "Joint notes should be populated");
    }
}
