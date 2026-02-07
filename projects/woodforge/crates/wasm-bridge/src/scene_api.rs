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

/// Default wood color when no species is assigned
const DEFAULT_WOOD_COLOR: [f32; 3] = [0.76, 0.60, 0.42];

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
            renderer.add_object(scene_node_id, mesh, model, DEFAULT_WOOD_COLOR, None);
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
            renderer.add_object(scene_node_id, mesh, model, DEFAULT_WOOD_COLOR, None);
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
pub fn select_node(id: &str) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        if let Some(ref renderer) = state.renderer {
            // Deselect all first
            for node in state.scene.all_nodes() {
                renderer.set_selected(node.id, false);
            }
            // Select the target
            if let Ok(node_id) = id.parse::<NodeId>() {
                renderer.set_selected(node_id, true);
            }
        }

        JsValue::TRUE
    })
}

#[wasm_bindgen]
pub fn deselect_all() {
    APP.with(|app| {
        let state = app.borrow();
        if let Some(ref renderer) = state.renderer {
            for node in state.scene.all_nodes() {
                renderer.set_selected(node.id, false);
            }
        }
    });
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

/// Ray cast from screen coordinates to find which board was clicked
#[wasm_bindgen]
pub fn pick_object(screen_x: f32, screen_y: f32) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();

        let renderer = match &state.renderer {
            Some(r) => r,
            None => return JsValue::NULL,
        };

        let (vw, vh) = renderer.viewport_size();
        if vw == 0 || vh == 0 {
            return JsValue::NULL;
        }

        // Convert screen coords to NDC (-1 to 1)
        let ndc_x = (2.0 * screen_x / vw as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / vh as f32);

        let (ray_origin, ray_dir) = renderer.camera_ray(ndc_x, ndc_y);

        // Test ray against all board AABBs
        let mut closest_id: Option<String> = None;
        let mut closest_t = f32::MAX;

        for node in state.scene.all_nodes() {
            if let Some(ref comp) = node.component {
                let (world_pos, _) = state.scene.world_transform(node.id);
                let center = glam::Vec3::new(
                    world_pos.x as f32,
                    world_pos.y as f32,
                    world_pos.z as f32,
                );
                let half = glam::Vec3::new(
                    (comp.dimensions.width / 2.0) as f32,
                    (comp.dimensions.height / 2.0) as f32,
                    (comp.dimensions.depth / 2.0) as f32,
                );
                let aabb_min = center - half;
                let aabb_max = center + half;

                if let Some(t) = ray_aabb_intersect(ray_origin, ray_dir, aabb_min, aabb_max) {
                    if t < closest_t {
                        closest_t = t;
                        closest_id = Some(node.id.to_string());
                    }
                }
            }
        }

        match closest_id {
            Some(id) => JsValue::from_str(&id),
            None => JsValue::NULL,
        }
    })
}

/// Ray-AABB intersection test (slab method)
fn ray_aabb_intersect(
    origin: glam::Vec3,
    dir: glam::Vec3,
    aabb_min: glam::Vec3,
    aabb_max: glam::Vec3,
) -> Option<f32> {
    let inv_dir = glam::Vec3::new(
        if dir.x.abs() > 1e-8 { 1.0 / dir.x } else { f32::MAX },
        if dir.y.abs() > 1e-8 { 1.0 / dir.y } else { f32::MAX },
        if dir.z.abs() > 1e-8 { 1.0 / dir.z } else { f32::MAX },
    );

    let t1 = (aabb_min - origin) * inv_dir;
    let t2 = (aabb_max - origin) * inv_dir;

    let tmin = t1.min(t2);
    let tmax = t1.max(t2);

    let t_enter = tmin.x.max(tmin.y).max(tmin.z);
    let t_exit = tmax.x.min(tmax.y).min(tmax.z);

    if t_enter <= t_exit && t_exit >= 0.0 {
        Some(t_enter.max(0.0))
    } else {
        None
    }
}
