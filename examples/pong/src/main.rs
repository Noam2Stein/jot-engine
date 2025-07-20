use jot::{game::*, gpu::*, math::*, renderer2d::*};

fn main() {
    run::<Pong>();
}

const BACKGROUND_COLOR: FVec4P = splat4p(0.0);
const LEFT_PLAYER_COLOR: FVec4P = splat4p(1.0);
const RIGHT_PLAYER_COLOR: FVec4P = splat4p(1.0);
const BALL_COLOR: FVec4P = splat4p(1.0);

const CAM_ORTHO_SIZE: s32 = s32::int(15);
const PLAYER_SIZE: SVec2 = vec2!(s32::int(1), s32::int(5));
const PLAYER_DISTANCE: s32 = s32::int(36);
const BALL_SIZE: SVec2 = vec2!(s32::int(1), s32::int(1));

const BALL_SPEED: s32 = s32::int(10);

struct Pong {
    left_player: SRect2C,
    right_player: SRect2C,
    ball: SRect2C,
    ball_velocity: SVec2,
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
                vec2!(
                    -(PLAYER_DISTANCE + PLAYER_SIZE.x()) / s32::int(2),
                    s32::ZERO
                ),
                PLAYER_SIZE,
            ),

            right_player: Rectangle::from_center_size(
                vec2!((PLAYER_DISTANCE + PLAYER_SIZE.x()) / s32::int(2), s32::ZERO),
                PLAYER_SIZE,
            ),

            ball: Rectangle::from_center_size(SVec2::ZERO, BALL_SIZE),
            ball_velocity: vec2!(BALL_SPEED, BALL_SPEED),

            renderer: Renderer2D::new(gpu),
        }
    }

    fn update(&mut self, delta_time: f64, _gpu: &Gpu) -> GameFlow {
        self.update_ball(delta_time);

        GameFlow::Continue
    }

    fn draw(&self, output: &GpuTexture<2>, gpu: &Gpu) {
        let left_player_quad = Quad {
            rect: self.left_player.to_storage(),
            depth: 0.0,
            color: LEFT_PLAYER_COLOR,
        };

        let right_player_quad = Quad {
            rect: self.right_player.to_storage(),
            depth: 0.0,
            color: RIGHT_PLAYER_COLOR,
        };

        let ball_quad = Quad {
            rect: self.ball.to_storage(),
            depth: 0.0,
            color: BALL_COLOR,
        };

        self.renderer.render(
            RenderInput2D {
                cam: Camera2D {
                    center: SVec2::ZERO,
                    ortho_size: CAM_ORTHO_SIZE.as_f32(),
                    background_color: BACKGROUND_COLOR.to_storage(),
                },
                quads: &[left_player_quad, right_player_quad, ball_quad],
            },
            output,
            gpu,
        );
    }
}

impl Pong {
    fn update_ball(&mut self, delta_time: f64) {
        self.ball
            .move_((self.ball_velocity.map(s32::as_f64) * delta_time).map(s32::from_f64));

        if self.ball.intersects(self.right_player) {
            self.ball_velocity.set_x(-self.ball_velocity.x().abs());
        } else if self.ball.intersects(self.left_player) {
            self.ball_velocity.set_x(self.ball_velocity.x().abs());
        }

        if self.ball.max().y() >= CAM_ORTHO_SIZE {
            self.ball_velocity.set_y(-self.ball_velocity.y().abs());
        } else if self.ball.min().y() <= -CAM_ORTHO_SIZE {
            self.ball_velocity.set_y(self.ball_velocity.y().abs());
        }

        if self.ball.center().x().abs()
            >= (PLAYER_DISTANCE + BALL_SIZE.x()) / s32::int(2) + s32::int(4)
        {
            self.ball = Rectangle::from_center_size(SVec2::ZERO, BALL_SIZE);
            self.ball_velocity = vec2!(BALL_SPEED, BALL_SPEED);
        }
    }
}
