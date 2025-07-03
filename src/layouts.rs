use crate::{camera, light, model,  binding::{self, Bindeable}};

pub struct Layouts {
    pub light_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub material_bind_group_layout: wgpu::BindGroupLayout,
    pub globals_bind_group_layout: wgpu::BindGroupLayout,
    pub shadow_texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Layouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let material_bind_group_layout = model::MaterialParams::bind_group_layout(&device);
        let camera_bind_group_layout = camera::CameraRaw::bind_group_layout(&device);
        let light_bind_group_layout = light::Light::bind_group_layout(&device);
        let texture_bind_group_layout = binding::TextureBinding::bind_group_layout(&device);
        let globals_bind_group_layout = binding::Globals::bind_group_layout(&device);
        let shadow_texture_bind_group_layout = binding::TextureBinding::shadow_texture_bind_group_layout(&device);
        return Layouts {
            light_bind_group_layout,
            camera_bind_group_layout,
            texture_bind_group_layout,
            material_bind_group_layout,
            globals_bind_group_layout,
            shadow_texture_bind_group_layout
        };
    }
}
