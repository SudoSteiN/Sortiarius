use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NodeId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

/// 3D dimensions in inches (internal unit)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimensions3 {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl Dimensions3 {
    pub fn new(width: f64, height: f64, depth: f64) -> Self {
        Self { width, height, depth }
    }
}

/// 3D position in inches (internal unit)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// 3D rotation in radians (Euler angles)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rotation3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Rotation3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// Tessellated mesh data for rendering
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshData {
    /// Flat array of vertex positions [x, y, z, x, y, z, ...]
    pub positions: Vec<f32>,
    /// Flat array of vertex normals [nx, ny, nz, nx, ny, nz, ...]
    pub normals: Vec<f32>,
    /// Triangle indices (u32 for large meshes)
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}
