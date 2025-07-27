use std::{fmt::Debug, marker::PhantomData};

use derive_where::derive_where;

use super::*;

pub use jot_gpu_proc_macros::{GpuBindings, GpuBindings_Local};

#[derive_where(Debug, Clone)]
pub struct GpuBindGroup<T: GpuBindings> {
    pub inner: wgpu::BindGroup,
    _t: PhantomData<T>,
}

pub unsafe trait GpuBindings: Debug + Clone {
    const BINDING_COUNT: usize;

    fn push_layout_entries(entries: &mut Vec<wgpu::BindGroupLayoutEntry>, binding: &mut u32);

    fn push_entries<'s>(&'s self, entries: &mut Vec<wgpu::BindGroupEntry<'s>>, binding: &mut u32);

    fn bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        let mut entries = Vec::with_capacity(Self::BINDING_COUNT);
        Self::push_layout_entries(&mut entries, &mut 0);

        gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            })
    }
}

impl Gpu {
    pub fn create_bind_group<T: GpuBindings>(&self, bindings: &T) -> GpuBindGroup<T> {
        let mut entries = Vec::with_capacity(T::BINDING_COUNT);
        bindings.push_entries(&mut entries, &mut 0);

        GpuBindGroup {
            inner: self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &T::bind_group_layout(self),
                entries: &entries,
            }),

            _t: PhantomData,
        }
    }
}

unsafe impl<T: GpuBindings> GpuBindings for &T {
    const BINDING_COUNT: usize = T::BINDING_COUNT;

    fn push_layout_entries(entries: &mut Vec<wgpu::BindGroupLayoutEntry>, binding: &mut u32) {
        T::push_layout_entries(entries, binding);
    }

    fn push_entries<'s>(&'s self, entries: &mut Vec<wgpu::BindGroupEntry<'s>>, binding: &mut u32) {
        T::push_entries(&self, entries, binding);
    }

    fn bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        T::bind_group_layout(gpu)
    }
}

impl<T: GpuBindings> GpuFrom<T> for GpuBindGroup<T> {
    fn gpu_from(value: T, gpu: &Gpu) -> Self {
        gpu.create_bind_group(&value)
    }
}
impl<T: GpuBindings> GpuFrom<&T> for GpuBindGroup<T> {
    fn gpu_from(value: &T, gpu: &Gpu) -> Self {
        gpu.create_bind_group(&value)
    }
}

macro_loop! {
    @for N in 0..=12 {
        unsafe impl<
            @for n in 0..@N {
                @[T @n]: GpuBindings,
            }
        > GpuBindings for (
            @for n in 0..@N {
                @[T @n],
            }
        ) {
            const BINDING_COUNT: usize = 0 @for n in 0..@N {
                + @[T @n]::BINDING_COUNT
            };

            fn push_layout_entries(_entries: &mut Vec<wgpu::BindGroupLayoutEntry>, _binding: &mut u32) {
                @for n in 0..@N {
                    @[T @n]::push_layout_entries(_entries, _binding);
                }
            }

            fn push_entries<'s>(&'s self, _entries: &mut Vec<wgpu::BindGroupEntry<'s>>, _binding: &mut u32) {
                @for n in 0..@N {
                    self.@n.push_entries(_entries, _binding);
                }
            }
        }
    }
}
