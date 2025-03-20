#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub tex_cords: [f32; 2],
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
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        };
    }
}



pub const VERTICES: &[Vertex] = &[
    //top
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.3, 0.3],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.5],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.1 ],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.1],
    },
    // bot
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.8, 0.2],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.2],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    // right
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.8],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.9, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.1],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.2, 0.1],
    },
    //left
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.2, 0.1],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.2, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    //front
    Vertex {
        pos: [1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0],
        tex_cords: [0.0, 0.0],
    },
    //back
    Vertex {
        pos: [1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, 1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0],
        tex_cords: [0.0, 0.0],
    },
];
pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];
