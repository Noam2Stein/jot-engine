use wgsl_inline::wgsl;

wgsl! {

@group(0) @binding(0)
var<uniform> aspect: f32;

struct Camera {
    center: vec2i,
    ortho_size: f32,
}

@group(0) @binding(1)
var<uniform> cam: Camera;

@group(1) @binding(0)
var texture_: texture_2d<f32>;

@group(1) @binding(1)
var sampler_: sampler;

@group(1) @binding(2)
var<uniform> pixels_per_unit: f32;

struct Vertex {
    @location(0) vertex_pos: vec2i,
    @location(1) texture_rect: vec4u,
    @location(2) center: vec2i,
    @location(3) depth: f32,
}

struct Fragment {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f,
}

@vertex
fn vs_main(input: Vertex) -> Fragment {
    var output: Fragment;

    let zero_to_one_vertex_pos = vec2u((input.vertex_pos + vec2(1)) / 2);
    let pixel_uv = input.texture_rect.xy + input.texture_rect.zw * zero_to_one_vertex_pos;
    output.uv = vec2f(pixel_uv) / vec2f(textureDimensions(texture_));

    let size = vec2f(input.texture_rect.zw) / pixels_per_unit;
    let world_pos = vec2f(input.center) / 256.0 + size * vec2f(input.vertex_pos) / 2.0;
    let cam_pos = world_pos - vec2f(cam.center) / 256.0;
    let cam_extents = vec2f(cam.ortho_size * aspect, cam.ortho_size);

    output.pos = vec4(cam_pos / cam_extents, input.depth, 1.0);
    return output;
}

@fragment
fn fs_main(input: Fragment) -> @location(0) vec4f {
    let color = textureSample(texture_, sampler_, input.uv);
    return color;
}


}
