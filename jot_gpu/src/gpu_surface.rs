use std::{
    marker::PhantomData,
    mem::take,
    sync::{Arc, RwLock},
};

use super::*;

#[derive(Debug, Clone, Default)]
pub struct GpuSurfaceDesc {
    pub depth_enabled: bool,
}

pub struct GpuSurface<'target> {
    inner: wgpu::Surface<'target>,
    depth_texture: Option<RwLock<wgpu::Texture>>,
    config: RwLock<wgpu::SurfaceConfiguration>,
}

pub struct GpuSurfaceTarget<'target> {
    inner: wgpu::SurfaceTarget<'target>,
    size: UVec2,
}

pub struct GpuSurfaceFrame<'f> {
    f: PhantomData<&'f ()>,
    inner: Option<wgpu::SurfaceTexture>,
    texture: GpuTexture<2>,
}

impl Gpu {
    pub fn create_surface<'target>(
        &self,
        target: impl Into<GpuSurfaceTarget<'target>>,
        desc: &GpuSurfaceDesc,
    ) -> GpuSurface<'target> {
        let target: GpuSurfaceTarget = target.into();

        let surface = self
            .instance
            .create_surface(target.inner)
            .expect("Failed to create a surface");

        let mut config = surface
            .get_default_config(&self.adapter, target.size.x(), target.size.y())
            .expect("Failed to create a default surface config");

        config.format = GpuTextureFormat::Rgba8Unorm;
        config.present_mode = wgpu::PresentMode::AutoVsync;

        surface.configure(&self.device, &config);

        let depth_texture = desc
            .depth_enabled
            .then(|| {
                self.device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: config.width,
                        height: config.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth32Float,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
            })
            .map(RwLock::new);

        GpuSurface {
            inner: surface,
            depth_texture,
            config: RwLock::new(config),
        }
    }
}
impl<'target> GpuSurface<'target> {
    pub fn resize(&self, size: UVec2, gpu: &Gpu) {
        let mut config = self.config.write().unwrap();

        config.width = size.x();
        config.height = size.y();

        self.inner.configure(&gpu.device, &config);

        if let Some(depth_texture) = &self.depth_texture {
            *depth_texture.write().unwrap() = gpu.device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: config.width,
                    height: config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
        }
    }

    pub fn next_frame(&mut self) -> GpuSurfaceFrame {
        let inner = self
            .inner
            .get_current_texture()
            .expect("Failed to get the next surface frame");

        let texture = GpuTexture {
            size: vec2p!(inner.texture.width(), inner.texture.height()),
            color: Some(inner.texture.clone()),
            color_view: Some(inner.texture.create_view(&Default::default())),
            depth: self
                .depth_texture
                .as_ref()
                .map(|rw| rw.read().unwrap().clone()),
            depth_view: self
                .depth_texture
                .as_ref()
                .map(|rw| rw.read().unwrap().create_view(&Default::default())),
        };

        GpuSurfaceFrame {
            f: PhantomData,
            inner: Some(inner),
            texture,
        }
    }
}

impl<'frame> GpuSurfaceFrame<'frame> {
    pub fn texture(&self) -> &GpuTexture<2> {
        &self.texture
    }
}
impl<'frame> Drop for GpuSurfaceFrame<'frame> {
    fn drop(&mut self) {
        take(&mut self.inner).unwrap().present();
    }
}

impl Into<GpuSurfaceTarget<'static>> for Arc<Window> {
    fn into(self) -> GpuSurfaceTarget<'static> {
        GpuSurfaceTarget {
            size: vec2!(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
impl<'window> Into<GpuSurfaceTarget<'window>> for &'window Window {
    fn into(self) -> GpuSurfaceTarget<'window> {
        GpuSurfaceTarget {
            size: vec2!(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
