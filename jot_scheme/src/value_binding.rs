use std::collections::HashMap;

use tinyset::Set64;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValueBindings {
    pub values: Set64<ValueCode>,
    pub flat: FlatBindings,
}

impl InputBindings for ValueBindings {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ValueBinding {
    Value(ValueCode),
}

impl ValueBindings {
    pub(super) fn to_map(&self) -> HashMap<ValueBinding, usize> {
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
