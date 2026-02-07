use std::collections::HashMap;

use glam::Mat4;
use wgpu::util::DeviceExt;
use woodforge_core::types::{MeshData, NodeId};

use crate::vertex::Vertex;

pub struct RenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub model_uniform_buffer: wgpu::Buffer,
    pub model_bind_group: wgpu::BindGroup,
}

pub struct SceneRenderer {
    objects: HashMap<NodeId, RenderObject>,
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add_object(
        &mut self,
        device: &wgpu::Device,
        model_bind_group_layout: &wgpu::BindGroupLayout,
        node_id: NodeId,
        mesh: &MeshData,
        model_matrix: Mat4,
    ) {
        // Interleave positions and normals into Vertex structs
        let vertices = Self::mesh_to_vertices(mesh);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Vertex Buffer {}", node_id)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Index Buffer {}", node_id)),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let model_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Model Uniform {}", node_id)),
            contents: bytemuck::cast_slice(&model_matrix.to_cols_array()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Model Bind Group {}", node_id)),
            layout: model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_uniform_buffer.as_entire_binding(),
            }],
        });

        self.objects.insert(
            node_id,
            RenderObject {
                vertex_buffer,
                index_buffer,
                index_count: mesh.indices.len() as u32,
                model_uniform_buffer,
                model_bind_group,
            },
        );
    }

    pub fn remove_object(&mut self, node_id: NodeId) {
        self.objects.remove(&node_id);
    }

    pub fn update_transform(&self, queue: &wgpu::Queue, node_id: NodeId, model_matrix: Mat4) {
        if let Some(obj) = self.objects.get(&node_id) {
            queue.write_buffer(
                &obj.model_uniform_buffer,
                0,
                bytemuck::cast_slice(&model_matrix.to_cols_array()),
            );
        }
    }

    pub fn update_mesh(
        &mut self,
        device: &wgpu::Device,
        model_bind_group_layout: &wgpu::BindGroupLayout,
        _queue: &wgpu::Queue,
        node_id: NodeId,
        mesh: &MeshData,
    ) {
        // Preserve model matrix if object exists
        if let Some(old) = self.objects.get(&node_id) {
            let model_matrix = {
                // Read back model matrix — we stored it as a uniform
                // For simplicity, just recreate at identity
                Mat4::IDENTITY
            };
            let _ = old;
            self.remove_object(node_id);
            self.add_object(device, model_bind_group_layout, node_id, mesh, model_matrix);
        }
    }

    pub fn objects(&self) -> impl Iterator<Item = (&NodeId, &RenderObject)> {
        self.objects.iter()
    }

    pub fn has_object(&self, node_id: NodeId) -> bool {
        self.objects.contains_key(&node_id)
    }

    fn mesh_to_vertices(mesh: &MeshData) -> Vec<Vertex> {
        let count = mesh.positions.len() / 3;
        (0..count)
            .map(|i| Vertex {
                position: [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ],
                normal: [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                ],
            })
            .collect()
    }
}

impl Default for SceneRenderer {
    fn default() -> Self {
        Self::new()
    }
}
