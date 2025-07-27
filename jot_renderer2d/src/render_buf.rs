use std::{
    marker::PhantomData,
    ops::{Bound, Range, RangeBounds},
};

use wgpu::util::DeviceExt;

use super::*;

#[derive(Debug, Clone)]
pub struct QuadBuffer2D<V: Visual2D, T: Transform2D> {
    pub(super) inner: wgpu::Buffer,

    _v: PhantomData<V>,
    _t: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct QuadBuffer2DSlice<'a, V: Visual2D, T: Transform2D> {
    pub(super) buf: &'a QuadBuffer2D<V, T>,
    pub(super) range: Range<usize>,
}

impl<V: Visual2D, T: Transform2D> QuadBuffer2D<V, T> {
    /// Creates a new buffer, which is then unexpandable.
    pub fn new(values: &[Quad2D<V, T>], gpu: &Gpu) -> Self {
        Self {
            inner: gpu
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("RenderBuffer2D"),
                    contents: slice_bytes(values),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),

            _v: PhantomData,
            _t: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.size() as usize / size_of::<Quad2D<V, T>>()
    }

    pub fn write(&self, offset: usize, values: &[Quad2D<V, T>], gpu: &Gpu) {
        gpu.queue
            .write_buffer(&self.inner, offset as u64, slice_bytes(values));
    }

    pub fn slice(&self, bounds: impl RangeBounds<usize>) -> QuadBuffer2DSlice<V, T> {
        let start = match bounds.start_bound() {
            Bound::Included(&bound) => bound,
            Bound::Excluded(&bound) => bound + 1,
            Bound::Unbounded => 0,
        };

        let end = match bounds.end_bound() {
            Bound::Included(&bound) => bound + 1,
            Bound::Excluded(&bound) => bound,
            Bound::Unbounded => self.len(),
        };

        QuadBuffer2DSlice {
            buf: self,
            range: Range { start, end },
        }
    }
}

impl<'a, V: Visual2D, T: Transform2D> QuadBuffer2DSlice<'a, V, T> {
    pub fn len(&self) -> usize {
        self.range.len()
    }

    pub fn write(&self, offset: usize, values: &[Quad2D<V, T>], gpu: &Gpu) {
        gpu.queue
            .write_buffer(&self.buf.inner, offset as u64, slice_bytes(values));
    }

    pub fn slice(&self, bounds: impl RangeBounds<usize>) -> QuadBuffer2DSlice<V, T> {
        let start = self.range.start
            + match bounds.start_bound() {
                Bound::Included(&bound) => bound,
                Bound::Excluded(&bound) => bound + 1,
                Bound::Unbounded => 0,
            };

        let end = self.range.end
            + match bounds.end_bound() {
                Bound::Included(&bound) => bound + 1,
                Bound::Excluded(&bound) => bound,
                Bound::Unbounded => self.len(),
            };

        QuadBuffer2DSlice {
            buf: self.buf,
            range: Range { start, end },
        }
    }
}

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * size_of::<T>()) }
}
