use std::{fmt::Debug, mem::offset_of};

use super::*;

pub trait Visual2D: Debug + Copy + PartialEq {
    type Bindings: GpuBindings;

    /// Declare vertex layout local to the type, which will be given a correct offset by the `Quad` type.
    const LAYOUT: &[wgpu::VertexAttribute];

    /// Declare vertex fields without location declaration.
    const WGSL_VERTEX_FIELDS: &[&str];

    /// Declare fragmnt fields without location declaration.
    /// `pos` is automatically included.
    const WGSL_FRAGMENT_FIELDS: &[&str];

    /// Declare uniforms, textures and such for the shader, at bind-group `2`.
    const WGSL_GLOBALS: &[&str];

    /// Inserted into the vertex function.
    ///
    /// Input (`input: Vertex`):
    /// - `vertex_pos: vec2i` which is `-1` or `1`.
    /// - your vertex attributes.
    ///
    /// Output:
    /// - write `let size: vec2f = ...;`.
    /// - your fragment fields.
    const WGSL_VERTEX_LOGIC: &str;

    /// The body of the fragment function (`input: Fragment`).
    const WGSL_FRAGMENT_LOGIC: &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sprite {
    pub texture_rect: URect2P,
}

#[derive(Debug, Clone, GpuBindings_Local)]
pub struct SpriteBindings {
    pub texture: GpuTexture<2>,
    pub pixels_per_unit: GpuBuffer<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColoredSprite {
    pub texture_rect: URect2P,
    pub color: FVec4P,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colored {
    pub size: FVec2P,
    pub color: FVec4P,
}

impl Visual2D for Sprite {
    type Bindings = SpriteBindings;

    const LAYOUT: &[wgpu::VertexAttribute] = &[wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Uint32x4,
        offset: offset_of!(Self, texture_rect) as u64,
        shader_location: 0,
    }];

    const WGSL_VERTEX_FIELDS: &[&str] = &["texture_rect: vec4u"];

    const WGSL_FRAGMENT_FIELDS: &[&str] = &["uv: vec2f"];

    const WGSL_GLOBALS: &[&str] = &[
        "@group(2) @binding(0) var texture_: texture_2d<f32>;",
        "@group(2) @binding(1) var sampler_: sampler;",
        "@group(2) @binding(2) var<uniform> pixels_per_unit: f32;",
    ];

    const WGSL_VERTEX_LOGIC: &str = "
        let zero_to_one_vertex_pos = vec2u((input.vertex_pos + vec2(1))) / 2;
        let pixel_uv = input.texture_rect.xy + input.texture_rect.zw * zero_to_one_vertex_pos;
        output.uv = vec2f(pixel_uv) / vec2f(textureDimensions(texture_));
        output.uv.y = 1.0 - output.uv.y;

        let size = vec2f(input.texture_rect.zw) / pixels_per_unit;
    ";

    const WGSL_FRAGMENT_LOGIC: &str = "
        let color = textureSample(texture_, sampler_, input.uv);
        return color;
    ";
}

impl Visual2D for ColoredSprite {
    type Bindings = <Sprite as Visual2D>::Bindings;

    const LAYOUT: &[wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Uint32x4,
            offset: offset_of!(Self, texture_rect) as u64,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: offset_of!(Self, color) as u64,
            shader_location: 1,
        },
    ];

    const WGSL_VERTEX_FIELDS: &[&str] = &["texture_rect: vec4u", "color: vec4f"];

    const WGSL_FRAGMENT_FIELDS: &[&str] = &["uv: vec2f", "color: vec4f"];

    const WGSL_GLOBALS: &[&str] = Sprite::WGSL_GLOBALS;

    const WGSL_VERTEX_LOGIC: &str = "
        let zero_to_one_vertex_pos = (input.vertex_pos + vec2(1)) / 2;
        let pixel_uv = vec2f(input.texture_rect.xy) + vec2f(input.texture_rect.zw) * zero_to_one_vertex_pos;
        output.uv = vec2f(pixel_uv) / vec2f(textureDimensions(texture_));
        output.uv.y = 1.0 - output.uv.y;

        output.color = input.color;

        let size = vec2f(input.texture_rect.zw) / pixels_per_unit;
    ";

    const WGSL_FRAGMENT_LOGIC: &str = "
        let color = textureSample(texture_, sampler_, input.uv) * input.color;
        return color;
    ";
}

impl Visual2D for Colored {
    type Bindings = ();

    const LAYOUT: &[wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: offset_of!(Self, size) as u64,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: offset_of!(Self, color) as u64,
            shader_location: 1,
        },
    ];

    const WGSL_VERTEX_FIELDS: &[&str] = &["size: vec2f", "color: vec4f"];

    const WGSL_FRAGMENT_FIELDS: &[&str] = &["color: vec4f"];

    const WGSL_GLOBALS: &[&str] = &[];

    const WGSL_VERTEX_LOGIC: &str = "
        output.color = input.color;

        let size = input.size;
    ";

    const WGSL_FRAGMENT_LOGIC: &str = "
        return input.color;
    ";
}
