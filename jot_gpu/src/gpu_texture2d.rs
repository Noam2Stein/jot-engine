use std::{fmt::Debug, marker::PhantomData};

use wgpu::util::DeviceExt;

use super::*;

#[derive(Debug, Clone)]
pub struct GpuTexture2D<P: GpuTexturePrim> {
    pub(super) inner: wgpu::Texture,
    pub(super) inner_view: wgpu::TextureView,
    pub(super) p: PhantomData<P>,
}

pub type GpuTextureFormat = wgpu::TextureFormat;
pub type GpuTextureUsages = wgpu::TextureUsages;
pub type GpuTextureAspect = wgpu::TextureAspect;

pub trait GpuTexturePrim: Debug + Copy {
    type Color: Copy;

    const SAMPLE_TYPE: wgpu::TextureSampleType;

    fn convert_color(color: Self::Color) -> DVec4;
}

impl Gpu {
    pub fn create_texture2d<P: GpuTexturePrim>(
        &self,
        size: UVec2,
        format: GpuTextureFormat,
        usages: GpuTextureUsages,
        data: Option<&[P]>,
    ) -> GpuTexture2D<P> {
        let desc = wgpu::TextureDescriptor {
            dimension: wgpu::TextureDimension::D2,
            format,
            label: None,
            view_formats: &[],
            mip_level_count: 1,
            sample_count: 1,
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            usage: usages,
        };

        let inner = if let Some(data) = data {
            self.device.create_texture_with_data(
                &self.queue,
                &desc,
                wgpu::util::TextureDataOrder::LayerMajor,
                unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * size_of::<P>(),
                    )
                },
            )
        } else {
            self.device.create_texture(&desc)
        };

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

    pub fn mutate(&self, rect: Option, aspect: GpuTextureAspect, data: &[P], gpu: &Gpu) {
        gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.inner,
                aspect,
                mip_level: 0,
                origin: wgpu::Origin3d,
            },
            data,
            data_layout,
            size,
        );
    }
}

impl GpuTexturePrim for u32 {
    type Color = UVec4;

    const SAMPLE_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Uint;

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
impl GpuTexturePrim for i32 {
    type Color = IVec4;

    const SAMPLE_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Sint;

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
impl GpuTexturePrim for f32 {
    type Color = FVec4;

    const SAMPLE_TYPE: wgpu::TextureSampleType =
        wgpu::TextureSampleType::Float { filterable: false };

    fn convert_color(color: Self::Color) -> DVec4 {
        color.as_dvec4()
    }
}
