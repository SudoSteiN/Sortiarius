use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::component::BoardComponent;
use crate::geometry::generate_board_mesh;
use crate::types::{Dimensions3, MeshData, NodeId, Position3, Rotation3};

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub id: NodeId,
    pub label: String,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub position: Position3,
    pub rotation: Rotation3,
    pub component: Option<BoardComponent>,
}

pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root_children: Vec<NodeId>,
    mesh_cache: HashMap<NodeId, MeshData>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_children: Vec::new(),
            mesh_cache: HashMap::new(),
        }
    }

    /// Add a node to the scene. If it has a BoardComponent with dimensions, tessellate and cache the mesh.
    pub fn add_node(
        &mut self,
        label: String,
        component: Option<BoardComponent>,
        position: Position3,
        rotation: Rotation3,
    ) -> NodeId {
        let id = NodeId::new();
        let node = SceneNode {
            id,
            label,
            parent: None,
            children: Vec::new(),
            position,
            rotation,
            component: component.clone(),
        };
        self.nodes.insert(id, node);
        self.root_children.push(id);

        // Tessellate mesh if this is a board
        if let Some(ref comp) = component {
            let mesh = generate_board_mesh(
                comp.dimensions.width,
                comp.dimensions.height,
                comp.dimensions.depth,
            );
            self.mesh_cache.insert(id, mesh);
        }

        id
    }

    /// Remove a node and all its descendants from the scene.
    pub fn remove_node(&mut self, id: NodeId) -> Option<SceneNode> {
        let node = self.nodes.remove(&id)?;

        // Remove from parent's children or root_children
        if let Some(parent_id) = node.parent {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.retain(|&c| c != id);
            }
        } else {
            self.root_children.retain(|&c| c != id);
        }

        // Recursively remove children
        let children = node.children.clone();
        for child_id in children {
            self.remove_node(child_id);
        }

        // Remove cached mesh
        self.mesh_cache.remove(&id);

        Some(node)
    }

    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id)
    }

    pub fn set_position(&mut self, id: NodeId, pos: Position3) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.position = pos;
        }
    }

    pub fn set_rotation(&mut self, id: NodeId, rot: Rotation3) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.rotation = rot;
        }
    }

    /// Reparent a node. Pass None to make it a root node.
    pub fn set_parent(&mut self, id: NodeId, new_parent: Option<NodeId>) {
        // Remove from old parent
        if let Some(node) = self.nodes.get(&id) {
            let old_parent = node.parent;
            if let Some(old_parent_id) = old_parent {
                if let Some(parent) = self.nodes.get_mut(&old_parent_id) {
                    parent.children.retain(|&c| c != id);
                }
            } else {
                self.root_children.retain(|&c| c != id);
            }
        }

        // Add to new parent
        if let Some(new_parent_id) = new_parent {
            if let Some(parent) = self.nodes.get_mut(&new_parent_id) {
                parent.children.push(id);
            }
        } else {
            self.root_children.push(id);
        }

        if let Some(node) = self.nodes.get_mut(&id) {
            node.parent = new_parent;
        }
    }

    /// Compute world-space transform by walking the parent chain.
    pub fn world_transform(&self, id: NodeId) -> (Position3, Rotation3) {
        let mut pos = Position3::zero();
        let mut rot = Rotation3::zero();

        // Collect parent chain
        let mut chain = Vec::new();
        let mut current = Some(id);
        while let Some(current_id) = current {
            if let Some(node) = self.nodes.get(&current_id) {
                chain.push(current_id);
                current = node.parent;
            } else {
                break;
            }
        }

        // Apply transforms from root to leaf
        for &node_id in chain.iter().rev() {
            if let Some(node) = self.nodes.get(&node_id) {
                // Simple additive transform (sufficient for axis-aligned boards)
                pos.x += node.position.x;
                pos.y += node.position.y;
                pos.z += node.position.z;
                rot.x += node.rotation.x;
                rot.y += node.rotation.y;
                rot.z += node.rotation.z;
            }
        }

        (pos, rot)
    }

    pub fn get_mesh(&self, id: NodeId) -> Option<&MeshData> {
        self.mesh_cache.get(&id)
    }

    pub fn update_mesh(&mut self, id: NodeId, mesh: MeshData) {
        self.mesh_cache.insert(id, mesh);
    }

    pub fn all_nodes(&self) -> impl Iterator<Item = &SceneNode> {
        self.nodes.values()
    }

    /// Get all nodes that have meshes, for rendering.
    pub fn all_nodes_with_mesh(&self) -> Vec<(NodeId, &SceneNode, &MeshData)> {
        self.nodes
            .values()
            .filter_map(|node| {
                self.mesh_cache
                    .get(&node.id)
                    .map(|mesh| (node.id, node, mesh))
            })
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Serialize the entire scene tree to a snapshot for JS consumption.
    pub fn snapshot(&self) -> SceneTreeSnapshot {
        let nodes = self
            .nodes
            .values()
            .map(|node| {
                let (dims, lumber_type, grain_dir) = if let Some(ref comp) = node.component {
                    (
                        Some([comp.dimensions.width, comp.dimensions.height, comp.dimensions.depth]),
                        match &comp.lumber_type {
                            crate::component::LumberType::Standard(ls) => {
                                Some(ls.nominal_label.clone())
                            }
                            crate::component::LumberType::Custom => Some("Custom".to_string()),
                        },
                        Some(format!("{:?}", comp.grain_direction)),
                    )
                } else {
                    (None, None, None)
                };

                SceneNodeSnapshot {
                    id: node.id.to_string(),
                    label: node.label.clone(),
                    parent_id: node.parent.map(|p| p.to_string()),
                    children: node.children.iter().map(|c| c.to_string()).collect(),
                    position: [node.position.x, node.position.y, node.position.z],
                    rotation: [node.rotation.x, node.rotation.y, node.rotation.z],
                    has_mesh: self.mesh_cache.contains_key(&node.id),
                    dimensions: dims,
                    lumber_type,
                    grain_direction: grain_dir,
                }
            })
            .collect();

        SceneTreeSnapshot { nodes }
    }

    /// Get all boards as (position, dimensions) for snap calculations.
    pub fn boards_for_snap(&self, exclude: Option<NodeId>) -> Vec<(Position3, Dimensions3)> {
        self.nodes
            .values()
            .filter(|n| {
                n.component.is_some() && exclude.map_or(true, |ex| n.id != ex)
            })
            .map(|n| {
                let (world_pos, _) = self.world_transform(n.id);
                let dims = n.component.as_ref().unwrap().dimensions;
                (world_pos, dims)
            })
            .collect()
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneTreeSnapshot {
    pub nodes: Vec<SceneNodeSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneNodeSnapshot {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub has_mesh: bool,
    pub dimensions: Option<[f64; 3]>,
    pub lumber_type: Option<String>,
    pub grain_direction: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{BoardComponent, GrainDirection, LumberType};
    use crate::lumber::LumberCatalog;

    fn make_board(label: &str, w: f64, h: f64, d: f64) -> BoardComponent {
        BoardComponent {
            node_id: NodeId::new(),
            label: label.to_string(),
            dimensions: Dimensions3::new(w, h, d),
            lumber_type: LumberType::Custom,
            grain_direction: GrainDirection::AlongLength,
        }
    }

    #[test]
    fn test_add_remove() {
        let mut scene = SceneGraph::new();
        let board = make_board("Test Board", 1.5, 3.5, 96.0);
        let id = scene.add_node("Test Board".into(), Some(board), Position3::zero(), Rotation3::zero());

        assert_eq!(scene.node_count(), 1);
        assert!(scene.get_mesh(id).is_some());

        scene.remove_node(id);
        assert_eq!(scene.node_count(), 0);
        assert!(scene.get_mesh(id).is_none());
    }

    #[test]
    fn test_world_transform() {
        let mut scene = SceneGraph::new();
        let parent_id = scene.add_node("Parent".into(), None, Position3::new(10.0, 0.0, 0.0), Rotation3::zero());
        let child_id = scene.add_node("Child".into(), None, Position3::new(5.0, 0.0, 0.0), Rotation3::zero());

        scene.set_parent(child_id, Some(parent_id));

        let (world_pos, _) = scene.world_transform(child_id);
        assert!((world_pos.x - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_snapshot() {
        let mut scene = SceneGraph::new();
        let board = make_board("2x4", 1.5, 3.5, 96.0);
        scene.add_node("2x4".into(), Some(board), Position3::zero(), Rotation3::zero());

        let snap = scene.snapshot();
        assert_eq!(snap.nodes.len(), 1);
        assert_eq!(snap.nodes[0].label, "2x4");
        assert!(snap.nodes[0].has_mesh);
        assert!(snap.nodes[0].dimensions.is_some());
    }

    #[test]
    fn test_standard_board() {
        let catalog = LumberCatalog::new();
        let lumber = catalog.find_by_label("2x4").unwrap();
        let id = NodeId::new();
        let board = BoardComponent::new_standard(id, lumber, 96.0);

        let mut scene = SceneGraph::new();
        let node_id = scene.add_node("2x4".into(), Some(board), Position3::zero(), Rotation3::zero());

        let mesh = scene.get_mesh(node_id).unwrap();
        assert_eq!(mesh.vertex_count(), 24);
        assert_eq!(mesh.triangle_count(), 12);
    }
}
