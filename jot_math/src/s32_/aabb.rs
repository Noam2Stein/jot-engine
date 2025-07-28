use super::*;

aabb_aliases!(pub S => s32);

impl AabbScalar for s32 {
    fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec / Self::int(2)
    }

    fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec * Self::int(2)
    }

    fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
    {
        (vec - rhs).abs()
    }

    fn aabb_vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec.min(other)
    }
    fn aabb_vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec.max(other)
    }
}
