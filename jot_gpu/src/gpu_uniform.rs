use std::marker::PhantomData;

use crevice::std140::Std140;
use wgpu::util::DeviceExt;

use super::*;

pub struct GpuUniform<T: AsStd140> {
    pub(super) inner: wgpu::Buffer,
    t: PhantomData<T>,
}

impl Gpu {
    pub fn create_uniform<T: AsStd140, V: AsStd140<Output = T::Output>>(
        &self,
        value: V,
        is_mutable: bool,
    ) -> GpuUniform<T> {
        let inner = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: value.as_std140().as_bytes(),
                usage: if is_mutable {
                    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::UNIFORM
                },
            });

        GpuUniform {
            inner,
            t: PhantomData,
        }
    }
}

impl<T: AsStd140> GpuUniform<T> {
    pub fn mutate<V: AsStd140<Output = T::Output>>(&self, value: V, gpu: &Gpu) {
        gpu.queue
            .write_buffer(&self.inner, 0, value.as_std140().as_bytes());
    }
}
