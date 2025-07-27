use super::*;

#[derive(Debug, Clone)]
pub struct RenderInput2D<'a, V: Visual2D, T: Transform2D, C: Camera2D> {
    pub cam: &'a GpuBindGroup<GpuBuffer<Std140<C>>>,
    pub quads: GpuBufferSlice<'a, Quad2D<V, T>>,
    pub background_color: Option<FVec4>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad2D<V: Visual2D, T: Transform2D> {
    pub visual: V,
    pub transform: T,
    pub depth: f32,
}
