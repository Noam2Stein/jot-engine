use jot_gpu::*;
use jot_input::*;
use jot_math::*;
use jot_window::*;

mod event;
mod run_;
pub use event::*;
pub use run_::*;

pub trait Game {
    const NAME: &str;

    fn new(_gpu: &GPU) -> Self;

    fn update(&mut self, _delta_time: f64, _gpu: &GPU) -> GameFlow {
        GameFlow::Continue
    }

    fn event(&mut self, event: &GameEvent, _gpu: &GPU) -> GameFlow {
        event.into()
    }

    fn draw(&self, output: &GPUTextureView, gpu: &GPU) {
        output.clear(fvec4(0.0, 0.0, 0.0, 0.0), gpu);
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameFlow {
    Continue,
    Exit,
}

impl From<&GameEvent> for GameFlow {
    fn from(value: &GameEvent) -> Self {
        match value {
            GameEvent::ExitRequested => Self::Exit,
            _ => Self::Continue,
        }
    }
}
