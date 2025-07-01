use super::*;

pub trait Prim {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
}

macro_rules! int_impl {
    ($ty:ident) => {
        impl Prim for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const THREE: Self = 3;
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

macro_rules! float_impl {
    ($ty:ident) => {
        impl Prim for $ty {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const TWO: Self = 2.0;
            const THREE: Self = 3.0;
        }
    };
}
float_impl!(f32);
float_impl!(f64);

impl Prim for s32 {
    const ZERO: Self = Self::int(0);
    const ONE: Self = Self::int(1);
    const TWO: Self = Self::int(2);
    const THREE: Self = Self::int(3);
}
