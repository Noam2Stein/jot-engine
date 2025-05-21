use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Move(Vec2),
    MoveOut,
    Button {
        button: MouseButton,
        state: ButtonState,
    },
    Scroll(Vec2),
}

pub use winit::event::MouseButton;
