use crate::{camera::CameraRaw, light::Light, texture};
use wgpu::{util::DeviceExt, BindingType};

pub trait Bindeable {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub struct Binding {
    pub bind_group: wgpu::BindGroup,
    pub buffers: Vec<wgpu::Buffer>,
    pub bind_index: u32,
}

impl Binding {
    pub fn create_binding(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        resources: Vec<&[u8]>,
        bind_index: u32,
    ) -> Binding {
        let mut buffers: Vec<wgpu::Buffer> = Vec::new();
        for resource in resources.iter() {
            buffers.push(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: *resource,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
            );
        }
        let bind_group_entries: Vec<wgpu::BindGroupEntry> = buffers
            .iter()
            .enumerate()
            .map(|(idx, buffer)| wgpu::BindGroupEntry {
                binding: idx as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &bind_group_entries,
        });
        return Binding {
            bind_group,
            buffers,
            bind_index,
        };
    }

    pub fn update_buffer(&mut self, queue: &wgpu::Queue, updated_resource: Vec<&[u8]>) {
        for (idx, buffer) in self.buffers.iter().enumerate() {
            queue.write_buffer(buffer, 0, updated_resource[idx]);
        }
    }
}

pub struct TextureBinding {
    pub bind_group: wgpu::BindGroup,
    pub bind_index: u32,
}

impl TextureBinding {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        resource: &texture::Texture,
        bind_index: u32,
    ) -> TextureBinding {
        return TextureBinding {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&resource.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&resource.sampler),
                    },
                ],
            }),
            bind_index,
        };
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
    }
    pub fn shadow_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });
    }
}

pub struct Globals {
    pub camera_raw: CameraRaw,
    pub light: Light,
}

impl Bindeable for Globals {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(80),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(96),
                    },
                    count: None,
                },
            ],
        });
    }
}
