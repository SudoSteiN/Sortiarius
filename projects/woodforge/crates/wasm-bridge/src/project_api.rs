use wasm_bindgen::prelude::*;

use woodforge_core::project::{Project, SavedBoard, SavedFinish, SavedJoint, FORMAT_VERSION};

use crate::APP;

// ========== Undo/Redo ==========

#[wasm_bindgen]
pub fn undo() -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        match state.history.undo() {
            Some(op) => {
                let desc = op.description();
                JsValue::from_str(&desc)
            }
            None => JsValue::NULL,
        }
    })
}

#[wasm_bindgen]
pub fn redo() -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        match state.history.redo() {
            Some(op) => {
                let desc = op.description();
                JsValue::from_str(&desc)
            }
            None => JsValue::NULL,
        }
    })
}

#[wasm_bindgen]
pub fn can_undo() -> bool {
    APP.with(|app| app.borrow().history.can_undo())
}

#[wasm_bindgen]
pub fn can_redo() -> bool {
    APP.with(|app| app.borrow().history.can_redo())
}

#[wasm_bindgen]
pub fn get_undo_description() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        // Peek at the top of undo stack
        if state.history.can_undo() {
            JsValue::from_str("Undo available")
        } else {
            JsValue::NULL
        }
    })
}

// ========== Save/Load ==========

#[wasm_bindgen]
pub fn save_project(name: &str, description: &str) -> String {
    APP.with(|app| {
        let state = app.borrow();

        let boards: Vec<SavedBoard> = state
            .scene
            .all_nodes()
            .filter_map(|node| {
                let comp = node.component.as_ref()?;
                Some(SavedBoard {
                    id: node.id.to_string(),
                    label: node.label.clone(),
                    position: [node.position.x, node.position.y, node.position.z],
                    rotation: [node.rotation.x, node.rotation.y, node.rotation.z],
                    dimensions: [
                        comp.dimensions.width,
                        comp.dimensions.height,
                        comp.dimensions.depth,
                    ],
                    parent_id: node.parent.map(|p| p.to_string()),
                    lumber_type: format!("{:?}", comp.lumber_type),
                    species_id: state.board_species.get(&node.id.to_string()).cloned(),
                    finish_id: state.board_finishes.get(&node.id.to_string()).cloned(),
                    grain_direction: format!("{:?}", comp.grain_direction),
                })
            })
            .collect();

        let joints: Vec<SavedJoint> = state
            .joint_store
            .all_joints()
            .iter()
            .map(|j| SavedJoint {
                id: j.id.to_string(),
                joint_type: format!("{:?}", j.joint_type),
                board_a_id: j.board_a.to_string(),
                board_b_id: j.board_b.to_string(),
                face_a: format!("{:?}", j.face_a),
                face_b: format!("{:?}", j.face_b),
                offset: [j.offset.x, j.offset.y, j.offset.z],
                parameters: serde_json::to_value(&j.parameters).unwrap_or_default(),
            })
            .collect();

        let finishes: Vec<SavedFinish> = state
            .board_finishes
            .iter()
            .map(|(id, _json)| SavedFinish {
                id: id.clone(),
                finish_type: "custom".to_string(),
                name: "Applied Finish".to_string(),
                color: [0.0, 0.0, 0.0],
                opacity: 1.0,
            })
            .collect();

        let project = Project {
            format_version: FORMAT_VERSION,
            name: name.to_string(),
            description: description.to_string(),
            created_at: String::new(),
            updated_at: String::new(),
            unit_system: state.unit_system,
            boards,
            joints,
            finishes,
            camera: None,
        };

        project.to_json().unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    })
}

#[wasm_bindgen]
pub fn load_project(json: &str) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let project = match Project::from_json(json) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Failed to load project: {:?}", e);
                return JsValue::from_str(&format!("Error: {:?}", e));
            }
        };

        // Clear current state
        state.scene = woodforge_core::scene::SceneGraph::new();
        state.joint_store = woodforge_core::joinery::JointStore::new();
        state.history.clear();
        state.board_species.clear();
        state.board_finishes.clear();
        state.unit_system = project.unit_system;

        if let Some(ref mut renderer) = state.renderer {
            renderer.clear_all_objects();
        }

        // Restore boards
        for board in &project.boards {
            let node_id = woodforge_core::types::NodeId::new();
            let comp = woodforge_core::component::BoardComponent::new_custom(
                node_id,
                board.label.clone(),
                board.dimensions[0],
                board.dimensions[1],
                board.dimensions[2],
            );

            let pos = woodforge_core::types::Position3::new(
                board.position[0],
                board.position[1],
                board.position[2],
            );
            let rot = woodforge_core::types::Rotation3::new(
                board.rotation[0],
                board.rotation[1],
                board.rotation[2],
            );

            let scene_node_id = state.scene.add_node(board.label.clone(), Some(comp), pos, rot);

            if let Some(ref species_id) = board.species_id {
                state
                    .board_species
                    .insert(scene_node_id.to_string(), species_id.clone());
            }

            if let Some(ref finish_id) = board.finish_id {
                state
                    .board_finishes
                    .insert(scene_node_id.to_string(), finish_id.clone());
            }

            // Add to renderer
            let color = board
                .species_id
                .as_ref()
                .and_then(|sid| state.species_db.find_by_id(sid))
                .map(|s| s.color)
                .unwrap_or([0.76, 0.60, 0.42]);

            let mesh_clone = state.scene.get_mesh(scene_node_id).cloned();
            if let (Some(ref mut renderer), Some(ref mesh)) = (&mut state.renderer, &mesh_clone) {
                let translation = glam::Mat4::from_translation(glam::Vec3::new(
                    pos.x as f32,
                    pos.y as f32,
                    pos.z as f32,
                ));
                let rotation = glam::Mat4::from_euler(
                    glam::EulerRot::XYZ,
                    rot.x as f32,
                    rot.y as f32,
                    rot.z as f32,
                );
                let model = translation * rotation;
                renderer.add_object(scene_node_id, mesh, model, color, None);
            }
        }

        let snapshot = state.scene.snapshot();
        serde_wasm_bindgen::to_value(&snapshot).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn new_project() -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        state.scene = woodforge_core::scene::SceneGraph::new();
        state.joint_store = woodforge_core::joinery::JointStore::new();
        state.history.clear();
        state.board_species.clear();
        state.board_finishes.clear();

        if let Some(ref mut renderer) = state.renderer {
            renderer.clear_all_objects();
        }

        let snapshot = state.scene.snapshot();
        serde_wasm_bindgen::to_value(&snapshot).unwrap_or(JsValue::NULL)
    })
}
