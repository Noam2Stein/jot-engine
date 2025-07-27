use std::ops::{Bound, Deref, DerefMut, RangeBounds};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct GpuBufferUninitSliceDesc<'a> {
    pub label: Option<&'a str>,
    pub usages: GpuBufferUsages,
    pub len: usize,
}

#[derive_where(Debug, Clone, Copy)]
pub struct GpuBufferSlice<'a, T: Sized> {
    pub buf: &'a GpuBuffer<[T]>,
    pub start: usize,
    pub end: usize,
}

impl Gpu {
    pub fn create_buffer_uninit_slice<T: Sized>(
        &self,
        desc: GpuBufferUninitSliceDesc,
    ) -> GpuBuffer<[T]> {
        GpuBuffer {
            inner: Some(self.device.create_buffer(&BufferDescriptor {
                label: desc.label,
                usage: desc.usages,
                size: (size_of::<T>() * desc.len) as u64,
                mapped_at_creation: false,
            })),

            _t: PhantomData,
        }
    }
}

impl<T: Sized, const N: usize> GpuBuffer<[T; N]> {
    pub fn len(&self) -> usize {
        N
    }

    pub fn as_slice(&self) -> &GpuBuffer<[T]> {
        unsafe { transmute::<&GpuBuffer<[T; N]>, &GpuBuffer<[T]>>(self) }
    }
    pub fn as_slice_mut(&mut self) -> &mut GpuBuffer<[T]> {
        unsafe { transmute::<&mut GpuBuffer<[T; N]>, &mut GpuBuffer<[T]>>(self) }
    }
    pub fn into_slice(self) -> GpuBuffer<[T]> {
        unsafe { transmute::<GpuBuffer<[T; N]>, GpuBuffer<[T]>>(self) }
    }

    pub fn set_range(&self, offset: usize, value: &[T], gpu: &Gpu) {
        AsRef::<GpuBuffer<[T]>>::as_ref(self).set_range(offset, value, gpu)
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> GpuBufferSlice<T> {
        AsRef::<GpuBuffer<[T]>>::as_ref(self).slice(range)
    }
}

impl<T: Sized> GpuBuffer<[T]> {
    pub fn len(&self) -> usize {
        if let Some(inner) = &self.inner {
            inner.size() as usize / size_of::<T>()
        } else {
            0
        }
    }

    pub fn set_range(&self, offset: usize, value: &[T], gpu: &Gpu) {
        gpu.queue.write_buffer(
            &self.inner.as_ref().expect("can't write to an empty buffer"),
            (offset * size_of::<T>()) as u64,
            unsafe {
                std::slice::from_raw_parts(
                    value as *const _ as *const u8,
                    size_of_val::<[T]>(value),
                )
            },
        );
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> GpuBufferSlice<T> {
        let start = match range.start_bound() {
            Bound::Excluded(&start) => start + 1,
            Bound::Included(&start) => start,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Excluded(&end) => end,
            Bound::Included(&end) => end + 1,
            Bound::Unbounded => self.len(),
        };

        GpuBufferSlice {
            buf: self,
            start,
            end,
        }
    }
}

impl<T: Sized, const N: usize> From<GpuBuffer<[T; N]>> for GpuBuffer<[T]> {
    fn from(value: GpuBuffer<[T; N]>) -> Self {
        value.into_slice()
    }
}
impl<T: Sized, const N: usize> AsRef<GpuBuffer<[T]>> for GpuBuffer<[T; N]> {
    fn as_ref(&self) -> &GpuBuffer<[T]> {
        self.as_slice()
    }
}
impl<T: Sized, const N: usize> AsMut<GpuBuffer<[T]>> for GpuBuffer<[T; N]> {
    fn as_mut(&mut self) -> &mut GpuBuffer<[T]> {
        self.as_slice_mut()
    }
}
impl<T: Sized, const N: usize> Deref for GpuBuffer<[T; N]> {
    type Target = GpuBuffer<[T]>;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T: Sized, const N: usize> DerefMut for GpuBuffer<[T; N]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<'a, T: Sized> GpuBufferSlice<'a, T> {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn set_range(&self, offset: usize, value: &[T], gpu: &Gpu) {
        assert!(offset + value.len() <= self.len());

        self.buf.set_range(self.start + offset, value, gpu);
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> GpuBufferSlice<T> {
        let start = match range.start_bound() {
            Bound::Excluded(&start) => start + 1,
            Bound::Included(&start) => start,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Excluded(&end) => end,
            Bound::Included(&end) => end + 1,
            Bound::Unbounded => self.len(),
        };

        assert!(start < self.len());
        assert!(end <= self.len());

        GpuBufferSlice {
            buf: self.buf,
            start: self.start + start,
            end: self.start + end,
        }
    }
}
