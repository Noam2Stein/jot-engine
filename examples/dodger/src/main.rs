use jot::{
    camera::*, collections::*, ecs::*, fixed::*, game::*, gpu::*, input::*, math::*, renderer2d::*,
    scheme::*,
};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn main() {
    run::<Dodger>();
}

const FPS: u32 = 60;
const TIME_STEP: s32 = s32::int(1).div(s32::int(FPS as i32));

struct Dodger {
    renderer: Renderer2D<100, Colored, Pos2D, Pos2Camera>,
    input: Resolver<PlayerInput>,
    ecs: Ecs,
    schedule: Schedule,
    fixed_time: FixedTime<FPS>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, InputType, Resource)]
struct PlayerInput {
    pub x: Axis,
}

#[derive(Component)]
struct Body {
    rect: SRect2C,
    velocity: SVec2,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Fall;

#[derive(Component)]
struct Kill;

#[derive(Component)]
struct FallDespawn;

#[derive(Resource)]
struct Spawner {
    wait: u32,
    rand: ChaCha20Rng,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Resource)]
enum DodgerFlow {
    #[default]
    Continue,
    Restart,
}

impl GameType for Dodger {
    const NAME: &str = "Empty Game";

    fn surface_desc() -> GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &Gpu) -> Self {
        Self {
            ecs: new_ecs(),
            schedule: new_schedule(),

            fixed_time: FixedTime::new(),

            renderer: Renderer2D::new(gpu, (), ()),

            input: Resolver::new(Bindings::<PlayerInput> {
                x: Bindings::<Axis> {
                    positive: Bindings::<Value> {
                        values: Set64::from_iter([
                            ValueCode::LeftStickRight,
                            ValueCode::RightStickRight,
                        ]),
                        flat: Bindings::<Button> {
                            buttons: Set64::from_iter([ButtonCode::DpadRight]),
                            keys: Set64::from_iter([KeyCode::ArrowRight, KeyCode::KeyD]),
                            ..Default::default()
                        },
                    },

                    negative: Bindings::<Value> {
                        values: Set64::from_iter([
                            ValueCode::LeftStickLeft,
                            ValueCode::RightStickLeft,
                        ]),
                        flat: Bindings::<Button> {
                            buttons: Set64::from_iter([ButtonCode::DpadLeft]),
                            keys: Set64::from_iter([KeyCode::ArrowLeft, KeyCode::KeyA]),
                            ..Default::default()
                        },
                    },
                },
            }),
        }
    }

    fn update(&mut self, delta_time: f64, _gpu: &Gpu) -> GameFlow {
        self.fixed_time.update(delta_time, || {
            self.ecs.insert_resource(self.input.step());
            self.ecs.insert_resource(DodgerFlow::Continue);

            self.schedule.run(&mut self.ecs);

            match self.ecs.get_resource::<DodgerFlow>().unwrap() {
                DodgerFlow::Continue => {}
                DodgerFlow::Restart => {
                    self.ecs = new_ecs();
                    self.schedule = new_schedule();
                }
            };

            GameFlow::Continue
        })
    }

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> GameFlow {
        match event {
            GameEvent::Input(InputEvent {
                device_id: _,
                event,
            }) => self.input.event(event),

            _ => {}
        }

        event.into()
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        self.renderer.render(
            RenderInput2D {
                cam_bind_group: Pos2Camera {
                    center: SVec2::ZERO,
                    ortho_size: 8.0,
                },
                quads: self
                    .ecs
                    .get_resource_mut::<HandleVec<_>>()
                    .unwrap()
                    .as_slice(),
                background_color: Some(splat4(0.0)),
            },
            output,
            gpu,
        );
    }
}

fn update_quads(
    query: Query<(&Body, &mut Handle<Quad2D<Colored, Pos2D>>)>,
    quads: Res<HandleVec<Quad2D<Colored, Pos2D>>>,
) {
    for (body, mut quad_handle) in query {
        quads.get_mut(&mut quad_handle).transform = Pos2D {
            pos: body.rect.center().to_storage(),
        };

        quads.get_mut(&mut quad_handle).visual.size =
            body.rect.size().map(s32::as_f32).to_storage();
    }
}

fn update_bodies(bodies: Query<&mut Body>) {
    for mut body in bodies {
        let delta = body.velocity * TIME_STEP;
        body.rect.move_(delta);
    }
}

fn update_player(players: Query<&mut Body, With<Player>>, input: Res<PlayerInput>) {
    for mut body in players {
        body.velocity = vec2!(input.x.as_s32() * s32::int(15), s32::int(0));
    }
}

fn update_fall(query: Query<&mut Body, With<Fall>>) {
    for mut body in query {
        body.velocity -= vec2!(s32::ZERO, s32::int(10) * TIME_STEP);
    }
}

fn update_kill(
    players: Query<&Body, With<Player>>,
    killers: Query<&Body, With<Kill>>,
    mut flow: ResMut<DodgerFlow>,
) {
    for killer in killers {
        for player in players {
            if killer.rect.intersects(player.rect) {
                *flow = DodgerFlow::Restart;
            }
        }
    }
}

fn update_spawner(
    mut spawner: ResMut<Spawner>,
    mut quads: ResMut<HandleVec<Quad2D<Colored, Pos2D>>>,
    mut commands: Commands,
) {
    if spawner.wait == 0 {
        spawner.wait = 5;

        let x = spawner.rand.next_u32() % (8 * 2 * 2);
        let x = x as i32 - 8 * 2;

        commands.spawn((
            quads.insert(Quad2D {
                visual: Colored {
                    size: splat2p(1.0),
                    color: splat4p(0.8).with_x(1.0),
                },
                depth: 0.0,
                transform: Pos2D {
                    pos: vec2p!(s32::int(0), s32::int(-6)),
                },
            }),
            Body {
                rect: Rectangle::from_center_size(
                    vec2!(s32::int(x), s32::from_f32(8.5)),
                    splat2(s32::int(1)),
                ),
                velocity: SVec2::default(),
            },
            Fall,
            Kill,
            FallDespawn,
        ));
    } else {
        spawner.wait -= 1;
    }
}

fn update_despawn(query: Query<(Entity, &Body), With<FallDespawn>>, mut commands: Commands) {
    for (entity, body) in query {
        if body.rect.max().y() < s32::int(-8) {
            commands.entity(entity).despawn();
        }
    }
}

fn new_ecs() -> Ecs {
    let mut ecs = Ecs::new();

    ecs.insert_resource(Spawner {
        wait: 0,
        rand: ChaCha20Rng::from_seed([
            3, 78, 2, 44, 12, 122, 35, 125, 1, 75, 21, 64, 12, 64, 68, 34, 75, 24, 46, 24, 12, 47,
            24, 47, 123, 32, 62, 34, 47, 34, 67, 79,
        ]),
    });

    let mut quads = HandleVec::with_capacity(100);

    ecs.spawn((
        quads.insert(Quad2D {
            visual: Colored {
                size: splat2p(1.0),
                color: splat4p(1.0),
            },
            transform: Pos2D {
                pos: vec2p!(s32::int(0), s32::int(-6)),
            },
            depth: 0.0,
        }),
        Body {
            rect: Rectangle::from_center_size(
                vec2!(s32::int(0), s32::int(-6)),
                splat2(s32::int(1)),
            ),
            velocity: SVec2::default(),
        },
        Player,
    ));

    ecs.insert_resource(quads);

    ecs
}

fn new_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems((
        update_bodies,
        update_player.before(update_bodies),
        update_quads.after(update_bodies),
        update_fall.before(update_bodies),
        update_kill.before(update_bodies),
        update_spawner.before(update_quads),
        update_despawn,
    ));

    schedule
}
