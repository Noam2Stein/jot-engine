use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FRect {
    pub center: FVec2,
    pub extents: FVec2,
}

impl FRect {
    pub const fn new(center: FVec2, extents: FVec2) -> Self {
        Self { center, extents }
    }
}
