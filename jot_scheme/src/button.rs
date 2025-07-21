use std::mem::replace;

use super::*;

use bitvec::array::BitArray;
use private::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Button {
    pub is_triggered: bool,
    pub is_held: bool,
}

impl InputState for Button {
    type Bindings = FlatBindings;
    type ResolverState = ButtonResolver;

    fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
        ButtonResolver {
            bindings: bindings.to_map(),
            held_bindings: BitArray::default(),
            is_triggered: false,
        }
    }

    fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent) {
        fn set_binding(resolver: &mut ButtonResolver, binding_idx: usize, is_held: bool) {
            if is_held && !resolver.held_bindings[binding_idx] {
                resolver.is_triggered = true;
            }

            resolver.held_bindings.set(binding_idx, is_held);
        }

        match event {
            InputDeviceEvent::Connect => {}
            InputDeviceEvent::Disconnect => {}

            InputDeviceEvent::Key(KeyEvent {
                physical_key: PhysicalKey::Code(key),
                state,
                ..
            }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Key(*key)) {
                    set_binding(resolver, *binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Mouse(MouseEvent::Button { button, state }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::MouseButton(*button))
                {
                    set_binding(resolver, *binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Mouse(MouseEvent::Scroll(scroll_delta)) => {
                if scroll_delta.x() > 0.0 {
                    if let Some(_) = resolver.bindings.get(&FlatBinding::MouseScrollRight) {
                        resolver.is_triggered = true;
                    }
                } else if scroll_delta.x() < 0.0 {
                    if let Some(_) = resolver.bindings.get(&FlatBinding::MouseScrollLeft) {
                        resolver.is_triggered = true;
                    }
                }

                if scroll_delta.y() > 0.0 {
                    if let Some(_) = resolver.bindings.get(&FlatBinding::MouseScrollUp) {
                        resolver.is_triggered = true;
                    }
                } else if scroll_delta.y() < 0.0 {
                    if let Some(_) = resolver.bindings.get(&FlatBinding::MouseScrollDown) {
                        resolver.is_triggered = true;
                    }
                }
            }

            InputDeviceEvent::Gamepad(GamepadEvent::Button { button, state }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Button(*button)) {
                    set_binding(resolver, *binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Gamepad(GamepadEvent::Value { code, value }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Value(*code)) {
                    set_binding(resolver, *binding_idx, *value > u8::MAX / 2);
                }
            }

            InputDeviceEvent::Mouse(_) => {}
            InputDeviceEvent::Key(_) => {}
        }
    }

    fn step(resolver: &mut Self::ResolverState) -> Self {
        Self {
            is_triggered: replace(&mut resolver.is_triggered, false),
            is_held: resolver.held_bindings.any(),
        }
    }
}

mod private {
    use std::collections::HashMap;

    use bitvec::array::BitArray;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ButtonResolver {
        pub(super) bindings: HashMap<FlatBinding, usize>,
        pub(super) held_bindings: BitArray<[u64; 1]>,
        pub(super) is_triggered: bool,
    }
}
