use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub tex_cords: [f32; 2],
    pub normals: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
            ],
        };
    }

    pub fn make_buffers(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        return (vertex_buffer, index_buffer);
    }
}

impl std::ops::Add<glam::f32::Vec3> for Vertex {
    type Output = Self;

    fn add(self, other: glam::f32::Vec3) -> Self {
        let new_pos: glam::f32::Vec3 = glam::f32::Vec3::from_array(self.pos) + other;
        return Self{
            pos: new_pos.into(),
            tex_cords:self.tex_cords,
            normals: self.normals,
            
        }
    }
}

pub const CUBE_VERTICES: &[Vertex] = &[
    //top
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.3, 0.3],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.5],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.1],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.1],
        normals: [0.0, 1.0, 0.0],
    },
    // bot
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.8, 0.2],
        normals: [0.0, -1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.2],
        normals: [0.0, -1.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, -1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, -1.0, 0.0],
    },
    // right
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.8],
        normals: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.9, 0.0],
        normals: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.1],
        normals: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.2, 0.1],
        normals: [1.0, 0.0, 0.0],
    },
    //left
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.2, 0.1],
        normals: [-1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.0],
        normals: [-1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [-1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [-1.0, 0.0, 0.0],
    },
    //front
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, 1.0],
    },
    //back
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, -1.0],
    },
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, -1.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, -1.0],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 0.0, -1.0],
    },
];
pub const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

pub const PLANE_VERTICIES: &[Vertex] = &[
    Vertex {
        pos: [1.0, 0.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 0.0, -1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 0.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 0.0, 1.0],
        tex_cords: [0.0, 0.0],
        normals: [0.0, 1.0, 0.0],
    },
];

pub const PLANE_INDICIES: &[u32] = &[0, 1, 2, 2, 3, 0];
