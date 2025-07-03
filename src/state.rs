use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::binding::Binding;
use crate::binding::TextureBinding;
use crate::camera::Camera;
use crate::instance::Instance;
use crate::layouts::Layouts;
use crate::light::Light;
use crate::model::create_plane;
use crate::model::MaterialParams;
use crate::model::Model;
use crate::texture::Texture;
use crate::vertex::Vertex;
use crate::vertex::CUBE_INDICES;
use crate::vertex::CUBE_VERTICES;

pub struct State {
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub render_pipeline: wgpu::RenderPipeline,
    pub light_pipeline: wgpu::RenderPipeline,
    pub shadow_pipeline: wgpu::RenderPipeline,
    pub layouts: Layouts,
    pub camera: Camera,
    pub light: Light,
    pub light_uniform: Binding,
    pub depth_texture: Texture,
    pub models: Vec<(Model, Vec<Instance>)>,
    pub material_uniform: Binding,
    pub shadow_cube_map: Texture,
    pub globals_uniform: Binding,
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();
        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/main.wgsl"));
        let light_shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/light.wgsl"));
        let shadow_shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/shadow.wgsl"));

        let models: Vec<(Model, Vec<Instance>)> = Vec::new();

        let mut camera = Camera::new(
            glam::Vec3::new(0.0, 1.0, 5.0),
            f32::to_radians(0.0),
            f32::to_radians(-90.0),
            2.5,
            f32::to_radians(60.0),
            size.width as f32 / size.height as f32,
        );
        let light = Light {
            pos: [-1.0, 6.0, 1.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
            view_proj: [[0.0; 4]; 4],
        };
        let layouts = Layouts::new(&device);
        let globals_uniform = Binding::create_binding(
            &device,
            &layouts.globals_bind_group_layout,
            vec![
                bytemuck::cast_slice(&[camera.to_camera_raw()]),
                bytemuck::cast_slice(&[light]),
            ],
            0,
        );
        let light_uniform = Binding::create_binding(
            &device,
            &layouts.light_bind_group_layout,
            vec![bytemuck::cast_slice(&[light])],
            0,
        );
        let material_uniform = Binding::create_binding(
            &device,
            &layouts.material_bind_group_layout,
            vec![bytemuck::cast_slice(&[MaterialParams::default()])],
            2,
        );

        let depth_texture = Texture::create_depth_texture(&device, size, 1);
        let shadow_cube_map = Texture::create_cube_depth_texture(
            &device,
            PhysicalSize {
                width: 1024,
                height: 1024,
            },
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &layouts.globals_bind_group_layout,
                    &layouts.texture_bind_group_layout,
                    &layouts.material_bind_group_layout,
                    &layouts.shadow_texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let light_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layouts.globals_bind_group_layout],
                push_constant_ranges: &[],
            });
        let shadow_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layouts.light_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = State::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            &shader,
            &[Vertex::desc(), Instance::desc()],
            surface_format,
        );

        let light_pipeline = State::create_render_pipeline(
            &device,
            &light_pipeline_layout,
            &light_shader,
            &[Vertex::desc()],
            surface_format,
        );

        let shadow_pipeline = State::create_shadow_pipeline(
            &device,
            &shadow_pipeline_layout,
            &shadow_shader,
            &[Vertex::desc(), Instance::desc()],
        );

        let mut state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            render_pipeline,
            camera,
            depth_texture,
            light_pipeline,
            material_uniform,
            layouts,
            shadow_pipeline,
            models,
            shadow_cube_map,
            globals_uniform,
            light,
            light_uniform,
        };
        state.configure_surface();
        state.create_scene();

        return state;
    }

    pub fn create_scene(&mut self) {
        let model = Model::load_model("./assets/model.obj", &self.device, &self.queue);
        let instance = vec![
            Instance::from_translation_rotation_scale(
                [1.0, 1.0, 1.0].into(),
                glam::f32::Quat::from_rotation_x(10.0f32.to_radians()),
                [0.33, 0.33, 0.33].into(),
            ),
            Instance::from_translation_rotation_scale(
                [4.0, 3.0, 1.0].into(),
                glam::f32::Quat::from_rotation_x(10.0f32.to_radians()),
                [0.33, 0.33, 0.33].into(),
            ),
            Instance::from_translation_rotation_scale(
                [-1.0, 3.0, -3.0].into(),
                glam::f32::Quat::from_rotation_x(10.0f32.to_radians()),
                [0.33, 0.33, 0.33].into(),
            ),
        ];
        self.models.push((model, instance));
        let plane1 = create_plane(&self.device, &self.queue, [124, 102, 92, 255]);
        self.models.push((
            plane1,
            vec![Instance::from_translation_rotation_scale(
                [0.0, -1.0, 0.0].into(),
                glam::f32::Quat::from_rotation_x(f32::to_radians(0.0)),
                [1.0, 1.0, 1.0].into(),
            )],
        ));
        let plane2 = create_plane(&self.device, &self.queue, [87, 212, 193, 255]);
        self.models.push((
            plane2,
            vec![Instance::from_translation_rotation_scale(
                [5.0, 0.0, 0.0].into(),
                glam::f32::Quat::from_rotation_z(f32::to_radians(90.0)),
                [1.0, 1.0, 1.0].into(),
            )],
        ));
        let plane3 = create_plane(&self.device, &self.queue, [87, 212, 97, 255]);
        self.models.push((
            plane3,
            vec![Instance::from_translation_rotation_scale(
                [0.0, 0.0, -5.0].into(),
                glam::f32::Quat::from_rotation_x(f32::to_radians(90.0)),
                [1.0, 1.0, 1.0].into(),
            )],
        ));
    }

    pub fn create_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        vertex_buffers: &[wgpu::VertexBufferLayout],
        surface_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        return render_pipeline;
    }

    fn create_shadow_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        vertex_buffers: &[wgpu::VertexBufferLayout],
    ) -> wgpu::RenderPipeline {
        return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &vec![],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 1.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn draw_model_instanced(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(
            self.globals_uniform.bind_index,
            &self.globals_uniform.bind_group,
            &[],
        );
        let shadow_cube_map_uniform = TextureBinding::new(
            &self.device,
            &self.layouts.shadow_texture_bind_group_layout,
            &self.shadow_cube_map,
            3,
        );
        render_pass.set_bind_group(
            shadow_cube_map_uniform.bind_index,
            &shadow_cube_map_uniform.bind_group,
            &[],
        );
        for (model, instances) in self.models.iter() {
            let instance_buffer = Instance::make_buffer(&self.device, &instances);
            for mesh in model.meshes.iter() {
                let material_id = match mesh.material_id {
                    Some(_) => 0,
                    None => 0,
                };
                let diffuse_texture_uniform = TextureBinding::new(
                    &self.device,
                    &self.layouts.texture_bind_group_layout,
                    &model.materials[material_id].diffuse_texture,
                    1,
                );
                self.material_uniform.update_buffer(
                    &self.queue,
                    vec![bytemuck::cast_slice(&[model.materials[material_id].params])],
                );
                render_pass.set_bind_group(
                    diffuse_texture_uniform.bind_index,
                    &diffuse_texture_uniform.bind_group,
                    &[],
                );
                render_pass.set_bind_group(
                    self.material_uniform.bind_index,
                    &self.material_uniform.bind_group,
                    &[],
                );
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.len_indicies as u32, 0, 0..instances.len() as u32);
            }
        }
    }

    pub fn draw_light(&mut self, render_pass: &mut wgpu::RenderPass) {
        let (vertex_buffer, index_buffer) =
            Vertex::make_buffers(&self.device, CUBE_VERTICES, CUBE_INDICES);
        render_pass.set_bind_group(
            self.globals_uniform.bind_index,
            &self.globals_uniform.bind_group,
            &[],
        );
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..CUBE_INDICES.len() as u32, 0, 0..1 as u32);
    }

    pub fn shadow_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let light_pos = glam::vec3(self.light.pos[0], self.light.pos[1], self.light.pos[2]);
        let views = [
            glam::Mat4::look_at_lh(light_pos, light_pos + glam::Vec3::X, glam::Vec3::Y), // +X
            glam::Mat4::look_at_lh(light_pos, light_pos - glam::Vec3::X, glam::Vec3::Y), // -X
            glam::Mat4::look_at_lh(light_pos, light_pos + glam::Vec3::Y, -glam::Vec3::Z), //  +Y
            glam::Mat4::look_at_lh(light_pos, light_pos - glam::Vec3::Y, glam::Vec3::Z), // -Y
            glam::Mat4::look_at_lh(light_pos, light_pos + glam::Vec3::Z, glam::Vec3::Y), // +Z
            glam::Mat4::look_at_lh(light_pos, light_pos - glam::Vec3::Z, glam::Vec3::Y), // -Z
        ];
        let perspective = glam::f32::Mat4::perspective_lh(f32::to_radians(90.0), 1.0, 0.1, 100.0);
        for i in 0..6 {
            let view_matrix = perspective * views[i];
            self.light.view_proj = view_matrix.to_cols_array_2d();
            self.light_uniform
                .update_buffer(&self.queue, vec![bytemuck::cast_slice(&[self.light])]);
            let new_light_uniform = Binding::create_binding(
                &self.device,
                &self.layouts.light_bind_group_layout,
                vec![bytemuck::cast_slice(&[self.light])],
                0,
            );
            let depth_texture_view =
                self.shadow_cube_map
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor {
                        base_array_layer: i as u32,
                        array_layer_count: Some(1),
                        label: None,
                        format: Some(wgpu::TextureFormat::Depth32Float),
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        aspect: wgpu::TextureAspect::DepthOnly,
                        usage: Some(
                            wgpu::TextureUsages::RENDER_ATTACHMENT
                                | wgpu::TextureUsages::TEXTURE_BINDING,
                        ),
                        ..Default::default()
                    });
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_viewport(0.0, 0.0, 1024.0, 1024.0, 0.0, 1.0);
            render_pass.set_pipeline(&self.shadow_pipeline);
            render_pass.set_bind_group(
                self.light_uniform.bind_index,
                &new_light_uniform.bind_group,
                &[],
            );
            for (model, instances) in self.models.iter() {
                let instance_buffer = Instance::make_buffer(&self.device, &instances);
                for mesh in model.meshes.iter() {
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(
                        0..mesh.len_indicies as u32,
                        0,
                        0..instances.len() as u32,
                    );
                }
            }
        }
    }

    pub fn spin_teapots(&mut self) {
        for instance in self.models[0].1.iter_mut() {
            *instance = glam::f32::Quat::from_rotation_y(1.0f32.to_radians()) * *instance
        }
    }

    pub fn render(&mut self) {
        let surface_texutre = self.surface.get_current_texture().unwrap();
        let texture_view = surface_texutre
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.shadow_pass(&mut encoder);
        self.globals_uniform.update_buffer(
            &self.queue,
            vec![
                bytemuck::cast_slice(&[self.camera.to_camera_raw()]),
                bytemuck::cast_slice(&[self.light]),
            ],
        );
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_bind_group(
            self.globals_uniform.bind_index,
            &self.globals_uniform.bind_group,
            &[],
        );
        render_pass.set_pipeline(&self.light_pipeline);
        self.draw_light(&mut render_pass);
        render_pass.set_pipeline(&self.render_pipeline);
        self.draw_model_instanced(&mut render_pass);
        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texutre.present();
    }
}
