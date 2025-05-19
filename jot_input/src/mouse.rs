use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Move(Vec2),
    MoveOut,
    Button {
        button: MouseButton,
        state: ButtonState,
    },
    ScrollX(f32),
    ScrollY(f32),
}

pub use winit::event::MouseButton;
