use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GamepadEvent {
    Button {
        button: ButtonCode,
        state: ButtonState,
    },
    Value {
        code: ValueCode,
        value: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonCode {
    DpadRight,
    DpadLeft,
    DpadUp,
    DpadDown,
    LeftThumb,
    RightThumb,
    East,
    West,
    North,
    South,
    Start,
    Select,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueCode {
    LeftStickRight,
    LeftStickLeft,
    LeftStickUp,
    LeftStickDown,
    RightStickRight,
    RightStickLeft,
    RightStickUp,
    RightStickDown,
    RightTrigger1,
    LeftTrigger1,
    RightTrigger2,
    LeftTrigger2,
    Unknown(u8),
}
