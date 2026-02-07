pub mod types;
pub mod component;
pub mod units;
pub mod lumber;
pub mod geometry;
pub mod scene;
pub mod snap;
pub mod species;
pub mod grain;
pub mod finish;
pub mod joinery;
pub mod joinery_geometry;
pub mod structural;
pub mod output;
pub mod project;
pub mod history;
pub mod optimizer;
pub mod cost;

pub use types::*;
pub use component::*;
pub use units::*;
pub use lumber::*;
pub use geometry::*;
pub use scene::{SceneGraph, SceneNode, SceneTreeSnapshot, SceneNodeSnapshot};
pub use snap::*;
pub use species::{WoodSpecies, SpeciesDatabase, Workability};
pub use finish::{FinishType, StainFinish, PaintFinish, OilFinish, Sheen};
pub use joinery::{JointType, Joint, JointFace, JointParams, JointDifficulty, JointStore};
pub use structural::{LoadStatus, DeflectionResult, CompressionResult, JointStrengthResult};
pub use project::{Project, SavedBoard, SavedJoint, SavedFinish, SavedCamera, ProjectError};
pub use history::{Operation, History};
pub use optimizer::{OptimizationResult, CutLayout, CutPiece, StockBoard};
pub use cost::{CostEstimate, PriceDatabase, SpeciesPrice};
pub use output::{CutList, CutListEntry, MaterialList, BuildInstructions, BuildStep};

#[cfg(test)]
mod integration_tests {
    use crate::component::BoardComponent;
    use crate::history::{History, Operation};
    use crate::joinery::{Joint, JointFace, JointParams, JointStore, JointType};
    use crate::lumber::LumberCatalog;
    use crate::optimizer::{CutPiece, StockBoard};
    use crate::output::{BoardInfo, CutListInput};
    use crate::project::{Project, SavedBoard, SavedJoint};
    use crate::scene::SceneGraph;
    use crate::species::SpeciesDatabase;
    use crate::structural::{self, LoadStatus};
    use crate::types::{NodeId, Position3, Rotation3};
    use crate::units::{self, UnitSystem};

    // ---------------------------------------------------------------
    // 1. Full board workflow
    // ---------------------------------------------------------------
    #[test]
    fn test_full_board_workflow() {
        let catalog = LumberCatalog::new();
        let lumber_2x4 = catalog.find_by_label("2x4").unwrap();

        let mut scene = SceneGraph::new();

        // Add first board (standard 2x4, 96" long)
        let board1_nid = NodeId::new();
        let board1 = BoardComponent::new_standard(board1_nid, lumber_2x4, 96.0);
        let id1 = scene.add_node(
            "2x4 Leg".into(),
            Some(board1),
            Position3::zero(),
            Rotation3::zero(),
        );

        // Verify node has correct dimensions
        let node1 = scene.get_node(id1).unwrap();
        let dims1 = node1.component.as_ref().unwrap().dimensions;
        assert!((dims1.width - 1.5).abs() < 0.001, "2x4 actual width should be 1.5");
        assert!((dims1.height - 3.5).abs() < 0.001, "2x4 actual height should be 3.5");
        assert!((dims1.depth - 96.0).abs() < 0.001, "Length should be 96.0");

        // Add second board (1x6)
        let lumber_1x6 = catalog.find_by_label("1x6").unwrap();
        let board2_nid = NodeId::new();
        let board2 = BoardComponent::new_standard(board2_nid, lumber_1x6, 48.0);
        let id2 = scene.add_node(
            "1x6 Shelf".into(),
            Some(board2),
            Position3::new(10.0, 0.0, 0.0),
            Rotation3::zero(),
        );

        // Verify both exist
        assert_eq!(scene.node_count(), 2);

        // Snapshot should contain both
        let snap = scene.snapshot();
        assert_eq!(snap.nodes.len(), 2);

        let labels: Vec<&str> = snap.nodes.iter().map(|n| n.label.as_str()).collect();
        assert!(labels.contains(&"2x4 Leg"), "Snapshot should contain 2x4 Leg");
        assert!(labels.contains(&"1x6 Shelf"), "Snapshot should contain 1x6 Shelf");

        // Both should have meshes
        assert!(scene.get_mesh(id1).is_some(), "Board 1 should have a mesh");
        assert!(scene.get_mesh(id2).is_some(), "Board 2 should have a mesh");
    }

    // ---------------------------------------------------------------
    // 2. Species + Structural analysis
    // ---------------------------------------------------------------
    #[test]
    fn test_species_structural_workflow() {
        let species_db = SpeciesDatabase::new();
        let pine = species_db.find_by_id("pine_sy").unwrap();

        // Verify species properties are reasonable
        assert!(pine.modulus_of_elasticity > 0.0);
        assert!(pine.density_lb_ft3 > 0.0);

        // Create a 1x10 shelf board: 0.75" thick x 9.25" wide, 36" span
        // Use pine MOE for deflection analysis
        let result = structural::analyze_shelf(
            36.0,   // span_inches
            9.25,   // board_width (what is actually the "width" perpendicular to load)
            0.75,   // board_height (thickness in direction of load)
            pine.modulus_of_elasticity,
            50.0,   // total_load_lbs
        );

        // The result should have a valid status
        assert!(
            result.status == LoadStatus::Green
                || result.status == LoadStatus::Yellow
                || result.status == LoadStatus::Red,
            "Status should be a valid LoadStatus variant"
        );
        assert!(result.deflection_inches >= 0.0, "Deflection should be non-negative");
        assert!(result.max_allowable > 0.0, "Max allowable should be positive");
        assert!(!result.span_ratio.is_empty(), "Span ratio string should not be empty");

        // A thin 1x10 under 50 lbs on 36" span -- expect noticeable deflection
        assert!(result.deflection_inches > 0.0, "Should have some deflection with load");
    }

    // ---------------------------------------------------------------
    // 3. Joinery workflow
    // ---------------------------------------------------------------
    #[test]
    fn test_joinery_workflow() {
        let mut store = JointStore::new();
        let board_a = NodeId::new();
        let board_b = NodeId::new();
        let joint_id = NodeId::new();

        let joint = Joint {
            id: joint_id,
            joint_type: JointType::MortiseAndTenon,
            board_a,
            board_b,
            face_a: JointFace::Right,
            face_b: JointFace::Left,
            offset: Position3::zero(),
            parameters: JointParams::MortiseAndTenon {
                tenon_width: 1.0,
                tenon_height: 1.0,
                tenon_depth: 2.0,
            },
        };

        store.add_joint(joint);

        // Verify joints_for_board returns the joint for both boards
        let joints_a = store.joints_for_board(board_a);
        assert_eq!(joints_a.len(), 1, "Board A should have 1 joint");
        assert_eq!(joints_a[0].id, joint_id);
        assert_eq!(joints_a[0].joint_type, JointType::MortiseAndTenon);

        let joints_b = store.joints_for_board(board_b);
        assert_eq!(joints_b.len(), 1, "Board B should have 1 joint");

        // An unrelated board should have no joints
        let joints_c = store.joints_for_board(NodeId::new());
        assert_eq!(joints_c.len(), 0, "Unrelated board should have 0 joints");

        // Remove the joint
        store.remove_joint(joint_id);

        // Verify empty
        assert_eq!(
            store.joints_for_board(board_a).len(),
            0,
            "After removal, board A should have 0 joints"
        );
        assert_eq!(
            store.joints_for_board(board_b).len(),
            0,
            "After removal, board B should have 0 joints"
        );
        assert_eq!(store.all_joints().len(), 0, "Store should be empty");
    }

    // ---------------------------------------------------------------
    // 4. Cut list + Optimization
    // ---------------------------------------------------------------
    #[test]
    fn test_cut_list_and_optimization() {
        let input = CutListInput {
            boards: vec![
                BoardInfo {
                    node_id: "n1".into(),
                    label: "Side A".into(),
                    width: 3.5,
                    height: 0.75,
                    depth: 24.0,
                    material: "Pine".into(),
                    grain_direction: "AlongLength".into(),
                },
                BoardInfo {
                    node_id: "n2".into(),
                    label: "Side B".into(),
                    width: 3.5,
                    height: 0.75,
                    depth: 36.0,
                    material: "Pine".into(),
                    grain_direction: "AlongLength".into(),
                },
                BoardInfo {
                    node_id: "n3".into(),
                    label: "Top".into(),
                    width: 5.5,
                    height: 0.75,
                    depth: 48.0,
                    material: "Pine".into(),
                    grain_direction: "AlongLength".into(),
                },
            ],
            joints: vec![],
        };

        // Generate the cut list
        let cut_list = crate::output::generate_cut_list(&input);
        assert!(
            !cut_list.entries.is_empty(),
            "Cut list should have entries for 3 boards"
        );
        assert!(cut_list.total_pieces >= 3, "Should have at least 3 total pieces");

        // Build CutPiece list from cut list entries for the optimizer
        let mut pieces: Vec<CutPiece> = Vec::new();
        for entry in &cut_list.entries {
            for i in 0..entry.quantity {
                pieces.push(CutPiece {
                    id: format!("{}-{}", entry.piece_id, i),
                    label: entry.label.clone(),
                    length: entry.length,
                    width: entry.width,
                    material: entry.material.clone(),
                });
            }
        }

        let stock = vec![StockBoard {
            label: "1x4 x 8'".into(),
            length: 96.0,
            width: 3.5,
            height: 0.75,
        }, StockBoard {
            label: "1x6 x 8'".into(),
            length: 96.0,
            width: 5.5,
            height: 0.75,
        }];

        let result = crate::optimizer::optimize_cuts(&pieces, &stock, 0.125);

        // Should have stock boards in the result
        assert!(
            result.total_stock_boards > 0,
            "Optimizer should produce at least 1 stock board"
        );
        // Waste percentage should be a valid number
        assert!(
            result.total_waste_percent >= 0.0 && result.total_waste_percent <= 100.0,
            "Waste percent ({}) should be between 0 and 100",
            result.total_waste_percent
        );
        // All layouts should have placements
        for layout in &result.layouts {
            assert!(
                !layout.placements.is_empty(),
                "Each layout should have at least one placement"
            );
            assert!(layout.utilization > 0.0, "Utilization should be positive");
        }
    }

    // ---------------------------------------------------------------
    // 5. Project save/load round-trip
    // ---------------------------------------------------------------
    #[test]
    fn test_project_save_load_roundtrip() {
        // Create a project with 2 boards and 1 joint
        let mut project = Project::new("Integration Test Bookshelf".to_string());
        project.description = "A test bookshelf project".to_string();
        project.unit_system = UnitSystem::Imperial;

        let board1_id = NodeId::new().to_string();
        let board2_id = NodeId::new().to_string();

        project.boards.push(SavedBoard {
            id: board1_id.clone(),
            label: "Left Side".into(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            dimensions: [0.75, 11.25, 48.0],
            parent_id: None,
            lumber_type: "Standard:1x12".into(),
            species_id: Some("pine_sy".into()),
            finish_id: None,
            grain_direction: "AlongLength".into(),
        });
        project.boards.push(SavedBoard {
            id: board2_id.clone(),
            label: "Shelf".into(),
            position: [0.0, 12.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            dimensions: [0.75, 11.25, 30.0],
            parent_id: None,
            lumber_type: "Standard:1x12".into(),
            species_id: Some("pine_sy".into()),
            finish_id: None,
            grain_direction: "AlongLength".into(),
        });

        let joint_id = NodeId::new().to_string();
        project.joints.push(SavedJoint {
            id: joint_id.clone(),
            joint_type: "Dado".into(),
            board_a_id: board1_id.clone(),
            board_b_id: board2_id.clone(),
            face_a: "Right".into(),
            face_b: "Left".into(),
            offset: [0.0, 12.0, 0.0],
            parameters: serde_json::json!({"width": 0.75, "depth": 0.375}),
        });

        // Serialize
        let json = project.to_json().expect("Serialization should succeed");
        assert!(!json.is_empty(), "JSON output should not be empty");

        // Deserialize
        let loaded = Project::from_json(&json).expect("Deserialization should succeed");

        // Verify the loaded data matches
        assert_eq!(loaded.name, "Integration Test Bookshelf");
        assert_eq!(loaded.description, "A test bookshelf project");
        assert_eq!(loaded.unit_system, UnitSystem::Imperial);
        assert_eq!(loaded.boards.len(), 2);
        assert_eq!(loaded.joints.len(), 1);

        // Check board data round-tripped
        assert_eq!(loaded.boards[0].id, board1_id);
        assert_eq!(loaded.boards[0].label, "Left Side");
        assert_eq!(loaded.boards[0].dimensions, [0.75, 11.25, 48.0]);
        assert_eq!(loaded.boards[1].id, board2_id);
        assert_eq!(loaded.boards[1].label, "Shelf");

        // Check joint data round-tripped
        assert_eq!(loaded.joints[0].id, joint_id);
        assert_eq!(loaded.joints[0].joint_type, "Dado");
        assert_eq!(loaded.joints[0].board_a_id, board1_id);
        assert_eq!(loaded.joints[0].board_b_id, board2_id);

        // Validation should pass with no errors
        let warnings = loaded.validate();
        let errors: Vec<_> = warnings
            .iter()
            .filter(|w| w.severity == crate::project::WarningSeverity::Error)
            .collect();
        assert!(errors.is_empty(), "Loaded project should have no validation errors");
    }

    // ---------------------------------------------------------------
    // 6. History undo/redo
    // ---------------------------------------------------------------
    #[test]
    fn test_history_undo_redo() {
        let mut history = History::new();

        // Initially nothing to undo/redo
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // Record adding a board
        history.record(Operation::AddBoard {
            node_id: "board-1".into(),
            label: "Leg".into(),
            dimensions: [1.5, 3.5, 30.0],
            position: [0.0, 0.0, 0.0],
            lumber_type: "Standard:2x4".into(),
        });

        // Record moving it
        history.record(Operation::MoveBoard {
            node_id: "board-1".into(),
            old_position: [0.0, 0.0, 0.0],
            new_position: [10.0, 0.0, 0.0],
        });

        // Record resizing it
        history.record(Operation::ResizeBoard {
            node_id: "board-1".into(),
            old_dimensions: [1.5, 3.5, 30.0],
            new_dimensions: [1.5, 5.5, 48.0],
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 3);

        // Undo the resize -- should get inverse (resize back to old dimensions)
        let undo_op = history.undo().unwrap();
        match undo_op {
            Operation::ResizeBoard {
                old_dimensions,
                new_dimensions,
                ..
            } => {
                // Inverse swaps old and new
                assert_eq!(old_dimensions, [1.5, 5.5, 48.0]);
                assert_eq!(new_dimensions, [1.5, 3.5, 30.0]);
            }
            _ => panic!("Expected ResizeBoard inverse"),
        }
        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 1);
        assert!(history.can_redo());

        // Undo the move
        let undo_op2 = history.undo().unwrap();
        match undo_op2 {
            Operation::MoveBoard {
                old_position,
                new_position,
                ..
            } => {
                assert_eq!(old_position, [10.0, 0.0, 0.0]);
                assert_eq!(new_position, [0.0, 0.0, 0.0]);
            }
            _ => panic!("Expected MoveBoard inverse"),
        }
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 2);

        // Redo the move
        let redo_op = history.redo().unwrap();
        match redo_op {
            Operation::MoveBoard {
                old_position,
                new_position,
                ..
            } => {
                assert_eq!(old_position, [0.0, 0.0, 0.0]);
                assert_eq!(new_position, [10.0, 0.0, 0.0]);
            }
            _ => panic!("Expected MoveBoard redo"),
        }
        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 1);

        // Record a new operation -- should clear redo stack
        history.record(Operation::SetSpecies {
            node_id: "board-1".into(),
            old_species: None,
            new_species: Some("pine_sy".into()),
        });
        assert!(!history.can_redo(), "New record should clear redo stack");
        assert_eq!(history.redo_count(), 0);
        assert_eq!(history.undo_count(), 3);
    }

    // ---------------------------------------------------------------
    // 7. Unit conversions + formatting round-trip
    // ---------------------------------------------------------------
    #[test]
    fn test_unit_conversions_and_formatting() {
        // Test inches -> mm -> inches round-trip
        let original_inches = 3.5;
        let mm = units::inches_to_mm(original_inches);
        let back_to_inches = units::mm_to_inches(mm);
        assert!(
            (back_to_inches - original_inches).abs() < 0.0001,
            "Inches -> mm -> inches round-trip failed: {} != {}",
            back_to_inches,
            original_inches
        );

        // Test inches -> feet -> inches round-trip
        let feet = units::inches_to_feet(96.0);
        assert!((feet - 8.0).abs() < 0.001, "96 inches should be 8 feet");
        let back = units::feet_to_inches(feet);
        assert!((back - 96.0).abs() < 0.001, "8 feet should be 96 inches");

        // Format in imperial -- should show fractions
        let imperial_str = units::format_dimension(1.5, UnitSystem::Imperial);
        assert!(
            imperial_str.contains("1/2"),
            "Imperial format of 1.5 should contain '1/2', got '{}'",
            imperial_str
        );

        let imperial_3_4 = units::format_dimension(0.75, UnitSystem::Imperial);
        assert!(
            imperial_3_4.contains("3/4"),
            "Imperial format of 0.75 should contain '3/4', got '{}'",
            imperial_3_4
        );

        // Format in metric -- should show mm
        let metric_str = units::format_dimension(3.5, UnitSystem::Metric);
        assert!(
            metric_str.contains("mm"),
            "Metric format should contain 'mm', got '{}'",
            metric_str
        );
        assert!(
            metric_str.contains("88.9"),
            "Metric format of 3.5\" should show 88.9 mm, got '{}'",
            metric_str
        );

        // Parse back from imperial string
        let parsed_imperial = units::parse_dimension("1-1/2\"").unwrap();
        assert!(
            (parsed_imperial - 1.5).abs() < 0.001,
            "Parsing '1-1/2\"' should give 1.5, got {}",
            parsed_imperial
        );

        let parsed_3_4 = units::parse_dimension("3/4\"").unwrap();
        assert!(
            (parsed_3_4 - 0.75).abs() < 0.001,
            "Parsing '3/4\"' should give 0.75, got {}",
            parsed_3_4
        );

        // Parse back from metric string
        let parsed_metric = units::parse_dimension("88.9mm").unwrap();
        assert!(
            (parsed_metric - 3.5).abs() < 0.01,
            "Parsing '88.9mm' should give ~3.5, got {}",
            parsed_metric
        );

        // Full round-trip: dimension -> format imperial -> parse -> verify
        let dim = 7.25; // 2x8 actual height
        let formatted = units::format_dimension(dim, UnitSystem::Imperial);
        assert!(
            formatted.contains("1/4"),
            "7.25 should format with 1/4 fraction, got '{}'",
            formatted
        );
        let parsed_back = units::parse_dimension(&formatted).unwrap();
        assert!(
            (parsed_back - dim).abs() < 0.01,
            "Round-trip of 7.25 failed: formatted='{}', parsed back={}",
            formatted,
            parsed_back
        );

        // Full round-trip: dimension -> format metric -> parse -> verify
        let dim2 = 1.5;
        let formatted_metric = units::format_dimension(dim2, UnitSystem::Metric);
        let parsed_metric2 = units::parse_dimension(&formatted_metric).unwrap();
        assert!(
            (parsed_metric2 - dim2).abs() < 0.01,
            "Metric round-trip of 1.5 failed: formatted='{}', parsed={}",
            formatted_metric,
            parsed_metric2
        );
    }
}
