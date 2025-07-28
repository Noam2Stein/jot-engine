use jot::{game::*, gpu::*, input::*, math::*, scene::*};

fn main() {
    run::<Game>();
}

struct Game {
    scene_runner: SceneRunner<SceneEnum>,
}

#[derive(SceneEnumType)]
enum SceneEnum {
    Red(RedScene),
    Blue(BlueScene),
}

struct RedScene {}

struct BlueScene {}

impl GameType for Game {
    const NAME: &str = "Cool Scenes";

    fn new(_gpu: &Gpu) -> Self {
        Self {
            scene_runner: SceneRunner::new(SceneEnum::Red(RedScene {})),
        }
    }

    fn update(&mut self, delta_time: f64, gpu: &Gpu) -> GameFlow {
        self.scene_runner.update(delta_time, gpu)
    }

    fn event(&mut self, event: &GameEvent, gpu: &Gpu) -> GameFlow {
        self.scene_runner.event(event, gpu)
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        self.scene_runner.draw(output, gpu);
    }
}

impl SceneType for RedScene {
    type SceneEnum = SceneEnum;

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        if let GameEvent::Input(InputEvent {
            device_id: _,
            event:
                InputDeviceEvent::Key(KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    state: ButtonState::Pressed,
                    ..
                }),
        }) = event
        {
            SceneFlow::Swap(SceneEnum::Blue(BlueScene {}))
        } else {
            event.into()
        }
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(1.0, 0.0, 0.0, 1.0)), None, gpu);
    }
}

impl SceneType for BlueScene {
    type SceneEnum = SceneEnum;

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        if let GameEvent::Input(InputEvent {
            device_id: _,
            event:
                InputDeviceEvent::Key(KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    state: ButtonState::Pressed,
                    ..
                }),
        }) = event
        {
            SceneFlow::Swap(SceneEnum::Red(RedScene {}))
        } else {
            event.into()
        }
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(0.0, 0.0, 1.0, 1.0)), None, gpu);
    }
}
