use super::*;

unsafe impl<const DIM: usize> GpuBindings for GpuTexture<DIM>
where
    Usize<DIM>: GpuTextureDimension,
{
    const BINDING_COUNT: usize = 1;

    fn push_layout_entries(entries: &mut Vec<wgpu::BindGroupLayoutEntry>, binding: &mut u32) {
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: *binding,
            visibility: wgpu::ShaderStages::all(), // placeholder probably
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: <Usize<DIM> as GpuTextureDimension>::TEXTURE_VIEW_DIMENSION,
                multisampled: false,
            },
            count: None,
        });

        *binding += 1;

        entries.push(wgpu::BindGroupLayoutEntry {
            binding: *binding,
            visibility: wgpu::ShaderStages::all(), // placeholder probably
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
            count: None,
        });

        *binding += 1;
    }

    fn push_entries<'s>(&'s self, entries: &mut Vec<wgpu::BindGroupEntry<'s>>, binding: &mut u32) {
        entries.push(wgpu::BindGroupEntry {
            binding: *binding,
            resource: wgpu::BindingResource::TextureView(
                self.color_view.as_ref().expect("expected color"),
            ),
        });

        *binding += 1;

        entries.push(wgpu::BindGroupEntry {
            binding: *binding,
            resource: wgpu::BindingResource::Sampler(
                self.sampler.as_ref().expect("expected sampler"),
            ),
        });

        *binding += 1;
    }
}
