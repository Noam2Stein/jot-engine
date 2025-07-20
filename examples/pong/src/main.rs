use jot::{game::*, gpu::*, math::*, renderer2d::*};

fn main() {
    run::<Pong>();
}

struct Pong {
    left_player: SRect2C,
    right_player: SRect2C,
    ball: SRect2C,
    renderer: Renderer2D<3>,
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
            left_player: Rectangle::from_center_size(
                vec2!(s32::int(-18), s32::int(0)),
                vec2!(s32::int(1), s32::int(5)),
            ),

            right_player: Rectangle::from_center_size(
                vec2!(s32::int(18), s32::int(0)),
                vec2!(s32::int(1), s32::int(5)),
            ),

            ball: Rectangle::from_center_size(
                vec2!(s32::int(0), s32::int(0)),
                vec2!(s32::int(1), s32::int(1)),
            ),

            renderer: Renderer2D::new(gpu),
        }
    }

    fn update(&mut self, _delta_time: f64, _gpu: &Gpu) -> GameFlow {
        GameFlow::Continue
    }

    fn draw(&self, output: &GpuTexture<2>, gpu: &Gpu) {
        let left_player_quad = Quad {
            rect: self.left_player.to_storage(),
            depth: 0.0,
            color: vec4p!(1.0, 1.0, 1.0, 1.0),
        };

        let right_player_quad = Quad {
            rect: self.right_player.to_storage(),
            depth: 0.0,
            color: vec4p!(1.0, 1.0, 1.0, 1.0),
        };

        let ball_quad = Quad {
            rect: self.ball.to_storage(),
            depth: 0.0,
            color: vec4p!(1.0, 1.0, 1.0, 1.0),
        };

        self.renderer.render(
            RenderInput2D {
                cam: Camera2D {
                    center: SVec2::ZERO,
                    ortho_size: 15.0,
                    background_color: splat4(0.0),
                },
                quads: &[left_player_quad, right_player_quad, ball_quad],
            },
            output,
            gpu,
        );
    }
}
