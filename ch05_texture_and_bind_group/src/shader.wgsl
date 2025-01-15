struct VertexInput {
    @location(0) position: vec3f,
    @location(1) tex_uv: vec2f,
};

struct VertexOutput {
   @builtin(position) pos: vec4<f32>,
   @location(0) tex_uv: vec2f,
}

struct FragmentInput {
   @builtin(position) pos: vec4<f32>,
   @location(0) tex_uv: vec2f,
}

@vertex
fn vs_main(vertex_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(vertex_in.position, 1.0);
    out.tex_uv = vertex_in.tex_uv;
    return out;
}

@group(0) @binding(0)
var the_texture: texture_2d<f32>;
@group(0) @binding(1)
var the_sampler: sampler;

@fragment
fn fs_main(fragment_in: FragmentInput) -> @location(0) vec4<f32> {
   return textureSample(the_texture, the_sampler, fragment_in.tex_uv);
}