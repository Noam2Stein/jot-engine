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

pub(crate) fn gilrs_to_button(button: gilrs::Button) -> Option<ButtonCode> {
    match button {
        gilrs::Button::DPadRight => Some(ButtonCode::DpadRight),
        gilrs::Button::DPadLeft => Some(ButtonCode::DpadLeft),
        gilrs::Button::DPadUp => Some(ButtonCode::DpadUp),
        gilrs::Button::DPadDown => Some(ButtonCode::DpadDown),
        gilrs::Button::LeftThumb => Some(ButtonCode::LeftThumb),
        gilrs::Button::RightThumb => Some(ButtonCode::RightThumb),
        gilrs::Button::East => Some(ButtonCode::East),
        gilrs::Button::West => Some(ButtonCode::West),
        gilrs::Button::North => Some(ButtonCode::North),
        gilrs::Button::South => Some(ButtonCode::South),
        gilrs::Button::C => None,
        gilrs::Button::Z => None,
        gilrs::Button::RightTrigger => None,
        gilrs::Button::LeftTrigger => None,
        gilrs::Button::RightTrigger2 => None,
        gilrs::Button::LeftTrigger2 => None,
        gilrs::Button::Start => Some(ButtonCode::Start),
        gilrs::Button::Select => Some(ButtonCode::Select),
        gilrs::Button::Mode => None,
        gilrs::Button::Unknown => None,
    }
}

pub(crate) fn gilrs_to_value(button: gilrs::Button) -> Option<ValueCode> {
    match button {
        gilrs::Button::DPadRight => None,
        gilrs::Button::DPadLeft => None,
        gilrs::Button::DPadUp => None,
        gilrs::Button::DPadDown => None,
        gilrs::Button::LeftThumb => None,
        gilrs::Button::RightThumb => None,
        gilrs::Button::East => None,
        gilrs::Button::West => None,
        gilrs::Button::North => None,
        gilrs::Button::South => None,
        gilrs::Button::C => None,
        gilrs::Button::Z => None,
        gilrs::Button::RightTrigger => Some(ValueCode::RightTrigger1),
        gilrs::Button::LeftTrigger => Some(ValueCode::LeftTrigger1),
        gilrs::Button::RightTrigger2 => Some(ValueCode::RightTrigger2),
        gilrs::Button::LeftTrigger2 => Some(ValueCode::LeftTrigger2),
        gilrs::Button::Start => None,
        gilrs::Button::Select => None,
        gilrs::Button::Mode => None,
        gilrs::Button::Unknown => None,
    }
}

pub(crate) fn gilrs_to_axis(axis: gilrs::Axis) -> Option<(ValueCode, ValueCode)> {
    match axis {
        gilrs::Axis::DPadX => None,
        gilrs::Axis::DPadY => None,
        gilrs::Axis::LeftStickX => Some((ValueCode::LeftStickRight, ValueCode::LeftStickLeft)),
        gilrs::Axis::LeftStickY => Some((ValueCode::LeftStickUp, ValueCode::LeftStickDown)),
        gilrs::Axis::RightStickX => Some((ValueCode::RightStickRight, ValueCode::RightStickLeft)),
        gilrs::Axis::RightStickY => Some((ValueCode::RightStickUp, ValueCode::RightStickDown)),
        gilrs::Axis::LeftZ => None,
        gilrs::Axis::RightZ => None,
        gilrs::Axis::Unknown => None,
    }
}
