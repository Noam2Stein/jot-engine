use super::*;

use derive_where::derive_where;
pub use jot_scene_proc_macros::SceneEnumType;

pub trait SceneEnumType: SceneType<SceneEnum = Self> {}

pub trait SceneType: Sized {
    type SceneEnum: SceneEnumType;

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
#[derive(Debug, Clone, Copy, Eq, Hash)]
#[derive_where(PartialEq, Default)]
pub enum SceneFlow<E: SceneEnumType> {
    #[derive_where(default)]
    Continue,
    #[derive_where(skip_inner)]
    Swap(E),
    Exit,
}

impl<E: SceneEnumType> From<GameFlow> for SceneFlow<E> {
    fn from(value: GameFlow) -> Self {
        match value {
            GameFlow::Continue => Self::Continue,
            GameFlow::Exit => Self::Exit,
        }
    }
}
impl<E: SceneEnumType> From<&GameEvent> for SceneFlow<E> {
    fn from(value: &GameEvent) -> Self {
        Self::from(GameFlow::from(value))
    }
}
