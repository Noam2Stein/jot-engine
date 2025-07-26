use jot::{game::*, gpu::*, input::*, math::*, scene::*};

fn main() {
    run::<MyGame>();
}

struct MyGame {
    scene_runner: SceneRunner<MySceneEnum>,
}

#[derive(SceneEnum)]
enum MySceneEnum {
    Red(RedScene),
    Blue(BlueScene),
}

struct RedScene {}

struct BlueScene {}

impl Game for MyGame {
    const NAME: &str = "Cool Scenes";

    fn new(_gpu: &Gpu) -> Self {
        Self {
            scene_runner: SceneRunner::new(MySceneEnum::Red(RedScene {})),
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

impl Scene for RedScene {
    type SceneEnum = MySceneEnum;

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
            SceneFlow::Swap(MySceneEnum::Blue(BlueScene {}))
        } else {
            event.into()
        }
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(1.0, 0.0, 0.0, 1.0)), None, gpu);
    }
}

impl Scene for BlueScene {
    type SceneEnum = MySceneEnum;

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
            SceneFlow::Swap(MySceneEnum::Red(RedScene {}))
        } else {
            event.into()
        }
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(0.0, 0.0, 1.0, 1.0)), None, gpu);
    }
}
