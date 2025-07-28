use super::*;

#[derive(Debug, Clone, Copy)]
pub struct TilemapRenderInput<
    'a,
    const CHUNK_HEIGHT: u32,
    V: Visual2D,
    T: TileTransform2D,
    C: TilemapCamera2D,
> {
    pub cam: C,
    pub cam_bind_group: &'a GpuBindGroup<GpuBuffer<Std140<C>>>,
    pub background_color: Option<FVec4>,

    pub visual_bind_group: &'a GpuBindGroup<V::Bindings>,
    pub transform_bind_group: &'a GpuBindGroup<T::Bindings>,
}
