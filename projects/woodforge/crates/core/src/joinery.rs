use serde::{Deserialize, Serialize};

use crate::types::{NodeId, Position3};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum JointType {
    ButtJoint,
    MiterJoint,
    PocketHole,
    MortiseAndTenon,
    Dado,
    Rabbet,
    HalfLap,
    CrossLap,
    DovetailThrough,
    DovetailHalfBlind,
    BoxJoint,
    FingerJoint,
    DowelJoint,
    BiscuitJoint,
    TongueAndGroove,
    BridleJoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum JointFace {
    Front,  // +Z
    Back,   // -Z
    Top,    // +Y
    Bottom, // -Y
    Right,  // +X
    Left,   // -X
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Joint {
    pub id: NodeId,
    pub joint_type: JointType,
    pub board_a: NodeId,
    pub board_b: NodeId,
    pub face_a: JointFace,
    pub face_b: JointFace,
    pub offset: Position3,
    pub parameters: JointParams,
}

/// Joint-specific parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JointParams {
    Butt,
    Miter {
        angle: f64,
    },
    PocketHole {
        screw_angle: f64,
        screw_count: u32,
    },
    MortiseAndTenon {
        tenon_width: f64,
        tenon_height: f64,
        tenon_depth: f64,
    },
    Dado {
        width: f64,
        depth: f64,
    },
    Rabbet {
        width: f64,
        depth: f64,
    },
    HalfLap {
        depth: f64,
    },
    DowelJoint {
        dowel_diameter: f64,
        dowel_count: u32,
        spacing: f64,
    },
    CrossLap {
        depth: f64,
    },
    TongueAndGroove {
        tongue_width: f64,
        tongue_depth: f64,
    },
    BridleJoint {
        slot_width: f64,
    },
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum JointDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl JointType {
    /// Get the default parameters for this joint type given board dimensions
    pub fn default_params(&self, board_width: f64, board_height: f64) -> JointParams {
        match self {
            JointType::ButtJoint => JointParams::Butt,
            JointType::MiterJoint => JointParams::Miter { angle: 45.0 },
            JointType::PocketHole => JointParams::PocketHole {
                screw_angle: 15.0,
                screw_count: 2,
            },
            JointType::MortiseAndTenon => JointParams::MortiseAndTenon {
                tenon_width: board_width / 3.0,
                tenon_height: board_height / 3.0,
                tenon_depth: board_width * 0.75,
            },
            JointType::Dado => JointParams::Dado {
                width: board_width,
                depth: board_height / 3.0,
            },
            JointType::Rabbet => JointParams::Rabbet {
                width: board_width / 2.0,
                depth: board_height / 2.0,
            },
            JointType::HalfLap => JointParams::HalfLap {
                depth: board_height / 2.0,
            },
            JointType::CrossLap => JointParams::CrossLap {
                depth: board_height / 2.0,
            },
            JointType::DovetailThrough => JointParams::Generic,
            JointType::DovetailHalfBlind => JointParams::Generic,
            JointType::BoxJoint => JointParams::Generic,
            JointType::FingerJoint => JointParams::Generic,
            JointType::DowelJoint => JointParams::DowelJoint {
                dowel_diameter: 0.375,
                dowel_count: 2,
                spacing: board_width / 3.0,
            },
            JointType::BiscuitJoint => JointParams::Generic,
            JointType::TongueAndGroove => JointParams::TongueAndGroove {
                tongue_width: board_width / 3.0,
                tongue_depth: board_height / 3.0,
            },
            JointType::BridleJoint => JointParams::BridleJoint {
                slot_width: board_width / 3.0,
            },
        }
    }

    /// Base strength rating (0-100) before species adjustment
    pub fn base_strength(&self) -> u32 {
        match self {
            JointType::ButtJoint => 15,
            JointType::MiterJoint => 20,
            JointType::PocketHole => 45,
            JointType::MortiseAndTenon => 85,
            JointType::Dado => 55,
            JointType::Rabbet => 40,
            JointType::HalfLap => 50,
            JointType::CrossLap => 45,
            JointType::DovetailThrough => 90,
            JointType::DovetailHalfBlind => 85,
            JointType::BoxJoint => 80,
            JointType::FingerJoint => 75,
            JointType::DowelJoint => 55,
            JointType::BiscuitJoint => 40,
            JointType::TongueAndGroove => 50,
            JointType::BridleJoint => 70,
        }
    }

    /// Difficulty level for build instructions
    pub fn difficulty(&self) -> JointDifficulty {
        match self {
            JointType::ButtJoint | JointType::PocketHole => JointDifficulty::Beginner,
            JointType::MiterJoint
            | JointType::Dado
            | JointType::Rabbet
            | JointType::HalfLap
            | JointType::CrossLap
            | JointType::DowelJoint
            | JointType::BiscuitJoint
            | JointType::TongueAndGroove => JointDifficulty::Intermediate,
            JointType::MortiseAndTenon
            | JointType::DovetailThrough
            | JointType::DovetailHalfBlind
            | JointType::BoxJoint
            | JointType::FingerJoint
            | JointType::BridleJoint => JointDifficulty::Advanced,
        }
    }

    /// Whether this joint type has geometry modification in MVP
    pub fn has_geometry(&self) -> bool {
        matches!(
            self,
            JointType::ButtJoint
                | JointType::MiterJoint
                | JointType::Dado
                | JointType::Rabbet
                | JointType::HalfLap
                | JointType::MortiseAndTenon
        )
    }

    /// Human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            JointType::ButtJoint => "Butt Joint",
            JointType::MiterJoint => "Miter Joint",
            JointType::PocketHole => "Pocket Hole",
            JointType::MortiseAndTenon => "Mortise and Tenon",
            JointType::Dado => "Dado",
            JointType::Rabbet => "Rabbet",
            JointType::HalfLap => "Half Lap",
            JointType::CrossLap => "Cross Lap",
            JointType::DovetailThrough => "Through Dovetail",
            JointType::DovetailHalfBlind => "Half-Blind Dovetail",
            JointType::BoxJoint => "Box Joint",
            JointType::FingerJoint => "Finger Joint",
            JointType::DowelJoint => "Dowel Joint",
            JointType::BiscuitJoint => "Biscuit Joint",
            JointType::TongueAndGroove => "Tongue and Groove",
            JointType::BridleJoint => "Bridle Joint",
        }
    }

    /// Brief description
    pub fn description(&self) -> &'static str {
        match self {
            JointType::ButtJoint => {
                "Two boards joined end-to-end or end-to-face, the simplest joint"
            }
            JointType::MiterJoint => {
                "Angled cut (typically 45 degrees) where two boards meet at a corner"
            }
            JointType::PocketHole => {
                "Angled screw driven through a pocket hole for hidden fastening"
            }
            JointType::MortiseAndTenon => {
                "A tenon tongue fits into a mortise pocket for strong structural joints"
            }
            JointType::Dado => {
                "A rectangular channel cut across the grain to receive another board"
            }
            JointType::Rabbet => "An L-shaped cut along the edge of a board",
            JointType::HalfLap => "Half the material removed from each board so they overlap flush",
            JointType::CrossLap => "Both boards notched at the intersection so they interlock",
            JointType::DovetailThrough => {
                "Interlocking trapezoidal pins and tails visible on both faces"
            }
            JointType::DovetailHalfBlind => {
                "Dovetail visible on one face only, hidden on the other"
            }
            JointType::BoxJoint => {
                "Interlocking rectangular fingers, a simpler alternative to dovetails"
            }
            JointType::FingerJoint => {
                "Multiple interlocking rectangular fingers along the board end"
            }
            JointType::DowelJoint => {
                "Cylindrical dowel pins inserted into aligned holes in both boards"
            }
            JointType::BiscuitJoint => {
                "Oval biscuit inserted into matching slots for alignment and strength"
            }
            JointType::TongueAndGroove => {
                "A protruding tongue on one board fits into a groove on another"
            }
            JointType::BridleJoint => {
                "An open mortise-and-tenon where the mortise is cut through the end"
            }
        }
    }

    /// Tools typically needed
    pub fn tools_needed(&self) -> Vec<&'static str> {
        match self {
            JointType::ButtJoint => vec!["saw", "clamps", "wood glue"],
            JointType::MiterJoint => vec!["miter saw", "clamps", "wood glue"],
            JointType::PocketHole => vec!["pocket hole jig", "drill", "pocket screws"],
            JointType::MortiseAndTenon => vec!["chisel", "mallet", "drill press", "table saw"],
            JointType::Dado => vec!["table saw", "dado blade set", "router"],
            JointType::Rabbet => vec!["table saw", "router", "rabbet bit"],
            JointType::HalfLap => vec!["table saw", "chisel", "mallet"],
            JointType::CrossLap => vec!["table saw", "chisel", "mallet"],
            JointType::DovetailThrough => vec!["dovetail saw", "chisel", "mallet", "marking gauge"],
            JointType::DovetailHalfBlind => {
                vec!["dovetail saw", "chisel", "mallet", "marking gauge"]
            }
            JointType::BoxJoint => vec!["table saw", "box joint jig", "chisel"],
            JointType::FingerJoint => vec!["table saw", "finger joint jig", "chisel"],
            JointType::DowelJoint => vec!["drill", "doweling jig", "dowel pins"],
            JointType::BiscuitJoint => vec!["biscuit joiner", "biscuits", "wood glue"],
            JointType::TongueAndGroove => vec!["table saw", "router", "tongue and groove bit set"],
            JointType::BridleJoint => vec!["table saw", "chisel", "mallet"],
        }
    }
}

/// All 16 joint types for iteration
pub const ALL_JOINT_TYPES: [JointType; 16] = [
    JointType::ButtJoint,
    JointType::MiterJoint,
    JointType::PocketHole,
    JointType::MortiseAndTenon,
    JointType::Dado,
    JointType::Rabbet,
    JointType::HalfLap,
    JointType::CrossLap,
    JointType::DovetailThrough,
    JointType::DovetailHalfBlind,
    JointType::BoxJoint,
    JointType::FingerJoint,
    JointType::DowelJoint,
    JointType::BiscuitJoint,
    JointType::TongueAndGroove,
    JointType::BridleJoint,
];

/// Storage for all joints in a project
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JointStore {
    joints: Vec<Joint>,
}

impl JointStore {
    pub fn new() -> Self {
        Self { joints: Vec::new() }
    }

    pub fn add_joint(&mut self, joint: Joint) {
        self.joints.push(joint);
    }

    pub fn remove_joint(&mut self, id: NodeId) {
        self.joints.retain(|j| j.id != id);
    }

    pub fn joints_for_board(&self, board_id: NodeId) -> Vec<&Joint> {
        self.joints
            .iter()
            .filter(|j| j.board_a == board_id || j.board_b == board_id)
            .collect()
    }

    pub fn all_joints(&self) -> &[Joint] {
        &self.joints
    }

    pub fn get_joint(&self, id: NodeId) -> Option<&Joint> {
        self.joints.iter().find(|j| j.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_types_have_names() {
        for jt in &ALL_JOINT_TYPES {
            let name = jt.display_name();
            assert!(!name.is_empty(), "{:?} has empty display_name", jt);
        }
        assert_eq!(ALL_JOINT_TYPES.len(), 16);
    }

    #[test]
    fn test_default_params() {
        let params = JointType::MortiseAndTenon.default_params(3.5, 0.75);
        match params {
            JointParams::MortiseAndTenon {
                tenon_width,
                tenon_height,
                tenon_depth,
            } => {
                assert!(tenon_width > 0.0 && tenon_width < 3.5);
                assert!(tenon_height > 0.0 && tenon_height < 0.75);
                assert!(tenon_depth > 0.0);
            }
            _ => panic!("Expected MortiseAndTenon params"),
        }
    }

    #[test]
    fn test_joint_store_crud() {
        let mut store = JointStore::new();
        assert_eq!(store.all_joints().len(), 0);

        let board_a = NodeId::new();
        let board_b = NodeId::new();
        let joint_id = NodeId::new();

        let joint = Joint {
            id: joint_id,
            joint_type: JointType::ButtJoint,
            board_a,
            board_b,
            face_a: JointFace::Right,
            face_b: JointFace::Left,
            offset: Position3::zero(),
            parameters: JointParams::Butt,
        };

        store.add_joint(joint);
        assert_eq!(store.all_joints().len(), 1);
        assert!(store.get_joint(joint_id).is_some());
        assert_eq!(store.joints_for_board(board_a).len(), 1);
        assert_eq!(store.joints_for_board(board_b).len(), 1);
        assert_eq!(store.joints_for_board(NodeId::new()).len(), 0);

        store.remove_joint(joint_id);
        assert_eq!(store.all_joints().len(), 0);
        assert!(store.get_joint(joint_id).is_none());
    }

    #[test]
    fn test_difficulty_levels() {
        assert_eq!(JointType::ButtJoint.difficulty(), JointDifficulty::Beginner);
        assert_eq!(
            JointType::PocketHole.difficulty(),
            JointDifficulty::Beginner
        );
        assert_eq!(JointType::Dado.difficulty(), JointDifficulty::Intermediate);
        assert_eq!(
            JointType::DovetailThrough.difficulty(),
            JointDifficulty::Advanced
        );
        assert_eq!(
            JointType::MortiseAndTenon.difficulty(),
            JointDifficulty::Advanced
        );
    }

    #[test]
    fn test_base_strength_range() {
        for jt in &ALL_JOINT_TYPES {
            let strength = jt.base_strength();
            assert!(strength <= 100, "{:?} strength {} > 100", jt, strength);
            assert!(strength > 0, "{:?} strength is 0", jt);
        }
    }

    #[test]
    fn test_all_types_have_descriptions() {
        for jt in &ALL_JOINT_TYPES {
            assert!(!jt.description().is_empty());
            assert!(!jt.tools_needed().is_empty());
        }
    }
}
