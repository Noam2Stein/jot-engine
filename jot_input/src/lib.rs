use jot_math::*;

pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub use gamepad::*;
pub use keyboard::*;
pub use mouse::*;

pub use winit::event::ElementState as ButtonState;

#[derive(Debug, Clone, PartialEq)]
pub enum InputDeviceEvent {
    Connect,
    Disconnect,
    Keyboard(KeyEvent),
    Gamepad(GamepadEvent),
    Mouse(MouseEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceId {
    Winit(winit::event::DeviceId),
    Gilrs(gilrs::GamepadId),
}
