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
    use std::{
        collections::HashMap,
        ops::{Add, AddAssign},
    };

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

    impl Add for &ButtonBindings {
        type Output = ButtonBindings;

        fn add(self, rhs: Self) -> Self::Output {
            let mut output = self.clone();

            output.keys.extend(rhs.keys.iter());
            output.mouse_buttons.extend(rhs.mouse_buttons.iter());
            output.mouse_scroll_right_enabled |= rhs.mouse_scroll_right_enabled;
            output.mouse_scroll_left_enabled |= rhs.mouse_scroll_left_enabled;
            output.mouse_scroll_up_enabled |= rhs.mouse_scroll_up_enabled;
            output.mouse_scroll_down_enabled |= rhs.mouse_scroll_down_enabled;
            output.buttons.extend(rhs.buttons.iter());
            output.values.extend(rhs.values.iter());

            output
        }
    }
    impl AddAssign<&ButtonBindings> for ButtonBindings {
        fn add_assign(&mut self, rhs: &ButtonBindings) {
            self.keys.extend(rhs.keys.iter());
            self.mouse_buttons.extend(rhs.mouse_buttons.iter());
            self.mouse_scroll_right_enabled |= rhs.mouse_scroll_right_enabled;
            self.mouse_scroll_left_enabled |= rhs.mouse_scroll_left_enabled;
            self.mouse_scroll_up_enabled |= rhs.mouse_scroll_up_enabled;
            self.mouse_scroll_down_enabled |= rhs.mouse_scroll_down_enabled;
            self.buttons.extend(rhs.buttons.iter());
            self.values.extend(rhs.values.iter());
        }
    }

    impl BindingsType for ButtonBindings {
        fn flatten(&mut self) {}
    }

    impl<T: Into<ButtonBindings>> From<Flat<T>> for ButtonBindings {
        fn from(value: Flat<T>) -> Self {
            value.0.into()
        }
    }
}

impl Button {
    pub fn key(key: KeyCode) -> Bindings<Self> {
        Bindings::<Self> {
            keys: Set64::from_iter([key]),
            ..Default::default()
        }
    }

    pub fn mouse_button(button: MouseButton) -> Bindings<Self> {
        Bindings::<Self> {
            mouse_buttons: Set64::from_iter([button]),
            ..Default::default()
        }
    }
    pub fn scroll_right() -> Bindings<Self> {
        Bindings::<Self> {
            mouse_scroll_right_enabled: true,
            ..Default::default()
        }
    }
    pub fn scroll_left() -> Bindings<Self> {
        Bindings::<Self> {
            mouse_scroll_left_enabled: true,
            ..Default::default()
        }
    }
    pub fn scroll_up() -> Bindings<Self> {
        Bindings::<Self> {
            mouse_scroll_up_enabled: true,
            ..Default::default()
        }
    }
    pub fn scroll_down() -> Bindings<Self> {
        Bindings::<Self> {
            mouse_scroll_down_enabled: true,
            ..Default::default()
        }
    }

    pub fn button(button: ButtonCode) -> Bindings<Self> {
        Bindings::<Self> {
            buttons: Set64::from_iter([button]),
            ..Default::default()
        }
    }
    pub fn value(value: ValueCode) -> Bindings<Self> {
        Bindings::<Self> {
            values: Set64::from_iter([value]),
            ..Default::default()
        }
    }
}

pub struct ScrollRight;
pub struct ScrollLeft;
pub struct ScrollUp;
pub struct ScrollDown;

impl From<KeyCode> for Bindings<Button> {
    fn from(key: KeyCode) -> Self {
        Button::key(key)
    }
}
impl From<MouseButton> for Bindings<Button> {
    fn from(button: MouseButton) -> Self {
        Button::mouse_button(button)
    }
}
impl From<ScrollRight> for Bindings<Button> {
    fn from(_: ScrollRight) -> Self {
        Button::scroll_right()
    }
}
impl From<ScrollLeft> for Bindings<Button> {
    fn from(_: ScrollLeft) -> Self {
        Button::scroll_left()
    }
}
impl From<ScrollUp> for Bindings<Button> {
    fn from(_: ScrollUp) -> Self {
        Button::scroll_up()
    }
}
impl From<ScrollDown> for Bindings<Button> {
    fn from(_: ScrollDown) -> Self {
        Button::scroll_down()
    }
}
impl From<ButtonCode> for Bindings<Button> {
    fn from(button: ButtonCode) -> Self {
        Button::button(button)
    }
}
impl From<ValueCode> for Bindings<Button> {
    fn from(value: ValueCode) -> Self {
        Button::value(value)
    }
}

repetitive! {
    @for len in 0..=16 {
        @let GenericParams = @{
            @for i in 0..len {
                @['T i]: Into<Bindings<Button>>,
            }
        };
        @let GenericArgs = @{
            @for i in 0..len {
                @['T i],
            }
        };
        @let Tuple = @{(@GenericArgs)};

        impl<@GenericParams> From<@Tuple> for Bindings<Button> {
            fn from(#[allow(unused_variables)] value: @Tuple) -> Self {
                #[allow(unused_mut)]
                let mut output = Default::default();

                @for i in 0..len {
                    output += &value.@i.into();
                }

                output
            }
        }
    }
}
