use super::*;

vector_aliases!(pub S => s32);

impl Scalar for s32 {
    type Vec2Alignment = <i32 as Scalar>::Vec2Alignment;
    type Vec3Alignment = <i32 as Scalar>::Vec3Alignment;
    type Vec4Alignment = <i32 as Scalar>::Vec4Alignment;
}

pub trait S32VectorExt<const N: usize>
where
    Usize<N>: VecLen,
{
    const ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;

    const EPSILON: Self;
    const NEG_EPSILON: Self;

    const MIN: Self;
    const MAX: Self;

    fn min(self, other: Vector<N, s32, impl VecAlignment>) -> Self;
    fn max(self, other: Vector<N, s32, impl VecAlignment>) -> Self;
    fn clamp(
        self,
        min: Vector<N, s32, impl VecAlignment>,
        max: Vector<N, s32, impl VecAlignment>,
    ) -> Self;

    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn atrunc(self) -> Self;

    fn abs(self) -> Self;
}

impl<const N: usize, A: VecAlignment> S32VectorExt<N> for Vector<N, s32, A>
where
    Usize<N>: VecLen,
{
    const ZERO: Self = Vector::splat(s32::ZERO);
    const ONE: Self = Vector::splat(s32::ONE);
    const NEG_ONE: Self = Vector::splat(s32::NEG_ONE);

    const EPSILON: Self = Vector::splat(s32::EPSILON);
    const NEG_EPSILON: Self = Vector::splat(s32::NEG_EPSILON);

    const MIN: Self = Vector::splat(s32::MIN);
    const MAX: Self = Vector::splat(s32::MAX);

    fn min(self, other: Vector<N, s32, impl VecAlignment>) -> Self {
        self.map_rhs(other, s32::min)
    }
    fn max(self, other: Vector<N, s32, impl VecAlignment>) -> Self {
        self.map_rhs(other, s32::max)
    }
    fn clamp(
        self,
        min: Vector<N, s32, impl VecAlignment>,
        max: Vector<N, s32, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    fn round(self) -> Self {
        self.map(s32::round)
    }
    fn floor(self) -> Self {
        self.map(s32::floor)
    }
    fn ceil(self) -> Self {
        self.map(s32::ceil)
    }
    fn trunc(self) -> Self {
        self.map(s32::trunc)
    }
    fn atrunc(self) -> Self {
        self.map(s32::atrunc)
    }

    fn abs(self) -> Self {
        self.map(s32::abs)
    }
}
