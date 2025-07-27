use super::*;

#[derive(Debug, Clone)]
pub struct RenderInput2D<'a, V: Visual2D, T: Transform2D, C: Camera2D> {
    pub cam: C,
    pub quads: QuadBuffer2DSlice<'a, V, T>,
    pub background_color: Option<FVec4>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad2D<V: Visual2D, T: Transform2D> {
    pub visual: V,
    pub transform: T,
    pub depth: f32,
}
