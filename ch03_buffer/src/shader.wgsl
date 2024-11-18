struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
   @builtin(position) pos: vec4<f32>,
   @location(0) color: vec3f,
}

struct FragmentInput {
   @builtin(position) pos: vec4<f32>,
   @location(0) color: vec3f,
}

@vertex
fn vs_main(vertex_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(vertex_in.position, 1.0);
    out.color = vertex_in.color;
    return out;
}

// 方式1
@fragment
fn fs_main(fragment_in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(fragment_in.color, 1.0);
}

// 方式2:甚至直接访问对应location的数据
//@fragment
//fn fs_main(@location(0) color: vec3f) -> @location(0) vec4<f32> {
//    return vec4<f32>(color, 1.0);
//}