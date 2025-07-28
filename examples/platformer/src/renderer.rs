use super::*;

pub struct Renderer {
    pub cam_buf: GpuBuffer<Std140<Camera>>,
    pub cam_bind_group: GpuBindGroup<GpuBuffer<Std140<Camera>>>,

    pub obj_buf: GpuBuffer<[Quad]>,

    pub visual_bind_group: GpuBindGroup<()>,
    pub transform_bind_group: GpuBindGroup<()>,
    pub renderer: Renderer2D<Colored, Pos2D, Pos2Camera>,
}

pub type Camera = Pos2Camera;
pub type Quad = Quad2D<Colored, Pos2D>;

#[derive(Debug, Clone, Copy)]
pub struct RenderInput<'a> {
    pub cam: Camera,
    pub background_color: FVec4,

    pub objs: &'a [Quad],
    pub tilemaps: &'a [&'a Tilemap<10, Colored, Pos2D>],
}

impl Renderer {
    pub fn new(gpu: &Gpu) -> Self {
        let cam_buf = gpu.create_buffer_uninit(GpuBufferUninitDesc {
            label: Some("Renderer Camera"),
            usages: GpuBufferUsages::UNIFORM | GpuBufferUsages::COPY_DST,
        });

        let cam_bind_group = gpu.create_bind_group(&cam_buf);

        let obj_buf = gpu.create_buffer_uninit_slice(GpuBufferUninitSliceDesc {
            label: Some("Renderer Objects"),
            usages: GpuBufferUsages::VERTEX | GpuBufferUsages::COPY_DST,
            len: 10,
        });

        let visual_bind_group = gpu.create_bind_group(&());
        let transform_bind_group = gpu.create_bind_group(&());

        let renderer = Renderer2D::new(gpu, None);

        Self {
            cam_buf,
            cam_bind_group,
            obj_buf,
            visual_bind_group,
            transform_bind_group,
            renderer,
        }
    }

    pub fn render(&self, input: RenderInput, output: &GpuTexture<2>, gpu: &Gpu) {
        let mut background_color = Some(input.background_color);

        self.cam_buf.set(&input.cam.as_std140(), gpu);

        for obj_batch in input.objs.chunks(self.obj_buf.len()) {
            self.obj_buf.set_range(0, obj_batch, gpu);

            self.renderer.render(
                RenderInput2D {
                    cam_bind_group: &self.cam_bind_group,
                    background_color,
                    quads: self.obj_buf.slice(0..obj_batch.len()),
                    visual_bind_group: &self.visual_bind_group,
                    transform_bind_group: &self.transform_bind_group,
                },
                output,
                gpu,
            );

            background_color = None;
        }

        for tilemap in input.tilemaps {
            tilemap.render(
                &TilemapRenderInput {
                    cam: input.cam,
                    cam_bind_group: &self.cam_bind_group,
                    background_color,
                    visual_bind_group: &self.visual_bind_group,
                    transform_bind_group: &self.transform_bind_group,
                },
                output,
                &self.renderer,
                gpu,
            );

            background_color = None;
        }
    }
}
