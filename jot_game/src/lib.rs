use jot_context::*;
use jot_graphics::*;
use jot_input::*;
use jot_math::*;
use jot_window::*;

mod event;
mod run;
pub use event::*;
pub use run::*;

pub trait Game {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default()
    }

    fn new(_ctx: &Context) -> Self;

    fn update(&mut self, _delta_time: f64, _ctx: &Context) -> GameFlow {
        GameFlow::Continue
    }

    fn event(&mut self, event: &GameEvent, _ctx: &Context) -> GameFlow {
        event.into()
    }

    fn draw(&mut self, output: &TextureView, ctx: &Context) {
        clear(vec4(0.0, 0.0, 0.0, 0.0), output, ctx);
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
