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
pub struct SVec3 {
    pub x: s32,
    pub y: s32,
    pub z: s32,
}

pub const fn svec3(x: s32, y: s32, z: s32) -> SVec3 {
    SVec3 { x, y, z }
}

impl SVec3 {
    pub const ZERO: Self = svec3(s32::ZERO, s32::ZERO, s32::ZERO);
    pub const ONE: Self = svec3(s32::ONE, s32::ONE, s32::ONE);
    pub const HALF: Self = svec3(s32::HALF, s32::HALF, s32::HALF);

    pub const RIGHT: Self = svec3(s32::ONE, s32::ZERO, s32::ZERO);
    pub const LEFT: Self = svec3(s32::NEG_ONE, s32::ZERO, s32::ZERO);
    pub const UP: Self = svec3(s32::ZERO, s32::ONE, s32::ZERO);
    pub const DOWN: Self = svec3(s32::ZERO, s32::NEG_ONE, s32::ZERO);
    pub const FORWARD: Self = svec3(s32::ZERO, s32::ZERO, s32::ONE);
    pub const BACKWARD: Self = svec3(s32::ZERO, s32::NEG_ONE, s32::NEG_ONE);

    pub const fn splat(value: s32) -> Self {
        svec3(value, value, value)
    }

    pub const fn from_u8(value: U8Vec3) -> Self {
        svec3(
            s32::from_u8(value.x),
            s32::from_u8(value.y),
            s32::from_u8(value.z),
        )
    }
    pub const fn from_u16(value: U16Vec3) -> Self {
        svec3(
            s32::from_u16(value.x),
            s32::from_u16(value.y),
            s32::from_u16(value.z),
        )
    }
    pub const fn from_u32(value: UVec3) -> Self {
        svec3(
            s32::from_u32(value.x),
            s32::from_u32(value.y),
            s32::from_u32(value.z),
        )
    }
    pub const fn from_u64(value: U64Vec3) -> Self {
        svec3(
            s32::from_u64(value.x),
            s32::from_u64(value.y),
            s32::from_u64(value.z),
        )
    }
    pub const fn from_usize(value: USizeVec3) -> Self {
        svec3(
            s32::from_usize(value.x),
            s32::from_usize(value.y),
            s32::from_usize(value.z),
        )
    }
    pub const fn from_i8(value: I8Vec3) -> Self {
        svec3(
            s32::from_i8(value.x),
            s32::from_i8(value.y),
            s32::from_i8(value.z),
        )
    }
    pub const fn from_i16(value: I16Vec3) -> Self {
        svec3(
            s32::from_i16(value.x),
            s32::from_i16(value.y),
            s32::from_i16(value.z),
        )
    }
    pub const fn from_i32(value: IVec3) -> Self {
        svec3(
            s32::from_i32(value.x),
            s32::from_i32(value.y),
            s32::from_i32(value.z),
        )
    }
    pub const fn from_i64(value: I64Vec3) -> Self {
        svec3(
            s32::from_i64(value.x),
            s32::from_i64(value.y),
            s32::from_i64(value.z),
        )
    }
    pub const fn from_f32(value: FVec3) -> Self {
        svec3(
            s32::from_f32(value.x),
            s32::from_f32(value.y),
            s32::from_f32(value.z),
        )
    }
    pub const fn from_f64(value: DVec3) -> Self {
        svec3(
            s32::from_f64(value.x),
            s32::from_f64(value.y),
            s32::from_f64(value.z),
        )
    }
    pub const fn as_f32(self) -> FVec3 {
        fvec3(self.x.as_f32(), self.y.as_f32(), self.z.as_f32())
    }
    pub const fn as_f64(self) -> DVec3 {
        dvec3(self.x.as_f64(), self.y.as_f64(), self.z.as_f64())
    }

    pub const fn int(x: i32, y: i32, z: i32) -> Self {
        svec3(s32::int(x), s32::int(y), s32::int(z))
    }

    pub const fn min(self, other: Self) -> Self {
        svec2(self.x.min(other.x), self.y.min(other.y))
    }
    pub const fn max(self, other: Self) -> Self {
        svec2(self.x.max(other.x), self.y.max(other.y))
    }
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        svec2(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    pub const fn round(self) -> Self {
        svec2(self.x.round(), self.y.round())
    }
    pub const fn floor(self) -> Self {
        svec2(self.x.floor(), self.y.floor())
    }
    pub const fn ceil(self) -> Self {
        svec2(self.x.ceil(), self.y.ceil())
    }
    pub const fn trunc(self) -> Self {
        svec2(self.x.trunc(), self.y.trunc())
    }
    pub const fn atrunc(self) -> Self {
        svec2(self.x.atrunc(), self.y.atrunc())
    }

    pub const fn iround(self) -> IVec2 {
        ivec2(self.x.iround(), self.y.iround())
    }
    pub const fn ifloor(self) -> IVec2 {
        ivec2(self.x.ifloor(), self.y.ifloor())
    }
    pub const fn iceil(self) -> IVec2 {
        ivec2(self.x.iceil(), self.y.iceil())
    }
    pub const fn itrunc(self) -> IVec2 {
        ivec2(self.x.itrunc(), self.y.itrunc())
    }
    pub const fn iatrunc(self) -> IVec2 {
        ivec2(self.x.iatrunc(), self.y.iatrunc())
    }

    pub const fn move_towards(self, target: Self, max_delta: Self) -> Self {
        svec2(
            self.x.move_towards(target.x, max_delta.x),
            self.y.move_towards(target.y, max_delta.y),
        )
    }
}

impl Mul<s32> for SVec3 {
    type Output = Self;

    fn mul(self, rhs: s32) -> Self::Output {
        svec3(self.x.mul(rhs), self.y.mul(rhs), self.z.mul(rhs))
    }
}
impl Div<s32> for SVec3 {
    type Output = Self;

    fn div(self, rhs: s32) -> Self::Output {
        svec3(self.x.div(rhs), self.y.div(rhs), self.z.div(rhs))
    }
}
