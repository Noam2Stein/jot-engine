use wgpu::util::DeviceExt;

use super::*;

#[derive(Debug, Clone)]
pub struct SharedResources2D {
    pub(super) vertex_buf: wgpu::Buffer,
    pub(super) index_buf: wgpu::Buffer,
}

impl SharedResources2D {
    pub fn new(gpu: &Gpu) -> Self {
        let vertex_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Vertex Buffer"),
                contents: slice_bytes(&Self::VERTICIES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Index Buffer"),
                contents: slice_bytes(&Self::INDICIES),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buf,
            index_buf,
        }
    }

    pub(super) const VERTICIES: [IVec2P; 4] =
        [vec2p!(-1, -1), vec2p!(1, -1), vec2p!(1, 1), vec2p!(-1, 1)];

    pub(super) const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<IVec2P>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Sint32x2,
            offset: 0,
            shader_location: 0,
        }],
    };

    pub(super) const INDICIES: [u16; 6] = [0, 1, 2, 2, 3, 0];
}

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * size_of::<T>()) }
}
