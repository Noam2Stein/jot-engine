use super::*;

pub use jot_scene_proc_macros::SceneEnum;

pub trait SceneEnum: Scene<SceneEnum = Self> {}

pub trait Scene: Sized {
    type SceneEnum: SceneEnum;

    fn update(&mut self, _delta_time: f64, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        SceneFlow::Continue
    }

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> SceneFlow<Self::SceneEnum> {
        event.into()
    }

    fn draw(&mut self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(0.0, 0.0, 0.0, 0.0)), None, gpu);
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SceneFlow<E: SceneEnum> {
    #[default]
    Continue,
    Swap(E),
    Exit,
}

impl<E: SceneEnum> From<GameFlow> for SceneFlow<E> {
    fn from(value: GameFlow) -> Self {
        match value {
            GameFlow::Continue => Self::Continue,
            GameFlow::Exit => Self::Exit,
        }
    }
}
impl<E: SceneEnum> From<&GameEvent> for SceneFlow<E> {
    fn from(value: &GameEvent) -> Self {
        Self::from(GameFlow::from(value))
    }
}
