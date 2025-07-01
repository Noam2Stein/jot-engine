use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SRect {
    pub center: SVec2,
    pub extents: SVec2,
}

impl SRect {
    pub const fn new(center: SVec2, extents: SVec2) -> Self {
        Self { center, extents }
    }
    pub const fn min_max(min: SVec2, max: SVec2) -> Self {
        Self {
            center: svec2(s32((min.x.0 + max.x.0) >> 1), s32((min.y.0 + max.y.0) >> 1)),
            extents: svec2(s32((max.x.0 - min.x.0) >> 1), s32((max.y.0 - min.y.0) >> 1)),
        }
    }

    pub const fn min(self) -> SVec2 {
        svec2(
            s32(self.center.x.0 - self.extents.x.0),
            s32(self.center.y.0 - self.extents.y.0),
        )
    }
    pub const fn max(self) -> SVec2 {
        svec2(
            s32(self.center.x.0 + self.extents.x.0),
            s32(self.center.y.0 + self.extents.y.0),
        )
    }

    pub const fn right(self) -> s32 {
        self.max().x
    }
    pub const fn left(self) -> s32 {
        self.min().x
    }
    pub const fn top(self) -> s32 {
        self.max().y
    }
    pub const fn bottom(self) -> s32 {
        self.min().y
    }
    pub const fn top_right(self) -> SVec2 {
        svec2(self.right(), self.top())
    }
    pub const fn top_left(self) -> SVec2 {
        svec2(self.left(), self.top())
    }
    pub const fn bottom_right(self) -> SVec2 {
        svec2(self.right(), self.bottom())
    }
    pub const fn bottom_left(self) -> SVec2 {
        svec2(self.left(), self.bottom())
    }

    pub const fn intersects(&self, other: &Self) -> bool {
        let x = self.left().0 < other.right().0 && self.right().0 > other.left().0;
        let y = self.bottom().0 < other.top().0 && self.top().0 > other.bottom().0;

        x && y
    }

    pub const fn expand(self, point: SVec2) -> Self {
        Self::min_max(self.min().min(point), self.max().max(point))
    }
    pub const fn offset(self, offset: SVec2) -> Self {
        Self {
            center: svec2(
                s32(self.center.x.0 + offset.x.0),
                s32(self.center.y.0 + offset.y.0),
            ),
            extents: self.extents,
        }
    }
}
