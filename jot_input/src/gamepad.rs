use std::mem::transmute;

use tinyset::Fits64;

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

impl ButtonCode {
    pub(crate) fn from_gilrs(button: gilrs::Button) -> Option<Self> {
        match button {
            gilrs::Button::DPadRight => Some(Self::DpadRight),
            gilrs::Button::DPadLeft => Some(Self::DpadLeft),
            gilrs::Button::DPadUp => Some(Self::DpadUp),
            gilrs::Button::DPadDown => Some(Self::DpadDown),
            gilrs::Button::LeftThumb => Some(Self::LeftThumb),
            gilrs::Button::RightThumb => Some(Self::RightThumb),
            gilrs::Button::East => Some(Self::East),
            gilrs::Button::West => Some(Self::West),
            gilrs::Button::North => Some(Self::North),
            gilrs::Button::South => Some(Self::South),
            gilrs::Button::C => None,
            gilrs::Button::Z => None,
            gilrs::Button::RightTrigger => None,
            gilrs::Button::LeftTrigger => None,
            gilrs::Button::RightTrigger2 => None,
            gilrs::Button::LeftTrigger2 => None,
            gilrs::Button::Start => Some(Self::Start),
            gilrs::Button::Select => Some(Self::Select),
            gilrs::Button::Mode => None,
            gilrs::Button::Unknown => None,
        }
    }
}

impl ValueCode {
    pub(crate) fn from_gilrs(button: gilrs::Button) -> Option<Self> {
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
            gilrs::Button::RightTrigger => Some(Self::RightTrigger1),
            gilrs::Button::LeftTrigger => Some(Self::LeftTrigger1),
            gilrs::Button::RightTrigger2 => Some(Self::RightTrigger2),
            gilrs::Button::LeftTrigger2 => Some(Self::LeftTrigger2),
            gilrs::Button::Start => None,
            gilrs::Button::Select => None,
            gilrs::Button::Mode => None,
            gilrs::Button::Unknown => None,
        }
    }

    pub(crate) fn from_gilrs_axis(axis: gilrs::Axis) -> Option<(Self, Self)> {
        match axis {
            gilrs::Axis::DPadX => None,
            gilrs::Axis::DPadY => None,
            gilrs::Axis::LeftStickX => Some((Self::LeftStickRight, Self::LeftStickLeft)),
            gilrs::Axis::LeftStickY => Some((Self::LeftStickUp, Self::LeftStickDown)),
            gilrs::Axis::RightStickX => Some((Self::RightStickRight, Self::RightStickLeft)),
            gilrs::Axis::RightStickY => Some((Self::RightStickUp, Self::RightStickDown)),
            gilrs::Axis::LeftZ => None,
            gilrs::Axis::RightZ => None,
            gilrs::Axis::Unknown => None,
        }
    }
}

impl Fits64 for ButtonCode {
    unsafe fn from_u64(x: u64) -> Self {
        unsafe { transmute::<u16, Self>(x as _) }
    }

    fn to_u64(self) -> u64 {
        unsafe { transmute::<Self, u16>(self) as _ }
    }
}

impl Fits64 for ValueCode {
    unsafe fn from_u64(x: u64) -> Self {
        unsafe { transmute::<u16, Self>(x as _) }
    }

    fn to_u64(self) -> u64 {
        unsafe { transmute::<Self, u16>(self) as _ }
    }
}
