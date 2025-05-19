use std::ops::{Div, Mul};

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::*;

#[repr(C)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
)]
#[mul(forward)]
#[mul_assign(forward)]
#[div(forward)]
#[div_assign(forward)]
pub struct SVec4 {
    pub x: s32,
    pub y: s32,
    pub z: s32,
    pub w: s32,
}

pub const fn svec4(x: s32, y: s32, z: s32, w: s32) -> SVec4 {
    SVec4 { x, y, z, w }
}

impl Mul<s32> for SVec4 {
    type Output = Self;

    fn mul(self, rhs: s32) -> Self::Output {
        svec4(
            self.x.mul(rhs),
            self.y.mul(rhs),
            self.z.mul(rhs),
            self.w.mul(rhs),
        )
    }
}
impl Div<s32> for SVec4 {
    type Output = Self;

    fn div(self, rhs: s32) -> Self::Output {
        svec4(
            self.x.div(rhs),
            self.y.div(rhs),
            self.z.div(rhs),
            self.w.div(rhs),
        )
    }
}
