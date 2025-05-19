use super::*;

pub trait Right {
    const RIGHT: Self;
}
pub trait Left {
    const LEFT: Self;
}
pub trait Up {
    const UP: Self;
}
pub trait Down {
    const DOWN: Self;
}

pub trait RightUp: Right + Up {
    const RIGHT_UP: Self;
}
pub trait LeftUp: Left + Up {
    const LEFT_UP: Self;
}
pub trait RightDown: Right + Down {
    const RIGHT_DOWN: Self;
}
pub trait LeftDown: Left + Down {
    const LEFT_DOWN: Self;
}

macro_rules! uint_impl {
    ($ty:ty) => {
        impl Right for $ty {
            const RIGHT: Self = 1;
        }
        impl Up for $ty {
            const UP: Self = 1;
        }
    };
}
macro_rules! sint_impl {
    ($ty:ty) => {
        impl Right for $ty {
            const RIGHT: Self = 1;
        }
        impl Left for $ty {
            const LEFT: Self = -1;
        }
        impl Up for $ty {
            const UP: Self = 1;
        }
        impl Down for $ty {
            const DOWN: Self = -1;
        }
    };
}
macro_rules! float_impl {
    ($ty:ty) => {
        impl Right for $ty {
            const RIGHT: Self = 1.0;
        }
        impl Left for $ty {
            const LEFT: Self = -1.0;
        }
        impl Up for $ty {
            const UP: Self = 1.0;
        }
        impl Down for $ty {
            const DOWN: Self = -1.0;
        }
    };
}
macro_rules! scaled_impl {
    ($ty:ty) => {
        impl Right for $ty {
            const RIGHT: Self = Self::ONE;
        }
        impl Left for $ty {
            const LEFT: Self = Self::NEG_ONE;
        }
        impl Up for $ty {
            const UP: Self = Self::ONE;
        }
        impl Down for $ty {
            const DOWN: Self = Self::NEG_ONE;
        }
    };
}
uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(u128);
uint_impl!(usize);
sint_impl!(i8);
sint_impl!(i16);
sint_impl!(i32);
sint_impl!(i64);
sint_impl!(i128);
sint_impl!(isize);
float_impl!(f32);
float_impl!(f64);
scaled_impl!(s32);
