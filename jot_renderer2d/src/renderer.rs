use std::{mem::offset_of, num::NonZeroU64};

use crevice::std140::AsStd140;
use wgpu::util::DeviceExt;

use super::*;

pub struct Renderer2D<const QUAD_CAP: usize> {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    instance_buf: wgpu::Buffer,

    cam_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    pipeline: wgpu::RenderPipeline,
}

impl<const QUAD_CAP: usize> Renderer2D<QUAD_CAP> {
    pub fn new(gpu: &Gpu) -> Self {
        const VERTICIES: [IVec2P; 4] = [vec2p!(-1, -1), vec2p!(1, -1), vec2p!(1, 1), vec2p!(-1, 1)];

        const VERTEX_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: size_of::<IVec2P>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Sint32x2,
                offset: 0,
                shader_location: 0,
            }],
        };

        const INDICIES: [u16; 6] = [0, 1, 2, 2, 3, 0];

        const INSTANCE_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: size_of::<Quad>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // rect.center
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: offset_of!(Quad, rect) as u64,
                    shader_location: 1,
                },
                // rect.extents
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: (offset_of!(Quad, rect) + size_of::<SVec2P>()) as u64,
                    shader_location: 2,
                },
                // depth
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: offset_of!(Quad, depth) as u64,
                    shader_location: 3,
                },
                // color
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: offset_of!(Quad, color) as u64,
                    shader_location: 4,
                },
            ],
        };

        let vertex_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Vertex Buffer"),
                contents: slice_bytes(&VERTICIES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Index Buffer"),
                contents: slice_bytes(&INDICIES),
                usage: wgpu::BufferUsages::INDEX,
            });

        let instance_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Renderer2D Instance Buffer"),
            mapped_at_creation: false,
            size: size_of::<[Quad; QUAD_CAP]>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let cam_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Renderer2D Camera Buffer"),
            mapped_at_creation: false,
            size: size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Renderer2D Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Some(
                                NonZeroU64::new(size_of::<CameraUniform>() as u64).unwrap(),
                            ),
                        },
                        count: None,
                        visibility: wgpu::ShaderStages::VERTEX,
                    }],
                });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Renderer2D Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &cam_buf,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Renderer2D Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Renderer2D Shader Module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader::SOURCE)),
            });

        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Renderer2D Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[VERTEX_LAYOUT, INSTANCE_LAYOUT],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: GpuTextureFormat::Rgba8Unorm,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Max,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        Self {
            vertex_buf,
            index_buf,
            instance_buf,

            cam_buf,
            bind_group,

            pipeline,
        }
    }

    pub fn render(&self, input: RenderInput2D, output: &GpuTexture<2>, gpu: &Gpu) {
        gpu.queue.write_buffer(
            &self.cam_buf,
            0,
            CameraUniform {
                center: input.cam.center.map(|s| s.0).to_storage(),
                extents: vec2p!(
                    output.size().x() as f32 / output.size().y() as f32 * input.cam.ortho_size,
                    input.cam.ortho_size,
                ),
            }
            .as_std140()
            .as_bytes(),
        );

        if input.quads.len() == 0 {
            output.clear(Some(input.cam.background_color), None, gpu);
        }

        for (batch_idx, batch) in input.quads.chunks(QUAD_CAP).enumerate() {
            let batch_bytes = slice_bytes(batch);

            gpu.queue.write_buffer(&self.instance_buf, 0, batch_bytes);

            let mut encoder = gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Renderer2D Command Encoder"),
                });

            let color_load = if batch_idx == 0 {
                wgpu::LoadOp::Clear(wgpu::Color {
                    r: input.cam.background_color.x() as f64,
                    g: input.cam.background_color.y() as f64,
                    b: input.cam.background_color.z() as f64,
                    a: input.cam.background_color.w() as f64,
                })
            } else {
                wgpu::LoadOp::Load
            };

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Renderer2D Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output
                        .color_view
                        .as_ref()
                        .expect("attempted to render to a colorless texture"),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: color_load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: output
                        .depth_view
                        .as_ref()
                        .expect("attempted to render to a depthless texture"),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            pass.set_vertex_buffer(1, self.instance_buf.slice(..batch_bytes.len() as u64));

            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.pipeline);

            pass.draw_indexed(0..6, 0, 0..batch.len() as u32);

            drop(pass);

            gpu.queue.submit([encoder.finish()]);
        }
    }
}

#[derive(AsStd140)]
struct CameraUniform {
    center: IVec2P,
    extents: FVec2P,
}

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * size_of::<T>()) }
}
