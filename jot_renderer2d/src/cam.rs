use std::fmt::Debug;

use crevice::std140::AsStd140;

use super::*;

pub trait Camera2D: Debug + Copy + PartialEq + AsStd140 {
    /// Declare the camera struct's fields.
    const WGSL_FIELDS: &[&str];

    /// Inserted into the vertex function.
    ///
    /// Input:
    /// - `cam: Camera`.
    /// - `world_pos: vec2f`.
    /// - `aspect: f32`.
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

/// Acts as a camera with its center at `(0, 0)` and its ortho_size as `1.0`.
#[derive(Debug, Clone, Copy, PartialEq, AsStd140)]
pub struct NoCamera2D {}

impl Camera2D for PosCamera2D {
    const WGSL_FIELDS: &[&str] = &["center: vec2i", "ortho_size: f32"];

    const WGSL_VERTEX_LOGIC: &str = "
        let cam_pos = world_pos - vec2f(cam.center) / 256.0;
        let cam_extents = vec2f(cam.ortho_size * aspect, cam.ortho_size);
        output.pos = vec4(cam_pos / cam_extents, input.depth, 1.0);
    ";
}

impl Camera2D for NoCamera2D {
    const WGSL_FIELDS: &[&str] = &[];

    const WGSL_VERTEX_LOGIC: &str = "
        output.pos = vec2(world_pos.x / aspect, world_pos.y);
    ";
}
