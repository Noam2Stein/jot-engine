use std::mem::replace;

use super::*;

/// `InputState` type for buttons, which can be either on or off.
///
/// Buttons track both whever or not they are held, and whever or not they were just pressed down.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Button {
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

mod private {
    use std::collections::HashMap;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ButtonBindings {
        pub keys: Set64<KeyCode>,
        pub mouse_buttons: Set64<MouseButton>,
        pub mouse_scroll_right_enabled: bool,
        pub mouse_scroll_left_enabled: bool,
        pub mouse_scroll_up_enabled: bool,
        pub mouse_scroll_down_enabled: bool,
        pub buttons: Set64<ButtonCode>,
        pub values: Set64<ValueCode>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ButtonResolver {
        binding_map: HashMap<ButtonBinding, usize>,
        held_bindings: BitArray<[u64; 1]>,
        is_triggered: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum ButtonBinding {
        Key(KeyCode),
        MouseButton(MouseButton),
        MouseScrollRight,
        MouseScrollLeft,
        MouseScrollUp,
        MouseScrollDown,
        Button(ButtonCode),
        Value(ValueCode),
    }

    impl InputType for Button {
        type Bindings = ButtonBindings;
        type ResolverState = ButtonResolver;

        fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
            ButtonResolver {
                binding_map: bindings.to_map(),
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
                    if let Some(binding_idx) = resolver.binding_map.get(&ButtonBinding::Key(*key)) {
                        set_binding(resolver, *binding_idx, state.is_pressed());
                    }
                }

                InputDeviceEvent::Mouse(MouseEvent::Button { button, state }) => {
                    if let Some(binding_idx) = resolver
                        .binding_map
                        .get(&ButtonBinding::MouseButton(*button))
                    {
                        set_binding(resolver, *binding_idx, state.is_pressed());
                    }
                }

                InputDeviceEvent::Mouse(MouseEvent::Scroll(scroll_delta)) => {
                    if scroll_delta.x() > 0.0 {
                        if let Some(_) = resolver.binding_map.get(&ButtonBinding::MouseScrollRight)
                        {
                            resolver.is_triggered = true;
                        }
                    } else if scroll_delta.x() < 0.0 {
                        if let Some(_) = resolver.binding_map.get(&ButtonBinding::MouseScrollLeft) {
                            resolver.is_triggered = true;
                        }
                    }

                    if scroll_delta.y() > 0.0 {
                        if let Some(_) = resolver.binding_map.get(&ButtonBinding::MouseScrollUp) {
                            resolver.is_triggered = true;
                        }
                    } else if scroll_delta.y() < 0.0 {
                        if let Some(_) = resolver.binding_map.get(&ButtonBinding::MouseScrollDown) {
                            resolver.is_triggered = true;
                        }
                    }
                }

                InputDeviceEvent::Gamepad(GamepadEvent::Button { button, state }) => {
                    if let Some(binding_idx) =
                        resolver.binding_map.get(&ButtonBinding::Button(*button))
                    {
                        set_binding(resolver, *binding_idx, state.is_pressed());
                    }
                }

                InputDeviceEvent::Gamepad(GamepadEvent::Value { code, value }) => {
                    if let Some(binding_idx) =
                        resolver.binding_map.get(&ButtonBinding::Value(*code))
                    {
                        set_binding(resolver, *binding_idx, *value > 8);
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

    impl ButtonBindings {
        fn to_map(&self) -> HashMap<ButtonBinding, usize> {
            let capacity = self.keys.len()
                + self.mouse_buttons.len()
                + self.mouse_scroll_right_enabled as usize
                + self.mouse_scroll_left_enabled as usize
                + self.mouse_scroll_up_enabled as usize
                + self.mouse_scroll_down_enabled as usize
                + self.buttons.len()
                + self.values.len();

            let mut output = HashMap::with_capacity(capacity);
            let mut idx = 0;

            for binding in self.keys.iter() {
                output.insert(ButtonBinding::Key(binding), idx);
                idx += 1;
            }

            for binding in self.mouse_buttons.iter() {
                output.insert(ButtonBinding::MouseButton(binding), idx);
                idx += 1;
            }

            if self.mouse_scroll_right_enabled {
                output.insert(ButtonBinding::MouseScrollRight, idx);
                idx += 1;
            }

            if self.mouse_scroll_left_enabled {
                output.insert(ButtonBinding::MouseScrollLeft, idx);
                idx += 1;
            }

            if self.mouse_scroll_up_enabled {
                output.insert(ButtonBinding::MouseScrollUp, idx);
                idx += 1;
            }

            if self.mouse_scroll_down_enabled {
                output.insert(ButtonBinding::MouseScrollDown, idx);
                idx += 1;
            }

            for binding in self.buttons.iter() {
                output.insert(ButtonBinding::Button(binding), idx);
                idx += 1;
            }

            for binding in self.values.iter() {
                output.insert(ButtonBinding::Value(binding), idx);
                idx += 1;
            }

            output
        }
    }
}
