use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, InputType)]
pub struct Input {
    pub x: Axis,
    pub y: Axis,
    pub jump: Button,
}

impl Input {
    pub fn default_bindings() -> Bindings<Self> {
        Bindings::<Self> {
            x: (
                (KeyCode::ArrowRight, KeyCode::ArrowLeft),
                (KeyCode::KeyD, KeyCode::KeyA),
                (ValueCode::LeftStickRight, ValueCode::LeftStickLeft),
                (ButtonCode::DpadRight, ButtonCode::DpadLeft),
            )
                .into(),

            y: (
                (KeyCode::ArrowUp, KeyCode::ArrowDown),
                (KeyCode::KeyW, KeyCode::KeyS),
                (ValueCode::LeftStickUp, ValueCode::LeftStickDown),
                (ButtonCode::DpadUp, ButtonCode::DpadDown),
            )
                .into(),

            jump: (KeyCode::Space, ButtonCode::South, ButtonCode::East).into(),
        }
        .flat()
    }
}
