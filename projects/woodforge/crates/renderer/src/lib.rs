mod vertex;
mod camera;
mod scene_renderer;

pub use vertex::Vertex;
pub use camera::{Camera, CameraController};
pub use scene_renderer::{RenderObject, SceneRenderer, ObjectUniformData};

use glam::Mat4;
use wgpu::util::DeviceExt;
use woodforge_core::types::{MeshData, NodeId};

const SHADER_SRC: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ObjectUniform {
    model: mat4x4<f32>,
    color: vec4<f32>,  // rgb = base color, a = selection flag (1=selected)
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> object: ObjectUniform;

@group(2) @binding(0)
var grain_texture: texture_2d<f32>;
@group(2) @binding(1)
var grain_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = object.model * vec4<f32>(in.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    out.world_normal = normalize((object.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.world_position = world_pos.xyz;
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.3, 1.0, 0.5));
    let normal = normalize(in.world_normal);

    // Lighting
    let ambient = 0.15;
    let diffuse = max(dot(normal, light_dir), 0.0) * 0.7;
    let rim = max(dot(normal, -light_dir), 0.0) * 0.15;
    let brightness = ambient + diffuse + rim;

    // Sample grain texture (1x1 white for untextured objects)
    let grain = textureSample(grain_texture, grain_sampler, in.uv * 0.1);

    // Base color from object uniform, modulated by grain texture
    let base_color = object.color.rgb * grain.rgb;
    var final_color = base_color * brightness;

    // Selection highlight: brighten + add blue tint
    if (object.color.a > 0.5) {
        final_color = final_color * 1.3 + vec3<f32>(0.1, 0.15, 0.35);
    }

    return vec4<f32>(final_color, 1.0);
}
"#;

/// Camera uniform data (view-projection matrix)
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniformData {
    view_proj: [f32; 16],
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    camera: Camera,
    camera_controller: CameraController,
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    model_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    scene_renderer: SceneRenderer,
}

impl Renderer {
    pub async fn new(
        instance: wgpu::Instance,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WoodForge Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("WoodForge Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SRC.into()),
        });

        // Camera uniform (group 0)
        let camera = Camera::new(width as f32 / height as f32);
        let camera_data = CameraUniformData {
            view_proj: camera.view_projection().to_cols_array(),
        };
        let camera_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera_data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        // Object uniform (group 1) — model matrix + color
        let model_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Object Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Texture bind group (group 2) — grain texture + sampler
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &model_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let depth_texture_view = Self::create_depth_texture(&device, width, height);
        let camera_controller = CameraController::new();

        // Initialize scene renderer with default texture
        let mut scene_renderer = SceneRenderer::new();
        scene_renderer.init_default_texture(&device, &queue, &texture_bind_group_layout);

        Self {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            depth_texture_view,
            camera,
            camera_controller,
            camera_uniform_buffer,
            camera_bind_group,
            model_bind_group_layout,
            texture_bind_group_layout,
            scene_renderer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture_view = Self::create_depth_texture(&self.device, width, height);
            self.camera.aspect = width as f32 / height as f32;
        }
    }

    pub fn render(&mut self) {
        let camera_data = CameraUniformData {
            view_proj: self.camera.view_projection().to_cols_array(),
        };
        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_data]),
        );

        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.102,
                            g: 0.102,
                            b: 0.180,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            for (_node_id, obj) in self.scene_renderer.objects() {
                render_pass.set_bind_group(1, &obj.model_bind_group, &[]);
                render_pass.set_bind_group(2, &obj.texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, obj.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    obj.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..obj.index_count, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn camera_orbit(&mut self, dx: f32, dy: f32) {
        self.camera_controller.orbit(&mut self.camera, dx, dy);
    }

    pub fn camera_pan(&mut self, dx: f32, dy: f32) {
        self.camera_controller.pan(&mut self.camera, dx, dy);
    }

    pub fn camera_zoom(&mut self, delta: f32) {
        self.camera_controller.zoom(&mut self.camera, delta);
    }

    /// Add a render object with per-object color and optional grain texture
    pub fn add_object(
        &mut self,
        node_id: NodeId,
        mesh: &MeshData,
        model_matrix: Mat4,
        color: [f32; 3],
        grain_texture: Option<(&[u8], u32, u32)>,
    ) {
        self.scene_renderer.add_object(
            &self.device,
            &self.queue,
            &self.model_bind_group_layout,
            &self.texture_bind_group_layout,
            node_id,
            mesh,
            model_matrix,
            color,
            grain_texture,
        );
    }

    pub fn remove_object(&mut self, node_id: NodeId) {
        self.scene_renderer.remove_object(node_id);
    }

    pub fn update_transform(&self, node_id: NodeId, model_matrix: Mat4) {
        self.scene_renderer
            .update_transform(&self.queue, node_id, model_matrix);
    }

    pub fn update_color(&self, node_id: NodeId, color: [f32; 3], selected: bool) {
        self.scene_renderer
            .update_color(&self.queue, node_id, color, selected);
    }

    pub fn set_selected(&self, node_id: NodeId, selected: bool) {
        self.scene_renderer
            .set_selected(&self.queue, node_id, selected);
    }

    /// Get camera eye position and forward direction for ray casting
    pub fn camera_ray(&self, ndc_x: f32, ndc_y: f32) -> (glam::Vec3, glam::Vec3) {
        let inv_proj = self.camera.projection_matrix().inverse();
        let inv_view = self.camera.view_matrix().inverse();

        let clip_near = glam::Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
        let clip_far = glam::Vec4::new(ndc_x, ndc_y, 1.0, 1.0);

        let view_near = inv_proj * clip_near;
        let view_far = inv_proj * clip_far;

        let world_near = inv_view * (view_near / view_near.w);
        let world_far = inv_view * (view_far / view_far.w);

        let origin = world_near.truncate();
        let direction = (world_far.truncate() - origin).normalize();

        (origin, direction)
    }

    pub fn viewport_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    /// Update the grain texture for an existing object
    pub fn update_grain_texture(&mut self, node_id: NodeId, pixels: &[u8], width: u32, height: u32) {
        self.scene_renderer.update_texture(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            node_id,
            pixels,
            width,
            height,
        );
    }

    /// Remove all objects from the scene renderer
    pub fn clear_all_objects(&mut self) {
        self.scene_renderer.clear_all();
    }

    fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
