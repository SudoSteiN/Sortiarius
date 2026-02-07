use crate::types::MeshData;

/// Generate a box mesh with proper face normals.
/// Box is centered at origin, extending from -dim/2 to +dim/2.
/// Width = X axis, Height = Y axis, Depth = Z axis.
///
/// Returns 24 vertices (4 per face × 6 faces) with face normals,
/// and 36 indices (2 triangles per face × 6 faces).
pub fn generate_board_mesh(width: f64, height: f64, depth: f64) -> MeshData {
    let hw = (width / 2.0) as f32;
    let hh = (height / 2.0) as f32;
    let hd = (depth / 2.0) as f32;

    // 6 faces × 4 vertices each = 24 vertices
    // Each face has its own normal for flat shading
    #[rustfmt::skip]
    let positions: Vec<f32> = vec![
        // Front face (+Z)
        -hw, -hh,  hd,
         hw, -hh,  hd,
         hw,  hh,  hd,
        -hw,  hh,  hd,
        // Back face (-Z)
         hw, -hh, -hd,
        -hw, -hh, -hd,
        -hw,  hh, -hd,
         hw,  hh, -hd,
        // Top face (+Y)
        -hw,  hh,  hd,
         hw,  hh,  hd,
         hw,  hh, -hd,
        -hw,  hh, -hd,
        // Bottom face (-Y)
        -hw, -hh, -hd,
         hw, -hh, -hd,
         hw, -hh,  hd,
        -hw, -hh,  hd,
        // Right face (+X)
         hw, -hh,  hd,
         hw, -hh, -hd,
         hw,  hh, -hd,
         hw,  hh,  hd,
        // Left face (-X)
        -hw, -hh, -hd,
        -hw, -hh,  hd,
        -hw,  hh,  hd,
        -hw,  hh, -hd,
    ];

    #[rustfmt::skip]
    let normals: Vec<f32> = vec![
        // Front face (+Z)
        0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0,
        // Back face (-Z)
        0.0, 0.0, -1.0,  0.0, 0.0, -1.0,  0.0, 0.0, -1.0,  0.0, 0.0, -1.0,
        // Top face (+Y)
        0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 1.0, 0.0,
        // Bottom face (-Y)
        0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0, -1.0, 0.0,  0.0, -1.0, 0.0,
        // Right face (+X)
        1.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0,
        // Left face (-X)
        -1.0, 0.0, 0.0,  -1.0, 0.0, 0.0,  -1.0, 0.0, 0.0,  -1.0, 0.0, 0.0,
    ];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        // Front
        0, 1, 2,  2, 3, 0,
        // Back
        4, 5, 6,  6, 7, 4,
        // Top
        8, 9, 10,  10, 11, 8,
        // Bottom
        12, 13, 14,  14, 15, 12,
        // Right
        16, 17, 18,  18, 19, 16,
        // Left
        20, 21, 22,  22, 23, 20,
    ];

    MeshData {
        positions,
        normals,
        indices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_mesh_counts() {
        let mesh = generate_board_mesh(1.5, 3.5, 96.0);
        assert_eq!(mesh.vertex_count(), 24);
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.positions.len(), 72); // 24 * 3
        assert_eq!(mesh.normals.len(), 72); // 24 * 3
        assert_eq!(mesh.indices.len(), 36); // 12 * 3
    }

    #[test]
    fn test_normals_unit_length() {
        let mesh = generate_board_mesh(2.0, 3.0, 4.0);
        for i in (0..mesh.normals.len()).step_by(3) {
            let nx = mesh.normals[i];
            let ny = mesh.normals[i + 1];
            let nz = mesh.normals[i + 2];
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            assert!((len - 1.0).abs() < 0.001, "Normal not unit length: {}", len);
        }
    }

    #[test]
    fn test_box_dimensions() {
        let mesh = generate_board_mesh(4.0, 6.0, 10.0);
        // Find min/max for each axis
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_z = f32::MAX;
        let mut max_z = f32::MIN;
        for i in (0..mesh.positions.len()).step_by(3) {
            min_x = min_x.min(mesh.positions[i]);
            max_x = max_x.max(mesh.positions[i]);
            min_y = min_y.min(mesh.positions[i + 1]);
            max_y = max_y.max(mesh.positions[i + 1]);
            min_z = min_z.min(mesh.positions[i + 2]);
            max_z = max_z.max(mesh.positions[i + 2]);
        }
        assert!((max_x - min_x - 4.0).abs() < 0.001);
        assert!((max_y - min_y - 6.0).abs() < 0.001);
        assert!((max_z - min_z - 10.0).abs() < 0.001);
    }
}
