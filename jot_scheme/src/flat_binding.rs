use std::collections::HashMap;

use tinyset::Set64;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FlatBindings {
    pub keys: Set64<KeyCode>,
    pub mouse_buttons: Set64<MouseButton>,
    pub mouse_scroll_right_enabled: bool,
    pub mouse_scroll_left_enabled: bool,
    pub mouse_scroll_up_enabled: bool,
    pub mouse_scroll_down_enabled: bool,
    pub buttons: Set64<ButtonCode>,
    pub values: Set64<ValueCode>,
}

impl InputBindings for FlatBindings {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum FlatBinding {
    Key(KeyCode),
    MouseButton(MouseButton),
    MouseScrollRight,
    MouseScrollLeft,
    MouseScrollUp,
    MouseScrollDown,
    Button(ButtonCode),
    Value(ValueCode),
}

impl FlatBindings {
    pub(super) fn to_map(&self) -> HashMap<FlatBinding, usize> {
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
            output.insert(FlatBinding::Key(binding), idx);
            idx += 1;
        }

        for binding in self.mouse_buttons.iter() {
            output.insert(FlatBinding::MouseButton(binding), idx);
            idx += 1;
        }

        if self.mouse_scroll_right_enabled {
            output.insert(FlatBinding::MouseScrollRight, idx);
            idx += 1;
        }

        if self.mouse_scroll_left_enabled {
            output.insert(FlatBinding::MouseScrollLeft, idx);
            idx += 1;
        }

        if self.mouse_scroll_up_enabled {
            output.insert(FlatBinding::MouseScrollUp, idx);
            idx += 1;
        }

        if self.mouse_scroll_down_enabled {
            output.insert(FlatBinding::MouseScrollDown, idx);
            idx += 1;
        }

        for binding in self.buttons.iter() {
            output.insert(FlatBinding::Button(binding), idx);
            idx += 1;
        }

        for binding in self.values.iter() {
            output.insert(FlatBinding::Value(binding), idx);
            idx += 1;
        }

        output
    }
}
