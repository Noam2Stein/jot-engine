use std::marker::PhantomData;

use super::*;

pub struct GpuBindGroup<B: GpuBindings> {
    inner: wgpu::BindGroup,
    b: PhantomData<B>,
}

pub trait GpuBindings {
    const BINDING_CAP: usize;

    fn push_descs(&self, descs: &mut Vec<wgpu::BindGroupLayoutEntry>);

    fn push_bindings<'a, 'b>(&'a self, bindings: &'b mut Vec<wgpu::BindGroupEntry<'a>>);
}

impl Gpu {
    pub fn create_bind_group<B: GpuBindings>(&self, bindings: B) -> GpuBindGroup<B> {
        let mut entries = Vec::with_capacity(B::BINDING_CAP);
        let mut layout_entries = Vec::with_capacity(B::BINDING_CAP);

        bindings.push_descs(&mut layout_entries);
        bindings.push_bindings(&mut entries);

        let layout_desc = wgpu::BindGroupLayoutDescriptor {
            entries: &layout_entries,
            label: None,
        };

        let layout = self.device.create_bind_group_layout(&layout_desc);

        let desc = wgpu::BindGroupDescriptor {
            entries: &entries,
            layout: &layout,
            label: None,
        };

        let inner = self.device.create_bind_group(&desc);

        GpuBindGroup {
            inner,
            b: PhantomData,
        }
    }
}

impl<T: AsStd140> GpuBindings for GpuUniform<T> {
    const BINDING_CAP: usize = 1;

    fn push_descs(&self, descs: &mut Vec<wgpu::BindGroupLayoutEntry>) {
        descs.push(wgpu::BindGroupLayoutEntry {
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            binding: descs.len() as u32,
            count: None,
        });
    }

    fn push_bindings<'a, 'b>(&'a self, bindings: &'b mut Vec<wgpu::BindGroupEntry<'a>>) {
        bindings.push(wgpu::BindGroupEntry {
            binding: bindings.len() as u32,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.inner,
                offset: 0,
                size: None,
            }),
        });
    }
}

impl<P: GpuTexturePrim> GpuBindings for GpuTexture2D<P> {
    const BINDING_CAP: usize = 1;

    fn push_descs(&self, descs: &mut Vec<wgpu::BindGroupLayoutEntry>) {
        descs.push(wgpu::BindGroupLayoutEntry {
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Texture {
                sample_type: P::SAMPLE_TYPE,
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            binding: descs.len() as u32,
            count: None,
        });
    }

    fn push_bindings<'a, 'b>(&'a self, bindings: &'b mut Vec<wgpu::BindGroupEntry<'a>>) {
        bindings.push(wgpu::BindGroupEntry {
            binding: bindings.len() as u32,
            resource: wgpu::BindingResource::TextureView(&self.inner_view),
        });
    }
}
