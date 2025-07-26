use crate::gameplay::GameplayScene;

use super::*;

pub struct Game {
    scene_runner: SceneRunner<SceneEnum>,
}

#[derive(SceneEnumType)]
pub enum SceneEnum {
    Gameplay(GameplayScene),
}

impl GameType for Game {
    const NAME: &str = "Platformer";

    fn surface_desc() -> GpuSurfaceDesc {
        GpuSurfaceDesc {
            depth_enabled: true,
        }
    }

    fn new(gpu: &Gpu) -> Self {
        Self {
            scene_runner: SceneRunner::new(SceneEnum::Gameplay(GameplayScene::new(gpu))),
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
