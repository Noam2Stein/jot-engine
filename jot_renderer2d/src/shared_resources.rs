use super::*;

#[derive(Debug, Clone)]
pub struct SharedResources2D {
    pub(super) vertex_buf: GpuBuffer<[IVec2P]>,
    pub(super) index_buf: GpuBuffer<[u16]>,

    pub(super) aspect_buf: GpuBuffer<f32>,
    pub(super) bind_group: GpuBindGroup<GpuBuffer<f32>>,
}

impl SharedResources2D {
    pub fn new(gpu: &Gpu) -> Self {
        let vertex_buf = gpu
            .create_buffer(GpuBufferDesc {
                label: Some("Renderer2D VertexBuffer"),
                usages: GpuBufferUsages::VERTEX,
                value: &Self::VERTICIES,
            })
            .into_slice();

        let index_buf = gpu
            .create_buffer(GpuBufferDesc {
                label: Some("Renderer2D IndexBuffer"),
                usages: GpuBufferUsages::INDEX,
                value: &Self::INDICIES,
            })
            .into_slice();

        let aspect_buf = gpu.create_buffer_uninit(GpuBufferUninitDesc {
            label: Some("Renderer2D Aspect Buffer"),
            usages: GpuBufferUsages::COPY_DST | GpuBufferUsages::UNIFORM,
        });

        let bind_group = gpu.create_bind_group(&aspect_buf);

        Self {
            vertex_buf,
            index_buf,

            aspect_buf,
            bind_group,
        }
    }

    pub(super) const VERTICIES: [IVec2P; 4] =
        [vec2p!(-1, -1), vec2p!(1, -1), vec2p!(1, 1), vec2p!(-1, 1)];

    pub(super) const INDICIES: [u16; 6] = [0, 1, 2, 2, 3, 0];
}
