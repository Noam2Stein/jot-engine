use jot_math::*;

pub use wgpu::{
    BindGroup as GPUBindGroup, Buffer as GPUBuffer, BufferDescriptor as GPUBufferDesc,
    BufferUsages as GPUBufferUsages, Color as GPUColor, CommandEncoder as GPUCommandEncoder,
    CommandEncoderDescriptor as GPUCommandEncoderDesc, ComputePipeline as GPUComputePipeline,
    ComputePipelineDescriptor as GPUComputePipelineDesc,
    CreateSurfaceError as CreateGPUSurfaceError, Device as GPUDevice,
    DeviceDescriptor as GPUDeviceDesc, Extent3d as GPUExtent3d, Instance as GPU,
    LoadOp as GPULoadOp, Maintain as GPUMaintain, MapMode as GPUMapMode, Operations as GPUOps,
    Origin3d as GPUOrigin3d, PipelineCache, PipelineCacheDescriptor as GPUPipelineCacheDesc,
    PipelineCompilationOptions as GPUPipelineCompilationOptions,
    PipelineLayout as GPUPipelineLayout, PipelineLayoutDescriptor as GPUPipelineLayoutDesc,
    PipelineStatisticsTypes as GPUPipelineStatsTypes, PresentMode as GPUPresentMode,
    Queue as GPUQueue, RenderPassColorAttachment as GPURenderPassColorAttachment,
    RenderPassDescriptor as GPURenderPassDesc, RenderPipeline as GPURenderPipeline,
    RequestAdapterOptions as GPURequestAdapterOptions, StoreOp as GPUStoreOp,
    Surface as GPUSurface, SurfaceCapabilities as GPUSurfaceCapabilities,
    SurfaceConfiguration as GPUSurfaceConfig, SurfaceError as GPUSurfaceError,
    SurfaceStatus as GPUSurfaceStatus, SurfaceTarget as GPUSurfaceTarget,
    SurfaceTargetUnsafe as GPUSurfaceTargetUnsafe, SurfaceTexture as GPUSurfaceTexture,
    TexelCopyBufferInfo as GPUTexelCopyBufferInfo,
    TexelCopyBufferLayout as GPUTexelCopyBufferLayout,
    TexelCopyTextureInfo as GPUTexelCopyTextureInfo, Texture as GPUTexture,
    TextureAspect as GPUTextureAspect, TextureDescriptor as GPUTextureDesc,
    TextureDimension as GPUTextureDimension, TextureFormat as GPUTextureFormat,
    TextureUsages as GPUTextureUsages, TextureView as GPUTextureView,
    TextureViewDescriptor as GPUTextureViewDesc, VertexBufferLayout as GPUVertexBufferLayout,
    vertex_attr_array as gpu_vertex_attrs,
};

mod clear;
mod context;
pub use clear::*;
pub use context::*;
