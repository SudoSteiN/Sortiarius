use serde::{Deserialize, Serialize};

use crate::lumber::LumberSize;
use crate::types::{Dimensions3, NodeId};

/// Direction of wood grain relative to the board
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GrainDirection {
    AlongLength,
    AlongWidth,
    AlongHeight,
}

impl Default for GrainDirection {
    fn default() -> Self {
        Self::AlongLength
    }
}

/// Whether the board is from standard lumber or custom dimensions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LumberType {
    Standard(LumberSize),
    Custom,
}

/// A board component in the scene
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardComponent {
    pub node_id: NodeId,
    pub label: String,
    pub dimensions: Dimensions3,
    pub lumber_type: LumberType,
    pub grain_direction: GrainDirection,
}

impl BoardComponent {
    pub fn new_standard(node_id: NodeId, lumber: &LumberSize, length: f64) -> Self {
        Self {
            node_id,
            label: lumber.nominal_label.clone(),
            dimensions: Dimensions3::new(lumber.actual_width, lumber.actual_height, length),
            lumber_type: LumberType::Standard(lumber.clone()),
            grain_direction: GrainDirection::default(),
        }
    }

    pub fn new_custom(node_id: NodeId, label: String, width: f64, height: f64, depth: f64) -> Self {
        Self {
            node_id,
            label,
            dimensions: Dimensions3::new(width, height, depth),
            lumber_type: LumberType::Custom,
            grain_direction: GrainDirection::default(),
        }
    }
}
