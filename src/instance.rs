use std::mem;
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub transform: [[f32; 4]; 4],
}

impl Instance {
    pub fn from_position_rotation(position: glam::f32::Vec3, rotation: f32) -> Instance {
        return Instance {
            transform: (glam::f32::Mat4::from_translation(position)
                * glam::f32::Mat4::from_quat(glam::f32::Quat::from_rotation_y(rotation)))
            .to_cols_array_2d(),
        };
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
            ],
        };
    }

    pub fn create_instances() -> [Instance; 4] {
        return [
            Instance::from_position_rotation(glam::f32::vec3(1.0, 1.0, 1.0), f32::to_radians(40.0)),
            Instance::from_position_rotation(glam::f32::vec3(3.0, 0.0, 2.0), f32::to_radians(80.0)),
            Instance::from_position_rotation(
                glam::f32::vec3(1.0, 0.0, 4.0),
                f32::to_radians(120.0),
            ),
            Instance::from_position_rotation(glam::f32::vec3(0.0, 2.0, 4.0), f32::to_radians(10.0)),
        ];
    }
}
