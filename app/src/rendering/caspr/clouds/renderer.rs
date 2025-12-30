use eframe::{egui, wgpu};
use wgpu::util::DeviceExt;

use crate::rendering::caspr;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
struct Uniforms {
    mvp_matrix: [[f32; 4]; 4],
    colour: [f32; 4],
}

impl Uniforms {
    pub fn get_uniform_buffer(self, label: Option<&str>, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&[self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn from_camera_and_settings(camera_data: &caspr::camera::Camera, _clouds_settings: &caspr::clouds::CloudSettings) -> Self {
        let rotation_3: &nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<3>, nalgebra::ArrayStorage<f32, 3, 3>> = camera_data.rotation.matrix();
        #[rustfmt::skip]
        let rotation_4 = nalgebra::Matrix4::new(
            rotation_3[(0, 0)], rotation_3[(0, 1)], rotation_3[(0, 2)], 0.0,
            rotation_3[(1, 0)], rotation_3[(1, 1)], rotation_3[(1, 2)], 0.0,
            rotation_3[(2, 0)], rotation_3[(2, 1)], rotation_3[(2, 2)], 0.0,
                           0.0,                0.0,                0.0, 1.0,
        );
        let mvp_matrix = camera_data.projection.get_projection_matrix(camera_data.fov, camera_data.viewport_rect) * rotation_4;

        let width = camera_data.viewport_rect.width();
        let height = camera_data.viewport_rect.height();
        // Matrix Logic:
        // x_ndc = (x_pixel / width) * 2
        // y_ndc = (y_pixel / height) * -2
        // y = 0 is at the top (https://gpuweb.github.io/gpuweb/#coordinate-systems), so have to flip it (since egui takes y=0 at the bottom)
        #[rustfmt::skip]
        let screen_to_ndc = nalgebra::Matrix4::new(
            2.0 / width,  0.0,           0.0,  0.0,
            0.0,         -2.0 / height,  0.0,  0.0,
            0.0,          0.0,           1.0,  0.0,
            0.0,          0.0,           0.0,  1.0,
        );

        let position_to_ndc = screen_to_ndc * mvp_matrix;

        let ndc_to_position = position_to_ndc.try_inverse().unwrap_or_else(|| {
            log::error!("Failed to invert the position_to_ndc matrix in clouds renderer, matrix is {:?}", position_to_ndc);
            nalgebra::Matrix4::identity()
        });
        let inv_matrix_columns: [[f32; 4]; 4] = [
            ndc_to_position.column(0).into(),
            ndc_to_position.column(1).into(),
            ndc_to_position.column(2).into(),
            ndc_to_position.column(3).into(),
        ];

        Self {
            mvp_matrix: inv_matrix_columns,
            colour: [0.3, 0.3, 0.3, 1.0],
        }
    }
}

pub struct CloudsRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,

    sampler: wgpu::Sampler,

    uniforms: Uniforms,
}

impl CloudsRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, target_format: wgpu::TextureFormat, camera_data: &caspr::camera::Camera, clouds_settings: &caspr::clouds::CloudSettings) -> Self {
        let uniforms = Uniforms::from_camera_and_settings(camera_data, clouds_settings);
        let shader = device.create_shader_module(wgpu::include_wgsl!("./clouds.wgsl"));

        let uniform_buffer = uniforms.get_uniform_buffer(Some("Clouds uniform buffer"), device);

        let texture_size = 256;
        let texture_data = Self::generate_default_filler_texture_data(texture_size);
        let texture = Self::create_cubemap_texture(device, texture_size);
        Self::write_cubemap_texture(queue, &texture, &texture_data, texture_size);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Clouds bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // Uniforms
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // Texture
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // Sampler
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = Self::create_bind_group(&bind_group_layout, &device, &texture_view, &uniform_buffer, &sampler);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Clouds pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Clouds pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            depth_stencil: None, // Basically does not check the Z buffer
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            bind_group,
            uniform_buffer,

            sampler,

            uniforms,
        }
    }

    fn create_bind_group(
        bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        texture_view: &wgpu::TextureView,
        uniform_buffer: &wgpu::Buffer,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Clouds bind group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        })
    }

    fn generate_default_filler_texture_data(size: u32) -> Vec<Vec<u8>> {
        vec![vec![0; (size * size) as usize]; 6]
    }

    pub fn generate_texture_data(size: u32) -> Vec<Vec<u8>> {
        // Generate colors for 6 faces
        let colors = [[255], [150], [120], [80], [50], [20]];

        let texture_data = colors
            .iter()
            .map(|colour| {
                let mut data = Vec::with_capacity((size * size) as usize);
                for _ in 0..(size * size) {
                    data.extend_from_slice(colour);
                }
                data
            })
            .collect();
        texture_data
    }

    fn create_cubemap_texture(device: &wgpu::Device, size: u32) -> wgpu::Texture {
        let texture_size = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        };

        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Clouds cubemap"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }

    fn update_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, texture_data: &[Vec<u8>], size: u32) {
        let texture = Self::create_cubemap_texture(device, size);
        Self::write_cubemap_texture(queue, &texture, texture_data, size);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let bind_group = Self::create_bind_group(&self.bind_group_layout, device, &texture_view, &self.uniform_buffer, &self.sampler);
        self.bind_group = bind_group;
    }

    fn write_cubemap_texture(queue: &wgpu::Queue, texture: &wgpu::Texture, texture_data: &[Vec<u8>], size: u32) {
        texture_data.iter().enumerate().for_each(|(layer, data)| {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: layer as u32 },
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(size),
                    rows_per_image: Some(size),
                },
                wgpu::Extent3d {
                    width: size,
                    height: size,
                    depth_or_array_layers: 1,
                },
            );
        });
    }

    pub fn update_uniform_buffer(&mut self, queue: &wgpu::Queue, camera_data: &caspr::camera::Camera, clouds_settings: &caspr::clouds::CloudSettings) {
        let uniforms = Uniforms::from_camera_and_settings(camera_data, clouds_settings);
        self.uniforms = uniforms;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }
}

pub struct CloudsCallback {
    pub camera_data: caspr::camera::Camera,
    pub clouds_settings: caspr::clouds::CloudSettings,
    pub clouds_texture_to_upload: Option<caspr::textures::cubemap::Cubemap<u8>>,
    pub target_format: wgpu::TextureFormat,

    pub render: bool,
}

impl eframe::egui_wgpu::CallbackTrait for CloudsCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut eframe::egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources
            .entry::<CloudsRenderer>()
            .or_insert_with(|| CloudsRenderer::new(device, queue, self.target_format, &self.camera_data, &self.clouds_settings));

        if self.camera_data.changed {
            renderer.update_uniform_buffer(queue, &self.camera_data, &self.clouds_settings);
        }
        if let Some(texture_info) = &self.clouds_texture_to_upload {
            if texture_info.changed {
                renderer.update_texture(device, queue, &texture_info.texture_data, texture_info.texture_size);
            }
        }

        Vec::new()
    }

    fn paint(&self, info: egui::PaintCallbackInfo, render_pass: &mut wgpu::RenderPass<'static>, resources: &eframe::egui_wgpu::CallbackResources) {
        if !self.render {
            return;
        }
        let renderer: &CloudsRenderer = resources.get().unwrap();

        let viewport = info.clip_rect_in_pixels();
        render_pass.set_viewport(viewport.left_px as f32, viewport.top_px as f32, viewport.width_px as f32, viewport.height_px as f32, 0.0, 1.0);

        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
