use super::*;

pub struct FullscreenSwitch {
    alt_is_held: bool,
}

pub enum FullscreenSwitchEvent {
    Alt(bool),
    Enter,
}

impl FullscreenSwitch {
    pub fn new() -> Self {
        Self { alt_is_held: false }
    }

    pub fn event(&mut self, event: impl TryInto<FullscreenSwitchEvent, Error = ()>, ctx: &Context) {
        match event.try_into() {
            Ok(FullscreenSwitchEvent::Alt(alt_is_pressed)) => self.alt_is_held = alt_is_pressed,
            Ok(FullscreenSwitchEvent::Enter) => {
                if self.alt_is_held {
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
            Err(()) => {}
        }
    }
}

impl TryFrom<&WindowEvent> for FullscreenSwitchEvent {
    type Error = ();

    fn try_from(value: &WindowEvent) -> Result<Self, Self::Error> {
        match value {
            WindowEvent::ModifiersChanged(modifiers) => Ok(Self::Alt(modifiers.state().alt_key())),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: _,
                        logical_key,
                        text: _,
                        location: _,
                        state,
                        repeat,
                        ..
                    },
                device_id: _,
                is_synthetic: _,
            } => match logical_key {
                Key::Named(NamedKey::Enter) => {
                    if state.is_pressed() && !*repeat {
                        Ok(Self::Enter)
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}
