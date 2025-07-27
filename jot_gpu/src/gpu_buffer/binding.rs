use super::*;

unsafe impl<T: Sized> GpuBindings for GpuBuffer<T> {
    const BINDING_COUNT: usize = 1;

    fn push_layout_entries(entries: &mut Vec<wgpu::BindGroupLayoutEntry>, binding: &mut u32) {
        if size_of::<T>() != 0 {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: *binding,
                visibility: wgpu::ShaderStages::all(), // placeholder probably
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None, // placeholder probably
            });
        }

        *binding += 1;
    }

    fn push_entries<'s>(&'s self, entries: &mut Vec<wgpu::BindGroupEntry<'s>>, binding: &mut u32) {
        if let Some(inner) = &self.inner {
            entries.push(wgpu::BindGroupEntry {
                binding: *binding,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: inner,
                    offset: 0,
                    size: None,
                }),
            });
        }

        *binding += 1;
    }
}
