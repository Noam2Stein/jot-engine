use super::*;

pub struct SceneRunner<E: SceneEnumType> {
    scene_enum: E,
}

impl<E: SceneEnumType> SceneRunner<E> {
    pub fn new(scene: E) -> Self {
        Self { scene_enum: scene }
    }

    pub fn update(&mut self, delta_time: f64, gpu: &Gpu) -> GameFlow {
        match self.scene_enum.update(delta_time, gpu) {
            SceneFlow::Continue => GameFlow::Continue,
            SceneFlow::Exit => GameFlow::Exit,

            SceneFlow::Swap(new_scene) => {
                self.scene_enum = new_scene;

                GameFlow::Continue
            }
        }
    }

    pub fn event(&mut self, event: &GameEvent, gpu: &Gpu) -> GameFlow {
        match self.scene_enum.event(event, gpu) {
            SceneFlow::Continue => GameFlow::Continue,
            SceneFlow::Exit => GameFlow::Exit,

            SceneFlow::Swap(new_scene) => {
                self.scene_enum = new_scene;

                GameFlow::Continue
            }
        }
    }

    pub fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        self.scene_enum.draw(output, gpu);
    }
}
