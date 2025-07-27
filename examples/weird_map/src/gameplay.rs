use noise::{NoiseFn, Perlin};

use crate::{
    game::SceneEnum,
    input::Input,
    renderer::{Quad, RenderInput, Renderer},
};

use super::*;

const FPS: u32 = 60;
const TIME_STEP: s32 = s32::ONE.div(s32::from_u32(FPS));

pub struct GameplayScene {
    renderer: Renderer,
    quads: HandleVec<Quad>,

    input: Resolver<Input>,
    fixed_time: FixedTime<FPS>,
    chunks: ChunkHolder2D<Chunk, 32, 32, 3, 3>,

    cam: Shake<DirectFollow<Pos2Camera>>,

    player_pos: SVec2,
    player_quad: Handle<Quad>,
}

struct Chunk {
    _quads: Vec<Handle<Quad>>,
}

impl GameplayScene {
    pub fn new(gpu: &Gpu) -> Self {
        let mut quads = HandleVec::new();

        Self {
            input: Resolver::new(Input::default_bindings()),
            fixed_time: FixedTime::new(),
            chunks: ChunkHolder2D::new(SVec2::ZERO, &mut quads),

            cam: Shake::new(DirectFollow::new(SVec2::ZERO, 50.0)),

            player_pos: SVec2::ZERO,
            player_quad: quads.insert(Quad {
                depth: 0.0,
                visual: Colored {
                    size: FVec2P::ONE,
                    color: vec4p!(1.0, 0.0, 0.0, 1.0),
                },
                transform: SVec2P::ZERO,
            }),

            renderer: Renderer::new(gpu, (), ()),
            quads,
        }
    }
}

impl SceneType for GameplayScene {
    type SceneEnum = SceneEnum;

    fn update(&mut self, delta_time: f64, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        self.fixed_time.update(delta_time, || {
            let input = self.input.step();

            self.player_pos += vec2!(input.x.as_s32(), input.y.as_s32()) * s32::int(10) * TIME_STEP;
            self.quads.get_mut(&mut self.player_quad).transform = self.player_pos.to_storage();

            self.cam.target_moved(self.player_pos);
            self.chunks.target_moved(self.player_pos, &mut self.quads);

            if input.jump.is_triggered {
                self.cam.shake(ShakeDesc::HEAVY);
            }

            self.cam.update(TIME_STEP);

            SceneFlow::Continue
        })
    }

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
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
            RenderInput {
                quads: self.quads.as_slice(),
                cam: self.cam.inner(),
                background_color: Some(vec4!(0.1, 0.2, 0.3, 0.0)),
            },
            output,
            gpu,
        );
    }
}

impl Chunk2D for Chunk {
    type Context = HandleVec<Quad>;

    fn load(chunk_pos: IVec2, ctx: &mut Self::Context) -> Self {
        let noise = Perlin::new(9433);

        let mut quads = Vec::with_capacity(32 * 32);

        for x in 0..32 {
            for y in 0..32 {
                let pos = chunk_pos * 32 + vec2!(x, y);
                let perlin_input = [pos.x() as f64 * 0.025, pos.y() as f64 * 0.025];

                let lvl = noise.get(perlin_input) as f32 / 2.0 + 0.5;

                quads.push(ctx.insert(Quad {
                    depth: 0.0,
                    visual: Colored {
                        size: FVec2P::ONE,
                        color: splat4p(lvl),
                    },
                    transform: pos.map(s32::from_i32).to_storage(),
                }));
            }
        }

        Self { _quads: quads }
    }

    fn unload(self, _ctx: &mut Self::Context) {}
}
