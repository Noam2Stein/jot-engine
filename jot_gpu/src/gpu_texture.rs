use super::*;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GPUTexture {
    inner: wgpu::Texture,
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GPUTextureView {
    inner: wgpu::TextureView,
}

pub type GPUTextureDesc<'a> = wgpu::TextureDescriptor<'a>;
pub type GPUTextureViewDesc<'a> = wgpu::TextureViewDescriptor<'a>;

impl GPU {
    pub fn create_texture(&self, desc: &GPUTextureDesc) -> GPUTexture {
        GPUTexture {
            inner: self.device.create_texture(desc),
        }
    }
}

impl GPUTexture {
    pub fn create_view(&self, desc: &GPUTextureViewDesc) -> GPUTextureView {
        GPUTextureView {
            inner: self.inner.create_view(desc),
        }
    }

    pub fn clear(&self, color: FVec4, gpu: &GPU) {
        self.create_view(&GPUTextureViewDesc::default())
            .clear(color, gpu);
    }
}
impl GPUTextureView {
    pub fn clear(&self, color: FVec4, gpu: &GPU) {
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Clear Encoder"),
            });

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Renderer - Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.inner,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color.x.into(),
                            g: color.y.into(),
                            b: color.z.into(),
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
    }
}

impl From<&GPUTexture> for GPUTextureView {
    fn from(value: &GPUTexture) -> Self {
        value.create_view(&GPUTextureViewDesc::default())
    }
}
