use jot::{camera::*, game::*, gpu::*, math::*, renderer2d::*};

#[allow(unused)]
mod assets;

fn main() {
    run::<FlappyBird>();
}

struct FlappyBird {
    quads: GpuBuffer<[Quad2D<Sprite, Pos2D>]>,
    cam_bind_group: GpuBindGroup<GpuBuffer<Std140<Pos2Camera>>>,
    renderer: Renderer2D<Sprite, Pos2D, Pos2Camera>,
    visual_bind_group: GpuBindGroup<SpriteBindings>,
    transform_bind_group: GpuBindGroup<()>,
}

impl GameType for FlappyBird {
    const NAME: &str = "Flappy Bird";

    fn surface_desc() -> jot::gpu::GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &jot::gpu::Gpu) -> Self {
        let visual_bind_group = gpu.create_bind_group(&SpriteBindings {
            texture: assets::bird.load(gpu).get().clone(),
            pixels_per_unit: gpu.create_buffer(GpuBufferDesc {
                label: None,
                usages: GpuBufferUsages::UNIFORM,
                value: &16.0,
            }),
        });

        let transform_bind_group = gpu.create_bind_group(&());

        let cam_bind_group = gpu.create_bind_group(
            &gpu.create_buffer(GpuBufferDesc {
                label: None,
                usages: GpuBufferUsages::UNIFORM,
                value: &Pos2Camera {
                    center: splat2(s32::ZERO),
                    ortho_size: 8.0,
                }
                .as_std140(),
            }),
        );

        let quads = gpu.create_buffer::<[_]>(GpuBufferDesc {
            label: None,
            usages: GpuBufferUsages::VERTEX,
            value: &[Quad2D {
                depth: 0.0,
                transform: Pos2D {
                    pos: vec2p!(s32::ZERO, s32::ZERO),
                },
                visual: Sprite {
                    texture_rect: Aabb::from_min_size(vec2p!(0, 0), vec2p!(16, 16)),
                },
            }],
        });

        Self {
            quads,
            cam_bind_group,
            visual_bind_group,
            transform_bind_group,
            renderer: Renderer2D::new(gpu, None),
        }
    }

    fn draw(&mut self, output: &jot::gpu::GpuTexture<2>, gpu: &jot::gpu::Gpu) {
        self.renderer.render(
            RenderInput2D {
                visual_bind_group: &self.visual_bind_group,
                transform_bind_group: &self.transform_bind_group,
                cam_bind_group: &self.cam_bind_group,
                quads: self.quads.slice(..),
                background_color: Some(vec4!(0.0, 0.5, 0.8, 0.0)),
            },
            output,
            gpu,
        );
    }
}
