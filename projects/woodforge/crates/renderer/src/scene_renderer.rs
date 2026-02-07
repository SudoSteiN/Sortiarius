use std::collections::HashMap;

use glam::Mat4;
use wgpu::util::DeviceExt;
use woodforge_core::types::{MeshData, NodeId};

use crate::vertex::Vertex;

/// Per-object uniform data: model matrix + color + selection state
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectUniformData {
    pub model: [f32; 16],
    pub color: [f32; 4], // RGB + selection flag (w=1 if selected)
}

pub struct RenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub model_uniform_buffer: wgpu::Buffer,
    pub model_bind_group: wgpu::BindGroup,
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct SceneRenderer {
    objects: HashMap<NodeId, RenderObject>,
    default_texture_bind_group: Option<wgpu::BindGroup>,
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            default_texture_bind_group: None,
        }
    }

    /// Create a 1x1 white fallback texture for objects without grain
    pub fn init_default_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Default White Texture"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &[255, 255, 255, 255],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        self.default_texture_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default Texture Bind Group"),
            layout: texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        }));
    }

    pub fn add_object(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        node_id: NodeId,
        mesh: &MeshData,
        model_matrix: Mat4,
        color: [f32; 3],
        grain_texture: Option<(&[u8], u32, u32)>, // (pixels, width, height)
    ) {
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

        let uniform_data = ObjectUniformData {
            model: model_matrix.to_cols_array(),
            color: [color[0], color[1], color[2], 0.0],
        };

        let model_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Object Uniform {}", node_id)),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Object Bind Group {}", node_id)),
            layout: model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_uniform_buffer.as_entire_binding(),
            }],
        });

        // Texture bind group: use grain texture or default white
        let texture_bind_group = if let Some((pixels, tw, th)) = grain_texture {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("Grain Texture {}", node_id)),
                size: wgpu::Extent3d { width: tw, height: th, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex, mip_level: 0,
                    origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All,
                },
                pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0, bytes_per_row: Some(4 * tw), rows_per_image: Some(th),
                },
                wgpu::Extent3d { width: tw, height: th, depth_or_array_layers: 1 },
            );
            let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&format!("Grain Bind Group {}", node_id)),
                layout: texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                ],
            })
        } else {
            // Use default white texture
            self.default_texture_bind_group.as_ref()
                .expect("init_default_texture must be called before add_object")
                .clone()
        };

        self.objects.insert(node_id, RenderObject {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
            model_uniform_buffer,
            model_bind_group,
            texture_bind_group,
        });
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

    pub fn update_color(&self, queue: &wgpu::Queue, node_id: NodeId, color: [f32; 3], selected: bool) {
        if let Some(obj) = self.objects.get(&node_id) {
            let color_data = [color[0], color[1], color[2], if selected { 1.0_f32 } else { 0.0 }];
            queue.write_buffer(
                &obj.model_uniform_buffer,
                64,
                bytemuck::cast_slice(&color_data),
            );
        }
    }

    pub fn set_selected(&self, queue: &wgpu::Queue, node_id: NodeId, selected: bool) {
        if let Some(obj) = self.objects.get(&node_id) {
            let flag: f32 = if selected { 1.0 } else { 0.0 };
            queue.write_buffer(
                &obj.model_uniform_buffer,
                76,
                bytemuck::cast_slice(&[flag]),
            );
        }
    }

    pub fn objects(&self) -> impl Iterator<Item = (&NodeId, &RenderObject)> {
        self.objects.iter()
    }

    pub fn has_object(&self, node_id: NodeId) -> bool {
        self.objects.contains_key(&node_id)
    }

    pub fn clear_all(&mut self) {
        self.objects.clear();
    }

    pub fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        node_id: NodeId,
        pixels: &[u8],
        tw: u32,
        th: u32,
    ) {
        if let Some(obj) = self.objects.get_mut(&node_id) {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("Updated Grain Texture {}", node_id)),
                size: wgpu::Extent3d { width: tw, height: th, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex, mip_level: 0,
                    origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All,
                },
                pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0, bytes_per_row: Some(4 * tw), rows_per_image: Some(th),
                },
                wgpu::Extent3d { width: tw, height: th, depth_or_array_layers: 1 },
            );
            let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
            obj.texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&format!("Updated Grain Bind Group {}", node_id)),
                layout: texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                ],
            });
        }
    }

    fn mesh_to_vertices(mesh: &MeshData) -> Vec<Vertex> {
        let count = mesh.positions.len() / 3;
        let has_uvs = mesh.uvs.len() >= count * 2;
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
                uv: if has_uvs {
                    [mesh.uvs[i * 2], mesh.uvs[i * 2 + 1]]
                } else {
                    [0.0, 0.0]
                },
            })
            .collect()
    }
}

impl Default for SceneRenderer {
    fn default() -> Self {
        Self::new()
    }
}
