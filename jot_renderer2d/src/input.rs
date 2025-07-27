use super::*;

#[derive(Debug, Clone)]
pub struct RenderInput2D<'a, V: Visual2D, T: Transform2D, C: Camera2D> {
    pub cam: &'a GpuBindGroup<GpuBuffer<Std140<C>>>,
    pub background_color: Option<FVec4>,

    pub quads: GpuBufferSlice<'a, Quad2D<V, T>>,
    pub visual_bind_group: &'a GpuBindGroup<V::Bindings>,
    pub transform_bind_group: &'a GpuBindGroup<T::Bindings>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad2D<V: Visual2D, T: Transform2D> {
    pub visual: V,
    pub transform: T,
    pub depth: f32,
}
