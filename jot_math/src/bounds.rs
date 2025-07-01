use paste::paste;
use seq_macro::seq;

use super::*;

macro_rules! define_bounds {
    { $($prim:ident $prefix:ident $prefix_low:ident), * $(,)? } => {
        seq! { N in 2..=4 { paste! {$(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct [<$prefix Bounds N>] {
                pub center: [<$prefix Vec N>],
                pub extents: [<$prefix Vec N>],
            }

            impl [<$prefix Bounds N>] {
                pub const fn new(center: [<$prefix Vec N>], extents: [<$prefix Vec N>]) -> Self {
                    Self { center, extents }
                }
                pub fn min_max(min: [<$prefix Vec N>], max: [<$prefix Vec N>]) -> Self {
                    Self {
                        center: (min + max) / $prim::TWO,
                        extents: (max - min) / $prim::TWO,
                    }
                }

                pub fn min(self) -> [<$prefix Vec N>] {
                    self.center - self.extents
                }
                pub fn max(self) -> [<$prefix Vec N>] {
                    self.center + self.extents
                }

                pub fn right(self) -> $prim {
                    self.max().x
                }
                pub fn left(self) -> $prim {
                    self.min().x
                }
                pub fn top(self) -> $prim {
                    self.max().y
                }
                pub fn bottom(self) -> $prim {
                    self.min().y
                }
                pub fn top_right(self) -> [<$prefix Vec 2>] {
                    [<$prefix_low vec 2>](self.right(), self.top())
                }
                pub fn top_left(self) -> [<$prefix Vec 2>] {
                    [<$prefix_low vec 2>](self.left(), self.top())
                }
                pub fn bottom_right(self) -> [<$prefix Vec 2>] {
                    [<$prefix_low vec 2>](self.right(), self.bottom())
                }
                pub fn bottom_left(self) -> [<$prefix Vec 2>] {
                    [<$prefix_low vec 2>](self.left(), self.bottom())
                }

                pub fn intersects(&self, other: &Self) -> bool {
                    let x = self.left() < other.right() && self.right() > other.left();
                    let y = self.bottom() < other.top() && self.top() > other.bottom();

                    x && y
                }

                pub fn expand(self, point: [<$prefix Vec N>]) -> Self {
                    Self::min_max(self.min().min(point), self.max().max(point))
                }
                pub fn offset(self, offset: SVec2) -> Self {
                    Self {
                        center: svec2(
                            s32(self.center.x.0 + offset.x.0),
                            s32(self.center.y.0 + offset.y.0),
                        ),
                        extents: self.extents,
                    }
                }
            }
        )*} } }
    };
}
define_bounds! {
    u8 U8 u8, u16 U16 u16, u32 U u, u64 U64 u64, usize USize usize,
    i8 I8 i8, i16 I16 i16, i32 I i, i64 I64 i64,
    f32 F f, f64 D d,

    s32 S s,
}
