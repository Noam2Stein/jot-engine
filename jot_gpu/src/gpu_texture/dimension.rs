use std::fmt::Debug;

use super::*;

pub trait GpuTextureDimension {
    type Size: Debug + Construct;

    const TEXTURE_DIMENSION: wgpu::TextureDimension;
    const TEXTURE_VIEW_DIMENSION: wgpu::TextureViewDimension;

    fn extents3d(size: Self::Size) -> wgpu::Extent3d;
}

impl GpuTextureDimension for Usize<1> {
    type Size = u32;

    const TEXTURE_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D1;
    const TEXTURE_VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D1;

    fn extents3d(size: Self::Size) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: size,
            height: 1,
            depth_or_array_layers: 1,
        }
    }
}

impl GpuTextureDimension for Usize<2> {
    type Size = UVec2P;

    const TEXTURE_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D2;
    const TEXTURE_VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;

    fn extents3d(size: Self::Size) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: size.x(),
            height: size.y(),
            depth_or_array_layers: 1,
        }
    }
}

impl GpuTextureDimension for Usize<3> {
    type Size = UVec3P;

    const TEXTURE_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D3;
    const TEXTURE_VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D3;

    fn extents3d(size: Self::Size) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: size.x(),
            height: size.y(),
            depth_or_array_layers: size.z(),
        }
    }
}

// This file is named "dimension" because it has logic for texture dimension generics.
// Additionally, reminded me of <https://www.youtube.com/watch?v=gAqfiJCbhUY>.
