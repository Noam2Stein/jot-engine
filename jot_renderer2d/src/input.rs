use super::*;

#[derive(Debug, Clone, Copy)]
pub struct RenderInput2D<'a> {
    pub cam: Camera2D,
    pub quads: &'a [Quad],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2D {
    pub center: SVec2,
    pub ortho_size: f32,
    pub background_color: FVec4,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub rect: SRect2CP,
    pub depth: f32,
    pub color: FVec4P,
}
