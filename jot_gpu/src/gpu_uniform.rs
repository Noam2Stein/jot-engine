use std::marker::PhantomData;

use super::*;

pub struct GpuUniform<T> {
    pub(super) inner: wgpu::Buffer,
    t: PhantomData<T>,
}
