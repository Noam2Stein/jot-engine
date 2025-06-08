use std::{
    mem::take,
    sync::{Arc, RwLock},
};

use super::*;

pub struct GPUSurface<'target> {
    inner: wgpu::Surface<'target>,
    config: RwLock<wgpu::SurfaceConfiguration>,
}

pub struct GPUSurfaceTarget<'target> {
    inner: wgpu::SurfaceTarget<'target>,
    size: UVec2,
}

pub struct GPUSurfaceFrame {
    inner: Option<wgpu::SurfaceTexture>,
}

impl GPU {
    pub fn create_surface<'target>(
        &self,
        target: impl Into<GPUSurfaceTarget<'target>>,
    ) -> GPUSurface<'target> {
        let target: GPUSurfaceTarget = target.into();

        let surface = self
            .instance
            .create_surface(target.inner)
            .expect("Failed to create a surface");

        let config = surface
            .get_default_config(&self.adapter, target.size.x, target.size.y)
            .expect("Failed to create a default surface config");

        surface.configure(&self.device, &config);

        GPUSurface {
            inner: surface,
            config: RwLock::new(config),
        }
    }
}
impl<'target> GPUSurface<'target> {
    pub fn resize(&self, size: UVec2, gpu: &GPU) {
        let mut config = self.config.write().unwrap();

        config.width = size.x;
        config.height = size.y;

        self.inner.configure(&gpu.device, &config);
    }

    pub fn next_frame(&mut self) -> GPUSurfaceFrame {
        GPUSurfaceFrame {
            inner: Some(
                self.inner
                    .get_current_texture()
                    .expect("Failed to get the next surface frame"),
            ),
        }
    }
}

impl GPUSurfaceFrame {
    pub fn texture(&self) -> &GPUTexture {
        unsafe { std::mem::transmute(&self.inner.as_ref().unwrap().texture) }
    }
}
impl Drop for GPUSurfaceFrame {
    fn drop(&mut self) {
        take(&mut self.inner).unwrap().present();
    }
}

impl Into<GPUSurfaceTarget<'static>> for Arc<Window> {
    fn into(self) -> GPUSurfaceTarget<'static> {
        GPUSurfaceTarget {
            size: uvec2(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
impl<'window> Into<GPUSurfaceTarget<'window>> for &'window Window {
    fn into(self) -> GPUSurfaceTarget<'window> {
        GPUSurfaceTarget {
            size: uvec2(self.inner_size().width, self.inner_size().height),
            inner: self.into(),
        }
    }
}
