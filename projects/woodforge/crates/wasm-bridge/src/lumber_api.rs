use wasm_bindgen::prelude::*;

use woodforge_core::units::{self, UnitSystem};

use crate::APP;

#[wasm_bindgen]
pub fn get_lumber_catalog() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        serde_wasm_bindgen::to_value(&state.catalog).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn get_standard_lengths() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        serde_wasm_bindgen::to_value(&state.catalog.standard_lengths).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn format_dim(inches: f64) -> String {
    APP.with(|app| {
        let state = app.borrow();
        units::format_dimension(inches, state.unit_system)
    })
}

#[wasm_bindgen]
pub fn parse_dim(input: &str) -> JsValue {
    match units::parse_dimension(input) {
        Some(inches) => JsValue::from_f64(inches),
        None => JsValue::NULL,
    }
}

#[wasm_bindgen]
pub fn set_unit_system(system: &str) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        state.unit_system = match system {
            "metric" => UnitSystem::Metric,
            _ => UnitSystem::Imperial,
        };
    });
}

#[wasm_bindgen]
pub fn get_unit_system() -> String {
    APP.with(|app| {
        let state = app.borrow();
        match state.unit_system {
            UnitSystem::Imperial => "imperial".to_string(),
            UnitSystem::Metric => "metric".to_string(),
        }
    })
}

#[wasm_bindgen]
pub fn set_grid_snap(enabled: bool) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        state.snap_config.grid_enabled = enabled;
    });
}

#[wasm_bindgen]
pub fn get_grid_snap() -> bool {
    APP.with(|app| {
        let state = app.borrow();
        state.snap_config.grid_enabled
    })
}
