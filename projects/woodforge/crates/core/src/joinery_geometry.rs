use crate::joinery::{JointFace, JointParams};
use crate::types::{Dimensions3, MeshData};

/// Build a single quad face from 4 corners and a normal.
/// Corners must be in CCW winding order when viewed from the front.
/// Returns (positions, normals, indices) with indices starting at 0.
fn build_box_face(corners: [[f32; 3]; 4], normal: [f32; 3]) -> (Vec<f32>, Vec<f32>, Vec<u32>) {
    let mut positions = Vec::with_capacity(12);
    let mut normals = Vec::with_capacity(12);
    for c in &corners {
        positions.extend_from_slice(c);
        normals.extend_from_slice(&normal);
    }
    let indices = vec![0, 1, 2, 0, 2, 3];
    (positions, normals, indices)
}

/// Merge multiple face tuples (positions, normals, indices) into a single MeshData.
fn merge_faces(faces: Vec<(Vec<f32>, Vec<f32>, Vec<u32>)>) -> MeshData {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for (fp, fn_, fi) in faces {
        let base = (positions.len() / 3) as u32;
        positions.extend_from_slice(&fp);
        normals.extend_from_slice(&fn_);
        for idx in fi {
            indices.push(base + idx);
        }
    }

    // Generate default UVs (0,0) for all vertices - joinery meshes
    // get proper UVs when re-tessellated through the geometry module
    let vertex_count = positions.len() / 3;
    let uvs = vec![0.0_f32; vertex_count * 2];

    MeshData {
        positions,
        normals,
        uvs,
        indices,
    }
}

/// Build a standard axis-aligned box mesh centered at origin.
fn build_box(w: f32, h: f32, d: f32) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;

    let faces = vec![
        // Front (+Z)
        build_box_face(
            [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]],
            [0.0, 0.0, 1.0],
        ),
        // Back (-Z)
        build_box_face(
            [
                [hw, -hh, -hd],
                [-hw, -hh, -hd],
                [-hw, hh, -hd],
                [hw, hh, -hd],
            ],
            [0.0, 0.0, -1.0],
        ),
        // Top (+Y)
        build_box_face(
            [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]],
            [0.0, 1.0, 0.0],
        ),
        // Bottom (-Y)
        build_box_face(
            [
                [-hw, -hh, -hd],
                [hw, -hh, -hd],
                [hw, -hh, hd],
                [-hw, -hh, hd],
            ],
            [0.0, -1.0, 0.0],
        ),
        // Right (+X)
        build_box_face(
            [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]],
            [1.0, 0.0, 0.0],
        ),
        // Left (-X)
        build_box_face(
            [
                [-hw, -hh, -hd],
                [-hw, -hh, hd],
                [-hw, hh, hd],
                [-hw, hh, -hd],
            ],
            [-1.0, 0.0, 0.0],
        ),
    ];

    merge_faces(faces)
}

/// Apply a joint's geometry modification to a board's mesh.
/// Returns a new MeshData with the modification applied.
pub fn apply_joint_geometry(
    mesh: &MeshData,
    dims: &Dimensions3,
    face: JointFace,
    params: &JointParams,
) -> MeshData {
    let w = dims.width as f32;
    let h = dims.height as f32;
    let d = dims.depth as f32;

    match params {
        JointParams::Butt | JointParams::Generic => mesh.clone(),
        JointParams::Miter { angle } => build_miter(w, h, d, face, *angle as f32),
        JointParams::Dado { width, depth } => {
            build_dado(w, h, d, face, *width as f32, *depth as f32)
        }
        JointParams::Rabbet { width, depth } => {
            build_rabbet(w, h, d, face, *width as f32, *depth as f32)
        }
        JointParams::HalfLap { depth } => build_half_lap(w, h, d, face, *depth as f32),
        JointParams::MortiseAndTenon {
            tenon_width,
            tenon_height,
            tenon_depth,
        } => build_mortise(
            w,
            h,
            d,
            face,
            *tenon_width as f32,
            *tenon_height as f32,
            *tenon_depth as f32,
        ),
        // Joint types without MVP geometry support
        _ => mesh.clone(),
    }
}

/// Miter: cut the specified face at an angle.
/// Replaces the face with an angled surface.
fn build_miter(w: f32, h: f32, d: f32, face: JointFace, angle_deg: f32) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;
    let angle_rad = angle_deg.to_radians();
    let cut = angle_rad.tan();

    // The miter removes a triangular wedge from the specified face.
    // We build the remaining shape as individual faces.
    match face {
        JointFace::Front => {
            // Cut on the +Z face. The cut goes from full depth at top to reduced at bottom.
            let cut_depth = (h * cut).min(d);
            let new_front_z = hd - cut_depth;

            let faces = vec![
                // Angled front face (the miter cut)
                build_box_face(
                    [
                        [-hw, -hh, hd],
                        [hw, -hh, hd],
                        [hw, hh, new_front_z],
                        [-hw, hh, new_front_z],
                    ],
                    [0.0, angle_rad.sin(), angle_rad.cos()],
                ),
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Top (+Y) - shortened
                build_box_face(
                    [
                        [-hw, hh, new_front_z],
                        [hw, hh, new_front_z],
                        [hw, hh, -hd],
                        [-hw, hh, -hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X) - trapezoid
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, -hh, -hd],
                        [hw, hh, -hd],
                        [hw, hh, new_front_z],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X) - trapezoid
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, hh, new_front_z],
                        [-hw, hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        JointFace::Back => {
            let cut_depth = (h * cut).min(d);
            let new_back_z = -hd + cut_depth;

            let faces = vec![
                // Front (+Z)
                build_box_face(
                    [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Angled back face
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, new_back_z],
                        [hw, hh, new_back_z],
                    ],
                    [0.0, angle_rad.sin(), -angle_rad.cos()],
                ),
                // Top (+Y)
                build_box_face(
                    [
                        [-hw, hh, hd],
                        [hw, hh, hd],
                        [hw, hh, new_back_z],
                        [-hw, hh, new_back_z],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X)
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, -hh, -hd],
                        [hw, hh, new_back_z],
                        [hw, hh, hd],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, hh, hd],
                        [-hw, hh, new_back_z],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        // For other faces, apply same pattern rotated. MVP focuses on Front/Back.
        _ => build_box(w, h, d),
    }
}

/// Dado: rectangular channel cut across the face.
/// The channel runs perpendicular to the long axis of the face.
fn build_dado(
    w: f32,
    h: f32,
    d: f32,
    face: JointFace,
    channel_width: f32,
    channel_depth: f32,
) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;
    let cw2 = channel_width / 2.0;
    let cd = channel_depth.min(h); // clamp

    match face {
        JointFace::Top => {
            // Channel runs along X axis, cut into +Y face
            // The board is split into: left section, channel, right section on top
            let channel_floor_y = hh - cd;

            let faces = vec![
                // Front (+Z) - full face
                build_box_face(
                    [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Back (-Z) - full face
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Top left of channel (+Y, z < -cw2)
                build_box_face(
                    [[-hw, hh, hd], [hw, hh, hd], [hw, hh, cw2], [-hw, hh, cw2]],
                    [0.0, 1.0, 0.0],
                ),
                // Top right of channel (+Y, z > cw2)
                build_box_face(
                    [
                        [-hw, hh, -cw2],
                        [hw, hh, -cw2],
                        [hw, hh, -hd],
                        [-hw, hh, -hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Channel floor
                build_box_face(
                    [
                        [-hw, channel_floor_y, cw2],
                        [hw, channel_floor_y, cw2],
                        [hw, channel_floor_y, -cw2],
                        [-hw, channel_floor_y, -cw2],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Channel wall near (+Z side)
                build_box_face(
                    [
                        [-hw, channel_floor_y, cw2],
                        [-hw, hh, cw2],
                        [hw, hh, cw2],
                        [hw, channel_floor_y, cw2],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Channel wall far (-Z side)
                build_box_face(
                    [
                        [hw, channel_floor_y, -cw2],
                        [hw, hh, -cw2],
                        [-hw, hh, -cw2],
                        [-hw, channel_floor_y, -cw2],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Bottom (-Y) - full face
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X) - full face
                build_box_face(
                    [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X) - full face
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, hh, hd],
                        [-hw, hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        JointFace::Front => {
            // Channel runs along X axis, cut into +Z face
            let channel_floor_z = hd - cd;

            let faces = vec![
                // Front left of channel
                build_box_face(
                    [
                        [-hw, -hh, hd],
                        [hw, -hh, hd],
                        [hw, -cw2, hd],
                        [-hw, -cw2, hd],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Front right of channel
                build_box_face(
                    [[-hw, cw2, hd], [hw, cw2, hd], [hw, hh, hd], [-hw, hh, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Channel floor
                build_box_face(
                    [
                        [-hw, -cw2, channel_floor_z],
                        [hw, -cw2, channel_floor_z],
                        [hw, cw2, channel_floor_z],
                        [-hw, cw2, channel_floor_z],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Channel wall bottom
                build_box_face(
                    [
                        [-hw, -cw2, hd],
                        [hw, -cw2, hd],
                        [hw, -cw2, channel_floor_z],
                        [-hw, -cw2, channel_floor_z],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Channel wall top
                build_box_face(
                    [
                        [hw, cw2, hd],
                        [-hw, cw2, hd],
                        [-hw, cw2, channel_floor_z],
                        [hw, cw2, channel_floor_z],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Top (+Y)
                build_box_face(
                    [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]],
                    [0.0, 1.0, 0.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X)
                build_box_face(
                    [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, hh, hd],
                        [-hw, hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        _ => build_box(w, h, d),
    }
}

/// Rabbet: L-shaped cut along the edge of the board at the specified face.
fn build_rabbet(
    w: f32,
    h: f32,
    d: f32,
    face: JointFace,
    rabbet_width: f32,
    rabbet_depth: f32,
) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;
    let rw = rabbet_width.min(w);
    let rd = rabbet_depth.min(h);

    match face {
        JointFace::Front => {
            // L-cut removes top-front corner: from top face down by rd, from front face back by rw
            let cut_y = hh - rd;
            let cut_z = hd - rw;

            let faces = vec![
                // Front face (lower portion, below cut)
                build_box_face(
                    [
                        [-hw, -hh, hd],
                        [hw, -hh, hd],
                        [hw, cut_y, hd],
                        [-hw, cut_y, hd],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Rabbet horizontal surface (step top)
                build_box_face(
                    [
                        [-hw, cut_y, hd],
                        [hw, cut_y, hd],
                        [hw, cut_y, cut_z],
                        [-hw, cut_y, cut_z],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Rabbet vertical surface (step wall)
                build_box_face(
                    [
                        [-hw, cut_y, cut_z],
                        [hw, cut_y, cut_z],
                        [hw, hh, cut_z],
                        [-hw, hh, cut_z],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Top face (shortened)
                build_box_face(
                    [
                        [-hw, hh, cut_z],
                        [hw, hh, cut_z],
                        [hw, hh, -hd],
                        [-hw, hh, -hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X) - L-shaped, split into two quads
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, -hh, -hd],
                        [hw, hh, -hd],
                        [hw, hh, cut_z],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, hh, cut_z],
                        [hw, cut_y, cut_z],
                        [hw, cut_y, hd],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X) - L-shaped, split into two quads
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, cut_y, hd],
                        [-hw, cut_y, cut_z],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [-hw, cut_y, cut_z],
                        [-hw, hh, cut_z],
                        [-hw, hh, -hd],
                        [-hw, -hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        _ => build_box(w, h, d),
    }
}

/// Half-lap: remove material from one half of the board's thickness at the specified face.
fn build_half_lap(w: f32, h: f32, d: f32, face: JointFace, lap_depth: f32) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;
    let ld = lap_depth.min(h);

    match face {
        JointFace::Front => {
            // Remove top half at the front end. The step goes from full height at back
            // to reduced height at front. We cut a notch: top material removed for depth ld
            // from the front face back some distance. For a half-lap the removal extends
            // the full depth of the mating board. We use half the board depth as default.
            let step_z = 0.0; // step at midpoint of depth
            let step_y = hh - ld;

            let faces = vec![
                // Front face (reduced height)
                build_box_face(
                    [
                        [-hw, -hh, hd],
                        [hw, -hh, hd],
                        [hw, step_y, hd],
                        [-hw, step_y, hd],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Step top surface
                build_box_face(
                    [
                        [-hw, step_y, hd],
                        [hw, step_y, hd],
                        [hw, step_y, step_z],
                        [-hw, step_y, step_z],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Step wall
                build_box_face(
                    [
                        [-hw, step_y, step_z],
                        [hw, step_y, step_z],
                        [hw, hh, step_z],
                        [-hw, hh, step_z],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Top face (back half only)
                build_box_face(
                    [
                        [-hw, hh, step_z],
                        [hw, hh, step_z],
                        [hw, hh, -hd],
                        [-hw, hh, -hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X) - L-shape
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, -hh, -hd],
                        [hw, hh, -hd],
                        [hw, hh, step_z],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, hh, step_z],
                        [hw, step_y, step_z],
                        [hw, step_y, hd],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X) - L-shape
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, step_y, hd],
                        [-hw, step_y, step_z],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [-hw, step_y, step_z],
                        [-hw, hh, step_z],
                        [-hw, hh, -hd],
                        [-hw, -hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        JointFace::Top => {
            // Remove from top, step at midpoint along depth
            let step_y = hh - ld;
            let step_z = 0.0;

            let faces = vec![
                // Front (+Z)
                build_box_face(
                    [
                        [-hw, -hh, hd],
                        [hw, -hh, hd],
                        [hw, step_y, hd],
                        [-hw, step_y, hd],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Reduced top (front half)
                build_box_face(
                    [
                        [-hw, step_y, hd],
                        [hw, step_y, hd],
                        [hw, step_y, step_z],
                        [-hw, step_y, step_z],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Full top (back half)
                build_box_face(
                    [
                        [-hw, hh, step_z],
                        [hw, hh, step_z],
                        [hw, hh, -hd],
                        [-hw, hh, -hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Step wall
                build_box_face(
                    [
                        [-hw, step_y, step_z],
                        [hw, step_y, step_z],
                        [hw, hh, step_z],
                        [-hw, hh, step_z],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X)
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, -hh, -hd],
                        [hw, hh, -hd],
                        [hw, hh, step_z],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [hw, -hh, hd],
                        [hw, hh, step_z],
                        [hw, step_y, step_z],
                        [hw, step_y, hd],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, step_y, hd],
                        [-hw, step_y, step_z],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
                build_box_face(
                    [
                        [-hw, step_y, step_z],
                        [-hw, hh, step_z],
                        [-hw, hh, -hd],
                        [-hw, -hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        _ => build_box(w, h, d),
    }
}

/// Mortise: rectangular pocket cut into the specified face.
fn build_mortise(
    w: f32,
    h: f32,
    d: f32,
    face: JointFace,
    tenon_w: f32,
    tenon_h: f32,
    tenon_d: f32,
) -> MeshData {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;

    // Mortise pocket dimensions (centered on the face)
    let pw = (tenon_w / 2.0).min(hw);
    let ph = (tenon_h / 2.0).min(hh);
    let pocket_depth = tenon_d.min(d);

    match face {
        JointFace::Front => {
            // Rectangular pocket in the +Z face
            let pocket_floor_z = hd - pocket_depth;

            let faces = vec![
                // Front face - 4 quads around the pocket opening
                // Bottom strip
                build_box_face(
                    [[-hw, -hh, hd], [hw, -hh, hd], [hw, -ph, hd], [-hw, -ph, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Top strip
                build_box_face(
                    [[-hw, ph, hd], [hw, ph, hd], [hw, hh, hd], [-hw, hh, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Left strip
                build_box_face(
                    [[-hw, -ph, hd], [-pw, -ph, hd], [-pw, ph, hd], [-hw, ph, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Right strip
                build_box_face(
                    [[pw, -ph, hd], [hw, -ph, hd], [hw, ph, hd], [pw, ph, hd]],
                    [0.0, 0.0, 1.0],
                ),
                // Pocket interior walls
                // Floor
                build_box_face(
                    [
                        [-pw, -ph, pocket_floor_z],
                        [pw, -ph, pocket_floor_z],
                        [pw, ph, pocket_floor_z],
                        [-pw, ph, pocket_floor_z],
                    ],
                    [0.0, 0.0, 1.0],
                ),
                // Left wall
                build_box_face(
                    [
                        [-pw, -ph, hd],
                        [-pw, -ph, pocket_floor_z],
                        [-pw, ph, pocket_floor_z],
                        [-pw, ph, hd],
                    ],
                    [1.0, 0.0, 0.0],
                ),
                // Right wall
                build_box_face(
                    [
                        [pw, -ph, pocket_floor_z],
                        [pw, -ph, hd],
                        [pw, ph, hd],
                        [pw, ph, pocket_floor_z],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
                // Bottom wall
                build_box_face(
                    [
                        [-pw, -ph, pocket_floor_z],
                        [-pw, -ph, hd],
                        [pw, -ph, hd],
                        [pw, -ph, pocket_floor_z],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Top wall
                build_box_face(
                    [
                        [-pw, ph, hd],
                        [-pw, ph, pocket_floor_z],
                        [pw, ph, pocket_floor_z],
                        [pw, ph, hd],
                    ],
                    [0.0, 1.0, 0.0],
                ),
                // Remaining 5 outer faces (unchanged)
                // Back (-Z)
                build_box_face(
                    [
                        [hw, -hh, -hd],
                        [-hw, -hh, -hd],
                        [-hw, hh, -hd],
                        [hw, hh, -hd],
                    ],
                    [0.0, 0.0, -1.0],
                ),
                // Top (+Y)
                build_box_face(
                    [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]],
                    [0.0, 1.0, 0.0],
                ),
                // Bottom (-Y)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [hw, -hh, -hd],
                        [hw, -hh, hd],
                        [-hw, -hh, hd],
                    ],
                    [0.0, -1.0, 0.0],
                ),
                // Right (+X)
                build_box_face(
                    [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]],
                    [1.0, 0.0, 0.0],
                ),
                // Left (-X)
                build_box_face(
                    [
                        [-hw, -hh, -hd],
                        [-hw, -hh, hd],
                        [-hw, hh, hd],
                        [-hw, hh, -hd],
                    ],
                    [-1.0, 0.0, 0.0],
                ),
            ];
            merge_faces(faces)
        }
        _ => build_box(w, h, d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joinery::JointFace;

    fn make_box_mesh(w: f32, h: f32, d: f32) -> (MeshData, Dimensions3) {
        let mesh = build_box(w, h, d);
        let dims = Dimensions3::new(w as f64, h as f64, d as f64);
        (mesh, dims)
    }

    fn is_valid_mesh(mesh: &MeshData) -> bool {
        let vc = mesh.vertex_count();
        if mesh.positions.len() != mesh.normals.len() {
            return false;
        }
        for &idx in &mesh.indices {
            if idx as usize >= vc {
                return false;
            }
        }
        // Check all normals are unit length (within tolerance)
        for i in 0..vc {
            let nx = mesh.normals[i * 3];
            let ny = mesh.normals[i * 3 + 1];
            let nz = mesh.normals[i * 3 + 2];
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            if (len - 1.0).abs() > 0.01 {
                return false;
            }
        }
        true
    }

    fn bounding_box(mesh: &MeshData) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for i in 0..mesh.vertex_count() {
            for axis in 0..3 {
                let v = mesh.positions[i * 3 + axis];
                if v < min[axis] {
                    min[axis] = v;
                }
                if v > max[axis] {
                    max[axis] = v;
                }
            }
        }
        (min, max)
    }

    #[test]
    fn test_butt_no_change() {
        let (mesh, dims) = make_box_mesh(3.5, 0.75, 24.0);
        let original_vc = mesh.vertex_count();
        let result = apply_joint_geometry(&mesh, &dims, JointFace::Front, &JointParams::Butt);
        assert_eq!(result.vertex_count(), original_vc);
        assert_eq!(result.indices.len(), mesh.indices.len());
    }

    #[test]
    fn test_dado_adds_vertices() {
        let (mesh, dims) = make_box_mesh(3.5, 0.75, 24.0);
        let original_vc = mesh.vertex_count();
        let result = apply_joint_geometry(
            &mesh,
            &dims,
            JointFace::Top,
            &JointParams::Dado {
                width: 0.75,
                depth: 0.25,
            },
        );
        assert!(
            result.vertex_count() > original_vc,
            "Dado should add vertices: got {} vs original {}",
            result.vertex_count(),
            original_vc
        );
        assert!(is_valid_mesh(&result));
    }

    #[test]
    fn test_rabbet_valid_mesh() {
        let (mesh, dims) = make_box_mesh(3.5, 0.75, 24.0);
        let result = apply_joint_geometry(
            &mesh,
            &dims,
            JointFace::Front,
            &JointParams::Rabbet {
                width: 0.375,
                depth: 0.375,
            },
        );
        assert!(is_valid_mesh(&result), "Rabbet mesh should be valid");
        assert!(result.vertex_count() > 0, "Rabbet should produce vertices");
    }

    #[test]
    fn test_half_lap_dimensions() {
        let (mesh, dims) = make_box_mesh(3.5, 1.0, 24.0);
        let half_depth = 0.5;
        let result = apply_joint_geometry(
            &mesh,
            &dims,
            JointFace::Front,
            &JointParams::HalfLap { depth: half_depth },
        );
        assert!(is_valid_mesh(&result));

        let (min, max) = bounding_box(&result);
        // The height at the front should be reduced by half_depth
        // but the back should remain full height
        let total_height = max[1] - min[1];
        assert!(
            total_height <= 1.0 + 0.001,
            "Total height should not exceed original: got {}",
            total_height
        );
        // The front portion should be at step_y = 0.5 - 0.5 = 0.0
        // so the max Y at front should be 0.0, but overall max Y is still 0.5
        assert!(
            (max[1] - 0.5).abs() < 0.001,
            "Max Y should remain at 0.5, got {}",
            max[1]
        );
    }

    #[test]
    fn test_mortise_pocket() {
        let (mesh, dims) = make_box_mesh(3.5, 2.0, 6.0);
        let result = apply_joint_geometry(
            &mesh,
            &dims,
            JointFace::Front,
            &JointParams::MortiseAndTenon {
                tenon_width: 1.0,
                tenon_height: 1.0,
                tenon_depth: 2.0,
            },
        );
        assert!(is_valid_mesh(&result), "Mortise mesh should be valid");
        // Mortise adds pocket walls + splits front face into strips
        assert!(
            result.vertex_count() > mesh.vertex_count(),
            "Mortise should add vertices: got {} vs original {}",
            result.vertex_count(),
            mesh.vertex_count()
        );
    }

    #[test]
    fn test_miter_valid_mesh() {
        let (mesh, dims) = make_box_mesh(3.5, 3.5, 6.0);
        let result = apply_joint_geometry(
            &mesh,
            &dims,
            JointFace::Front,
            &JointParams::Miter { angle: 45.0 },
        );
        assert!(is_valid_mesh(&result), "Miter mesh should be valid");
    }

    #[test]
    fn test_generic_no_change() {
        let (mesh, dims) = make_box_mesh(3.5, 0.75, 24.0);
        let original_vc = mesh.vertex_count();
        let result = apply_joint_geometry(&mesh, &dims, JointFace::Front, &JointParams::Generic);
        assert_eq!(result.vertex_count(), original_vc);
    }

    #[test]
    fn test_build_box_standard() {
        let mesh = build_box(2.0, 1.0, 3.0);
        assert_eq!(mesh.vertex_count(), 24); // 4 vertices per face * 6 faces
        assert_eq!(mesh.triangle_count(), 12); // 2 triangles per face * 6 faces
        assert!(is_valid_mesh(&mesh));
    }
}
