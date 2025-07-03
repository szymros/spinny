use std::mem;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub transform: [[f32; 4]; 4],
}

impl Instance {
    pub fn from_translation_rotation_scale(
        translation: glam::f32::Vec3,
        rotation: glam::f32::Quat,
        scale: glam::f32::Vec3,
    ) -> Instance {
        let transform =
            glam::f32::Mat4::from_scale_rotation_translation(scale,rotation, translation).to_cols_array_2d();
        return Instance { transform };
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
            ],
        };
    }

    pub fn make_buffer(device: &wgpu::Device, instances: &[Instance]) -> wgpu::Buffer {
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        return instance_buffer;
    }
}

impl std::ops::Mul<Instance> for glam::f32::Quat {
    type Output = Instance;

    fn mul(self, rhs: Instance) -> Self::Output {
        let instance_matrix = glam::f32::Mat4::from_cols_array_2d(&rhs.transform);
        let transform = (glam::f32::Mat4::from_quat(self) * instance_matrix).to_cols_array_2d();
        return Instance { transform };
    }
}
