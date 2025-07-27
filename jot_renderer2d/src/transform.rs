use std::fmt::Debug;

use super::*;

pub trait Transform2D: Debug + Copy + PartialEq {
    type Resources: Debug;

    /// Declare vertex layout local to the type, which will be given a correct offset by the `Quad` type.
    const LAYOUT: &[wgpu::VertexAttribute];

    /// Declare vertex fields without location declaration.
    const WGSL_VERTEX_FIELDS: &[&str];

    /// Declare uniforms, textures and such for the shader, at bind-group `2`.
    const WGSL_GLOBALS: &[&str];

    /// Inserted into the vertex function.
    ///
    /// Input:
    /// - `input.vertex_pos: vec2i` which is `-1` or `1`.
    /// - `input.` your vertex attributes.
    /// - `size: vec2f`.
    ///
    /// Output:
    /// - write `let world_pos: vec2f = ...;`.
    const WGSL_VERTEX_LOGIC: &str;

    fn create_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout;

    fn create_bind_group(
        resources: Self::Resources,
        layout: &wgpu::BindGroupLayout,
        gpu: &Gpu,
    ) -> wgpu::BindGroup;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos2D {
    pub pos: SVec2P,
}

impl Transform2D for Pos2D {
    type Resources = ();

    const LAYOUT: &[wgpu::VertexAttribute] = &[wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Sint32x2,
        offset: 0,
        shader_location: 0,
    }];

    const WGSL_VERTEX_FIELDS: &[&str] = &["center: vec2i"];

    const WGSL_GLOBALS: &[&str] = &[];

    const WGSL_VERTEX_LOGIC: &str = "
        let world_pos = vec2f(input.center) / 256.0 + size * vec2f(input.vertex_pos) / 2.0;
    ";

    fn create_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Renderer2D SVec2P BindGroupLayout"),
                entries: &[],
            })
    }

    fn create_bind_group(
        _resources: Self::Resources,
        layout: &wgpu::BindGroupLayout,
        gpu: &Gpu,
    ) -> wgpu::BindGroup {
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Renderer2D SVec2P BindGroup"),
            layout,
            entries: &[],
        })
    }
}
