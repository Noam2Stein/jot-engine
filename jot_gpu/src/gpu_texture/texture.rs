use std::fmt::Debug;

use wgpu::util::DeviceExt;

use super::*;

#[derive(Debug, Clone)]
pub struct GpuTextureDesc<'a, const DIM: usize>
where
    Usize<DIM>: GpuTextureDimension,
{
    pub size: <Usize<DIM> as GpuTextureDimension>::Size,

    pub color_format: Option<GpuTextureFormat>,
    pub color_data: Option<&'a [u8]>,

    pub depth_format: Option<GpuTextureFormat>,
    pub depth_data: Option<&'a [u8]>,

    pub usages: GpuTextureUsages,
}

#[derive(Debug, Clone)]
pub struct GpuTexture<const DIM: usize>
where
    Usize<DIM>: GpuTextureDimension,
{
    pub size: <Usize<DIM> as GpuTextureDimension>::Size,

    pub color: Option<wgpu::Texture>,
    pub color_view: Option<wgpu::TextureView>,

    pub depth: Option<wgpu::Texture>,
    pub depth_view: Option<wgpu::TextureView>,
}

pub type GpuTextureFormat = wgpu::TextureFormat;
pub type GpuTextureUsages = wgpu::TextureUsages;

impl Gpu {
    pub fn create_texture<const DIM: usize>(&self, desc: &GpuTextureDesc<DIM>) -> GpuTexture<DIM>
    where
        Usize<DIM>: GpuTextureDimension,
    {
        let color = if let Some(format) = desc.color_format {
            let texture_desc = wgpu::TextureDescriptor {
                dimension: <Usize<DIM> as GpuTextureDimension>::TEXTURE_DIMENSION,
                format,
                label: None,
                view_formats: &[],
                mip_level_count: 1,
                sample_count: 1,
                size: <Usize<DIM> as GpuTextureDimension>::extents3d(desc.size),
                usage: desc.usages,
            };

            let texture = if let Some(data) = desc.color_data {
                self.device.create_texture_with_data(
                    &self.queue,
                    &texture_desc,
                    wgpu::util::TextureDataOrder::LayerMajor,
                    data,
                )
            } else {
                self.device.create_texture(&texture_desc)
            };

            Some(texture)
        } else {
            None
        };

        let depth = if let Some(format) = desc.color_format {
            let texture_desc = wgpu::TextureDescriptor {
                dimension: <Usize<DIM> as GpuTextureDimension>::TEXTURE_DIMENSION,
                format,
                label: None,
                view_formats: &[],
                mip_level_count: 1,
                sample_count: 1,
                size: <Usize<DIM> as GpuTextureDimension>::extents3d(desc.size),
                usage: desc.usages,
            };

            let texture = if let Some(data) = desc.color_data {
                self.device.create_texture_with_data(
                    &self.queue,
                    &texture_desc,
                    wgpu::util::TextureDataOrder::LayerMajor,
                    data,
                )
            } else {
                self.device.create_texture(&texture_desc)
            };

            Some(texture)
        } else {
            None
        };

        let color_view = color
            .as_ref()
            .map(|texture| texture.create_view(&Default::default()));

        let depth_view = depth
            .as_ref()
            .map(|texture| texture.create_view(&Default::default()));

        GpuTexture {
            size: desc.size,

            color,
            color_view,
            depth,
            depth_view,
        }
    }
}

impl<'a, const DIM: usize> GpuTexture<DIM>
where
    Usize<DIM>: GpuTextureDimension,
{
    pub fn size(&self) -> <Usize<DIM> as GpuTextureDimension>::Size {
        self.size
    }

    pub fn clear(&self, color: Option<FVec4>, depth: Option<f32>, gpu: &Gpu) {
        if color.is_none() && depth.is_none() {
            return;
        }

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Clear Encoder"),
            });

        let color_attachment = color.map(|color| wgpu::RenderPassColorAttachment {
            view: self
                .color_view
                .as_ref()
                .expect("attempted to clear color of colorless texture"),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: color.x() as f64,
                    g: color.y() as f64,
                    b: color.z() as f64,
                    a: color.w() as f64,
                }),
                store: wgpu::StoreOp::Store,
            },
        });

        let depth_stencil_attachment = depth.map(|depth| wgpu::RenderPassDepthStencilAttachment {
            view: &self
                .depth_view
                .as_ref()
                .expect("attempted to clear depth of depthless texture"),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(depth),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Renderer - Render Pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        gpu.queue.submit(std::iter::once(encoder.finish()));
    }
}
