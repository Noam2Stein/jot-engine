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
#[repr(align(16))]
pub struct SVec3A {
    pub x: s32,
    pub y: s32,
    pub z: s32,
}

pub const fn svec3a(x: s32, y: s32, z: s32) -> SVec3A {
    SVec3A { x, y, z }
}

impl Mul<s32> for SVec3A {
    type Output = Self;

    fn mul(self, rhs: s32) -> Self::Output {
        svec3a(self.x.mul(rhs), self.y.mul(rhs), self.z.mul(rhs))
    }
}
impl Div<s32> for SVec3A {
    type Output = Self;

    fn div(self, rhs: s32) -> Self::Output {
        svec3a(self.x.div(rhs), self.y.div(rhs), self.z.div(rhs))
    }
}
