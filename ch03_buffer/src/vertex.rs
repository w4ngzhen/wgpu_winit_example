#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// 以下两个impl是bytemuck提供的两个特殊trait
unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

pub const VERTEX_LIST: &[Vertex] = &[
    Vertex { position: [0.0, 1.0, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, 0.0, 0.0], color: [0.0, 0.0, 1.0] },
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
                shader_location: 1, // 我们把颜色信息数据指定为location = 1的地方
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    }
}