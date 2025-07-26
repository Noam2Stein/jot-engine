use jot::{game::*, gpu::*, math::*, renderer2d::*};

#[allow(unused)]
mod assets;

fn main() {
    run::<FlappyBird>();
}

struct FlappyBird {
    renderer: Renderer2D<1, Sprite, SVec2P, PosCamera2D>,
}

impl GameType for FlappyBird {
    const NAME: &str = "Flappy Bird";

    fn surface_desc() -> jot::gpu::GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &jot::gpu::Gpu) -> Self {
        Self {
            renderer: Renderer2D::new(
                gpu,
                SpriteResources {
                    texture: assets::bird.load(gpu).get().clone(),
                    pixels_per_unit: 16.0,
                },
                (),
            ),
        }
    }

    fn draw(&mut self, output: &jot::gpu::GpuTexture<2>, gpu: &jot::gpu::Gpu) {
        self.renderer.render(
            RenderInput2D {
                cam: PosCamera2D {
                    center: splat2(s32::ZERO),
                    ortho_size: 8.0,
                },
                quads: &[Quad2D {
                    depth: 0.0,
                    transform: vec2p!(s32::ZERO, s32::ZERO),
                    visual: Sprite {
                        texture_rect: Rectangle::from_min_size(vec2p!(0, 0), vec2p!(16, 16)),
                    },
                }],
                background_color: vec4!(0.0, 0.5, 0.8, 0.0),
            },
            output,
            gpu,
        );
    }
}
