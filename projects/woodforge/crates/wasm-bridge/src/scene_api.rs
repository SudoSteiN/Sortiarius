use wasm_bindgen::prelude::*;

use glam::Mat4;
use woodforge_core::{
    component::BoardComponent,
    scene::SceneTreeSnapshot,
    snap::compute_snap,
    types::{NodeId, Position3, Rotation3},
};

use crate::APP;

fn snapshot_to_js(snapshot: &SceneTreeSnapshot) -> JsValue {
    serde_wasm_bindgen::to_value(snapshot).unwrap_or(JsValue::NULL)
}

fn node_model_matrix(pos: &Position3, rot: &Rotation3) -> Mat4 {
    let translation = Mat4::from_translation(glam::Vec3::new(
        pos.x as f32,
        pos.y as f32,
        pos.z as f32,
    ));
    let rotation = Mat4::from_euler(
        glam::EulerRot::XYZ,
        rot.x as f32,
        rot.y as f32,
        rot.z as f32,
    );
    translation * rotation
}

#[wasm_bindgen]
pub fn add_standard_board(label: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let lumber = match state.catalog.find_by_label(label) {
            Some(l) => l.clone(),
            None => {
                log::warn!("Unknown lumber label: {}", label);
                return JsValue::NULL;
            }
        };

        let default_length = 96.0; // 8 feet
        let node_id = NodeId::new();
        let board = BoardComponent::new_standard(node_id, &lumber, default_length);

        let pos = Position3::zero();
        let rot = Rotation3::zero();

        let scene_node_id = state.scene.add_node(
            label.to_string(),
            Some(board),
            pos,
            rot,
        );

        // Clone mesh data to avoid borrow conflict
        let mesh_clone = state.scene.get_mesh(scene_node_id).cloned();
        if let (Some(ref mut renderer), Some(ref mesh)) = (&mut state.renderer, &mesh_clone) {
            let model = node_model_matrix(&pos, &rot);
            renderer.add_object(scene_node_id, mesh, model);
        }

        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn add_custom_board(width: f64, height: f64, depth: f64) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let node_id = NodeId::new();
        let label = format!("{:.2}\" x {:.2}\" x {:.2}\"", width, height, depth);
        let board = BoardComponent::new_custom(node_id, label.clone(), width, height, depth);

        let pos = Position3::zero();
        let rot = Rotation3::zero();

        let scene_node_id = state.scene.add_node(
            label,
            Some(board),
            pos,
            rot,
        );

        let mesh_clone = state.scene.get_mesh(scene_node_id).cloned();
        if let (Some(ref mut renderer), Some(ref mesh)) = (&mut state.renderer, &mesh_clone) {
            let model = node_model_matrix(&pos, &rot);
            renderer.add_object(scene_node_id, mesh, model);
        }

        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn remove_node(id: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let node_id: NodeId = match id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        if let Some(ref mut renderer) = state.renderer {
            renderer.remove_object(node_id);
        }

        state.scene.remove_node(node_id);

        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn set_position(id: &str, x: f64, y: f64, z: f64) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let node_id: NodeId = match id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        let pos = Position3::new(x, y, z);
        state.scene.set_position(node_id, pos);

        // Read position/rotation, then update renderer
        let transform = state.scene.get_node(node_id).map(|n| (n.position, n.rotation));
        if let (Some(ref renderer), Some((p, r))) = (&state.renderer, transform) {
            let model = node_model_matrix(&p, &r);
            renderer.update_transform(node_id, model);
        }

        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn set_rotation(id: &str, x: f64, y: f64, z: f64) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let node_id: NodeId = match id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        let rot = Rotation3::new(x, y, z);
        state.scene.set_rotation(node_id, rot);

        let transform = state.scene.get_node(node_id).map(|n| (n.position, n.rotation));
        if let (Some(ref renderer), Some((p, r))) = (&state.renderer, transform) {
            let model = node_model_matrix(&p, &r);
            renderer.update_transform(node_id, model);
        }

        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn get_scene_tree() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        let snapshot = state.scene.snapshot();
        snapshot_to_js(&snapshot)
    })
}

#[wasm_bindgen]
pub fn compute_snap_position(id: &str, x: f64, y: f64, z: f64) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let node_id: NodeId = match id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        let proposed = Position3::new(x, y, z);
        let others = state.scene.boards_for_snap(Some(node_id));
        let result = compute_snap(&proposed, &state.snap_config, &others);

        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    })
}
