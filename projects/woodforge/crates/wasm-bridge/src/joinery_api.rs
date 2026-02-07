use wasm_bindgen::prelude::*;

use woodforge_core::{
    joinery::{Joint, JointFace, JointType},
    types::{NodeId, Position3},
};

use crate::APP;

#[wasm_bindgen]
pub fn get_joint_types() -> JsValue {
    let types = vec![
        ("butt", "Butt Joint", "Beginner"),
        ("miter", "Miter Joint", "Beginner"),
        ("pocket_hole", "Pocket Hole", "Beginner"),
        ("dado", "Dado", "Intermediate"),
        ("rabbet", "Rabbet", "Beginner"),
        ("half_lap", "Half Lap", "Intermediate"),
        ("cross_lap", "Cross Lap", "Intermediate"),
        ("mortise_tenon", "Mortise & Tenon", "Advanced"),
        ("dovetail_through", "Dovetail (Through)", "Advanced"),
        ("dovetail_half_blind", "Dovetail (Half-Blind)", "Advanced"),
        ("box_joint", "Box Joint", "Intermediate"),
        ("finger_joint", "Finger Joint", "Advanced"),
        ("dowel", "Dowel Joint", "Beginner"),
        ("biscuit", "Biscuit Joint", "Beginner"),
        ("tongue_groove", "Tongue & Groove", "Intermediate"),
        ("bridle", "Bridle Joint", "Advanced"),
    ];
    serde_wasm_bindgen::to_value(&types).unwrap_or(JsValue::NULL)
}

fn parse_joint_type(s: &str) -> Option<JointType> {
    match s {
        "butt" => Some(JointType::ButtJoint),
        "miter" => Some(JointType::MiterJoint),
        "pocket_hole" => Some(JointType::PocketHole),
        "dado" => Some(JointType::Dado),
        "rabbet" => Some(JointType::Rabbet),
        "half_lap" => Some(JointType::HalfLap),
        "cross_lap" => Some(JointType::CrossLap),
        "mortise_tenon" => Some(JointType::MortiseAndTenon),
        "dovetail_through" => Some(JointType::DovetailThrough),
        "dovetail_half_blind" => Some(JointType::DovetailHalfBlind),
        "box_joint" => Some(JointType::BoxJoint),
        "finger_joint" => Some(JointType::FingerJoint),
        "dowel" => Some(JointType::DowelJoint),
        "biscuit" => Some(JointType::BiscuitJoint),
        "tongue_groove" => Some(JointType::TongueAndGroove),
        "bridle" => Some(JointType::BridleJoint),
        _ => None,
    }
}

fn parse_face(s: &str) -> Option<JointFace> {
    match s {
        "front" => Some(JointFace::Front),
        "back" => Some(JointFace::Back),
        "top" => Some(JointFace::Top),
        "bottom" => Some(JointFace::Bottom),
        "right" => Some(JointFace::Right),
        "left" => Some(JointFace::Left),
        _ => None,
    }
}

#[wasm_bindgen]
pub fn add_joint(
    joint_type_str: &str,
    board_a_id: &str,
    board_b_id: &str,
    face_a_str: &str,
    face_b_str: &str,
) -> JsValue {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let jt = match parse_joint_type(joint_type_str) {
            Some(jt) => jt,
            None => return JsValue::NULL,
        };
        let face_a = match parse_face(face_a_str) {
            Some(f) => f,
            None => return JsValue::NULL,
        };
        let face_b = match parse_face(face_b_str) {
            Some(f) => f,
            None => return JsValue::NULL,
        };
        let board_a: NodeId = match board_a_id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };
        let board_b: NodeId = match board_b_id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        // Get board dimensions for default params
        let (bw, bh) = state
            .scene
            .get_node(board_a)
            .and_then(|n| n.component.as_ref())
            .map(|c| (c.dimensions.width, c.dimensions.height))
            .unwrap_or((1.5, 3.5));

        let params = jt.default_params(bw, bh);
        let joint_id = NodeId::new();

        let joint = Joint {
            id: joint_id,
            joint_type: jt,
            board_a,
            board_b,
            face_a,
            face_b,
            offset: Position3::zero(),
            parameters: params,
        };

        state.joint_store.add_joint(joint);

        state.history.record(woodforge_core::history::Operation::AddJoint {
            joint_id: joint_id.to_string(),
            joint_type: joint_type_str.to_string(),
            board_a: board_a_id.to_string(),
            board_b: board_b_id.to_string(),
        });

        JsValue::from_str(&joint_id.to_string())
    })
}

#[wasm_bindgen]
pub fn remove_joint(joint_id: &str) -> bool {
    APP.with(|app| {
        let mut state = app.borrow_mut();

        let nid: NodeId = match joint_id.parse() {
            Ok(id) => id,
            Err(_) => return false,
        };

        let joint_info = state.joint_store.get_joint(nid).map(|j| {
            (
                format!("{:?}", j.joint_type),
                j.board_a.to_string(),
                j.board_b.to_string(),
            )
        });

        state.joint_store.remove_joint(nid);

        if let Some((jt, ba, bb)) = joint_info {
            state.history.record(woodforge_core::history::Operation::RemoveJoint {
                joint_id: joint_id.to_string(),
                joint_type: jt,
                board_a: ba,
                board_b: bb,
                params_json: "{}".to_string(),
            });
        }

        true
    })
}

#[wasm_bindgen]
pub fn get_joints_for_board(board_id: &str) -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        let nid: NodeId = match board_id.parse() {
            Ok(id) => id,
            Err(_) => return JsValue::NULL,
        };

        let joints = state.joint_store.joints_for_board(nid);
        serde_wasm_bindgen::to_value(&joints).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn get_all_joints() -> JsValue {
    APP.with(|app| {
        let state = app.borrow();
        serde_wasm_bindgen::to_value(state.joint_store.all_joints()).unwrap_or(JsValue::NULL)
    })
}
