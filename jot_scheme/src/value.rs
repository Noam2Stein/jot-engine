use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Value {
    /// Values are represented by an int ranging from `0` to `16`.
    pub value: u8,

    /// Is true if at least a single binding was pressed down this frame.
    ///
    /// This is true even if the button was already held down by one binding,
    /// as long as a different binding was just pressed.
    pub is_triggered: bool,

    /// Is true if atleast a single binding is held down.
    ///
    /// Don't use this value to detect if a button was just pressed (with the `is_held & !was_held_previous_frame` technique).
    /// Instead, use the `is_triggered` field.
    pub is_held: bool,
}

impl Value {
    pub fn as_s32(&self) -> s32 {
        s32::from_u8(self.value) / s32::int(16)
    }
}

mod private {
    use std::collections::HashMap;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ValueBindings {
        pub values: Set64<ValueCode>,
        pub flat: Bindings<Button>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct ValueResolver {
        flat: Resolver<Button>,
        button: Resolver<Button>,
        binding_map: HashMap<ValueBinding, usize>,
        binding_values: SmallVec<[u8; 64]>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum ValueBinding {
        Value(ValueCode),
    }

    impl Input for Value {
        type Bindings = ValueBindings;
        type ResolverState = ValueResolver;

        fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
            let binding_map = bindings.to_map();

            ValueResolver {
                button: Resolver::new(bindings.to_button_bindings()),
                flat: Resolver::new(bindings.flat),
                binding_values: SmallVec::from_elem(0, binding_map.len()),
                binding_map,
            }
        }

        fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent) {
            resolver.flat.event(event);
            resolver.button.event(event);

            match event {
                InputDeviceEvent::Gamepad(GamepadEvent::Value { code, value }) => {
                    if let Some(binding_idx) = resolver.binding_map.get(&ValueBinding::Value(*code))
                    {
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
            let flag_value = if resolver.flat.step().is_held { 16 } else { 0 };

            let value = flag_value.max(resolver.binding_values.iter().copied().max().unwrap_or(0));

            let Button {
                is_triggered,
                is_held,
            } = resolver.button.step();

            Self {
                value,
                is_triggered,
                is_held,
            }
        }
    }

    impl ValueBindings {
        fn to_button_bindings(&self) -> Bindings<Button> {
            Bindings::<Button> {
                keys: self.flat.keys.clone(),
                mouse_buttons: self.flat.mouse_buttons.clone(),
                mouse_scroll_right_enabled: self.flat.mouse_scroll_right_enabled,
                mouse_scroll_left_enabled: self.flat.mouse_scroll_left_enabled,
                mouse_scroll_up_enabled: self.flat.mouse_scroll_up_enabled,
                mouse_scroll_down_enabled: self.flat.mouse_scroll_down_enabled,
                buttons: self.flat.buttons.clone(),

                values: self.values.iter().chain(self.flat.values.iter()).collect(),
            }
        }

        fn to_map(&self) -> HashMap<ValueBinding, usize> {
            let capacity = self.values.len();

            let mut output = HashMap::with_capacity(capacity);
            let mut idx = 0;

            for binding in self.values.iter() {
                output.insert(ValueBinding::Value(binding), idx);
                idx += 1;
            }

            output
        }
    }
}
