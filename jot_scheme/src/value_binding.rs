use tinyset::Set64;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValueBindings {
    pub values: Set64<ValueCode>,
    pub flat: FlatBindings,
}

impl InputBindings for ValueBindings {}
