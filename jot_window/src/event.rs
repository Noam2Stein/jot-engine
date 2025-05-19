use winit::event::WindowEvent;

use crate::*;

#[derive(Debug, Clone)]
pub enum Event {
    ExitRequested,
    ViewResized(UVec2),
    ViewMinimized,
    Focused(bool),
    InputDevice {
        event: InputDeviceEvent,
        device: InputDeviceId,
    },
    Modifiers(ModifiersState),
}

impl Event {
    pub fn from_winit(event: &WindowEvent) -> Option<Self> {
        match event {
            WindowEvent::CloseRequested => Some(Self::ExitRequested),
            WindowEvent::Resized(size) => Some(if size.width > 0 && size.height > 0 {
                Self::ViewResized(uvec2(size.width, size.height))
            } else {
                Self::ViewMinimized
            }),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic: _,
            } => Some(Self::InputDevice {
                event: InputDeviceEvent::Keyboard(event.clone()),
                device: InputDeviceId::Winit(*device_id),
            }),
            WindowEvent::ModifiersChanged(event) => Some(Self::Modifiers(event.state())),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => Some(Self::InputDevice {
                event: InputDeviceEvent::Mouse(MouseEvent::Button {
                    button: *button,
                    state: *state,
                }),
                device: InputDeviceId::Winit(*device_id),
            }),
            WindowEvent::Focused(is_focused) => Some(Event::Focused(*is_focused)),
            _ => None,
        }
    }
}
