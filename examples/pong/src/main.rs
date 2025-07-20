use jot::{game::*, gpu::*, math::*, renderer2d::*};

fn main() {
    run::<Pong>();
}

struct Pong {
    time: f64,
    renderer: Renderer2D<50>,
}

impl Game for Pong {
    const NAME: &str = "Pong";

    fn surface_desc() -> GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &Gpu) -> Self {
        Self {
            time: 0.0,
            renderer: Renderer2D::new(gpu),
        }
    }

    fn update(&mut self, _delta_time: f64, _gpu: &Gpu) -> GameFlow {
        self.time += _delta_time;

        GameFlow::Continue
    }

    fn draw(&self, output: &GpuTexture<2>, gpu: &Gpu) {
        self.renderer.render(
            RenderInput2D {
                cam: Camera2D {
                    center: SVec2::ZERO,
                    ortho_size: 8.0,
                    background_color: splat4(0.0),
                },
                quads: &[Quad {
                    rect: Rectangle::from_center_size(SVec2::ZERO, SVec2::ONE),
                    depth: 0.0,
                    color: vec4p!(1.0, 0.0, 0.0, 1.0),
                }],
            },
            output,
            gpu,
        );
    }
}
