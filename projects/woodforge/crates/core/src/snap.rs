use serde::{Deserialize, Serialize};

use crate::types::{Dimensions3, Position3};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SnapType {
    Grid,
    Face,
    Edge,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapResult {
    pub snapped_position: Position3,
    pub snap_type: SnapType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapConfig {
    pub grid_enabled: bool,
    pub grid_size: f64,
    pub face_snap_enabled: bool,
    pub face_snap_threshold: f64,
    pub edge_snap_enabled: bool,
    pub edge_snap_threshold: f64,
}

impl Default for SnapConfig {
    fn default() -> Self {
        Self {
            grid_enabled: true,
            grid_size: 0.5,
            face_snap_enabled: true,
            face_snap_threshold: 0.5,
            edge_snap_enabled: true,
            edge_snap_threshold: 0.5,
        }
    }
}

/// AABB min/max corners for an axis-aligned box
fn board_aabb(pos: &Position3, dims: &Dimensions3) -> (Position3, Position3) {
    let min = Position3::new(
        pos.x - dims.width / 2.0,
        pos.y - dims.height / 2.0,
        pos.z - dims.depth / 2.0,
    );
    let max = Position3::new(
        pos.x + dims.width / 2.0,
        pos.y + dims.height / 2.0,
        pos.z + dims.depth / 2.0,
    );
    (min, max)
}

/// Snap proposed position to nearest grid, face, or edge.
/// `other_nodes` contains (position, dimensions) of all other boards in the scene.
pub fn compute_snap(
    proposed: &Position3,
    config: &SnapConfig,
    other_nodes: &[(Position3, Dimensions3)],
) -> SnapResult {
    let mut result = SnapResult {
        snapped_position: *proposed,
        snap_type: SnapType::None,
    };

    // Try face snap first (highest priority)
    if config.face_snap_enabled && !other_nodes.is_empty() {
        if let Some(face_snapped) = try_face_snap(proposed, config.face_snap_threshold, other_nodes) {
            return SnapResult {
                snapped_position: face_snapped,
                snap_type: SnapType::Face,
            };
        }
    }

    // Try edge snap
    if config.edge_snap_enabled && !other_nodes.is_empty() {
        if let Some(edge_snapped) = try_edge_snap(proposed, config.edge_snap_threshold, other_nodes) {
            return SnapResult {
                snapped_position: edge_snapped,
                snap_type: SnapType::Edge,
            };
        }
    }

    // Fall back to grid snap
    if config.grid_enabled {
        result.snapped_position = snap_to_grid(proposed, config.grid_size);
        result.snap_type = SnapType::Grid;
    }

    result
}

fn snap_to_grid(pos: &Position3, grid_size: f64) -> Position3 {
    Position3::new(
        (pos.x / grid_size).round() * grid_size,
        (pos.y / grid_size).round() * grid_size,
        (pos.z / grid_size).round() * grid_size,
    )
}

/// Try to snap each axis of `proposed` to a face of any other board.
/// Returns snapped position if any axis snaps.
fn try_face_snap(
    proposed: &Position3,
    threshold: f64,
    others: &[(Position3, Dimensions3)],
) -> Option<Position3> {
    let mut snapped = *proposed;
    let mut did_snap = false;

    for (other_pos, other_dims) in others {
        let (other_min, other_max) = board_aabb(other_pos, other_dims);

        // Check X faces
        let faces_x = [other_min.x, other_max.x];
        for &face_x in &faces_x {
            if (proposed.x - face_x).abs() < threshold {
                snapped.x = face_x;
                did_snap = true;
            }
        }

        // Check Y faces
        let faces_y = [other_min.y, other_max.y];
        for &face_y in &faces_y {
            if (proposed.y - face_y).abs() < threshold {
                snapped.y = face_y;
                did_snap = true;
            }
        }

        // Check Z faces
        let faces_z = [other_min.z, other_max.z];
        for &face_z in &faces_z {
            if (proposed.z - face_z).abs() < threshold {
                snapped.z = face_z;
                did_snap = true;
            }
        }
    }

    if did_snap { Some(snapped) } else { None }
}

/// Try to snap to edges (intersection of two faces on different axes).
fn try_edge_snap(
    proposed: &Position3,
    threshold: f64,
    others: &[(Position3, Dimensions3)],
) -> Option<Position3> {
    let mut snapped = *proposed;
    let mut snap_count = 0;

    for (other_pos, other_dims) in others {
        let (other_min, other_max) = board_aabb(other_pos, other_dims);

        // Edge = two axes match faces simultaneously
        let mut axis_snaps = [false; 3];
        let axes = [
            (proposed.x, other_min.x, other_max.x),
            (proposed.y, other_min.y, other_max.y),
            (proposed.z, other_min.z, other_max.z),
        ];

        for (i, &(val, face_min, face_max)) in axes.iter().enumerate() {
            if (val - face_min).abs() < threshold || (val - face_max).abs() < threshold {
                axis_snaps[i] = true;
            }
        }

        let axis_snap_count = axis_snaps.iter().filter(|&&s| s).count();
        if axis_snap_count >= 2 {
            // Snap to the nearest face on each snapping axis
            if axis_snaps[0] {
                let (min_x, max_x) = (other_min.x, other_max.x);
                snapped.x = if (proposed.x - min_x).abs() < (proposed.x - max_x).abs() { min_x } else { max_x };
            }
            if axis_snaps[1] {
                let (min_y, max_y) = (other_min.y, other_max.y);
                snapped.y = if (proposed.y - min_y).abs() < (proposed.y - max_y).abs() { min_y } else { max_y };
            }
            if axis_snaps[2] {
                let (min_z, max_z) = (other_min.z, other_max.z);
                snapped.z = if (proposed.z - min_z).abs() < (proposed.z - max_z).abs() { min_z } else { max_z };
            }
            snap_count += 1;
        }
    }

    if snap_count > 0 { Some(snapped) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_snap() {
        let config = SnapConfig {
            grid_enabled: true,
            grid_size: 0.5,
            face_snap_enabled: false,
            edge_snap_enabled: false,
            ..Default::default()
        };
        let result = compute_snap(&Position3::new(2.3, 1.1, 0.7), &config, &[]);
        assert_eq!(result.snap_type, SnapType::Grid);
        assert!((result.snapped_position.x - 2.5).abs() < 0.001);
        assert!((result.snapped_position.y - 1.0).abs() < 0.001);
        assert!((result.snapped_position.z - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_face_snap() {
        let config = SnapConfig {
            grid_enabled: false,
            face_snap_enabled: true,
            face_snap_threshold: 0.5,
            edge_snap_enabled: false,
            ..Default::default()
        };
        // Board at origin, 2x2x2 → faces at ±1
        let others = vec![(Position3::zero(), Dimensions3::new(2.0, 2.0, 2.0))];
        // Proposed position near the +X face (x=1.0)
        let result = compute_snap(&Position3::new(1.3, 0.0, 0.0), &config, &others);
        assert_eq!(result.snap_type, SnapType::Face);
        assert!((result.snapped_position.x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_no_snap() {
        let config = SnapConfig {
            grid_enabled: false,
            face_snap_enabled: false,
            edge_snap_enabled: false,
            ..Default::default()
        };
        let pos = Position3::new(5.0, 5.0, 5.0);
        let result = compute_snap(&pos, &config, &[]);
        assert_eq!(result.snap_type, SnapType::None);
        assert_eq!(result.snapped_position.x, 5.0);
    }
}
