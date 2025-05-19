use winit::window::Fullscreen;

use crate::*;

pub struct FullscreenSwitch {
    alt_is_held: bool,
}

impl FullscreenSwitch {
    pub fn new() -> Self {
        Self { alt_is_held: false }
    }

    pub fn event(&mut self, event: &Event, ctx: &AppContext) {
        match event {
            Event::Modifiers(modifiers) => self.alt_is_held = modifiers.alt_key(),
            Event::InputDevice {
                event:
                    InputDeviceEvent::Keyboard(KeyEvent {
                        physical_key: _,
                        logical_key,
                        text: _,
                        location: _,
                        state,
                        repeat,
                        ..
                    }),
                device: _,
            } => match logical_key {
                Key::Named(NamedKey::Enter) => {
                    if self.alt_is_held && state.is_pressed() && !*repeat {
                        match ctx.window.fullscreen() {
                            Some(_) => {
                                ctx.window.set_fullscreen(None);
                            }
                            None => {
                                ctx.window
                                    .set_fullscreen(Some(Fullscreen::Borderless(None)));
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
