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

    fn surface_desc() -> GpuSurfaceDesc {
        Default::default()
    }

    fn new(_gpu: &Gpu) -> Self;

    fn update(&mut self, _delta_time: f64, _gpu: &Gpu) -> GameFlow {
        GameFlow::Continue
    }

    fn event(&mut self, event: &GameEvent, _gpu: &Gpu) -> GameFlow {
        event.into()
    }

    fn draw(&self, output: &GpuTexture<2>, gpu: &Gpu) {
        output.clear(Some(vec4!(0.0, 0.0, 0.0, 0.0)), None, gpu);
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
