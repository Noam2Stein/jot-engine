use wgsl_inline::wgsl;

wgsl! {
    //
    //
    // Uniforms
    //
    //

    struct Camera {
        center: vec2i,
        extents: vec2f,
    };

    @group(0) @binding(0) var<uniform> cam: Camera;

    //
    //
    // Vertex
    //
    //

    struct VertexInput {
        // Vertex Buffer
        @location(0) vertex_pos: vec2i,

        // Instance Buffer
        @location(1) center: vec2i,
        @location(2) extents: vec2i,
        @location(3) depth: f32,
        @location(4) color: vec4f,
    };

    @vertex
    fn vs_main(in: VertexInput) -> FragmentInput {
        var out: FragmentInput;

        let world_pos = in.center + in.extents * in.vertex_pos;
        let cam_pos = world_pos - cam.center;
        let pos = vec2f(cam_pos) / 256.0 / cam.extents;

        out.pos = vec4f(pos, in.depth, 1.0);
        out.color = in.color;

        return out;
    }

    //
    //
    // Fragment
    //
    //

    struct FragmentInput {
        @builtin(position) pos: vec4f,
        @location(0) color: vec4f,
    };

    @fragment
    fn fs_main(in: FragmentInput) -> @location(0) vec4f {
        return in.color;
    }
}
