use super::*;

rectangle_aliases!(pub S => s32);

impl RectScalar for s32 {
    fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
    {
        vec / Self::int(2)
    }

    fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
    {
        vec * Self::int(2)
    }

    fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
    {
        (vec - rhs).abs()
    }

    fn rect_vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
    {
        vec.min(other)
    }
    fn rect_vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
    {
        vec.max(other)
    }
}
