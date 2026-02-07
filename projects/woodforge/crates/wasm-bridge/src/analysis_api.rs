use wasm_bindgen::prelude::*;

use woodforge_core::{
    cost,
    optimizer::{self, CutPiece, StockBoard},
    output::{self, BoardInfo, CutListInput, JointInfo},
    structural,
};

use crate::APP;

// ========== Structural Analysis ==========

#[wasm_bindgen]
pub fn analyze_shelf_deflection(
    span: f64,
    board_width: f64,
    board_height: f64,
    species_id: &str,
    total_load: f64,
) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let moe = state
            .species_db
            .find_by_id(species_id)
            .map(|s| s.modulus_of_elasticity)
            .unwrap_or(1_700_000.0);

        let result = structural::analyze_shelf(span, board_width, board_height, moe, total_load);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn analyze_compression(
    length: f64,
    width: f64,
    height: f64,
    species_id: &str,
    load: f64,
) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let species = state.species_db.find_by_id(species_id);
        let moe = species.map(|s| s.modulus_of_elasticity).unwrap_or(1_700_000.0);
        let cs = species.map(|s| s.compressive_strength).unwrap_or(5_000.0);

        let result = structural::calculate_compression(length, width, height, moe, cs, load);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn analyze_joint_strength(joint_type_str: &str, species_id: &str) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let density = state
            .species_db
            .find_by_id(species_id)
            .map(|s| s.density_lb_ft3)
            .unwrap_or(35.0);

        let base_strength = match joint_type_str {
            "butt" => 15,
            "miter" => 20,
            "pocket_hole" => 45,
            "dado" => 55,
            "rabbet" => 40,
            "half_lap" => 50,
            "mortise_tenon" => 85,
            "dovetail_through" => 90,
            "dovetail_half_blind" => 85,
            "box_joint" => 70,
            "finger_joint" => 75,
            "dowel" => 55,
            "biscuit" => 45,
            "tongue_groove" => 50,
            "bridle" => 65,
            _ => 30,
        };

        let species = state.species_db.find_by_id(species_id);
        let janka = species.map(|s| s.janka_hardness).unwrap_or(870);
        let moe = species.map(|s| s.modulus_of_elasticity).unwrap_or(1_700_000.0);

        let result = structural::calculate_joint_strength(base_strength, janka, density, moe);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

// ========== Output Generation ==========

fn build_cut_list_input(
    app: &std::cell::Ref<crate::AppState>,
) -> CutListInput {
    let boards: Vec<BoardInfo> = app
        .scene
        .all_nodes()
        .filter_map(|node| {
            let comp = node.component.as_ref()?;
            let species_name = app
                .board_species
                .get(&node.id.to_string())
                .and_then(|sid| app.species_db.find_by_id(sid))
                .map(|s| s.name.to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            Some(BoardInfo {
                node_id: node.id.to_string(),
                label: node.label.clone(),
                width: comp.dimensions.width,
                height: comp.dimensions.height,
                depth: comp.dimensions.depth,
                material: species_name,
                grain_direction: format!("{:?}", comp.grain_direction),
            })
        })
        .collect();

    let joints: Vec<JointInfo> = app
        .joint_store
        .all_joints()
        .iter()
        .map(|j| JointInfo {
            joint_type: format!("{:?}", j.joint_type),
            board_a_id: j.board_a.to_string(),
            board_b_id: j.board_b.to_string(),
            description: format!("{:?} on {:?}/{:?}", j.joint_type, j.face_a, j.face_b),
        })
        .collect();

    CutListInput { boards, joints }
}

#[wasm_bindgen]
pub fn generate_cut_list() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        let input = build_cut_list_input(&state);
        let result = output::generate_cut_list(&input);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn generate_material_list() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        let input = build_cut_list_input(&state);
        let result = output::generate_material_list(&input, 0.15, &state.catalog.standard_lengths);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn generate_build_instructions() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        let input = build_cut_list_input(&state);
        let has_finish = !state.board_finishes.is_empty();
        let result = output::generate_build_instructions(&input, has_finish);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

// ========== Cut Optimization ==========

#[wasm_bindgen]
pub fn optimize_cut_layout(kerf_width: f64) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let pieces: Vec<CutPiece> = state
            .scene
            .all_nodes()
            .filter_map(|node| {
                let comp = node.component.as_ref()?;
                let species = state
                    .board_species
                    .get(&node.id.to_string())
                    .and_then(|sid| state.species_db.find_by_id(sid))
                    .map(|s| s.name.to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                Some(CutPiece {
                    id: node.id.to_string(),
                    label: node.label.clone(),
                    length: comp.dimensions.depth,
                    width: comp.dimensions.width,
                    material: species,
                })
            })
            .collect();

        let stock: Vec<StockBoard> = state
            .catalog
            .standard_lengths
            .iter()
            .map(|&len| StockBoard {
                label: format!("{:.0}\"", len),
                length: len,
                width: 12.0,
                height: 1.0,
            })
            .collect();

        let result = optimizer::optimize_cuts(&pieces, &stock, kerf_width);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

// ========== Cost Estimation ==========

#[wasm_bindgen]
pub fn estimate_project_cost() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        // Collect material board feet by species
        let mut species_bf: std::collections::HashMap<String, (String, f64)> =
            std::collections::HashMap::new();

        for node in state.scene.all_nodes() {
            if let Some(ref comp) = node.component {
                let species_id = state
                    .board_species
                    .get(&node.id.to_string())
                    .cloned()
                    .unwrap_or_else(|| "pine".to_string());
                let species_name = state
                    .species_db
                    .find_by_id(&species_id)
                    .map(|s| s.name.to_string())
                    .unwrap_or_else(|| "Pine".to_string());

                // Board feet = (width * height * depth) / 144
                let bf = (comp.dimensions.width * comp.dimensions.height * comp.dimensions.depth) / 144.0;
                let entry = species_bf.entry(species_id).or_insert((species_name, 0.0));
                entry.1 += bf;
            }
        }

        let material_bf: Vec<(String, String, f64)> = species_bf
            .into_iter()
            .map(|(sid, (name, bf))| (sid, name, bf))
            .collect();

        // Collect joint hardware
        let mut joint_counts: std::collections::HashMap<String, (String, u32)> =
            std::collections::HashMap::new();
        for joint in state.joint_store.all_joints() {
            let key = format!("{:?}", joint.joint_type);
            let display = format!("{:?}", joint.joint_type);
            let entry = joint_counts.entry(key).or_insert((display, 0));
            entry.1 += 1;
        }

        let joints: Vec<(String, String, u32)> = joint_counts
            .into_iter()
            .map(|(jt, (name, count))| (jt, name, count))
            .collect();

        let result = cost::estimate_costs(&material_bf, &joints, &state.price_db, 0.15);
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn get_species_prices() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        serde_wasm_bindgen::to_value(&state.price_db).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn set_species_price(species_id: &str, price_per_bf: f64) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        state.price_db.set_price(species_id, price_per_bf);
    });
}
