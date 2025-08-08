use std::{marker::PhantomData, mem::transmute};

use derive_where::derive_where;
use wgpu::{
    BufferDescriptor,
    util::{BufferInitDescriptor, DeviceExt},
};

use super::*;

mod array;
mod binding;
pub use array::*;

#[derive_where(crate = "derive_where")]
#[derive_where(Debug, Clone)]
pub struct GpuBuffer<T: ?Sized> {
    pub inner: Option<wgpu::Buffer>,
    _t: PhantomData<T>,
}

#[derive(Debug)]
#[derive_where(crate = "derive_where")]
#[derive_where(Clone, Copy)]
pub struct GpuBufferDesc<'a, T: ?Sized> {
    pub label: Option<&'a str>,
    pub usages: GpuBufferUsages,
    pub value: &'a T,
}

#[derive(Debug, Clone, Copy)]
pub struct GpuBufferUninitDesc<'a> {
    pub label: Option<&'a str>,
    pub usages: GpuBufferUsages,
}

pub type GpuBufferUsages = wgpu::BufferUsages;

impl Gpu {
    pub fn create_buffer<T: ?Sized>(&self, desc: GpuBufferDesc<T>) -> GpuBuffer<T> {
        let size = size_of_val::<T>(desc.value);

        let inner = if size == 0 {
            None
        } else {
            Some(self.device.create_buffer_init(&BufferInitDescriptor {
                label: desc.label,
                usage: desc.usages,
                contents: unsafe {
                    std::slice::from_raw_parts(desc.value as *const _ as *const u8, size)
                },
            }))
        };

        GpuBuffer {
            inner,
            _t: PhantomData,
        }
    }

    pub fn create_buffer_uninit<T: Sized>(&self, desc: GpuBufferUninitDesc) -> GpuBuffer<T> {
        let size = size_of::<T>();

        let inner = if size == 0 {
            None
        } else {
            Some(self.device.create_buffer(&BufferDescriptor {
                label: desc.label,
                usage: desc.usages,
                size: size as u64,
                mapped_at_creation: false,
            }))
        };

        GpuBuffer {
            inner,
            _t: PhantomData,
        }
    }
}

impl<T: ?Sized> GpuBuffer<T> {
    pub fn set(&self, value: &T, gpu: &Gpu)
    where
        T: Sized,
    {
        if let Some(inner) = &self.inner {
            gpu.queue.write_buffer(inner, 0, unsafe {
                std::slice::from_raw_parts(value as *const _ as *const u8, size_of::<T>())
            });
        }
    }
}
