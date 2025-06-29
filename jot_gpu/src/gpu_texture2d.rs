use std::marker::PhantomData;

use super::*;

#[derive(Debug, Clone)]
pub struct GpuTexture2D<P: GpuTexturePrim> {
    pub(super) inner: wgpu::Texture,
    pub(super) inner_view: wgpu::TextureView,
    pub(super) p: PhantomData<P>,
}

#[derive(Debug, Clone)]
pub struct GpuTexture2DDesc {
    pub size: UVec2,
    pub format: GpuTextureFormat,
    pub usages: GpuTextureUsages,
}

pub type GpuTextureFormat = wgpu::TextureFormat;
pub type GpuTextureUsages = wgpu::TextureUsages;
pub type GpuSampleType = wgpu::TextureSampleType;

pub trait GpuTexturePrim: Copy {
    type Color: Copy;

    const SAMPLE_TYPE: GpuSampleType;

    fn convert_color(color: Self::Color) -> DVec4;
}

impl Gpu {
    pub fn create_texture2d<P: GpuTexturePrim>(&self, desc: &GpuTexture2DDesc) -> GpuTexture2D<P> {
        let inner = self.device.create_texture(&wgpu::TextureDescriptor {
            dimension: wgpu::TextureDimension::D2,
            format: desc.format,
            label: None,
            view_formats: &[],
            mip_level_count: 1,
            sample_count: 1,
            size: wgpu::Extent3d {
                width: desc.size.x,
                height: desc.size.y,
                depth_or_array_layers: 1,
            },
            usage: desc.usages,
        });

        let inner_view = inner.create_view(&Default::default());

        GpuTexture2D {
            inner,
            inner_view,
            p: PhantomData,
        }
    }
}

impl<P: GpuTexturePrim> GpuTexture2D<P> {
    pub fn size(&self) -> UVec2 {
        uvec2(self.inner.width(), self.inner.height())
    }

    pub fn clear(&self, color: P::Color, gpu: &Gpu) {
        let color_f64 = P::convert_color(color);

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Clear Encoder"),
            });

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Renderer - Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.inner_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color_f64.x,
                            g: color_f64.y,
                            b: color_f64.z,
                            a: color_f64.w,
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

impl GpuTexturePrim for u32 {
    type Color = UVec4;

    const SAMPLE_TYPE: GpuSampleType = GpuSampleType::Uint;

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
impl GpuTexturePrim for i32 {
    type Color = IVec4;

    const SAMPLE_TYPE: GpuSampleType = GpuSampleType::Sint;

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
impl GpuTexturePrim for f32 {
    type Color = FVec4;

    const SAMPLE_TYPE: GpuSampleType = GpuSampleType::Float { filterable: false };

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
