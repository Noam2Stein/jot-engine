use jot::{
    camera::*, collections::*, fixed::*, game::*, gpu::*, input::*, math::*, renderer2d::*,
    scheme::*,
};

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

const FPS: u32 = 120;
const TIME_STEP: s32 = s32::int(1).div(s32::int(FPS as i32));

const BALL_SPEED: s32 = s32::int(10);
const PLAYER_SPEED: s32 = s32::int(30);

struct Pong {
    fixed_time: FixedTime<FPS>,
    renderer: Renderer2D<3, Colored, SVec2P, Pos2Camera>,

    state: PongState,
    input: Resolver<PongInput>,
}

struct PongState {
    left_player: SRect2C,

    right_player: SRect2C,

    ball: SRect2C,
    ball_velocity: SVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, InputType)]
struct PongInput {
    pub left: Axis,
    pub right: Axis,
}

impl GameType for Pong {
    const NAME: &str = "Pong";

    fn surface_desc() -> GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &Gpu) -> Self {
        Self {
            state: PongState {
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
            },

            fixed_time: FixedTime::new(),
            renderer: Renderer2D::new(gpu, (), ()),

            input: Resolver::new(Bindings::<PongInput> {
                left: Bindings::<Axis> {
                    positive: Bindings::<Value> {
                        flat: Bindings::<Button> {
                            keys: Set64::from_iter([KeyCode::KeyW]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    negative: Bindings::<Value> {
                        flat: Bindings::<Button> {
                            keys: Set64::from_iter([KeyCode::KeyS]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },

                right: Bindings::<Axis> {
                    positive: Bindings::<Value> {
                        flat: Bindings::<Button> {
                            keys: Set64::from_iter([KeyCode::ArrowUp]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    negative: Bindings::<Value> {
                        flat: Bindings::<Button> {
                            keys: Set64::from_iter([KeyCode::ArrowDown]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            }),
        }
    }

    fn update(&mut self, delta_time: f64, _gpu: &Gpu) -> GameFlow {
        self.fixed_time
            .update(delta_time, || self.state.fixed_update(self.input.step()))
    }

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> GameFlow {
        match event {
            GameEvent::Input(InputEvent {
                device_id: _,
                event,
            }) => {
                self.input.event(event);
            }

            _ => {}
        }

        GameFlow::from(event)
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        self.renderer.render(
            RenderInput2D {
                cam: Pos2Camera {
                    center: SVec2::ZERO,
                    ortho_size: CAM_ORTHO_SIZE.as_f32(),
                },
                quads: &self.state.renderer_quads(),
                background_color: BACKGROUND_COLOR.to_storage(),
            },
            output,
            gpu,
        );
    }
}

impl PongState {
    fn fixed_update(&mut self, input: PongInput) -> GameFlow {
        self.update_players(input);
        self.update_ball();

        GameFlow::Continue
    }

    fn update_players(&mut self, input: PongInput) {
        self.left_player
            .move_y(input.left.as_s32() * PLAYER_SPEED * TIME_STEP);

        self.right_player
            .move_y(input.right.as_s32() * PLAYER_SPEED * TIME_STEP);
    }

    fn update_ball(&mut self) {
        self.ball.move_(self.ball_velocity * TIME_STEP);

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

    fn renderer_quads(&self) -> [Quad2D<Colored, SVec2P>; 3] {
        let left_player_quad = Quad2D {
            transform: self.left_player.center().to_storage(),
            visual: Colored {
                size: self.left_player.size().map(s32::as_f32).to_storage(),
                color: LEFT_PLAYER_COLOR,
            },
            depth: 0.0,
        };

        let right_player_quad = Quad2D {
            transform: self.right_player.center().to_storage(),
            visual: Colored {
                size: self.right_player.size().map(s32::as_f32).to_storage(),
                color: RIGHT_PLAYER_COLOR,
            },
            depth: 0.0,
        };

        let ball_quad = Quad2D {
            transform: self.ball.center().to_storage(),
            visual: Colored {
                size: self.ball.size().map(s32::as_f32).to_storage(),
                color: BALL_COLOR,
            },
            depth: 0.0,
        };

        [left_player_quad, right_player_quad, ball_quad]
    }
}
