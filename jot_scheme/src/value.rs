use super::*;

use private::*;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Value {
    pub value: u8,
}

impl InputState for Value {
    type Bindings = ValueBindings;
    type ResolverState = ValueResolver;

    fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
        let binding_map = bindings.to_map();

        ValueResolver {
            flat: InputResolver::new(bindings.flat),
            binding_values: SmallVec::from_elem(0, binding_map.len()),
            binding_map,
        }
    }

    fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent) {
        resolver.flat.event(event);

        match event {
            InputDeviceEvent::Gamepad(GamepadEvent::Value { code, value }) => {
                if let Some(binding_idx) = resolver.binding_map.get(&ValueBinding::Value(*code)) {
                    resolver.binding_values[*binding_idx] = *value;
                }
            }

            InputDeviceEvent::Connect => {}
            InputDeviceEvent::Disconnect => {}
            InputDeviceEvent::Key(_) => {}
            InputDeviceEvent::Mouse(_) => {}
            InputDeviceEvent::Gamepad(_) => {}
        }
    }

    fn step(resolver: &mut Self::ResolverState) -> Self {
        let flag_value = if resolver.flat.step().is_held {
            u8::MAX
        } else {
            0
        };

        let max_value = flag_value.max(resolver.binding_values.iter().copied().max().unwrap_or(0));

        Self { value: max_value }
    }
}

mod private {
    use std::collections::HashMap;

    use smallvec::SmallVec;

    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct ValueResolver {
        pub(super) flat: InputResolver<Button>,
        pub(super) binding_map: HashMap<ValueBinding, usize>,
        pub(super) binding_values: SmallVec<[u8; 64]>,
    }
}
