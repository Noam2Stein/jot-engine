use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Move(FVec2),
    MoveOut,
    Button {
        button: MouseButton,
        state: ButtonState,
    },
    Scroll(FVec2),
}

pub use winit::event::MouseButton;
