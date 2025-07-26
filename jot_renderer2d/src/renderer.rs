use std::{marker::PhantomData, mem::offset_of, num::NonZeroU64};

use const_format::{StrWriter, unwrap, writec};
use crevice::std140::{AsStd140, Std140};
use wgpu::util::DeviceExt;

use super::*;

pub struct Renderer2D<const QUAD_CAP: usize, V: Visual2D, T: Transform2D, C: Camera2D> {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    instance_buf: wgpu::Buffer,

    aspect_buf: wgpu::Buffer,
    cam_buf: Option<wgpu::Buffer>,
    bind_group: wgpu::BindGroup,

    visual_bind_group: wgpu::BindGroup,
    transform_bind_group: wgpu::BindGroup,

    pipeline: wgpu::RenderPipeline,

    _v: PhantomData<V>,
    _t: PhantomData<T>,
    _c: PhantomData<C>,
}

impl<const QUAD_CAP: usize, V: Visual2D, T: Transform2D, C: Camera2D>
    Renderer2D<QUAD_CAP, V, T, C>
{
    pub fn new(
        gpu: &Gpu,
        visual_resoureces: V::Resources,
        transform_resources: T::Resources,
    ) -> Self {
        let vertex_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Vertex Buffer"),
                contents: slice_bytes(&Self::VERTICIES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buf = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderer2D Index Buffer"),
                contents: slice_bytes(&Self::INDICIES),
                usage: wgpu::BufferUsages::INDEX,
            });

        let instance_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Renderer2D Instance Buffer"),
            mapped_at_creation: false,
            size: size_of::<[Quad2D<V, T>; QUAD_CAP]>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let aspect_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Renderer2D Aspect Buffer"),
            mapped_at_creation: false,
            size: size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cam_buf = if !C::WGSL_FIELDS.is_empty() {
            Some(gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Renderer2D Camera Buffer"),
                mapped_at_creation: false,
                size: size_of::<CameraUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }))
        } else {
            None
        };

        let aspect_layout_entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(size_of::<f32>() as u64).unwrap()),
            },
            count: None,
            visibility: wgpu::ShaderStages::all(),
        };

        let bind_group_layout = if let Some(_) = &cam_buf {
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Renderer2D Bind Group Layout"),
                    entries: &[
                        aspect_layout_entry,
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: Some(
                                    NonZeroU64::new(size_of::<CameraUniform>() as u64).unwrap(),
                                ),
                            },
                            count: None,
                            visibility: wgpu::ShaderStages::VERTEX,
                        },
                    ],
                })
        } else {
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Renderer2D Bind Group Layout"),
                    entries: &[aspect_layout_entry],
                })
        };

        let aspect_resource = wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &aspect_buf,
                offset: 0,
                size: None,
            }),
        };

        let bind_group = if let Some(cam_buf) = &cam_buf {
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Renderer2D Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    aspect_resource,
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &cam_buf,
                            offset: 0,
                            size: None,
                        }),
                    },
                ],
            })
        } else {
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Renderer2D Bind Group"),
                layout: &bind_group_layout,
                entries: &[aspect_resource],
            })
        };

        let visual_bind_group_layout = V::create_bind_group_layout(gpu);
        let visual_bind_group =
            V::create_bind_group(visual_resoureces, &visual_bind_group_layout, gpu);

        let transform_bind_group_layout = T::create_bind_group_layout(gpu);
        let transform_bind_group =
            T::create_bind_group(transform_resources, &transform_bind_group_layout, gpu);

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Renderer2D Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout, &visual_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Renderer2D Shader Module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(Self::SHADER)),
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
                    buffers: &[Self::VERTEX_LAYOUT, Self::INSTANCE_LAYOUT],
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

            aspect_buf,
            cam_buf,
            bind_group,

            visual_bind_group,
            transform_bind_group,

            pipeline,

            _v: PhantomData,
            _t: PhantomData,
            _c: PhantomData,
        }
    }

    pub fn render(&self, input: RenderInput2D<V, T, C>, output: &GpuTexture<2>, gpu: &Gpu) {
        gpu.queue.write_buffer(
            &self.aspect_buf,
            0,
            slice_bytes(&[output.size().x() as f32 / output.size().y() as f32]),
        );

        if let Some(cam_buf) = &self.cam_buf {
            gpu.queue
                .write_buffer(cam_buf, 0, input.cam.as_std140().as_bytes());
        }

        if input.quads.len() == 0 {
            output.clear(Some(input.background_color), None, gpu);
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
                    r: input.background_color.x() as f64,
                    g: input.background_color.y() as f64,
                    b: input.background_color.z() as f64,
                    a: input.background_color.w() as f64,
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
            pass.set_bind_group(1, &self.visual_bind_group, &[]);
            pass.set_bind_group(2, &self.transform_bind_group, &[]);
            pass.set_pipeline(&self.pipeline);

            pass.draw_indexed(0..6, 0, 0..batch.len() as u32);

            drop(pass);

            gpu.queue.submit([encoder.finish()]);
        }
    }

    const VERTICIES: [IVec2P; 4] = [vec2p!(-1, -1), vec2p!(1, -1), vec2p!(1, 1), vec2p!(-1, 1)];

    const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<IVec2P>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Sint32x2,
            offset: 0,
            shader_location: 0,
        }],
    };

    const INDICIES: [u16; 6] = [0, 1, 2, 2, 3, 0];

    const INSTANCE_LAYOUT_ATTRIBUTES: [wgpu::VertexAttribute; 64] = {
        let mut output = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float16,
            offset: 0,
            shader_location: 0,
        }; 64];
        let mut output_idx = 0;

        let mut i = 0;
        while i < V::LAYOUT.len() {
            output[output_idx] = wgpu::VertexAttribute {
                format: V::LAYOUT[i].format,
                offset: offset_of!(Quad2D<V, T>, visual) as u64 + V::LAYOUT[i].offset,
                shader_location: output_idx as u32 + 1,
            };
            output_idx += 1;

            i += 1;
        }

        let mut i = 0;
        while i < T::LAYOUT.len() {
            output[output_idx] = wgpu::VertexAttribute {
                format: T::LAYOUT[i].format,
                offset: offset_of!(Quad2D<V, T>, transform) as u64 + T::LAYOUT[i].offset,
                shader_location: output_idx as u32 + 1,
            };
            output_idx += 1;

            i += 1;
        }

        // depth
        output[output_idx] = wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32,
            offset: offset_of!(Quad2D<V, T>, depth) as u64,
            shader_location: output_idx as u32 + 1,
        };
        #[allow(unused_assignments)]
        {
            output_idx += 1;
        }

        output
    };

    const INSTANCE_LAYOUT: wgpu::VertexBufferLayout<'static> = {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Quad2D<V, T>>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &unsafe {
                std::slice::from_raw_parts(
                    Self::INSTANCE_LAYOUT_ATTRIBUTES.as_ptr(),
                    V::LAYOUT.len() + T::LAYOUT.len() + 1,
                )
            },
        }
    };

    const SHADER_RAW: StrWriter<[u8; 2048]> = {
        let w = &mut StrWriter::new([0; 2048]);

        // Aspect
        unwrap!(writec!(
            w,
            "@group(0) @binding(0) var<uniform> aspect: f32;"
        ));

        // Camera
        if !C::WGSL_FIELDS.is_empty() {
            unwrap!(writec!(w, "struct Camera {{"));

            let mut i = 0;
            while i < C::WGSL_FIELDS.len() {
                unwrap!(writec!(w, "{},", C::WGSL_FIELDS[i]));
                i += 1;
            }

            unwrap!(writec!(w, "}}"));
            unwrap!(writec!(
                w,
                "@group(0) @binding(1) var<uniform> cam: Camera;"
            ));
        }

        // Visual
        let mut i = 0;
        while i < V::WGSL_GLOBALS.len() {
            unwrap!(writec!(w, "{}", V::WGSL_GLOBALS[i]));
            i += 1;
        }

        // Transform
        let mut i = 0;
        while i < T::WGSL_GLOBALS.len() {
            unwrap!(writec!(w, "{}", T::WGSL_GLOBALS[i]));
            i += 1;
        }

        // Vertex
        unwrap!(writec!(w, "struct Vertex {{"));
        unwrap!(writec!(w, "@location(0) vertex_pos: vec2i,"));

        let mut i = 0;
        while i < V::WGSL_VERTEX_FIELDS.len() {
            unwrap!(writec!(
                w,
                "@location({}) {},",
                1 + i,
                V::WGSL_VERTEX_FIELDS[i]
            ));
            i += 1;
        }

        let mut i = 0;
        while i < T::WGSL_VERTEX_FIELDS.len() {
            unwrap!(writec!(
                w,
                "@location({}) {},",
                1 + V::WGSL_VERTEX_FIELDS.len() + i,
                T::WGSL_VERTEX_FIELDS[i]
            ));
            i += 1;
        }

        unwrap!(writec!(
            w,
            "@location({}) depth: f32,",
            1 + V::WGSL_VERTEX_FIELDS.len() + T::WGSL_VERTEX_FIELDS.len()
        ));

        unwrap!(writec!(w, "}}"));

        // Fragment
        unwrap!(writec!(w, "struct Fragment {{"));
        unwrap!(writec!(w, "@builtin(position) pos: vec4f,"));

        let mut i = 0;
        while i < V::WGSL_FRAGMENT_FIELDS.len() {
            unwrap!(writec!(w, "@location({i}) {},", V::WGSL_FRAGMENT_FIELDS[i]));
            i += 1;
        }

        unwrap!(writec!(w, "}}"));

        // Vertex Main
        unwrap!(writec!(
            w,
            "@vertex fn vs_main(input: Vertex) -> Fragment {{"
        ));
        unwrap!(writec!(w, "var output: Fragment;"));
        unwrap!(writec!(w, "{}", V::WGSL_VERTEX_LOGIC));
        unwrap!(writec!(w, "{}", T::WGSL_VERTEX_LOGIC));
        unwrap!(writec!(w, "{}", C::WGSL_VERTEX_LOGIC));
        unwrap!(writec!(w, "return output;"));
        unwrap!(writec!(w, "}}"));

        // Fragment Main
        unwrap!(writec!(
            w,
            "@fragment fn fs_main(input: Fragment) -> @location(0) vec4f {{"
        ));
        unwrap!(writec!(w, "{}", V::WGSL_FRAGMENT_LOGIC));
        unwrap!(writec!(w, "}}"));

        *w
    };

    const SHADER: &str = { Self::SHADER_RAW.r().as_str() };
}

#[derive(AsStd140)]
struct CameraUniform {
    center: IVec2P,
    extents: FVec2P,
}

fn slice_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * size_of::<T>()) }
}
