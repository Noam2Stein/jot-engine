use std::{
    marker::PhantomData,
    mem::take,
    sync::{Arc, RwLock},
};

use super::*;

pub struct GpuSurface<'target> {
    inner: wgpu::Surface<'target>,
    config: RwLock<wgpu::SurfaceConfiguration>,
}

pub struct GpuSurfaceTarget<'target> {
    inner: wgpu::SurfaceTarget<'target>,
    size: UVec2,
}

pub struct GpuSurfaceFrame<'f> {
    f: PhantomData<&'f ()>,
    inner: Option<wgpu::SurfaceTexture>,
    texture: GpuTexture2D<f32>,
}

impl Gpu {
    pub fn create_surface<'target>(
        &self,
        target: impl Into<GpuSurfaceTarget<'target>>,
    ) -> GpuSurface<'target> {
        let target: GpuSurfaceTarget = target.into();

        let surface = self
            .instance
            .create_surface(target.inner)
            .expect("Failed to create a surface");

        let config = surface
            .get_default_config(&self.adapter, target.size.x, target.size.y)
            .expect("Failed to create a default surface config");

        surface.configure(&self.device, &config);

        GpuSurface {
            inner: surface,
            config: RwLock::new(config),
        }
    }
}
impl<'target> GpuSurface<'target> {
    pub fn resize(&self, size: UVec2, gpu: &Gpu) {
        let mut config = self.config.write().unwrap();

        config.width = size.x;
        config.height = size.y;

        self.inner.configure(&gpu.device, &config);
    }

    pub fn next_frame(&mut self) -> GpuSurfaceFrame {
        let inner = self
            .inner
            .get_current_texture()
            .expect("Failed to get the next surface frame");

        let texture = GpuTexture2D {
            inner: inner.texture.clone(),
            inner_view: inner.texture.create_view(&Default::default()),
            p: PhantomData,
        };

        GpuSurfaceFrame {
            f: PhantomData,
            inner: Some(inner),
            texture,
        }
    }
}

impl<'frame> GpuSurfaceFrame<'frame> {
    pub fn texture(&self) -> &GpuTexture2D<f32> {
        unsafe { std::mem::transmute(&self.inner.as_ref().unwrap().texture) }
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
            size: uvec2(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
impl<'window> Into<GpuSurfaceTarget<'window>> for &'window Window {
    fn into(self) -> GpuSurfaceTarget<'window> {
        GpuSurfaceTarget {
            size: uvec2(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
