use jot_gpu::*;
use jot_input::*;
use jot_math::*;
use jot_window::*;

mod event;
mod run_;
pub use event::*;
pub use run_::*;

pub trait Game {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default().with_title("Jot Game Window")
    }

    fn new(_ctx: &GPUContext) -> Self;

    fn update(&mut self, _delta_time: f64, _ctx: &GPUContext) -> GameFlow {
        GameFlow::Continue
    }

    fn event(&mut self, event: &GameEvent, _ctx: &GPUContext) -> GameFlow {
        event.into()
    }

    fn draw(&mut self, output: &GPUTextureView, ctx: &GPUContext) {
        clear(fvec4(0.0, 0.0, 0.0, 0.0), output, ctx);
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
