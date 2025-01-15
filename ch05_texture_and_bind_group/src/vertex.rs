#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_uv: [f32; 2],
}

// 以下两个impl是bytemuck提供的两个特殊trait
unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

pub const VERTEX_LIST: &[Vertex] = &[
    // a
    Vertex { position: [0.0, 0.5, 0.0], tex_uv: [0.5, 1.0] },
    // b
    Vertex { position: [-0.5, 0.3, 0.0], tex_uv: [0.0, 0.7] },
    // c
    Vertex { position: [-0.5, -0.3, 0.0], tex_uv: [0.0, 0.3] },
    // d
    Vertex { position: [0.0, -0.5, 0.0], tex_uv: [0.5, 0.0] },
    // e
    Vertex { position: [0.5, -0.3, 0.0], tex_uv: [1.0, 0.3] },
    // f
    Vertex { position: [0.5, 0.3, 0.0], tex_uv: [1.0, 0.7] },
];

pub const VERTEX_INDEX_LIST: &[u16] = &[
    0, 1, 2, // abc
    0, 2, 3, // acd
    0, 3, 4, // ade
    0, 4, 5, // aef
];

pub fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                // 这里的偏移，是要偏移position的字节长度
                offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2, // <-- 对应上面tex_uv: [f32; 2]
            },
        ],
    }
}