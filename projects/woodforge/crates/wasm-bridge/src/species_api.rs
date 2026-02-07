use wasm_bindgen::prelude::*;

use woodforge_core::{
    finish::{self, FinishType, OilFinish, PaintFinish, Sheen, StainFinish},
    grain::{self, GrainParams},
};

use crate::APP;

#[wasm_bindgen]
pub fn get_all_species() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        serde_wasm_bindgen::to_value(state.species_db.all_species()).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn get_species(species_id: &str) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        match state.species_db.find_by_id(species_id) {
            Some(s) => serde_wasm_bindgen::to_value(s).unwrap_or(JsValue::NULL),
            None => JsValue::NULL,
        }
    })
}

#[wasm_bindgen]
pub fn set_board_species(board_id: &str, species_id: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let species = match state.species_db.find_by_id(species_id) {
            Some(s) => s.clone(),
            None => return JsValue::NULL,
        };

        let old_species = state
            .board_species
            .insert(board_id.to_string(), species_id.to_string());

        // Record undo operation
        state.history.record(woodforge_core::history::Operation::SetSpecies {
            node_id: board_id.to_string(),
            old_species,
            new_species: Some(species_id.to_string()),
        });

        // Update renderer color
        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, species.color, false);
            }
        }

        // Generate grain texture and update renderer
        let grain_pixels = grain::generate_grain_texture(
            256,
            256,
            species.color,
            &GrainParams {
                ring_frequency: 8.0 * species.grain_scale,
                ..Default::default()
            },
            42,
        );

        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref mut renderer) = &mut state.renderer {
                renderer.update_grain_texture(node_id, &grain_pixels, 256, 256);
            }
        }

        serde_wasm_bindgen::to_value(&species).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn clear_board_species(board_id: &str) -> bool {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        let old = state.board_species.remove(board_id);

        state.history.record(woodforge_core::history::Operation::SetSpecies {
            node_id: board_id.to_string(),
            old_species: old,
            new_species: None,
        });

        // Reset to default color
        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, [0.76, 0.60, 0.42], false);
            }
        }
        true
    })
}

#[wasm_bindgen]
pub fn get_board_species(board_id: &str) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        match state.board_species.get(board_id) {
            Some(species_id) => match state.species_db.find_by_id(species_id) {
                Some(s) => serde_wasm_bindgen::to_value(s).unwrap_or(JsValue::NULL),
                None => JsValue::NULL,
            },
            None => JsValue::NULL,
        }
    })
}

#[wasm_bindgen]
pub fn apply_stain_finish(board_id: &str, name: &str, r: f32, g: f32, b: f32, opacity: f32) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let finish = FinishType::Stain(StainFinish {
            name: name.to_string(),
            color: [r, g, b],
            opacity,
        });

        // Get base color
        let base_color = state
            .board_species
            .get(board_id)
            .and_then(|sid| state.species_db.find_by_id(sid))
            .map(|s| s.color)
            .unwrap_or([0.76, 0.60, 0.42]);

        let result_color = finish::apply_finish(base_color, &finish);

        let finish_json = serde_json::to_string(&finish).unwrap_or_default();
        let old_finish = state
            .board_finishes
            .insert(board_id.to_string(), finish_json.clone());

        state.history.record(woodforge_core::history::Operation::SetFinish {
            node_id: board_id.to_string(),
            old_finish,
            new_finish: Some(finish_json),
        });

        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, result_color, false);
            }
        }

        serde_wasm_bindgen::to_value(&result_color).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn apply_paint_finish(board_id: &str, name: &str, r: f32, g: f32, b: f32, sheen: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let sheen_val = match sheen {
            "matte" => Sheen::Matte,
            "satin" => Sheen::Satin,
            "semi-gloss" => Sheen::SemiGloss,
            "gloss" => Sheen::Gloss,
            _ => Sheen::Satin,
        };

        let finish = FinishType::Paint(PaintFinish {
            name: name.to_string(),
            color: [r, g, b],
            sheen: sheen_val,
        });

        let base_color = state
            .board_species
            .get(board_id)
            .and_then(|sid| state.species_db.find_by_id(sid))
            .map(|s| s.color)
            .unwrap_or([0.76, 0.60, 0.42]);

        let result_color = finish::apply_finish(base_color, &finish);

        let finish_json = serde_json::to_string(&finish).unwrap_or_default();
        let old_finish = state
            .board_finishes
            .insert(board_id.to_string(), finish_json.clone());

        state.history.record(woodforge_core::history::Operation::SetFinish {
            node_id: board_id.to_string(),
            old_finish,
            new_finish: Some(finish_json),
        });

        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, result_color, false);
            }
        }

        serde_wasm_bindgen::to_value(&result_color).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn apply_oil_finish(board_id: &str, name: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let finish = FinishType::Oil(OilFinish {
            name: name.to_string(),
            color_shift: [0.05, 0.03, -0.02],
            enhances_grain: true,
        });

        let base_color = state
            .board_species
            .get(board_id)
            .and_then(|sid| state.species_db.find_by_id(sid))
            .map(|s| s.color)
            .unwrap_or([0.76, 0.60, 0.42]);

        let result_color = finish::apply_finish(base_color, &finish);

        let finish_json = serde_json::to_string(&finish).unwrap_or_default();
        let old_finish = state
            .board_finishes
            .insert(board_id.to_string(), finish_json.clone());

        state.history.record(woodforge_core::history::Operation::SetFinish {
            node_id: board_id.to_string(),
            old_finish,
            new_finish: Some(finish_json),
        });

        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, result_color, false);
            }
        }

        serde_wasm_bindgen::to_value(&result_color).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn clear_finish(board_id: &str) -> bool {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let old_finish = state.board_finishes.remove(board_id);

        state.history.record(woodforge_core::history::Operation::SetFinish {
            node_id: board_id.to_string(),
            old_finish,
            new_finish: None,
        });

        // Reset to species color or default
        let color = state
            .board_species
            .get(board_id)
            .and_then(|sid| state.species_db.find_by_id(sid))
            .map(|s| s.color)
            .unwrap_or([0.76, 0.60, 0.42]);

        if let Ok(node_id) = board_id.parse::<woodforge_core::types::NodeId>() {
            if let Some(ref renderer) = state.renderer {
                renderer.update_color(node_id, color, false);
            }
        }
        true
    })
}
