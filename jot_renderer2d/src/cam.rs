use std::fmt::Debug;

use crevice::std140::AsStd140;

use super::*;

pub trait Camera2D: Debug + Copy + PartialEq + AsStd140 {
    /// Declare the camera struct.
    const WGSL_STRUCT: &[&str];

    /// Inserted into the vertex function.
    ///
    /// Input:
    /// - `cam: Camera`.
    /// - `world_pos: vec2f`.
    ///
    /// Output:
    /// - set `output.pos`.
    const WGSL_VERTEX_LOGIC: &str;
}

#[derive(Debug, Clone, Copy, PartialEq, AsStd140)]
pub struct PosCamera2D {
    pub center: SVec2,
    pub ortho_size: f32,
}

impl Transform2D for SVec2P {
    type Resources = ();

    const LAYOUT: &[wgpu::VertexAttribute] = &[wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Sint32x2,
        offset: 0,
        shader_location: 0,
    }];

    const WGSL_VERTEX_FIELDS: &[&str] = &["center: vec2i"];

    const WGSL_GLOBALS: &[&str] = &[];

    const WGSL_VERTEX_LOGIC: &str = "
        let world_pos = vec2f(in.center + size * input.vertex_pos / 2) / 256.0;
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
