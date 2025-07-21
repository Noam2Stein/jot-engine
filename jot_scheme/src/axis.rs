use jot_scheme_proc_macros::Input_Local;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Input_Local)]
pub struct Axis {
    pub positive: Value,
    pub negative: Value,
}

impl Axis {
    pub fn value(&self) -> i8 {
        self.positive.value as i8 - self.negative.value as i8
    }

    pub fn as_s32(&self) -> s32 {
        s32::from_i8(self.value()) / s32::int(16)
    }
}
