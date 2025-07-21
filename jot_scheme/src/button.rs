use super::*;

use bitvec::array::BitArray;
use private::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Button {
    pub is_held: bool,
}

impl InputState for Button {
    type Bindings = FlatBindings;
    type ResolverState = ButtonResolver;

    fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
        ButtonResolver {
            bindings: bindings.to_map(),
            held_bindings: BitArray::default(),
        }
    }

    fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent) {
        match event {
            InputDeviceEvent::Connect => {}
            InputDeviceEvent::Disconnect => {}

            InputDeviceEvent::Key(KeyEvent {
                physical_key: PhysicalKey::Code(key),
                state,
                ..
            }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Key(*key)) {
                    resolver.held_bindings.set(*binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Mouse(MouseEvent::Button { button, state }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::MouseButton(*button))
                {
                    resolver.held_bindings.set(*binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Gamepad(GamepadEvent::Button { button, state }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Button(*button)) {
                    resolver.held_bindings.set(*binding_idx, state.is_pressed());
                }
            }

            InputDeviceEvent::Gamepad(GamepadEvent::Value { code, value }) => {
                if let Some(binding_idx) = resolver.bindings.get(&FlatBinding::Value(*code)) {
                    resolver
                        .held_bindings
                        .set(*binding_idx, *value > u8::MAX / 2);
                }
            }

            InputDeviceEvent::Mouse(_) => {}
            InputDeviceEvent::Key(_) => {}
        }
    }

    fn step(resolver: &mut Self::ResolverState) -> Self {
        Self {
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
    }
}
