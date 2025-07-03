use crate::binding::Bindeable;

pub struct Camera {
    pub direction: glam::f32::Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub perspective: glam::f32::Mat4,
    pub position: glam::f32::Vec3,
    pub view_matrix: glam::f32::Mat4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraRaw {
    pub view_matrix: [[f32; 4]; 4],
    pub position: [f32; 4],
}

impl Bindeable for CameraRaw {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(80),
                },
                count: None,
            }],
        });
    }
}

pub const CAMERA_NEAR_PLANE: f32 = 0.1;
pub const CAMERA_FAR_PLANE: f32 = 100.0;

impl Camera {
    fn direction(pitch: f32, yaw: f32) -> glam::Vec3 {
        return glam::f32::Vec3::new(
            f32::cos(yaw) * f32::cos(pitch),
            f32::sin(pitch),
            f32::sin(yaw) * f32::cos(pitch),
        )
        .normalize();
    }
    pub fn new(
        position: glam::Vec3,
        pitch: f32,
        yaw: f32,
        speed: f32,
        fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let perspective =
            glam::f32::Mat4::perspective_rh(fov, aspect_ratio, CAMERA_NEAR_PLANE, CAMERA_FAR_PLANE);
        let direction = Camera::direction(pitch, yaw);
        let look_at = glam::f32::Mat4::look_at_rh(position, position + direction, glam::Vec3::Y);
        let view_matrix = perspective * look_at;
        return Camera {
            position,
            view_matrix,
            direction,
            pitch,
            yaw,
            speed,
            perspective,
        };
    }

    pub fn update_view_matrix(&mut self) {
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
        let direction = Camera::direction(self.pitch, self.yaw);
        let look_at =
            glam::f32::Mat4::look_at_rh(self.position, self.position + direction, glam::Vec3::Y);
        self.direction = direction;
        self.view_matrix = self.perspective * look_at;
    }

    pub fn to_camera_raw(&mut self) -> CameraRaw {
        return CameraRaw {
            view_matrix: self.view_matrix.to_cols_array_2d(),
            position: [self.position.x, self.position.y, self.position.z, 0.0],
        };
    }
}
