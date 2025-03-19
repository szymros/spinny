pub struct Camera {
   pub position: glam::Vec3,
   pub direction: glam::Vec3,
   pub pitch: f32,
   pub yaw: f32,
   pub speed: f32,
   pub perspective: glam::f32::Mat4,
}

impl Camera {
    pub fn new(
        position: glam::Vec3,
        target: glam::Vec3,
        pitch: f32,
        yaw: f32,
        speed: f32,
        fov: f32,
        z_near: f32,
        z_far: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let perspective = glam::f32::Mat4::perspective_rh(fov, aspect_ratio, z_near, z_far);
        return Camera {
            position,
            direction: target,
            pitch,
            yaw,
            speed,
            perspective,
        };
    }
    pub fn get_view_matrix(&mut self) -> glam::f32::Mat4 {
        if (self.pitch > 89.0) {
            self.pitch = 89.0;
        }
        if (self.pitch < -89.0) {
            self.pitch = -89.0;
        }
        self.direction.x = f32::cos(self.yaw) * f32::cos(self.pitch);
        self.direction.y = f32::sin(self.pitch);
        self.direction.z = f32::sin(self.yaw) * f32::cos(self.pitch);
        self.direction = self.direction.normalize();
        let look_at = glam::f32::Mat4::look_at_rh(
            self.position,
            self.position + self.direction,  
            glam::Vec3::Y,
        );
        return self.perspective * look_at;
    }
}
