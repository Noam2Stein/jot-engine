use gilrs::Gilrs;
use jot_math::*;

pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub use gamepad::*;
pub use keyboard::*;
pub use mouse::*;

pub use winit::event::ElementState as ButtonState;
use winit::event::{MouseScrollDelta, WindowEvent};

/// A gloal input event.
/// Not tied to a specific device.
pub struct InputEvent {
    pub device_id: InputDeviceId,
    pub event: InputDeviceEvent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputDeviceEvent {
    Connect,
    Disconnect,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Gamepad(GamepadEvent),
}

/// An input event tied to a specific device.
/// Doesn't store the `DeviceId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceId {
    Winit(winit::event::DeviceId),
    Gilrs(gilrs::GamepadId),
}

pub struct InputProvider {
    gilrs: Gilrs,
}

impl InputProvider {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
        }
    }

    pub fn map_events<E: TryInto<InputEvent>>(
        &mut self,
        events: impl IntoIterator<Item = E>,
    ) -> impl Iterator<Item = InputEvent> {
        events.into_iter().filter_map(|event| event.try_into().ok())
    }

    pub fn poll_events(&mut self) -> impl Iterator<Item = InputEvent> {
        std::iter::from_fn(|| {
            self.gilrs
                .next_event()
                .map::<Box<[InputEvent]>, _>(|event| {
                    macro_rules! events {
                        [$($event:expr), * $(,)?] => {
                            Box::new([$(
                                InputEvent {
                                    device_id: InputDeviceId::Gilrs(event.id),
                                    event: $event,
                                },
                            )*])
                        };
                    }

                    match event.event {
                        gilrs::EventType::Connected => events![InputDeviceEvent::Connect],
                        gilrs::EventType::Disconnected => events![InputDeviceEvent::Disconnect],
                        gilrs::EventType::ButtonPressed(button, _code) => {
                            if let Some(button_code) = ButtonCode::from_gilrs(button) {
                                events![InputDeviceEvent::Gamepad(GamepadEvent::Button {
                                    button: button_code,
                                    state: ButtonState::Pressed
                                })]
                            } else {
                                events![]
                            }
                        }
                        gilrs::EventType::ButtonReleased(button, _code) => {
                            if let Some(button_code) = ButtonCode::from_gilrs(button) {
                                events![InputDeviceEvent::Gamepad(GamepadEvent::Button {
                                    button: button_code,
                                    state: ButtonState::Released
                                })]
                            } else {
                                events![]
                            }
                        }
                        gilrs::EventType::ButtonChanged(button, value, _code) => {
                            if let Some(value_code) = ValueCode::from_gilrs(button) {
                                events![InputDeviceEvent::Gamepad(GamepadEvent::Value {
                                    code: value_code,
                                    value: (value * 16.0) as u8,
                                })]
                            } else {
                                events![]
                            }
                        }
                        gilrs::EventType::AxisChanged(axis, value, _code) => {
                            if let Some((positive_value_code, negative_value_code)) =
                                ValueCode::from_gilrs_axis(axis)
                            {
                                events![
                                    InputDeviceEvent::Gamepad(GamepadEvent::Value {
                                        code: positive_value_code,
                                        value: (value * 16.0).max(0.0) as u8,
                                    }),
                                    InputDeviceEvent::Gamepad(GamepadEvent::Value {
                                        code: negative_value_code,
                                        value: (value * -16.0).max(0.0) as u8,
                                    })
                                ]
                            } else {
                                events![]
                            }
                        }
                        gilrs::EventType::ButtonRepeated(_, _) => events![],
                        gilrs::EventType::Dropped => events![],
                        _ => events![],
                    }
                })
        })
        .flatten()
    }
}

impl TryFrom<&WindowEvent> for InputEvent {
    type Error = ();

    fn try_from(value: &WindowEvent) -> Result<Self, Self::Error> {
        match value {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic: _,
            } => Ok(InputEvent {
                device_id: InputDeviceId::Winit(*device_id),
                event: InputDeviceEvent::Key(event.clone()),
            }),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => Ok(InputEvent {
                device_id: InputDeviceId::Winit(*device_id),
                event: InputDeviceEvent::Mouse(MouseEvent::Button {
                    button: *button,
                    state: *state,
                }),
            }),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase: _,
            } => Ok(InputEvent {
                device_id: InputDeviceId::Winit(*device_id),
                event: InputDeviceEvent::Mouse(MouseEvent::Scroll(match delta {
                    MouseScrollDelta::LineDelta(right, down) => vec2!(*right, -*down),
                    MouseScrollDelta::PixelDelta(delta) => vec2!(delta.x as f32, -delta.y as f32),
                })),
            }),
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => Ok(InputEvent {
                device_id: InputDeviceId::Winit(*device_id),
                event: InputDeviceEvent::Mouse(MouseEvent::Move(vec2!(
                    position.x as f32,
                    position.y as f32,
                ))),
            }),
            WindowEvent::CursorLeft { device_id } => Ok(InputEvent {
                device_id: InputDeviceId::Winit(*device_id),
                event: InputDeviceEvent::Mouse(MouseEvent::MoveOut),
            }),
            _ => Err(()),
        }
    }
}
