use std::default;

use crate::binding::Bindeable;
use crate::texture::{self, Texture};
use crate::vertex::{Vertex, PLANE_INDICIES, PLANE_VERTICIES};

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub len_indicies: usize,
    pub material_id: Option<usize>,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn load_model(obj_file_path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Model {
        let (loaded_models, loaded_materials) = tobj::load_obj(
            obj_file_path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .unwrap();
        let mut meshes: Vec<Mesh> = Vec::new();
        for model in loaded_models.iter() {
            let vertices = (0..model.mesh.positions.len() / 3)
                .map(|i| Vertex {
                    pos: [
                        model.mesh.positions[i * 3],
                        model.mesh.positions[i * 3 + 1],
                        model.mesh.positions[i * 3 + 2],
                    ],
                    tex_cords: [
                        model.mesh.texcoords[i * 2],
                        1.0 - model.mesh.texcoords[i * 2 + 1],
                    ],
                    normals: [
                        model.mesh.normals[i * 3],
                        model.mesh.normals[i * 3 + 1],
                        model.mesh.normals[i * 3 + 2],
                    ],
                })
                .collect::<Vec<_>>();
            let (vertex_buffer, index_buffer) =
                Vertex::make_buffers(device, &vertices, &model.mesh.indices);
            meshes.push(Mesh {
                vertex_buffer,
                index_buffer,
                material_id: model.mesh.material_id,
                len_indicies: model.mesh.indices.len(),
            });
        }
        let mut materials: Vec<Material> = Vec::new();
        for material in loaded_materials.unwrap().iter() {
            let texture = match &material.diffuse_texture {
                Some(texture_file_path) => {
                    let image_rgba = image::open(&("./assets/".to_owned() + &texture_file_path))
                        .unwrap()
                        .to_rgba8();
                    Texture::load_texture(device, queue, &image_rgba)
                }
                None => Texture::create_solid_color_texture(device, queue, [125, 125, 125, 125]),
            };
            materials.push(Material::new(
                material.ambient.unwrap_or([0.5, 0.5, 0.5]),
                material.diffuse.unwrap_or([0.5, 0.5, 0.5]),
                material.specular.unwrap_or([0.5, 0.5, 0.5]),
                material.shininess.unwrap_or(32.0),
                texture,
            ));
        }

        return Model { meshes, materials };
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialParams {
    pub ambient: [f32; 3],
    pub _padding1: u32,
    pub diffuse: [f32; 3],
    pub _padding2: u32,
    pub specular: [f32; 3],
    pub shininess: f32,
}

pub struct Material {
    pub params: MaterialParams,
    pub diffuse_texture: texture::Texture,
}

impl Material {
    pub fn new(
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
        diffuse_texture: texture::Texture,
    ) -> Self {
        return Material {
            params: MaterialParams {
                ambient,
                diffuse,
                specular,
                shininess,
                _padding1: 0,
                _padding2: 0,
            },
            diffuse_texture,
        };
    }
}

impl Bindeable for MaterialParams {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(48),
                },
                count: None,
            }],
        });
    }
}

impl default::Default for MaterialParams {
    fn default() -> Self {
        return MaterialParams {
            ambient: [0.5, 0.5, 0.5],
            _padding1: 0,
            diffuse: [0.5, 0.5, 0.5],
            _padding2: 0,
            specular: [0.5, 0.5, 0.5],
            shininess: 0.5,
        };
    }
}

pub fn create_plane(device: &wgpu::Device, queue: &wgpu::Queue, color: [u8; 4]) -> Model {
    let mut verticies: Vec<Vertex> = Vec::new();
    let mut indicies: Vec<u32> = Vec::new();
    for i in -10..10 {
        for j in -10..10 {
            let moved_verticies: Vec<Vertex> = PLANE_VERTICIES
                .iter()
                .map(|vertex| *vertex + glam::f32::Vec3::new(i as f32, 0.0, j as f32))
                .collect();
            verticies.extend(moved_verticies);
            let current_indicies_len = indicies.len();
            indicies.extend(
                PLANE_INDICIES
                    .iter()
                    .map(|x| *x + current_indicies_len as u32),
            );
        }
    }
    let (vertex_buffer, index_buffer) = Vertex::make_buffers(device, &verticies, &indicies);
    let mesh = Mesh {
        vertex_buffer,
        index_buffer,
        len_indicies: indicies.len(),
        material_id: Some(0),
    };
    let texture = Texture::create_solid_color_texture(device, queue, color);
    let material = Material::new(
        [0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        32.0,
        texture,
    );
    return Model {
        meshes: vec![mesh],
        materials: vec![material],
    };
}
