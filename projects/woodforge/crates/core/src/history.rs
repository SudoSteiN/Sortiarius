use serde::{Deserialize, Serialize};

/// An undoable operation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    AddBoard {
        node_id: String,
        label: String,
        dimensions: [f64; 3],
        position: [f64; 3],
        lumber_type: String,
    },
    RemoveBoard {
        node_id: String,
        label: String,
        dimensions: [f64; 3],
        position: [f64; 3],
        rotation: [f64; 3],
        lumber_type: String,
        species_id: Option<String>,
    },
    MoveBoard {
        node_id: String,
        old_position: [f64; 3],
        new_position: [f64; 3],
    },
    RotateBoard {
        node_id: String,
        old_rotation: [f64; 3],
        new_rotation: [f64; 3],
    },
    AddJoint {
        joint_id: String,
        joint_type: String,
        board_a: String,
        board_b: String,
    },
    RemoveJoint {
        joint_id: String,
        joint_type: String,
        board_a: String,
        board_b: String,
        params_json: String,
    },
    SetSpecies {
        node_id: String,
        old_species: Option<String>,
        new_species: Option<String>,
    },
    SetFinish {
        node_id: String,
        old_finish: Option<String>,
        new_finish: Option<String>,
    },
    ResizeBoard {
        node_id: String,
        old_dimensions: [f64; 3],
        new_dimensions: [f64; 3],
    },
}

pub struct History {
    undo_stack: Vec<Operation>,
    redo_stack: Vec<Operation>,
    max_history: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 100,
        }
    }

    /// Record an operation (clears redo stack).
    /// If the undo stack exceeds max_history, the oldest operation is removed.
    pub fn record(&mut self, op: Operation) {
        self.redo_stack.clear();
        self.undo_stack.push(op);
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    /// Pop from undo stack, push to redo stack, return the inverse operation
    /// (which the caller applies to the scene to undo the action).
    pub fn undo(&mut self) -> Option<Operation> {
        let op = self.undo_stack.pop()?;
        let inverse = op.inverse();
        self.redo_stack.push(op);
        Some(inverse)
    }

    /// Pop from redo stack, push to undo stack, return the operation
    /// (which the caller applies to the scene to redo the action).
    pub fn redo(&mut self) -> Option<Operation> {
        let op = self.redo_stack.pop()?;
        self.undo_stack.push(op.clone());
        Some(op)
    }

    /// Can we undo?
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Can we redo?
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Number of undoable operations
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Number of redoable operations
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl Operation {
    /// Create the inverse (undo) operation
    pub fn inverse(&self) -> Operation {
        match self {
            Operation::AddBoard {
                node_id,
                label,
                dimensions,
                position,
                lumber_type,
            } => Operation::RemoveBoard {
                node_id: node_id.clone(),
                label: label.clone(),
                dimensions: *dimensions,
                position: *position,
                rotation: [0.0, 0.0, 0.0],
                lumber_type: lumber_type.clone(),
                species_id: None,
            },
            Operation::RemoveBoard {
                node_id,
                label,
                dimensions,
                position,
                lumber_type,
                ..
            } => Operation::AddBoard {
                node_id: node_id.clone(),
                label: label.clone(),
                dimensions: *dimensions,
                position: *position,
                lumber_type: lumber_type.clone(),
            },
            Operation::MoveBoard {
                node_id,
                old_position,
                new_position,
            } => Operation::MoveBoard {
                node_id: node_id.clone(),
                old_position: *new_position,
                new_position: *old_position,
            },
            Operation::RotateBoard {
                node_id,
                old_rotation,
                new_rotation,
            } => Operation::RotateBoard {
                node_id: node_id.clone(),
                old_rotation: *new_rotation,
                new_rotation: *old_rotation,
            },
            Operation::AddJoint {
                joint_id,
                joint_type,
                board_a,
                board_b,
            } => Operation::RemoveJoint {
                joint_id: joint_id.clone(),
                joint_type: joint_type.clone(),
                board_a: board_a.clone(),
                board_b: board_b.clone(),
                params_json: "{}".to_string(),
            },
            Operation::RemoveJoint {
                joint_id,
                joint_type,
                board_a,
                board_b,
                ..
            } => Operation::AddJoint {
                joint_id: joint_id.clone(),
                joint_type: joint_type.clone(),
                board_a: board_a.clone(),
                board_b: board_b.clone(),
            },
            Operation::SetSpecies {
                node_id,
                old_species,
                new_species,
            } => Operation::SetSpecies {
                node_id: node_id.clone(),
                old_species: new_species.clone(),
                new_species: old_species.clone(),
            },
            Operation::SetFinish {
                node_id,
                old_finish,
                new_finish,
            } => Operation::SetFinish {
                node_id: node_id.clone(),
                old_finish: new_finish.clone(),
                new_finish: old_finish.clone(),
            },
            Operation::ResizeBoard {
                node_id,
                old_dimensions,
                new_dimensions,
            } => Operation::ResizeBoard {
                node_id: node_id.clone(),
                old_dimensions: *new_dimensions,
                new_dimensions: *old_dimensions,
            },
        }
    }

    /// Human-readable description for UI
    pub fn description(&self) -> String {
        match self {
            Operation::AddBoard { label, .. } => format!("Add board '{}'", label),
            Operation::RemoveBoard { label, .. } => format!("Remove board '{}'", label),
            Operation::MoveBoard { node_id, .. } => format!("Move board {}", node_id),
            Operation::RotateBoard { node_id, .. } => format!("Rotate board {}", node_id),
            Operation::AddJoint { joint_type, .. } => format!("Add {} joint", joint_type),
            Operation::RemoveJoint { joint_type, .. } => format!("Remove {} joint", joint_type),
            Operation::SetSpecies {
                node_id,
                new_species,
                ..
            } => match new_species {
                Some(s) => format!("Set species to '{}' on {}", s, node_id),
                None => format!("Clear species on {}", node_id),
            },
            Operation::SetFinish {
                node_id,
                new_finish,
                ..
            } => match new_finish {
                Some(f) => format!("Set finish to '{}' on {}", f, node_id),
                None => format!("Clear finish on {}", node_id),
            },
            Operation::ResizeBoard {
                node_id,
                new_dimensions,
                ..
            } => {
                format!(
                    "Resize board {} to [{:.2}, {:.2}, {:.2}]",
                    node_id, new_dimensions[0], new_dimensions[1], new_dimensions[2]
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_add_board() -> Operation {
        Operation::AddBoard {
            node_id: "board-1".to_string(),
            label: "2x4 Leg".to_string(),
            dimensions: [1.5, 3.5, 30.0],
            position: [0.0, 0.0, 0.0],
            lumber_type: "Standard:2x4".to_string(),
        }
    }

    fn sample_move() -> Operation {
        Operation::MoveBoard {
            node_id: "board-1".to_string(),
            old_position: [0.0, 0.0, 0.0],
            new_position: [10.0, 5.0, 0.0],
        }
    }

    #[test]
    fn test_undo_redo_cycle() {
        let mut history = History::new();
        let op = sample_add_board();

        history.record(op);
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo should return the inverse (RemoveBoard)
        let undo_op = history.undo().unwrap();
        match undo_op {
            Operation::RemoveBoard { ref label, .. } => assert_eq!(label, "2x4 Leg"),
            _ => panic!("Expected RemoveBoard as undo of AddBoard"),
        }
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Redo should return the original AddBoard
        let redo_op = history.redo().unwrap();
        match redo_op {
            Operation::AddBoard { ref label, .. } => assert_eq!(label, "2x4 Leg"),
            _ => panic!("Expected AddBoard as redo"),
        }
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_record_clears_redo() {
        let mut history = History::new();
        history.record(sample_add_board());
        history.record(sample_move());

        // Undo one operation
        history.undo();
        assert_eq!(history.redo_count(), 1);

        // Recording a new operation should clear redo
        history.record(Operation::RotateBoard {
            node_id: "board-1".to_string(),
            old_rotation: [0.0, 0.0, 0.0],
            new_rotation: [0.0, 1.57, 0.0],
        });
        assert_eq!(history.redo_count(), 0);
        assert!(!history.can_redo());
    }

    #[test]
    fn test_inverse_operations() {
        // AddBoard <-> RemoveBoard
        let add = sample_add_board();
        let inv = add.inverse();
        match inv {
            Operation::RemoveBoard { ref node_id, .. } => assert_eq!(node_id, "board-1"),
            _ => panic!("AddBoard inverse should be RemoveBoard"),
        }
        // Double inverse returns to original type
        let double_inv = inv.inverse();
        match double_inv {
            Operation::AddBoard { ref node_id, .. } => assert_eq!(node_id, "board-1"),
            _ => panic!("RemoveBoard inverse should be AddBoard"),
        }

        // MoveBoard: positions swap
        let mv = sample_move();
        let mv_inv = mv.inverse();
        match mv_inv {
            Operation::MoveBoard {
                old_position,
                new_position,
                ..
            } => {
                assert_eq!(old_position, [10.0, 5.0, 0.0]);
                assert_eq!(new_position, [0.0, 0.0, 0.0]);
            }
            _ => panic!("MoveBoard inverse should be MoveBoard"),
        }

        // RotateBoard: rotations swap
        let rot = Operation::RotateBoard {
            node_id: "b".to_string(),
            old_rotation: [0.0, 0.0, 0.0],
            new_rotation: [1.0, 2.0, 3.0],
        };
        let rot_inv = rot.inverse();
        match rot_inv {
            Operation::RotateBoard {
                old_rotation,
                new_rotation,
                ..
            } => {
                assert_eq!(old_rotation, [1.0, 2.0, 3.0]);
                assert_eq!(new_rotation, [0.0, 0.0, 0.0]);
            }
            _ => panic!("RotateBoard inverse should be RotateBoard"),
        }

        // SetSpecies: old/new swap
        let sp = Operation::SetSpecies {
            node_id: "b".to_string(),
            old_species: Some("oak".to_string()),
            new_species: Some("walnut".to_string()),
        };
        let sp_inv = sp.inverse();
        match sp_inv {
            Operation::SetSpecies {
                old_species,
                new_species,
                ..
            } => {
                assert_eq!(old_species, Some("walnut".to_string()));
                assert_eq!(new_species, Some("oak".to_string()));
            }
            _ => panic!("SetSpecies inverse should be SetSpecies"),
        }

        // SetFinish: old/new swap
        let fin = Operation::SetFinish {
            node_id: "b".to_string(),
            old_finish: None,
            new_finish: Some("satin".to_string()),
        };
        let fin_inv = fin.inverse();
        match fin_inv {
            Operation::SetFinish {
                old_finish,
                new_finish,
                ..
            } => {
                assert_eq!(old_finish, Some("satin".to_string()));
                assert_eq!(new_finish, None);
            }
            _ => panic!("SetFinish inverse should be SetFinish"),
        }

        // ResizeBoard: dimensions swap
        let resize = Operation::ResizeBoard {
            node_id: "b".to_string(),
            old_dimensions: [1.5, 3.5, 30.0],
            new_dimensions: [1.5, 5.5, 48.0],
        };
        let resize_inv = resize.inverse();
        match resize_inv {
            Operation::ResizeBoard {
                old_dimensions,
                new_dimensions,
                ..
            } => {
                assert_eq!(old_dimensions, [1.5, 5.5, 48.0]);
                assert_eq!(new_dimensions, [1.5, 3.5, 30.0]);
            }
            _ => panic!("ResizeBoard inverse should be ResizeBoard"),
        }

        // AddJoint <-> RemoveJoint
        let add_j = Operation::AddJoint {
            joint_id: "j1".to_string(),
            joint_type: "mortise_tenon".to_string(),
            board_a: "a".to_string(),
            board_b: "b".to_string(),
        };
        match add_j.inverse() {
            Operation::RemoveJoint { ref joint_id, .. } => assert_eq!(joint_id, "j1"),
            _ => panic!("AddJoint inverse should be RemoveJoint"),
        }

        let rem_j = Operation::RemoveJoint {
            joint_id: "j1".to_string(),
            joint_type: "mortise_tenon".to_string(),
            board_a: "a".to_string(),
            board_b: "b".to_string(),
            params_json: "{}".to_string(),
        };
        match rem_j.inverse() {
            Operation::AddJoint { ref joint_id, .. } => assert_eq!(joint_id, "j1"),
            _ => panic!("RemoveJoint inverse should be AddJoint"),
        }
    }

    #[test]
    fn test_max_history() {
        let mut history = History::new();
        // Record 105 operations (max is 100)
        for i in 0..105 {
            history.record(Operation::MoveBoard {
                node_id: format!("board-{}", i),
                old_position: [0.0, 0.0, 0.0],
                new_position: [i as f64, 0.0, 0.0],
            });
        }
        assert_eq!(history.undo_count(), 100);

        // The oldest 5 should have been dropped; first remaining should be board-5
        let first_undo = history.undo().unwrap();
        match first_undo {
            // We undo the most recent (board-104), getting its inverse
            Operation::MoveBoard { node_id, .. } => assert_eq!(node_id, "board-104"),
            _ => panic!("Expected MoveBoard"),
        }
    }

    #[test]
    fn test_descriptions() {
        let ops = vec![
            sample_add_board(),
            Operation::RemoveBoard {
                node_id: "b".to_string(),
                label: "Rail".to_string(),
                dimensions: [1.0, 2.0, 3.0],
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0],
                lumber_type: "Custom".to_string(),
                species_id: None,
            },
            sample_move(),
            Operation::RotateBoard {
                node_id: "b".to_string(),
                old_rotation: [0.0, 0.0, 0.0],
                new_rotation: [0.0, 1.57, 0.0],
            },
            Operation::AddJoint {
                joint_id: "j".to_string(),
                joint_type: "butt".to_string(),
                board_a: "a".to_string(),
                board_b: "b".to_string(),
            },
            Operation::RemoveJoint {
                joint_id: "j".to_string(),
                joint_type: "dado".to_string(),
                board_a: "a".to_string(),
                board_b: "b".to_string(),
                params_json: "{}".to_string(),
            },
            Operation::SetSpecies {
                node_id: "b".to_string(),
                old_species: None,
                new_species: Some("oak".to_string()),
            },
            Operation::SetFinish {
                node_id: "b".to_string(),
                old_finish: None,
                new_finish: Some("oil".to_string()),
            },
            Operation::ResizeBoard {
                node_id: "b".to_string(),
                old_dimensions: [1.0, 2.0, 3.0],
                new_dimensions: [4.0, 5.0, 6.0],
            },
        ];

        for op in &ops {
            let desc = op.description();
            assert!(
                !desc.is_empty(),
                "Description should not be empty for {:?}",
                op
            );
        }
    }

    #[test]
    fn test_empty_undo_redo() {
        let mut history = History::new();
        assert!(history.undo().is_none());
        assert!(history.redo().is_none());
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_clear() {
        let mut history = History::new();
        history.record(sample_add_board());
        history.record(sample_move());
        history.undo();

        assert!(history.can_undo());
        assert!(history.can_redo());

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }
}
