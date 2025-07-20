use std::{
    fmt::{Debug, Display},
    ops::{Div, DivAssign, Mul, MulAssign},
};

use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};

use super::*;

mod mat;
mod rect;
mod vec;
pub use mat::*;
pub use rect::*;
pub use vec::*;

/// Fixed-point number with 32 bits, and 8 fractional bits.
#[allow(non_camel_case_types)]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Neg, Add, Sub, AddAssign, SubAssign,
)]
pub struct s32(pub i32);

pub trait AsS32 {
    type Output;

    fn as_s32(self) -> Self::Output;
}

impl s32 {
    pub const BITS: usize = 32;
    pub const FRACT_BITS: usize = 8;
    pub const FRACT_SCALE_F32: f32 = 1.0 / 256.0;
    pub const FRACT_SCALE_F64: f64 = 1.0 / 256.0;

    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1 << Self::FRACT_BITS);
    pub const NEG_ONE: Self = Self(-1 << Self::FRACT_BITS);
    pub const EPSILON: Self = Self(1);
    pub const NEG_EPSILON: Self = Self(-1);
    pub const HALF: Self = Self(1 << (Self::FRACT_BITS - 1));
    pub const NEG_HALF: Self = Self(-1 << (Self::FRACT_BITS - 1));

    pub const MIN: Self = Self(i32::MIN);
    pub const MAX: Self = Self(i32::MAX);

    pub const fn from_u8(value: u8) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_u16(value: u16) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_u32(value: u32) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_u64(value: u64) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_u128(value: u128) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_usize(value: usize) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_i8(value: i8) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_i16(value: i16) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_i32(value: i32) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_i64(value: i64) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_i128(value: i128) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_isize(value: isize) -> Self {
        Self::int(value as i32)
    }
    pub const fn from_f32(value: f32) -> Self {
        Self((value * 256.0) as i32)
    }
    pub const fn from_f64(value: f64) -> Self {
        Self((value * 256.0) as i32)
    }

    pub const fn as_f32(self) -> f32 {
        (self.0 as f32) * Self::FRACT_SCALE_F32
    }
    pub const fn as_f64(self) -> f64 {
        (self.0 as f64) * Self::FRACT_SCALE_F64
    }

    pub const fn int(value: i32) -> Self {
        Self(value << Self::FRACT_BITS)
    }
    pub const fn pixels(value: i32) -> Self {
        Self(value << (Self::FRACT_BITS - 4))
    }

    pub const fn neg(self) -> Self {
        Self(-self.0)
    }
    pub const fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
    pub const fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
    pub const fn mul(self, rhs: Self) -> Self {
        Self((((self.0 as i64) * rhs.0 as i64) >> Self::FRACT_BITS) as i32)
    }
    pub const fn div(self, rhs: Self) -> Self {
        Self((((self.0 as i64) << Self::FRACT_BITS) / rhs.0 as i64) as i32)
    }

    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 { self } else { other }
    }
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 { self } else { other }
    }
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        if self.0 > max.0 {
            max
        } else if self.0 < min.0 {
            min
        } else {
            self
        }
    }

    pub const fn floor(self) -> Self {
        Self(self.0 >> Self::FRACT_BITS << Self::FRACT_BITS)
    }
    pub const fn ceil(self) -> Self {
        Self(self.0 + Self::ONE.0 - Self::EPSILON.0).floor()
    }
    pub const fn round(self) -> Self {
        Self(self.0 + Self::HALF.0 - Self::EPSILON.0).floor()
    }
    pub const fn trunc(self) -> Self {
        if self.0 >= 0 {
            self.floor()
        } else {
            self.ceil()
        }
    }
    pub const fn atrunc(self) -> Self {
        if self.0 >= 0 {
            self.ceil()
        } else {
            self.floor()
        }
    }

    pub const fn ifloor(self) -> i32 {
        self.0 >> Self::FRACT_BITS
    }
    pub const fn iceil(self) -> i32 {
        Self(self.0 + Self::ONE.0 - Self::EPSILON.0).ifloor()
    }
    pub const fn iround(self) -> i32 {
        Self(self.0 + Self::HALF.0 - Self::EPSILON.0).ifloor()
    }
    pub const fn itrunc(self) -> i32 {
        if self.0 >= 0 {
            self.ifloor()
        } else {
            self.iceil()
        }
    }
    pub const fn iatrunc(self) -> i32 {
        if self.0 >= 0 {
            self.iceil()
        } else {
            self.ifloor()
        }
    }

    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub const fn move_towards(self, target: Self, max_delta: Self) -> Self {
        target.clamp(Self(self.0 + max_delta.0), Self(self.0 - max_delta.0))
    }
}

impl Mul for s32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}
impl MulAssign for s32 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl Div for s32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}
impl DivAssign for s32 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Debug for s32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_f64())
    }
}
impl Display for s32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}
