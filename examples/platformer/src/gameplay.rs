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
    input: Resolver<Input>,
    fixed_time: FixedTime<FPS>,

    cam: Shake<DirectFollow<Pos2Camera>>,

    player_pos: SVec2,
    quads: Vec<Quad>,
}

impl GameplayScene {
    pub fn new(gpu: &Gpu) -> Self {
        Self {
            renderer: Renderer::new(gpu, (), ()),
            input: Resolver::new(Input::default_bindings()),
            fixed_time: FixedTime::new(),

            cam: Shake::new(DirectFollow::new(SVec2::ZERO, 10.0)),

            player_pos: SVec2::ZERO,
            quads: {
                // Assumes Quad, Colored, FVec2P, vec4p!, vec2p!, s32::int in scope

                let mut quads = Vec::with_capacity(2000);

                for i in 0..2000 {
                    // 50 columns, 40 rows (same grid count as before)
                    let cols = 50;
                    let row = i / cols;
                    let col = i % cols;

                    // Spaced 5 units apart (4× denser than 20 units)
                    // Using same slight offset to avoid perfect grid
                    let x = col as i32 * 5 - (cols as i32 * 5 / 2) + (row % 3 * 2);
                    let y = row as i32 * 5 - (40 * 5 / 2) + (col % 3 * 2);

                    quads.push(Quad {
                        depth: 0.1,
                        visual: Colored {
                            size: FVec2P::ONE,
                            color: vec4p!(1.0, 0.8, 0.6, 1.0),
                        },
                        transform: vec2p!(s32::int(x), s32::int(y)),
                    });
                }

                quads
            },
        }
    }
}

impl SceneType for GameplayScene {
    type SceneEnum = SceneEnum;

    fn update(&mut self, delta_time: f64, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        self.fixed_time.update(delta_time, || {
            let input = self.input.step();

            self.player_pos += vec2!(input.x.as_s32(), input.y.as_s32()) * s32::int(10) * TIME_STEP;
            self.cam.target_moved(self.player_pos);

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
        self.quads[0] = Quad {
            depth: 0.0,
            visual: Colored {
                size: FVec2P::ONE,
                color: splat4p(1.0),
            },
            transform: self.player_pos.to_storage(),
        };

        self.renderer.render(
            RenderInput {
                quads: &self.quads,
                cam_bind_group: self.cam.inner(),
                background_color: vec4!(0.1, 0.2, 0.3, 0.0),
            },
            output,
            gpu,
        );
    }
}
